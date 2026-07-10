#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_npc_transition.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_npc_transition_blend_v1'
  'bevy-classic-rts-npc-transition.json'
  'bevy-classic-rts-npc-transition.ppm'
  'classic-rts-npc-transition'
  'alert_gate == true'
  'engage_gate == true'
  'pickup_gate == true'
  'pounce_gate == true'
  'recover_gate == true'
  'resume_gate == true'
  'transition_stage_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_NPC_TRANSITION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS NPC transition script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_NPC_TRANSITION_CONTRACT'
  'native_classic_rts_npc_transition_evidence_json'
  'classic_rts_npc_transition_stage'
  'classic_draw_rts_npc_transition_marks'
  'CLASSIC_RTS_NPC_TRANSITION_ALERT_COLOR'
  'CLASSIC_RTS_NPC_TRANSITION_ENGAGE_COLOR'
  'CLASSIC_RTS_NPC_TRANSITION_PICKUP_COLOR'
  'CLASSIC_RTS_NPC_TRANSITION_POUNCE_COLOR'
  'CLASSIC_RTS_NPC_TRANSITION_RECOVER_COLOR'
  'CLASSIC_RTS_NPC_TRANSITION_RESUME_COLOR'
  'Original Trillionnium NPC transition overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS NPC transition source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_npc_transition.sh'
  'bevy-classic-rts-npc-transition.json'
  'classic_rts_npc_transition_green'
  'rts_npc_transition_alert_gate'
  'rts_npc_transition_engage_gate'
  'rts_npc_transition_pickup_gate'
  'rts_npc_transition_pounce_gate'
  'rts_npc_transition_recover_gate'
  'rts_npc_transition_resume_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS NPC transition readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_npc_transition_blend_v1'
  'bevy_classic_rts_npc_transition_contract_guard'
  'bevy_classic_rts_npc_transition_gate'
  'bevy_classic_rts_npc_transition_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_npc_transition.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS NPC transition release line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS NPC transition evidence remains connected to renderer, readiness, release review, and original art policy"
