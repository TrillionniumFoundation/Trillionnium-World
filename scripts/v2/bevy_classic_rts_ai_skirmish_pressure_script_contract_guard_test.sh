#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_ai_skirmish_pressure.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_ai_skirmish_pressure_v1'
  'bevy-classic-rts-ai-skirmish-pressure.json'
  'bevy-classic-rts-ai-skirmish-pressure.ppm'
  'classic-rts-ai-skirmish-pressure'
  'input_path == "apply_live_native_action_with_source(classic_rts_ai_skirmish_pressure_input)"'
  'RTS:QUEUE:ai:skirmish_wave'
  'RTS:ABILITY:guard_break'
  'ai_wave_gate == true'
  'ai_counter_gate == true'
  'ai_pressure_resolution_gate == true'
  'ai_retreat_gate == true'
  'player_response_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS AI skirmish pressure script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_AI_SKIRMISH_PRESSURE_CONTRACT'
  'native_classic_rts_ai_skirmish_pressure_evidence_json'
  'classic-rts-ai-skirmish-pressure'
  'classic_rts_ai_skirmish_pressure_input'
  'rts_ai_wave_unit_ids'
  'rts_ai_pressure_tile_ids'
  'rts_ai_counter_tile_ids'
  'rts_ai_retreat_tile_id'
  'rts_ai_pressure_percent'
  'rts_ai_response_log'
  'rts_ai_skirmish_state'
  'CLASSIC_RTS_AI_WAVE_COLOR'
  'CLASSIC_RTS_AI_PRESSURE_COLOR'
  'CLASSIC_RTS_AI_COUNTER_COLOR'
  'CLASSIC_RTS_AI_RETREAT_COLOR'
  'CLASSIC_RTS_AI_PRESSURE_BAR_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS AI skirmish pressure source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_ai_skirmish_pressure.sh'
  'bevy-classic-rts-ai-skirmish-pressure.json'
  'classic_rts_ai_skirmish_pressure_green'
  'rts_ai_skirmish_pressure_ai_wave_gate'
  'rts_ai_skirmish_pressure_ai_counter_gate'
  'rts_ai_skirmish_pressure_resolution_gate'
  'rts_ai_skirmish_pressure_retreat_gate'
  'rts_ai_skirmish_player_response_gate'
  'rts_ai_skirmish_pressure_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS AI skirmish pressure readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS AI skirmish pressure evidence remains connected to AI wave, pressure lane, player counter, retreat state, renderer overlays, and readiness"
