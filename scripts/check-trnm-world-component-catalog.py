#!/usr/bin/env python3
"""Validate the complete physical Trillionnium World component catalogue.

The catalogue is deliberately broader than the active release denominator. Every
Rust or Node manifest physically present in the current repository must be
classified as active, candidate, compatibility, legacy, or scope-dependent.
"""

from __future__ import annotations

import json
from pathlib import Path
import sys
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_CATALOG = ROOT / "docs/component-catalog.json"
MAX_BYTES = 1_048_576
MAX_DEPTH = 32
MAX_NODES = 20_000


class CatalogError(RuntimeError):
    pass


def _reject_constant(value: str) -> None:
    raise CatalogError(f"non-finite JSON constant is forbidden: {value}")


def _object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise CatalogError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def _budget(value: Any, depth: int = 0, count: list[int] | None = None) -> None:
    if count is None:
        count = [0]
    count[0] += 1
    if count[0] > MAX_NODES:
        raise CatalogError(f"JSON node budget exceeded: {MAX_NODES}")
    if depth > MAX_DEPTH:
        raise CatalogError(f"JSON depth exceeded: {MAX_DEPTH}")
    if isinstance(value, dict):
        for key, child in value.items():
            if not isinstance(key, str):
                raise CatalogError("JSON object key is not a string")
            _budget(child, depth + 1, count)
    elif isinstance(value, list):
        for child in value:
            _budget(child, depth + 1, count)


def load_catalog(path: Path) -> dict[str, Any]:
    data = path.read_bytes()
    if len(data) > MAX_BYTES:
        raise CatalogError(f"catalog exceeds {MAX_BYTES} bytes")
    try:
        text = data.decode("utf-8", errors="strict")
    except UnicodeDecodeError as exc:
        raise CatalogError(f"catalog is not strict UTF-8: {exc}") from exc
    try:
        value = json.loads(
            text,
            object_pairs_hook=_object,
            parse_constant=_reject_constant,
        )
    except (json.JSONDecodeError, CatalogError) as exc:
        raise CatalogError(f"invalid catalog JSON: {exc}") from exc
    _budget(value)
    if not isinstance(value, dict):
        raise CatalogError("catalog root must be an object")
    return value


def require_string(item: dict[str, Any], field: str, component_id: str) -> str:
    value = item.get(field)
    if not isinstance(value, str) or not value.strip():
        raise CatalogError(f"{component_id}: {field} must be a nonblank string")
    if len(value.encode("utf-8")) > 512:
        raise CatalogError(f"{component_id}: {field} exceeds 512 UTF-8 bytes")
    return value


def repository_manifests() -> set[str]:
    manifests: set[str] = set()
    for root_name in ("trillionnium", "contracts"):
        root = ROOT / root_name
        if not root.is_dir():
            raise CatalogError(f"required component root is missing: {root_name}")
        for path in root.rglob("Cargo.toml"):
            if any(part in {"target", ".git"} for part in path.parts):
                continue
            manifests.add(path.relative_to(ROOT).as_posix())
    web_manifest = ROOT / "web4-frontend/package.json"
    if web_manifest.is_file():
        manifests.add(web_manifest.relative_to(ROOT).as_posix())
    return manifests


