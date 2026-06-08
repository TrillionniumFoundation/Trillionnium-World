#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_continuous_player_flow.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-continuous-player-flow'
  'bevy-classic-rts-continuous-player-flow.json'
  'bevy-classic-rts-continuous-player-flow.ppm'
  'trillionnium_world_bevy_classic_rts_continuous_player_flow_v1'
  'trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1'
  'trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1'
  'trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1'
  'trillionnium_world_bevy_classic_rts_production_interaction_polish_v1'
  'trillionnium_world_bevy_classic_rts_session_state_continuity_v1'
  'trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1'
  'trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1'
  'runtime_screen_mode == "player_runtime_continuous_player_flow_screen"'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'continuous_player_flow_step_count == 6'
  'title_account_gate == true'
  'command_feedback_gate == true'
  'save_resume_gate == true'
  'outcome_open_world_gate == true'
  'continuous_player_flow_gate == true'
  'production_ready_ui_claimed == false'
  'screen_for_screen_openra_ui_claimed == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTINUOUS_PLAYER_FLOW_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS continuous player flow script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTINUOUS_PLAYER_FLOW_CONTRACT'
  'native_classic_rts_continuous_player_flow_evidence_json'
  'TRNM RUST/BEVY CONTINUOUS PLAYER FLOW'
  'TITLE / ACCOUNT'
  'MATCH SETUP'
  'IN-MATCH HUD'
  'COMMAND FEEDBACK'
  'SAVE / RESUME'
  'OUTCOME / WORLD'
  'player_runtime_continuous_player_flow_screen'
  'player_runtime_shell_meta_screen'
  'player_runtime_match_setup_screen'
  'player_runtime_in_match_hud_screen'
  'player_runtime_command_interaction_screen'
  'player_runtime_session_resume_screen'
  'player_runtime_campaign_outcome_screen'
  'continuous_player_flow_steps'
  'transition_sequence'
  'continuous_player_flow_gate'
  'internal_continuous_player_flow_claimed'
  'production_ready_ui_claimed'
  'screen_for_screen_openra_ui_claimed'
  'third_party_asset_copied'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS continuous player flow source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_continuous_player_flow.sh'
  'rts_continuous_player_flow'
  'classic_rts_continuous_player_flow_green'
  'bevy-classic-rts-continuous-player-flow.json'
  'bevy-classic-rts-continuous-player-flow.ppm'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS continuous player flow readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_continuous_player_flow.sh'
  'bevy_classic_rts_continuous_player_flow_script_contract_guard_test.sh'
  'bevy_classic_rts_continuous_player_flow_gate'
  'trillionnium_world_bevy_classic_rts_continuous_player_flow_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing classic RTS continuous player flow release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS continuous player flow gate remains connected to Rust CLI, runtime player-flow sources, playtest readiness, release-review CI, and no-external-evidence boundaries"
