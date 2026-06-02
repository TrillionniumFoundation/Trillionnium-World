#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-screen-ui-replication.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-screen-ui-replication.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-full-screen-ui-replication "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 768
  and .source_contracts.campaign_entry == "trillionnium_world_bevy_classic_rts_campaign_entry_v1"
  and .source_contracts.visual_fidelity == "trillionnium_world_bevy_classic_rts_visual_fidelity_v1"
  and .source_contracts.map_ui_modeling_readiness == "trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness_v1"
  and .source_contracts.production_ui_skin == "trillionnium_world_bevy_classic_rts_production_ui_skin_v1"
  and .source_contracts.production_interaction_polish == "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1"
  and .source_contracts.build_lifecycle == "trillionnium_world_bevy_classic_rts_build_lifecycle_v1"
  and .source_contracts.tech_tree == "trillionnium_world_bevy_classic_rts_tech_tree_v1"
  and .source_contracts.campaign_outcome_ui_readiness == "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1"
  and .source_contracts.combat_readability_pressure_readiness == "trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness_v1"
  and .replication_surface_count == 10
  and (.replication_surface_names | index("TITLE/CAMPAIGN ENTRY") != null)
  and (.replication_surface_names | index("TACTICAL VIEWPORT") != null)
  and (.replication_surface_names | index("MAP/MINIMAP CAMERA") != null)
  and (.replication_surface_names | index("PRODUCTION HUD SKIN") != null)
  and (.replication_surface_names | index("COMMAND INTERACTIONS") != null)
  and (.replication_surface_names | index("BUILD + TECH TREE") != null)
  and (.replication_surface_names | index("UNIT STATUS CARD") != null)
  and (.replication_surface_names | index("ABILITY/COMBAT UI") != null)
  and (.replication_surface_names | index("CAMPAIGN OUTCOME") != null)
  and (.replication_surface_names | index("OPEN-WORLD HANDOFF") != null)
  and (.replication_slot_ids | index("title_campaign_shell") != null)
  and (.replication_slot_ids | index("match_viewport_hud") != null)
  and (.replication_slot_ids | index("map_minimap_camera") != null)
  and (.replication_slot_ids | index("production_hud_surfaces") != null)
  and (.replication_slot_ids | index("interaction_feedback") != null)
  and (.replication_slot_ids | index("build_tech_overlay") != null)
  and (.replication_slot_ids | index("unit_status_card") != null)
  and (.replication_slot_ids | index("ability_combat_overlay") != null)
  and (.replication_slot_ids | index("outcome_reward_panel") != null)
  and (.replication_slot_ids | index("handoff_replay_resume") != null)
  and .screen_matrix_pixel_counts.board > 80000
  and .screen_matrix_pixel_counts.title_campaign > 2000
  and .screen_matrix_pixel_counts.tactical_viewport > 2000
  and .screen_matrix_pixel_counts.map_minimap > 2000
  and .screen_matrix_pixel_counts.production_hud_skin > 2000
  and .screen_matrix_pixel_counts.command_interaction > 2000
  and .screen_matrix_pixel_counts.build_tech > 2000
  and .screen_matrix_pixel_counts.unit_status > 2000
  and .screen_matrix_pixel_counts.combat_overlay > 2000
  and .screen_matrix_pixel_counts.campaign_outcome > 2000
  and .screen_matrix_pixel_counts.open_world_handoff > 2000
  and .screen_matrix_pixel_counts.highlight > 3000
  and (.source_headline.title_actions | index("CAMPAIGN:START") != null)
  and (.source_headline.title_actions | index("CAMPAIGN:CONTINUE") != null)
  and (.source_headline.title_actions | index("CAMPAIGN:REPLAY") != null)
  and .source_headline.campaign_input_action_count == 73
  and .source_headline.visual_selected_unit_count >= 4
  and .source_headline.map_ui_preview_count == 6
  and .source_headline.production_ui_skin_surface_count == 8
  and .source_headline.interaction_surface_count == 6
  and .source_headline.tech_state == "unlocked:relay_guard"
  and .source_headline.campaign_outcome_preview_count == 5
  and .source_headline.combat_readability_preview_count == 5
  and .title_campaign_gate == true
  and .tactical_viewport_gate == true
  and .map_minimap_gate == true
  and .production_skin_gate == true
  and .interaction_polish_gate == true
  and .build_tech_gate == true
  and .combat_ui_gate == true
  and .campaign_outcome_gate == true
  and .source_policy_gate == true
  and .replication_preview_gate == true
  and .source_preview_gate == true
  and .full_screen_ui_replication_gate == true
  and .internal_full_screen_ui_replication_claimed == true
  and .external_evidence_ignored_for_current_replication_pass == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FULL_SCREEN_UI_REPLICATION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
