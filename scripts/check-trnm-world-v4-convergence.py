#!/usr/bin/env python3
"""Fail-closed validator for the current Trillionnium World V4 candidate."""

from __future__ import annotations

import argparse
import collections
import copy
import json
import pathlib
import re
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
STATE = ROOT / "docs/status/world-v4-convergence-state-2026-08-30.json"
SCHEMA = ROOT / "docs/status/world-v4-convergence-state-2026-08-30.schema.json"
ADDENDUM = ROOT / "docs/development/TRILLIONNIUM_WORLD_PLAN_V4_CONVERGENCE_ADDENDUM_2026-08-30.md"
CURRENT_PLAN = ROOT / "CURRENT_PLAN.md"
DOCS_INDEX = ROOT / "docs/README.md"
CANONICAL = ROOT / "trillionnium/contracts/trnm-world-transition-v1/src/canonical.rs"
NEGATIVE_VECTORS = ROOT / "docs/protocol/vectors/trnm-world-transition-negative-v1.json"
GAME_SERVER = ROOT / "trillionnium/crates/trnm-game-server"
BUILD_RS = GAME_SERVER / "build.rs"
LIB_RS = GAME_SERVER / "src/lib.rs"
LIB_TEMPLATE = GAME_SERVER / "src/lib.rs.in"
CEX_RS = GAME_SERVER / "src/cex.rs"
CEX_TEMPLATE = GAME_SERVER / "src/cex.rs.in"
WORKER_RS = GAME_SERVER / "src/settlement_worker.rs"
WORKER_TEMPLATE = GAME_SERVER / "src/settlement_worker.rs.in"
WORKER_LEGACY = GAME_SERVER / "src/settlement_worker_legacy.rs"
WORKER_RUNTIME = GAME_SERVER / "src/settlement_worker_runtime_v2.rs"
P0_010_EVIDENCE = ROOT / "docs/evidence/v4/WORLD-P0-010-single-candidate-closure.json"

SHA = re.compile(r"^[0-9a-f]{40}$")
STATES = {
    "source_open",
    "source_implemented_unverified",
    "source_verified",
    "repository_control_blocked",
    "blocked_upstream",
    "environment_evidence_required",
    "human_evidence_required",
    "commercial_approval_required",
    "closed",
}
NO_GO = {
    "dual_canonical_world_and_nakama_authority",
    "external_io_under_mutable_game_row_locks",
    "expired_lease_mutation",
    "blind_retry_after_ambiguous_remote_success",
    "semantic_difference_between_reviewed_and_compiled_source",
    "write_capable_candidate_ci",
    "missing_exact_head_checks",
    "ruleset_inferred_from_source_files",
    "automated_evidence_used_as_human_public_network_custody_or_commercial_credit",
}


class ValidationError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValidationError(message)


