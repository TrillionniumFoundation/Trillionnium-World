#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
mkdir -p "$(dirname "$OUT")"

if grep -Fq 'CLASSIC_FIRST_CONTACT_BASIN_ACTORS' "$SOURCE"; then
  echo "[FAIL] First Contact actors must be derived from trnm-rts-data, not a Bevy-local actor table" >&2
  exit 1
fi

required_source_lines=(
  'fn classic_first_contact_map_actors_from_rts_data() -> Vec<ClassicFirstContactActor>'
  'first_contact_basin_map()'
  '.actors'
  'classic_first_contact_actor_from_rts_data_actor(actor)'
  'let map_actors = classic_first_contact_map_actors_from_rts_data();'
  'let actor_template_count = classic_first_contact_map_actors_from_rts_data().len();'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE"; then
    echo "[FAIL] missing First Contact RTS data actor derivation source line: $line" >&2
    exit 1
  fi
done

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-first-contact-basin-spec >"$OUT"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1"
  and .green == true
  and .map_id == "first_contact_basin"
  and .map_size.width == 34
  and .map_size.height == 34
  and .actor_count == 39
  and .spawn_count == 4
  and .flux_bloom_count == 11
  and .beacon_count == 4
  and .expansion_count == 4
  and .unit_rule_count >= 4
  and .building_rule_count >= 2
  and .map_actor_gate == true
  and .map_topology_gate == true
  and .rules_gate == true
  and .rts_data_contract == "trnm_rts_data_map_model_v1"
  and .rts_data_map_model.contract_version == "trnm_rts_data_map_model_v1"
  and .rts_data_map_model.map_id == "first_contact_basin"
  and (.rts_data_map_model.actors | length) == 39
  and .rts_data_map_summary.actor_count == 39
  and .rts_data_map_summary.source_integration_mode == "gpl_internal_component"
  and .rts_data_source_manifest.integration_mode == "gpl_internal_component"
  and .rts_data_source_manifest.copied_or_derived == true
  and (.rts_data_source_manifest.source_paths | index("mods/trnm/maps/first-contact-basin/map.yaml") != null)
  and (.rts_data_canonical_sha256 | type == "string" and length == 64)
  and .rts_data_validation_error == null
  and .rts_data_consumer_gate == true
  and .rts_data_terrain_profile_count == 1156
  and .rts_data_terrain_profile_samples.border.role == "border"
  and .rts_data_terrain_profile_samples.lane.role == "lane"
  and .rts_data_terrain_profile_samples.center.height == 2
  and .rts_data_terrain_profile_samples.base_pad.base_pad == true
  and .rts_data_terrain_profile_samples.resource_zone.resource_zone == true
  and .rts_data_terrain_profile_gate == true
  and .rts_data_renderer_projection.renderable_tile_count == 1024
  and .rts_data_renderer_projection.lane_tile_count == 240
  and .rts_data_renderer_projection.resource_zone_tile_count == 79
  and .rts_data_renderer_projection.base_pad_tile_count == 144
  and .rts_data_renderer_projection.minimap_anchor_actor_count == 39
  and .rts_data_renderer_projection.resource_actor_tile_count == 11
  and .rts_data_renderer_projection.objective_actor_tile_count == 4
  and .rts_data_renderer_projection.spawn_actor_tile_count == 4
  and (.rts_data_renderer_projection.lane_tile_samples[] | select(.x == 16 and .y == 1))
  and (.rts_data_renderer_projection.resource_actor_tile_samples[] | select(.x == 12 and .y == 16))
  and (.rts_data_renderer_projection.objective_actor_tile_samples[] | select(.x == 16 and .y == 9))
  and (.rts_data_renderer_projection.spawn_actor_tile_samples[] | select(.x == 8 and .y == 8))
  and (.rts_data_renderer_projection.minimap_anchor_actor_samples | index("Actor0") != null)
  and .rts_data_renderer_projection_gate == true
  and .rts_data_opening_profile.contract_version == "trnm_rts_data_first_contact_opening_profile_v1"
  and .rts_data_opening_profile.map_id == "first_contact_basin"
  and .rts_data_opening_profile.active_beacon_tile.x == 16
  and .rts_data_opening_profile.active_beacon_tile.y == 9
  and .rts_data_opening_profile.active_relay_tile.x == 11
  and .rts_data_opening_profile.active_relay_tile.y == 8
  and .rts_data_command_feedback_profile.contract_version == "trnm_rts_data_first_contact_command_feedback_v1"
  and .rts_data_command_feedback_profile.target_tile.x == 16
  and .rts_data_command_feedback_profile.target_tile.y == 9
  and .rts_data_command_feedback_profile.blocked_tile.x == 15
  and .rts_data_command_feedback_profile.blocked_tile.y == 16
  and .rts_data_opening_profile_gate == true
  and .rts_data_command_feedback_gate == true
  and (.rts_data_player_startup_profiles | length) == 4
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi0" and .faction == "horizon" and .spawn_tile.x == 8 and .spawn_tile.y == 8 and .faction_unit_rule_id == "trnm.horizon.scout"))
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi1" and .faction == "forge" and .spawn_tile.x == 25 and .spawn_tile.y == 25 and .faction_unit_rule_id == "trnm.forge.warden"))
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi2" and .faction == "horizon" and .spawn_tile.x == 25 and .spawn_tile.y == 8 and .faction_unit_rule_id == "trnm.horizon.scout"))
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi3" and .faction == "forge" and .spawn_tile.x == 8 and .spawn_tile.y == 25 and .faction_unit_rule_id == "trnm.forge.warden"))
  and .rts_data_player_startup_gate == true
  and .rts_data_actor_presentation_contract == "trnm_rts_data_first_contact_actor_presentation_v1"
  and .rts_data_actor_glyph_contract == "trnm_rts_data_first_contact_actor_glyph_v1"
  and (.rts_data_actor_presentation_profiles | length) >= 13
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "mpspawn" and .glyph.body == "spawn_pad" and .glyph.accent == "owner_stripe" and .glyph.footprint_width_cells == 3))
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "trnm.worker" and .color_role == "worker" and .glyph_role == "worker" and .structure == false and .selectable == true and .glyph.body == "unit" and .glyph.accent == "worker_cargo" and .glyph.selection_ring == true))
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "trnm.command.core" and .color_role == "command_core" and .glyph_role == "command_core" and .structure == true and .health_bar_width >= 32 and .glyph.body == "structure" and .glyph.accent == "command_spire" and .glyph.footprint_width_cells == 2))
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "trnm.flux.beacon" and .color_role == "objective" and .glyph_role == "beacon" and .structure == true and .glyph.body == "objective_beacon" and .glyph.accent == "beacon_core"))
  and .rts_data_actor_presentation_gate == true
  and .rts_data_visual_telemetry_contract == "trnm_rts_data_first_contact_visual_telemetry_v1"
  and .rts_data_visual_telemetry_profile.contract_version == "trnm_rts_data_first_contact_visual_telemetry_v1"
  and .rts_data_visual_telemetry_profile.map_id == "first_contact_basin"
  and (.rts_data_visual_telemetry_profile.unit_statuses | length) == 4
  and (.rts_data_visual_telemetry_profile.tactical_tracks | length) == 6
  and (.rts_data_visual_telemetry_profile.unit_statuses[] | select(.tile.x == 8 and .tile.y == 8 and .role_badge == "W" and .role_color == "health" and .health_percent == 82 and .shield_percent == 44))
  and (.rts_data_visual_telemetry_profile.tactical_tracks[] | select(.from_tile.x == 11 and .from_tile.y == 8 and .to_tile.x == 16 and .to_tile.y == 9 and .color_role == "action_trail"))
  and .rts_data_visual_telemetry_gate == true
  and .rts_data_player_screen_contract == "trnm_rts_data_first_contact_player_screen_v1"
  and .rts_data_player_screen_profile.contract_version == "trnm_rts_data_first_contact_player_screen_v1"
  and .rts_data_player_screen_profile.map_id == "first_contact_basin"
  and .rts_data_player_screen_profile.room_id == "first-contact-basin"
  and .rts_data_player_screen_profile.layout.player_map.map_origin_x == 16
  and .rts_data_player_screen_profile.layout.player_map.map_origin_y == 54
  and .rts_data_player_screen_profile.layout.player_map.right_reserved_px == 292
  and .rts_data_player_screen_profile.layout.player_map.bottom_reserved_px == 158
  and .rts_data_player_screen_profile.layout.player_map.cell_width.min == 12
  and .rts_data_player_screen_profile.layout.player_map.cell_width.max == 28
  and .rts_data_player_screen_profile.layout.player_map.cell_height.min == 8
  and .rts_data_player_screen_profile.layout.player_map.cell_height.max == 15
  and .rts_data_player_screen_layout_profile.player_map.map_origin_x == 16
  and .rts_data_player_screen_layout_profile.spec_map.map_origin_x == 24
  and .rts_data_player_screen_layout_profile.spec_map.map_origin_y == 110
  and .rts_data_player_screen_layout_profile.spec_map.right_reserved_px == 266
  and .rts_data_player_screen_layout_profile.spec_map.cell_width.min == 10
  and .rts_data_player_screen_layout_profile.spec_map.cell_width.max == 22
  and .rts_data_player_screen_layout_profile.map_outer_padding_px == 8
  and .rts_data_player_screen_layout_profile.map_inner_padding_px == 4
  and .rts_data_player_screen_layout_gate == true
  and .rts_data_player_screen_profile.chrome.top_title == "TRNM RTS"
  and .rts_data_player_screen_profile.chrome.skirmish_status_label == "LOCAL SKIRMISH  OWNED ASSETS"
  and .rts_data_player_screen_profile.chrome.tactical_view_title == "TACTICAL VIEW"
  and .rts_data_player_screen_profile.chrome.tactical_view_camera_prefix == "CAM"
  and .rts_data_player_screen_profile.chrome.tactical_view_zoom_prefix == "Z"
  and .rts_data_player_screen_profile.chrome.tactical_view_default_camera_tile.x == 16
  and .rts_data_player_screen_profile.chrome.tactical_view_default_camera_tile.y == 16
  and .rts_data_player_screen_profile.chrome.tactical_view_status_fallback == "GROUP 1  ATTACK QUEUED"
  and .rts_data_player_screen_profile.chrome.tactical_view_status_max_chars == 40
  and (.rts_data_player_screen_profile.chrome.resource_readouts | length) == 4
  and (.rts_data_player_screen_profile.chrome.resource_readouts[] | select(.kind == "credits" and .label == "CRED"))
  and (.rts_data_player_screen_profile.chrome.resource_readouts[] | select(.kind == "power" and .label == "PWR"))
  and (.rts_data_player_screen_profile.chrome.resource_readouts[] | select(.kind == "supply" and .label == "SUP"))
  and (.rts_data_player_screen_profile.chrome.resource_readouts[] | select(.kind == "visibility" and .label == "VIS"))
  and .rts_data_player_screen_profile.chrome.radar_title == "RADAR"
  and .rts_data_player_screen_profile.chrome.production_title == "PRODUCTION"
  and .rts_data_player_screen_profile.chrome.build_palette_title == "BUILD PALETTE"
  and .rts_data_player_screen_profile.chrome.production_empty_label == "ready"
  and .rts_data_player_screen_profile.chrome.production_slot_visible_count == 4
  and .rts_data_player_screen_profile.chrome.production_slot_column_count == 2
  and (.rts_data_player_screen_profile.chrome.build_palette_slots | length) == 8
  and (.rts_data_player_screen_profile.chrome.build_palette_slots[] | select(.label == "PWR" and .queue_id == "build:power_node@5,3"))
  and (.rts_data_player_screen_profile.chrome.build_palette_slots[] | select(.label == "RAX" and .queue_id == "build:training_hall@4,3"))
  and (.rts_data_player_screen_profile.chrome.build_palette_slots[] | select(.label == "UPG" and .queue_id == "upgrade:signal_blade"))
  and .rts_data_player_screen_profile.chrome.build_palette_visible_count == 8
  and .rts_data_player_screen_profile.chrome.build_palette_column_count == 4
  and .rts_data_player_screen_profile.chrome.tactics_title == "TACTICS"
  and (.rts_data_player_screen_profile.chrome.tactics_rows | length) == 5
  and (.rts_data_player_screen_profile.chrome.tactics_rows[] | select(.kind == "order" and .label == "ORDER" and .max_value_chars == 20))
  and (.rts_data_player_screen_profile.chrome.tactics_rows[] | select(.kind == "target" and .label == "TARGET" and .empty_label == "NONE"))
  and (.rts_data_player_screen_profile.chrome.tactics_rows[] | select(.kind == "camera" and .label == "CAM" and .empty_label == "-"))
  and (.rts_data_player_screen_profile.chrome.tactics_rows[] | select(.kind == "queue" and .label == "QUEUE"))
  and (.rts_data_player_screen_profile.chrome.tactics_rows[] | select(.kind == "build" and .label == "BUILD" and .empty_label == "NONE"))
  and .rts_data_player_screen_chrome_profile.selection_panel_title == "SELECTION"
  and .rts_data_player_screen_chrome_profile.selection_card_visible_count == 5
  and (.rts_data_player_screen_chrome_profile.selection_card_frame_ids | length) == 5
  and (.rts_data_player_screen_chrome_profile.selection_card_frame_ids | index("actor_player_idle_south") != null)
  and (.rts_data_player_screen_chrome_profile.selection_card_frame_ids | index("prop_banner") != null)
  and .rts_data_player_screen_chrome_profile.selection_card_health_fallback_percent == 80
  and .rts_data_player_screen_chrome_profile.selection_feedback_label_max_chars == 62
  and .rts_data_player_screen_chrome_profile.command_panel_title == "COMMANDS"
  and .rts_data_player_screen_chrome_profile.command_grid_slot_count == 12
  and .rts_data_player_screen_chrome_profile.command_grid_column_count == 6
  and (.rts_data_player_screen_chrome_profile.command_grid_slot_ids | length) == 6
  and (.rts_data_player_screen_chrome_profile.command_grid_slot_ids | index("relay") != null)
  and (.rts_data_player_screen_chrome_profile.command_grid_slot_ids | index("signal") != null)
  and .rts_data_player_screen_chrome_profile.command_slot_fallback_id == "hold"
  and .rts_data_player_screen_chrome_profile.order_queue_title == "ORDER QUEUE"
  and .rts_data_player_screen_chrome_profile.order_queue_empty_label == "NO ORDERS"
  and .rts_data_player_screen_chrome_profile.order_queue_visible_count == 5
  and .rts_data_player_screen_chrome_profile.order_queue_label_max_chars == 32
  and .rts_data_player_screen_chrome_profile.group_summary_prefix == "GROUP"
  and .rts_data_player_screen_chrome_profile.group_summary_suffix == "UNITS SELECTED"
  and .rts_data_player_screen_chrome_profile.production_slot_visible_count == 4
  and .rts_data_player_screen_chrome_profile.production_slot_column_count == 2
  and .rts_data_player_screen_chrome_profile.build_palette_visible_count == 8
  and .rts_data_player_screen_chrome_profile.build_palette_column_count == 4
  and .rts_data_player_screen_chrome_gate == true
  and .rts_data_player_screen_profile.camera_focus_tile.x == 16
  and .rts_data_player_screen_profile.camera_focus_tile.y == 16
  and .rts_data_player_screen_profile.command_destination_tile.x == 16
  and .rts_data_player_screen_profile.command_destination_tile.y == 9
  and (.rts_data_player_screen_profile.command_queue | length) == 4
  and (.rts_data_player_screen_profile.command_queue | index("build:trnm.flux.relay") != null)
  and (.rts_data_player_screen_profile.command_queue | index("train:trnm.worker") != null)
  and (.rts_data_player_screen_profile.command_queue | index("attack:trnm.flux.beacon") != null)
  and (.rts_data_player_screen_profile.production_queue | length) == 3
  and (.rts_data_player_screen_profile.production_queue | index("train:guard") != null)
  and (.rts_data_player_screen_profile.production_queue | index("upgrade:signal_blade") != null)
  and (.rts_data_player_screen_profile.build_queue | length) == 2
  and (.rts_data_player_screen_profile.build_queue | index("build:watch_tower") != null)
  and (.rts_data_player_screen_profile.build_queue | index("upgrade:training_hall") != null)
  and .rts_data_player_screen_profile.unit_health_percents == [96,78,71,34]
  and .rts_data_player_screen_profile.active_ability_id == "worker"
  and (.rts_data_player_screen_chrome_profile.command_grid_slot_ids | index("worker") != null)
  and .rts_data_player_screen_profile.ability_cooldown_percents == [0,0,16,0,42,25]
  and (.rts_data_player_screen_profile.ability_cooldown_percents | length) == (.rts_data_player_screen_chrome_profile.command_grid_slot_ids | length)
  and (.rts_data_player_screen_profile.visible_tiles | length) == 64
  and (.rts_data_player_screen_profile.fogged_tiles | length) == 6
  and .rts_data_player_screen_gate == true
  and .rts_evidence_contract == "trnm_rts_evidence_v1"
  and .rts_evidence_bevy_runtime_adapter.contract_version == "trnm_rts_evidence_bevy_runtime_adapter_v1"
  and .rts_evidence_bevy_runtime_adapter.runtime_contract == "trnm_rts_bevy_runtime_adapter_v1"
  and .rts_evidence_bevy_runtime_adapter.green == true
  and .rts_evidence_bevy_runtime_adapter_gate == true
  and .rts_bevy_runtime_adapter_contract == "trnm_rts_bevy_runtime_adapter_v1"
  and .rts_bevy_runtime_adapter_gate == true
  and .rts_bevy_runtime_minimap_cell_sample.x == 134
  and .rts_bevy_runtime_minimap_cell_sample.y == 175
  and .rts_evidence_bevy_runtime_adapter.scroll_camera_stage_count_sample == 6
  and .rts_evidence_bevy_runtime_adapter.scroll_camera_first_focus_tile_sample == {"x":9,"y":7}
  and .rts_evidence_bevy_runtime_adapter.scroll_camera_minimap_jump_tile_sample == "minimap_cursor_jump"
  and .rts_evidence_bevy_runtime_adapter.scroll_camera_bounds_clamped_sample == true
  and .rts_evidence_bevy_runtime_adapter.camera_minimap_stage_count_sample == 6
  and .rts_evidence_bevy_runtime_adapter.camera_minimap_viewport_rect_sample == {"x":19,"y":8,"width":33,"height":19}
  and .rts_evidence_bevy_runtime_adapter.camera_minimap_selection_follow_tile_sample == "mirror_captain"
  and .rts_evidence_bevy_runtime_adapter.camera_minimap_revealed_union_count_sample == 35
  and .rts_evidence_bevy_runtime_adapter.camera_minimap_zoom_rect_area_sample == 308
  and .rts_bevy_runtime_path_preview_sample == "queue_stack"
  and .rts_evidence_bevy_runtime_adapter.command_queue_path_preview_stage_count_sample == 6
  and .rts_evidence_bevy_runtime_adapter.command_queue_path_preview_action_kinds_sample == ["select-control-group","move","move","attack","queue","queue"]
  and .rts_evidence_bevy_runtime_adapter.command_queue_path_preview_action_payloads_sample == ["box:frontline","8,4:line","9,2:rally","arena_creep_attack","build:watch_tower@7,4","cancel:build:0"]
  and .rts_evidence_bevy_runtime_adapter.command_queue_path_preview_history_entries_sample == ["command_queue_path_preview:queue_stack","command_queue_path_preview:shift_waypoints","command_queue_path_preview:rally_chain","command_queue_path_preview:attack_focus","command_queue_path_preview:build_reservation","command_queue_path_preview:cancel_repath"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_stage_sample == "commit_spacing"
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_stage_count_sample == 6
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_action_payloads_sample == ["box:frontline","8,4:wedge","8,4:line","8,4:wedge","6,5:split","9,2:rally"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_history_entries_sample == ["formation_move_preview:destination_ghost","formation_move_preview:wedge_spacing","formation_move_preview:line_reflow","formation_move_preview:collision_avoidance","formation_move_preview:split_avoidance","formation_move_preview:commit_spacing"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_destination_slots_sample == ["8,4","7,4","8,5","9,4"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_preview_split_route_sample == ["5,5","6,4","6,5","7,5","6,6"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_formation_preview_stage_count_sample == 4
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_formation_preview_action_payloads_sample == ["28","1,31:line","1,31:line","1,31:line"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_formation_preview_history_entries_sample == ["control_group_recall_formation_preview:recall_focus_hud","control_group_recall_formation_preview:formation_anchor_slots","control_group_recall_formation_preview:queued_valid_members","control_group_recall_formation_preview:filtered_invalid"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_formation_preview_slot_tiles_sample == ["1,31","2,31"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_formation_preview_filtered_members_sample == ["missing:multi0.recall.formation.missing","foreign:map.actor1"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_override_preview_stage_count_sample == 4
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_override_preview_action_payloads_sample == ["26","18,31:line","27","20,30:line"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_override_preview_history_entries_sample == ["control_group_recall_override_preview:group_26_recall_focus","control_group_recall_override_preview:group_26_queued_order","control_group_recall_override_preview:group_27_override_cancel","control_group_recall_override_preview:group_27_final_filtered"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_override_preview_final_tiles_sample == ["20,30","22,30"]
  and .rts_evidence_bevy_runtime_adapter.control_group_recall_override_preview_canceled_members_sample == ["multi0.recall.override.runner","multi0.recall.override.wing"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_execution_stage_sample == "arrival_lock"
  and .rts_evidence_bevy_runtime_adapter.formation_move_execution_stage_names_sample == ["slot_claim","path_reservation","stagger_step","crowd_avoidance","blocked_reroute","arrival_lock"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_execution_action_payloads_sample == ["box:frontline","8,4:wedge","8,4:line","6,5:split","8,4:wedge","9,2:rally"]
  and .rts_evidence_bevy_runtime_adapter.formation_move_execution_arrival_route_sample == ["6,5","7,5","8,5","9,4","9,2"]
  and .rts_evidence_bevy_runtime_adapter.local_obstruction_recovery_stage_sample == "flow_resume"
  and .rts_evidence_bevy_runtime_adapter.local_obstruction_recovery_stage_names_sample == ["detect_block","hold_queue","side_step","gap_claim","flow_resume"]
  and .rts_evidence_bevy_runtime_adapter.local_obstruction_recovery_action_payloads_sample == ["8,4:wedge","8,4:line","6,5:split","box:frontline","9,2:rally"]
  and .rts_evidence_bevy_runtime_adapter.local_obstruction_recovery_blocked_tiles_sample == ["7,4","7,5"]
  and .rts_evidence_bevy_runtime_adapter.local_obstruction_recovery_resume_route_sample == ["6,5","7,5","8,5","9,4","9,2"]
  and .rts_evidence_bevy_runtime_adapter.npc_behavior_stage_sample == "creep_retreat"
  and .rts_evidence_bevy_runtime_adapter.combat_impact_stage_sample == "damage_tick"
  and .rts_evidence_bevy_runtime_adapter.locomotion_blend_stage_sample == "formation_slide"
  and .rts_evidence_bevy_runtime_adapter.npc_transition_stage_sample == "hit_recover"
  and .rts_evidence_bevy_runtime_adapter.depth_readability_stage_sample == "target_priority"
  and .rts_evidence_bevy_runtime_adapter.structure_modeling_stage_sample == "repair_beam"
  and .rts_evidence_bevy_runtime_adapter.environment_life_stage_sample == "resource_glint"
  and .rts_evidence_bevy_runtime_adapter.worker_harvest_animation_stage_sample == "return_path"
  and .rts_evidence_bevy_runtime_adapter.production_spawn_animation_stage_sample == "supply_flash"
  and .rts_evidence_bevy_runtime_adapter.action_cadence_attack_mark_count_sample == 22
  and .rts_evidence_bevy_runtime_adapter.action_cadence_carry_mark_count_sample == 8
  and .rts_evidence_bevy_runtime_adapter.action_cadence_idle_mark_count_sample == 4
  and .rts_evidence_bevy_runtime_adapter.action_cadence_creep_windup_offset_sample == -24
  and .rts_evidence_bevy_runtime_adapter.action_sequence_phase_sample == "recovery"
  and .rts_evidence_bevy_runtime_adapter.action_sequence_windup_mark_count_sample == 9
  and .rts_evidence_bevy_runtime_adapter.action_sequence_strike_mark_count_sample == 12
  and .rts_evidence_bevy_runtime_adapter.action_sequence_carry_down_mark_count_sample == 5
  and .rts_evidence_bevy_runtime_adapter.action_sequence_idle_mark_count_sample == 6
  and .rts_evidence_bevy_runtime_adapter.unit_model_depth_guard_mark_count_sample == 8
  and .rts_evidence_bevy_runtime_adapter.unit_model_depth_worker_mark_count_sample == 8
  and .rts_evidence_bevy_runtime_adapter.unit_model_depth_creep_mark_count_sample == 8
  and .rts_evidence_bevy_runtime_adapter.unit_model_depth_creep_role_prop_count_sample == 2
  and .rts_evidence_bevy_runtime_adapter.unit_model_depth_face_shade_offset_sample == -32
  and .rts_evidence_bevy_runtime_adapter.command_surface_stage_sample == "target_queue"
  and .rts_bevy_runtime_command_grid_hit_sample == 0
  and (.rts_evidence_bevy_runtime_adapter.tile_line_sample | length) == 9
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[0].step_index == 0
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[0].tile_x == 8
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[0].tile_y == 8
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[4].step_index == 4
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[4].tile_x == 10
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[4].tile_y == 12
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[8].step_index == 8
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[8].tile_x == 12
  and .rts_evidence_bevy_runtime_adapter.tile_line_sample[8].tile_y == 16
  and .rts_evidence_bevy_runtime_adapter.combat_engagement_tiles_sample == ["9,3","10,3","10,2","11,2"]
  and .rts_evidence_bevy_runtime_adapter.combat_flash_tiles_sample == ["6,5","6,4"]
  and .rts_evidence_bevy_runtime_adapter.combat_target_tile_sample.x == 9
  and .rts_evidence_bevy_runtime_adapter.combat_target_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.combat_target_priority_sample == ["arena_creep_attack","arena_guard_support","arena_worker_support"]
  and .rts_evidence_bevy_runtime_adapter.combat_projectile_trail_sample == ["5,5","6,5","7,4","8,3"]
  and .rts_evidence_bevy_runtime_adapter.combat_ability_effect_tiles_sample == ["10,3","10,2","11,2","9,3"]
  and .rts_evidence_bevy_runtime_adapter.combat_threat_levels_sample == [88,66,41]
  and .rts_evidence_bevy_runtime_adapter.combat_damage_ticks_sample == [16,21,35]
  and .rts_evidence_bevy_runtime_adapter.combat_projectile_id_sample == "guard_break_bolt"
  and .rts_evidence_bevy_runtime_adapter.ai_pressure_wave_units_sample == ["lane_scout","mirror_raider","siege_runner"]
  and .rts_evidence_bevy_runtime_adapter.ai_pressure_tiles_sample == ["9,3","8,4","7,4","6,5"]
  and .rts_evidence_bevy_runtime_adapter.ai_pressure_counter_tiles_sample == ["5,5","6,5","6,4","7,5"]
  and .rts_evidence_bevy_runtime_adapter.enemy_pressure_wave_units_sample == ["enemy_raider","enemy_signal_guard","enemy_sapper"]
  and .rts_evidence_bevy_runtime_adapter.enemy_pressure_lane_tiles_sample == ["10,2","9,3","8,4","7,4","6,5"]
  and .rts_evidence_bevy_runtime_adapter.recon_scout_route_tiles_sample == ["5,5","6,4","7,4","8,3","9,2","10,2"]
  and .rts_evidence_bevy_runtime_adapter.recon_fog_reveal_tiles_sample == ["7,4","8,3","8,2","9,2","9,3","10,2","10,3","11,1","11,2"]
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_structures_sample == ["enemy_watch_post","enemy_barracks","enemy_resource_vault"]
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_units_sample == ["enemy_scout","enemy_worker","enemy_guard"]
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_structure_tile_sample.x == 11
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_structure_tile_sample.y == 2
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_unit_tile_sample.x == 11
  and .rts_evidence_bevy_runtime_adapter.recon_enemy_unit_tile_sample.y == 2
  and .rts_evidence_bevy_runtime_adapter.base_assault_path_tiles_sample == ["5,5","6,5","7,4","8,4","9,3","10,3"]
  and .rts_evidence_bevy_runtime_adapter.base_assault_targets_sample == ["enemy_watch_post","enemy_barracks","enemy_resource_vault"]
  and .rts_evidence_bevy_runtime_adapter.aftermath_debris_tiles_sample == ["9,3","10,3","10,4","11,3"]
  and .rts_evidence_bevy_runtime_adapter.aftermath_smoke_tiles_sample == ["10,2","10,3","11,3"]
  and .rts_evidence_bevy_runtime_adapter.commander_aura_tiles_sample == ["6,5","7,4","8,4","9,3","10,3"]
  and .rts_evidence_bevy_runtime_adapter.commander_loot_items_sample == ["barracks_map_cache","field_banner_relic","repair_kit_crate"]
  and .rts_evidence_bevy_runtime_adapter.expansion_claim_tiles_sample == ["8,2","9,2","10,2","9,3","10,3"]
  and .rts_evidence_bevy_runtime_adapter.expansion_structure_tile_sample.x == 8
  and .rts_evidence_bevy_runtime_adapter.expansion_structure_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.expansion_workers_sample == ["expansion_worker_alpha","expansion_worker_beta","expansion_worker_gamma"]
  and .rts_evidence_bevy_runtime_adapter.counterattack_units_sample == ["counter_raider_alpha","counter_raider_beta","counter_sapper"]
  and .rts_evidence_bevy_runtime_adapter.counterattack_route_tiles_sample == ["11,2","10,2","9,3","8,3","7,4","9,2"]
  and .rts_evidence_bevy_runtime_adapter.army_units_sample == ["relay_guard_alpha","relay_guard_beta","wayfinder_scout","field_mender"]
  and .rts_evidence_bevy_runtime_adapter.army_rally_tiles_sample == ["5,5","6,5","7,4","8,4","8,3"]
  and .rts_evidence_bevy_runtime_adapter.player_army_unit_tile_sample.x == 6
  and .rts_evidence_bevy_runtime_adapter.player_army_unit_tile_sample.y == 4
  and .rts_evidence_bevy_runtime_adapter.central_keep_route_tiles_sample == ["12,3","12,4","13,4","13,3","14,3"]
  and .rts_evidence_bevy_runtime_adapter.central_keep_tile_sample.x == 13
  and .rts_evidence_bevy_runtime_adapter.central_keep_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.boss_guard_units_sample == ["keep_warden_alpha","keep_warden_beta","ward_sentinel"]
  and .rts_evidence_bevy_runtime_adapter.player_siege_line_tiles_sample == ["11,4","12,4","13,4","12,3"]
  and .rts_evidence_bevy_runtime_adapter.keep_breach_tiles_sample == ["13,3","13,4","14,3","14,4"]
  and .rts_evidence_bevy_runtime_adapter.guardian_counter_units_sample == ["high_warden","ward_lancer","last_mirror_guard"]
  and .rts_evidence_bevy_runtime_adapter.keep_claim_tiles_sample == ["12,3","13,3","14,3","13,4"]
  and .rts_evidence_bevy_runtime_adapter.objective_tiles_sample == ["6,5","6,4","7,5","9,2"]
  and .rts_evidence_bevy_runtime_adapter.creep_camp_tiles_sample == ["8,3","8,2","9,3","9,2"]
  and .rts_evidence_bevy_runtime_adapter.terrain_route_tiles_sample == ["5,5","6,5","7,4","8,3"]
  and .rts_evidence_bevy_runtime_adapter.terrain_choke_tiles_sample == ["7,4","7,3","8,4"]
  and .rts_evidence_bevy_runtime_adapter.expansion_tiles_sample == ["9,2","10,2","10,3"]
  and .rts_evidence_bevy_runtime_adapter.siege_units_sample == ["stonebreak_cart"]
  and .rts_evidence_bevy_runtime_adapter.siege_push_route_tiles_sample == ["9,2","9,3","10,3","10,2","11,2","10,3"]
  and .rts_evidence_bevy_runtime_adapter.siege_breach_tiles_sample == ["9,3","10,3","10,2","11,2","10,3"]
  and .rts_evidence_bevy_runtime_adapter.enemy_fortification_tile_sample.x == 10
  and .rts_evidence_bevy_runtime_adapter.enemy_fortification_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.enemy_repair_units_sample == ["repair_adept_alpha","repair_adept_beta"]
  and .rts_evidence_bevy_runtime_adapter.enemy_flank_units_sample == ["ridge_sentry_left","ridge_sentry_right","ridge_sapper"]
  and .rts_evidence_bevy_runtime_adapter.enemy_flank_tile_sample.x == 8
  and .rts_evidence_bevy_runtime_adapter.enemy_flank_tile_sample.y == 4
  and .rts_evidence_bevy_runtime_adapter.player_hold_tiles_sample == ["8,3","9,3","9,4","10,3"]
  and .rts_evidence_bevy_runtime_adapter.inner_lane_tiles_sample == ["10,3","11,2","11,3","12,3","12,4"]
  and .rts_evidence_bevy_runtime_adapter.inner_gate_tile_sample.x == 11
  and .rts_evidence_bevy_runtime_adapter.inner_gate_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.signal_lock_tile_sample.x == 12
  and .rts_evidence_bevy_runtime_adapter.signal_lock_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.inner_defenders_sample == ["inner_guard_alpha","inner_guard_beta","signal_lancer"]
  and .rts_evidence_bevy_runtime_adapter.supply_convoy_sample == ["convoy_cart","field_medic","ammo_runner"]
  and .rts_evidence_bevy_runtime_adapter.split_squad_tiles_sample == ["10,4","11,4","12,4","12,3"]
  and .rts_evidence_bevy_runtime_adapter.inner_core_tile_sample.x == 12
  and .rts_evidence_bevy_runtime_adapter.inner_core_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.restored_zones_sample == ["central_keep","signal_core","inner_lane","forest_relay"]
  and .rts_evidence_bevy_runtime_adapter.rebuild_structures_sample == ["signal_core","inner_latch","mirror_ward"]
  and .rts_evidence_bevy_runtime_adapter.garrison_units_sample == ["mirror_guard_alpha","signal_lancer","field_engineer"]
  and .rts_evidence_bevy_runtime_adapter.open_world_route_tiles_sample == ["13,3","12,3","11,3","10,2","9,2"]
  and .rts_evidence_bevy_runtime_adapter.open_world_panels_sample == ["room_panel:league-coliseum","task_panel:task-fixture-first-route","combat_panel:league-coliseum","save_panel:post_rts_restore"]
  and .rts_evidence_bevy_runtime_adapter.siege_unit_tile_sample.x == 9
  and .rts_evidence_bevy_runtime_adapter.siege_unit_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.harvest_tile_sample.x == 3
  and .rts_evidence_bevy_runtime_adapter.harvest_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.dropoff_tile_sample.x == 5
  and .rts_evidence_bevy_runtime_adapter.dropoff_tile_sample.y == 5
  and .rts_evidence_bevy_runtime_adapter.build_site_tiles_sample == ["7,4","7,5","8,4"]
  and .rts_evidence_bevy_runtime_adapter.structure_tile_sample.x == 4
  and .rts_evidence_bevy_runtime_adapter.structure_tile_sample.y == 3
  and .rts_evidence_bevy_runtime_adapter.unlock_unit_tile_sample.x == 7
  and .rts_evidence_bevy_runtime_adapter.unlock_unit_tile_sample.y == 5
  and .rts_evidence_bevy_runtime_adapter.queue_gold_cost_sample == 210
  and .rts_evidence_bevy_runtime_adapter.queue_available_gold_sample == 40
  and .rts_evidence_bevy_runtime_adapter.queue_affordable_sample == false
  and .rts_evidence_bevy_runtime_adapter.queue_build_parts_sample == ["watch_tower","7,4"]
  and .rts_evidence_bevy_runtime_adapter.queue_production_lane_sample == true
  and .rts_evidence_bevy_runtime_adapter.queue_feedback_chip_sample == "feedback:build_placed:watch_tower@7,4"
  and .rts_evidence_bevy_runtime_adapter.blocked_feedback_chip_visible_sample == true
  and .rts_evidence_bevy_runtime_adapter.queue_blocked_feedback_label_sample == "QUEUE LOCK NEED 210G"
  and .rts_evidence_bevy_runtime_adapter.command_panel_slot_id_sample == "attack"
  and .rts_evidence_bevy_runtime_adapter.command_panel_build_palette_queue_id_sample == "build:watch_tower@7,4"
  and .rts_evidence_bevy_runtime_adapter.command_panel_production_slot_queue_id_sample == "build:watch_tower@7,4"
  and .rts_evidence_bevy_runtime_adapter.command_panel_sidebar_cancel_queue_id_sample == "cancel:build:0"
  and .rts_evidence_bevy_runtime_adapter.command_panel_palette_cancel_queue_id_sample == "cancel:active_build"
  and .rts_evidence_bevy_runtime_adapter.command_panel_sidebar_slot_status_label_sample == "B1 66 R"
  and .rts_evidence_bevy_runtime_adapter.command_panel_palette_state_label_sample == "ACT"
  and .rts_evidence_bevy_runtime_adapter.command_panel_sidebar_queue_summary_sample == "P:worker@42% B:watch_tower@66%"
  and .rts_evidence_bevy_runtime_adapter.command_panel_spawned_unit_id_sample == "worker_3"
  and .rts_evidence_bevy_runtime_adapter.command_panel_structure_id_sample == "watch_tower"
  and .rts_evidence_bevy_runtime_adapter.scripted_demo_pauses_queue_tick_sample == true
  and .rts_evidence_bevy_runtime_adapter.scripted_demo_stage_from_frame_sample == 4
  and .rts_evidence_bevy_runtime_adapter.scripted_demo_stage_id_sample == "cancel_refund"
  and .rts_evidence_bevy_runtime_adapter.scripted_demo_stage_title_sample == "WORKER QUEUED"
  and .rts_evidence_bevy_runtime_adapter.selection_default_units_sample == ["player", "square_guard_patrol", "square_worker_carry", "square_creep_wander"]
  and .rts_evidence_bevy_runtime_adapter.selection_same_class_units_sample == ["player", "square_guard_front", "square_guard_patrol"]
  and .rts_evidence_bevy_runtime_adapter.selection_guard_tile_sample.x == 7
  and .rts_evidence_bevy_runtime_adapter.selection_guard_tile_sample.y == 5
  and .rts_evidence_bevy_runtime_adapter.selection_drag_units_sample == ["player", "square_guard_front", "square_guard_patrol", "square_worker_carry", "square_worker_harvest"]
  and .rts_evidence_bevy_runtime_adapter.selection_drag_rejected_units_sample == ["square_creep_wander"]
  and .rts_evidence_bevy_runtime_adapter.selection_drag_distance_sq_sample == 107300
  and .rts_evidence_bevy_runtime_adapter.selection_drag_ready_sample == true
  and .rts_evidence_bevy_runtime_adapter.selection_drag_group_id_sample == "drag:4,4->8,5"
  and .rts_evidence_bevy_runtime_adapter.selection_drag_player_label_sample == "DRAG SELECT 5 UNITS 4,4->8,5"
  and .rts_evidence_bevy_runtime_adapter.selection_tiles_for_units_sample == ["5,4", "4,5"]
  and .rts_evidence_bevy_runtime_adapter.control_group_hotkey_slot_sample == "10"
  and .rts_evidence_bevy_runtime_adapter.control_group_default_slot_three_units_sample == ["square_worker_carry", "square_worker_harvest"]
  and .rts_evidence_bevy_runtime_adapter.control_group_assignment_units_sample == ["square_worker_carry", "square_worker_harvest"]
  and .rts_evidence_bevy_runtime_adapter.control_group_summary_slot_ten_sample.slot == "10"
  and .rts_evidence_bevy_runtime_adapter.control_group_summary_slot_ten_sample.key_label == "0"
  and .rts_evidence_bevy_runtime_adapter.control_group_summary_slot_ten_sample.member_count == 2
  and .rts_evidence_bevy_runtime_adapter.control_group_summary_slot_ten_sample.occupied == true
  and .rts_evidence_bevy_runtime_adapter.control_group_summary_slot_ten_sample.active == true
  and .rts_evidence_bevy_runtime_adapter.control_group_merged_units_sample == ["player", "square_worker_carry"]
  and .rts_evidence_bevy_runtime_adapter.selection_clear_parts_sample == ["hostile", "square_creep_wander", "9,4"]
  and .rts_evidence_bevy_runtime_adapter.move_command_parts_sample == ["9,2", "attack_move"]
  and .rts_evidence_bevy_runtime_adapter.line_path_tiles_sample == ["6,5", "7,4", "8,3"]
  and .rts_evidence_bevy_runtime_adapter.focus_fire_units_sample == ["relay_guard_alpha", "relay_guard_beta", "wayfinder_scout", "field_mender"]
  and .rts_evidence_bevy_runtime_adapter.creep_camp_units_sample == ["forest_alpha_creep", "forest_stalker", "forest_shaman"]
  and .rts_evidence_bevy_runtime_adapter.command_parts_samples == [["claim", "relay_beacon", "9,2"], ["clear", "forest_creep_camp", "8,3"], ["mark", "enemy_base", "10,2"], ["pressure", "counter_wave", "enemy_gate"], ["upgrade", "signal_blade", "training_hall"], ["train", "mixed_vanguard", "training_hall"], ["breach", "enemy_barracks", "10,3"], ["destroy", "enemy_barracks", "10,3"], ["level", "mirror_captain", "forest_relay"], ["claim", "forest_relay", "9,2"], ["tech", "stonebreak_cart", "relay_outpost"]]
  and .rts_evidence_bevy_runtime_adapter.selection_command_stamp_sample.kind == "control-group"
  and .rts_evidence_bevy_runtime_adapter.selection_command_stamp_sample.target_id == "5"
  and .rts_evidence_bevy_runtime_adapter.selection_command_stamp_sample.player_label == "HOTKEY GROUP 5 ASSIGNED 2 UNITS"
  and .rts_evidence_bevy_runtime_adapter.move_command_stamp_sample.kind == "move"
  and .rts_evidence_bevy_runtime_adapter.move_command_stamp_sample.tile_id == "7,4"
  and .rts_evidence_bevy_runtime_adapter.move_command_stamp_sample.player_label == "MAP MOVE SENT 7,4"
  and .rts_evidence_bevy_runtime_adapter.ability_command_stamp_sample.kind == "ability"
  and .rts_evidence_bevy_runtime_adapter.ability_command_stamp_sample.tile_id == "6,5"
  and .rts_evidence_bevy_runtime_adapter.ability_command_stamp_sample.target_id == "arena_creep_attack"
  and .rts_evidence_bevy_runtime_adapter.ability_command_stamp_sample.player_label == "COMMAND BAR ABILITY SENT FOCUS FIRE"
  and .rts_evidence_bevy_runtime_adapter.order_queue_replay_action_samples == [{"kind":"attack","payload":"arena_creep_attack"},{"kind":"move","payload":"9,2:line"},{"kind":"move","payload":"minimap:rally:5,2"},{"kind":"queue","payload":"train:worker"},{"kind":"select-control-group","payload":"3"},{"kind":"ability","payload":"focus_fire"}]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_strip_stage_sample == "group_27_override"
  and .rts_evidence_bevy_runtime_adapter.command_feedback_strip_fixture_stage_names_sample == ["group_26_queued","group_27_override","group_28_formation","group_28_filtered"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_strip_fixture_action_payloads_sample == ["18,31:line","27","1,31:line","1,31:line"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_strip_fixture_focus_tiles_sample == ["18,30","21,30","1,30","1,30"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_strip_fixture_filtered_members_sample == ["missing:multi0.recall.formation.missing","foreign:map.actor1"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_lifecycle_stage_sample == "dimmed"
  and .rts_evidence_bevy_runtime_adapter.command_feedback_lifecycle_fixture_stage_names_sample == ["fresh","dimmed","cleared"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_lifecycle_fixture_action_payloads_sample == ["18,31:line","1,31:line","28"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_lifecycle_fixture_age_ticks_sample == [0,4,8]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_lifecycle_fixture_events_sample == ["control_group_command_feedback_lifecycle:fresh","control_group_command_feedback_lifecycle:dimmed","control_group_command_feedback_lifecycle:cleared"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_replay_step_names_sample == ["select_group_26","queue_group_26","select_group_27","override_group_27","select_group_28","formation_group_28","bounded_history_after_clear"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_replay_preview_stages_sample == ["group_26_queued","group_27_override","group_28_formation","cleared_history_bounded"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_replay_retained_group_ids_sample == ["26","27","28"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_replay_pruned_group_ids_sample == ["25","24"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_replay_history_badges_sample == ["QUEUE","CANCEL_FINAL","FORMATION_FILTER_CLEAR"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_step_names_sample == ["move_without_group_selection","select_group_26_setup","move_invalid_tile_after_selection","attack_without_target","ability_before_attack_target","queue_without_queue_id","queue_unaffordable_build_after_selection","select_without_group_id"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_preview_stages_sample == ["group_selection_required","invalid_tile","attack_target_required","history_preserved_after_rejections"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_input_sources_sample == ["classic_rts_mouse_viewport","classic_rts_hotkey","classic_rts_mouse_viewport","classic_rts_mouse_viewport","classic_rts_hotkey","classic_rts_mouse_sidebar","classic_rts_mouse_sidebar","classic_rts_hotkey"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_blocked_reasons_sample == ["rts_group_selection_required","rts_invalid_tile:bad-tile","rts_attack_target_required","rts_attack_required_before_ability","rts_queue_id_required","rts_queue_unaffordable:build:watch_tower@7,4","rts_group_id_required"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_visual_stages_sample == ["group_selection_required","invalid_tile","attack_target_required","history_preserved_after_rejections"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_retained_group_ids_sample == ["26","27","28"]
  and .rts_evidence_bevy_runtime_adapter.command_feedback_rejection_replay_pruned_group_ids_sample == ["25","24"]
  and .rts_evidence_bevy_runtime_adapter.command_history_visible_sample == true
  and .rts_evidence_bevy_runtime_adapter.command_history_prune_visible_sample == true
  and .rts_evidence_bevy_runtime_adapter.command_history_fixture_stage_names_sample == ["fresh_history_appended","dimmed_history_retained","cleared_history_retained"]
  and .rts_evidence_bevy_runtime_adapter.command_history_fixture_lifecycle_stages_sample == ["fresh","dimmed","cleared"]
  and .rts_evidence_bevy_runtime_adapter.command_history_fixture_group_ids_sample == ["26","27","28"]
  and .rts_evidence_bevy_runtime_adapter.command_history_prune_fixture_stage_names_sample == ["overflow_input_pruned","recent_three_retained","cleared_history_bounded"]
  and .rts_evidence_bevy_runtime_adapter.command_history_prune_fixture_pruned_group_ids_sample == ["25","24"]
  and .rts_evidence_bevy_runtime_adapter.command_history_prune_fixture_prune_reasons_sample == ["recent_three_capacity","recent_three_capacity"]
  and .rts_evidence_bevy_runtime_adapter.command_execution_feedback_kind_samples == ["move","follow","attack","harvest"]
  and .rts_evidence_bevy_runtime_adapter.command_execution_target_label_samples == ["8,4","player","arena_creep_attack","gold_vein"]
  and .rts_evidence_bevy_runtime_adapter.command_execution_player_label_samples == ["MOVE EXECUTING 8,4","FOLLOWING PLAYER","ATTACK FOCUS ARENA CREEP ATTACK","HARVEST GOLD VEIN TO TOWN HALL"]
  and .rts_evidence_bevy_runtime_adapter.command_execution_target_tile_samples == [{"x":8,"y":4},{"x":5,"y":4},{"x":6,"y":5},{"x":3,"y":3}]
  and .rts_evidence_bevy_runtime_adapter.hover_target_preview_kind_sample == "attack"
  and .rts_evidence_bevy_runtime_adapter.hover_cursor_kind_sample == "ability"
  and .rts_evidence_bevy_runtime_adapter.hover_cursor_label_sample == "COMMAND BAR CURSOR ABILITY READY"
  and .rts_evidence_bevy_runtime_adapter.blocked_cursor_kind_sample == "blocked"
  and .rts_evidence_bevy_runtime_adapter.blocked_cursor_label_sample == "MAP CURSOR BLOCKED LOCK"
  and .rts_evidence_bevy_runtime_adapter.hover_player_label_sample == "MAP ATTACK READY SQUARE CREEP WANDER"
  and .rts_evidence_bevy_runtime_adapter.hover_queue_player_label_sample == "SIDEBAR QUEUE READY WATCH TOWER 7,4 210G"
  and .rts_evidence_bevy_runtime_adapter.blocked_hover_player_label_sample == "MAP MOVE LOCK SELECT UNITS"
  and .rts_evidence_bevy_runtime_adapter.unit_status_stage_sample == "commander"
  and .rts_evidence_bevy_runtime_adapter.unit_status_unit_id_sample == "mirror_captain"
  and .rts_evidence_bevy_runtime_adapter.unit_status_health_sample == 76
  and .rts_evidence_bevy_runtime_adapter.unit_status_energy_sample == 68
  and .rts_evidence_bevy_runtime_adapter.unit_status_role_badges_sample == ["AUR","LVL","CMD"]
  and .rts_evidence_bevy_runtime_adapter.selection_feedback_stage_sample == "attack_lock"
  and .rts_evidence_bevy_runtime_adapter.ability_tooltip_stage_sample == "range_preview"
  and .rts_evidence_bevy_runtime_adapter.control_group_hotkey_feedback_stage_sample == "double_tap_camera"
  and .rts_bevy_runtime_map_projection.map_x == 16
  and .rts_bevy_runtime_map_projection.map_y == 54
  and .rts_bevy_runtime_map_projection.cell_w == 28
  and .rts_bevy_runtime_map_projection.cell_h == 14
  and .rts_bevy_runtime_map_projection.map_w == 952
  and .rts_bevy_runtime_map_projection.map_h == 476
  and .rts_bevy_runtime_tile_rect_sample.x == 464
  and .rts_bevy_runtime_tile_rect_sample.y == 278
  and .rts_bevy_runtime_tile_rect_sample.width == 28
  and .rts_bevy_runtime_tile_rect_sample.height == 14
  and .rts_bevy_runtime_terrain_seed_sample.surface_seed == 12
  and .rts_bevy_runtime_terrain_seed_sample.detail_seed == 20
  and .rts_bevy_runtime_map_projection_gate == true
  and .rts_online_contract == "trnm_rts_online_protocol_v1"
  and .rts_online_protocol_fixture.contract_version == "trnm_rts_online_first_contact_fixture_v1"
  and .rts_online_protocol_fixture.green == true
  and .rts_online_protocol_fixture.envelope.map_id == "first_contact_basin"
  and (.rts_online_protocol_fixture.envelope.update_sha256 | length) == 64
  and (.rts_online_protocol_fixture.envelope.scope.visible_chunks | length) == 3
  and (.rts_online_protocol_fixture.envelope.scope.fogged_chunks | length) == 2
  and (.rts_online_protocol_fixture.envelope.scope.visible_actor_ids | index("trnm.flux.beacon.center") != null)
  and .rts_online_protocol_fixture.authority.contract_version == "trnm_rts_online_authority_v1"
  and .rts_online_protocol_fixture.authority.green == true
  and .rts_online_protocol_fixture.authority.authority_tick == 43
  and (.rts_online_protocol_fixture.authority.authority_sha256 | length) == 64
  and (.rts_online_protocol_fixture.authority.client_requests | length) == 1
  and (.rts_online_protocol_fixture.authority.accepted_orders | length) == 1
  and (.rts_online_protocol_fixture.authority.accepted_orders[0].source == "server")
  and (.rts_online_protocol_fixture.authority.rejected_orders | length) == 1
  and .rts_online_protocol_fixture.authority.rejected_orders[0].reason == "target_actor_not_visible"
  and (.rts_online_protocol_fixture.authority.scoped_updates | length) == 1
  and (.rts_online_protocol_fixture.authority.scoped_updates[0].update_sha256 | length) == 64
  and (.rts_online_protocol_fixture.authority.scoped_updates[0].scope.visible_actor_ids | index("trnm.enemy.keep.fogged") == null)
  and .rts_online_protocol_fixture.transport.contract_version == "trnm_rts_online_loopback_transport_v1"
  and .rts_online_protocol_fixture.transport.green == true
  and .rts_online_protocol_fixture.transport.session_id == "first-contact-loopback-session"
  and .rts_online_protocol_fixture.transport.request_frame.direction == "client_to_server"
  and .rts_online_protocol_fixture.transport.request_frame.payload_kind == "client_request"
  and .rts_online_protocol_fixture.transport.request_frame.wire_magic == "TRNMRTS1"
  and (.rts_online_protocol_fixture.transport.request_frame.encoded_len > 96)
  and (.rts_online_protocol_fixture.transport.request_frame.payload_sha256 | length) == 64
  and (.rts_online_protocol_fixture.transport.request_frame.frame_sha256 | length) == 64
  and .rts_online_protocol_fixture.transport.response_frame.direction == "server_to_client"
  and .rts_online_protocol_fixture.transport.response_frame.payload_kind == "scoped_update"
  and .rts_online_protocol_fixture.transport.response_frame.wire_magic == "TRNMRTS1"
  and (.rts_online_protocol_fixture.transport.response_frame.encoded_len > 96)
  and (.rts_online_protocol_fixture.transport.response_frame.payload_sha256 | length) == 64
  and (.rts_online_protocol_fixture.transport.response_frame.frame_sha256 | length) == 64
  and .rts_online_protocol_fixture.transport.request_ack_matches_envelope == true
  and .rts_online_protocol_fixture.transport.response_matches_authority == true
  and .rts_online_protocol_fixture.transport.server_authoritative == true
  and .rts_online_protocol_fixture.transport.visibility_scoped_response == true
  and .rts_online_protocol_fixture.transport.socket_opened == false
  and .rts_online_protocol_fixture.transport.hosted_service_claimed == false
  and .rts_online_protocol_fixture.transport.public_launch_ready == false
  and .rts_online_protocol_fixture.lifecycle.phase == "playing"
  and .rts_online_protocol_fixture.lifecycle.bot_count == 1
  and .rts_online_protocol_gate == true
  and .bevy_data_actor_parity_gate == true
  and .bevy_map_model_adapter_gate == true
  and .ui_runtime_gate == true
  and (.rules[] | select(.id == "trnm.worker" and .cost == 200 and .hp == 8000))
  and (.rules[] | select(.id == "trnm.horizon.scout" and .speed == 92))
  and (.rules[] | select(.id == "trnm.forge.warden" and .hp == 18000))
  and (.rules[] | select(.id == "trnm.command.core" and .cost == 1600))
  and (.rules[] | select(.id == "trnm.flux.relay" and .cost == 500))
' "$OUT" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_BASIN_SPEC_GREEN %s\n' "$OUT"
