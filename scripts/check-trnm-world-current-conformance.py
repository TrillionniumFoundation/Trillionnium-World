#!/usr/bin/env python3
"""Check high-risk current-document claims against the physical source tree.

This is intentionally narrower than implementation verification. It catches
known truth drift where a current document claims a removed generator, silently
activates excluded source, promotes fixture adapters, or describes an absent
production host runtime as present.
"""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
CURRENT_TEXT = [
    ROOT / "README.md",
    ROOT / "PROJECT_BOUNDARY.md",
    ROOT / "CURRENT_PLAN.md",
    ROOT / "RELEASE_READINESS.md",
    ROOT / "docs/README.md",
    ROOT / "docs/modules/README.md",
    ROOT / "docs/development/trnm-world-module-decomposition-v1.md",
    ROOT / "contracts/README.md",
    ROOT / "trillionnium/crates/platform/README.md",
    ROOT / "web4-frontend/README.md",
]


class ConformanceError(RuntimeError):
    pass


def read(path: Path) -> str:
    if not path.is_file():
        raise ConformanceError(f"required current source/document missing: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8", errors="strict")


def load_json(path: Path) -> dict[str, Any]:
    text = read(path)
    duplicates: list[str] = []

    def object_pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                duplicates.append(key)
            result[key] = value
        return result

    def reject_constant(value: str) -> None:
        raise ConformanceError(f"non-finite value in {path.relative_to(ROOT)}: {value}")

    try:
        value = json.loads(text, object_pairs_hook=object_pairs, parse_constant=reject_constant)
    except json.JSONDecodeError as exc:
        raise ConformanceError(f"invalid JSON in {path.relative_to(ROOT)}: {exc}") from exc
    if duplicates:
        raise ConformanceError(
            f"duplicate JSON keys in {path.relative_to(ROOT)}: {','.join(sorted(set(duplicates)))}"
        )
    if not isinstance(value, dict):
        raise ConformanceError(f"JSON root must be object: {path.relative_to(ROOT)}")
    return value


def require(text: str, pattern: str, label: str) -> None:
    if not re.search(pattern, text, re.IGNORECASE | re.MULTILINE):
        raise ConformanceError(f"missing required current boundary: {label}")


def forbid(text: str, pattern: str, label: str) -> None:
    if re.search(pattern, text, re.IGNORECASE | re.MULTILINE):
        raise ConformanceError(f"forbidden stale/current overclaim: {label}")


