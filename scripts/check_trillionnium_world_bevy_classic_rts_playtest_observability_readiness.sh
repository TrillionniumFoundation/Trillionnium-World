#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-playtest-observability-readiness.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-playtest-observability-readiness"
mkdir -p "$PREVIEW_DIR" "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-playtest-observability-readiness "$PREVIEW_DIR" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_playtest_observability_readiness_v1"
  and .green == true
  and .preview_count == 4
  and .source_contracts.organic_terminal_gap == "trillionnium_world_bevy_classic_rts_organic_terminal_gap_v1"
  and .source_contracts.terminal_observation_gap == "trillionnium_world_bevy_classic_rts_terminal_observation_gap_v1"
  and .source_contracts.replay_metrics_gap == "trillionnium_world_bevy_classic_rts_replay_metrics_gap_v1"
  and .source_contracts.endurance_skirmish_gap == "trillionnium_world_bevy_classic_rts_endurance_skirmish_gap_v1"
  and .organic_terminal_gate == true
  and .terminal_observation_gate == true
  and .replay_metrics_gate == true
  and .endurance_skirmish_gate == true
  and .gap_policy_gate == true
  and .preview_gate == true
  and .organic_terminal_summary.stage_count == 6
  and .organic_terminal_summary.winner == "Multi2"
  and .organic_terminal_summary.winner_count >= 1
  and .organic_terminal_summary.loser_count >= 1
  and .organic_terminal_summary.final_match_result_state == "victory:organic_terminal_observed:Multi2"
  and .terminal_observation_summary.stage_count == 6
  and .terminal_observation_summary.winner == "Multi2"
  and .terminal_observation_summary.terminal_probe_loser_count == 3
  and .terminal_observation_summary.terminal_victory_rules_ready == true
  and .terminal_observation_summary.final_match_result_state == "victory:terminal_observation:Multi2"
  and .replay_metrics_summary.stage_count == 6
  and .replay_metrics_summary.client_slot_count == 4
  and .replay_metrics_summary.actor_order_tokens >= 12
  and .replay_metrics_summary.unique_actor_token_count >= 6
  and .replay_metrics_summary.economy_tokens >= 12
  and .replay_metrics_summary.combat_tokens >= 12
  and .replay_metrics_summary.elapsed_seconds >= 55
  and .replay_metrics_summary.outcome_signal == "sustained_engagement_no_terminal_victory"
  and .endurance_skirmish_summary.stage_count == 6
  and .endurance_skirmish_summary.client_slot_count == 4
  and .endurance_skirmish_summary.elapsed_seconds >= 120
  and .endurance_skirmish_summary.peak_active_units >= 24
  and .endurance_skirmish_summary.contested_beacon_peak >= 2
  and .endurance_skirmish_summary.combat_event_count >= 20
  and .endurance_skirmish_summary.outcome_signal == "sustained_engagement_no_terminal_victory"
' "$SUMMARY" >/dev/null

test -s "$PREVIEW_DIR/organic-terminal-gap.ppm"
test -s "$PREVIEW_DIR/terminal-observation-gap.ppm"
test -s "$PREVIEW_DIR/replay-metrics-gap.ppm"
test -s "$PREVIEW_DIR/endurance-skirmish-gap.ppm"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PLAYTEST_OBSERVABILITY_READINESS_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
