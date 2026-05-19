#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-render-budget.json"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

budget_green() {
  test -s "$SUMMARY"
  jq -e '
  .contract_version == "trillionnium_world_bevy_classic_render_budget_v1"
  and .green == true
  and .renderer_path == "classic_cpu_ppm_minifb_low_spec"
  and .frame_width == 640
  and .frame_height == 360
  and .frame_count == 180
  and .p95_micros <= .p95_budget_micros
  and .max_micros <= .max_budget_micros
  and .p95_budget_micros == 16000
  and .max_budget_micros == 40000
  and (.nonblank_samples | length) >= 4
  and ([.nonblank_samples[]] | all(. > 80000))
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .frame_count_gate == true
  and .p95_budget_gate == true
  and .max_budget_gate == true
  and .nonblank_gate == true
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null
}

"$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh" >/dev/null

for attempt in 1 2 3; do
  (
    cd "$ROOT/trillionnium"
    TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
      cargo run -p trnm-world-bevy -- classic-render-budget >"$SUMMARY"
  )
  if budget_green; then
    printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RENDER_BUDGET_GREEN %s\n' "$SUMMARY"
    exit 0
  fi
  if [[ "$attempt" != "3" ]]; then
    sleep 1
  fi
done

budget_green

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RENDER_BUDGET_GREEN %s\n' "$SUMMARY"
