#!/usr/bin/env python3
"""Check active workspace documentation structure, not implementation conformance."""
from __future__ import annotations
import argparse
from pathlib import Path, PurePosixPath
import re
import sys
import tomllib

MATRIX = "docs/development/trnm-world-module-documentation-matrix-v1.md"
REQUIRED = (
    ("Purpose",), ("Authority and non-goals",),
    ("Public contracts", "Runtime composition"), ("State and invariants",),
    ("Dependencies and boundaries",), ("Failure and recovery",),
    ("Testing and evidence",), ("Compatibility and change control",),
)
FORBIDDEN_OVERCLAIMS = (
    "production authorization granted", "public online enabled", "public player market enabled",
    "world owns wallet custody", "world owns chain finality", "world signs canonical matchcompletedv1",
)
MAX_DOCUMENT_BYTES = 256 * 1024


class DocumentationFailure(ValueError):
    pass


def read_inside(root: Path, relative: str) -> str:
    path = PurePosixPath(relative)
    if (not path.parts or path.is_absolute() or ".." in path.parts or "\\" in relative or
            path.as_posix() != relative):
        raise DocumentationFailure(f"unsafe path: {relative}")
    target = root / relative
    cursor = root
    for part in path.parts:
        cursor = cursor / part
        if cursor.is_symlink():
            raise DocumentationFailure(f"symlink is not a module documentation source: {relative}")
    if not target.is_file() or target.stat().st_size > MAX_DOCUMENT_BYTES:
        raise DocumentationFailure(f"missing or oversized file: {relative}")
    text = target.read_text(encoding="utf-8")
    if not text.strip():
        raise DocumentationFailure(f"empty file: {relative}")
    return text


def visible_markdown(text: str) -> str:
    if text.count("<!--") != text.count("-->"):
        raise DocumentationFailure("unbalanced Markdown comment")
    text = re.sub(r"<!--.*?-->", "", text, flags=re.S)
    lines, fence, length = [], None, 0
    for line in text.splitlines():
        match = re.match(r"^ {0,3}(`{3,}|~{3,})(.*)$", line)
        if fence:
            if match and match[1][0] == fence and len(match[1]) >= length and not match[2].strip():
                fence = None
            continue
        if match:
            fence, length = match[1][0], len(match[1])
            continue
        lines.append(line)
    if fence:
        raise DocumentationFailure("unclosed Markdown code fence")
    return "\n".join(lines)


def sections(text: str) -> dict[str, str]:
    visible = visible_markdown(text)
    matches = list(re.finditer(r"(?m)^## ([^\n]+)\s*$", visible))
    result = {}
    for index, match in enumerate(matches):
        heading = match[1].strip()
        if heading in result:
            raise DocumentationFailure(f"duplicate section: {heading}")
        end = matches[index + 1].start() if index + 1 < len(matches) else len(visible)
        result[heading] = visible[match.end():end].strip()
    return result


def validate(root: Path) -> list[str]:
    root = root.resolve()
    workspace = tomllib.loads(read_inside(root, "trillionnium/Cargo.toml")).get("workspace", {})
    members = workspace.get("members")
    if (not isinstance(members, list) or not members or len(members) > 128 or
            not all(isinstance(m, str) for m in members) or len(members) != len(set(members))):
        raise DocumentationFailure("workspace members must be a nonempty, unique explicit list")
    names = []
    for member in members:
        if any(c in member for c in "*?["):
            raise DocumentationFailure("expand Cargo member globs explicitly before this structural gate")
        manifest = tomllib.loads(read_inside(root, f"trillionnium/{member}/Cargo.toml"))
        name = manifest.get("package", {}).get("name")
        if not isinstance(name, str) or not re.fullmatch(r"[a-zA-Z0-9_-]+", name) or name in names:
            raise DocumentationFailure("invalid or duplicate package identity")
        names.append(name)
        relative = f"trillionnium/{member}/README.md"
        text = read_inside(root, relative)
        if not re.search(r"(?m)^# " + re.escape(name) + r"\s*$", visible_markdown(text)):
            raise DocumentationFailure(f"{relative}: package title mismatch")
        parsed = sections(text)
        for alternatives in REQUIRED:
            found = [parsed[key] for key in alternatives if key in parsed]
            if not found:
                raise DocumentationFailure(f"{relative}: missing {' / '.join(alternatives)}")
            for body in found:
                if len(body) < 40 or re.fullmatch(r"(?is)[\W_]*(?:todo|tbd|pending|placeholder)[\W_]*", body):
                    raise DocumentationFailure(f"{relative}: empty/placeholder section {' / '.join(alternatives)}")
        normalized = " ".join(visible_markdown(text).lower().split())
        if any(marker in normalized for marker in FORBIDDEN_OVERCLAIMS):
            raise DocumentationFailure(f"{relative}: prohibited authority/release assertion")
    rows = re.findall(r"(?m)^\|\s*`([^`]+)`\s*\|", visible_markdown(read_inside(root, MATRIX)))
    if len(rows) != len(set(rows)) or set(rows) != set(names):
        raise DocumentationFailure("matrix row identities must exactly match active Cargo package identities")
    return sorted(names)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        names = validate(args.root)
    except (DocumentationFailure, OSError, UnicodeError, tomllib.TOMLDecodeError) as error:
        print(f"TRNM World module documentation: FAIL: {error}", file=sys.stderr)
        return 1
    print(f"TRNM World active module documentation: PASS ({len(names)}/{len(names)}; entry/section coverage only)")
    print("Implementation, detailed-design sufficiency, standalone components and release evidence are not validated by this check.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
