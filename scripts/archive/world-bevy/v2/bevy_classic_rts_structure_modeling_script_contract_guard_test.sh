#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_structure_modeling.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_structure_modeling_v1'
  'bevy-classic-rts-structure-modeling.json'
  'bevy-classic-rts-structure-modeling.ppm'
  'classic-rts-structure-modeling'
  'foundation_gate == true'
  'scaffold_gate == true'
  'construction_spark_gate == true'
  'production_glow_gate == true'
  'damage_crack_gate == true'
  'repair_beam_gate == true'
  'structure_stage_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_STRUCTURE_MODELING_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS structure modeling script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_STRUCTURE_MODELING_CONTRACT'
  'native_classic_rts_structure_modeling_evidence_json'
  'classic_rts_structure_modeling_stage'
  'classic_draw_rts_structure_modeling_scene_overlay'
  'CLASSIC_RTS_STRUCTURE_FOUNDATION_SHADOW_COLOR'
  'CLASSIC_RTS_STRUCTURE_SCAFFOLD_COLOR'
  'CLASSIC_RTS_STRUCTURE_CONSTRUCTION_SPARK_COLOR'
  'CLASSIC_RTS_STRUCTURE_PRODUCTION_GLOW_COLOR'
  'CLASSIC_RTS_STRUCTURE_DAMAGE_CRACK_COLOR'
  'CLASSIC_RTS_STRUCTURE_REPAIR_BEAM_COLOR'
  'Original Trillionnium structure-modeling overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS structure modeling source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_structure_modeling.sh'
  'bevy-classic-rts-structure-modeling.json'
  'classic_rts_structure_modeling_green'
  'rts_structure_modeling_foundation_gate'
  'rts_structure_modeling_scaffold_gate'
  'rts_structure_modeling_construction_spark_gate'
  'rts_structure_modeling_production_glow_gate'
  'rts_structure_modeling_damage_crack_gate'
  'rts_structure_modeling_repair_beam_gate'
  'rts_structure_modeling_structure_stage_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS structure modeling readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_structure_modeling_v1'
  'bevy_classic_rts_structure_modeling_contract_guard'
  'bevy_classic_rts_structure_modeling_gate'
  'bevy_classic_rts_structure_modeling_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_structure_modeling.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS structure modeling release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS structure modeling evidence remains connected to renderer, CLI, readiness, release-review, and original art policy"
