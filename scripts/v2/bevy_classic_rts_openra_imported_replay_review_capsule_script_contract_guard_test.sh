#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_review_capsule.sh"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
LIB="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

test -x "$SCRIPT"

grep -F 'classic-rts-openra-imported-replay-review-capsule' "$SCRIPT" >/dev/null
grep -F 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REVIEW_CAPSULE_GREEN' "$SCRIPT" >/dev/null
grep -F 'trillionnium_world_bevy_classic_rts_openra_imported_replay_review_capsule_v1' "$SCRIPT" >/dev/null
grep -F 'openra_imported_replay_review_capsule_v1_json' "$SCRIPT" >/dev/null
grep -F 'bevy_owned_openra_imported_replay_review_capsule_not_openra_runtime_parity' "$SCRIPT" >/dev/null
grep -F 'missing_primary_ledger_review_item' "$SCRIPT" >/dev/null
grep -F 'review_item_sha_tamper' "$SCRIPT" >/dev/null
grep -F 'source_bundle_green_flip' "$SCRIPT" >/dev/null
grep -F 'bundle_artifact_count_mismatch' "$SCRIPT" >/dev/null
grep -F 'review_checklist_failure' "$SCRIPT" >/dev/null
grep -F 'public_launch_boundary_flip' "$SCRIPT" >/dev/null
grep -F 'bevy_openra_binary_replay_compatible == false' "$SCRIPT" >/dev/null
grep -F 'bevy_openra_runtime_parity_claimed == false' "$SCRIPT" >/dev/null
grep -F 'public_launch_ready == false' "$SCRIPT" >/dev/null

grep -F 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REVIEW_CAPSULE_CONTRACT' "$LIB" >/dev/null
grep -F 'native_classic_rts_openra_imported_replay_review_capsule_evidence_json' "$LIB" >/dev/null
grep -F 'openra_imported_replay_review_capsule_v1_json' "$LIB" >/dev/null
grep -F 'review_item_sha_mismatch' "$LIB" >/dev/null
grep -F 'compatibility_boundary_mismatch' "$LIB" >/dev/null
grep -F 'bevy_owned_openra_imported_replay_review_capsule_not_openra_runtime_parity' "$LIB" >/dev/null

grep -F 'classic-rts-openra-imported-replay-review-capsule' "$MAIN" >/dev/null
grep -F 'native_classic_rts_openra_imported_replay_review_capsule_evidence_json' "$MAIN" >/dev/null

grep -F 'check_trillionnium_world_bevy_classic_rts_openra_imported_replay_review_capsule.sh' "$RELEASE_CI" >/dev/null
grep -F 'bevy_classic_rts_openra_imported_replay_review_capsule_script_contract_guard_test.sh' "$RELEASE_CI" >/dev/null
grep -F 'bevy_classic_rts_openra_imported_replay_review_capsule_gate' "$RELEASE_CI" >/dev/null
grep -F 'trillionnium_world_bevy_classic_rts_openra_imported_replay_review_capsule_v1' "$RELEASE_CI" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REVIEW_CAPSULE_SCRIPT_CONTRACT_GUARD_GREEN %s\n' "$SCRIPT"
