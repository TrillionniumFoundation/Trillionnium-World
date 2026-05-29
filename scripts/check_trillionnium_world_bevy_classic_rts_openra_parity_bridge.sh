#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-parity-bridge.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-parity-bridge"
mkdir -p "$PREVIEW_DIR" "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-openra-parity-bridge "$PREVIEW_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_parity_bridge_v1"
  and .green == true
  and .preview_count == 4
  and .comparison_axis_count == 4
  and .source_contracts.organic_terminal_gap == "trillionnium_world_bevy_classic_rts_organic_terminal_gap_v1"
  and .source_contracts.terminal_observation_gap == "trillionnium_world_bevy_classic_rts_terminal_observation_gap_v1"
  and .source_contracts.replay_metrics_gap == "trillionnium_world_bevy_classic_rts_replay_metrics_gap_v1"
  and .source_contracts.endurance_skirmish_gap == "trillionnium_world_bevy_classic_rts_endurance_skirmish_gap_v1"
  and .openra_target_commits.organic_terminal == "5f1bf76"
  and .openra_target_commits.terminal_readiness == "174525a"
  and .openra_target_commits.terminal_probe == "bf42eb1"
  and .openra_target_commits.strategic_terminal == "9e08464"
  and .openra_target_commits.replay_summary == "d5ceade"
  and .openra_target_commits.battle_outcome == "9b2664b"
  and .openra_target_commits.endurance_skirmish == "2cb80a0"
  and .openra_target_commits.longrun_skirmish == "5227d99"
  and .openra_target_commits.multibot_autostart == "4b966c1"
  and .gap_states.organic_terminal == "bevy_deterministic_observation_not_openra_natural_gameover"
  and .gap_states.terminal_observation == "bevy_terminal_observation_vocabulary_not_natural_openra_match"
  and .gap_states.replay_metrics == "bevy_replay_metric_vocabulary_not_openra_replay_file"
  and .gap_states.endurance_skirmish == "bevy_endurance_vocabulary_not_openra_headless_client_match"
  and .bridge_summary.terminal_winner == "Multi2"
  and .bridge_summary.terminal_winner_beacons == 2
  and .bridge_summary.terminal_total_beacons == 4
  and .bridge_summary.terminal_hold_ticks == 3000
  and .bridge_summary.organic_final_match_result_state == "victory:organic_terminal_observed:Multi2"
  and .bridge_summary.terminal_observation_final_match_result_state == "victory:terminal_observation:Multi2"
  and .bridge_summary.replay_elapsed_seconds >= 55
  and .bridge_summary.replay_actor_order_tokens >= 12
  and .bridge_summary.replay_unique_actor_token_count >= 6
  and .bridge_summary.endurance_elapsed_seconds >= 120
  and .bridge_summary.endurance_peak_active_units >= 24
  and .bridge_summary.endurance_combat_event_count >= 20
  and .source_contract_gate == true
  and .source_green_gate == true
  and .terminal_rule_comparison_gate == true
  and .replay_outcome_comparison_gate == true
  and .replay_metrics_comparison_gate == true
  and .headless_endurance_comparison_gate == true
  and .openra_target_commit_gate == true
  and .gap_visibility_gate == true
  and .no_parity_claim_gate == true
  and .boundary_gate == true
  and .preview_gate == true
  and .comparison_matrix_gate == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW_DIR/organic-terminal-gap.ppm"
test -s "$PREVIEW_DIR/terminal-observation-gap.ppm"
test -s "$PREVIEW_DIR/replay-metrics-gap.ppm"
test -s "$PREVIEW_DIR/endurance-skirmish-gap.ppm"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_PARITY_BRIDGE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
