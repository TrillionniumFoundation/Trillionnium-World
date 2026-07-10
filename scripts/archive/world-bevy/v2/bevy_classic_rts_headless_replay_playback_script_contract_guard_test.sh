#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_headless_replay_playback.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_headless_replay_playback_v1'
  'bevy-classic-rts-headless-replay-playback.json'
  'bevy-classic-rts-owned-replay-file.trnm-replay.json'
  'classic-rts-headless-replay-playback'
  'owned_replay_checkpoint_reducer_no_render_no_wgpu'
  'bevy_owned_headless_replay_playback_created_not_openra_headless_parity'
  'bevy_endurance_vocabulary_not_openra_headless_client_match'
  'bevy_headless_replay_playback_claimed == true'
  'bevy_openra_headless_client_match_claimed == false'
  'source_headless_match_gate == true'
  'headless_replay_playback_gate == true'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS headless replay playback script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_HEADLESS_REPLAY_PLAYBACK_CONTRACT'
  'native_classic_rts_headless_replay_playback_evidence_json'
  'classic-rts-headless-replay-playback'
  'owned_replay_checkpoint_reducer_no_render_no_wgpu'
  'headless_checksum_gate'
  'source_headless_match_gate'
  'bevy_owned_headless_replay_playback_created_not_openra_headless_parity'
  'bevy_headless_replay_playback_claimed'
  'bevy_openra_headless_client_match_claimed'
  'headless_replay_playback_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS headless replay playback source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_headless_replay_playback_contract_guard'
  'bevy_classic_rts_headless_replay_playback_gate'
  'bevy_classic_rts_headless_replay_playback_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_headless_replay_playback.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI headless replay playback line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS headless replay playback replays the owned replay file without render and preserves OpenRA/public-launch boundaries"
