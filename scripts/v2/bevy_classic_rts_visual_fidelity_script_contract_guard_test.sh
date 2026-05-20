#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_visual_fidelity.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_visual_fidelity_v1'
  'bevy-classic-rts-visual-fidelity.json'
  'bevy-classic-rts-visual-fidelity.ppm'
  'classic-rts-visual-fidelity'
  'mature_rts_hud_gate == true'
  'model_fidelity_gate == true'
  'npc_animation_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_VISUAL_FIDELITY_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS visual fidelity script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_VISUAL_FIDELITY_CONTRACT'
  'native_classic_rts_visual_fidelity_evidence_json'
  'classic_draw_rts_fidelity_overlay'
  'classic_draw_rts_fidelity_portrait'
  'classic_draw_rts_fidelity_unit_card'
  'classic_draw_rts_fidelity_command_cell'
  'CLASSIC_RTS_FIDELITY_PANEL_COLOR'
  'CLASSIC_RTS_FIDELITY_COMMAND_GRID_COLOR'
  'CLASSIC_RTS_FIDELITY_ACTION_TRAIL_COLOR'
  'warcraft_iii_asset_copied'
  'Original Trillionnium low-spec 2.5D/isometric RTS presentation'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS visual fidelity source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_visual_fidelity.sh'
  'bevy-classic-rts-visual-fidelity.json'
  'classic_rts_visual_fidelity_green'
  'rts_visual_fidelity_mature_hud_gate'
  'rts_visual_fidelity_model_gate'
  'rts_visual_fidelity_npc_animation_gate'
  'rts_visual_fidelity_original_art_policy_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS visual fidelity readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS visual fidelity evidence remains connected to renderer, original art policy, readiness, and mature RTS HUD gates"
