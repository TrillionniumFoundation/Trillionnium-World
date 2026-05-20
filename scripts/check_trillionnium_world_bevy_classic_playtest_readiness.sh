#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-readiness.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_preview.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_selector.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_player_motion_probe.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_input_frame_budget.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_render_budget.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_scene_preview.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_renderer_probe.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_isometric_modeling.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_model_catalog.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_slot_map.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack_scene_probe.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_override_probe.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_loop.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_live_input_sequence.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_pathing_formation.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_collision_engagement.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_target_aggro_focus.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_economy_build.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_selection_minimap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_build_lifecycle.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_tech_tree.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_projectile_ability.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_ai_skirmish_pressure.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_objective_victory_loop.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_creep_camp_terrain_route.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_fog_scouting_intel.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_enemy_base_tech_pressure.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_army_production_rally.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_base_assault_resolution.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_battle_aftermath.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_commander_progression.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_expansion_counterattack.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_tier_two_siege_push.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_siege_breach_counterplay.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_inner_lane_breakthrough.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_central_keep_pressure.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_central_keep_breakthrough.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_mirror_city_restoration.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_open_world_after_action.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_handoff.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_entry.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_client_boundary.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_runner_status.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_launcher.sh" >/dev/null

jq -n \
  --slurpfile manifest "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-manifest-lint.json" \
  --slurpfile animation "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.json" \
  --slurpfile selector "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-selector.json" \
  --slurpfile motion "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-player-motion-probe.json" \
  --slurpfile input_budget "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-input-frame-budget.json" \
  --slurpfile budget "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-render-budget.json" \
  --slurpfile scene "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-scene-preview.json" \
  --slurpfile probe "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.json" \
  --slurpfile iso "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-isometric-modeling.json" \
  --slurpfile catalog "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.json" \
  --slurpfile slots "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-asset-slot-map.json" \
  --slurpfile art_pack "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack.json" \
  --slurpfile art_scene "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack-scene-probe.json" \
  --slurpfile override "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-asset-override-probe.json" \
  --slurpfile rts "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.json" \
  --slurpfile rts_live "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-input-sequence.json" \
  --slurpfile rts_path "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-pathing-formation.json" \
  --slurpfile rts_collision "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-collision-engagement.json" \
  --slurpfile rts_target "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-target-aggro-focus.json" \
  --slurpfile rts_economy "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-economy-build.json" \
  --slurpfile rts_select "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-minimap.json" \
  --slurpfile rts_build_lifecycle "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-build-lifecycle.json" \
  --slurpfile rts_tech_tree "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tech-tree.json" \
  --slurpfile rts_projectile "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-projectile-ability.json" \
  --slurpfile rts_ai "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ai-skirmish-pressure.json" \
  --slurpfile rts_objective "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-objective-victory-loop.json" \
  --slurpfile rts_creep_camp "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-creep-camp-terrain-route.json" \
  --slurpfile rts_fog "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-fog-scouting-intel.json" \
  --slurpfile rts_enemy_base "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-enemy-base-tech-pressure.json" \
  --slurpfile rts_army "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-army-production-rally.json" \
  --slurpfile rts_base_assault "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-base-assault-resolution.json" \
  --slurpfile rts_aftermath "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-battle-aftermath.json" \
  --slurpfile rts_commander "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-commander-progression.json" \
  --slurpfile rts_expansion "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-expansion-counterattack.json" \
  --slurpfile rts_tier_two "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tier-two-siege-push.json" \
  --slurpfile rts_breach "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-siege-breach-counterplay.json" \
  --slurpfile rts_inner "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-inner-lane-breakthrough.json" \
  --slurpfile rts_keep "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-pressure.json" \
  --slurpfile rts_keep_break "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-breakthrough.json" \
  --slurpfile rts_restore "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-mirror-city-restoration.json" \
  --slurpfile rts_open_world "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-open-world-after-action.json" \
  --slurpfile rts_campaign "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-handoff.json" \
  --slurpfile rts_campaign_entry "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-entry.json" \
  --slurpfile boundary "$ROOT/acceptance/S6_public_launch/latest/client-boundary-cleanliness.json" \
  --slurpfile runner "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json" \
  --slurpfile launcher "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-launcher.json" '
  def ok($x): ($x[0].green == true);
  {
    contract_version: "trillionnium_world_bevy_classic_playtest_readiness_v1",
    green: (
      ok($manifest)
      and ok($animation)
      and ok($selector)
      and ok($motion)
      and ok($input_budget)
      and ok($budget)
      and ok($scene)
      and ok($probe)
      and ok($iso)
      and ok($catalog)
      and ok($slots)
      and ok($art_pack)
      and ok($art_scene)
      and ok($override)
      and ok($rts)
      and ok($rts_live)
      and ok($rts_path)
      and ok($rts_collision)
      and ok($rts_target)
      and ok($rts_economy)
      and ok($rts_select)
      and ok($rts_build_lifecycle)
      and ok($rts_tech_tree)
      and ok($rts_projectile)
      and ok($rts_ai)
      and ok($rts_objective)
      and ok($rts_creep_camp)
      and ok($rts_fog)
      and ok($rts_enemy_base)
      and ok($rts_army)
      and ok($rts_base_assault)
      and ok($rts_aftermath)
      and ok($rts_commander)
      and ok($rts_expansion)
      and ok($rts_tier_two)
      and ok($rts_breach)
      and ok($rts_inner)
      and ok($rts_keep)
      and ok($rts_keep_break)
      and ok($rts_restore)
      and ok($rts_open_world)
      and ok($rts_campaign)
      and ok($rts_campaign_entry)
      and (($boundary[0].green == true) or ($boundary[0].status == "green"))
      and ok($runner)
      and ok($launcher)
      and $manifest[0].cex_runtime_player_client_allowed == false
      and $budget[0].p95_budget_gate == true
      and $motion[0].accepted_input_gate == true
      and $input_budget[0].accepted_input_gate == true
      and $input_budget[0].response_p95_budget_gate == true
      and $input_budget[0].response_max_budget_gate == true
      and $selector[0].animation_transition_gate == true
      and $scene[0].dynamic_landmark_animation_gate == true
      and $probe[0].hud_probe_gate == true
      and $iso[0].projection_gate == true
      and $iso[0].depth_sort_gate == true
      and $iso[0].terrain_detail_gate == true
      and $iso[0].unit_detail_gate == true
      and $iso[0].command_feedback_gate == true
      and $iso[0].doodad_detail_gate == true
      and $iso[0].environment_detail_gate == true
      and $slots[0].manifest_frame_slots_gate == true
      and $slots[0].procedural_slots_gate == true
      and $slots[0].replacement_boundary_gate == true
      and $art_pack[0].required_model_gate == true
      and $art_pack[0].player_art_gate == true
      and $art_pack[0].enemy_art_gate == true
      and $art_pack[0].doodad_art_gate == true
      and $art_pack[0].terrain_art_gate == true
      and $art_pack[0].world_prop_art_gate == true
      and $art_pack[0].vfx_art_gate == true
      and $art_pack[0].model_detail_gate == true
      and $art_pack[0].unit_detail_gate == true
      and $art_pack[0].doodad_detail_gate == true
      and $art_pack[0].terrain_detail_gate == true
      and $art_pack[0].world_prop_detail_gate == true
      and $art_pack[0].vfx_detail_gate == true
      and $art_pack[0].replacement_boundary_gate == true
      and $art_scene[0].override_presence_gate == true
      and $art_scene[0].color_probe_gate == true
      and $art_scene[0].terrain_override_presence_gate == true
      and $art_scene[0].terrain_color_probe_gate == true
      and $art_scene[0].world_prop_override_presence_gate == true
      and $art_scene[0].world_prop_color_probe_gate == true
      and $art_scene[0].environment_override_presence_gate == true
      and $art_scene[0].environment_detail_color_probe_gate == true
      and $art_scene[0].vfx_override_presence_gate == true
      and $art_scene[0].vfx_color_probe_gate == true
      and $art_scene[0].replacement_boundary_gate == true
      and $override[0].override_frame_gate == true
      and $override[0].replacement_boundary_gate == true
      and $rts[0].selection_gate == true
      and $rts[0].command_queue_gate == true
      and $rts[0].strategy_hud_gate == true
      and $rts[0].macro_loop_gate == true
      and $rts[0].tactical_combat_gate == true
      and $rts[0].gameplay_surface_gate == true
      and $rts[0].move_selected_unit_count >= 4
      and $rts[0].attack_selected_unit_count >= 4
      and $rts_live[0].live_input_gate == true
      and $rts_live[0].selection_live_gate == true
      and $rts_live[0].production_live_gate == true
      and $rts_live[0].move_live_gate == true
      and $rts_live[0].attack_live_gate == true
      and $rts_live[0].ability_live_gate == true
      and $rts_live[0].accepted_input_count == 5
      and $rts_path[0].live_pathing_input_gate == true
      and $rts_path[0].path_tile_gate == true
      and $rts_path[0].blocked_tile_gate == true
      and $rts_path[0].formation_slot_gate == true
      and $rts_path[0].command_visual_gate == true
      and $rts_path[0].accepted_input_count == 2
      and $rts_collision[0].live_collision_input_gate == true
      and $rts_collision[0].collision_response_gate == true
      and $rts_collision[0].engagement_response_gate == true
      and $rts_collision[0].accepted_input_count == 3
      and $rts_target[0].live_targeting_input_gate == true
      and $rts_target[0].target_priority_gate == true
      and $rts_target[0].aggro_gate == true
      and $rts_target[0].focus_fire_gate == true
      and $rts_target[0].threat_feedback_gate == true
      and $rts_target[0].accepted_input_count == 4
      and $rts_economy[0].live_economy_input_gate == true
      and $rts_economy[0].harvest_loop_gate == true
      and $rts_economy[0].build_loop_gate == true
      and $rts_economy[0].production_loop_gate == true
      and $rts_economy[0].accepted_input_count == 4
      and $rts_select[0].live_selection_minimap_input_gate == true
      and $rts_select[0].selection_box_gate == true
      and $rts_select[0].control_group_gate == true
      and $rts_select[0].minimap_command_gate == true
      and $rts_select[0].split_route_gate == true
      and $rts_select[0].accepted_input_count == 4
      and $rts_build_lifecycle[0].live_build_lifecycle_input_gate == true
      and $rts_build_lifecycle[0].build_placement_gate == true
      and $rts_build_lifecycle[0].completion_gate == true
      and $rts_build_lifecycle[0].repair_gate == true
      and $rts_build_lifecycle[0].cancel_refund_gate == true
      and $rts_build_lifecycle[0].accepted_input_count == 6
      and $rts_tech_tree[0].live_tech_tree_input_gate == true
      and $rts_tech_tree[0].faction_base_gate == true
      and $rts_tech_tree[0].research_gate == true
      and $rts_tech_tree[0].upgrade_gate == true
      and $rts_tech_tree[0].unlock_gate == true
      and $rts_tech_tree[0].dependency_gate == true
      and $rts_tech_tree[0].accepted_input_count == 6
      and $rts_projectile[0].live_projectile_ability_input_gate == true
      and $rts_projectile[0].projectile_trail_gate == true
      and $rts_projectile[0].projectile_impact_gate == true
      and $rts_projectile[0].ability_radius_gate == true
      and $rts_projectile[0].damage_tick_gate == true
      and $rts_projectile[0].armor_shield_gate == true
      and $rts_projectile[0].accepted_input_count == 5
      and $rts_ai[0].live_ai_skirmish_input_gate == true
      and $rts_ai[0].ai_wave_gate == true
      and $rts_ai[0].ai_counter_gate == true
      and $rts_ai[0].ai_pressure_resolution_gate == true
      and $rts_ai[0].ai_retreat_gate == true
      and $rts_ai[0].player_response_gate == true
      and $rts_ai[0].accepted_input_count == 5
      and $rts_objective[0].live_objective_input_gate == true
      and $rts_objective[0].objective_marker_gate == true
      and $rts_objective[0].capture_progress_gate == true
      and $rts_objective[0].victory_resolution_gate == true
      and $rts_objective[0].defeat_pressure_gate == true
      and $rts_objective[0].extraction_gate == true
      and $rts_objective[0].accepted_input_count == 6
      and $rts_creep_camp[0].live_creep_camp_input_gate == true
      and $rts_creep_camp[0].terrain_route_gate == true
      and $rts_creep_camp[0].choke_gate == true
      and $rts_creep_camp[0].camp_clear_gate == true
      and $rts_creep_camp[0].scout_reveal_gate == true
      and $rts_creep_camp[0].expansion_route_gate == true
      and $rts_creep_camp[0].accepted_input_count == 6
      and $rts_fog[0].live_fog_scouting_input_gate == true
      and $rts_fog[0].scout_route_gate == true
      and $rts_fog[0].fog_reveal_gate == true
      and $rts_fog[0].enemy_structure_intel_gate == true
      and $rts_fog[0].enemy_unit_intel_gate == true
      and $rts_fog[0].intel_log_gate == true
      and $rts_fog[0].visibility_bar_gate == true
      and $rts_fog[0].accepted_input_count == 6
      and $rts_enemy_base[0].live_enemy_base_tech_pressure_input_gate == true
      and $rts_enemy_base[0].intel_dependency_gate == true
      and $rts_enemy_base[0].enemy_tech_gate == true
      and $rts_enemy_base[0].enemy_production_gate == true
      and $rts_enemy_base[0].player_counter_gate == true
      and $rts_enemy_base[0].defense_ready_gate == true
      and $rts_enemy_base[0].pressure_warning_gate == true
      and $rts_enemy_base[0].accepted_input_count == 6
      and $rts_army[0].live_army_production_input_gate == true
      and $rts_army[0].supply_gate == true
      and $rts_army[0].production_batch_gate == true
      and $rts_army[0].rally_gate == true
      and $rts_army[0].control_group_gate == true
      and $rts_army[0].composition_gate == true
      and $rts_army[0].accepted_input_count == 6
      and $rts_base_assault[0].live_base_assault_input_gate == true
      and $rts_base_assault[0].army_dependency_gate == true
      and $rts_base_assault[0].assault_path_gate == true
      and $rts_base_assault[0].enemy_base_health_gate == true
      and $rts_base_assault[0].breach_resolution_gate == true
      and $rts_base_assault[0].reward_gate == true
      and $rts_base_assault[0].accepted_input_count == 9
      and $rts_aftermath[0].live_aftermath_input_gate == true
      and $rts_aftermath[0].assault_dependency_gate == true
      and $rts_aftermath[0].destruction_gate == true
      and $rts_aftermath[0].veteran_gate == true
      and $rts_aftermath[0].match_result_gate == true
      and $rts_aftermath[0].next_action_gate == true
      and $rts_aftermath[0].reward_gate == true
      and $rts_aftermath[0].accepted_input_count == 12
      and $rts_commander[0].live_commander_input_gate == true
      and $rts_commander[0].aftermath_dependency_gate == true
      and $rts_commander[0].loot_gate == true
      and $rts_commander[0].commander_level_gate == true
      and $rts_commander[0].ability_point_gate == true
      and $rts_commander[0].aura_gate == true
      and $rts_commander[0].accepted_input_count == 15
      and $rts_expansion[0].live_expansion_input_gate == true
      and $rts_expansion[0].commander_dependency_gate == true
      and $rts_expansion[0].expansion_claim_gate == true
      and $rts_expansion[0].expansion_build_gate == true
      and $rts_expansion[0].expansion_worker_income_gate == true
      and $rts_expansion[0].counterattack_gate == true
      and $rts_expansion[0].defense_gate == true
      and $rts_expansion[0].accepted_input_count == 19
      and $rts_tier_two[0].live_tier_two_input_gate == true
      and $rts_tier_two[0].expansion_dependency_gate == true
      and $rts_tier_two[0].tier_two_tech_gate == true
      and $rts_tier_two[0].tier_two_upgrade_gate == true
      and $rts_tier_two[0].siege_unit_gate == true
      and $rts_tier_two[0].enemy_fortification_gate == true
      and $rts_tier_two[0].siege_push_gate == true
      and $rts_tier_two[0].accepted_input_count == 24
      and $rts_breach[0].live_siege_breach_input_gate == true
      and $rts_breach[0].tier_two_dependency_gate == true
      and $rts_breach[0].breach_window_gate == true
      and $rts_breach[0].repair_reaction_gate == true
      and $rts_breach[0].flank_pressure_gate == true
      and $rts_breach[0].hold_line_gate == true
      and $rts_breach[0].resolution_gate == true
      and $rts_breach[0].accepted_input_count == 29
      and $rts_inner[0].live_inner_lane_input_gate == true
      and $rts_inner[0].siege_breach_dependency_gate == true
      and $rts_inner[0].inner_route_gate == true
      and $rts_inner[0].inner_gate_gate == true
      and $rts_inner[0].supply_convoy_gate == true
      and $rts_inner[0].split_squad_gate == true
      and $rts_inner[0].second_line_clear_gate == true
      and $rts_inner[0].signal_core_secure_gate == true
      and $rts_inner[0].accepted_input_count == 35
      and $runner[0].gates.override_dir_gate == true
      and $runner[0].gates.cex_path_gate == true
    ),
    checks: {
      manifest_lint_green: ok($manifest),
      animation_preview_green: ok($animation),
      animation_selector_green: ok($selector),
      player_motion_green: ok($motion),
      input_frame_budget_green: ok($input_budget),
      render_budget_green: ok($budget),
      scene_preview_green: ok($scene),
      renderer_probe_green: ok($probe),
      isometric_modeling_green: ok($iso),
      model_catalog_green: ok($catalog),
      asset_slot_map_green: ok($slots),
      classic_art_pack_green: ok($art_pack),
      classic_art_pack_scene_probe_green: ok($art_scene),
      asset_override_probe_green: ok($override),
      classic_rts_control_loop_green: ok($rts),
      classic_rts_live_input_sequence_green: ok($rts_live),
      classic_rts_pathing_formation_green: ok($rts_path),
      classic_rts_collision_engagement_green: ok($rts_collision),
      classic_rts_target_aggro_focus_green: ok($rts_target),
      classic_rts_economy_build_green: ok($rts_economy),
      classic_rts_selection_minimap_green: ok($rts_select),
      classic_rts_build_lifecycle_green: ok($rts_build_lifecycle),
      classic_rts_tech_tree_green: ok($rts_tech_tree),
      classic_rts_projectile_ability_green: ok($rts_projectile),
      classic_rts_ai_skirmish_pressure_green: ok($rts_ai),
      classic_rts_objective_victory_loop_green: ok($rts_objective),
      classic_rts_creep_camp_terrain_route_green: ok($rts_creep_camp),
      classic_rts_fog_scouting_intel_green: ok($rts_fog),
      classic_rts_enemy_base_tech_pressure_green: ok($rts_enemy_base),
      classic_rts_army_production_rally_green: ok($rts_army),
      classic_rts_base_assault_resolution_green: ok($rts_base_assault),
      classic_rts_battle_aftermath_green: ok($rts_aftermath),
      classic_rts_commander_progression_green: ok($rts_commander),
      classic_rts_expansion_counterattack_green: ok($rts_expansion),
      classic_rts_tier_two_siege_push_green: ok($rts_tier_two),
      classic_rts_siege_breach_counterplay_green: ok($rts_breach),
      classic_rts_inner_lane_breakthrough_green: ok($rts_inner),
      classic_rts_central_keep_pressure_green: ok($rts_keep),
      classic_rts_central_keep_breakthrough_green: ok($rts_keep_break),
      classic_rts_mirror_city_restoration_green: ok($rts_restore),
      classic_rts_open_world_after_action_green: ok($rts_open_world),
      classic_rts_campaign_handoff_green: ok($rts_campaign),
      classic_rts_campaign_entry_green: ok($rts_campaign_entry),
      client_boundary_green: (($boundary[0].green == true) or ($boundary[0].status == "green")),
      playtest_runner_status_green: ok($runner),
      playtest_launcher_green: ok($launcher)
    },
    headline: {
      frame_count: $manifest[0].frame_count,
      animation_clip_count: $animation[0].clip_count,
      motion_sample_count: $motion[0].sample_count,
      motion_accepted_input_count: $motion[0].accepted_input_count,
      input_frame_sample_count: $input_budget[0].sample_count,
      input_frame_accepted_input_count: $input_budget[0].accepted_input_count,
      input_frame_p95_micros: $input_budget[0].p95_micros,
      input_frame_max_micros: $input_budget[0].max_micros,
      render_p50_micros: $budget[0].p50_micros,
      render_p95_micros: $budget[0].p95_micros,
      render_max_micros: $budget[0].max_micros,
      scene_unique_color_count: $scene[0].unique_color_count,
      renderer_probe_hud_text_pixels: $probe[0].hud_text_pixels,
      isometric_unique_color_count: $iso[0].unique_color_count,
      isometric_non_background_pixels: $iso[0].non_background_pixels,
      isometric_shadow_pixel_count: $iso[0].shadow_pixel_count,
      isometric_procedural_model_pixel_count: $iso[0].procedural_model_pixel_count,
      isometric_canopy_pixel_count: $iso[0].canopy_pixel_count,
      isometric_rts_building_pixel_count: $iso[0].rts_building_pixel_count,
      isometric_rts_model_entity_count: $iso[0].rts_model_entity_count,
      isometric_terrain_detail_pixel_count: $iso[0].terrain_detail_pixel_count,
      isometric_terrain_road_pixel_count: $iso[0].terrain_road_pixel_count,
      isometric_terrain_water_pixel_count: $iso[0].terrain_water_pixel_count,
      isometric_terrain_cliff_pixel_count: $iso[0].terrain_cliff_pixel_count,
      isometric_terrain_foundation_pixel_count: $iso[0].terrain_foundation_pixel_count,
      isometric_unit_detail_pixel_count: $iso[0].unit_detail_pixel_count,
      isometric_unit_ring_pixel_count: $iso[0].unit_ring_pixel_count,
      isometric_unit_health_pixel_count: $iso[0].unit_health_pixel_count,
      isometric_unit_silhouette_pixel_count: $iso[0].unit_silhouette_pixel_count,
      isometric_rts_neutral_unit_entity_count: $iso[0].rts_neutral_unit_entity_count,
      isometric_neutral_unit_detail_pixel_count: $iso[0].neutral_unit_detail_pixel_count,
      isometric_neutral_guard_pixel_count: $iso[0].neutral_guard_pixel_count,
      isometric_neutral_worker_pixel_count: $iso[0].neutral_worker_pixel_count,
      isometric_neutral_creep_pixel_count: $iso[0].neutral_creep_pixel_count,
      isometric_command_feedback_pixel_count: $iso[0].command_feedback_pixel_count,
      isometric_command_marker_pixel_count: $iso[0].command_marker_pixel_count,
      isometric_attack_arc_pixel_count: $iso[0].attack_arc_pixel_count,
      isometric_hit_flash_pixel_count: $iso[0].hit_flash_pixel_count,
      isometric_rts_doodad_entity_count: $iso[0].rts_doodad_entity_count,
      isometric_doodad_detail_pixel_count: $iso[0].doodad_detail_pixel_count,
      isometric_doodad_stone_pixel_count: $iso[0].doodad_stone_pixel_count,
      isometric_doodad_wood_pixel_count: $iso[0].doodad_wood_pixel_count,
      isometric_doodad_fire_pixel_count: $iso[0].doodad_fire_pixel_count,
      isometric_doodad_crystal_pixel_count: $iso[0].doodad_crystal_pixel_count,
      isometric_rts_environment_entity_count: $iso[0].rts_environment_entity_count,
      isometric_environment_detail_pixel_count: $iso[0].environment_detail_pixel_count,
      isometric_environment_foliage_pixel_count: $iso[0].environment_foliage_pixel_count,
      isometric_environment_ruin_pixel_count: $iso[0].environment_ruin_pixel_count,
      isometric_environment_gold_pixel_count: $iso[0].environment_gold_pixel_count,
      isometric_environment_bridge_pixel_count: $iso[0].environment_bridge_pixel_count,
      model_catalog_rendered_frame_count: $catalog[0].rendered_frame_count,
      asset_slot_count: $slots[0].slot_count,
      asset_slot_category_count: $slots[0].category_count,
      asset_manifest_frame_slot_count: $slots[0].manifest_frame_slot_count,
      asset_procedural_model_slot_count: $slots[0].procedural_model_slot_count,
      asset_doodad_slot_count: $slots[0].doodad_slot_count,
      asset_terrain_detail_slot_count: $slots[0].terrain_detail_slot_count,
      asset_vfx_slot_count: $slots[0].vfx_slot_count,
      asset_neutral_unit_slot_count: $slots[0].neutral_unit_slot_count,
      art_pack_asset_count: $art_pack[0].asset_count,
      art_pack_override_frame_count: $art_pack[0].override_frame_count,
      art_pack_preview_height: $art_pack[0].preview_height,
      art_pack_preview_non_background_pixels: $art_pack[0].preview_non_background_pixels,
      art_pack_model_detail_asset_count: $art_pack[0].model_detail_asset_count,
      art_pack_model_unique_color_total: $art_pack[0].model_unique_color_total,
      art_pack_model_shadow_pixel_count: $art_pack[0].model_shadow_pixel_count,
      art_pack_model_highlight_pixel_count: $art_pack[0].model_highlight_pixel_count,
      art_pack_player_unit_detail_asset_count: $art_pack[0].player_unit_detail_asset_count,
      art_pack_enemy_unit_detail_asset_count: $art_pack[0].enemy_unit_detail_asset_count,
      art_pack_unit_unique_color_total: $art_pack[0].unit_unique_color_total,
      art_pack_unit_shadow_pixel_count: $art_pack[0].unit_shadow_pixel_count,
      art_pack_unit_highlight_pixel_count: $art_pack[0].unit_highlight_pixel_count,
      art_pack_neutral_unit_detail_asset_count: $art_pack[0].neutral_unit_detail_asset_count,
      art_pack_neutral_unit_unique_color_total: $art_pack[0].neutral_unit_unique_color_total,
      art_pack_neutral_unit_shadow_pixel_count: $art_pack[0].neutral_unit_shadow_pixel_count,
      art_pack_neutral_unit_highlight_pixel_count: $art_pack[0].neutral_unit_highlight_pixel_count,
      art_pack_neutral_unit_detail_pixel_count: $art_pack[0].neutral_unit_detail_pixel_count,
      art_pack_doodad_detail_asset_count: $art_pack[0].doodad_detail_asset_count,
      art_pack_doodad_unique_color_total: $art_pack[0].doodad_unique_color_total,
      art_pack_doodad_shadow_pixel_count: $art_pack[0].doodad_shadow_pixel_count,
      art_pack_doodad_detail_pixel_count: $art_pack[0].doodad_detail_pixel_count,
      art_pack_terrain_detail_asset_count: $art_pack[0].terrain_detail_asset_count,
      art_pack_terrain_unique_color_total: $art_pack[0].terrain_unique_color_total,
      art_pack_terrain_detail_pixel_count: $art_pack[0].terrain_detail_pixel_count,
      art_pack_world_prop_detail_asset_count: $art_pack[0].world_prop_detail_asset_count,
      art_pack_world_prop_unique_color_total: $art_pack[0].world_prop_unique_color_total,
      art_pack_world_prop_detail_pixel_count: $art_pack[0].world_prop_detail_pixel_count,
      art_pack_vfx_detail_asset_count: $art_pack[0].vfx_detail_asset_count,
      art_pack_vfx_unique_color_total: $art_pack[0].vfx_unique_color_total,
      art_pack_vfx_detail_pixel_count: $art_pack[0].vfx_detail_pixel_count,
      art_pack_scene_non_background_pixels: $art_scene[0].non_background_pixels,
      art_pack_scene_player_color_count: $art_scene[0].player_color_count,
      art_pack_scene_enemy_attack_color_count: $art_scene[0].enemy_attack_color_count,
      art_pack_scene_terrain_grass_color_count: $art_scene[0].terrain_grass_color_count,
      art_pack_scene_terrain_road_color_count: $art_scene[0].terrain_road_color_count,
      art_pack_scene_terrain_water_color_count: $art_scene[0].terrain_water_color_count,
      art_pack_scene_terrain_wall_roof_color_count: $art_scene[0].terrain_wall_roof_color_count,
      art_pack_scene_world_prop_runtime_color_count: $art_scene[0].world_prop_runtime_color_count,
      art_pack_scene_neutral_unit_runtime_color_count: $art_scene[0].neutral_unit_runtime_color_count,
      art_pack_scene_environment_detail_color_count: $art_scene[0].environment_detail_color_count,
      art_pack_scene_command_marker_color_count: $art_scene[0].command_marker_color_count,
      art_pack_scene_attack_arc_color_count: $art_scene[0].attack_arc_color_count,
      art_pack_scene_hit_flash_color_count: $art_scene[0].hit_flash_color_count,
      asset_override_frame_count: $override[0].override_frame_count,
      asset_override_probe_pixel_count: $override[0].override_probe_pixel_count,
      asset_override_non_background_pixels: $override[0].non_background_pixels,
      rts_control_loop_non_background_pixels: $rts[0].non_background_pixels,
      rts_control_loop_move_selected_unit_count: $rts[0].move_selected_unit_count,
      rts_control_loop_attack_selected_unit_count: $rts[0].attack_selected_unit_count,
      rts_control_loop_selection_marker_pixel_count: $rts[0].selection_marker_pixel_count,
      rts_control_loop_formation_line_pixel_count: $rts[0].formation_line_pixel_count,
      rts_control_loop_command_marker_pixel_count: $rts[0].command_marker_pixel_count,
      rts_control_loop_attack_feedback_pixel_count: $rts[0].attack_feedback_pixel_count,
      rts_control_loop_strategy_panel_pixel_count: $rts[0].strategy_panel_pixel_count,
      rts_control_loop_minimap_pixel_count: $rts[0].minimap_pixel_count,
      rts_control_loop_fog_pixel_count: $rts[0].fog_pixel_count,
      rts_control_loop_vision_pixel_count: $rts[0].vision_pixel_count,
      rts_control_loop_resource_hud_pixel_count: $rts[0].resource_hud_pixel_count,
      rts_control_loop_production_queue_pixel_count: $rts[0].production_queue_pixel_count,
      rts_control_loop_move_training_progress_percent: $rts[0].move_training_progress_percent,
      rts_control_loop_attack_build_progress_percent: $rts[0].attack_build_progress_percent,
      rts_control_loop_unit_health_card_pixel_count: $rts[0].unit_health_card_pixel_count,
      rts_control_loop_ability_command_pixel_count: $rts[0].ability_command_pixel_count,
      rts_control_loop_target_health_pixel_count: $rts[0].target_health_pixel_count,
      rts_control_loop_attack_target_health_percent: $rts[0].attack_target_health_percent,
      rts_live_input_accepted_input_count: $rts_live[0].accepted_input_count,
      rts_live_input_selection_marker_pixel_count: $rts_live[0].selection_marker_pixel_count,
      rts_live_input_command_marker_pixel_count: $rts_live[0].command_marker_pixel_count,
      rts_live_input_attack_feedback_pixel_count: $rts_live[0].attack_feedback_pixel_count,
      rts_live_input_production_queue_pixel_count: $rts_live[0].production_queue_pixel_count,
      rts_live_input_ability_command_pixel_count: $rts_live[0].ability_command_pixel_count,
      rts_live_input_target_health_pixel_count: $rts_live[0].target_health_pixel_count,
      rts_live_input_target_health_percent: $rts_live[0].final_target_health_percent,
      rts_pathing_accepted_input_count: $rts_path[0].accepted_input_count,
      rts_pathing_path_tile_count: ($rts_path[0].path_tile_ids | length),
      rts_pathing_blocked_tile_count: ($rts_path[0].blocked_tile_ids | length),
      rts_pathing_formation_slot_count: ($rts_path[0].formation_slot_tile_ids | length),
      rts_pathing_path_tile_pixel_count: $rts_path[0].path_tile_pixel_count,
      rts_pathing_blocked_tile_pixel_count: $rts_path[0].blocked_tile_pixel_count,
      rts_pathing_formation_slot_pixel_count: $rts_path[0].formation_slot_pixel_count,
      rts_pathing_selection_marker_pixel_count: $rts_path[0].selection_marker_pixel_count,
      rts_pathing_command_marker_pixel_count: $rts_path[0].command_marker_pixel_count,
      rts_collision_accepted_input_count: $rts_collision[0].accepted_input_count,
      rts_collision_move_disperse_tile_count: ($rts_collision[0].move_disperse_tile_ids | length),
      rts_collision_engagement_tile_count: ($rts_collision[0].engagement_tile_ids | length),
      rts_collision_contact_flash_tile_count: ($rts_collision[0].contact_flash_tile_ids | length),
      rts_collision_dispersion_slot_pixel_count: $rts_collision[0].dispersion_slot_pixel_count,
      rts_collision_engagement_range_pixel_count: $rts_collision[0].engagement_range_pixel_count,
      rts_collision_contact_flash_pixel_count: $rts_collision[0].contact_flash_pixel_count,
      rts_collision_blocked_tile_pixel_count: $rts_collision[0].blocked_tile_pixel_count,
      rts_collision_attack_feedback_pixel_count: $rts_collision[0].attack_feedback_pixel_count,
      rts_targeting_accepted_input_count: $rts_target[0].accepted_input_count,
      rts_targeting_priority_count: ($rts_target[0].final_target_priority_ids | length),
      rts_targeting_focus_fire_unit_count: ($rts_target[0].final_focus_fire_unit_ids | length),
      rts_targeting_threat_level_count: ($rts_target[0].final_threat_level_percents | length),
      rts_targeting_target_priority_pixel_count: $rts_target[0].target_priority_pixel_count,
      rts_targeting_aggro_pixel_count: $rts_target[0].aggro_pixel_count,
      rts_targeting_focus_fire_pixel_count: $rts_target[0].focus_fire_pixel_count,
      rts_targeting_threat_bar_pixel_count: $rts_target[0].threat_bar_pixel_count,
      rts_targeting_attack_feedback_pixel_count: $rts_target[0].attack_feedback_pixel_count,
      rts_economy_accepted_input_count: $rts_economy[0].accepted_input_count,
      rts_economy_harvest_node_count: ($rts_economy[0].final_harvest_node_ids | length),
      rts_economy_worker_assignment_count: ($rts_economy[0].final_worker_assignment_ids | length),
      rts_economy_build_site_tile_count: ($rts_economy[0].final_build_site_tile_ids | length),
      rts_economy_building_progress_percent: $rts_economy[0].final_building_progress_percent,
      rts_economy_harvest_node_pixel_count: $rts_economy[0].harvest_node_pixel_count,
      rts_economy_worker_route_pixel_count: $rts_economy[0].worker_route_pixel_count,
      rts_economy_dropoff_pixel_count: $rts_economy[0].dropoff_pixel_count,
      rts_economy_build_blueprint_pixel_count: $rts_economy[0].build_blueprint_pixel_count,
      rts_economy_build_progress_pixel_count: $rts_economy[0].build_progress_pixel_count,
      rts_economy_production_queue_pixel_count: $rts_economy[0].production_queue_pixel_count,
      rts_selection_minimap_accepted_input_count: $rts_select[0].accepted_input_count,
      rts_selection_box_tile_count: ($rts_select[0].final_selection_box_tile_ids | length),
      rts_control_group_assignment_count: ($rts_select[0].final_control_group_assignments | length),
      rts_active_control_group_count: ($rts_select[0].final_active_control_group_ids | length),
      rts_minimap_command_tile_id: $rts_select[0].final_minimap_command_tile_id,
      rts_split_route_tile_count: ($rts_select[0].final_group_route_tile_ids | length),
      rts_selection_minimap_pixel_count: (
        $rts_select[0].selection_box_pixel_count
        + $rts_select[0].minimap_command_pixel_count
        + $rts_select[0].group_two_pixel_count
        + $rts_select[0].split_route_pixel_count
      ),
      rts_selection_box_pixel_count: $rts_select[0].selection_box_pixel_count,
      rts_minimap_command_pixel_count: $rts_select[0].minimap_command_pixel_count,
      rts_group_two_pixel_count: $rts_select[0].group_two_pixel_count,
      rts_split_route_pixel_count: $rts_select[0].split_route_pixel_count,
      rts_build_lifecycle_accepted_input_count: $rts_build_lifecycle[0].accepted_input_count,
      rts_build_lifecycle_completed_structure_count: ($rts_build_lifecycle[0].final_completed_structure_ids | length),
      rts_build_lifecycle_cancelled_structure_count: ($rts_build_lifecycle[0].final_cancelled_structure_ids | length),
      rts_build_lifecycle_repair_progress_percent: $rts_build_lifecycle[0].final_repair_progress_percent,
      rts_build_lifecycle_refund_count: ($rts_build_lifecycle[0].final_refund_delta_log | length),
      rts_build_lifecycle_pixel_count: (
        $rts_build_lifecycle[0].structure_complete_pixel_count
        + $rts_build_lifecycle[0].structure_health_pixel_count
        + $rts_build_lifecycle[0].repair_pixel_count
        + $rts_build_lifecycle[0].cancel_refund_pixel_count
      ),
      rts_build_lifecycle_structure_complete_pixel_count: $rts_build_lifecycle[0].structure_complete_pixel_count,
      rts_build_lifecycle_structure_health_pixel_count: $rts_build_lifecycle[0].structure_health_pixel_count,
      rts_build_lifecycle_repair_pixel_count: $rts_build_lifecycle[0].repair_pixel_count,
      rts_build_lifecycle_cancel_refund_pixel_count: $rts_build_lifecycle[0].cancel_refund_pixel_count,
      rts_tech_tree_accepted_input_count: $rts_tech_tree[0].accepted_input_count,
      rts_tech_tree_faction_id: $rts_tech_tree[0].final_faction_id,
      rts_tech_tree_base_structure_count: ($rts_tech_tree[0].final_base_structure_ids | length),
      rts_tech_tree_research_count: ($rts_tech_tree[0].final_tech_research_ids | length),
      rts_tech_tree_completed_upgrade_count: ($rts_tech_tree[0].final_completed_upgrade_ids | length),
      rts_tech_tree_unlocked_unit_count: ($rts_tech_tree[0].final_unlocked_unit_ids | length),
      rts_tech_tree_unlocked_structure_count: ($rts_tech_tree[0].final_unlocked_structure_ids | length),
      rts_tech_tree_requirement_count: ($rts_tech_tree[0].final_tech_requirements_log | length),
      rts_tech_tree_progress_percent: $rts_tech_tree[0].final_tech_progress_percent,
      rts_tech_tree_pixel_count: (
        $rts_tech_tree[0].tech_base_pixel_count
        + $rts_tech_tree[0].tech_research_pixel_count
        + $rts_tech_tree[0].tech_upgrade_pixel_count
        + $rts_tech_tree[0].tech_unlock_pixel_count
        + $rts_tech_tree[0].tech_requirement_pixel_count
      ),
      rts_tech_tree_base_pixel_count: $rts_tech_tree[0].tech_base_pixel_count,
      rts_tech_tree_research_pixel_count: $rts_tech_tree[0].tech_research_pixel_count,
      rts_tech_tree_upgrade_pixel_count: $rts_tech_tree[0].tech_upgrade_pixel_count,
      rts_tech_tree_unlock_pixel_count: $rts_tech_tree[0].tech_unlock_pixel_count,
      rts_tech_tree_requirement_pixel_count: $rts_tech_tree[0].tech_requirement_pixel_count,
      rts_projectile_ability_accepted_input_count: $rts_projectile[0].accepted_input_count,
      rts_projectile_ability_active_projectile_id: $rts_projectile[0].final_active_projectile_id,
      rts_projectile_ability_trail_tile_count: ($rts_projectile[0].final_projectile_trail_tile_ids | length),
      rts_projectile_ability_effect_tile_count: ($rts_projectile[0].final_ability_effect_tile_ids | length),
      rts_projectile_ability_damage_tick_count: ($rts_projectile[0].final_ability_damage_ticks | length),
      rts_projectile_ability_damage_total: ($rts_projectile[0].final_ability_damage_ticks | add),
      rts_projectile_ability_target_health_percent: $rts_projectile[0].final_target_health_percent,
      rts_projectile_ability_target_armor_percent: $rts_projectile[0].final_target_armor_percent,
      rts_projectile_ability_target_shield_percent: $rts_projectile[0].final_target_shield_percent,
      rts_projectile_ability_pixel_count: (
        $rts_projectile[0].projectile_trail_pixel_count
        + $rts_projectile[0].projectile_impact_pixel_count
        + $rts_projectile[0].ability_radius_pixel_count
        + $rts_projectile[0].damage_tick_pixel_count
        + $rts_projectile[0].armor_shield_pixel_count
      ),
      rts_projectile_ability_trail_pixel_count: $rts_projectile[0].projectile_trail_pixel_count,
      rts_projectile_ability_impact_pixel_count: $rts_projectile[0].projectile_impact_pixel_count,
      rts_projectile_ability_radius_pixel_count: $rts_projectile[0].ability_radius_pixel_count,
      rts_projectile_ability_damage_tick_pixel_count: $rts_projectile[0].damage_tick_pixel_count,
      rts_projectile_ability_armor_shield_pixel_count: $rts_projectile[0].armor_shield_pixel_count,
      rts_ai_skirmish_accepted_input_count: $rts_ai[0].accepted_input_count,
      rts_ai_skirmish_wave_unit_count: ($rts_ai[0].final_ai_wave_unit_ids | length),
      rts_ai_skirmish_pressure_tile_count: ($rts_ai[0].final_ai_pressure_tile_ids | length),
      rts_ai_skirmish_counter_tile_count: ($rts_ai[0].final_ai_counter_tile_ids | length),
      rts_ai_skirmish_retreat_tile_id: $rts_ai[0].final_ai_retreat_tile_id,
      rts_ai_skirmish_pressure_percent: $rts_ai[0].final_ai_pressure_percent,
      rts_ai_skirmish_state: $rts_ai[0].final_ai_skirmish_state,
      rts_ai_skirmish_pressure_pixel_count: (
        $rts_ai[0].ai_wave_pixel_count
        + $rts_ai[0].ai_pressure_pixel_count
        + $rts_ai[0].ai_counter_pixel_count
        + $rts_ai[0].ai_retreat_pixel_count
        + $rts_ai[0].ai_pressure_bar_pixel_count
      ),
      rts_ai_skirmish_wave_pixel_count: $rts_ai[0].ai_wave_pixel_count,
      rts_ai_skirmish_lane_pixel_count: $rts_ai[0].ai_pressure_pixel_count,
      rts_ai_skirmish_counter_pixel_count: $rts_ai[0].ai_counter_pixel_count,
      rts_ai_skirmish_retreat_pixel_count: $rts_ai[0].ai_retreat_pixel_count,
      rts_ai_skirmish_pressure_bar_pixel_count: $rts_ai[0].ai_pressure_bar_pixel_count,
      rts_objective_victory_loop_accepted_input_count: $rts_objective[0].accepted_input_count,
      rts_objective_victory_loop_tile_count: ($rts_objective[0].final_objective_tile_ids | length),
      rts_objective_victory_loop_capture_percent: $rts_objective[0].final_objective_capture_percent,
      rts_objective_victory_loop_owner_state: $rts_objective[0].final_objective_owner_state,
      rts_objective_victory_loop_result_state: $rts_objective[0].final_objective_result_state,
      rts_objective_victory_loop_extraction_tile_id: $rts_objective[0].final_objective_extraction_tile_id,
      rts_objective_victory_loop_defeat_risk_percent: $rts_objective[0].final_defeat_risk_percent,
      rts_objective_victory_loop_ai_pressure_percent: $rts_objective[0].final_ai_pressure_percent,
      rts_objective_victory_loop_pixel_count: (
        $rts_objective[0].objective_pixel_count
        + $rts_objective[0].capture_bar_pixel_count
        + $rts_objective[0].victory_pixel_count
        + $rts_objective[0].defeat_risk_pixel_count
        + $rts_objective[0].extraction_pixel_count
      ),
      rts_objective_victory_loop_objective_pixel_count: $rts_objective[0].objective_pixel_count,
      rts_objective_victory_loop_capture_bar_pixel_count: $rts_objective[0].capture_bar_pixel_count,
      rts_objective_victory_loop_victory_pixel_count: $rts_objective[0].victory_pixel_count,
      rts_objective_victory_loop_defeat_risk_pixel_count: $rts_objective[0].defeat_risk_pixel_count,
      rts_objective_victory_loop_extraction_pixel_count: $rts_objective[0].extraction_pixel_count,
      rts_creep_camp_terrain_route_accepted_input_count: $rts_creep_camp[0].accepted_input_count,
      rts_creep_camp_terrain_route_camp_tile_count: ($rts_creep_camp[0].final_creep_camp_tile_ids | length),
      rts_creep_camp_terrain_route_unit_count: ($rts_creep_camp[0].final_creep_camp_unit_ids | length),
      rts_creep_camp_terrain_route_state: $rts_creep_camp[0].final_creep_camp_state,
      rts_creep_camp_terrain_route_route_tile_count: ($rts_creep_camp[0].final_terrain_route_tile_ids | length),
      rts_creep_camp_terrain_route_choke_tile_count: ($rts_creep_camp[0].final_terrain_choke_tile_ids | length),
      rts_creep_camp_terrain_route_expansion_tile_count: ($rts_creep_camp[0].final_expansion_tile_ids | length),
      rts_creep_camp_terrain_route_scout_reveal_percent: $rts_creep_camp[0].final_scout_reveal_percent,
      rts_creep_camp_terrain_route_target_health_percent: $rts_creep_camp[0].final_target_health_percent,
      rts_creep_camp_terrain_route_pixel_count: (
        $rts_creep_camp[0].camp_pixel_count
        + $rts_creep_camp[0].terrain_route_pixel_count
        + $rts_creep_camp[0].choke_pixel_count
        + $rts_creep_camp[0].expansion_pixel_count
        + $rts_creep_camp[0].scout_reveal_pixel_count
      ),
      rts_creep_camp_terrain_route_camp_pixel_count: $rts_creep_camp[0].camp_pixel_count,
      rts_creep_camp_terrain_route_route_pixel_count: $rts_creep_camp[0].terrain_route_pixel_count,
      rts_creep_camp_terrain_route_choke_pixel_count: $rts_creep_camp[0].choke_pixel_count,
      rts_creep_camp_terrain_route_expansion_pixel_count: $rts_creep_camp[0].expansion_pixel_count,
      rts_creep_camp_terrain_route_reveal_pixel_count: $rts_creep_camp[0].scout_reveal_pixel_count,
      rts_fog_scouting_intel_accepted_input_count: $rts_fog[0].accepted_input_count,
      rts_fog_scouting_intel_scout_unit_count: ($rts_fog[0].final_scout_unit_ids | length),
      rts_fog_scouting_intel_scout_route_tile_count: ($rts_fog[0].final_scout_route_tile_ids | length),
      rts_fog_scouting_intel_fog_reveal_tile_count: ($rts_fog[0].final_fog_reveal_tile_ids | length),
      rts_fog_scouting_intel_enemy_structure_count: ($rts_fog[0].final_revealed_enemy_structure_ids | length),
      rts_fog_scouting_intel_enemy_unit_count: ($rts_fog[0].final_revealed_enemy_unit_ids | length),
      rts_fog_scouting_intel_visibility_percent: $rts_fog[0].final_visibility_percent,
      rts_fog_scouting_intel_pixel_count: (
        $rts_fog[0].scout_route_pixel_count
        + $rts_fog[0].fog_reveal_pixel_count
        + $rts_fog[0].enemy_structure_pixel_count
        + $rts_fog[0].enemy_intel_pixel_count
        + $rts_fog[0].visibility_bar_pixel_count
      ),
      rts_fog_scouting_intel_scout_route_pixel_count: $rts_fog[0].scout_route_pixel_count,
      rts_fog_scouting_intel_fog_reveal_pixel_count: $rts_fog[0].fog_reveal_pixel_count,
      rts_fog_scouting_intel_enemy_structure_pixel_count: $rts_fog[0].enemy_structure_pixel_count,
      rts_fog_scouting_intel_enemy_unit_pixel_count: $rts_fog[0].enemy_intel_pixel_count,
      rts_fog_scouting_intel_visibility_bar_pixel_count: $rts_fog[0].visibility_bar_pixel_count,
      rts_enemy_base_tech_pressure_accepted_input_count: $rts_enemy_base[0].accepted_input_count,
      rts_enemy_base_tech_pressure_enemy_tech_count: ($rts_enemy_base[0].final_enemy_base_tech_ids | length),
      rts_enemy_base_tech_pressure_enemy_production_count: ($rts_enemy_base[0].final_enemy_production_queue | length),
      rts_enemy_base_tech_pressure_wave_unit_count: ($rts_enemy_base[0].final_enemy_pressure_wave_unit_ids | length),
      rts_enemy_base_tech_pressure_player_counter_count: ($rts_enemy_base[0].final_player_counter_tech_ids | length),
      rts_enemy_base_tech_pressure_defense_structure_count: ($rts_enemy_base[0].final_player_defense_structure_ids | length),
      rts_enemy_base_tech_pressure_warning_percent: $rts_enemy_base[0].final_enemy_pressure_warning_percent,
      rts_enemy_base_tech_pressure_state: $rts_enemy_base[0].final_enemy_base_pressure_state,
      rts_enemy_base_tech_pressure_pixel_count: (
        $rts_enemy_base[0].enemy_tech_pixel_count
        + $rts_enemy_base[0].enemy_production_pixel_count
        + $rts_enemy_base[0].player_counter_tech_pixel_count
        + $rts_enemy_base[0].defense_ready_pixel_count
        + $rts_enemy_base[0].pressure_warning_pixel_count
      ),
      rts_enemy_base_tech_pressure_enemy_tech_pixel_count: $rts_enemy_base[0].enemy_tech_pixel_count,
      rts_enemy_base_tech_pressure_enemy_production_pixel_count: $rts_enemy_base[0].enemy_production_pixel_count,
      rts_enemy_base_tech_pressure_player_counter_pixel_count: $rts_enemy_base[0].player_counter_tech_pixel_count,
      rts_enemy_base_tech_pressure_defense_ready_pixel_count: $rts_enemy_base[0].defense_ready_pixel_count,
      rts_enemy_base_tech_pressure_warning_pixel_count: $rts_enemy_base[0].pressure_warning_pixel_count,
      rts_army_production_rally_accepted_input_count: $rts_army[0].accepted_input_count,
      rts_army_production_rally_supply_used: $rts_army[0].final_army_supply_used,
      rts_army_production_rally_supply_cap: $rts_army[0].final_army_supply_cap,
      rts_army_production_rally_batch_count: ($rts_army[0].final_army_production_batch_ids | length),
      rts_army_production_rally_spawned_unit_count: ($rts_army[0].final_army_spawned_unit_ids | length),
      rts_army_production_rally_rally_tile_count: ($rts_army[0].final_army_rally_tile_ids | length),
      rts_army_production_rally_composition_log_count: ($rts_army[0].final_army_composition_log | length),
      rts_army_production_rally_state: $rts_army[0].final_army_production_state,
      rts_army_production_rally_training_progress_percent: $rts_army[0].final_training_progress_percent,
      rts_army_production_rally_pixel_count: (
        $rts_army[0].supply_pixel_count
        + $rts_army[0].spawned_unit_pixel_count
        + $rts_army[0].rally_line_pixel_count
        + $rts_army[0].composition_pixel_count
      ),
      rts_army_production_rally_supply_pixel_count: $rts_army[0].supply_pixel_count,
      rts_army_production_rally_spawned_unit_pixel_count: $rts_army[0].spawned_unit_pixel_count,
      rts_army_production_rally_rally_line_pixel_count: $rts_army[0].rally_line_pixel_count,
      rts_army_production_rally_composition_pixel_count: $rts_army[0].composition_pixel_count,
      rts_base_assault_resolution_accepted_input_count: $rts_base_assault[0].accepted_input_count,
      rts_base_assault_resolution_army_spawned_unit_count: ($rts_base_assault[0].final_army_spawned_unit_ids | length),
      rts_base_assault_resolution_target_count: ($rts_base_assault[0].final_base_assault_target_ids | length),
      rts_base_assault_resolution_path_tile_count: ($rts_base_assault[0].final_base_assault_path_tile_ids | length),
      rts_base_assault_resolution_min_enemy_structure_health: ($rts_base_assault[0].final_enemy_structure_health_percents | min),
      rts_base_assault_resolution_breach_percent: $rts_base_assault[0].final_base_breach_percent,
      rts_base_assault_resolution_state: $rts_base_assault[0].final_base_assault_result_state,
      rts_base_assault_resolution_reward_count: ($rts_base_assault[0].final_base_assault_reward_log | length),
      rts_base_assault_resolution_pixel_count: (
        $rts_base_assault[0].assault_path_pixel_count
        + $rts_base_assault[0].breach_pixel_count
        + $rts_base_assault[0].enemy_base_health_pixel_count
        + $rts_base_assault[0].assault_reward_pixel_count
      ),
      rts_base_assault_resolution_assault_path_pixel_count: $rts_base_assault[0].assault_path_pixel_count,
      rts_base_assault_resolution_breach_pixel_count: $rts_base_assault[0].breach_pixel_count,
      rts_base_assault_resolution_enemy_base_health_pixel_count: $rts_base_assault[0].enemy_base_health_pixel_count,
      rts_base_assault_resolution_assault_reward_pixel_count: $rts_base_assault[0].assault_reward_pixel_count,
      rts_battle_aftermath_accepted_input_count: $rts_aftermath[0].accepted_input_count,
      rts_battle_aftermath_destroyed_structure_count: ($rts_aftermath[0].final_destroyed_structure_ids | length),
      rts_battle_aftermath_debris_tile_count: ($rts_aftermath[0].final_debris_tile_ids | length),
      rts_battle_aftermath_smoke_tile_count: ($rts_aftermath[0].final_smoke_tile_ids | length),
      rts_battle_aftermath_veteran_unit_count: ($rts_aftermath[0].final_veteran_unit_ids | length),
      rts_battle_aftermath_veteran_log_count: ($rts_aftermath[0].final_veteran_level_log | length),
      rts_battle_aftermath_growth_level: $rts_aftermath[0].final_growth_level,
      rts_battle_aftermath_match_result_state: $rts_aftermath[0].final_match_result_state,
      rts_battle_aftermath_next_action_count: ($rts_aftermath[0].final_next_action_ids | length),
      rts_battle_aftermath_next_extraction_tile: $rts_aftermath[0].final_objective_extraction_tile_id,
      rts_battle_aftermath_pixel_count: (
        $rts_aftermath[0].debris_pixel_count
        + $rts_aftermath[0].smoke_pixel_count
        + $rts_aftermath[0].veteran_pixel_count
        + $rts_aftermath[0].match_result_pixel_count
        + $rts_aftermath[0].next_action_pixel_count
      ),
      rts_battle_aftermath_debris_pixel_count: $rts_aftermath[0].debris_pixel_count,
      rts_battle_aftermath_smoke_pixel_count: $rts_aftermath[0].smoke_pixel_count,
      rts_battle_aftermath_veteran_pixel_count: $rts_aftermath[0].veteran_pixel_count,
      rts_battle_aftermath_match_result_pixel_count: $rts_aftermath[0].match_result_pixel_count,
      rts_battle_aftermath_next_action_pixel_count: $rts_aftermath[0].next_action_pixel_count,
      rts_commander_progression_accepted_input_count: $rts_commander[0].accepted_input_count,
      rts_commander_progression_unit_id: $rts_commander[0].final_commander_unit_id,
      rts_commander_progression_level: $rts_commander[0].final_commander_level,
      rts_commander_progression_ability_point_count: $rts_commander[0].final_commander_ability_point_count,
      rts_commander_progression_aura_tile_count: ($rts_commander[0].final_commander_aura_tile_ids | length),
      rts_commander_progression_ability_log_count: ($rts_commander[0].final_commander_ability_log | length),
      rts_commander_progression_loot_count: ($rts_commander[0].final_loot_item_ids | length),
      rts_commander_progression_pickup_count: ($rts_commander[0].final_loot_pickup_log | length),
      rts_commander_progression_active_ability: $rts_commander[0].final_active_ability_id,
      rts_commander_progression_pixel_count: (
        $rts_commander[0].commander_pixel_count
        + $rts_commander[0].aura_pixel_count
        + $rts_commander[0].loot_pixel_count
        + $rts_commander[0].ability_point_pixel_count
      ),
      rts_commander_progression_commander_pixel_count: $rts_commander[0].commander_pixel_count,
      rts_commander_progression_aura_pixel_count: $rts_commander[0].aura_pixel_count,
      rts_commander_progression_loot_pixel_count: $rts_commander[0].loot_pixel_count,
      rts_commander_progression_ability_point_pixel_count: $rts_commander[0].ability_point_pixel_count,
      rts_expansion_counterattack_accepted_input_count: $rts_expansion[0].accepted_input_count,
      rts_expansion_counterattack_tile_count: ($rts_expansion[0].final_expansion_tile_ids | length),
      rts_expansion_counterattack_structure_count: ($rts_expansion[0].final_expansion_structure_ids | length),
      rts_expansion_counterattack_worker_count: ($rts_expansion[0].final_expansion_worker_unit_ids | length),
      rts_expansion_counterattack_income_per_minute: $rts_expansion[0].final_expansion_income_per_minute,
      rts_expansion_counterattack_wave_unit_count: ($rts_expansion[0].final_enemy_counterattack_unit_ids | length),
      rts_expansion_counterattack_route_tile_count: ($rts_expansion[0].final_enemy_counterattack_route_tile_ids | length),
      rts_expansion_counterattack_defense_state: $rts_expansion[0].final_expansion_defense_state,
      rts_expansion_counterattack_pixel_count: (
        $rts_expansion[0].expansion_tile_pixel_count
        + $rts_expansion[0].expansion_base_pixel_count
        + $rts_expansion[0].expansion_worker_pixel_count
        + $rts_expansion[0].expansion_income_pixel_count
        + $rts_expansion[0].counterattack_pixel_count
        + $rts_expansion[0].expansion_defense_pixel_count
      ),
      rts_expansion_counterattack_tile_pixel_count: $rts_expansion[0].expansion_tile_pixel_count,
      rts_expansion_counterattack_base_pixel_count: $rts_expansion[0].expansion_base_pixel_count,
      rts_expansion_counterattack_worker_pixel_count: $rts_expansion[0].expansion_worker_pixel_count,
      rts_expansion_counterattack_income_pixel_count: $rts_expansion[0].expansion_income_pixel_count,
      rts_expansion_counterattack_wave_pixel_count: $rts_expansion[0].counterattack_pixel_count,
      rts_expansion_counterattack_defense_pixel_count: $rts_expansion[0].expansion_defense_pixel_count,
      rts_tier_two_siege_push_accepted_input_count: $rts_tier_two[0].accepted_input_count,
      rts_tier_two_siege_push_tech_count: ($rts_tier_two[0].final_tier_two_tech_ids | length),
      rts_tier_two_siege_push_upgrade_count: ($rts_tier_two[0].final_tier_two_upgrade_ids | length),
      rts_tier_two_siege_push_unit_count: ($rts_tier_two[0].final_siege_unit_ids | length),
      rts_tier_two_siege_push_route_tile_count: ($rts_tier_two[0].final_siege_push_route_tile_ids | length),
      rts_tier_two_siege_push_enemy_fortification_count: ($rts_tier_two[0].final_enemy_fortification_ids | length),
      rts_tier_two_siege_push_state: $rts_tier_two[0].final_tier_two_push_state,
      rts_tier_two_siege_push_breach_percent: $rts_tier_two[0].final_base_breach_percent,
      rts_tier_two_siege_push_pixel_count: (
        $rts_tier_two[0].tier_two_tech_pixel_count
        + $rts_tier_two[0].siege_unit_pixel_count
        + $rts_tier_two[0].siege_route_pixel_count
        + $rts_tier_two[0].enemy_fortification_pixel_count
        + $rts_tier_two[0].siege_damage_pixel_count
      ),
      rts_tier_two_siege_push_tech_pixel_count: $rts_tier_two[0].tier_two_tech_pixel_count,
      rts_tier_two_siege_push_unit_pixel_count: $rts_tier_two[0].siege_unit_pixel_count,
      rts_tier_two_siege_push_route_pixel_count: $rts_tier_two[0].siege_route_pixel_count,
      rts_tier_two_siege_push_enemy_fortification_pixel_count: $rts_tier_two[0].enemy_fortification_pixel_count,
      rts_tier_two_siege_push_damage_pixel_count: $rts_tier_two[0].siege_damage_pixel_count,
      rts_siege_breach_counterplay_accepted_input_count: $rts_breach[0].accepted_input_count,
      rts_siege_breach_counterplay_target: $rts_breach[0].final_siege_breach_target_id,
      rts_siege_breach_counterplay_tile_count: ($rts_breach[0].final_siege_breach_tile_ids | length),
      rts_siege_breach_counterplay_repair_unit_count: ($rts_breach[0].final_enemy_repair_unit_ids | length),
      rts_siege_breach_counterplay_flank_unit_count: ($rts_breach[0].final_enemy_flank_unit_ids | length),
      rts_siege_breach_counterplay_hold_tile_count: ($rts_breach[0].final_player_hold_tile_ids | length),
      rts_siege_breach_counterplay_state: $rts_breach[0].final_siege_breach_state,
      rts_siege_breach_counterplay_breach_percent: $rts_breach[0].final_base_breach_percent,
      rts_siege_breach_counterplay_pixel_count: (
        $rts_breach[0].breach_pixel_count
        + $rts_breach[0].repair_pixel_count
        + $rts_breach[0].flank_pixel_count
        + $rts_breach[0].hold_pixel_count
        + $rts_breach[0].resolution_pixel_count
      ),
      rts_siege_breach_counterplay_breach_pixel_count: $rts_breach[0].breach_pixel_count,
      rts_siege_breach_counterplay_repair_pixel_count: $rts_breach[0].repair_pixel_count,
      rts_siege_breach_counterplay_flank_pixel_count: $rts_breach[0].flank_pixel_count,
      rts_siege_breach_counterplay_hold_pixel_count: $rts_breach[0].hold_pixel_count,
      rts_siege_breach_counterplay_resolution_pixel_count: $rts_breach[0].resolution_pixel_count,
      rts_inner_lane_breakthrough_accepted_input_count: $rts_inner[0].accepted_input_count,
      rts_inner_lane_breakthrough_tile_count: ($rts_inner[0].final_inner_lane_tile_ids | length),
      rts_inner_lane_breakthrough_gate_count: ($rts_inner[0].final_inner_gate_ids | length),
      rts_inner_lane_breakthrough_defender_count: ($rts_inner[0].final_inner_defender_unit_ids | length),
      rts_inner_lane_breakthrough_supply_count: ($rts_inner[0].final_supply_convoy_ids | length),
      rts_inner_lane_breakthrough_split_tile_count: ($rts_inner[0].final_split_squad_tile_ids | length),
      rts_inner_lane_breakthrough_state: $rts_inner[0].final_inner_objective_state,
      rts_inner_lane_breakthrough_match_result: $rts_inner[0].final_match_result_state,
      rts_inner_lane_breakthrough_capture_percent: $rts_inner[0].final_objective_capture_percent,
      rts_inner_lane_breakthrough_pixel_count: (
        $rts_inner[0].inner_route_pixel_count
        + $rts_inner[0].inner_gate_pixel_count
        + $rts_inner[0].inner_defender_pixel_count
        + $rts_inner[0].inner_supply_pixel_count
        + $rts_inner[0].inner_split_pixel_count
        + $rts_inner[0].inner_core_pixel_count
      ),
      rts_inner_lane_breakthrough_route_pixel_count: $rts_inner[0].inner_route_pixel_count,
      rts_inner_lane_breakthrough_gate_pixel_count: $rts_inner[0].inner_gate_pixel_count,
      rts_inner_lane_breakthrough_defender_pixel_count: $rts_inner[0].inner_defender_pixel_count,
      rts_inner_lane_breakthrough_supply_pixel_count: $rts_inner[0].inner_supply_pixel_count,
      rts_inner_lane_breakthrough_split_pixel_count: $rts_inner[0].inner_split_pixel_count,
      rts_inner_lane_breakthrough_core_pixel_count: $rts_inner[0].inner_core_pixel_count,
      rts_central_keep_pressure_accepted_input_count: $rts_keep[0].accepted_input_count,
      rts_central_keep_pressure_target_count: ($rts_keep[0].final_central_keep_target_ids | length),
      rts_central_keep_pressure_route_tile_count: ($rts_keep[0].final_central_keep_route_tile_ids | length),
      rts_central_keep_pressure_shield_percent: $rts_keep[0].final_keep_shield_percent,
      rts_central_keep_pressure_guard_count: ($rts_keep[0].final_boss_guard_unit_ids | length),
      rts_central_keep_pressure_siege_line_count: ($rts_keep[0].final_player_siege_line_tile_ids | length),
      rts_central_keep_pressure_state: $rts_keep[0].final_central_keep_state,
      rts_central_keep_pressure_match_result: $rts_keep[0].final_match_result_state,
      rts_central_keep_pressure_pixel_count: (
        $rts_keep[0].keep_route_pixel_count
        + $rts_keep[0].keep_shield_pixel_count
        + $rts_keep[0].keep_guard_pixel_count
        + $rts_keep[0].keep_siege_line_pixel_count
        + $rts_keep[0].keep_pressure_pixel_count
      ),
      rts_central_keep_pressure_route_pixel_count: $rts_keep[0].keep_route_pixel_count,
      rts_central_keep_pressure_shield_pixel_count: $rts_keep[0].keep_shield_pixel_count,
      rts_central_keep_pressure_guard_pixel_count: $rts_keep[0].keep_guard_pixel_count,
      rts_central_keep_pressure_siege_line_pixel_count: $rts_keep[0].keep_siege_line_pixel_count,
      rts_central_keep_pressure_pressure_pixel_count: $rts_keep[0].keep_pressure_pixel_count,
      rts_central_keep_breakthrough_accepted_input_count: $rts_keep_break[0].accepted_input_count,
      rts_central_keep_breakthrough_breach_percent: $rts_keep_break[0].final_keep_breach_percent,
      rts_central_keep_breakthrough_guardian_count: ($rts_keep_break[0].final_guardian_counter_unit_ids | length),
      rts_central_keep_breakthrough_hold_tile_count: ($rts_keep_break[0].final_player_hold_tile_ids | length),
      rts_central_keep_breakthrough_claim_tile_count: ($rts_keep_break[0].final_keep_claim_tile_ids | length),
      rts_central_keep_breakthrough_state: $rts_keep_break[0].final_central_keep_breakthrough_state,
      rts_central_keep_breakthrough_match_result: $rts_keep_break[0].final_match_result_state,
      rts_central_keep_breakthrough_pixel_count: (
        $rts_keep_break[0].keep_breach_pixel_count
        + $rts_keep_break[0].keep_counter_pixel_count
        + $rts_keep_break[0].keep_claim_pixel_count
        + $rts_keep_break[0].keep_victory_pixel_count
      ),
      rts_central_keep_breakthrough_breach_pixel_count: $rts_keep_break[0].keep_breach_pixel_count,
      rts_central_keep_breakthrough_counter_pixel_count: $rts_keep_break[0].keep_counter_pixel_count,
      rts_central_keep_breakthrough_claim_pixel_count: $rts_keep_break[0].keep_claim_pixel_count,
      rts_central_keep_breakthrough_victory_pixel_count: $rts_keep_break[0].keep_victory_pixel_count,
      rts_mirror_city_restoration_accepted_input_count: $rts_restore[0].accepted_input_count,
      rts_mirror_city_restoration_zone_count: ($rts_restore[0].final_restored_zone_ids | length),
      rts_mirror_city_restoration_rebuild_count: ($rts_restore[0].final_rebuild_structure_ids | length),
      rts_mirror_city_restoration_garrison_count: ($rts_restore[0].final_garrison_unit_ids | length),
      rts_mirror_city_restoration_state: $rts_restore[0].final_victory_handoff_state,
      rts_mirror_city_restoration_match_result: $rts_restore[0].final_match_result_state,
      rts_mirror_city_restoration_pixel_count: (
        $rts_restore[0].restore_zone_pixel_count
        + $rts_restore[0].rebuild_core_pixel_count
        + $rts_restore[0].garrison_pixel_count
        + $rts_restore[0].handoff_pixel_count
      ),
      rts_mirror_city_restoration_zone_pixel_count: $rts_restore[0].restore_zone_pixel_count,
      rts_mirror_city_restoration_rebuild_pixel_count: $rts_restore[0].rebuild_core_pixel_count,
      rts_mirror_city_restoration_garrison_pixel_count: $rts_restore[0].garrison_pixel_count,
      rts_mirror_city_restoration_handoff_pixel_count: $rts_restore[0].handoff_pixel_count,
      rts_open_world_after_action_accepted_input_count: $rts_open_world[0].accepted_input_count,
      rts_open_world_after_action_room_id: $rts_open_world[0].final_current_room_id,
      rts_open_world_after_action_map_scene: $rts_open_world[0].final_map_scene,
      rts_open_world_after_action_route_tile_count: ($rts_open_world[0].final_open_world_route_tile_ids | length),
      rts_open_world_after_action_panel_count: ($rts_open_world[0].final_open_world_panel_ids | length),
      rts_open_world_after_action_task_count: ($rts_open_world[0].final_open_world_task_ids | length),
      rts_open_world_after_action_handoff_state: $rts_open_world[0].final_open_world_handoff_state,
      rts_open_world_after_action_pixel_count: (
        $rts_open_world[0].open_world_route_pixel_count
        + $rts_open_world[0].open_world_panel_pixel_count
        + $rts_open_world[0].open_world_resume_pixel_count
      ),
      rts_open_world_after_action_route_pixel_count: $rts_open_world[0].open_world_route_pixel_count,
      rts_open_world_after_action_panel_pixel_count: $rts_open_world[0].open_world_panel_pixel_count,
      rts_open_world_after_action_resume_pixel_count: $rts_open_world[0].open_world_resume_pixel_count,
      rts_campaign_handoff_accepted_input_count: $rts_campaign[0].accepted_input_count,
      rts_campaign_handoff_capture_frame_count: $rts_campaign[0].capture_frame_count,
      rts_campaign_handoff_room_id: $rts_campaign[0].final_current_room_id,
      rts_campaign_handoff_map_scene: $rts_campaign[0].final_map_scene,
      rts_campaign_handoff_route_director_task_id: $rts_campaign[0].final_route_director_task_id,
      rts_campaign_handoff_snapshot_json_byte_count: $rts_campaign[0].snapshot_json_byte_count,
      rts_campaign_handoff_restored_room_id: $rts_campaign[0].restored_current_room_id,
      rts_campaign_handoff_pixel_count: (
        $rts_campaign[0].victory_pixel_count
        + $rts_campaign[0].expansion_pixel_count
        + $rts_campaign[0].breach_pixel_count
        + $rts_campaign[0].keep_pixel_count
        + $rts_campaign[0].restoration_pixel_count
        + $rts_campaign[0].open_world_pixel_count
      ),
      rts_campaign_entry_input_action_count: $rts_campaign_entry[0].input_action_count,
      rts_campaign_entry_start_input_count: $rts_campaign_entry[0].start_input_count,
      rts_campaign_entry_replay_input_count: $rts_campaign_entry[0].replay_input_count,
      rts_campaign_entry_slot_bytes: $rts_campaign_entry[0].campaign_slot_bytes,
      rts_campaign_entry_room_id: $rts_campaign_entry[0].final_current_room_id,
      rts_campaign_entry_map_scene: $rts_campaign_entry[0].final_map_scene,
      rts_campaign_entry_open_world_handoff_state: $rts_campaign_entry[0].final_open_world_handoff_state,
      runner_main_pid: $runner[0].service.main_pid,
      runner_process_cwd: $runner[0].runtime.process_cwd,
      launcher_main_pid: $launcher[0].live_runner.service.main_pid,
      launcher_player_entry_action_count: $launcher[0].player_entry.input_action_count,
      launcher_campaign_slot_bytes: $launcher[0].player_entry.campaign_slot_bytes,
      launcher_resume_room_id: $launcher[0].player_entry.final_current_room_id,
      launcher_resume_action_label: $launcher[0].player_entry.final_contextual_primary_action_label
    },
    gates: {
      cex_runtime_player_client_allowed: $manifest[0].cex_runtime_player_client_allowed,
      wgpu_required: $manifest[0].wgpu_required,
      manifest_boundary_gate: $manifest[0].boundary_gate,
      animation_action_coverage_gate: $animation[0].action_coverage_gate,
      selector_transition_gate: $selector[0].animation_transition_gate,
      motion_direction_coverage_gate: $motion[0].direction_coverage_gate,
      input_frame_direction_coverage_gate: $input_budget[0].direction_coverage_gate,
      input_frame_p95_budget_gate: $input_budget[0].response_p95_budget_gate,
      input_frame_max_budget_gate: $input_budget[0].response_max_budget_gate,
      render_p95_budget_gate: $budget[0].p95_budget_gate,
      render_max_budget_gate: $budget[0].max_budget_gate,
      scene_dynamic_landmark_animation_gate: $scene[0].dynamic_landmark_animation_gate,
      renderer_probe_scene_frame_gate: $probe[0].scene_frame_gate,
      isometric_projection_gate: $iso[0].projection_gate,
      isometric_depth_sort_gate: $iso[0].depth_sort_gate,
      isometric_diamond_tile_gate: $iso[0].diamond_tile_gate,
      isometric_shadow_anchor_gate: $iso[0].shadow_anchor_gate,
      isometric_procedural_volume_gate: $iso[0].procedural_volume_gate,
      isometric_rts_model_set_gate: $iso[0].rts_model_set_gate,
      isometric_terrain_detail_gate: $iso[0].terrain_detail_gate,
      isometric_unit_detail_gate: $iso[0].unit_detail_gate,
      isometric_neutral_unit_detail_gate: $iso[0].neutral_unit_detail_gate,
      isometric_command_feedback_gate: $iso[0].command_feedback_gate,
      isometric_doodad_detail_gate: $iso[0].doodad_detail_gate,
      isometric_environment_detail_gate: $iso[0].environment_detail_gate,
      isometric_sprite_anchor_gate: $iso[0].sprite_anchor_gate,
      catalog_all_frames_rendered_gate: $catalog[0].all_frames_rendered_gate,
      asset_slot_required_categories_gate: $slots[0].required_categories_present_gate,
      asset_slot_manifest_frame_slots_gate: $slots[0].manifest_frame_slots_gate,
      asset_slot_procedural_slots_gate: $slots[0].procedural_slots_gate,
      asset_slot_replacement_boundary_gate: $slots[0].replacement_boundary_gate,
      art_pack_required_model_gate: $art_pack[0].required_model_gate,
      art_pack_player_art_gate: $art_pack[0].player_art_gate,
      art_pack_enemy_art_gate: $art_pack[0].enemy_art_gate,
      art_pack_neutral_unit_art_gate: $art_pack[0].neutral_unit_art_gate,
      art_pack_doodad_art_gate: $art_pack[0].doodad_art_gate,
      art_pack_terrain_art_gate: $art_pack[0].terrain_art_gate,
      art_pack_world_prop_art_gate: $art_pack[0].world_prop_art_gate,
      art_pack_vfx_art_gate: $art_pack[0].vfx_art_gate,
      art_pack_model_detail_gate: $art_pack[0].model_detail_gate,
      art_pack_unit_detail_gate: $art_pack[0].unit_detail_gate,
      art_pack_neutral_unit_detail_gate: $art_pack[0].neutral_unit_detail_gate,
      art_pack_doodad_detail_gate: $art_pack[0].doodad_detail_gate,
      art_pack_terrain_detail_gate: $art_pack[0].terrain_detail_gate,
      art_pack_world_prop_detail_gate: $art_pack[0].world_prop_detail_gate,
      art_pack_vfx_detail_gate: $art_pack[0].vfx_detail_gate,
      art_pack_replacement_boundary_gate: $art_pack[0].replacement_boundary_gate,
      art_pack_scene_override_presence_gate: $art_scene[0].override_presence_gate,
      art_pack_scene_color_probe_gate: $art_scene[0].color_probe_gate,
      art_pack_scene_terrain_override_presence_gate: $art_scene[0].terrain_override_presence_gate,
      art_pack_scene_terrain_color_probe_gate: $art_scene[0].terrain_color_probe_gate,
      art_pack_scene_world_prop_override_presence_gate: $art_scene[0].world_prop_override_presence_gate,
      art_pack_scene_world_prop_color_probe_gate: $art_scene[0].world_prop_color_probe_gate,
      art_pack_scene_neutral_unit_override_presence_gate: $art_scene[0].neutral_unit_override_presence_gate,
      art_pack_scene_neutral_unit_color_probe_gate: $art_scene[0].neutral_unit_color_probe_gate,
      art_pack_scene_environment_override_presence_gate: $art_scene[0].environment_override_presence_gate,
      art_pack_scene_environment_detail_color_probe_gate: $art_scene[0].environment_detail_color_probe_gate,
      art_pack_scene_vfx_override_presence_gate: $art_scene[0].vfx_override_presence_gate,
      art_pack_scene_vfx_color_probe_gate: $art_scene[0].vfx_color_probe_gate,
      art_pack_scene_replacement_boundary_gate: $art_scene[0].replacement_boundary_gate,
      asset_override_frame_gate: $override[0].override_frame_gate,
      asset_override_replacement_boundary_gate: $override[0].replacement_boundary_gate,
      rts_control_loop_selection_gate: $rts[0].selection_gate,
      rts_control_loop_command_queue_gate: $rts[0].command_queue_gate,
      rts_control_loop_strategy_hud_gate: $rts[0].strategy_hud_gate,
      rts_control_loop_macro_loop_gate: $rts[0].macro_loop_gate,
      rts_control_loop_tactical_combat_gate: $rts[0].tactical_combat_gate,
      rts_control_loop_gameplay_surface_gate: $rts[0].gameplay_surface_gate,
      rts_live_input_live_input_gate: $rts_live[0].live_input_gate,
      rts_live_input_selection_live_gate: $rts_live[0].selection_live_gate,
      rts_live_input_production_live_gate: $rts_live[0].production_live_gate,
      rts_live_input_move_live_gate: $rts_live[0].move_live_gate,
      rts_live_input_attack_live_gate: $rts_live[0].attack_live_gate,
      rts_live_input_ability_live_gate: $rts_live[0].ability_live_gate,
      rts_pathing_live_input_gate: $rts_path[0].live_pathing_input_gate,
      rts_pathing_path_tile_gate: $rts_path[0].path_tile_gate,
      rts_pathing_blocked_tile_gate: $rts_path[0].blocked_tile_gate,
      rts_pathing_formation_slot_gate: $rts_path[0].formation_slot_gate,
      rts_pathing_command_visual_gate: $rts_path[0].command_visual_gate,
      rts_collision_live_input_gate: $rts_collision[0].live_collision_input_gate,
      rts_collision_collision_response_gate: $rts_collision[0].collision_response_gate,
      rts_collision_engagement_response_gate: $rts_collision[0].engagement_response_gate,
      rts_targeting_live_input_gate: $rts_target[0].live_targeting_input_gate,
      rts_targeting_target_priority_gate: $rts_target[0].target_priority_gate,
      rts_targeting_aggro_gate: $rts_target[0].aggro_gate,
      rts_targeting_focus_fire_gate: $rts_target[0].focus_fire_gate,
      rts_targeting_threat_feedback_gate: $rts_target[0].threat_feedback_gate,
      rts_economy_live_input_gate: $rts_economy[0].live_economy_input_gate,
      rts_economy_harvest_loop_gate: $rts_economy[0].harvest_loop_gate,
      rts_economy_build_loop_gate: $rts_economy[0].build_loop_gate,
      rts_economy_production_loop_gate: $rts_economy[0].production_loop_gate,
      rts_selection_minimap_live_input_gate: $rts_select[0].live_selection_minimap_input_gate,
      rts_selection_box_gate: $rts_select[0].selection_box_gate,
      rts_control_group_gate: $rts_select[0].control_group_gate,
      rts_minimap_command_gate: $rts_select[0].minimap_command_gate,
      rts_split_route_gate: $rts_select[0].split_route_gate,
      rts_build_lifecycle_live_input_gate: $rts_build_lifecycle[0].live_build_lifecycle_input_gate,
      rts_build_lifecycle_build_placement_gate: $rts_build_lifecycle[0].build_placement_gate,
      rts_build_lifecycle_completion_gate: $rts_build_lifecycle[0].completion_gate,
      rts_build_lifecycle_repair_gate: $rts_build_lifecycle[0].repair_gate,
      rts_build_lifecycle_cancel_refund_gate: $rts_build_lifecycle[0].cancel_refund_gate,
      rts_tech_tree_live_input_gate: $rts_tech_tree[0].live_tech_tree_input_gate,
      rts_tech_tree_faction_base_gate: $rts_tech_tree[0].faction_base_gate,
      rts_tech_tree_research_gate: $rts_tech_tree[0].research_gate,
      rts_tech_tree_upgrade_gate: $rts_tech_tree[0].upgrade_gate,
      rts_tech_tree_unlock_gate: $rts_tech_tree[0].unlock_gate,
      rts_tech_tree_dependency_gate: $rts_tech_tree[0].dependency_gate,
      rts_projectile_ability_live_input_gate: $rts_projectile[0].live_projectile_ability_input_gate,
      rts_projectile_ability_projectile_trail_gate: $rts_projectile[0].projectile_trail_gate,
      rts_projectile_ability_projectile_impact_gate: $rts_projectile[0].projectile_impact_gate,
      rts_projectile_ability_ability_radius_gate: $rts_projectile[0].ability_radius_gate,
      rts_projectile_ability_damage_tick_gate: $rts_projectile[0].damage_tick_gate,
      rts_projectile_ability_armor_shield_gate: $rts_projectile[0].armor_shield_gate,
      rts_ai_skirmish_pressure_live_input_gate: $rts_ai[0].live_ai_skirmish_input_gate,
      rts_ai_skirmish_pressure_ai_wave_gate: $rts_ai[0].ai_wave_gate,
      rts_ai_skirmish_pressure_ai_counter_gate: $rts_ai[0].ai_counter_gate,
      rts_ai_skirmish_pressure_resolution_gate: $rts_ai[0].ai_pressure_resolution_gate,
      rts_ai_skirmish_pressure_retreat_gate: $rts_ai[0].ai_retreat_gate,
      rts_ai_skirmish_player_response_gate: $rts_ai[0].player_response_gate,
      rts_objective_victory_loop_live_input_gate: $rts_objective[0].live_objective_input_gate,
      rts_objective_victory_loop_marker_gate: $rts_objective[0].objective_marker_gate,
      rts_objective_victory_loop_capture_gate: $rts_objective[0].capture_progress_gate,
      rts_objective_victory_loop_victory_gate: $rts_objective[0].victory_resolution_gate,
      rts_objective_victory_loop_defeat_pressure_gate: $rts_objective[0].defeat_pressure_gate,
      rts_objective_victory_loop_extraction_gate: $rts_objective[0].extraction_gate,
      rts_creep_camp_terrain_route_live_input_gate: $rts_creep_camp[0].live_creep_camp_input_gate,
      rts_creep_camp_terrain_route_terrain_gate: $rts_creep_camp[0].terrain_route_gate,
      rts_creep_camp_terrain_route_choke_gate: $rts_creep_camp[0].choke_gate,
      rts_creep_camp_terrain_route_clear_gate: $rts_creep_camp[0].camp_clear_gate,
      rts_creep_camp_terrain_route_reveal_gate: $rts_creep_camp[0].scout_reveal_gate,
      rts_creep_camp_terrain_route_expansion_gate: $rts_creep_camp[0].expansion_route_gate,
      rts_fog_scouting_intel_live_input_gate: $rts_fog[0].live_fog_scouting_input_gate,
      rts_fog_scouting_intel_scout_route_gate: $rts_fog[0].scout_route_gate,
      rts_fog_scouting_intel_fog_reveal_gate: $rts_fog[0].fog_reveal_gate,
      rts_fog_scouting_intel_enemy_structure_gate: $rts_fog[0].enemy_structure_intel_gate,
      rts_fog_scouting_intel_enemy_unit_gate: $rts_fog[0].enemy_unit_intel_gate,
      rts_fog_scouting_intel_intel_log_gate: $rts_fog[0].intel_log_gate,
      rts_fog_scouting_intel_visibility_gate: $rts_fog[0].visibility_bar_gate,
      rts_enemy_base_tech_pressure_live_input_gate: $rts_enemy_base[0].live_enemy_base_tech_pressure_input_gate,
      rts_enemy_base_tech_pressure_intel_dependency_gate: $rts_enemy_base[0].intel_dependency_gate,
      rts_enemy_base_tech_pressure_enemy_tech_gate: $rts_enemy_base[0].enemy_tech_gate,
      rts_enemy_base_tech_pressure_enemy_production_gate: $rts_enemy_base[0].enemy_production_gate,
      rts_enemy_base_tech_pressure_player_counter_gate: $rts_enemy_base[0].player_counter_gate,
      rts_enemy_base_tech_pressure_defense_ready_gate: $rts_enemy_base[0].defense_ready_gate,
      rts_enemy_base_tech_pressure_warning_gate: $rts_enemy_base[0].pressure_warning_gate,
      rts_army_production_rally_live_input_gate: $rts_army[0].live_army_production_input_gate,
      rts_army_production_rally_supply_gate: $rts_army[0].supply_gate,
      rts_army_production_rally_production_batch_gate: $rts_army[0].production_batch_gate,
      rts_army_production_rally_rally_gate: $rts_army[0].rally_gate,
      rts_army_production_rally_control_group_gate: $rts_army[0].control_group_gate,
      rts_army_production_rally_composition_gate: $rts_army[0].composition_gate,
      rts_base_assault_resolution_live_input_gate: $rts_base_assault[0].live_base_assault_input_gate,
      rts_base_assault_resolution_army_dependency_gate: $rts_base_assault[0].army_dependency_gate,
      rts_base_assault_resolution_assault_path_gate: $rts_base_assault[0].assault_path_gate,
      rts_base_assault_resolution_enemy_base_health_gate: $rts_base_assault[0].enemy_base_health_gate,
      rts_base_assault_resolution_breach_gate: $rts_base_assault[0].breach_resolution_gate,
      rts_base_assault_resolution_reward_gate: $rts_base_assault[0].reward_gate,
      rts_battle_aftermath_live_input_gate: $rts_aftermath[0].live_aftermath_input_gate,
      rts_battle_aftermath_assault_dependency_gate: $rts_aftermath[0].assault_dependency_gate,
      rts_battle_aftermath_destruction_gate: $rts_aftermath[0].destruction_gate,
      rts_battle_aftermath_veteran_gate: $rts_aftermath[0].veteran_gate,
      rts_battle_aftermath_match_result_gate: $rts_aftermath[0].match_result_gate,
      rts_battle_aftermath_next_action_gate: $rts_aftermath[0].next_action_gate,
      rts_battle_aftermath_reward_gate: $rts_aftermath[0].reward_gate,
      rts_commander_progression_live_input_gate: $rts_commander[0].live_commander_input_gate,
      rts_commander_progression_aftermath_dependency_gate: $rts_commander[0].aftermath_dependency_gate,
      rts_commander_progression_loot_gate: $rts_commander[0].loot_gate,
      rts_commander_progression_level_gate: $rts_commander[0].commander_level_gate,
      rts_commander_progression_ability_point_gate: $rts_commander[0].ability_point_gate,
      rts_commander_progression_aura_gate: $rts_commander[0].aura_gate,
      rts_expansion_counterattack_live_input_gate: $rts_expansion[0].live_expansion_input_gate,
      rts_expansion_counterattack_commander_dependency_gate: $rts_expansion[0].commander_dependency_gate,
      rts_expansion_counterattack_claim_gate: $rts_expansion[0].expansion_claim_gate,
      rts_expansion_counterattack_build_gate: $rts_expansion[0].expansion_build_gate,
      rts_expansion_counterattack_worker_income_gate: $rts_expansion[0].expansion_worker_income_gate,
      rts_expansion_counterattack_counterattack_gate: $rts_expansion[0].counterattack_gate,
      rts_expansion_counterattack_defense_gate: $rts_expansion[0].defense_gate,
      rts_tier_two_siege_push_live_input_gate: $rts_tier_two[0].live_tier_two_input_gate,
      rts_tier_two_siege_push_expansion_dependency_gate: $rts_tier_two[0].expansion_dependency_gate,
      rts_tier_two_siege_push_tech_gate: $rts_tier_two[0].tier_two_tech_gate,
      rts_tier_two_siege_push_upgrade_gate: $rts_tier_two[0].tier_two_upgrade_gate,
      rts_tier_two_siege_push_unit_gate: $rts_tier_two[0].siege_unit_gate,
      rts_tier_two_siege_push_enemy_fortification_gate: $rts_tier_two[0].enemy_fortification_gate,
      rts_tier_two_siege_push_push_gate: $rts_tier_two[0].siege_push_gate,
      rts_siege_breach_counterplay_live_input_gate: $rts_breach[0].live_siege_breach_input_gate,
      rts_siege_breach_counterplay_tier_two_dependency_gate: $rts_breach[0].tier_two_dependency_gate,
      rts_siege_breach_counterplay_breach_window_gate: $rts_breach[0].breach_window_gate,
      rts_siege_breach_counterplay_repair_reaction_gate: $rts_breach[0].repair_reaction_gate,
      rts_siege_breach_counterplay_flank_pressure_gate: $rts_breach[0].flank_pressure_gate,
      rts_siege_breach_counterplay_hold_line_gate: $rts_breach[0].hold_line_gate,
      rts_siege_breach_counterplay_resolution_gate: $rts_breach[0].resolution_gate,
      rts_inner_lane_breakthrough_live_input_gate: $rts_inner[0].live_inner_lane_input_gate,
      rts_inner_lane_breakthrough_siege_breach_dependency_gate: $rts_inner[0].siege_breach_dependency_gate,
      rts_inner_lane_breakthrough_route_gate: $rts_inner[0].inner_route_gate,
      rts_inner_lane_breakthrough_gate_gate: $rts_inner[0].inner_gate_gate,
      rts_inner_lane_breakthrough_supply_gate: $rts_inner[0].supply_convoy_gate,
      rts_inner_lane_breakthrough_split_gate: $rts_inner[0].split_squad_gate,
      rts_inner_lane_breakthrough_clear_gate: $rts_inner[0].second_line_clear_gate,
      rts_inner_lane_breakthrough_secure_gate: $rts_inner[0].signal_core_secure_gate,
      rts_central_keep_pressure_live_input_gate: $rts_keep[0].live_central_keep_input_gate,
      rts_central_keep_pressure_inner_lane_dependency_gate: $rts_keep[0].inner_lane_dependency_gate,
      rts_central_keep_pressure_route_gate: $rts_keep[0].keep_route_gate,
      rts_central_keep_pressure_shield_gate: $rts_keep[0].keep_shield_gate,
      rts_central_keep_pressure_guard_gate: $rts_keep[0].keep_guard_gate,
      rts_central_keep_pressure_siege_line_gate: $rts_keep[0].keep_siege_line_gate,
      rts_central_keep_pressure_pressure_gate: $rts_keep[0].keep_pressure_gate,
      rts_central_keep_breakthrough_live_input_gate: $rts_keep_break[0].live_keep_breakthrough_input_gate,
      rts_central_keep_breakthrough_pressure_dependency_gate: $rts_keep_break[0].central_keep_pressure_dependency_gate,
      rts_central_keep_breakthrough_breach_gate: $rts_keep_break[0].keep_breach_gate,
      rts_central_keep_breakthrough_guardian_counter_gate: $rts_keep_break[0].guardian_counter_gate,
      rts_central_keep_breakthrough_hold_gate: $rts_keep_break[0].keep_hold_gate,
      rts_central_keep_breakthrough_break_gate: $rts_keep_break[0].keep_break_gate,
      rts_central_keep_breakthrough_claim_gate: $rts_keep_break[0].keep_claim_gate,
      rts_mirror_city_restoration_live_input_gate: $rts_restore[0].live_restoration_input_gate,
      rts_mirror_city_restoration_victory_dependency_gate: $rts_restore[0].victory_dependency_gate,
      rts_mirror_city_restoration_restore_gate: $rts_restore[0].restore_city_gate,
      rts_mirror_city_restoration_rebuild_gate: $rts_restore[0].rebuild_core_gate,
      rts_mirror_city_restoration_garrison_gate: $rts_restore[0].garrison_gate,
      rts_mirror_city_restoration_handoff_gate: $rts_restore[0].handoff_gate,
      rts_open_world_after_action_live_input_gate: $rts_open_world[0].live_open_world_input_gate,
      rts_open_world_after_action_restoration_dependency_gate: $rts_open_world[0].restoration_dependency_gate,
      rts_open_world_after_action_route_gate: $rts_open_world[0].open_world_route_gate,
      rts_open_world_after_action_panel_gate: $rts_open_world[0].open_world_panel_gate,
      rts_open_world_after_action_resume_gate: $rts_open_world[0].open_world_resume_gate,
      rts_open_world_after_action_command_gate: $rts_open_world[0].command_gate,
      rts_campaign_handoff_live_input_gate: $rts_campaign[0].live_campaign_input_gate,
      rts_campaign_handoff_early_campaign_gate: $rts_campaign[0].early_campaign_gate,
      rts_campaign_handoff_mid_campaign_gate: $rts_campaign[0].mid_campaign_gate,
      rts_campaign_handoff_end_campaign_gate: $rts_campaign[0].end_campaign_gate,
      rts_campaign_handoff_open_world_resume_gate: $rts_campaign[0].open_world_resume_gate,
      rts_campaign_handoff_snapshot_round_trip_gate: $rts_campaign[0].snapshot_round_trip_gate,
      rts_campaign_handoff_render_milestone_gate: $rts_campaign[0].render_milestone_gate,
      rts_campaign_entry_title_entry_gate: $rts_campaign_entry[0].title_entry_gate,
      rts_campaign_entry_start_gate: $rts_campaign_entry[0].start_gate,
      rts_campaign_entry_slot_snapshot_gate: $rts_campaign_entry[0].slot_snapshot_gate,
      rts_campaign_entry_continue_gate: $rts_campaign_entry[0].continue_gate,
      rts_campaign_entry_continue_unlock_gate: $rts_campaign_entry[0].continue_unlock_gate,
      rts_campaign_entry_replay_gate: $rts_campaign_entry[0].replay_gate,
      runner_service_process_gate: $runner[0].gates.service_process_gate,
      runner_release_binary_gate: $runner[0].gates.release_binary_gate,
      runner_classic_env_gate: $runner[0].gates.classic_env_gate,
      runner_override_dir_gate: $runner[0].gates.override_dir_gate,
      runner_cex_path_gate: $runner[0].gates.cex_path_gate,
      launcher_player_launch_ready_gate: $launcher[0].gates.player_launch_ready_gate,
      launcher_campaign_entry_gate: $launcher[0].gates.campaign_entry_gate,
      launcher_campaign_slot_gate: $launcher[0].gates.campaign_slot_gate,
      launcher_open_world_resume_gate: $launcher[0].gates.open_world_resume_gate,
      launcher_player_command_gate: $launcher[0].gates.player_command_gate,
      launcher_service_process_gate: $launcher[0].gates.service_process_gate,
      launcher_release_binary_gate: $launcher[0].gates.release_binary_gate,
      launcher_cex_path_gate: $launcher[0].gates.cex_path_gate
    },
    artifacts: {
      manifest_lint: "acceptance/S5_native_bevy_device/latest/bevy-classic-manifest-lint.json",
      animation_preview: "acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.json",
      animation_preview_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.ppm",
      animation_selector: "acceptance/S5_native_bevy_device/latest/bevy-classic-animation-selector.json",
      player_motion_probe: "acceptance/S5_native_bevy_device/latest/bevy-classic-player-motion-probe.json",
      player_motion_probe_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-player-motion-probe.ppm",
      input_frame_budget: "acceptance/S5_native_bevy_device/latest/bevy-classic-input-frame-budget.json",
      render_budget: "acceptance/S5_native_bevy_device/latest/bevy-classic-render-budget.json",
      scene_preview: "acceptance/S5_native_bevy_device/latest/bevy-classic-scene-preview.json",
      scene_preview_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-scene-preview.ppm",
      renderer_probe: "acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.json",
      renderer_probe_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.ppm",
      isometric_modeling: "acceptance/S5_native_bevy_device/latest/bevy-classic-isometric-modeling.json",
      isometric_modeling_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-isometric-modeling.ppm",
      model_catalog: "acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.json",
      model_catalog_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.ppm",
      asset_slot_map: "acceptance/S5_native_bevy_device/latest/bevy-classic-asset-slot-map.json",
      classic_art_pack: "acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack.json",
      classic_art_pack_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack.ppm",
      classic_art_pack_scene_probe: "acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack-scene-probe.json",
      classic_art_pack_scene_probe_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack-scene-probe.ppm",
      asset_override_probe: "acceptance/S5_native_bevy_device/latest/bevy-classic-asset-override-probe.json",
      asset_override_probe_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-asset-override-probe.ppm",
      classic_rts_control_loop: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.json",
      classic_rts_control_loop_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.ppm",
      classic_rts_live_input_sequence: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-input-sequence.json",
      classic_rts_live_input_sequence_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-input-sequence.ppm",
      classic_rts_pathing_formation: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-pathing-formation.json",
      classic_rts_pathing_formation_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-pathing-formation.ppm",
      classic_rts_collision_engagement: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-collision-engagement.json",
      classic_rts_collision_engagement_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-collision-engagement.ppm",
      classic_rts_target_aggro_focus: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-target-aggro-focus.json",
      classic_rts_target_aggro_focus_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-target-aggro-focus.ppm",
      classic_rts_economy_build: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-economy-build.json",
      classic_rts_economy_build_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-economy-build.ppm",
      classic_rts_selection_minimap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-minimap.json",
      classic_rts_selection_minimap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-minimap.ppm",
      classic_rts_build_lifecycle: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-build-lifecycle.json",
      classic_rts_build_lifecycle_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-build-lifecycle.ppm",
      classic_rts_tech_tree: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tech-tree.json",
      classic_rts_tech_tree_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tech-tree.ppm",
      classic_rts_projectile_ability: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-projectile-ability.json",
      classic_rts_projectile_ability_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-projectile-ability.ppm",
      classic_rts_ai_skirmish_pressure: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ai-skirmish-pressure.json",
      classic_rts_ai_skirmish_pressure_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ai-skirmish-pressure.ppm",
      classic_rts_objective_victory_loop: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-objective-victory-loop.json",
      classic_rts_objective_victory_loop_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-objective-victory-loop.ppm",
      classic_rts_creep_camp_terrain_route: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-creep-camp-terrain-route.json",
      classic_rts_creep_camp_terrain_route_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-creep-camp-terrain-route.ppm",
      classic_rts_fog_scouting_intel: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-fog-scouting-intel.json",
      classic_rts_fog_scouting_intel_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-fog-scouting-intel.ppm",
      classic_rts_enemy_base_tech_pressure: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-enemy-base-tech-pressure.json",
      classic_rts_enemy_base_tech_pressure_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-enemy-base-tech-pressure.ppm",
      classic_rts_army_production_rally: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-army-production-rally.json",
      classic_rts_army_production_rally_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-army-production-rally.ppm",
      classic_rts_base_assault_resolution: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-base-assault-resolution.json",
      classic_rts_base_assault_resolution_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-base-assault-resolution.ppm",
      classic_rts_battle_aftermath: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-battle-aftermath.json",
      classic_rts_battle_aftermath_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-battle-aftermath.ppm",
      classic_rts_commander_progression: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-commander-progression.json",
      classic_rts_commander_progression_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-commander-progression.ppm",
      classic_rts_expansion_counterattack: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-expansion-counterattack.json",
      classic_rts_expansion_counterattack_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-expansion-counterattack.ppm",
      classic_rts_tier_two_siege_push: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tier-two-siege-push.json",
      classic_rts_tier_two_siege_push_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tier-two-siege-push.ppm",
      classic_rts_siege_breach_counterplay: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-siege-breach-counterplay.json",
      classic_rts_siege_breach_counterplay_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-siege-breach-counterplay.ppm",
      classic_rts_inner_lane_breakthrough: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-inner-lane-breakthrough.json",
      classic_rts_inner_lane_breakthrough_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-inner-lane-breakthrough.ppm",
      classic_rts_central_keep_pressure: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-pressure.json",
      classic_rts_central_keep_pressure_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-pressure.ppm",
      classic_rts_central_keep_breakthrough: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-breakthrough.json",
      classic_rts_central_keep_breakthrough_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-breakthrough.ppm",
      classic_rts_mirror_city_restoration: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-mirror-city-restoration.json",
      classic_rts_mirror_city_restoration_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-mirror-city-restoration.ppm",
      classic_rts_open_world_after_action: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-open-world-after-action.json",
      classic_rts_open_world_after_action_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-open-world-after-action.ppm",
      classic_rts_campaign_handoff: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-handoff.json",
      classic_rts_campaign_handoff_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-handoff.ppm",
      classic_rts_campaign_entry: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-entry.json",
      playtest_runner_status: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json",
      playtest_launcher: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-launcher.json"
    },
    source_of_truth: "Classic playtest readiness summarizes low-spec trnm-world-bevy evidence only; it does not claim CEX runtime ownership or wgpu/Bevy renderer performance."
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_playtest_readiness_v1"
  and .green == true
  and .checks.manifest_lint_green == true
  and .checks.animation_preview_green == true
  and .checks.animation_selector_green == true
  and .checks.player_motion_green == true
  and .checks.input_frame_budget_green == true
  and .checks.render_budget_green == true
  and .checks.scene_preview_green == true
  and .checks.renderer_probe_green == true
  and .checks.isometric_modeling_green == true
  and .checks.model_catalog_green == true
  and .checks.asset_slot_map_green == true
  and .checks.classic_art_pack_green == true
  and .checks.classic_art_pack_scene_probe_green == true
  and .checks.asset_override_probe_green == true
  and .checks.classic_rts_control_loop_green == true
  and .checks.classic_rts_live_input_sequence_green == true
  and .checks.classic_rts_pathing_formation_green == true
  and .checks.classic_rts_collision_engagement_green == true
  and .checks.classic_rts_target_aggro_focus_green == true
  and .checks.classic_rts_economy_build_green == true
  and .checks.classic_rts_selection_minimap_green == true
  and .checks.classic_rts_build_lifecycle_green == true
  and .checks.classic_rts_tech_tree_green == true
  and .checks.classic_rts_projectile_ability_green == true
  and .checks.classic_rts_ai_skirmish_pressure_green == true
  and .checks.classic_rts_objective_victory_loop_green == true
  and .checks.classic_rts_creep_camp_terrain_route_green == true
  and .checks.classic_rts_fog_scouting_intel_green == true
  and .checks.classic_rts_enemy_base_tech_pressure_green == true
  and .checks.classic_rts_army_production_rally_green == true
  and .checks.classic_rts_base_assault_resolution_green == true
  and .checks.classic_rts_battle_aftermath_green == true
  and .checks.classic_rts_commander_progression_green == true
  and .checks.classic_rts_expansion_counterattack_green == true
  and .checks.classic_rts_tier_two_siege_push_green == true
  and .checks.classic_rts_siege_breach_counterplay_green == true
  and .checks.classic_rts_inner_lane_breakthrough_green == true
  and .checks.classic_rts_central_keep_pressure_green == true
  and .checks.classic_rts_central_keep_breakthrough_green == true
  and .checks.classic_rts_mirror_city_restoration_green == true
  and .checks.classic_rts_open_world_after_action_green == true
  and .checks.classic_rts_campaign_handoff_green == true
  and .checks.classic_rts_campaign_entry_green == true
  and .checks.client_boundary_green == true
  and .checks.playtest_runner_status_green == true
  and .checks.playtest_launcher_green == true
  and .headline.frame_count >= 43
  and .headline.animation_clip_count >= 4
  and .headline.motion_sample_count == 8
  and .headline.motion_accepted_input_count == 8
  and .headline.input_frame_sample_count == 96
  and .headline.input_frame_accepted_input_count == 96
  and .headline.input_frame_p95_micros <= 20000
  and .headline.input_frame_max_micros <= 50000
  and .headline.render_p95_micros <= 16000
  and .headline.render_max_micros <= 40000
  and .headline.isometric_unique_color_count >= 36
  and .headline.isometric_non_background_pixels > 80000
  and .headline.isometric_shadow_pixel_count > 250
  and .headline.isometric_procedural_model_pixel_count > 5000
  and .headline.isometric_canopy_pixel_count > 2500
  and .headline.isometric_procedural_model_pixel_count > 10000
  and .headline.isometric_canopy_pixel_count > 4000
  and .headline.isometric_rts_building_pixel_count > 1500
  and .headline.isometric_rts_model_entity_count >= 3
  and .headline.isometric_terrain_detail_pixel_count > 6000
  and .headline.isometric_terrain_road_pixel_count > 1000
  and .headline.isometric_terrain_water_pixel_count > 300
  and .headline.isometric_terrain_cliff_pixel_count > 1000
  and .headline.isometric_terrain_foundation_pixel_count > 500
  and .headline.isometric_unit_detail_pixel_count > 900
  and .headline.isometric_unit_ring_pixel_count > 250
  and .headline.isometric_unit_health_pixel_count > 90
  and .headline.isometric_unit_silhouette_pixel_count > 500
  and .headline.isometric_rts_neutral_unit_entity_count >= 8
  and .headline.isometric_neutral_unit_detail_pixel_count > 450
  and .headline.isometric_neutral_guard_pixel_count > 70
  and .headline.isometric_neutral_worker_pixel_count > 70
  and .headline.isometric_neutral_creep_pixel_count > 70
  and .headline.isometric_command_feedback_pixel_count > 500
  and .headline.isometric_command_marker_pixel_count > 250
  and .headline.isometric_attack_arc_pixel_count > 100
  and .headline.isometric_hit_flash_pixel_count > 80
  and .headline.isometric_rts_doodad_entity_count >= 12
  and .headline.isometric_doodad_detail_pixel_count > 900
  and .headline.isometric_doodad_stone_pixel_count > 150
  and .headline.isometric_doodad_wood_pixel_count > 150
  and .headline.isometric_doodad_fire_pixel_count > 40
  and .headline.isometric_doodad_crystal_pixel_count > 120
  and .headline.isometric_rts_environment_entity_count >= 12
  and .headline.isometric_environment_detail_pixel_count > 2500
  and .headline.isometric_environment_foliage_pixel_count > 1000
  and .headline.isometric_environment_ruin_pixel_count > 40
  and .headline.isometric_environment_gold_pixel_count > 20
  and .headline.isometric_environment_bridge_pixel_count > 60
  and .headline.asset_slot_count >= 72
  and .headline.asset_slot_category_count >= 8
  and .headline.asset_manifest_frame_slot_count >= 43
  and .headline.asset_procedural_model_slot_count >= 5
  and .headline.asset_doodad_slot_count >= 8
  and .headline.asset_terrain_detail_slot_count >= 4
  and .headline.asset_vfx_slot_count >= 6
  and .headline.asset_neutral_unit_slot_count >= 6
  and .headline.art_pack_asset_count >= 62
  and .headline.art_pack_override_frame_count >= 62
  and .headline.art_pack_preview_height >= 1680
  and .headline.art_pack_preview_non_background_pixels > 35000
  and .headline.art_pack_model_detail_asset_count >= 5
  and .headline.art_pack_model_unique_color_total >= 45
  and .headline.art_pack_model_shadow_pixel_count > 300
  and .headline.art_pack_model_highlight_pixel_count > 120
  and .headline.art_pack_player_unit_detail_asset_count >= 13
  and .headline.art_pack_enemy_unit_detail_asset_count >= 4
  and .headline.art_pack_unit_unique_color_total >= 100
  and .headline.art_pack_unit_shadow_pixel_count > 130
  and .headline.art_pack_unit_highlight_pixel_count > 100
  and .headline.art_pack_neutral_unit_detail_asset_count >= 6
  and .headline.art_pack_neutral_unit_unique_color_total >= 42
  and .headline.art_pack_neutral_unit_shadow_pixel_count > 48
  and .headline.art_pack_neutral_unit_highlight_pixel_count > 24
  and .headline.art_pack_neutral_unit_detail_pixel_count > 360
  and .headline.art_pack_doodad_detail_asset_count >= 8
  and .headline.art_pack_doodad_unique_color_total >= 24
  and .headline.art_pack_doodad_shadow_pixel_count > 40
  and .headline.art_pack_doodad_detail_pixel_count > 420
  and .headline.art_pack_terrain_detail_asset_count >= 11
  and .headline.art_pack_terrain_unique_color_total >= 44
  and .headline.art_pack_terrain_detail_pixel_count > 1350
  and .headline.art_pack_world_prop_detail_asset_count >= 9
  and .headline.art_pack_world_prop_unique_color_total >= 31
  and .headline.art_pack_world_prop_detail_pixel_count > 800
  and .headline.art_pack_vfx_detail_asset_count >= 6
  and .headline.art_pack_vfx_unique_color_total >= 18
  and .headline.art_pack_vfx_detail_pixel_count > 700
  and .headline.art_pack_scene_non_background_pixels > 120000
  and .headline.art_pack_scene_player_color_count > 20
  and .headline.art_pack_scene_enemy_attack_color_count > 20
  and .headline.art_pack_scene_terrain_grass_color_count > 600
  and .headline.art_pack_scene_terrain_road_color_count > 100
  and .headline.art_pack_scene_terrain_water_color_count > 40
  and .headline.art_pack_scene_terrain_wall_roof_color_count > 80
  and .headline.art_pack_scene_world_prop_runtime_color_count > 900
  and .headline.art_pack_scene_neutral_unit_runtime_color_count > 350
  and .headline.art_pack_scene_environment_detail_color_count > 2000
  and .headline.art_pack_scene_command_marker_color_count > 200
  and .headline.art_pack_scene_attack_arc_color_count > 100
  and .headline.art_pack_scene_hit_flash_color_count > 80
  and .headline.asset_override_frame_count >= 1
  and .headline.asset_override_probe_pixel_count > 300
  and .headline.asset_override_non_background_pixels > 300
  and .headline.rts_control_loop_non_background_pixels > 120000
  and .headline.rts_control_loop_move_selected_unit_count >= 4
  and .headline.rts_control_loop_attack_selected_unit_count >= 4
  and .headline.rts_control_loop_selection_marker_pixel_count > 500
  and .headline.rts_control_loop_formation_line_pixel_count > 200
  and .headline.rts_control_loop_command_marker_pixel_count > 600
  and .headline.rts_control_loop_attack_feedback_pixel_count > 180
  and .headline.rts_control_loop_strategy_panel_pixel_count > 4000
  and .headline.rts_control_loop_minimap_pixel_count > 2800
  and .headline.rts_control_loop_fog_pixel_count > 400
  and .headline.rts_control_loop_vision_pixel_count > 120
  and .headline.rts_control_loop_resource_hud_pixel_count > 120
  and .headline.rts_control_loop_production_queue_pixel_count > 900
  and .headline.rts_control_loop_move_training_progress_percent >= 50
  and .headline.rts_control_loop_attack_build_progress_percent >= 50
  and .headline.rts_control_loop_unit_health_card_pixel_count > 280
  and .headline.rts_control_loop_ability_command_pixel_count > 800
  and .headline.rts_control_loop_target_health_pixel_count > 60
  and .headline.rts_control_loop_attack_target_health_percent < 60
  and .headline.rts_live_input_accepted_input_count == 5
  and .headline.rts_live_input_selection_marker_pixel_count > 1000
  and .headline.rts_live_input_command_marker_pixel_count > 600
  and .headline.rts_live_input_attack_feedback_pixel_count > 180
  and .headline.rts_live_input_production_queue_pixel_count > 1000
  and .headline.rts_live_input_ability_command_pixel_count > 800
  and .headline.rts_live_input_target_health_pixel_count > 60
  and .headline.rts_live_input_target_health_percent < 60
  and .headline.rts_pathing_accepted_input_count == 2
  and .headline.rts_pathing_path_tile_count >= 3
  and .headline.rts_pathing_blocked_tile_count >= 1
  and .headline.rts_pathing_formation_slot_count >= 4
  and .headline.rts_pathing_path_tile_pixel_count > 80
  and .headline.rts_pathing_blocked_tile_pixel_count > 40
  and .headline.rts_pathing_formation_slot_pixel_count > 80
  and .headline.rts_pathing_selection_marker_pixel_count > 800
  and .headline.rts_pathing_command_marker_pixel_count > 500
  and .headline.rts_collision_accepted_input_count == 3
  and .headline.rts_collision_move_disperse_tile_count >= 4
  and .headline.rts_collision_engagement_tile_count >= 4
  and .headline.rts_collision_contact_flash_tile_count >= 2
  and .headline.rts_collision_dispersion_slot_pixel_count > 120
  and .headline.rts_collision_engagement_range_pixel_count > 120
  and .headline.rts_collision_contact_flash_pixel_count > 80
  and .headline.rts_collision_blocked_tile_pixel_count > 40
  and .headline.rts_collision_attack_feedback_pixel_count > 180
  and .headline.rts_targeting_accepted_input_count == 4
  and .headline.rts_targeting_priority_count >= 3
  and .headline.rts_targeting_focus_fire_unit_count >= 4
  and .headline.rts_targeting_threat_level_count >= 3
  and .headline.rts_targeting_target_priority_pixel_count > 80
  and .headline.rts_targeting_aggro_pixel_count > 80
  and .headline.rts_targeting_focus_fire_pixel_count > 80
  and .headline.rts_targeting_threat_bar_pixel_count > 40
  and .headline.rts_targeting_attack_feedback_pixel_count > 180
  and .headline.rts_economy_accepted_input_count == 4
  and .headline.rts_economy_harvest_node_count >= 1
  and .headline.rts_economy_worker_assignment_count >= 2
  and .headline.rts_economy_build_site_tile_count >= 3
  and .headline.rts_economy_building_progress_percent >= 42
  and .headline.rts_economy_harvest_node_pixel_count > 80
  and .headline.rts_economy_worker_route_pixel_count > 80
  and .headline.rts_economy_dropoff_pixel_count > 80
  and .headline.rts_economy_build_blueprint_pixel_count > 80
  and .headline.rts_economy_build_progress_pixel_count > 20
  and .headline.rts_economy_production_queue_pixel_count > 1000
  and .headline.rts_selection_minimap_accepted_input_count == 4
  and .headline.rts_selection_box_tile_count >= 4
  and .headline.rts_control_group_assignment_count >= 2
  and .headline.rts_active_control_group_count >= 2
  and .headline.rts_minimap_command_tile_id == "9,2"
  and .headline.rts_split_route_tile_count >= 4
  and .headline.rts_selection_minimap_pixel_count > 380
  and .headline.rts_selection_box_pixel_count > 160
  and .headline.rts_minimap_command_pixel_count > 80
  and .headline.rts_group_two_pixel_count > 20
  and .headline.rts_split_route_pixel_count > 120
  and .headline.rts_build_lifecycle_accepted_input_count == 6
  and .headline.rts_build_lifecycle_completed_structure_count >= 1
  and .headline.rts_build_lifecycle_cancelled_structure_count >= 1
  and .headline.rts_build_lifecycle_repair_progress_percent >= 76
  and .headline.rts_build_lifecycle_refund_count >= 2
  and .headline.rts_build_lifecycle_pixel_count > 200
  and .headline.rts_build_lifecycle_structure_complete_pixel_count > 80
  and .headline.rts_build_lifecycle_structure_health_pixel_count > 20
  and .headline.rts_build_lifecycle_repair_pixel_count > 60
  and .headline.rts_build_lifecycle_cancel_refund_pixel_count > 40
  and .headline.rts_tech_tree_accepted_input_count == 6
  and .headline.rts_tech_tree_faction_id == "mirror_guard"
  and .headline.rts_tech_tree_base_structure_count >= 3
  and .headline.rts_tech_tree_research_count >= 1
  and .headline.rts_tech_tree_completed_upgrade_count >= 1
  and .headline.rts_tech_tree_unlocked_unit_count >= 3
  and .headline.rts_tech_tree_unlocked_structure_count >= 1
  and .headline.rts_tech_tree_requirement_count >= 4
  and .headline.rts_tech_tree_progress_percent == 100
  and .headline.rts_tech_tree_pixel_count > 360
  and .headline.rts_tech_tree_base_pixel_count > 140
  and .headline.rts_tech_tree_research_pixel_count > 50
  and .headline.rts_tech_tree_upgrade_pixel_count > 40
  and .headline.rts_tech_tree_unlock_pixel_count > 70
  and .headline.rts_tech_tree_requirement_pixel_count > 60
  and .headline.rts_projectile_ability_accepted_input_count == 5
  and .headline.rts_projectile_ability_active_projectile_id == "guard_break_bolt"
  and .headline.rts_projectile_ability_trail_tile_count >= 4
  and .headline.rts_projectile_ability_effect_tile_count >= 4
  and .headline.rts_projectile_ability_damage_tick_count >= 3
  and .headline.rts_projectile_ability_damage_total >= 72
  and .headline.rts_projectile_ability_target_health_percent <= 18
  and .headline.rts_projectile_ability_target_armor_percent == 18
  and .headline.rts_projectile_ability_target_shield_percent == 0
  and .headline.rts_projectile_ability_pixel_count > 360
  and .headline.rts_projectile_ability_trail_pixel_count > 80
  and .headline.rts_projectile_ability_impact_pixel_count > 80
  and .headline.rts_projectile_ability_radius_pixel_count > 140
  and .headline.rts_projectile_ability_damage_tick_pixel_count > 40
  and .headline.rts_projectile_ability_armor_shield_pixel_count > 20
  and .headline.rts_ai_skirmish_accepted_input_count == 5
  and .headline.rts_ai_skirmish_wave_unit_count >= 3
  and .headline.rts_ai_skirmish_pressure_tile_count >= 4
  and .headline.rts_ai_skirmish_counter_tile_count >= 4
  and .headline.rts_ai_skirmish_retreat_tile_id == "9,2"
  and .headline.rts_ai_skirmish_pressure_percent <= 34
  and .headline.rts_ai_skirmish_state == "countered:guard_break:skirmish_wave"
  and .headline.rts_ai_skirmish_pressure_pixel_count > 420
  and .headline.rts_ai_skirmish_wave_pixel_count > 80
  and .headline.rts_ai_skirmish_lane_pixel_count > 120
  and .headline.rts_ai_skirmish_counter_pixel_count > 80
  and .headline.rts_ai_skirmish_retreat_pixel_count > 40
  and .headline.rts_ai_skirmish_pressure_bar_pixel_count > 20
  and .headline.rts_objective_victory_loop_accepted_input_count == 6
  and .headline.rts_objective_victory_loop_tile_count >= 3
  and .headline.rts_objective_victory_loop_capture_percent == 100
  and .headline.rts_objective_victory_loop_owner_state == "player:relay_beacon"
  and .headline.rts_objective_victory_loop_result_state == "victory:relay_beacon_extracted"
  and .headline.rts_objective_victory_loop_extraction_tile_id == "9,2"
  and .headline.rts_objective_victory_loop_defeat_risk_percent <= 8
  and .headline.rts_objective_victory_loop_ai_pressure_percent <= 34
  and .headline.rts_objective_victory_loop_pixel_count > 180
  and .headline.rts_objective_victory_loop_objective_pixel_count > 80
  and .headline.rts_objective_victory_loop_capture_bar_pixel_count > 20
  and .headline.rts_objective_victory_loop_victory_pixel_count > 20
  and .headline.rts_objective_victory_loop_defeat_risk_pixel_count > 5
  and .headline.rts_objective_victory_loop_extraction_pixel_count > 40
  and .headline.rts_creep_camp_terrain_route_accepted_input_count == 6
  and .headline.rts_creep_camp_terrain_route_camp_tile_count >= 4
  and .headline.rts_creep_camp_terrain_route_unit_count >= 3
  and .headline.rts_creep_camp_terrain_route_state == "cleared:forest_creep_camp"
  and .headline.rts_creep_camp_terrain_route_route_tile_count >= 4
  and .headline.rts_creep_camp_terrain_route_choke_tile_count >= 3
  and .headline.rts_creep_camp_terrain_route_expansion_tile_count >= 3
  and .headline.rts_creep_camp_terrain_route_scout_reveal_percent == 100
  and .headline.rts_creep_camp_terrain_route_target_health_percent <= 18
  and .headline.rts_creep_camp_terrain_route_pixel_count > 300
  and .headline.rts_creep_camp_terrain_route_camp_pixel_count > 100
  and .headline.rts_creep_camp_terrain_route_route_pixel_count > 80
  and .headline.rts_creep_camp_terrain_route_choke_pixel_count > 40
  and .headline.rts_creep_camp_terrain_route_expansion_pixel_count > 50
  and .headline.rts_creep_camp_terrain_route_reveal_pixel_count > 20
  and .headline.rts_fog_scouting_intel_accepted_input_count == 6
  and .headline.rts_fog_scouting_intel_scout_unit_count >= 2
  and .headline.rts_fog_scouting_intel_scout_route_tile_count >= 5
  and .headline.rts_fog_scouting_intel_fog_reveal_tile_count >= 8
  and .headline.rts_fog_scouting_intel_enemy_structure_count >= 3
  and .headline.rts_fog_scouting_intel_enemy_unit_count >= 3
  and .headline.rts_fog_scouting_intel_visibility_percent == 100
  and .headline.rts_fog_scouting_intel_pixel_count > 300
  and .headline.rts_fog_scouting_intel_scout_route_pixel_count > 80
  and .headline.rts_fog_scouting_intel_fog_reveal_pixel_count > 80
  and .headline.rts_fog_scouting_intel_enemy_structure_pixel_count > 80
  and .headline.rts_fog_scouting_intel_enemy_unit_pixel_count > 60
  and .headline.rts_fog_scouting_intel_visibility_bar_pixel_count > 20
  and .headline.rts_enemy_base_tech_pressure_accepted_input_count == 6
  and .headline.rts_enemy_base_tech_pressure_enemy_tech_count >= 2
  and .headline.rts_enemy_base_tech_pressure_enemy_production_count >= 2
  and .headline.rts_enemy_base_tech_pressure_wave_unit_count >= 3
  and .headline.rts_enemy_base_tech_pressure_player_counter_count >= 2
  and .headline.rts_enemy_base_tech_pressure_defense_structure_count >= 2
  and .headline.rts_enemy_base_tech_pressure_warning_percent <= 48
  and .headline.rts_enemy_base_tech_pressure_state == "counter_ready:enemy_base"
  and .headline.rts_enemy_base_tech_pressure_pixel_count > 300
  and .headline.rts_enemy_base_tech_pressure_enemy_tech_pixel_count > 80
  and .headline.rts_enemy_base_tech_pressure_enemy_production_pixel_count > 80
  and .headline.rts_enemy_base_tech_pressure_player_counter_pixel_count > 50
  and .headline.rts_enemy_base_tech_pressure_defense_ready_pixel_count > 80
  and .headline.rts_enemy_base_tech_pressure_warning_pixel_count > 20
  and .headline.rts_army_production_rally_accepted_input_count == 6
  and .headline.rts_army_production_rally_supply_cap >= 18
  and .headline.rts_army_production_rally_supply_used >= 10
  and .headline.rts_army_production_rally_supply_used <= .headline.rts_army_production_rally_supply_cap
  and .headline.rts_army_production_rally_batch_count >= 2
  and .headline.rts_army_production_rally_spawned_unit_count >= 4
  and .headline.rts_army_production_rally_rally_tile_count >= 5
  and .headline.rts_army_production_rally_composition_log_count >= 5
  and .headline.rts_army_production_rally_state == "assigned:control_group_3:group_3"
  and .headline.rts_army_production_rally_training_progress_percent == 100
  and .headline.rts_army_production_rally_pixel_count > 340
  and .headline.rts_army_production_rally_supply_pixel_count > 20
  and .headline.rts_army_production_rally_spawned_unit_pixel_count > 160
  and .headline.rts_army_production_rally_rally_line_pixel_count > 80
  and .headline.rts_army_production_rally_composition_pixel_count > 80
  and .headline.rts_base_assault_resolution_accepted_input_count == 9
  and .headline.rts_base_assault_resolution_army_spawned_unit_count >= 4
  and .headline.rts_base_assault_resolution_target_count >= 3
  and .headline.rts_base_assault_resolution_path_tile_count >= 6
  and .headline.rts_base_assault_resolution_min_enemy_structure_health <= 18
  and .headline.rts_base_assault_resolution_breach_percent == 100
  and .headline.rts_base_assault_resolution_state == "breached:enemy_barracks"
  and .headline.rts_base_assault_resolution_reward_count >= 2
  and .headline.rts_base_assault_resolution_pixel_count > 260
  and .headline.rts_base_assault_resolution_assault_path_pixel_count > 120
  and .headline.rts_base_assault_resolution_breach_pixel_count > 80
  and .headline.rts_base_assault_resolution_enemy_base_health_pixel_count > 40
  and .headline.rts_base_assault_resolution_assault_reward_pixel_count > 8
  and .headline.rts_battle_aftermath_accepted_input_count == 12
  and .headline.rts_battle_aftermath_destroyed_structure_count >= 1
  and .headline.rts_battle_aftermath_debris_tile_count >= 4
  and .headline.rts_battle_aftermath_smoke_tile_count >= 3
  and .headline.rts_battle_aftermath_veteran_unit_count >= 3
  and .headline.rts_battle_aftermath_veteran_log_count >= 3
  and .headline.rts_battle_aftermath_growth_level >= 2
  and .headline.rts_battle_aftermath_match_result_state == "victory_ready:secure_expansion"
  and .headline.rts_battle_aftermath_next_action_count >= 3
  and .headline.rts_battle_aftermath_next_extraction_tile == "9,2"
  and .headline.rts_battle_aftermath_pixel_count > 240
  and .headline.rts_battle_aftermath_debris_pixel_count > 100
  and .headline.rts_battle_aftermath_smoke_pixel_count > 60
  and .headline.rts_battle_aftermath_veteran_pixel_count > 40
  and .headline.rts_battle_aftermath_match_result_pixel_count > 20
  and .headline.rts_battle_aftermath_next_action_pixel_count > 20
  and .headline.rts_commander_progression_accepted_input_count == 15
  and .headline.rts_commander_progression_unit_id == "mirror_captain"
  and .headline.rts_commander_progression_level >= 3
  and .headline.rts_commander_progression_ability_point_count == 0
  and .headline.rts_commander_progression_aura_tile_count >= 5
  and .headline.rts_commander_progression_ability_log_count >= 2
  and .headline.rts_commander_progression_loot_count >= 3
  and .headline.rts_commander_progression_pickup_count >= 3
  and .headline.rts_commander_progression_active_ability == "rally_aura"
  and .headline.rts_commander_progression_pixel_count > 180
  and .headline.rts_commander_progression_commander_pixel_count > 40
  and .headline.rts_commander_progression_aura_pixel_count > 80
  and .headline.rts_commander_progression_loot_pixel_count > 40
  and .headline.rts_commander_progression_ability_point_pixel_count > 20
  and .gates.cex_runtime_player_client_allowed == false
  and .gates.wgpu_required == false
  and .gates.manifest_boundary_gate == true
  and .gates.animation_action_coverage_gate == true
  and .gates.selector_transition_gate == true
  and .gates.motion_direction_coverage_gate == true
  and .gates.input_frame_direction_coverage_gate == true
  and .gates.input_frame_p95_budget_gate == true
  and .gates.input_frame_max_budget_gate == true
  and .gates.render_p95_budget_gate == true
  and .gates.render_max_budget_gate == true
  and .gates.scene_dynamic_landmark_animation_gate == true
  and .gates.renderer_probe_scene_frame_gate == true
  and .gates.isometric_projection_gate == true
  and .gates.isometric_depth_sort_gate == true
  and .gates.isometric_diamond_tile_gate == true
  and .gates.isometric_shadow_anchor_gate == true
  and .gates.isometric_procedural_volume_gate == true
  and .gates.isometric_rts_model_set_gate == true
  and .gates.isometric_terrain_detail_gate == true
  and .gates.isometric_unit_detail_gate == true
  and .gates.isometric_neutral_unit_detail_gate == true
  and .gates.isometric_command_feedback_gate == true
  and .gates.isometric_doodad_detail_gate == true
  and .gates.isometric_environment_detail_gate == true
  and .gates.isometric_sprite_anchor_gate == true
  and .gates.catalog_all_frames_rendered_gate == true
  and .gates.asset_slot_required_categories_gate == true
  and .gates.asset_slot_manifest_frame_slots_gate == true
  and .gates.asset_slot_procedural_slots_gate == true
  and .gates.asset_slot_replacement_boundary_gate == true
  and .gates.art_pack_required_model_gate == true
  and .gates.art_pack_player_art_gate == true
  and .gates.art_pack_enemy_art_gate == true
  and .gates.art_pack_neutral_unit_art_gate == true
  and .gates.art_pack_doodad_art_gate == true
  and .gates.art_pack_terrain_art_gate == true
  and .gates.art_pack_world_prop_art_gate == true
  and .gates.art_pack_vfx_art_gate == true
  and .gates.art_pack_model_detail_gate == true
  and .gates.art_pack_unit_detail_gate == true
  and .gates.art_pack_neutral_unit_detail_gate == true
  and .gates.art_pack_doodad_detail_gate == true
  and .gates.art_pack_terrain_detail_gate == true
  and .gates.art_pack_world_prop_detail_gate == true
  and .gates.art_pack_vfx_detail_gate == true
  and .gates.art_pack_replacement_boundary_gate == true
  and .gates.art_pack_scene_override_presence_gate == true
  and .gates.art_pack_scene_color_probe_gate == true
  and .gates.art_pack_scene_terrain_override_presence_gate == true
  and .gates.art_pack_scene_terrain_color_probe_gate == true
  and .gates.art_pack_scene_world_prop_override_presence_gate == true
  and .gates.art_pack_scene_world_prop_color_probe_gate == true
  and .gates.art_pack_scene_neutral_unit_override_presence_gate == true
  and .gates.art_pack_scene_neutral_unit_color_probe_gate == true
  and .gates.art_pack_scene_environment_override_presence_gate == true
  and .gates.art_pack_scene_environment_detail_color_probe_gate == true
  and .gates.art_pack_scene_vfx_override_presence_gate == true
  and .gates.art_pack_scene_vfx_color_probe_gate == true
  and .gates.art_pack_scene_replacement_boundary_gate == true
  and .gates.asset_override_frame_gate == true
  and .gates.asset_override_replacement_boundary_gate == true
  and .gates.rts_control_loop_selection_gate == true
  and .gates.rts_control_loop_command_queue_gate == true
  and .gates.rts_control_loop_strategy_hud_gate == true
  and .gates.rts_control_loop_macro_loop_gate == true
  and .gates.rts_control_loop_tactical_combat_gate == true
  and .gates.rts_control_loop_gameplay_surface_gate == true
  and .gates.rts_live_input_live_input_gate == true
  and .gates.rts_live_input_selection_live_gate == true
  and .gates.rts_live_input_production_live_gate == true
  and .gates.rts_live_input_move_live_gate == true
  and .gates.rts_live_input_attack_live_gate == true
  and .gates.rts_live_input_ability_live_gate == true
  and .gates.rts_pathing_live_input_gate == true
  and .gates.rts_pathing_path_tile_gate == true
  and .gates.rts_pathing_blocked_tile_gate == true
  and .gates.rts_pathing_formation_slot_gate == true
  and .gates.rts_pathing_command_visual_gate == true
  and .gates.rts_collision_live_input_gate == true
  and .gates.rts_collision_collision_response_gate == true
  and .gates.rts_collision_engagement_response_gate == true
  and .gates.rts_targeting_live_input_gate == true
  and .gates.rts_targeting_target_priority_gate == true
  and .gates.rts_targeting_aggro_gate == true
  and .gates.rts_targeting_focus_fire_gate == true
  and .gates.rts_targeting_threat_feedback_gate == true
  and .gates.rts_economy_live_input_gate == true
  and .gates.rts_economy_harvest_loop_gate == true
  and .gates.rts_economy_build_loop_gate == true
  and .gates.rts_economy_production_loop_gate == true
  and .gates.rts_selection_minimap_live_input_gate == true
  and .gates.rts_selection_box_gate == true
  and .gates.rts_control_group_gate == true
  and .gates.rts_minimap_command_gate == true
  and .gates.rts_split_route_gate == true
  and .gates.rts_build_lifecycle_live_input_gate == true
  and .gates.rts_build_lifecycle_build_placement_gate == true
  and .gates.rts_build_lifecycle_completion_gate == true
  and .gates.rts_build_lifecycle_repair_gate == true
  and .gates.rts_build_lifecycle_cancel_refund_gate == true
  and .gates.rts_tech_tree_live_input_gate == true
  and .gates.rts_tech_tree_faction_base_gate == true
  and .gates.rts_tech_tree_research_gate == true
  and .gates.rts_tech_tree_upgrade_gate == true
  and .gates.rts_tech_tree_unlock_gate == true
  and .gates.rts_tech_tree_dependency_gate == true
  and .gates.rts_projectile_ability_live_input_gate == true
  and .gates.rts_projectile_ability_projectile_trail_gate == true
  and .gates.rts_projectile_ability_projectile_impact_gate == true
  and .gates.rts_projectile_ability_ability_radius_gate == true
  and .gates.rts_projectile_ability_damage_tick_gate == true
  and .gates.rts_projectile_ability_armor_shield_gate == true
  and .gates.rts_ai_skirmish_pressure_live_input_gate == true
  and .gates.rts_ai_skirmish_pressure_ai_wave_gate == true
  and .gates.rts_ai_skirmish_pressure_ai_counter_gate == true
  and .gates.rts_ai_skirmish_pressure_resolution_gate == true
  and .gates.rts_ai_skirmish_pressure_retreat_gate == true
  and .gates.rts_ai_skirmish_player_response_gate == true
  and .gates.rts_objective_victory_loop_live_input_gate == true
  and .gates.rts_objective_victory_loop_marker_gate == true
  and .gates.rts_objective_victory_loop_capture_gate == true
  and .gates.rts_objective_victory_loop_victory_gate == true
  and .gates.rts_objective_victory_loop_defeat_pressure_gate == true
  and .gates.rts_objective_victory_loop_extraction_gate == true
  and .gates.rts_creep_camp_terrain_route_live_input_gate == true
  and .gates.rts_creep_camp_terrain_route_terrain_gate == true
  and .gates.rts_creep_camp_terrain_route_choke_gate == true
  and .gates.rts_creep_camp_terrain_route_clear_gate == true
  and .gates.rts_creep_camp_terrain_route_reveal_gate == true
  and .gates.rts_creep_camp_terrain_route_expansion_gate == true
  and .gates.rts_fog_scouting_intel_live_input_gate == true
  and .gates.rts_fog_scouting_intel_scout_route_gate == true
  and .gates.rts_fog_scouting_intel_fog_reveal_gate == true
  and .gates.rts_fog_scouting_intel_enemy_structure_gate == true
  and .gates.rts_fog_scouting_intel_enemy_unit_gate == true
  and .gates.rts_fog_scouting_intel_intel_log_gate == true
  and .gates.rts_fog_scouting_intel_visibility_gate == true
  and .gates.rts_enemy_base_tech_pressure_live_input_gate == true
  and .gates.rts_enemy_base_tech_pressure_intel_dependency_gate == true
  and .gates.rts_enemy_base_tech_pressure_enemy_tech_gate == true
  and .gates.rts_enemy_base_tech_pressure_enemy_production_gate == true
  and .gates.rts_enemy_base_tech_pressure_player_counter_gate == true
  and .gates.rts_enemy_base_tech_pressure_defense_ready_gate == true
  and .gates.rts_enemy_base_tech_pressure_warning_gate == true
  and .gates.rts_army_production_rally_live_input_gate == true
  and .gates.rts_army_production_rally_supply_gate == true
  and .gates.rts_army_production_rally_production_batch_gate == true
  and .gates.rts_army_production_rally_rally_gate == true
  and .gates.rts_army_production_rally_control_group_gate == true
  and .gates.rts_army_production_rally_composition_gate == true
  and .gates.rts_base_assault_resolution_live_input_gate == true
  and .gates.rts_base_assault_resolution_army_dependency_gate == true
  and .gates.rts_base_assault_resolution_assault_path_gate == true
  and .gates.rts_base_assault_resolution_enemy_base_health_gate == true
  and .gates.rts_base_assault_resolution_breach_gate == true
  and .gates.rts_base_assault_resolution_reward_gate == true
  and .gates.rts_battle_aftermath_live_input_gate == true
  and .gates.rts_battle_aftermath_assault_dependency_gate == true
  and .gates.rts_battle_aftermath_destruction_gate == true
  and .gates.rts_battle_aftermath_veteran_gate == true
  and .gates.rts_battle_aftermath_match_result_gate == true
  and .gates.rts_battle_aftermath_next_action_gate == true
  and .gates.rts_battle_aftermath_reward_gate == true
  and .gates.rts_commander_progression_live_input_gate == true
  and .gates.rts_commander_progression_aftermath_dependency_gate == true
  and .gates.rts_commander_progression_loot_gate == true
  and .gates.rts_commander_progression_level_gate == true
  and .gates.rts_commander_progression_ability_point_gate == true
  and .gates.rts_commander_progression_aura_gate == true
  and .gates.rts_expansion_counterattack_live_input_gate == true
  and .gates.rts_expansion_counterattack_commander_dependency_gate == true
  and .gates.rts_expansion_counterattack_claim_gate == true
  and .gates.rts_expansion_counterattack_build_gate == true
  and .gates.rts_expansion_counterattack_worker_income_gate == true
  and .gates.rts_expansion_counterattack_counterattack_gate == true
  and .gates.rts_expansion_counterattack_defense_gate == true
  and .gates.rts_tier_two_siege_push_live_input_gate == true
  and .gates.rts_tier_two_siege_push_expansion_dependency_gate == true
  and .gates.rts_tier_two_siege_push_tech_gate == true
  and .gates.rts_tier_two_siege_push_upgrade_gate == true
  and .gates.rts_tier_two_siege_push_unit_gate == true
  and .gates.rts_tier_two_siege_push_enemy_fortification_gate == true
  and .gates.rts_tier_two_siege_push_push_gate == true
  and .gates.rts_siege_breach_counterplay_live_input_gate == true
  and .gates.rts_siege_breach_counterplay_tier_two_dependency_gate == true
  and .gates.rts_siege_breach_counterplay_breach_window_gate == true
  and .gates.rts_siege_breach_counterplay_repair_reaction_gate == true
  and .gates.rts_siege_breach_counterplay_flank_pressure_gate == true
  and .gates.rts_siege_breach_counterplay_hold_line_gate == true
  and .gates.rts_siege_breach_counterplay_resolution_gate == true
  and .gates.rts_inner_lane_breakthrough_live_input_gate == true
  and .gates.rts_inner_lane_breakthrough_siege_breach_dependency_gate == true
  and .gates.rts_inner_lane_breakthrough_route_gate == true
  and .gates.rts_inner_lane_breakthrough_gate_gate == true
  and .gates.rts_inner_lane_breakthrough_supply_gate == true
  and .gates.rts_inner_lane_breakthrough_split_gate == true
  and .gates.rts_inner_lane_breakthrough_clear_gate == true
  and .gates.rts_inner_lane_breakthrough_secure_gate == true
  and .gates.rts_central_keep_pressure_live_input_gate == true
  and .gates.rts_central_keep_pressure_inner_lane_dependency_gate == true
  and .gates.rts_central_keep_pressure_route_gate == true
  and .gates.rts_central_keep_pressure_shield_gate == true
  and .gates.rts_central_keep_pressure_guard_gate == true
  and .gates.rts_central_keep_pressure_siege_line_gate == true
  and .gates.rts_central_keep_pressure_pressure_gate == true
  and .gates.rts_central_keep_breakthrough_live_input_gate == true
  and .gates.rts_central_keep_breakthrough_pressure_dependency_gate == true
  and .gates.rts_central_keep_breakthrough_breach_gate == true
  and .gates.rts_central_keep_breakthrough_guardian_counter_gate == true
  and .gates.rts_central_keep_breakthrough_hold_gate == true
  and .gates.rts_central_keep_breakthrough_break_gate == true
  and .gates.rts_central_keep_breakthrough_claim_gate == true
  and .gates.rts_mirror_city_restoration_live_input_gate == true
  and .gates.rts_mirror_city_restoration_victory_dependency_gate == true
  and .gates.rts_mirror_city_restoration_restore_gate == true
  and .gates.rts_mirror_city_restoration_rebuild_gate == true
  and .gates.rts_mirror_city_restoration_garrison_gate == true
  and .gates.rts_mirror_city_restoration_handoff_gate == true
  and .gates.rts_open_world_after_action_live_input_gate == true
  and .gates.rts_open_world_after_action_restoration_dependency_gate == true
  and .gates.rts_open_world_after_action_route_gate == true
  and .gates.rts_open_world_after_action_panel_gate == true
  and .gates.rts_open_world_after_action_resume_gate == true
  and .gates.rts_open_world_after_action_command_gate == true
  and .gates.rts_campaign_handoff_live_input_gate == true
  and .gates.rts_campaign_handoff_early_campaign_gate == true
  and .gates.rts_campaign_handoff_mid_campaign_gate == true
  and .gates.rts_campaign_handoff_end_campaign_gate == true
  and .gates.rts_campaign_handoff_open_world_resume_gate == true
  and .gates.rts_campaign_handoff_snapshot_round_trip_gate == true
  and .gates.rts_campaign_handoff_render_milestone_gate == true
  and .gates.rts_campaign_entry_title_entry_gate == true
  and .gates.rts_campaign_entry_start_gate == true
  and .gates.rts_campaign_entry_slot_snapshot_gate == true
  and .gates.rts_campaign_entry_continue_gate == true
  and .gates.rts_campaign_entry_continue_unlock_gate == true
  and .gates.rts_campaign_entry_replay_gate == true
  and .gates.runner_service_process_gate == true
  and .gates.runner_release_binary_gate == true
  and .gates.runner_classic_env_gate == true
  and .gates.runner_override_dir_gate == true
  and .gates.runner_cex_path_gate == true
  and .gates.launcher_player_launch_ready_gate == true
  and .gates.launcher_campaign_entry_gate == true
  and .gates.launcher_campaign_slot_gate == true
  and .gates.launcher_open_world_resume_gate == true
  and .gates.launcher_player_command_gate == true
  and .gates.launcher_service_process_gate == true
  and .gates.launcher_release_binary_gate == true
  and .gates.launcher_cex_path_gate == true
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYTEST_READINESS_GREEN %s\n' "$SUMMARY"
