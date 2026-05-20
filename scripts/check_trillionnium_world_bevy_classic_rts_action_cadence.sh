#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-cadence.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-cadence.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-action-cadence "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_action_cadence_v1"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 720
  and .renderer_path == "classic_draw_scene"
  and (.stage_summaries | length) == 6
  and ([.stage_summaries[] | select(.stage == "guard_windup" and .event == "guard_attack_windup")] | length) == 1
  and ([.stage_summaries[] | select(.stage == "guard_strike" and .event == "guard_attack_strike")] | length) == 1
  and ([.stage_summaries[] | select(.stage == "creep_recovery" and .event == "creep_attack_recovery")] | length) == 1
  and ([.stage_summaries[] | select(.stage == "worker_carry_bob" and .event == "worker_carry_bob")] | length) == 1
  and ([.stage_summaries[] | select(.stage == "idle_breathing_line" and .event == "neutral_idle_breathing")] | length) == 1
  and .windup_pixel_count > 140
  and .strike_pixel_count > 220
  and .recovery_pixel_count > 140
  and .carry_bob_pixel_count > 70
  and .idle_breath_pixel_count > 40
  and .shadow_smear_pixel_count > 100
  and .windup_gate == true
  and .strike_gate == true
  and .recovery_gate == true
  and .carry_bob_gate == true
  and .idle_breath_gate == true
  and .shadow_smear_gate == true
  and .scene_renderer_gate == true
  and .event_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_ACTION_CADENCE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
