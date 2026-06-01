#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-review-receipt.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-review-receipt"
RECEIPT="$PREVIEW_DIR/openra-imported-replay-review-receipt.json"
NEGATIVE_CORPUS="$PREVIEW_DIR/openra-imported-replay-review-receipt-negative-corpus.json"
CAPSULE_SUMMARY="$PREVIEW_DIR/openra-imported-replay-review-capsule-summary.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-openra-imported-replay-review-receipt "$PREVIEW_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_replay_review_receipt_v1"
  and .green == true
  and .adapter_state == "bevy_owned_openra_imported_replay_review_receipt_not_openra_runtime_parity"
  and .source_contracts.openra_imported_replay_review_capsule == "trillionnium_world_bevy_classic_rts_openra_imported_replay_review_capsule_v1"
  and .source_contracts.openra_imported_replay_artifact_bundle == "trillionnium_world_bevy_classic_rts_openra_imported_replay_artifact_bundle_v1"
  and .source_contracts.openra_imported_replay_repro_manifest == "trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest_v1"
  and .source_contracts.openra_imported_replay_audit_ledger == "trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger_v1"
  and .source_contracts.openra_imported_headless_comparison_harness == "trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness_v1"
  and .source_contracts.openra_order_payload_decoder == "trillionnium_world_bevy_classic_rts_openra_order_payload_decoder_v1"
  and .source_contracts.openra_replay_compat_adapter == "trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter_v1"
  and (.review_receipt_sha256 | type == "string" and length == 64)
  and (.negative_corpus_sha256 | type == "string" and length == 64)
  and .receipt_summary.review_item_count >= 14
  and .receipt_summary.required_review_item_count >= 14
  and .receipt_summary.checklist_count >= 5
  and .receipt_summary.checklist_passed_count == .receipt_summary.checklist_count
  and .receipt_summary.receipt_assertion_count >= 6
  and .receipt_summary.receipt_assertion_passed_count == .receipt_summary.receipt_assertion_count
  and .receipt_summary.artifact_count >= 14
  and .receipt_summary.verified_artifact_count == .receipt_summary.artifact_count
  and .receipt_summary.ledger_entry_count >= 29
  and .receipt_summary.decoded_record_count >= 20
  and .receipt_summary.snapshot_count >= 6
  and .receipt_summary.final_frame >= 3000
  and .receipt_summary.winner == "Multi2"
  and .receipt_summary.handoff_state == "ready_for_local_review_not_public_launch"
  and (.receipt_summary.source_replay_sha256 | type == "string" and length == 64)
  and (.receipt_summary.decoded_stream_sha256 | type == "string" and length == 64)
  and .receipt_summary.primary_final_ledger_sha256 == .receipt_summary.rerun_final_ledger_sha256
  and .receipt_summary.primary_ledger_file_sha256 == .receipt_summary.rerun_ledger_file_sha256
  and (.receipt_summary.review_capsule_sha256 | type == "string" and length == 64)
  and (.receipt_summary.review_capsule_manifest_sha256 | type == "string" and length == 64)
  and .receipt_summary.review_capsule_manifest_sha256 == .receipt_summary.review_capsule_sha256
  and (.receipt_summary.review_capsule_file_sha256 | type == "string" and length == 64)
  and (.receipt_summary.review_capsule_negative_corpus_sha256 | type == "string" and length == 64)
  and (.receipt_summary.artifact_bundle_sha256 | type == "string" and length == 64)
  and .source_contract_gate == true
  and .source_review_capsule_gate == true
  and .capsule_manifest_gate == true
  and .review_item_gate == true
  and .checklist_gate == true
  and .negative_source_gate == true
  and .receipt_assertion_gate == true
  and .receipt_validation_gate == true
  and .receipt_file_gate == true
  and .negative_corpus_gate == true
  and .compatibility_boundary_gate == true
  and .negative_case_count >= 7
  and .detected_negative_case_count == .negative_case_count
  and .openra_imported_replay_review_receipt_gate == true
  and .bevy_openra_imported_replay_review_receipt_claimed == true
  and .bevy_openra_imported_replay_review_capsule_claimed == true
  and .bevy_openra_imported_replay_artifact_bundle_claimed == true
  and .bevy_openra_imported_replay_repro_manifest_claimed == true
  and .bevy_openra_imported_replay_audit_ledger_claimed == true
  and .bevy_openra_imported_headless_comparison_harness_claimed == true
  and .bevy_openra_order_payload_decoder_claimed == true
  and .bevy_openra_native_order_payload_decoder_claimed == false
  and .bevy_openra_binary_replay_compatible == false
  and .bevy_openra_order_serializer_claimed == false
  and .bevy_openra_network_order_stream_claimed == false
  and .bevy_openra_replay_file_claimed == false
  and .bevy_openra_headless_client_match_claimed == false
  and .bevy_openra_runtime_parity_claimed == false
  and .bevy_openra_parity_claimed == false
  and .public_launch_ready == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

