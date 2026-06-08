#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

test -x "$SCRIPT"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication_v1'
  'bevy-classic-rts-full-game-visual-ui-replication.json'
  'bevy-classic-rts-full-game-visual-ui-replication.ppm'
  'classic-rts-full-game-visual-ui-replication'
  'runtime_screen_mode == "player_runtime_full_game_visual_ui_screen"'
  'coverage_surface_count == 18'
  'source_contract_gate == true'
  'runtime_screen_chain_gate == true'
  'player_flow_gate == true'
  'full_game_visual_ui_replication_gate == true'
  'internal_rust_full_game_visual_ui_replication_claimed == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FULL_GAME_VISUAL_UI_REPLICATION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing full-game visual/UI replication script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FULL_GAME_VISUAL_UI_REPLICATION_CONTRACT'
  'native_classic_rts_full_game_visual_ui_replication_evidence_json'
  'classic-rts-full-game-visual-ui-replication'
  'player_runtime_full_game_visual_ui_screen'
  'title_account_shell'
  'match_setup_start'
  'tactical_viewport'
  'map_minimap_camera'
  'command_grid'
  'session_slot_save_load'
  'open_world_handoff'
  'coverage_surface_count'
  'runtime_screen_chain_gate'
  'player_flow_gate'
  'internal_rust_full_game_visual_ui_replication_claimed'
  'screen_for_screen_openra_ui_claimed'
  'third_party_asset_copied'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing full-game visual/UI replication source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication.sh'
  'bevy-classic-rts-full-game-visual-ui-replication.json'
  'bevy-classic-rts-full-game-visual-ui-replication.ppm'
  'classic_rts_full_game_visual_ui_replication_green'
  'rts_full_game_visual_ui_replication_gate'
  'rts_full_game_visual_ui_replication_surface_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing full-game visual/UI replication readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication.sh'
  'bevy_classic_rts_full_game_visual_ui_replication_script_contract_guard_test.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing full-game visual/UI replication release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] full-game visual/UI replication gate remains connected to Rust CLI, playtest readiness, release-review CI, and no-external-evidence boundaries"
