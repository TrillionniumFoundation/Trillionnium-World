#!/usr/bin/env python3
"""Fail closed on unclassified tracked Rust include! ownership seams."""

from __future__ import annotations

import argparse
import json
from pathlib import Path, PurePosixPath
import re
import sys
from typing import Iterable

INVENTORY = "scripts/contracts/trnm-world-include-boundaries-v1.json"
MAX_INVENTORY_BYTES = 512 * 1024
MAX_SOURCE_BYTES = 4 * 1024 * 1024
ALLOWED_CLASSES = {
    "handwritten-runtime",
    "test-only",
    "excluded-legacy-runtime",
    "excluded-legacy-test",
    "generated",
    "vendored",
    "asset-embedding",
}
ALLOWED_STATES = {"migrate", "frozen", "retained"}
EXCLUDED_DIR_NAMES = {".git", "target", "run", "acceptance", "artifacts", "__pycache__"}
ROOT_KEYS = {"schema", "as_of", "baseline_commit", "policy", "entries"}
ENTRY_KEYS = {"path", "expression", "class", "migration_state", "owner", "reason"}
POLICY_KEYS = {
    "new_unclassified_include",
    "handwritten_runtime",
    "test_only",
    "excluded_legacy",
    "generated_vendored_assets",
}
HEX40 = re.compile(r"^[0-9a-f]{40}$")
FULL_LINE_INCLUDE = re.compile(r"^\s*include!\s*\((?P<expression>.+)\)\s*;\s*$")
LITERAL_INCLUDE = re.compile(r'^"(?P<path>[^"]+)"$')
MANIFEST_INCLUDE = re.compile(
    r'^concat!\(env!\("CARGO_MANIFEST_DIR"\),"(?P<path>/[^"]+)"\)$'
)


class IncludeBoundaryFailure(ValueError):
    pass


