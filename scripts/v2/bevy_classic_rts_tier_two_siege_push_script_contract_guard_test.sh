#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_tier_two_siege_push.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_tier_two_siege_push_v1'
  'bevy-classic-rts-tier-two-siege-push.json'
  'bevy-classic-rts-tier-two-siege-push.ppm'
  'classic-rts-tier-two-siege-push'
  'input_path == "apply_live_native_action_with_source(classic_rts_tier_two_siege_push_input)"'
  'RTS:QUEUE:tier2:tech:relay_foundry@relay_outpost'
  'RTS:QUEUE:tier2:upgrade:siege_harness@relay_foundry'
  'RTS:QUEUE:tier2:train:stonebreak_cart@relay_foundry'
  'RTS:QUEUE:tier2:enemy_fortify:gate_bulwark@10,3'
  'RTS:QUEUE:tier2:push:gate_bulwark@10,3'
  'expansion_dependency_gate == true'
  'tier_two_tech_gate == true'
  'tier_two_upgrade_gate == true'
  'siege_unit_gate == true'
  'enemy_fortification_gate == true'
  'siege_push_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS tier-two siege script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_TIER_TWO_SIEGE_PUSH_CONTRACT'
  'native_classic_rts_tier_two_siege_push_evidence_json'
  'classic-rts-tier-two-siege-push'
  'classic_rts_tier_two_siege_push_input'
  'rts_tier_two_tech_ids'
  'rts_tier_two_upgrade_ids'
  'rts_siege_unit_ids'
  'rts_siege_push_route_tile_ids'
  'rts_enemy_fortification_ids'
  'rts_siege_damage_log'
  'rts_tier_two_push_state'
  'CLASSIC_RTS_TIER_TWO_TECH_COLOR'
  'CLASSIC_RTS_SIEGE_UNIT_COLOR'
  'CLASSIC_RTS_SIEGE_ROUTE_COLOR'
  'CLASSIC_RTS_ENEMY_FORTIFY_COLOR'
  'CLASSIC_RTS_SIEGE_DAMAGE_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS tier-two siege source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_tier_two_siege_push.sh'
  'bevy-classic-rts-tier-two-siege-push.json'
  'classic_rts_tier_two_siege_push_green'
  'rts_tier_two_siege_push_live_input_gate'
  'rts_tier_two_siege_push_expansion_dependency_gate'
  'rts_tier_two_siege_push_tech_gate'
  'rts_tier_two_siege_push_upgrade_gate'
  'rts_tier_two_siege_push_unit_gate'
  'rts_tier_two_siege_push_enemy_fortification_gate'
  'rts_tier_two_siege_push_push_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS tier-two siege readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS tier-two siege push evidence remains connected to defended expansion, tier-two tech, siege production, enemy fortification, push damage, and readiness"
