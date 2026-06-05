#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_release_review_convergence.sh"

required_lines=(
  'trillionnium_world_release_review_convergence_v1'
  'check_trillionnium_world_release_review_status.sh'
  'release-review-convergence.json'
  'release-review-convergence-status.log'
  'check_trillionnium_world_release_review_quickcheck.sh'
  'check_trillionnium_world_release_review_status.sh'
  'check_trillionnium_world_cex_adapter_readiness.sh'
  'root_readme_world_release_review_quickcheck_guard_test.sh'
  'release_review_status_script_contract_guard_test.sh'
  'bevy-build-branch-title-route-all-branch-keyboard-replay.json'
  'bevy-action-coach.json'
  'bevy-player-hud-debug-layer.json'
  'bevy-live-window-screenshot-sequence.json'
  'bevy-sprite-texture-sampling.json'
  'bevy-live-window-sampled-texture-correlation.json'
  'bevy-render-asset-eligibility.json'
  'cex-production-adapter-readiness.json'
  'cex_adapter_readiness'
  'trillionnium_world_cex_adapter_readiness_gate_v1'
  'cex_trillionnium_world_production_adapter_v1'
  'public_launch_consumes_local_playability'
  'public-launch-readiness.json'
  'release-signoff-summary.json'
  'release-review-quickcheck.json'
  'release-review-status.json'
  'release-review-status.md'
  'host_side_bevy_runtime_replay_not_android_real_device'
  'trillionnium_world_bevy_sprite_texture_sampling_v1'
  'trillionnium_world_bevy_live_window_sampled_texture_correlation_v1'
  'trillionnium_world_bevy_render_asset_eligibility_v1'
  'cex_adapter_readiness_ready'
  'Native/Bevy keyboard replay, classic animation preview/selector, classic player motion, action coach, HUD/debug layer, player UI rescue, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof.'
  'CEX adapter readiness is incubator runtime adapter evidence, not real external public-launch evidence.'
  'android_s5_real_device_claimed: false'
  'Green For Review'
  'Still Requires Real External Evidence'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] release review convergence script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] release review convergence script keeps refresh dependency, docs/workflow/script/evidence checks, summary output, and Android S5 boundary"
