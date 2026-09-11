#!/usr/bin/env python3
"""Negative tests for current document/source conformance."""

from __future__ import annotations

from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def run(root: Path, expect_success: bool) -> None:
    checker = root / "scripts/check-trnm-world-current-conformance.py"
    completed = subprocess.run(
        [sys.executable, str(checker)],
        cwd=root,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        timeout=30,
        check=False,
    )
    if (completed.returncode == 0) != expect_success:
        raise AssertionError(f"unexpected rc={completed.returncode}\n{completed.stdout}")


def copy_and_mutate(relative: str, mutate) -> None:
    with tempfile.TemporaryDirectory(prefix="trnm-current-conformance-") as temporary:
        clone = Path(temporary) / "repo"
        shutil.copytree(
            ROOT,
            clone,
            ignore=shutil.ignore_patterns(".git", "target", "node_modules", "run", "assets"),
        )
        target = clone / relative
        text = target.read_text(encoding="utf-8")
        mutated = mutate(text)
        if mutated == text:
            raise AssertionError(f"negative fixture did not mutate {relative}")
        target.write_text(mutated, encoding="utf-8")
        run(clone, False)


def main() -> None:
    run(ROOT, True)
    copy_and_mutate(
        "docs/development/trnm-world-module-decomposition-v1.md",
        lambda text: text.replace(
            "The former semantic `trnm-game-server/build.rs` and `src/lib.rs.in` generation authority is retired and removed from the current candidate.",
            "trnm-game-server still contains a build script and source template.",
        ),
    )
    copy_and_mutate(
        "trillionnium/crates/platform/README.md",
        lambda text: text.replace(
            "Release denominator: **none**", "Release denominator: **game-product**"
        ).replace("release_denominator=none", "release_denominator=game-product"),
    )
    copy_and_mutate(
        "web4-frontend/README.md",
        lambda text: text.replace("World game-product release denominator: **none**", "World game-product release denominator: **game-product**"),
    )
    copy_and_mutate(
        "contracts/README.md",
        lambda text: text + "\nproduction_authorization=granted\n",
    )
    copy_and_mutate(
        "PROJECT_BOUNDARY.md",
        lambda text: text.replace("Public online remains NO-GO", "Public online is enabled"),
    )
    print("TRNM_WORLD_CURRENT_CONFORMANCE_TESTS=PASS")


if __name__ == "__main__":
    main()
