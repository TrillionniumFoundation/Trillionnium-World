#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_combat_impact.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_combat_impact_loop_v1'
  'bevy-classic-rts-combat-impact.json'
  'bevy-classic-rts-combat-impact.ppm'
  'classic-rts-combat-impact'
  'hit_gate == true'
  'stagger_gate == true'
  'damage_gate == true'
  'death_gate == true'
  'corpse_gate == true'
  'dissolve_gate == true'
  'victory_gate == true'
  'impact_stage_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMBAT_IMPACT_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS combat impact script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMBAT_IMPACT_CONTRACT'
  'native_classic_rts_combat_impact_evidence_json'
  'classic_rts_combat_impact_stage'
  'classic_draw_rts_combat_impact_marks'
  'CLASSIC_RTS_COMBAT_IMPACT_HIT_COLOR'
  'CLASSIC_RTS_COMBAT_IMPACT_STAGGER_COLOR'
  'CLASSIC_RTS_COMBAT_IMPACT_DAMAGE_COLOR'
  'CLASSIC_RTS_COMBAT_IMPACT_DEATH_COLOR'
  'CLASSIC_RTS_COMBAT_IMPACT_CORPSE_COLOR'
  'CLASSIC_RTS_COMBAT_IMPACT_DISSOLVE_COLOR'
  'CLASSIC_RTS_COMBAT_IMPACT_VICTORY_COLOR'
  'Original Trillionnium combat impact overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS combat impact source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_combat_impact.sh'
  'bevy-classic-rts-combat-impact.json'
  'classic_rts_combat_impact_green'
  'rts_combat_impact_hit_gate'
  'rts_combat_impact_stagger_gate'
  'rts_combat_impact_damage_gate'
  'rts_combat_impact_death_gate'
  'rts_combat_impact_corpse_gate'
  'rts_combat_impact_dissolve_gate'
  'rts_combat_impact_victory_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS combat impact readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_combat_impact_loop_v1'
  'bevy_classic_rts_combat_impact_contract_guard'
  'bevy_classic_rts_combat_impact_gate'
  'bevy_classic_rts_combat_impact_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_combat_impact.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS combat impact release line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS combat impact evidence remains connected to renderer, readiness, release review, and original art policy"
