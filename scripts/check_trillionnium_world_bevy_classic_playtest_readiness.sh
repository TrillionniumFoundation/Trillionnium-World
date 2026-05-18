#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-readiness.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_preview.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_selector.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_player_motion_probe.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_render_budget.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_scene_preview.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_renderer_probe.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_model_catalog.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_client_boundary.sh" >/dev/null

jq -n \
  --slurpfile manifest "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-manifest-lint.json" \
  --slurpfile animation "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.json" \
  --slurpfile selector "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-selector.json" \
  --slurpfile motion "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-player-motion-probe.json" \
  --slurpfile budget "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-render-budget.json" \
  --slurpfile scene "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-scene-preview.json" \
  --slurpfile probe "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.json" \
  --slurpfile catalog "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.json" \
  --slurpfile boundary "$ROOT/acceptance/S6_public_launch/latest/client-boundary-cleanliness.json" '
  def ok($x): ($x[0].green == true);
  {
    contract_version: "trillionnium_world_bevy_classic_playtest_readiness_v1",
    green: (
      ok($manifest)
      and ok($animation)
      and ok($selector)
      and ok($motion)
      and ok($budget)
      and ok($scene)
      and ok($probe)
      and ok($catalog)
      and (($boundary[0].green == true) or ($boundary[0].status == "green"))
      and $manifest[0].cex_runtime_player_client_allowed == false
      and $budget[0].p95_budget_gate == true
      and $motion[0].accepted_input_gate == true
      and $selector[0].animation_transition_gate == true
      and $scene[0].dynamic_landmark_animation_gate == true
      and $probe[0].hud_probe_gate == true
    ),
    checks: {
      manifest_lint_green: ok($manifest),
      animation_preview_green: ok($animation),
      animation_selector_green: ok($selector),
      player_motion_green: ok($motion),
      render_budget_green: ok($budget),
      scene_preview_green: ok($scene),
      renderer_probe_green: ok($probe),
      model_catalog_green: ok($catalog),
      client_boundary_green: (($boundary[0].green == true) or ($boundary[0].status == "green"))
    },
    headline: {
      frame_count: $manifest[0].frame_count,
      animation_clip_count: $animation[0].clip_count,
      motion_sample_count: $motion[0].sample_count,
      motion_accepted_input_count: $motion[0].accepted_input_count,
      render_p50_micros: $budget[0].p50_micros,
      render_p95_micros: $budget[0].p95_micros,
      render_max_micros: $budget[0].max_micros,
      scene_unique_color_count: $scene[0].unique_color_count,
      renderer_probe_hud_text_pixels: $probe[0].hud_text_pixels,
      model_catalog_rendered_frame_count: $catalog[0].rendered_frame_count
    },
    gates: {
      cex_runtime_player_client_allowed: $manifest[0].cex_runtime_player_client_allowed,
      wgpu_required: $manifest[0].wgpu_required,
      manifest_boundary_gate: $manifest[0].boundary_gate,
      animation_action_coverage_gate: $animation[0].action_coverage_gate,
      selector_transition_gate: $selector[0].animation_transition_gate,
      motion_direction_coverage_gate: $motion[0].direction_coverage_gate,
      render_p95_budget_gate: $budget[0].p95_budget_gate,
      render_max_budget_gate: $budget[0].max_budget_gate,
      scene_dynamic_landmark_animation_gate: $scene[0].dynamic_landmark_animation_gate,
      renderer_probe_scene_frame_gate: $probe[0].scene_frame_gate,
      catalog_all_frames_rendered_gate: $catalog[0].all_frames_rendered_gate
    },
    artifacts: {
      manifest_lint: "acceptance/S5_native_bevy_device/latest/bevy-classic-manifest-lint.json",
      animation_preview: "acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.json",
      animation_preview_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.ppm",
      animation_selector: "acceptance/S5_native_bevy_device/latest/bevy-classic-animation-selector.json",
      player_motion_probe: "acceptance/S5_native_bevy_device/latest/bevy-classic-player-motion-probe.json",
      player_motion_probe_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-player-motion-probe.ppm",
      render_budget: "acceptance/S5_native_bevy_device/latest/bevy-classic-render-budget.json",
      scene_preview: "acceptance/S5_native_bevy_device/latest/bevy-classic-scene-preview.json",
      scene_preview_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-scene-preview.ppm",
      renderer_probe: "acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.json",
      renderer_probe_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.ppm",
      model_catalog: "acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.json",
      model_catalog_ppm: "acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.ppm"
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
  and .checks.render_budget_green == true
  and .checks.scene_preview_green == true
  and .checks.renderer_probe_green == true
  and .checks.model_catalog_green == true
  and .checks.client_boundary_green == true
  and .headline.frame_count >= 43
  and .headline.animation_clip_count >= 4
  and .headline.motion_sample_count == 8
  and .headline.motion_accepted_input_count == 8
  and .headline.render_p95_micros <= 16000
  and .headline.render_max_micros <= 40000
  and .gates.cex_runtime_player_client_allowed == false
  and .gates.wgpu_required == false
  and .gates.manifest_boundary_gate == true
  and .gates.animation_action_coverage_gate == true
  and .gates.selector_transition_gate == true
  and .gates.motion_direction_coverage_gate == true
  and .gates.render_p95_budget_gate == true
  and .gates.render_max_budget_gate == true
  and .gates.scene_dynamic_landmark_animation_gate == true
  and .gates.renderer_probe_scene_frame_gate == true
  and .gates.catalog_all_frames_rendered_gate == true
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYTEST_READINESS_GREEN %s\n' "$SUMMARY"
