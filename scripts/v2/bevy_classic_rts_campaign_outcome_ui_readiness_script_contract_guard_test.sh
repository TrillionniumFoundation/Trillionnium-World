#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
PACKET="$ROOT/scripts/check_trillionnium_world_release_review_packet.sh"
INTEGRITY="$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1'
  'classic_rts_campaign_outcome_ui_readiness_green'
  'bevy-classic-rts-campaign-outcome-ui-readiness.json'
  'bevy-classic-rts-campaign-outcome-ui-readiness'
  'classic-rts-campaign-outcome-ui-readiness'
  'first-minute-readiness.ppm'
  'objective-victory-loop.ppm'
  'base-assault-resolution.ppm'
  'battle-aftermath.ppm'
  'open-world-after-action.ppm'
  'preview_count == 5'
  'runtime_screen_mode == "player_runtime_campaign_outcome_screen"'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'title to victory to aftermath to open-world resume'
  'relay beacon extracted victory and defeat-risk summary'
  'league-coliseum arena_outdoor resume state'
  'first_minute_gate == true'
  'objective_victory_gate == true'
  'base_assault_gate == true'
  'battle_aftermath_gate == true'
  'open_world_return_gate == true'
  'player_first_campaign_outcome_screen_gate == true'
  'player_runtime_first_minute_readiness_screen'
  'player_first_first_minute_screen_gate == true'
  'first_minute_pixel_counts.player_first_campaign_view_non_background > 600000'
  'victory_summary.non_background_pixels > 250000'
  'base_assault_summary.non_background_pixels > 350000'
  'player_runtime_battle_aftermath_screen'
  'player_first_battle_aftermath_screen_gate == true'
  'battle_aftermath_pixel_counts.player_first_battle_view_non_background > 250000'
  'player_runtime_open_world_after_action_screen'
  'player_first_open_world_after_action_screen_gate == true'
  'native_boundary_gate == true'
  'preview_gate == true'
  'campaign_outcome_ui_readiness_gate == true'
  'internal_campaign_outcome_ui_readiness_claimed == true'
  'external_evidence_ignored_for_current_outcome_pass == true'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS campaign outcome UI readiness script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_OUTCOME_UI_READINESS_CONTRACT'
  'native_classic_rts_campaign_outcome_ui_readiness_evidence_json'
  'classic-rts-campaign-outcome-ui-readiness'
  'classic_rts_campaign_outcome_ui_readiness_green'
  'player_runtime_campaign_outcome_screen'
  'runtime_screen_layout'
  'campaign_outcome_ui_readiness_gate'
  'player_first_campaign_outcome_screen_gate'
  'player_runtime_first_minute_readiness_screen'
  'player_first_first_minute_screen_gate'
  'first_minute_pixel_counts'
  'victory_pixel_count'
  'breach_pixel_count'
  'player_runtime_battle_aftermath_screen'
  'player_first_battle_aftermath_screen_gate'
  'battle_aftermath_pixel_counts'
  'first-minute-readiness.ppm'
  'objective-victory-loop.ppm'
  'base-assault-resolution.ppm'
  'battle-aftermath.ppm'
  'open-world-after-action.ppm'
  'player_runtime_open_world_after_action_screen'
  'player_first_open_world_after_action_screen_gate'
  'TITLE campaign entry'
  'objective claim/extract victory'
  'battle aftermath rewards'
  'open-world route resume'
  'internal_campaign_outcome_ui_readiness_claimed'
  'external_evidence_ignored_for_current_outcome_pass'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS campaign outcome UI readiness source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness.sh'
  'bevy-classic-rts-campaign-outcome-ui-readiness.json'
  'classic_rts_campaign_outcome_ui_readiness_green'
  'rts_campaign_outcome_ui_readiness'
  'rts_campaign_outcome_ui_readiness_player_first_screen_gate'
  'rts_campaign_outcome_ui_readiness_first_minute_player_first_non_background'
  'rts_campaign_outcome_ui_readiness_aftermath_player_first_view_non_background'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS campaign outcome UI readiness readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'bevy_classic_rts_campaign_outcome_ui_readiness_contract_guard'
  'bevy_classic_rts_campaign_outcome_ui_readiness_gate'
  'native_bevy_classic_rts_campaign_outcome_ui_readiness'
  'campaign_outcome_ui_readiness_semantics'
  'rts_campaign_outcome_ui_readiness_player_first_screen_gate'
  'player_first_campaign_outcome_screen_gate == true'
  'first_minute_summary.runtime_screen_mode == "player_runtime_first_minute_readiness_screen"'
  'aftermath_summary.runtime_screen_mode == "player_runtime_battle_aftermath_screen"'
  'fake_packet_artifact_count == 120'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$CI" "$PACKET" "$INTEGRITY"; then
    echo "[FAIL] missing classic RTS campaign outcome UI readiness release line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS campaign outcome UI readiness remains connected to Bevy-native first-minute, victory, base-assault, aftermath, open-world resume, CI, packet, and integrity semantics"
