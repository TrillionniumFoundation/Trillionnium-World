#!/usr/bin/env python3
"""Validate the complete maintained Rust external-contract documentation closure."""

from __future__ import annotations

import argparse
import datetime as dt
from pathlib import Path, PurePosixPath
import re
import sys
import tomllib

from trnm_world_strict_json import load_strict_json

ROOT = Path(__file__).resolve().parents[1]
WORKSPACE_MANIFEST = "contracts/Cargo.toml"
INDEX_PATH = "docs/modules/contracts/README.md"
DOCUMENT_CATALOG = "docs/catalog.json"
COMPONENT_CATALOG = "docs/component-catalog.json"
MAX_DOCUMENT_BYTES = 512 * 1024
MIN_SECTION_CHARS = 160

REQUIRED_HEADINGS = (
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
    "owner": "trillionnium-world-contracts",
    "implementation_conformance": "not-implied",
}
FORBIDDEN_OVERCLAIMS = (
    r"production_authorization:\s*granted",
    r"public[- ]mainnet[- ]ready",
    r"production[- ]ready",
    r"canonical host runtime (?:is )?(?:complete|connected)",
    r"custody (?:is )?(?:complete|approved)",
)
REQUIRED_TERMS = {
    "audit-events": ("immutable", "idempotency", "privacy", "audit"),
    "bridge-relay": ("finality", "nonce", "reorganization", "audit"),
    "governance-guard": ("timelock", "pause", "version", "audit"),
    "settlement-vault": ("custody", "idempotency", "accounting", "audit"),
}


class DocumentationError(RuntimeError):
    pass


def safe_relative(value: str) -> PurePosixPath:
    if (
        not isinstance(value, str)
        or not value
        or value != value.strip()
        or value.startswith("/")
        or value.startswith("./")
        or value.endswith("/")
        or "\\" in value
    ):
        raise DocumentationError(f"unsafe relative path: {value!r}")
    relative = PurePosixPath(value)
    if (
        relative.is_absolute()
        or relative.as_posix() != value
        or any(part in {"", ".", ".."} for part in relative.parts)
    ):
        raise DocumentationError(f"unsafe relative path: {value!r}")
    return relative


def checked_file(root: Path, relative: str) -> Path:
    relative_path = safe_relative(relative)
    root_real = root.resolve(strict=True)
    current = root_real
    for part in relative_path.parts:
        current = current / part
        if current.is_symlink():
            raise DocumentationError(f"path traverses symlink: {relative}")
    try:
        resolved = current.resolve(strict=True)
        resolved.relative_to(root_real)
    except (OSError, ValueError) as exc:
        raise DocumentationError(f"missing or escaping path: {relative}") from exc
    if not resolved.is_file():
        raise DocumentationError(f"not a regular file: {relative}")
    return resolved


def read_text(root: Path, relative: str) -> str:
    path = checked_file(root, relative)
    data = path.read_bytes()
    if len(data) > MAX_DOCUMENT_BYTES:
        raise DocumentationError(f"document exceeds byte budget: {relative}")
    try:
        return data.decode("utf-8", errors="strict")
    except UnicodeDecodeError as exc:
        raise DocumentationError(f"document is not strict UTF-8: {relative}") from exc


def frontmatter(text: str, relative: str) -> dict[str, str]:
    if not text.startswith("---\n"):
        raise DocumentationError(f"{relative}: missing YAML frontmatter")
    end = text.find("\n---\n", 4)
    if end < 0:
        raise DocumentationError(f"{relative}: unterminated YAML frontmatter")
    result: dict[str, str] = {}
    for line in text[4:end].splitlines():
        if not line.strip() or line.startswith(" "):
            continue
        if ":" not in line:
            raise DocumentationError(f"{relative}: malformed frontmatter line {line!r}")
        key, value = line.split(":", 1)
        key = key.strip()
        value = value.strip()
        if key in result:
            raise DocumentationError(f"{relative}: duplicate frontmatter key {key}")
        result[key] = value
    return result


def sections(text: str, relative: str) -> dict[str, str]:
    matches = list(re.finditer(r"(?m)^## ([^\n]+)\s*$", text))
    result: dict[str, str] = {}
    for index, match in enumerate(matches):
        heading = match.group(1).strip()
        if heading in result:
            raise DocumentationError(f"{relative}: duplicate section {heading}")
        start = match.end()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(text)
        result[heading] = text[start:end]
    return result


