#!/usr/bin/env python3
"""Negative tests for current document/source conformance."""

from __future__ import annotations

import json
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


def copy_and_apply(mutate) -> None:
    with tempfile.TemporaryDirectory(prefix="trnm-current-conformance-") as temporary:
        clone = Path(temporary) / "repo"
        shutil.copytree(
            ROOT,
            clone,
            ignore=shutil.ignore_patterns(".git", "target", "node_modules", "run", "assets"),
        )
        mutate(clone)
        run(clone, False)


def copy_and_mutate(relative: str, mutate) -> None:
    def apply(clone: Path) -> None:
        target = clone / relative
        text = target.read_text(encoding="utf-8")
        mutated = mutate(text)
        if mutated == text:
            raise AssertionError(f"negative fixture did not mutate {relative}")
        target.write_text(mutated, encoding="utf-8")

    copy_and_apply(apply)


def grant_component_catalog_authorization(clone: Path) -> None:
    target = clone / "docs/component-catalog.json"
    value = json.loads(target.read_text(encoding="utf-8"))
    if value.get("production_authorization") != "not_granted":
        raise AssertionError("component catalog fixture baseline drift")
    value["production_authorization"] = "granted"
    target.write_text(
        json.dumps(value, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )


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
        lambda text: text.replace(
            "World game-product release denominator: **none**",
            "World game-product release denominator: **game-product**",
        ).replace("world_release_denominator=none", "world_release_denominator=game-product"),
    )
    copy_and_apply(grant_component_catalog_authorization)
    copy_and_mutate(
        "PROJECT_BOUNDARY.md",
        lambda text: text.replace(
            "Public online remains NO-GO, public player markets remain disabled and production authorization remains `not_granted` until every dependency in the release matrix is independently green.",
            "Public online is enabled.",
        ),
    )
    print("TRNM_WORLD_CURRENT_CONFORMANCE_TESTS=PASS")


if __name__ == "__main__":
    main()
