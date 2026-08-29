#!/usr/bin/env python3
"""Validate the evidence-bound Trillionnium World V4 convergence state."""

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
STATE_PATH = ROOT / "docs/status/world-v4-convergence-state-2026-08-30.json"
SCHEMA_PATH = ROOT / "docs/status/world-v4-convergence-state-2026-08-30.schema.json"
ADDENDUM_PATH = (
    ROOT
    / "docs/development/TRILLIONNIUM_WORLD_PLAN_V4_CONVERGENCE_ADDENDUM_2026-08-30.md"
)
CURRENT_PLAN_PATH = ROOT / "CURRENT_PLAN.md"
DOCS_INDEX_PATH = ROOT / "docs/README.md"
CANONICAL_SOURCE = (
    ROOT / "trillionnium/contracts/trnm-world-transition-v1/src/canonical.rs"
)
NEGATIVE_VECTORS = (
    ROOT / "docs/protocol/vectors/trnm-world-transition-negative-v1.json"
)
GAME_SERVER_ROOT = ROOT / "trillionnium/crates/trnm-game-server"
BUILD_SCRIPT = GAME_SERVER_ROOT / "build.rs"
DIRECT_CEX_SOURCE = GAME_SERVER_ROOT / "src/cex.rs"
CEX_TEMPLATE = GAME_SERVER_ROOT / "src/cex.rs.in"
DIRECT_CEX_CONTRACT = GAME_SERVER_ROOT / "tests/direct_cex_source_contract.rs"
GENERATED_AUTHORITIES = [
    GAME_SERVER_ROOT / "src/lib.rs.in",
    GAME_SERVER_ROOT / "src/settlement_worker.rs.in",
]

