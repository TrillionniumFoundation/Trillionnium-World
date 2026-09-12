#!/usr/bin/env python3
"""Hostile fixtures for the non-manifest operational-surface catalog."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

CHECKER_PATH = Path(__file__).with_name("check-trnm-world-operational-surfaces.py")
SPEC = importlib.util.spec_from_file_location("trnm_world_operational_surfaces", CHECKER_PATH)
assert SPEC is not None and SPEC.loader is not None
CHECKER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECKER)

PATH_KIND = dict(CHECKER.EXPECTED_KIND_BY_PATH)


def write(path: Path, text: str = "fixture\n") -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def lifecycle_for(path: str) -> tuple[str, str]:
    if path == ".cargo":
        return "active-build-control", "build-control"
    if path in {".githooks", ".github"}:
        return "active-governance-control", "governance-control"
    if path in {"assets", "packaging"}:
        return "active-game-product", "game-product"
    if path == "config":
        return "compatibility-laboratory", "game-product"
    if path == "deploy/postgres":
        return "cutover-candidate", "world-authority-candidate"
    if path == "deploy/systemd":
        return "compatibility-laboratory", "game-product"
    if path == "playtests":
        return "evidence-protocol", "human-evidence"
    return "qualification-control", "release-evidence"


def data_class_for(path: str) -> str:
    if path == "assets":
        return "licensed-content"
    if path == "config":
        return "configuration-no-secrets"
    if path == "deploy/postgres":
        return "persistence-schema"
    if path == "playtests":
        return "human-evidence-template"
    return "no-user-data"


def make_fixture(root: Path) -> dict[str, object]:
    write(root / "docs/contract.md", "operational surface contract\n")
    write(root / "gates/check.py", "# gate\n")
    # The production catalog deliberately treats assets as one logical surface,
    # while failing closed on its two direct child packs.
    write(root / "assets/first_contact/fixture.txt")
    write(root / "assets/trnm-world/fixture.txt")
    surfaces: list[dict[str, object]] = []
    for index, (path, kind) in enumerate(PATH_KIND.items()):
        sentinel = f"{path}/sentinel-{index}.txt"
        write(root / sentinel)
        lifecycle, denominator = lifecycle_for(path)
        surfaces.append(
            {
                "id": f"surface-{index}",
                "path": path,
                "kind": kind,
                "lifecycle": lifecycle,
                "release_denominator": denominator,
                "owner": "fixture-owner",
                "authority": "fixture-authority",
                "security_class": "internal",
                "data_class": data_class_for(path),
                "canonical_docs": ["docs/contract.md"],
                "gate": "gates/check.py",
                "sentinels": [sentinel],
            }
        )
    catalog = {
        "schema": "trnm_world_operational_surface_catalog_v1",
        "as_of": "2026-09-10",
        "repository": "TrillionniumFoundation/Trillionnium-World",
        "production_authorization": "not_granted",
        "surfaces": surfaces,
    }
    target = root / CHECKER.DEFAULT_CATALOG
    write(target, json.dumps(catalog, indent=2) + "\n")
    return catalog


class OperationalSurfaceCatalogTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory(prefix="trnm-operational-surfaces-")
        self.root = Path(self.temp.name) / "root"
        self.root.mkdir()
        self.catalog = make_fixture(self.root)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def assert_fails(self, pattern: str) -> None:
        with self.assertRaisesRegex(CHECKER.SurfaceCatalogError, pattern):
            CHECKER.validate(self.catalog, self.root)

    def test_complete_fixture_passes(self) -> None:
        count, files, size = CHECKER.validate(self.catalog, self.root)
        self.assertEqual(count, len(PATH_KIND))
        self.assertGreater(files, 0)
        self.assertGreater(size, 0)

    def test_missing_surface_fails_closed(self) -> None:
        self.catalog["surfaces"].pop()
        self.assert_fails("surface count drift")

    def test_duplicate_surface_id_is_rejected(self) -> None:
        self.catalog["surfaces"][1]["id"] = self.catalog["surfaces"][0]["id"]
        self.assert_fails("duplicate surface id")

    def test_duplicate_surface_path_is_rejected(self) -> None:
        self.catalog["surfaces"][1]["path"] = self.catalog["surfaces"][0]["path"]
        self.assert_fails("duplicate surface path")

    def test_wrong_kind_is_rejected(self) -> None:
        self.catalog["surfaces"][0]["kind"] = "wrong"
        self.assert_fails("requires kind")

    def test_wrong_denominator_is_rejected(self) -> None:
        self.catalog["surfaces"][0]["release_denominator"] = "none"
        self.assert_fails("requires denominator")

    def test_production_authorization_cannot_be_granted(self) -> None:
        self.catalog["production_authorization"] = "granted"
        self.assert_fails("cannot grant production authorization")

    def test_sentinel_must_stay_inside_surface(self) -> None:
        self.catalog["surfaces"][0]["sentinels"] = ["docs/contract.md"]
        self.assert_fails("outside its surface")

    def test_missing_sentinel_is_rejected(self) -> None:
        self.catalog["surfaces"][0]["sentinels"] = [".cargo/missing.txt"]
        self.assert_fails("repository path is missing")

    def test_empty_canonical_document_is_rejected(self) -> None:
        write(self.root / "docs/empty.md", "")
        self.catalog["surfaces"][0]["canonical_docs"] = ["docs/empty.md"]
        self.assert_fails("repository file is empty")

    def test_symlink_inside_surface_is_rejected(self) -> None:
        target = self.root / ".cargo/real.txt"
        write(target)
        (self.root / ".cargo/link.txt").symlink_to(target.name)
        self.assert_fails("symlink file")

    def test_extra_key_is_rejected(self) -> None:
        self.catalog["surfaces"][0]["extra"] = True
        self.assert_fails("key mismatch")

    def test_unsafe_path_is_rejected(self) -> None:
        self.catalog["surfaces"][0]["path"] = "../escape"
        self.assert_fails("unsafe path segment")

    def test_asset_child_drift_is_rejected(self) -> None:
        write(self.root / "assets/unclassified/file.txt")
        self.assert_fails("asset child surface drift")

    def test_deploy_child_drift_is_rejected(self) -> None:
        write(self.root / "deploy/kubernetes/file.txt")
        self.assert_fails("deploy child surface drift")

    def test_duplicate_json_key_is_rejected(self) -> None:
        target = self.root / CHECKER.DEFAULT_CATALOG
        target.write_text(
            '{"schema":"trnm_world_operational_surface_catalog_v1",'
            '"schema":"duplicate","as_of":"2026-09-10",'
            '"repository":"TrillionniumFoundation/Trillionnium-World",'
            '"production_authorization":"not_granted","surfaces":[]}\n',
            encoding="utf-8",
        )
        with self.assertRaisesRegex(CHECKER.SurfaceCatalogError, "duplicate JSON key"):
            CHECKER._load(target)


if __name__ == "__main__":
    unittest.main(verbosity=2)
