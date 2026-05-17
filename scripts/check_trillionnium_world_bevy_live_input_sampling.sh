#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-live-input-sampling.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- live-input-sampling >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_live_input_sampling_v1"
  and .green == true
  and .button_and_keyboard_unified_gate == "apply_live_native_action"
  and .movement_lock_blocks_duplicate_input == true
  and (.input_samples[] | select(.stage == "initial") | .availability[] | select(.action == "TALK") | .enabled) == true
  and (.input_samples[] | select(.stage == "initial") | .availability[] | select(.action == "TRAIN") | .enabled) == false
  and (.input_samples[] | select(.stage == "initial") | .availability[] | select(.action == "FIGHT") | .enabled) == false
  and (.input_samples[] | select(.stage == "after_TALK") | .availability[] | select(.action == "TRAIN") | .enabled) == true
  and (.input_samples[] | select(.stage == "after_TRAIN") | .availability[] | select(.action == "MOVE:north") | .enabled) == true
  and (.input_samples[] | select(.stage == "after_MOVE:north") | .availability[] | select(.action == "FIGHT") | .enabled) == true
  and (.input_samples[] | select(.stage == "after_FIGHT") | .availability[] | select(.action == "COMPLETE") | .enabled) == true
  and (.input_samples[] | select(.stage == "after_COMPLETE") | .availability[] | select(.action == "EQUIP") | .enabled) == true
  and (.button_events | length) >= 6
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_LIVE_INPUT_SAMPLING_GREEN %s\n' "$SUMMARY"
