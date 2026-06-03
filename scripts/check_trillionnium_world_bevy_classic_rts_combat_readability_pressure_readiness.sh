#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-readability-pressure-readiness.json"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-readability-pressure-readiness"
mkdir -p "$PREVIEW_DIR" "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-combat-readability-pressure-readiness "$PREVIEW_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness_v1"
  and .status == "classic_rts_combat_readability_pressure_readiness_green"
  and .green == true
  and .preview_count == 5
  and .source_contracts.unit_status_portrait == "trillionnium_world_bevy_classic_rts_unit_status_portrait_v1"
  and .source_contracts.selection_command_feedback == "trillionnium_world_bevy_classic_rts_selection_command_feedback_v1"
  and .source_contracts.ability_tooltip_telegraph == "trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1"
  and .source_contracts.depth_readability == "trillionnium_world_bevy_classic_rts_depth_readability_v1"
  and .source_contracts.central_keep_pressure == "trillionnium_world_bevy_classic_rts_central_keep_pressure_v1"
  and .unit_status_gate == true
  and .command_feedback_gate == true
  and .ability_telegraph_gate == true
  and .depth_readability_gate == true
  and .pressure_feedback_gate == true
  and .source_policy_gate == true
  and .preview_gate == true
  and .unit_status_summary.portrait_frame_pixel_count > 1200
  and .unit_status_summary.health_bar_pixel_count > 300
  and .unit_status_summary.mana_bar_pixel_count > 240
  and .unit_status_summary.role_badge_pixel_count > 600
  and .command_feedback_summary.marquee_pixel_count > 350
  and .command_feedback_summary.attack_pixel_count > 320
  and .command_feedback_summary.error_pixel_count > 420
  and .command_feedback_summary.ack_pixel_count > 240
  and .ability_telegraph_summary.accepted_input_count == 6
  and .ability_telegraph_summary.tooltip_pixel_count > 900
  and .ability_telegraph_summary.range_pixel_count > 500
  and .ability_telegraph_summary.warning_pixel_count > 900
  and .depth_summary.foreground_pixel_count > 120
  and .depth_summary.behind_pixel_count > 120
  and .depth_summary.building_mask_pixel_count > 140
  and .depth_summary.target_priority_pixel_count > 130
  and .pressure_summary.accepted_input_count == 40
  and .pressure_summary.final_defeat_risk_percent >= 42
  and .pressure_summary.final_target_health_percent == 58
  and .pressure_summary.final_target_shield_percent == 24
  and .pressure_summary.final_central_keep_state == "pressure_locked:central_keep"
  and (.pressure_summary.final_next_action_ids | index("press_central_keep") != null)
  and (.pressure_summary.final_next_action_ids | index("break_central_keep") != null)
  and .internal_combat_readability_pressure_readiness_claimed == true
  and .external_evidence_ignored_for_current_combat_readability_pass == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW_DIR/unit-status-portrait.ppm"
test -s "$PREVIEW_DIR/selection-command-feedback.ppm"
test -s "$PREVIEW_DIR/ability-tooltip-telegraph.ppm"
test -s "$PREVIEW_DIR/depth-readability.ppm"
test -s "$PREVIEW_DIR/central-keep-pressure.ppm"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMBAT_READABILITY_PRESSURE_READINESS_GREEN %s %s\n' "$SUMMARY" "$PREVIEW_DIR"
