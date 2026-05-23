#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-input-frame-budget.json"
mkdir -p "$(dirname "$SUMMARY")"

budget_green() {
  test -s "$SUMMARY"
  jq -e '
  .contract_version == "trillionnium_world_bevy_classic_input_frame_budget_v1"
  and .green == true
  and .sample_count == 96
  and .accepted_input_count == 96
  and .accepted_input_gate == true
  and .direction_coverage_gate == true
  and .response_p95_budget_gate == true
  and .response_max_budget_gate == true
  and .p95_micros <= 20000
  and .max_micros <= 50000
  and .rendered_frame_gate == true
  and .selected_frame_manifest_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null
}

for attempt in 1 2 3; do
  (
    cd "$ROOT/trillionnium"
    CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" \
      cargo run -p trnm-world-bevy -- classic-input-frame-budget >"$SUMMARY"
  )
  if budget_green; then
    printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_INPUT_FRAME_BUDGET_GREEN %s\n' "$SUMMARY"
    exit 0
  fi
  if [[ "$attempt" != "3" ]]; then
    sleep 1
  fi
done

budget_green

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_INPUT_FRAME_BUDGET_GREEN %s\n' "$SUMMARY"
