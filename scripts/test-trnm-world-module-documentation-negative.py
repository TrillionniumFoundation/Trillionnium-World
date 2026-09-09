#!/usr/bin/env python3
"""Executable structure-gate regressions, including false-positive headings."""
from __future__ import annotations
import argparse
import importlib.util
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location("world_module_docs", Path(__file__).with_name("check-trnm-world-module-documentation.py"))
assert SPEC and SPEC.loader
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)
BODY = "This fixture describes owned state, typed interfaces, explicit failure handling and the required regression evidence."


def document(name: str, runtime: bool = False) -> str:
    headings = [options[0] for options in M.REQUIRED]
    if runtime:
        headings[2] = "Runtime composition"
    return f"# {name}\n\n" + "\n\n".join(f"## {h}\n\n{BODY}" for h in headings) + "\n"


class DocumentationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="world-doc-unit-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.names = ["trnm-one", "trnm-two"]
        self.make_fixture()

    def make_fixture(self):
        (self.root / "trillionnium").mkdir(exist_ok=True)
        (self.root / "trillionnium/Cargo.toml").write_text("[workspace]\nmembers = [" + ",".join(f'"crates/{name}"' for name in self.names) + "]\n")
        for name in self.names:
            directory = self.root / "trillionnium/crates" / name
            directory.mkdir(parents=True, exist_ok=True)
            (directory / "Cargo.toml").write_text(f'[package]\nname = "{name}"\nversion = "0.1.0"\n')
            (directory / "README.md").write_text(document(name))
        matrix = self.root / M.MATRIX
        matrix.parent.mkdir(parents=True, exist_ok=True)
        matrix.write_text("| Module | Owner |\n|---|---|\n" + "".join(f"| `{name}` | World |\n" for name in self.names))

    @property
    def readme(self):
        return self.root / "trillionnium/crates/trnm-one/README.md"

    def check_failure(self):
        with self.assertRaises((M.DocumentationFailure, OSError, ValueError)):
            M.validate(self.root)

    def test_valid_fixture(self):
        self.assertEqual(M.validate(self.root), sorted(self.names))

    def test_member_set_is_derived(self):
        self.names.append("trnm-three")
        self.make_fixture()
        self.assertEqual(len(M.validate(self.root)), 3)

    def test_missing_readme(self):
        self.readme.unlink()
        self.check_failure()

    def test_empty_readme(self):
        self.readme.write_text("")
        self.check_failure()

    def test_public_contract_section_required(self):
        self.readme.write_text(self.readme.read_text().replace("## Public contracts", "## Not a contract"))
        self.check_failure()

    def test_runtime_composition_alternative(self):
        self.readme.write_text(document("trnm-one", runtime=True))
        self.assertEqual(len(M.validate(self.root)), 2)

    def test_empty_section(self):
        self.readme.write_text(self.readme.read_text().replace("## Public contracts\n\n" + BODY, "## Public contracts\n"))
        self.check_failure()

    def test_commented_heading_is_not_coverage(self):
        self.readme.write_text(self.readme.read_text().replace("## Public contracts", "<!-- ## Public contracts -->"))
        self.check_failure()

    def test_fenced_heading_is_not_coverage(self):
        self.readme.write_text(self.readme.read_text().replace("## Public contracts", "```markdown\n## Public contracts\n```"))
        self.check_failure()

    def test_tilde_fence_is_not_coverage(self):
        self.readme.write_text(self.readme.read_text().replace("## Public contracts", "~~~\n## Public contracts\n~~~"))
        self.check_failure()

    def test_duplicate_heading(self):
        self.readme.write_text(self.readme.read_text() + "\n## Public contracts\n\n" + BODY)
        self.check_failure()

    def test_placeholder_section(self):
        self.readme.write_text(self.readme.read_text().replace("## Public contracts\n\n" + BODY, "## Public contracts\n\nTODO"))
        self.check_failure()

    def test_duplicate_member(self):
        path = self.root / "trillionnium/Cargo.toml"
        path.write_text('[workspace]\nmembers = ["crates/trnm-one", "crates/trnm-one"]\n')
        self.check_failure()

    def test_empty_member_set(self):
        (self.root / "trillionnium/Cargo.toml").write_text('[workspace]\nmembers = []\n')
        self.check_failure()

    def test_member_traversal(self):
        (self.root / "trillionnium/Cargo.toml").write_text('[workspace]\nmembers = ["../outside"]\n')
        self.check_failure()

    def test_symlink_readme(self):
        other = self.root / "outside.md"
        other.write_text(self.readme.read_text())
        self.readme.unlink()
        self.readme.symlink_to(other)
        self.check_failure()

    def test_missing_matrix_row(self):
        matrix = self.root / M.MATRIX
        matrix.write_text("| `trnm-one` | World |\n")
        self.check_failure()

    def test_extra_matrix_row(self):
        matrix = self.root / M.MATRIX
        matrix.write_text(matrix.read_text() + "| `not-a-member` | Other |\n")
        self.check_failure()

    def test_duplicate_matrix_row(self):
        matrix = self.root / M.MATRIX
        matrix.write_text(matrix.read_text() + "| `trnm-one` | World |\n")
        self.check_failure()

    def test_bad_package_title(self):
        self.readme.write_text(self.readme.read_text().replace("# trnm-one", "# other"))
        self.check_failure()

    def test_prohibited_authority_assertion(self):
        self.readme.write_text(self.readme.read_text() + "\nWorld owns wallet custody.\n")
        self.check_failure()

    def test_unclosed_fence(self):
        self.readme.write_text(self.readme.read_text() + "\n```\n")
        self.check_failure()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixtures-only", action="store_true", help="Skip the real-checkout baseline; reports unit fixtures only.")
    args = parser.parse_args()
    if not args.fixtures_only:
        M.validate(Path(__file__).resolve().parents[1])
    result = unittest.TextTestRunner(verbosity=1).run(unittest.defaultTestLoader.loadTestsFromTestCase(DocumentationTests))
    if not result.wasSuccessful():
        raise SystemExit(1)
    print("TRNM World module documentation negative fixtures: PASS (structure only)")
