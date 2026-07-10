#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-review-capsule.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-review-capsule"
CAPSULE="$PREVIEW_DIR/openra-imported-replay-review-capsule.json"
NEGATIVE_CORPUS="$PREVIEW_DIR/openra-imported-replay-review-capsule-negative-corpus.json"
BUNDLE_SUMMARY="$PREVIEW_DIR/openra-imported-replay-artifact-bundle-summary.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-openra-imported-replay-review-capsule "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_replay_review_capsule_v1"
  and .green == true
  and .adapter_state == "bevy_owned_openra_imported_replay_review_capsule_not_openra_runtime_parity"
  and .source_contracts.openra_imported_replay_artifact_bundle == "trillionnium_world_bevy_classic_rts_openra_imported_replay_artifact_bundle_v1"
  and .source_contracts.openra_imported_replay_repro_manifest == "trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest_v1"
  and .source_contracts.openra_imported_replay_audit_ledger == "trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger_v1"
  and .source_contracts.openra_imported_headless_comparison_harness == "trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness_v1"
  and .source_contracts.openra_order_payload_decoder == "trillionnium_world_bevy_classic_rts_openra_order_payload_decoder_v1"
  and .source_contracts.openra_replay_compat_adapter == "trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter_v1"
  and (.review_capsule_sha256 | type == "string" and length == 64)
  and (.negative_corpus_sha256 | type == "string" and length == 64)
  and .review_summary.review_item_count >= 14
  and .review_summary.required_review_item_count >= 14
  and .review_summary.checklist_count >= 5
  and .review_summary.checklist_passed_count == .review_summary.checklist_count
  and .review_summary.artifact_count >= 14
  and .review_summary.verified_artifact_count == .review_summary.artifact_count
  and .review_summary.ledger_entry_count >= 29
  and .review_summary.decoded_record_count >= 20
  and .review_summary.snapshot_count >= 6
  and .review_summary.final_frame >= 3000
  and .review_summary.winner == "Multi2"
  and (.review_summary.source_replay_sha256 | type == "string" and length == 64)
  and (.review_summary.decoded_stream_sha256 | type == "string" and length == 64)
  and .review_summary.primary_final_ledger_sha256 == .review_summary.rerun_final_ledger_sha256
  and .review_summary.primary_ledger_file_sha256 == .review_summary.rerun_ledger_file_sha256
  and (.review_summary.bundle_sha256 | type == "string" and length == 64)
  and (.review_summary.bundle_file_sha256 | type == "string" and length == 64)
  and (.review_summary.bundle_negative_corpus_sha256 | type == "string" and length == 64)
  and .source_contract_gate == true
  and .source_bundle_gate == true
  and .bundle_manifest_gate == true
  and .negative_source_gate == true
  and .review_item_gate == true
  and .checklist_gate == true
  and .capsule_validation_gate == true
  and .capsule_file_gate == true
  and .negative_corpus_gate == true
  and .compatibility_boundary_gate == true
  and .negative_case_count >= 6
  and .detected_negative_case_count == .negative_case_count
  and .openra_imported_replay_review_capsule_gate == true
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
  .review_capsule_schema == "openra_imported_replay_review_capsule_v1_json"
  and .source_contracts.openra_imported_replay_artifact_bundle == "trillionnium_world_bevy_classic_rts_openra_imported_replay_artifact_bundle_v1"
  and .review_summary.review_item_count >= 14
  and .review_summary.checklist_count == .review_summary.checklist_passed_count
  and .review_summary.artifact_count >= 14
  and .review_summary.verified_artifact_count == .review_summary.artifact_count
  and .review_summary.winner == "Multi2"
  and .review_summary.primary_final_ledger_sha256 == .review_summary.rerun_final_ledger_sha256
  and .review_summary.primary_ledger_file_sha256 == .review_summary.rerun_ledger_file_sha256
  and ((.review_items | length) == .review_summary.review_item_count)
  and all(.review_items[]; .required_for_review == true and .present == true and .schema_gate == true and (.sha256 | type == "string" and length == 64) and .byte_len > 0)
  and (.review_items | map(.item_id) | index("repro_manifest") != null)
  and (.review_items | map(.item_id) | index("primary_ledger") != null)
  and (.review_items | map(.item_id) | index("rerun_ledger") != null)
  and (.review_items | map(.item_id) | index("primary_imported_headless_summary") != null)
  and (.review_items | map(.item_id) | index("rerun_imported_headless_summary") != null)
  and all(.review_checklist[]; .passed == true)
  and .boundary_claims.bevy_openra_binary_replay_compatible == false
  and .boundary_claims.bevy_openra_runtime_parity_claimed == false
  and .boundary_claims.public_launch_ready == false
' "$CAPSULE" >/dev/null

jq -e '
  length >= 6
  and all(.[]; .detected == true)
  and (map(.case) | index("missing_primary_ledger_review_item") != null)
  and (map(.case) | index("review_item_sha_tamper") != null)
  and (map(.case) | index("source_bundle_green_flip") != null)
  and (map(.case) | index("bundle_artifact_count_mismatch") != null)
  and (map(.case) | index("review_checklist_failure") != null)
  and (map(.case) | index("public_launch_boundary_flip") != null)
' "$NEGATIVE_CORPUS" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_replay_artifact_bundle_v1"
  and .green == true
  and .openra_imported_replay_artifact_bundle_gate == true
  and .bundle_summary.winner == "Multi2"
  and .bundle_summary.verified_artifact_count == .bundle_summary.artifact_count
' "$BUNDLE_SUMMARY" >/dev/null

test -s "$CAPSULE"
test -s "$NEGATIVE_CORPUS"
test -s "$BUNDLE_SUMMARY"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REVIEW_CAPSULE_GREEN %s %s\n' "$SUMMARY" "$CAPSULE"
