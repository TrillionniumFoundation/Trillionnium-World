#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_in_match_hud_state_replication.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-in-match-hud-state-replication'
  'bevy-classic-rts-in-match-hud-state-replication.json'
  'bevy-classic-rts-in-match-hud-state-replication.ppm'
  'trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1'
  'trillionnium_world_bevy_classic_rts_production_ui_skin_v1'
  'trillionnium_world_bevy_classic_rts_production_interaction_polish_v1'
  'trillionnium_world_bevy_classic_rts_selection_minimap_v1'
  'trillionnium_world_bevy_classic_rts_unit_status_portrait_v1'
  'trillionnium_world_bevy_classic_rts_selection_command_feedback_v1'
  'trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1'
  'trillionnium_world_bevy_classic_rts_camera_minimap_sync_v1'
  'trillionnium_world_bevy_classic_rts_command_queue_path_preview_v1'
  'trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1'
  'trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1'
  'trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1'
  'runtime_screen_mode == "player_runtime_in_match_hud_screen"'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'runtime_screen_layout.tactical_viewport'
  'runtime_screen_layout.bottom_command_grid'
  'hud_surface_count == 8'
  'in_match_hud_state_replication_gate == true'
  'external_evidence_ignored_for_current_replication_pass == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_IN_MATCH_HUD_STATE_REPLICATION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing in-match HUD/state replication script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_IN_MATCH_HUD_STATE_REPLICATION_CONTRACT'
  'native_classic_rts_in_match_hud_state_replication_evidence_json'
  'TRNM RUST/BEVY IN-MATCH HUD STATE REPLICATION'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_UI_SKIN_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_INTERACTION_POLISH_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SELECTION_MINIMAP_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_UNIT_STATUS_PORTRAIT_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SELECTION_COMMAND_FEEDBACK_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ABILITY_TOOLTIP_TELEGRAPH_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMERA_MINIMAP_SYNC_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMAND_QUEUE_PATH_PREVIEW_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FULL_SCREEN_UI_REPLICATION_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MATCH_SETUP_UI_REPLICATION_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_OUTCOME_UI_READINESS_CONTRACT'
  'RESOURCES'
  'SELECTION'
  'COMMAND GRID'
  'MINIMAP'
  'PRODUCTION'
  'ABILITIES'
  'COMBAT ALERTS'
  'OBJECTIVE'
  'player_runtime_in_match_hud_screen'
  'runtime_screen_layout'
  'evidence_board_only'
  'tactical_viewport'
  'bottom_command_grid'
  'runtime_screen_gate'
  'in_match_hud_state_replication_gate'
  'external_evidence_ignored_for_current_replication_pass'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing in-match HUD/state replication source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_in_match_hud_state_replication.sh'
  'rts_in_match_hud_state_replication'
  'classic_rts_in_match_hud_state_replication_green'
  'bevy-classic-rts-in-match-hud-state-replication.json'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing in-match HUD/state replication readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_in_match_hud_state_replication.sh'
  'bevy_classic_rts_in_match_hud_state_replication_script_contract_guard_test.sh'
  'bevy_classic_rts_in_match_hud_state_replication_gate'
  'trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing in-match HUD/state replication release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS in-match HUD/state replication gate remains connected to Rust CLI, dynamic HUD state sources, playtest readiness, release-review CI, and no-external-evidence boundaries"
