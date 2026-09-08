#!/usr/bin/env python3
"""Validate the maintained Rust external-contract MVP design surfaces."""

from __future__ import annotations

from datetime import date
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
INDEX = ROOT / "docs/modules/contracts/README.md"
DESIGNS = {
    "audit-events": ROOT / "docs/modules/contracts/audit-events-design.md",
    "settlement-vault": ROOT / "docs/modules/contracts/settlement-vault-design.md",
    "bridge-relay": ROOT / "docs/modules/contracts/bridge-relay-design.md",
    "governance-guard": ROOT / "docs/modules/contracts/governance-guard-design.md",
}
SOURCES = {
    name: ROOT / "contracts" / name / "src" for name in DESIGNS
}
REQUIRED_HEADINGS = [
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
]
FORBIDDEN_OVERCLAIMS = [
    r"production_authorization:\s*granted",
    r"public[- ]mainnet[- ]ready",
    r"production[- ]ready",
    r"canonical host runtime (?:is )?(?:complete|connected)",
    r"custody (?:is )?(?:complete|approved)",
]


class DocumentationError(RuntimeError):
    pass


def frontmatter(text: str, path: Path) -> dict[str, str]:
    if not text.startswith("---\n"):
        raise DocumentationError(f"{path}: missing YAML frontmatter")
    end = text.find("\n---\n", 4)
    if end < 0:
        raise DocumentationError(f"{path}: unterminated YAML frontmatter")
    result: dict[str, str] = {}
    for line in text[4:end].splitlines():
        if not line.strip() or line.startswith(" "):
            continue
        if ":" not in line:
            raise DocumentationError(f"{path}: malformed frontmatter line {line!r}")
        key, value = line.split(":", 1)
        key = key.strip()
        value = value.strip()
        if key in result:
            raise DocumentationError(f"{path}: duplicate frontmatter key {key}")
        result[key] = value
    return result


def validate_design(name: str, path: Path, as_of: date) -> None:
    if not path.is_file():
        raise DocumentationError(f"missing design: {path.relative_to(ROOT)}")
    text = path.read_text(encoding="utf-8", errors="strict")
    if len(text.encode("utf-8")) < 4_000:
        raise DocumentationError(f"{path}: design is too shallow")
    metadata = frontmatter(text, path)
    if metadata.get("status") != "current-candidate":
        raise DocumentationError(f"{path}: status must be current-candidate")
    if metadata.get("owner") != "trillionnium-world-contracts":
        raise DocumentationError(f"{path}: owner mismatch")
    if metadata.get("module") != name:
        raise DocumentationError(f"{path}: module mismatch")
    if metadata.get("implementation_conformance") != "not-implied":
        raise DocumentationError(f"{path}: implementation_conformance must be not-implied")
    try:
        reviewed = date.fromisoformat(metadata["last_reviewed"])
        due = date.fromisoformat(metadata["review_due"])
    except (KeyError, ValueError) as exc:
        raise DocumentationError(f"{path}: invalid review metadata") from exc
    if reviewed > as_of:
        raise DocumentationError(f"{path}: last_reviewed is in the future")
    if due < as_of:
        raise DocumentationError(f"{path}: review is expired")
    if due < reviewed:
        raise DocumentationError(f"{path}: review_due precedes last_reviewed")

    for heading in REQUIRED_HEADINGS:
        if not re.search(rf"^##\s+{re.escape(heading)}\s*$", text, re.MULTILINE):
            raise DocumentationError(f"{path}: missing heading {heading!r}")
    if f"contracts/{name}/src/" not in text:
        raise DocumentationError(f"{path}: missing primary-source traceability")
    if "Open work" not in text and "open work" not in text:
        raise DocumentationError(f"{path}: missing explicit open work")
    if "not" not in text.lower() or "production" not in text.lower():
        raise DocumentationError(f"{path}: missing honest non-production boundary")
    if "audit" not in text.lower():
        raise DocumentationError(f"{path}: missing audit/evidence boundary")
    if not SOURCES[name].is_dir() or not any(SOURCES[name].glob("*.rs")):
        raise DocumentationError(f"{path}: primary source directory is missing or empty")
    lowered = text.lower()
    for pattern in FORBIDDEN_OVERCLAIMS:
        if re.search(pattern, lowered):
            raise DocumentationError(f"{path}: forbidden production overclaim matching {pattern!r}")


def main() -> int:
    as_of = date.today()
    if len(sys.argv) == 3 and sys.argv[1] == "--as-of":
        as_of = date.fromisoformat(sys.argv[2])
    elif len(sys.argv) != 1:
        print("usage: check-trnm-world-contract-module-documentation.py [--as-of YYYY-MM-DD]", file=sys.stderr)
        return 2
    try:
        if not INDEX.is_file():
            raise DocumentationError("contract module index is missing")
        index_text = INDEX.read_text(encoding="utf-8", errors="strict")
        if "production authorization" not in index_text.lower() and "production" not in index_text.lower():
            raise DocumentationError("contract index lacks non-production boundary")
        for name, path in DESIGNS.items():
            if name not in index_text or path.name not in index_text:
                raise DocumentationError(f"contract index does not link {name}")
            validate_design(name, path, as_of)
    except (DocumentationError, OSError, UnicodeError) as exc:
        print(f"TRNM_WORLD_CONTRACT_MODULE_DOCUMENTATION=FAIL reason={exc}", file=sys.stderr)
        return 1
    print(
        "TRNM_WORLD_CONTRACT_MODULE_DOCUMENTATION=PASS "
        f"modules={len(DESIGNS)} implementation_conformance=not-implied "
        "production_authorization=not_granted"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
