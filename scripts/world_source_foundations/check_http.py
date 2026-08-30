from __future__ import annotations

from datetime import date
from common import OPENAPI, ROOT, TEMPLATES, load, resolve_pointer, router_source, source_routes

openapi = load(OPENAPI)
templates = load(TEMPLATES)
if openapi.get("openapi") != "3.1.0":
    raise SystemExit("OpenAPI version must be exactly 3.1.0")
paths = openapi.get("paths")
if not isinstance(paths, dict) or len(paths) != 50:
    raise SystemExit(f"expected 50 implemented routes, found {len(paths or {})}")

if openapi.get("x-authority-boundary") != {
    "owner": "Trillionnium-World",
    "profile": "world_legacy_local_alpha",
    "canonical-online-authority": "Trillionnium-Nakama",
    "public-online": "no_go",
    "player-market": "disabled",
}:
    raise SystemExit("OpenAPI authority boundary drifted")
if not openapi.get("x-unknown-field-policy") or not openapi.get("x-unknown-enum-policy"):
    raise SystemExit("OpenAPI unknown field/enum policy is missing")
retirement = openapi.get("x-retirement", {})
for field in ("owner", "target", "not-before", "gate"):
    if not retirement.get(field):
        raise SystemExit(f"OpenAPI retirement field missing: {field}")
date.fromisoformat(retirement["not-before"])

error_schema = openapi["components"]["schemas"]["ErrorEnvelope"]
if error_schema.get("additionalProperties") is not False:
    raise SystemExit("error envelope must reject unknown fields")
if set(error_schema.get("required", [])) != {"error", "retryable"}:
    raise SystemExit("error envelope required fields drifted")

source_path, source = router_source()
implemented = source_routes(source)
documented: dict[tuple[str, str], str] = {}
operation_ids: set[str] = set()
for path, item in paths.items():
    ref = item.get("$ref", "")
    if not ref.startswith("./path-templates.json#/"):
        raise SystemExit(f"path does not use a controlled template: {path}")
    target = resolve_pointer(templates, ref.split("#", 1)[1])
    methods = [method for method in ("get", "post", "put", "delete", "patch") if method in target]
    if len(methods) != 1:
        raise SystemExit(f"path template must contain one operation: {path}")
    method = methods[0]
    handler = item.get("x-implementation-handler")
    operation_id = item.get("x-operation-id")
    if not handler or operation_id != handler or operation_id in operation_ids:
        raise SystemExit(f"invalid or duplicate operation identity: {path}")
    operation_ids.add(operation_id)
    documented[(path, method)] = handler

    operation = target[method]
    responses = operation.get("responses", {})
    for code in ("200", "4XX", "5XX"):
        schema_ref = (
            responses.get(code, {})
            .get("content", {})
            .get("application/json", {})
            .get("schema", {})
            .get("$ref")
        )
        if not schema_ref or "trnm-world-legacy-local-alpha-v1.openapi.json#/components/schemas/" not in schema_ref:
            raise SystemExit(f"{path} {method} response {code} lacks a root schema")
    if method == "post":
        request_ref = (
            operation.get("requestBody", {})
            .get("content", {})
            .get("application/json", {})
            .get("schema", {})
            .get("$ref")
        )
        if not request_ref or not request_ref.endswith("#/components/schemas/JsonObject"):
            raise SystemExit(f"{path} POST request schema is missing")
    if "{" in path:
        import re
        names = set(re.findall(r"\{([^}]+)\}", path))
        parameters = {
            parameter.get("name")
            for parameter in target.get("parameters", [])
            if parameter.get("in") == "path" and parameter.get("required") is True
        }
        if names != parameters:
            raise SystemExit(f"path parameter mismatch for {path}: {names} != {parameters}")

if implemented != documented:
    missing = sorted(set(implemented) - set(documented))
    extra = sorted(set(documented) - set(implemented))
    changed = sorted(
        (key, implemented[key], documented[key])
        for key in set(implemented) & set(documented)
        if implemented[key] != documented[key]
    )
    raise SystemExit(
        f"OpenAPI/router mismatch\\nmissing={missing}\\nextra={extra}\\nhandler_changes={changed}"
    )
print(f"http_contract=passed routes={len(documented)} source={source_path.relative_to(ROOT)}")
