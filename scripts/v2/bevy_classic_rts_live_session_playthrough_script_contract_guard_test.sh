#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_live_session_playthrough.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

test -x "$SCRIPT"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_live_session_playthrough_v1'
  'bevy-classic-rts-live-session-playthrough.json'
  'bevy-classic-rts-live-session-playthrough.ppm'
  'bevy-classic-rts-live-session-playthrough.trace.json'
  'bevy-classic-rts-live-session-playthrough-slots'
  'TRNM_WORLD_BEVY_SESSION_SLOT_DIR'
  'classic-rts-live-session-playthrough'
  'classic_rts_live_session_seed_v1'
  'same_process_session_playthrough == true'
  'apply_native_first_playable_action + apply_live_native_action_with_source(classic_rts_live_session_playthrough_input)'
  'stage_count == 6'
  'campaign_handoff_input_count >= 70'
  'live_command_input_count == 5'
  'slot_a_bytes > 10000'
  'runtime_screen_mode == "player_runtime_live_session_playthrough_screen"'
  'runtime_screen_gate == true'
  'player_first_live_session_screen_gate == true'
  'player_first_live_view_non_background > 250000'
  'player_first_live_view_frame > 8000'
  'player_first_live_status_strip > 10000'
  'player_first_live_stage_rail > 25000'
  'live_session_playthrough_gate == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LIVE_SESSION_PLAYTHROUGH_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS live session playthrough script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LIVE_SESSION_PLAYTHROUGH_CONTRACT'
  'native_classic_rts_live_session_playthrough_evidence_json'
  'classic-rts-live-session-playthrough'
  'classic_rts_live_session_seed_v1'
  'classic_rts_live_session_playthrough_input'
  'TITLE / ACCOUNT'
  'MATCH SETUP / START'
  'IN-MATCH HUD'
  'COMMAND FEEDBACK'
  'SAVE / LOAD / RESUME'
  'OUTCOME / OPEN WORLD'
  'NativeControlAction::OpenTitleMenu'
  'NativeControlAction::LoginAccountFromTitle'
  'NativeControlAction::StartCampaignFromTitle'
  'NativeControlAction::SaveSelectedSlot'
  'NativeControlAction::LoadSelectedSlot'
  'NativeControlAction::ContinueAfterLoad'
  'same_process_trace_gate'
  'player_runtime_live_session_playthrough_screen'
  'runtime_screen_gate'
  'player_first_live_session_screen_gate'
  'live_session_playthrough_gate'
  'production_ready_ui_claimed'
  'screen_for_screen_openra_ui_claimed'
  'third_party_asset_copied'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS live session playthrough source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_live_session_playthrough.sh'
  'bevy-classic-rts-live-session-playthrough.json'
  'bevy-classic-rts-live-session-playthrough.ppm'
  'bevy-classic-rts-live-session-playthrough.trace.json'
  'classic_rts_live_session_playthrough_green'
  'rts_live_session_playthrough_runtime_screen_gate == true'
  'rts_live_session_playthrough_player_first_live_session_screen_gate == true'
  'rts_live_session_playthrough_player_first_live_view_non_background > 250000'
  'rts_live_session_playthrough_player_first_live_view_frame_pixel_count > 8000'
  'rts_live_session_playthrough_player_first_live_status_strip_pixel_count > 10000'
  'rts_live_session_playthrough_player_first_live_stage_rail_pixel_count > 25000'
  'rts_live_session_playthrough_gate'
  'rts_live_session_playthrough_stage_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS live session playthrough readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_live_session_playthrough.sh'
  'bevy_classic_rts_live_session_playthrough_script_contract_guard_test.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing classic RTS live session playthrough release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS live session playthrough gate remains connected to Rust CLI, same-process trace, playtest readiness, release-review CI, and no-external-evidence boundaries"
