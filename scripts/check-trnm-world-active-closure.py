#!/usr/bin/env python3
"""Validate the current World closure truth without manufacturing evidence."""

from __future__ import annotations

import argparse
from pathlib import Path
import re
import sys
import tomllib
from typing import Any

from trnm_world_strict_json import StrictJsonError, load_strict_json

BOUNDARY = "PROJECT_BOUNDARY.json"
CATALOG = "docs/catalog.json"
LEDGER = "docs/development/trnm-world-gap-closure-ledger-v6.json"
CEX_LOCK = "docs/integration/trnm-world-cex-current-pending-lock-v2.json"
PROTECTION = "docs/governance/main-protection-contract-v2.json"
RELEASE = "RELEASE_READINESS.md"
PLAN = "CURRENT_PLAN.md"
TOOLCHAIN = "rust-toolchain.toml"

REPOSITORY = "TrillionniumFoundation/Trillionnium-World"
PULL_REQUEST = 104
BRANCH = "fix/world-convergence-v6-20260908"
SAFE_TOOLCHAIN = "1.98.1"
CURRENT_CEX = {
    "repository": "TrillionniumFoundation/CEX",
    "pull_request": 53,
    "branch": "integration/cex-v12-sequence54-20260908",
    "commit": "6a14d0da4cf1aefd4689708a6967a4ae1c0e84fc",
    "tree": "e425423fda89ce5438879518367f4b12ad2ef3f5",
    "base": "db75c74094748a1139fd64b0360e122d8ec797a0",
    "prospective_merge": "53676ad0ea2712b5efa2bf2a49359e00035521c4",
    "sequence": 54,
}
CURRENT_NAKAMA = {
    "repository": "TrillionniumFoundation/TrillionniumGame",
    "pull_request": 63,
    "commit": "6dd5a101c67d78572a9dcaa3ab6ff96ef1966850",
}
CURRENT_CHAIN = {
    "repository": "TrillionniumFoundation/Trillionnium-Chain",
    "pull_request": 62,
    "commit": "521726e47ce0efad6b732a5fc31afcd2cc52593d",
}
CURRENT_INTEGRATION = {
    "repository": "TrillionniumFoundation/Trillionnium-Integration",
    "pull_request": 8,
}
OBSERVED_MAIN_CONTEXTS = {
    "trnm-game-ci",
    "trnm-world-p0-boundaries",
    "trnm-world-status-evidence",
}
SHA1 = re.compile(r"^[0-9a-f]{40}$")


