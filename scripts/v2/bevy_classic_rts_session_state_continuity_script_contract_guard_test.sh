#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_session_state_continuity.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-session-state-continuity'
  'bevy-classic-rts-session-state-continuity.json'
  'bevy-classic-rts-session-state-continuity.ppm'
  'trillionnium_world_bevy_classic_rts_session_state_continuity_v1'
  'SUMMARY_RAW="$(mktemp "${SUMMARY}.raw.XXXXXX")"'
  'SUMMARY_TMP="$(mktemp "${SUMMARY}.tmp.XXXXXX")"'
  'source_contract_count = (.source_contracts | keys | length)'
  'source_path_count = (.source_paths | keys | length)'
  'source_headline_field_count = (.source_headline | keys | length)'
  'runtime_screen_layout_count = (.runtime_screen_layout | keys | length)'
  'trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1'
  'trillionnium_world_bevy_session_slot_confirm_v1'
  'trillionnium_world_bevy_session_load_resume_v1'
  'trillionnium_world_bevy_session_recovery_ui_v1'
  'trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1'
  'trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1'
  'trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1'
  'trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1'
  'trnm_rts_evidence_session_state_continuity_review_v1'
  'rts_evidence_session_state_continuity_review.green == true'
  'rts_evidence_session_state_continuity_review.shell_meta_contract == "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1"'
  'rts_evidence_session_state_continuity_review.session_slot_confirm_contract == "trillionnium_world_bevy_session_slot_confirm_v1"'
  'rts_evidence_session_state_continuity_review.session_state_continuity_gate == true'
  'rts_evidence_session_state_continuity_review.input_path | contains("session-state continuity source JSON")'
  'rts_evidence_session_state_continuity_review_gate == true'
  'runtime_screen_mode == "player_runtime_session_resume_screen"'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'runtime_screen_layout.resume_chain_lane'
  'runtime_screen_layout.primary_tactical_viewport'
  'runtime_screen_layout.hud_restore_panel'
  'state_continuity_surface_count == 8'
  'state_continuity_surface_name_count == (.state_continuity_surface_names | length)'
  'state_continuity_slot_id_count == (.state_continuity_slot_ids | length)'
  'state_continuity_source_surface_count == (.state_continuity_source_surfaces | length)'
  'state_continuity_pixel_count_field_count == (.state_continuity_pixel_counts | keys | length)'
  'resume_chain_count == (.resume_chain | length)'
  'gate_count == 16'
  'passed_gate_count == 16'
  'failed_gate_count == 0'
  'player_first_session_resume_screen_gate == true'
  'session_state_continuity_gate == true'
  'external_evidence_ignored_for_current_replication_pass == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SESSION_STATE_CONTINUITY_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS session state continuity script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SESSION_STATE_CONTINUITY_CONTRACT'
  'native_classic_rts_session_state_continuity_evidence_json'
  'TRNM RUST/BEVY SESSION RESUME PLAYER SCREEN'
  'MATCH SETUP SNAPSHOT'
  'SESSION SLOT WRITE'
  'LOAD RESUME LOCK'
  'CONTINUE UNLOCK'
  'IN-MATCH HUD RESTORE'
  'OUTCOME REWARD STATE'
  'OPEN-WORLD RESUME'
  'RECOVERY UI GUARD'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SHELL_META_UI_REPLICATION_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_SESSION_SLOT_CONFIRM_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_SESSION_LOAD_RESUME_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_SESSION_RECOVERY_UI_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MATCH_SETUP_UI_REPLICATION_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_IN_MATCH_HUD_STATE_REPLICATION_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_OUTCOME_UI_READINESS_CONTRACT'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_UI_CONTINUITY_CONTRACT'
  'TRNM_RTS_EVIDENCE_SESSION_STATE_CONTINUITY_REVIEW_CONTRACT'
  'rts_session_state_continuity_review'
  'rts_evidence_session_state_continuity_review'
  'player_runtime_session_resume_screen'
  'player_runtime_in_match_hud_screen'
  'runtime_screen_layout'
  'evidence_board_only'
  'resume_chain_lane'
  'primary_tactical_viewport'
  'player_first_session_resume_screen_gate'
  'player_first_resume_view_non_background'
  'hud_restore_panel'
  'runtime_screen_gate'
  'session_state_continuity_gate'
  'external_evidence_ignored_for_current_replication_pass'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS session state continuity source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_session_state_continuity.sh'
  'rts_session_state_continuity'
  'classic_rts_session_state_continuity_green'
  'rts_session_state_continuity_player_first_session_resume_screen_gate'
  'rts_session_state_continuity_player_first_resume_view_non_background'
  'bevy-classic-rts-session-state-continuity.json'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS session state continuity readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_session_state_continuity.sh'
  'bevy_classic_rts_session_state_continuity_script_contract_guard_test.sh'
  'bevy_classic_rts_session_state_continuity_gate'
  'trillionnium_world_bevy_classic_rts_session_state_continuity_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing classic RTS session state continuity release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS session state continuity gate remains connected to Rust CLI, save/load resume sources, playtest readiness, release-review CI, and no-external-evidence boundaries"
