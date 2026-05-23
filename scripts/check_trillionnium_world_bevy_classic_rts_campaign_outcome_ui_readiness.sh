#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-outcome-ui-readiness.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-outcome-ui-readiness"
mkdir -p "$PREVIEW_DIR" "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-campaign-outcome-ui-readiness "$PREVIEW_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1"
  and .green == true
  and .preview_count == 5
  and .source_contracts.first_minute_readiness == "trillionnium_world_bevy_classic_rts_first_minute_readiness_v1"
  and .source_contracts.objective_victory_loop == "trillionnium_world_bevy_classic_rts_objective_victory_loop_v1"
  and .source_contracts.base_assault_resolution == "trillionnium_world_bevy_classic_rts_base_assault_resolution_v1"
  and .source_contracts.battle_aftermath == "trillionnium_world_bevy_classic_rts_battle_aftermath_v1"
  and .source_contracts.open_world_after_action == "trillionnium_world_bevy_classic_rts_open_world_after_action_v1"
  and .first_minute_gate == true
  and .objective_victory_gate == true
  and .base_assault_gate == true
  and .battle_aftermath_gate == true
  and .open_world_return_gate == true
  and .native_boundary_gate == true
  and .preview_gate == true
  and (.campaign_flow | index("TITLE campaign entry") != null)
  and (.campaign_flow | index("objective claim/extract victory") != null)
  and (.campaign_flow | index("battle aftermath rewards") != null)
  and (.campaign_flow | index("open-world route resume") != null)
  and .first_minute_summary.input_action_count == 73
  and .first_minute_summary.final_room == "league-coliseum"
  and .first_minute_summary.final_objective_status == "open_world_after_action_ready"
  and .victory_summary.accepted_input_count == 6
  and .victory_summary.final_objective_capture_percent == 100
  and .victory_summary.final_objective_result_state == "victory:relay_beacon_extracted"
  and .victory_summary.final_defeat_risk_percent <= 8
  and .base_assault_summary.accepted_input_count == 9
  and .base_assault_summary.final_base_breach_percent == 100
  and .base_assault_summary.final_base_assault_result_state == "breached:enemy_barracks"
  and .aftermath_summary.accepted_input_count == 12
  and .aftermath_summary.final_match_result_state == "victory_ready:secure_expansion"
  and .aftermath_summary.final_growth_level >= 2
  and (.aftermath_summary.final_next_action_ids | index("secure_expansion") != null)
  and .open_world_summary.accepted_input_count == 3
  and .open_world_summary.final_current_room_id == "league-coliseum"
  and .open_world_summary.final_map_scene == "arena_outdoor"
  and .open_world_summary.final_open_world_handoff_state == "resumed:league-coliseum"
' "$SUMMARY" >/dev/null

test -s "$PREVIEW_DIR/first-minute-readiness.ppm"
test -s "$PREVIEW_DIR/objective-victory-loop.ppm"
test -s "$PREVIEW_DIR/base-assault-resolution.ppm"
test -s "$PREVIEW_DIR/battle-aftermath.ppm"
test -s "$PREVIEW_DIR/open-world-after-action.ppm"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_OUTCOME_UI_READINESS_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
