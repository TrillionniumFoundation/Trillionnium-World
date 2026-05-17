#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-player-ui-rescue.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- player-ui-rescue >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_player_ui_rescue_v1"
  and .green == true
  and .player_status_gate == true
  and .route_panel_gate == true
  and .quest_panel_gate == true
  and .action_layer_gate == true
  and .debug_deprioritized_gate == true
  and .event_log_separation_gate == true
  and .button_wall_deprioritized_gate == true
  and .contextual_deck_layout_gate == true
  and .right_rail_summary_gate == true
  and .top_hud_density_gate == true
  and .toast_lane_gate == true
  and .visual_hierarchy_gate == true
  and .art_direction_gate == true
  and .scene_readability_gate == true
  and .sprite_asset_quality_gate == true
  and .map_model_visual_gate == true
  and .map_occlusion_gate == true
  and .ui_polish_gate == true
  and .tileset_polish_gate == true
  and .authored_art_pack_gate == true
  and .runtime_gate == true
  and (.player_layer.character_status_text | contains("PLAYER HUD"))
  and (.player_layer.character_status_text | contains("Goal"))
  and (.player_layer.room_panel_text | contains("PLAYER ROUTE |"))
  and (.player_layer.room_panel_text | contains("NEXT STEP:"))
  and (.player_layer.room_panel_text | contains("PROGRESS:"))
  and (.player_layer.room_panel_text | contains("CHECKLIST") | not)
  and (.player_layer.input_hint_text | contains("PLAYER ACTIONS | READY:"))
  and (.player_layer.input_hint_text | contains("DEV INPUT |"))
  and (.button_wall_policy.hidden_button_count >= 24)
  and (.button_wall_policy.deprioritized_button_count > .button_wall_policy.foreground_button_count)
  and (.button_wall_policy.player_deck_visible_count <= 32)
  and (.button_wall_policy.player_deck_hidden_count > .button_wall_policy.player_deck_visible_count)
  and (.player_layer.visible_quest_summary_text | contains("CURRENT OBJECTIVE |"))
  and (.player_layer.visible_stats_summary_text | contains("STATS |"))
  and (.player_layer.visible_bag_summary_text | contains("AFFIX |"))
  and (.player_layer.visible_event_summary_text | contains("LAST |"))
  and (.player_layer.visible_event_summary_text | contains("INPUT |"))
  and (.player_layer.visible_event_summary_text | contains("DEBUG LAYER") | not)
  and (.player_layer.primary_cta_text | contains("PRIMARY | Enter ->"))
  and (.player_layer.primary_cta_text | contains("SHORTCUT |"))
  and (.player_layer.movement_hint_text | contains("Numpad"))
  and (.player_layer.movement_hint_text | contains("WASD"))
  and (.player_layer.feedback_banner_text | contains("TOAST"))
  and (.player_layer.feedback_banner_font_size <= 9)
  and (.player_layer.feedback_banner_y > -100)
  and (.player_layer.feedback_banner_y < -60)
  and (.action_row_policy.active_action_row_count >= 1)
  and (.action_row_policy.active_action_row_count <= 4)
  and (.art_direction_policy.surface_count >= 6)
  and (.art_direction_policy.surface_ids | index("map_focus_glow") != null)
  and (.art_direction_policy.surface_ids | index("primary_cta_gold_glow") != null)
  and (.art_direction_policy.surface_ids | index("action_deck_depth_shadow") != null)
  and (.art_direction_policy.palette_roles | index("warm_gold") != null)
  and (.art_direction_policy.palette_roles | index("cyan_focus") != null)
  and (.art_direction_policy.palette_roles | index("neutral_shadow") != null)
  and (.scene_readability_policy.surface_count >= 6)
  and (.scene_readability_policy.surface_ids | index("player_selection_ring") != null)
  and (.scene_readability_policy.surface_ids | index("npc_interaction_ring") != null)
  and (.scene_readability_policy.surface_ids | index("enemy_threat_ring") != null)
  and (.scene_readability_policy.surface_ids | index("objective_route_arrow") != null)
  and (.scene_readability_policy.surface_ids | index("loot_pickup_sparkle") != null)
  and (.scene_readability_policy.surface_ids | index("combat_hit_splash") != null)
  and (.scene_readability_policy.focus_roles | index("player_identity") != null)
  and (.scene_readability_policy.focus_roles | index("combat_feedback") != null)
  and (.scene_readability_policy.visible_actor_kinds | index("npc") != null)
  and (.scene_readability_policy.visible_actor_kinds | index("enemy") != null)
  and (.scene_readability_policy.visible_actor_kinds | index("drop") != null)
  and (.scene_readability_policy.map_quality_surface_gate == true)
  and (.sprite_asset_policy.surface_count >= 18)
  and (.sprite_asset_policy.actor_kinds | index("player") != null)
  and (.sprite_asset_policy.actor_kinds | index("npc") != null)
  and (.sprite_asset_policy.actor_kinds | index("enemy") != null)
  and (.sprite_asset_policy.actor_kinds | index("drop") != null)
  and (.sprite_asset_policy.actor_kinds | index("feedback") != null)
  and (.sprite_asset_policy.asset_roles | index("player_body_layer") != null)
  and (.sprite_asset_policy.asset_roles | index("player_head") != null)
  and (.sprite_asset_policy.asset_roles | index("actor_shadow") != null)
  and (.sprite_asset_policy.asset_roles | index("actor_body") != null)
  and (.sprite_asset_policy.asset_roles | index("npc_dialogue_badge") != null)
  and (.sprite_asset_policy.asset_roles | index("enemy_hp_badge") != null)
  and (.sprite_asset_policy.asset_roles | index("loot_pickup_badge") != null)
  and (.sprite_asset_policy.asset_roles | index("combat_hit_feedback_marker") != null)
  and (.map_model_visual_policy.surface_count >= 80)
  and (.map_model_visual_policy.building_count >= 20)
  and (.map_model_visual_policy.road_count >= 20)
  and (.map_model_visual_policy.greenery_count >= 5)
  and (.map_model_visual_policy.terrain_count >= 4)
  and (.map_model_visual_policy.layers | index("building") != null)
  and (.map_model_visual_policy.layers | index("road") != null)
  and (.map_model_visual_policy.layers | index("greenery") != null)
  and (.map_model_visual_policy.layers | index("terrain") != null)
  and (.map_model_visual_policy.visual_roles | index("building_mass") != null)
  and (.map_model_visual_policy.visual_roles | index("walkable_road_path") != null)
  and (.map_model_visual_policy.visual_roles | index("greenery_cluster") != null)
  and (.map_model_visual_policy.visual_roles | index("terrain_zone_surface") != null)
  and (.map_occlusion_policy.surface_count >= 5)
  and (.map_occlusion_policy.weighted_ratio <= 0.10)
  and (.map_occlusion_policy.max_panel_area <= 16000)
  and (.map_occlusion_policy.max_panel_alpha <= 0.38)
  and (.map_occlusion_policy.map_roles | index("edge_dialogue_hint") != null)
  and (.map_occlusion_policy.map_roles | index("edge_scene_hint") != null)
  and (.map_occlusion_policy.map_roles | index("edge_step_hint") != null)
  and (.map_occlusion_policy.map_roles | index("edge_combat_hint") != null)
  and (.map_occlusion_policy.map_roles | index("bottom_story_summary") != null)
  and (.ui_polish_policy.surface_count >= 10)
  and (.ui_polish_policy.max_font_size <= 18)
  and (.ui_polish_policy.regions | index("top_hud") != null)
  and (.ui_polish_policy.regions | index("right_rail") != null)
  and (.ui_polish_policy.regions | index("action_deck") != null)
  and (.ui_polish_policy.regions | index("movement_cluster") != null)
  and (.ui_polish_policy.regions | index("primary_cta") != null)
  and (.ui_polish_policy.regions | index("map_edge_hints") != null)
  and (.ui_polish_policy.typography_roles | index("hud_compact") != null)
  and (.ui_polish_policy.typography_roles | index("summary_card") != null)
  and (.ui_polish_policy.typography_roles | index("action_deck_container") != null)
  and (.ui_polish_policy.typography_roles | index("primary_cta") != null)
  and (.ui_polish_policy.visual_priorities | index("primary") != null)
  and (.ui_polish_policy.visual_priorities | index("secondary") != null)
  and (.ui_polish_policy.visual_priorities | index("tertiary") != null)
  and (.tileset_polish_policy.surface_count >= 90)
  and (.tileset_polish_policy.atlas_families | index("city_ground_tileset_v1") != null)
  and (.tileset_polish_policy.atlas_families | index("road_tileset_v1") != null)
  and (.tileset_polish_policy.atlas_families | index("building_tileset_v1") != null)
  and (.tileset_polish_policy.atlas_families | index("greenery_tileset_v1") != null)
  and (.tileset_polish_policy.atlas_families | index("water_tileset_v1") != null)
  and (.tileset_polish_policy.atlas_families | index("hud_icon_tileset_v1") != null)
  and (.tileset_polish_policy.layers | index("terrain") != null)
  and (.tileset_polish_policy.layers | index("road") != null)
  and (.tileset_polish_policy.layers | index("building") != null)
  and (.tileset_polish_policy.layers | index("greenery") != null)
  and (.tileset_polish_policy.layers | index("water") != null)
  and (.tileset_polish_policy.layers | index("hud") != null)
  and (.tileset_polish_policy.asset_roles | index("road_edge_highlight") != null)
  and (.tileset_polish_policy.asset_roles | index("building_roof_cap") != null)
  and (.tileset_polish_policy.asset_roles | index("greenery_canopy_cluster") != null)
  and (.tileset_polish_policy.asset_roles | index("water_edge_shimmer") != null)
  and (.tileset_polish_policy.asset_roles | index("hud_status_icon") != null)
  and (.tileset_polish_policy.asset_roles | index("primary_cta_glyph") != null)
  and (.tileset_polish_policy.palette_roles | index("gold_accent") != null)
  and (.tileset_polish_policy.palette_roles | index("foliage") != null)
  and (.tileset_polish_policy.detail_roles | index("cta_arrow_marker") != null)
  and (.authored_art_pack_policy.surface_count >= 120)
  and (.authored_art_pack_policy.asset_pack_ids | index("trnm_world_authored_art_pack_v1") != null)
  and (.authored_art_pack_policy.asset_kinds | index("terrain_tile") != null)
  and (.authored_art_pack_policy.asset_kinds | index("road_tile") != null)
  and (.authored_art_pack_policy.asset_kinds | index("building_tile") != null)
  and (.authored_art_pack_policy.asset_kinds | index("foliage_sprite") != null)
  and (.authored_art_pack_policy.asset_kinds | index("water_tile") != null)
  and (.authored_art_pack_policy.asset_kinds | index("hud_icon") != null)
  and (.authored_art_pack_policy.asset_kinds | index("hud_glyph") != null)
  and (.authored_art_pack_policy.asset_kinds | index("actor_sprite") != null)
  and (.authored_art_pack_policy.asset_kinds | index("feedback_glyph") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("terrain") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("road") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("building") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("greenery") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("water") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("hud") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("actor") != null)
  and (.authored_art_pack_policy.gameplay_layers | index("feedback") != null)
  and (.authored_art_pack_policy.replacement_slots | index("tile_sprite_slot") != null)
  and (.authored_art_pack_policy.replacement_slots | index("hud_icon_slot") != null)
  and (.authored_art_pack_policy.replacement_slots | index("hud_glyph_slot") != null)
  and (.authored_art_pack_policy.replacement_slots | index("actor_sprite_slot") != null)
  and (.authored_art_pack_policy.replacement_slots | index("feedback_glyph_slot") != null)
  and (.authored_art_pack_policy.source_origins | index("local_authored_primitive_manifest_v1") != null)
  and (.authored_art_pack_policy.license_scopes | index("project_owned_internal_placeholder") != null)
  and (.authored_art_pack_policy.min_target_resolution_px >= 32)
  and (.authored_art_pack_policy.export_ready_count == .authored_art_pack_policy.surface_count)
  and (.android_s5_real_device_claimed == false)
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_PLAYER_UI_RESCUE_GREEN $SUMMARY"
