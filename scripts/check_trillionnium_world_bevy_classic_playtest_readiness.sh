#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-readiness.json"
REFRESH="${TRNM_BEVY_PLAYTEST_READINESS_REFRESH:-1}"

for arg in "$@"; do
  case "$arg" in
    --refresh)
      REFRESH=1
      ;;
    --no-refresh)
      REFRESH=0
      ;;
    *)
      printf 'unknown option: %s\n' "$arg" >&2
      exit 2
      ;;
  esac
done

mkdir -p "$(dirname "$SUMMARY")"
SUMMARY_FILTER="$(mktemp)"
VALIDATION_FILTER="$(mktemp)"
VALIDATION_CHUNK_DIR="$(mktemp -d)"
SUMMARY_WITH_COUNTS="$(mktemp)"
trap 'rm -f "$SUMMARY_FILTER" "$VALIDATION_FILTER" "$SUMMARY_WITH_COUNTS"; rm -rf "$VALIDATION_CHUNK_DIR"' EXIT
sed -n '/^# BEGIN_PLAYTEST_READINESS_SUMMARY_FILTER$/,/^# END_PLAYTEST_READINESS_SUMMARY_FILTER$/p' "$0" | sed '1d;$d' >"$SUMMARY_FILTER"
sed -n '/^# BEGIN_PLAYTEST_READINESS_VALIDATION_FILTER$/,/^# END_PLAYTEST_READINESS_VALIDATION_FILTER$/p' "$0" | sed '1d;$d' >"$VALIDATION_FILTER"

run_validation_filter_in_chunks() {
  local filter="$1"
  local json="$2"
  local chunk_dir="$3"
  awk -v dir="$chunk_dir" -v max_lines=160 '
    function open_chunk() {
      chunk += 1
      file = sprintf("%s/validation-%03d.jq", dir, chunk)
      line_count = 0
      first = 1
    }
    /^[[:space:]]*$/ { next }
    {
      line = $0
      if (chunk == 0 || line_count >= max_lines) {
        open_chunk()
      }
      if (first) {
        sub(/^[[:space:]]*and[[:space:]]+/, "  ", line)
        print line > file
        first = 0
      } else {
        if (line !~ /^[[:space:]]*and[[:space:]]+/) {
          print "  and " line > file
        } else {
          print line > file
        }
      }
      line_count += 1
    }
  ' "$filter"

  local chunk
  for chunk in "$chunk_dir"/validation-*.jq; do
    jq -e -f "$chunk" "$json" >/dev/null
  done
}

if [[ "$REFRESH" != "0" ]]; then
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
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_autonomous_bot_skirmish.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_organic_terminal_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_terminal_observation_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_replay_metrics_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_endurance_skirmish_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_decision_state_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_map_intel_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_macro_economy_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_harassment_defense_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_multi_front_pressure_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_expansion_control_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_tech_transition_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_army_composition_gap.sh" >/dev/null
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
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_visual_fidelity.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_affordance.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_surface.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_structure_modeling.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_environment_life.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_map_model_gap.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_worker_harvest_animation.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_spawn_animation.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_unit_status_portrait.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_selection_command_feedback.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_scrollable_map.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_camera_minimap_sync.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_command_queue_path_preview.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_formation_move_preview.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_formation_move_execution.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_local_obstruction_recovery.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_action_cadence.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_unit_model_depth.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_action_sequence.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_npc_behavior.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_combat_impact.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_locomotion_blend.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_npc_transition.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_depth_readability.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_first_minute_readiness.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_first_contact_basin_spec.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_art_replication.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_asset_atlas.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_ui_skin.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_interaction_polish.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_full_screen_ui_replication.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_shell_meta_ui_replication.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_match_setup_ui_replication.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_ui_continuity.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_in_match_hud_state_replication.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_session_state_continuity.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_continuous_player_flow.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_live_session_playthrough.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_screen_for_screen_ui_replication.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_engine_port_asset_parity.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_playtest_observability_readiness.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_client_boundary.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_launcher.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_runner_status.sh" >/dev/null
fi

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
  --slurpfile rts_auto_bot "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-autonomous-bot-skirmish.json" \
  --slurpfile rts_organic_terminal_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-organic-terminal-gap.json" \
  --slurpfile rts_terminal_observation_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-terminal-observation-gap.json" \
  --slurpfile rts_replay_metrics_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-replay-metrics-gap.json" \
  --slurpfile rts_endurance_skirmish_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-endurance-skirmish-gap.json" \
  --slurpfile rts_bot_decision_state_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-decision-state-gap.json" \
  --slurpfile rts_bot_adaptive_build_order_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-adaptive-build-order-gap.json" \
  --slurpfile rts_bot_tactical_micro_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tactical-micro-gap.json" \
  --slurpfile rts_bot_map_intel_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-map-intel-gap.json" \
  --slurpfile rts_bot_macro_economy_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-macro-economy-gap.json" \
  --slurpfile rts_bot_harassment_defense_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-harassment-defense-gap.json" \
  --slurpfile rts_bot_multi_front_pressure_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-multi-front-pressure-gap.json" \
  --slurpfile rts_bot_expansion_control_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-expansion-control-gap.json" \
  --slurpfile rts_bot_tech_transition_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tech-transition-gap.json" \
  --slurpfile rts_bot_army_composition_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-army-composition-gap.json" \
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
  --slurpfile rts_visual_fidelity "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-visual-fidelity.json" \
  --slurpfile rts_command_affordance "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-affordance.json" \
  --slurpfile rts_command_surface "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-surface.json" \
  --slurpfile rts_structure_modeling "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-structure-modeling.json" \
  --slurpfile rts_environment_life "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-environment-life.json" \
  --slurpfile rts_map_model_gap "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-model-gap.json" \
  --slurpfile rts_worker_harvest_animation "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-worker-harvest-animation.json" \
  --slurpfile rts_production_spawn_animation "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-spawn-animation.json" \
  --slurpfile rts_unit_status_portrait "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-status-portrait.json" \
  --slurpfile rts_selection_command_feedback "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-command-feedback.json" \
  --slurpfile rts_ability_tooltip_telegraph "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ability-tooltip-telegraph.json" \
  --slurpfile rts_control_group_hotkey_feedback "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-hotkey-feedback.json" \
  --slurpfile rts_scrollable_map "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-scrollable-map.json" \
  --slurpfile rts_camera_minimap_sync "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-camera-minimap-sync.json" \
  --slurpfile rts_command_queue_path_preview "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-queue-path-preview.json" \
  --slurpfile rts_formation_move_preview "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-preview.json" \
  --slurpfile rts_formation_move_execution "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-execution.json" \
  --slurpfile rts_local_obstruction_recovery "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-local-obstruction-recovery.json" \
  --slurpfile rts_action_cadence "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-cadence.json" \
  --slurpfile rts_unit_model_depth "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-model-depth.json" \
  --slurpfile rts_action_sequence "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-sequence.json" \
  --slurpfile rts_npc_behavior "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-behavior.json" \
  --slurpfile rts_combat_impact "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-impact.json" \
  --slurpfile rts_locomotion_blend "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-locomotion-blend.json" \
  --slurpfile rts_npc_transition "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-transition.json" \
  --slurpfile rts_depth_readability "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-depth-readability.json" \
  --slurpfile rts_first_minute_readiness "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-minute-readiness.json" \
  --slurpfile rts_map_ui_modeling_readiness "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-ui-modeling-readiness.json" \
  --slurpfile rts_first_contact_basin_spec "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json" \
  --slurpfile rts_production_art_replication "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-art-replication.json" \
  --slurpfile rts_production_asset_atlas "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-asset-atlas.json" \
  --slurpfile rts_production_ui_skin "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-ui-skin.json" \
  --slurpfile rts_production_interaction_polish "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-interaction-polish.json" \
  --slurpfile rts_full_screen_ui_replication "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-screen-ui-replication.json" \
  --slurpfile rts_shell_meta_ui_replication "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-shell-meta-ui-replication.json" \
  --slurpfile rts_match_setup_ui_replication "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-match-setup-ui-replication.json" \
  --slurpfile rts_campaign_outcome_ui_readiness "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-outcome-ui-readiness.json" \
  --slurpfile rts_campaign_ui_continuity "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-ui-continuity.json" \
  --slurpfile rts_in_match_hud_state_replication "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-in-match-hud-state-replication.json" \
  --slurpfile rts_session_state_continuity "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-session-state-continuity.json" \
  --slurpfile rts_continuous_player_flow "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-continuous-player-flow.json" \
  --slurpfile rts_live_session_playthrough "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-session-playthrough.json" \
  --slurpfile rts_full_game_visual_ui_replication "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-game-visual-ui-replication.json" \
  --slurpfile rts_openra_screen_for_screen_ui_replication "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-screen-for-screen-ui-replication.json" \
  --slurpfile rts_openra_engine_port_asset_parity "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-engine-port-asset-parity.json" \
  --slurpfile rts_combat_readability_pressure_readiness "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-readability-pressure-readiness.json" \
  --slurpfile rts_playtest_observability_readiness "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-playtest-observability-readiness.json" \
  --slurpfile boundary "$ROOT/acceptance/S6_public_launch/latest/client-boundary-cleanliness.json" \
  --slurpfile runner "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json" \
  --slurpfile launcher "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-launcher.json" \
  -f "$SUMMARY_FILTER" >"$SUMMARY"

jq '
  .check_count = (.checks | length)
  | .passed_check_count = ([.checks[]] | map(select(. == true)) | length)
  | .failed_check_count = ([.checks[]] | map(select(. != true)) | length)
  | .artifact_count = (.artifacts | length)
  | .gate_count = (.gates | length)
  | .true_gate_count = ([.gates[]] | map(select(. == true)) | length)
  | .false_boundary_gate_count = ([
      .gates
      | to_entries[]
      | select((.key == "cex_runtime_player_client_allowed" or .key == "wgpu_required") and .value == false)
    ] | length)
  | .passed_gate_count = (.true_gate_count + .false_boundary_gate_count)
  | .failed_gate_count = (.gate_count - .passed_gate_count)
' "$SUMMARY" >"$SUMMARY_WITH_COUNTS"
mv "$SUMMARY_WITH_COUNTS" "$SUMMARY"

: <<'PLAYTEST_READINESS_SUMMARY_FILTER_BLOCK'
# BEGIN_PLAYTEST_READINESS_SUMMARY_FILTER
  def ok($x): ($x[0].green == true);
  {
    contract_version: "trillionnium_world_bevy_classic_playtest_readiness_v1",
    status: "classic_playtest_readiness_green",
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
      and ok($rts_auto_bot)
      and ok($rts_organic_terminal_gap)
      and ok($rts_terminal_observation_gap)
      and ok($rts_replay_metrics_gap)
      and ok($rts_endurance_skirmish_gap)
      and ok($rts_bot_decision_state_gap)
      and ok($rts_bot_adaptive_build_order_gap)
      and ok($rts_bot_tactical_micro_gap)
      and ok($rts_bot_map_intel_gap)
      and ok($rts_bot_macro_economy_gap)
      and ok($rts_bot_harassment_defense_gap)
      and ok($rts_bot_multi_front_pressure_gap)
      and ok($rts_bot_expansion_control_gap)
      and ok($rts_bot_tech_transition_gap)
      and ok($rts_bot_army_composition_gap)
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
      and ok($rts_visual_fidelity)
      and ok($rts_command_affordance)
      and ok($rts_command_surface)
      and ok($rts_structure_modeling)
      and ok($rts_environment_life)
      and ok($rts_map_model_gap)
      and ok($rts_worker_harvest_animation)
      and ok($rts_production_spawn_animation)
      and ok($rts_unit_status_portrait)
      and ok($rts_selection_command_feedback)
      and ok($rts_ability_tooltip_telegraph)
      and ok($rts_control_group_hotkey_feedback)
      and ok($rts_scrollable_map)
      and ok($rts_camera_minimap_sync)
      and ok($rts_command_queue_path_preview)
      and ok($rts_formation_move_preview)
      and ok($rts_formation_move_execution)
      and ok($rts_local_obstruction_recovery)
      and ok($rts_action_cadence)
      and ok($rts_unit_model_depth)
      and ok($rts_action_sequence)
      and ok($rts_npc_behavior)
      and ok($rts_combat_impact)
      and ok($rts_locomotion_blend)
      and ok($rts_npc_transition)
      and ok($rts_depth_readability)
      and ok($rts_first_minute_readiness)
      and ok($rts_map_ui_modeling_readiness)
      and ok($rts_first_contact_basin_spec)
      and ok($rts_production_art_replication)
      and ok($rts_production_asset_atlas)
      and ok($rts_production_ui_skin)
      and ok($rts_production_interaction_polish)
      and ok($rts_full_screen_ui_replication)
      and ok($rts_shell_meta_ui_replication)
      and ok($rts_match_setup_ui_replication)
      and ok($rts_campaign_outcome_ui_readiness)
      and ok($rts_campaign_ui_continuity)
      and ok($rts_in_match_hud_state_replication)
      and ok($rts_session_state_continuity)
      and ok($rts_continuous_player_flow)
      and ok($rts_live_session_playthrough)
      and ok($rts_full_game_visual_ui_replication)
      and ok($rts_openra_screen_for_screen_ui_replication)
      and ok($rts_openra_engine_port_asset_parity)
      and ok($rts_combat_readability_pressure_readiness)
      and ok($rts_playtest_observability_readiness)
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
      and $rts_live[0].production_feedback_chip_gate == true
      and $rts_live[0].move_live_gate == true
      and $rts_live[0].waypoint_live_gate == true
      and $rts_live[0].hold_live_gate == true
      and $rts_live[0].patrol_live_gate == true
      and $rts_live[0].attack_move_live_gate == true
      and $rts_live[0].stop_live_gate == true
      and $rts_live[0].attack_live_gate == true
      and $rts_live[0].ability_live_gate == true
      and $rts_live[0].command_feedback_chip_gate == true
      and $rts_live[0].live_command_queue_path_preview_gate == true
      and $rts_live[0].right_click_execution_feedback_gate == true
      and $rts_live[0].right_click_execution_feedback_player_label_gate == true
      and $rts_live[0].rts_core_frame_order_gate == true
      and $rts_live[0].rts_core_headless_replay_gate == true
      and $rts_live[0].right_click_execution_feedback_label_pixel_count > 700
      and $rts_live[0].context_cursor_gate == true
      and $rts_live[0].viewport_world_input_gate == true
      and $rts_live[0].control_group_hotkey_gate == true
      and $rts_live[0].accepted_input_count == 10
      and $rts_path[0].live_pathing_input_gate == true
      and $rts_path[0].path_tile_gate == true
      and $rts_path[0].blocked_tile_gate == true
      and $rts_path[0].formation_slot_gate == true
      and $rts_path[0].command_visual_gate == true
      and $rts_path[0].rts_pathing_core_frame_order_gate == true
      and $rts_path[0].rts_pathing_core_headless_replay_gate == true
      and $rts_path[0].accepted_input_count == 2
      and $rts_collision[0].live_collision_input_gate == true
      and $rts_collision[0].collision_response_gate == true
      and $rts_collision[0].engagement_response_gate == true
      and $rts_collision[0].rts_collision_core_frame_order_gate == true
      and $rts_collision[0].rts_collision_core_headless_replay_gate == true
      and $rts_collision[0].accepted_input_count == 3
      and $rts_target[0].live_targeting_input_gate == true
      and $rts_target[0].target_priority_gate == true
      and $rts_target[0].aggro_gate == true
      and $rts_target[0].focus_fire_gate == true
      and $rts_target[0].threat_feedback_gate == true
      and $rts_target[0].rts_targeting_core_frame_order_gate == true
      and $rts_target[0].rts_targeting_core_headless_replay_gate == true
      and $rts_target[0].accepted_input_count == 4
      and $rts_economy[0].live_economy_input_gate == true
      and $rts_economy[0].harvest_loop_gate == true
      and $rts_economy[0].build_loop_gate == true
      and $rts_economy[0].production_loop_gate == true
      and $rts_economy[0].rts_economy_core_frame_order_gate == true
      and $rts_economy[0].rts_economy_core_headless_replay_gate == true
      and $rts_economy[0].accepted_input_count == 4
      and $rts_select[0].live_selection_minimap_input_gate == true
      and $rts_select[0].selection_box_gate == true
      and $rts_select[0].control_group_gate == true
      and $rts_select[0].minimap_command_gate == true
      and $rts_select[0].split_route_gate == true
      and $rts_select[0].rts_selection_minimap_core_frame_order_gate == true
      and $rts_select[0].rts_selection_minimap_core_headless_replay_gate == true
      and $rts_select[0].accepted_input_count == 4
      and $rts_build_lifecycle[0].live_build_lifecycle_input_gate == true
      and $rts_build_lifecycle[0].build_placement_gate == true
      and $rts_build_lifecycle[0].completion_gate == true
      and $rts_build_lifecycle[0].repair_gate == true
      and $rts_build_lifecycle[0].cancel_refund_gate == true
      and $rts_build_lifecycle[0].rts_production_lifecycle_core_frame_order_gate == true
      and $rts_build_lifecycle[0].rts_production_lifecycle_core_headless_replay_gate == true
      and $rts_build_lifecycle[0].accepted_input_count == 6
      and $rts_tech_tree[0].live_tech_tree_input_gate == true
      and $rts_tech_tree[0].faction_base_gate == true
      and $rts_tech_tree[0].research_gate == true
      and $rts_tech_tree[0].upgrade_gate == true
      and $rts_tech_tree[0].unlock_gate == true
      and $rts_tech_tree[0].dependency_gate == true
      and $rts_tech_tree[0].rts_tech_tree_core_frame_order_gate == true
      and $rts_tech_tree[0].rts_tech_tree_core_headless_replay_gate == true
      and $rts_tech_tree[0].accepted_input_count == 6
      and $rts_projectile[0].live_projectile_ability_input_gate == true
      and $rts_projectile[0].projectile_trail_gate == true
      and $rts_projectile[0].projectile_impact_gate == true
      and $rts_projectile[0].ability_radius_gate == true
      and $rts_projectile[0].damage_tick_gate == true
      and $rts_projectile[0].armor_shield_gate == true
      and $rts_projectile[0].rts_projectile_ability_core_frame_order_gate == true
      and $rts_projectile[0].rts_projectile_ability_core_headless_replay_gate == true
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
      and $rts_auto_bot[0].no_live_player_input_gate == true
      and $rts_auto_bot[0].autonomous_timeline_gate == true
      and $rts_auto_bot[0].bot_roster_gate == true
      and $rts_auto_bot[0].economy_gate == true
      and $rts_auto_bot[0].production_gate == true
      and $rts_auto_bot[0].combat_gate == true
      and $rts_auto_bot[0].terminal_gate == true
      and $rts_auto_bot[0].autonomous_bot_skirmish_gate == true
      and $rts_auto_bot[0].input_action_count == 0
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
      and $rts_fog[0].rts_fog_core_frame_order_gate == true
      and $rts_fog[0].rts_fog_core_headless_replay_gate == true
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
      classic_rts_autonomous_bot_skirmish_green: ok($rts_auto_bot),
      classic_rts_organic_terminal_gap_green: ok($rts_organic_terminal_gap),
      classic_rts_terminal_observation_gap_green: ok($rts_terminal_observation_gap),
      classic_rts_replay_metrics_gap_green: ok($rts_replay_metrics_gap),
      classic_rts_endurance_skirmish_gap_green: ok($rts_endurance_skirmish_gap),
      classic_rts_bot_decision_state_gap_green: ok($rts_bot_decision_state_gap),
      classic_rts_bot_adaptive_build_order_gap_green: ok($rts_bot_adaptive_build_order_gap),
      classic_rts_bot_tactical_micro_gap_green: ok($rts_bot_tactical_micro_gap),
      classic_rts_bot_map_intel_gap_green: ok($rts_bot_map_intel_gap),
      classic_rts_bot_macro_economy_gap_green: ok($rts_bot_macro_economy_gap),
      classic_rts_bot_harassment_defense_gap_green: ok($rts_bot_harassment_defense_gap),
      classic_rts_bot_multi_front_pressure_gap_green: ok($rts_bot_multi_front_pressure_gap),
      classic_rts_bot_expansion_control_gap_green: ok($rts_bot_expansion_control_gap),
      classic_rts_bot_tech_transition_gap_green: ok($rts_bot_tech_transition_gap),
      classic_rts_bot_army_composition_gap_green: ok($rts_bot_army_composition_gap),
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
      classic_rts_visual_fidelity_green: ok($rts_visual_fidelity),
      classic_rts_command_affordance_green: ok($rts_command_affordance),
      classic_rts_command_surface_green: ok($rts_command_surface),
      classic_rts_structure_modeling_green: ok($rts_structure_modeling),
      classic_rts_environment_life_green: ok($rts_environment_life),
      classic_rts_map_model_gap_green: ok($rts_map_model_gap),
      classic_rts_worker_harvest_animation_green: ok($rts_worker_harvest_animation),
      classic_rts_production_spawn_animation_green: ok($rts_production_spawn_animation),
      classic_rts_unit_status_portrait_green: ok($rts_unit_status_portrait),
      classic_rts_selection_command_feedback_green: ok($rts_selection_command_feedback),
      classic_rts_ability_tooltip_telegraph_green: ok($rts_ability_tooltip_telegraph),
      classic_rts_control_group_hotkey_feedback_green: ok($rts_control_group_hotkey_feedback),
      classic_rts_scrollable_map_green: ok($rts_scrollable_map),
      classic_rts_camera_minimap_sync_green: ok($rts_camera_minimap_sync),
      classic_rts_command_queue_path_preview_green: ok($rts_command_queue_path_preview),
      classic_rts_formation_move_preview_green: ok($rts_formation_move_preview),
      classic_rts_formation_move_execution_green: ok($rts_formation_move_execution),
      classic_rts_local_obstruction_recovery_green: ok($rts_local_obstruction_recovery),
      classic_rts_action_cadence_green: ok($rts_action_cadence),
      classic_rts_unit_model_depth_green: ok($rts_unit_model_depth),
      classic_rts_action_sequence_green: ok($rts_action_sequence),
      classic_rts_npc_behavior_green: ok($rts_npc_behavior),
      classic_rts_combat_impact_green: ok($rts_combat_impact),
      classic_rts_locomotion_blend_green: ok($rts_locomotion_blend),
      classic_rts_npc_transition_green: ok($rts_npc_transition),
      classic_rts_depth_readability_green: ok($rts_depth_readability),
      classic_rts_first_minute_readiness_green: ok($rts_first_minute_readiness),
      classic_rts_map_ui_modeling_readiness_green: ok($rts_map_ui_modeling_readiness),
      classic_rts_first_contact_basin_spec_green: ok($rts_first_contact_basin_spec),
      classic_rts_production_art_replication_green: ok($rts_production_art_replication),
      classic_rts_production_asset_atlas_green: ok($rts_production_asset_atlas),
      classic_rts_production_ui_skin_green: ok($rts_production_ui_skin),
      classic_rts_production_interaction_polish_green: ok($rts_production_interaction_polish),
      classic_rts_full_screen_ui_replication_green: ok($rts_full_screen_ui_replication),
      classic_rts_shell_meta_ui_replication_green: ok($rts_shell_meta_ui_replication),
      classic_rts_match_setup_ui_replication_green: ok($rts_match_setup_ui_replication),
      classic_rts_campaign_outcome_ui_readiness_green: ok($rts_campaign_outcome_ui_readiness),
      classic_rts_campaign_ui_continuity_green: ok($rts_campaign_ui_continuity),
      classic_rts_in_match_hud_state_replication_green: ok($rts_in_match_hud_state_replication),
      classic_rts_session_state_continuity_green: ok($rts_session_state_continuity),
      classic_rts_continuous_player_flow_green: ok($rts_continuous_player_flow),
      classic_rts_live_session_playthrough_green: ok($rts_live_session_playthrough),
      classic_rts_full_game_visual_ui_replication_green: ok($rts_full_game_visual_ui_replication),
      classic_rts_openra_screen_for_screen_ui_replication_green: ok($rts_openra_screen_for_screen_ui_replication),
      classic_rts_openra_engine_port_asset_parity_green: ok($rts_openra_engine_port_asset_parity),
      classic_rts_combat_readability_pressure_readiness_green: ok($rts_combat_readability_pressure_readiness),
      classic_rts_playtest_observability_readiness_green: ok($rts_playtest_observability_readiness),
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
      rts_live_input_hover_preview_pixel_count: $rts_live[0].hover_preview_pixel_count,
      rts_live_input_final_hover_player_label: $rts_live[0].final_hover_player_label,
      rts_live_input_context_cursor_pixel_count: $rts_live[0].context_cursor_pixel_count,
      rts_live_input_final_context_cursor_label: $rts_live[0].final_context_cursor_player_label,
      rts_live_input_viewport_world_shifted_tile_id: $rts_live[0].viewport_world_input_sample.shifted_tile_id,
      rts_live_input_viewport_world_shifted_action_label: $rts_live[0].viewport_world_input_sample.shifted_action_label,
      rts_live_input_drag_select_preview_pixel_count: $rts_live[0].drag_select_preview_pixel_count,
      rts_live_input_drag_select_preview_label: ($rts_live[0].drag_select_preview_samples[0].player_label // ""),
      rts_live_input_drag_select_commit_selected_unit_count: ($rts_live[0].drag_select_commit_sample.selected_unit_ids | length),
      rts_live_input_drag_select_commit_label: $rts_live[0].drag_select_commit_sample.command_stamp_player_label,
      rts_live_input_drag_select_commit_selection_marker_pixel_count: $rts_live[0].drag_select_commit_selection_marker_pixel_count,
      rts_live_input_drag_select_filter_selected_unit_count: ($rts_live[0].drag_select_filter_sample.selected_unit_ids | length),
      rts_live_input_drag_select_filter_rejected_unit_count: ($rts_live[0].drag_select_filter_sample.preview_rejected_unit_ids | length),
      rts_live_input_drag_select_filter_label: $rts_live[0].drag_select_filter_sample.command_stamp_player_label,
      rts_live_input_drag_select_filter_selection_marker_pixel_count: $rts_live[0].drag_select_filter_selection_marker_pixel_count,
      rts_live_input_unit_click_select_marker_pixel_count: $rts_live[0].unit_click_select_marker_pixel_count,
      rts_live_input_unit_click_select_stamp_pixel_count: $rts_live[0].unit_click_select_stamp_pixel_count,
      rts_live_input_unit_click_select_unit_count: ($rts_live[0].unit_click_select_sample.selected_unit_ids | length),
      rts_live_input_unit_click_select_label: $rts_live[0].unit_click_select_sample.command_stamp_player_label,
      rts_live_input_selection_clear_stamp_pixel_count: $rts_live[0].selection_clear_stamp_pixel_count,
      rts_live_input_selection_clear_command_disabled_pixel_count: $rts_live[0].selection_clear_command_disabled_pixel_count,
      rts_live_input_selection_clear_residual_marker_pixel_count: $rts_live[0].selection_clear_residual_marker_pixel_count,
      rts_live_input_selection_clear_empty_label: ([ $rts_live[0].selection_clear_samples[] | select(.stage == "empty_viewport_clear") | .command_stamp_player_label ][0] // ""),
      rts_live_input_selection_clear_hostile_label: ([ $rts_live[0].selection_clear_samples[] | select(.stage == "hostile_viewport_clear") | .command_stamp_player_label ][0] // ""),
      rts_live_input_right_click_target_label: $rts_live[0].right_click_target_sample.command_stamp_player_label,
      rts_live_input_right_click_target_hover_label: $rts_live[0].right_click_target_hover_sample.player_label,
      rts_live_input_right_click_target_selected_unit_count: ($rts_live[0].right_click_target_sample.selected_unit_ids | length),
      rts_live_input_right_click_target_attack_marker_pixel_count: $rts_live[0].right_click_target_attack_marker_pixel_count,
      rts_live_input_right_click_target_sample_count: ($rts_live[0].right_click_target_samples | length),
      rts_live_input_right_click_target_move_label: ([ $rts_live[0].right_click_target_samples[] | select(.stage == "right_click_empty_move") | .command_stamp_player_label ][0] // ""),
      rts_live_input_right_click_target_follow_label: ([ $rts_live[0].right_click_target_samples[] | select(.stage == "right_click_friendly_follow") | .command_stamp_player_label ][0] // ""),
      rts_live_input_right_click_target_harvest_label: ([ $rts_live[0].right_click_target_samples[] | select(.stage == "right_click_resource_harvest") | .command_stamp_player_label ][0] // ""),
      rts_live_input_rts_core_frame_order_count: ($rts_live[0].rts_core_frame_orders | length),
      rts_live_input_rts_core_frame_order_kinds: $rts_live[0].rts_core_frame_order_kind_labels,
      rts_live_input_rts_core_frame_order_stream_sha256: $rts_live[0].rts_core_frame_order_stream_sha256,
      rts_live_input_rts_core_headless_checkpoint_sha256: $rts_live[0].rts_core_headless_checkpoint_sha256,
      rts_live_input_rts_core_headless_applied_order_count: $rts_live[0].rts_core_headless_applied_order_count,
      rts_live_input_rts_core_headless_actor_count: $rts_live[0].rts_core_headless_actor_count,
      rts_live_input_rts_core_headless_final_frame: $rts_live[0].rts_core_headless_final_frame,
      rts_live_input_right_click_target_follow_stamp_pixel_count: $rts_live[0].right_click_target_follow_stamp_pixel_count,
      rts_live_input_right_click_target_harvest_stamp_pixel_count: $rts_live[0].right_click_target_harvest_stamp_pixel_count,
      rts_live_input_right_click_target_preview_path_pixel_count: $rts_live[0].right_click_target_preview_path_pixel_count,
      rts_live_input_right_click_target_preview_attack_pixel_count: $rts_live[0].right_click_target_preview_attack_pixel_count,
      rts_live_input_right_click_target_preview_follow_pixel_count: $rts_live[0].right_click_target_preview_follow_pixel_count,
      rts_live_input_right_click_target_preview_harvest_pixel_count: $rts_live[0].right_click_target_preview_harvest_pixel_count,
      rts_live_input_right_click_execution_feedback_frame_pixel_count: $rts_live[0].right_click_execution_feedback_frame_pixel_count,
      rts_live_input_right_click_execution_feedback_path_pixel_count: $rts_live[0].right_click_execution_feedback_path_pixel_count,
      rts_live_input_right_click_execution_feedback_target_pixel_count: $rts_live[0].right_click_execution_feedback_target_pixel_count,
      rts_live_input_right_click_execution_feedback_follow_pixel_count: $rts_live[0].right_click_execution_feedback_follow_pixel_count,
      rts_live_input_right_click_execution_feedback_harvest_pixel_count: $rts_live[0].right_click_execution_feedback_harvest_pixel_count,
      rts_live_input_right_click_execution_feedback_viewport_marker_pixel_count: $rts_live[0].right_click_execution_feedback_viewport_marker_pixel_count,
      rts_live_input_right_click_execution_feedback_label_pixel_count: $rts_live[0].right_click_execution_feedback_label_pixel_count,
      rts_live_input_right_click_execution_feedback_move_label: ([ $rts_live[0].right_click_target_samples[] | select(.stage == "right_click_empty_move") | .execution_feedback_player_label ][0] // ""),
      rts_live_input_right_click_execution_feedback_attack_label: ([ $rts_live[0].right_click_target_samples[] | select(.stage == "drag_filter_then_right_click_hostile") | .execution_feedback_player_label ][0] // ""),
      rts_live_input_right_click_execution_feedback_follow_label: ([ $rts_live[0].right_click_target_samples[] | select(.stage == "right_click_friendly_follow") | .execution_feedback_player_label ][0] // ""),
      rts_live_input_right_click_execution_feedback_harvest_label: ([ $rts_live[0].right_click_target_samples[] | select(.stage == "right_click_resource_harvest") | .execution_feedback_player_label ][0] // ""),
      rts_live_input_unit_shift_select_marker_pixel_count: $rts_live[0].unit_shift_select_marker_pixel_count,
      rts_live_input_unit_shift_select_stamp_pixel_count: $rts_live[0].unit_shift_select_stamp_pixel_count,
      rts_live_input_unit_shift_select_add_unit_count: ([ $rts_live[0].unit_shift_select_samples[] | select(.stage == "shift_add_patrol") | .selected_unit_ids | length ][0] // 0),
      rts_live_input_unit_shift_select_remove_unit_count: ([ $rts_live[0].unit_shift_select_samples[] | select(.stage == "shift_remove_player") | .selected_unit_ids | length ][0] // 0),
      rts_live_input_unit_shift_select_add_label: ([ $rts_live[0].unit_shift_select_samples[] | select(.stage == "shift_add_patrol") | .command_stamp_player_label ][0] // ""),
      rts_live_input_unit_shift_select_remove_label: ([ $rts_live[0].unit_shift_select_samples[] | select(.stage == "shift_remove_player") | .command_stamp_player_label ][0] // ""),
      rts_live_input_unit_double_click_select_marker_pixel_count: $rts_live[0].unit_double_click_select_marker_pixel_count,
      rts_live_input_unit_double_click_select_stamp_pixel_count: $rts_live[0].unit_double_click_select_stamp_pixel_count,
      rts_live_input_unit_double_click_select_unit_count: ($rts_live[0].unit_double_click_select_sample.selected_unit_ids | length),
      rts_live_input_unit_double_click_select_label: $rts_live[0].unit_double_click_select_sample.command_stamp_player_label,
      rts_live_input_control_group_hotkey_marker_pixel_count: $rts_live[0].control_group_hotkey_marker_pixel_count,
      rts_live_input_control_group_hotkey_stamp_pixel_count: $rts_live[0].control_group_hotkey_stamp_pixel_count,
      rts_live_input_control_group_hotkey_assign_label: ([ $rts_live[0].control_group_hotkey_samples[] | select(.stage == "ctrl_assign_group_5") | .command_stamp_player_label ][0] // ""),
      rts_live_input_control_group_hotkey_recall_label: ([ $rts_live[0].control_group_hotkey_samples[] | select(.stage == "recall_group_5") | .command_stamp_player_label ][0] // ""),
      rts_live_input_control_group_hotkey_camera_label: ([ $rts_live[0].control_group_hotkey_samples[] | select(.stage == "double_tap_camera_group_5") | .command_stamp_player_label ][0] // ""),
      rts_live_input_control_group_hotkey_append_label: ([ $rts_live[0].control_group_hotkey_samples[] | select(.stage == "ctrl_shift_append_group_5") | .command_stamp_player_label ][0] // ""),
      rts_live_input_control_group_hotkey_recall_add_label: ([ $rts_live[0].control_group_hotkey_samples[] | select(.stage == "shift_recall_add_group_5") | .command_stamp_player_label ][0] // ""),
      rts_live_input_control_group_hotkey_append_unit_count: ([ $rts_live[0].control_group_hotkey_samples[] | select(.stage == "ctrl_shift_append_group_5") | .selected_unit_ids | length ][0] // 0),
      rts_live_input_control_group_hotkey_recall_add_unit_count: ([ $rts_live[0].control_group_hotkey_samples[] | select(.stage == "shift_recall_add_group_5") | .selected_unit_ids | length ][0] // 0),
      rts_live_input_control_group_slot_pixel_count: $rts_live[0].control_group_slot_pixel_count,
      rts_live_input_control_group_slot_5_member_count: ([ $rts_live[0].control_group_hotkey_samples[] | select(.stage == "ctrl_shift_append_group_5") | .control_group_slot_summaries[] | select(.slot == "5") | .member_count ][0] // 0),
      rts_live_input_control_group_slot_0_key_label: ([ $rts_live[0].control_group_hotkey_samples[] | select(.stage == "shift_recall_add_group_5") | .control_group_slot_summaries[] | select(.slot == "10") | .key_label ][0] // ""),
      rts_live_input_command_stamp_pixel_count: $rts_live[0].command_stamp_pixel_count,
      rts_live_input_final_command_stamp_player_label: $rts_live[0].final_command_stamp_player_label,
      rts_live_input_command_feedback_chip_count: $rts_live[0].command_feedback_chip_count,
      rts_live_input_command_queue_path_preview_slot_pixel_count: $rts_live[0].live_command_queue_path_preview_slot_pixel_count,
      rts_live_input_command_queue_path_preview_path_pixel_count: $rts_live[0].live_command_queue_path_preview_path_pixel_count,
      rts_live_input_command_queue_path_preview_waypoint_pixel_count: $rts_live[0].live_command_queue_path_preview_waypoint_pixel_count,
      rts_live_input_command_queue_path_preview_target_pixel_count: $rts_live[0].live_command_queue_path_preview_target_pixel_count,
      rts_live_input_command_queue_path_preview_cancel_pixel_count: $rts_live[0].live_command_queue_path_preview_cancel_pixel_count,
      rts_pathing_accepted_input_count: $rts_path[0].accepted_input_count,
      rts_pathing_path_tile_count: ($rts_path[0].path_tile_ids | length),
      rts_pathing_blocked_tile_count: ($rts_path[0].blocked_tile_ids | length),
      rts_pathing_formation_slot_count: ($rts_path[0].formation_slot_tile_ids | length),
      rts_pathing_path_tile_pixel_count: $rts_path[0].path_tile_pixel_count,
      rts_pathing_blocked_tile_pixel_count: $rts_path[0].blocked_tile_pixel_count,
      rts_pathing_formation_slot_pixel_count: $rts_path[0].formation_slot_pixel_count,
      rts_pathing_selection_marker_pixel_count: $rts_path[0].selection_marker_pixel_count,
      rts_pathing_command_marker_pixel_count: $rts_path[0].command_marker_pixel_count,
      rts_pathing_core_frame_order_count: ($rts_path[0].rts_pathing_core_frame_orders | length),
      rts_pathing_core_frame_order_kinds: $rts_path[0].rts_pathing_core_frame_order_kind_labels,
      rts_pathing_core_frame_order_stream_sha256: $rts_path[0].rts_pathing_core_frame_order_stream_sha256,
      rts_pathing_core_headless_checkpoint_sha256: $rts_path[0].rts_pathing_core_headless_checkpoint_sha256,
      rts_pathing_core_headless_applied_order_count: $rts_path[0].rts_pathing_core_headless_applied_order_count,
      rts_pathing_core_headless_actor_count: $rts_path[0].rts_pathing_core_headless_actor_count,
      rts_pathing_core_headless_final_frame: $rts_path[0].rts_pathing_core_headless_final_frame,
      rts_collision_accepted_input_count: $rts_collision[0].accepted_input_count,
      rts_collision_move_disperse_tile_count: ($rts_collision[0].move_disperse_tile_ids | length),
      rts_collision_engagement_tile_count: ($rts_collision[0].engagement_tile_ids | length),
      rts_collision_contact_flash_tile_count: ($rts_collision[0].contact_flash_tile_ids | length),
      rts_collision_dispersion_slot_pixel_count: $rts_collision[0].dispersion_slot_pixel_count,
      rts_collision_engagement_range_pixel_count: $rts_collision[0].engagement_range_pixel_count,
      rts_collision_contact_flash_pixel_count: $rts_collision[0].contact_flash_pixel_count,
      rts_collision_blocked_tile_pixel_count: $rts_collision[0].blocked_tile_pixel_count,
      rts_collision_attack_feedback_pixel_count: $rts_collision[0].attack_feedback_pixel_count,
      rts_collision_core_frame_order_count: ($rts_collision[0].rts_collision_core_frame_orders | length),
      rts_collision_core_frame_order_kinds: $rts_collision[0].rts_collision_core_frame_order_kind_labels,
      rts_collision_core_frame_order_stream_sha256: $rts_collision[0].rts_collision_core_frame_order_stream_sha256,
      rts_collision_core_headless_checkpoint_sha256: $rts_collision[0].rts_collision_core_headless_checkpoint_sha256,
      rts_collision_core_headless_applied_order_count: $rts_collision[0].rts_collision_core_headless_applied_order_count,
      rts_collision_core_headless_actor_count: $rts_collision[0].rts_collision_core_headless_actor_count,
      rts_collision_core_headless_final_frame: $rts_collision[0].rts_collision_core_headless_final_frame,
      rts_collision_core_headless_attack_order_count: $rts_collision[0].rts_collision_core_headless_attack_order_count,
      rts_targeting_accepted_input_count: $rts_target[0].accepted_input_count,
      rts_targeting_priority_count: ($rts_target[0].final_target_priority_ids | length),
      rts_targeting_focus_fire_unit_count: ($rts_target[0].final_focus_fire_unit_ids | length),
      rts_targeting_threat_level_count: ($rts_target[0].final_threat_level_percents | length),
      rts_targeting_core_frame_order_count: ($rts_target[0].rts_targeting_core_frame_orders | length),
      rts_targeting_core_frame_order_kinds: $rts_target[0].rts_targeting_core_frame_order_kind_labels,
      rts_targeting_core_headless_checkpoint_sha256: $rts_target[0].rts_targeting_core_headless_checkpoint_sha256,
      rts_targeting_core_headless_applied_order_count: $rts_target[0].rts_targeting_core_headless_applied_order_count,
      rts_targeting_core_headless_actor_count: $rts_target[0].rts_targeting_core_headless_actor_count,
      rts_targeting_core_headless_final_frame: $rts_target[0].rts_targeting_core_headless_final_frame,
      rts_targeting_core_headless_ability_order_count: $rts_target[0].rts_targeting_core_headless_ability_order_count,
      rts_targeting_core_headless_ability_rule_ids: $rts_target[0].rts_targeting_core_headless_ability_rule_ids,
      rts_targeting_core_headless_ability_target_actor_ids: $rts_target[0].rts_targeting_core_headless_ability_target_actor_ids,
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
      rts_economy_core_frame_order_count: ($rts_economy[0].rts_economy_core_frame_orders | length),
      rts_economy_core_frame_order_kinds: $rts_economy[0].rts_economy_core_frame_order_kind_labels,
      rts_economy_core_frame_order_stream_sha256: $rts_economy[0].rts_economy_core_frame_order_stream_sha256,
      rts_economy_core_headless_checkpoint_sha256: $rts_economy[0].rts_economy_core_headless_checkpoint_sha256,
      rts_economy_core_headless_applied_order_count: $rts_economy[0].rts_economy_core_headless_applied_order_count,
      rts_economy_core_headless_actor_count: $rts_economy[0].rts_economy_core_headless_actor_count,
      rts_economy_core_headless_final_frame: $rts_economy[0].rts_economy_core_headless_final_frame,
      rts_economy_core_lifecycle_order_count: $rts_economy[0].rts_economy_core_lifecycle_order_count,
      rts_economy_core_build_order_count: $rts_economy[0].rts_economy_core_build_order_count,
      rts_economy_core_train_order_count: $rts_economy[0].rts_economy_core_train_order_count,
      rts_economy_core_harvest_order_count: $rts_economy[0].rts_economy_core_harvest_order_count,
      rts_economy_core_build_rule_ids: $rts_economy[0].rts_economy_core_build_rule_ids,
      rts_economy_core_train_rule_ids: $rts_economy[0].rts_economy_core_train_rule_ids,
      rts_selection_minimap_accepted_input_count: $rts_select[0].accepted_input_count,
      rts_selection_box_tile_count: ($rts_select[0].final_selection_box_tile_ids | length),
      rts_control_group_assignment_count: ($rts_select[0].final_control_group_assignments | length),
      rts_active_control_group_count: ($rts_select[0].final_active_control_group_ids | length),
      rts_minimap_command_tile_id: $rts_select[0].final_minimap_command_tile_id,
      rts_minimap_rally_tile_seen: (
        ($rts_select[0].stage_summaries // [])
        | any(.stage == "minimap_rally" and .minimap_command_tile_id == "9,2")
      ),
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
      rts_selection_minimap_core_frame_order_count: ($rts_select[0].rts_selection_minimap_core_frame_orders | length),
      rts_selection_minimap_core_frame_order_kinds: $rts_select[0].rts_selection_minimap_core_frame_order_kind_labels,
      rts_selection_minimap_core_frame_order_stream_sha256: $rts_select[0].rts_selection_minimap_core_frame_order_stream_sha256,
      rts_selection_minimap_core_headless_checkpoint_sha256: $rts_select[0].rts_selection_minimap_core_headless_checkpoint_sha256,
      rts_selection_minimap_core_headless_applied_order_count: $rts_select[0].rts_selection_minimap_core_headless_applied_order_count,
      rts_selection_minimap_core_headless_actor_count: $rts_select[0].rts_selection_minimap_core_headless_actor_count,
      rts_selection_minimap_core_headless_final_frame: $rts_select[0].rts_selection_minimap_core_headless_final_frame,
      rts_build_lifecycle_accepted_input_count: $rts_build_lifecycle[0].accepted_input_count,
      rts_build_lifecycle_completed_structure_count: ($rts_build_lifecycle[0].final_completed_structure_ids | length),
      rts_build_lifecycle_cancelled_structure_count: ($rts_build_lifecycle[0].final_cancelled_structure_ids | length),
      rts_build_lifecycle_repair_progress_percent: $rts_build_lifecycle[0].final_repair_progress_percent,
      rts_build_lifecycle_refund_count: ($rts_build_lifecycle[0].final_refund_delta_log | length),
      rts_build_lifecycle_core_frame_order_count: ($rts_build_lifecycle[0].rts_production_lifecycle_core_frame_orders | length),
      rts_build_lifecycle_core_frame_order_kinds: $rts_build_lifecycle[0].rts_production_lifecycle_core_frame_order_kind_labels,
      rts_build_lifecycle_core_frame_order_stream_sha256: $rts_build_lifecycle[0].rts_production_lifecycle_core_frame_order_stream_sha256,
      rts_build_lifecycle_core_headless_checkpoint_sha256: $rts_build_lifecycle[0].rts_production_lifecycle_core_headless_checkpoint_sha256,
      rts_build_lifecycle_core_headless_applied_order_count: $rts_build_lifecycle[0].rts_production_lifecycle_core_headless_applied_order_count,
      rts_build_lifecycle_core_headless_actor_count: $rts_build_lifecycle[0].rts_production_lifecycle_core_headless_actor_count,
      rts_build_lifecycle_core_headless_final_frame: $rts_build_lifecycle[0].rts_production_lifecycle_core_headless_final_frame,
      rts_build_lifecycle_core_lifecycle_order_count: $rts_build_lifecycle[0].rts_production_lifecycle_core_lifecycle_order_count,
      rts_build_lifecycle_core_build_order_count: $rts_build_lifecycle[0].rts_production_lifecycle_core_build_order_count,
      rts_build_lifecycle_core_complete_order_count: $rts_build_lifecycle[0].rts_production_lifecycle_core_complete_order_count,
      rts_build_lifecycle_core_repair_order_count: $rts_build_lifecycle[0].rts_production_lifecycle_core_repair_order_count,
      rts_build_lifecycle_core_cancel_order_count: $rts_build_lifecycle[0].rts_production_lifecycle_core_cancel_order_count,
      rts_build_lifecycle_core_refund_order_count: $rts_build_lifecycle[0].rts_production_lifecycle_core_refund_order_count,
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
      rts_tech_tree_core_frame_order_count: ($rts_tech_tree[0].rts_tech_tree_core_frame_orders | length),
      rts_tech_tree_core_headless_checkpoint_sha256: $rts_tech_tree[0].rts_tech_tree_core_headless_checkpoint_sha256,
      rts_tech_tree_core_headless_applied_order_count: $rts_tech_tree[0].rts_tech_tree_core_headless_applied_order_count,
      rts_tech_tree_core_headless_actor_count: $rts_tech_tree[0].rts_tech_tree_core_headless_actor_count,
      rts_tech_tree_core_headless_final_frame: $rts_tech_tree[0].rts_tech_tree_core_headless_final_frame,
      rts_tech_tree_core_tech_order_count: $rts_tech_tree[0].rts_tech_tree_core_tech_order_count,
      rts_tech_tree_core_research_order_count: $rts_tech_tree[0].rts_tech_tree_core_research_order_count,
      rts_tech_tree_core_upgrade_order_count: $rts_tech_tree[0].rts_tech_tree_core_upgrade_order_count,
      rts_tech_tree_core_unlock_order_count: $rts_tech_tree[0].rts_tech_tree_core_unlock_order_count,
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
      rts_projectile_ability_core_frame_order_count: ($rts_projectile[0].rts_projectile_ability_core_frame_orders | length),
      rts_projectile_ability_core_frame_order_kinds: $rts_projectile[0].rts_projectile_ability_core_frame_order_kind_labels,
      rts_projectile_ability_core_headless_checkpoint_sha256: $rts_projectile[0].rts_projectile_ability_core_headless_checkpoint_sha256,
      rts_projectile_ability_core_headless_applied_order_count: $rts_projectile[0].rts_projectile_ability_core_headless_applied_order_count,
      rts_projectile_ability_core_headless_actor_count: $rts_projectile[0].rts_projectile_ability_core_headless_actor_count,
      rts_projectile_ability_core_headless_final_frame: $rts_projectile[0].rts_projectile_ability_core_headless_final_frame,
      rts_projectile_ability_core_headless_ability_order_count: $rts_projectile[0].rts_projectile_ability_core_headless_ability_order_count,
      rts_projectile_ability_core_headless_ability_rule_ids: $rts_projectile[0].rts_projectile_ability_core_headless_ability_rule_ids,
      rts_projectile_ability_core_headless_ability_target_actor_ids: $rts_projectile[0].rts_projectile_ability_core_headless_ability_target_actor_ids,
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
      rts_objective_victory_loop_openra_target_commit: $rts_objective[0].openra_parity_target_commit,
      rts_objective_victory_loop_openra_target_natural_terminal: $rts_objective[0].openra_parity_target_natural_terminal,
      rts_objective_victory_loop_openra_target_winner_beacons: $rts_objective[0].openra_parity_target_winner_beacons,
      rts_objective_victory_loop_openra_target_total_beacons: $rts_objective[0].openra_parity_target_total_beacons,
      rts_objective_victory_loop_openra_target_hold_ticks: $rts_objective[0].openra_parity_target_hold_ticks,
      rts_objective_victory_loop_bevy_terminal_parity_claimed: $rts_objective[0].bevy_terminal_parity_claimed,
      rts_objective_victory_loop_bevy_controlled_beacons: $rts_objective[0].bevy_objective_controlled_beacons,
      rts_objective_victory_loop_bevy_total_beacons: $rts_objective[0].bevy_objective_total_beacons,
      rts_objective_victory_loop_bevy_control_ratio_percent: $rts_objective[0].bevy_objective_control_ratio_percent,
      rts_objective_victory_loop_bevy_hold_ticks: $rts_objective[0].bevy_objective_hold_ticks,
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
      rts_objective_victory_loop_core_frame_order_stream_sha256: $rts_objective[0].rts_objective_core_frame_order_stream_sha256,
      rts_objective_victory_loop_core_headless_checkpoint_sha256: $rts_objective[0].rts_objective_core_headless_checkpoint_sha256,
      rts_objective_victory_loop_core_frame_order_kinds: $rts_objective[0].rts_objective_core_frame_order_kind_labels,
      rts_objective_victory_loop_core_applied_order_count: $rts_objective[0].rts_objective_core_headless_applied_order_count,
      rts_objective_victory_loop_core_actor_count: $rts_objective[0].rts_objective_core_headless_actor_count,
      rts_objective_victory_loop_core_final_frame: $rts_objective[0].rts_objective_core_headless_final_frame,
      rts_objective_victory_loop_core_objective_order_count: $rts_objective[0].rts_objective_core_headless_objective_order_count,
      rts_objective_victory_loop_core_capture_order_count: $rts_objective[0].rts_objective_core_headless_capture_order_count,
      rts_objective_victory_loop_core_extract_order_count: $rts_objective[0].rts_objective_core_headless_extract_order_count,
      rts_objective_victory_loop_core_objective_ids: $rts_objective[0].rts_objective_core_headless_objective_ids,
      rts_objective_victory_loop_core_objective_tile_ids: $rts_objective[0].rts_objective_core_headless_objective_tile_ids,
      rts_autonomous_bot_skirmish_input_action_count: $rts_auto_bot[0].input_action_count,
      rts_autonomous_bot_skirmish_stage_count: ($rts_auto_bot[0].stage_summaries | length),
      rts_autonomous_bot_skirmish_winner: $rts_auto_bot[0].bevy_terminal_winner,
      rts_autonomous_bot_skirmish_winner_beacons: $rts_auto_bot[0].bevy_terminal_winner_beacons,
      rts_autonomous_bot_skirmish_total_beacons: $rts_auto_bot[0].bevy_terminal_total_beacons,
      rts_autonomous_bot_skirmish_hold_ticks: $rts_auto_bot[0].bevy_terminal_hold_ticks,
      rts_autonomous_bot_skirmish_parity_claimed: $rts_auto_bot[0].bevy_terminal_parity_claimed,
      rts_autonomous_bot_skirmish_match_result: $rts_auto_bot[0].final_match_result_state,
      rts_autonomous_bot_skirmish_spawned_unit_count: ($rts_auto_bot[0].final_army_spawned_unit_ids | length),
      rts_autonomous_bot_skirmish_supply_used: $rts_auto_bot[0].final_army_supply_used,
      rts_autonomous_bot_skirmish_supply_cap: $rts_auto_bot[0].final_army_supply_cap,
      rts_autonomous_bot_skirmish_pixel_count: (
        $rts_auto_bot[0].ai_wave_pixel_count
        + $rts_auto_bot[0].ai_pressure_pixel_count
        + $rts_auto_bot[0].ai_counter_pixel_count
        + $rts_auto_bot[0].objective_pixel_count
        + $rts_auto_bot[0].capture_bar_pixel_count
        + $rts_auto_bot[0].match_result_pixel_count
      ),
      rts_autonomous_bot_skirmish_ai_wave_pixel_count: $rts_auto_bot[0].ai_wave_pixel_count,
      rts_autonomous_bot_skirmish_ai_pressure_pixel_count: $rts_auto_bot[0].ai_pressure_pixel_count,
      rts_autonomous_bot_skirmish_ai_counter_pixel_count: $rts_auto_bot[0].ai_counter_pixel_count,
      rts_autonomous_bot_skirmish_objective_pixel_count: $rts_auto_bot[0].objective_pixel_count,
      rts_autonomous_bot_skirmish_capture_bar_pixel_count: $rts_auto_bot[0].capture_bar_pixel_count,
      rts_autonomous_bot_skirmish_match_result_pixel_count: $rts_auto_bot[0].match_result_pixel_count,
      rts_organic_terminal_gap_stage_count: ($rts_organic_terminal_gap[0].stage_summaries | length),
      rts_organic_terminal_gap_winner: $rts_organic_terminal_gap[0].bevy_terminal_winner,
      rts_organic_terminal_gap_winner_beacons: $rts_organic_terminal_gap[0].bevy_terminal_winner_beacons,
      rts_organic_terminal_gap_total_beacons: $rts_organic_terminal_gap[0].bevy_terminal_total_beacons,
      rts_organic_terminal_gap_hold_ticks: $rts_organic_terminal_gap[0].bevy_terminal_hold_ticks,
      rts_organic_terminal_gap_state: $rts_organic_terminal_gap[0].bevy_terminal_observation_gap_state,
      rts_organic_terminal_gap_openra_parity_target_commit: $rts_organic_terminal_gap[0].openra_parity_target_commit,
      rts_organic_terminal_gap_winner_count: $rts_organic_terminal_gap[0].winner_count,
      rts_organic_terminal_gap_loser_count: $rts_organic_terminal_gap[0].loser_count,
      rts_organic_terminal_gap_match_result: $rts_organic_terminal_gap[0].final_match_result_state,
      rts_organic_terminal_gap_pixel_count: (
        $rts_organic_terminal_gap[0].ai_wave_pixel_count
        + $rts_organic_terminal_gap[0].ai_pressure_pixel_count
        + $rts_organic_terminal_gap[0].objective_pixel_count
        + $rts_organic_terminal_gap[0].capture_bar_pixel_count
        + $rts_organic_terminal_gap[0].match_result_pixel_count
      ),
      rts_organic_terminal_gap_ai_wave_pixel_count: $rts_organic_terminal_gap[0].ai_wave_pixel_count,
      rts_organic_terminal_gap_ai_pressure_pixel_count: $rts_organic_terminal_gap[0].ai_pressure_pixel_count,
      rts_organic_terminal_gap_objective_pixel_count: $rts_organic_terminal_gap[0].objective_pixel_count,
      rts_organic_terminal_gap_capture_bar_pixel_count: $rts_organic_terminal_gap[0].capture_bar_pixel_count,
      rts_organic_terminal_gap_match_result_pixel_count: $rts_organic_terminal_gap[0].match_result_pixel_count,
      rts_terminal_observation_gap_stage_count: ($rts_terminal_observation_gap[0].stage_summaries | length),
      rts_terminal_observation_gap_winner: $rts_terminal_observation_gap[0].bevy_terminal_winner,
      rts_terminal_observation_gap_winner_beacons: $rts_terminal_observation_gap[0].bevy_terminal_winner_beacons,
      rts_terminal_observation_gap_total_beacons: $rts_terminal_observation_gap[0].bevy_terminal_total_beacons,
      rts_terminal_observation_gap_hold_ticks: $rts_terminal_observation_gap[0].bevy_terminal_hold_ticks,
      rts_terminal_observation_gap_state: $rts_terminal_observation_gap[0].bevy_terminal_observation_gap_state,
      rts_terminal_observation_gap_openra_readiness_commit: $rts_terminal_observation_gap[0].openra_terminal_readiness_target_commit,
      rts_terminal_observation_gap_openra_probe_commit: $rts_terminal_observation_gap[0].openra_terminal_probe_target_commit,
      rts_terminal_observation_gap_openra_strategic_commit: $rts_terminal_observation_gap[0].openra_strategic_terminal_target_commit,
      rts_terminal_observation_gap_terminal_rules_ready: $rts_terminal_observation_gap[0].terminal_victory_rules_ready,
      rts_terminal_observation_gap_game_over: $rts_terminal_observation_gap[0].terminal_probe_game_over,
      rts_terminal_observation_gap_loser_count: $rts_terminal_observation_gap[0].terminal_probe_loser_count,
      rts_terminal_observation_gap_match_result: $rts_terminal_observation_gap[0].final_match_result_state,
      rts_terminal_observation_gap_pixel_count: (
        $rts_terminal_observation_gap[0].ai_wave_pixel_count
        + $rts_terminal_observation_gap[0].ai_pressure_pixel_count
        + $rts_terminal_observation_gap[0].ai_counter_pixel_count
        + $rts_terminal_observation_gap[0].objective_pixel_count
        + $rts_terminal_observation_gap[0].capture_bar_pixel_count
        + $rts_terminal_observation_gap[0].match_result_pixel_count
      ),
      rts_terminal_observation_gap_ai_wave_pixel_count: $rts_terminal_observation_gap[0].ai_wave_pixel_count,
      rts_terminal_observation_gap_ai_pressure_pixel_count: $rts_terminal_observation_gap[0].ai_pressure_pixel_count,
      rts_terminal_observation_gap_ai_counter_pixel_count: $rts_terminal_observation_gap[0].ai_counter_pixel_count,
      rts_terminal_observation_gap_objective_pixel_count: $rts_terminal_observation_gap[0].objective_pixel_count,
      rts_terminal_observation_gap_capture_bar_pixel_count: $rts_terminal_observation_gap[0].capture_bar_pixel_count,
      rts_terminal_observation_gap_match_result_pixel_count: $rts_terminal_observation_gap[0].match_result_pixel_count,
      rts_replay_metrics_gap_stage_count: ($rts_replay_metrics_gap[0].stage_summaries | length),
      rts_replay_metrics_gap_state: $rts_replay_metrics_gap[0].bevy_replay_metrics_gap_state,
      rts_replay_metrics_gap_openra_replay_summary_commit: $rts_replay_metrics_gap[0].openra_replay_summary_target_commit,
      rts_replay_metrics_gap_openra_battle_outcome_commit: $rts_replay_metrics_gap[0].openra_battle_outcome_target_commit,
      rts_replay_metrics_gap_startgame_order: $rts_replay_metrics_gap[0].replay_startgame_order,
      rts_replay_metrics_gap_client_slot_count: ($rts_replay_metrics_gap[0].replay_client_slots | length),
      rts_replay_metrics_gap_bot_mentions: $rts_replay_metrics_gap[0].replay_bot_mentions,
      rts_replay_metrics_gap_actor_order_tokens: $rts_replay_metrics_gap[0].replay_actor_order_tokens,
      rts_replay_metrics_gap_unique_actor_token_count: $rts_replay_metrics_gap[0].replay_unique_actor_token_count,
      rts_replay_metrics_gap_economy_tokens: $rts_replay_metrics_gap[0].replay_economy_tokens,
      rts_replay_metrics_gap_tech_tokens: $rts_replay_metrics_gap[0].replay_tech_tokens,
      rts_replay_metrics_gap_combat_tokens: $rts_replay_metrics_gap[0].replay_combat_tokens,
      rts_replay_metrics_gap_configured_seconds: $rts_replay_metrics_gap[0].configured_seconds,
      rts_replay_metrics_gap_elapsed_seconds: $rts_replay_metrics_gap[0].elapsed_seconds,
      rts_replay_metrics_gap_outcome_signal: $rts_replay_metrics_gap[0].outcome_signal,
      rts_replay_metrics_gap_winner_claimed: $rts_replay_metrics_gap[0].winner_claimed,
      rts_replay_metrics_gap_pixel_count: (
        $rts_replay_metrics_gap[0].ai_wave_pixel_count
        + $rts_replay_metrics_gap[0].ai_pressure_pixel_count
        + $rts_replay_metrics_gap[0].ai_counter_pixel_count
        + $rts_replay_metrics_gap[0].objective_pixel_count
        + $rts_replay_metrics_gap[0].match_result_pixel_count
      ),
      rts_replay_metrics_gap_ai_wave_pixel_count: $rts_replay_metrics_gap[0].ai_wave_pixel_count,
      rts_replay_metrics_gap_ai_pressure_pixel_count: $rts_replay_metrics_gap[0].ai_pressure_pixel_count,
      rts_replay_metrics_gap_ai_counter_pixel_count: $rts_replay_metrics_gap[0].ai_counter_pixel_count,
      rts_replay_metrics_gap_objective_pixel_count: $rts_replay_metrics_gap[0].objective_pixel_count,
      rts_replay_metrics_gap_match_result_pixel_count: $rts_replay_metrics_gap[0].match_result_pixel_count,
      rts_endurance_skirmish_gap_stage_count: ($rts_endurance_skirmish_gap[0].stage_summaries | length),
      rts_endurance_skirmish_gap_state: $rts_endurance_skirmish_gap[0].bevy_endurance_skirmish_gap_state,
      rts_endurance_skirmish_gap_openra_endurance_commit: $rts_endurance_skirmish_gap[0].openra_endurance_skirmish_target_commit,
      rts_endurance_skirmish_gap_openra_longrun_commit: $rts_endurance_skirmish_gap[0].openra_longrun_skirmish_target_commit,
      rts_endurance_skirmish_gap_openra_autostart_commit: $rts_endurance_skirmish_gap[0].openra_multibot_autostart_target_commit,
      rts_endurance_skirmish_gap_startgame_order: $rts_endurance_skirmish_gap[0].endurance_startgame_order,
      rts_endurance_skirmish_gap_autostart_order: $rts_endurance_skirmish_gap[0].endurance_autostart_order,
      rts_endurance_skirmish_gap_client_slot_count: ($rts_endurance_skirmish_gap[0].endurance_client_slots | length),
      rts_endurance_skirmish_gap_configured_seconds: $rts_endurance_skirmish_gap[0].configured_seconds,
      rts_endurance_skirmish_gap_elapsed_seconds: $rts_endurance_skirmish_gap[0].elapsed_seconds,
      rts_endurance_skirmish_gap_peak_active_units: $rts_endurance_skirmish_gap[0].peak_active_units,
      rts_endurance_skirmish_gap_contested_beacon_peak: $rts_endurance_skirmish_gap[0].contested_beacon_peak,
      rts_endurance_skirmish_gap_economy_events: $rts_endurance_skirmish_gap[0].economy_event_count,
      rts_endurance_skirmish_gap_combat_events: $rts_endurance_skirmish_gap[0].combat_event_count,
      rts_endurance_skirmish_gap_tech_events: $rts_endurance_skirmish_gap[0].tech_event_count,
      rts_endurance_skirmish_gap_outcome_signal: $rts_endurance_skirmish_gap[0].outcome_signal,
      rts_endurance_skirmish_gap_winner_claimed: $rts_endurance_skirmish_gap[0].winner_claimed,
      rts_endurance_skirmish_gap_pixel_count: (
        $rts_endurance_skirmish_gap[0].ai_wave_pixel_count
        + $rts_endurance_skirmish_gap[0].ai_pressure_pixel_count
        + $rts_endurance_skirmish_gap[0].ai_counter_pixel_count
        + $rts_endurance_skirmish_gap[0].objective_pixel_count
        + $rts_endurance_skirmish_gap[0].match_result_pixel_count
      ),
      rts_endurance_skirmish_gap_ai_wave_pixel_count: $rts_endurance_skirmish_gap[0].ai_wave_pixel_count,
      rts_endurance_skirmish_gap_ai_pressure_pixel_count: $rts_endurance_skirmish_gap[0].ai_pressure_pixel_count,
      rts_endurance_skirmish_gap_ai_counter_pixel_count: $rts_endurance_skirmish_gap[0].ai_counter_pixel_count,
      rts_endurance_skirmish_gap_objective_pixel_count: $rts_endurance_skirmish_gap[0].objective_pixel_count,
      rts_endurance_skirmish_gap_match_result_pixel_count: $rts_endurance_skirmish_gap[0].match_result_pixel_count,
      rts_bot_decision_state_gap_stage_count: $rts_bot_decision_state_gap[0].bot_decision_stage_count,
      rts_bot_decision_state_gap_state: $rts_bot_decision_state_gap[0].bevy_bot_decision_gap_state,
      rts_bot_decision_state_gap_openra_economy_tech_commit: $rts_bot_decision_state_gap[0].openra_bot_economy_tech_target_commit,
      rts_bot_decision_state_gap_openra_beacon_pressure_commit: $rts_bot_decision_state_gap[0].openra_bot_beacon_pressure_target_commit,
      rts_bot_decision_state_gap_openra_organic_terminal_commit: $rts_bot_decision_state_gap[0].openra_organic_bot_terminal_target_commit,
      rts_bot_decision_state_gap_core_frame_order_stream_sha256: $rts_bot_decision_state_gap[0].rts_bot_decision_core_frame_order_stream_sha256,
      rts_bot_decision_state_gap_core_headless_checkpoint_sha256: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_checkpoint_sha256,
      rts_bot_decision_state_gap_core_frame_order_kinds: $rts_bot_decision_state_gap[0].rts_bot_decision_core_frame_order_kind_labels,
      rts_bot_decision_state_gap_core_headless_applied_orders: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_applied_order_count,
      rts_bot_decision_state_gap_core_headless_actor_count: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_actor_count,
      rts_bot_decision_state_gap_core_headless_final_frame: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_final_frame,
      rts_bot_decision_state_gap_core_headless_harvest_actor_orders: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_harvest_actor_order_count,
      rts_bot_decision_state_gap_core_headless_scout_orders: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_scout_order_count,
      rts_bot_decision_state_gap_core_headless_capture_orders: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_capture_order_count,
      rts_bot_decision_state_gap_core_headless_research_orders: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_research_order_count,
      rts_bot_decision_state_gap_core_headless_attack_orders: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_attack_order_count,
      rts_bot_decision_state_gap_core_headless_micro_move_orders: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_micro_move_order_count,
      rts_bot_decision_state_gap_core_headless_recon_ids: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_recon_ids,
      rts_bot_decision_state_gap_core_headless_objective_ids: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_objective_ids,
      rts_bot_decision_state_gap_core_headless_researched_rules: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_researched_rule_ids,
      rts_bot_decision_state_gap_core_headless_combat_targets: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_combat_target_actor_ids,
      rts_bot_decision_state_gap_core_headless_combat_tiles: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_combat_target_tile_ids,
      rts_bot_decision_state_gap_core_headless_combat_formations: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_combat_formation_ids,
      rts_bot_decision_state_gap_decision_signals: $rts_bot_decision_state_gap[0].decision_signal_count,
      rts_bot_decision_state_gap_economy_decisions: $rts_bot_decision_state_gap[0].economy_decision_count,
      rts_bot_decision_state_gap_objective_decisions: $rts_bot_decision_state_gap[0].objective_decision_count,
      rts_bot_decision_state_gap_combat_decisions: $rts_bot_decision_state_gap[0].combat_decision_count,
      rts_bot_decision_state_gap_tech_decisions: $rts_bot_decision_state_gap[0].tech_decision_count,
      rts_bot_decision_state_gap_final_state: $rts_bot_decision_state_gap[0].final_bot_decision_state,
      rts_bot_decision_state_gap_final_pressure_percent: $rts_bot_decision_state_gap[0].final_rts_ai_pressure_percent,
      rts_bot_decision_state_gap_final_defeat_risk_percent: $rts_bot_decision_state_gap[0].final_rts_defeat_risk_percent,
      rts_bot_decision_state_gap_final_capture_percent: $rts_bot_decision_state_gap[0].final_objective_capture_percent,
      rts_bot_decision_state_gap_match_result: $rts_bot_decision_state_gap[0].final_match_result_state,
      rts_bot_decision_state_gap_pixel_count: (
        $rts_bot_decision_state_gap[0].ai_wave_pixel_count
        + $rts_bot_decision_state_gap[0].ai_pressure_pixel_count
        + $rts_bot_decision_state_gap[0].ai_counter_pixel_count
        + $rts_bot_decision_state_gap[0].objective_pixel_count
        + $rts_bot_decision_state_gap[0].capture_bar_pixel_count
        + $rts_bot_decision_state_gap[0].match_result_pixel_count
      ),
      rts_bot_decision_state_gap_ai_wave_pixel_count: $rts_bot_decision_state_gap[0].ai_wave_pixel_count,
      rts_bot_decision_state_gap_ai_pressure_pixel_count: $rts_bot_decision_state_gap[0].ai_pressure_pixel_count,
      rts_bot_decision_state_gap_ai_counter_pixel_count: $rts_bot_decision_state_gap[0].ai_counter_pixel_count,
      rts_bot_decision_state_gap_objective_pixel_count: $rts_bot_decision_state_gap[0].objective_pixel_count,
      rts_bot_decision_state_gap_capture_bar_pixel_count: $rts_bot_decision_state_gap[0].capture_bar_pixel_count,
      rts_bot_decision_state_gap_match_result_pixel_count: $rts_bot_decision_state_gap[0].match_result_pixel_count,
      rts_bot_adaptive_build_order_gap_stage_count: $rts_bot_adaptive_build_order_gap[0].adaptive_stage_count,
      rts_bot_adaptive_build_order_gap_state: $rts_bot_adaptive_build_order_gap[0].bevy_bot_adaptive_build_gap_state,
	      rts_bot_adaptive_build_order_gap_openra_economy_tech_commit: $rts_bot_adaptive_build_order_gap[0].openra_bot_economy_tech_target_commit,
	      rts_bot_adaptive_build_order_gap_openra_beacon_pressure_commit: $rts_bot_adaptive_build_order_gap[0].openra_bot_beacon_pressure_target_commit,
	      rts_bot_adaptive_build_order_gap_openra_organic_terminal_commit: $rts_bot_adaptive_build_order_gap[0].openra_organic_bot_terminal_target_commit,
	      rts_bot_adaptive_build_order_gap_core_frame_order_stream_sha256: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_frame_order_stream_sha256,
	      rts_bot_adaptive_build_order_gap_core_headless_checkpoint_sha256: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_checkpoint_sha256,
	      rts_bot_adaptive_build_order_gap_core_frame_order_kinds: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_frame_order_kind_labels,
	      rts_bot_adaptive_build_order_gap_core_headless_applied_orders: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_applied_order_count,
	      rts_bot_adaptive_build_order_gap_core_headless_actor_count: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_actor_count,
	      rts_bot_adaptive_build_order_gap_core_headless_final_frame: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_final_frame,
	      rts_bot_adaptive_build_order_gap_core_headless_harvest_actor_orders: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_harvest_actor_order_count,
	      rts_bot_adaptive_build_order_gap_core_headless_build_orders: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_build_order_count,
	      rts_bot_adaptive_build_order_gap_core_headless_train_orders: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_train_order_count,
	      rts_bot_adaptive_build_order_gap_core_headless_build_rules: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_build_rule_ids,
	      rts_bot_adaptive_build_order_gap_core_headless_train_rules: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_train_rule_ids,
	      rts_bot_adaptive_build_order_gap_core_headless_scout_orders: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_scout_order_count,
	      rts_bot_adaptive_build_order_gap_core_headless_recon_ids: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_recon_ids,
	      rts_bot_adaptive_build_order_gap_core_headless_recon_tiles: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_recon_tile_ids,
	      rts_bot_adaptive_build_order_gap_core_headless_research_orders: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_research_order_count,
	      rts_bot_adaptive_build_order_gap_core_headless_researched_rules: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_researched_rule_ids,
	      rts_bot_adaptive_build_order_gap_core_headless_research_sources: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_research_source_actor_ids,
	      rts_bot_adaptive_build_order_gap_core_headless_attack_orders: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_attack_order_count,
	      rts_bot_adaptive_build_order_gap_core_headless_micro_move_orders: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_micro_move_order_count,
	      rts_bot_adaptive_build_order_gap_core_headless_combat_targets: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_combat_target_actor_ids,
	      rts_bot_adaptive_build_order_gap_core_headless_combat_tiles: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_combat_target_tile_ids,
	      rts_bot_adaptive_build_order_gap_core_headless_combat_formations: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_combat_formation_ids,
	      rts_bot_adaptive_build_order_gap_adaptive_signals: $rts_bot_adaptive_build_order_gap[0].adaptive_signal_count,
	      rts_bot_adaptive_build_order_gap_opening_build_orders: $rts_bot_adaptive_build_order_gap[0].opening_build_order_count,
      rts_bot_adaptive_build_order_gap_scout_triggers: $rts_bot_adaptive_build_order_gap[0].scout_trigger_count,
      rts_bot_adaptive_build_order_gap_branch_switches: $rts_bot_adaptive_build_order_gap[0].branch_switch_count,
      rts_bot_adaptive_build_order_gap_counter_tech_switches: $rts_bot_adaptive_build_order_gap[0].counter_tech_switch_count,
      rts_bot_adaptive_build_order_gap_pressure_windows: $rts_bot_adaptive_build_order_gap[0].pressure_window_count,
      rts_bot_adaptive_build_order_gap_retreat_rebuilds: $rts_bot_adaptive_build_order_gap[0].retreat_rebuild_count,
      rts_bot_adaptive_build_order_gap_final_state: $rts_bot_adaptive_build_order_gap[0].final_adaptive_state,
      rts_bot_adaptive_build_order_gap_final_pressure_percent: $rts_bot_adaptive_build_order_gap[0].final_rts_ai_pressure_percent,
      rts_bot_adaptive_build_order_gap_final_defeat_risk_percent: $rts_bot_adaptive_build_order_gap[0].final_rts_defeat_risk_percent,
      rts_bot_adaptive_build_order_gap_final_capture_percent: $rts_bot_adaptive_build_order_gap[0].final_objective_capture_percent,
      rts_bot_adaptive_build_order_gap_match_result: $rts_bot_adaptive_build_order_gap[0].final_match_result_state,
      rts_bot_adaptive_build_order_gap_pixel_count: (
        $rts_bot_adaptive_build_order_gap[0].ai_wave_pixel_count
        + $rts_bot_adaptive_build_order_gap[0].ai_pressure_pixel_count
        + $rts_bot_adaptive_build_order_gap[0].ai_counter_pixel_count
        + $rts_bot_adaptive_build_order_gap[0].objective_pixel_count
        + $rts_bot_adaptive_build_order_gap[0].capture_bar_pixel_count
        + $rts_bot_adaptive_build_order_gap[0].match_result_pixel_count
      ),
      rts_bot_adaptive_build_order_gap_ai_wave_pixel_count: $rts_bot_adaptive_build_order_gap[0].ai_wave_pixel_count,
      rts_bot_adaptive_build_order_gap_ai_pressure_pixel_count: $rts_bot_adaptive_build_order_gap[0].ai_pressure_pixel_count,
      rts_bot_adaptive_build_order_gap_ai_counter_pixel_count: $rts_bot_adaptive_build_order_gap[0].ai_counter_pixel_count,
      rts_bot_adaptive_build_order_gap_objective_pixel_count: $rts_bot_adaptive_build_order_gap[0].objective_pixel_count,
      rts_bot_adaptive_build_order_gap_capture_bar_pixel_count: $rts_bot_adaptive_build_order_gap[0].capture_bar_pixel_count,
      rts_bot_adaptive_build_order_gap_match_result_pixel_count: $rts_bot_adaptive_build_order_gap[0].match_result_pixel_count,
      rts_bot_tactical_micro_gap_stage_count: $rts_bot_tactical_micro_gap[0].micro_stage_count,
      rts_bot_tactical_micro_gap_state: $rts_bot_tactical_micro_gap[0].bevy_bot_tactical_micro_gap_state,
      rts_bot_tactical_micro_gap_openra_economy_tech_commit: $rts_bot_tactical_micro_gap[0].openra_bot_economy_tech_target_commit,
      rts_bot_tactical_micro_gap_openra_beacon_pressure_commit: $rts_bot_tactical_micro_gap[0].openra_bot_beacon_pressure_target_commit,
      rts_bot_tactical_micro_gap_openra_organic_terminal_commit: $rts_bot_tactical_micro_gap[0].openra_organic_bot_terminal_target_commit,
      rts_bot_tactical_micro_gap_core_frame_order_stream_sha256: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_frame_order_stream_sha256,
      rts_bot_tactical_micro_gap_core_headless_checkpoint_sha256: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_checkpoint_sha256,
      rts_bot_tactical_micro_gap_core_frame_order_kinds: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_frame_order_kind_labels,
      rts_bot_tactical_micro_gap_core_headless_applied_orders: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_applied_order_count,
      rts_bot_tactical_micro_gap_core_headless_actor_count: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_actor_count,
      rts_bot_tactical_micro_gap_core_headless_final_frame: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_final_frame,
      rts_bot_tactical_micro_gap_core_headless_attack_orders: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_attack_order_count,
      rts_bot_tactical_micro_gap_core_headless_focus_fire_orders: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_focus_fire_order_count,
      rts_bot_tactical_micro_gap_core_headless_micro_move_orders: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_micro_move_order_count,
      rts_bot_tactical_micro_gap_core_headless_ability_orders: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_ability_order_count,
      rts_bot_tactical_micro_gap_core_headless_combat_targets: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_combat_target_actor_ids,
      rts_bot_tactical_micro_gap_core_headless_combat_tiles: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_combat_target_tile_ids,
      rts_bot_tactical_micro_gap_core_headless_combat_formations: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_combat_formation_ids,
      rts_bot_tactical_micro_gap_core_headless_ability_rules: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_ability_rule_ids,
      rts_bot_tactical_micro_gap_core_headless_ability_targets: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_ability_target_actor_ids,
      rts_bot_tactical_micro_gap_micro_signals: $rts_bot_tactical_micro_gap[0].micro_signal_count,
      rts_bot_tactical_micro_gap_target_swaps: $rts_bot_tactical_micro_gap[0].target_swap_count,
      rts_bot_tactical_micro_gap_focus_fire_orders: $rts_bot_tactical_micro_gap[0].focus_fire_order_count,
      rts_bot_tactical_micro_gap_kite_steps: $rts_bot_tactical_micro_gap[0].kite_step_count,
      rts_bot_tactical_micro_gap_flank_angles: $rts_bot_tactical_micro_gap[0].flank_angle_count,
      rts_bot_tactical_micro_gap_ability_timings: $rts_bot_tactical_micro_gap[0].ability_timing_count,
      rts_bot_tactical_micro_gap_low_health_pullbacks: $rts_bot_tactical_micro_gap[0].low_health_pullback_count,
      rts_bot_tactical_micro_gap_final_state: $rts_bot_tactical_micro_gap[0].final_micro_state,
      rts_bot_tactical_micro_gap_final_pressure_percent: $rts_bot_tactical_micro_gap[0].final_rts_ai_pressure_percent,
      rts_bot_tactical_micro_gap_final_defeat_risk_percent: $rts_bot_tactical_micro_gap[0].final_rts_defeat_risk_percent,
      rts_bot_tactical_micro_gap_final_capture_percent: $rts_bot_tactical_micro_gap[0].final_objective_capture_percent,
      rts_bot_tactical_micro_gap_match_result: $rts_bot_tactical_micro_gap[0].final_match_result_state,
      rts_bot_tactical_micro_gap_pixel_count: (
        $rts_bot_tactical_micro_gap[0].ai_wave_pixel_count
        + $rts_bot_tactical_micro_gap[0].ai_pressure_pixel_count
        + $rts_bot_tactical_micro_gap[0].ai_counter_pixel_count
        + $rts_bot_tactical_micro_gap[0].objective_pixel_count
        + $rts_bot_tactical_micro_gap[0].capture_bar_pixel_count
        + $rts_bot_tactical_micro_gap[0].match_result_pixel_count
      ),
      rts_bot_tactical_micro_gap_ai_wave_pixel_count: $rts_bot_tactical_micro_gap[0].ai_wave_pixel_count,
      rts_bot_tactical_micro_gap_ai_pressure_pixel_count: $rts_bot_tactical_micro_gap[0].ai_pressure_pixel_count,
      rts_bot_tactical_micro_gap_ai_counter_pixel_count: $rts_bot_tactical_micro_gap[0].ai_counter_pixel_count,
      rts_bot_tactical_micro_gap_objective_pixel_count: $rts_bot_tactical_micro_gap[0].objective_pixel_count,
      rts_bot_tactical_micro_gap_capture_bar_pixel_count: $rts_bot_tactical_micro_gap[0].capture_bar_pixel_count,
      rts_bot_tactical_micro_gap_match_result_pixel_count: $rts_bot_tactical_micro_gap[0].match_result_pixel_count,
      rts_bot_map_intel_gap_stage_count: $rts_bot_map_intel_gap[0].intel_stage_count,
      rts_bot_map_intel_gap_state: $rts_bot_map_intel_gap[0].bevy_bot_map_intel_gap_state,
      rts_bot_map_intel_gap_openra_economy_tech_commit: $rts_bot_map_intel_gap[0].openra_bot_economy_tech_target_commit,
      rts_bot_map_intel_gap_openra_beacon_pressure_commit: $rts_bot_map_intel_gap[0].openra_bot_beacon_pressure_target_commit,
      rts_bot_map_intel_gap_openra_organic_terminal_commit: $rts_bot_map_intel_gap[0].openra_organic_bot_terminal_target_commit,
      rts_bot_map_intel_gap_core_frame_order_stream_sha256: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_frame_order_stream_sha256,
      rts_bot_map_intel_gap_core_headless_checkpoint_sha256: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_checkpoint_sha256,
      rts_bot_map_intel_gap_core_frame_order_kinds: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_frame_order_kind_labels,
      rts_bot_map_intel_gap_core_headless_applied_orders: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_applied_order_count,
      rts_bot_map_intel_gap_core_headless_actor_count: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_actor_count,
      rts_bot_map_intel_gap_core_headless_final_frame: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_final_frame,
      rts_bot_map_intel_gap_core_headless_recon_orders: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_recon_order_count,
      rts_bot_map_intel_gap_core_headless_scout_orders: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_scout_order_count,
      rts_bot_map_intel_gap_core_headless_mark_orders: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_mark_order_count,
      rts_bot_map_intel_gap_core_headless_sweep_orders: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_sweep_order_count,
      rts_bot_map_intel_gap_core_headless_scan_orders: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_scan_order_count,
      rts_bot_map_intel_gap_core_headless_recon_ids: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_recon_ids,
      rts_bot_map_intel_gap_core_headless_recon_tiles: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_recon_tile_ids,
      rts_bot_map_intel_gap_core_headless_micro_move_orders: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_micro_move_order_count,
      rts_bot_map_intel_gap_core_headless_combat_tiles: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_combat_target_tile_ids,
      rts_bot_map_intel_gap_core_headless_combat_formations: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_combat_formation_ids,
      rts_bot_map_intel_gap_intel_signals: $rts_bot_map_intel_gap[0].intel_signal_count,
      rts_bot_map_intel_gap_scout_sweeps: $rts_bot_map_intel_gap[0].scout_sweep_count,
      rts_bot_map_intel_gap_fog_memory_stamps: $rts_bot_map_intel_gap[0].fog_memory_stamp_count,
      rts_bot_map_intel_gap_expansion_threats: $rts_bot_map_intel_gap[0].expansion_threat_count,
      rts_bot_map_intel_gap_enemy_tech_reads: $rts_bot_map_intel_gap[0].enemy_tech_read_count,
      rts_bot_map_intel_gap_hidden_army_predictions: $rts_bot_map_intel_gap[0].hidden_army_prediction_count,
      rts_bot_map_intel_gap_pressure_rotations: $rts_bot_map_intel_gap[0].pressure_rotation_count,
      rts_bot_map_intel_gap_final_state: $rts_bot_map_intel_gap[0].final_intel_state,
      rts_bot_map_intel_gap_final_pressure_percent: $rts_bot_map_intel_gap[0].final_rts_ai_pressure_percent,
      rts_bot_map_intel_gap_final_defeat_risk_percent: $rts_bot_map_intel_gap[0].final_rts_defeat_risk_percent,
      rts_bot_map_intel_gap_final_capture_percent: $rts_bot_map_intel_gap[0].final_objective_capture_percent,
      rts_bot_map_intel_gap_match_result: $rts_bot_map_intel_gap[0].final_match_result_state,
      rts_bot_map_intel_gap_pixel_count: (
        $rts_bot_map_intel_gap[0].ai_wave_pixel_count
        + $rts_bot_map_intel_gap[0].ai_pressure_pixel_count
        + $rts_bot_map_intel_gap[0].ai_counter_pixel_count
        + $rts_bot_map_intel_gap[0].objective_pixel_count
        + $rts_bot_map_intel_gap[0].capture_bar_pixel_count
        + $rts_bot_map_intel_gap[0].match_result_pixel_count
      ),
      rts_bot_map_intel_gap_ai_wave_pixel_count: $rts_bot_map_intel_gap[0].ai_wave_pixel_count,
      rts_bot_map_intel_gap_ai_pressure_pixel_count: $rts_bot_map_intel_gap[0].ai_pressure_pixel_count,
      rts_bot_map_intel_gap_ai_counter_pixel_count: $rts_bot_map_intel_gap[0].ai_counter_pixel_count,
      rts_bot_map_intel_gap_objective_pixel_count: $rts_bot_map_intel_gap[0].objective_pixel_count,
      rts_bot_map_intel_gap_capture_bar_pixel_count: $rts_bot_map_intel_gap[0].capture_bar_pixel_count,
      rts_bot_map_intel_gap_match_result_pixel_count: $rts_bot_map_intel_gap[0].match_result_pixel_count,
      rts_bot_macro_economy_gap_stage_count: $rts_bot_macro_economy_gap[0].macro_stage_count,
      rts_bot_macro_economy_gap_state: $rts_bot_macro_economy_gap[0].bevy_bot_macro_economy_gap_state,
      rts_bot_macro_economy_gap_openra_economy_tech_commit: $rts_bot_macro_economy_gap[0].openra_bot_economy_tech_target_commit,
      rts_bot_macro_economy_gap_openra_beacon_pressure_commit: $rts_bot_macro_economy_gap[0].openra_bot_beacon_pressure_target_commit,
      rts_bot_macro_economy_gap_openra_organic_terminal_commit: $rts_bot_macro_economy_gap[0].openra_organic_bot_terminal_target_commit,
      rts_bot_macro_economy_gap_core_frame_order_stream_sha256: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_frame_order_stream_sha256,
      rts_bot_macro_economy_gap_core_headless_checkpoint_sha256: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_checkpoint_sha256,
      rts_bot_macro_economy_gap_core_frame_order_kinds: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_frame_order_kind_labels,
      rts_bot_macro_economy_gap_core_headless_applied_orders: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_applied_order_count,
      rts_bot_macro_economy_gap_core_headless_actor_count: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_actor_count,
      rts_bot_macro_economy_gap_core_headless_final_frame: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_final_frame,
      rts_bot_macro_economy_gap_core_headless_harvest_actor_orders: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_harvest_actor_order_count,
      rts_bot_macro_economy_gap_core_headless_build_orders: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_build_order_count,
      rts_bot_macro_economy_gap_core_headless_train_orders: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_train_order_count,
      rts_bot_macro_economy_gap_core_headless_build_rules: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_build_rule_ids,
      rts_bot_macro_economy_gap_core_headless_train_rules: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_train_rule_ids,
      rts_bot_macro_economy_gap_core_headless_research_orders: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_research_order_count,
      rts_bot_macro_economy_gap_core_headless_researched_rules: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_researched_rule_ids,
      rts_bot_macro_economy_gap_core_headless_research_source_actor_ids: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_research_source_actor_ids,
      rts_bot_macro_economy_gap_core_headless_attack_orders: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_attack_order_count,
      rts_bot_macro_economy_gap_core_headless_micro_move_orders: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_micro_move_order_count,
      rts_bot_macro_economy_gap_core_headless_combat_targets: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_combat_target_actor_ids,
      rts_bot_macro_economy_gap_core_headless_combat_tiles: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_combat_target_tile_ids,
      rts_bot_macro_economy_gap_core_headless_combat_formations: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_combat_formation_ids,
      rts_bot_macro_economy_gap_macro_signals: $rts_bot_macro_economy_gap[0].macro_signal_count,
      rts_bot_macro_economy_gap_worker_saturation: $rts_bot_macro_economy_gap[0].worker_saturation_count,
      rts_bot_macro_economy_gap_expansion_timings: $rts_bot_macro_economy_gap[0].expansion_timing_count,
      rts_bot_macro_economy_gap_supply_recoveries: $rts_bot_macro_economy_gap[0].supply_recovery_count,
      rts_bot_macro_economy_gap_production_cycles: $rts_bot_macro_economy_gap[0].production_cycle_count,
      rts_bot_macro_economy_gap_tech_ramps: $rts_bot_macro_economy_gap[0].tech_ramp_count,
      rts_bot_macro_economy_gap_resource_denies: $rts_bot_macro_economy_gap[0].resource_deny_count,
      rts_bot_macro_economy_gap_final_state: $rts_bot_macro_economy_gap[0].final_macro_state,
      rts_bot_macro_economy_gap_final_pressure_percent: $rts_bot_macro_economy_gap[0].final_rts_ai_pressure_percent,
      rts_bot_macro_economy_gap_final_defeat_risk_percent: $rts_bot_macro_economy_gap[0].final_rts_defeat_risk_percent,
      rts_bot_macro_economy_gap_final_capture_percent: $rts_bot_macro_economy_gap[0].final_objective_capture_percent,
      rts_bot_macro_economy_gap_match_result: $rts_bot_macro_economy_gap[0].final_match_result_state,
      rts_bot_macro_economy_gap_pixel_count: (
        $rts_bot_macro_economy_gap[0].ai_wave_pixel_count
        + $rts_bot_macro_economy_gap[0].ai_pressure_pixel_count
        + $rts_bot_macro_economy_gap[0].ai_counter_pixel_count
        + $rts_bot_macro_economy_gap[0].objective_pixel_count
        + $rts_bot_macro_economy_gap[0].capture_bar_pixel_count
        + $rts_bot_macro_economy_gap[0].match_result_pixel_count
      ),
      rts_bot_macro_economy_gap_ai_wave_pixel_count: $rts_bot_macro_economy_gap[0].ai_wave_pixel_count,
      rts_bot_macro_economy_gap_ai_pressure_pixel_count: $rts_bot_macro_economy_gap[0].ai_pressure_pixel_count,
      rts_bot_macro_economy_gap_ai_counter_pixel_count: $rts_bot_macro_economy_gap[0].ai_counter_pixel_count,
      rts_bot_macro_economy_gap_objective_pixel_count: $rts_bot_macro_economy_gap[0].objective_pixel_count,
      rts_bot_macro_economy_gap_capture_bar_pixel_count: $rts_bot_macro_economy_gap[0].capture_bar_pixel_count,
      rts_bot_macro_economy_gap_match_result_pixel_count: $rts_bot_macro_economy_gap[0].match_result_pixel_count,
      rts_bot_harassment_defense_gap_stage_count: $rts_bot_harassment_defense_gap[0].harassment_stage_count,
      rts_bot_harassment_defense_gap_state: $rts_bot_harassment_defense_gap[0].bevy_bot_harassment_defense_gap_state,
      rts_bot_harassment_defense_gap_openra_economy_tech_commit: $rts_bot_harassment_defense_gap[0].openra_bot_economy_tech_target_commit,
      rts_bot_harassment_defense_gap_openra_beacon_pressure_commit: $rts_bot_harassment_defense_gap[0].openra_bot_beacon_pressure_target_commit,
      rts_bot_harassment_defense_gap_openra_organic_terminal_commit: $rts_bot_harassment_defense_gap[0].openra_organic_bot_terminal_target_commit,
      rts_bot_harassment_defense_gap_harassment_signals: $rts_bot_harassment_defense_gap[0].harassment_signal_count,
      rts_bot_harassment_defense_gap_worker_pullbacks: $rts_bot_harassment_defense_gap[0].worker_pullback_count,
      rts_bot_harassment_defense_gap_repair_cycles: $rts_bot_harassment_defense_gap[0].repair_cycle_count,
      rts_bot_harassment_defense_gap_static_defense_responses: $rts_bot_harassment_defense_gap[0].static_defense_response_count,
      rts_bot_harassment_defense_gap_counter_raids: $rts_bot_harassment_defense_gap[0].counter_raid_count,
      rts_bot_harassment_defense_gap_retreat_paths: $rts_bot_harassment_defense_gap[0].retreat_path_count,
      rts_bot_harassment_defense_gap_rebuild_secures: $rts_bot_harassment_defense_gap[0].rebuild_secure_count,
      rts_bot_harassment_defense_gap_final_state: $rts_bot_harassment_defense_gap[0].final_harassment_state,
      rts_bot_harassment_defense_gap_final_pressure_percent: $rts_bot_harassment_defense_gap[0].final_rts_ai_pressure_percent,
      rts_bot_harassment_defense_gap_final_defeat_risk_percent: $rts_bot_harassment_defense_gap[0].final_rts_defeat_risk_percent,
      rts_bot_harassment_defense_gap_final_capture_percent: $rts_bot_harassment_defense_gap[0].final_objective_capture_percent,
      rts_bot_harassment_defense_gap_match_result: $rts_bot_harassment_defense_gap[0].final_match_result_state,
      rts_bot_harassment_defense_gap_pixel_count: (
        $rts_bot_harassment_defense_gap[0].ai_wave_pixel_count
        + $rts_bot_harassment_defense_gap[0].ai_pressure_pixel_count
        + $rts_bot_harassment_defense_gap[0].ai_counter_pixel_count
        + $rts_bot_harassment_defense_gap[0].objective_pixel_count
        + $rts_bot_harassment_defense_gap[0].capture_bar_pixel_count
        + $rts_bot_harassment_defense_gap[0].match_result_pixel_count
      ),
      rts_bot_harassment_defense_gap_ai_wave_pixel_count: $rts_bot_harassment_defense_gap[0].ai_wave_pixel_count,
      rts_bot_harassment_defense_gap_ai_pressure_pixel_count: $rts_bot_harassment_defense_gap[0].ai_pressure_pixel_count,
      rts_bot_harassment_defense_gap_ai_counter_pixel_count: $rts_bot_harassment_defense_gap[0].ai_counter_pixel_count,
      rts_bot_harassment_defense_gap_objective_pixel_count: $rts_bot_harassment_defense_gap[0].objective_pixel_count,
      rts_bot_harassment_defense_gap_capture_bar_pixel_count: $rts_bot_harassment_defense_gap[0].capture_bar_pixel_count,
      rts_bot_harassment_defense_gap_match_result_pixel_count: $rts_bot_harassment_defense_gap[0].match_result_pixel_count,
      rts_bot_multi_front_pressure_gap_stage_count: $rts_bot_multi_front_pressure_gap[0].multi_front_stage_count,
      rts_bot_multi_front_pressure_gap_state: $rts_bot_multi_front_pressure_gap[0].bevy_bot_multi_front_pressure_gap_state,
      rts_bot_multi_front_pressure_gap_openra_economy_tech_commit: $rts_bot_multi_front_pressure_gap[0].openra_bot_economy_tech_target_commit,
      rts_bot_multi_front_pressure_gap_openra_beacon_pressure_commit: $rts_bot_multi_front_pressure_gap[0].openra_bot_beacon_pressure_target_commit,
      rts_bot_multi_front_pressure_gap_openra_organic_terminal_commit: $rts_bot_multi_front_pressure_gap[0].openra_organic_bot_terminal_target_commit,
      rts_bot_multi_front_pressure_gap_multi_front_signals: $rts_bot_multi_front_pressure_gap[0].multi_front_signal_count,
      rts_bot_multi_front_pressure_gap_split_lanes: $rts_bot_multi_front_pressure_gap[0].split_lane_count,
      rts_bot_multi_front_pressure_gap_decoy_pressures: $rts_bot_multi_front_pressure_gap[0].decoy_pressure_count,
      rts_bot_multi_front_pressure_gap_rotations: $rts_bot_multi_front_pressure_gap[0].rotation_count,
      rts_bot_multi_front_pressure_gap_reinforce_joins: $rts_bot_multi_front_pressure_gap[0].reinforce_join_count,
      rts_bot_multi_front_pressure_gap_simultaneous_hits: $rts_bot_multi_front_pressure_gap[0].simultaneous_hit_count,
      rts_bot_multi_front_pressure_gap_terminal_collapses: $rts_bot_multi_front_pressure_gap[0].terminal_collapse_count,
      rts_bot_multi_front_pressure_gap_final_state: $rts_bot_multi_front_pressure_gap[0].final_multi_front_state,
      rts_bot_multi_front_pressure_gap_final_pressure_percent: $rts_bot_multi_front_pressure_gap[0].final_rts_ai_pressure_percent,
      rts_bot_multi_front_pressure_gap_final_defeat_risk_percent: $rts_bot_multi_front_pressure_gap[0].final_rts_defeat_risk_percent,
      rts_bot_multi_front_pressure_gap_final_capture_percent: $rts_bot_multi_front_pressure_gap[0].final_objective_capture_percent,
      rts_bot_multi_front_pressure_gap_match_result: $rts_bot_multi_front_pressure_gap[0].final_match_result_state,
      rts_bot_multi_front_pressure_gap_pixel_count: (
        $rts_bot_multi_front_pressure_gap[0].ai_wave_pixel_count
        + $rts_bot_multi_front_pressure_gap[0].ai_pressure_pixel_count
        + $rts_bot_multi_front_pressure_gap[0].ai_counter_pixel_count
        + $rts_bot_multi_front_pressure_gap[0].objective_pixel_count
        + $rts_bot_multi_front_pressure_gap[0].capture_bar_pixel_count
        + $rts_bot_multi_front_pressure_gap[0].match_result_pixel_count
      ),
      rts_bot_multi_front_pressure_gap_ai_wave_pixel_count: $rts_bot_multi_front_pressure_gap[0].ai_wave_pixel_count,
      rts_bot_multi_front_pressure_gap_ai_pressure_pixel_count: $rts_bot_multi_front_pressure_gap[0].ai_pressure_pixel_count,
      rts_bot_multi_front_pressure_gap_ai_counter_pixel_count: $rts_bot_multi_front_pressure_gap[0].ai_counter_pixel_count,
      rts_bot_multi_front_pressure_gap_objective_pixel_count: $rts_bot_multi_front_pressure_gap[0].objective_pixel_count,
      rts_bot_multi_front_pressure_gap_capture_bar_pixel_count: $rts_bot_multi_front_pressure_gap[0].capture_bar_pixel_count,
      rts_bot_multi_front_pressure_gap_match_result_pixel_count: $rts_bot_multi_front_pressure_gap[0].match_result_pixel_count,
      rts_bot_expansion_control_gap_stage_count: $rts_bot_expansion_control_gap[0].expansion_control_stage_count,
      rts_bot_expansion_control_gap_state: $rts_bot_expansion_control_gap[0].bevy_bot_expansion_control_gap_state,
      rts_bot_expansion_control_gap_openra_economy_tech_commit: $rts_bot_expansion_control_gap[0].openra_bot_economy_tech_target_commit,
      rts_bot_expansion_control_gap_openra_beacon_pressure_commit: $rts_bot_expansion_control_gap[0].openra_bot_beacon_pressure_target_commit,
      rts_bot_expansion_control_gap_openra_organic_terminal_commit: $rts_bot_expansion_control_gap[0].openra_organic_bot_terminal_target_commit,
      rts_bot_expansion_control_gap_expansion_control_signals: $rts_bot_expansion_control_gap[0].expansion_control_signal_count,
      rts_bot_expansion_control_gap_natural_probes: $rts_bot_expansion_control_gap[0].natural_probe_count,
      rts_bot_expansion_control_gap_third_node_denies: $rts_bot_expansion_control_gap[0].third_node_deny_count,
      rts_bot_expansion_control_gap_refinery_pickoffs: $rts_bot_expansion_control_gap[0].refinery_pickoff_count,
      rts_bot_expansion_control_gap_contain_rings: $rts_bot_expansion_control_gap[0].contain_ring_count,
      rts_bot_expansion_control_gap_reexpand_punishes: $rts_bot_expansion_control_gap[0].reexpand_punish_count,
      rts_bot_expansion_control_gap_map_locks: $rts_bot_expansion_control_gap[0].map_lock_count,
      rts_bot_expansion_control_gap_final_state: $rts_bot_expansion_control_gap[0].final_expansion_control_state,
      rts_bot_expansion_control_gap_final_pressure_percent: $rts_bot_expansion_control_gap[0].final_rts_ai_pressure_percent,
      rts_bot_expansion_control_gap_final_defeat_risk_percent: $rts_bot_expansion_control_gap[0].final_rts_defeat_risk_percent,
      rts_bot_expansion_control_gap_final_capture_percent: $rts_bot_expansion_control_gap[0].final_objective_capture_percent,
      rts_bot_expansion_control_gap_match_result: $rts_bot_expansion_control_gap[0].final_match_result_state,
      rts_bot_expansion_control_gap_pixel_count: (
        $rts_bot_expansion_control_gap[0].ai_wave_pixel_count
        + $rts_bot_expansion_control_gap[0].ai_pressure_pixel_count
        + $rts_bot_expansion_control_gap[0].ai_counter_pixel_count
        + $rts_bot_expansion_control_gap[0].objective_pixel_count
        + $rts_bot_expansion_control_gap[0].capture_bar_pixel_count
        + $rts_bot_expansion_control_gap[0].match_result_pixel_count
      ),
      rts_bot_expansion_control_gap_ai_wave_pixel_count: $rts_bot_expansion_control_gap[0].ai_wave_pixel_count,
      rts_bot_expansion_control_gap_ai_pressure_pixel_count: $rts_bot_expansion_control_gap[0].ai_pressure_pixel_count,
      rts_bot_expansion_control_gap_ai_counter_pixel_count: $rts_bot_expansion_control_gap[0].ai_counter_pixel_count,
      rts_bot_expansion_control_gap_objective_pixel_count: $rts_bot_expansion_control_gap[0].objective_pixel_count,
      rts_bot_expansion_control_gap_capture_bar_pixel_count: $rts_bot_expansion_control_gap[0].capture_bar_pixel_count,
      rts_bot_expansion_control_gap_match_result_pixel_count: $rts_bot_expansion_control_gap[0].match_result_pixel_count,
      rts_bot_tech_transition_gap_stage_count: $rts_bot_tech_transition_gap[0].tech_transition_stage_count,
      rts_bot_tech_transition_gap_state: $rts_bot_tech_transition_gap[0].bevy_bot_tech_transition_gap_state,
      rts_bot_tech_transition_gap_openra_economy_tech_commit: $rts_bot_tech_transition_gap[0].openra_bot_economy_tech_target_commit,
      rts_bot_tech_transition_gap_openra_beacon_pressure_commit: $rts_bot_tech_transition_gap[0].openra_bot_beacon_pressure_target_commit,
      rts_bot_tech_transition_gap_openra_organic_terminal_commit: $rts_bot_tech_transition_gap[0].openra_organic_bot_terminal_target_commit,
      rts_bot_tech_transition_gap_tech_transition_signals: $rts_bot_tech_transition_gap[0].tech_transition_signal_count,
      rts_bot_tech_transition_gap_signal_reads: $rts_bot_tech_transition_gap[0].signal_read_count,
      rts_bot_tech_transition_gap_counter_switches: $rts_bot_tech_transition_gap[0].counter_switch_count,
      rts_bot_tech_transition_gap_anti_air_timings: $rts_bot_tech_transition_gap[0].anti_air_timing_count,
      rts_bot_tech_transition_gap_siege_responses: $rts_bot_tech_transition_gap[0].siege_response_count,
      rts_bot_tech_transition_gap_upgrade_windows: $rts_bot_tech_transition_gap[0].upgrade_window_count,
      rts_bot_tech_transition_gap_terminal_tech_locks: $rts_bot_tech_transition_gap[0].terminal_tech_lock_count,
      rts_bot_tech_transition_gap_final_state: $rts_bot_tech_transition_gap[0].final_tech_transition_state,
      rts_bot_tech_transition_gap_final_pressure_percent: $rts_bot_tech_transition_gap[0].final_rts_ai_pressure_percent,
      rts_bot_tech_transition_gap_final_defeat_risk_percent: $rts_bot_tech_transition_gap[0].final_rts_defeat_risk_percent,
      rts_bot_tech_transition_gap_final_capture_percent: $rts_bot_tech_transition_gap[0].final_objective_capture_percent,
      rts_bot_tech_transition_gap_match_result: $rts_bot_tech_transition_gap[0].final_match_result_state,
      rts_bot_tech_transition_gap_pixel_count: (
        $rts_bot_tech_transition_gap[0].ai_wave_pixel_count
        + $rts_bot_tech_transition_gap[0].ai_pressure_pixel_count
        + $rts_bot_tech_transition_gap[0].ai_counter_pixel_count
        + $rts_bot_tech_transition_gap[0].objective_pixel_count
        + $rts_bot_tech_transition_gap[0].capture_bar_pixel_count
        + $rts_bot_tech_transition_gap[0].match_result_pixel_count
      ),
      rts_bot_tech_transition_gap_ai_wave_pixel_count: $rts_bot_tech_transition_gap[0].ai_wave_pixel_count,
      rts_bot_tech_transition_gap_ai_pressure_pixel_count: $rts_bot_tech_transition_gap[0].ai_pressure_pixel_count,
      rts_bot_tech_transition_gap_ai_counter_pixel_count: $rts_bot_tech_transition_gap[0].ai_counter_pixel_count,
      rts_bot_tech_transition_gap_objective_pixel_count: $rts_bot_tech_transition_gap[0].objective_pixel_count,
      rts_bot_tech_transition_gap_capture_bar_pixel_count: $rts_bot_tech_transition_gap[0].capture_bar_pixel_count,
      rts_bot_tech_transition_gap_match_result_pixel_count: $rts_bot_tech_transition_gap[0].match_result_pixel_count,
      rts_bot_army_composition_gap_stage_count: $rts_bot_army_composition_gap[0].army_composition_stage_count,
      rts_bot_army_composition_gap_state: $rts_bot_army_composition_gap[0].bevy_bot_army_composition_gap_state,
      rts_bot_army_composition_gap_openra_economy_tech_commit: $rts_bot_army_composition_gap[0].openra_bot_economy_tech_target_commit,
      rts_bot_army_composition_gap_openra_beacon_pressure_commit: $rts_bot_army_composition_gap[0].openra_bot_beacon_pressure_target_commit,
      rts_bot_army_composition_gap_openra_organic_terminal_commit: $rts_bot_army_composition_gap[0].openra_organic_bot_terminal_target_commit,
      rts_bot_army_composition_gap_army_composition_signals: $rts_bot_army_composition_gap[0].army_composition_signal_count,
      rts_bot_army_composition_gap_unit_mix_reads: $rts_bot_army_composition_gap[0].unit_mix_read_count,
      rts_bot_army_composition_gap_frontline_ratios: $rts_bot_army_composition_gap[0].frontline_ratio_count,
      rts_bot_army_composition_gap_counter_mix_swaps: $rts_bot_army_composition_gap[0].counter_mix_swap_count,
      rts_bot_army_composition_gap_reinforce_curves: $rts_bot_army_composition_gap[0].reinforce_curve_count,
      rts_bot_army_composition_gap_specialist_timings: $rts_bot_army_composition_gap[0].specialist_timing_count,
      rts_bot_army_composition_gap_composition_locks: $rts_bot_army_composition_gap[0].composition_lock_count,
      rts_bot_army_composition_gap_final_state: $rts_bot_army_composition_gap[0].final_army_composition_state,
      rts_bot_army_composition_gap_final_pressure_percent: $rts_bot_army_composition_gap[0].final_rts_ai_pressure_percent,
      rts_bot_army_composition_gap_final_defeat_risk_percent: $rts_bot_army_composition_gap[0].final_rts_defeat_risk_percent,
      rts_bot_army_composition_gap_final_capture_percent: $rts_bot_army_composition_gap[0].final_objective_capture_percent,
      rts_bot_army_composition_gap_match_result: $rts_bot_army_composition_gap[0].final_match_result_state,
      rts_bot_army_composition_gap_pixel_count: (
        $rts_bot_army_composition_gap[0].ai_wave_pixel_count
        + $rts_bot_army_composition_gap[0].ai_pressure_pixel_count
        + $rts_bot_army_composition_gap[0].ai_counter_pixel_count
        + $rts_bot_army_composition_gap[0].objective_pixel_count
        + $rts_bot_army_composition_gap[0].capture_bar_pixel_count
        + $rts_bot_army_composition_gap[0].match_result_pixel_count
      ),
      rts_bot_army_composition_gap_ai_wave_pixel_count: $rts_bot_army_composition_gap[0].ai_wave_pixel_count,
      rts_bot_army_composition_gap_ai_pressure_pixel_count: $rts_bot_army_composition_gap[0].ai_pressure_pixel_count,
      rts_bot_army_composition_gap_ai_counter_pixel_count: $rts_bot_army_composition_gap[0].ai_counter_pixel_count,
      rts_bot_army_composition_gap_objective_pixel_count: $rts_bot_army_composition_gap[0].objective_pixel_count,
      rts_bot_army_composition_gap_capture_bar_pixel_count: $rts_bot_army_composition_gap[0].capture_bar_pixel_count,
      rts_bot_army_composition_gap_match_result_pixel_count: $rts_bot_army_composition_gap[0].match_result_pixel_count,
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
      rts_fog_scouting_intel_core_frame_order_stream_sha256: $rts_fog[0].rts_fog_core_frame_order_stream_sha256,
      rts_fog_scouting_intel_core_headless_checkpoint_sha256: $rts_fog[0].rts_fog_core_headless_checkpoint_sha256,
      rts_fog_scouting_intel_core_frame_order_kinds: $rts_fog[0].rts_fog_core_frame_order_kind_labels,
      rts_fog_scouting_intel_core_applied_order_count: $rts_fog[0].rts_fog_core_headless_applied_order_count,
      rts_fog_scouting_intel_core_actor_count: $rts_fog[0].rts_fog_core_headless_actor_count,
      rts_fog_scouting_intel_core_final_frame: $rts_fog[0].rts_fog_core_headless_final_frame,
      rts_fog_scouting_intel_core_recon_order_count: $rts_fog[0].rts_fog_core_headless_recon_order_count,
      rts_fog_scouting_intel_core_scout_order_count: $rts_fog[0].rts_fog_core_headless_scout_order_count,
      rts_fog_scouting_intel_core_sweep_order_count: $rts_fog[0].rts_fog_core_headless_sweep_order_count,
      rts_fog_scouting_intel_core_scan_order_count: $rts_fog[0].rts_fog_core_headless_scan_order_count,
      rts_fog_scouting_intel_core_mark_order_count: $rts_fog[0].rts_fog_core_headless_mark_order_count,
      rts_fog_scouting_intel_core_recon_ids: $rts_fog[0].rts_fog_core_headless_recon_ids,
      rts_fog_scouting_intel_core_recon_tile_ids: $rts_fog[0].rts_fog_core_headless_recon_tile_ids,
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
      rts_open_world_after_action_runtime_screen_mode: $rts_open_world[0].runtime_screen_mode,
      rts_open_world_after_action_player_first_view_non_background: $rts_open_world[0].open_world_after_action_pixel_counts.player_first_open_world_view_non_background,
      rts_open_world_after_action_player_first_view_frame_pixel_count: $rts_open_world[0].open_world_after_action_pixel_counts.player_first_open_world_view_frame,
      rts_open_world_after_action_player_first_status_strip_pixel_count: $rts_open_world[0].open_world_after_action_pixel_counts.player_first_open_world_status_strip,
      rts_open_world_after_action_player_first_route_panel_pixel_count: $rts_open_world[0].open_world_after_action_pixel_counts.player_first_open_world_route_panel,
      rts_open_world_after_action_player_first_timeline_pixel_count: $rts_open_world[0].open_world_after_action_pixel_counts.player_first_open_world_timeline,
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
      rts_visual_fidelity_panel_pixel_count: $rts_visual_fidelity[0].fidelity_panel_pixel_count,
      rts_visual_fidelity_model_edge_pixel_count: $rts_visual_fidelity[0].model_edge_pixel_count,
      rts_visual_fidelity_command_grid_pixel_count: $rts_visual_fidelity[0].command_grid_pixel_count,
      rts_visual_fidelity_npc_action_pixel_count: $rts_visual_fidelity[0].npc_action_pixel_count,
      rts_production_art_replication_source_contract_count: $rts_production_art_replication[0].source_contract_count,
      rts_production_art_replication_required_asset_kind_count: $rts_production_art_replication[0].required_asset_kind_count,
      rts_production_art_replication_required_gameplay_layer_count: $rts_production_art_replication[0].required_gameplay_layer_count,
      rts_production_art_replication_required_replacement_slot_count: $rts_production_art_replication[0].required_replacement_slot_count,
      rts_production_art_replication_gate_count: $rts_production_art_replication[0].gate_count,
      rts_production_art_replication_passed_gate_count: $rts_production_art_replication[0].passed_gate_count,
      rts_production_art_replication_failed_gate_count: $rts_production_art_replication[0].failed_gate_count,
      rts_production_asset_atlas_source_contract_count: $rts_production_asset_atlas[0].source_contract_count,
      rts_production_asset_atlas_source_path_count: $rts_production_asset_atlas[0].source_path_count,
      rts_production_asset_atlas_family_name_count: $rts_production_asset_atlas[0].atlas_family_name_count,
      rts_production_asset_atlas_binding_replacement_slot_count: $rts_production_asset_atlas[0].binding_replacement_slot_count,
      rts_production_asset_atlas_binding_runtime_target_count: $rts_production_asset_atlas[0].binding_runtime_target_count,
      rts_production_asset_atlas_runtime_material_slot_count: $rts_production_asset_atlas[0].runtime_material_slot_count,
      rts_production_asset_atlas_runtime_scene_layer_count: $rts_production_asset_atlas[0].runtime_scene_layer_count,
      rts_production_asset_atlas_gate_count: $rts_production_asset_atlas[0].gate_count,
      rts_production_asset_atlas_passed_gate_count: $rts_production_asset_atlas[0].passed_gate_count,
      rts_production_asset_atlas_failed_gate_count: $rts_production_asset_atlas[0].failed_gate_count,
      rts_production_asset_atlas_frame_count: $rts_production_asset_atlas[0].atlas_frame_count,
      rts_production_asset_atlas_sprite_binding_count: $rts_production_asset_atlas[0].sprite_binding_count,
      rts_production_asset_atlas_material_asset_count: $rts_production_asset_atlas[0].material_asset_count,
      rts_production_asset_atlas_family_count: $rts_production_asset_atlas[0].atlas_family_count,
      rts_production_asset_atlas_board_pixel_count: $rts_production_asset_atlas[0].atlas_board_pixel_count,
      rts_production_asset_atlas_runtime_binding_lane_pixel_count: $rts_production_asset_atlas[0].runtime_binding_lane_pixel_count,
      rts_production_asset_atlas_uv_rect_pixel_count: $rts_production_asset_atlas[0].uv_rect_pixel_count,
      rts_production_ui_skin_surface_count: $rts_production_ui_skin[0].ui_skin_surface_count,
      rts_production_ui_skin_source_contract_count: $rts_production_ui_skin[0].source_contract_count,
      rts_production_ui_skin_source_path_count: $rts_production_ui_skin[0].source_path_count,
      rts_production_ui_skin_runtime_screen_layout_count: $rts_production_ui_skin[0].runtime_screen_layout_count,
      rts_production_ui_skin_pixel_count_field_count: $rts_production_ui_skin[0].production_ui_skin_pixel_count_field_count,
      rts_production_ui_skin_surface_name_count: $rts_production_ui_skin[0].ui_skin_surface_name_count,
      rts_production_ui_skin_replacement_slot_count: $rts_production_ui_skin[0].ui_skin_replacement_slot_count,
      rts_production_ui_skin_source_surface_count: $rts_production_ui_skin[0].ui_skin_source_surface_count,
      rts_production_ui_skin_gate_count: $rts_production_ui_skin[0].gate_count,
      rts_production_ui_skin_passed_gate_count: $rts_production_ui_skin[0].passed_gate_count,
      rts_production_ui_skin_failed_gate_count: $rts_production_ui_skin[0].failed_gate_count,
      rts_production_ui_skin_board_pixel_count: $rts_production_ui_skin[0].ui_skin_board_pixel_count,
      rts_production_ui_skin_hud_chrome_pixel_count: $rts_production_ui_skin[0].hud_chrome_pixel_count,
      rts_production_ui_skin_command_grid_pixel_count: $rts_production_ui_skin[0].command_grid_skin_pixel_count,
      rts_production_ui_skin_minimap_bezel_pixel_count: $rts_production_ui_skin[0].minimap_bezel_pixel_count,
      rts_production_ui_skin_unit_card_pixel_count: $rts_production_ui_skin[0].unit_card_skin_pixel_count,
      rts_production_ui_skin_tooltip_pixel_count: $rts_production_ui_skin[0].tooltip_skin_pixel_count,
      rts_production_ui_skin_feedback_marker_pixel_count: $rts_production_ui_skin[0].feedback_marker_pixel_count,
      rts_production_ui_skin_hotkey_strip_pixel_count: $rts_production_ui_skin[0].hotkey_strip_pixel_count,
      rts_production_ui_skin_status_bar_pixel_count: $rts_production_ui_skin[0].status_bar_skin_pixel_count,
      rts_production_ui_skin_runtime_screen_mode: $rts_production_ui_skin[0].runtime_screen_mode,
      rts_production_ui_skin_player_first_hud_view_non_background: $rts_production_ui_skin[0].production_ui_skin_pixel_counts.player_first_production_hud_view_non_background,
      rts_production_ui_skin_player_first_hud_view_frame_pixel_count: $rts_production_ui_skin[0].production_ui_skin_pixel_counts.player_first_production_hud_view_frame,
      rts_production_ui_skin_player_first_hud_bottom_chrome_pixel_count: $rts_production_ui_skin[0].production_ui_skin_pixel_counts.player_first_production_hud_bottom_chrome,
      rts_production_ui_skin_player_first_hud_command_grid_pixel_count: $rts_production_ui_skin[0].production_ui_skin_pixel_counts.player_first_production_hud_command_grid,
      rts_production_ui_skin_player_first_hud_minimap_bezel_pixel_count: $rts_production_ui_skin[0].production_ui_skin_pixel_counts.player_first_production_hud_minimap_bezel,
      rts_production_ui_skin_player_first_hud_unit_card_pixel_count: $rts_production_ui_skin[0].production_ui_skin_pixel_counts.player_first_production_hud_unit_card,
      rts_production_ui_skin_player_first_hud_feedback_lane_pixel_count: $rts_production_ui_skin[0].production_ui_skin_pixel_counts.player_first_production_hud_feedback_lane,
      rts_production_ui_skin_player_first_hud_hotkey_status_pixel_count: $rts_production_ui_skin[0].production_ui_skin_pixel_counts.player_first_production_hud_hotkey_status,
      rts_production_interaction_polish_surface_count: $rts_production_interaction_polish[0].interaction_surface_count,
      rts_production_interaction_polish_source_contract_count: $rts_production_interaction_polish[0].source_contract_count,
      rts_production_interaction_polish_source_path_count: $rts_production_interaction_polish[0].source_path_count,
      rts_production_interaction_polish_runtime_screen_layout_count: $rts_production_interaction_polish[0].runtime_screen_layout_count,
      rts_production_interaction_polish_pixel_count_field_count: $rts_production_interaction_polish[0].interaction_pixel_count_field_count,
      rts_production_interaction_polish_surface_name_count: $rts_production_interaction_polish[0].interaction_surface_name_count,
      rts_production_interaction_polish_replacement_slot_count: $rts_production_interaction_polish[0].interaction_replacement_slot_count,
      rts_production_interaction_polish_source_surface_count: $rts_production_interaction_polish[0].interaction_source_surface_count,
      rts_production_interaction_polish_gate_count: $rts_production_interaction_polish[0].gate_count,
      rts_production_interaction_polish_passed_gate_count: $rts_production_interaction_polish[0].passed_gate_count,
      rts_production_interaction_polish_failed_gate_count: $rts_production_interaction_polish[0].failed_gate_count,
      rts_production_interaction_polish_board_pixel_count: $rts_production_interaction_polish[0].interaction_board_pixel_count,
      rts_production_interaction_polish_drag_select_pixel_count: $rts_production_interaction_polish[0].drag_select_skin_pixel_count,
      rts_production_interaction_polish_right_click_pixel_count: $rts_production_interaction_polish[0].right_click_move_skin_pixel_count,
      rts_production_interaction_polish_attack_lock_pixel_count: $rts_production_interaction_polish[0].attack_lock_skin_pixel_count,
      rts_production_interaction_polish_build_ghost_pixel_count: $rts_production_interaction_polish[0].build_ghost_skin_pixel_count,
      rts_production_interaction_polish_queue_path_pixel_count: $rts_production_interaction_polish[0].queue_path_skin_pixel_count,
      rts_production_interaction_polish_scroll_minimap_pixel_count: $rts_production_interaction_polish[0].scroll_minimap_skin_pixel_count,
      rts_production_interaction_polish_hud_binding_pixel_count: $rts_production_interaction_polish[0].hud_binding_pixel_count,
      rts_production_interaction_polish_player_first_view_non_background: $rts_production_interaction_polish[0].interaction_pixel_counts.player_first_command_interaction_view_non_background,
      rts_production_interaction_polish_player_first_view_frame_pixel_count: $rts_production_interaction_polish[0].interaction_pixel_counts.player_first_command_interaction_view_frame,
      rts_production_interaction_polish_player_first_status_strip_pixel_count: $rts_production_interaction_polish[0].interaction_pixel_counts.player_first_command_interaction_status_strip,
      rts_production_interaction_polish_player_first_right_rail_pixel_count: $rts_production_interaction_polish[0].interaction_pixel_counts.player_first_command_interaction_right_rail,
      rts_production_interaction_polish_player_first_command_lane_pixel_count: $rts_production_interaction_polish[0].interaction_pixel_counts.player_first_command_interaction_command_lane,
      rts_full_screen_ui_replication_surface_count: $rts_full_screen_ui_replication[0].replication_surface_count,
      rts_full_screen_ui_replication_board_pixel_count: $rts_full_screen_ui_replication[0].screen_matrix_pixel_counts.board,
      rts_full_screen_ui_replication_title_campaign_pixel_count: $rts_full_screen_ui_replication[0].screen_matrix_pixel_counts.title_campaign,
      rts_full_screen_ui_replication_tactical_viewport_pixel_count: $rts_full_screen_ui_replication[0].screen_matrix_pixel_counts.tactical_viewport,
      rts_full_screen_ui_replication_player_first_tactical_view_non_background: $rts_full_screen_ui_replication[0].full_screen_ui_pixel_counts.player_first_full_screen_tactical_view_non_background,
      rts_full_screen_ui_replication_player_first_tactical_view_frame_pixel_count: $rts_full_screen_ui_replication[0].full_screen_ui_pixel_counts.player_first_full_screen_tactical_view_frame,
      rts_full_screen_ui_replication_player_first_status_strip_pixel_count: $rts_full_screen_ui_replication[0].full_screen_ui_pixel_counts.player_first_full_screen_status_strip,
      rts_full_screen_ui_replication_map_minimap_pixel_count: $rts_full_screen_ui_replication[0].screen_matrix_pixel_counts.map_minimap,
      rts_full_screen_ui_replication_build_tech_pixel_count: $rts_full_screen_ui_replication[0].screen_matrix_pixel_counts.build_tech,
      rts_shell_meta_ui_replication_surface_count: $rts_shell_meta_ui_replication[0].shell_meta_surface_count,
      rts_shell_meta_ui_replication_board_pixel_count: $rts_shell_meta_ui_replication[0].shell_meta_pixel_counts.board,
      rts_shell_meta_ui_replication_account_pixel_count: $rts_shell_meta_ui_replication[0].shell_meta_pixel_counts.account_title,
      rts_shell_meta_ui_replication_session_slot_pixel_count: $rts_shell_meta_ui_replication[0].shell_meta_pixel_counts.session_slot_menu,
      rts_shell_meta_ui_replication_pause_pixel_count: $rts_shell_meta_ui_replication[0].shell_meta_pixel_counts.pause_resume,
      rts_shell_meta_ui_replication_input_pixel_count: $rts_shell_meta_ui_replication[0].shell_meta_pixel_counts.input_hud,
      rts_shell_meta_ui_replication_player_first_surface_non_background: $rts_shell_meta_ui_replication[0].shell_meta_player_first_pixel_counts.player_first_shell_meta_surface_non_background,
      rts_shell_meta_ui_replication_player_first_frame_pixel_count: $rts_shell_meta_ui_replication[0].shell_meta_player_first_pixel_counts.player_first_shell_meta_frame,
      rts_shell_meta_ui_replication_player_first_account_bar_pixel_count: $rts_shell_meta_ui_replication[0].shell_meta_player_first_pixel_counts.player_first_shell_meta_account_bar,
      rts_shell_meta_ui_replication_player_first_session_panel_pixel_count: $rts_shell_meta_ui_replication[0].shell_meta_player_first_pixel_counts.player_first_shell_meta_session_panel,
      rts_shell_meta_ui_replication_player_first_right_rail_pixel_count: $rts_shell_meta_ui_replication[0].shell_meta_player_first_pixel_counts.player_first_shell_meta_right_rail,
      rts_shell_meta_ui_replication_player_first_handoff_strip_pixel_count: $rts_shell_meta_ui_replication[0].shell_meta_player_first_pixel_counts.player_first_shell_meta_handoff_strip,
      rts_match_setup_ui_replication_surface_count: $rts_match_setup_ui_replication[0].setup_surface_count,
      rts_match_setup_ui_replication_board_pixel_count: $rts_match_setup_ui_replication[0].setup_pixel_counts.board,
      rts_match_setup_ui_replication_map_select_pixel_count: $rts_match_setup_ui_replication[0].setup_pixel_counts.map_select,
      rts_match_setup_ui_replication_faction_select_pixel_count: $rts_match_setup_ui_replication[0].setup_pixel_counts.faction_select,
      rts_match_setup_ui_replication_start_ready_pixel_count: $rts_match_setup_ui_replication[0].setup_pixel_counts.start_ready,
      rts_match_setup_ui_replication_player_first_map_non_background: $rts_match_setup_ui_replication[0].match_setup_player_first_pixel_counts.player_first_match_setup_map_non_background,
      rts_match_setup_ui_replication_player_first_map_frame_pixel_count: $rts_match_setup_ui_replication[0].match_setup_player_first_pixel_counts.player_first_match_setup_map_frame,
      rts_match_setup_ui_replication_player_first_status_strip_pixel_count: $rts_match_setup_ui_replication[0].match_setup_player_first_pixel_counts.player_first_match_setup_status_strip,
      rts_match_setup_ui_replication_player_first_rules_rail_pixel_count: $rts_match_setup_ui_replication[0].match_setup_player_first_pixel_counts.player_first_match_setup_rules_rail,
      rts_match_setup_ui_replication_player_first_ready_strip_pixel_count: $rts_match_setup_ui_replication[0].match_setup_player_first_pixel_counts.player_first_match_setup_ready_strip,
      rts_match_setup_ui_replication_map_id: $rts_match_setup_ui_replication[0].source_headline.map_id,
      rts_match_setup_ui_replication_faction_id: $rts_match_setup_ui_replication[0].source_headline.faction_id,
      rts_first_contact_basin_spec_map_id: $rts_first_contact_basin_spec[0].map_id,
      rts_first_contact_basin_spec_actor_count: $rts_first_contact_basin_spec[0].actor_count,
      rts_first_contact_basin_spec_spawn_count: $rts_first_contact_basin_spec[0].spawn_count,
      rts_first_contact_basin_spec_contract_field_count: $rts_first_contact_basin_spec[0].contract_field_count,
      rts_first_contact_basin_spec_guard_object_count: $rts_first_contact_basin_spec[0].guard_object_count,
      rts_first_contact_basin_spec_guard_gate_count: $rts_first_contact_basin_spec[0].guard_gate_count,
      rts_first_contact_basin_spec_top_level_gate_count: $rts_first_contact_basin_spec[0].top_level_gate_count,
      rts_first_contact_basin_spec_map_model_actor_count: $rts_first_contact_basin_spec[0].rts_data_map_model_actor_count,
      rts_first_contact_basin_spec_map_model_player_count: $rts_first_contact_basin_spec[0].rts_data_map_model_player_count,
      rts_first_contact_basin_spec_map_model_rule_count: $rts_first_contact_basin_spec[0].rts_data_map_model_rule_count,
      rts_first_contact_basin_spec_runtime_command_queue_count: $rts_first_contact_basin_spec[0].runtime_player_screen_command_queue_count,
      rts_first_contact_basin_spec_runtime_production_queue_count: $rts_first_contact_basin_spec[0].runtime_player_screen_production_queue_count,
      rts_first_contact_basin_spec_runtime_build_queue_count: $rts_first_contact_basin_spec[0].runtime_player_screen_build_queue_count,
      rts_first_contact_basin_spec_runtime_visible_tile_count: $rts_first_contact_basin_spec[0].runtime_player_screen_visible_tile_count,
      rts_first_contact_basin_spec_runtime_fogged_tile_count: $rts_first_contact_basin_spec[0].runtime_player_screen_fogged_tile_count,
      rts_first_contact_basin_spec_runtime_ability_command_count: $rts_first_contact_basin_spec[0].runtime_player_screen_ability_command_count,
      rts_first_contact_basin_spec_offline_command_queue_count: $rts_first_contact_basin_spec[0].offline_consumption_command_queue_count,
      rts_first_contact_basin_spec_offline_production_queue_count: $rts_first_contact_basin_spec[0].offline_consumption_production_queue_count,
      rts_first_contact_basin_spec_offline_build_queue_count: $rts_first_contact_basin_spec[0].offline_consumption_build_queue_count,
      rts_first_contact_basin_spec_offline_ability_command_count: $rts_first_contact_basin_spec[0].offline_consumption_ability_command_count,
      rts_first_contact_basin_spec_offline_ready_label_count: $rts_first_contact_basin_spec[0].offline_lobby_ready_label_count,
      rts_first_contact_runtime_review_contract: $rts_first_contact_basin_spec[0].rts_evidence_bevy_runtime_adapter.contract_version,
      rts_first_contact_runtime_review_contract_count: ($rts_first_contact_basin_spec[0].rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_contracts | length),
      rts_first_contact_runtime_review_contracts: $rts_first_contact_basin_spec[0].rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_contracts,
      rts_first_contact_runtime_review_before_command_queue: $rts_first_contact_basin_spec[0].rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_before_command_queue_sample,
      rts_first_contact_runtime_review_after_command_queue: $rts_first_contact_basin_spec[0].rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_after_command_queue_sample,
      rts_first_contact_runtime_review_ready_state_labels: $rts_first_contact_basin_spec[0].rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_ready_state_labels_sample,
      rts_first_contact_runtime_review_command_stamp_tile: $rts_first_contact_basin_spec[0].rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_command_stamp_tile_sample,
      rts_first_contact_runtime_review_source_of_truth: $rts_first_contact_basin_spec[0].rts_evidence_bevy_runtime_adapter.source_of_truth,
      rts_campaign_outcome_ui_readiness_runtime_screen_mode: $rts_campaign_outcome_ui_readiness[0].runtime_screen_mode,
      rts_campaign_outcome_ui_readiness_evidence_board_only: $rts_campaign_outcome_ui_readiness[0].evidence_board_only,
      rts_campaign_outcome_ui_readiness_first_minute_player_first_non_background: $rts_campaign_outcome_ui_readiness[0].first_minute_summary.first_minute_pixel_counts.player_first_campaign_view_non_background,
      rts_campaign_outcome_ui_readiness_first_minute_player_first_route_rail: $rts_campaign_outcome_ui_readiness[0].first_minute_summary.first_minute_pixel_counts.player_first_campaign_route_rail,
      rts_campaign_outcome_ui_readiness_victory_non_background_pixels: $rts_campaign_outcome_ui_readiness[0].victory_summary.non_background_pixels,
      rts_campaign_outcome_ui_readiness_victory_extraction_pixel_count: $rts_campaign_outcome_ui_readiness[0].victory_summary.extraction_pixel_count,
      rts_campaign_outcome_ui_readiness_base_assault_non_background_pixels: $rts_campaign_outcome_ui_readiness[0].base_assault_summary.non_background_pixels,
      rts_campaign_outcome_ui_readiness_base_assault_breach_pixel_count: $rts_campaign_outcome_ui_readiness[0].base_assault_summary.breach_pixel_count,
      rts_campaign_outcome_ui_readiness_aftermath_player_first_view_non_background: $rts_campaign_outcome_ui_readiness[0].aftermath_summary.battle_aftermath_pixel_counts.player_first_battle_view_non_background,
      rts_campaign_outcome_ui_readiness_aftermath_player_first_outcome_panel: $rts_campaign_outcome_ui_readiness[0].aftermath_summary.battle_aftermath_pixel_counts.player_first_battle_outcome_panel,
      rts_campaign_outcome_ui_readiness_open_world_player_first_view_non_background: $rts_campaign_outcome_ui_readiness[0].open_world_summary.open_world_after_action_pixel_counts.player_first_open_world_view_non_background,
      rts_campaign_outcome_ui_readiness_open_world_player_first_route_panel: $rts_campaign_outcome_ui_readiness[0].open_world_summary.open_world_after_action_pixel_counts.player_first_open_world_route_panel,
      rts_campaign_ui_continuity_capture_frame_count: $rts_campaign_ui_continuity[0].capture_frame_count,
      rts_campaign_ui_continuity_non_background_pixels: $rts_campaign_ui_continuity[0].non_background_pixels,
      rts_campaign_ui_continuity_final_room_id: $rts_campaign_ui_continuity[0].final_current_room_id,
      rts_campaign_ui_continuity_restored_room_id: $rts_campaign_ui_continuity[0].restored_current_room_id,
      rts_campaign_ui_continuity_primary_action_label: $rts_campaign_ui_continuity[0].final_contextual_primary_action_label,
      rts_campaign_ui_continuity_runtime_screen_layout_count: $rts_campaign_ui_continuity[0].runtime_screen_layout_count,
      rts_campaign_ui_continuity_review_field_count: $rts_campaign_ui_continuity[0].rts_evidence_campaign_ui_continuity_review_field_count,
      rts_campaign_ui_continuity_final_action_label_count: $rts_campaign_ui_continuity[0].final_contextual_action_label_count,
      rts_campaign_ui_continuity_final_active_task_count: $rts_campaign_ui_continuity[0].final_active_task_count,
      rts_campaign_ui_continuity_restored_action_label_count: $rts_campaign_ui_continuity[0].restored_contextual_action_label_count,
      rts_campaign_ui_continuity_restored_active_task_count: $rts_campaign_ui_continuity[0].restored_active_task_count,
      rts_campaign_ui_continuity_milestone_count: $rts_campaign_ui_continuity[0].milestone_count,
      rts_campaign_ui_continuity_pixel_count_field_count: $rts_campaign_ui_continuity[0].campaign_continuity_pixel_count_field_count,
      rts_campaign_ui_continuity_gate_count: $rts_campaign_ui_continuity[0].gate_count,
      rts_campaign_ui_continuity_passed_gate_count: $rts_campaign_ui_continuity[0].passed_gate_count,
      rts_campaign_ui_continuity_failed_gate_count: $rts_campaign_ui_continuity[0].failed_gate_count,
      rts_in_match_hud_state_replication_surface_count: $rts_in_match_hud_state_replication[0].hud_surface_count,
      rts_in_match_hud_state_replication_runtime_layout_count: $rts_in_match_hud_state_replication[0].runtime_screen_layout_count,
      rts_in_match_hud_state_replication_hud_pixel_field_count: $rts_in_match_hud_state_replication[0].hud_pixel_count_field_count,
      rts_in_match_hud_state_replication_player_first_pixel_field_count: $rts_in_match_hud_state_replication[0].in_match_hud_player_first_pixel_count_field_count,
      rts_in_match_hud_state_replication_surface_name_count: $rts_in_match_hud_state_replication[0].hud_surface_name_count,
      rts_in_match_hud_state_replication_gate_count: $rts_in_match_hud_state_replication[0].gate_count,
      rts_in_match_hud_state_replication_passed_gate_count: $rts_in_match_hud_state_replication[0].passed_gate_count,
      rts_in_match_hud_state_replication_failed_gate_count: $rts_in_match_hud_state_replication[0].failed_gate_count,
      rts_in_match_hud_state_replication_non_background_pixels: $rts_in_match_hud_state_replication[0].hud_pixel_counts.non_background,
      rts_in_match_hud_state_replication_command_grid_pixel_count: $rts_in_match_hud_state_replication[0].hud_pixel_counts.command_grid,
      rts_in_match_hud_state_replication_minimap_pixel_count: $rts_in_match_hud_state_replication[0].hud_pixel_counts.minimap,
      rts_in_match_hud_state_replication_player_first_view_non_background: $rts_in_match_hud_state_replication[0].in_match_hud_player_first_pixel_counts.player_first_in_match_hud_view_non_background,
      rts_in_match_hud_state_replication_player_first_view_frame_pixel_count: $rts_in_match_hud_state_replication[0].in_match_hud_player_first_pixel_counts.player_first_in_match_hud_view_frame,
      rts_in_match_hud_state_replication_player_first_top_status_strip_pixel_count: $rts_in_match_hud_state_replication[0].in_match_hud_player_first_pixel_counts.player_first_in_match_hud_top_status_strip,
      rts_in_match_hud_state_replication_player_first_surface_card_pixel_count: $rts_in_match_hud_state_replication[0].in_match_hud_player_first_pixel_counts.player_first_in_match_hud_surface_cards,
      rts_in_match_hud_state_replication_player_first_right_rail_non_background: $rts_in_match_hud_state_replication[0].in_match_hud_player_first_pixel_counts.player_first_in_match_hud_right_rail_non_background,
      rts_in_match_hud_state_replication_player_first_bottom_command_lane_pixel_count: $rts_in_match_hud_state_replication[0].in_match_hud_player_first_pixel_counts.player_first_in_match_hud_bottom_command_lane,
      rts_in_match_hud_state_replication_player_first_control_color_pixel_count: $rts_in_match_hud_state_replication[0].in_match_hud_player_first_pixel_counts.player_first_in_match_hud_control_colors,
      rts_in_match_hud_state_replication_army_supply_used: $rts_in_match_hud_state_replication[0].army_supply_used,
      rts_in_match_hud_state_replication_army_supply_cap: $rts_in_match_hud_state_replication[0].army_supply_cap,
      rts_session_state_continuity_surface_count: $rts_session_state_continuity[0].state_continuity_surface_count,
      rts_session_state_continuity_non_background_pixels: $rts_session_state_continuity[0].state_continuity_pixel_counts.non_background,
      rts_session_state_continuity_player_first_resume_view_non_background: $rts_session_state_continuity[0].state_continuity_pixel_counts.player_first_resume_view_non_background,
      rts_session_state_continuity_player_first_resume_view_frame: $rts_session_state_continuity[0].state_continuity_pixel_counts.player_first_resume_view_frame,
      rts_session_state_continuity_player_first_resume_status_strip: $rts_session_state_continuity[0].state_continuity_pixel_counts.player_first_resume_status_strip,
      rts_session_state_continuity_player_first_resume_stage_rail: $rts_session_state_continuity[0].state_continuity_pixel_counts.player_first_resume_stage_rail,
      rts_session_state_continuity_slot_a_bytes: $rts_session_state_continuity[0].source_headline.load_resume_slot_a_bytes,
      rts_session_state_continuity_final_objective_status: $rts_session_state_continuity[0].source_headline.load_resume_final_objective_status,
      rts_session_state_continuity_open_world_state: $rts_session_state_continuity[0].source_headline.campaign_outcome_open_world_state,
      rts_session_state_continuity_restored_room_id: $rts_session_state_continuity[0].source_headline.campaign_continuity_restored_room_id,
      rts_continuous_player_flow_step_count: $rts_continuous_player_flow[0].continuous_player_flow_step_count,
      rts_continuous_player_flow_non_background_pixels: $rts_continuous_player_flow[0].flow_pixel_counts.non_background,
      rts_continuous_player_flow_title_account_pixel_count: $rts_continuous_player_flow[0].flow_pixel_counts.title_account,
      rts_continuous_player_flow_match_setup_pixel_count: $rts_continuous_player_flow[0].flow_pixel_counts.match_setup,
      rts_continuous_player_flow_in_match_hud_pixel_count: $rts_continuous_player_flow[0].flow_pixel_counts.in_match_hud,
      rts_continuous_player_flow_command_feedback_pixel_count: $rts_continuous_player_flow[0].flow_pixel_counts.command_feedback,
      rts_continuous_player_flow_save_load_resume_pixel_count: $rts_continuous_player_flow[0].flow_pixel_counts.save_load_resume,
      rts_continuous_player_flow_outcome_open_world_pixel_count: $rts_continuous_player_flow[0].flow_pixel_counts.outcome_open_world,
      rts_continuous_player_flow_player_first_flow_view_non_background: $rts_continuous_player_flow[0].flow_pixel_counts.player_first_flow_view_non_background,
      rts_continuous_player_flow_player_first_flow_view_frame_pixel_count: $rts_continuous_player_flow[0].flow_pixel_counts.player_first_flow_view_frame,
      rts_continuous_player_flow_player_first_flow_status_strip_pixel_count: $rts_continuous_player_flow[0].flow_pixel_counts.player_first_flow_status_strip,
      rts_continuous_player_flow_player_first_flow_stage_rail_pixel_count: $rts_continuous_player_flow[0].flow_pixel_counts.player_first_flow_stage_rail,
      rts_continuous_player_flow_final_objective_status: $rts_continuous_player_flow[0].source_headline.session_final_objective_status,
      rts_continuous_player_flow_open_world_state: $rts_continuous_player_flow[0].source_headline.session_open_world_state,
      rts_continuous_player_flow_restored_room_id: $rts_continuous_player_flow[0].source_headline.campaign_continuity_restored_room_id,
      rts_continuous_player_flow_review_contract: $rts_continuous_player_flow[0].rts_evidence_continuous_player_flow_review_contract,
      rts_continuous_player_flow_review_source_of_truth: $rts_continuous_player_flow[0].rts_evidence_continuous_player_flow_review.source_of_truth,
      rts_live_session_playthrough_runtime_screen_mode: $rts_live_session_playthrough[0].runtime_screen_mode,
      rts_live_session_playthrough_stage_count: $rts_live_session_playthrough[0].stage_count,
      rts_live_session_playthrough_top_level_action_count: $rts_live_session_playthrough[0].top_level_action_count,
      rts_live_session_playthrough_accepted_input_count: $rts_live_session_playthrough[0].accepted_input_count,
      rts_live_session_playthrough_campaign_handoff_input_count: $rts_live_session_playthrough[0].campaign_handoff_input_count,
      rts_live_session_playthrough_live_command_input_count: $rts_live_session_playthrough[0].live_command_input_count,
      rts_live_session_playthrough_slot_a_bytes: $rts_live_session_playthrough[0].slot_a_bytes,
      rts_live_session_playthrough_non_background_pixels: $rts_live_session_playthrough[0].pixel_counts.non_background,
      rts_live_session_playthrough_player_first_live_view_non_background: $rts_live_session_playthrough[0].pixel_counts.player_first_live_view_non_background,
      rts_live_session_playthrough_player_first_live_view_frame_pixel_count: $rts_live_session_playthrough[0].pixel_counts.player_first_live_view_frame,
      rts_live_session_playthrough_player_first_live_status_strip_pixel_count: $rts_live_session_playthrough[0].pixel_counts.player_first_live_status_strip,
      rts_live_session_playthrough_player_first_live_stage_rail_pixel_count: $rts_live_session_playthrough[0].pixel_counts.player_first_live_stage_rail,
      rts_live_session_playthrough_final_objective_status: $rts_live_session_playthrough[0].final_state.objective_status,
      rts_live_session_playthrough_open_world_state: $rts_live_session_playthrough[0].final_state.open_world_handoff_state,
      rts_live_session_playthrough_resume_room_id: $rts_live_session_playthrough[0].final_state.open_world_resume_room_id,
      rts_live_session_playthrough_review_contract: $rts_live_session_playthrough[0].rts_evidence_live_session_playthrough_review_contract,
      rts_live_session_playthrough_review_source_of_truth: $rts_live_session_playthrough[0].rts_evidence_live_session_playthrough_review.source_of_truth,
      rts_full_game_visual_ui_replication_runtime_screen_mode: $rts_full_game_visual_ui_replication[0].runtime_screen_mode,
      rts_full_game_visual_ui_replication_evidence_board_only: $rts_full_game_visual_ui_replication[0].evidence_board_only,
      rts_full_game_visual_ui_replication_surface_count: $rts_full_game_visual_ui_replication[0].coverage_surface_count,
      rts_full_game_visual_ui_replication_source_contract_count: $rts_full_game_visual_ui_replication[0].source_contract_count,
      rts_full_game_visual_ui_replication_source_path_count: $rts_full_game_visual_ui_replication[0].source_path_count,
      rts_full_game_visual_ui_replication_source_review_contract_count: $rts_full_game_visual_ui_replication[0].source_review_contract_count,
      rts_full_game_visual_ui_replication_source_review_gate_count: $rts_full_game_visual_ui_replication[0].source_review_gate_count,
      rts_full_game_visual_ui_replication_source_review_source_count: $rts_full_game_visual_ui_replication[0].source_review_source_count,
      rts_full_game_visual_ui_replication_source_headline_field_count: $rts_full_game_visual_ui_replication[0].source_headline_field_count,
      rts_full_game_visual_ui_replication_single_screen_runtime_layout_count: $rts_full_game_visual_ui_replication[0].single_screen_runtime_layout_count,
      rts_full_game_visual_ui_replication_pixel_count_field_count: $rts_full_game_visual_ui_replication[0].pixel_count_field_count,
      rts_full_game_visual_ui_replication_coverage_surface_name_count: $rts_full_game_visual_ui_replication[0].coverage_surface_name_count,
      rts_full_game_visual_ui_replication_command_grid_role_id_count: $rts_full_game_visual_ui_replication[0].command_grid_role_id_count,
      rts_full_game_visual_ui_replication_command_grid_icon_signature_count: $rts_full_game_visual_ui_replication[0].command_grid_icon_signature_count,
      rts_full_game_visual_ui_replication_command_grid_state_sample_count: $rts_full_game_visual_ui_replication[0].command_grid_state_sample_count,
      rts_full_game_visual_ui_replication_gate_count: $rts_full_game_visual_ui_replication[0].gate_count,
      rts_full_game_visual_ui_replication_passed_gate_count: $rts_full_game_visual_ui_replication[0].passed_gate_count,
      rts_full_game_visual_ui_replication_failed_gate_count: $rts_full_game_visual_ui_replication[0].failed_gate_count,
      rts_full_game_visual_ui_replication_non_background_pixels: $rts_full_game_visual_ui_replication[0].pixel_counts.non_background,
      rts_full_game_visual_ui_replication_hud_chrome_pixel_count: $rts_full_game_visual_ui_replication[0].pixel_counts.hud_chrome,
      rts_full_game_visual_ui_replication_command_pixel_count: $rts_full_game_visual_ui_replication[0].pixel_counts.command,
      rts_full_game_visual_ui_replication_session_pixel_count: $rts_full_game_visual_ui_replication[0].pixel_counts.session,
      rts_full_game_visual_ui_replication_outcome_pixel_count: $rts_full_game_visual_ui_replication[0].pixel_counts.outcome,
      rts_full_game_visual_ui_replication_player_first_tactical_preview_non_background: $rts_full_game_visual_ui_replication[0].pixel_counts.player_first_tactical_preview_non_background,
      rts_full_game_visual_ui_replication_player_first_tactical_viewport_frame_pixel_count: $rts_full_game_visual_ui_replication[0].pixel_counts.player_first_tactical_viewport_frame,
      rts_full_game_visual_ui_replication_player_first_tactical_status_strip_pixel_count: $rts_full_game_visual_ui_replication[0].pixel_counts.player_first_tactical_status_strip,
      rts_full_game_visual_ui_replication_command_grid_unique_icon_signature_count: $rts_full_game_visual_ui_replication[0].full_game_command_grid_unique_icon_signature_count,
      rts_full_game_visual_ui_replication_command_grid_active_role: $rts_full_game_visual_ui_replication[0].full_game_command_grid_active_role,
      rts_full_game_visual_ui_replication_command_grid_active_slot_count: $rts_full_game_visual_ui_replication[0].full_game_command_grid_active_slot_count,
      rts_full_game_visual_ui_replication_command_grid_sent_slot_count: $rts_full_game_visual_ui_replication[0].full_game_command_grid_sent_slot_count,
      rts_full_game_visual_ui_replication_live_session_stage_count: $rts_full_game_visual_ui_replication[0].source_headline.live_session_stage_count,
      rts_full_game_visual_ui_replication_live_session_accepted_input_count: $rts_full_game_visual_ui_replication[0].source_headline.live_session_accepted_input_count,
      rts_full_game_visual_ui_replication_final_objective_status: $rts_full_game_visual_ui_replication[0].source_headline.live_session_final_objective_status,
      rts_full_game_visual_ui_replication_open_world_state: $rts_full_game_visual_ui_replication[0].source_headline.live_session_open_world_state,
      rts_full_game_visual_ui_replication_review_contract: $rts_full_game_visual_ui_replication[0].rts_evidence_full_game_visual_ui_replication_review_contract,
      rts_full_game_visual_ui_replication_review_source_of_truth: $rts_full_game_visual_ui_replication[0].rts_evidence_full_game_visual_ui_replication_review.source_of_truth,
      rts_openra_screen_for_screen_ui_replication_screen_count: $rts_openra_screen_for_screen_ui_replication[0].openra_reference_screen_count,
      rts_openra_screen_for_screen_ui_replication_surface_count: $rts_openra_screen_for_screen_ui_replication[0].replicated_interaction_surface_count,
      rts_openra_screen_for_screen_ui_replication_widget_root_count: $rts_openra_screen_for_screen_ui_replication[0].openra_widget_root_count,
      rts_openra_screen_for_screen_ui_replication_source_contract_count: $rts_openra_screen_for_screen_ui_replication[0].source_contract_count,
      rts_openra_screen_for_screen_ui_replication_source_headline_field_count: $rts_openra_screen_for_screen_ui_replication[0].source_headline_field_count,
      rts_openra_screen_for_screen_ui_replication_screen_layout_count: $rts_openra_screen_for_screen_ui_replication[0].screen_layout_count,
      rts_openra_screen_for_screen_ui_replication_pixel_count_field_count: $rts_openra_screen_for_screen_ui_replication[0].pixel_count_field_count,
      rts_openra_screen_for_screen_ui_replication_openra_style_ingame_pixel_count_field_count: $rts_openra_screen_for_screen_ui_replication[0].openra_style_ingame_pixel_count_field_count,
      rts_openra_screen_for_screen_ui_replication_widget_root_name_count: $rts_openra_screen_for_screen_ui_replication[0].openra_widget_root_name_count,
      rts_openra_screen_for_screen_ui_replication_reference_source_count: $rts_openra_screen_for_screen_ui_replication[0].openra_reference_source_count,
      rts_openra_screen_for_screen_ui_replication_surface_name_count: $rts_openra_screen_for_screen_ui_replication[0].replicated_interaction_surface_name_count,
      rts_openra_screen_for_screen_ui_replication_gate_count: $rts_openra_screen_for_screen_ui_replication[0].gate_count,
      rts_openra_screen_for_screen_ui_replication_passed_gate_count: $rts_openra_screen_for_screen_ui_replication[0].passed_gate_count,
      rts_openra_screen_for_screen_ui_replication_failed_gate_count: $rts_openra_screen_for_screen_ui_replication[0].failed_gate_count,
      rts_openra_screen_for_screen_ui_replication_runtime_screen_mode: $rts_openra_screen_for_screen_ui_replication[0].runtime_screen_mode,
      rts_openra_screen_for_screen_ui_replication_evidence_board_only: $rts_openra_screen_for_screen_ui_replication[0].evidence_board_only,
      rts_openra_screen_for_screen_ui_replication_non_background_pixels: $rts_openra_screen_for_screen_ui_replication[0].pixel_counts.non_background,
      rts_openra_screen_for_screen_ui_replication_mainmenu_pixel_count: $rts_openra_screen_for_screen_ui_replication[0].pixel_counts.mainmenu,
      rts_openra_screen_for_screen_ui_replication_ingame_pixel_count: $rts_openra_screen_for_screen_ui_replication[0].pixel_counts.ingame,
      rts_openra_screen_for_screen_ui_replication_postgame_pixel_count: $rts_openra_screen_for_screen_ui_replication[0].pixel_counts.postgame_stats,
      rts_openra_screen_for_screen_ui_replication_player_first_ingame_view_non_background: $rts_openra_screen_for_screen_ui_replication[0].openra_style_ingame_pixel_counts.player_first_openra_style_ingame_view_non_background,
      rts_openra_screen_for_screen_ui_replication_player_first_ingame_sidebar_non_background: $rts_openra_screen_for_screen_ui_replication[0].openra_style_ingame_pixel_counts.player_first_openra_style_ingame_sidebar_non_background,
      rts_openra_screen_for_screen_ui_replication_player_first_ingame_command_lane_non_background: $rts_openra_screen_for_screen_ui_replication[0].openra_style_ingame_pixel_counts.player_first_openra_style_ingame_command_lane_non_background,
      rts_openra_screen_for_screen_ui_replication_style_screen_set_claimed: $rts_openra_screen_for_screen_ui_replication[0].openra_style_widget_root_screen_set_claimed,
      rts_openra_screen_for_screen_ui_replication_claimed: $rts_openra_screen_for_screen_ui_replication[0].openra_screen_for_screen_ui_replication_claimed,
      rts_openra_screen_for_screen_ui_replication_asset_parity_claimed: $rts_openra_screen_for_screen_ui_replication[0].openra_pixel_perfect_asset_parity_claimed,
      rts_openra_screen_for_screen_ui_replication_engine_port_claimed: $rts_openra_screen_for_screen_ui_replication[0].openra_engine_port_claimed,
      rts_openra_screen_for_screen_ui_replication_review_contract: $rts_openra_screen_for_screen_ui_replication[0].rts_evidence_openra_style_screen_set_review_contract,
      rts_openra_screen_for_screen_ui_replication_review_source_of_truth: $rts_openra_screen_for_screen_ui_replication[0].rts_evidence_openra_style_screen_set_review.source_of_truth,
      rts_openra_engine_port_asset_parity_module_count: $rts_openra_engine_port_asset_parity[0].ported_engine_module_count,
      rts_openra_engine_port_asset_parity_widget_root_count: $rts_openra_engine_port_asset_parity[0].openra_widget_root_count,
      rts_openra_engine_port_asset_parity_screen_count: $rts_openra_engine_port_asset_parity[0].openra_chrome_screen_count,
      rts_openra_engine_port_asset_parity_source_contract_count: $rts_openra_engine_port_asset_parity[0].source_contract_count,
      rts_openra_engine_port_asset_parity_source_headline_field_count: $rts_openra_engine_port_asset_parity[0].source_headline_field_count,
      rts_openra_engine_port_asset_parity_asset_manifest_field_count: $rts_openra_engine_port_asset_parity[0].asset_manifest_field_count,
      rts_openra_engine_port_asset_parity_pixel_parity_field_count: $rts_openra_engine_port_asset_parity[0].pixel_parity_field_count,
      rts_openra_engine_port_asset_parity_manifest_frame_id_count: $rts_openra_engine_port_asset_parity[0].pixel_parity_manifest_frame_id_count,
      rts_openra_engine_port_asset_parity_sample_report_count: $rts_openra_engine_port_asset_parity[0].pixel_parity_sample_report_count,
      rts_openra_engine_port_asset_parity_artifact_path_count: $rts_openra_engine_port_asset_parity[0].artifact_path_count,
      rts_openra_engine_port_asset_parity_pixel_count_field_count: $rts_openra_engine_port_asset_parity[0].pixel_count_field_count,
      rts_openra_engine_port_asset_parity_gate_count: $rts_openra_engine_port_asset_parity[0].gate_count,
      rts_openra_engine_port_asset_parity_passed_gate_count: $rts_openra_engine_port_asset_parity[0].passed_gate_count,
      rts_openra_engine_port_asset_parity_failed_gate_count: $rts_openra_engine_port_asset_parity[0].failed_gate_count,
      rts_openra_engine_port_asset_parity_sample_count: $rts_openra_engine_port_asset_parity[0].pixel_parity.sample_count,
      rts_openra_engine_port_asset_parity_sha_match_count: $rts_openra_engine_port_asset_parity[0].pixel_parity.sample_sha_match_count,
      rts_openra_engine_port_asset_parity_pixel_count: $rts_openra_engine_port_asset_parity[0].pixel_parity.sample_pixel_count,
      rts_openra_engine_port_asset_parity_visible_pixel_count: $rts_openra_engine_port_asset_parity[0].pixel_parity.sample_visible_pixel_count,
      rts_openra_engine_port_asset_parity_pixel_mismatch_count: $rts_openra_engine_port_asset_parity[0].pixel_parity.sample_pixel_mismatch_count,
      rts_openra_engine_port_asset_parity_reference_render_mismatch_count: $rts_openra_engine_port_asset_parity[0].pixel_parity.reference_render_pixel_mismatch_count,
      rts_openra_engine_port_asset_parity_claimed: $rts_openra_engine_port_asset_parity[0].openra_engine_port_claimed,
      rts_openra_engine_port_asset_parity_full_engine_claimed: $rts_openra_engine_port_asset_parity[0].openra_full_engine_port_claimed,
      rts_openra_engine_port_asset_parity_owned_asset_parity_claimed: $rts_openra_engine_port_asset_parity[0].trillionnium_owned_asset_pack_pixel_parity_claimed,
      rts_openra_engine_port_asset_parity_asset_parity_claimed: $rts_openra_engine_port_asset_parity[0].openra_pixel_perfect_asset_parity_claimed,
      rts_openra_engine_port_asset_parity_westwood_claimed: $rts_openra_engine_port_asset_parity[0].openra_westwood_pixel_perfect_asset_parity_claimed,
      rts_command_affordance_drag_marquee_pixel_count: $rts_command_affordance[0].drag_marquee_pixel_count,
      rts_command_affordance_right_click_marker_pixel_count: $rts_command_affordance[0].right_click_marker_pixel_count,
      rts_command_affordance_attack_cursor_pixel_count: $rts_command_affordance[0].attack_cursor_pixel_count,
      rts_command_affordance_hotkey_pixel_count: $rts_command_affordance[0].hotkey_pixel_count,
      rts_command_affordance_command_ack_pixel_count: $rts_command_affordance[0].command_ack_pixel_count,
      rts_command_surface_selection_frame_pixel_count: $rts_command_surface[0].selection_frame_pixel_count,
      rts_command_surface_ready_pixel_count: $rts_command_surface[0].ready_pixel_count,
      rts_command_surface_disabled_pixel_count: $rts_command_surface[0].disabled_pixel_count,
      rts_command_surface_cooldown_pixel_count: $rts_command_surface[0].cooldown_pixel_count,
      rts_command_surface_target_panel_pixel_count: $rts_command_surface[0].target_panel_pixel_count,
      rts_command_surface_queue_confirm_pixel_count: $rts_command_surface[0].queue_confirm_pixel_count,
      rts_command_surface_group_tab_pixel_count: $rts_command_surface[0].group_tab_pixel_count,
      rts_structure_modeling_foundation_shadow_pixel_count: $rts_structure_modeling[0].foundation_shadow_pixel_count,
      rts_structure_modeling_scaffold_pixel_count: $rts_structure_modeling[0].scaffold_pixel_count,
      rts_structure_modeling_construction_spark_pixel_count: $rts_structure_modeling[0].construction_spark_pixel_count,
      rts_structure_modeling_production_glow_pixel_count: $rts_structure_modeling[0].production_glow_pixel_count,
      rts_structure_modeling_damage_crack_pixel_count: $rts_structure_modeling[0].damage_crack_pixel_count,
      rts_structure_modeling_repair_beam_pixel_count: $rts_structure_modeling[0].repair_beam_pixel_count,
      rts_environment_life_tree_sway_pixel_count: $rts_environment_life[0].tree_sway_pixel_count,
      rts_environment_life_torch_flicker_pixel_count: $rts_environment_life[0].torch_flicker_pixel_count,
      rts_environment_life_water_shimmer_pixel_count: $rts_environment_life[0].water_shimmer_pixel_count,
      rts_environment_life_banner_flutter_pixel_count: $rts_environment_life[0].banner_flutter_pixel_count,
      rts_environment_life_resource_glint_pixel_count: $rts_environment_life[0].resource_glint_pixel_count,
      rts_environment_life_ambient_dust_pixel_count: $rts_environment_life[0].ambient_dust_pixel_count,
      rts_map_model_gap_stage_count: ($rts_map_model_gap[0].stage_summaries | length),
      rts_map_model_gap_lane_pixel_count: $rts_map_model_gap[0].lane_pixel_count,
      rts_map_model_gap_resource_pixel_count: $rts_map_model_gap[0].resource_pixel_count,
      rts_map_model_gap_height_pixel_count: $rts_map_model_gap[0].height_pixel_count,
      rts_map_model_gap_choke_pixel_count: $rts_map_model_gap[0].choke_pixel_count,
      rts_map_model_gap_structure_pixel_count: $rts_map_model_gap[0].structure_pixel_count,
      rts_map_model_gap_unit_role_pixel_count: $rts_map_model_gap[0].unit_role_pixel_count,
      rts_map_model_gap_occlusion_pixel_count: $rts_map_model_gap[0].occlusion_pixel_count,
      rts_map_model_gap_openra_parity_target_commit: $rts_map_model_gap[0].openra_parity_target_commit,
      rts_map_model_gap_bevy_openra_parity_state: $rts_map_model_gap[0].bevy_openra_parity_state,
      rts_map_model_gap_bevy_openra_parity_claimed: $rts_map_model_gap[0].bevy_openra_parity_claimed,
      rts_worker_harvest_animation_approach_pixel_count: $rts_worker_harvest_animation[0].approach_pixel_count,
      rts_worker_harvest_animation_tool_swing_pixel_count: $rts_worker_harvest_animation[0].tool_swing_pixel_count,
      rts_worker_harvest_animation_resource_pop_pixel_count: $rts_worker_harvest_animation[0].resource_pop_pixel_count,
      rts_worker_harvest_animation_carry_load_pixel_count: $rts_worker_harvest_animation[0].carry_load_pixel_count,
      rts_worker_harvest_animation_dropoff_burst_pixel_count: $rts_worker_harvest_animation[0].dropoff_burst_pixel_count,
      rts_worker_harvest_animation_return_path_pixel_count: $rts_worker_harvest_animation[0].return_path_pixel_count,
      rts_production_spawn_animation_queue_pulse_pixel_count: $rts_production_spawn_animation[0].queue_pulse_pixel_count,
      rts_production_spawn_animation_training_tick_pixel_count: $rts_production_spawn_animation[0].training_tick_pixel_count,
      rts_production_spawn_animation_spawn_door_pixel_count: $rts_production_spawn_animation[0].spawn_door_pixel_count,
      rts_production_spawn_animation_rally_flag_pixel_count: $rts_production_spawn_animation[0].rally_flag_pixel_count,
      rts_production_spawn_animation_formation_join_pixel_count: $rts_production_spawn_animation[0].formation_join_pixel_count,
      rts_production_spawn_animation_supply_flash_pixel_count: $rts_production_spawn_animation[0].supply_flash_pixel_count,
      rts_production_spawn_animation_accepted_input_count: $rts_production_spawn_animation[0].accepted_input_count,
      rts_production_spawn_animation_supply_used: $rts_production_spawn_animation[0].final_army_supply_used,
      rts_production_spawn_animation_supply_cap: $rts_production_spawn_animation[0].final_army_supply_cap,
      rts_production_spawn_animation_spawned_unit_count: ($rts_production_spawn_animation[0].final_army_spawned_unit_ids | length),
      rts_production_spawn_animation_rally_tile_count: ($rts_production_spawn_animation[0].final_army_rally_tile_ids | length),
      rts_production_spawn_animation_training_progress_percent: $rts_production_spawn_animation[0].final_training_progress_percent,
      rts_unit_status_portrait_frame_pixel_count: $rts_unit_status_portrait[0].portrait_frame_pixel_count,
      rts_unit_status_health_bar_pixel_count: $rts_unit_status_portrait[0].health_bar_pixel_count,
      rts_unit_status_mana_bar_pixel_count: $rts_unit_status_portrait[0].mana_bar_pixel_count,
      rts_unit_status_xp_bar_pixel_count: $rts_unit_status_portrait[0].xp_bar_pixel_count,
      rts_unit_status_buff_badge_pixel_count: $rts_unit_status_portrait[0].buff_badge_pixel_count,
      rts_unit_status_role_badge_pixel_count: $rts_unit_status_portrait[0].role_badge_pixel_count,
      rts_unit_status_queue_badge_pixel_count: $rts_unit_status_portrait[0].queue_badge_pixel_count,
      rts_selection_command_feedback_marquee_pixel_count: $rts_selection_command_feedback[0].marquee_pixel_count,
      rts_selection_command_feedback_confirm_pixel_count: $rts_selection_command_feedback[0].confirm_pixel_count,
      rts_selection_command_feedback_rally_pixel_count: $rts_selection_command_feedback[0].rally_pixel_count,
      rts_selection_command_feedback_move_pixel_count: $rts_selection_command_feedback[0].move_pixel_count,
      rts_selection_command_feedback_attack_pixel_count: $rts_selection_command_feedback[0].attack_pixel_count,
      rts_selection_command_feedback_error_pixel_count: $rts_selection_command_feedback[0].error_pixel_count,
      rts_selection_command_feedback_ack_pixel_count: $rts_selection_command_feedback[0].ack_pixel_count,
      rts_ability_tooltip_telegraph_tooltip_pixel_count: $rts_ability_tooltip_telegraph[0].tooltip_pixel_count,
      rts_ability_tooltip_telegraph_range_pixel_count: $rts_ability_tooltip_telegraph[0].range_pixel_count,
      rts_ability_tooltip_telegraph_windup_pixel_count: $rts_ability_tooltip_telegraph[0].windup_pixel_count,
      rts_ability_tooltip_telegraph_cooldown_pixel_count: $rts_ability_tooltip_telegraph[0].cooldown_pixel_count,
      rts_ability_tooltip_telegraph_queue_pixel_count: $rts_ability_tooltip_telegraph[0].queue_pixel_count,
      rts_ability_tooltip_telegraph_warning_pixel_count: $rts_ability_tooltip_telegraph[0].warning_pixel_count,
      rts_ability_tooltip_telegraph_accepted_input_count: $rts_ability_tooltip_telegraph[0].accepted_input_count,
      rts_ability_tooltip_telegraph_ability_count: ($rts_ability_tooltip_telegraph[0].final_ability_command_ids | length),
      rts_ability_tooltip_telegraph_cooldown_count: ($rts_ability_tooltip_telegraph[0].final_ability_cooldown_percents | length),
      rts_ability_tooltip_telegraph_queue_count: ($rts_ability_tooltip_telegraph[0].final_production_queue | length),
      rts_control_group_hotkey_feedback_assign_pixel_count: $rts_control_group_hotkey_feedback[0].assign_pixel_count,
      rts_control_group_hotkey_feedback_recall_pixel_count: $rts_control_group_hotkey_feedback[0].recall_pixel_count,
      rts_control_group_hotkey_feedback_camera_pixel_count: $rts_control_group_hotkey_feedback[0].camera_pixel_count,
      rts_control_group_hotkey_feedback_idle_pixel_count: $rts_control_group_hotkey_feedback[0].idle_pixel_count,
      rts_control_group_hotkey_feedback_production_pixel_count: $rts_control_group_hotkey_feedback[0].production_pixel_count,
      rts_control_group_hotkey_feedback_ability_pixel_count: $rts_control_group_hotkey_feedback[0].ability_pixel_count,
      rts_control_group_hotkey_feedback_accepted_input_count: $rts_control_group_hotkey_feedback[0].accepted_input_count,
      rts_control_group_hotkey_feedback_group_count: ($rts_control_group_hotkey_feedback[0].final_active_control_group_ids | length),
      rts_control_group_hotkey_feedback_queue_count: ($rts_control_group_hotkey_feedback[0].final_production_queue | length),
      rts_scrollable_map_camera_frame_pixel_count: $rts_scrollable_map[0].camera_frame_pixel_count,
      rts_scrollable_map_edge_pixel_count: $rts_scrollable_map[0].edge_pixel_count,
      rts_scrollable_map_drag_pixel_count: $rts_scrollable_map[0].drag_pixel_count,
      rts_scrollable_map_zoom_pixel_count: $rts_scrollable_map[0].zoom_pixel_count,
      rts_scrollable_map_minimap_pixel_count: $rts_scrollable_map[0].minimap_pixel_count,
      rts_scrollable_map_clamp_pixel_count: $rts_scrollable_map[0].clamp_pixel_count,
      rts_scrollable_map_input_action_count: $rts_scrollable_map[0].input_action_count,
      rts_camera_minimap_sync_viewport_pixel_count: $rts_camera_minimap_sync[0].viewport_pixel_count,
      rts_camera_minimap_sync_fog_pixel_count: $rts_camera_minimap_sync[0].fog_pixel_count,
      rts_camera_minimap_sync_reveal_pixel_count: $rts_camera_minimap_sync[0].reveal_pixel_count,
      rts_camera_minimap_sync_selection_pixel_count: $rts_camera_minimap_sync[0].selection_pixel_count,
      rts_camera_minimap_sync_route_pixel_count: $rts_camera_minimap_sync[0].route_pixel_count,
      rts_camera_minimap_sync_input_action_count: $rts_camera_minimap_sync[0].input_action_count,
      rts_camera_minimap_sync_revealed_tile_union_count: $rts_camera_minimap_sync[0].revealed_tile_union_count,
      rts_camera_minimap_sync_stage_summary_count: $rts_camera_minimap_sync[0].stage_summary_count,
      rts_camera_minimap_sync_stage_name_count: $rts_camera_minimap_sync[0].stage_name_count,
      rts_camera_minimap_sync_input_source_count: $rts_camera_minimap_sync[0].input_source_count,
      rts_camera_minimap_sync_large_map_field_count: $rts_camera_minimap_sync[0].large_map_field_count,
      rts_camera_minimap_sync_gate_count: $rts_camera_minimap_sync[0].gate_count,
      rts_camera_minimap_sync_passed_gate_count: $rts_camera_minimap_sync[0].passed_gate_count,
      rts_camera_minimap_sync_failed_gate_count: $rts_camera_minimap_sync[0].failed_gate_count,
      rts_command_queue_path_preview_queue_slot_pixel_count: $rts_command_queue_path_preview[0].queue_slot_pixel_count,
      rts_command_queue_path_preview_path_pixel_count: $rts_command_queue_path_preview[0].path_pixel_count,
      rts_command_queue_path_preview_waypoint_pixel_count: $rts_command_queue_path_preview[0].waypoint_pixel_count,
      rts_command_queue_path_preview_target_pixel_count: $rts_command_queue_path_preview[0].target_pixel_count,
      rts_command_queue_path_preview_reservation_pixel_count: $rts_command_queue_path_preview[0].reservation_pixel_count,
      rts_command_queue_path_preview_cancel_pixel_count: $rts_command_queue_path_preview[0].cancel_pixel_count,
      rts_command_queue_path_preview_accepted_input_count: $rts_command_queue_path_preview[0].accepted_input_count,
      rts_command_queue_path_preview_final_queue_count: ($rts_command_queue_path_preview[0].final_command_queue | length),
      rts_formation_move_preview_ghost_pixel_count: $rts_formation_move_preview[0].ghost_pixel_count,
      rts_formation_move_preview_path_pixel_count: $rts_formation_move_preview[0].path_pixel_count,
      rts_formation_move_preview_slot_pixel_count: $rts_formation_move_preview[0].slot_pixel_count,
      rts_formation_move_preview_collision_pixel_count: $rts_formation_move_preview[0].collision_pixel_count,
      rts_formation_move_preview_disperse_pixel_count: $rts_formation_move_preview[0].disperse_pixel_count,
      rts_formation_move_preview_commit_pixel_count: $rts_formation_move_preview[0].commit_pixel_count,
      rts_formation_move_preview_accepted_input_count: $rts_formation_move_preview[0].accepted_input_count,
      rts_formation_move_preview_final_slot_count: ($rts_formation_move_preview[0].final_formation_slot_tile_ids | length),
      rts_formation_move_execution_slot_pixel_count: $rts_formation_move_execution[0].slot_pixel_count,
      rts_formation_move_execution_reservation_pixel_count: $rts_formation_move_execution[0].reservation_pixel_count,
      rts_formation_move_execution_step_pixel_count: $rts_formation_move_execution[0].step_pixel_count,
      rts_formation_move_execution_avoidance_pixel_count: $rts_formation_move_execution[0].avoidance_pixel_count,
      rts_formation_move_execution_reroute_pixel_count: $rts_formation_move_execution[0].reroute_pixel_count,
      rts_formation_move_execution_arrival_pixel_count: $rts_formation_move_execution[0].arrival_pixel_count,
      rts_formation_move_execution_accepted_input_count: $rts_formation_move_execution[0].accepted_input_count,
      rts_formation_move_execution_final_slot_count: ($rts_formation_move_execution[0].final_formation_slot_tile_ids | length),
      rts_local_obstruction_recovery_block_pixel_count: $rts_local_obstruction_recovery[0].block_pixel_count,
      rts_local_obstruction_recovery_queue_pixel_count: $rts_local_obstruction_recovery[0].queue_pixel_count,
      rts_local_obstruction_recovery_side_step_pixel_count: $rts_local_obstruction_recovery[0].side_step_pixel_count,
      rts_local_obstruction_recovery_gap_pixel_count: $rts_local_obstruction_recovery[0].gap_pixel_count,
      rts_local_obstruction_recovery_resume_pixel_count: $rts_local_obstruction_recovery[0].resume_pixel_count,
      rts_local_obstruction_recovery_accepted_input_count: $rts_local_obstruction_recovery[0].accepted_input_count,
      rts_local_obstruction_recovery_final_route_count: ($rts_local_obstruction_recovery[0].final_group_route_tile_ids | length),
      rts_action_cadence_windup_pixel_count: $rts_action_cadence[0].windup_pixel_count,
      rts_action_cadence_strike_pixel_count: $rts_action_cadence[0].strike_pixel_count,
      rts_action_cadence_recovery_pixel_count: $rts_action_cadence[0].recovery_pixel_count,
      rts_action_cadence_carry_bob_pixel_count: $rts_action_cadence[0].carry_bob_pixel_count,
      rts_action_cadence_idle_breath_pixel_count: $rts_action_cadence[0].idle_breath_pixel_count,
      rts_action_cadence_shadow_smear_pixel_count: $rts_action_cadence[0].shadow_smear_pixel_count,
      rts_unit_model_depth_rim_pixel_count: $rts_unit_model_depth[0].rim_pixel_count,
      rts_unit_model_depth_armor_pixel_count: $rts_unit_model_depth[0].armor_pixel_count,
      rts_unit_model_depth_role_prop_pixel_count: $rts_unit_model_depth[0].role_prop_pixel_count,
      rts_unit_model_depth_face_shade_pixel_count: $rts_unit_model_depth[0].face_shade_pixel_count,
      rts_unit_model_depth_ground_contact_pixel_count: $rts_unit_model_depth[0].ground_contact_pixel_count,
      rts_unit_model_depth_layer_shadow_pixel_count: $rts_unit_model_depth[0].layer_shadow_pixel_count,
      rts_action_sequence_idle_pixel_count: $rts_action_sequence[0].idle_pixel_count,
      rts_action_sequence_windup_pixel_count: $rts_action_sequence[0].windup_pixel_count,
      rts_action_sequence_strike_pixel_count: $rts_action_sequence[0].strike_pixel_count,
      rts_action_sequence_recovery_pixel_count: $rts_action_sequence[0].recovery_pixel_count,
      rts_action_sequence_carry_up_pixel_count: $rts_action_sequence[0].carry_up_pixel_count,
      rts_action_sequence_carry_down_pixel_count: $rts_action_sequence[0].carry_down_pixel_count,
      rts_action_sequence_frame_ghost_pixel_count: $rts_action_sequence[0].frame_ghost_pixel_count,
      rts_npc_behavior_patrol_pixel_count: $rts_npc_behavior[0].patrol_pixel_count,
      rts_npc_behavior_engage_pixel_count: $rts_npc_behavior[0].engage_pixel_count,
      rts_npc_behavior_work_pixel_count: $rts_npc_behavior[0].work_pixel_count,
      rts_npc_behavior_carry_pixel_count: $rts_npc_behavior[0].carry_pixel_count,
      rts_npc_behavior_stalk_pixel_count: $rts_npc_behavior[0].stalk_pixel_count,
      rts_npc_behavior_retreat_pixel_count: $rts_npc_behavior[0].retreat_pixel_count,
      rts_npc_behavior_route_pixel_count: $rts_npc_behavior[0].route_pixel_count,
      rts_combat_impact_hit_pixel_count: $rts_combat_impact[0].hit_pixel_count,
      rts_combat_impact_stagger_pixel_count: $rts_combat_impact[0].stagger_pixel_count,
      rts_combat_impact_damage_pixel_count: $rts_combat_impact[0].damage_pixel_count,
      rts_combat_impact_death_pixel_count: $rts_combat_impact[0].death_pixel_count,
      rts_combat_impact_corpse_pixel_count: $rts_combat_impact[0].corpse_pixel_count,
      rts_combat_impact_dissolve_pixel_count: $rts_combat_impact[0].dissolve_pixel_count,
      rts_combat_impact_victory_pixel_count: $rts_combat_impact[0].victory_pixel_count,
      rts_locomotion_blend_path_pixel_count: $rts_locomotion_blend[0].path_pixel_count,
      rts_locomotion_blend_left_step_pixel_count: $rts_locomotion_blend[0].left_step_pixel_count,
      rts_locomotion_blend_right_step_pixel_count: $rts_locomotion_blend[0].right_step_pixel_count,
      rts_locomotion_blend_turn_pixel_count: $rts_locomotion_blend[0].turn_pixel_count,
      rts_locomotion_blend_slide_pixel_count: $rts_locomotion_blend[0].slide_pixel_count,
      rts_locomotion_blend_brake_pixel_count: $rts_locomotion_blend[0].brake_pixel_count,
      rts_npc_transition_alert_pixel_count: $rts_npc_transition[0].alert_pixel_count,
      rts_npc_transition_engage_pixel_count: $rts_npc_transition[0].engage_pixel_count,
      rts_npc_transition_pickup_pixel_count: $rts_npc_transition[0].pickup_pixel_count,
      rts_npc_transition_pounce_pixel_count: $rts_npc_transition[0].pounce_pixel_count,
      rts_npc_transition_recover_pixel_count: $rts_npc_transition[0].recover_pixel_count,
      rts_npc_transition_resume_pixel_count: $rts_npc_transition[0].resume_pixel_count,
      rts_depth_readability_foreground_pixel_count: $rts_depth_readability[0].foreground_pixel_count,
      rts_depth_readability_behind_pixel_count: $rts_depth_readability[0].behind_pixel_count,
      rts_depth_readability_building_mask_pixel_count: $rts_depth_readability[0].building_mask_pixel_count,
      rts_depth_readability_target_priority_pixel_count: $rts_depth_readability[0].target_priority_pixel_count,
      rts_depth_readability_path_occlusion_pixel_count: $rts_depth_readability[0].path_occlusion_pixel_count,
      rts_depth_readability_cutaway_pixel_count: $rts_depth_readability[0].cutaway_pixel_count,
      rts_combat_readability_pressure_player_first_view_non_background: $rts_combat_readability_pressure_readiness[0].combat_pressure_pixel_counts.player_first_combat_pressure_view_non_background,
      rts_combat_readability_pressure_player_first_view_frame_pixel_count: $rts_combat_readability_pressure_readiness[0].combat_pressure_pixel_counts.player_first_combat_pressure_view_frame,
      rts_combat_readability_pressure_player_first_status_strip_pixel_count: $rts_combat_readability_pressure_readiness[0].combat_pressure_pixel_counts.player_first_combat_pressure_status_strip,
      rts_combat_readability_pressure_player_first_rail_pixel_count: $rts_combat_readability_pressure_readiness[0].combat_pressure_pixel_counts.player_first_combat_pressure_rail,
      rts_combat_readability_pressure_player_first_command_lane_pixel_count: $rts_combat_readability_pressure_readiness[0].combat_pressure_pixel_counts.player_first_combat_pressure_command_lane,
      rts_combat_readability_pressure_player_first_alert_pixel_count: $rts_combat_readability_pressure_readiness[0].combat_pressure_pixel_counts.player_first_combat_pressure_alert,
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
      rts_live_input_production_feedback_chip_gate: $rts_live[0].production_feedback_chip_gate,
      rts_live_input_move_live_gate: $rts_live[0].move_live_gate,
      rts_live_input_waypoint_live_gate: $rts_live[0].waypoint_live_gate,
      rts_live_input_hold_live_gate: $rts_live[0].hold_live_gate,
      rts_live_input_patrol_live_gate: $rts_live[0].patrol_live_gate,
      rts_live_input_attack_move_live_gate: $rts_live[0].attack_move_live_gate,
      rts_live_input_stop_live_gate: $rts_live[0].stop_live_gate,
      rts_live_input_attack_live_gate: $rts_live[0].attack_live_gate,
      rts_live_input_ability_live_gate: $rts_live[0].ability_live_gate,
      rts_live_input_command_feedback_chip_gate: $rts_live[0].command_feedback_chip_gate,
      rts_live_input_command_queue_path_preview_shift_waypoints_gate: $rts_live[0].live_command_queue_path_preview_shift_waypoints_gate,
      rts_live_input_command_queue_path_preview_queue_stack_gate: $rts_live[0].live_command_queue_path_preview_queue_stack_gate,
      rts_live_input_command_queue_path_preview_rally_chain_gate: $rts_live[0].live_command_queue_path_preview_rally_chain_gate,
      rts_live_input_command_queue_path_preview_attack_focus_gate: $rts_live[0].live_command_queue_path_preview_attack_focus_gate,
      rts_live_input_command_queue_path_preview_cancel_repath_gate: $rts_live[0].live_command_queue_path_preview_cancel_repath_gate,
      rts_live_input_command_queue_path_preview_gate: $rts_live[0].live_command_queue_path_preview_gate,
      rts_live_input_hover_preview_gate: $rts_live[0].hover_preview_gate,
      rts_live_input_context_cursor_gate: $rts_live[0].context_cursor_gate,
      rts_live_input_viewport_world_input_gate: $rts_live[0].viewport_world_input_gate,
      rts_live_input_drag_select_preview_gate: $rts_live[0].drag_select_preview_gate,
      rts_live_input_drag_select_commit_gate: $rts_live[0].drag_select_commit_gate,
      rts_live_input_drag_select_filter_gate: $rts_live[0].drag_select_filter_gate,
      rts_live_input_unit_click_select_gate: $rts_live[0].unit_click_select_gate,
      rts_live_input_selection_clear_gate: $rts_live[0].selection_clear_gate,
      rts_live_input_right_click_target_gate: $rts_live[0].right_click_target_semantics_gate,
      rts_live_input_right_click_target_preview_gate: $rts_live[0].right_click_target_preview_gate,
      rts_live_input_right_click_execution_feedback_gate: $rts_live[0].right_click_execution_feedback_gate,
      rts_live_input_right_click_execution_feedback_player_label_gate: $rts_live[0].right_click_execution_feedback_player_label_gate,
      rts_live_input_rts_core_frame_order_gate: $rts_live[0].rts_core_frame_order_gate,
      rts_live_input_rts_core_headless_replay_gate: $rts_live[0].rts_core_headless_replay_gate,
      rts_live_input_unit_shift_select_gate: $rts_live[0].unit_shift_select_gate,
      rts_live_input_unit_double_click_select_gate: $rts_live[0].unit_double_click_select_gate,
      rts_live_input_control_group_hotkey_gate: $rts_live[0].control_group_hotkey_gate,
      rts_live_input_control_group_slot_visual_gate: $rts_live[0].control_group_slot_visual_gate,
      rts_live_input_command_stamp_gate: $rts_live[0].command_stamp_gate,
      rts_pathing_live_input_gate: $rts_path[0].live_pathing_input_gate,
      rts_pathing_path_tile_gate: $rts_path[0].path_tile_gate,
      rts_pathing_blocked_tile_gate: $rts_path[0].blocked_tile_gate,
      rts_pathing_formation_slot_gate: $rts_path[0].formation_slot_gate,
      rts_pathing_command_visual_gate: $rts_path[0].command_visual_gate,
      rts_pathing_core_frame_order_gate: $rts_path[0].rts_pathing_core_frame_order_gate,
      rts_pathing_core_headless_replay_gate: $rts_path[0].rts_pathing_core_headless_replay_gate,
      rts_collision_live_input_gate: $rts_collision[0].live_collision_input_gate,
      rts_collision_collision_response_gate: $rts_collision[0].collision_response_gate,
      rts_collision_engagement_response_gate: $rts_collision[0].engagement_response_gate,
      rts_collision_core_frame_order_gate: $rts_collision[0].rts_collision_core_frame_order_gate,
      rts_collision_core_headless_replay_gate: $rts_collision[0].rts_collision_core_headless_replay_gate,
      rts_targeting_live_input_gate: $rts_target[0].live_targeting_input_gate,
      rts_targeting_target_priority_gate: $rts_target[0].target_priority_gate,
      rts_targeting_aggro_gate: $rts_target[0].aggro_gate,
      rts_targeting_focus_fire_gate: $rts_target[0].focus_fire_gate,
      rts_targeting_threat_feedback_gate: $rts_target[0].threat_feedback_gate,
      rts_targeting_core_frame_order_gate: $rts_target[0].rts_targeting_core_frame_order_gate,
      rts_targeting_core_headless_replay_gate: $rts_target[0].rts_targeting_core_headless_replay_gate,
      rts_economy_live_input_gate: $rts_economy[0].live_economy_input_gate,
      rts_economy_harvest_loop_gate: $rts_economy[0].harvest_loop_gate,
      rts_economy_build_loop_gate: $rts_economy[0].build_loop_gate,
      rts_economy_production_loop_gate: $rts_economy[0].production_loop_gate,
      rts_economy_core_frame_order_gate: $rts_economy[0].rts_economy_core_frame_order_gate,
      rts_economy_core_headless_replay_gate: $rts_economy[0].rts_economy_core_headless_replay_gate,
      rts_selection_minimap_live_input_gate: $rts_select[0].live_selection_minimap_input_gate,
      rts_selection_box_gate: $rts_select[0].selection_box_gate,
      rts_control_group_gate: $rts_select[0].control_group_gate,
      rts_minimap_command_gate: $rts_select[0].minimap_command_gate,
      rts_split_route_gate: $rts_select[0].split_route_gate,
      rts_selection_minimap_core_frame_order_gate: $rts_select[0].rts_selection_minimap_core_frame_order_gate,
      rts_selection_minimap_core_headless_replay_gate: $rts_select[0].rts_selection_minimap_core_headless_replay_gate,
      rts_build_lifecycle_live_input_gate: $rts_build_lifecycle[0].live_build_lifecycle_input_gate,
      rts_build_lifecycle_build_placement_gate: $rts_build_lifecycle[0].build_placement_gate,
      rts_build_lifecycle_completion_gate: $rts_build_lifecycle[0].completion_gate,
      rts_build_lifecycle_repair_gate: $rts_build_lifecycle[0].repair_gate,
      rts_build_lifecycle_cancel_refund_gate: $rts_build_lifecycle[0].cancel_refund_gate,
      rts_build_lifecycle_core_frame_order_gate: $rts_build_lifecycle[0].rts_production_lifecycle_core_frame_order_gate,
      rts_build_lifecycle_core_headless_replay_gate: $rts_build_lifecycle[0].rts_production_lifecycle_core_headless_replay_gate,
      rts_tech_tree_live_input_gate: $rts_tech_tree[0].live_tech_tree_input_gate,
      rts_tech_tree_faction_base_gate: $rts_tech_tree[0].faction_base_gate,
      rts_tech_tree_research_gate: $rts_tech_tree[0].research_gate,
      rts_tech_tree_upgrade_gate: $rts_tech_tree[0].upgrade_gate,
      rts_tech_tree_unlock_gate: $rts_tech_tree[0].unlock_gate,
      rts_tech_tree_dependency_gate: $rts_tech_tree[0].dependency_gate,
      rts_tech_tree_core_frame_order_gate: $rts_tech_tree[0].rts_tech_tree_core_frame_order_gate,
      rts_tech_tree_core_headless_replay_gate: $rts_tech_tree[0].rts_tech_tree_core_headless_replay_gate,
      rts_projectile_ability_live_input_gate: $rts_projectile[0].live_projectile_ability_input_gate,
      rts_projectile_ability_projectile_trail_gate: $rts_projectile[0].projectile_trail_gate,
      rts_projectile_ability_projectile_impact_gate: $rts_projectile[0].projectile_impact_gate,
      rts_projectile_ability_ability_radius_gate: $rts_projectile[0].ability_radius_gate,
      rts_projectile_ability_damage_tick_gate: $rts_projectile[0].damage_tick_gate,
      rts_projectile_ability_armor_shield_gate: $rts_projectile[0].armor_shield_gate,
      rts_projectile_ability_core_frame_order_gate: $rts_projectile[0].rts_projectile_ability_core_frame_order_gate,
      rts_projectile_ability_core_headless_replay_gate: $rts_projectile[0].rts_projectile_ability_core_headless_replay_gate,
      rts_ai_skirmish_pressure_live_input_gate: $rts_ai[0].live_ai_skirmish_input_gate,
      rts_ai_skirmish_pressure_ai_wave_gate: $rts_ai[0].ai_wave_gate,
      rts_ai_skirmish_pressure_ai_counter_gate: $rts_ai[0].ai_counter_gate,
      rts_ai_skirmish_pressure_resolution_gate: $rts_ai[0].ai_pressure_resolution_gate,
      rts_ai_skirmish_pressure_retreat_gate: $rts_ai[0].ai_retreat_gate,
      rts_ai_skirmish_player_response_gate: $rts_ai[0].player_response_gate,
      rts_ai_skirmish_pressure_core_frame_order_gate: $rts_ai[0].rts_ai_skirmish_core_frame_order_gate,
      rts_ai_skirmish_pressure_core_headless_replay_gate: $rts_ai[0].rts_ai_skirmish_core_headless_replay_gate,
      rts_objective_victory_loop_live_input_gate: $rts_objective[0].live_objective_input_gate,
      rts_objective_victory_loop_marker_gate: $rts_objective[0].objective_marker_gate,
      rts_objective_victory_loop_capture_gate: $rts_objective[0].capture_progress_gate,
      rts_objective_victory_loop_victory_gate: $rts_objective[0].victory_resolution_gate,
      rts_objective_victory_loop_defeat_pressure_gate: $rts_objective[0].defeat_pressure_gate,
      rts_objective_victory_loop_extraction_gate: $rts_objective[0].extraction_gate,
      rts_objective_victory_loop_openra_parity_bridge_gate: $rts_objective[0].openra_parity_bridge_gate,
      rts_objective_victory_loop_core_frame_order_gate: $rts_objective[0].rts_objective_core_frame_order_gate,
      rts_objective_victory_loop_core_headless_replay_gate: $rts_objective[0].rts_objective_core_headless_replay_gate,
      rts_autonomous_bot_skirmish_no_live_player_input_gate: $rts_auto_bot[0].no_live_player_input_gate,
      rts_autonomous_bot_skirmish_timeline_gate: $rts_auto_bot[0].autonomous_timeline_gate,
      rts_autonomous_bot_skirmish_bot_roster_gate: $rts_auto_bot[0].bot_roster_gate,
      rts_autonomous_bot_skirmish_economy_gate: $rts_auto_bot[0].economy_gate,
      rts_autonomous_bot_skirmish_production_gate: $rts_auto_bot[0].production_gate,
      rts_autonomous_bot_skirmish_combat_gate: $rts_auto_bot[0].combat_gate,
      rts_autonomous_bot_skirmish_terminal_gate: $rts_auto_bot[0].terminal_gate,
      rts_autonomous_bot_skirmish_renderer_gate: $rts_auto_bot[0].renderer_gate,
      rts_autonomous_bot_skirmish_gate: $rts_auto_bot[0].autonomous_bot_skirmish_gate,
      rts_organic_terminal_gap_stage_gate: $rts_organic_terminal_gap[0].stage_gate,
      rts_organic_terminal_gap_observation_report_gate: $rts_organic_terminal_gap[0].observation_report_gate,
      rts_organic_terminal_gap_openra_target_gate: $rts_organic_terminal_gap[0].openra_target_gate,
      rts_organic_terminal_gap_bevy_gap_gate: $rts_organic_terminal_gap[0].bevy_gap_gate,
      rts_organic_terminal_gap_renderer_gate: $rts_organic_terminal_gap[0].renderer_gate,
      rts_organic_terminal_gap_openra_gap_not_closed_gate: $rts_organic_terminal_gap[0].openra_gap_not_closed_gate,
      rts_organic_terminal_gap_gate: $rts_organic_terminal_gap[0].organic_terminal_gap_gate,
      rts_terminal_observation_gap_stage_gate: $rts_terminal_observation_gap[0].stage_gate,
      rts_terminal_observation_gap_readiness_gate: $rts_terminal_observation_gap[0].terminal_readiness_gate,
      rts_terminal_observation_gap_observation_gate: $rts_terminal_observation_gap[0].terminal_observation_gate,
      rts_terminal_observation_gap_openra_target_gate: $rts_terminal_observation_gap[0].openra_target_gate,
      rts_terminal_observation_gap_bevy_gap_gate: $rts_terminal_observation_gap[0].bevy_gap_gate,
      rts_terminal_observation_gap_renderer_gate: $rts_terminal_observation_gap[0].renderer_gate,
      rts_terminal_observation_gap_openra_gap_not_closed_gate: $rts_terminal_observation_gap[0].openra_gap_not_closed_gate,
      rts_terminal_observation_gap_gate: $rts_terminal_observation_gap[0].terminal_observation_gap_gate,
      rts_replay_metrics_gap_stage_gate: $rts_replay_metrics_gap[0].replay_metrics_stage_gate,
      rts_replay_metrics_gap_roster_gate: $rts_replay_metrics_gap[0].replay_roster_gate,
      rts_replay_metrics_gap_token_gate: $rts_replay_metrics_gap[0].replay_token_gate,
      rts_replay_metrics_gap_battle_outcome_summary_gate: $rts_replay_metrics_gap[0].battle_outcome_summary_gate,
      rts_replay_metrics_gap_bevy_gap_gate: $rts_replay_metrics_gap[0].bevy_gap_gate,
      rts_replay_metrics_gap_openra_target_gate: $rts_replay_metrics_gap[0].openra_replay_metrics_target_gate,
      rts_replay_metrics_gap_renderer_gate: $rts_replay_metrics_gap[0].renderer_gate,
      rts_replay_metrics_gap_openra_gap_not_closed_gate: $rts_replay_metrics_gap[0].openra_gap_not_closed_gate,
      rts_replay_metrics_gap_gate: $rts_replay_metrics_gap[0].replay_metrics_gap_gate,
      rts_endurance_skirmish_gap_stage_gate: $rts_endurance_skirmish_gap[0].endurance_stage_gate,
      rts_endurance_skirmish_gap_roster_gate: $rts_endurance_skirmish_gap[0].endurance_roster_gate,
      rts_endurance_skirmish_gap_duration_gate: $rts_endurance_skirmish_gap[0].endurance_duration_gate,
      rts_endurance_skirmish_gap_pressure_gate: $rts_endurance_skirmish_gap[0].endurance_pressure_gate,
      rts_endurance_skirmish_gap_battle_outcome_gate: $rts_endurance_skirmish_gap[0].battle_outcome_gate,
      rts_endurance_skirmish_gap_bevy_gap_gate: $rts_endurance_skirmish_gap[0].bevy_gap_gate,
      rts_endurance_skirmish_gap_openra_target_gate: $rts_endurance_skirmish_gap[0].openra_endurance_target_gate,
      rts_endurance_skirmish_gap_renderer_gate: $rts_endurance_skirmish_gap[0].renderer_gate,
      rts_endurance_skirmish_gap_openra_gap_not_closed_gate: $rts_endurance_skirmish_gap[0].openra_gap_not_closed_gate,
      rts_endurance_skirmish_gap_gate: $rts_endurance_skirmish_gap[0].endurance_skirmish_gap_gate,
      rts_bot_decision_state_gap_stage_gate: $rts_bot_decision_state_gap[0].bot_decision_stage_gate,
      rts_bot_decision_state_gap_signal_gate: $rts_bot_decision_state_gap[0].bot_decision_signal_gate,
      rts_bot_decision_state_gap_economy_gate: $rts_bot_decision_state_gap[0].bot_decision_economy_gate,
      rts_bot_decision_state_gap_scout_gate: $rts_bot_decision_state_gap[0].bot_decision_scout_gate,
      rts_bot_decision_state_gap_capture_gate: $rts_bot_decision_state_gap[0].bot_decision_capture_gate,
      rts_bot_decision_state_gap_tech_gate: $rts_bot_decision_state_gap[0].bot_decision_tech_gate,
      rts_bot_decision_state_gap_counter_gate: $rts_bot_decision_state_gap[0].bot_decision_counter_gate,
      rts_bot_decision_state_gap_attack_gate: $rts_bot_decision_state_gap[0].bot_decision_attack_gate,
      rts_bot_decision_state_gap_retreat_gate: $rts_bot_decision_state_gap[0].bot_decision_retreat_gate,
      rts_bot_decision_state_gap_bevy_gap_gate: $rts_bot_decision_state_gap[0].bevy_gap_gate,
      rts_bot_decision_state_gap_openra_target_gate: $rts_bot_decision_state_gap[0].openra_bot_decision_target_gate,
      rts_bot_decision_state_gap_renderer_gate: $rts_bot_decision_state_gap[0].renderer_gate,
      rts_bot_decision_state_gap_openra_gap_not_closed_gate: $rts_bot_decision_state_gap[0].openra_gap_not_closed_gate,
      rts_bot_decision_state_gap_core_frame_order_gate: $rts_bot_decision_state_gap[0].rts_bot_decision_core_frame_order_gate,
      rts_bot_decision_state_gap_core_headless_replay_gate: $rts_bot_decision_state_gap[0].rts_bot_decision_core_headless_replay_gate,
      rts_bot_decision_state_gap_gate: $rts_bot_decision_state_gap[0].bot_decision_state_gap_gate,
      rts_bot_adaptive_build_order_gap_stage_gate: $rts_bot_adaptive_build_order_gap[0].adaptive_stage_gate,
      rts_bot_adaptive_build_order_gap_signal_gate: $rts_bot_adaptive_build_order_gap[0].adaptive_signal_gate,
      rts_bot_adaptive_build_order_gap_opening_gate: $rts_bot_adaptive_build_order_gap[0].adaptive_opening_gate,
      rts_bot_adaptive_build_order_gap_scout_gate: $rts_bot_adaptive_build_order_gap[0].adaptive_scout_gate,
      rts_bot_adaptive_build_order_gap_branch_gate: $rts_bot_adaptive_build_order_gap[0].adaptive_branch_gate,
      rts_bot_adaptive_build_order_gap_tech_gate: $rts_bot_adaptive_build_order_gap[0].adaptive_tech_gate,
      rts_bot_adaptive_build_order_gap_pressure_gate: $rts_bot_adaptive_build_order_gap[0].adaptive_pressure_gate,
      rts_bot_adaptive_build_order_gap_retreat_rebuild_gate: $rts_bot_adaptive_build_order_gap[0].adaptive_retreat_rebuild_gate,
      rts_bot_adaptive_build_order_gap_bevy_gap_gate: $rts_bot_adaptive_build_order_gap[0].bevy_gap_gate,
      rts_bot_adaptive_build_order_gap_openra_target_gate: $rts_bot_adaptive_build_order_gap[0].openra_adaptive_build_target_gate,
	      rts_bot_adaptive_build_order_gap_renderer_gate: $rts_bot_adaptive_build_order_gap[0].renderer_gate,
	      rts_bot_adaptive_build_order_gap_openra_gap_not_closed_gate: $rts_bot_adaptive_build_order_gap[0].openra_gap_not_closed_gate,
	      rts_bot_adaptive_build_order_gap_core_frame_order_gate: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_frame_order_gate,
	      rts_bot_adaptive_build_order_gap_core_headless_replay_gate: $rts_bot_adaptive_build_order_gap[0].rts_bot_adaptive_core_headless_replay_gate,
	      rts_bot_adaptive_build_order_gap_gate: $rts_bot_adaptive_build_order_gap[0].adaptive_build_order_gap_gate,
      rts_bot_tactical_micro_gap_stage_gate: $rts_bot_tactical_micro_gap[0].micro_stage_gate,
      rts_bot_tactical_micro_gap_signal_gate: $rts_bot_tactical_micro_gap[0].micro_signal_gate,
      rts_bot_tactical_micro_gap_target_gate: $rts_bot_tactical_micro_gap[0].micro_target_gate,
      rts_bot_tactical_micro_gap_focus_gate: $rts_bot_tactical_micro_gap[0].micro_focus_gate,
      rts_bot_tactical_micro_gap_kite_gate: $rts_bot_tactical_micro_gap[0].micro_kite_gate,
      rts_bot_tactical_micro_gap_flank_gate: $rts_bot_tactical_micro_gap[0].micro_flank_gate,
      rts_bot_tactical_micro_gap_ability_gate: $rts_bot_tactical_micro_gap[0].micro_ability_gate,
      rts_bot_tactical_micro_gap_pullback_gate: $rts_bot_tactical_micro_gap[0].micro_pullback_gate,
      rts_bot_tactical_micro_gap_bevy_gap_gate: $rts_bot_tactical_micro_gap[0].bevy_gap_gate,
      rts_bot_tactical_micro_gap_openra_target_gate: $rts_bot_tactical_micro_gap[0].openra_tactical_micro_target_gate,
      rts_bot_tactical_micro_gap_renderer_gate: $rts_bot_tactical_micro_gap[0].renderer_gate,
      rts_bot_tactical_micro_gap_openra_gap_not_closed_gate: $rts_bot_tactical_micro_gap[0].openra_gap_not_closed_gate,
      rts_bot_tactical_micro_gap_core_frame_order_gate: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_frame_order_gate,
      rts_bot_tactical_micro_gap_core_headless_replay_gate: $rts_bot_tactical_micro_gap[0].rts_bot_tactical_micro_core_headless_replay_gate,
      rts_bot_tactical_micro_gap_gate: $rts_bot_tactical_micro_gap[0].tactical_micro_gap_gate,
      rts_bot_map_intel_gap_stage_gate: $rts_bot_map_intel_gap[0].intel_stage_gate,
      rts_bot_map_intel_gap_signal_gate: $rts_bot_map_intel_gap[0].intel_signal_gate,
      rts_bot_map_intel_gap_scout_gate: $rts_bot_map_intel_gap[0].intel_scout_gate,
      rts_bot_map_intel_gap_fog_memory_gate: $rts_bot_map_intel_gap[0].intel_fog_memory_gate,
      rts_bot_map_intel_gap_expansion_gate: $rts_bot_map_intel_gap[0].intel_expansion_gate,
      rts_bot_map_intel_gap_tech_gate: $rts_bot_map_intel_gap[0].intel_tech_gate,
      rts_bot_map_intel_gap_hidden_army_gate: $rts_bot_map_intel_gap[0].intel_hidden_army_gate,
      rts_bot_map_intel_gap_rotation_gate: $rts_bot_map_intel_gap[0].intel_rotation_gate,
      rts_bot_map_intel_gap_bevy_gap_gate: $rts_bot_map_intel_gap[0].bevy_gap_gate,
      rts_bot_map_intel_gap_openra_target_gate: $rts_bot_map_intel_gap[0].openra_map_intel_target_gate,
      rts_bot_map_intel_gap_renderer_gate: $rts_bot_map_intel_gap[0].renderer_gate,
      rts_bot_map_intel_gap_openra_gap_not_closed_gate: $rts_bot_map_intel_gap[0].openra_gap_not_closed_gate,
      rts_bot_map_intel_gap_core_frame_order_gate: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_frame_order_gate,
      rts_bot_map_intel_gap_core_headless_replay_gate: $rts_bot_map_intel_gap[0].rts_bot_map_intel_core_headless_replay_gate,
      rts_bot_map_intel_gap_gate: $rts_bot_map_intel_gap[0].map_intel_gap_gate,
      rts_bot_macro_economy_gap_stage_gate: $rts_bot_macro_economy_gap[0].macro_stage_gate,
      rts_bot_macro_economy_gap_signal_gate: $rts_bot_macro_economy_gap[0].macro_signal_gate,
      rts_bot_macro_economy_gap_worker_gate: $rts_bot_macro_economy_gap[0].macro_worker_gate,
      rts_bot_macro_economy_gap_expand_gate: $rts_bot_macro_economy_gap[0].macro_expand_gate,
      rts_bot_macro_economy_gap_supply_gate: $rts_bot_macro_economy_gap[0].macro_supply_gate,
      rts_bot_macro_economy_gap_production_gate: $rts_bot_macro_economy_gap[0].macro_production_gate,
      rts_bot_macro_economy_gap_tech_gate: $rts_bot_macro_economy_gap[0].macro_tech_gate,
      rts_bot_macro_economy_gap_deny_rebuild_gate: $rts_bot_macro_economy_gap[0].macro_deny_rebuild_gate,
      rts_bot_macro_economy_gap_bevy_gap_gate: $rts_bot_macro_economy_gap[0].bevy_gap_gate,
      rts_bot_macro_economy_gap_openra_target_gate: $rts_bot_macro_economy_gap[0].openra_macro_economy_target_gate,
      rts_bot_macro_economy_gap_renderer_gate: $rts_bot_macro_economy_gap[0].renderer_gate,
      rts_bot_macro_economy_gap_openra_gap_not_closed_gate: $rts_bot_macro_economy_gap[0].openra_gap_not_closed_gate,
      rts_bot_macro_economy_gap_core_frame_order_gate: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_frame_order_gate,
      rts_bot_macro_economy_gap_core_headless_replay_gate: $rts_bot_macro_economy_gap[0].rts_bot_macro_economy_core_headless_replay_gate,
      rts_bot_macro_economy_gap_gate: $rts_bot_macro_economy_gap[0].macro_economy_gap_gate,
      rts_bot_harassment_defense_gap_stage_gate: $rts_bot_harassment_defense_gap[0].harassment_stage_gate,
      rts_bot_harassment_defense_gap_signal_gate: $rts_bot_harassment_defense_gap[0].harassment_signal_gate,
      rts_bot_harassment_defense_gap_worker_gate: $rts_bot_harassment_defense_gap[0].harassment_worker_gate,
      rts_bot_harassment_defense_gap_repair_gate: $rts_bot_harassment_defense_gap[0].harassment_repair_gate,
      rts_bot_harassment_defense_gap_static_defense_gate: $rts_bot_harassment_defense_gap[0].harassment_static_defense_gate,
      rts_bot_harassment_defense_gap_counter_raid_gate: $rts_bot_harassment_defense_gap[0].harassment_counter_raid_gate,
      rts_bot_harassment_defense_gap_retreat_gate: $rts_bot_harassment_defense_gap[0].harassment_retreat_gate,
      rts_bot_harassment_defense_gap_rebuild_gate: $rts_bot_harassment_defense_gap[0].harassment_rebuild_gate,
      rts_bot_harassment_defense_gap_bevy_gap_gate: $rts_bot_harassment_defense_gap[0].bevy_gap_gate,
      rts_bot_harassment_defense_gap_openra_target_gate: $rts_bot_harassment_defense_gap[0].openra_harassment_defense_target_gate,
      rts_bot_harassment_defense_gap_renderer_gate: $rts_bot_harassment_defense_gap[0].renderer_gate,
      rts_bot_harassment_defense_gap_openra_gap_not_closed_gate: $rts_bot_harassment_defense_gap[0].openra_gap_not_closed_gate,
      rts_bot_harassment_defense_gap_core_frame_order_gate: $rts_bot_harassment_defense_gap[0].rts_bot_harassment_defense_core_frame_order_gate,
      rts_bot_harassment_defense_gap_core_headless_replay_gate: $rts_bot_harassment_defense_gap[0].rts_bot_harassment_defense_core_headless_replay_gate,
      rts_bot_harassment_defense_gap_gate: $rts_bot_harassment_defense_gap[0].harassment_defense_gap_gate,
      rts_bot_multi_front_pressure_gap_stage_gate: $rts_bot_multi_front_pressure_gap[0].multi_front_stage_gate,
      rts_bot_multi_front_pressure_gap_signal_gate: $rts_bot_multi_front_pressure_gap[0].multi_front_signal_gate,
      rts_bot_multi_front_pressure_gap_split_gate: $rts_bot_multi_front_pressure_gap[0].multi_front_split_gate,
      rts_bot_multi_front_pressure_gap_decoy_gate: $rts_bot_multi_front_pressure_gap[0].multi_front_decoy_gate,
      rts_bot_multi_front_pressure_gap_rotation_gate: $rts_bot_multi_front_pressure_gap[0].multi_front_rotation_gate,
      rts_bot_multi_front_pressure_gap_reinforce_gate: $rts_bot_multi_front_pressure_gap[0].multi_front_reinforce_gate,
      rts_bot_multi_front_pressure_gap_simultaneous_gate: $rts_bot_multi_front_pressure_gap[0].multi_front_simultaneous_gate,
      rts_bot_multi_front_pressure_gap_terminal_gate: $rts_bot_multi_front_pressure_gap[0].multi_front_terminal_gate,
      rts_bot_multi_front_pressure_gap_bevy_gap_gate: $rts_bot_multi_front_pressure_gap[0].bevy_gap_gate,
      rts_bot_multi_front_pressure_gap_openra_target_gate: $rts_bot_multi_front_pressure_gap[0].openra_multi_front_pressure_target_gate,
      rts_bot_multi_front_pressure_gap_renderer_gate: $rts_bot_multi_front_pressure_gap[0].renderer_gate,
      rts_bot_multi_front_pressure_gap_openra_gap_not_closed_gate: $rts_bot_multi_front_pressure_gap[0].openra_gap_not_closed_gate,
      rts_bot_multi_front_pressure_gap_gate: $rts_bot_multi_front_pressure_gap[0].multi_front_pressure_gap_gate,
      rts_bot_expansion_control_gap_stage_gate: $rts_bot_expansion_control_gap[0].expansion_control_stage_gate,
      rts_bot_expansion_control_gap_signal_gate: $rts_bot_expansion_control_gap[0].expansion_control_signal_gate,
      rts_bot_expansion_control_gap_natural_gate: $rts_bot_expansion_control_gap[0].expansion_control_natural_gate,
      rts_bot_expansion_control_gap_third_node_gate: $rts_bot_expansion_control_gap[0].expansion_control_third_node_gate,
      rts_bot_expansion_control_gap_refinery_gate: $rts_bot_expansion_control_gap[0].expansion_control_refinery_gate,
      rts_bot_expansion_control_gap_contain_gate: $rts_bot_expansion_control_gap[0].expansion_control_contain_gate,
      rts_bot_expansion_control_gap_reexpand_gate: $rts_bot_expansion_control_gap[0].expansion_control_reexpand_gate,
      rts_bot_expansion_control_gap_lock_gate: $rts_bot_expansion_control_gap[0].expansion_control_lock_gate,
      rts_bot_expansion_control_gap_bevy_gap_gate: $rts_bot_expansion_control_gap[0].bevy_gap_gate,
      rts_bot_expansion_control_gap_openra_target_gate: $rts_bot_expansion_control_gap[0].openra_expansion_control_target_gate,
      rts_bot_expansion_control_gap_renderer_gate: $rts_bot_expansion_control_gap[0].renderer_gate,
      rts_bot_expansion_control_gap_openra_gap_not_closed_gate: $rts_bot_expansion_control_gap[0].openra_gap_not_closed_gate,
      rts_bot_expansion_control_gap_gate: $rts_bot_expansion_control_gap[0].expansion_control_gap_gate,
      rts_bot_tech_transition_gap_stage_gate: $rts_bot_tech_transition_gap[0].tech_transition_stage_gate,
      rts_bot_tech_transition_gap_signal_gate: $rts_bot_tech_transition_gap[0].tech_transition_signal_gate,
      rts_bot_tech_transition_gap_signal_read_gate: $rts_bot_tech_transition_gap[0].tech_transition_signal_read_gate,
      rts_bot_tech_transition_gap_counter_gate: $rts_bot_tech_transition_gap[0].tech_transition_counter_gate,
      rts_bot_tech_transition_gap_anti_air_gate: $rts_bot_tech_transition_gap[0].tech_transition_anti_air_gate,
      rts_bot_tech_transition_gap_siege_gate: $rts_bot_tech_transition_gap[0].tech_transition_siege_gate,
      rts_bot_tech_transition_gap_upgrade_gate: $rts_bot_tech_transition_gap[0].tech_transition_upgrade_gate,
      rts_bot_tech_transition_gap_terminal_gate: $rts_bot_tech_transition_gap[0].tech_transition_terminal_gate,
      rts_bot_tech_transition_gap_bevy_gap_gate: $rts_bot_tech_transition_gap[0].bevy_gap_gate,
      rts_bot_tech_transition_gap_openra_target_gate: $rts_bot_tech_transition_gap[0].openra_tech_transition_target_gate,
      rts_bot_tech_transition_gap_renderer_gate: $rts_bot_tech_transition_gap[0].renderer_gate,
      rts_bot_tech_transition_gap_openra_gap_not_closed_gate: $rts_bot_tech_transition_gap[0].openra_gap_not_closed_gate,
      rts_bot_tech_transition_gap_gate: $rts_bot_tech_transition_gap[0].tech_transition_gap_gate,
      rts_bot_army_composition_gap_stage_gate: $rts_bot_army_composition_gap[0].army_composition_stage_gate,
      rts_bot_army_composition_gap_signal_gate: $rts_bot_army_composition_gap[0].army_composition_signal_gate,
      rts_bot_army_composition_gap_unit_mix_gate: $rts_bot_army_composition_gap[0].army_composition_unit_mix_gate,
      rts_bot_army_composition_gap_ratio_gate: $rts_bot_army_composition_gap[0].army_composition_ratio_gate,
      rts_bot_army_composition_gap_counter_gate: $rts_bot_army_composition_gap[0].army_composition_counter_gate,
      rts_bot_army_composition_gap_reinforce_gate: $rts_bot_army_composition_gap[0].army_composition_reinforce_gate,
      rts_bot_army_composition_gap_specialist_gate: $rts_bot_army_composition_gap[0].army_composition_specialist_gate,
      rts_bot_army_composition_gap_lock_gate: $rts_bot_army_composition_gap[0].army_composition_lock_gate,
      rts_bot_army_composition_gap_bevy_gap_gate: $rts_bot_army_composition_gap[0].bevy_gap_gate,
      rts_bot_army_composition_gap_openra_target_gate: $rts_bot_army_composition_gap[0].openra_army_composition_target_gate,
      rts_bot_army_composition_gap_renderer_gate: $rts_bot_army_composition_gap[0].renderer_gate,
      rts_bot_army_composition_gap_openra_gap_not_closed_gate: $rts_bot_army_composition_gap[0].openra_gap_not_closed_gate,
      rts_bot_army_composition_gap_gate: $rts_bot_army_composition_gap[0].army_composition_gap_gate,
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
      rts_fog_scouting_intel_core_frame_order_gate: $rts_fog[0].rts_fog_core_frame_order_gate,
      rts_fog_scouting_intel_core_headless_replay_gate: $rts_fog[0].rts_fog_core_headless_replay_gate,
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
      rts_open_world_after_action_runtime_screen_gate: $rts_open_world[0].runtime_screen_gate,
      rts_open_world_after_action_player_first_screen_gate: $rts_open_world[0].player_first_open_world_after_action_screen_gate,
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
      rts_visual_fidelity_mature_hud_gate: $rts_visual_fidelity[0].mature_rts_hud_gate,
      rts_visual_fidelity_selected_units_gate: $rts_visual_fidelity[0].selected_units_gate,
      rts_visual_fidelity_command_surface_gate: $rts_visual_fidelity[0].command_surface_gate,
      rts_visual_fidelity_model_gate: $rts_visual_fidelity[0].model_fidelity_gate,
      rts_visual_fidelity_npc_animation_gate: $rts_visual_fidelity[0].npc_animation_gate,
      rts_visual_fidelity_original_art_policy_gate: $rts_visual_fidelity[0].original_art_policy_gate,
      rts_production_asset_atlas_sprite_sheet_gate: $rts_production_asset_atlas[0].sprite_sheet_gate,
      rts_production_asset_atlas_texture_atlas_binding_gate: $rts_production_asset_atlas[0].texture_atlas_binding_gate,
      rts_production_asset_atlas_runtime_texture_asset_gate: $rts_production_asset_atlas[0].runtime_texture_asset_gate,
      rts_production_asset_atlas_preview_gate: $rts_production_asset_atlas[0].production_asset_atlas_preview_gate,
      rts_production_asset_atlas_gate: $rts_production_asset_atlas[0].production_asset_atlas_gate,
      rts_production_asset_atlas_no_copy_boundary_gate: $rts_production_asset_atlas[0].no_copy_boundary_gate,
      rts_production_ui_skin_asset_atlas_gate: $rts_production_ui_skin[0].asset_atlas_gate,
      rts_production_ui_skin_command_surface_skin_gate: $rts_production_ui_skin[0].command_surface_skin_gate,
      rts_production_ui_skin_selection_minimap_skin_gate: $rts_production_ui_skin[0].selection_minimap_skin_gate,
      rts_production_ui_skin_unit_status_skin_gate: $rts_production_ui_skin[0].unit_status_skin_gate,
      rts_production_ui_skin_command_feedback_skin_gate: $rts_production_ui_skin[0].command_feedback_skin_gate,
      rts_production_ui_skin_tooltip_skin_gate: $rts_production_ui_skin[0].tooltip_skin_gate,
      rts_production_ui_skin_hotkey_skin_gate: $rts_production_ui_skin[0].hotkey_skin_gate,
      rts_production_ui_skin_preview_gate: $rts_production_ui_skin[0].production_ui_skin_preview_gate,
      rts_production_ui_skin_player_first_screen_gate: $rts_production_ui_skin[0].player_first_production_hud_skin_screen_gate,
      rts_production_ui_skin_source_preview_gate: $rts_production_ui_skin[0].source_preview_gate,
      rts_production_ui_skin_no_copy_boundary_gate: $rts_production_ui_skin[0].no_copy_boundary_gate,
      rts_production_ui_skin_gate: $rts_production_ui_skin[0].production_ui_skin_gate,
      rts_production_interaction_polish_ui_skin_gate: $rts_production_interaction_polish[0].ui_skin_gate,
      rts_production_interaction_polish_command_affordance_gate: $rts_production_interaction_polish[0].command_affordance_gate,
      rts_production_interaction_polish_selection_feedback_gate: $rts_production_interaction_polish[0].selection_feedback_gate,
      rts_production_interaction_polish_build_lifecycle_gate: $rts_production_interaction_polish[0].build_lifecycle_gate,
      rts_production_interaction_polish_scrollable_map_gate: $rts_production_interaction_polish[0].scrollable_map_gate,
      rts_production_interaction_polish_command_queue_path_gate: $rts_production_interaction_polish[0].command_queue_path_gate,
      rts_production_interaction_polish_preview_gate: $rts_production_interaction_polish[0].production_interaction_polish_preview_gate,
      rts_production_interaction_polish_player_first_screen_gate: $rts_production_interaction_polish[0].player_first_command_interaction_screen_gate,
      rts_production_interaction_polish_source_preview_gate: $rts_production_interaction_polish[0].source_preview_gate,
      rts_production_interaction_polish_no_copy_boundary_gate: $rts_production_interaction_polish[0].no_copy_boundary_gate,
      rts_production_interaction_polish_gate: $rts_production_interaction_polish[0].production_interaction_polish_gate,
      rts_full_screen_ui_replication_title_campaign_gate: $rts_full_screen_ui_replication[0].title_campaign_gate,
      rts_full_screen_ui_replication_tactical_viewport_gate: $rts_full_screen_ui_replication[0].tactical_viewport_gate,
      rts_full_screen_ui_replication_map_minimap_gate: $rts_full_screen_ui_replication[0].map_minimap_gate,
      rts_full_screen_ui_replication_production_skin_gate: $rts_full_screen_ui_replication[0].production_skin_gate,
      rts_full_screen_ui_replication_interaction_polish_gate: $rts_full_screen_ui_replication[0].interaction_polish_gate,
      rts_full_screen_ui_replication_build_tech_gate: $rts_full_screen_ui_replication[0].build_tech_gate,
      rts_full_screen_ui_replication_combat_ui_gate: $rts_full_screen_ui_replication[0].combat_ui_gate,
      rts_full_screen_ui_replication_campaign_outcome_gate: $rts_full_screen_ui_replication[0].campaign_outcome_gate,
      rts_full_screen_ui_replication_player_first_screen_gate: $rts_full_screen_ui_replication[0].player_first_full_screen_ui_surface_gate,
      rts_full_screen_ui_replication_gate: $rts_full_screen_ui_replication[0].full_screen_ui_replication_gate,
      rts_shell_meta_ui_replication_full_screen_gate: $rts_shell_meta_ui_replication[0].full_screen_ui_replication_gate,
      rts_shell_meta_ui_replication_account_title_gate: $rts_shell_meta_ui_replication[0].account_title_gate,
      rts_shell_meta_ui_replication_title_menu_gate: $rts_shell_meta_ui_replication[0].title_menu_gate,
      rts_shell_meta_ui_replication_character_create_gate: $rts_shell_meta_ui_replication[0].character_create_gate,
      rts_shell_meta_ui_replication_session_slot_menu_gate: $rts_shell_meta_ui_replication[0].session_slot_menu_gate,
      rts_shell_meta_ui_replication_session_save_slot_gate: $rts_shell_meta_ui_replication[0].session_save_slot_gate,
      rts_shell_meta_ui_replication_session_slot_confirm_gate: $rts_shell_meta_ui_replication[0].session_slot_confirm_gate,
      rts_shell_meta_ui_replication_session_load_resume_gate: $rts_shell_meta_ui_replication[0].session_load_resume_gate,
      rts_shell_meta_ui_replication_session_recovery_gate: $rts_shell_meta_ui_replication[0].session_recovery_gate,
      rts_shell_meta_ui_replication_pause_menu_gate: $rts_shell_meta_ui_replication[0].pause_menu_gate,
      rts_shell_meta_ui_replication_settings_menu_gate: $rts_shell_meta_ui_replication[0].settings_menu_gate,
      rts_shell_meta_ui_replication_input_hud_gate: $rts_shell_meta_ui_replication[0].input_hud_gate,
      rts_shell_meta_ui_replication_visible_hit_test_gate: $rts_shell_meta_ui_replication[0].visible_hit_test_gate,
      rts_shell_meta_ui_replication_first_minute_gate: $rts_shell_meta_ui_replication[0].first_minute_onboarding_gate,
      rts_shell_meta_ui_replication_player_first_screen_gate: $rts_shell_meta_ui_replication[0].player_first_shell_meta_screen_gate,
      rts_shell_meta_ui_replication_gate: $rts_shell_meta_ui_replication[0].shell_meta_ui_replication_gate,
      rts_match_setup_ui_replication_shell_meta_gate: $rts_match_setup_ui_replication[0].shell_meta_gate,
      rts_match_setup_ui_replication_campaign_entry_gate: $rts_match_setup_ui_replication[0].campaign_entry_gate,
      rts_match_setup_ui_replication_map_spec_gate: $rts_match_setup_ui_replication[0].map_spec_gate,
      rts_match_setup_ui_replication_map_ui_gate: $rts_match_setup_ui_replication[0].map_ui_gate,
      rts_match_setup_ui_replication_faction_gate: $rts_match_setup_ui_replication[0].faction_gate,
      rts_match_setup_ui_replication_no_external_boundary_gate: $rts_match_setup_ui_replication[0].no_external_boundary_gate,
      rts_match_setup_ui_replication_player_first_screen_gate: $rts_match_setup_ui_replication[0].player_first_match_setup_screen_gate,
      rts_match_setup_ui_replication_gate: $rts_match_setup_ui_replication[0].match_setup_ui_replication_gate,
      rts_first_contact_basin_spec_gate: ok($rts_first_contact_basin_spec),
      rts_first_contact_runtime_review_gate: $rts_first_contact_basin_spec[0].rts_evidence_bevy_runtime_adapter.first_contact_runtime_review_gate,
      rts_first_contact_runtime_adapter_evidence_gate: $rts_first_contact_basin_spec[0].rts_evidence_bevy_runtime_adapter_gate,
      rts_first_contact_offline_adapter_consumption_gate: $rts_first_contact_basin_spec[0].rts_online_offline_adapter_consumption_gate,
      rts_first_contact_offline_adapter_session_transition_gate: $rts_first_contact_basin_spec[0].rts_online_offline_adapter_session_transition_gate,
      rts_first_contact_offline_adapter_lobby_ready_gate: $rts_first_contact_basin_spec[0].rts_online_offline_adapter_lobby_ready_gate,
      rts_campaign_outcome_ui_readiness_runtime_screen_gate: $rts_campaign_outcome_ui_readiness[0].runtime_screen_gate,
      rts_campaign_outcome_ui_readiness_first_minute_gate: $rts_campaign_outcome_ui_readiness[0].first_minute_gate,
      rts_campaign_outcome_ui_readiness_objective_victory_gate: $rts_campaign_outcome_ui_readiness[0].objective_victory_gate,
      rts_campaign_outcome_ui_readiness_base_assault_gate: $rts_campaign_outcome_ui_readiness[0].base_assault_gate,
      rts_campaign_outcome_ui_readiness_battle_aftermath_gate: $rts_campaign_outcome_ui_readiness[0].battle_aftermath_gate,
      rts_campaign_outcome_ui_readiness_open_world_return_gate: $rts_campaign_outcome_ui_readiness[0].open_world_return_gate,
      rts_campaign_outcome_ui_readiness_player_first_screen_gate: $rts_campaign_outcome_ui_readiness[0].player_first_campaign_outcome_screen_gate,
      rts_campaign_outcome_ui_readiness_gate: $rts_campaign_outcome_ui_readiness[0].campaign_outcome_ui_readiness_gate,
      rts_campaign_ui_continuity_handoff_green_gate: $rts_campaign_ui_continuity[0].handoff_green_gate,
      rts_campaign_ui_continuity_preview_resolution_gate: $rts_campaign_ui_continuity[0].preview_resolution_gate,
      rts_campaign_ui_continuity_live_input_gate: $rts_campaign_ui_continuity[0].live_input_gate,
      rts_campaign_ui_continuity_milestone_gate: $rts_campaign_ui_continuity[0].milestone_gate,
      rts_campaign_ui_continuity_map_ui_state_gate: $rts_campaign_ui_continuity[0].map_ui_state_gate,
      rts_campaign_ui_continuity_restored_ui_state_gate: $rts_campaign_ui_continuity[0].restored_ui_state_gate,
      rts_campaign_ui_continuity_persistence_gate: $rts_campaign_ui_continuity[0].persistence_gate,
      rts_campaign_ui_continuity_render_readability_gate: $rts_campaign_ui_continuity[0].render_readability_gate,
      rts_campaign_ui_continuity_native_client_boundary_gate: $rts_campaign_ui_continuity[0].native_client_boundary_gate,
      rts_in_match_hud_state_replication_selection_gate: $rts_in_match_hud_state_replication[0].selection_gate,
      rts_in_match_hud_state_replication_command_gate: $rts_in_match_hud_state_replication[0].command_gate,
      rts_in_match_hud_state_replication_resource_gate: $rts_in_match_hud_state_replication[0].resource_gate,
      rts_in_match_hud_state_replication_production_gate: $rts_in_match_hud_state_replication[0].production_gate,
      rts_in_match_hud_state_replication_ability_gate: $rts_in_match_hud_state_replication[0].ability_gate,
      rts_in_match_hud_state_replication_combat_alert_gate: $rts_in_match_hud_state_replication[0].combat_alert_gate,
      rts_in_match_hud_state_replication_minimap_objective_gate: $rts_in_match_hud_state_replication[0].minimap_objective_gate,
      rts_in_match_hud_state_replication_native_client_boundary_gate: $rts_in_match_hud_state_replication[0].native_client_boundary_gate,
      rts_in_match_hud_state_replication_player_first_screen_gate: $rts_in_match_hud_state_replication[0].player_first_in_match_hud_screen_gate,
      rts_in_match_hud_state_replication_gate: $rts_in_match_hud_state_replication[0].in_match_hud_state_replication_gate,
      rts_session_state_continuity_shell_meta_gate: $rts_session_state_continuity[0].shell_meta_gate,
      rts_session_state_continuity_session_slot_confirm_gate: $rts_session_state_continuity[0].session_slot_confirm_gate,
      rts_session_state_continuity_session_load_resume_gate: $rts_session_state_continuity[0].session_load_resume_gate,
      rts_session_state_continuity_session_recovery_gate: $rts_session_state_continuity[0].session_recovery_gate,
      rts_session_state_continuity_match_setup_gate: $rts_session_state_continuity[0].match_setup_gate,
      rts_session_state_continuity_hud_restore_gate: $rts_session_state_continuity[0].hud_restore_gate,
      rts_session_state_continuity_campaign_outcome_gate: $rts_session_state_continuity[0].campaign_outcome_gate,
      rts_session_state_continuity_campaign_continuity_gate: $rts_session_state_continuity[0].campaign_continuity_gate,
      rts_session_state_continuity_chain_gate: $rts_session_state_continuity[0].state_continuity_chain_gate,
      rts_session_state_continuity_native_client_boundary_gate: $rts_session_state_continuity[0].native_client_boundary_gate,
      rts_session_state_continuity_player_first_session_resume_screen_gate: $rts_session_state_continuity[0].player_first_session_resume_screen_gate,
      rts_session_state_continuity_gate: $rts_session_state_continuity[0].session_state_continuity_gate,
      rts_continuous_player_flow_title_account_gate: $rts_continuous_player_flow[0].title_account_gate,
      rts_continuous_player_flow_match_setup_gate: $rts_continuous_player_flow[0].match_setup_gate,
      rts_continuous_player_flow_in_match_hud_gate: $rts_continuous_player_flow[0].in_match_hud_gate,
      rts_continuous_player_flow_command_feedback_gate: $rts_continuous_player_flow[0].command_feedback_gate,
      rts_continuous_player_flow_save_resume_gate: $rts_continuous_player_flow[0].save_resume_gate,
      rts_continuous_player_flow_outcome_open_world_gate: $rts_continuous_player_flow[0].outcome_open_world_gate,
      rts_continuous_player_flow_chain_gate: $rts_continuous_player_flow[0].continuous_player_flow_chain_gate,
      rts_continuous_player_flow_player_first_continuous_flow_screen_gate: $rts_continuous_player_flow[0].player_first_continuous_flow_screen_gate,
      rts_continuous_player_flow_native_client_boundary_gate: $rts_continuous_player_flow[0].native_client_boundary_gate,
      rts_continuous_player_flow_gate: $rts_continuous_player_flow[0].continuous_player_flow_gate,
      rts_continuous_player_flow_rts_evidence_review_gate: $rts_continuous_player_flow[0].rts_evidence_continuous_player_flow_review_gate,
      rts_live_session_playthrough_title_account_gate: $rts_live_session_playthrough[0].title_account_gate,
      rts_live_session_playthrough_match_setup_gate: $rts_live_session_playthrough[0].match_setup_gate,
      rts_live_session_playthrough_in_match_hud_gate: $rts_live_session_playthrough[0].in_match_hud_gate,
      rts_live_session_playthrough_command_feedback_gate: $rts_live_session_playthrough[0].command_feedback_gate,
      rts_live_session_playthrough_save_resume_gate: $rts_live_session_playthrough[0].save_resume_gate,
      rts_live_session_playthrough_outcome_open_world_gate: $rts_live_session_playthrough[0].outcome_open_world_gate,
      rts_live_session_playthrough_same_process_trace_gate: $rts_live_session_playthrough[0].same_process_trace_gate,
      rts_live_session_playthrough_player_first_live_session_screen_gate: $rts_live_session_playthrough[0].player_first_live_session_screen_gate,
      rts_live_session_playthrough_runtime_screen_gate: $rts_live_session_playthrough[0].runtime_screen_gate,
      rts_live_session_playthrough_native_client_boundary_gate: $rts_live_session_playthrough[0].native_client_boundary_gate,
      rts_live_session_playthrough_rts_evidence_review_gate: $rts_live_session_playthrough[0].rts_evidence_live_session_playthrough_review_gate,
      rts_live_session_playthrough_gate: $rts_live_session_playthrough[0].live_session_playthrough_gate,
      rts_full_game_visual_ui_replication_source_contract_gate: $rts_full_game_visual_ui_replication[0].source_contract_gate,
      rts_full_game_visual_ui_replication_source_green_gate: $rts_full_game_visual_ui_replication[0].source_green_gate,
      rts_full_game_visual_ui_replication_runtime_screen_chain_gate: $rts_full_game_visual_ui_replication[0].runtime_screen_chain_gate,
      rts_full_game_visual_ui_replication_runtime_screen_gate: $rts_full_game_visual_ui_replication[0].runtime_screen_gate,
      rts_full_game_visual_ui_replication_player_flow_gate: $rts_full_game_visual_ui_replication[0].player_flow_gate,
      rts_full_game_visual_ui_replication_coverage_surface_gate: $rts_full_game_visual_ui_replication[0].coverage_surface_gate,
      rts_full_game_visual_ui_replication_preview_gate: $rts_full_game_visual_ui_replication[0].preview_gate,
      rts_full_game_visual_ui_replication_player_first_tactical_composition_gate: $rts_full_game_visual_ui_replication[0].player_first_tactical_composition_gate,
      rts_full_game_visual_ui_replication_command_grid_readability_gate: $rts_full_game_visual_ui_replication[0].full_game_command_grid_readability_gate,
      rts_full_game_visual_ui_replication_player_first_screen_gate: $rts_full_game_visual_ui_replication[0].player_first_full_game_visual_ui_screen_gate,
      rts_full_game_visual_ui_replication_no_copy_boundary_gate: $rts_full_game_visual_ui_replication[0].no_copy_boundary_gate,
      rts_full_game_visual_ui_replication_rts_evidence_review_gate: $rts_full_game_visual_ui_replication[0].rts_evidence_full_game_visual_ui_replication_review_gate,
      rts_full_game_visual_ui_replication_gate: $rts_full_game_visual_ui_replication[0].full_game_visual_ui_replication_gate,
      rts_openra_screen_for_screen_ui_replication_source_contract_gate: $rts_openra_screen_for_screen_ui_replication[0].source_contract_gate,
      rts_openra_screen_for_screen_ui_replication_source_green_gate: $rts_openra_screen_for_screen_ui_replication[0].source_green_gate,
      rts_openra_screen_for_screen_ui_replication_runtime_vocabulary_gate: $rts_openra_screen_for_screen_ui_replication[0].openra_runtime_vocabulary_gate,
      rts_openra_screen_for_screen_ui_replication_widget_root_reference_gate: $rts_openra_screen_for_screen_ui_replication[0].widget_root_reference_gate,
      rts_openra_screen_for_screen_ui_replication_screen_set_gate: $rts_openra_screen_for_screen_ui_replication[0].screen_set_gate,
      rts_openra_screen_for_screen_ui_replication_source_screen_chain_gate: $rts_openra_screen_for_screen_ui_replication[0].source_screen_chain_gate,
      rts_openra_screen_for_screen_ui_replication_preview_gate: $rts_openra_screen_for_screen_ui_replication[0].preview_gate,
      rts_openra_screen_for_screen_ui_replication_no_asset_copy_boundary_gate: $rts_openra_screen_for_screen_ui_replication[0].no_asset_copy_boundary_gate,
      rts_openra_screen_for_screen_ui_replication_player_first_ingame_screen_gate: $rts_openra_screen_for_screen_ui_replication[0].player_first_openra_style_ingame_screen_gate,
      rts_openra_screen_for_screen_ui_replication_style_screen_set_gate: $rts_openra_screen_for_screen_ui_replication[0].openra_style_ui_screen_set_replication_gate,
      rts_openra_screen_for_screen_ui_replication_gate: $rts_openra_screen_for_screen_ui_replication[0].openra_screen_for_screen_ui_replication_gate,
      rts_openra_screen_for_screen_ui_replication_rts_evidence_review_gate: $rts_openra_screen_for_screen_ui_replication[0].rts_evidence_openra_style_screen_set_review_gate,
      rts_openra_engine_port_asset_parity_source_contract_gate: $rts_openra_engine_port_asset_parity[0].source_contract_gate,
      rts_openra_engine_port_asset_parity_source_green_gate: $rts_openra_engine_port_asset_parity[0].source_green_gate,
      rts_openra_engine_port_asset_parity_engine_module_gate: $rts_openra_engine_port_asset_parity[0].engine_module_gate,
      rts_openra_engine_port_asset_parity_rules_mod_port_gate: $rts_openra_engine_port_asset_parity[0].rules_mod_port_gate,
      rts_openra_engine_port_asset_parity_chrome_widget_port_gate: $rts_openra_engine_port_asset_parity[0].chrome_widget_port_gate,
      rts_openra_engine_port_asset_parity_asset_loader_port_gate: $rts_openra_engine_port_asset_parity[0].asset_loader_port_gate,
      rts_openra_engine_port_asset_parity_pixel_perfect_gate: $rts_openra_engine_port_asset_parity[0].pixel_perfect_asset_parity_gate,
      rts_openra_engine_port_asset_parity_write_gate: $rts_openra_engine_port_asset_parity[0].write_gate,
      rts_openra_engine_port_asset_parity_no_copy_boundary_gate: $rts_openra_engine_port_asset_parity[0].no_copy_boundary_gate,
      rts_openra_engine_port_asset_parity_gate: $rts_openra_engine_port_asset_parity[0].openra_engine_port_asset_parity_gate,
      rts_command_affordance_live_input_gate: $rts_command_affordance[0].live_command_affordance_input_gate,
      rts_command_affordance_drag_select_gate: $rts_command_affordance[0].drag_select_gate,
      rts_command_affordance_right_click_move_gate: $rts_command_affordance[0].right_click_move_gate,
      rts_command_affordance_attack_cursor_gate: $rts_command_affordance[0].attack_cursor_gate,
      rts_command_affordance_hotkey_ack_gate: $rts_command_affordance[0].hotkey_ack_gate,
      rts_command_affordance_original_art_policy_gate: $rts_command_affordance[0].original_art_policy_gate,
      rts_command_surface_selection_surface_gate: $rts_command_surface[0].selection_surface_gate,
      rts_command_surface_command_grid_surface_gate: $rts_command_surface[0].command_grid_surface_gate,
      rts_command_surface_cooldown_disabled_surface_gate: $rts_command_surface[0].cooldown_disabled_surface_gate,
      rts_command_surface_target_queue_surface_gate: $rts_command_surface[0].target_queue_surface_gate,
      rts_command_surface_surface_stage_gate: $rts_command_surface[0].surface_stage_gate,
      rts_command_surface_scene_renderer_gate: $rts_command_surface[0].scene_renderer_gate,
      rts_command_surface_original_art_policy_gate: $rts_command_surface[0].original_art_policy_gate,
      rts_structure_modeling_foundation_gate: $rts_structure_modeling[0].foundation_gate,
      rts_structure_modeling_scaffold_gate: $rts_structure_modeling[0].scaffold_gate,
      rts_structure_modeling_construction_spark_gate: $rts_structure_modeling[0].construction_spark_gate,
      rts_structure_modeling_production_glow_gate: $rts_structure_modeling[0].production_glow_gate,
      rts_structure_modeling_damage_crack_gate: $rts_structure_modeling[0].damage_crack_gate,
      rts_structure_modeling_repair_beam_gate: $rts_structure_modeling[0].repair_beam_gate,
      rts_structure_modeling_structure_stage_gate: $rts_structure_modeling[0].structure_stage_gate,
      rts_structure_modeling_scene_renderer_gate: $rts_structure_modeling[0].scene_renderer_gate,
      rts_structure_modeling_original_art_policy_gate: $rts_structure_modeling[0].original_art_policy_gate,
      rts_environment_life_tree_sway_gate: $rts_environment_life[0].tree_sway_gate,
      rts_environment_life_torch_flicker_gate: $rts_environment_life[0].torch_flicker_gate,
      rts_environment_life_water_shimmer_gate: $rts_environment_life[0].water_shimmer_gate,
      rts_environment_life_banner_flutter_gate: $rts_environment_life[0].banner_flutter_gate,
      rts_environment_life_resource_glint_gate: $rts_environment_life[0].resource_glint_gate,
      rts_environment_life_ambient_dust_gate: $rts_environment_life[0].ambient_dust_gate,
      rts_environment_life_environment_stage_gate: $rts_environment_life[0].environment_stage_gate,
      rts_environment_life_scene_renderer_gate: $rts_environment_life[0].scene_renderer_gate,
      rts_environment_life_original_art_policy_gate: $rts_environment_life[0].original_art_policy_gate,
      rts_map_model_gap_lane_gate: $rts_map_model_gap[0].lane_gate,
      rts_map_model_gap_resource_gate: $rts_map_model_gap[0].resource_gate,
      rts_map_model_gap_height_gate: $rts_map_model_gap[0].height_gate,
      rts_map_model_gap_choke_gate: $rts_map_model_gap[0].choke_gate,
      rts_map_model_gap_structure_silhouette_gate: $rts_map_model_gap[0].structure_silhouette_gate,
      rts_map_model_gap_unit_role_gate: $rts_map_model_gap[0].unit_role_gate,
      rts_map_model_gap_occlusion_gate: $rts_map_model_gap[0].occlusion_gate,
      rts_map_model_gap_stage_gate: $rts_map_model_gap[0].map_model_stage_gate,
      rts_map_model_gap_map_topology_gate: $rts_map_model_gap[0].map_topology_gate,
      rts_map_model_gap_model_readability_gate: $rts_map_model_gap[0].model_readability_gate,
      rts_map_model_gap_scene_renderer_gate: $rts_map_model_gap[0].scene_renderer_gate,
      rts_map_model_gap_openra_gap_not_closed_gate: $rts_map_model_gap[0].openra_gap_not_closed_gate,
      rts_map_model_gap_original_art_policy_gate: $rts_map_model_gap[0].original_art_policy_gate,
      rts_worker_harvest_animation_approach_gate: $rts_worker_harvest_animation[0].approach_gate,
      rts_worker_harvest_animation_tool_swing_gate: $rts_worker_harvest_animation[0].tool_swing_gate,
      rts_worker_harvest_animation_resource_pop_gate: $rts_worker_harvest_animation[0].resource_pop_gate,
      rts_worker_harvest_animation_carry_load_gate: $rts_worker_harvest_animation[0].carry_load_gate,
      rts_worker_harvest_animation_dropoff_burst_gate: $rts_worker_harvest_animation[0].dropoff_burst_gate,
      rts_worker_harvest_animation_return_path_gate: $rts_worker_harvest_animation[0].return_path_gate,
      rts_worker_harvest_animation_harvest_stage_gate: $rts_worker_harvest_animation[0].harvest_stage_gate,
      rts_worker_harvest_animation_economy_runtime_gate: $rts_worker_harvest_animation[0].economy_runtime_gate,
      rts_worker_harvest_animation_scene_renderer_gate: $rts_worker_harvest_animation[0].scene_renderer_gate,
      rts_worker_harvest_animation_original_art_policy_gate: $rts_worker_harvest_animation[0].original_art_policy_gate,
      rts_production_spawn_animation_queue_pulse_gate: $rts_production_spawn_animation[0].queue_pulse_gate,
      rts_production_spawn_animation_training_tick_gate: $rts_production_spawn_animation[0].training_tick_gate,
      rts_production_spawn_animation_spawn_door_gate: $rts_production_spawn_animation[0].spawn_door_gate,
      rts_production_spawn_animation_rally_flag_gate: $rts_production_spawn_animation[0].rally_flag_gate,
      rts_production_spawn_animation_formation_join_gate: $rts_production_spawn_animation[0].formation_join_gate,
      rts_production_spawn_animation_supply_flash_gate: $rts_production_spawn_animation[0].supply_flash_gate,
      rts_production_spawn_animation_production_stage_gate: $rts_production_spawn_animation[0].production_stage_gate,
      rts_production_spawn_animation_production_runtime_gate: $rts_production_spawn_animation[0].production_runtime_gate,
      rts_production_spawn_animation_scene_renderer_gate: $rts_production_spawn_animation[0].scene_renderer_gate,
      rts_production_spawn_animation_original_art_policy_gate: $rts_production_spawn_animation[0].original_art_policy_gate,
      rts_unit_status_portrait_frame_gate: $rts_unit_status_portrait[0].portrait_frame_gate,
      rts_unit_status_health_bar_gate: $rts_unit_status_portrait[0].health_bar_gate,
      rts_unit_status_mana_bar_gate: $rts_unit_status_portrait[0].mana_bar_gate,
      rts_unit_status_xp_bar_gate: $rts_unit_status_portrait[0].xp_bar_gate,
      rts_unit_status_buff_badge_gate: $rts_unit_status_portrait[0].buff_badge_gate,
      rts_unit_status_role_badge_gate: $rts_unit_status_portrait[0].role_badge_gate,
      rts_unit_status_queue_badge_gate: $rts_unit_status_portrait[0].queue_badge_gate,
      rts_unit_status_status_stage_gate: $rts_unit_status_portrait[0].status_stage_gate,
      rts_unit_status_status_runtime_gate: $rts_unit_status_portrait[0].status_runtime_gate,
      rts_unit_status_scene_renderer_gate: $rts_unit_status_portrait[0].scene_renderer_gate,
      rts_unit_status_original_art_policy_gate: $rts_unit_status_portrait[0].original_art_policy_gate,
      rts_selection_command_feedback_marquee_gate: $rts_selection_command_feedback[0].marquee_gate,
      rts_selection_command_feedback_confirm_gate: $rts_selection_command_feedback[0].confirm_gate,
      rts_selection_command_feedback_rally_gate: $rts_selection_command_feedback[0].rally_gate,
      rts_selection_command_feedback_move_gate: $rts_selection_command_feedback[0].move_gate,
      rts_selection_command_feedback_attack_gate: $rts_selection_command_feedback[0].attack_gate,
      rts_selection_command_feedback_error_gate: $rts_selection_command_feedback[0].error_gate,
      rts_selection_command_feedback_ack_gate: $rts_selection_command_feedback[0].ack_gate,
      rts_selection_command_feedback_feedback_stage_gate: $rts_selection_command_feedback[0].feedback_stage_gate,
      rts_selection_command_feedback_command_runtime_gate: $rts_selection_command_feedback[0].command_runtime_gate,
      rts_selection_command_feedback_scene_renderer_gate: $rts_selection_command_feedback[0].scene_renderer_gate,
      rts_selection_command_feedback_original_art_policy_gate: $rts_selection_command_feedback[0].original_art_policy_gate,
      rts_ability_tooltip_telegraph_tooltip_gate: $rts_ability_tooltip_telegraph[0].tooltip_gate,
      rts_ability_tooltip_telegraph_range_gate: $rts_ability_tooltip_telegraph[0].range_gate,
      rts_ability_tooltip_telegraph_windup_gate: $rts_ability_tooltip_telegraph[0].windup_gate,
      rts_ability_tooltip_telegraph_cooldown_gate: $rts_ability_tooltip_telegraph[0].cooldown_gate,
      rts_ability_tooltip_telegraph_queue_gate: $rts_ability_tooltip_telegraph[0].queue_gate,
      rts_ability_tooltip_telegraph_warning_gate: $rts_ability_tooltip_telegraph[0].warning_gate,
      rts_ability_tooltip_telegraph_telegraph_stage_gate: $rts_ability_tooltip_telegraph[0].telegraph_stage_gate,
      rts_ability_tooltip_telegraph_ability_runtime_gate: $rts_ability_tooltip_telegraph[0].ability_runtime_gate,
      rts_ability_tooltip_telegraph_scene_renderer_gate: $rts_ability_tooltip_telegraph[0].scene_renderer_gate,
      rts_ability_tooltip_telegraph_original_art_policy_gate: $rts_ability_tooltip_telegraph[0].original_art_policy_gate,
      rts_control_group_hotkey_feedback_assign_gate: $rts_control_group_hotkey_feedback[0].assign_gate,
      rts_control_group_hotkey_feedback_recall_gate: $rts_control_group_hotkey_feedback[0].recall_gate,
      rts_control_group_hotkey_feedback_camera_gate: $rts_control_group_hotkey_feedback[0].camera_gate,
      rts_control_group_hotkey_feedback_idle_gate: $rts_control_group_hotkey_feedback[0].idle_gate,
      rts_control_group_hotkey_feedback_production_gate: $rts_control_group_hotkey_feedback[0].production_gate,
      rts_control_group_hotkey_feedback_ability_gate: $rts_control_group_hotkey_feedback[0].ability_gate,
      rts_control_group_hotkey_feedback_hotkey_stage_gate: $rts_control_group_hotkey_feedback[0].hotkey_stage_gate,
      rts_control_group_hotkey_feedback_hotkey_runtime_gate: $rts_control_group_hotkey_feedback[0].hotkey_runtime_gate,
      rts_control_group_hotkey_feedback_scene_renderer_gate: $rts_control_group_hotkey_feedback[0].scene_renderer_gate,
      rts_control_group_hotkey_feedback_original_art_policy_gate: $rts_control_group_hotkey_feedback[0].original_art_policy_gate,
      rts_scrollable_map_keyboard_pan_gate: $rts_scrollable_map[0].keyboard_pan_gate,
      rts_scrollable_map_edge_scroll_gate: $rts_scrollable_map[0].edge_scroll_gate,
      rts_scrollable_map_drag_pan_gate: $rts_scrollable_map[0].drag_pan_gate,
      rts_scrollable_map_wheel_zoom_gate: $rts_scrollable_map[0].wheel_zoom_gate,
      rts_scrollable_map_minimap_jump_gate: $rts_scrollable_map[0].minimap_jump_gate,
      rts_scrollable_map_boundary_clamp_gate: $rts_scrollable_map[0].boundary_clamp_gate,
      rts_scrollable_map_map_layer_projection_gate: $rts_scrollable_map[0].map_layer_projection_gate,
      rts_scrollable_map_hud_fixed_gate: $rts_scrollable_map[0].hud_fixed_gate,
      rts_scrollable_map_scene_renderer_gate: $rts_scrollable_map[0].scene_renderer_gate,
      rts_scrollable_map_original_art_policy_gate: $rts_scrollable_map[0].original_art_policy_gate,
      rts_camera_minimap_sync_viewport_sync_gate: $rts_camera_minimap_sync[0].viewport_sync_gate,
      rts_camera_minimap_sync_fog_reveal_gate: $rts_camera_minimap_sync[0].fog_reveal_gate,
      rts_camera_minimap_sync_selection_follow_gate: $rts_camera_minimap_sync[0].selection_follow_gate,
      rts_camera_minimap_sync_control_group_sync_gate: $rts_camera_minimap_sync[0].control_group_sync_gate,
      rts_camera_minimap_sync_route_projection_gate: $rts_camera_minimap_sync[0].route_projection_gate,
      rts_camera_minimap_sync_zoom_rect_sync_gate: $rts_camera_minimap_sync[0].zoom_rect_sync_gate,
      rts_camera_minimap_sync_minimap_runtime_gate: $rts_camera_minimap_sync[0].minimap_runtime_gate,
      rts_camera_minimap_sync_scene_renderer_gate: $rts_camera_minimap_sync[0].scene_renderer_gate,
      rts_camera_minimap_sync_original_art_policy_gate: $rts_camera_minimap_sync[0].original_art_policy_gate,
      rts_command_queue_path_preview_live_input_gate: $rts_command_queue_path_preview[0].live_input_gate,
      rts_command_queue_path_preview_queue_stack_gate: $rts_command_queue_path_preview[0].queue_stack_gate,
      rts_command_queue_path_preview_shift_waypoint_gate: $rts_command_queue_path_preview[0].shift_waypoint_gate,
      rts_command_queue_path_preview_rally_chain_gate: $rts_command_queue_path_preview[0].rally_chain_gate,
      rts_command_queue_path_preview_attack_focus_gate: $rts_command_queue_path_preview[0].attack_focus_gate,
      rts_command_queue_path_preview_build_reservation_gate: $rts_command_queue_path_preview[0].build_reservation_gate,
      rts_command_queue_path_preview_cancel_repath_gate: $rts_command_queue_path_preview[0].cancel_repath_gate,
      rts_command_queue_path_preview_scene_renderer_gate: $rts_command_queue_path_preview[0].scene_renderer_gate,
      rts_command_queue_path_preview_original_art_policy_gate: $rts_command_queue_path_preview[0].original_art_policy_gate,
      rts_formation_move_preview_live_input_gate: $rts_formation_move_preview[0].live_input_gate,
      rts_formation_move_preview_destination_ghost_gate: $rts_formation_move_preview[0].destination_ghost_gate,
      rts_formation_move_preview_wedge_spacing_gate: $rts_formation_move_preview[0].wedge_spacing_gate,
      rts_formation_move_preview_line_reflow_gate: $rts_formation_move_preview[0].line_reflow_gate,
      rts_formation_move_preview_collision_avoidance_gate: $rts_formation_move_preview[0].collision_avoidance_gate,
      rts_formation_move_preview_split_avoidance_gate: $rts_formation_move_preview[0].split_avoidance_gate,
      rts_formation_move_preview_commit_spacing_gate: $rts_formation_move_preview[0].commit_spacing_gate,
      rts_formation_move_preview_scene_renderer_gate: $rts_formation_move_preview[0].scene_renderer_gate,
      rts_formation_move_preview_original_art_policy_gate: $rts_formation_move_preview[0].original_art_policy_gate,
      rts_formation_move_execution_live_input_gate: $rts_formation_move_execution[0].live_input_gate,
      rts_formation_move_execution_slot_claim_gate: $rts_formation_move_execution[0].slot_claim_gate,
      rts_formation_move_execution_path_reservation_gate: $rts_formation_move_execution[0].path_reservation_gate,
      rts_formation_move_execution_stagger_step_gate: $rts_formation_move_execution[0].stagger_step_gate,
      rts_formation_move_execution_crowd_avoidance_gate: $rts_formation_move_execution[0].crowd_avoidance_gate,
      rts_formation_move_execution_blocked_reroute_gate: $rts_formation_move_execution[0].blocked_reroute_gate,
      rts_formation_move_execution_arrival_lock_gate: $rts_formation_move_execution[0].arrival_lock_gate,
      rts_formation_move_execution_scene_renderer_gate: $rts_formation_move_execution[0].scene_renderer_gate,
      rts_formation_move_execution_original_art_policy_gate: $rts_formation_move_execution[0].original_art_policy_gate,
      rts_local_obstruction_recovery_live_input_gate: $rts_local_obstruction_recovery[0].live_input_gate,
      rts_local_obstruction_recovery_detect_block_gate: $rts_local_obstruction_recovery[0].detect_block_gate,
      rts_local_obstruction_recovery_hold_queue_gate: $rts_local_obstruction_recovery[0].hold_queue_gate,
      rts_local_obstruction_recovery_side_step_gate: $rts_local_obstruction_recovery[0].side_step_gate,
      rts_local_obstruction_recovery_gap_claim_gate: $rts_local_obstruction_recovery[0].gap_claim_gate,
      rts_local_obstruction_recovery_flow_resume_gate: $rts_local_obstruction_recovery[0].flow_resume_gate,
      rts_local_obstruction_recovery_scene_renderer_gate: $rts_local_obstruction_recovery[0].scene_renderer_gate,
      rts_local_obstruction_recovery_original_art_policy_gate: $rts_local_obstruction_recovery[0].original_art_policy_gate,
      rts_action_cadence_windup_gate: $rts_action_cadence[0].windup_gate,
      rts_action_cadence_strike_gate: $rts_action_cadence[0].strike_gate,
      rts_action_cadence_recovery_gate: $rts_action_cadence[0].recovery_gate,
      rts_action_cadence_carry_bob_gate: $rts_action_cadence[0].carry_bob_gate,
      rts_action_cadence_idle_breath_gate: $rts_action_cadence[0].idle_breath_gate,
      rts_action_cadence_shadow_smear_gate: $rts_action_cadence[0].shadow_smear_gate,
      rts_action_cadence_scene_renderer_gate: $rts_action_cadence[0].scene_renderer_gate,
      rts_action_cadence_event_gate: $rts_action_cadence[0].event_gate,
      rts_action_cadence_original_art_policy_gate: $rts_action_cadence[0].original_art_policy_gate,
      rts_unit_model_depth_rim_gate: $rts_unit_model_depth[0].rim_gate,
      rts_unit_model_depth_armor_gate: $rts_unit_model_depth[0].armor_gate,
      rts_unit_model_depth_role_prop_gate: $rts_unit_model_depth[0].role_prop_gate,
      rts_unit_model_depth_face_shade_gate: $rts_unit_model_depth[0].face_shade_gate,
      rts_unit_model_depth_ground_contact_gate: $rts_unit_model_depth[0].ground_contact_gate,
      rts_unit_model_depth_layer_shadow_gate: $rts_unit_model_depth[0].layer_shadow_gate,
      rts_unit_model_depth_scene_renderer_gate: $rts_unit_model_depth[0].scene_renderer_gate,
      rts_unit_model_depth_role_coverage_gate: $rts_unit_model_depth[0].role_coverage_gate,
      rts_unit_model_depth_original_art_policy_gate: $rts_unit_model_depth[0].original_art_policy_gate,
      rts_action_sequence_idle_gate: $rts_action_sequence[0].idle_gate,
      rts_action_sequence_windup_gate: $rts_action_sequence[0].windup_gate,
      rts_action_sequence_strike_gate: $rts_action_sequence[0].strike_gate,
      rts_action_sequence_recovery_gate: $rts_action_sequence[0].recovery_gate,
      rts_action_sequence_carry_up_gate: $rts_action_sequence[0].carry_up_gate,
      rts_action_sequence_carry_down_gate: $rts_action_sequence[0].carry_down_gate,
      rts_action_sequence_frame_ghost_gate: $rts_action_sequence[0].frame_ghost_gate,
      rts_action_sequence_sequence_phase_gate: $rts_action_sequence[0].sequence_phase_gate,
      rts_action_sequence_scene_renderer_gate: $rts_action_sequence[0].scene_renderer_gate,
      rts_action_sequence_original_art_policy_gate: $rts_action_sequence[0].original_art_policy_gate,
      rts_npc_behavior_patrol_gate: $rts_npc_behavior[0].patrol_gate,
      rts_npc_behavior_engage_gate: $rts_npc_behavior[0].engage_gate,
      rts_npc_behavior_work_gate: $rts_npc_behavior[0].work_gate,
      rts_npc_behavior_carry_gate: $rts_npc_behavior[0].carry_gate,
      rts_npc_behavior_stalk_gate: $rts_npc_behavior[0].stalk_gate,
      rts_npc_behavior_retreat_gate: $rts_npc_behavior[0].retreat_gate,
      rts_npc_behavior_route_gate: $rts_npc_behavior[0].route_gate,
      rts_npc_behavior_behavior_stage_gate: $rts_npc_behavior[0].behavior_stage_gate,
      rts_npc_behavior_scene_renderer_gate: $rts_npc_behavior[0].scene_renderer_gate,
      rts_npc_behavior_original_art_policy_gate: $rts_npc_behavior[0].original_art_policy_gate,
      rts_combat_impact_hit_gate: $rts_combat_impact[0].hit_gate,
      rts_combat_impact_stagger_gate: $rts_combat_impact[0].stagger_gate,
      rts_combat_impact_damage_gate: $rts_combat_impact[0].damage_gate,
      rts_combat_impact_death_gate: $rts_combat_impact[0].death_gate,
      rts_combat_impact_corpse_gate: $rts_combat_impact[0].corpse_gate,
      rts_combat_impact_dissolve_gate: $rts_combat_impact[0].dissolve_gate,
      rts_combat_impact_victory_gate: $rts_combat_impact[0].victory_gate,
      rts_combat_impact_impact_stage_gate: $rts_combat_impact[0].impact_stage_gate,
      rts_combat_impact_scene_renderer_gate: $rts_combat_impact[0].scene_renderer_gate,
      rts_combat_impact_original_art_policy_gate: $rts_combat_impact[0].original_art_policy_gate,
      rts_locomotion_blend_path_gate: $rts_locomotion_blend[0].path_gate,
      rts_locomotion_blend_left_step_gate: $rts_locomotion_blend[0].left_step_gate,
      rts_locomotion_blend_right_step_gate: $rts_locomotion_blend[0].right_step_gate,
      rts_locomotion_blend_turn_gate: $rts_locomotion_blend[0].turn_gate,
      rts_locomotion_blend_slide_gate: $rts_locomotion_blend[0].slide_gate,
      rts_locomotion_blend_brake_gate: $rts_locomotion_blend[0].brake_gate,
      rts_locomotion_blend_locomotion_stage_gate: $rts_locomotion_blend[0].locomotion_stage_gate,
      rts_locomotion_blend_scene_renderer_gate: $rts_locomotion_blend[0].scene_renderer_gate,
      rts_locomotion_blend_original_art_policy_gate: $rts_locomotion_blend[0].original_art_policy_gate,
      rts_npc_transition_alert_gate: $rts_npc_transition[0].alert_gate,
      rts_npc_transition_engage_gate: $rts_npc_transition[0].engage_gate,
      rts_npc_transition_pickup_gate: $rts_npc_transition[0].pickup_gate,
      rts_npc_transition_pounce_gate: $rts_npc_transition[0].pounce_gate,
      rts_npc_transition_recover_gate: $rts_npc_transition[0].recover_gate,
      rts_npc_transition_resume_gate: $rts_npc_transition[0].resume_gate,
      rts_npc_transition_transition_stage_gate: $rts_npc_transition[0].transition_stage_gate,
      rts_npc_transition_scene_renderer_gate: $rts_npc_transition[0].scene_renderer_gate,
      rts_npc_transition_original_art_policy_gate: $rts_npc_transition[0].original_art_policy_gate,
      rts_depth_readability_foreground_gate: $rts_depth_readability[0].foreground_gate,
      rts_depth_readability_behind_gate: $rts_depth_readability[0].behind_gate,
      rts_depth_readability_building_mask_gate: $rts_depth_readability[0].building_mask_gate,
      rts_depth_readability_target_priority_gate: $rts_depth_readability[0].target_priority_gate,
      rts_depth_readability_path_occlusion_gate: $rts_depth_readability[0].path_occlusion_gate,
      rts_depth_readability_cutaway_gate: $rts_depth_readability[0].cutaway_gate,
      rts_depth_readability_depth_stage_gate: $rts_depth_readability[0].depth_stage_gate,
      rts_depth_readability_scene_renderer_gate: $rts_depth_readability[0].scene_renderer_gate,
      rts_depth_readability_original_art_policy_gate: $rts_depth_readability[0].original_art_policy_gate,
      rts_combat_readability_pressure_player_first_screen_gate: $rts_combat_readability_pressure_readiness[0].player_first_combat_pressure_screen_gate,
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
      classic_rts_autonomous_bot_skirmish: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-autonomous-bot-skirmish.json",
      classic_rts_autonomous_bot_skirmish_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-autonomous-bot-skirmish.ppm",
      classic_rts_organic_terminal_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-organic-terminal-gap.json",
      classic_rts_organic_terminal_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-organic-terminal-gap.ppm",
      classic_rts_terminal_observation_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-terminal-observation-gap.json",
      classic_rts_terminal_observation_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-terminal-observation-gap.ppm",
      classic_rts_replay_metrics_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-replay-metrics-gap.json",
      classic_rts_replay_metrics_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-replay-metrics-gap.ppm",
      classic_rts_endurance_skirmish_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-endurance-skirmish-gap.json",
      classic_rts_endurance_skirmish_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-endurance-skirmish-gap.ppm",
      classic_rts_bot_decision_state_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-decision-state-gap.json",
      classic_rts_bot_decision_state_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-decision-state-gap.ppm",
      classic_rts_bot_adaptive_build_order_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-adaptive-build-order-gap.json",
      classic_rts_bot_adaptive_build_order_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-adaptive-build-order-gap.ppm",
      classic_rts_bot_tactical_micro_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tactical-micro-gap.json",
      classic_rts_bot_tactical_micro_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tactical-micro-gap.ppm",
      classic_rts_bot_map_intel_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-map-intel-gap.json",
      classic_rts_bot_map_intel_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-map-intel-gap.ppm",
      classic_rts_bot_macro_economy_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-macro-economy-gap.json",
      classic_rts_bot_macro_economy_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-macro-economy-gap.ppm",
      classic_rts_bot_harassment_defense_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-harassment-defense-gap.json",
      classic_rts_bot_harassment_defense_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-harassment-defense-gap.ppm",
      classic_rts_bot_multi_front_pressure_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-multi-front-pressure-gap.json",
      classic_rts_bot_multi_front_pressure_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-multi-front-pressure-gap.ppm",
      classic_rts_bot_expansion_control_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-expansion-control-gap.json",
      classic_rts_bot_expansion_control_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-expansion-control-gap.ppm",
      classic_rts_bot_tech_transition_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tech-transition-gap.json",
      classic_rts_bot_tech_transition_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tech-transition-gap.ppm",
      classic_rts_bot_army_composition_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-army-composition-gap.json",
      classic_rts_bot_army_composition_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-army-composition-gap.ppm",
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
      classic_rts_visual_fidelity: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-visual-fidelity.json",
      classic_rts_visual_fidelity_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-visual-fidelity.ppm",
      classic_rts_command_affordance: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-affordance.json",
      classic_rts_command_affordance_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-affordance.ppm",
      classic_rts_command_surface: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-surface.json",
      classic_rts_command_surface_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-surface.ppm",
      classic_rts_structure_modeling: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-structure-modeling.json",
      classic_rts_structure_modeling_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-structure-modeling.ppm",
      classic_rts_environment_life: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-environment-life.json",
      classic_rts_environment_life_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-environment-life.ppm",
      classic_rts_map_model_gap: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-model-gap.json",
      classic_rts_map_model_gap_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-model-gap.ppm",
      classic_rts_worker_harvest_animation: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-worker-harvest-animation.json",
      classic_rts_worker_harvest_animation_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-worker-harvest-animation.ppm",
      classic_rts_production_spawn_animation: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-spawn-animation.json",
      classic_rts_production_spawn_animation_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-spawn-animation.ppm",
      classic_rts_unit_status_portrait: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-status-portrait.json",
      classic_rts_unit_status_portrait_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-status-portrait.ppm",
      classic_rts_selection_command_feedback: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-command-feedback.json",
      classic_rts_selection_command_feedback_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-command-feedback.ppm",
      classic_rts_ability_tooltip_telegraph: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ability-tooltip-telegraph.json",
      classic_rts_ability_tooltip_telegraph_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ability-tooltip-telegraph.ppm",
      classic_rts_control_group_hotkey_feedback: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-hotkey-feedback.json",
      classic_rts_control_group_hotkey_feedback_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-hotkey-feedback.ppm",
      classic_rts_scrollable_map: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-scrollable-map.json",
      classic_rts_scrollable_map_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-scrollable-map.ppm",
      classic_rts_camera_minimap_sync: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-camera-minimap-sync.json",
      classic_rts_camera_minimap_sync_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-camera-minimap-sync.ppm",
      classic_rts_command_queue_path_preview: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-queue-path-preview.json",
      classic_rts_command_queue_path_preview_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-queue-path-preview.ppm",
      classic_rts_formation_move_preview: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-preview.json",
      classic_rts_formation_move_preview_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-preview.ppm",
      classic_rts_formation_move_execution: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-execution.json",
      classic_rts_formation_move_execution_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-execution.ppm",
      classic_rts_local_obstruction_recovery: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-local-obstruction-recovery.json",
      classic_rts_local_obstruction_recovery_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-local-obstruction-recovery.ppm",
      classic_rts_action_cadence: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-cadence.json",
      classic_rts_action_cadence_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-cadence.ppm",
      classic_rts_unit_model_depth: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-model-depth.json",
      classic_rts_unit_model_depth_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-model-depth.ppm",
      classic_rts_action_sequence: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-sequence.json",
      classic_rts_action_sequence_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-sequence.ppm",
      classic_rts_npc_behavior: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-behavior.json",
      classic_rts_npc_behavior_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-behavior.ppm",
      classic_rts_combat_impact: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-impact.json",
      classic_rts_combat_impact_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-impact.ppm",
      classic_rts_locomotion_blend: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-locomotion-blend.json",
      classic_rts_locomotion_blend_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-locomotion-blend.ppm",
      classic_rts_npc_transition: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-transition.json",
      classic_rts_npc_transition_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-transition.ppm",
      classic_rts_depth_readability: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-depth-readability.json",
      classic_rts_depth_readability_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-depth-readability.ppm",
      classic_rts_first_minute_readiness: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-minute-readiness.json",
      classic_rts_first_minute_readiness_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-minute-readiness.ppm",
      classic_rts_map_ui_modeling_readiness: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-ui-modeling-readiness.json",
      classic_rts_map_ui_modeling_readiness_dir: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-ui-modeling-readiness/",
      classic_rts_production_art_replication: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-art-replication.json",
      classic_rts_production_art_replication_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-art-replication.ppm",
      classic_rts_production_asset_atlas: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-asset-atlas.json",
      classic_rts_production_asset_atlas_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-asset-atlas.ppm",
      classic_rts_production_ui_skin: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-ui-skin.json",
      classic_rts_production_ui_skin_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-ui-skin.ppm",
      classic_rts_production_interaction_polish: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-interaction-polish.json",
      classic_rts_production_interaction_polish_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-interaction-polish.ppm",
      classic_rts_full_screen_ui_replication: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-screen-ui-replication.json",
      classic_rts_full_screen_ui_replication_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-screen-ui-replication.ppm",
      classic_rts_shell_meta_ui_replication: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-shell-meta-ui-replication.json",
      classic_rts_shell_meta_ui_replication_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-shell-meta-ui-replication.ppm",
      classic_rts_match_setup_ui_replication: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-match-setup-ui-replication.json",
      classic_rts_match_setup_ui_replication_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-match-setup-ui-replication.ppm",
      classic_rts_campaign_outcome_ui_readiness: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-outcome-ui-readiness.json",
      classic_rts_campaign_outcome_ui_readiness_dir: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-outcome-ui-readiness/",
      classic_rts_campaign_ui_continuity: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-ui-continuity.json",
      classic_rts_campaign_ui_continuity_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-ui-continuity.ppm",
      classic_rts_in_match_hud_state_replication: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-in-match-hud-state-replication.json",
      classic_rts_in_match_hud_state_replication_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-in-match-hud-state-replication.ppm",
      classic_rts_session_state_continuity: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-session-state-continuity.json",
      classic_rts_session_state_continuity_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-session-state-continuity.ppm",
      classic_rts_continuous_player_flow: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-continuous-player-flow.json",
      classic_rts_continuous_player_flow_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-continuous-player-flow.ppm",
      classic_rts_live_session_playthrough: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-session-playthrough.json",
      classic_rts_live_session_playthrough_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-session-playthrough.ppm",
      classic_rts_live_session_playthrough_trace: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-session-playthrough.trace.json",
      classic_rts_full_game_visual_ui_replication: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-game-visual-ui-replication.json",
      classic_rts_full_game_visual_ui_replication_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-game-visual-ui-replication.ppm",
      classic_rts_openra_screen_for_screen_ui_replication: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-screen-for-screen-ui-replication.json",
      classic_rts_openra_screen_for_screen_ui_replication_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-screen-for-screen-ui-replication.ppm",
      classic_rts_openra_engine_port_asset_parity: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-engine-port-asset-parity.json",
      classic_rts_openra_engine_port_asset_parity_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-engine-port-asset-parity.ppm",
      classic_rts_combat_readability_pressure_readiness: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-readability-pressure-readiness.json",
      classic_rts_combat_readability_pressure_readiness_dir: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-readability-pressure-readiness/",
      classic_rts_combat_readability_pressure_screen: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-readability-pressure-readiness/combat-pressure-screen.ppm",
      classic_rts_playtest_observability_readiness: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-playtest-observability-readiness.json",
      classic_rts_playtest_observability_readiness_dir: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-playtest-observability-readiness/",
      playtest_runner_status: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json",
      playtest_launcher: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-launcher.json"
    },
    internal_classic_playtest_readiness_claimed: true,
    external_evidence_ignored_for_current_playtest_pass: true,
    android_s5_real_device_claimed: false,
    public_launch_ready: false,
    production_ready_ui_claimed: false,
    screen_for_screen_openra_ui_claimed: false,
    openra_engine_port_claimed: false,
    warcraft_iii_asset_copied: false,
    openra_asset_copied: false,
    third_party_asset_copied: false,
    source_of_truth: "Classic playtest readiness summarizes low-spec trnm-world-bevy evidence only; it does not claim CEX runtime ownership, production-ready UI, Android S5 real-device readiness, public launch readiness, OpenRA screen-for-screen UI, OpenRA engine port, or copied third-party assets."
  }
# END_PLAYTEST_READINESS_SUMMARY_FILTER
PLAYTEST_READINESS_SUMMARY_FILTER_BLOCK

run_validation_filter_in_chunks "$VALIDATION_FILTER" "$SUMMARY" "$VALIDATION_CHUNK_DIR"

: <<'PLAYTEST_READINESS_VALIDATION_FILTER_BLOCK'
# BEGIN_PLAYTEST_READINESS_VALIDATION_FILTER
  .contract_version == "trillionnium_world_bevy_classic_playtest_readiness_v1"
  and .status == "classic_playtest_readiness_green"
  and .green == true
  and .check_count == (.checks | length)
  and .passed_check_count == ([.checks[]] | map(select(. == true)) | length)
  and .failed_check_count == ([.checks[]] | map(select(. != true)) | length)
  and .failed_check_count == 0
  and .artifact_count == (.artifacts | length)
  and .gate_count == (.gates | length)
  and .true_gate_count == ([.gates[]] | map(select(. == true)) | length)
  and .false_boundary_gate_count == ([.gates | to_entries[] | select((.key == "cex_runtime_player_client_allowed" or .key == "wgpu_required") and .value == false)] | length)
  and .passed_gate_count == (.true_gate_count + .false_boundary_gate_count)
  and .failed_gate_count == (.gate_count - .passed_gate_count)
  and .failed_gate_count == 0
  and .internal_classic_playtest_readiness_claimed == true
  and .external_evidence_ignored_for_current_playtest_pass == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
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
  and .checks.classic_rts_autonomous_bot_skirmish_green == true
  and .checks.classic_rts_organic_terminal_gap_green == true
  and .checks.classic_rts_terminal_observation_gap_green == true
  and .checks.classic_rts_replay_metrics_gap_green == true
  and .checks.classic_rts_endurance_skirmish_gap_green == true
  and .checks.classic_rts_bot_decision_state_gap_green == true
  and .checks.classic_rts_bot_adaptive_build_order_gap_green == true
  and .checks.classic_rts_bot_tactical_micro_gap_green == true
  and .checks.classic_rts_bot_map_intel_gap_green == true
  and .checks.classic_rts_bot_macro_economy_gap_green == true
  and .checks.classic_rts_bot_harassment_defense_gap_green == true
  and .checks.classic_rts_bot_multi_front_pressure_gap_green == true
  and .checks.classic_rts_bot_expansion_control_gap_green == true
  and .checks.classic_rts_bot_tech_transition_gap_green == true
  and .checks.classic_rts_bot_army_composition_gap_green == true
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
  and .checks.classic_rts_visual_fidelity_green == true
  and .checks.classic_rts_command_affordance_green == true
  and .checks.classic_rts_command_surface_green == true
  and .checks.classic_rts_structure_modeling_green == true
  and .checks.classic_rts_environment_life_green == true
  and .checks.classic_rts_map_model_gap_green == true
  and .checks.classic_rts_worker_harvest_animation_green == true
  and .checks.classic_rts_production_spawn_animation_green == true
  and .checks.classic_rts_unit_status_portrait_green == true
  and .checks.classic_rts_selection_command_feedback_green == true
  and .checks.classic_rts_ability_tooltip_telegraph_green == true
  and .checks.classic_rts_control_group_hotkey_feedback_green == true
  and .checks.classic_rts_scrollable_map_green == true
  and .checks.classic_rts_camera_minimap_sync_green == true
  and .checks.classic_rts_command_queue_path_preview_green == true
  and .checks.classic_rts_formation_move_preview_green == true
  and .checks.classic_rts_formation_move_execution_green == true
  and .checks.classic_rts_local_obstruction_recovery_green == true
  and .checks.classic_rts_action_cadence_green == true
  and .checks.classic_rts_unit_model_depth_green == true
  and .checks.classic_rts_action_sequence_green == true
  and .checks.classic_rts_npc_behavior_green == true
  and .checks.classic_rts_combat_impact_green == true
  and .checks.classic_rts_locomotion_blend_green == true
  and .checks.classic_rts_npc_transition_green == true
  and .checks.classic_rts_depth_readability_green == true
  and .checks.classic_rts_first_minute_readiness_green == true
  and .checks.classic_rts_map_ui_modeling_readiness_green == true
  and .checks.classic_rts_first_contact_basin_spec_green == true
  and .checks.classic_rts_production_art_replication_green == true
  and .checks.classic_rts_production_asset_atlas_green == true
  and .checks.classic_rts_production_ui_skin_green == true
  and .checks.classic_rts_production_interaction_polish_green == true
  and .checks.classic_rts_full_screen_ui_replication_green == true
  and .checks.classic_rts_shell_meta_ui_replication_green == true
  and .checks.classic_rts_match_setup_ui_replication_green == true
  and .checks.classic_rts_campaign_ui_continuity_green == true
  and .checks.classic_rts_in_match_hud_state_replication_green == true
  and .checks.classic_rts_session_state_continuity_green == true
  and .checks.classic_rts_continuous_player_flow_green == true
  and .checks.classic_rts_live_session_playthrough_green == true
  and .checks.classic_rts_full_game_visual_ui_replication_green == true
  and .checks.classic_rts_openra_screen_for_screen_ui_replication_green == true
  and .checks.classic_rts_openra_engine_port_asset_parity_green == true
  and .headline.rts_production_art_replication_source_contract_count == 3
  and .headline.rts_production_art_replication_required_asset_kind_count == 9
  and .headline.rts_production_art_replication_required_gameplay_layer_count == 8
  and .headline.rts_production_art_replication_required_replacement_slot_count == 5
  and .headline.rts_production_art_replication_gate_count == 6
  and .headline.rts_production_art_replication_passed_gate_count == 6
  and .headline.rts_production_art_replication_failed_gate_count == 0
  and .headline.rts_production_asset_atlas_source_contract_count == 4
  and .headline.rts_production_asset_atlas_source_path_count == 7
  and .headline.rts_production_asset_atlas_family_name_count == 10
  and .headline.rts_production_asset_atlas_binding_replacement_slot_count == 7
  and .headline.rts_production_asset_atlas_binding_runtime_target_count == 4
  and .headline.rts_production_asset_atlas_runtime_material_slot_count == 4
  and .headline.rts_production_asset_atlas_runtime_scene_layer_count == 4
  and .headline.rts_production_asset_atlas_gate_count == 8
  and .headline.rts_production_asset_atlas_passed_gate_count == 8
  and .headline.rts_production_asset_atlas_failed_gate_count == 0
  and .headline.rts_production_asset_atlas_frame_count >= 32
  and .headline.rts_production_asset_atlas_sprite_binding_count >= 32
  and .headline.rts_production_asset_atlas_material_asset_count == 4
  and .headline.rts_production_asset_atlas_family_count == 10
  and .headline.rts_production_asset_atlas_board_pixel_count > 80000
  and .headline.rts_production_asset_atlas_runtime_binding_lane_pixel_count > 8000
  and .headline.rts_production_asset_atlas_uv_rect_pixel_count > 6000
  and .headline.rts_production_ui_skin_surface_count == 8
  and .headline.rts_production_ui_skin_source_contract_count == 7
  and .headline.rts_production_ui_skin_source_path_count == 7
  and .headline.rts_production_ui_skin_runtime_screen_layout_count == 7
  and .headline.rts_production_ui_skin_pixel_count_field_count == 8
  and .headline.rts_production_ui_skin_surface_name_count == 8
  and .headline.rts_production_ui_skin_replacement_slot_count == 8
  and .headline.rts_production_ui_skin_source_surface_count == 8
  and .headline.rts_production_ui_skin_gate_count == 13
  and .headline.rts_production_ui_skin_passed_gate_count == 13
  and .headline.rts_production_ui_skin_failed_gate_count == 0
  and .headline.rts_production_ui_skin_board_pixel_count > 80000
  and .headline.rts_production_ui_skin_hud_chrome_pixel_count > 1000
  and .headline.rts_production_ui_skin_command_grid_pixel_count > 1000
  and .headline.rts_production_ui_skin_minimap_bezel_pixel_count > 1000
  and .headline.rts_production_ui_skin_unit_card_pixel_count > 1000
  and .headline.rts_production_ui_skin_tooltip_pixel_count > 1000
  and .headline.rts_production_ui_skin_feedback_marker_pixel_count > 1000
  and .headline.rts_production_ui_skin_hotkey_strip_pixel_count > 1000
  and .headline.rts_production_ui_skin_status_bar_pixel_count > 1000
  and .headline.rts_production_ui_skin_runtime_screen_mode == "player_runtime_production_hud_skin_screen"
  and .headline.rts_production_ui_skin_player_first_hud_view_non_background > 250000
  and .headline.rts_production_ui_skin_player_first_hud_view_frame_pixel_count > 8000
  and .headline.rts_production_ui_skin_player_first_hud_bottom_chrome_pixel_count > 100000
  and .headline.rts_production_ui_skin_player_first_hud_command_grid_pixel_count > 20000
  and .headline.rts_production_ui_skin_player_first_hud_minimap_bezel_pixel_count > 20000
  and .headline.rts_production_ui_skin_player_first_hud_unit_card_pixel_count > 30000
  and .headline.rts_production_ui_skin_player_first_hud_feedback_lane_pixel_count > 30000
  and .headline.rts_production_ui_skin_player_first_hud_hotkey_status_pixel_count > 25000
  and .headline.rts_production_interaction_polish_surface_count == 6
  and .headline.rts_production_interaction_polish_source_contract_count == 6
  and .headline.rts_production_interaction_polish_source_path_count == 6
  and .headline.rts_production_interaction_polish_runtime_screen_layout_count == 6
  and .headline.rts_production_interaction_polish_pixel_count_field_count == 5
  and .headline.rts_production_interaction_polish_surface_name_count == 6
  and .headline.rts_production_interaction_polish_replacement_slot_count == 6
  and .headline.rts_production_interaction_polish_source_surface_count == 6
  and .headline.rts_production_interaction_polish_gate_count == 12
  and .headline.rts_production_interaction_polish_passed_gate_count == 12
  and .headline.rts_production_interaction_polish_failed_gate_count == 0
  and .headline.rts_production_interaction_polish_board_pixel_count > 80000
  and .headline.rts_production_interaction_polish_drag_select_pixel_count > 1000
  and .headline.rts_production_interaction_polish_right_click_pixel_count > 1000
  and .headline.rts_production_interaction_polish_attack_lock_pixel_count > 1000
  and .headline.rts_production_interaction_polish_build_ghost_pixel_count > 1000
  and .headline.rts_production_interaction_polish_queue_path_pixel_count > 1000
  and .headline.rts_production_interaction_polish_scroll_minimap_pixel_count > 1000
  and .headline.rts_production_interaction_polish_hud_binding_pixel_count > 8000
  and .headline.rts_production_interaction_polish_player_first_view_non_background > 120000
  and .headline.rts_production_interaction_polish_player_first_view_frame_pixel_count > 8000
  and .headline.rts_production_interaction_polish_player_first_status_strip_pixel_count > 10000
  and .headline.rts_production_interaction_polish_player_first_right_rail_pixel_count > 50000
  and .headline.rts_production_interaction_polish_player_first_command_lane_pixel_count > 60000
  and .headline.rts_full_screen_ui_replication_surface_count == 10
  and .headline.rts_full_screen_ui_replication_board_pixel_count > 80000
  and .headline.rts_full_screen_ui_replication_title_campaign_pixel_count > 2000
  and .headline.rts_full_screen_ui_replication_tactical_viewport_pixel_count > 2000
  and .headline.rts_full_screen_ui_replication_player_first_tactical_view_non_background > 80000
  and .headline.rts_full_screen_ui_replication_player_first_tactical_view_frame_pixel_count > 8000
  and .headline.rts_full_screen_ui_replication_player_first_status_strip_pixel_count > 6000
  and .headline.rts_full_screen_ui_replication_map_minimap_pixel_count > 2000
  and .headline.rts_full_screen_ui_replication_build_tech_pixel_count > 2000
  and .headline.rts_shell_meta_ui_replication_surface_count == 12
  and .headline.rts_shell_meta_ui_replication_board_pixel_count > 80000
  and .headline.rts_shell_meta_ui_replication_account_pixel_count > 1000
  and .headline.rts_shell_meta_ui_replication_session_slot_pixel_count > 1000
  and .headline.rts_shell_meta_ui_replication_pause_pixel_count > 1000
  and .headline.rts_shell_meta_ui_replication_input_pixel_count > 1000
  and .headline.rts_shell_meta_ui_replication_player_first_surface_non_background > 450000
  and .headline.rts_shell_meta_ui_replication_player_first_frame_pixel_count > 12000
  and .headline.rts_shell_meta_ui_replication_player_first_account_bar_pixel_count > 30000
  and .headline.rts_shell_meta_ui_replication_player_first_session_panel_pixel_count > 35000
  and .headline.rts_shell_meta_ui_replication_player_first_right_rail_pixel_count > 30000
  and .headline.rts_shell_meta_ui_replication_player_first_handoff_strip_pixel_count > 90000
  and .headline.rts_match_setup_ui_replication_surface_count == 10
  and .headline.rts_match_setup_ui_replication_board_pixel_count > 80000
  and .headline.rts_match_setup_ui_replication_map_select_pixel_count > 2000
  and .headline.rts_match_setup_ui_replication_faction_select_pixel_count > 2000
  and .headline.rts_match_setup_ui_replication_start_ready_pixel_count > 2000
  and .headline.rts_match_setup_ui_replication_player_first_map_non_background > 80000
  and .headline.rts_match_setup_ui_replication_player_first_map_frame_pixel_count > 8000
  and .headline.rts_match_setup_ui_replication_player_first_status_strip_pixel_count > 15000
  and .headline.rts_match_setup_ui_replication_player_first_rules_rail_pixel_count > 50000
  and .headline.rts_match_setup_ui_replication_player_first_ready_strip_pixel_count > 40000
  and .headline.rts_match_setup_ui_replication_map_id == "first_contact_basin"
  and .headline.rts_match_setup_ui_replication_faction_id == "mirror_guard"
  and .headline.rts_first_contact_basin_spec_map_id == "first_contact_basin"
  and .headline.rts_first_contact_basin_spec_actor_count == 39
  and .headline.rts_first_contact_basin_spec_spawn_count == 4
  and .headline.rts_first_contact_basin_spec_contract_field_count == 32
  and .headline.rts_first_contact_basin_spec_guard_object_count == 16
  and .headline.rts_first_contact_basin_spec_guard_gate_count == 16
  and .headline.rts_first_contact_basin_spec_top_level_gate_count == 45
  and .headline.rts_first_contact_basin_spec_map_model_actor_count == 39
  and .headline.rts_first_contact_basin_spec_map_model_player_count == 6
  and .headline.rts_first_contact_basin_spec_map_model_rule_count == 19
  and .headline.rts_first_contact_basin_spec_runtime_command_queue_count == 4
  and .headline.rts_first_contact_basin_spec_runtime_production_queue_count == 3
  and .headline.rts_first_contact_basin_spec_runtime_build_queue_count == 2
  and .headline.rts_first_contact_basin_spec_runtime_visible_tile_count == 64
  and .headline.rts_first_contact_basin_spec_runtime_fogged_tile_count == 6
  and .headline.rts_first_contact_basin_spec_runtime_ability_command_count == 6
  and .headline.rts_first_contact_basin_spec_offline_command_queue_count == 1
  and .headline.rts_first_contact_basin_spec_offline_production_queue_count == 3
  and .headline.rts_first_contact_basin_spec_offline_build_queue_count == 2
  and .headline.rts_first_contact_basin_spec_offline_ability_command_count == 6
  and .headline.rts_first_contact_basin_spec_offline_ready_label_count == 4
  and .headline.rts_first_contact_runtime_review_contract == "trnm_rts_evidence_bevy_runtime_adapter_v1"
  and .headline.rts_first_contact_runtime_review_contract_count == 5
  and (.headline.rts_first_contact_runtime_review_contracts | index("trnm_rts_bevy_runtime_first_contact_player_screen_application_v1") != null)
  and (.headline.rts_first_contact_runtime_review_contracts | index("trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1") != null)
  and (.headline.rts_first_contact_runtime_review_contracts | index("trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1") != null)
  and (.headline.rts_first_contact_runtime_review_contracts | index("trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1") != null)
  and (.headline.rts_first_contact_runtime_review_contracts | index("trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1") != null)
  and (.headline.rts_first_contact_runtime_review_before_command_queue | index("build:trnm.flux.relay") != null)
  and .headline.rts_first_contact_runtime_review_after_command_queue == ["move:8,4"]
  and (.headline.rts_first_contact_runtime_review_ready_state_labels | index("authority:offline_loopback:no_socket") != null)
  and .headline.rts_first_contact_runtime_review_command_stamp_tile == "8,4"
  and (.headline.rts_first_contact_runtime_review_source_of_truth | contains("First Contact player-screen/offline-adapter application, consumption, session-transition, and lobby-ready review samples"))
  and .checks.classic_rts_campaign_outcome_ui_readiness_green == true
  and .headline.rts_campaign_outcome_ui_readiness_runtime_screen_mode == "player_runtime_campaign_outcome_screen"
  and .headline.rts_campaign_outcome_ui_readiness_evidence_board_only == false
  and .headline.rts_campaign_outcome_ui_readiness_first_minute_player_first_non_background > 600000
  and .headline.rts_campaign_outcome_ui_readiness_first_minute_player_first_route_rail > 100000
  and .headline.rts_campaign_outcome_ui_readiness_victory_non_background_pixels > 250000
  and .headline.rts_campaign_outcome_ui_readiness_victory_extraction_pixel_count > 40
  and .headline.rts_campaign_outcome_ui_readiness_base_assault_non_background_pixels > 350000
  and .headline.rts_campaign_outcome_ui_readiness_base_assault_breach_pixel_count > 80
  and .headline.rts_campaign_outcome_ui_readiness_aftermath_player_first_view_non_background > 250000
  and .headline.rts_campaign_outcome_ui_readiness_aftermath_player_first_outcome_panel > 90000
  and .headline.rts_campaign_outcome_ui_readiness_open_world_player_first_view_non_background > 250000
  and .headline.rts_campaign_outcome_ui_readiness_open_world_player_first_route_panel > 90000
  and .headline.rts_campaign_ui_continuity_capture_frame_count == 16
  and .headline.rts_campaign_ui_continuity_non_background_pixels > 500000
  and .headline.rts_campaign_ui_continuity_final_room_id == "league-coliseum"
  and .headline.rts_campaign_ui_continuity_restored_room_id == "league-coliseum"
  and .headline.rts_campaign_ui_continuity_primary_action_label == "COMBAT:attack"
  and .headline.rts_campaign_ui_continuity_runtime_screen_layout_count == 3
  and .headline.rts_campaign_ui_continuity_review_field_count == 26
  and .headline.rts_campaign_ui_continuity_final_action_label_count == 24
  and .headline.rts_campaign_ui_continuity_final_active_task_count == 1
  and .headline.rts_campaign_ui_continuity_restored_action_label_count == 24
  and .headline.rts_campaign_ui_continuity_restored_active_task_count == 1
  and .headline.rts_campaign_ui_continuity_milestone_count == 16
  and .headline.rts_campaign_ui_continuity_pixel_count_field_count == 4
  and .headline.rts_campaign_ui_continuity_gate_count == 12
  and .headline.rts_campaign_ui_continuity_passed_gate_count == 12
  and .headline.rts_campaign_ui_continuity_failed_gate_count == 0
  and .headline.rts_in_match_hud_state_replication_surface_count == 8
  and .headline.rts_in_match_hud_state_replication_runtime_layout_count == 7
  and .headline.rts_in_match_hud_state_replication_hud_pixel_field_count == 10
  and .headline.rts_in_match_hud_state_replication_player_first_pixel_field_count == 7
  and .headline.rts_in_match_hud_state_replication_surface_name_count == 8
  and .headline.rts_in_match_hud_state_replication_gate_count == 12
  and .headline.rts_in_match_hud_state_replication_passed_gate_count == 12
  and .headline.rts_in_match_hud_state_replication_failed_gate_count == 0
  and .headline.rts_in_match_hud_state_replication_non_background_pixels > 100000
  and .headline.rts_in_match_hud_state_replication_command_grid_pixel_count > 40
  and .headline.rts_in_match_hud_state_replication_minimap_pixel_count > 40
  and .headline.rts_in_match_hud_state_replication_player_first_view_non_background > 350000
  and .headline.rts_in_match_hud_state_replication_player_first_view_frame_pixel_count > 8000
  and .headline.rts_in_match_hud_state_replication_player_first_top_status_strip_pixel_count > 45000
  and .headline.rts_in_match_hud_state_replication_player_first_surface_card_pixel_count > 40000
  and .headline.rts_in_match_hud_state_replication_player_first_right_rail_non_background > 90000
  and .headline.rts_in_match_hud_state_replication_player_first_bottom_command_lane_pixel_count > 60000
  and .headline.rts_in_match_hud_state_replication_player_first_control_color_pixel_count > 8000
  and .headline.rts_in_match_hud_state_replication_army_supply_used == 9
  and .headline.rts_in_match_hud_state_replication_army_supply_cap == 18
  and .headline.rts_session_state_continuity_surface_count == 8
  and .headline.rts_session_state_continuity_non_background_pixels > 300000
  and .headline.rts_session_state_continuity_player_first_resume_view_non_background > 250000
  and .headline.rts_session_state_continuity_player_first_resume_view_frame > 8000
  and .headline.rts_session_state_continuity_player_first_resume_status_strip > 10000
  and .headline.rts_session_state_continuity_player_first_resume_stage_rail > 70000
  and .headline.rts_session_state_continuity_slot_a_bytes > 512
  and .headline.rts_session_state_continuity_final_objective_status == "first_playable_loop_complete"
  and .headline.rts_session_state_continuity_open_world_state == "resumed:league-coliseum"
  and .headline.rts_session_state_continuity_restored_room_id == "league-coliseum"
  and .headline.rts_continuous_player_flow_step_count == 6
  and .headline.rts_continuous_player_flow_non_background_pixels > 250000
  and .headline.rts_continuous_player_flow_title_account_pixel_count > 2000
  and .headline.rts_continuous_player_flow_match_setup_pixel_count > 2000
  and .headline.rts_continuous_player_flow_in_match_hud_pixel_count > 2000
  and .headline.rts_continuous_player_flow_command_feedback_pixel_count > 2000
  and .headline.rts_continuous_player_flow_save_load_resume_pixel_count > 2000
  and .headline.rts_continuous_player_flow_outcome_open_world_pixel_count > 2000
  and .headline.rts_continuous_player_flow_player_first_flow_view_non_background > 300000
  and .headline.rts_continuous_player_flow_player_first_flow_view_frame_pixel_count > 8000
  and .headline.rts_continuous_player_flow_player_first_flow_status_strip_pixel_count > 10000
  and .headline.rts_continuous_player_flow_player_first_flow_stage_rail_pixel_count > 50000
  and .headline.rts_continuous_player_flow_final_objective_status == "first_playable_loop_complete"
  and .headline.rts_continuous_player_flow_open_world_state == "resumed:league-coliseum"
  and .headline.rts_continuous_player_flow_restored_room_id == "league-coliseum"
  and .headline.rts_continuous_player_flow_review_contract == "trnm_rts_evidence_continuous_player_flow_review_v1"
  and (.headline.rts_continuous_player_flow_review_source_of_truth | contains("six-step continuous player flow"))
  and .headline.rts_live_session_playthrough_runtime_screen_mode == "player_runtime_live_session_playthrough_screen"
  and .headline.rts_live_session_playthrough_stage_count == 6
  and .headline.rts_live_session_playthrough_top_level_action_count >= 12
  and .headline.rts_live_session_playthrough_accepted_input_count >= 78
  and .headline.rts_live_session_playthrough_campaign_handoff_input_count >= 70
  and .headline.rts_live_session_playthrough_live_command_input_count == 5
  and .headline.rts_live_session_playthrough_slot_a_bytes > 10000
  and .headline.rts_live_session_playthrough_non_background_pixels > 300000
  and .headline.rts_live_session_playthrough_player_first_live_view_non_background > 250000
  and .headline.rts_live_session_playthrough_player_first_live_view_frame_pixel_count > 8000
  and .headline.rts_live_session_playthrough_player_first_live_status_strip_pixel_count > 10000
  and .headline.rts_live_session_playthrough_player_first_live_stage_rail_pixel_count > 25000
  and .headline.rts_live_session_playthrough_final_objective_status == "open_world_after_action_ready"
  and .headline.rts_live_session_playthrough_open_world_state == "resumed:league-coliseum"
  and .headline.rts_live_session_playthrough_resume_room_id == "league-coliseum"
  and .headline.rts_live_session_playthrough_review_contract == "trnm_rts_evidence_live_session_playthrough_review_v1"
  and (.headline.rts_live_session_playthrough_review_source_of_truth | contains("same-process local live session playthrough"))
  and .headline.rts_full_game_visual_ui_replication_runtime_screen_mode == "player_runtime_full_game_visual_ui_screen"
  and .headline.rts_full_game_visual_ui_replication_evidence_board_only == false
  and .headline.rts_full_game_visual_ui_replication_surface_count == 18
  and .headline.rts_full_game_visual_ui_replication_source_contract_count == 14
  and .headline.rts_full_game_visual_ui_replication_source_path_count == 14
  and .headline.rts_full_game_visual_ui_replication_source_review_contract_count == 3
  and .headline.rts_full_game_visual_ui_replication_source_review_gate_count == 3
  and .headline.rts_full_game_visual_ui_replication_source_review_source_count == 3
  and .headline.rts_full_game_visual_ui_replication_source_headline_field_count == 15
  and .headline.rts_full_game_visual_ui_replication_single_screen_runtime_layout_count == 5
  and .headline.rts_full_game_visual_ui_replication_pixel_count_field_count == 14
  and .headline.rts_full_game_visual_ui_replication_coverage_surface_name_count == 18
  and .headline.rts_full_game_visual_ui_replication_command_grid_role_id_count == 12
  and .headline.rts_full_game_visual_ui_replication_command_grid_icon_signature_count == 12
  and .headline.rts_full_game_visual_ui_replication_command_grid_state_sample_count == 12
  and .headline.rts_full_game_visual_ui_replication_gate_count == 13
  and .headline.rts_full_game_visual_ui_replication_passed_gate_count == 13
  and .headline.rts_full_game_visual_ui_replication_failed_gate_count == 0
  and .headline.rts_full_game_visual_ui_replication_non_background_pixels > 900000
  and .headline.rts_full_game_visual_ui_replication_hud_chrome_pixel_count > 120000
  and .headline.rts_full_game_visual_ui_replication_command_pixel_count > 20000
  and .headline.rts_full_game_visual_ui_replication_session_pixel_count > 10000
  and .headline.rts_full_game_visual_ui_replication_outcome_pixel_count > 10000
  and .headline.rts_full_game_visual_ui_replication_player_first_tactical_preview_non_background > 350000
  and .headline.rts_full_game_visual_ui_replication_player_first_tactical_viewport_frame_pixel_count > 8000
  and .headline.rts_full_game_visual_ui_replication_player_first_tactical_status_strip_pixel_count > 10000
  and .headline.rts_full_game_visual_ui_replication_command_grid_unique_icon_signature_count >= 6
  and .headline.rts_full_game_visual_ui_replication_command_grid_active_role == "signal"
  and .headline.rts_full_game_visual_ui_replication_command_grid_active_slot_count >= 1
  and .headline.rts_full_game_visual_ui_replication_command_grid_sent_slot_count >= 3
  and .headline.rts_full_game_visual_ui_replication_live_session_stage_count == 6
  and .headline.rts_full_game_visual_ui_replication_live_session_accepted_input_count >= 78
  and .headline.rts_full_game_visual_ui_replication_final_objective_status == "open_world_after_action_ready"
  and .headline.rts_full_game_visual_ui_replication_open_world_state == "resumed:league-coliseum"
  and .headline.rts_full_game_visual_ui_replication_review_contract == "trnm_rts_evidence_full_game_visual_ui_replication_review_v1"
  and (.headline.rts_full_game_visual_ui_replication_review_source_of_truth | contains("full-game visual/UI replication aggregate"))
  and .headline.rts_openra_screen_for_screen_ui_replication_screen_count == 8
  and .headline.rts_openra_screen_for_screen_ui_replication_surface_count == 8
  and .headline.rts_openra_screen_for_screen_ui_replication_widget_root_count == 4
  and .headline.rts_openra_screen_for_screen_ui_replication_source_contract_count == 8
  and .headline.rts_openra_screen_for_screen_ui_replication_source_headline_field_count == 10
  and .headline.rts_openra_screen_for_screen_ui_replication_screen_layout_count == 8
  and .headline.rts_openra_screen_for_screen_ui_replication_pixel_count_field_count == 10
  and .headline.rts_openra_screen_for_screen_ui_replication_openra_style_ingame_pixel_count_field_count == 5
  and .headline.rts_openra_screen_for_screen_ui_replication_widget_root_name_count == 4
  and .headline.rts_openra_screen_for_screen_ui_replication_reference_source_count == 3
  and .headline.rts_openra_screen_for_screen_ui_replication_surface_name_count == 8
  and .headline.rts_openra_screen_for_screen_ui_replication_gate_count == 13
  and .headline.rts_openra_screen_for_screen_ui_replication_passed_gate_count == 13
  and .headline.rts_openra_screen_for_screen_ui_replication_failed_gate_count == 0
  and .headline.rts_openra_screen_for_screen_ui_replication_runtime_screen_mode == "player_runtime_openra_style_ingame_screen_set"
  and .headline.rts_openra_screen_for_screen_ui_replication_evidence_board_only == false
  and .headline.rts_openra_screen_for_screen_ui_replication_non_background_pixels > 1200000
  and .headline.rts_openra_screen_for_screen_ui_replication_mainmenu_pixel_count > 8000
  and .headline.rts_openra_screen_for_screen_ui_replication_ingame_pixel_count > 8000
  and .headline.rts_openra_screen_for_screen_ui_replication_postgame_pixel_count > 8000
  and .headline.rts_openra_screen_for_screen_ui_replication_player_first_ingame_view_non_background > 70000
  and .headline.rts_openra_screen_for_screen_ui_replication_player_first_ingame_sidebar_non_background > 30000
  and .headline.rts_openra_screen_for_screen_ui_replication_player_first_ingame_command_lane_non_background > 5000
  and .headline.rts_openra_screen_for_screen_ui_replication_style_screen_set_claimed == true
  and .headline.rts_openra_screen_for_screen_ui_replication_claimed == false
  and .headline.rts_openra_screen_for_screen_ui_replication_asset_parity_claimed == false
  and .headline.rts_openra_screen_for_screen_ui_replication_engine_port_claimed == false
  and .headline.rts_openra_screen_for_screen_ui_replication_review_contract == "trnm_rts_evidence_openra_style_screen_set_review_v1"
  and (.headline.rts_openra_screen_for_screen_ui_replication_review_source_of_truth | contains("OpenRA-style screen-set"))
  and .headline.rts_openra_engine_port_asset_parity_module_count >= 10
  and .headline.rts_openra_engine_port_asset_parity_widget_root_count == 4
  and .headline.rts_openra_engine_port_asset_parity_screen_count == 8
  and .headline.rts_openra_engine_port_asset_parity_source_contract_count == 3
  and .headline.rts_openra_engine_port_asset_parity_source_headline_field_count == 8
  and .headline.rts_openra_engine_port_asset_parity_asset_manifest_field_count == 10
  and .headline.rts_openra_engine_port_asset_parity_pixel_parity_field_count == 15
  and .headline.rts_openra_engine_port_asset_parity_manifest_frame_id_count == .headline.rts_openra_engine_port_asset_parity_sample_count
  and .headline.rts_openra_engine_port_asset_parity_sample_report_count == .headline.rts_openra_engine_port_asset_parity_sample_count
  and .headline.rts_openra_engine_port_asset_parity_artifact_path_count == 5
  and .headline.rts_openra_engine_port_asset_parity_pixel_count_field_count == 4
  and .headline.rts_openra_engine_port_asset_parity_gate_count == 10
  and .headline.rts_openra_engine_port_asset_parity_passed_gate_count == 10
  and .headline.rts_openra_engine_port_asset_parity_failed_gate_count == 0
  and .headline.rts_openra_engine_port_asset_parity_sample_count >= 12
  and .headline.rts_openra_engine_port_asset_parity_sha_match_count == .headline.rts_openra_engine_port_asset_parity_sample_count
  and .headline.rts_openra_engine_port_asset_parity_pixel_count >= 3000
  and .headline.rts_openra_engine_port_asset_parity_visible_pixel_count > 1000
  and .headline.rts_openra_engine_port_asset_parity_pixel_mismatch_count == 0
  and .headline.rts_openra_engine_port_asset_parity_reference_render_mismatch_count == 0
  and .headline.rts_openra_engine_port_asset_parity_claimed == false
  and .headline.rts_openra_engine_port_asset_parity_full_engine_claimed == false
  and .headline.rts_openra_engine_port_asset_parity_owned_asset_parity_claimed == true
  and .headline.rts_openra_engine_port_asset_parity_asset_parity_claimed == false
  and .headline.rts_openra_engine_port_asset_parity_westwood_claimed == false
  and .checks.classic_rts_combat_readability_pressure_readiness_green == true
  and .gates.rts_combat_readability_pressure_player_first_screen_gate == true
  and .headline.rts_combat_readability_pressure_player_first_view_non_background > 120000
  and .headline.rts_combat_readability_pressure_player_first_view_frame_pixel_count > 8000
  and .headline.rts_combat_readability_pressure_player_first_status_strip_pixel_count > 20000
  and .headline.rts_combat_readability_pressure_player_first_rail_pixel_count > 70000
  and .headline.rts_combat_readability_pressure_player_first_command_lane_pixel_count > 50000
  and .headline.rts_combat_readability_pressure_player_first_alert_pixel_count > 5000
  and .checks.classic_rts_playtest_observability_readiness_green == true
  and .checks.client_boundary_green == true
  and .checks.playtest_runner_status_green == true
  and .checks.playtest_launcher_green == true
  and .headline.rts_map_model_gap_stage_count == 6
  and .headline.rts_map_model_gap_lane_pixel_count > 4000
  and .headline.rts_map_model_gap_resource_pixel_count > 1000
  and .headline.rts_map_model_gap_height_pixel_count > 3000
  and .headline.rts_map_model_gap_choke_pixel_count > 1000
  and .headline.rts_map_model_gap_structure_pixel_count > 3000
  and .headline.rts_map_model_gap_unit_role_pixel_count > 1000
  and .headline.rts_map_model_gap_occlusion_pixel_count > 2000
  and .headline.rts_map_model_gap_openra_parity_target_commit == "5f1bf76"
  and .headline.rts_map_model_gap_bevy_openra_parity_state == "map_model_catching_up_not_claimed"
  and .headline.rts_map_model_gap_bevy_openra_parity_claimed == false
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
  and .headline.rts_live_input_accepted_input_count == 10
  and .headline.rts_live_input_selection_marker_pixel_count > 1000
  and .headline.rts_live_input_command_marker_pixel_count > 600
  and .headline.rts_live_input_production_queue_pixel_count > 1000
  and .headline.rts_live_input_ability_command_pixel_count > 800
  and .headline.rts_live_input_target_health_pixel_count > 60
  and .headline.rts_live_input_target_health_percent < 60
  and .headline.rts_live_input_hover_preview_pixel_count > 40
  and .headline.rts_live_input_final_hover_player_label == "MINIMAP RALLY READY 5,2"
  and .headline.rts_live_input_context_cursor_pixel_count > 80
  and .headline.rts_live_input_final_context_cursor_label == "MINIMAP CURSOR RALLY READY"
  and .headline.rts_live_input_drag_select_preview_pixel_count > 80
  and .headline.rts_live_input_drag_select_preview_label == "DRAG SELECT 2 UNITS 2,2->6,4"
  and .headline.rts_live_input_drag_select_commit_selected_unit_count == 2
  and .headline.rts_live_input_drag_select_commit_label == "DRAG SELECT SENT 2 UNITS"
  and .headline.rts_live_input_drag_select_commit_selection_marker_pixel_count > 250
  and .headline.rts_live_input_drag_select_filter_selected_unit_count == 5
  and .headline.rts_live_input_drag_select_filter_rejected_unit_count == 1
  and .headline.rts_live_input_drag_select_filter_label == "DRAG SELECT SENT 5 UNITS"
  and .headline.rts_live_input_drag_select_filter_selection_marker_pixel_count > 400
  and .headline.rts_live_input_unit_click_select_marker_pixel_count > 80
  and .headline.rts_live_input_unit_click_select_stamp_pixel_count > 80
  and .headline.rts_live_input_unit_click_select_unit_count == 1
  and .headline.rts_live_input_unit_click_select_label == "MAP SELECT SENT 1 UNIT"
  and .headline.rts_live_input_selection_clear_stamp_pixel_count > 80
  and .headline.rts_live_input_selection_clear_command_disabled_pixel_count > 500
  and .headline.rts_live_input_selection_clear_residual_marker_pixel_count < 80
  and .headline.rts_live_input_selection_clear_empty_label == "MAP SELECTION CLEARED"
  and .headline.rts_live_input_selection_clear_hostile_label == "MAP SELECTION CLEARED HOSTILE"
  and .headline.rts_live_input_right_click_target_label == "MAP ATTACK SENT SQUARE CREEP WANDER"
  and .headline.rts_live_input_right_click_target_hover_label == "MAP ATTACK READY SQUARE CREEP WANDER"
  and .headline.rts_live_input_right_click_target_selected_unit_count == 5
  and .headline.rts_live_input_right_click_target_attack_marker_pixel_count > 20
  and .headline.rts_live_input_right_click_target_sample_count == 4
  and .headline.rts_live_input_right_click_target_move_label == "MAP MOVE SENT 4,3"
  and .headline.rts_live_input_right_click_target_follow_label == "MAP FOLLOW SENT PLAYER"
  and .headline.rts_live_input_right_click_target_harvest_label == "MAP HARVEST SENT GOLD VEIN 3,3"
  and .headline.rts_live_input_rts_core_frame_order_count == 13
  and (.headline.rts_live_input_rts_core_frame_order_kinds | tostring == "[\"train\",\"move\",\"move\",\"hold\",\"patrol\",\"attack_move\",\"stop\",\"attack\",\"ability\",\"move\",\"attack\",\"follow\",\"harvest\"]")
  and (.headline.rts_live_input_rts_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_live_input_rts_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .headline.rts_live_input_rts_core_headless_applied_order_count == 13
  and .headline.rts_live_input_rts_core_headless_actor_count == 8
  and .headline.rts_live_input_rts_core_headless_final_frame == 423
  and .headline.rts_live_input_right_click_target_follow_stamp_pixel_count > 80
  and .headline.rts_live_input_right_click_target_harvest_stamp_pixel_count > 80
  and .headline.rts_live_input_right_click_target_preview_path_pixel_count > 300
  and .headline.rts_live_input_right_click_target_preview_attack_pixel_count > 80
  and .headline.rts_live_input_right_click_target_preview_follow_pixel_count > 80
  and .headline.rts_live_input_right_click_target_preview_harvest_pixel_count > 80
  and .headline.rts_live_input_right_click_execution_feedback_frame_pixel_count > 800
  and .headline.rts_live_input_right_click_execution_feedback_path_pixel_count > 300
  and .headline.rts_live_input_right_click_execution_feedback_target_pixel_count > 80
  and .headline.rts_live_input_right_click_execution_feedback_follow_pixel_count > 80
  and .headline.rts_live_input_right_click_execution_feedback_harvest_pixel_count > 80
  and .headline.rts_live_input_right_click_execution_feedback_viewport_marker_pixel_count > 500
  and .headline.rts_live_input_right_click_execution_feedback_label_pixel_count > 700
  and .headline.rts_live_input_right_click_execution_feedback_move_label == "MOVE EXECUTING 4,3"
  and .headline.rts_live_input_right_click_execution_feedback_attack_label == "ATTACK FOCUS SQUARE CREEP WANDER"
  and .headline.rts_live_input_right_click_execution_feedback_follow_label == "FOLLOWING PLAYER"
  and .headline.rts_live_input_right_click_execution_feedback_harvest_label == "HARVEST GOLD VEIN TO TOWN HALL"
  and .headline.rts_live_input_unit_shift_select_marker_pixel_count > 80
  and .headline.rts_live_input_unit_shift_select_stamp_pixel_count > 80
  and .headline.rts_live_input_unit_shift_select_add_unit_count == 2
  and .headline.rts_live_input_unit_shift_select_remove_unit_count == 1
  and .headline.rts_live_input_unit_shift_select_add_label == "MAP SHIFT SELECT SENT 2 UNITS"
  and .headline.rts_live_input_unit_shift_select_remove_label == "MAP SHIFT SELECT SENT 1 UNIT"
  and .headline.rts_live_input_unit_double_click_select_marker_pixel_count > 80
  and .headline.rts_live_input_unit_double_click_select_stamp_pixel_count > 80
  and .headline.rts_live_input_unit_double_click_select_unit_count == 3
  and .headline.rts_live_input_unit_double_click_select_label == "MAP DOUBLE SELECT SENT 3 UNITS"
  and .headline.rts_live_input_control_group_hotkey_marker_pixel_count > 80
  and .headline.rts_live_input_control_group_hotkey_stamp_pixel_count > 80
  and .headline.rts_live_input_control_group_hotkey_assign_label == "HOTKEY GROUP 5 ASSIGNED 2 UNITS"
  and .headline.rts_live_input_control_group_hotkey_recall_label == "HOTKEY GROUP 5 RECALLED 2 UNITS"
  and .headline.rts_live_input_control_group_hotkey_camera_label == "HOTKEY GROUP 5 CAMERA SNAP"
  and .headline.rts_live_input_control_group_hotkey_append_label == "HOTKEY GROUP 5 APPENDED 3 UNITS"
  and .headline.rts_live_input_control_group_hotkey_recall_add_label == "HOTKEY GROUP 5 ADDED 4 UNITS"
  and .headline.rts_live_input_control_group_hotkey_append_unit_count == 3
  and .headline.rts_live_input_control_group_hotkey_recall_add_unit_count == 4
  and .headline.rts_live_input_control_group_slot_pixel_count > 20
  and .headline.rts_live_input_control_group_slot_5_member_count == 3
  and .headline.rts_live_input_control_group_slot_0_key_label == "0"
  and .headline.rts_live_input_command_stamp_pixel_count > 120
  and .headline.rts_live_input_final_command_stamp_player_label == "COMMAND ABILITY SENT FOCUS FIRE"
  and .headline.rts_live_input_command_queue_path_preview_slot_pixel_count > 1200
  and .headline.rts_live_input_command_queue_path_preview_path_pixel_count > 400
  and .headline.rts_live_input_command_queue_path_preview_waypoint_pixel_count > 200
  and .headline.rts_live_input_command_queue_path_preview_target_pixel_count > 80
  and .headline.rts_live_input_command_queue_path_preview_cancel_pixel_count > 80
  and .headline.rts_pathing_accepted_input_count == 2
  and .headline.rts_pathing_path_tile_count >= 3
  and .headline.rts_pathing_blocked_tile_count >= 1
  and .headline.rts_pathing_formation_slot_count >= 4
  and .headline.rts_pathing_path_tile_pixel_count > 80
  and .headline.rts_pathing_blocked_tile_pixel_count > 40
  and .headline.rts_pathing_formation_slot_pixel_count > 80
  and .headline.rts_pathing_selection_marker_pixel_count > 800
  and .headline.rts_pathing_command_marker_pixel_count > 500
  and .headline.rts_pathing_core_frame_order_count == 1
  and .headline.rts_pathing_core_frame_order_kinds == ["move"]
  and (.headline.rts_pathing_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_pathing_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .headline.rts_pathing_core_headless_applied_order_count == 1
  and .headline.rts_pathing_core_headless_actor_count == 4
  and .headline.rts_pathing_core_headless_final_frame == 720
  and .headline.rts_collision_accepted_input_count == 3
  and .headline.rts_collision_move_disperse_tile_count >= 4
  and .headline.rts_collision_engagement_tile_count >= 4
  and .headline.rts_collision_contact_flash_tile_count >= 2
  and .headline.rts_collision_dispersion_slot_pixel_count > 120
  and .headline.rts_collision_engagement_range_pixel_count > 120
  and .headline.rts_collision_contact_flash_pixel_count > 80
  and .headline.rts_collision_blocked_tile_pixel_count > 40
  and .headline.rts_collision_attack_feedback_pixel_count > 180
  and .headline.rts_collision_core_frame_order_count == 2
  and .headline.rts_collision_core_frame_order_kinds == ["move", "attack"]
  and (.headline.rts_collision_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_collision_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .headline.rts_collision_core_headless_applied_order_count == 2
  and .headline.rts_collision_core_headless_actor_count == 4
  and .headline.rts_collision_core_headless_final_frame == 741
  and .headline.rts_collision_core_headless_attack_order_count == 4
  and .headline.rts_targeting_accepted_input_count == 4
  and .headline.rts_targeting_priority_count >= 3
  and .headline.rts_targeting_focus_fire_unit_count >= 4
  and .headline.rts_targeting_threat_level_count >= 3
  and .headline.rts_targeting_core_frame_order_count == 3
  and .headline.rts_targeting_core_frame_order_kinds == ["move", "attack", "ability"]
  and .headline.rts_targeting_core_headless_applied_order_count == 3
  and .headline.rts_targeting_core_headless_actor_count >= 4
  and .headline.rts_targeting_core_headless_final_frame == 683
  and .headline.rts_targeting_core_headless_ability_order_count == 1
  and (.headline.rts_targeting_core_headless_ability_rule_ids | index("focus_fire") != null)
  and (.headline.rts_targeting_core_headless_ability_target_actor_ids | index("arena_creep_attack") != null)
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
  and .headline.rts_economy_core_frame_order_count == 3
  and .headline.rts_economy_core_frame_order_kinds == ["harvest","build","train"]
  and (.headline.rts_economy_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_economy_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .headline.rts_economy_core_headless_applied_order_count == 3
  and .headline.rts_economy_core_headless_actor_count == 4
  and .headline.rts_economy_core_headless_final_frame == 693
  and .headline.rts_economy_core_lifecycle_order_count == 2
  and .headline.rts_economy_core_build_order_count == 1
  and .headline.rts_economy_core_train_order_count == 1
  and .headline.rts_economy_core_harvest_order_count == 4
  and (.headline.rts_economy_core_build_rule_ids | index("watch_tower") != null)
  and (.headline.rts_economy_core_train_rule_ids | index("worker") != null)
  and .headline.rts_selection_minimap_accepted_input_count == 4
  and .headline.rts_selection_box_tile_count >= 4
  and .headline.rts_control_group_assignment_count >= 2
  and .headline.rts_active_control_group_count >= 2
  and .headline.rts_minimap_command_tile_id == "6,5"
  and .headline.rts_minimap_rally_tile_seen == true
  and .headline.rts_split_route_tile_count >= 4
  and .headline.rts_selection_minimap_pixel_count > 380
  and .headline.rts_selection_box_pixel_count > 160
  and .headline.rts_minimap_command_pixel_count > 80
  and .headline.rts_group_two_pixel_count > 20
  and .headline.rts_split_route_pixel_count > 120
  and .headline.rts_selection_minimap_core_frame_order_count == 2
  and .headline.rts_selection_minimap_core_frame_order_kinds == ["move","move"]
  and (.headline.rts_selection_minimap_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_selection_minimap_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .headline.rts_selection_minimap_core_headless_applied_order_count == 2
  and .headline.rts_selection_minimap_core_headless_actor_count == 4
  and .headline.rts_selection_minimap_core_headless_final_frame == 713
  and .headline.rts_build_lifecycle_accepted_input_count == 6
  and .headline.rts_build_lifecycle_completed_structure_count >= 1
  and .headline.rts_build_lifecycle_cancelled_structure_count >= 1
  and .headline.rts_build_lifecycle_repair_progress_percent >= 76
  and .headline.rts_build_lifecycle_refund_count >= 1
  and .headline.rts_build_lifecycle_core_frame_order_count == 6
  and (.headline.rts_build_lifecycle_core_frame_order_kinds | tostring == "[\"build\",\"complete\",\"repair\",\"build\",\"cancel\",\"refund\"]")
  and (.headline.rts_build_lifecycle_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_build_lifecycle_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and .headline.rts_build_lifecycle_core_headless_applied_order_count == 6
  and .headline.rts_build_lifecycle_core_headless_actor_count == 1
  and .headline.rts_build_lifecycle_core_headless_final_frame == 525
  and .headline.rts_build_lifecycle_core_lifecycle_order_count == 6
  and .headline.rts_build_lifecycle_core_build_order_count == 2
  and .headline.rts_build_lifecycle_core_complete_order_count == 1
  and .headline.rts_build_lifecycle_core_repair_order_count == 1
  and .headline.rts_build_lifecycle_core_cancel_order_count == 1
  and .headline.rts_build_lifecycle_core_refund_order_count == 1
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
  and .headline.rts_tech_tree_core_frame_order_count == 5
  and .headline.rts_tech_tree_core_headless_applied_order_count == 5
  and .headline.rts_tech_tree_core_headless_actor_count == 4
  and .headline.rts_tech_tree_core_headless_final_frame == 644
  and .headline.rts_tech_tree_core_tech_order_count == 3
  and .headline.rts_tech_tree_core_research_order_count == 1
  and .headline.rts_tech_tree_core_upgrade_order_count == 1
  and .headline.rts_tech_tree_core_unlock_order_count == 1
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
  and .headline.rts_projectile_ability_core_frame_order_count == 4
  and .headline.rts_projectile_ability_core_frame_order_kinds == ["move", "attack", "ability", "ability"]
  and .headline.rts_projectile_ability_core_headless_applied_order_count == 4
  and .headline.rts_projectile_ability_core_headless_actor_count >= 2
  and .headline.rts_projectile_ability_core_headless_final_frame == 704
  and .headline.rts_projectile_ability_core_headless_ability_order_count == 2
  and (.headline.rts_projectile_ability_core_headless_ability_rule_ids | index("focus_fire") != null)
  and (.headline.rts_projectile_ability_core_headless_ability_rule_ids | index("guard_break") != null)
  and ((.headline.rts_projectile_ability_core_headless_ability_target_actor_ids | map(select(. == "arena_creep_attack")) | length) == 2)
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
  and .headline.rts_objective_victory_loop_tile_count == 4
  and .headline.rts_objective_victory_loop_capture_percent == 100
  and .headline.rts_objective_victory_loop_owner_state == "player:relay_beacon"
  and .headline.rts_objective_victory_loop_result_state == "victory:relay_beacon_extracted"
  and .headline.rts_objective_victory_loop_extraction_tile_id == "9,2"
  and .headline.rts_objective_victory_loop_defeat_risk_percent <= 8
  and .headline.rts_objective_victory_loop_ai_pressure_percent <= 34
  and .headline.rts_objective_victory_loop_openra_target_commit == "5f1bf76"
  and .headline.rts_objective_victory_loop_openra_target_natural_terminal == true
  and .headline.rts_objective_victory_loop_openra_target_winner_beacons == 2
  and .headline.rts_objective_victory_loop_openra_target_total_beacons == 4
  and .headline.rts_objective_victory_loop_openra_target_hold_ticks == 3000
  and .headline.rts_objective_victory_loop_bevy_terminal_parity_claimed == false
  and .headline.rts_objective_victory_loop_bevy_controlled_beacons == 2
  and .headline.rts_objective_victory_loop_bevy_total_beacons == 4
  and .headline.rts_objective_victory_loop_bevy_control_ratio_percent == 50
  and .headline.rts_objective_victory_loop_bevy_hold_ticks == 3000
  and .headline.rts_objective_victory_loop_pixel_count > 180
  and .headline.rts_objective_victory_loop_objective_pixel_count > 80
  and .headline.rts_objective_victory_loop_capture_bar_pixel_count > 20
  and .headline.rts_objective_victory_loop_victory_pixel_count > 20
  and .headline.rts_objective_victory_loop_defeat_risk_pixel_count > 5
  and .headline.rts_objective_victory_loop_extraction_pixel_count > 40
  and (.headline.rts_objective_victory_loop_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_objective_victory_loop_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_objective_victory_loop_core_frame_order_kinds | tostring == "[\"queue\",\"attack\",\"ability\",\"capture\",\"extract\"]")
  and .headline.rts_objective_victory_loop_core_applied_order_count == 5
  and .headline.rts_objective_victory_loop_core_actor_count >= 4
  and .headline.rts_objective_victory_loop_core_final_frame == 804
  and .headline.rts_objective_victory_loop_core_objective_order_count == 2
  and .headline.rts_objective_victory_loop_core_capture_order_count == 1
  and .headline.rts_objective_victory_loop_core_extract_order_count == 1
  and (.headline.rts_objective_victory_loop_core_objective_ids | index("relay_beacon") != null)
  and (.headline.rts_objective_victory_loop_core_objective_tile_ids | index("6,5") != null)
  and (.headline.rts_objective_victory_loop_core_objective_tile_ids | index("9,2") != null)
  and .headline.rts_autonomous_bot_skirmish_input_action_count == 0
  and .headline.rts_autonomous_bot_skirmish_stage_count == 6
  and .headline.rts_autonomous_bot_skirmish_winner == "Multi2"
  and .headline.rts_autonomous_bot_skirmish_winner_beacons == 2
  and .headline.rts_autonomous_bot_skirmish_total_beacons == 4
  and .headline.rts_autonomous_bot_skirmish_hold_ticks == 3000
  and .headline.rts_autonomous_bot_skirmish_parity_claimed == false
  and .headline.rts_autonomous_bot_skirmish_match_result == "victory:bot_terminal:Multi2"
  and .headline.rts_autonomous_bot_skirmish_spawned_unit_count >= 5
  and .headline.rts_autonomous_bot_skirmish_supply_used >= 14
  and .headline.rts_autonomous_bot_skirmish_supply_cap >= 22
  and .headline.rts_autonomous_bot_skirmish_pixel_count > 500
  and .headline.rts_autonomous_bot_skirmish_ai_wave_pixel_count > 80
  and .headline.rts_autonomous_bot_skirmish_ai_pressure_pixel_count > 120
  and .headline.rts_autonomous_bot_skirmish_ai_counter_pixel_count > 80
  and .headline.rts_autonomous_bot_skirmish_objective_pixel_count > 80
  and .headline.rts_autonomous_bot_skirmish_capture_bar_pixel_count > 20
  and .headline.rts_autonomous_bot_skirmish_match_result_pixel_count > 20
  and .headline.rts_organic_terminal_gap_stage_count == 6
  and .headline.rts_organic_terminal_gap_winner == "Multi2"
  and .headline.rts_organic_terminal_gap_winner_beacons == 2
  and .headline.rts_organic_terminal_gap_total_beacons == 4
  and .headline.rts_organic_terminal_gap_hold_ticks == 3000
  and .headline.rts_organic_terminal_gap_state == "bevy_deterministic_observation_not_openra_natural_gameover"
  and .headline.rts_organic_terminal_gap_openra_parity_target_commit == "5f1bf76"
  and .headline.rts_organic_terminal_gap_winner_count >= 1
  and .headline.rts_organic_terminal_gap_loser_count >= 1
  and .headline.rts_organic_terminal_gap_match_result == "victory:organic_terminal_observed:Multi2"
  and .headline.rts_organic_terminal_gap_pixel_count > 500
  and .headline.rts_organic_terminal_gap_ai_wave_pixel_count > 80
  and .headline.rts_organic_terminal_gap_ai_pressure_pixel_count > 120
  and .headline.rts_organic_terminal_gap_objective_pixel_count > 80
  and .headline.rts_organic_terminal_gap_capture_bar_pixel_count > 20
  and .headline.rts_organic_terminal_gap_match_result_pixel_count > 20
  and .headline.rts_terminal_observation_gap_stage_count == 6
  and .headline.rts_terminal_observation_gap_winner == "Multi2"
  and .headline.rts_terminal_observation_gap_winner_beacons == 2
  and .headline.rts_terminal_observation_gap_total_beacons == 4
  and .headline.rts_terminal_observation_gap_hold_ticks == 3000
  and .headline.rts_terminal_observation_gap_state == "bevy_terminal_observation_vocabulary_not_natural_openra_match"
  and .headline.rts_terminal_observation_gap_openra_readiness_commit == "174525a"
  and .headline.rts_terminal_observation_gap_openra_probe_commit == "bf42eb1"
  and .headline.rts_terminal_observation_gap_openra_strategic_commit == "9e08464"
  and .headline.rts_terminal_observation_gap_terminal_rules_ready == true
  and .headline.rts_terminal_observation_gap_game_over == true
  and .headline.rts_terminal_observation_gap_loser_count == 3
  and .headline.rts_terminal_observation_gap_match_result == "victory:terminal_observation:Multi2"
  and .headline.rts_terminal_observation_gap_pixel_count > 500
  and .headline.rts_terminal_observation_gap_ai_wave_pixel_count > 80
  and .headline.rts_terminal_observation_gap_ai_pressure_pixel_count > 120
  and .headline.rts_terminal_observation_gap_ai_counter_pixel_count > 80
  and .headline.rts_terminal_observation_gap_objective_pixel_count > 80
  and .headline.rts_terminal_observation_gap_capture_bar_pixel_count > 20
  and .headline.rts_terminal_observation_gap_match_result_pixel_count > 20
  and .headline.rts_replay_metrics_gap_stage_count == 6
  and .headline.rts_replay_metrics_gap_state == "bevy_replay_metric_vocabulary_not_openra_replay_file"
  and .headline.rts_replay_metrics_gap_openra_replay_summary_commit == "d5ceade"
  and .headline.rts_replay_metrics_gap_openra_battle_outcome_commit == "9b2664b"
  and .headline.rts_replay_metrics_gap_startgame_order == true
  and .headline.rts_replay_metrics_gap_client_slot_count == 4
  and .headline.rts_replay_metrics_gap_bot_mentions >= 3
  and .headline.rts_replay_metrics_gap_actor_order_tokens >= 12
  and .headline.rts_replay_metrics_gap_unique_actor_token_count >= 6
  and .headline.rts_replay_metrics_gap_economy_tokens >= 12
  and .headline.rts_replay_metrics_gap_tech_tokens >= 6
  and .headline.rts_replay_metrics_gap_combat_tokens >= 12
  and .headline.rts_replay_metrics_gap_configured_seconds >= 55
  and .headline.rts_replay_metrics_gap_elapsed_seconds >= 55
  and .headline.rts_replay_metrics_gap_outcome_signal == "sustained_engagement_no_terminal_victory"
  and .headline.rts_replay_metrics_gap_winner_claimed == false
  and .headline.rts_replay_metrics_gap_pixel_count > 500
  and .headline.rts_replay_metrics_gap_ai_wave_pixel_count > 80
  and .headline.rts_replay_metrics_gap_ai_pressure_pixel_count > 120
  and .headline.rts_replay_metrics_gap_ai_counter_pixel_count > 80
  and .headline.rts_replay_metrics_gap_objective_pixel_count > 80
  and .headline.rts_replay_metrics_gap_match_result_pixel_count > 20
  and .headline.rts_endurance_skirmish_gap_stage_count == 6
  and .headline.rts_endurance_skirmish_gap_state == "bevy_endurance_vocabulary_not_openra_headless_client_match"
  and .headline.rts_endurance_skirmish_gap_openra_endurance_commit == "2cb80a0"
  and .headline.rts_endurance_skirmish_gap_openra_longrun_commit == "5227d99"
  and .headline.rts_endurance_skirmish_gap_openra_autostart_commit == "4b966c1"
  and .headline.rts_endurance_skirmish_gap_startgame_order == true
  and .headline.rts_endurance_skirmish_gap_autostart_order == true
  and .headline.rts_endurance_skirmish_gap_client_slot_count == 4
  and .headline.rts_endurance_skirmish_gap_configured_seconds >= 120
  and .headline.rts_endurance_skirmish_gap_elapsed_seconds >= 120
  and .headline.rts_endurance_skirmish_gap_peak_active_units >= 24
  and .headline.rts_endurance_skirmish_gap_contested_beacon_peak >= 2
  and .headline.rts_endurance_skirmish_gap_economy_events >= 12
  and .headline.rts_endurance_skirmish_gap_combat_events >= 20
  and .headline.rts_endurance_skirmish_gap_tech_events >= 6
  and .headline.rts_endurance_skirmish_gap_outcome_signal == "sustained_engagement_no_terminal_victory"
  and .headline.rts_endurance_skirmish_gap_winner_claimed == false
  and .headline.rts_endurance_skirmish_gap_pixel_count > 500
  and .headline.rts_endurance_skirmish_gap_ai_wave_pixel_count > 80
  and .headline.rts_endurance_skirmish_gap_ai_pressure_pixel_count > 120
  and .headline.rts_endurance_skirmish_gap_ai_counter_pixel_count > 80
  and .headline.rts_endurance_skirmish_gap_objective_pixel_count > 80
  and .headline.rts_endurance_skirmish_gap_match_result_pixel_count > 20
  and .headline.rts_bot_decision_state_gap_stage_count == 6
  and .headline.rts_bot_decision_state_gap_state == "bevy_bot_decision_vocabulary_not_openra_native_bot_ai"
  and .headline.rts_bot_decision_state_gap_openra_economy_tech_commit == "f6c47d9"
  and .headline.rts_bot_decision_state_gap_openra_beacon_pressure_commit == "2b6f25b"
  and .headline.rts_bot_decision_state_gap_openra_organic_terminal_commit == "5f1bf76"
  and (.headline.rts_bot_decision_state_gap_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_bot_decision_state_gap_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_bot_decision_state_gap_core_frame_order_kinds | tostring == "[\"harvest\",\"recon\",\"capture\",\"research\",\"attack\",\"move\"]")
  and .headline.rts_bot_decision_state_gap_core_headless_applied_orders == 6
  and .headline.rts_bot_decision_state_gap_core_headless_actor_count >= 3
  and .headline.rts_bot_decision_state_gap_core_headless_final_frame == 1005
  and .headline.rts_bot_decision_state_gap_core_headless_harvest_actor_orders >= 3
  and .headline.rts_bot_decision_state_gap_core_headless_scout_orders == 1
  and .headline.rts_bot_decision_state_gap_core_headless_capture_orders == 1
  and .headline.rts_bot_decision_state_gap_core_headless_research_orders == 1
  and .headline.rts_bot_decision_state_gap_core_headless_attack_orders == 1
  and .headline.rts_bot_decision_state_gap_core_headless_micro_move_orders == 1
  and (.headline.rts_bot_decision_state_gap_core_headless_recon_ids | index("beacon_ring") != null)
  and (.headline.rts_bot_decision_state_gap_core_headless_objective_ids | index("relay_beacon") != null)
  and (.headline.rts_bot_decision_state_gap_core_headless_researched_rules | index("signal_array") != null)
  and (.headline.rts_bot_decision_state_gap_core_headless_combat_targets | index("counter_push") != null)
  and (.headline.rts_bot_decision_state_gap_core_headless_combat_tiles | index("8,4") != null)
  and (.headline.rts_bot_decision_state_gap_core_headless_combat_formations | index("attack_commit_repath") != null)
  and .headline.rts_bot_decision_state_gap_decision_signals >= 18
  and .headline.rts_bot_decision_state_gap_economy_decisions >= 3
  and .headline.rts_bot_decision_state_gap_objective_decisions >= 4
  and .headline.rts_bot_decision_state_gap_combat_decisions >= 4
  and .headline.rts_bot_decision_state_gap_tech_decisions >= 2
  and .headline.rts_bot_decision_state_gap_final_state == "attack_commit_with_counter_repath"
  and .headline.rts_bot_decision_state_gap_final_pressure_percent >= 70
  and .headline.rts_bot_decision_state_gap_final_defeat_risk_percent <= 35
  and .headline.rts_bot_decision_state_gap_final_capture_percent >= 90
  and .headline.rts_bot_decision_state_gap_match_result == "bot_decision_gap:attack_commit_with_counter_repath"
  and .headline.rts_bot_decision_state_gap_pixel_count > 500
  and .headline.rts_bot_decision_state_gap_ai_wave_pixel_count > 80
  and .headline.rts_bot_decision_state_gap_ai_pressure_pixel_count > 120
  and .headline.rts_bot_decision_state_gap_ai_counter_pixel_count > 80
  and .headline.rts_bot_decision_state_gap_objective_pixel_count > 80
  and .headline.rts_bot_decision_state_gap_capture_bar_pixel_count > 20
  and .headline.rts_bot_decision_state_gap_match_result_pixel_count > 20
  and .headline.rts_bot_adaptive_build_order_gap_stage_count == 6
  and .headline.rts_bot_adaptive_build_order_gap_state == "bevy_adaptive_build_order_vocabulary_not_openra_native_ai_planner"
	  and .headline.rts_bot_adaptive_build_order_gap_openra_economy_tech_commit == "f6c47d9"
	  and .headline.rts_bot_adaptive_build_order_gap_openra_beacon_pressure_commit == "2b6f25b"
	  and .headline.rts_bot_adaptive_build_order_gap_openra_organic_terminal_commit == "5f1bf76"
	  and (.headline.rts_bot_adaptive_build_order_gap_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
	  and (.headline.rts_bot_adaptive_build_order_gap_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
	  and .headline.rts_bot_adaptive_build_order_gap_core_frame_order_kinds == ["harvest", "build", "train", "recon", "build", "research", "train", "attack", "move"]
	  and .headline.rts_bot_adaptive_build_order_gap_core_headless_applied_orders == 9
	  and .headline.rts_bot_adaptive_build_order_gap_core_headless_actor_count >= 3
	  and .headline.rts_bot_adaptive_build_order_gap_core_headless_final_frame == 1408
	  and .headline.rts_bot_adaptive_build_order_gap_core_headless_harvest_actor_orders >= 3
	  and .headline.rts_bot_adaptive_build_order_gap_core_headless_build_orders == 2
	  and .headline.rts_bot_adaptive_build_order_gap_core_headless_train_orders == 2
	  and (.headline.rts_bot_adaptive_build_order_gap_core_headless_build_rules | index("relay_refinery") != null)
	  and (.headline.rts_bot_adaptive_build_order_gap_core_headless_build_rules | index("forge_natural_defense") != null)
	  and (.headline.rts_bot_adaptive_build_order_gap_core_headless_train_rules | index("trnm.horizon.skimmer") != null)
	  and .headline.rts_bot_adaptive_build_order_gap_core_headless_scout_orders == 1
	  and (.headline.rts_bot_adaptive_build_order_gap_core_headless_recon_ids | index("enemy_fast_beacon") != null)
	  and (.headline.rts_bot_adaptive_build_order_gap_core_headless_recon_tiles | index("6,5") != null)
	  and .headline.rts_bot_adaptive_build_order_gap_core_headless_research_orders == 1
	  and (.headline.rts_bot_adaptive_build_order_gap_core_headless_researched_rules | index("signal_array") != null)
	  and (.headline.rts_bot_adaptive_build_order_gap_core_headless_research_sources | index("town_hall") != null)
	  and .headline.rts_bot_adaptive_build_order_gap_core_headless_attack_orders == 1
	  and .headline.rts_bot_adaptive_build_order_gap_core_headless_micro_move_orders == 1
	  and (.headline.rts_bot_adaptive_build_order_gap_core_headless_combat_targets | index("beacon_pressure_window") != null)
	  and (.headline.rts_bot_adaptive_build_order_gap_core_headless_combat_tiles | index("9,5") != null)
	  and (.headline.rts_bot_adaptive_build_order_gap_core_headless_combat_formations | index("pullback_rebuild_then_reattack") != null)
	  and .headline.rts_bot_adaptive_build_order_gap_adaptive_signals >= 24
	  and .headline.rts_bot_adaptive_build_order_gap_opening_build_orders >= 3
  and .headline.rts_bot_adaptive_build_order_gap_scout_triggers >= 2
  and .headline.rts_bot_adaptive_build_order_gap_branch_switches >= 3
  and .headline.rts_bot_adaptive_build_order_gap_counter_tech_switches >= 2
  and .headline.rts_bot_adaptive_build_order_gap_pressure_windows >= 2
  and .headline.rts_bot_adaptive_build_order_gap_retreat_rebuilds >= 2
  and .headline.rts_bot_adaptive_build_order_gap_final_state == "pressure_window_rebuild_reattack"
  and .headline.rts_bot_adaptive_build_order_gap_final_pressure_percent >= 70
  and .headline.rts_bot_adaptive_build_order_gap_final_defeat_risk_percent <= 20
  and .headline.rts_bot_adaptive_build_order_gap_final_capture_percent >= 90
  and .headline.rts_bot_adaptive_build_order_gap_match_result == "adaptive_build_gap:pressure_window_rebuild_reattack"
  and .headline.rts_bot_adaptive_build_order_gap_pixel_count > 500
  and .headline.rts_bot_adaptive_build_order_gap_ai_wave_pixel_count > 80
  and .headline.rts_bot_adaptive_build_order_gap_ai_pressure_pixel_count > 120
  and .headline.rts_bot_adaptive_build_order_gap_ai_counter_pixel_count > 80
  and .headline.rts_bot_adaptive_build_order_gap_objective_pixel_count > 80
  and .headline.rts_bot_adaptive_build_order_gap_capture_bar_pixel_count > 20
  and .headline.rts_bot_adaptive_build_order_gap_match_result_pixel_count > 20
  and .headline.rts_bot_tactical_micro_gap_stage_count == 6
  and .headline.rts_bot_tactical_micro_gap_state == "bevy_tactical_micro_vocabulary_not_openra_native_combat_ai"
  and .headline.rts_bot_tactical_micro_gap_openra_economy_tech_commit == "f6c47d9"
  and .headline.rts_bot_tactical_micro_gap_openra_beacon_pressure_commit == "2b6f25b"
  and .headline.rts_bot_tactical_micro_gap_openra_organic_terminal_commit == "5f1bf76"
  and (.headline.rts_bot_tactical_micro_gap_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_bot_tactical_micro_gap_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_bot_tactical_micro_gap_core_frame_order_kinds | tostring == "[\"attack\",\"focus_fire\",\"move\",\"move\",\"ability\",\"move\"]")
  and .headline.rts_bot_tactical_micro_gap_core_headless_applied_orders == 6
  and .headline.rts_bot_tactical_micro_gap_core_headless_actor_count >= 2
  and .headline.rts_bot_tactical_micro_gap_core_headless_final_frame == 1205
  and .headline.rts_bot_tactical_micro_gap_core_headless_attack_orders == 1
  and .headline.rts_bot_tactical_micro_gap_core_headless_focus_fire_orders == 1
  and .headline.rts_bot_tactical_micro_gap_core_headless_micro_move_orders == 3
  and .headline.rts_bot_tactical_micro_gap_core_headless_ability_orders == 1
  and (.headline.rts_bot_tactical_micro_gap_core_headless_combat_targets | index("warden_frontline") != null)
  and (.headline.rts_bot_tactical_micro_gap_core_headless_combat_targets | index("low_armor_striker") != null)
  and (.headline.rts_bot_tactical_micro_gap_core_headless_combat_tiles | index("8,5") != null)
  and (.headline.rts_bot_tactical_micro_gap_core_headless_combat_tiles | index("7,4") != null)
  and (.headline.rts_bot_tactical_micro_gap_core_headless_combat_tiles | index("6,4") != null)
  and (.headline.rts_bot_tactical_micro_gap_core_headless_combat_formations | index("kite_step") != null)
  and (.headline.rts_bot_tactical_micro_gap_core_headless_combat_formations | index("flank_split") != null)
  and (.headline.rts_bot_tactical_micro_gap_core_headless_combat_formations | index("pullback") != null)
  and (.headline.rts_bot_tactical_micro_gap_core_headless_ability_rules | index("signal_burst") != null)
  and (.headline.rts_bot_tactical_micro_gap_core_headless_ability_targets | index("relay_beacon") != null)
  and .headline.rts_bot_tactical_micro_gap_micro_signals >= 24
  and .headline.rts_bot_tactical_micro_gap_target_swaps >= 3
  and .headline.rts_bot_tactical_micro_gap_focus_fire_orders >= 3
  and .headline.rts_bot_tactical_micro_gap_kite_steps >= 3
  and .headline.rts_bot_tactical_micro_gap_flank_angles >= 2
  and .headline.rts_bot_tactical_micro_gap_ability_timings >= 2
  and .headline.rts_bot_tactical_micro_gap_low_health_pullbacks >= 2
  and .headline.rts_bot_tactical_micro_gap_final_state == "pullback_regroup_reattack"
  and .headline.rts_bot_tactical_micro_gap_final_pressure_percent >= 70
  and .headline.rts_bot_tactical_micro_gap_final_defeat_risk_percent <= 20
  and .headline.rts_bot_tactical_micro_gap_final_capture_percent >= 90
  and .headline.rts_bot_tactical_micro_gap_match_result == "tactical_micro_gap:pullback_regroup_reattack"
  and .headline.rts_bot_tactical_micro_gap_pixel_count > 500
  and .headline.rts_bot_tactical_micro_gap_ai_wave_pixel_count > 80
  and .headline.rts_bot_tactical_micro_gap_ai_pressure_pixel_count > 120
  and .headline.rts_bot_tactical_micro_gap_ai_counter_pixel_count > 80
  and .headline.rts_bot_tactical_micro_gap_objective_pixel_count > 80
  and .headline.rts_bot_tactical_micro_gap_capture_bar_pixel_count > 20
  and .headline.rts_bot_tactical_micro_gap_match_result_pixel_count > 20
  and .headline.rts_bot_map_intel_gap_stage_count == 6
  and .headline.rts_bot_map_intel_gap_state == "bevy_map_intel_vocabulary_not_openra_native_shroud_memory_ai"
  and .headline.rts_bot_map_intel_gap_openra_economy_tech_commit == "f6c47d9"
  and .headline.rts_bot_map_intel_gap_openra_beacon_pressure_commit == "2b6f25b"
  and .headline.rts_bot_map_intel_gap_openra_organic_terminal_commit == "5f1bf76"
  and (.headline.rts_bot_map_intel_gap_core_frame_order_stream_sha256 | type == "string" and length == 64)
  and (.headline.rts_bot_map_intel_gap_core_headless_checkpoint_sha256 | type == "string" and length == 64)
  and .headline.rts_bot_map_intel_gap_core_frame_order_kinds == ["recon", "recon", "recon", "recon", "recon", "move"]
  and .headline.rts_bot_map_intel_gap_core_headless_applied_orders == 6
  and .headline.rts_bot_map_intel_gap_core_headless_actor_count >= 3
  and .headline.rts_bot_map_intel_gap_core_headless_final_frame == 1605
  and .headline.rts_bot_map_intel_gap_core_headless_recon_orders == 5
  and .headline.rts_bot_map_intel_gap_core_headless_scout_orders == 2
  and .headline.rts_bot_map_intel_gap_core_headless_mark_orders == 1
  and .headline.rts_bot_map_intel_gap_core_headless_sweep_orders == 1
  and .headline.rts_bot_map_intel_gap_core_headless_scan_orders == 1
  and (.headline.rts_bot_map_intel_gap_core_headless_recon_ids | index("fog_memory_last_seen_grid") != null)
  and (.headline.rts_bot_map_intel_gap_core_headless_recon_ids | index("enemy_signal_array_tech") != null)
  and (.headline.rts_bot_map_intel_gap_core_headless_recon_tiles | index("5,5") != null)
  and (.headline.rts_bot_map_intel_gap_core_headless_recon_tiles | index("8,4") != null)
  and .headline.rts_bot_map_intel_gap_core_headless_micro_move_orders == 1
  and (.headline.rts_bot_map_intel_gap_core_headless_combat_tiles | index("9,5") != null)
  and (.headline.rts_bot_map_intel_gap_core_headless_combat_formations | index("rotate_pressure_to_confirmed_beacon") != null)
  and .headline.rts_bot_map_intel_gap_intel_signals >= 24
  and .headline.rts_bot_map_intel_gap_scout_sweeps >= 3
  and .headline.rts_bot_map_intel_gap_fog_memory_stamps >= 4
  and .headline.rts_bot_map_intel_gap_expansion_threats >= 3
  and .headline.rts_bot_map_intel_gap_enemy_tech_reads >= 2
  and .headline.rts_bot_map_intel_gap_hidden_army_predictions >= 2
  and .headline.rts_bot_map_intel_gap_pressure_rotations >= 2
  and .headline.rts_bot_map_intel_gap_final_state == "rotate_pressure_confirmed_beacon"
  and .headline.rts_bot_map_intel_gap_final_pressure_percent >= 80
  and .headline.rts_bot_map_intel_gap_final_defeat_risk_percent <= 20
  and .headline.rts_bot_map_intel_gap_final_capture_percent >= 90
  and .headline.rts_bot_map_intel_gap_match_result == "map_intel_gap:rotate_pressure_confirmed_beacon"
  and .headline.rts_bot_map_intel_gap_pixel_count > 500
  and .headline.rts_bot_map_intel_gap_ai_wave_pixel_count > 80
  and .headline.rts_bot_map_intel_gap_ai_pressure_pixel_count > 120
  and .headline.rts_bot_map_intel_gap_ai_counter_pixel_count > 80
  and .headline.rts_bot_map_intel_gap_objective_pixel_count > 80
  and .headline.rts_bot_map_intel_gap_capture_bar_pixel_count > 20
  and .headline.rts_bot_map_intel_gap_match_result_pixel_count > 20
  and .headline.rts_bot_macro_economy_gap_stage_count == 6
  and .headline.rts_bot_macro_economy_gap_state == "bevy_macro_economy_vocabulary_not_openra_native_economy_ai"
  and .headline.rts_bot_macro_economy_gap_openra_economy_tech_commit == "f6c47d9"
  and .headline.rts_bot_macro_economy_gap_openra_beacon_pressure_commit == "2b6f25b"
  and .headline.rts_bot_macro_economy_gap_openra_organic_terminal_commit == "5f1bf76"
  and (.headline.rts_bot_macro_economy_gap_core_frame_order_stream_sha256 | type == "string" and length == 64)
  and (.headline.rts_bot_macro_economy_gap_core_headless_checkpoint_sha256 | type == "string" and length == 64)
  and .headline.rts_bot_macro_economy_gap_core_frame_order_kinds == ["harvest", "train", "build", "build", "train", "train", "research", "attack", "move"]
  and .headline.rts_bot_macro_economy_gap_core_headless_applied_orders == 9
  and .headline.rts_bot_macro_economy_gap_core_headless_actor_count >= 3
  and .headline.rts_bot_macro_economy_gap_core_headless_final_frame == 1808
  and .headline.rts_bot_macro_economy_gap_core_headless_harvest_actor_orders >= 3
  and .headline.rts_bot_macro_economy_gap_core_headless_build_orders == 2
  and .headline.rts_bot_macro_economy_gap_core_headless_train_orders == 3
  and (.headline.rts_bot_macro_economy_gap_core_headless_build_rules | index("natural_refinery") != null)
  and (.headline.rts_bot_macro_economy_gap_core_headless_build_rules | index("supply_cache") != null)
  and (.headline.rts_bot_macro_economy_gap_core_headless_train_rules | index("trnm.worker") != null)
  and (.headline.rts_bot_macro_economy_gap_core_headless_train_rules | index("trnm.horizon.skimmer") != null)
  and (.headline.rts_bot_macro_economy_gap_core_headless_train_rules | index("trnm.forge.warden") != null)
  and .headline.rts_bot_macro_economy_gap_core_headless_research_orders == 1
  and (.headline.rts_bot_macro_economy_gap_core_headless_researched_rules | index("signal_array") != null)
  and (.headline.rts_bot_macro_economy_gap_core_headless_research_source_actor_ids | index("town_hall") != null)
  and .headline.rts_bot_macro_economy_gap_core_headless_attack_orders == 1
  and .headline.rts_bot_macro_economy_gap_core_headless_micro_move_orders == 1
  and (.headline.rts_bot_macro_economy_gap_core_headless_combat_targets | index("enemy_rebuild_node") != null)
  and (.headline.rts_bot_macro_economy_gap_core_headless_combat_tiles | index("9,2") != null)
  and (.headline.rts_bot_macro_economy_gap_core_headless_combat_formations | index("deny_enemy_node_rebuild_army") != null)
  and .headline.rts_bot_macro_economy_gap_macro_signals >= 24
  and .headline.rts_bot_macro_economy_gap_worker_saturation >= 12
  and .headline.rts_bot_macro_economy_gap_expansion_timings >= 3
  and .headline.rts_bot_macro_economy_gap_supply_recoveries >= 3
  and .headline.rts_bot_macro_economy_gap_production_cycles >= 4
  and .headline.rts_bot_macro_economy_gap_tech_ramps >= 2
  and .headline.rts_bot_macro_economy_gap_resource_denies >= 2
  and .headline.rts_bot_macro_economy_gap_final_state == "deny_rebuild_pressure"
  and .headline.rts_bot_macro_economy_gap_final_pressure_percent >= 80
  and .headline.rts_bot_macro_economy_gap_final_defeat_risk_percent <= 20
  and .headline.rts_bot_macro_economy_gap_final_capture_percent >= 90
  and .headline.rts_bot_macro_economy_gap_match_result == "macro_economy_gap:deny_rebuild_pressure"
  and .headline.rts_bot_macro_economy_gap_pixel_count > 500
  and .headline.rts_bot_macro_economy_gap_ai_wave_pixel_count > 80
  and .headline.rts_bot_macro_economy_gap_ai_pressure_pixel_count > 120
  and .headline.rts_bot_macro_economy_gap_ai_counter_pixel_count > 80
  and .headline.rts_bot_macro_economy_gap_objective_pixel_count > 80
  and .headline.rts_bot_macro_economy_gap_capture_bar_pixel_count > 20
  and .headline.rts_bot_macro_economy_gap_match_result_pixel_count > 20
  and .headline.rts_bot_harassment_defense_gap_stage_count == 6
  and .headline.rts_bot_harassment_defense_gap_state == "bevy_harassment_defense_vocabulary_not_openra_native_harassment_ai"
  and .headline.rts_bot_harassment_defense_gap_openra_economy_tech_commit == "f6c47d9"
  and .headline.rts_bot_harassment_defense_gap_openra_beacon_pressure_commit == "2b6f25b"
  and .headline.rts_bot_harassment_defense_gap_openra_organic_terminal_commit == "5f1bf76"
  and .headline.rts_bot_harassment_defense_gap_harassment_signals >= 24
  and .headline.rts_bot_harassment_defense_gap_worker_pullbacks >= 4
  and .headline.rts_bot_harassment_defense_gap_repair_cycles >= 3
  and .headline.rts_bot_harassment_defense_gap_static_defense_responses >= 3
  and .headline.rts_bot_harassment_defense_gap_counter_raids >= 3
  and .headline.rts_bot_harassment_defense_gap_retreat_paths >= 2
  and .headline.rts_bot_harassment_defense_gap_rebuild_secures >= 2
  and .headline.rts_bot_harassment_defense_gap_final_state == "counter_raid_rebuild_secured"
  and .headline.rts_bot_harassment_defense_gap_final_pressure_percent >= 80
  and .headline.rts_bot_harassment_defense_gap_final_defeat_risk_percent <= 20
  and .headline.rts_bot_harassment_defense_gap_final_capture_percent >= 90
  and .headline.rts_bot_harassment_defense_gap_match_result == "harassment_defense_gap:counter_raid_rebuild_secured"
  and .headline.rts_bot_harassment_defense_gap_pixel_count > 500
  and .headline.rts_bot_harassment_defense_gap_ai_wave_pixel_count > 80
  and .headline.rts_bot_harassment_defense_gap_ai_pressure_pixel_count > 120
  and .headline.rts_bot_harassment_defense_gap_ai_counter_pixel_count > 80
  and .headline.rts_bot_harassment_defense_gap_objective_pixel_count > 80
  and .headline.rts_bot_harassment_defense_gap_capture_bar_pixel_count > 20
  and .headline.rts_bot_harassment_defense_gap_match_result_pixel_count > 20
  and .headline.rts_bot_multi_front_pressure_gap_stage_count == 6
  and .headline.rts_bot_multi_front_pressure_gap_state == "bevy_multi_front_pressure_vocabulary_not_openra_native_split_map_ai"
  and .headline.rts_bot_multi_front_pressure_gap_openra_economy_tech_commit == "f6c47d9"
  and .headline.rts_bot_multi_front_pressure_gap_openra_beacon_pressure_commit == "2b6f25b"
  and .headline.rts_bot_multi_front_pressure_gap_openra_organic_terminal_commit == "5f1bf76"
  and .headline.rts_bot_multi_front_pressure_gap_multi_front_signals >= 24
  and .headline.rts_bot_multi_front_pressure_gap_split_lanes >= 2
  and .headline.rts_bot_multi_front_pressure_gap_decoy_pressures >= 3
  and .headline.rts_bot_multi_front_pressure_gap_rotations >= 3
  and .headline.rts_bot_multi_front_pressure_gap_reinforce_joins >= 3
  and .headline.rts_bot_multi_front_pressure_gap_simultaneous_hits >= 2
  and .headline.rts_bot_multi_front_pressure_gap_terminal_collapses >= 2
  and .headline.rts_bot_multi_front_pressure_gap_final_state == "terminal_collapse_secured"
  and .headline.rts_bot_multi_front_pressure_gap_final_pressure_percent >= 80
  and .headline.rts_bot_multi_front_pressure_gap_final_defeat_risk_percent <= 20
  and .headline.rts_bot_multi_front_pressure_gap_final_capture_percent >= 90
  and .headline.rts_bot_multi_front_pressure_gap_match_result == "multi_front_pressure_gap:terminal_collapse_secured"
  and .headline.rts_bot_multi_front_pressure_gap_pixel_count > 500
  and .headline.rts_bot_multi_front_pressure_gap_ai_wave_pixel_count > 80
  and .headline.rts_bot_multi_front_pressure_gap_ai_pressure_pixel_count > 120
  and .headline.rts_bot_multi_front_pressure_gap_ai_counter_pixel_count > 80
  and .headline.rts_bot_multi_front_pressure_gap_objective_pixel_count > 80
  and .headline.rts_bot_multi_front_pressure_gap_capture_bar_pixel_count > 20
  and .headline.rts_bot_multi_front_pressure_gap_match_result_pixel_count > 20
  and .headline.rts_bot_expansion_control_gap_stage_count == 6
  and .headline.rts_bot_expansion_control_gap_state == "bevy_expansion_control_vocabulary_not_openra_native_map_control_ai"
  and .headline.rts_bot_expansion_control_gap_openra_economy_tech_commit == "f6c47d9"
  and .headline.rts_bot_expansion_control_gap_openra_beacon_pressure_commit == "2b6f25b"
  and .headline.rts_bot_expansion_control_gap_openra_organic_terminal_commit == "5f1bf76"
  and .headline.rts_bot_expansion_control_gap_expansion_control_signals >= 24
  and .headline.rts_bot_expansion_control_gap_natural_probes >= 3
  and .headline.rts_bot_expansion_control_gap_third_node_denies >= 3
  and .headline.rts_bot_expansion_control_gap_refinery_pickoffs >= 2
  and .headline.rts_bot_expansion_control_gap_contain_rings >= 3
  and .headline.rts_bot_expansion_control_gap_reexpand_punishes >= 2
  and .headline.rts_bot_expansion_control_gap_map_locks >= 2
  and .headline.rts_bot_expansion_control_gap_final_state == "map_control_lock_secured"
  and .headline.rts_bot_expansion_control_gap_final_pressure_percent >= 85
  and .headline.rts_bot_expansion_control_gap_final_defeat_risk_percent <= 20
  and .headline.rts_bot_expansion_control_gap_final_capture_percent >= 90
  and .headline.rts_bot_expansion_control_gap_match_result == "expansion_control_gap:map_control_lock_secured"
  and .headline.rts_bot_expansion_control_gap_pixel_count > 500
  and .headline.rts_bot_expansion_control_gap_ai_wave_pixel_count > 80
  and .headline.rts_bot_expansion_control_gap_ai_pressure_pixel_count > 120
  and .headline.rts_bot_expansion_control_gap_ai_counter_pixel_count > 80
  and .headline.rts_bot_expansion_control_gap_objective_pixel_count > 80
  and .headline.rts_bot_expansion_control_gap_capture_bar_pixel_count > 20
  and .headline.rts_bot_expansion_control_gap_match_result_pixel_count > 20
  and .headline.rts_bot_tech_transition_gap_stage_count == 6
  and .headline.rts_bot_tech_transition_gap_state == "bevy_tech_transition_vocabulary_not_openra_native_tech_switch_ai"
  and .headline.rts_bot_tech_transition_gap_openra_economy_tech_commit == "f6c47d9"
  and .headline.rts_bot_tech_transition_gap_openra_beacon_pressure_commit == "2b6f25b"
  and .headline.rts_bot_tech_transition_gap_openra_organic_terminal_commit == "5f1bf76"
  and .headline.rts_bot_tech_transition_gap_tech_transition_signals >= 24
  and .headline.rts_bot_tech_transition_gap_signal_reads >= 3
  and .headline.rts_bot_tech_transition_gap_counter_switches >= 3
  and .headline.rts_bot_tech_transition_gap_anti_air_timings >= 2
  and .headline.rts_bot_tech_transition_gap_siege_responses >= 2
  and .headline.rts_bot_tech_transition_gap_upgrade_windows >= 3
  and .headline.rts_bot_tech_transition_gap_terminal_tech_locks >= 2
  and .headline.rts_bot_tech_transition_gap_final_state == "terminal_tech_lock_secured"
  and .headline.rts_bot_tech_transition_gap_final_pressure_percent >= 90
  and .headline.rts_bot_tech_transition_gap_final_defeat_risk_percent <= 15
  and .headline.rts_bot_tech_transition_gap_final_capture_percent >= 95
  and .headline.rts_bot_tech_transition_gap_match_result == "tech_transition_gap:terminal_tech_lock_secured"
  and .headline.rts_bot_tech_transition_gap_pixel_count > 500
  and .headline.rts_bot_tech_transition_gap_ai_wave_pixel_count > 80
  and .headline.rts_bot_tech_transition_gap_ai_pressure_pixel_count > 120
  and .headline.rts_bot_tech_transition_gap_ai_counter_pixel_count > 80
  and .headline.rts_bot_tech_transition_gap_objective_pixel_count > 80
  and .headline.rts_bot_tech_transition_gap_capture_bar_pixel_count > 20
  and .headline.rts_bot_tech_transition_gap_match_result_pixel_count > 20
  and .headline.rts_bot_army_composition_gap_stage_count == 6
  and .headline.rts_bot_army_composition_gap_state == "bevy_army_composition_vocabulary_not_openra_native_unit_mix_ai"
  and .headline.rts_bot_army_composition_gap_openra_economy_tech_commit == "f6c47d9"
  and .headline.rts_bot_army_composition_gap_openra_beacon_pressure_commit == "2b6f25b"
  and .headline.rts_bot_army_composition_gap_openra_organic_terminal_commit == "5f1bf76"
  and .headline.rts_bot_army_composition_gap_army_composition_signals >= 24
  and .headline.rts_bot_army_composition_gap_unit_mix_reads >= 3
  and .headline.rts_bot_army_composition_gap_frontline_ratios >= 3
  and .headline.rts_bot_army_composition_gap_counter_mix_swaps >= 3
  and .headline.rts_bot_army_composition_gap_reinforce_curves >= 3
  and .headline.rts_bot_army_composition_gap_specialist_timings >= 2
  and .headline.rts_bot_army_composition_gap_composition_locks >= 2
  and .headline.rts_bot_army_composition_gap_final_state == "terminal_composition_lock_secured"
  and .headline.rts_bot_army_composition_gap_final_pressure_percent >= 90
  and .headline.rts_bot_army_composition_gap_final_defeat_risk_percent <= 15
  and .headline.rts_bot_army_composition_gap_final_capture_percent >= 95
  and .headline.rts_bot_army_composition_gap_match_result == "army_composition_gap:terminal_composition_lock_secured"
  and .headline.rts_bot_army_composition_gap_pixel_count > 500
  and .headline.rts_bot_army_composition_gap_ai_wave_pixel_count > 80
  and .headline.rts_bot_army_composition_gap_ai_pressure_pixel_count > 120
  and .headline.rts_bot_army_composition_gap_ai_counter_pixel_count > 80
  and .headline.rts_bot_army_composition_gap_objective_pixel_count > 80
  and .headline.rts_bot_army_composition_gap_capture_bar_pixel_count > 20
  and .headline.rts_bot_army_composition_gap_match_result_pixel_count > 20
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
  and (.headline.rts_fog_scouting_intel_core_frame_order_stream_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_fog_scouting_intel_core_headless_checkpoint_sha256 | test("^[0-9a-f]{64}$"))
  and (.headline.rts_fog_scouting_intel_core_frame_order_kinds | tostring == "[\"recon\",\"move\",\"recon\",\"recon\",\"recon\"]")
  and .headline.rts_fog_scouting_intel_core_applied_order_count == 5
  and .headline.rts_fog_scouting_intel_core_actor_count >= 2
  and .headline.rts_fog_scouting_intel_core_final_frame == 904
  and .headline.rts_fog_scouting_intel_core_recon_order_count == 4
  and .headline.rts_fog_scouting_intel_core_scout_order_count == 1
  and .headline.rts_fog_scouting_intel_core_sweep_order_count == 1
  and .headline.rts_fog_scouting_intel_core_scan_order_count == 1
  and .headline.rts_fog_scouting_intel_core_mark_order_count == 1
  and (.headline.rts_fog_scouting_intel_core_recon_ids | index("enemy_base") != null)
  and (.headline.rts_fog_scouting_intel_core_recon_ids | index("watchtower_scan") != null)
  and (.headline.rts_fog_scouting_intel_core_recon_tile_ids | index("10,2") != null)
  and (.headline.rts_fog_scouting_intel_core_recon_tile_ids | index("7,4") != null)
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
  and .headline.rts_production_spawn_animation_accepted_input_count == 6
  and .headline.rts_production_spawn_animation_supply_cap >= 18
  and .headline.rts_production_spawn_animation_supply_used >= 10
  and .headline.rts_production_spawn_animation_supply_used <= .headline.rts_production_spawn_animation_supply_cap
  and .headline.rts_production_spawn_animation_spawned_unit_count >= 4
  and .headline.rts_production_spawn_animation_rally_tile_count >= 5
  and .headline.rts_production_spawn_animation_training_progress_percent == 100
  and .headline.rts_production_spawn_animation_queue_pulse_pixel_count > 120
  and .headline.rts_production_spawn_animation_training_tick_pixel_count > 120
  and .headline.rts_production_spawn_animation_spawn_door_pixel_count > 120
  and .headline.rts_production_spawn_animation_rally_flag_pixel_count > 120
  and .headline.rts_production_spawn_animation_formation_join_pixel_count > 120
  and .headline.rts_production_spawn_animation_supply_flash_pixel_count > 120
  and .headline.rts_unit_status_portrait_frame_pixel_count > 1200
  and .headline.rts_unit_status_health_bar_pixel_count > 300
  and .headline.rts_unit_status_mana_bar_pixel_count > 240
  and .headline.rts_unit_status_xp_bar_pixel_count > 200
  and .headline.rts_unit_status_buff_badge_pixel_count > 160
  and .headline.rts_unit_status_role_badge_pixel_count > 600
  and .headline.rts_unit_status_queue_badge_pixel_count > 500
  and .headline.rts_selection_command_feedback_marquee_pixel_count > 350
  and .headline.rts_selection_command_feedback_confirm_pixel_count > 260
  and .headline.rts_selection_command_feedback_rally_pixel_count > 280
  and .headline.rts_selection_command_feedback_move_pixel_count > 300
  and .headline.rts_selection_command_feedback_attack_pixel_count > 320
  and .headline.rts_selection_command_feedback_error_pixel_count > 420
  and .headline.rts_selection_command_feedback_ack_pixel_count > 240
  and .headline.rts_ability_tooltip_telegraph_accepted_input_count == 6
  and .headline.rts_ability_tooltip_telegraph_ability_count >= 6
  and .headline.rts_ability_tooltip_telegraph_cooldown_count >= 6
  and .headline.rts_ability_tooltip_telegraph_queue_count >= 4
  and .headline.rts_ability_tooltip_telegraph_tooltip_pixel_count > 900
  and .headline.rts_ability_tooltip_telegraph_range_pixel_count > 500
  and .headline.rts_ability_tooltip_telegraph_windup_pixel_count > 600
  and .headline.rts_ability_tooltip_telegraph_cooldown_pixel_count > 450
  and .headline.rts_ability_tooltip_telegraph_queue_pixel_count > 700
  and .headline.rts_ability_tooltip_telegraph_warning_pixel_count > 900
  and .headline.rts_control_group_hotkey_feedback_accepted_input_count == 6
  and .headline.rts_control_group_hotkey_feedback_group_count >= 4
  and .headline.rts_control_group_hotkey_feedback_queue_count >= 4
  and .headline.rts_control_group_hotkey_feedback_assign_pixel_count > 1000
  and .headline.rts_control_group_hotkey_feedback_recall_pixel_count > 450
  and .headline.rts_control_group_hotkey_feedback_camera_pixel_count > 900
  and .headline.rts_control_group_hotkey_feedback_idle_pixel_count > 900
  and .headline.rts_control_group_hotkey_feedback_production_pixel_count > 700
  and .headline.rts_control_group_hotkey_feedback_ability_pixel_count > 700
  and .headline.rts_scrollable_map_input_action_count == 6
  and .headline.rts_scrollable_map_camera_frame_pixel_count > 4000
  and .headline.rts_scrollable_map_edge_pixel_count > 1000
  and .headline.rts_scrollable_map_drag_pixel_count > 250
  and .headline.rts_scrollable_map_zoom_pixel_count > 900
  and .headline.rts_scrollable_map_minimap_pixel_count > 600
  and .headline.rts_scrollable_map_clamp_pixel_count > 1000
  and .headline.rts_camera_minimap_sync_input_action_count == 6
  and .headline.rts_camera_minimap_sync_revealed_tile_union_count >= 12
  and .headline.rts_camera_minimap_sync_viewport_pixel_count > 2400
  and .headline.rts_camera_minimap_sync_fog_pixel_count > 8000
  and .headline.rts_camera_minimap_sync_reveal_pixel_count > 800
  and .headline.rts_camera_minimap_sync_selection_pixel_count > 1000
  and .headline.rts_camera_minimap_sync_route_pixel_count > 900
  and .headline.rts_camera_minimap_sync_stage_summary_count == 6
  and .headline.rts_camera_minimap_sync_stage_name_count == 6
  and .headline.rts_camera_minimap_sync_input_source_count == 6
  and .headline.rts_camera_minimap_sync_large_map_field_count == 5
  and .headline.rts_camera_minimap_sync_gate_count == 16
  and .headline.rts_camera_minimap_sync_passed_gate_count == 16
  and .headline.rts_camera_minimap_sync_failed_gate_count == 0
  and .headline.rts_command_queue_path_preview_accepted_input_count == 6
  and .headline.rts_command_queue_path_preview_final_queue_count >= 12
  and .headline.rts_command_queue_path_preview_queue_slot_pixel_count > 1200
  and .headline.rts_command_queue_path_preview_path_pixel_count > 400
  and .headline.rts_command_queue_path_preview_waypoint_pixel_count > 400
  and .headline.rts_command_queue_path_preview_target_pixel_count > 300
  and .headline.rts_command_queue_path_preview_reservation_pixel_count > 250
  and .headline.rts_command_queue_path_preview_cancel_pixel_count > 250
  and .headline.rts_formation_move_preview_accepted_input_count == 6
  and .headline.rts_formation_move_preview_final_slot_count >= 4
  and .headline.rts_formation_move_preview_ghost_pixel_count > 1200
  and .headline.rts_formation_move_preview_path_pixel_count > 500
  and .headline.rts_formation_move_preview_slot_pixel_count > 250
  and .headline.rts_formation_move_preview_collision_pixel_count > 250
  and .headline.rts_formation_move_preview_disperse_pixel_count > 120
  and .headline.rts_formation_move_preview_commit_pixel_count > 160
  and .headline.rts_formation_move_execution_accepted_input_count == 6
  and .headline.rts_formation_move_execution_final_slot_count >= 4
  and .headline.rts_formation_move_execution_slot_pixel_count > 600
  and .headline.rts_formation_move_execution_reservation_pixel_count > 500
  and .headline.rts_formation_move_execution_step_pixel_count > 160
  and .headline.rts_formation_move_execution_avoidance_pixel_count > 220
  and .headline.rts_formation_move_execution_reroute_pixel_count > 280
  and .headline.rts_formation_move_execution_arrival_pixel_count > 250
  and .headline.rts_local_obstruction_recovery_accepted_input_count == 5
  and .headline.rts_local_obstruction_recovery_final_route_count >= 5
  and .headline.rts_local_obstruction_recovery_block_pixel_count > 180
  and .headline.rts_local_obstruction_recovery_queue_pixel_count > 220
  and .headline.rts_local_obstruction_recovery_side_step_pixel_count > 160
  and .headline.rts_local_obstruction_recovery_gap_pixel_count > 180
  and .headline.rts_local_obstruction_recovery_resume_pixel_count > 160
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
  and .gates.rts_live_input_waypoint_live_gate == true
  and .gates.rts_live_input_hold_live_gate == true
  and .gates.rts_live_input_patrol_live_gate == true
  and .gates.rts_live_input_attack_move_live_gate == true
  and .gates.rts_live_input_stop_live_gate == true
  and .gates.rts_live_input_attack_live_gate == true
  and .gates.rts_live_input_ability_live_gate == true
  and .gates.rts_live_input_command_feedback_chip_gate == true
  and .gates.rts_live_input_command_queue_path_preview_shift_waypoints_gate == true
  and .gates.rts_live_input_command_queue_path_preview_queue_stack_gate == true
  and .gates.rts_live_input_command_queue_path_preview_rally_chain_gate == true
  and .gates.rts_live_input_command_queue_path_preview_attack_focus_gate == true
  and .gates.rts_live_input_command_queue_path_preview_cancel_repath_gate == true
  and .gates.rts_live_input_command_queue_path_preview_gate == true
  and .gates.rts_live_input_hover_preview_gate == true
  and .gates.rts_live_input_context_cursor_gate == true
  and .gates.rts_live_input_drag_select_preview_gate == true
  and .gates.rts_live_input_drag_select_commit_gate == true
  and .gates.rts_live_input_drag_select_filter_gate == true
  and .gates.rts_live_input_unit_click_select_gate == true
  and .gates.rts_live_input_selection_clear_gate == true
  and .gates.rts_live_input_right_click_target_gate == true
  and .gates.rts_live_input_right_click_target_preview_gate == true
  and .gates.rts_live_input_right_click_execution_feedback_gate == true
  and .gates.rts_live_input_right_click_execution_feedback_player_label_gate == true
  and .gates.rts_live_input_rts_core_frame_order_gate == true
  and .gates.rts_live_input_rts_core_headless_replay_gate == true
  and .gates.rts_live_input_unit_shift_select_gate == true
  and .gates.rts_live_input_unit_double_click_select_gate == true
  and .gates.rts_live_input_control_group_hotkey_gate == true
  and .gates.rts_live_input_control_group_slot_visual_gate == true
  and .gates.rts_live_input_command_stamp_gate == true
  and .gates.rts_pathing_live_input_gate == true
  and .gates.rts_pathing_path_tile_gate == true
  and .gates.rts_pathing_blocked_tile_gate == true
  and .gates.rts_pathing_formation_slot_gate == true
  and .gates.rts_pathing_command_visual_gate == true
  and .gates.rts_pathing_core_frame_order_gate == true
  and .gates.rts_pathing_core_headless_replay_gate == true
  and .gates.rts_collision_live_input_gate == true
  and .gates.rts_collision_collision_response_gate == true
  and .gates.rts_collision_engagement_response_gate == true
  and .gates.rts_collision_core_frame_order_gate == true
  and .gates.rts_collision_core_headless_replay_gate == true
  and .gates.rts_targeting_live_input_gate == true
  and .gates.rts_targeting_target_priority_gate == true
  and .gates.rts_targeting_aggro_gate == true
  and .gates.rts_targeting_focus_fire_gate == true
  and .gates.rts_targeting_threat_feedback_gate == true
  and .gates.rts_targeting_core_frame_order_gate == true
  and .gates.rts_targeting_core_headless_replay_gate == true
  and .gates.rts_economy_live_input_gate == true
  and .gates.rts_economy_harvest_loop_gate == true
  and .gates.rts_economy_build_loop_gate == true
  and .gates.rts_economy_production_loop_gate == true
  and .gates.rts_economy_core_frame_order_gate == true
  and .gates.rts_economy_core_headless_replay_gate == true
  and .gates.rts_selection_minimap_live_input_gate == true
  and .gates.rts_selection_box_gate == true
  and .gates.rts_control_group_gate == true
  and .gates.rts_minimap_command_gate == true
  and .gates.rts_split_route_gate == true
  and .gates.rts_selection_minimap_core_frame_order_gate == true
  and .gates.rts_selection_minimap_core_headless_replay_gate == true
  and .gates.rts_build_lifecycle_live_input_gate == true
  and .gates.rts_build_lifecycle_build_placement_gate == true
  and .gates.rts_build_lifecycle_completion_gate == true
  and .gates.rts_build_lifecycle_repair_gate == true
  and .gates.rts_build_lifecycle_cancel_refund_gate == true
  and .gates.rts_build_lifecycle_core_frame_order_gate == true
  and .gates.rts_build_lifecycle_core_headless_replay_gate == true
  and .gates.rts_tech_tree_live_input_gate == true
  and .gates.rts_tech_tree_faction_base_gate == true
  and .gates.rts_tech_tree_research_gate == true
  and .gates.rts_tech_tree_upgrade_gate == true
  and .gates.rts_tech_tree_unlock_gate == true
  and .gates.rts_tech_tree_dependency_gate == true
  and .gates.rts_tech_tree_core_frame_order_gate == true
  and .gates.rts_tech_tree_core_headless_replay_gate == true
  and .gates.rts_projectile_ability_live_input_gate == true
  and .gates.rts_projectile_ability_projectile_trail_gate == true
  and .gates.rts_projectile_ability_projectile_impact_gate == true
  and .gates.rts_projectile_ability_ability_radius_gate == true
  and .gates.rts_projectile_ability_damage_tick_gate == true
  and .gates.rts_projectile_ability_armor_shield_gate == true
  and .gates.rts_projectile_ability_core_frame_order_gate == true
  and .gates.rts_projectile_ability_core_headless_replay_gate == true
  and .gates.rts_ai_skirmish_pressure_live_input_gate == true
  and .gates.rts_ai_skirmish_pressure_ai_wave_gate == true
  and .gates.rts_ai_skirmish_pressure_ai_counter_gate == true
  and .gates.rts_ai_skirmish_pressure_resolution_gate == true
  and .gates.rts_ai_skirmish_pressure_retreat_gate == true
  and .gates.rts_ai_skirmish_player_response_gate == true
  and .gates.rts_ai_skirmish_pressure_core_frame_order_gate == true
  and .gates.rts_ai_skirmish_pressure_core_headless_replay_gate == true
  and .gates.rts_objective_victory_loop_live_input_gate == true
  and .gates.rts_objective_victory_loop_marker_gate == true
  and .gates.rts_objective_victory_loop_capture_gate == true
  and .gates.rts_objective_victory_loop_victory_gate == true
  and .gates.rts_objective_victory_loop_defeat_pressure_gate == true
  and .gates.rts_objective_victory_loop_extraction_gate == true
  and .gates.rts_objective_victory_loop_openra_parity_bridge_gate == true
  and .gates.rts_objective_victory_loop_core_frame_order_gate == true
  and .gates.rts_objective_victory_loop_core_headless_replay_gate == true
  and .gates.rts_autonomous_bot_skirmish_no_live_player_input_gate == true
  and .gates.rts_autonomous_bot_skirmish_timeline_gate == true
  and .gates.rts_autonomous_bot_skirmish_bot_roster_gate == true
  and .gates.rts_autonomous_bot_skirmish_economy_gate == true
  and .gates.rts_autonomous_bot_skirmish_production_gate == true
  and .gates.rts_autonomous_bot_skirmish_combat_gate == true
  and .gates.rts_autonomous_bot_skirmish_terminal_gate == true
  and .gates.rts_autonomous_bot_skirmish_renderer_gate == true
  and .gates.rts_autonomous_bot_skirmish_gate == true
  and .gates.rts_organic_terminal_gap_stage_gate == true
  and .gates.rts_organic_terminal_gap_observation_report_gate == true
  and .gates.rts_organic_terminal_gap_openra_target_gate == true
  and .gates.rts_organic_terminal_gap_bevy_gap_gate == true
  and .gates.rts_organic_terminal_gap_renderer_gate == true
  and .gates.rts_organic_terminal_gap_openra_gap_not_closed_gate == true
  and .gates.rts_organic_terminal_gap_gate == true
  and .gates.rts_terminal_observation_gap_stage_gate == true
  and .gates.rts_terminal_observation_gap_readiness_gate == true
  and .gates.rts_terminal_observation_gap_observation_gate == true
  and .gates.rts_terminal_observation_gap_openra_target_gate == true
  and .gates.rts_terminal_observation_gap_bevy_gap_gate == true
  and .gates.rts_terminal_observation_gap_renderer_gate == true
  and .gates.rts_terminal_observation_gap_openra_gap_not_closed_gate == true
  and .gates.rts_terminal_observation_gap_gate == true
  and .gates.rts_replay_metrics_gap_stage_gate == true
  and .gates.rts_replay_metrics_gap_roster_gate == true
  and .gates.rts_replay_metrics_gap_token_gate == true
  and .gates.rts_replay_metrics_gap_battle_outcome_summary_gate == true
  and .gates.rts_replay_metrics_gap_bevy_gap_gate == true
  and .gates.rts_replay_metrics_gap_openra_target_gate == true
  and .gates.rts_replay_metrics_gap_renderer_gate == true
  and .gates.rts_replay_metrics_gap_openra_gap_not_closed_gate == true
  and .gates.rts_replay_metrics_gap_gate == true
  and .gates.rts_endurance_skirmish_gap_stage_gate == true
  and .gates.rts_endurance_skirmish_gap_roster_gate == true
  and .gates.rts_endurance_skirmish_gap_duration_gate == true
  and .gates.rts_endurance_skirmish_gap_pressure_gate == true
  and .gates.rts_endurance_skirmish_gap_battle_outcome_gate == true
  and .gates.rts_endurance_skirmish_gap_bevy_gap_gate == true
  and .gates.rts_endurance_skirmish_gap_openra_target_gate == true
  and .gates.rts_endurance_skirmish_gap_renderer_gate == true
  and .gates.rts_endurance_skirmish_gap_openra_gap_not_closed_gate == true
  and .gates.rts_endurance_skirmish_gap_gate == true
  and .gates.rts_bot_decision_state_gap_stage_gate == true
  and .gates.rts_bot_decision_state_gap_signal_gate == true
  and .gates.rts_bot_decision_state_gap_economy_gate == true
  and .gates.rts_bot_decision_state_gap_scout_gate == true
  and .gates.rts_bot_decision_state_gap_capture_gate == true
  and .gates.rts_bot_decision_state_gap_tech_gate == true
  and .gates.rts_bot_decision_state_gap_counter_gate == true
  and .gates.rts_bot_decision_state_gap_attack_gate == true
  and .gates.rts_bot_decision_state_gap_retreat_gate == true
  and .gates.rts_bot_decision_state_gap_bevy_gap_gate == true
  and .gates.rts_bot_decision_state_gap_openra_target_gate == true
  and .gates.rts_bot_decision_state_gap_renderer_gate == true
  and .gates.rts_bot_decision_state_gap_openra_gap_not_closed_gate == true
  and .gates.rts_bot_decision_state_gap_core_frame_order_gate == true
  and .gates.rts_bot_decision_state_gap_core_headless_replay_gate == true
  and .gates.rts_bot_decision_state_gap_gate == true
  and .gates.rts_bot_adaptive_build_order_gap_stage_gate == true
  and .gates.rts_bot_adaptive_build_order_gap_signal_gate == true
  and .gates.rts_bot_adaptive_build_order_gap_opening_gate == true
  and .gates.rts_bot_adaptive_build_order_gap_scout_gate == true
  and .gates.rts_bot_adaptive_build_order_gap_branch_gate == true
  and .gates.rts_bot_adaptive_build_order_gap_tech_gate == true
  and .gates.rts_bot_adaptive_build_order_gap_pressure_gate == true
  and .gates.rts_bot_adaptive_build_order_gap_retreat_rebuild_gate == true
  and .gates.rts_bot_adaptive_build_order_gap_bevy_gap_gate == true
  and .gates.rts_bot_adaptive_build_order_gap_openra_target_gate == true
	  and .gates.rts_bot_adaptive_build_order_gap_renderer_gate == true
	  and .gates.rts_bot_adaptive_build_order_gap_openra_gap_not_closed_gate == true
	  and .gates.rts_bot_adaptive_build_order_gap_core_frame_order_gate == true
	  and .gates.rts_bot_adaptive_build_order_gap_core_headless_replay_gate == true
	  and .gates.rts_bot_adaptive_build_order_gap_gate == true
  and .gates.rts_bot_tactical_micro_gap_stage_gate == true
  and .gates.rts_bot_tactical_micro_gap_signal_gate == true
  and .gates.rts_bot_tactical_micro_gap_target_gate == true
  and .gates.rts_bot_tactical_micro_gap_focus_gate == true
  and .gates.rts_bot_tactical_micro_gap_kite_gate == true
  and .gates.rts_bot_tactical_micro_gap_flank_gate == true
  and .gates.rts_bot_tactical_micro_gap_ability_gate == true
  and .gates.rts_bot_tactical_micro_gap_pullback_gate == true
  and .gates.rts_bot_tactical_micro_gap_bevy_gap_gate == true
  and .gates.rts_bot_tactical_micro_gap_openra_target_gate == true
  and .gates.rts_bot_tactical_micro_gap_renderer_gate == true
  and .gates.rts_bot_tactical_micro_gap_openra_gap_not_closed_gate == true
  and .gates.rts_bot_tactical_micro_gap_core_frame_order_gate == true
  and .gates.rts_bot_tactical_micro_gap_core_headless_replay_gate == true
  and .gates.rts_bot_tactical_micro_gap_gate == true
  and .gates.rts_bot_map_intel_gap_stage_gate == true
  and .gates.rts_bot_map_intel_gap_signal_gate == true
  and .gates.rts_bot_map_intel_gap_scout_gate == true
  and .gates.rts_bot_map_intel_gap_fog_memory_gate == true
  and .gates.rts_bot_map_intel_gap_expansion_gate == true
  and .gates.rts_bot_map_intel_gap_tech_gate == true
  and .gates.rts_bot_map_intel_gap_hidden_army_gate == true
  and .gates.rts_bot_map_intel_gap_rotation_gate == true
  and .gates.rts_bot_map_intel_gap_bevy_gap_gate == true
  and .gates.rts_bot_map_intel_gap_openra_target_gate == true
  and .gates.rts_bot_map_intel_gap_renderer_gate == true
  and .gates.rts_bot_map_intel_gap_openra_gap_not_closed_gate == true
  and .gates.rts_bot_map_intel_gap_core_frame_order_gate == true
  and .gates.rts_bot_map_intel_gap_core_headless_replay_gate == true
  and .gates.rts_bot_map_intel_gap_gate == true
  and .gates.rts_bot_macro_economy_gap_stage_gate == true
  and .gates.rts_bot_macro_economy_gap_signal_gate == true
  and .gates.rts_bot_macro_economy_gap_worker_gate == true
  and .gates.rts_bot_macro_economy_gap_expand_gate == true
  and .gates.rts_bot_macro_economy_gap_supply_gate == true
  and .gates.rts_bot_macro_economy_gap_production_gate == true
  and .gates.rts_bot_macro_economy_gap_tech_gate == true
  and .gates.rts_bot_macro_economy_gap_deny_rebuild_gate == true
  and .gates.rts_bot_macro_economy_gap_bevy_gap_gate == true
  and .gates.rts_bot_macro_economy_gap_openra_target_gate == true
  and .gates.rts_bot_macro_economy_gap_renderer_gate == true
  and .gates.rts_bot_macro_economy_gap_openra_gap_not_closed_gate == true
  and .gates.rts_bot_macro_economy_gap_core_frame_order_gate == true
  and .gates.rts_bot_macro_economy_gap_core_headless_replay_gate == true
  and .gates.rts_bot_macro_economy_gap_gate == true
  and .gates.rts_bot_harassment_defense_gap_stage_gate == true
  and .gates.rts_bot_harassment_defense_gap_signal_gate == true
  and .gates.rts_bot_harassment_defense_gap_worker_gate == true
  and .gates.rts_bot_harassment_defense_gap_repair_gate == true
  and .gates.rts_bot_harassment_defense_gap_static_defense_gate == true
  and .gates.rts_bot_harassment_defense_gap_counter_raid_gate == true
  and .gates.rts_bot_harassment_defense_gap_retreat_gate == true
  and .gates.rts_bot_harassment_defense_gap_rebuild_gate == true
  and .gates.rts_bot_harassment_defense_gap_bevy_gap_gate == true
  and .gates.rts_bot_harassment_defense_gap_openra_target_gate == true
  and .gates.rts_bot_harassment_defense_gap_renderer_gate == true
  and .gates.rts_bot_harassment_defense_gap_openra_gap_not_closed_gate == true
  and .gates.rts_bot_harassment_defense_gap_core_frame_order_gate == true
  and .gates.rts_bot_harassment_defense_gap_core_headless_replay_gate == true
  and .gates.rts_bot_harassment_defense_gap_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_stage_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_signal_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_split_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_decoy_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_rotation_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_reinforce_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_simultaneous_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_terminal_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_bevy_gap_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_openra_target_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_renderer_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_openra_gap_not_closed_gate == true
  and .gates.rts_bot_multi_front_pressure_gap_gate == true
  and .gates.rts_bot_expansion_control_gap_stage_gate == true
  and .gates.rts_bot_expansion_control_gap_signal_gate == true
  and .gates.rts_bot_expansion_control_gap_natural_gate == true
  and .gates.rts_bot_expansion_control_gap_third_node_gate == true
  and .gates.rts_bot_expansion_control_gap_refinery_gate == true
  and .gates.rts_bot_expansion_control_gap_contain_gate == true
  and .gates.rts_bot_expansion_control_gap_reexpand_gate == true
  and .gates.rts_bot_expansion_control_gap_lock_gate == true
  and .gates.rts_bot_expansion_control_gap_bevy_gap_gate == true
  and .gates.rts_bot_expansion_control_gap_openra_target_gate == true
  and .gates.rts_bot_expansion_control_gap_renderer_gate == true
  and .gates.rts_bot_expansion_control_gap_openra_gap_not_closed_gate == true
  and .gates.rts_bot_expansion_control_gap_gate == true
  and .gates.rts_bot_tech_transition_gap_stage_gate == true
  and .gates.rts_bot_tech_transition_gap_signal_gate == true
  and .gates.rts_bot_tech_transition_gap_signal_read_gate == true
  and .gates.rts_bot_tech_transition_gap_counter_gate == true
  and .gates.rts_bot_tech_transition_gap_anti_air_gate == true
  and .gates.rts_bot_tech_transition_gap_siege_gate == true
  and .gates.rts_bot_tech_transition_gap_upgrade_gate == true
  and .gates.rts_bot_tech_transition_gap_terminal_gate == true
  and .gates.rts_bot_tech_transition_gap_bevy_gap_gate == true
  and .gates.rts_bot_tech_transition_gap_openra_target_gate == true
  and .gates.rts_bot_tech_transition_gap_renderer_gate == true
  and .gates.rts_bot_tech_transition_gap_openra_gap_not_closed_gate == true
  and .gates.rts_bot_tech_transition_gap_gate == true
  and .gates.rts_bot_army_composition_gap_stage_gate == true
  and .gates.rts_bot_army_composition_gap_signal_gate == true
  and .gates.rts_bot_army_composition_gap_unit_mix_gate == true
  and .gates.rts_bot_army_composition_gap_ratio_gate == true
  and .gates.rts_bot_army_composition_gap_counter_gate == true
  and .gates.rts_bot_army_composition_gap_reinforce_gate == true
  and .gates.rts_bot_army_composition_gap_specialist_gate == true
  and .gates.rts_bot_army_composition_gap_lock_gate == true
  and .gates.rts_bot_army_composition_gap_bevy_gap_gate == true
  and .gates.rts_bot_army_composition_gap_openra_target_gate == true
  and .gates.rts_bot_army_composition_gap_renderer_gate == true
  and .gates.rts_bot_army_composition_gap_openra_gap_not_closed_gate == true
  and .gates.rts_bot_army_composition_gap_gate == true
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
  and .gates.rts_fog_scouting_intel_core_frame_order_gate == true
  and .gates.rts_fog_scouting_intel_core_headless_replay_gate == true
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
  and .gates.rts_open_world_after_action_runtime_screen_gate == true
  and .gates.rts_open_world_after_action_player_first_screen_gate == true
  and .headline.rts_open_world_after_action_runtime_screen_mode == "player_runtime_open_world_after_action_screen"
  and .headline.rts_open_world_after_action_player_first_view_non_background > 250000
  and .headline.rts_open_world_after_action_player_first_view_frame_pixel_count > 8000
  and .headline.rts_open_world_after_action_player_first_status_strip_pixel_count > 20000
  and .headline.rts_open_world_after_action_player_first_route_panel_pixel_count > 90000
  and .headline.rts_open_world_after_action_player_first_timeline_pixel_count > 10000
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
  and .gates.rts_match_setup_ui_replication_shell_meta_gate == true
  and .gates.rts_match_setup_ui_replication_campaign_entry_gate == true
  and .gates.rts_match_setup_ui_replication_map_spec_gate == true
  and .gates.rts_match_setup_ui_replication_map_ui_gate == true
  and .gates.rts_match_setup_ui_replication_faction_gate == true
  and .gates.rts_match_setup_ui_replication_no_external_boundary_gate == true
  and .gates.rts_match_setup_ui_replication_player_first_screen_gate == true
  and .gates.rts_match_setup_ui_replication_gate == true
  and .gates.rts_first_contact_basin_spec_gate == true
  and .gates.rts_first_contact_runtime_review_gate == true
  and .gates.rts_first_contact_runtime_adapter_evidence_gate == true
  and .gates.rts_first_contact_offline_adapter_consumption_gate == true
  and .gates.rts_first_contact_offline_adapter_session_transition_gate == true
  and .gates.rts_first_contact_offline_adapter_lobby_ready_gate == true
  and .gates.rts_campaign_outcome_ui_readiness_runtime_screen_gate == true
  and .gates.rts_campaign_outcome_ui_readiness_first_minute_gate == true
  and .gates.rts_campaign_outcome_ui_readiness_objective_victory_gate == true
  and .gates.rts_campaign_outcome_ui_readiness_base_assault_gate == true
  and .gates.rts_campaign_outcome_ui_readiness_battle_aftermath_gate == true
  and .gates.rts_campaign_outcome_ui_readiness_open_world_return_gate == true
  and .gates.rts_campaign_outcome_ui_readiness_player_first_screen_gate == true
  and .gates.rts_campaign_outcome_ui_readiness_gate == true
  and .gates.rts_campaign_ui_continuity_handoff_green_gate == true
  and .gates.rts_campaign_ui_continuity_preview_resolution_gate == true
  and .gates.rts_campaign_ui_continuity_live_input_gate == true
  and .gates.rts_campaign_ui_continuity_milestone_gate == true
  and .gates.rts_campaign_ui_continuity_map_ui_state_gate == true
  and .gates.rts_campaign_ui_continuity_restored_ui_state_gate == true
  and .gates.rts_campaign_ui_continuity_persistence_gate == true
  and .gates.rts_campaign_ui_continuity_render_readability_gate == true
  and .gates.rts_campaign_ui_continuity_native_client_boundary_gate == true
  and .gates.rts_in_match_hud_state_replication_selection_gate == true
  and .gates.rts_in_match_hud_state_replication_command_gate == true
  and .gates.rts_in_match_hud_state_replication_resource_gate == true
  and .gates.rts_in_match_hud_state_replication_production_gate == true
  and .gates.rts_in_match_hud_state_replication_ability_gate == true
  and .gates.rts_in_match_hud_state_replication_combat_alert_gate == true
  and .gates.rts_in_match_hud_state_replication_minimap_objective_gate == true
  and .gates.rts_in_match_hud_state_replication_native_client_boundary_gate == true
  and .gates.rts_in_match_hud_state_replication_player_first_screen_gate == true
  and .gates.rts_in_match_hud_state_replication_gate == true
  and .gates.rts_session_state_continuity_shell_meta_gate == true
  and .gates.rts_session_state_continuity_session_slot_confirm_gate == true
  and .gates.rts_session_state_continuity_session_load_resume_gate == true
  and .gates.rts_session_state_continuity_session_recovery_gate == true
  and .gates.rts_session_state_continuity_match_setup_gate == true
  and .gates.rts_session_state_continuity_hud_restore_gate == true
  and .gates.rts_session_state_continuity_campaign_outcome_gate == true
  and .gates.rts_session_state_continuity_campaign_continuity_gate == true
  and .gates.rts_session_state_continuity_chain_gate == true
  and .gates.rts_session_state_continuity_native_client_boundary_gate == true
  and .gates.rts_session_state_continuity_player_first_session_resume_screen_gate == true
  and .gates.rts_session_state_continuity_gate == true
  and .gates.rts_continuous_player_flow_title_account_gate == true
  and .gates.rts_continuous_player_flow_match_setup_gate == true
  and .gates.rts_continuous_player_flow_in_match_hud_gate == true
  and .gates.rts_continuous_player_flow_command_feedback_gate == true
  and .gates.rts_continuous_player_flow_save_resume_gate == true
  and .gates.rts_continuous_player_flow_outcome_open_world_gate == true
  and .gates.rts_continuous_player_flow_chain_gate == true
  and .gates.rts_continuous_player_flow_player_first_continuous_flow_screen_gate == true
  and .gates.rts_continuous_player_flow_native_client_boundary_gate == true
  and .gates.rts_continuous_player_flow_gate == true
  and .gates.rts_continuous_player_flow_rts_evidence_review_gate == true
  and .gates.rts_live_session_playthrough_title_account_gate == true
  and .gates.rts_live_session_playthrough_match_setup_gate == true
  and .gates.rts_live_session_playthrough_in_match_hud_gate == true
  and .gates.rts_live_session_playthrough_command_feedback_gate == true
  and .gates.rts_live_session_playthrough_save_resume_gate == true
  and .gates.rts_live_session_playthrough_outcome_open_world_gate == true
  and .gates.rts_live_session_playthrough_same_process_trace_gate == true
  and .gates.rts_live_session_playthrough_player_first_live_session_screen_gate == true
  and .gates.rts_live_session_playthrough_runtime_screen_gate == true
  and .gates.rts_live_session_playthrough_native_client_boundary_gate == true
  and .gates.rts_live_session_playthrough_rts_evidence_review_gate == true
  and .gates.rts_live_session_playthrough_gate == true
  and .gates.rts_full_game_visual_ui_replication_source_contract_gate == true
  and .gates.rts_full_game_visual_ui_replication_source_green_gate == true
  and .gates.rts_full_game_visual_ui_replication_runtime_screen_chain_gate == true
  and .gates.rts_full_game_visual_ui_replication_runtime_screen_gate == true
  and .gates.rts_full_game_visual_ui_replication_player_flow_gate == true
  and .gates.rts_full_game_visual_ui_replication_coverage_surface_gate == true
  and .gates.rts_full_game_visual_ui_replication_preview_gate == true
  and .gates.rts_full_game_visual_ui_replication_player_first_tactical_composition_gate == true
  and .gates.rts_full_game_visual_ui_replication_command_grid_readability_gate == true
  and .gates.rts_full_game_visual_ui_replication_player_first_screen_gate == true
  and .gates.rts_full_game_visual_ui_replication_no_copy_boundary_gate == true
  and .gates.rts_full_game_visual_ui_replication_rts_evidence_review_gate == true
  and .gates.rts_full_game_visual_ui_replication_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_source_contract_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_source_green_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_runtime_vocabulary_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_widget_root_reference_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_screen_set_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_source_screen_chain_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_preview_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_no_asset_copy_boundary_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_player_first_ingame_screen_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_style_screen_set_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_gate == true
  and .gates.rts_openra_screen_for_screen_ui_replication_rts_evidence_review_gate == true
  and .gates.rts_openra_engine_port_asset_parity_source_contract_gate == true
  and .gates.rts_openra_engine_port_asset_parity_source_green_gate == true
  and .gates.rts_openra_engine_port_asset_parity_engine_module_gate == true
  and .gates.rts_openra_engine_port_asset_parity_rules_mod_port_gate == true
  and .gates.rts_openra_engine_port_asset_parity_chrome_widget_port_gate == true
  and .gates.rts_openra_engine_port_asset_parity_asset_loader_port_gate == true
  and .gates.rts_openra_engine_port_asset_parity_pixel_perfect_gate == true
  and .gates.rts_openra_engine_port_asset_parity_write_gate == true
  and .gates.rts_openra_engine_port_asset_parity_no_copy_boundary_gate == true
  and .gates.rts_openra_engine_port_asset_parity_gate == true
  and .gates.rts_visual_fidelity_mature_hud_gate == true
  and .gates.rts_visual_fidelity_selected_units_gate == true
  and .gates.rts_visual_fidelity_command_surface_gate == true
  and .gates.rts_visual_fidelity_model_gate == true
  and .gates.rts_visual_fidelity_npc_animation_gate == true
  and .gates.rts_visual_fidelity_original_art_policy_gate == true
  and .gates.rts_production_asset_atlas_sprite_sheet_gate == true
  and .gates.rts_production_asset_atlas_texture_atlas_binding_gate == true
  and .gates.rts_production_asset_atlas_runtime_texture_asset_gate == true
  and .gates.rts_production_asset_atlas_preview_gate == true
  and .gates.rts_production_asset_atlas_gate == true
  and .gates.rts_production_asset_atlas_no_copy_boundary_gate == true
  and .gates.rts_production_ui_skin_asset_atlas_gate == true
  and .gates.rts_production_ui_skin_command_surface_skin_gate == true
  and .gates.rts_production_ui_skin_selection_minimap_skin_gate == true
  and .gates.rts_production_ui_skin_unit_status_skin_gate == true
  and .gates.rts_production_ui_skin_command_feedback_skin_gate == true
  and .gates.rts_production_ui_skin_tooltip_skin_gate == true
  and .gates.rts_production_ui_skin_hotkey_skin_gate == true
  and .gates.rts_production_ui_skin_preview_gate == true
  and .gates.rts_production_ui_skin_player_first_screen_gate == true
  and .gates.rts_production_ui_skin_source_preview_gate == true
  and .gates.rts_production_ui_skin_no_copy_boundary_gate == true
  and .gates.rts_production_ui_skin_gate == true
  and .gates.rts_production_interaction_polish_ui_skin_gate == true
  and .gates.rts_production_interaction_polish_command_affordance_gate == true
  and .gates.rts_production_interaction_polish_selection_feedback_gate == true
  and .gates.rts_production_interaction_polish_build_lifecycle_gate == true
  and .gates.rts_production_interaction_polish_scrollable_map_gate == true
  and .gates.rts_production_interaction_polish_command_queue_path_gate == true
  and .gates.rts_production_interaction_polish_preview_gate == true
  and .gates.rts_production_interaction_polish_player_first_screen_gate == true
  and .gates.rts_production_interaction_polish_source_preview_gate == true
  and .gates.rts_production_interaction_polish_no_copy_boundary_gate == true
  and .gates.rts_production_interaction_polish_gate == true
  and .gates.rts_full_screen_ui_replication_player_first_screen_gate == true
  and .gates.rts_full_screen_ui_replication_gate == true
  and .gates.rts_shell_meta_ui_replication_full_screen_gate == true
  and .gates.rts_shell_meta_ui_replication_account_title_gate == true
  and .gates.rts_shell_meta_ui_replication_title_menu_gate == true
  and .gates.rts_shell_meta_ui_replication_character_create_gate == true
  and .gates.rts_shell_meta_ui_replication_session_slot_menu_gate == true
  and .gates.rts_shell_meta_ui_replication_session_save_slot_gate == true
  and .gates.rts_shell_meta_ui_replication_session_slot_confirm_gate == true
  and .gates.rts_shell_meta_ui_replication_session_load_resume_gate == true
  and .gates.rts_shell_meta_ui_replication_session_recovery_gate == true
  and .gates.rts_shell_meta_ui_replication_pause_menu_gate == true
  and .gates.rts_shell_meta_ui_replication_settings_menu_gate == true
  and .gates.rts_shell_meta_ui_replication_input_hud_gate == true
  and .gates.rts_shell_meta_ui_replication_visible_hit_test_gate == true
  and .gates.rts_shell_meta_ui_replication_first_minute_gate == true
  and .gates.rts_shell_meta_ui_replication_player_first_screen_gate == true
  and .gates.rts_shell_meta_ui_replication_gate == true
  and .gates.rts_command_affordance_live_input_gate == true
  and .gates.rts_command_affordance_drag_select_gate == true
  and .gates.rts_command_affordance_right_click_move_gate == true
  and .gates.rts_command_affordance_attack_cursor_gate == true
  and .gates.rts_command_affordance_hotkey_ack_gate == true
  and .gates.rts_command_affordance_original_art_policy_gate == true
  and .gates.rts_command_surface_selection_surface_gate == true
  and .gates.rts_command_surface_command_grid_surface_gate == true
  and .gates.rts_command_surface_cooldown_disabled_surface_gate == true
  and .gates.rts_command_surface_target_queue_surface_gate == true
  and .gates.rts_command_surface_surface_stage_gate == true
  and .gates.rts_command_surface_scene_renderer_gate == true
  and .gates.rts_command_surface_original_art_policy_gate == true
  and .gates.rts_structure_modeling_foundation_gate == true
  and .gates.rts_structure_modeling_scaffold_gate == true
  and .gates.rts_structure_modeling_construction_spark_gate == true
  and .gates.rts_structure_modeling_production_glow_gate == true
  and .gates.rts_structure_modeling_damage_crack_gate == true
  and .gates.rts_structure_modeling_repair_beam_gate == true
  and .gates.rts_structure_modeling_structure_stage_gate == true
  and .gates.rts_structure_modeling_scene_renderer_gate == true
  and .gates.rts_structure_modeling_original_art_policy_gate == true
  and .gates.rts_environment_life_tree_sway_gate == true
  and .gates.rts_environment_life_torch_flicker_gate == true
  and .gates.rts_environment_life_water_shimmer_gate == true
  and .gates.rts_environment_life_banner_flutter_gate == true
  and .gates.rts_environment_life_resource_glint_gate == true
  and .gates.rts_environment_life_ambient_dust_gate == true
  and .gates.rts_environment_life_environment_stage_gate == true
  and .gates.rts_environment_life_scene_renderer_gate == true
  and .gates.rts_environment_life_original_art_policy_gate == true
  and .gates.rts_map_model_gap_lane_gate == true
  and .gates.rts_map_model_gap_resource_gate == true
  and .gates.rts_map_model_gap_height_gate == true
  and .gates.rts_map_model_gap_choke_gate == true
  and .gates.rts_map_model_gap_structure_silhouette_gate == true
  and .gates.rts_map_model_gap_unit_role_gate == true
  and .gates.rts_map_model_gap_occlusion_gate == true
  and .gates.rts_map_model_gap_stage_gate == true
  and .gates.rts_map_model_gap_map_topology_gate == true
  and .gates.rts_map_model_gap_model_readability_gate == true
  and .gates.rts_map_model_gap_scene_renderer_gate == true
  and .gates.rts_map_model_gap_openra_gap_not_closed_gate == true
  and .gates.rts_map_model_gap_original_art_policy_gate == true
  and .gates.rts_worker_harvest_animation_approach_gate == true
  and .gates.rts_worker_harvest_animation_tool_swing_gate == true
  and .gates.rts_worker_harvest_animation_resource_pop_gate == true
  and .gates.rts_worker_harvest_animation_carry_load_gate == true
  and .gates.rts_worker_harvest_animation_dropoff_burst_gate == true
  and .gates.rts_worker_harvest_animation_return_path_gate == true
  and .gates.rts_worker_harvest_animation_harvest_stage_gate == true
  and .gates.rts_worker_harvest_animation_economy_runtime_gate == true
  and .gates.rts_worker_harvest_animation_scene_renderer_gate == true
  and .gates.rts_worker_harvest_animation_original_art_policy_gate == true
  and .gates.rts_production_spawn_animation_queue_pulse_gate == true
  and .gates.rts_production_spawn_animation_training_tick_gate == true
  and .gates.rts_production_spawn_animation_spawn_door_gate == true
  and .gates.rts_production_spawn_animation_rally_flag_gate == true
  and .gates.rts_production_spawn_animation_formation_join_gate == true
  and .gates.rts_production_spawn_animation_supply_flash_gate == true
  and .gates.rts_production_spawn_animation_production_stage_gate == true
  and .gates.rts_production_spawn_animation_production_runtime_gate == true
  and .gates.rts_production_spawn_animation_scene_renderer_gate == true
  and .gates.rts_production_spawn_animation_original_art_policy_gate == true
  and .gates.rts_unit_status_portrait_frame_gate == true
  and .gates.rts_unit_status_health_bar_gate == true
  and .gates.rts_unit_status_mana_bar_gate == true
  and .gates.rts_unit_status_xp_bar_gate == true
  and .gates.rts_unit_status_buff_badge_gate == true
  and .gates.rts_unit_status_role_badge_gate == true
  and .gates.rts_unit_status_queue_badge_gate == true
  and .gates.rts_unit_status_status_stage_gate == true
  and .gates.rts_unit_status_status_runtime_gate == true
  and .gates.rts_unit_status_scene_renderer_gate == true
  and .gates.rts_unit_status_original_art_policy_gate == true
  and .gates.rts_selection_command_feedback_marquee_gate == true
  and .gates.rts_selection_command_feedback_confirm_gate == true
  and .gates.rts_selection_command_feedback_rally_gate == true
  and .gates.rts_selection_command_feedback_move_gate == true
  and .gates.rts_selection_command_feedback_attack_gate == true
  and .gates.rts_selection_command_feedback_error_gate == true
  and .gates.rts_selection_command_feedback_ack_gate == true
  and .gates.rts_selection_command_feedback_feedback_stage_gate == true
  and .gates.rts_selection_command_feedback_command_runtime_gate == true
  and .gates.rts_selection_command_feedback_scene_renderer_gate == true
  and .gates.rts_selection_command_feedback_original_art_policy_gate == true
  and .gates.rts_ability_tooltip_telegraph_tooltip_gate == true
  and .gates.rts_ability_tooltip_telegraph_range_gate == true
  and .gates.rts_ability_tooltip_telegraph_windup_gate == true
  and .gates.rts_ability_tooltip_telegraph_cooldown_gate == true
  and .gates.rts_ability_tooltip_telegraph_queue_gate == true
  and .gates.rts_ability_tooltip_telegraph_warning_gate == true
  and .gates.rts_ability_tooltip_telegraph_telegraph_stage_gate == true
  and .gates.rts_ability_tooltip_telegraph_ability_runtime_gate == true
  and .gates.rts_ability_tooltip_telegraph_scene_renderer_gate == true
  and .gates.rts_ability_tooltip_telegraph_original_art_policy_gate == true
  and .gates.rts_control_group_hotkey_feedback_assign_gate == true
  and .gates.rts_control_group_hotkey_feedback_recall_gate == true
  and .gates.rts_control_group_hotkey_feedback_camera_gate == true
  and .gates.rts_control_group_hotkey_feedback_idle_gate == true
  and .gates.rts_control_group_hotkey_feedback_production_gate == true
  and .gates.rts_control_group_hotkey_feedback_ability_gate == true
  and .gates.rts_control_group_hotkey_feedback_hotkey_stage_gate == true
  and .gates.rts_control_group_hotkey_feedback_hotkey_runtime_gate == true
  and .gates.rts_control_group_hotkey_feedback_scene_renderer_gate == true
  and .gates.rts_control_group_hotkey_feedback_original_art_policy_gate == true
  and .gates.rts_scrollable_map_keyboard_pan_gate == true
  and .gates.rts_scrollable_map_edge_scroll_gate == true
  and .gates.rts_scrollable_map_drag_pan_gate == true
  and .gates.rts_scrollable_map_wheel_zoom_gate == true
  and .gates.rts_scrollable_map_minimap_jump_gate == true
  and .gates.rts_scrollable_map_boundary_clamp_gate == true
  and .gates.rts_scrollable_map_map_layer_projection_gate == true
  and .gates.rts_scrollable_map_hud_fixed_gate == true
  and .gates.rts_scrollable_map_scene_renderer_gate == true
  and .gates.rts_scrollable_map_original_art_policy_gate == true
  and .gates.rts_camera_minimap_sync_viewport_sync_gate == true
  and .gates.rts_camera_minimap_sync_fog_reveal_gate == true
  and .gates.rts_camera_minimap_sync_selection_follow_gate == true
  and .gates.rts_camera_minimap_sync_control_group_sync_gate == true
  and .gates.rts_camera_minimap_sync_route_projection_gate == true
  and .gates.rts_camera_minimap_sync_zoom_rect_sync_gate == true
  and .gates.rts_camera_minimap_sync_minimap_runtime_gate == true
  and .gates.rts_camera_minimap_sync_scene_renderer_gate == true
  and .gates.rts_camera_minimap_sync_original_art_policy_gate == true
  and .gates.rts_command_queue_path_preview_live_input_gate == true
  and .gates.rts_command_queue_path_preview_queue_stack_gate == true
  and .gates.rts_command_queue_path_preview_shift_waypoint_gate == true
  and .gates.rts_command_queue_path_preview_rally_chain_gate == true
  and .gates.rts_command_queue_path_preview_attack_focus_gate == true
  and .gates.rts_command_queue_path_preview_build_reservation_gate == true
  and .gates.rts_command_queue_path_preview_cancel_repath_gate == true
  and .gates.rts_command_queue_path_preview_scene_renderer_gate == true
  and .gates.rts_command_queue_path_preview_original_art_policy_gate == true
  and .gates.rts_formation_move_preview_live_input_gate == true
  and .gates.rts_formation_move_preview_destination_ghost_gate == true
  and .gates.rts_formation_move_preview_wedge_spacing_gate == true
  and .gates.rts_formation_move_preview_line_reflow_gate == true
  and .gates.rts_formation_move_preview_collision_avoidance_gate == true
  and .gates.rts_formation_move_preview_split_avoidance_gate == true
  and .gates.rts_formation_move_preview_commit_spacing_gate == true
  and .gates.rts_formation_move_preview_scene_renderer_gate == true
  and .gates.rts_formation_move_preview_original_art_policy_gate == true
  and .gates.rts_formation_move_execution_live_input_gate == true
  and .gates.rts_formation_move_execution_slot_claim_gate == true
  and .gates.rts_formation_move_execution_path_reservation_gate == true
  and .gates.rts_formation_move_execution_stagger_step_gate == true
  and .gates.rts_formation_move_execution_crowd_avoidance_gate == true
  and .gates.rts_formation_move_execution_blocked_reroute_gate == true
  and .gates.rts_formation_move_execution_arrival_lock_gate == true
  and .gates.rts_formation_move_execution_scene_renderer_gate == true
  and .gates.rts_formation_move_execution_original_art_policy_gate == true
  and .gates.rts_local_obstruction_recovery_live_input_gate == true
  and .gates.rts_local_obstruction_recovery_detect_block_gate == true
  and .gates.rts_local_obstruction_recovery_hold_queue_gate == true
  and .gates.rts_local_obstruction_recovery_side_step_gate == true
  and .gates.rts_local_obstruction_recovery_gap_claim_gate == true
  and .gates.rts_local_obstruction_recovery_flow_resume_gate == true
  and .gates.rts_local_obstruction_recovery_scene_renderer_gate == true
  and .gates.rts_local_obstruction_recovery_original_art_policy_gate == true
  and .gates.rts_action_cadence_windup_gate == true
  and .gates.rts_action_cadence_strike_gate == true
  and .gates.rts_action_cadence_recovery_gate == true
  and .gates.rts_action_cadence_carry_bob_gate == true
  and .gates.rts_action_cadence_idle_breath_gate == true
  and .gates.rts_action_cadence_shadow_smear_gate == true
  and .gates.rts_action_cadence_scene_renderer_gate == true
  and .gates.rts_action_cadence_event_gate == true
  and .gates.rts_action_cadence_original_art_policy_gate == true
  and .gates.rts_unit_model_depth_rim_gate == true
  and .gates.rts_unit_model_depth_armor_gate == true
  and .gates.rts_unit_model_depth_role_prop_gate == true
  and .gates.rts_unit_model_depth_face_shade_gate == true
  and .gates.rts_unit_model_depth_ground_contact_gate == true
  and .gates.rts_unit_model_depth_layer_shadow_gate == true
  and .gates.rts_unit_model_depth_scene_renderer_gate == true
  and .gates.rts_unit_model_depth_role_coverage_gate == true
  and .gates.rts_unit_model_depth_original_art_policy_gate == true
  and .gates.rts_action_sequence_idle_gate == true
  and .gates.rts_action_sequence_windup_gate == true
  and .gates.rts_action_sequence_strike_gate == true
  and .gates.rts_action_sequence_recovery_gate == true
  and .gates.rts_action_sequence_carry_up_gate == true
  and .gates.rts_action_sequence_carry_down_gate == true
  and .gates.rts_action_sequence_frame_ghost_gate == true
  and .gates.rts_action_sequence_sequence_phase_gate == true
  and .gates.rts_action_sequence_scene_renderer_gate == true
  and .gates.rts_action_sequence_original_art_policy_gate == true
  and .gates.rts_npc_behavior_patrol_gate == true
  and .gates.rts_npc_behavior_engage_gate == true
  and .gates.rts_npc_behavior_work_gate == true
  and .gates.rts_npc_behavior_carry_gate == true
  and .gates.rts_npc_behavior_stalk_gate == true
  and .gates.rts_npc_behavior_retreat_gate == true
  and .gates.rts_npc_behavior_route_gate == true
  and .gates.rts_npc_behavior_behavior_stage_gate == true
  and .gates.rts_npc_behavior_scene_renderer_gate == true
  and .gates.rts_npc_behavior_original_art_policy_gate == true
  and .gates.rts_combat_impact_hit_gate == true
  and .gates.rts_combat_impact_stagger_gate == true
  and .gates.rts_combat_impact_damage_gate == true
  and .gates.rts_combat_impact_death_gate == true
  and .gates.rts_combat_impact_corpse_gate == true
  and .gates.rts_combat_impact_dissolve_gate == true
  and .gates.rts_combat_impact_victory_gate == true
  and .gates.rts_combat_impact_impact_stage_gate == true
  and .gates.rts_combat_impact_scene_renderer_gate == true
  and .gates.rts_combat_impact_original_art_policy_gate == true
  and .gates.rts_locomotion_blend_path_gate == true
  and .gates.rts_locomotion_blend_left_step_gate == true
  and .gates.rts_locomotion_blend_right_step_gate == true
  and .gates.rts_locomotion_blend_turn_gate == true
  and .gates.rts_locomotion_blend_slide_gate == true
  and .gates.rts_locomotion_blend_brake_gate == true
  and .gates.rts_locomotion_blend_locomotion_stage_gate == true
  and .gates.rts_locomotion_blend_scene_renderer_gate == true
  and .gates.rts_locomotion_blend_original_art_policy_gate == true
  and .gates.rts_npc_transition_alert_gate == true
  and .gates.rts_npc_transition_engage_gate == true
  and .gates.rts_npc_transition_pickup_gate == true
  and .gates.rts_npc_transition_pounce_gate == true
  and .gates.rts_npc_transition_recover_gate == true
  and .gates.rts_npc_transition_resume_gate == true
  and .gates.rts_npc_transition_transition_stage_gate == true
  and .gates.rts_npc_transition_scene_renderer_gate == true
  and .gates.rts_npc_transition_original_art_policy_gate == true
  and .gates.rts_depth_readability_foreground_gate == true
  and .gates.rts_depth_readability_behind_gate == true
  and .gates.rts_depth_readability_building_mask_gate == true
  and .gates.rts_depth_readability_target_priority_gate == true
  and .gates.rts_depth_readability_path_occlusion_gate == true
  and .gates.rts_depth_readability_cutaway_gate == true
  and .gates.rts_depth_readability_depth_stage_gate == true
  and .gates.rts_depth_readability_scene_renderer_gate == true
  and .gates.rts_depth_readability_original_art_policy_gate == true
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
# END_PLAYTEST_READINESS_VALIDATION_FILTER
PLAYTEST_READINESS_VALIDATION_FILTER_BLOCK

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYTEST_READINESS_GREEN %s\n' "$SUMMARY"
