#!/usr/bin/env python3
"""Static contract for the GET-only CEX Sequence-50 verifier."""

from __future__ import annotations

import ast
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "verify-trnm-world-cex-sequence-50.py"
REQUIRED_CONTEXTS = {
    "cex-v12/postgres-migration",
    "cex-v12/rust-format",
    "cex-v12/rust-test",
    "cex-v12/rust-clippy",
    "cex-v12/rust-release",
    "cex-v12/gateway-exact-reserve",
    "cex-v12/execution-settlement",
    "cex-v12/provider-reconciliation",
    "cex-v12/aggregate-qualification",
}
MUTATION_ONLY_ENDPOINTS = (
    "/statuses/",
    "/check-runs",
    "/merges",
    "/git/refs",
    "/git/blobs",
    "/git/tags",
    "/releases",
    "/deployments",
)


def fail(message: str) -> None:
    print(f"CEX verifier contract failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    source = SCRIPT.read_text(encoding="utf-8")
    try:
        tree = ast.parse(source, filename=str(SCRIPT))
    except SyntaxError as error:
        fail(f"invalid Python: {error}")

    required = {
        "TrillionniumFoundation/CEX",
        "dc0862b8cf88a1f4e6328d519947e19b81122de0",
        "762e33a3f16c14347a44cec1d862a8e0ab447ad8",
        "fix/cex-v12-seq45-full-gap-closure-20260902",
        "0088_enforce_provider_terminal_evidence_binding.sql",
        "TRNM_GITHUB_READ_TOKEN",
        "runner_id",
        "steps",
        "digest",
        "sbom",
        "provenance",
        "protected",
        "dismiss_stale_reviews",
        "require_last_push_approval",
        "world_adoption_eligible",
        "not_granted_by_this_verifier",
    } | REQUIRED_CONTEXTS
    missing = sorted(item for item in required if item not in source)
    if missing:
        fail(f"missing immutable or fail-closed terms: {missing}")

    for node in ast.walk(tree):
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.func.attr == "Request":
            for keyword in node.keywords:
                if keyword.arg == "method":
                    fail("HTTP method override is forbidden; verifier must remain GET-only")

    for method in ("POST", "PUT", "PATCH", "DELETE"):
        if f'api("{method}"' in source or f"api('{method}'" in source:
            fail(f"mutation method present: {method}")
    for endpoint in MUTATION_ONLY_ENDPOINTS:
        if endpoint in source:
            fail(f"mutation-only endpoint present: {endpoint}")

    if source.count("urllib.request.Request(") != 2:
        fail("only the JSON GET and artifact GET request constructors are permitted")
    required_guards = (
        "if not workflow_runs",
        "if not jobs",
        "runner_id != 0",
        "bool(steps)",
        "if not retained",
        "bound_manifest",
        "found_sbom",
        "found_provenance",
        "if not approvals",
        "branch.get(\"protected\") is not True",
    )
    for guard in required_guards:
        if guard not in source:
            fail(f"missing evidence guard: {guard}")

    print("TRNM_WORLD_CEX_SEQUENCE_50_VERIFIER_CONTRACT_V3=PASS")


if __name__ == "__main__":
    main()
