#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
README="$ROOT/README.md"

required_lines=(
  '### 5.4 Trillionnium World release review quickcheck'
  './scripts/check_trillionnium_world_release_review_quickcheck.sh'
  './scripts/check_trillionnium_world_release_review_status.sh'
  './scripts/check_trillionnium_world_release_review_convergence.sh'
  './scripts/check_trillionnium_world_release_review_packet.sh'
  './scripts/check_trillionnium_world_release_review_packet_integrity.sh'
  './scripts/check_trillionnium_world_release_review_ci_gate.sh'
  './scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh'
  './scripts/check_trillionnium_world_next_execution_plan.sh'
  './scripts/check_trillionnium_world_review_slice_manifest.sh'
  './scripts/check_trillionnium_world_review_triage_queue.sh'
  './scripts/check_trillionnium_world_review_primary_owner_plan.sh'
  './scripts/check_trillionnium_world_review_release_owner_queue.sh'
  './scripts/check_trillionnium_world_review_runtime_owner_queue.sh'
  './scripts/check_trillionnium_world_public_launch_blocker_execution_ledger.sh'
  './scripts/check_trillionnium_world_public_launch_operator_handoff.sh'
  './scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh'
  './scripts/check_trillionnium_world_production_map_pack_public_evidence.sh'
  './scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh'
  './scripts/check_trillionnium_world_cohort_commercial_evidence.sh'
  './scripts/check_trillionnium_world_external_ops_evidence_collection.sh'
  './scripts/check_trillionnium_world_external_ops_evidence.sh'
  './scripts/check_trillionnium_world_bevy_action_coach.sh'
  './scripts/check_trillionnium_world_bevy_player_hud_debug_layer.sh'
  './scripts/check_trillionnium_world_bevy_player_ui_rescue.sh'
  './scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh'
  './scripts/check_trillionnium_world_bevy_sprite_texture_sampling.sh'
  './scripts/check_trillionnium_world_bevy_live_window_sampled_texture_correlation.sh'
  './scripts/check_trillionnium_world_bevy_render_asset_eligibility.sh'
  './scripts/check_trillionnium_world_cex_adapter_readiness.sh'
  './scripts/check_trillionnium_world_s5_device_evidence.sh --require-device'
  './scripts/check_trillionnium_world_s5_real_device_evidence.sh'
  './scripts/check_trillionnium_world_release_review_quickcheck.sh --require-ready'
  'acceptance/S6_public_launch/latest/release-review-quickcheck.json'
  'acceptance/S6_public_launch/latest/release-review-status.md'
  'acceptance/S6_public_launch/latest/release-review-convergence.json'
  'acceptance/S6_public_launch/latest/release-review-packet.json'
  'acceptance/S6_public_launch/latest/release-review-packet-integrity.json'
  'acceptance/S6_public_launch/latest/release-review-ci-gate.json'
  'acceptance/S6_public_launch/latest/release-review-checkpoint-manifest.json'
  'acceptance/S6_public_launch/latest/trillionnium-world-next-execution-plan.json'
  'acceptance/S6_public_launch/latest/trillionnium-world-review-slice-manifest.json'
  'acceptance/S6_public_launch/latest/trillionnium-world-review-triage-queue.json'
  'acceptance/S6_public_launch/latest/trillionnium-world-review-primary-owner-plan.json'
  'acceptance/S6_public_launch/latest/trillionnium-world-review-release-owner-queue.json'
  'acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-owner-queue.json'
  'acceptance/S6_public_launch/latest/trillionnium-world-public-launch-blocker-execution-ledger.json'
  'acceptance/S6_public_launch/latest/public-launch-operator-handoff.json'
  'checksum-binding the six collection actions, templates, validator commands, bundle template, and negative fixtures'
  'grouping the current dirty working tree into review/commit slices without staging, committing, or claiming public-launch evidence'
  'Native/Bevy keyboard replay, classic animation preview/selector, classic player motion, action coach, player HUD/debug layer, player UI rescue, live-window screenshot and mouse hit-test evidence, sprite texture sampling, sampled texture live-window correlation, render asset eligibility, CEX adapter readiness'
  'It does not claim GPU upload, render-world extraction completion, Android S5 real-device readiness, or external public-launch readiness.'
  'packet artifact count now at `128`'
  'current packet at artifact count `128`'
  'whole-screen First Contact readability'
  'review-slice manifest'
  'review triage queue'
  'review primary-owner plan'
  'release-owner queue'
  'runtime-owner queue'
  'blocker execution ledger'
  'public-launch blockers preserved'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$README"; then
    echo "[FAIL] README missing Trillionnium World release review quickcheck line: $line" >&2
    exit 1
  fi
done

echo "[PASS] README keeps Trillionnium World release review quickcheck/status/convergence/packet/integrity/ci-gate/checkpoint/next-plan commands, S5 collection+validation, strict mode, output files, and Android S5 boundary"
