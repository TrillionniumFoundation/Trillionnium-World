#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_action_coach.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_action_coach_v1'
  'action_coach_green'
  'bevy-action-coach.json'
  'action-coach'
  'native_action_coach_evidence_json'
  'ACTION COACH | Enter/NumpadEnter ->'
  'coach_stage_gate == true'
  'enter_execution_gate == true'
  'final_next_gate == true'
  'input_hint_contract_gate == true'
  '["TALK","TRAIN","MOVE:north","FIGHT"]'
  'external_evidence_ignored_for_current_action_coach_pass'
  'android_s5_real_device_claimed == false'
  'public_launch_ready == false'
  'production_ready_ui_claimed == false'
  'screen_for_screen_openra_ui_claimed == false'
  'openra_engine_port_claimed == false'
  'third_party_asset_copied == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] action coach contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] action coach binds Enter/NumpadEnter guidance to focused action execution without S5/public/OpenRA-copy claims"
