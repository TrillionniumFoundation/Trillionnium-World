#!/usr/bin/env python3
"""Negative regressions for the isolated World-domain authority documentation gate."""

from __future__ import annotations

import datetime as dt
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location(
    "world_authority_docs",
    Path(__file__).with_name("check-trnm-world-authority-documentation.py"),
)
assert SPEC and SPEC.loader
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)

GENERIC = (
    "This section defines a concrete review contract for typed interfaces, owned state, "
    "authority boundaries, deterministic behavior, concurrency, durability, retries, "
    "stable failures, migration, resource ceilings, security and executable evidence. "
)
ALL_TERMS = (
    "WorldState Nakama CEX canonical WorldCommand expected revision state rebuildable "
    "source cache authority fixture-only ODbL sensitive public-network escaping data-* "
    "XSS server WorldRepository WorldLedgerAdapter lookup-before-submit file-backed "
    "PostgreSQL no-dual-writer not production"
)


def detailed_design(name: str, due: str = "2026-10-08") -> str:
    bodies = []
    for heading in M.DESIGN_SECTIONS:
        body = GENERIC + ALL_TERMS
        if heading == "Test and evidence traceability":
            body += (
                " Source is trillionnium/crates/world-authority/example/src/lib.rs; "
                "normative material is docs/adr/example.md; scripts/check-example.sh "
                "and tests cover the claim."
            )
        bodies.append(f"## {heading}\n\n{body}")
    return (
        "---\n"
        "status: current-candidate\n"
        "owner: fixture-owner\n"
        f"module: {name}\n"
        "last_reviewed: 2026-09-08\n"
        f"review_due: {due}\n"
        "implementation_conformance: not-implied\n"
        "---\n\n"
        f"# {name} detailed design\n\n"
        + "\n\n".join(bodies)
        + "\n"
    )


def module_readme(name: str, link: bool = True) -> str:
    sections = []
    for heading in M.README_SECTIONS:
        body = GENERIC + ALL_TERMS
        if heading == "Detailed design":
            body = (
                f"See ../../../../docs/modules/world-authority/{name}-design.md " + GENERIC
                if link
                else GENERIC
            )
        sections.append(f"## {heading}\n\n{body}")
    return f"# {name}\n\n" + "\n\n".join(sections) + "\n"


class AuthorityDocumentationTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory(prefix="world-authority-docs-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.names = sorted(M.EXPECTED_MODULES)
        self.make_fixture()

    def write(self, relative: str, content: str) -> None:
        path = self.root / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")

    def make_fixture(self) -> None:
        self.write(
            M.WORKSPACE,
            "[workspace]\nmembers = [\n"
            + "".join(f'  "{name}",\n' for name in self.names)
            + "]\n\n"
            "[workspace.metadata.trnm]\n"
            'cex_role = "authenticated-ingress-and-projection-consumer-only"\n'
            'production_authorization = "not_granted"\n',
        )
        workspace_index = ["# workspace"]
        design_index = ["# designs"]
        documents = [
            {"path": M.WORKSPACE_README, "class": "world-authority-workspace", "owner": "fixture", "status": "current-candidate", "review_due": "2026-10-08"},
            {"path": M.INDEX, "class": "world-authority-module-index", "owner": "fixture", "status": "current-candidate", "review_due": "2026-10-08"},
            {"path": "docs/architecture/cex-world-authority-cutover-v1.md", "class": "world-authority-architecture", "owner": "fixture", "status": "current-candidate", "review_due": "2026-10-08"},
            {"path": "docs/contracts/trillionnium-world-authority-cutover-v1.json", "class": "world-authority-contract", "owner": "fixture", "status": "current-candidate", "review_due": "2026-10-08"},
            {"path": "docs/contracts/trillionnium-world-authority-provenance-v1.json", "class": "world-authority-contract", "owner": "fixture", "status": "current-candidate", "review_due": "2026-10-08"},
        ]
        self.write("docs/architecture/cex-world-authority-cutover-v1.md", "# architecture\n")
        self.write("docs/contracts/trillionnium-world-authority-cutover-v1.json", "{}\n")
        self.write("docs/contracts/trillionnium-world-authority-provenance-v1.json", "{}\n")
        self.write(
            M.ADR,
            "# ADR\n\nNakama remains the sole target canonical online owner. "
            "CEX remains the sole wallet/ledger owner. "
            "Each aggregate has one admitted writer epoch.\n",
        )
        for name in self.names:
            readme_path = f"trillionnium/crates/world-authority/{name}/README.md"
            design_path = f"docs/modules/world-authority/{name}-design.md"
            self.write(
                f"trillionnium/crates/world-authority/{name}/Cargo.toml",
                f'[package]\nname = "{name}"\nversion = "0.1.0"\n',
            )
            self.write(readme_path, module_readme(name))
            self.write(design_path, detailed_design(name))
            workspace_index.append(f"- {name}")
            design_index.append(f"- {name}: {name}-design.md")
            documents.extend([
                {"path": readme_path, "class": "world-authority-module-contract", "owner": "fixture", "status": "current-candidate", "review_due": "2026-10-08"},
                {"path": design_path, "class": "world-authority-module-design", "owner": "fixture", "status": "current-candidate", "review_due": "2026-10-08"},
            ])
        self.write(M.WORKSPACE_README, "\n".join(workspace_index) + "\n")
        self.write(M.INDEX, "\n".join(design_index) + "\n")
        self.write(M.CATALOG, json.dumps({"schema": "trnm_world_document_catalog_v1", "as_of": "2026-09-08", "truth_order": [], "documents": documents}) + "\n")

    def validate(self) -> list[str]:
        return M.validate(self.root, dt.date(2026, 9, 8))

    def expect_failure(self) -> None:
        with self.assertRaises((M.DocumentationFailure, M.BASE.DocumentationFailure, OSError, ValueError)):
            self.validate()

    def test_valid_fixture(self) -> None:
        self.assertEqual(self.validate(), self.names)

    def test_missing_readme(self) -> None:
        (self.root / f"trillionnium/crates/world-authority/{self.names[0]}/README.md").unlink()
        self.expect_failure()

    def test_missing_design_link(self) -> None:
        name = self.names[0]
        self.write(f"trillionnium/crates/world-authority/{name}/README.md", module_readme(name, link=False))
        self.expect_failure()

    def test_shallow_design_section(self) -> None:
        name = self.names[0]
        path = self.root / f"docs/modules/world-authority/{name}-design.md"
        path.write_text(path.read_text().replace(GENERIC + ALL_TERMS, "too short", 1))
        self.expect_failure()

    def test_stale_review(self) -> None:
        name = self.names[0]
        self.write(f"docs/modules/world-authority/{name}-design.md", detailed_design(name, due="2026-09-07"))
        self.expect_failure()

    def test_catalog_class_drift(self) -> None:
        catalog = json.loads((self.root / M.CATALOG).read_text())
        for item in catalog["documents"]:
            if item["path"].endswith("trnm-world-domain-design.md"):
                item["class"] = "module-design"
        self.write(M.CATALOG, json.dumps(catalog))
        self.expect_failure()

    def test_workspace_module_drift(self) -> None:
        path = self.root / M.WORKSPACE
        path.write_text(path.read_text().replace("]\n\n", '  "trnm-world-extra",\n]\n\n'))
        self.write("trillionnium/crates/world-authority/trnm-world-extra/Cargo.toml", '[package]\nname = "trnm-world-extra"\nversion = "0.1.0"\n')
        self.expect_failure()

    def test_production_authorization_drift(self) -> None:
        path = self.root / M.WORKSPACE
        path.write_text(path.read_text().replace("not_granted", "granted"))
        self.expect_failure()

    def test_authority_overclaim(self) -> None:
        name = self.names[0]
        path = self.root / f"docs/modules/world-authority/{name}-design.md"
        path.write_text(path.read_text() + "\nWorld owns canonical online order.\n")
        self.expect_failure()

    def test_missing_binding_adr_statement(self) -> None:
        self.write(M.ADR, "# ADR\n")
        self.expect_failure()


if __name__ == "__main__":
    result = unittest.TextTestRunner(verbosity=1).run(unittest.defaultTestLoader.loadTestsFromTestCase(AuthorityDocumentationTests))
    if not result.wasSuccessful():
        raise SystemExit(1)
    print("TRNM World authority documentation negative fixtures: PASS")
