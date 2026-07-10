#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback_v1'
  'bevy-classic-rts-control-group-hotkey-feedback.json'
  'bevy-classic-rts-control-group-hotkey-feedback.ppm'
  'classic-rts-control-group-hotkey-feedback'
  'assign_gate == true'
  'recall_gate == true'
  'camera_gate == true'
  'idle_gate == true'
  'production_gate == true'
  'ability_gate == true'
  'hotkey_stage_gate == true'
  'hotkey_runtime_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_HOTKEY_FEEDBACK_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS control group hotkey feedback script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_HOTKEY_FEEDBACK_CONTRACT'
  'native_classic_rts_control_group_hotkey_feedback_evidence_json'
  'classic_rts_control_group_hotkey_feedback_stage'
  'classic_draw_rts_control_group_hotkey_feedback_overlay'
  'CLASSIC_RTS_CONTROL_GROUP_ASSIGN_COLOR'
  'CLASSIC_RTS_CONTROL_GROUP_RECALL_COLOR'
  'CLASSIC_RTS_CONTROL_GROUP_CAMERA_COLOR'
  'CLASSIC_RTS_CONTROL_GROUP_IDLE_COLOR'
  'CLASSIC_RTS_CONTROL_GROUP_PRODUCTION_COLOR'
  'CLASSIC_RTS_CONTROL_GROUP_ABILITY_COLOR'
  'Original Trillionnium control-group/hotkey overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS control group hotkey feedback source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback.sh'
  'bevy-classic-rts-control-group-hotkey-feedback.json'
  'classic_rts_control_group_hotkey_feedback_green'
  'rts_control_group_hotkey_feedback_assign_gate'
  'rts_control_group_hotkey_feedback_recall_gate'
  'rts_control_group_hotkey_feedback_camera_gate'
  'rts_control_group_hotkey_feedback_idle_gate'
  'rts_control_group_hotkey_feedback_production_gate'
  'rts_control_group_hotkey_feedback_ability_gate'
  'rts_control_group_hotkey_feedback_hotkey_runtime_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS control group hotkey feedback readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback_v1'
  'bevy_classic_rts_control_group_hotkey_feedback_contract_guard'
  'bevy_classic_rts_control_group_hotkey_feedback_gate'
  'bevy_classic_rts_control_group_hotkey_feedback_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS control group hotkey feedback release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS control group hotkey feedback evidence remains connected to renderer, CLI, readiness, release-review, hotkey runtime, and original art policy"
