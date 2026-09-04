#!/usr/bin/env python3
"""Prove the active-module documentation gate fails closed."""

from __future__ import annotations

import pathlib
import shutil
import subprocess
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts/check-trnm-world-module-documentation.py"
MEMBERS = (
    "trnm-economy-protocol",
    "trnm-rpg-core",
    "trnm-campaign-core",
    "trnm-rts-protocol",
    "trnm-rts-sim",
    "trnm-online-protocol",
    "trnm-game-server",
    "trnm-first-contact",
)


def run(root: pathlib.Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["python3", str(CHECKER), str(root)],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )


def fixture() -> tempfile.TemporaryDirectory[str]:
    temporary = tempfile.TemporaryDirectory(prefix="trnm-world-module-docs-")
    root = pathlib.Path(temporary.name)
    (root / "trillionnium").mkdir(parents=True)
    shutil.copy2(ROOT / "trillionnium/Cargo.toml", root / "trillionnium/Cargo.toml")
    for member in MEMBERS:
        target = root / "trillionnium/crates" / member / "README.md"
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(ROOT / "trillionnium/crates" / member / "README.md", target)
    matrix = root / "docs/development/trnm-world-module-documentation-matrix-v1.md"
    matrix.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(ROOT / "docs/development/trnm-world-module-documentation-matrix-v1.md", matrix)
    temporary.root = root  # type: ignore[attr-defined]
    return temporary


baseline = run(ROOT)
if baseline.returncode != 0:
    raise SystemExit(f"baseline module documentation check failed:\n{baseline.stdout}")

with fixture() as path:
    root = pathlib.Path(path)
    (root / "trillionnium/crates/trnm-rts-protocol/README.md").unlink()
    result = run(root)
    if result.returncode == 0:
        raise SystemExit("missing module README unexpectedly passed")

with fixture() as path:
    root = pathlib.Path(path)
    target = root / "trillionnium/crates/trnm-campaign-core/README.md"
    target.write_text(
        target.read_text(encoding="utf-8").replace("## Failure and recovery", "## Recovery"),
        encoding="utf-8",
    )
    result = run(root)
    if result.returncode == 0:
        raise SystemExit("missing mandatory section unexpectedly passed")

print("TRNM World module documentation negative fixtures: PASS")
