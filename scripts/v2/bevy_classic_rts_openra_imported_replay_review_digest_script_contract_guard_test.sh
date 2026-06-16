#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_review_digest.sh"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
LIB="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

test -x "$SCRIPT"

grep -F 'classic-rts-openra-imported-replay-review-digest' "$SCRIPT" >/dev/null
grep -F 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REVIEW_DIGEST_GREEN' "$SCRIPT" >/dev/null
grep -F 'trillionnium_world_bevy_classic_rts_openra_imported_replay_review_digest_v1' "$SCRIPT" >/dev/null
grep -F 'openra_imported_replay_review_digest_v1_json' "$SCRIPT" >/dev/null
grep -F 'bevy_owned_openra_imported_replay_review_digest_not_openra_runtime_parity' "$SCRIPT" >/dev/null
grep -F 'source_review_receipt_green_flip' "$SCRIPT" >/dev/null
grep -F 'review_digest_receipt_schema_tamper' "$SCRIPT" >/dev/null
grep -F 'review_digest_assertion_failure' "$SCRIPT" >/dev/null
grep -F 'review_digest_negative_detection_flip' "$SCRIPT" >/dev/null
grep -F 'review_digest_review_item_count_drift' "$SCRIPT" >/dev/null
grep -F 'review_digest_winner_drift' "$SCRIPT" >/dev/null
grep -F 'review_digest_public_launch_boundary_flip' "$SCRIPT" >/dev/null
grep -F 'bevy_openra_binary_replay_compatible == false' "$SCRIPT" >/dev/null
grep -F 'bevy_openra_runtime_parity_claimed == false' "$SCRIPT" >/dev/null
grep -F 'public_launch_ready == false' "$SCRIPT" >/dev/null

grep -F 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REVIEW_DIGEST_CONTRACT' "$LIB" >/dev/null
grep -F 'native_classic_rts_openra_imported_replay_review_digest_evidence_json' "$LIB" >/dev/null
grep -F 'openra_imported_replay_review_digest_v1_json' "$LIB" >/dev/null
grep -F 'source_review_receipt_gate_mismatch' "$LIB" >/dev/null
grep -F 'receipt_schema_mismatch' "$LIB" >/dev/null
grep -F 'receipt_assertion_mismatch' "$LIB" >/dev/null
grep -F 'negative_corpus_mismatch' "$LIB" >/dev/null
grep -F 'compatibility_boundary_mismatch' "$LIB" >/dev/null
grep -F 'bevy_owned_openra_imported_replay_review_digest_not_openra_runtime_parity' "$LIB" >/dev/null

grep -F 'classic-rts-openra-imported-replay-review-digest' "$MAIN" >/dev/null
grep -F 'native_classic_rts_openra_imported_replay_review_digest_evidence_json' "$MAIN" >/dev/null

grep -F 'check_trillionnium_world_bevy_classic_rts_openra_imported_replay_review_digest.sh' "$RELEASE_CI" >/dev/null
grep -F 'bevy_classic_rts_openra_imported_replay_review_digest_script_contract_guard_test.sh' "$RELEASE_CI" >/dev/null
grep -F 'bevy_classic_rts_openra_imported_replay_review_digest_gate' "$RELEASE_CI" >/dev/null
grep -F 'trillionnium_world_bevy_classic_rts_openra_imported_replay_review_digest_v1' "$RELEASE_CI" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REVIEW_DIGEST_SCRIPT_CONTRACT_GUARD_GREEN %s\n' "$SCRIPT"
