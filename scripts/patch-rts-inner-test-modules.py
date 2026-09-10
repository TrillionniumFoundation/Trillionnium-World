#!/usr/bin/env python3
"""Replace the three reviewed RTS test-body include! seams with Rust modules."""

from __future__ import annotations

from pathlib import Path
import sys

NAMES = ("rts_tests_01", "rts_tests_02", "rts_tests_03")


def include_block(name: str) -> str:
    return f'''include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/lib_parts/tests/{name}.rs"
));'''


def module_block(name: str) -> str:
    return f'''#[path = "{name}.rs"]
mod {name};'''


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: patch-rts-inner-test-modules.py REPOSITORY_ROOT", file=sys.stderr)
        return 64
    root = Path(sys.argv[1])
    directory = root / "trillionnium/crates/trnm-rts-sim/src/lib_parts/tests"
    wrapper = directory / "part_01.rs"
    text = wrapper.read_text(encoding="utf-8")
    for name in NAMES:
        old = include_block(name)
        new = module_block(name)
        if text.count(old) != 1:
            print(f"{name}: include seam did not match exactly once", file=sys.stderr)
            return 1
        text = text.replace(old, new)
        child = directory / f"{name}.rs"
        child_text = child.read_text(encoding="utf-8")
        if not child_text.startswith("use super::*;\n"):
            child.write_text("use super::*;\n\n" + child_text, encoding="utf-8", newline="\n")
    wrapper.write_text(text, encoding="utf-8", newline="\n")
    if "include!(" in wrapper.read_text(encoding="utf-8"):
        print("RTS wrapper still contains include!", file=sys.stderr)
        return 1
    print("WORLD_RTS_INNER_TEST_MODULE_MIGRATION=PATCHED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
