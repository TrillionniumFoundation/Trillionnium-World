#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_launcher.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE_REVIEW="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_playtest_launcher_v1'
  'trillionnium_world_bevy_classic_rts_campaign_entry_v1'
  'trillionnium_world_bevy_classic_playtest_runner_status_v1'
  'bevy-classic-playtest-launcher.json'
  'check_trillionnium_world_bevy_classic_rts_campaign_entry.sh'
  'check_trillionnium_world_bevy_classic_playtest_runner_status.sh'
  'CAMPAIGN:START'
  'CAMPAIGN:CONTINUE'
  'CAMPAIGN:REPLAY'
  'CONTINUE:SESSION'
  'input_action_count == 73'
  'campaign_slot_bytes > 20000'
  'final_current_room_id == "league-coliseum"'
  'final_contextual_primary_action_label == "COMBAT:attack"'
  'player_launch_ready_gate'
  'cex_path_gate'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYTEST_LAUNCHER_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic playtest launcher script line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_playtest_launcher.sh'
  'bevy-classic-playtest-launcher.json'
  'playtest_launcher_green'
  'launcher_player_launch_ready_gate'
  'launcher_campaign_slot_gate'
  'launcher_service_process_gate'
  'launcher_release_binary_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic playtest launcher readiness line: $line" >&2
    exit 1
  fi
done

required_release_review_lines=(
  'check_trillionnium_world_bevy_classic_playtest_launcher.sh'
  'bevy_classic_playtest_launcher_contract_guard'
  'bevy_classic_playtest_launcher_gate'
  'bevy_classic_playtest_launcher_script_contract_guard_test.sh'
  'trillionnium_world_bevy_classic_playtest_launcher_v1'
)

for line in "${required_release_review_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_REVIEW" "$ROOT/scripts/v2/release_review_ci_gate_script_contract_guard_test.sh"; then
    echo "[FAIL] missing classic playtest launcher release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic playtest launcher evidence remains connected to campaign entry, live runner status, readiness, and release-review CI"
