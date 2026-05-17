#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT/RELEASE_READINESS.md"

required_lines=(
  '### 4. Trillionnium World Release Review Handoff (local gate, not public readiness)'
  'scripts/check_trillionnium_world_release_review_ci_gate.sh'
  'scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh'
  'acceptance/S6_public_launch/latest/release-review-ci-gate.json'
  'acceptance/S6_public_launch/latest/release-review-checkpoint-manifest.json'
  'trillionnium_world_release_review_ci_gate_v1'
  'release_review_ci_gate_green_with_public_launch_blockers'
  'ready_for_release_review=true'
  'public_launch_ready=false'
  'android_s5_real_device_claimed=false'
  'host-side Native/Bevy local playability, texture sampling/correlation, render-asset eligibility, and CEX adapter readiness only'
  'sprite texture sampling'
  'sampled texture live-window correlation'
  'render asset eligibility'
  'CEX adapter readiness'
  'CEX production adapter readiness'
  'groups dirty paths into review/commit slices'
  'it stages nothing, commits nothing, and does not replace real public-launch evidence'
  'S5 Android real-device matrix'
  'first beta cohort evidence'
  'commercial launch drill evidence'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$DOC"; then
    echo "[FAIL] RELEASE_READINESS missing release review handoff line: $line" >&2
    exit 1
  fi
done

echo "[PASS] RELEASE_READINESS keeps Trillionnium World release review handoff/checkpoint entries and public-readiness boundary"