def parse_date(value: str, field: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except (TypeError, ValueError) as exc:
        raise DocumentationError(f"invalid {field}: {value!r}") from exc


def workspace_members(root: Path) -> list[str]:
    manifest = tomllib.loads(read_text(root, WORKSPACE_MANIFEST))
    members = manifest.get("workspace", {}).get("members")
    if (
        not isinstance(members, list)
        or not members
        or len(members) > 32
        or not all(isinstance(item, str) for item in members)
        or len(members) != len(set(members))
    ):
        raise DocumentationError(
            "contracts workspace members must be a nonempty unique explicit list"
        )

    names: list[str] = []
    for member in members:
        safe_relative(member)
        if any(marker in member for marker in "*?["):
            raise DocumentationError(f"workspace globs are forbidden: {member}")
        manifest_path = f"contracts/{member}/Cargo.toml"
        package = tomllib.loads(read_text(root, manifest_path)).get("package", {}).get("name")
        if package != member:
            raise DocumentationError(
                f"package identity mismatch: member={member!r} package={package!r}"
            )
        names.append(member)

    if set(names) != set(REQUIRED_TERMS):
        raise DocumentationError(
            "external-contract module classification drift; "
            f"expected={sorted(REQUIRED_TERMS)} actual={sorted(names)}"
        )
    return sorted(names)


def catalogue_entries(root: Path, relative: str) -> dict[str, dict[str, object]]:
    value = load_strict_json(checked_file(root, relative))
    key = "documents" if relative == DOCUMENT_CATALOG else "components"
    if not isinstance(value, dict) or not isinstance(value.get(key), list):
        raise DocumentationError(f"{relative}: invalid catalogue root")
    entries: dict[str, dict[str, object]] = {}
    for item in value[key]:
        if not isinstance(item, dict):
            raise DocumentationError(f"{relative}: non-object catalogue entry")
        path = item.get("path")
        if not isinstance(path, str) or path in entries:
            raise DocumentationError(f"{relative}: invalid or duplicate path {path!r}")
        safe_relative(path)
        entries[path] = item
    return entries


def validate_index(root: Path, names: list[str], as_of: dt.date) -> None:
    text = read_text(root, INDEX_PATH)
    metadata = frontmatter(text, INDEX_PATH)
    for key, value in REQUIRED_METADATA.items():
        if metadata.get(key) != value:
            raise DocumentationError(f"{INDEX_PATH}: {key} must be {value}")
    if parse_date(metadata.get("last_reviewed", ""), "index last_reviewed") > as_of:
        raise DocumentationError(f"{INDEX_PATH}: last_reviewed is in the future")
    if parse_date(metadata.get("review_due", ""), "index review_due") < as_of:
        raise DocumentationError(f"{INDEX_PATH}: review is expired")
    for name in names:
        design_name = f"{name}-design.md"
        if name not in text or design_name not in text:
            raise DocumentationError(f"{INDEX_PATH}: missing discoverability for {name}")


def validate_local_contract(root: Path, name: str) -> None:
    relative = f"contracts/{name}/README.md"
    text = read_text(root, relative)
    normalized = " ".join(text.lower().split())
    expected_link = f"../../docs/modules/contracts/{name}-design.md"
    if expected_link not in text:
        raise DocumentationError(f"{relative}: missing detailed-design link {expected_link}")
    for heading in (
        "Purpose",
        "Authority and non-goals",
        "Public contract",
        "State and invariants",
        "Host and durability boundary",
        "Verification",
    ):
        if not re.search(rf"(?m)^## {re.escape(heading)}\s*$", text):
            raise DocumentationError(f"{relative}: missing local contract section {heading}")
    if len(normalized) < 900:
        raise DocumentationError(f"{relative}: local contract is too shallow")
    if "production" not in normalized or "not" not in normalized:
        raise DocumentationError(f"{relative}: missing fail-closed non-production boundary")


def validate_design(
    root: Path,
    name: str,
    as_of: dt.date,
    document_catalogue: dict[str, dict[str, object]],
    component_catalogue: dict[str, dict[str, object]],
) -> None:
    relative = f"docs/modules/contracts/{name}-design.md"
    text = read_text(root, relative)
    metadata = frontmatter(text, relative)
    for key, value in REQUIRED_METADATA.items():
        if metadata.get(key) != value:
            raise DocumentationError(f"{relative}: {key} must be {value}")
    if metadata.get("module") != name:
        raise DocumentationError(f"{relative}: module mismatch")
    reviewed = parse_date(metadata.get("last_reviewed", ""), f"{relative} last_reviewed")
    due = parse_date(metadata.get("review_due", ""), f"{relative} review_due")
    if reviewed > as_of:
        raise DocumentationError(f"{relative}: last_reviewed is in the future")
    if due < as_of:
        raise DocumentationError(f"{relative}: review is expired")
    if due < reviewed:
        raise DocumentationError(f"{relative}: review_due precedes last_reviewed")

    parsed_sections = sections(text, relative)
    for heading in REQUIRED_HEADINGS:
        body = parsed_sections.get(heading)
        if body is None:
            raise DocumentationError(f"{relative}: missing section {heading}")
        if len(" ".join(body.split())) < MIN_SECTION_CHARS:
            raise DocumentationError(f"{relative}: shallow section {heading}")

    normalized = " ".join(text.lower().split())
    for pattern in FORBIDDEN_OVERCLAIMS:
        if re.search(pattern, normalized):
            raise DocumentationError(
                f"{relative}: forbidden production overclaim matching {pattern!r}"
            )
    for term in REQUIRED_TERMS[name]:
        if term not in normalized:
            raise DocumentationError(f"{relative}: required term is absent: {term}")

    trace = parsed_sections["Test and evidence traceability"].lower()
    if (
        f"contracts/{name}/src/" not in trace
        or "docs/" not in trace
        or ("tests" not in trace and "scripts/" not in trace)
    ):
        raise DocumentationError(
            f"{relative}: traceability must bind source, documentation and tests/scripts"
        )

    source_root = root / "contracts" / name / "src"
    if not source_root.is_dir() or not any(source_root.glob("*.rs")):
        raise DocumentationError(f"{relative}: primary source directory is missing or empty")

    document_entry = document_catalogue.get(relative)
    if (
        document_entry is None
        or document_entry.get("class") != "contract-module-design"
        or document_entry.get("owner") != "trillionnium-world-contracts"
        or document_entry.get("status") != "current-candidate"
    ):
        raise DocumentationError(f"{relative}: document catalogue entry is absent or inconsistent")
    if parse_date(
        str(document_entry.get("review_due", "")),
        f"{relative} catalogue review_due",
    ) != due:
        raise DocumentationError(f"{relative}: catalogue/frontmatter review_due drift")

    component = component_catalogue.get(f"contracts/{name}")
    if (
        component is None
        or component.get("lifecycle") != "mvp-perimeter"
        or component.get("release_denominator") != "scope-dependent"
        or component.get("owner") != "trillionnium-world-contracts"
    ):
        raise DocumentationError(f"{relative}: component catalogue lifecycle/owner drift")
    canonical_docs = component.get("canonical_docs")
    expected_docs = [f"contracts/{name}/README.md", relative]
    if canonical_docs != expected_docs:
        raise DocumentationError(
            f"{relative}: component catalogue canonical_docs drift; "
            f"expected={expected_docs!r} actual={canonical_docs!r}"
        )


def validate(root: Path, as_of: dt.date) -> list[str]:
    root = root.resolve()
    names = workspace_members(root)
    document_catalogue = catalogue_entries(root, DOCUMENT_CATALOG)
    component_catalogue = catalogue_entries(root, COMPONENT_CATALOG)
    validate_index(root, names, as_of)

    index_entry = document_catalogue.get(INDEX_PATH)
    if (
        index_entry is None
        or index_entry.get("class") != "contract-module-index"
        or index_entry.get("owner") != "trillionnium-world-contracts"
    ):
        raise DocumentationError(
            f"{INDEX_PATH}: document catalogue entry is absent or inconsistent"
        )

    for name in names:
        local_readme = f"contracts/{name}/README.md"
        local_entry = document_catalogue.get(local_readme)
        if (
            local_entry is None
            or local_entry.get("class") != "contract-module-contract"
            or local_entry.get("owner") != "trillionnium-world-contracts"
        ):
            raise DocumentationError(
                f"{local_readme}: document catalogue entry is absent or inconsistent"
            )
        validate_local_contract(root, name)
        validate_design(
            root,
            name,
            as_of,
            document_catalogue,
            component_catalogue,
        )
    return names


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=str(ROOT))
    parser.add_argument("--as-of", default=dt.date.today().isoformat())
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    try:
        names = validate(Path(args.root), parse_date(args.as_of, "--as-of"))
    except (
        DocumentationError,
        OSError,
        UnicodeError,
        ValueError,
        tomllib.TOMLDecodeError,
    ) as exc:
        print(
            f"TRNM_WORLD_CONTRACT_MODULE_DOCUMENTATION=FAIL reason={exc}",
            file=sys.stderr,
        )
        return 1
    print(
        "TRNM_WORLD_CONTRACT_MODULE_DOCUMENTATION=PASS "
        f"modules={len(names)}/4 implementation_conformance=not-implied "
        "production_authorization=not_granted"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
