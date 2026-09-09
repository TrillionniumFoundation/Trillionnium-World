#!/usr/bin/env python3
"""Validate the current document catalogue and detailed designs for every active World crate."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import os
from pathlib import Path, PurePosixPath
import re
import sys
import tomllib

CATALOG = "docs/catalog.json"
MODULE_INDEX = "docs/modules/README.md"
MATRIX = "docs/development/trnm-world-module-documentation-matrix-v1.md"
MAX_DOCUMENT_BYTES = 512 * 1024
MAX_CATALOG_BYTES = 256 * 1024
MAX_JSON_DEPTH = 64
MAX_JSON_NODES = 16_384
MIN_SECTION_CHARS = 160
REQUIRED_SECTIONS = (
    "Scope and authority",
    "Public interfaces",
    "State model and invariants",
    "Dependency direction",
    "Execution, concurrency and cancellation",
    "Persistence and durability",
    "Idempotency, retry and failure taxonomy",
    "Versioning and migration",
    "Resource and performance budgets",
    "Observability and security",
    "Test and evidence traceability",
    "Change checklist and open work",
)
REQUIRED_METADATA = {
    "status": "current-candidate",
    "implementation_conformance": "not-implied",
}
ALLOWED_CATALOG_STATUS = {"current", "current-candidate", "historical", "template"}
SPECIAL_STATUS_CLASS = {
    "historical": {"historical-execution-snapshot"},
    "template": {"evidence-template"},
}
REQUIRED_TRUTH_ORDER = (
    "PROJECT_BOUNDARY.md",
    "CURRENT_PLAN.md",
    "docs/catalog.json",
)
REQUIRED_CATALOG_PATHS = {
    "AGENTS.md",
    "PROJECT_BOUNDARY.md",
    "PROJECT_BOUNDARY.json",
    "CURRENT_PLAN.md",
    "README.md",
    "OPERATIONS.md",
    "docs/README.md",
    "docs/catalog.json",
    MODULE_INDEX,
    MATRIX,
    "docs/adr/0001-realtime-authority-and-match-evidence-ownership.md",
    "docs/adr/0002-transaction-free-external-settlement.md",
    "docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md",
    "docs/architecture/trnm-world-system-context-v1.md",
    "docs/database/trnm-world-postgres-contract-v1.md",
    "docs/security/trnm-world-threat-model-v1.md",
    "docs/development/trnm-world-testing-strategy-v2.md",
    "docs/release/trnm-world-release-gate-matrix-v2.md",
}
FORBIDDEN_OVERCLAIMS = (
    "production authorization granted",
    "public online enabled",
    "public player market enabled",
    "world owns wallet custody",
    "world owns chain finality",
    "world signs canonical matchcompletedv1",
    "world owns canonical online order",
)
MODULE_TERMS = {
    "trnm-economy-protocol": ("idempotency", "receipt", "cex", "custody"),
    "trnm-rpg-core": ("stable identifiers", "quest", "provenance", "content revision"),
    "trnm-campaign-core": ("town", "battlepending", "postbattlepending", "state preserving", "schema revision"),
    "trnm-rts-protocol": ("128 kib", "160 utf-8 bytes", "256 subjects", "nakama"),
    "trnm-rts-sim": ("10 ticks per second", "65,536", "state hash", "nakama"),
    "trnm-online-protocol": ("nakama", "full snapshot", "reconnect", "canonical online"),
    "trnm-game-server": ("world_legacy_local_alpha", "capture", "lookup-before-submit", "pitr"),
    "trnm-first-contact": ("bevy", "untrusted", "command journal", "human evidence"),
}


class DocumentationFailure(ValueError):
    pass


def safe_relative(relative: str) -> PurePosixPath:
    if not isinstance(relative, str) or not relative or relative != relative.strip():
        raise DocumentationFailure(f"invalid relative path: {relative!r}")
    if "\\" in relative or relative.startswith("/") or relative.startswith("./") or relative.endswith("/"):
        raise DocumentationFailure(f"unsafe relative path: {relative}")
    value = PurePosixPath(relative)
    if value.is_absolute() or value.as_posix() != relative or any(part in {"", ".", ".."} for part in value.parts):
        raise DocumentationFailure(f"unsafe relative path: {relative}")
    return value


def checked_file(root: Path, relative: str) -> Path:
    value = safe_relative(relative)
    root_real = root.resolve(strict=True)
    current = root_real
    for part in value.parts:
        current = current / part
        try:
            if current.is_symlink():
                raise DocumentationFailure(f"document path traverses a symlink: {relative}")
        except OSError as error:
            raise DocumentationFailure(f"cannot inspect document path: {relative}") from error
    try:
        resolved = current.resolve(strict=True)
        resolved.relative_to(root_real)
    except (OSError, ValueError) as error:
        raise DocumentationFailure(f"document path escapes or is missing: {relative}") from error
    if not resolved.is_file():
        raise DocumentationFailure(f"not a regular document file: {relative}")
    return resolved


def read_text(root: Path, relative: str) -> str:
    path = checked_file(root, relative)
    data = path.read_bytes()
    if len(data) > MAX_DOCUMENT_BYTES:
        raise DocumentationFailure(f"document exceeds {MAX_DOCUMENT_BYTES} bytes: {relative}")
    try:
        return data.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise DocumentationFailure(f"document is not strict UTF-8: {relative}") from error


def _reject_constant(value: str) -> None:
    raise DocumentationFailure(f"non-finite JSON constant is forbidden: {value}")


def _strict_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise DocumentationFailure(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def _json_budget(value: object, depth: int = 0, nodes: list[int] | None = None) -> None:
    if nodes is None:
        nodes = [0]
    nodes[0] += 1
    if nodes[0] > MAX_JSON_NODES:
        raise DocumentationFailure(f"JSON node budget exceeded: {MAX_JSON_NODES}")
    if depth > MAX_JSON_DEPTH:
        raise DocumentationFailure(f"JSON depth exceeded: {MAX_JSON_DEPTH}")
    if isinstance(value, dict):
        for key, child in value.items():
            if not isinstance(key, str):
                raise DocumentationFailure("JSON object key is not a string")
            _json_budget(child, depth + 1, nodes)
    elif isinstance(value, list):
        for child in value:
            _json_budget(child, depth + 1, nodes)


def load_strict_json(root: Path, relative: str) -> object:
    path = checked_file(root, relative)
    data = path.read_bytes()
    if len(data) > MAX_CATALOG_BYTES:
        raise DocumentationFailure(f"catalogue exceeds {MAX_CATALOG_BYTES} bytes")
    try:
        text = data.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise DocumentationFailure("catalogue is not strict UTF-8") from error
    try:
        value = json.loads(
            text,
            object_pairs_hook=_strict_object,
            parse_constant=_reject_constant,
        )
    except json.JSONDecodeError as error:
        raise DocumentationFailure(f"invalid catalogue JSON: {error}") from error
    _json_budget(value)
    return value


def visible_markdown(text: str) -> str:
    return re.sub(r"```.*?```", "", text, flags=re.DOTALL)


def parse_front_matter(text: str) -> dict[str, str]:
    if not text.startswith("---\n"):
        raise DocumentationFailure("missing front matter")
    end = text.find("\n---\n", 4)
    if end == -1:
        raise DocumentationFailure("unterminated front matter")
    result: dict[str, str] = {}
    for raw in text[4:end].splitlines():
        if not raw.strip() or raw.startswith(" ") or ":" not in raw:
            continue
        key, value = raw.split(":", 1)
        result[key.strip()] = value.strip()
    return result


def parse_sections(text: str) -> dict[str, str]:
    matches = list(re.finditer(r"(?m)^## ([^\n]+)\s*$", text))
    sections: dict[str, str] = {}
    for index, match in enumerate(matches):
        heading = match.group(1).strip()
        if heading in sections:
            raise DocumentationFailure(f"duplicate section: {heading}")
        start = match.end()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(text)
        sections[heading] = text[start:end]
    return sections


def parse_date(value: str, field: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except (TypeError, ValueError) as error:
        raise DocumentationFailure(f"invalid {field}: {value}") from error


def workspace_members(root: Path) -> list[tuple[str, str]]:
    workspace = tomllib.loads(read_text(root, "trillionnium/Cargo.toml")).get("workspace", {})
    members = workspace.get("members")
    if (
        not isinstance(members, list)
        or not members
        or len(members) > 128
        or not all(isinstance(item, str) for item in members)
        or len(members) != len(set(members))
    ):
        raise DocumentationFailure("workspace members must be a nonempty unique explicit list")
    result: list[tuple[str, str]] = []
    names: set[str] = set()
    for member in members:
        safe_relative(f"trillionnium/{member}/Cargo.toml")
        if any(marker in member for marker in "*?["):
            raise DocumentationFailure("Cargo member globs are not accepted")
        manifest = tomllib.loads(read_text(root, f"trillionnium/{member}/Cargo.toml"))
        name = manifest.get("package", {}).get("name")
        if not isinstance(name, str) or not re.fullmatch(r"[A-Za-z0-9_-]+", name) or name in names:
            raise DocumentationFailure(f"invalid or duplicate package identity: {name!r}")
        names.add(name)
        result.append((member, name))
    if names != set(MODULE_TERMS):
        missing = sorted(set(MODULE_TERMS) - names)
        extra = sorted(names - set(MODULE_TERMS))
        raise DocumentationFailure(f"module classification drift; missing={missing}, extra={extra}")
    return result


def validate_catalog(root: Path, as_of: dt.date) -> dict[str, dict[str, str]]:
    data = load_strict_json(root, CATALOG)
    if not isinstance(data, dict):
        raise DocumentationFailure("document catalogue root must be an object")
    if set(data) != {"schema", "as_of", "truth_order", "documents"}:
        raise DocumentationFailure("document catalogue root key set drift")
    if data.get("schema") != "trnm_world_document_catalog_v1":
        raise DocumentationFailure("document catalogue schema drift")
    parse_date(data.get("as_of", ""), "catalogue as_of")
    truth_order = data.get("truth_order")
    if (
        not isinstance(truth_order, list)
        or not truth_order
        or not all(isinstance(item, str) for item in truth_order)
        or len(truth_order) != len(set(truth_order))
        or tuple(truth_order[: len(REQUIRED_TRUTH_ORDER)]) != REQUIRED_TRUTH_ORDER
    ):
        raise DocumentationFailure("truth_order must begin with the binding boundary, current plan and catalogue")
    for relative in truth_order:
        safe_relative(relative)
    documents = data.get("documents")
    if not isinstance(documents, list) or not documents:
        raise DocumentationFailure("document catalogue is empty")
    by_path: dict[str, dict[str, str]] = {}
    required = {"path", "class", "owner", "status", "review_due"}
    for item in documents:
        if not isinstance(item, dict):
            raise DocumentationFailure("catalogue entry is not an object")
        if set(item) != required:
            raise DocumentationFailure(f"catalogue entry key set drift: {item!r}")
        relative = item["path"]
        if not isinstance(relative, str) or relative in by_path:
            raise DocumentationFailure(f"invalid or duplicate catalogue path: {relative!r}")
        safe_relative(relative)
        for key in ("class", "owner", "status", "review_due"):
            if not isinstance(item[key], str) or not item[key].strip():
                raise DocumentationFailure(f"invalid catalogue {key} for {relative}")
        status = item["status"]
        document_class = item["class"]
        if status not in ALLOWED_CATALOG_STATUS:
            raise DocumentationFailure(f"unsupported catalogue status for {relative}: {status}")
        expected_classes = SPECIAL_STATUS_CLASS.get(status)
        if expected_classes is not None:
            if document_class not in expected_classes:
                raise DocumentationFailure(
                    f"{relative}: status {status!r} requires class in {sorted(expected_classes)}"
                )
            if relative in truth_order:
                raise DocumentationFailure(f"{relative}: {status} material cannot enter truth_order")
        elif document_class in set().union(*SPECIAL_STATUS_CLASS.values()):
            raise DocumentationFailure(
                f"{relative}: special class {document_class!r} requires its matching status"
            )
        if status == "template" and not relative.startswith("docs/release/templates/"):
            raise DocumentationFailure(f"{relative}: evidence template is outside docs/release/templates")
        if parse_date(item["review_due"], f"review_due for {relative}") < as_of:
            raise DocumentationFailure(f"stale catalogue material: {relative}")
        read_text(root, relative)
        by_path[relative] = item
    if not set(truth_order).issubset(by_path):
        raise DocumentationFailure("truth_order contains unregistered paths")
    for relative in truth_order:
        if by_path[relative]["status"] not in {"current", "current-candidate"}:
            raise DocumentationFailure(f"truth_order contains non-current material: {relative}")
    missing = sorted(REQUIRED_CATALOG_PATHS - set(by_path))
    if missing:
        raise DocumentationFailure(f"required current documents are absent from catalogue: {missing}")
    return by_path


def validate(root: Path, as_of: dt.date) -> list[str]:
    root = root.resolve()
    catalogue = validate_catalog(root, as_of)
    members = workspace_members(root)
    index = visible_markdown(read_text(root, MODULE_INDEX))
    matrix = visible_markdown(read_text(root, MATRIX))
    matrix_names = re.findall(r"(?m)^\|\s*`([^`]+)`\s*\|", matrix)
    names = [name for _, name in members]
    if len(matrix_names) != len(set(matrix_names)) or set(matrix_names) != set(names):
        raise DocumentationFailure("module matrix identities do not exactly match Cargo package identities")

    expected_designs: set[str] = set()
    for member, name in members:
        readme_path = f"trillionnium/{member}/README.md"
        readme = visible_markdown(read_text(root, readme_path))
        expected_link = f"../../../docs/modules/{name}-design.md"
        if expected_link not in readme:
            raise DocumentationFailure(f"{readme_path}: missing detailed-design link {expected_link}")

        design_path = f"docs/modules/{name}-design.md"
        expected_designs.add(design_path)
        entry = catalogue.get(design_path)
        if not entry or entry.get("class") != "module-design":
            raise DocumentationFailure(f"{design_path}: missing module-design catalogue entry")
        text = read_text(root, design_path)
        metadata = parse_front_matter(text)
        if metadata.get("module") != name:
            raise DocumentationFailure(f"{design_path}: module metadata mismatch")
        for key, value in REQUIRED_METADATA.items():
            if metadata.get(key) != value:
                raise DocumentationFailure(f"{design_path}: {key} must be {value}")
        if not metadata.get("owner"):
            raise DocumentationFailure(f"{design_path}: owner is required")
        if parse_date(metadata.get("last_reviewed", ""), f"last_reviewed for {name}") > as_of:
            raise DocumentationFailure(f"{design_path}: last_reviewed is in the future")
        if parse_date(metadata.get("review_due", ""), f"review_due for {name}") < as_of:
            raise DocumentationFailure(f"{design_path}: review is stale")
        visible = visible_markdown(text)
        if not re.search(rf"(?m)^# {re.escape(name)} detailed design\s*$", visible):
            raise DocumentationFailure(f"{design_path}: title mismatch")
        sections = parse_sections(text)
        for heading in REQUIRED_SECTIONS:
            body = sections.get(heading)
            if body is None:
                raise DocumentationFailure(f"{design_path}: missing section {heading}")
            normalized_body = " ".join(body.split())
            if len(normalized_body) < MIN_SECTION_CHARS:
                raise DocumentationFailure(f"{design_path}: shallow section {heading}")
        normalized = " ".join(visible.lower().split())
        if any(marker in normalized for marker in FORBIDDEN_OVERCLAIMS):
            raise DocumentationFailure(f"{design_path}: prohibited authority/release assertion")
        for term in MODULE_TERMS[name]:
            if term.lower() not in normalized:
                raise DocumentationFailure(f"{design_path}: required module term is absent: {term}")
        trace = sections["Test and evidence traceability"].lower()
        if "trillionnium/" not in trace or "docs/" not in trace or not ("tests" in trace or "scripts/" in trace):
            raise DocumentationFailure(f"{design_path}: traceability must bind source, docs and tests/scripts")
        if name not in index or design_path.rsplit("/", 1)[-1] not in index:
            raise DocumentationFailure(f"{MODULE_INDEX}: missing discoverability for {name}")

    catalog_designs = {
        path for path, item in catalogue.items() if item.get("class") == "module-design"
    }
    if catalog_designs != expected_designs:
        raise DocumentationFailure(
            f"catalogued module-design set drift; expected={sorted(expected_designs)}, actual={sorted(catalog_designs)}"
        )
    return sorted(names)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=str(Path(__file__).resolve().parents[1]))
    parser.add_argument("--as-of", default=dt.date.today().isoformat())
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    root = Path(args.root)
    try:
        names = validate(root, parse_date(args.as_of, "--as-of"))
    except (DocumentationFailure, OSError, ValueError, tomllib.TOMLDecodeError) as error:
        print(f"TRNM World detailed documentation: FAIL: {error}", file=sys.stderr)
        return 1
    print(f"TRNM World detailed documentation: PASS ({len(names)}/8 active detailed designs)")
    print("Implementation conformance, hosted CI, governance, standalone components and release evidence remain separate.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