class ActiveClosureFailure(ValueError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ActiveClosureFailure(message)


def object_value(value: Any, label: str) -> dict[str, Any]:
    require(isinstance(value, dict), f"{label} must be an object")
    return value


def list_value(value: Any, label: str) -> list[Any]:
    require(isinstance(value, list), f"{label} must be a list")
    return value


def read_text(root: Path, relative: str, maximum_bytes: int = 512 * 1024) -> str:
    path = root / relative
    require(path.is_file() and not path.is_symlink(), f"missing or symlinked file: {relative}")
    size = path.stat().st_size
    require(0 < size <= maximum_bytes, f"empty or oversized file: {relative}")
    try:
        text = path.read_bytes().decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise ActiveClosureFailure(f"invalid UTF-8 in {relative}: {error}") from error
    require(bool(text.strip()), f"empty file: {relative}")
    return text


def load(root: Path, relative: str) -> dict[str, Any]:
    try:
        value = load_strict_json(root / relative)
    except (OSError, StrictJsonError) as error:
        raise ActiveClosureFailure(f"cannot load {relative}: {error}") from error
    return object_value(value, relative)


def catalog_entry(catalog: dict[str, Any], relative: str) -> dict[str, Any]:
    matches = [
        item
        for item in list_value(catalog.get("documents"), "catalog documents")
        if isinstance(item, dict) and item.get("path") == relative
    ]
    require(len(matches) == 1, f"catalog must contain exactly one entry for {relative}")
    return matches[0]


def exact_dependency(actual: dict[str, Any], expected: dict[str, Any], label: str) -> None:
    for key, value in expected.items():
        require(actual.get(key) == value, f"{label} {key} drift: {actual.get(key)!r} != {value!r}")
    for key in ("commit", "tree", "base", "prospective_merge"):
        value = actual.get(key)
        if value is not None:
            require(isinstance(value, str) and SHA1.fullmatch(value) is not None, f"{label} {key} is not a Git SHA")


def reject_current_stale_identity(relative: str, text: str) -> None:
    stale = (
        "Pull request: `#46`",
        '"pull_request": 46',
        '"pull_request": 24',
        "fix/cex-v12-seq45-full-gap-closure-20260902",
        "dc0862b8cf88a1f4e6328d519947e19b81122de0",
        "fix/cex-v12-seq53-close-repository-gaps-20260904",
        "04491cff7cb317324f57a10de07872d39f3e56c2",
        "f8897db7f5879a8ae84a2ce90ec169e5df9eccb9",
        "CEX PR #29",
    )
    for marker in stale:
        require(marker not in text, f"{relative} contains stale current identity: {marker}")


def validate(root: Path) -> None:
    root = root.resolve()

    toolchain = tomllib.loads(read_text(root, TOOLCHAIN))
    config = object_value(toolchain.get("toolchain"), "toolchain")
    require(config.get("channel") == SAFE_TOOLCHAIN, "root Rust toolchain must be 1.98.1")
    components = config.get("components")
    require(
        isinstance(components, list) and {"rustfmt", "clippy"}.issubset(set(components)),
        "toolchain must include rustfmt and clippy",
    )

    boundary = load(root, BOUNDARY)
    documentation = object_value(boundary.get("documentation"), "boundary documentation")
    require(boundary.get("project_id") == "trillionnium-world", "project id drift")
    require(boundary.get("lane") == "game-product", "project lane drift")
    require(
        object_value(boundary.get("remote"), "boundary remote").get("canonical_slug") == REPOSITORY,
        "canonical repository drift",
    )
    require(documentation.get("current_plan") == PLAN, "current plan pointer drift")
    require(documentation.get("current_gap_ledger") == LEDGER, "current gap ledger pointer drift")
    require(documentation.get("current_cex_pending_lock") == CEX_LOCK, "current CEX lock pointer drift")
    require(documentation.get("release_readiness") == RELEASE, "release readiness pointer drift")
    release_boundary = object_value(boundary.get("release"), "boundary release")
    require(release_boundary.get("public_online") == "no_go", "public online boundary weakened")
    require(release_boundary.get("public_player_market") == "disabled", "public market boundary weakened")
    require(
        release_boundary.get("production_authorization") == "not_granted",
        "boundary production authorization drift",
    )

    catalog = load(root, CATALOG)
    require(catalog.get("schema") == "trnm_world_document_catalog_v1", "catalog schema drift")
    truth_order = list_value(catalog.get("truth_order"), "catalog truth_order")
    require(len(truth_order) == len(set(truth_order)), "catalog truth_order contains duplicates")
    prefix = [
        "PROJECT_BOUNDARY.md",
        PLAN,
        CATALOG,
        "docs/status/world-plan-v4-execution-truth-2026-09-08.json",
        "docs/status/CURRENT.md",
    ]
    require(truth_order[: len(prefix)] == prefix, "catalog truth hierarchy prefix drift")
    for relative, expected_class in (
        (RELEASE, "release-truth"),
        (LEDGER, "gap-ledger"),
        (CEX_LOCK, "integration-lock"),
    ):
        require(relative in truth_order, f"{relative} is absent from truth_order")
        entry = catalog_entry(catalog, relative)
        require(entry.get("class") == expected_class, f"{relative} catalog class drift")
        require(
            entry.get("status") in {"current", "current-candidate"},
            f"{relative} catalog status drift",
        )

    ledger = load(root, LEDGER)
    require(ledger.get("schema") == "trnm_world_gap_closure_ledger_v6", "V6 ledger schema drift")
    review = object_value(ledger.get("review_surface"), "V6 review surface")
    require(review.get("repository") == REPOSITORY, "V6 repository drift")
    require(review.get("pull_request") == PULL_REQUEST, "V6 pull request drift")
    require(review.get("branch") == BRANCH, "V6 branch drift")
    require(review.get("draft_required") is True, "V6 candidate must remain Draft")
    require(
        review.get("production_authorization") == "not_granted",
        "V6 review authorization drift",
    )
    for key in (
        "base_commit",
        "observed_head_before_update",
        "observed_tree_before_update",
        "observed_prospective_merge_before_update",
    ):
        require(
            isinstance(review.get(key), str) and SHA1.fullmatch(review[key]) is not None,
            f"V6 {key} is not a Git SHA",
        )
    require(
        object_value(ledger.get("toolchain"), "V6 toolchain").get("current_candidate")
        == SAFE_TOOLCHAIN,
        "V6 toolchain drift",
    )
    source = object_value(ledger.get("repository_source"), "V6 source")
    require(source.get("state") == "closed_candidate", "repository source/documentation state drift")
    require(
        source.get("qualification_scope") == "source_and_documentation_only",
        "repository source scope overclaim",
    )
    require(source.get("strict_machine_truth_parser") is True, "strict parser closure lost")
    require(
        source.get("unconditional_pull_request_documentation_gate") is True,
        "documentation trigger closure lost",
    )
    evidence = object_value(ledger.get("repository_evidence"), "V6 repository evidence")
    require(evidence.get("exact_head_workflow_runs") == 0, "unverified workflow run count promoted")
    for key in (
        "exact_head_ci_closed",
        "prospective_merge_ci_closed",
        "independent_review_closed",
        "server_governance_closed",
    ):
        require(evidence.get(key) is False, f"unproved repository denominator promoted: {key}")
    require(
        set(evidence.get("observed_main_required_contexts", [])) == OBSERVED_MAIN_CONTEXTS,
        "observed main contexts drift",
    )
    require(
        evidence.get("desired_contract_matches_observation") is False,
        "server governance mismatch was hidden",
    )
    require(evidence.get("rulesets_observed") == 0, "unobserved ruleset count promoted")
    dependencies = object_value(ledger.get("dependencies"), "V6 dependencies")
    exact_dependency(object_value(dependencies.get("cex"), "V6 CEX"), CURRENT_CEX, "V6 CEX")
    exact_dependency(object_value(dependencies.get("nakama"), "V6 Nakama"), CURRENT_NAKAMA, "V6 Nakama")
    exact_dependency(object_value(dependencies.get("chain"), "V6 Chain"), CURRENT_CHAIN, "V6 Chain")
    exact_dependency(
        object_value(dependencies.get("integration"), "V6 Integration"),
        CURRENT_INTEGRATION,
        "V6 Integration",
    )
    for name in ("cex", "nakama", "chain", "integration"):
        item = object_value(dependencies.get(name), f"V6 {name}")
        require(item.get("qualified") is False, f"{name} was promoted without evidence")
        require(
            item.get("production_authorization") == "not_granted",
            f"{name} production authorization drift",
        )
    closure = object_value(ledger.get("closure"), "V6 closure")
    require(
        closure.get("repository_source_documentation_closed_candidate") is True,
        "source/documentation candidate closure lost",
    )
    for key in (
        "world_owned_source_development_closed",
        "exact_head_ci_closed",
        "prospective_merge_ci_closed",
        "independent_review_closed",
        "server_governance_closed",
        "cross_repository_qualification_closed",
        "external_evidence_closed",
        "all_repository_gaps_closed",
        "all_plan_gaps_closed",
        "public_online",
        "trusted_settlement",
        "public_player_market",
        "production_ready",
    ):
        require(closure.get(key) is False, f"unproved closure promoted: {key}")
    require(
        closure.get("production_authorization") == "not_granted",
        "V6 production authorization drift",
    )

    cex_lock = load(root, CEX_LOCK)
    require(
        cex_lock.get("schema") == "trnm_world_cex_pending_component_lock_v2",
        "current CEX lock schema drift",
    )
    require(
        cex_lock.get("lock_state") == "coordination_only_blocked",
        "CEX lock must remain coordination-only",
    )
    require(
        cex_lock.get("selection_state") == "observed_unqualified_candidate",
        "CEX selection state drift",
    )
    world = object_value(cex_lock.get("world"), "CEX lock World")
    require(
        world.get("repository") == REPOSITORY
        and world.get("pull_request") == PULL_REQUEST
        and world.get("branch") == BRANCH,
        "CEX lock World identity drift",
    )
    for key in (
        "base",
        "observed_commit_before_update",
        "observed_tree_before_update",
        "observed_prospective_merge_before_update",
    ):
        require(
            isinstance(world.get(key), str) and SHA1.fullmatch(world[key]) is not None,
            f"CEX lock World {key} is not a Git SHA",
        )
    exact_dependency(object_value(cex_lock.get("cex"), "CEX lock CEX"), CURRENT_CEX, "CEX lock CEX")
    require(
        len(list_value(cex_lock.get("required_before_selection"), "CEX selection requirements")) >= 8,
        "CEX qualification requirements weakened",
    )
    prohibited = set(list_value(cex_lock.get("prohibited_promotions"), "CEX prohibited promotions"))
    require(
        {
            "qualified",
            "trusted_settlement",
            "production_ready",
            "custody_approved",
            "production_authorization",
        }.issubset(prohibited),
        "CEX prohibited promotions weakened",
    )
    require(cex_lock.get("world_adoption") == "disabled", "World CEX adoption enabled without qualification")
    for key in (
        "qualified",
        "trusted_settlement",
        "custody_approved",
        "public_player_market",
        "production_ready",
    ):
        require(cex_lock.get(key) is False, f"CEX lock overclaim: {key}")
    require(
        cex_lock.get("production_authorization") == "not_granted",
        "CEX lock production authorization drift",
    )

    protection = load(root, PROTECTION)
    require(
        protection.get("server_state") == "not_enforced_until_api_readback",
        "desired branch protection misrepresented as enforced",
    )
    require(
        protection.get("production_authorization") == "not_granted",
        "protection contract authorization drift",
    )

    release = read_text(root, RELEASE)
    for marker in (
        "status: current",
        "production_authorization: not_granted",
        "**Not release-ready.",
        "all_repository_gaps_closed=false",
        "all_plan_gaps_closed=false",
        "production_ready=false",
        "CEX PR #53",
        CURRENT_CEX["commit"],
        CURRENT_CEX["tree"],
    ):
        require(marker in release, f"release truth missing marker: {marker}")
    require(
        "public online, trusted settlement, the public player market and commercial release remain no-go / disabled"
        in release.lower(),
        "release NO-GO statement missing",
    )
    require(
        "/Users/" not in release and ".openclaw/workspace" not in release,
        "release truth contains a personal path",
    )

    plan = read_text(root, PLAN)
    for marker in (
        "Pull request: `#104`",
        f"Branch: `{BRANCH}`",
        LEDGER,
        CEX_LOCK,
        RELEASE,
        "CEX PR #53",
        CURRENT_CEX["commit"],
        CURRENT_CEX["tree"],
        "Production authorization remains **not granted**",
    ):
        require(marker in plan, f"CURRENT_PLAN.md missing current marker: {marker}")

    for relative in (PLAN, RELEASE, LEDGER, CEX_LOCK, CATALOG):
        reject_current_stale_identity(relative, read_text(root, relative))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        validate(args.root)
    except (
        ActiveClosureFailure,
        OSError,
        UnicodeError,
        StrictJsonError,
        tomllib.TOMLDecodeError,
    ) as error:
        print(f"TRNM World active closure truth: FAIL: {error}", file=sys.stderr)
        return 1
    print(
        "TRNM World active closure truth: PASS "
        "(toolchain, release truth, V6 ledger and current CEX coordination lock)"
    )
    print(
        "Hosted CI, server governance, independent review, cross-repository "
        "and external evidence remain separate."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
