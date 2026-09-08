#!/usr/bin/env python3
"""Negative regressions for the detailed World documentation and catalogue gate."""

from __future__ import annotations

import datetime as dt
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location(
    "world_detailed_docs",
    Path(__file__).with_name("check-trnm-world-detailed-documentation.py"),
)
assert SPEC and SPEC.loader
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)

GENERIC = (
    "This section gives a concrete, reviewable contract for owned state, typed interfaces, "
    "authority boundaries, deterministic behavior, failure handling, resource ceilings, "
    "security controls, migration effects, and executable evidence. It intentionally exceeds "
    "the shallow-section threshold and does not claim implementation or release conformance. "
)
ALL_TERMS = (
    "idempotency receipt CEX custody stable identifiers quest provenance content revision "
    "Town BattlePending PostBattlePending state preserving schema revision 128 KiB "
    "160 UTF-8 bytes 256 subjects Nakama 10 ticks per second 65,536 state hash "
    "full snapshot reconnect canonical online world_legacy_local_alpha capture "
    "lookup-before-submit PITR Bevy untrusted command journal human evidence"
)


def design(name: str, due: str = "2026-10-08") -> str:
    sections = []
    for heading in M.REQUIRED_SECTIONS:
        body = GENERIC + ALL_TERMS
        if heading == "Test and evidence traceability":
            body += " Source: trillionnium/crates/example/src/lib.rs; docs/protocol/example.md; tests and scripts/example.py."
        if heading == "Change checklist and open work":
            body += " Open work remains explicit and cannot be promoted by documentation alone."
        sections.append(f"## {heading}\n\n{body}")
    return (
        "---\n"
        "status: current-candidate\n"
        "owner: test-owner\n"
        f"module: {name}\n"
        "last_reviewed: 2026-09-08\n"
        f"review_due: {due}\n"
        "implementation_conformance: not-implied\n"
        "---\n\n"
        f"# {name} detailed design\n\n"
        + "\n\n".join(sections)
        + "\n"
    )


class DetailedDocumentationTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory(prefix="world-detailed-docs-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.names = list(M.MODULE_TERMS)
        self.make_fixture()

    def write(self, relative: str, content: str) -> None:
        target = self.root / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content, encoding="utf-8")

    def write_bytes(self, relative: str, content: bytes) -> None:
        target = self.root / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(content)

    def make_fixture(self) -> None:
        members = [f"crates/{name}" for name in self.names]
        self.write(
            "trillionnium/Cargo.toml",
            "[workspace]\nmembers = [\n"
            + "".join(f'  "{member}",\n' for member in members)
            + "]\n",
        )
        for name in self.names:
            self.write(
                f"trillionnium/crates/{name}/Cargo.toml",
                f'[package]\nname = "{name}"\nversion = "0.1.0"\n',
            )
            self.write(
                f"trillionnium/crates/{name}/README.md",
                f"# {name}\n\n## Detailed design\n\n"
                f"[design](../../../docs/modules/{name}-design.md)\n",
            )
            self.write(f"docs/modules/{name}-design.md", design(name))

        self.write(
            M.MODULE_INDEX,
            "# modules\n\n"
            + "\n".join(
                f"- `{name}`: [`{name}-design.md`]({name}-design.md)" for name in self.names
            ),
        )
        self.write(
            M.MATRIX,
            "| Crate | Owner |\n|---|---|\n"
            + "".join(f"| `{name}` | test |\n" for name in self.names),
        )

        baseline = {
            "AGENTS.md": "# agents\n",
            "PROJECT_BOUNDARY.md": "# boundary\n",
            "PROJECT_BOUNDARY.json": "{}\n",
            "CURRENT_PLAN.md": "# plan\n",
            "README.md": "# repository\n",
            "OPERATIONS.md": "# operations\n",
            "docs/README.md": "# docs\n",
            "docs/adr/0001-realtime-authority-and-match-evidence-ownership.md": "# adr1\n",
            "docs/adr/0002-transaction-free-external-settlement.md": "# adr2\n",
            "docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md": "# adr3\n",
            "docs/architecture/trnm-world-system-context-v1.md": "# context\n",
            "docs/database/trnm-world-postgres-contract-v1.md": "# database\n",
            "docs/security/trnm-world-threat-model-v1.md": "# threat\n",
            "docs/development/trnm-world-testing-strategy-v2.md": "# tests\n",
            "docs/release/trnm-world-release-gate-matrix-v2.md": "# release\n",
            "docs/status/world-plan-v4-execution-truth-2026-09-02.json": "{}\n",
            "docs/status/CURRENT.md": "# status\n",
        }
        for path, content in baseline.items():
            self.write(path, content)

        paths = set(M.REQUIRED_CATALOG_PATHS)
        paths.update(
            {
                M.CATALOG,
                M.MODULE_INDEX,
                M.MATRIX,
                "docs/status/world-plan-v4-execution-truth-2026-09-02.json",
                "docs/status/CURRENT.md",
            }
        )
        paths.update(f"docs/modules/{name}-design.md" for name in self.names)
        documents = []
        for path in sorted(paths):
            if not (self.root / path).exists():
                self.write(path, "# fixture\n")
            documents.append(
                {
                    "path": path,
                    "class": "module-design"
                    if path.startswith("docs/modules/trnm-") and path.endswith("-design.md")
                    else "fixture",
                    "owner": "test-owner",
                    "status": "current-candidate",
                    "review_due": "2026-10-08",
                }
            )
        catalog = {
            "schema": "trnm_world_document_catalog_v1",
            "as_of": "2026-09-08",
            "truth_order": [
                "PROJECT_BOUNDARY.md",
                "CURRENT_PLAN.md",
                "docs/catalog.json",
                "docs/status/world-plan-v4-execution-truth-2026-09-02.json",
                "docs/status/CURRENT.md",
            ],
            "documents": documents,
        }
        self.write(M.CATALOG, json.dumps(catalog, indent=2) + "\n")

    def validate(self) -> list[str]:
        return M.validate(self.root, dt.date(2026, 9, 8))

    def expect_failure(self) -> None:
        with self.assertRaises((M.DocumentationFailure, OSError, ValueError)):
            self.validate()

    def test_valid_fixture(self) -> None:
        self.assertEqual(self.validate(), sorted(self.names))

    def test_missing_design(self) -> None:
        (self.root / "docs/modules/trnm-rpg-core-design.md").unlink()
        self.expect_failure()

    def test_shallow_section(self) -> None:
        path = self.root / "docs/modules/trnm-rpg-core-design.md"
        path.write_text(
            path.read_text().replace(GENERIC + ALL_TERMS, "too short", 1),
            encoding="utf-8",
        )
        self.expect_failure()

    def test_stale_review(self) -> None:
        path = self.root / "docs/modules/trnm-rpg-core-design.md"
        path.write_text(design("trnm-rpg-core", due="2026-09-07"), encoding="utf-8")
        self.expect_failure()

    def test_missing_readme_link(self) -> None:
        path = self.root / "trillionnium/crates/trnm-rpg-core/README.md"
        path.write_text("# trnm-rpg-core\n", encoding="utf-8")
        self.expect_failure()

    def test_unregistered_design(self) -> None:
        catalog = json.loads((self.root / M.CATALOG).read_text())
        catalog["documents"] = [
            item
            for item in catalog["documents"]
            if item["path"] != "docs/modules/trnm-rpg-core-design.md"
        ]
        (self.root / M.CATALOG).write_text(json.dumps(catalog), encoding="utf-8")
        self.expect_failure()

    def test_forbidden_overclaim(self) -> None:
        path = self.root / "docs/modules/trnm-rpg-core-design.md"
        path.write_text(path.read_text() + "\nWorld owns wallet custody.\n", encoding="utf-8")
        self.expect_failure()

    def test_missing_traceability(self) -> None:
        path = self.root / "docs/modules/trnm-rpg-core-design.md"
        text = path.read_text()
        text = text.replace(
            " Source: trillionnium/crates/example/src/lib.rs; docs/protocol/example.md; tests and scripts/example.py.",
            "",
        )
        path.write_text(text, encoding="utf-8")
        self.expect_failure()

    def test_duplicate_section(self) -> None:
        path = self.root / "docs/modules/trnm-rpg-core-design.md"
        path.write_text(
            path.read_text() + "\n## Scope and authority\n\n" + GENERIC + ALL_TERMS + "\n",
            encoding="utf-8",
        )
        self.expect_failure()

    def test_unsafe_catalog_path(self) -> None:
        catalog = json.loads((self.root / M.CATALOG).read_text())
        catalog["documents"].append(
            {
                "path": "../outside.md",
                "class": "fixture",
                "owner": "test",
                "status": "current-candidate",
                "review_due": "2026-10-08",
            }
        )
        (self.root / M.CATALOG).write_text(json.dumps(catalog), encoding="utf-8")
        self.expect_failure()

    def test_duplicate_top_level_catalog_key(self) -> None:
        path = self.root / M.CATALOG
        raw = path.read_text(encoding="utf-8")
        raw = raw.replace(
            '"schema": "trnm_world_document_catalog_v1",',
            '"schema": "shadow",\n  "schema": "trnm_world_document_catalog_v1",',
            1,
        )
        path.write_text(raw, encoding="utf-8")
        self.expect_failure()

    def test_duplicate_nested_catalog_key(self) -> None:
        path = self.root / M.CATALOG
        raw = path.read_text(encoding="utf-8")
        raw = raw.replace(
            '"owner": "test-owner",',
            '"owner": "shadow-owner",\n      "owner": "test-owner",',
            1,
        )
        path.write_text(raw, encoding="utf-8")
        self.expect_failure()

    def test_non_finite_catalog_tokens(self) -> None:
        baseline = (self.root / M.CATALOG).read_text(encoding="utf-8")
        for token in ("NaN", "Infinity", "-Infinity"):
            with self.subTest(token=token):
                self.make_fixture()
                path = self.root / M.CATALOG
                raw = path.read_text(encoding="utf-8")
                raw = raw.replace(
                    '"as_of": "2026-09-08",',
                    f'"probe": {token},\n  "as_of": "2026-09-08",',
                    1,
                )
                path.write_text(raw, encoding="utf-8")
                self.expect_failure()
        (self.root / M.CATALOG).write_text(baseline, encoding="utf-8")

    def test_catalog_depth_budget(self) -> None:
        path = self.root / M.CATALOG
        raw = path.read_text(encoding="utf-8")
        nested = "[" * (M.MAX_JSON_DEPTH + 1) + "0" + "]" * (M.MAX_JSON_DEPTH + 1)
        path.write_text('{"probe":' + nested + "," + raw[1:], encoding="utf-8")
        self.expect_failure()

    def test_catalog_size_budget(self) -> None:
        self.write_bytes(M.CATALOG, b" " * (M.MAX_CATALOG_BYTES + 1))
        self.expect_failure()

    def test_catalog_invalid_utf8(self) -> None:
        self.write_bytes(
            M.CATALOG,
            b'{"schema":"trnm_world_document_catalog_v1","bad":"\xff"}',
        )
        self.expect_failure()

    def test_extra_workspace_member_requires_classification(self) -> None:
        path = self.root / "trillionnium/Cargo.toml"
        path.write_text(path.read_text().replace("]\n", '  "crates/trnm-extra",\n]\n'))
        self.write(
            "trillionnium/crates/trnm-extra/Cargo.toml",
            '[package]\nname = "trnm-extra"\nversion = "0.1.0"\n',
        )
        self.expect_failure()


if __name__ == "__main__":
    result = unittest.TextTestRunner(verbosity=1).run(
        unittest.defaultTestLoader.loadTestsFromTestCase(DetailedDocumentationTests)
    )
    if not result.wasSuccessful():
        raise SystemExit(1)
    print("TRNM World detailed documentation negative fixtures: PASS")
