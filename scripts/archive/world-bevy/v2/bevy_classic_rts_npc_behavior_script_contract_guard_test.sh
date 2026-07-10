#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_npc_behavior.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_npc_behavior_loop_v1'
  'bevy-classic-rts-npc-behavior.json'
  'bevy-classic-rts-npc-behavior.ppm'
  'classic-rts-npc-behavior'
  'patrol_gate == true'
  'engage_gate == true'
  'work_gate == true'
  'carry_gate == true'
  'stalk_gate == true'
  'retreat_gate == true'
  'behavior_stage_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_NPC_BEHAVIOR_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS NPC behavior script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_NPC_BEHAVIOR_CONTRACT'
  'native_classic_rts_npc_behavior_evidence_json'
  'classic_rts_npc_behavior_stage'
  'classic_draw_rts_npc_behavior_marks'
  'CLASSIC_RTS_NPC_BEHAVIOR_PATROL_COLOR'
  'CLASSIC_RTS_NPC_BEHAVIOR_ENGAGE_COLOR'
  'CLASSIC_RTS_NPC_BEHAVIOR_WORK_COLOR'
  'CLASSIC_RTS_NPC_BEHAVIOR_CARRY_COLOR'
  'CLASSIC_RTS_NPC_BEHAVIOR_STALK_COLOR'
  'CLASSIC_RTS_NPC_BEHAVIOR_RETREAT_COLOR'
  'Original Trillionnium NPC behavior overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS NPC behavior source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_npc_behavior.sh'
  'bevy-classic-rts-npc-behavior.json'
  'classic_rts_npc_behavior_green'
  'rts_npc_behavior_patrol_gate'
  'rts_npc_behavior_engage_gate'
  'rts_npc_behavior_work_gate'
  'rts_npc_behavior_carry_gate'
  'rts_npc_behavior_stalk_gate'
  'rts_npc_behavior_retreat_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS NPC behavior readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_npc_behavior_loop_v1'
  'bevy_classic_rts_npc_behavior_contract_guard'
  'bevy_classic_rts_npc_behavior_gate'
  'bevy_classic_rts_npc_behavior_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_npc_behavior.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS NPC behavior release line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS NPC behavior evidence remains connected to renderer, readiness, release review, and original art policy"