def load(path: pathlib.Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ValidationError(f"cannot load {path.relative_to(ROOT)}: {error}") from error


def items_by_id(data: dict[str, Any]) -> dict[str, dict[str, Any]]:
    raw = data.get("items")
    require(isinstance(raw, list) and raw, "items must be a non-empty array")
    result: dict[str, dict[str, Any]] = {}
    for item in raw:
        require(isinstance(item, dict), "every item must be an object")
        item_id = item.get("id")
        require(isinstance(item_id, str) and item_id, "every item needs an id")
        require(item_id not in result, f"duplicate item id {item_id}")
        result[item_id] = item
    return result


def validate_state(data: dict[str, Any]) -> None:
    require(data.get("schema") == "trnm_world_v4_convergence_state_v1", "wrong state schema")
    require(data.get("repository") == "TrillionniumFoundation/Trillionnium-World", "wrong repository")
    require(data.get("as_of") == "2026-08-30", "wrong state date")

    candidate = data.get("candidate")
    require(isinstance(candidate, dict), "candidate must be an object")
    require(candidate.get("branch") == "fix/world-plan-v4-convergence-2026-08-30", "wrong candidate branch")
    for field in ("parent_commit", "main_observed"):
        require(isinstance(candidate.get(field), str) and SHA.fullmatch(candidate[field]), f"invalid candidate.{field}")

    controls = data.get("observed_repository_controls")
    require(isinstance(controls, dict), "repository controls must be an object")
    for field in ("candidate_check_runs", "overlapping_candidate_check_runs", "ruleset_count"):
        require(isinstance(controls.get(field), int) and controls[field] >= 0, f"invalid controls.{field}")
    require(isinstance(controls.get("required_checks_observed"), list), "required checks must be an array")

    posture = data.get("release_posture")
    require(isinstance(posture, dict), "release posture must be an object")
    require(posture.get("public_online") == "no_go", "public-online overclaim")
    require(posture.get("public_player_market") == "disabled", "public-market overclaim")
    require(posture.get("commercial_release") == "no_go", "commercial overclaim")
    require(posture.get("canonical_nakama_online_authority") == "blocked_upstream", "Nakama overclaim")

    items = items_by_id(data)
    observed = collections.Counter()
    for item_id, item in items.items():
        state = item.get("state")
        require(state in STATES, f"{item_id} has invalid state {state!r}")
        observed[state] += 1
        require(isinstance(item.get("owner"), str) and item["owner"], f"{item_id} has no owner")
        evidence = item.get("evidence_required")
        require(isinstance(evidence, list) and evidence and len(evidence) == len(set(evidence)), f"{item_id} has invalid evidence")
        require(all(isinstance(value, str) and value for value in evidence), f"{item_id} has empty evidence")
        require(isinstance(item.get("next_gate"), str) and item["next_gate"], f"{item_id} has no next gate")

    counts = data.get("counts")
    require(isinstance(counts, dict), "counts must be an object")
    for state in STATES:
        require(counts.get(state) == observed.get(state, 0), f"count mismatch for {state}")

    required_states = {
        "WORLD-P0-003": "blocked_upstream",
        "WORLD-P0-004": "blocked_upstream",
        "WORLD-P0-005A": "repository_control_blocked",
        "WORLD-P0-005B": "repository_control_blocked",
        "WORLD-P0-005C": "repository_control_blocked",
        "WORLD-P2-002A": "human_evidence_required",
        "WORLD-P2-003": "commercial_approval_required",
        "WORLD-P0-010": "closed",
    }
    for item_id, expected in required_states.items():
        require(items.get(item_id, {}).get("state") == expected, f"{item_id} truth drifted")

    if observed["source_verified"]:
        require(controls["candidate_check_runs"] > 0, "source verification has no exact-head run")
        require(bool(controls["required_checks_observed"]), "source verification has no required checks")
    require(set(data.get("no_go", [])) == NO_GO, "no-go set drifted")


def validate_source_shape(data: dict[str, Any]) -> None:
    items = items_by_id(data)
    p0_009 = items["WORLD-P0-009"]["state"]

    for path in (STATE, SCHEMA, ADDENDUM, CURRENT_PLAN, DOCS_INDEX, CANONICAL, NEGATIVE_VECTORS, LIB_RS, CEX_RS, WORKER_RS, WORKER_LEGACY, WORKER_RUNTIME, P0_010_EVIDENCE):
        require(path.exists(), f"missing {path.relative_to(ROOT)}")
    require(not CEX_TEMPLATE.exists(), "CEX template authority was reintroduced")
    require(not WORKER_TEMPLATE.exists(), "worker template authority was reintroduced")

    lib = LIB_RS.read_text(encoding="utf-8")
    generated_shape = BUILD_RS.exists() or LIB_TEMPLATE.exists()
    if generated_shape:
        require(BUILD_RS.exists() and LIB_TEMPLATE.exists(), "partial game-server generator state")
        require(p0_009 == "source_open", "semantic build transform was hidden")
        build = BUILD_RS.read_text(encoding="utf-8")
        require("src/lib.rs.in" in build and "generate_game_server" in build and "OUT_DIR" in build, "game-server transform drifted")
        require("trnm_game_server_lib_generated.rs" in lib and "OUT_DIR" in lib, "compiled wrapper drifted")
    else:
        require(p0_009 in {"source_implemented_unverified", "source_verified", "closed"}, "direct source status was not advanced")
        for marker in (
            "const MIGRATION_V16:",
            "const MIGRATION_V17:",
            "const MIGRATION_V18:",
            "const MIGRATION_V19:",
            "terminal settlement is owned by trnm-settlement-worker; in-process settlement is prohibited",
        ):
            require(marker in lib, f"direct game-server source missing {marker}")
        for forbidden in (
            "trnm_game_server_lib_generated.rs",
            "reconcile_economy(&state.cex",
            "settle_pending_matches(&settlement_state",
        ):
            require(forbidden not in lib, f"direct game-server source retained {forbidden}")

    cex = CEX_RS.read_text(encoding="utf-8")
    for marker in ("pub struct CexClient", "MAX_REMOTE_ERROR_BODY_BYTES", "bounded_error_body", "StatusCode::CONFLICT"):
        require(marker in cex, f"direct CEX source missing {marker}")
    for forbidden in ("OUT_DIR", "trnm_cex_generated.rs", "reqwest::blocking"):
        require(forbidden not in cex, f"direct CEX source contains {forbidden}")

    worker = WORKER_RS.read_text(encoding="utf-8")
    for marker in ('include!("settlement_worker_legacy.rs")', 'include!("settlement_worker_runtime_v2.rs")', "run_v2 as run"):
        require(marker in worker, f"direct worker wrapper missing {marker}")
    require("OUT_DIR" not in worker and "trnm_settlement_worker_generated.rs" not in worker, "worker generation was reintroduced")

    canonical = CANONICAL.read_text(encoding="utf-8")
    for marker in ("let normalized_key = key.to_ascii_lowercase();", "case_folded_authority_key_still_fails_closed", "CanonicalError::ForbiddenAuthorityKey"):
        require(marker in canonical, f"canonical parser missing {marker}")

    vectors = load(NEGATIVE_VECTORS)
    names = {item.get("name") for item in vectors.get("vectors", []) if isinstance(item, dict)}
    for name in ("case_folded_nakama_key", "case_folded_completion_key", "nested_case_folded_chain_key"):
        require(name in names, f"negative vectors missing {name}")

    closure = load(P0_010_EVIDENCE)
    require(closure.get("schema") == "trnm_world_single_candidate_closure_v1", "P0-010 evidence schema drifted")
    require(closure.get("canonical_pr") == 39, "P0-010 evidence does not bind PR 39")


def validate_repository() -> None:
    data = load(STATE)
    schema = load(SCHEMA)
    require(schema.get("$id", "").endswith("world-v4-convergence-state-2026-08-30.schema.json"), "wrong schema document")
    validate_state(data)
    validate_source_shape(data)
    current = CURRENT_PLAN.read_text(encoding="utf-8")
    index = DOCS_INDEX.read_text(encoding="utf-8")
    require("TRILLIONNIUM_WORLD_PLAN_V4_CONVERGENCE_ADDENDUM_2026-08-30.md" in current, "current plan omits addendum")
    require("world-v4-convergence-state-2026-08-30.json" in current, "current plan omits state")
    require("TRILLIONNIUM_WORLD_PLAN_V4_CONVERGENCE_ADDENDUM_2026-08-30.md" in index, "docs index omits addendum")


def negative_self_test() -> None:
    base = load(STATE)
    mutations: list[tuple[str, dict[str, Any]]] = []
    for label, mutate in (
        ("public-online", lambda value: value["release_posture"].__setitem__("public_online", "ready")),
        ("Nakama-authority", lambda value: value["release_posture"].__setitem__("canonical_nakama_online_authority", "ready")),
        ("human-evidence", lambda value: next(item for item in value["items"] if item["id"] == "WORLD-P2-002A").__setitem__("state", "closed")),
        ("duplicate-candidate", lambda value: next(item for item in value["items"] if item["id"] == "WORLD-P0-010").__setitem__("state", "source_open")),
    ):
        candidate = copy.deepcopy(base)
        mutate(candidate)
        mutations.append((label, candidate))
    for label, value in mutations:
        try:
            validate_state(value)
        except ValidationError:
            continue
        raise ValidationError(f"negative self-test accepted {label}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--negative-self-test", action="store_true")
    args = parser.parse_args()
    try:
        validate_repository()
        if args.negative_self_test:
            negative_self_test()
    except ValidationError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1
    print("Trillionnium World V4 convergence state: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
