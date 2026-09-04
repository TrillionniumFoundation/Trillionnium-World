#!/usr/bin/env python3
"""Fail closed when an active World crate lacks its module contract."""

from __future__ import annotations

import pathlib
import sys
import tomllib

ROOT = pathlib.Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else pathlib.Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "trillionnium/Cargo.toml"
EXPECTED = {
    "crates/trnm-economy-protocol",
    "crates/trnm-rpg-core",
    "crates/trnm-campaign-core",
    "crates/trnm-rts-protocol",
    "crates/trnm-rts-sim",
    "crates/trnm-online-protocol",
    "crates/trnm-game-server",
    "crates/trnm-first-contact",
}
REQUIRED_HEADINGS = (
    "## Purpose",
    "## Authority and non-goals",
    "## State and invariants",
    "## Dependencies and boundaries",
    "## Failure and recovery",
    "## Testing and evidence",
    "## Compatibility and change control",
)
FORBIDDEN_OVERCLAIMS = (
    "production authorization granted",
    "public online enabled",
    "public player market enabled",
    "world owns wallet custody",
    "world owns chain finality",
    "world signs canonical matchcompletedv1",
)


def fail(message: str) -> None:
    raise SystemExit(f"TRNM World module documentation: FAIL: {message}")


if not MANIFEST.is_file():
    fail("missing trillionnium/Cargo.toml")
try:
    value = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))
except (OSError, tomllib.TOMLDecodeError) as error:
    fail(f"cannot parse workspace manifest: {error}")

members = value.get("workspace", {}).get("members")
if not isinstance(members, list) or set(members) != EXPECTED:
    fail(f"active member set drifted: {members!r}")

for member in sorted(EXPECTED):
    path = ROOT / "trillionnium" / member / "README.md"
    if not path.is_file():
        fail(f"missing module contract: {path.relative_to(ROOT)}")
    text = path.read_text(encoding="utf-8")
    if len(text.strip()) < 1000:
        fail(f"module contract is too small: {path.relative_to(ROOT)}")
    for heading in REQUIRED_HEADINGS:
        if heading not in text:
            fail(f"{path.relative_to(ROOT)} missing {heading}")
    lowered = " ".join(text.lower().split())
    for marker in FORBIDDEN_OVERCLAIMS:
        if marker in lowered:
            fail(f"{path.relative_to(ROOT)} contains forbidden overclaim {marker!r}")

matrix = ROOT / "docs/development/trnm-world-module-documentation-matrix-v1.md"
if not matrix.is_file() or not matrix.read_text(encoding="utf-8").strip():
    fail("module documentation matrix is missing or empty")
for member in EXPECTED:
    if member.removeprefix("crates/") not in matrix.read_text(encoding="utf-8"):
        fail(f"module matrix omits {member}")

print("TRNM World active module documentation: PASS (8/8)")
