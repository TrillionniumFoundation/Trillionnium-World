#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_review_receipt.sh"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
LIB="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

test -x "$SCRIPT"

grep -F 'classic-rts-openra-imported-replay-review-receipt' "$SCRIPT" >/dev/null
grep -F 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REVIEW_RECEIPT_GREEN' "$SCRIPT" >/dev/null
grep -F 'trillionnium_world_bevy_classic_rts_openra_imported_replay_review_receipt_v1' "$SCRIPT" >/dev/null
grep -F 'openra_imported_replay_review_receipt_v1_json' "$SCRIPT" >/dev/null
grep -F 'bevy_owned_openra_imported_replay_review_receipt_not_openra_runtime_parity' "$SCRIPT" >/dev/null
grep -F 'source_review_capsule_green_flip' "$SCRIPT" >/dev/null
grep -F 'review_receipt_item_sha_tamper' "$SCRIPT" >/dev/null
grep -F 'missing_rerun_ledger_review_item' "$SCRIPT" >/dev/null
grep -F 'review_receipt_checklist_failure' "$SCRIPT" >/dev/null
grep -F 'review_receipt_negative_detection_flip' "$SCRIPT" >/dev/null
grep -F 'review_receipt_winner_drift' "$SCRIPT" >/dev/null
grep -F 'review_receipt_public_launch_boundary_flip' "$SCRIPT" >/dev/null
grep -F 'bevy_openra_binary_replay_compatible == false' "$SCRIPT" >/dev/null
grep -F 'bevy_openra_runtime_parity_claimed == false' "$SCRIPT" >/dev/null
grep -F 'public_launch_ready == false' "$SCRIPT" >/dev/null

grep -F 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REVIEW_RECEIPT_CONTRACT' "$LIB" >/dev/null
grep -F 'native_classic_rts_openra_imported_replay_review_receipt_evidence_json' "$LIB" >/dev/null
grep -F 'TRNM_OPENRA_IMPORTED_REPLAY_REVIEW_CAPSULE_SUMMARY' "$LIB" >/dev/null
grep -F 'TRNM_OPENRA_IMPORTED_REPLAY_REVIEW_CAPSULE_DIR' "$LIB" >/dev/null
grep -F 'openra_imported_replay_review_receipt_v1_json' "$LIB" >/dev/null
grep -F 'source_review_capsule_gate_mismatch' "$LIB" >/dev/null
grep -F 'review_item_sha_mismatch' "$LIB" >/dev/null
grep -F 'negative_corpus_mismatch' "$LIB" >/dev/null
grep -F 'compatibility_boundary_mismatch' "$LIB" >/dev/null
grep -F 'bevy_owned_openra_imported_replay_review_receipt_not_openra_runtime_parity' "$LIB" >/dev/null

grep -F 'classic-rts-openra-imported-replay-review-receipt' "$MAIN" >/dev/null
grep -F 'native_classic_rts_openra_imported_replay_review_receipt_evidence_json' "$MAIN" >/dev/null

grep -F 'check_trillionnium_world_bevy_classic_rts_openra_imported_replay_review_receipt.sh' "$RELEASE_CI" >/dev/null
grep -F 'TRNM_OPENRA_IMPORTED_REPLAY_REVIEW_CAPSULE_SUMMARY' "$RELEASE_CI" >/dev/null
grep -F 'TRNM_OPENRA_IMPORTED_REPLAY_REVIEW_CAPSULE_DIR' "$RELEASE_CI" >/dev/null
grep -F 'bevy_classic_rts_openra_imported_replay_review_receipt_script_contract_guard_test.sh' "$RELEASE_CI" >/dev/null
grep -F 'bevy_classic_rts_openra_imported_replay_review_receipt_gate' "$RELEASE_CI" >/dev/null
grep -F 'trillionnium_world_bevy_classic_rts_openra_imported_replay_review_receipt_v1' "$RELEASE_CI" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REVIEW_RECEIPT_SCRIPT_CONTRACT_GUARD_GREEN %s\n' "$SCRIPT"
