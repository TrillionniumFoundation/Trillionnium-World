#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-audit-ledger.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-audit-ledger"
LEDGER="$PREVIEW_DIR/openra-imported-replay-audit-ledger.jsonl"
LEDGER_REPORT="$PREVIEW_DIR/openra-imported-replay-audit-ledger.json"
NEGATIVE_CORPUS="$PREVIEW_DIR/openra-imported-replay-audit-ledger-negative-corpus.json"
IMPORTED_HEADLESS_SUMMARY="$PREVIEW_DIR/openra-imported-headless-comparison-harness.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-openra-imported-replay-audit-ledger "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger_v1"
  and .green == true
  and .adapter_state == "bevy_owned_openra_imported_replay_hash_ledger_not_openra_runtime_parity"
  and .source_contracts.openra_imported_headless_comparison_harness == "trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness_v1"
  and .source_contracts.openra_imported_replay_reducer == "trillionnium_world_bevy_classic_rts_openra_imported_replay_reducer_v1"
  and .source_contracts.openra_order_payload_decoder == "trillionnium_world_bevy_classic_rts_openra_order_payload_decoder_v1"
  and .source_contracts.openra_replay_compat_adapter == "trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter_v1"
  and (.ledger_file_sha256 | type == "string" and length == 64)
  and (.ledger_report_sha256 | type == "string" and length == 64)
  and (.final_ledger_sha256 | type == "string" and length == 64)
  and (.negative_corpus_sha256 | type == "string" and length == 64)
  and .ledger_summary.entry_count >= 29
  and .ledger_summary.decoded_record_count >= 20
  and .ledger_summary.snapshot_count >= 6
  and .ledger_summary.final_frame >= 3000
  and .ledger_summary.winner == "Multi2"
  and .ledger_summary.headless_mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu"
  and (.ledger_summary.source_replay_sha256 | type == "string" and length == 64)
  and (.ledger_summary.decoded_stream_sha256 | type == "string" and length == 64)
  and .ledger_summary.final_ledger_sha256 == .final_ledger_sha256
  and .ledger_summary.negative_case_count >= 7
  and .ledger_summary.detected_negative_case_count == .ledger_summary.negative_case_count
  and .source_contract_gate == true
  and .source_green_gate == true
  and .record_parse_gate == true
  and .decoded_stream_gate == true
  and .snapshot_parse_gate == true
  and .snapshot_gate == true
  and .headless_alignment_gate == true
  and .ledger_validation_gate == true
  and .ledger_file_gate == true
  and .ledger_report_gate == true
  and .negative_corpus_gate == true
  and .compatibility_boundary_gate == true
  and .openra_imported_replay_audit_ledger_gate == true
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

jq -s -e '
  length >= 29
  and all(.[]; .entry_schema == "openra_imported_replay_audit_ledger_entry_v1_json")
  and (.[0].previous_ledger_sha256 == "GENESIS")
  and all(.[]; (.entry_sha256 | type == "string" and length == 64))
  and (map(.entry_kind) | index("decoded_order_record") != null)
  and (map(.entry_kind) | index("reducer_snapshot") != null)
  and (map(.entry_kind) | index("headless_summary") != null)
  and (.[length - 1].entry_kind == "headless_summary")
  and (.[length - 1].payload.winner == "Multi2")
  and (.[length - 1].payload.headless_mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu")
  and (.[length - 1].payload.decoded_stream_sha256 | type == "string" and length == 64)
' "$LEDGER" >/dev/null

jq -e '
  .ledger_schema == "openra_imported_replay_audit_ledger_v1_json"
  and (.ledger_file_sha256 | type == "string" and length == 64)
  and (.final_ledger_sha256 | type == "string" and length == 64)
  and .summary.entry_count >= 29
  and .summary.decoded_record_count >= 20
  and .summary.snapshot_count >= 6
  and .summary.final_frame >= 3000
  and .summary.winner == "Multi2"
  and .summary.headless_mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu"
  and (.summary.source_replay_sha256 | type == "string" and length == 64)
  and (.summary.decoded_stream_sha256 | type == "string" and length == 64)
  and .summary.detected_negative_case_count == .summary.negative_case_count
' "$LEDGER_REPORT" >/dev/null

jq -e '
  length >= 7
  and all(.[]; .detected == true)
  and (map(.case) | index("dropped_ledger_entry") != null)
  and (map(.case) | index("previous_sha_break") != null)
  and (map(.case) | index("payload_tamper_without_rehash") != null)
  and (map(.case) | index("frame_regression") != null)
  and (map(.case) | index("final_sha_mismatch") != null)
  and (map(.case) | index("decoded_stream_sha_mismatch") != null)
  and (map(.case) | index("winner_mismatch") != null)
' "$NEGATIVE_CORPUS" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness_v1"
  and .green == true
  and .comparison_summary.comparison_count >= 17
  and .comparison_summary.mismatch_count == 0
  and .comparison_summary.winner == "Multi2"
  and (.comparison_summary.decoded_stream_sha256 | type == "string" and length == 64)
' "$IMPORTED_HEADLESS_SUMMARY" >/dev/null

test -s "$LEDGER"
test -s "$LEDGER_REPORT"
test -s "$NEGATIVE_CORPUS"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_AUDIT_LEDGER_GREEN %s %s %s\n' "$SUMMARY" "$LEDGER" "$LEDGER_REPORT"
