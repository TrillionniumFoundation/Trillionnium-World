#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-repro-manifest.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-repro-manifest"
MANIFEST="$PREVIEW_DIR/openra-imported-replay-repro-manifest.json"
DIFF="$PREVIEW_DIR/openra-imported-replay-repro-diff.json"
NEGATIVE_CORPUS="$PREVIEW_DIR/openra-imported-replay-repro-manifest-negative-corpus.json"
PRIMARY_SUMMARY="$PREVIEW_DIR/openra-imported-replay-audit-ledger-primary.json"
RERUN_SUMMARY="$PREVIEW_DIR/openra-imported-replay-audit-ledger-rerun.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-openra-imported-replay-repro-manifest "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest_v1"
  and .green == true
  and .adapter_state == "bevy_owned_openra_imported_replay_repro_manifest_not_openra_runtime_parity"
  and .source_contracts.openra_imported_replay_audit_ledger == "trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger_v1"
  and .source_contracts.openra_imported_headless_comparison_harness == "trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness_v1"
  and .source_contracts.openra_order_payload_decoder == "trillionnium_world_bevy_classic_rts_openra_order_payload_decoder_v1"
  and .source_contracts.openra_replay_compat_adapter == "trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter_v1"
  and (.manifest_sha256 | type == "string" and length == 64)
  and (.diff_sha256 | type == "string" and length == 64)
  and (.negative_corpus_sha256 | type == "string" and length == 64)
  and .repro_summary.comparison_count >= 20
  and .repro_summary.aligned_count == .repro_summary.comparison_count
  and .repro_summary.mismatch_count == 0
  and .repro_summary.ledger_entry_count >= 29
  and .repro_summary.decoded_record_count >= 20
  and .repro_summary.snapshot_count >= 6
  and .repro_summary.final_frame >= 3000
  and .repro_summary.winner == "Multi2"
  and .repro_summary.headless_mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu"
  and (.repro_summary.source_replay_sha256 | type == "string" and length == 64)
  and (.repro_summary.decoded_stream_sha256 | type == "string" and length == 64)
  and .repro_summary.primary_ledger_file_sha256 == .repro_summary.rerun_ledger_file_sha256
  and .repro_summary.primary_final_ledger_sha256 == .repro_summary.rerun_final_ledger_sha256
  and .repro_summary.primary_negative_corpus_sha256 == .repro_summary.rerun_negative_corpus_sha256
  and .repro_summary.negative_case_count >= 5
  and .repro_summary.detected_negative_case_count == .repro_summary.negative_case_count
  and .source_contract_gate == true
  and .source_green_gate == true
  and .stable_summary_gate == true
  and .artifact_read_gate == true
  and .diff_gate == true
  and .manifest_gate == true
  and .negative_corpus_gate == true
  and .compatibility_boundary_gate == true
  and .openra_imported_replay_repro_manifest_gate == true
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
  .manifest_schema == "openra_imported_replay_repro_manifest_v1_json"
  and .source_contracts.openra_imported_replay_audit_ledger == "trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger_v1"
  and .summary.comparison_count >= 20
  and .summary.aligned_count == .summary.comparison_count
  and .summary.mismatch_count == 0
  and .summary.ledger_entry_count >= 29
  and .summary.winner == "Multi2"
  and .summary.headless_mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu"
  and .summary.primary_ledger_file_sha256 == .summary.rerun_ledger_file_sha256
  and .summary.primary_final_ledger_sha256 == .summary.rerun_final_ledger_sha256
  and .summary.primary_negative_corpus_sha256 == .summary.rerun_negative_corpus_sha256
  and .summary.detected_negative_case_count == .summary.negative_case_count
' "$MANIFEST" >/dev/null

jq -e '
  .diff_schema == "openra_imported_replay_repro_diff_v1_json"
  and .comparison_count >= 20
  and .aligned_count == .comparison_count
  and .mismatch_count == 0
  and (.mismatches | length == 0)
  and all(.comparisons[]; .matched == true)
  and (.comparisons | map(.field) | index("ledger_file_bytes_sha256") != null)
  and (.comparisons | map(.field) | index("negative_corpus_semantic_sha256") != null)
' "$DIFF" >/dev/null

jq -e '
  length >= 5
  and all(.[]; .detected == true)
  and (map(.case) | index("final_ledger_sha_mismatch") != null)
  and (map(.case) | index("decoded_stream_sha_mismatch") != null)
  and (map(.case) | index("winner_mismatch") != null)
  and (map(.case) | index("negative_case_detection_mismatch") != null)
  and (map(.case) | index("green_flag_mismatch") != null)
' "$NEGATIVE_CORPUS" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger_v1"
  and .green == true
  and .openra_imported_replay_audit_ledger_gate == true
  and .ledger_summary.entry_count >= 29
  and .ledger_summary.winner == "Multi2"
' "$PRIMARY_SUMMARY" "$RERUN_SUMMARY" >/dev/null

PRIMARY_LEDGER="$(jq -r '.ledger_path' "$PRIMARY_SUMMARY")"
RERUN_LEDGER="$(jq -r '.ledger_path' "$RERUN_SUMMARY")"
PRIMARY_NEGATIVE="$(jq -r '.negative_corpus_path' "$PRIMARY_SUMMARY")"
RERUN_NEGATIVE="$(jq -r '.negative_corpus_path' "$RERUN_SUMMARY")"

test -s "$MANIFEST"
test -s "$DIFF"
test -s "$NEGATIVE_CORPUS"
test -s "$PRIMARY_SUMMARY"
test -s "$RERUN_SUMMARY"
cmp -s "$PRIMARY_LEDGER" "$RERUN_LEDGER"
cmp -s "$PRIMARY_NEGATIVE" "$RERUN_NEGATIVE"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REPRO_MANIFEST_GREEN %s %s %s\n' "$SUMMARY" "$MANIFEST" "$DIFF"
