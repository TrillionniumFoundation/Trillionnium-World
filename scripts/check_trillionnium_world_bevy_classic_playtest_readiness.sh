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
"$ROOT/scripts/check_trillionnium_world_client_boundary.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_runner_status.sh" >/dev/null

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
  --slurpfile boundary "$ROOT/acceptance/S6_public_launch/latest/client-boundary-cleanliness.json" \
  --slurpfile runner "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json" '
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
      and (($boundary[0].green == true) or ($boundary[0].status == "green"))
      and ok($runner)
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
      client_boundary_green: (($boundary[0].green == true) or ($boundary[0].status == "green")),
      playtest_runner_status_green: ok($runner)
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
      isometric_command_feedback_pixel_count: $iso[0].command_feedback_pixel_count,
      isometric_command_marker_pixel_count: $iso[0].command_marker_pixel_count,
      isometric_attack_arc_pixel_count: $iso[0].attack_arc_pixel_count,
      isometric_hit_flash_pixel_count: $iso[0].hit_flash_pixel_count,
      model_catalog_rendered_frame_count: $catalog[0].rendered_frame_count,
      runner_main_pid: $runner[0].service.main_pid,
      runner_process_cwd: $runner[0].runtime.process_cwd
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
      isometric_command_feedback_gate: $iso[0].command_feedback_gate,
      isometric_sprite_anchor_gate: $iso[0].sprite_anchor_gate,
      catalog_all_frames_rendered_gate: $catalog[0].all_frames_rendered_gate,
      runner_service_process_gate: $runner[0].gates.service_process_gate,
      runner_release_binary_gate: $runner[0].gates.release_binary_gate,
      runner_classic_env_gate: $runner[0].gates.classic_env_gate,
      runner_cex_path_gate: $runner[0].gates.cex_path_gate
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
      playtest_runner_status: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json"
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
  and .checks.client_boundary_green == true
  and .checks.playtest_runner_status_green == true
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
  and .headline.isometric_command_feedback_pixel_count > 500
  and .headline.isometric_command_marker_pixel_count > 250
  and .headline.isometric_attack_arc_pixel_count > 100
  and .headline.isometric_hit_flash_pixel_count > 80
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
  and .gates.isometric_command_feedback_gate == true
  and .gates.isometric_sprite_anchor_gate == true
  and .gates.catalog_all_frames_rendered_gate == true
  and .gates.runner_service_process_gate == true
  and .gates.runner_release_binary_gate == true
  and .gates.runner_classic_env_gate == true
  and .gates.runner_cex_path_gate == true
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYTEST_READINESS_GREEN %s\n' "$SUMMARY"
