#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_depth_readability.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_depth_readability_v1'
  'bevy-classic-rts-depth-readability.json'
  'bevy-classic-rts-depth-readability.ppm'
  'classic-rts-depth-readability'
  'foreground_gate == true'
  'behind_gate == true'
  'building_mask_gate == true'
  'target_priority_gate == true'
  'path_occlusion_gate == true'
  'cutaway_gate == true'
  'depth_stage_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_DEPTH_READABILITY_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS depth readability script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_DEPTH_READABILITY_CONTRACT'
  'native_classic_rts_depth_readability_evidence_json'
  'classic_rts_depth_readability_stage'
  'classic_draw_rts_depth_readability_marks'
  'CLASSIC_RTS_DEPTH_FOREGROUND_COLOR'
  'CLASSIC_RTS_DEPTH_BEHIND_COLOR'
  'CLASSIC_RTS_DEPTH_BUILDING_MASK_COLOR'
  'CLASSIC_RTS_DEPTH_TARGET_PRIORITY_COLOR'
  'CLASSIC_RTS_DEPTH_PATH_OCCLUSION_COLOR'
  'CLASSIC_RTS_DEPTH_CUTAWAY_COLOR'
  'Original Trillionnium depth-readability overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS depth readability source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_depth_readability.sh'
  'bevy-classic-rts-depth-readability.json'
  'classic_rts_depth_readability_green'
  'rts_depth_readability_foreground_gate'
  'rts_depth_readability_behind_gate'
  'rts_depth_readability_building_mask_gate'
  'rts_depth_readability_target_priority_gate'
  'rts_depth_readability_path_occlusion_gate'
  'rts_depth_readability_cutaway_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS depth readability readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_depth_readability_v1'
  'bevy_classic_rts_depth_readability_contract_guard'
  'bevy_classic_rts_depth_readability_gate'
  'bevy_classic_rts_depth_readability_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_depth_readability.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS depth readability release line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS depth readability evidence remains connected to renderer, readiness, release review, and original art policy"
