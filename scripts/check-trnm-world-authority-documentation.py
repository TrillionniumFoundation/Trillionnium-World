#!/usr/bin/env python3
"""Validate module contracts and detailed designs for the isolated World-domain authority workspace."""

from __future__ import annotations

import argparse
import datetime as dt
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import re
import sys
import tomllib

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location(
    "trnm_world_detailed_docs_base",
    HERE / "check-trnm-world-detailed-documentation.py",
)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("unable to load detailed documentation checker")
BASE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BASE)

WORKSPACE = "trillionnium/crates/world-authority/Cargo.toml"
WORKSPACE_README = "trillionnium/crates/world-authority/README.md"
INDEX = "docs/modules/world-authority/README.md"
CATALOG = "docs/catalog.json"
ADR = "docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md"
MAX_DOCUMENT_BYTES = 512 * 1024
MIN_README_SECTION_CHARS = 80
MIN_DESIGN_SECTION_CHARS = 140

EXPECTED_MODULES = {
    "trnm-world-domain",
    "trnm-world-command",
    "trnm-world-projection",
    "trnm-world-map-provider",
    "trnm-world-ui-fragments",
    "trnm-world-api",
    "trnm-world-server",
}
README_SECTIONS = (
    "Purpose",
    "Authority and non-goals",
    "Public contracts",
    "State and invariants",
    "Dependencies and boundaries",
    "Failure and recovery",
    "Testing and evidence",
    "Compatibility and change control",
    "Detailed design",
)
DESIGN_SECTIONS = BASE.REQUIRED_SECTIONS
MODULE_TERMS = {
    "trnm-world-domain": ("worldstate", "nakama", "cex", "canonical"),
    "trnm-world-command": ("worldcommand", "expected revision", "state", "nakama"),
    "trnm-world-projection": ("rebuildable", "source", "cache", "authority"),
    "trnm-world-map-provider": ("fixture-only", "odbl", "sensitive", "public-network"),
    "trnm-world-ui-fragments": ("escaping", "data-*", "xss", "server"),
    "trnm-world-api": ("worldrepository", "worldledgeradapter", "lookup-before-submit", "nakama"),
    "trnm-world-server": ("file-backed", "postgresql", "no-dual-writer", "not production"),
}
FORBIDDEN_OVERCLAIMS = (
    "production authorization granted",
    "production adapter complete",
    "public player route enabled",
    "public online enabled",
    "world owns canonical online order",
    "world signs canonical matchcompletedv1",
    "world owns wallet custody",
    "world owns chain finality",
)
REQUIRED_CATALOG_CLASSES = {
    WORKSPACE_README: "world-authority-workspace",
    INDEX: "world-authority-module-index",
    "docs/architecture/cex-world-authority-cutover-v1.md": "world-authority-architecture",
    "docs/contracts/trillionnium-world-authority-cutover-v1.json": "world-authority-contract",
    "docs/contracts/trillionnium-world-authority-provenance-v1.json": "world-authority-contract",
}


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


def read_text(root: Path, relative: str) -> str:
    path = safe_relative(relative)
    cursor = root
    for part in path.parts:
        cursor = cursor / part
        if cursor.is_symlink():
            raise DocumentationFailure(f"symlink is not accepted: {relative}")
    if not cursor.is_file():
        raise DocumentationFailure(f"missing file: {relative}")
    if cursor.stat().st_size > MAX_DOCUMENT_BYTES:
        raise DocumentationFailure(f"oversized file: {relative}")
    text = cursor.read_text(encoding="utf-8")
    if not text.strip():
        raise DocumentationFailure(f"empty file: {relative}")
    return text


