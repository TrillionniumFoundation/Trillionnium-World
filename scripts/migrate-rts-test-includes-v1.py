#!/usr/bin/env python3
"""Convert the three reviewed RTS test-body includes into ordinary Rust modules."""

from __future__ import annotations

from pathlib import Path
import re
import sys

PART_RELATIVE = Path("trillionnium/crates/trnm-rts-sim/src/lib_parts/tests/part_01.rs")
TEST_NAMES = ("rts_tests_01", "rts_tests_02", "rts_tests_03")


def include_pattern(name: str) -> re.Pattern[str]:
    return re.compile(
        r'include!\(concat!\(\s*env!\("CARGO_MANIFEST_DIR"\),\s*'
        + re.escape(f'"/src/lib_parts/tests/{name}.rs"')
        + r'\s*\)\);',
        re.MULTILINE,
    )


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: migrate-rts-test-includes-v1.py REPOSITORY_ROOT", file=sys.stderr)
        return 64
    root = Path(sys.argv[1]).resolve()
    part = root / PART_RELATIVE
    text = part.read_text(encoding="utf-8")
    changed = []

    for name in TEST_NAMES:
        module = f'#[path = "{name}.rs"]\nmod {name};'
        pattern = include_pattern(name)
        matches = list(pattern.finditer(text))
        if len(matches) == 1:
            text = pattern.sub(module, text, count=1)
            changed.append(PART_RELATIVE.as_posix())
        elif module in text and len(matches) == 0:
            pass
        else:
            print(
                f"{name}: expected exactly one include or one existing module declaration",
                file=sys.stderr,
            )
            return 1

        child_relative = PART_RELATIVE.parent / f"{name}.rs"
        child = root / child_relative
        child_text = child.read_text(encoding="utf-8")
        if child_text.startswith("use super::*;\n"):
            continue
        if "#![" in child_text[:256]:
            print(f"{child_relative}: inner attribute prevents safe import insertion", file=sys.stderr)
            return 1
        child.write_text("use super::*;\n\n" + child_text, encoding="utf-8", newline="\n")
        changed.append(child_relative.as_posix())

    part.write_text(text, encoding="utf-8", newline="\n")
    remaining = [
        name for name in TEST_NAMES
        if f'/src/lib_parts/tests/{name}.rs' in text or f"include!" in text and name in text
    ]
    if remaining:
        print(f"RTS include migration incomplete: {remaining}", file=sys.stderr)
        return 1
    for name in TEST_NAMES:
        declaration = f'#[path = "{name}.rs"]\nmod {name};'
        if text.count(declaration) != 1:
            print(f"RTS module declaration drift: {name}", file=sys.stderr)
            return 1
    print("WORLD_RTS_TEST_INCLUDE_MIGRATION=PASS")
    for path in sorted(set(changed)):
        print(f"CHANGED={path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
