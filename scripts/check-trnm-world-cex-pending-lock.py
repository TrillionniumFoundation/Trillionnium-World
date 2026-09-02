#!/usr/bin/env python3
"""Validate that the current World-to-CEX lock remains coordination-only."""

from __future__ import annotations

import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
LOCK = ROOT / "docs" / "integration" / "trnm-world-cex-sequence-50-pending-lock-v1.json"
EXPECTED_CONTEXTS = {
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


def fail(message: str) -> None:
    print(f"CEX pending lock contract failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    data = json.loads(LOCK.read_text(encoding="utf-8"))
    if data.get("schema") != "trnm_world_cex_pending_component_lock_v1":
        fail("unexpected schema")
    world = data.get("world", {})
    cex = data.get("cex", {})
    expected = {
        "repository": "TrillionniumFoundation/CEX",
        "pull_request": 24,
        "branch": "fix/cex-v12-seq45-full-gap-closure-20260902",
        "commit": "dc0862b8cf88a1f4e6328d519947e19b81122de0",
        "tree": "762e33a3f16c14347a44cec1d862a8e0ab447ad8",
        "trigger_sequence": 50,
        "migration_head": "0088_enforce_provider_terminal_evidence_binding.sql",
        "qualification": "pending_non_empty_exact_head_evidence",
        "production_authorization": "not_granted",
    }
    for key, value in expected.items():
        if cex.get(key) != value:
            fail(f"CEX {key} drifted")
    if world.get("repository") != "TrillionniumFoundation/Trillionnium-World":
        fail("World repository drifted")
    if world.get("pull_request") != 46:
        fail("World PR drifted")
    if world.get("qualified_source_tree") != "5e613185f5a2abda42df371f3755e73667717309":
        fail("World qualified source tree drifted")
    if set(data.get("required_exact_head_contexts", [])) != EXPECTED_CONTEXTS:
        fail("required exact-head context set drifted")
    if len(data.get("required_before_qualification", [])) < 7:
        fail("qualification requirements were weakened")
    if data.get("lock_state") != "coordination_only_blocked":
        fail("lock must remain coordination-only")
    if data.get("world_adoption") != "disabled":
        fail("World adoption must remain disabled")
    for field in ("trusted_settlement", "production_ready"):
        if data.get(field) is not False:
            fail(f"{field} must remain false")
    prohibited = set(data.get("prohibited_promotions", []))
    for required in {"qualified", "trusted_settlement", "production_ready", "custody_approved"}:
        if required not in prohibited:
            fail(f"missing prohibited promotion: {required}")
    print("TRNM_WORLD_CEX_PENDING_LOCK=PASS")


if __name__ == "__main__":
    main()
