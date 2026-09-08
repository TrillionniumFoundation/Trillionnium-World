#!/usr/bin/env python3
"""Validate that the current World-to-CEX observation remains coordination-only."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys
from typing import Any

from trnm_world_strict_json import StrictJsonError, load_strict_json

LOCK = "docs/integration/trnm-world-cex-current-pending-lock-v2.json"
EXPECTED_WORLD = {
    "repository": "TrillionniumFoundation/Trillionnium-World",
    "pull_request": 104,
    "branch": "fix/world-convergence-v6-20260908",
}
EXPECTED_CEX = {
    "repository": "TrillionniumFoundation/CEX",
    "pull_request": 29,
    "branch": "fix/cex-v12-seq53-close-repository-gaps-20260904",
    "commit": "04491cff7cb317324f57a10de07872d39f3e56c2",
    "tree": "f8897db7f5879a8ae84a2ce90ec169e5df9eccb9",
    "sequence": 53,
}


class PendingLockFailure(ValueError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise PendingLockFailure(message)


def object_value(value: Any, label: str) -> dict[str, Any]:
    require(isinstance(value, dict), f"{label} must be an object")
    return value


def validate(root: Path) -> None:
    try:
        data = load_strict_json(root.resolve() / LOCK)
    except (OSError, StrictJsonError) as error:
        raise PendingLockFailure(f"cannot load current CEX lock: {error}") from error
    data = object_value(data, "current CEX lock")
    require(data.get("schema") == "trnm_world_cex_pending_component_lock_v2", "unexpected schema")
    require(data.get("lock_state") == "coordination_only_blocked", "lock must remain coordination-only")
    require(data.get("selection_state") == "observed_unqualified_candidate", "selection state drift")
    world = object_value(data.get("world"), "World identity")
    cex = object_value(data.get("cex"), "CEX identity")
    for key, value in EXPECTED_WORLD.items():
        require(world.get(key) == value, f"World {key} drifted")
    for key, value in EXPECTED_CEX.items():
        require(cex.get(key) == value, f"CEX {key} drifted")
    require(world.get("qualification") == "pending_non_empty_exact_head_and_merge_evidence", "World qualification state drift")
    require(cex.get("qualification") == "pending_non_empty_exact_head_evidence_and_independent_approval", "CEX qualification state drift")
    require(world.get("production_authorization") == "not_granted", "World production authorization drift")
    require(cex.get("production_authorization") == "not_granted", "CEX production authorization drift")
    requirements = data.get("required_before_selection")
    require(isinstance(requirements, list) and len(requirements) >= 8 and len(requirements) == len(set(requirements)), "selection requirements were weakened")
    prohibited = data.get("prohibited_promotions")
    require(isinstance(prohibited, list), "prohibited promotions missing")
    require(
        {"qualified", "trusted_settlement", "production_ready", "custody_approved", "production_authorization"}.issubset(set(prohibited)),
        "prohibited promotions were weakened",
    )
    require(data.get("world_adoption") == "disabled", "World adoption must remain disabled")
    for field in ("qualified", "trusted_settlement", "custody_approved", "public_player_market", "production_ready"):
        require(data.get(field) is False, f"{field} must remain false")
    require(data.get("production_authorization") == "not_granted", "lock production authorization drift")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        validate(args.root)
    except (PendingLockFailure, OSError, UnicodeError, StrictJsonError) as error:
        print(f"TRNM World/CEX pending lock: FAIL: {error}", file=sys.stderr)
        return 1
    print("TRNM_WORLD_CEX_PENDING_LOCK_V2=PASS")
    print("The observation is coordination-only and grants no qualification, custody or production credit.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
