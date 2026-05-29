#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-owned-replay-file.json"
REPLAY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-owned-replay-file.trnm-replay.json"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-owned-replay-file "$REPLAY" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_owned_replay_file_v1"
  and .green == true
  and .replay_format == "trnm_owned_replay_v1_json"
  and (.replay_file_sha256 | length) == 64
  and .replay_file_sha256 == .replay_file_sha256_readback
  and (.map_sha256 | length) == 64
  and (.rules_sha256 | length) == 64
  and .recorded_input_count >= 6
  and .playback_checkpoint_count == .recorded_input_count
  and .checksum_mismatch_count == 0
  and .source_outcome.winner == "Multi2"
  and .playback_outcome.winner == "Multi2"
  and .source_outcome.final_match_result_state == "victory:organic_terminal_observed:Multi2"
  and .playback_outcome.final_match_result_state == "victory:organic_terminal_observed:Multi2"
  and .playback_outcome.final_tick >= 3000
  and .playback_outcome.winner_count == 1
  and .playback_outcome.loser_count == 3
  and .final_source_checkpoint_sha256 == .final_playback_checkpoint_sha256
  and .gap_state == "bevy_owned_replay_file_created_not_openra_replay_parity"
  and .prior_gap_state == "bevy_replay_metric_vocabulary_not_openra_replay_file"
  and .openra_gap_not_closed_gate == true
  and .bevy_owned_replay_file_claimed == true
  and .bevy_openra_replay_file_claimed == false
  and .bevy_openra_parity_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .replay_file_write_gate == true
  and .replay_file_read_gate == true
  and .replay_file_parse_gate == true
  and .replay_file_format_gate == true
  and .replay_file_hash_gate == true
  and .map_rules_hash_gate == true
  and .recorded_input_gate == true
  and .checkpoint_checksum_gate == true
  and .playback_outcome_gate == true
  and .bridge_gate == true
  and .source_gate == true
  and .gap_visibility_gate == true
  and .no_openra_parity_claim_gate == true
  and .owned_replay_file_gate == true
' "$SUMMARY" >/dev/null

jq -e '
  .format == "trnm_owned_replay_v1_json"
  and .contract_version == "trillionnium_world_bevy_classic_rts_owned_replay_file_v1"
  and .producer == "trnm-world-bevy"
  and .engine_id == "bevy_native_client_v1"
  and .seed == 2026052901
  and (.map_sha256 | length) == 64
  and (.rules_sha256 | length) == 64
  and .recorded_input_count >= 6
  and (.recorded_inputs | length) == .recorded_input_count
  and (.recorded_inputs | map(select(.kind == "rts_owned_replay_checkpoint")) | length) == .recorded_input_count
  and .source_outcome.winner == "Multi2"
  and .source_outcome.final_match_result_state == "victory:organic_terminal_observed:Multi2"
  and .boundary.bevy_owned_replay_file_claimed == true
  and .boundary.bevy_openra_replay_file_claimed == false
  and .boundary.bevy_openra_parity_claimed == false
  and .boundary.android_s5_real_device_claimed == false
  and .boundary.public_launch_ready == false
' "$REPLAY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OWNED_REPLAY_FILE_GREEN %s %s\n' "$SUMMARY" "$REPLAY"
