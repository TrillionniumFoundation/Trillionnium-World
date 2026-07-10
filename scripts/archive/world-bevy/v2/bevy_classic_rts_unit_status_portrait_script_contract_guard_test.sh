#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_unit_status_portrait.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_unit_status_portrait_v1'
  'bevy-classic-rts-unit-status-portrait.json'
  'bevy-classic-rts-unit-status-portrait.ppm'
  'classic-rts-unit-status-portrait'
  'portrait_frame_gate == true'
  'health_bar_gate == true'
  'mana_bar_gate == true'
  'xp_bar_gate == true'
  'buff_badge_gate == true'
  'role_badge_gate == true'
  'queue_badge_gate == true'
  'status_stage_gate == true'
  'status_runtime_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_UNIT_STATUS_PORTRAIT_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS unit status portrait script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_UNIT_STATUS_PORTRAIT_CONTRACT'
  'native_classic_rts_unit_status_portrait_evidence_json'
  'classic_rts_unit_status_portrait_stage'
  'classic_draw_rts_unit_status_portrait_overlay'
  'CLASSIC_RTS_STATUS_PORTRAIT_FRAME_COLOR'
  'CLASSIC_RTS_STATUS_HEALTH_BAR_COLOR'
  'CLASSIC_RTS_STATUS_MANA_BAR_COLOR'
  'CLASSIC_RTS_STATUS_XP_BAR_COLOR'
  'CLASSIC_RTS_STATUS_BUFF_BADGE_COLOR'
  'CLASSIC_RTS_STATUS_ROLE_BADGE_COLOR'
  'CLASSIC_RTS_STATUS_QUEUE_BADGE_COLOR'
  'Original Trillionnium unit-status portrait overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS unit status portrait source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_unit_status_portrait.sh'
  'bevy-classic-rts-unit-status-portrait.json'
  'classic_rts_unit_status_portrait_green'
  'rts_unit_status_portrait_frame_gate'
  'rts_unit_status_health_bar_gate'
  'rts_unit_status_mana_bar_gate'
  'rts_unit_status_xp_bar_gate'
  'rts_unit_status_buff_badge_gate'
  'rts_unit_status_role_badge_gate'
  'rts_unit_status_queue_badge_gate'
  'rts_unit_status_status_runtime_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS unit status portrait readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_unit_status_portrait_v1'
  'bevy_classic_rts_unit_status_portrait_contract_guard'
  'bevy_classic_rts_unit_status_portrait_gate'
  'bevy_classic_rts_unit_status_portrait_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_unit_status_portrait.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS unit status portrait release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS unit status portrait evidence remains connected to renderer, CLI, readiness, release-review, status runtime, and original art policy"
