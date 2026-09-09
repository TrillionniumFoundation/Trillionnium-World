#!/usr/bin/env python3
"""Exercise the contract-module documentation checker through hostile copies."""

from __future__ import annotations

from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts/check-trnm-world-contract-module-documentation.py"
TARGET = ROOT / "docs/modules/contracts/audit-events-design.md"


def run(root: Path, expect_success: bool) -> None:
    checker = root / "scripts/check-trnm-world-contract-module-documentation.py"
    completed = subprocess.run(
        [sys.executable, str(checker), "--as-of", "2026-09-09"],
        cwd=root,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        timeout=30,
        check=False,
    )
    if (completed.returncode == 0) != expect_success:
        raise AssertionError(
            f"unexpected checker result rc={completed.returncode}\n{completed.stdout}"
        )


def mutate_copy(mutator) -> None:
    with tempfile.TemporaryDirectory(prefix="trnm-contract-doc-") as temporary:
        clone = Path(temporary) / "repo"
        shutil.copytree(
            ROOT,
            clone,
            ignore=shutil.ignore_patterns(".git", "target", "node_modules", "run"),
        )
        target = clone / TARGET.relative_to(ROOT)
        text = target.read_text(encoding="utf-8")
        target.write_text(mutator(text), encoding="utf-8")
        run(clone, expect_success=False)


def main() -> None:
    run(ROOT, expect_success=True)
    mutate_copy(lambda text: text.replace("## State model and invariants", "## State"))
    mutate_copy(lambda text: text.replace("status: current-candidate", "status: current"))
    mutate_copy(lambda text: text.replace("owner: trillionnium-world-contracts", "owner: nobody"))
    mutate_copy(lambda text: text.replace("implementation_conformance: not-implied", "implementation_conformance: complete"))
    mutate_copy(lambda text: text.replace("review_due: 2026-10-08", "review_due: 2026-09-01"))
    mutate_copy(lambda text: text.replace("contracts/audit-events/src/", "missing/source/"))
    mutate_copy(lambda text: text + "\nproduction_authorization: granted\n")
    print("TRNM_WORLD_CONTRACT_MODULE_DOCUMENTATION_TESTS=PASS")


if __name__ == "__main__":
    main()
