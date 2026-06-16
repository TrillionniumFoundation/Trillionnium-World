#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_screen_for_screen_ui_replication.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

test -x "$SCRIPT"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_screen_for_screen_ui_replication_v1'
  'bevy-classic-rts-openra-screen-for-screen-ui-replication.json'
  'bevy-classic-rts-openra-screen-for-screen-ui-replication.ppm'
  'classic-rts-openra-screen-for-screen-ui-replication'
  'screen_for_screen_mode == "openra_style_widget_root_screen_set_and_interaction_surface_replication_original_trillionnium_art"'
  'runtime_screen_mode == "player_runtime_openra_style_ingame_screen_set"'
  'openra_reference_screen_count == 8'
  'replicated_interaction_surface_count == 8'
  'openra_style_widget_root_screen_set_claimed == true'
  'screen_for_screen_openra_ui_claimed == false'
  'openra_screen_for_screen_ui_replication_claimed == false'
  'openra_pixel_perfect_asset_parity_claimed == false'
  'openra_engine_port_claimed == false'
  'openra_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_SCREEN_FOR_SCREEN_UI_REPLICATION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing OpenRA screen-for-screen UI replication script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_SCREEN_FOR_SCREEN_UI_REPLICATION_CONTRACT'
  'native_classic_rts_openra_screen_for_screen_ui_replication_evidence_json'
  'classic-rts-openra-screen-for-screen-ui-replication'
  'MAINMENU_shellmap_root'
  'SKIRMISH_mission_browser'
  'MULTIPLAYER_server_browser'
  'LOBBY_setup_room'
  'LOADING_briefing_progress'
  'INGAME_ROOT_sidebar_hud'
  'PAUSE_options_overlay'
  'POSTGAME_statistics'
  'ShellmapRoot=MAINMENU'
  'IngameRoot=INGAME_ROOT'
  'player_runtime_openra_style_ingame_screen_set'
  'player_first_openra_style_ingame_screen_gate'
  'openra_screen_for_screen_ui_replication_gate'
  'openra_style_widget_root_screen_set_claimed'
  'screen_for_screen_openra_ui_claimed'
  'openra_pixel_perfect_asset_parity_claimed'
  'openra_engine_port_claimed'
  'openra_asset_copied'
  'https://docs.openra.net/en/playtest/traits/#loadwidgetatgamestart'
  'https://github.com/OpenRA/OpenRA'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing OpenRA screen-for-screen UI replication source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_openra_screen_for_screen_ui_replication.sh'
  'bevy-classic-rts-openra-screen-for-screen-ui-replication.json'
  'bevy-classic-rts-openra-screen-for-screen-ui-replication.ppm'
  'classic_rts_openra_screen_for_screen_ui_replication_green'
  'rts_openra_screen_for_screen_ui_replication_gate'
  'rts_openra_screen_for_screen_ui_replication_screen_count'
  'rts_openra_screen_for_screen_ui_replication_style_screen_set_claimed'
  'rts_openra_screen_for_screen_ui_replication_claimed'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing OpenRA screen-for-screen UI replication readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_openra_screen_for_screen_ui_replication.sh'
  'bevy_classic_rts_openra_screen_for_screen_ui_replication_script_contract_guard_test.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing OpenRA screen-for-screen UI replication release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] OpenRA screen-for-screen UI replication gate remains connected to Rust CLI, playtest readiness, release-review CI, and no-asset-copy boundaries"
