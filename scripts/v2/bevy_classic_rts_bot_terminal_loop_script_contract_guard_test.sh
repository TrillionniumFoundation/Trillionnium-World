#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_terminal_loop.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_bot_terminal_loop_v1'
  'bevy-classic-rts-bot-terminal-loop.json'
  'bevy-classic-rts-bot-terminal-loop.ppm'
  'classic-rts-bot-terminal-loop'
  'input_action_count == 0'
  'no_live_player_input_gate == true'
  'bot_slot_count == 4'
  'bevy_terminal_winner_beacons == 2'
  'bevy_terminal_total_beacons == 4'
  'bevy_terminal_hold_ticks == 3000'
  'bevy_terminal_parity_claimed == false'
  'openra_parity_target_commit == "5f1bf76"'
  'forced_capture_hook_enabled == false'
  'forced_surrender_hook_enabled == false'
  'bevy_terminal_rule_simulation_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS bot terminal loop script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'native_classic_rts_bot_terminal_loop_evidence_json'
  'classic-rts-bot-terminal-loop'
  'BOT_SLOT_COUNT'
  'TOTAL_BEACONS'
  'WINNER_BEACONS'
  'TERMINAL_HOLD_TICKS'
  'TERMINAL_WINNER'
  'deterministic_bot_terminal_rule_simulation'
  'bot_control_2_of_4_flux_beacons_for_3000_ticks'
  'bevy_terminal_rule_simulation_gate'
  'forced_capture_hook_enabled'
  'forced_surrender_hook_enabled'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS bot terminal loop source line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot terminal loop evidence remains bound to four bots, four beacons, 2-of-4 terminal hold, no player input, no forced hooks, and renderer overlays"
