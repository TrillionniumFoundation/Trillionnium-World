#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
RUNNER="$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh"

release_ci_bevy_scripts=(
  "$ROOT/scripts/check_trillionnium_world_bevy_account_client_boundary.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_account_title_flow.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_action_coach.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_asset_store_registration.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_art_pack.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_material_application.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_material_consumption.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_render_frame.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_sprite_sheet.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_authored_texture_atlas_binding.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_player_hud_debug_layer.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_player_ui_rescue.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_render_asset_eligibility.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_asset.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_manifest_probe.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_sprite_asset_binding.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_sprite_texture_sampling.sh"
)

if [[ ! -x "$RUNNER" ]]; then
  echo "[FAIL] artifact wrapper missing or not executable: $RUNNER" >&2
  exit 1
fi

for script in "${release_ci_bevy_scripts[@]}"; do
  if [[ ! -x "$script" ]]; then
    echo "[FAIL] release CI Bevy script missing or not executable: $script" >&2
    exit 1
  fi
  if ! grep -Fq 'run_trillionnium_world_bevy_artifact_command.sh' "$script"; then
    echo "[FAIL] release CI Bevy script does not use artifact wrapper: $script" >&2
    exit 1
  fi
  if grep -Fq 'cargo run -p trnm-world-bevy' "$script"; then
    echo "[FAIL] release CI Bevy script still invokes cargo directly: $script" >&2
    exit 1
  fi
done

printf 'TRILLIONNIUM_BEVY_RELEASE_CI_ARTIFACT_WRAPPER_SCRIPT_CONTRACT_GREEN wrapper_count=%s\n' "${#release_ci_bevy_scripts[@]}"
