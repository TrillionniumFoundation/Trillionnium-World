#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-headless-comparison-harness.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-headless-comparison-harness"
COMPARISON="$PREVIEW_DIR/openra-headless-comparison-harness.json"
MISMATCH_MATRIX="$PREVIEW_DIR/openra-headless-comparison-mismatch-matrix.json"
REDUCER="$PREVIEW_DIR/openra-order-replay-reducer/openra-order-replay-reducer.json"
REPLAY_ADAPTER="$PREVIEW_DIR/openra-replay-compat-adapter/openra-replay-summary-adapter.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-openra-headless-comparison-harness "$PREVIEW_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_headless_comparison_harness_v1"
  and .green == true
  and .adapter_state == "bevy_owned_openra_style_reducer_compared_with_headless_replay_harness"
  and .source_contracts.openra_order_replay_reducer == "trillionnium_world_bevy_classic_rts_openra_order_replay_reducer_v1"
  and .source_contracts.openra_replay_compat_adapter == "trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter_v1"
  and (.comparison_sha256 | type == "string" and length == 64)
  and (.mismatch_matrix_sha256 | type == "string" and length == 64)
  and .comparison_summary.comparison_schema == "openra_headless_comparison_harness_v1_json"
  and .comparison_summary.comparison_count >= 12
  and .comparison_summary.aligned_count == .comparison_summary.comparison_count
  and .comparison_summary.mismatch_count == 0
  and .comparison_summary.negative_case_count >= 6
  and .comparison_summary.detected_negative_case_count == .comparison_summary.negative_case_count
  and .comparison_summary.final_frame >= 3000
  and .comparison_summary.replay_final_tick == .comparison_summary.final_frame
  and .comparison_summary.winner == "Multi2"
  and .comparison_summary.headless_mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu"
  and (.comparison_summary.reducer_state_sha256 | type == "string" and length == 64)
  and (.comparison_summary.replay_adapter_sha256 | type == "string" and length == 64)
  and (.comparison_summary.source_stream_sha256 | type == "string" and length == 64)
  and .source_contract_gate == true
  and .source_green_gate == true
  and .source_artifact_gate == true
  and .snapshot_parse_gate == true
  and .comparison_alignment_gate == true
  and .headless_comparison_gate == true
  and .mismatch_matrix_gate == true
  and .comparison_file_gate == true
  and .compatibility_boundary_gate == true
  and .openra_headless_comparison_harness_gate == true
  and .bevy_openra_headless_comparison_harness_claimed == true
  and .bevy_openra_order_replay_reducer_claimed == true
  and .bevy_openra_order_serializer_fixture_claimed == true
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
  .comparison_schema == "openra_headless_comparison_harness_v1_json"
  and (.comparisons | length) >= 12
  and all(.comparisons[]; .aligned == true)
  and .summary.comparison_count == (.comparisons | length)
  and .summary.aligned_count == .summary.comparison_count
  and .summary.mismatch_count == 0
  and .summary.negative_case_count >= 6
  and .summary.final_frame >= 3000
  and .summary.winner == "Multi2"
  and .summary.headless_mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu"
  and (.summary.reducer_trace_sha256 | type == "string" and length == 64)
  and (.summary.source_stream_sha256 | type == "string" and length == 64)
' "$COMPARISON" >/dev/null

jq -e '
  length >= 6
  and all(.[]; .mismatch_detected == true)
  and (map(.case) | index("winner_mismatch_probe") != null)
  and (map(.case) | index("frame_regression_probe") != null)
  and (map(.case) | index("map_uid_mismatch_probe") != null)
  and (map(.case) | index("rules_uid_mismatch_probe") != null)
  and (map(.case) | index("match_state_mismatch_probe") != null)
  and (map(.case) | index("snapshot_drop_probe") != null)
  and (map(.case) | index("headless_wgpu_toggle_probe") != null)
' "$MISMATCH_MATRIX" >/dev/null

jq -e '
  .state_schema == "openra_order_stream_reducer_state_v1_json"
  and .phase == "replay_complete"
  and .final_frame >= 3000
  and .winner == "Multi2"
  and (.losers | length) == 3
  and .final_controlled_beacons == 2
  and (.final_match_result_state | contains("Multi2"))
' "$REDUCER" >/dev/null

jq -e '
  .adapter_schema == "openra_replay_summary_adapter_v1_json"
  and .outcome.final_tick >= 3000
  and .outcome.winner == "Multi2"
  and .outcome.loser_count == 3
  and .headless.mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu"
  and .headless.wgpu_required == false
  and .compatibility.openra_replay_summary_schema_mapped == true
  and .compatibility.openra_runtime_parity_claimed == false
' "$REPLAY_ADAPTER" >/dev/null

test -s "$COMPARISON"
test -s "$MISMATCH_MATRIX"
test -s "$REDUCER"
test -s "$REPLAY_ADAPTER"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_HEADLESS_COMPARISON_HARNESS_GREEN %s %s %s\n' "$SUMMARY" "$COMPARISON" "$MISMATCH_MATRIX"
