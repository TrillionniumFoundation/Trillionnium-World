#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1'
  'bevy-classic-rts-ability-tooltip-telegraph.json'
  'bevy-classic-rts-ability-tooltip-telegraph.ppm'
  'classic-rts-ability-tooltip-telegraph'
  'tooltip_gate == true'
  'range_gate == true'
  'windup_gate == true'
  'cooldown_gate == true'
  'queue_gate == true'
  'warning_gate == true'
  'telegraph_stage_gate == true'
  'ability_runtime_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ABILITY_TOOLTIP_TELEGRAPH_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS ability tooltip telegraph script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ABILITY_TOOLTIP_TELEGRAPH_CONTRACT'
  'native_classic_rts_ability_tooltip_telegraph_evidence_json'
  'classic_rts_ability_tooltip_telegraph_stage'
  'classic_draw_rts_ability_tooltip_telegraph_overlay'
  'CLASSIC_RTS_ABILITY_TELEGRAPH_TOOLTIP_COLOR'
  'CLASSIC_RTS_ABILITY_TELEGRAPH_RANGE_COLOR'
  'CLASSIC_RTS_ABILITY_TELEGRAPH_WINDUP_COLOR'
  'CLASSIC_RTS_ABILITY_TELEGRAPH_COOLDOWN_COLOR'
  'CLASSIC_RTS_ABILITY_TELEGRAPH_QUEUE_COLOR'
  'CLASSIC_RTS_ABILITY_TELEGRAPH_WARNING_COLOR'
  'Original Trillionnium ability tooltip/telegraph overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS ability tooltip telegraph source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph.sh'
  'bevy-classic-rts-ability-tooltip-telegraph.json'
  'classic_rts_ability_tooltip_telegraph_green'
  'rts_ability_tooltip_telegraph_tooltip_gate'
  'rts_ability_tooltip_telegraph_range_gate'
  'rts_ability_tooltip_telegraph_windup_gate'
  'rts_ability_tooltip_telegraph_cooldown_gate'
  'rts_ability_tooltip_telegraph_queue_gate'
  'rts_ability_tooltip_telegraph_warning_gate'
  'rts_ability_tooltip_telegraph_ability_runtime_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS ability tooltip telegraph readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1'
  'bevy_classic_rts_ability_tooltip_telegraph_contract_guard'
  'bevy_classic_rts_ability_tooltip_telegraph_gate'
  'bevy_classic_rts_ability_tooltip_telegraph_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS ability tooltip telegraph release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS ability tooltip telegraph evidence remains connected to renderer, CLI, readiness, release-review, ability runtime, and original art policy"