SHA_RE = re.compile(r"^[0-9a-f]{40}$")
ALLOWED_STATES = {
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
EXPECTED_NO_GO = {
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
    """A fail-closed convergence-state validation error."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValidationError(message)


def load_json(path: pathlib.Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ValidationError(f"cannot load {path.relative_to(ROOT)}: {error}") from error


def item_map(data: dict[str, Any]) -> dict[str, dict[str, Any]]:
    items = data.get("items")
    require(isinstance(items, list) and items, "items must be a non-empty array")
    result: dict[str, dict[str, Any]] = {}
    for item in items:
        require(isinstance(item, dict), "every item must be an object")
        item_id = item.get("id")
        require(isinstance(item_id, str) and item_id, "every item needs an id")
        require(item_id not in result, f"duplicate item id {item_id}")
        result[item_id] = item
    return result


def validate_data(data: dict[str, Any]) -> None:
    require(data.get("schema") == "trnm_world_v4_convergence_state_v1", "wrong schema")
    require(data.get("as_of") == "2026-08-30", "wrong as_of date")
    require(
        data.get("repository") == "TrillionniumFoundation/Trillionnium-World",
        "wrong repository",
    )

    candidate = data.get("candidate")
    require(isinstance(candidate, dict), "candidate must be an object")
    require(
        candidate.get("branch") == "fix/world-plan-v4-convergence-2026-08-30",
        "wrong convergence branch",
    )
    for field in ("parent_commit", "main_observed"):
        require(
            isinstance(candidate.get(field), str) and SHA_RE.fullmatch(candidate[field]),
            f"candidate.{field} is not a commit SHA",
        )
    overlap = candidate.get("overlapping_candidate")
    require(isinstance(overlap, dict), "overlapping candidate must be recorded")
    require(
        overlap.get("disposition") == "supersede_after_unique_change_review",
        "overlapping candidate disposition drifted",
    )
    require(
        isinstance(overlap.get("commit"), str) and SHA_RE.fullmatch(overlap["commit"]),
        "overlapping candidate commit is invalid",
    )

    controls = data.get("observed_repository_controls")
    require(isinstance(controls, dict), "repository controls must be an object")
    require(controls.get("candidate_check_runs") == 0, "unverified checks were invented")
    require(
        controls.get("overlapping_candidate_check_runs") == 0,
        "overlapping candidate checks were invented",
    )
    require(controls.get("ruleset_count") == 0, "unobserved ruleset was invented")
    require(controls.get("branch_protection_observed") is False, "branch protection overclaim")
    require(controls.get("required_checks_observed") == [], "required checks overclaim")
    require(
        controls.get("code_owner_review_enforced_observed") is False,
        "code-owner enforcement overclaim",
    )
    require(
        controls.get("administrators_enforced_observed") is False,
        "administrator enforcement overclaim",
    )
    probe = controls.get("actions_probe")
    require(isinstance(probe, dict), "Actions probe must be recorded")
    require(probe.get("workflow_runs") == 0, "probe run overclaim")
    require(
        probe.get("interpretation")
        == "repository_or_organization_actions_execution_not_observed",
        "probe interpretation drifted",
    )

    posture = data.get("release_posture")
    require(isinstance(posture, dict), "release posture must be an object")
    require(posture.get("public_online") == "no_go", "public online overclaim")
    require(
        posture.get("public_player_market") == "disabled",
        "public player market overclaim",
    )
    require(posture.get("commercial_release") == "no_go", "commercial overclaim")
    require(
        posture.get("exact_head_remote_verification") == "repository_control_blocked",
        "remote verification overclaim",
    )
    require(
        posture.get("canonical_nakama_online_authority") == "blocked_upstream",
        "Nakama authority overclaim",
    )

    items = item_map(data)
    observed_counts = collections.Counter()
    for item_id, item in items.items():
        state = item.get("state")
        require(state in ALLOWED_STATES, f"{item_id} has invalid state {state!r}")
        observed_counts[state] += 1
        require(isinstance(item.get("owner"), str) and item["owner"], f"{item_id} has no owner")
        require(isinstance(item.get("claim"), str) and item["claim"], f"{item_id} has no claim")
        evidence = item.get("evidence_required")
        require(
            isinstance(evidence, list)
            and evidence
            and all(isinstance(value, str) and value for value in evidence),
            f"{item_id} has invalid evidence requirements",
        )
        require(len(evidence) == len(set(evidence)), f"{item_id} repeats evidence classes")
        require(
            isinstance(item.get("next_gate"), str) and item["next_gate"],
            f"{item_id} has no next gate",
        )

    declared_counts = data.get("counts")
    require(isinstance(declared_counts, dict), "counts must be an object")
    for state in ALLOWED_STATES:
        require(
            declared_counts.get(state) == observed_counts.get(state, 0),
            f"count mismatch for {state}: declared={declared_counts.get(state)!r} "
            f"observed={observed_counts.get(state, 0)}",
        )

    require(observed_counts["source_verified"] == 0, "source verification invented without runs")
    require(observed_counts["closed"] == 0, "closure invented without required evidence")
    require(items["WORLD-P0-009"]["state"] == "source_open", "generated source gap hidden")
    require(items["WORLD-P0-010"]["state"] == "source_open", "duplicate truth gap hidden")
    require(
        items["WORLD-P0-005A"]["state"] == "repository_control_blocked",
        "Actions blocker hidden",
    )
    require(
        items["WORLD-P0-005B"]["state"] == "repository_control_blocked",
        "ruleset blocker hidden",
    )
    require(items["WORLD-P0-003"]["state"] == "blocked_upstream", "Nakama gap hidden")
    require(items["WORLD-P0-004"]["state"] == "blocked_upstream", "Integration gap hidden")
    require(
        items["WORLD-P2-002A"]["state"] == "human_evidence_required",
        "human evidence overclaim",
    )
    require(
        items["WORLD-P2-003"]["state"] == "commercial_approval_required",
        "commercial approval overclaim",
    )

    require(set(data.get("no_go", [])) == EXPECTED_NO_GO, "no-go set drifted")


def validate_repository() -> None:
    for path in (
        STATE_PATH,
        SCHEMA_PATH,
        ADDENDUM_PATH,
        CURRENT_PLAN_PATH,
        DOCS_INDEX_PATH,
        CANONICAL_SOURCE,
        NEGATIVE_VECTORS,
        BUILD_SCRIPT,
        DIRECT_CEX_SOURCE,
        DIRECT_CEX_CONTRACT,
        *GENERATED_AUTHORITIES,
    ):
        require(path.exists(), f"missing {path.relative_to(ROOT)}")
    require(not CEX_TEMPLATE.exists(), "CEX template authority was reintroduced")

    state = load_json(STATE_PATH)
    schema = load_json(SCHEMA_PATH)
    require(
        schema.get("$id", "").endswith(
            "world-v4-convergence-state-2026-08-30.schema.json"
        ),
        "wrong convergence schema",
    )
    validate_data(state)

    canonical = CANONICAL_SOURCE.read_text(encoding="utf-8")
    for marker in (
        "let normalized_key = key.to_ascii_lowercase();",
        "case_folded_authority_key_still_fails_closed",
        "CanonicalError::ForbiddenAuthorityKey",
    ):
        require(marker in canonical, f"canonical parser missing {marker}")

    vectors = load_json(NEGATIVE_VECTORS)
    vector_names = {
        item.get("name")
        for item in vectors.get("vectors", [])
        if isinstance(item, dict)
    }
    for name in (
        "case_folded_nakama_key",
        "case_folded_completion_key",
        "nested_case_folded_chain_key",
    ):
        require(name in vector_names, f"negative vectors missing {name}")

    build = BUILD_SCRIPT.read_text(encoding="utf-8")
    for path in GENERATED_AUTHORITIES:
        require(path.name in build, f"build script no longer names {path.name}; update status")
    for marker in ("generate_game_server", "generate_settlement_worker", "OUT_DIR"):
        require(marker in build, f"semantic source transform marker missing: {marker}")
    for forbidden in ("generate_cex", "src/cex.rs.in", "trnm_cex_generated.rs"):
        require(forbidden not in build, f"retired CEX transform reintroduced: {forbidden}")

    cex = DIRECT_CEX_SOURCE.read_text(encoding="utf-8")
    for marker in (
        "pub struct CexClient",
        "MAX_REMOTE_ERROR_BODY_BYTES",
        "bounded_error_body",
        "StatusCode::CONFLICT",
        "decode isolated signer response after possible commit",
        "decode CEX receipt after possible commit",
        "synchronous EconomyBackend I/O is prohibited",
    ):
        require(marker in cex, f"direct CEX source missing {marker}")
    for forbidden in ("OUT_DIR", "trnm_cex_generated.rs", "reqwest::blocking", "blocking_client"):
        require(forbidden not in cex, f"direct CEX source contains forbidden {forbidden}")

    direct_contract = DIRECT_CEX_CONTRACT.read_text(encoding="utf-8")
    require("cex_transport_is_directly_compiled_reviewed_source" in direct_contract, "direct CEX contract missing")

    current_plan = CURRENT_PLAN_PATH.read_text(encoding="utf-8")
    docs_index = DOCS_INDEX_PATH.read_text(encoding="utf-8")
    addendum_rel = "TRILLIONNIUM_WORLD_PLAN_V4_CONVERGENCE_ADDENDUM_2026-08-30.md"
    state_rel = "world-v4-convergence-state-2026-08-30.json"
    require(addendum_rel in current_plan, "CURRENT_PLAN.md does not bind the addendum")
    require(state_rel in current_plan, "CURRENT_PLAN.md does not bind convergence state")
    require(addendum_rel in docs_index, "docs index omits convergence addendum")
    require(state_rel in docs_index, "docs index omits convergence state")


def run_negative_self_test(base: dict[str, Any]) -> None:
    mutations: list[tuple[str, Any]] = []

    value = copy.deepcopy(base)
    value["release_posture"]["public_online"] = "ready"
    mutations.append(("public-online-overclaim", value))

    value = copy.deepcopy(base)
    value["observed_repository_controls"]["candidate_check_runs"] = 1
    mutations.append(("invented-check-run", value))

    value = copy.deepcopy(base)
    value["items"][0]["state"] = "closed"
    mutations.append(("invented-closure", value))

    value = copy.deepcopy(base)
    for item in value["items"]:
        if item["id"] == "WORLD-P0-009":
            item["state"] = "source_verified"
    mutations.append(("hidden-generated-source-gap", value))

    value = copy.deepcopy(base)
    value["observed_repository_controls"]["ruleset_count"] = 1
    mutations.append(("invented-ruleset", value))

    for name, mutated in mutations:
        try:
            validate_data(mutated)
        except ValidationError:
            continue
        raise ValidationError(f"negative self-test unexpectedly passed: {name}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="prove common evidence overclaims fail closed",
    )
    arguments = parser.parse_args()

    try:
        validate_repository()
        if arguments.self_test:
            run_negative_self_test(load_json(STATE_PATH))
    except ValidationError as error:
        print(f"TRNM World V4 convergence: FAIL: {error}", file=sys.stderr)
        return 1

    suffix = " + negative self-test" if arguments.self_test else ""
    print(f"TRNM World V4 convergence: PASS{suffix}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
