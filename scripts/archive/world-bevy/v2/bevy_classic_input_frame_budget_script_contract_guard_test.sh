#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_input_frame_budget.sh"
RENDER_SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_render_budget.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
BUDGET_SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/classic_budget_renderer.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_script_lines=(
  'trillionnium_world_bevy_classic_input_frame_budget_v1'
  'bevy-classic-input-frame-budget.json'
  'classic-input-frame-budget'
  'classic_input_frame_budget_green'
  'ready_for_release_review == true'
  'gate_count == 7'
  'sample_detail_count == (.samples | length)'
  'accepted_direction_count == (.accepted_directions | length)'
  'selected_frame_id_count == (.selected_frame_ids | length)'
  'nonblank_sample_count == (.nonblank_samples | length)'
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

required_render_script_lines=(
  'trillionnium_world_bevy_classic_render_budget_v1'
  'bevy-classic-render-budget.json'
  'classic-render-budget'
  'classic_render_budget_green'
  'ready_for_release_review == true'
  'gate_count == 5'
  'nonblank_sample_count == (.nonblank_samples | length)'
  'p95_budget_micros == 16000'
  'max_budget_micros == 40000'
  'cex_runtime_player_client_allowed == false'
  'wgpu_required == false'
)

for line in "${required_render_script_lines[@]}"; do
  if ! grep -Fq "$line" "$RENDER_SCRIPT"; then
    echo "[FAIL] missing render budget script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'mod classic_budget_renderer;'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_INPUT_FRAME_BUDGET_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RENDER_BUDGET_CONTRACT'
  'classic_budget_renderer::native_classic_input_frame_budget_evidence_json()'
  'classic_budget_renderer::native_classic_render_budget_evidence_json()'
  'pub(super) fn native_classic_input_frame_budget_evidence_json'
  'pub(super) fn native_classic_render_budget_evidence_json'
  'native_classic_input_frame_budget_evidence_json'
  'native_classic_render_budget_evidence_json'
  'NativeControlAction::Move -> apply_live_native_action -> classic_draw_scene'
  'Classic render budget measures repeated low-spec classic_draw_scene CPU frames'
  'classic_draw_scene(&mut buffer'
  'p95_budget_micros = 20_000_u64'
  'max_budget_micros = 50_000_u64'
  'p95_budget_micros = 16_000_u64'
  'max_budget_micros = 40_000_u64'
  'response_p95_budget_gate'
  'response_max_budget_gate'
  'p95_budget_gate'
  'max_budget_gate'
  'nonblank_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$BUDGET_SOURCE" "$MAIN"; then
    echo "[FAIL] missing input-frame budget source line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic input/render budget scripts keep movement responsiveness and low-spec renderer budget contracts"
