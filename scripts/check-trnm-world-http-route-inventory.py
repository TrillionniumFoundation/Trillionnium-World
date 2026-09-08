#!/usr/bin/env python3
"""Bind the documented World compatibility HTTP route inventory to source."""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import re
import sys

INVENTORY = "docs/protocol/trnm-world-http-route-inventory-v1.json"
SOURCE = "trillionnium/crates/trnm-game-server/src/lib_parts/http_routing/part_01.rs"
DOC = "docs/protocol/trnm-world-http-api-v1.md"
SCHEMAS = (
    "docs/protocol/schemas/trnm-world-online-stream-v1.schema.json",
    "docs/protocol/schemas/trnm-world-error-envelope-v1.schema.json",
)


class RouteFailure(ValueError):
    pass


def load_text(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file() or path.is_symlink():
        raise RouteFailure(f"missing or unsafe file: {relative}")
    text = path.read_text(encoding="utf-8")
    if not text.strip():
        raise RouteFailure(f"empty file: {relative}")
    return text


def source_routes(text: str) -> set[tuple[str, str, str]]:
    pattern = re.compile(
        r"\.route\(\s*\"([^\"]+)\"\s*,\s*(get|post)\(\s*([A-Za-z0-9_:]+)\s*\)\s*\)",
        re.S,
    )
    routes = {(method.upper(), path, handler) for path, method, handler in pattern.findall(text)}
    if not routes:
        raise RouteFailure("no routes parsed from build_router")
    if len(routes) != len(pattern.findall(text)):
        raise RouteFailure("duplicate route tuples in source")
    return routes


def validate(root: Path) -> int:
    root = root.resolve()
    source = load_text(root, SOURCE)
    try:
        inventory = json.loads(load_text(root, INVENTORY))
    except json.JSONDecodeError as error:
        raise RouteFailure(f"invalid route inventory JSON: {error}") from error
    if inventory.get("schema") != "trnm_world_http_route_inventory_v1":
        raise RouteFailure("unexpected route inventory schema")
    if inventory.get("profile") != "world_legacy_local_alpha":
        raise RouteFailure("route inventory must remain compatibility-only")
    if inventory.get("canonical_public_authority") is not False:
        raise RouteFailure("route inventory cannot claim canonical public authority")
    if inventory.get("source") != SOURCE:
        raise RouteFailure("route inventory source path drift")
    rows = inventory.get("routes")
    if not isinstance(rows, list) or not rows:
        raise RouteFailure("route inventory is empty")
    documented: set[tuple[str, str, str]] = set()
    for row in rows:
        if not isinstance(row, dict):
            raise RouteFailure("route row must be an object")
        item = (row.get("method"), row.get("path"), row.get("handler"))
        if not all(isinstance(value, str) and value for value in item):
            raise RouteFailure("route row has invalid method/path/handler")
        if item[0] not in {"GET", "POST"}:
            raise RouteFailure(f"unsupported documented method: {item[0]}")
        if row.get("surface") not in {"health", "online", "product", "operations", "production"}:
            raise RouteFailure(f"invalid route surface: {row.get('surface')}")
        if row.get("canonical_authority") is not False:
            raise RouteFailure("compatibility route cannot claim canonical authority")
        if item in documented:
            raise RouteFailure(f"duplicate documented route: {item}")
        documented.add(item)
    actual = source_routes(source)
    if actual != documented:
        missing = sorted(actual - documented)
        extra = sorted(documented - actual)
        raise RouteFailure(f"route inventory drift; missing={missing}; extra={extra}")
    if inventory.get("body_limit_required") is not True or "DefaultBodyLimit::max(body_limit)" not in source:
        raise RouteFailure("global request body limit contract missing")
    if inventory.get("rate_limit_required") is not True or "production_rate_limit" not in source:
        raise RouteFailure("rate-limit middleware contract missing")
    doc = load_text(root, DOC)
    for method, path, handler in documented:
        if f"`{method}`" not in doc or f"`{path}`" not in doc or f"`{handler}`" not in doc:
            raise RouteFailure(f"HTTP document omits route tuple: {(method, path, handler)}")
    for schema_path in SCHEMAS:
        try:
            schema = json.loads(load_text(root, schema_path))
        except json.JSONDecodeError as error:
            raise RouteFailure(f"invalid schema {schema_path}: {error}") from error
        if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
            raise RouteFailure(f"schema draft drift: {schema_path}")
    return len(actual)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        count = validate(args.root)
    except (RouteFailure, OSError, UnicodeError, json.JSONDecodeError) as error:
        print(f"TRNM World HTTP route inventory: FAIL: {error}", file=sys.stderr)
        return 1
    print(f"TRNM World HTTP route inventory: PASS ({count} exact routes)")
    print("This proves router/document agreement only, not deployed authorization or release readiness.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
