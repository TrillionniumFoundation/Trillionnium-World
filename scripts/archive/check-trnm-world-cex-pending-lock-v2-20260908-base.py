#!/usr/bin/env python3
"""Validate that the current World-to-CEX observation remains coordination-only."""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
from pathlib import Path
import re
import sys
from typing import Any

from trnm_world_strict_json import StrictJsonError, load_strict_json

LOCK = "docs/integration/trnm-world-cex-current-pending-lock-v2.json"
EXPECTED_WORLD = {
    "repository": "TrillionniumFoundation/Trillionnium-World",
    "pull_request": 104,
    "branch": "fix/world-convergence-v6-20260908",
    "base": "0f6117a56263bcb5bf89e34b8bcda557a7da2e6d",
    "observed_commit_before_update": "b218dbfb6c22f6b44e8ee542d2ab62c27d160349",
    "observed_tree_before_update": "1c3a79ad518716fc574deda039a49d8e29aa07f9",
    "observed_prospective_merge_before_update": "56c3d79b98dc478cd6148ebb4d19c27c72d9ab89",
}
EXPECTED_CEX = {
    "repository": "TrillionniumFoundation/CEX",
    "pull_request": 53,
    "branch": "integration/cex-v12-sequence54-20260908",
    "commit": "6a14d0da4cf1aefd4689708a6967a4ae1c0e84fc",
    "tree": "e425423fda89ce5438879518367f4b12ad2ef3f5",
    "base": "db75c74094748a1139fd64b0360e122d8ec797a0",
    "prospective_merge": "53676ad0ea2712b5efa2bf2a49359e00035521c4",
    "sequence": 54,
}
EXPECTED_REQUIREMENTS = {
    "CEX exact head and prospective merge have non-empty terminal-success repository-native workflows",
    "CEX source, migrations, manifest, SBOM and provenance bind one immutable commit and tree",
    "World exact head and prospective merge have non-empty terminal-success workflows",
    "World and CEX agree on exact request, receipt, idempotency and ambiguity-recovery bytes",
    "response-loss, conflict, replay, stale lease and provider-unknown-outcome matrices pass",
    "independent CEX, World, security and Integration reviewers approve the unchanged component tuple",
    "Integration records the exact accepted World/CEX/Nakama/Chain component lock",
    "custody, deployment, rollback and no-dual-writer evidence remain separately satisfied",
}
REQUIRED_PROHIBITIONS = {
    "qualified",
    "trusted_settlement",
    "production_ready",
    "custody_approved",
    "public_player_market",
    "production_authorization",
}
SHA1 = re.compile(r"^[0-9a-f]{40}$")


class PendingLockFailure(ValueError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise PendingLockFailure(message)


def object_value(value: Any, label: str) -> dict[str, Any]:
    require(isinstance(value, dict), f"{label} must be an object")
    return value


def exact_fields(actual: dict[str, Any], expected: dict[str, Any], label: str) -> None:
    for key, value in expected.items():
        require(actual.get(key) == value, f"{label} {key} drifted")
    for key in (
        "base",
        "commit",
        "tree",
        "prospective_merge",
        "observed_commit_before_update",
        "observed_tree_before_update",
        "observed_prospective_merge_before_update",
    ):
        value = actual.get(key)
        if value is not None:
            require(
                isinstance(value, str) and SHA1.fullmatch(value) is not None,
                f"{label} {key} is not a Git SHA",
            )


def validate(root: Path) -> None:
    try:
        data = load_strict_json(root.resolve() / LOCK)
    except (OSError, StrictJsonError) as error:
        raise PendingLockFailure(f"cannot load current CEX lock: {error}") from error
    data = object_value(data, "current CEX lock")

    require(
        data.get("schema") == "trnm_world_cex_pending_component_lock_v2",
        "unexpected schema",
    )
    recorded = data.get("recorded_at_utc")
    require(
        isinstance(recorded, str)
        and re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", recorded) is not None,
        "recorded_at_utc must be explicit UTC",
    )
    datetime.strptime(recorded, "%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=timezone.utc)

    require(
        data.get("lock_state") == "coordination_only_blocked",
        "lock must remain coordination-only",
    )
    require(
        data.get("selection_state") == "observed_unqualified_candidate",
        "selection state drift",
    )

    world = object_value(data.get("world"), "World identity")
    cex = object_value(data.get("cex"), "CEX identity")
    exact_fields(world, EXPECTED_WORLD, "World")
    exact_fields(cex, EXPECTED_CEX, "CEX")

    require(
        world.get("qualification") == "pending_non_empty_exact_head_and_merge_evidence",
        "World qualification state drift",
    )
    require(
        cex.get("qualification")
        == "pending_non_empty_exact_head_and_merge_evidence_and_independent_approval",
        "CEX qualification state drift",
    )
    require(
        world.get("production_authorization") == "not_granted",
        "World production authorization drift",
    )
    require(
        cex.get("production_authorization") == "not_granted",
        "CEX production authorization drift",
    )

    requirements = data.get("required_before_selection")
    require(
        isinstance(requirements, list)
        and len(requirements) == len(set(requirements))
        and set(requirements) == EXPECTED_REQUIREMENTS,
        "selection requirements drifted or were weakened",
    )
    prohibited = data.get("prohibited_promotions")
    require(
        isinstance(prohibited, list)
        and len(prohibited) == len(set(prohibited))
        and REQUIRED_PROHIBITIONS.issubset(set(prohibited)),
        "prohibited promotions were weakened",
    )

    require(data.get("world_adoption") == "disabled", "World adoption must remain disabled")
    for field in (
        "qualified",
        "trusted_settlement",
        "custody_approved",
        "public_player_market",
        "production_ready",
    ):
        require(data.get(field) is False, f"{field} must remain false")
    require(
        data.get("production_authorization") == "not_granted",
        "lock production authorization drift",
    )
    rule = data.get("no_credit_rule")
    require(
        isinstance(rule, str)
        and "not a component selection" in rule
        and "production authorization" in rule,
        "no-credit rule was weakened",
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "root",
        nargs="?",
        type=Path,
        default=Path(__file__).resolve().parents[1],
    )
    args = parser.parse_args()
    try:
        validate(args.root)
    except (PendingLockFailure, OSError, UnicodeError, StrictJsonError) as error:
        print(f"TRNM World/CEX pending lock: FAIL: {error}", file=sys.stderr)
        return 1
    print("TRNM_WORLD_CEX_PENDING_LOCK_V2=PASS")
    print(
        "The observation is coordination-only and grants no qualification, "
        "custody or production credit."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