jq -e '
  .review_receipt_schema == "openra_imported_replay_review_receipt_v1_json"
  and .source_contracts.openra_imported_replay_review_capsule == "trillionnium_world_bevy_classic_rts_openra_imported_replay_review_capsule_v1"
  and .receipt_summary.review_item_count >= 14
  and .receipt_summary.checklist_count == .receipt_summary.checklist_passed_count
  and .receipt_summary.receipt_assertion_count == .receipt_summary.receipt_assertion_passed_count
  and .receipt_summary.artifact_count >= 14
  and .receipt_summary.verified_artifact_count == .receipt_summary.artifact_count
  and .receipt_summary.winner == "Multi2"
  and .receipt_summary.primary_final_ledger_sha256 == .receipt_summary.rerun_final_ledger_sha256
  and .receipt_summary.primary_ledger_file_sha256 == .receipt_summary.rerun_ledger_file_sha256
  and all(.receipt_assertions[]; .passed == true and (.assertion_id | type == "string" and length > 0))
  and (.receipt_assertions | map(.assertion_id) | index("source_review_capsule_green") != null)
  and (.receipt_assertions | map(.assertion_id) | index("capsule_hash_matches_manifest") != null)
  and (.receipt_assertions | map(.assertion_id) | index("review_items_complete") != null)
  and (.receipt_assertions | map(.assertion_id) | index("review_checklist_green") != null)
  and (.receipt_assertions | map(.assertion_id) | index("negative_corpus_detected") != null)
  and (.receipt_assertions | map(.assertion_id) | index("compatibility_boundaries_unclaimed") != null)
  and .handoff_boundaries.bevy_openra_binary_replay_compatible == false
  and .handoff_boundaries.bevy_openra_runtime_parity_claimed == false
  and .handoff_boundaries.public_launch_ready == false
' "$RECEIPT" >/dev/null

jq -e '
  length >= 7
  and all(.[]; .detected == true)
  and (map(.case) | index("source_review_capsule_green_flip") != null)
  and (map(.case) | index("review_receipt_item_sha_tamper") != null)
  and (map(.case) | index("missing_rerun_ledger_review_item") != null)
  and (map(.case) | index("review_receipt_checklist_failure") != null)
  and (map(.case) | index("review_receipt_negative_detection_flip") != null)
  and (map(.case) | index("review_receipt_winner_drift") != null)
  and (map(.case) | index("review_receipt_public_launch_boundary_flip") != null)
' "$NEGATIVE_CORPUS" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_replay_review_capsule_v1"
  and .green == true
  and .openra_imported_replay_review_capsule_gate == true
  and .review_summary.winner == "Multi2"
  and .review_summary.checklist_count == .review_summary.checklist_passed_count
' "$CAPSULE_SUMMARY" >/dev/null

test -s "$RECEIPT"
test -s "$NEGATIVE_CORPUS"
test -s "$CAPSULE_SUMMARY"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REVIEW_RECEIPT_GREEN %s %s\n' "$SUMMARY" "$RECEIPT"
