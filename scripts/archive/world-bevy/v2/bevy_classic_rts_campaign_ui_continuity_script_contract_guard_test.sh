#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_ui_continuity.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-campaign-ui-continuity'
  'bevy-classic-rts-campaign-ui-continuity.json'
  'bevy-classic-rts-campaign-ui-continuity.ppm'
  'trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1'
  'trillionnium_world_bevy_classic_rts_campaign_handoff_v1'
  'runtime_screen_mode == "player_runtime_campaign_ui_continuity_screen"'
  'evidence_board_only == false'
  'runtime_screen_layout_count == (.runtime_screen_layout | keys | length)'
  'rts_evidence_campaign_ui_continuity_review_field_count == (.rts_evidence_campaign_ui_continuity_review | keys | length)'
  'final_contextual_action_label_count == (.final_contextual_action_labels | length)'
  'restored_contextual_action_label_count == (.restored_contextual_action_labels | length)'
  'campaign_continuity_pixel_count_field_count == (.campaign_continuity_pixel_counts | keys | length)'
  'gate_count == ([.handoff_green_gate, .preview_resolution_gate, .live_input_gate, .milestone_gate, .map_ui_state_gate, .restored_ui_state_gate, .persistence_gate, .render_readability_gate, .native_client_boundary_gate, .rts_evidence_campaign_ui_continuity_review_gate, .runtime_screen_gate, .player_first_campaign_continuity_screen_gate] | length)'
  'failed_gate_count == 0'
  'player_first_campaign_continuity_screen_gate == true'
  'campaign_continuity_pixel_counts.player_first_campaign_view_non_background > 600000'
  'capture_frame_count == 16'
  'final_current_room_id == "league-coliseum"'
  'restored_current_room_id == "league-coliseum"'
  'final_contextual_primary_action_label == "COMBAT:attack"'
  'handoff_green_gate == true'
  'persistence_gate == true'
  'render_readability_gate == true'
  'native_client_boundary_gate == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_UI_CONTINUITY_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing campaign UI continuity script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_UI_CONTINUITY_CONTRACT'
  'native_classic_rts_campaign_ui_continuity_evidence_json'
  'classic-rts-campaign-ui-continuity'
  'player_runtime_campaign_ui_continuity_screen'
  'player_first_campaign_continuity_screen_gate'
  'campaign_handoff_pixel_counts'
  'classic_render_player_first_campaign_continuity_screen'
  'capture_frame_count'
  'final_contextual_primary_action_label'
  'final_open_world_handoff_state'
  'restored_open_world_handoff_state'
  'final_active_task_ids'
  'restored_active_task_ids'
  'league-coliseum'
  'COMBAT:attack'
  'handoff_green_gate'
  'persistence_gate'
  'render_readability_gate'
  'native_client_boundary_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing campaign UI continuity source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_campaign_ui_continuity.sh'
  'rts_campaign_ui_continuity'
  'classic_rts_campaign_ui_continuity_green'
  'bevy-classic-rts-campaign-ui-continuity.json'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing campaign UI continuity readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_campaign_ui_continuity.sh'
  'bevy_classic_rts_campaign_ui_continuity_script_contract_guard_test.sh'
  'bevy_classic_rts_campaign_ui_continuity_gate'
  'trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing campaign UI continuity release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS campaign UI continuity gate remains connected to the Rust CLI, restored native/open-world UI state, playtest readiness, and release-review CI"
