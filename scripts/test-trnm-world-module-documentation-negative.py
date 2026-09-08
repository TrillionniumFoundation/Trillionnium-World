#!/usr/bin/env python3
"""Regression tests for the World module documentation contract."""
from __future__ import annotations

import argparse
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location(
    "world_module_docs", Path(__file__).with_name("check-trnm-world-module-documentation.py")
)
assert SPEC and SPEC.loader
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)

SUMMARY_BODY = (
    "This section describes the module-owned state, typed interfaces, explicit "
    "failure handling and the required deterministic regression evidence."
)
DESIGN_BODY = (
    "This detailed section records the current implementation boundary, the "
    "correctness invariant, the responsible caller and callee, bounded failure "
    "behavior, verification evidence and the change-control consequence. It is "
    "long enough to reject title-only or placeholder documentation contracts."
)


def summary(name: str, runtime: bool = False) -> str:
    headings = [options[0] for options in M.SUMMARY_REQUIRED]
    if runtime:
        headings[2] = "Runtime composition"
    return f"# {name}\n\n" + "\n\n".join(f"## {heading}\n\n{SUMMARY_BODY}" for heading in headings) + "\n"


def design(name: str) -> str:
    header = (
        "---\nstatus: current-candidate\nowner: fixture\n"
        "last_reviewed: 2026-09-08\nreview_due: 2026-10-08\n---\n\n"
        f"# {name} technical design\n\n"
    )
    return header + "\n\n".join(f"## {heading}\n\n{DESIGN_BODY}" for heading in M.DESIGN_REQUIRED) + "\n"


class DocumentationTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory(prefix="world-doc-v2-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.names = ["trnm-one", "trnm-two"]
        self.make_fixture()

    def write(self, relative: str, content: str) -> None:
        path = self.root / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")

    def make_fixture(self) -> None:
        self.write(
            "trillionnium/Cargo.toml",
            "[workspace]\nmembers = [" + ",".join(f'\"crates/{name}\"' for name in self.names) + "]\n",
        )
        modules = []
        for name in self.names:
            self.write(f"trillionnium/crates/{name}/Cargo.toml", f'[package]\nname = "{name}"\nversion = "0.1.0"\n')
            self.write(f"trillionnium/crates/{name}/README.md", summary(name))
            self.write(f"docs/modules/{name}.md", design(name))
            modules.append(
                {
                    "package": name,
                    "owner": "fixture",
                    "summary_document": f"trillionnium/crates/{name}/README.md",
                    "technical_design": f"docs/modules/{name}.md",
                    "release_authority": False,
                }
            )
        self.write(
            M.DESIGN_MANIFEST,
            json.dumps({"schema": "trnm_world_module_contracts_v2", "modules": modules}, indent=2) + "\n",
        )
        rows = "".join(f"| `{name}` | World | [`{name}.md`]({name}.md) |\n" for name in self.names)
        self.write(M.DESIGN_INDEX, "# Modules\n\n| Module | Owner | Design |\n|---|---|---|\n" + rows)
        self.write(M.MATRIX, "| Module | Owner |\n|---|---|\n" + "".join(f"| `{name}` | World |\n" for name in self.names))
        self.write("README.md", "# Fixture\n")
        self.write("OPERATIONS.md", "# Operations\n\nRepository-relative commands only.\n")
        self.write("RELEASE_READINESS.md", "# Readiness\n\nProduction authorization is not granted.\n")
        self.write("docs/README.md", "# Documentation\n\nCurrent documentation index.\n")
        self.write("docs/adr/0003-world-authority-service-boundary.md", "# Boundary\n\nNakama owns canonical online authority.\n")
        self.write("docs/development/TRILLIONNIUM_WORLD_CLOSURE_EXECUTION_BOARD_V5.md", "# Board\n\nExternal evidence remains separate.\n")

    @property
    def first_summary(self) -> Path:
        return self.root / "trillionnium/crates/trnm-one/README.md"

    @property
    def first_design(self) -> Path:
        return self.root / "docs/modules/trnm-one.md"

    def must_fail(self) -> None:
        with self.assertRaises((M.DocumentationFailure, OSError, ValueError)):
            M.validate(self.root)

    def test_valid_fixture(self) -> None:
        self.assertEqual(M.validate(self.root), sorted(self.names))

    def test_runtime_composition_alternative(self) -> None:
        self.first_summary.write_text(summary("trnm-one", runtime=True), encoding="utf-8")
        self.assertEqual(len(M.validate(self.root)), 2)

    def test_missing_summary(self) -> None:
        self.first_summary.unlink()
        self.must_fail()

    def test_missing_design(self) -> None:
        self.first_design.unlink()
        self.must_fail()

    def test_design_section_required(self) -> None:
        text = self.first_design.read_text(encoding="utf-8")
        self.first_design.write_text(text.replace("## Concurrency and cancellation", "## Scheduling"), encoding="utf-8")
        self.must_fail()

    def test_design_section_must_be_substantive(self) -> None:
        text = self.first_design.read_text(encoding="utf-8")
        self.first_design.write_text(text.replace(DESIGN_BODY, "TODO", 1), encoding="utf-8")
        self.must_fail()

    def test_duplicate_manifest_package(self) -> None:
        path = self.root / M.DESIGN_MANIFEST
        data = json.loads(path.read_text(encoding="utf-8"))
        data["modules"].append(dict(data["modules"][0]))
        path.write_text(json.dumps(data), encoding="utf-8")
        self.must_fail()

    def test_manifest_package_set_must_match_workspace(self) -> None:
        path = self.root / M.DESIGN_MANIFEST
        data = json.loads(path.read_text(encoding="utf-8"))
        data["modules"].pop()
        path.write_text(json.dumps(data), encoding="utf-8")
        self.must_fail()

    def test_wrong_summary_path(self) -> None:
        path = self.root / M.DESIGN_MANIFEST
        data = json.loads(path.read_text(encoding="utf-8"))
        data["modules"][0]["summary_document"] = "README.md"
        path.write_text(json.dumps(data), encoding="utf-8")
        self.must_fail()

    def test_document_cannot_grant_release_authority(self) -> None:
        path = self.root / M.DESIGN_MANIFEST
        data = json.loads(path.read_text(encoding="utf-8"))
        data["modules"][0]["release_authority"] = True
        path.write_text(json.dumps(data), encoding="utf-8")
        self.must_fail()

    def test_broken_design_index_link(self) -> None:
        index = self.root / M.DESIGN_INDEX
        index.write_text(index.read_text(encoding="utf-8").replace("trnm-one.md", "missing.md"), encoding="utf-8")
        self.must_fail()

    def test_personal_path_rejected(self) -> None:
        self.first_design.write_text(self.first_design.read_text(encoding="utf-8") + "\n/Users/alice/project\n", encoding="utf-8")
        self.must_fail()

    def test_overclaim_rejected(self) -> None:
        self.first_design.write_text(self.first_design.read_text(encoding="utf-8") + "\nPublic online enabled.\n", encoding="utf-8")
        self.must_fail()

    def test_semantic_build_script_rejected(self) -> None:
        self.write("trillionnium/crates/trnm-game-server/build.rs", "fn main() {}\n")
        self.must_fail()

    def test_stale_docs_assertion_rejected(self) -> None:
        self.write("docs/README.md", "# Docs\n\nbuild.rs still transforms into the compiled library\n")
        self.must_fail()

    def test_commented_heading_does_not_count(self) -> None:
        text = self.first_design.read_text(encoding="utf-8")
        self.first_design.write_text(text.replace("## Dependency direction", "<!-- ## Dependency direction -->"), encoding="utf-8")
        self.must_fail()

    def test_fenced_heading_does_not_count(self) -> None:
        text = self.first_design.read_text(encoding="utf-8")
        self.first_design.write_text(text.replace("## Dependency direction", "```markdown\n## Dependency direction\n```"), encoding="utf-8")
        self.must_fail()

    def test_symlink_design_rejected(self) -> None:
        outside = self.root / "outside.md"
        outside.write_text(self.first_design.read_text(encoding="utf-8"), encoding="utf-8")
        self.first_design.unlink()
        self.first_design.symlink_to(outside)
        self.must_fail()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixtures-only", action="store_true")
    args = parser.parse_args()
    if not args.fixtures_only:
        M.validate(Path(__file__).resolve().parents[1])
    result = unittest.TextTestRunner(verbosity=1).run(
        unittest.defaultTestLoader.loadTestsFromTestCase(DocumentationTests)
    )
    if not result.wasSuccessful():
        raise SystemExit(1)
    print("TRNM World module documentation negative fixtures: PASS (v2 contract)")
