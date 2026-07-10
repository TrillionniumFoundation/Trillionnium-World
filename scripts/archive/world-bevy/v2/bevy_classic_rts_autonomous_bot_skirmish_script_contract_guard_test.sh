#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_autonomous_bot_skirmish.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_autonomous_bot_skirmish_v1'
  'bevy-classic-rts-autonomous-bot-skirmish.json'
  'bevy-classic-rts-autonomous-bot-skirmish.ppm'
  'classic-rts-autonomous-bot-skirmish'
  'input_action_count == 0'
  'no_live_player_input_gate == true'
  'bot_slot_count == 4'
  'bevy_terminal_winner_beacons == 2'
  'bevy_terminal_total_beacons == 4'
  'bevy_terminal_hold_ticks == 3000'
  'deterministic_autonomous_bot_skirmish_timeline'
  'bevy_terminal_parity_claimed == false'
  'openra_parity_target_commit == "5f1bf76"'
  'rts_core_contract == "trnm_rts_core_frame_order_v1"'
  'rts_autonomous_bot_core_frame_order_gate == true'
  'rts_autonomous_bot_core_headless_replay_gate == true'
  'rts_autonomous_bot_core_headless_capture_order_count == 3'
  'rts_autonomous_bot_core_headless_train_order_count == 1'
  'rts_autonomous_bot_core_headless_attack_order_count == 1'
  'forced_capture_hook_enabled == false'
  'forced_surrender_hook_enabled == false'
  'autonomous_bot_skirmish_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS autonomous bot skirmish script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'native_classic_rts_autonomous_bot_skirmish_evidence_json'
  'classic-rts-autonomous-bot-skirmish'
  'BOT_SLOT_COUNT'
  'TOTAL_BEACONS'
  'WINNER_BEACONS'
  'TERMINAL_HOLD_TICKS'
  'TERMINAL_WINNER'
  'deterministic_autonomous_bot_skirmish_timeline'
  'spawn_and_mine'
  'scout_beacons'
  'army_production_rally'
  'beacon_fight'
  'terminal_resolution'
  'rts_autonomous_bot_core_frame_order_gate'
  'rts_autonomous_bot_core_headless_replay_gate'
  'autonomous_bot_skirmish_gate'
  'forced_capture_hook_enabled'
  'forced_surrender_hook_enabled'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS autonomous bot skirmish source line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS autonomous bot skirmish evidence remains bound to economy, scouting, Beacon control, production, combat, terminal hold, no player input, no forced hooks, and renderer overlays"