def main() -> int:
    try:
        texts = {path.relative_to(ROOT).as_posix(): read(path) for path in CURRENT_TEXT}
        combined = "\n".join(texts.values())

        # Removed semantic source generation must stay absent and truthfully described.
        removed = [
            ROOT / "trillionnium/crates/trnm-game-server/build.rs",
            ROOT / "trillionnium/crates/trnm-game-server/src/lib.rs.in",
        ]
        present = [path.relative_to(ROOT).as_posix() for path in removed if path.exists()]
        if present:
            raise ConformanceError("removed semantic-generation paths reappeared: " + ",".join(present))
        decomposition = texts["docs/development/trnm-world-module-decomposition-v1.md"]
        require(decomposition, r"semantic.*(?:build\.rs|lib\.rs\.in).*(?:retired|removed)", "semantic generator retired")
        forbid(
            decomposition,
            r"trnm-game-server\s+still\s+contains.*(?:build script|source template)",
            "decomposition says removed generator still exists",
        )
        require(decomposition, r"include!", "remaining include ownership debt")
        require(decomposition, r"true Rust module|real Rust module", "target real Rust modules")

        # Physical lifecycle inventory must match root boundaries.
        catalog = load_json(ROOT / "docs/component-catalog.json")
        if catalog.get("production_authorization") != "not_granted":
            raise ConformanceError("component catalogue grants production authorization")
        components = catalog.get("components")
        if not isinstance(components, list):
            raise ConformanceError("component catalogue components are missing")
        by_id = {
            item.get("id"): item
            for item in components
            if isinstance(item, dict) and isinstance(item.get("id"), str)
        }
        for expected in (
            "game-product-workspace",
            "world-authority-workspace",
            "legacy-platform-workspace",
            "external-contracts-workspace",
            "web4-frontend",
        ):
            if expected not in by_id:
                raise ConformanceError(f"component catalogue missing {expected}")
        legacy = by_id["legacy-platform-workspace"]
        if legacy.get("lifecycle") != "excluded-legacy" or legacy.get("release_denominator") != "none":
            raise ConformanceError("legacy platform lifecycle/denominator drift")
        web4 = by_id["web4-frontend"]
        if web4.get("release_denominator") != "none":
            raise ConformanceError("Web4 entered World release denominator")
        contracts = by_id["external-contracts-workspace"]
        if contracts.get("lifecycle") != "mvp-perimeter" or contracts.get("release_denominator") != "scope-dependent":
            raise ConformanceError("external-contract MVP lifecycle/denominator drift")

        root_readme = texts["README.md"]
        project_boundary = texts["PROJECT_BOUNDARY.md"]
        platform_readme = texts["trillionnium/crates/platform/README.md"]
        web4_readme = texts["web4-frontend/README.md"]
        contract_readme = texts["contracts/README.md"]
        for source, label in (
            (root_readme, "root README"),
            (project_boundary, "project boundary"),
            (platform_readme, "platform README"),
        ):
            require(source, r"platform", f"{label} identifies platform")
            require(source, r"excluded|legacy|not an active", f"{label} excludes legacy platform")
        require(web4_readme, r"release denominator.*none|world_release_denominator=none", "Web4 denominator none")
        require(contract_readme, r"MVP", "contracts MVP boundary")
        require(contract_readme, r"not.*production|production.*not", "contracts non-production boundary")
        require(contract_readme, r"canonical host.*(?:not|false)|not connected", "contracts host runtime absent")

        # Authority and release claims must remain fail-closed.
        for path in (
            "README.md",
            "PROJECT_BOUNDARY.md",
            "CURRENT_PLAN.md",
            "RELEASE_READINESS.md",
            "contracts/README.md",
            "trillionnium/crates/platform/README.md",
            "web4-frontend/README.md",
        ):
            text = texts[path]
            forbid(text, r"production_authorization\s*[:=]\s*granted", f"{path} production authorization")
        require(project_boundary, r"Nakama.*canonical online", "Nakama canonical online authority")
        require(project_boundary, r"CEX.*wallet/ledger.*custody", "CEX custody authority")
        require(project_boundary, r"public online.*NO-GO|public online.*remain.*disabled", "public online disabled")

        # Current docs may mention historical path drift only as a warning, never as live paths.
        for path, text in texts.items():
            if path == "contracts/README.md":
                continue
            forbid(text, r"(?:^|[ `(])trillionnium-rust/", f"{path} uses historical trillionnium-rust path")
            forbid(text, r"(?:^|[ `(])contracts-rust/", f"{path} uses historical contracts-rust path")

        # Current document catalogue itself retains the established truth hierarchy.
        document_catalog = load_json(ROOT / "docs/catalog.json")
        expected_order = [
            "PROJECT_BOUNDARY.md",
            "CURRENT_PLAN.md",
            "docs/catalog.json",
            "docs/status/world-plan-v4-execution-truth-2026-09-08.json",
            "docs/status/CURRENT.md",
            "RELEASE_READINESS.md",
            "docs/development/trnm-world-gap-closure-ledger-v6.json",
            "docs/integration/trnm-world-cex-current-pending-lock-v2.json",
        ]
        if document_catalog.get("truth_order") != expected_order:
            raise ConformanceError("canonical document truth_order drift")
    except (ConformanceError, OSError, UnicodeError) as exc:
        print(f"TRNM_WORLD_CURRENT_CONFORMANCE=FAIL reason={exc}", file=sys.stderr)
        return 1

    print(
        "TRNM_WORLD_CURRENT_CONFORMANCE=PASS "
        "semantic_generator=absent platform=excluded-legacy "
        "contracts=mvp-perimeter web4_denominator=none "
        "production_authorization=not_granted"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