def normalize_expression(value: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise IncludeBoundaryFailure("include expression must be a nonempty string")
    return re.sub(r"\s+", "", value)


def safe_relative(value: str, field: str) -> PurePosixPath:
    if (
        not isinstance(value, str)
        or not value
        or value != value.strip()
        or "\\" in value
        or value.startswith("/")
        or value.startswith("./")
        or value.endswith("/")
    ):
        raise IncludeBoundaryFailure(f"unsafe {field}: {value!r}")
    result = PurePosixPath(value)
    if (
        result.is_absolute()
        or result.as_posix() != value
        or any(part in {"", ".", ".."} for part in result.parts)
    ):
        raise IncludeBoundaryFailure(f"unsafe {field}: {value!r}")
    return result


def checked_file(root: Path, relative: str, field: str) -> Path:
    path = safe_relative(relative, field)
    root_real = root.resolve(strict=True)
    current = root_real
    for part in path.parts:
        current = current / part
        if current.is_symlink():
            raise IncludeBoundaryFailure(f"{field} traverses a symlink: {relative}")
    try:
        resolved = current.resolve(strict=True)
        resolved.relative_to(root_real)
    except (OSError, ValueError) as error:
        raise IncludeBoundaryFailure(f"{field} escapes or is missing: {relative}") from error
    if not resolved.is_file():
        raise IncludeBoundaryFailure(f"{field} is not a regular file: {relative}")
    return resolved


def load_inventory(root: Path, relative: str) -> dict[str, object]:
    path = checked_file(root, relative, "inventory path")
    data = path.read_bytes()
    if len(data) > MAX_INVENTORY_BYTES:
        raise IncludeBoundaryFailure("include inventory exceeds the byte budget")
    try:
        value = json.loads(
            data.decode("utf-8", errors="strict"),
            object_pairs_hook=_strict_object,
            parse_constant=_reject_constant,
        )
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise IncludeBoundaryFailure(f"invalid strict inventory JSON: {error}") from error
    if not isinstance(value, dict) or set(value) != ROOT_KEYS:
        raise IncludeBoundaryFailure("include inventory root key set drift")
    if value.get("schema") != "trnm_world_include_boundary_inventory_v1":
        raise IncludeBoundaryFailure("include inventory schema drift")
    if not HEX40.fullmatch(str(value.get("baseline_commit", ""))):
        raise IncludeBoundaryFailure("include inventory baseline_commit is not 40 lowercase hex")
    policy = value.get("policy")
    if not isinstance(policy, dict) or set(policy) != POLICY_KEYS:
        raise IncludeBoundaryFailure("include inventory policy key set drift")
    if policy != {
        "new_unclassified_include": "reject",
        "handwritten_runtime": "migrate_to_explicit_modules",
        "test_only": "migrate_to_nested_test_modules",
        "excluded_legacy": "frozen_no_growth",
        "generated_vendored_assets": "classify_before_admission",
    }:
        raise IncludeBoundaryFailure("include inventory policy semantics drift")
    entries = value.get("entries")
    if not isinstance(entries, list):
        raise IncludeBoundaryFailure("include inventory entries must be a list")
    return value


def _strict_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise IncludeBoundaryFailure(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def _reject_constant(value: str) -> None:
    raise IncludeBoundaryFailure(f"non-finite JSON constant is forbidden: {value}")


def rust_code_mask(text: str) -> str:
    """Return a same-length mask with comments and literal contents blanked."""
    out = list(text)
    length = len(text)
    index = 0
    block_depth = 0

    def blank(start: int, end: int) -> None:
        for offset in range(start, end):
            if out[offset] != "\n":
                out[offset] = " "

    while index < length:
        if block_depth:
            if text.startswith("/*", index):
                blank(index, index + 2)
                block_depth += 1
                index += 2
            elif text.startswith("*/", index):
                blank(index, index + 2)
                block_depth -= 1
                index += 2
            else:
                if out[index] != "\n":
                    out[index] = " "
                index += 1
            continue

        if text.startswith("//", index):
            end = text.find("\n", index)
            end = length if end == -1 else end
            blank(index, end)
            index = end
            continue
        if text.startswith("/*", index):
            blank(index, index + 2)
            block_depth = 1
            index += 2
            continue

        raw = re.match(r'(?:br|rb|r)(?P<hashes>#{0,255})"', text[index:])
        if raw:
            hashes = raw.group("hashes")
            opening = raw.end()
            closing = '"' + hashes
            end = text.find(closing, index + opening)
            if end == -1:
                raise IncludeBoundaryFailure("unterminated Rust raw string")
            end += len(closing)
            blank(index, end)
            index = end
            continue

        prefix = 2 if text.startswith('b"', index) else 1 if text[index] == '"' else 0
        if prefix:
            cursor = index + prefix
            escaped = False
            while cursor < length:
                char = text[cursor]
                if escaped:
                    escaped = False
                elif char == "\\":
                    escaped = True
                elif char == '"':
                    cursor += 1
                    break
                cursor += 1
            else:
                raise IncludeBoundaryFailure("unterminated Rust string")
            blank(index, cursor)
            index = cursor
            continue

        if text[index] == "'":
            cursor = index + 1
            escaped = False
            closing = None
            while cursor < min(length, index + 16) and text[cursor] != "\n":
                char = text[cursor]
                if escaped:
                    escaped = False
                elif char == "\\":
                    escaped = True
                elif char == "'":
                    closing = cursor + 1
                    break
                cursor += 1
            if closing is not None:
                blank(index, closing)
                index = closing
                continue

        index += 1

    if block_depth:
        raise IncludeBoundaryFailure("unterminated Rust block comment")
    return "".join(out)


def source_paths(root: Path) -> Iterable[Path]:
    for path in sorted(root.rglob("*.rs")):
        try:
            relative = path.relative_to(root)
        except ValueError:
            continue
        if any(part in EXCLUDED_DIR_NAMES for part in relative.parts):
            continue
        if path.is_symlink():
            raise IncludeBoundaryFailure(f"Rust source is a symlink: {relative.as_posix()}")
        if path.is_file():
            yield path


def scan_includes(root: Path) -> dict[tuple[str, str], int]:
    observed: dict[tuple[str, str], int] = {}
    for path in source_paths(root):
        data = path.read_bytes()
        if len(data) > MAX_SOURCE_BYTES:
            raise IncludeBoundaryFailure(f"Rust source exceeds byte budget: {path.relative_to(root)}")
        try:
            text = data.decode("utf-8", errors="strict")
        except UnicodeDecodeError as error:
            raise IncludeBoundaryFailure(f"Rust source is not strict UTF-8: {path.relative_to(root)}") from error
        mask = rust_code_mask(text)
        relative = path.relative_to(root).as_posix()
        for match in re.finditer(r"\binclude!\s*\(", mask):
            line_number = text.count("\n", 0, match.start()) + 1
            line_start = text.rfind("\n", 0, match.start()) + 1
            line_end = text.find("\n", match.start())
            line_end = len(text) if line_end == -1 else line_end
            line = text[line_start:line_end]
            parsed = FULL_LINE_INCLUDE.fullmatch(line)
            if parsed is None:
                raise IncludeBoundaryFailure(
                    f"{relative}:{line_number}: include! must be a one-line standalone statement"
                )
            expression = normalize_expression(parsed.group("expression"))
            key = (relative, expression)
            if key in observed:
                raise IncludeBoundaryFailure(
                    f"{relative}:{line_number}: duplicate include expression"
                )
            observed[key] = line_number
    return observed


def nearest_manifest_root(root: Path, source: Path) -> Path:
    current = source.parent
    root_real = root.resolve(strict=True)
    while True:
        if (current / "Cargo.toml").is_file():
            return current
        if current == root_real:
            break
        current = current.parent
    raise IncludeBoundaryFailure(
        f"cannot locate Cargo.toml for manifest-relative include: {source.relative_to(root)}"
    )


def validate_target(root: Path, source_relative: str, expression: str) -> None:
    source = checked_file(root, source_relative, "include source")
    literal = LITERAL_INCLUDE.fullmatch(expression)
    manifest = MANIFEST_INCLUDE.fullmatch(expression)
    if literal:
        relative = literal.group("path")
        if "\\" in relative or relative.startswith("/") or any(
            part in {"", ".", ".."} for part in PurePosixPath(relative).parts
        ):
            raise IncludeBoundaryFailure(
                f"{source_relative}: unsafe literal include target {relative!r}"
            )
        target = source.parent.joinpath(*PurePosixPath(relative).parts)
    elif manifest:
        relative = manifest.group("path").lstrip("/")
        safe_relative(relative, "manifest-relative include target")
        target = nearest_manifest_root(root, source).joinpath(*PurePosixPath(relative).parts)
    else:
        raise IncludeBoundaryFailure(
            f"{source_relative}: unsupported dynamic include expression: {expression}"
        )

    root_real = root.resolve(strict=True)
    current = root_real
    try:
        relative_target = target.relative_to(root_real)
    except ValueError as error:
        raise IncludeBoundaryFailure(f"{source_relative}: include target escapes repository") from error
    for part in relative_target.parts:
        current = current / part
        if current.is_symlink():
            raise IncludeBoundaryFailure(
                f"{source_relative}: include target traverses a symlink"
            )
    try:
        resolved = target.resolve(strict=True)
        resolved.relative_to(root_real)
    except (OSError, ValueError) as error:
        raise IncludeBoundaryFailure(
            f"{source_relative}: include target is missing or escapes repository"
        ) from error
    if not resolved.is_file():
        raise IncludeBoundaryFailure(f"{source_relative}: include target is not a file")


def validate(root: Path, inventory_relative: str = INVENTORY) -> tuple[int, int]:
    root = root.resolve(strict=True)
    inventory = load_inventory(root, inventory_relative)
    raw_entries = inventory["entries"]
    expected: dict[tuple[str, str], dict[str, str]] = {}

    for raw in raw_entries:
        if not isinstance(raw, dict) or set(raw) != ENTRY_KEYS:
            raise IncludeBoundaryFailure("include inventory entry key set drift")
        if not all(isinstance(raw[key], str) and raw[key].strip() for key in ENTRY_KEYS):
            raise IncludeBoundaryFailure("include inventory entry contains a blank/non-string field")
        source_relative = raw["path"]
        safe_relative(source_relative, "include source path")
        if not source_relative.endswith(".rs"):
            raise IncludeBoundaryFailure(f"include source is not Rust: {source_relative}")
        expression = normalize_expression(raw["expression"])
        include_class = raw["class"]
        state = raw["migration_state"]
        if include_class not in ALLOWED_CLASSES:
            raise IncludeBoundaryFailure(f"unsupported include class: {include_class}")
        if state not in ALLOWED_STATES:
            raise IncludeBoundaryFailure(f"unsupported migration state: {state}")
        legacy = include_class.startswith("excluded-legacy-")
        if legacy != source_relative.startswith("trillionnium/crates/platform/"):
            raise IncludeBoundaryFailure(
                f"{source_relative}: excluded-legacy classification/path mismatch"
            )
        test_path = (
            "/tests/" in f"/{source_relative}"
            or Path(source_relative).stem.endswith("tests")
            or "/tests/" in f"/{expression}"
        )
        if include_class in {"test-only", "excluded-legacy-test"} and not test_path:
            raise IncludeBoundaryFailure(f"{source_relative}: test classification/path mismatch")
        if include_class in {"handwritten-runtime", "excluded-legacy-runtime"} and test_path:
            raise IncludeBoundaryFailure(f"{source_relative}: runtime classification/path mismatch")
        if legacy and state != "frozen":
            raise IncludeBoundaryFailure(f"{source_relative}: excluded legacy include is not frozen")
        if include_class in {"handwritten-runtime", "test-only"} and state != "migrate":
            raise IncludeBoundaryFailure(f"{source_relative}: active handwritten include is not marked migrate")
        key = (source_relative, expression)
        if key in expected:
            raise IncludeBoundaryFailure(f"duplicate include inventory entry: {key}")
        validate_target(root, source_relative, expression)
        expected[key] = raw

    observed = scan_includes(root)
    missing = sorted(set(expected) - set(observed))
    unclassified = sorted(set(observed) - set(expected))
    if missing or unclassified:
        raise IncludeBoundaryFailure(
            f"include inventory drift; missing={missing}, unclassified={unclassified}"
        )
    active = sum(
        1
        for item in expected.values()
        if item["class"] in {"handwritten-runtime", "test-only"}
    )
    frozen = len(expected) - active
    return active, frozen


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=str(Path(__file__).resolve().parents[1]))
    parser.add_argument("--inventory", default=INVENTORY)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    try:
        active, frozen = validate(Path(args.root), args.inventory)
    except (IncludeBoundaryFailure, OSError, ValueError) as error:
        print(f"TRNM World include boundary: FAIL: {error}", file=sys.stderr)
        return 1
    print(
        "TRNM World include boundary: PASS "
        f"(active_migration={active}, frozen_legacy={frozen})"
    )
    print("Classification does not imply migration completion or production authorization.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
