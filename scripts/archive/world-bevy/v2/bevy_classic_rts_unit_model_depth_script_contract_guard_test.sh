#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_unit_model_depth.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_unit_model_depth_v1'
  'bevy-classic-rts-unit-model-depth.json'
  'bevy-classic-rts-unit-model-depth.ppm'
  'classic-rts-unit-model-depth'
  'rim_gate == true'
  'armor_gate == true'
  'role_prop_gate == true'
  'face_shade_gate == true'
  'ground_contact_gate == true'
  'layer_shadow_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_UNIT_MODEL_DEPTH_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS unit model depth script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_UNIT_MODEL_DEPTH_CONTRACT'
  'native_classic_rts_unit_model_depth_evidence_json'
  'classic_draw_rts_unit_model_depth_marks'
  'CLASSIC_RTS_UNIT_MODEL_DEPTH_RIM_COLOR'
  'CLASSIC_RTS_UNIT_MODEL_DEPTH_ARMOR_COLOR'
  'CLASSIC_RTS_UNIT_MODEL_DEPTH_ROLE_PROP_COLOR'
  'CLASSIC_RTS_UNIT_MODEL_DEPTH_FACE_SHADE_COLOR'
  'CLASSIC_RTS_UNIT_MODEL_DEPTH_GROUND_CONTACT_COLOR'
  'CLASSIC_RTS_UNIT_MODEL_DEPTH_LAYER_SHADOW_COLOR'
  'Original Trillionnium unit model depth marks'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS unit model depth source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_unit_model_depth.sh'
  'bevy-classic-rts-unit-model-depth.json'
  'classic_rts_unit_model_depth_green'
  'rts_unit_model_depth_rim_gate'
  'rts_unit_model_depth_armor_gate'
  'rts_unit_model_depth_role_prop_gate'
  'rts_unit_model_depth_face_shade_gate'
  'rts_unit_model_depth_ground_contact_gate'
  'rts_unit_model_depth_layer_shadow_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS unit model depth readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_unit_model_depth_v1'
  'bevy_classic_rts_unit_model_depth_contract_guard'
  'bevy_classic_rts_unit_model_depth_gate'
  'bevy_classic_rts_unit_model_depth_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_unit_model_depth.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS unit model depth release line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS unit model depth evidence remains connected to renderer, readiness, release review, and original art policy"
