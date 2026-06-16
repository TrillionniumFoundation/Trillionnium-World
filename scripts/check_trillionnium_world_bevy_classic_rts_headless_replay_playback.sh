#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-headless-replay-playback.json"
REPLAY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-owned-replay-file.trnm-replay.json"
mkdir -p "$(dirname "$SUMMARY")"

if [[ ! -s "$REPLAY" ]]; then
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_owned_replay_file.sh" >/dev/null
fi

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-headless-replay-playback "$REPLAY" >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_headless_replay_playback_v1"
  and .green == true
  and .replay_contract_version == "trillionnium_world_bevy_classic_rts_owned_replay_file_v1"
  and .replay_format == "trnm_owned_replay_v1_json"
  and (.replay_file_sha256 | length) == 64
  and .headless_playback_mode == "owned_replay_checkpoint_reducer_no_render_no_wgpu"
  and .classic_draw_scene_used == false
  and .rendered_frame_count == 0
  and .no_render_frame_count == 0
  and .wgpu_required == false
  and (.map_sha256 | length) == 64
  and (.rules_sha256 | length) == 64
  and .recorded_input_count >= 6
  and .headless_checkpoint_count == .recorded_input_count
  and .checksum_mismatch_count == 0
  and .source_outcome.winner == "Multi2"
  and .headless_outcome.winner == "Multi2"
  and .source_outcome.final_match_result_state == "victory:organic_terminal_observed:Multi2"
  and .headless_outcome.final_match_result_state == "victory:organic_terminal_observed:Multi2"
  and .headless_outcome.final_tick >= 3000
  and .headless_outcome.winner_count == 1
  and .headless_outcome.loser_count == 3
  and .final_source_checkpoint_sha256 == .final_headless_checkpoint_sha256
  and .prior_gap_state == "bevy_endurance_vocabulary_not_openra_headless_client_match"
  and .gap_state == "bevy_owned_headless_replay_playback_created_not_openra_headless_parity"
  and .bevy_owned_replay_file_claimed == true
  and .bevy_headless_replay_playback_claimed == true
  and .bevy_openra_replay_file_claimed == false
  and .bevy_openra_headless_client_match_claimed == false
  and .bevy_openra_parity_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .replay_file_read_gate == true
  and .replay_file_parse_gate == true
  and .replay_file_contract_gate == true
  and .replay_file_hash_gate == true
  and .map_rules_hash_gate == true
  and .headless_input_gate == true
  and .tick_monotonic_gate == true
  and .no_render_path_gate == true
  and .headless_checksum_gate == true
  and .headless_outcome_gate == true
  and .source_headless_match_gate == true
  and .owned_replay_boundary_gate == true
  and .no_openra_headless_parity_claim_gate == true
  and .headless_replay_playback_gate == true
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_HEADLESS_REPLAY_PLAYBACK_GREEN %s %s\n' "$SUMMARY" "$REPLAY"
