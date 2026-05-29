#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_owned_replay_file.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_owned_replay_file_v1'
  'bevy-classic-rts-owned-replay-file.json'
  'bevy-classic-rts-owned-replay-file.trnm-replay.json'
  'classic-rts-owned-replay-file'
  'trnm_owned_replay_v1_json'
  'bevy_owned_replay_file_created_not_openra_replay_parity'
  'bevy_replay_metric_vocabulary_not_openra_replay_file'
  'bevy_owned_replay_file_claimed == true'
  'bevy_openra_replay_file_claimed == false'
  'bevy_openra_parity_claimed == false'
  'owned_replay_file_gate == true'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS owned replay file script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OWNED_REPLAY_FILE_CONTRACT'
  'native_classic_rts_owned_replay_file_evidence_json'
  'classic-rts-owned-replay-file'
  'trnm_owned_replay_v1_json'
  'rts_owned_replay_checkpoint'
  'checkpoint_checksum_gate'
  'playback_outcome_gate'
  'bevy_owned_replay_file_created_not_openra_replay_parity'
  'bevy_owned_replay_file_claimed'
  'bevy_openra_replay_file_claimed'
  'owned_replay_file_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS owned replay file source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_owned_replay_file_contract_guard'
  'bevy_classic_rts_owned_replay_file_gate'
  'bevy_classic_rts_owned_replay_file_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_owned_replay_file.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI owned replay file line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS owned replay file writes and replays Trillionnium-owned replay evidence while preserving OpenRA parity boundaries"
