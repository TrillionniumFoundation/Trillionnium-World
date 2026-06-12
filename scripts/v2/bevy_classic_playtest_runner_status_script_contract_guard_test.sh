#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_runner_status.sh"

required_lines=(
  'trillionnium_world_bevy_classic_playtest_runner_status_v1'
  'bevy-classic-playtest-runner-status.json'
  'trillionnium-bevy-playtest.service'
  'target/release/trnm-world-bevy'
  'TRNM_WORLD_BEVY_LOW_SPEC'
  'TRNM_WORLD_BEVY_CLASSIC_RENDERER'
  'TRNM_WORLD_BEVY_CLASSIC_FPS'
  'TRNM_WORLD_BEVY_CLASSIC_PLAYER_SCREEN'
  'WINIT_UNIX_BACKEND'
  'TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST'
  'service_process_gate'
  'release_binary_gate'
  'classic_env_gate'
  'player_screen_env_gate'
  'trillionnium_world_bevy_classic_player_screen_runner_visual_v1'
  'bevy-classic-player-screen-runner-status.png'
  'player_screen_window_gate'
  'player_screen_title_gate'
  'player_screen_proof_debug_absent_gate'
  'player_screen_screenshot_gate'
  'player_screen_region_gate'
  'player_screen_visual_gate'
  'room=first-contact-basin'
  'map/HUD/command pixels'
  'proof/debug default screens are explicitly rejected'
  'x11_backend_gate'
  'cex_path_gate'
  'CEX paths are explicitly rejected'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing runner status contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic playtest runner status script keeps the live Bevy runner contract"
