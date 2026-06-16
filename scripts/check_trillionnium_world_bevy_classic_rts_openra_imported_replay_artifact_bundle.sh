#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-artifact-bundle.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-artifact-bundle"
BUNDLE="$PREVIEW_DIR/openra-imported-replay-artifact-bundle.json"
NEGATIVE_CORPUS="$PREVIEW_DIR/openra-imported-replay-artifact-bundle-negative-corpus.json"
REPRO_SUMMARY="$PREVIEW_DIR/openra-imported-replay-repro-manifest.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-openra-imported-replay-artifact-bundle "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_replay_artifact_bundle_v1"
  and .green == true
  and .adapter_state == "bevy_owned_openra_imported_replay_artifact_bundle_not_openra_runtime_parity"
  and .source_contracts.openra_imported_replay_repro_manifest == "trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest_v1"
  and .source_contracts.openra_imported_replay_audit_ledger == "trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger_v1"
  and .source_contracts.openra_imported_headless_comparison_harness == "trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness_v1"
  and .source_contracts.openra_order_payload_decoder == "trillionnium_world_bevy_classic_rts_openra_order_payload_decoder_v1"
  and .source_contracts.openra_replay_compat_adapter == "trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter_v1"
  and (.bundle_sha256 | type == "string" and length == 64)
  and (.negative_corpus_sha256 | type == "string" and length == 64)
  and .bundle_summary.artifact_count >= 14
  and .bundle_summary.verified_artifact_count == .bundle_summary.artifact_count
  and .bundle_summary.ledger_entry_count >= 29
  and .bundle_summary.decoded_record_count >= 20
  and .bundle_summary.snapshot_count >= 6
  and .bundle_summary.final_frame >= 3000
  and .bundle_summary.winner == "Multi2"
  and (.bundle_summary.source_replay_sha256 | type == "string" and length == 64)
  and (.bundle_summary.decoded_stream_sha256 | type == "string" and length == 64)
  and .bundle_summary.primary_final_ledger_sha256 == .bundle_summary.rerun_final_ledger_sha256
  and .bundle_summary.primary_ledger_file_sha256 == .bundle_summary.rerun_ledger_file_sha256
  and .bundle_summary.negative_case_count >= 5
  and .bundle_summary.detected_negative_case_count == .bundle_summary.negative_case_count
  and .source_contract_gate == true
  and .source_green_gate == true
  and .artifact_index_gate == true
  and .paired_artifact_gate == true
  and .bundle_validation_gate == true
  and .bundle_file_gate == true
  and .negative_corpus_gate == true
  and .compatibility_boundary_gate == true
  and .openra_imported_replay_artifact_bundle_gate == true
  and .bevy_openra_imported_replay_artifact_bundle_claimed == true
  and .bevy_openra_imported_replay_repro_manifest_claimed == true
  and .bevy_openra_imported_replay_audit_ledger_claimed == true
  and .bevy_openra_imported_headless_comparison_harness_claimed == true
  and .bevy_openra_imported_replay_reducer_claimed == true
  and .bevy_openra_order_payload_decoder_claimed == true
  and .bevy_openra_native_order_payload_decoder_claimed == false
  and .bevy_openra_replay_summary_adapter_claimed == true
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
  .bundle_schema == "openra_imported_replay_artifact_bundle_v1_json"
  and .source_contracts.openra_imported_replay_repro_manifest == "trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest_v1"
  and .summary.artifact_count >= 14
  and .summary.verified_artifact_count == .summary.artifact_count
  and .summary.ledger_entry_count >= 29
  and .summary.winner == "Multi2"
  and .summary.primary_final_ledger_sha256 == .summary.rerun_final_ledger_sha256
  and .summary.primary_ledger_file_sha256 == .summary.rerun_ledger_file_sha256
  and .summary.detected_negative_case_count == .summary.negative_case_count
  and ((.artifacts | length) == .summary.artifact_count)
  and all(.artifacts[]; .schema_gate == true and (.sha256 | type == "string" and length == 64) and .byte_len > 0)
  and (.artifacts | map(.artifact_id) | index("repro_manifest") != null)
  and (.artifacts | map(.artifact_id) | index("primary_ledger") != null)
  and (.artifacts | map(.artifact_id) | index("rerun_ledger") != null)
  and (.artifacts | map(.artifact_id) | index("primary_imported_headless_summary") != null)
  and (.artifacts | map(.artifact_id) | index("rerun_imported_headless_summary") != null)
' "$BUNDLE" >/dev/null

jq -e '
  length >= 5
  and all(.[]; .detected == true)
  and (map(.case) | index("dropped_primary_ledger_artifact") != null)
  and (map(.case) | index("artifact_sha_tamper") != null)
  and (map(.case) | index("repro_diff_mismatch_count") != null)
  and (map(.case) | index("source_repro_green_flip") != null)
  and (map(.case) | index("public_launch_boundary_flip") != null)
' "$NEGATIVE_CORPUS" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest_v1"
  and .green == true
  and .openra_imported_replay_repro_manifest_gate == true
  and .repro_summary.mismatch_count == 0
  and .repro_summary.winner == "Multi2"
' "$REPRO_SUMMARY" >/dev/null

test -s "$BUNDLE"
test -s "$NEGATIVE_CORPUS"
test -s "$REPRO_SUMMARY"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_ARTIFACT_BUNDLE_GREEN %s %s\n' "$SUMMARY" "$BUNDLE"
