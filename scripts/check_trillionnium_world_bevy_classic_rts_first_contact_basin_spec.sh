#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json"
mkdir -p "$(dirname "$OUT")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-first-contact-basin-spec >"$OUT"
)

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
  and .rts_bevy_runtime_path_preview_sample == "queue_stack"
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
