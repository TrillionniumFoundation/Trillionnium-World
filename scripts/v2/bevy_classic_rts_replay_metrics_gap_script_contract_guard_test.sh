#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_replay_metrics_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_replay_metrics_gap_v1'
  'bevy-classic-rts-replay-metrics-gap.json'
  'bevy-classic-rts-replay-metrics-gap.ppm'
  'classic-rts-replay-metrics-gap'
  'bevy_replay_metric_vocabulary_not_openra_replay_file'
  'bevy_replay_file_claimed == false'
  'bevy_replay_parity_claimed == false'
  'openra_replay_summary_target_commit == "d5ceade"'
  'openra_battle_outcome_target_commit == "9b2664b"'
  'replay_startgame_order == true'
  'winner_claimed == false'
  'replay_metrics_gap_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS replay metrics gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_REPLAY_METRICS_GAP_CONTRACT'
  'native_classic_rts_replay_metrics_gap_evidence_json'
  'classic-rts-replay-metrics-gap'
  'startgame_order'
  'outcome_summary'
  'bevy_replay_metric_vocabulary_not_openra_replay_file'
  'OPENRA_REPLAY_SUMMARY_COMMIT'
  'OPENRA_BATTLE_OUTCOME_COMMIT'
  'replay_actor_mix'
  'battle_outcome_summary_gate'
  'replay_metrics_gap_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS replay metrics gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_replay_metrics_gap.sh'
  'bevy-classic-rts-replay-metrics-gap.json'
  'classic_rts_replay_metrics_gap_green'
  'rts_replay_metrics_gap_stage_count'
  'rts_replay_metrics_gap_token_gate'
  'rts_replay_metrics_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS replay metrics gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS replay metrics gap evidence remains bound to OpenRA replay/outcome token metrics while keeping Bevy replay parity unclaimed"
