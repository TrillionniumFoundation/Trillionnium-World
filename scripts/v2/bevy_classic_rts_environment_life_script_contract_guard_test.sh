#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_environment_life.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_environment_life_v1'
  'bevy-classic-rts-environment-life.json'
  'bevy-classic-rts-environment-life.ppm'
  'classic-rts-environment-life'
  'tree_sway_gate == true'
  'torch_flicker_gate == true'
  'water_shimmer_gate == true'
  'banner_flutter_gate == true'
  'resource_glint_gate == true'
  'ambient_dust_gate == true'
  'environment_stage_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ENVIRONMENT_LIFE_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS environment life script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ENVIRONMENT_LIFE_CONTRACT'
  'native_classic_rts_environment_life_evidence_json'
  'classic_rts_environment_life_stage'
  'classic_draw_rts_environment_life_scene_overlay'
  'CLASSIC_RTS_ENVIRONMENT_TREE_SWAY_COLOR'
  'CLASSIC_RTS_ENVIRONMENT_TORCH_FLICKER_COLOR'
  'CLASSIC_RTS_ENVIRONMENT_WATER_SHIMMER_COLOR'
  'CLASSIC_RTS_ENVIRONMENT_BANNER_FLUTTER_COLOR'
  'CLASSIC_RTS_ENVIRONMENT_RESOURCE_GLINT_COLOR'
  'CLASSIC_RTS_ENVIRONMENT_AMBIENT_DUST_COLOR'
  'Original Trillionnium environment-life overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS environment life source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_environment_life.sh'
  'bevy-classic-rts-environment-life.json'
  'classic_rts_environment_life_green'
  'rts_environment_life_tree_sway_gate'
  'rts_environment_life_torch_flicker_gate'
  'rts_environment_life_water_shimmer_gate'
  'rts_environment_life_banner_flutter_gate'
  'rts_environment_life_resource_glint_gate'
  'rts_environment_life_ambient_dust_gate'
  'rts_environment_life_environment_stage_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS environment life readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_environment_life_v1'
  'bevy_classic_rts_environment_life_contract_guard'
  'bevy_classic_rts_environment_life_gate'
  'bevy_classic_rts_environment_life_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_environment_life.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS environment life release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS environment life evidence remains connected to renderer, CLI, readiness, release-review, and original art policy"
