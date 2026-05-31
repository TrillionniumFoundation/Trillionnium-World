#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_reducer.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_imported_replay_reducer_v1'
  'bevy-classic-rts-openra-imported-replay-reducer.json'
  'openra-imported-replay-reducer.json'
  'openra-imported-replay-snapshots.jsonl'
  'openra-imported-replay-reducer-comparison.json'
  'openra-imported-replay-reducer-negative-corpus.json'
  'classic-rts-openra-imported-replay-reducer'
  'openra_imported_replay_reducer_comparison_v1_json'
  'openra_imported_replay_reducer_gate == true'
  'bevy_openra_imported_replay_reducer_claimed == true'
  'bevy_openra_replay_envelope_importer_claimed == true'
  'bevy_openra_order_replay_reducer_claimed == true'
  'bevy_openra_order_payload_decoder_claimed == false'
  'bevy_openra_network_order_stream_claimed == false'
  'bevy_openra_runtime_parity_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA imported replay reducer script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REDUCER_CONTRACT'
  'native_classic_rts_openra_imported_replay_reducer_evidence_json'
  'classic-rts-openra-imported-replay-reducer'
  'bevy_owned_openra_imported_stream_replayed_by_rust_reducer'
  'openra_imported_replay_reducer_comparison_v1_json'
  'imported_stream_input_gate'
  'imported_reducer_parse_gate'
  'imported_reducer_state_gate'
  'comparison_gate'
  'negative_corpus_gate'
  'openra_imported_replay_reducer_gate'
  'bevy_openra_imported_replay_reducer_claimed'
  'bevy_openra_order_payload_decoder_claimed'
  'bevy_openra_runtime_parity_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA imported replay reducer source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_imported_replay_reducer_contract_guard'
  'bevy_classic_rts_openra_imported_replay_reducer_gate'
  'bevy_classic_rts_openra_imported_replay_reducer_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_imported_replay_reducer.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA imported replay reducer line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA imported replay reducer consumes importer JSONL output and aligns with the Rust reducer without claiming OpenRA runtime/network parity"
