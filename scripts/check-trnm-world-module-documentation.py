#!/usr/bin/env python3
"""Validate active World module documentation as review contracts.

This gate proves discoverability and structural completeness only. It does not
prove implementation conformance, hosted execution, deployment or release
eligibility.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path, PurePosixPath
import re
import sys
import tomllib
from urllib.parse import unquote

MATRIX = "docs/development/trnm-world-module-documentation-matrix-v1.md"
DESIGN_MANIFEST = "docs/modules/contracts-v2.json"
DESIGN_INDEX = "docs/modules/README.md"
SUMMARY_REQUIRED = (
    ("Purpose",),
    ("Authority and non-goals",),
    ("Public contracts", "Runtime composition"),
    ("State and invariants",),
    ("Dependencies and boundaries",),
    ("Failure and recovery",),
    ("Testing and evidence",),
    ("Compatibility and change control",),
)
DESIGN_REQUIRED = (
    "Current implementation",
    "Responsibilities and non-responsibilities",
    "Public interfaces and data contracts",
    "State model and invariants",
    "Dependency direction",
    "Concurrency and cancellation",
    "Durability and recovery",
    "Failure, retry and idempotency",
    "Versioning and migration",
    "Resource budgets and performance",
    "Observability and security",
    "Verification traceability",
    "Known gaps and change checklist",
)
FORBIDDEN_OVERCLAIMS = (
    "production authorization granted",
    "public online enabled",
    "public player market enabled",
    "world owns wallet custody",
    "world owns chain finality",
    "world signs canonical matchcompletedv1",
    "all plan gaps closed: true",
    '"production_ready": true',
)
CURRENT_DOCUMENTS = (
    "README.md",
    "OPERATIONS.md",
    "RELEASE_READINESS.md",
    "docs/README.md",
    "docs/modules/README.md",
    "docs/adr/0003-world-authority-service-boundary.md",
    "docs/development/TRILLIONNIUM_WORLD_CLOSURE_EXECUTION_BOARD_V5.md",
)
MAX_DOCUMENT_BYTES = 256 * 1024
MIN_SUMMARY_SECTION_CHARS = 40
MIN_DESIGN_SECTION_CHARS = 160


class DocumentationFailure(ValueError):
    pass


def safe_relative(relative: str) -> PurePosixPath:
    path = PurePosixPath(relative)
    if (
        not path.parts
        or path.is_absolute()
        or ".." in path.parts
        or "\\" in relative
        or path.as_posix() != relative
    ):
        raise DocumentationFailure(f"unsafe path: {relative}")
    return path


def read_inside(root: Path, relative: str) -> str:
    path = safe_relative(relative)
    target = root / path
    cursor = root
    for part in path.parts:
        cursor = cursor / part
        if cursor.is_symlink():
            raise DocumentationFailure(f"symlink is not a documentation source: {relative}")
    if not target.is_file():
        raise DocumentationFailure(f"missing file: {relative}")
    if target.stat().st_size > MAX_DOCUMENT_BYTES:
        raise DocumentationFailure(f"oversized documentation file: {relative}")
    text = target.read_text(encoding="utf-8")
    if not text.strip():
        raise DocumentationFailure(f"empty file: {relative}")
    return text


def visible_markdown(text: str) -> str:
    if text.count("<!--") != text.count("-->"):
        raise DocumentationFailure("unbalanced Markdown comment")
    text = re.sub(r"<!--.*?-->", "", text, flags=re.S)
    lines: list[str] = []
    fence: str | None = None
    length = 0
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
    result: dict[str, str] = {}
    for index, match in enumerate(matches):
        heading = match[1].strip()
        if heading in result:
            raise DocumentationFailure(f"duplicate section: {heading}")
        end = matches[index + 1].start() if index + 1 < len(matches) else len(visible)
        result[heading] = visible[match.end():end].strip()
    return result


def reject_placeholder(relative: str, heading: str, body: str, minimum: int) -> None:
    normalized = " ".join(body.split())
    if len(normalized) < minimum:
        raise DocumentationFailure(f"{relative}: section too small: {heading}")
    if re.fullmatch(r"(?is)[\W_]*(?:todo|tbd|pending|placeholder|n/a)[\W_]*", normalized):
        raise DocumentationFailure(f"{relative}: placeholder section: {heading}")


def validate_local_links(root: Path, relative: str, text: str) -> None:
    base = PurePosixPath(relative).parent
    visible = visible_markdown(text)
    for raw in re.findall(r"(?<!!)\[[^\]]*\]\(([^)]+)\)", visible):
        destination = raw.strip().split(maxsplit=1)[0].strip("<>")
        if not destination or destination.startswith(("#", "http://", "https://", "mailto:")):
            continue
        destination = unquote(destination.split("#", 1)[0].split("?", 1)[0])
        if not destination:
            continue
        candidate = base / PurePosixPath(destination)
        parts: list[str] = []
        for part in candidate.parts:
            if part in ("", "."):
                continue
            if part == "..":
                if not parts:
                    raise DocumentationFailure(f"{relative}: link escapes repository: {raw}")
                parts.pop()
            else:
                parts.append(part)
        normalized = PurePosixPath(*parts)
        target = root / normalized
        cursor = root
        for part in normalized.parts:
            cursor = cursor / part
            if cursor.is_symlink():
                raise DocumentationFailure(f"{relative}: link resolves through symlink: {raw}")
        if not target.exists():
            raise DocumentationFailure(f"{relative}: broken local link: {raw}")


def reject_overclaims(relative: str, text: str) -> None:
    normalized = " ".join(visible_markdown(text).lower().split())
    for marker in FORBIDDEN_OVERCLAIMS:
        if marker in normalized:
            raise DocumentationFailure(f"{relative}: prohibited authority/release assertion: {marker}")


def reject_personal_paths(relative: str, text: str) -> None:
    patterns = (
        r"/Users/[A-Za-z0-9._-]+/",
        r"/home/(?!runner(?:/|$)|github(?:/|$))[A-Za-z0-9._-]+/",
        r"[A-Za-z]:\\Users\\[A-Za-z0-9._-]+\\",
        r"\.openclaw/workspace",
    )
    for pattern in patterns:
        if re.search(pattern, text):
            raise DocumentationFailure(f"{relative}: personal or compatibility path is forbidden")


def workspace_packages(root: Path) -> tuple[list[str], dict[str, str]]:
    workspace = tomllib.loads(read_inside(root, "trillionnium/Cargo.toml")).get("workspace", {})
    members = workspace.get("members")
    if (
        not isinstance(members, list)
        or not members
        or len(members) > 128
        or not all(isinstance(member, str) for member in members)
        or len(members) != len(set(members))
    ):
        raise DocumentationFailure("workspace members must be a nonempty unique explicit list")
    names: list[str] = []
    member_by_name: dict[str, str] = {}
    for member in members:
        if any(char in member for char in "*?["):
            raise DocumentationFailure("expand Cargo member globs before documentation validation")
        safe_relative(f"trillionnium/{member}")
        manifest = tomllib.loads(read_inside(root, f"trillionnium/{member}/Cargo.toml"))
        name = manifest.get("package", {}).get("name")
        if not isinstance(name, str) or not re.fullmatch(r"[a-zA-Z0-9_-]+", name) or name in names:
            raise DocumentationFailure("invalid or duplicate package identity")
        names.append(name)
        member_by_name[name] = member
    return names, member_by_name


def validate_summary(root: Path, name: str, member: str) -> str:
    relative = f"trillionnium/{member}/README.md"
    text = read_inside(root, relative)
    if not re.search(r"(?m)^# " + re.escape(name) + r"\s*$", visible_markdown(text)):
        raise DocumentationFailure(f"{relative}: package title mismatch")
    parsed = sections(text)
    for alternatives in SUMMARY_REQUIRED:
        matches = [(heading, parsed[heading]) for heading in alternatives if heading in parsed]
        if not matches:
            raise DocumentationFailure(f"{relative}: missing {' / '.join(alternatives)}")
        for heading, body in matches:
            reject_placeholder(relative, heading, body, MIN_SUMMARY_SECTION_CHARS)
    reject_overclaims(relative, text)
    return relative


def validate_design_manifest(root: Path, names: list[str], member_by_name: dict[str, str]) -> list[str]:
    try:
        data = json.loads(read_inside(root, DESIGN_MANIFEST))
    except json.JSONDecodeError as error:
        raise DocumentationFailure(f"{DESIGN_MANIFEST}: invalid JSON: {error}") from error
    if data.get("schema") != "trnm_world_module_contracts_v2":
        raise DocumentationFailure(f"{DESIGN_MANIFEST}: unexpected schema")
    entries = data.get("modules")
    if not isinstance(entries, list) or not entries:
        raise DocumentationFailure(f"{DESIGN_MANIFEST}: modules must be a nonempty list")
    packages = [entry.get("package") for entry in entries if isinstance(entry, dict)]
    if len(packages) != len(entries) or len(packages) != len(set(packages)):
        raise DocumentationFailure(f"{DESIGN_MANIFEST}: duplicate or invalid package rows")
    if set(packages) != set(names):
        raise DocumentationFailure(f"{DESIGN_MANIFEST}: package set differs from Cargo workspace")

    designs: list[str] = []
    for entry in entries:
        package = entry["package"]
        expected_summary = f"trillionnium/{member_by_name[package]}/README.md"
        if entry.get("summary_document") != expected_summary:
            raise DocumentationFailure(f"{DESIGN_MANIFEST}: wrong summary path for {package}")
        relative = entry.get("technical_design")
        if not isinstance(relative, str):
            raise DocumentationFailure(f"{DESIGN_MANIFEST}: missing technical design for {package}")
        safe_relative(relative)
        if entry.get("release_authority") is not False:
            raise DocumentationFailure(f"{DESIGN_MANIFEST}: module docs cannot grant release authority")
        text = read_inside(root, relative)
        visible = visible_markdown(text)
        if not re.search(r"(?m)^# " + re.escape(package) + r" technical design\s*$", visible):
            raise DocumentationFailure(f"{relative}: technical design title mismatch")
        for metadata in ("status: current-candidate", "owner:", "last_reviewed:", "review_due:"):
            if metadata not in text:
                raise DocumentationFailure(f"{relative}: missing metadata {metadata}")
        parsed = sections(text)
        for heading in DESIGN_REQUIRED:
            if heading not in parsed:
                raise DocumentationFailure(f"{relative}: missing detailed section {heading}")
            reject_placeholder(relative, heading, parsed[heading], MIN_DESIGN_SECTION_CHARS)
        validate_local_links(root, relative, text)
        reject_overclaims(relative, text)
        reject_personal_paths(relative, text)
        designs.append(relative)
    return designs


def validate_indexes(root: Path, names: list[str]) -> None:
    matrix = visible_markdown(read_inside(root, MATRIX))
    matrix_rows = re.findall(r"(?m)^\|\s*`([^`]+)`\s*\|", matrix)
    if len(matrix_rows) != len(set(matrix_rows)) or set(matrix_rows) != set(names):
        raise DocumentationFailure("module matrix rows must exactly match active Cargo packages")

    index_text = read_inside(root, DESIGN_INDEX)
    index_rows = re.findall(r"(?m)^\|\s*`([^`]+)`\s*\|", visible_markdown(index_text))
    if len(index_rows) != len(set(index_rows)) or set(index_rows) != set(names):
        raise DocumentationFailure("module design index rows must exactly match active Cargo packages")
    validate_local_links(root, DESIGN_INDEX, index_text)
    reject_overclaims(DESIGN_INDEX, index_text)
    reject_personal_paths(DESIGN_INDEX, index_text)


def validate_repository_facts(root: Path) -> None:
    for forbidden in (
        "trillionnium/crates/trnm-game-server/build.rs",
        "trillionnium/crates/trnm-game-server/src/lib.rs.in",
    ):
        if (root / forbidden).exists():
            raise DocumentationFailure(f"forbidden semantic source-generation path exists: {forbidden}")

    for relative in CURRENT_DOCUMENTS:
        text = read_inside(root, relative)
        reject_overclaims(relative, text)
        reject_personal_paths(relative, text)
        validate_local_links(root, relative, text)

    docs_index = read_inside(root, "docs/README.md")
    stale_assertions = (
        "build.rs still transforms into the compiled library",
        "semantic build.rs / src/lib.rs.in authority is still open",
    )
    normalized = " ".join(docs_index.lower().split())
    for assertion in stale_assertions:
        if assertion in normalized:
            raise DocumentationFailure(f"docs/README.md: stale source-generation assertion: {assertion}")


def validate(root: Path) -> list[str]:
    root = root.resolve()
    names, member_by_name = workspace_packages(root)
    summaries = [validate_summary(root, name, member_by_name[name]) for name in names]
    designs = validate_design_manifest(root, names, member_by_name)
    validate_indexes(root, names)
    validate_repository_facts(root)
    if len(summaries) != len(designs):
        raise DocumentationFailure("summary/design cardinality mismatch")
    return sorted(names)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        names = validate(args.root)
    except (DocumentationFailure, OSError, UnicodeError, json.JSONDecodeError, tomllib.TOMLDecodeError) as error:
        print(f"TRNM World module documentation: FAIL: {error}", file=sys.stderr)
        return 1
    print(f"TRNM World module documentation: PASS ({len(names)}/{len(names)} summaries + detailed designs)")
    print("Implementation conformance, hosted CI, deployment and release evidence remain separate gates.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
