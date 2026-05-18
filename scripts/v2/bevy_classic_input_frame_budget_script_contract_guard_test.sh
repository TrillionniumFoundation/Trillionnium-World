#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_input_frame_budget.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_script_lines=(
  'trillionnium_world_bevy_classic_input_frame_budget_v1'
  'bevy-classic-input-frame-budget.json'
  'classic-input-frame-budget'
  'accepted_input_count == 96'
  'response_p95_budget_gate'
  'response_max_budget_gate'
  'cex_runtime_player_client_allowed == false'
  'wgpu_required == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing input-frame budget script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_INPUT_FRAME_BUDGET_CONTRACT'
  'native_classic_input_frame_budget_evidence_json'
  'NativeControlAction::Move -> apply_live_native_action -> classic_draw_scene'
  'p95_budget_micros = 20_000_u64'
  'max_budget_micros = 50_000_u64'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing input-frame budget source line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic input-frame budget script keeps the movement responsiveness contract"
