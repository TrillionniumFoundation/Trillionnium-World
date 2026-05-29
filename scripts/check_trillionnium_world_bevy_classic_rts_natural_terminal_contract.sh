#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-natural-terminal-contract.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-natural-terminal-contract"
REPLAY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-owned-replay-file.trnm-replay.json"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

if [[ ! -s "$REPLAY" ]]; then
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_owned_replay_file.sh" >/dev/null
fi

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-natural-terminal-contract "$PREVIEW_DIR" "$REPLAY" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_natural_terminal_contract_v1"
  and .green == true
  and .terminal_contract_state == "bevy_natural_terminal_contract_v1_not_openra_natural_match"
  and .terminal_winner == "Multi2"
  and .terminal_winner_beacons == 2
  and .terminal_total_beacons == 4
  and .terminal_hold_ticks == 3000
  and .terminal_rule == "control_2_of_4_flux_beacons_for_3000_ticks"
  and (.terminal_objective_tile_ids | length) == 4
  and .terminal_outcome.winner == "Multi2"
  and .terminal_outcome.winner_count == 1
  and .terminal_outcome.loser_count == 3
  and .terminal_outcome.organic_final_tick >= 3000
  and .terminal_outcome.terminal_observation_final_tick >= 3000
  and .terminal_outcome.headless_final_tick >= 3000
  and .terminal_outcome.organic_match_result_state == "victory:organic_terminal_observed:Multi2"
  and .terminal_outcome.terminal_observation_match_result_state == "victory:terminal_observation:Multi2"
  and .terminal_outcome.headless_match_result_state == "victory:organic_terminal_observed:Multi2"
  and .terminal_outcome.organic_objective_status == "organic_terminal_observed:Multi2:2_of_4"
  and .terminal_outcome.terminal_observation_objective_status == "terminal_observed:Multi2:2_of_4"
  and (.terminal_outcome.headless_final_checkpoint_sha256 | length) == 64
  and .organic_terminal_gate == true
  and .terminal_observation_gate == true
  and .headless_terminal_gate == true
  and .terminal_rule_contract_gate == true
  and .terminal_timeline_gate == true
  and .terminal_outcome_contract_gate == true
  and .preview_gate == true
  and .gap_visibility_gate == true
  and .no_openra_natural_terminal_claim_gate == true
  and .boundary_gate == true
  and .natural_terminal_contract_gate == true
  and .bevy_natural_terminal_contract_claimed == true
  and .bevy_openra_natural_terminal_match_claimed == false
  and .bevy_openra_headless_client_match_claimed == false
  and .bevy_openra_parity_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW_DIR/organic-terminal.ppm"
test -s "$PREVIEW_DIR/terminal-observation.ppm"
printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_NATURAL_TERMINAL_CONTRACT_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