def parse_date(value: str, field: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as error:
        raise DocumentationFailure(f"invalid {field}: {value}") from error


def normalized_visible(text: str) -> str:
    return " ".join(BASE.visible_markdown(text).lower().split())


def catalog_entries(root: Path, as_of: dt.date) -> dict[str, dict[str, str]]:
    try:
        data = json.loads(read_text(root, CATALOG))
    except json.JSONDecodeError as error:
        raise DocumentationFailure(f"invalid {CATALOG}: {error}") from error
    if data.get("schema") != "trnm_world_document_catalog_v1":
        raise DocumentationFailure("catalogue schema drift")
    documents = data.get("documents")
    if not isinstance(documents, list):
        raise DocumentationFailure("catalogue documents must be an array")
    entries: dict[str, dict[str, str]] = {}
    for item in documents:
        if not isinstance(item, dict):
            raise DocumentationFailure("catalogue entry must be an object")
        relative = item.get("path")
        if not isinstance(relative, str) or relative in entries:
            raise DocumentationFailure(f"invalid or duplicate catalogue path: {relative!r}")
        for field in ("class", "owner", "status", "review_due"):
            if not isinstance(item.get(field), str) or not item[field].strip():
                raise DocumentationFailure(f"invalid catalogue {field}: {relative}")
        if parse_date(item["review_due"], f"catalogue review_due for {relative}") < as_of:
            raise DocumentationFailure(f"stale catalogue entry: {relative}")
        entries[relative] = item
    return entries


def validate(root: Path, as_of: dt.date) -> list[str]:
    root = root.resolve()
    workspace_text = read_text(root, WORKSPACE)
    workspace = tomllib.loads(workspace_text).get("workspace", {})
    members = workspace.get("members")
    if (
        not isinstance(members, list)
        or not members
        or not all(isinstance(member, str) for member in members)
        or len(members) != len(set(members))
        or any(any(marker in member for marker in "*?[") for member in members)
    ):
        raise DocumentationFailure("authority workspace members must be an explicit unique list")
    metadata = workspace.get("metadata", {}).get("trnm", {})
    if metadata.get("production_authorization") != "not_granted":
        raise DocumentationFailure("authority workspace must retain production_authorization=not_granted")
    if metadata.get("cex_role") != "authenticated-ingress-and-projection-consumer-only":
        raise DocumentationFailure("authority workspace CEX role drift")

    packages: list[tuple[str, str]] = []
    names: set[str] = set()
    for member in members:
        safe_relative(f"trillionnium/crates/world-authority/{member}/Cargo.toml")
        manifest = tomllib.loads(
            read_text(root, f"trillionnium/crates/world-authority/{member}/Cargo.toml")
        )
        name = manifest.get("package", {}).get("name")
        if not isinstance(name, str) or not re.fullmatch(r"[A-Za-z0-9_-]+", name) or name in names:
            raise DocumentationFailure(f"invalid or duplicate package identity: {name!r}")
        if name != member:
            raise DocumentationFailure(f"member/package mismatch: {member} != {name}")
        names.add(name)
        packages.append((member, name))
    if names != EXPECTED_MODULES:
        raise DocumentationFailure(
            f"authority module set drift; missing={sorted(EXPECTED_MODULES - names)}, "
            f"extra={sorted(names - EXPECTED_MODULES)}"
        )

    entries = catalog_entries(root, as_of)
    for path, expected_class in REQUIRED_CATALOG_CLASSES.items():
        entry = entries.get(path)
        if not entry or entry.get("class") != expected_class:
            raise DocumentationFailure(f"{path}: missing catalogue class {expected_class}")
        read_text(root, path)

    workspace_index = normalized_visible(read_text(root, WORKSPACE_README))
    design_index = normalized_visible(read_text(root, INDEX))
    adr = normalized_visible(read_text(root, ADR))
    for required in (
        "nakama remains the sole target canonical online owner",
        "cex remains the sole wallet/ledger owner",
        "one admitted writer epoch",
    ):
        if required not in adr:
            raise DocumentationFailure(f"{ADR}: missing binding authority statement: {required}")

    expected_designs: set[str] = set()
    expected_readmes: set[str] = set()
    for member, name in packages:
        readme_path = f"trillionnium/crates/world-authority/{member}/README.md"
        design_path = f"docs/modules/world-authority/{name}-design.md"
        expected_readmes.add(readme_path)
        expected_designs.add(design_path)

        readme = read_text(root, readme_path)
        visible_readme = BASE.visible_markdown(readme)
        if not re.search(rf"(?m)^# {re.escape(name)}\s*$", visible_readme):
            raise DocumentationFailure(f"{readme_path}: title mismatch")
        sections = BASE.parse_sections(readme)
        for heading in README_SECTIONS:
            body = sections.get(heading)
            if body is None:
                raise DocumentationFailure(f"{readme_path}: missing section {heading}")
            if len(" ".join(body.split())) < MIN_README_SECTION_CHARS:
                raise DocumentationFailure(f"{readme_path}: shallow section {heading}")
        expected_link = f"../../../../docs/modules/world-authority/{name}-design.md"
        if expected_link not in readme:
            raise DocumentationFailure(f"{readme_path}: missing detailed design link")

        design = read_text(root, design_path)
        metadata = BASE.parse_front_matter(design)
        if metadata.get("status") != "current-candidate":
            raise DocumentationFailure(f"{design_path}: status must be current-candidate")
        if metadata.get("module") != name:
            raise DocumentationFailure(f"{design_path}: module metadata mismatch")
        if metadata.get("implementation_conformance") != "not-implied":
            raise DocumentationFailure(f"{design_path}: implementation_conformance must be not-implied")
        if not metadata.get("owner"):
            raise DocumentationFailure(f"{design_path}: owner is required")
        if parse_date(metadata.get("last_reviewed", ""), f"last_reviewed for {name}") > as_of:
            raise DocumentationFailure(f"{design_path}: last_reviewed is in the future")
        if parse_date(metadata.get("review_due", ""), f"review_due for {name}") < as_of:
            raise DocumentationFailure(f"{design_path}: review is stale")
        visible_design = BASE.visible_markdown(design)
        if not re.search(rf"(?m)^# {re.escape(name)} detailed design\s*$", visible_design):
            raise DocumentationFailure(f"{design_path}: title mismatch")
        design_sections = BASE.parse_sections(design)
        for heading in DESIGN_SECTIONS:
            body = design_sections.get(heading)
            if body is None:
                raise DocumentationFailure(f"{design_path}: missing section {heading}")
            if len(" ".join(body.split())) < MIN_DESIGN_SECTION_CHARS:
                raise DocumentationFailure(f"{design_path}: shallow section {heading}")

        normalized = normalized_visible(readme + "\n" + design)
        if any(marker in normalized for marker in FORBIDDEN_OVERCLAIMS):
            raise DocumentationFailure(f"{name}: prohibited authority/release overclaim")
        for term in MODULE_TERMS[name]:
            if term not in normalized:
                raise DocumentationFailure(f"{design_path}: required module term is absent: {term}")
        trace = design_sections["Test and evidence traceability"].lower()
        if "trillionnium/" not in trace or "docs/" not in trace or "scripts/" not in trace:
            raise DocumentationFailure(f"{design_path}: traceability must bind source, docs and scripts")

        readme_entry = entries.get(readme_path)
        design_entry = entries.get(design_path)
        if not readme_entry or readme_entry.get("class") != "world-authority-module-contract":
            raise DocumentationFailure(f"{readme_path}: missing module-contract catalogue entry")
        if not design_entry or design_entry.get("class") != "world-authority-module-design":
            raise DocumentationFailure(f"{design_path}: missing module-design catalogue entry")
        if name not in workspace_index or name not in design_index:
            raise DocumentationFailure(f"{name}: missing workspace/design index entry")
        if design_path.rsplit("/", 1)[-1].lower() not in design_index:
            raise DocumentationFailure(f"{INDEX}: missing design link for {name}")

    actual_designs = {
        path for path, entry in entries.items()
        if entry.get("class") == "world-authority-module-design"
    }
    actual_readmes = {
        path for path, entry in entries.items()
        if entry.get("class") == "world-authority-module-contract"
    }
    if actual_designs != expected_designs:
        raise DocumentationFailure(
            f"catalogued authority designs drift; expected={sorted(expected_designs)}, "
            f"actual={sorted(actual_designs)}"
        )
    if actual_readmes != expected_readmes:
        raise DocumentationFailure(
            f"catalogued authority contracts drift; expected={sorted(expected_readmes)}, "
            f"actual={sorted(actual_readmes)}"
        )
    return sorted(names)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=HERE.parent)
    parser.add_argument(
        "--as-of",
        default=os.environ.get("TRNM_DOC_AS_OF")
        or dt.datetime.now(dt.timezone.utc).date().isoformat(),
    )
    args = parser.parse_args()
    try:
        names = validate(args.root, parse_date(args.as_of, "as_of"))
    except (
        DocumentationFailure,
        BASE.DocumentationFailure,
        OSError,
        UnicodeError,
        json.JSONDecodeError,
        tomllib.TOMLDecodeError,
    ) as error:
        print(f"TRNM World authority documentation: FAIL: {error}", file=sys.stderr)
        return 1
    print(
        f"TRNM World authority documentation: PASS "
        f"({len(names)}/{len(names)} modules; isolated cutover workspace)"
    )
    print(
        "Implementation, hosted execution, server controls, external evidence "
        "and production authorization remain separate denominators."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