def validate(catalog: dict[str, Any]) -> None:
    if catalog.get("schema") != "trnm_world_component_catalog_v1":
        raise CatalogError("unexpected component catalog schema")
    if catalog.get("repository") != "TrillionniumFoundation/Trillionnium-World":
        raise CatalogError("catalog repository identity mismatch")
    if catalog.get("production_authorization") != "not_granted":
        raise CatalogError("component catalogue cannot grant production authorization")

    components = catalog.get("components")
    if not isinstance(components, list) or not components:
        raise CatalogError("components must be a nonempty array")
    if len(components) > 256:
        raise CatalogError("component count exceeds 256")

    ids: set[str] = set()
    paths: set[str] = set()
    manifests: set[str] = set()
    allowed_lifecycles = {
        "active",
        "active-compatibility",
        "compatibility-laboratory",
        "cutover-candidate",
        "fixture-candidate",
        "excluded-legacy",
        "mvp-perimeter",
        "historical-compatible-subproject",
    }
    active_denominators = {"game-product", "world-authority-candidate"}

    for raw in components:
        if not isinstance(raw, dict):
            raise CatalogError("each component must be an object")
        component_id = require_string(raw, "id", "<unknown>")
        path_text = require_string(raw, "path", component_id)
        manifest_text = require_string(raw, "manifest", component_id)
        kind = require_string(raw, "kind", component_id)
        lifecycle = require_string(raw, "lifecycle", component_id)
        denominator = require_string(raw, "release_denominator", component_id)
        require_string(raw, "owner", component_id)
        require_string(raw, "gate", component_id)

        if component_id in ids:
            raise CatalogError(f"duplicate component id: {component_id}")
        if path_text in paths:
            raise CatalogError(f"duplicate component path: {path_text}")
        if manifest_text in manifests:
            raise CatalogError(f"duplicate component manifest: {manifest_text}")
        ids.add(component_id)
        paths.add(path_text)
        manifests.add(manifest_text)

        if lifecycle not in allowed_lifecycles:
            raise CatalogError(f"{component_id}: unsupported lifecycle {lifecycle!r}")
        if kind not in {"rust-workspace", "rust-crate", "node-application"}:
            raise CatalogError(f"{component_id}: unsupported component kind {kind!r}")
        if lifecycle == "excluded-legacy" and denominator != "none":
            raise CatalogError(f"{component_id}: excluded legacy must have denominator none")
        if lifecycle in {"active", "active-compatibility", "compatibility-laboratory"} and denominator != "game-product":
            raise CatalogError(f"{component_id}: active game component has wrong denominator")
        if lifecycle in {"cutover-candidate", "fixture-candidate"} and denominator != "world-authority-candidate":
            raise CatalogError(f"{component_id}: World authority candidate has wrong denominator")
        if denominator in active_denominators and lifecycle in {"excluded-legacy", "historical-compatible-subproject"}:
            raise CatalogError(f"{component_id}: historical component entered an active denominator")

        component_path = ROOT / path_text
        manifest_path = ROOT / manifest_text
        if not component_path.is_dir():
            raise CatalogError(f"{component_id}: component path missing: {path_text}")
        if not manifest_path.is_file():
            raise CatalogError(f"{component_id}: manifest missing: {manifest_text}")

        docs = raw.get("canonical_docs")
        if not isinstance(docs, list) or not docs:
            raise CatalogError(f"{component_id}: canonical_docs must be nonempty")
        if len(docs) > 16:
            raise CatalogError(f"{component_id}: too many canonical docs")
        seen_docs: set[str] = set()
        for doc in docs:
            if not isinstance(doc, str) or not doc.strip():
                raise CatalogError(f"{component_id}: invalid canonical doc")
            if doc in seen_docs:
                raise CatalogError(f"{component_id}: duplicate canonical doc {doc}")
            seen_docs.add(doc)
            if not (ROOT / doc).is_file():
                raise CatalogError(f"{component_id}: canonical doc missing: {doc}")

        if denominator in active_denominators and len(docs) < 2:
            raise CatalogError(f"{component_id}: active/candidate component needs contract plus design/index")

    discovered = repository_manifests()
    missing = sorted(discovered - manifests)
    stale = sorted(manifests - discovered)
    if missing:
        raise CatalogError("unclassified repository manifests: " + ", ".join(missing))
    if stale:
        raise CatalogError("catalog manifests absent from repository: " + ", ".join(stale))

    expected_game = {
        "trnm-economy-protocol",
        "trnm-rpg-core",
        "trnm-campaign-core",
        "trnm-rts-protocol",
        "trnm-rts-sim",
        "trnm-online-protocol",
        "trnm-game-server",
        "trnm-first-contact",
    }
    actual_game = {
        item["id"]
        for item in components
        if isinstance(item, dict) and item.get("kind") == "rust-crate" and item.get("release_denominator") == "game-product"
    }
    if actual_game != expected_game:
        raise CatalogError(f"game-product crate denominator mismatch: {sorted(actual_game)}")

    expected_authority = {
        "trnm-world-domain",
        "trnm-world-command",
        "trnm-world-projection",
        "trnm-world-map-provider",
        "trnm-world-ui-fragments",
        "trnm-world-api",
        "trnm-world-server",
    }
    actual_authority = {
        item["id"]
        for item in components
        if isinstance(item, dict) and item.get("kind") == "rust-crate" and item.get("release_denominator") == "world-authority-candidate"
    }
    if actual_authority != expected_authority:
        raise CatalogError(f"World authority crate denominator mismatch: {sorted(actual_authority)}")


def main() -> int:
    catalog_path = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else DEFAULT_CATALOG
    if not catalog_path.is_file():
        print(f"TRNM_WORLD_COMPONENT_CATALOG=FAIL missing={catalog_path}", file=sys.stderr)
        return 1
    try:
        catalog = load_catalog(catalog_path)
        validate(catalog)
    except (CatalogError, OSError) as exc:
        print(f"TRNM_WORLD_COMPONENT_CATALOG=FAIL reason={exc}", file=sys.stderr)
        return 1
    print(
        "TRNM_WORLD_COMPONENT_CATALOG=PASS "
        f"components={len(catalog['components'])} manifests={len(repository_manifests())} "
        "production_authorization=not_granted"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
