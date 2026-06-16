#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
RUNNER="$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh"

classic_foundation_scripts=(
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_preview.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_animation_selector.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack_scene_probe.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_override_probe.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_pack.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_slot_map.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_input_frame_budget.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_isometric_modeling.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_model_catalog.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_player_motion_probe.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_render_budget.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_renderer_probe.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_scene_preview.sh"
)

if [[ ! -x "$RUNNER" ]]; then
  echo "[FAIL] artifact wrapper missing or not executable: $RUNNER" >&2
  exit 1
fi
if ! grep -Fq '[[ "$BIN" != /* && -x "$ROOT/$BIN" ]]' "$RUNNER"; then
  echo "[FAIL] artifact wrapper does not resolve relative artifact paths from repo root" >&2
  exit 1
fi

for script in "${classic_foundation_scripts[@]}"; do
  if [[ ! -x "$script" ]]; then
    echo "[FAIL] classic foundation script missing or not executable: $script" >&2
    exit 1
  fi
  if ! grep -Fq 'run_trillionnium_world_bevy_artifact_command.sh' "$script"; then
    echo "[FAIL] classic foundation script does not use artifact wrapper: $script" >&2
    exit 1
  fi
  if grep -Fq 'cargo run -p trnm-world-bevy' "$script"; then
    echo "[FAIL] classic foundation script still invokes cargo directly: $script" >&2
    exit 1
  fi
done

printf 'TRILLIONNIUM_BEVY_CLASSIC_FOUNDATION_ARTIFACT_WRAPPER_SCRIPT_CONTRACT_GREEN wrapper_count=%s\n' "${#classic_foundation_scripts[@]}"
