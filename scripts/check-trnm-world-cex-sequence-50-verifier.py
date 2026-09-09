#!/usr/bin/env python3
"""Static contract for the read-only CEX Sequence-50 verifier."""

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
FORBIDDEN = (
    'method="POST"',
    'method="PUT"',
    'method="PATCH"',
    'method="DELETE"',
    "api(\"POST\"",
    "api(\"PUT\"",
    "api(\"PATCH\"",
    "api(\"DELETE\"",
    "/statuses/",
    "/check-runs",
    "/merges",
    "/git/refs",
    "/git/blobs",
    "/git/trees",
    "/git/commits",
    "/releases",
    "/deployments",
)


def fail(message: str) -> None:
    print(f"CEX verifier contract failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    source = SCRIPT.read_text(encoding="utf-8")
    try:
        ast.parse(source, filename=str(SCRIPT))
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
    } | REQUIRED_CONTEXTS
    missing = sorted(item for item in required if item not in source)
    if missing:
        fail(f"missing immutable or fail-closed terms: {missing}")
    for fragment in FORBIDDEN:
        if fragment in source:
            fail(f"write capability present: {fragment}")
    if "urllib.request.Request" not in source or 'method=' in source:
        fail("verifier must use GET-only requests without a method override")
    if "world_adoption_eligible" not in source or "not_granted_by_this_verifier" not in source:
        fail("adoption/production boundary is absent")
    print("TRNM_WORLD_CEX_SEQUENCE_50_VERIFIER_CONTRACT=PASS")


if __name__ == "__main__":
    main()
