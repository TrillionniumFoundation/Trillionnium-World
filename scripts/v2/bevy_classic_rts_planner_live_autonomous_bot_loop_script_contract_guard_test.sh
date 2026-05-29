#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_planner_live_autonomous_bot_loop.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_planner_live_autonomous_bot_loop_v1'
  'bevy-classic-rts-planner-live-autonomous-bot-loop.json'
  'bevy-classic-rts-planner-live-autonomous-bot-loop'
  'planner-live-autonomous-bot-loop.decisions.json'
  'classic-rts-planner-live-autonomous-bot-loop'
  'bevy_planner_drives_live_autonomous_bot_timeline_not_openra_bot_match'
  'stabilize_macro_workers'
  'scout_resource_beacons'
  'confirm_enemy_pressure_lane'
  'unlock_tier_two_tech'
  'transition_siege_push'
  'terminal_contract_alignment'
  'planner_live_autonomous_bot_loop_gate == true'
  'bevy_planner_live_autonomous_bot_loop_claimed == true'
  'bevy_openra_live_bot_match_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS planner live autonomous bot loop script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PLANNER_LIVE_AUTONOMOUS_BOT_LOOP_CONTRACT'
  'native_classic_rts_planner_live_autonomous_bot_loop_evidence_json'
  'classic-rts-planner-live-autonomous-bot-loop'
  'planner-live-autonomous-bot-loop.decisions.json'
  'bevy_planner_drives_live_autonomous_bot_timeline_not_openra_bot_match'
  'stabilize_macro_workers'
  'scout_resource_beacons'
  'confirm_enemy_pressure_lane'
  'unlock_tier_two_tech'
  'transition_siege_push'
  'terminal_contract_alignment'
  'decision_log_sha256'
  'decision_mapping_gate'
  'live_timeline_gate'
  'terminal_alignment_gate'
  'decision_log_replay_gate'
  'planner_live_autonomous_bot_loop_gate'
  'bevy_openra_live_bot_match_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS planner live autonomous bot loop source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_planner_live_autonomous_bot_loop.sh'
  'bevy_classic_rts_planner_live_autonomous_bot_loop_contract_guard'
  'bevy_classic_rts_planner_live_autonomous_bot_loop_gate'
  'bevy_classic_rts_planner_live_autonomous_bot_loop_script_contract_guard_test.sh'
  'trillionnium_world_bevy_classic_rts_planner_live_autonomous_bot_loop_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI planner live autonomous bot loop line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS planner live autonomous bot loop maps planner phases to autonomous bot stages, writes a replayable decision log, and keeps OpenRA/public-launch claims blocked"
