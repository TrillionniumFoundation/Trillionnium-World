#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_command_vocab_adapter.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_command_vocab_adapter_v1'
  'bevy-classic-rts-openra-command-vocab-adapter.json'
  'openra-command-vocabulary-adapter.json'
  'classic-rts-openra-command-vocab-adapter'
  'openra_replay_command_vocab_adapter_v1_json'
  'StartGame'
  'SyncFrame'
  'BotOrder'
  'TerminalProbe'
  'GameOver'
  'Outcome'
  'openra_command_vocab_adapter_gate == true'
  'openra_order_serializer_claimed == false'
  'openra_network_order_stream_claimed == false'
  'openra_runtime_parity_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA command vocab adapter script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_COMMAND_VOCAB_ADAPTER_CONTRACT'
  'native_classic_rts_openra_command_vocab_adapter_evidence_json'
  'classic-rts-openra-command-vocab-adapter'
  'bevy_owned_replay_to_openra_style_command_vocabulary_adapter_not_binary_openra_replay'
  'openra_replay_command_vocab_adapter_v1_json'
  'command_vocabulary_gate'
  'checkpoint_command_gate'
  'event_command_gate'
  'outcome_command_gate'
  'openra_command_vocab_adapter_gate'
  'bevy_openra_command_vocabulary_adapter_claimed'
  'bevy_openra_order_serializer_claimed'
  'bevy_openra_network_order_stream_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA command vocab adapter source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_command_vocab_adapter_contract_guard'
  'bevy_classic_rts_openra_command_vocab_adapter_gate'
  'bevy_classic_rts_openra_command_vocab_adapter_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_command_vocab_adapter.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA command vocab adapter line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA command vocabulary adapter maps owned replay events into OpenRA-style orders without claiming binary/runtime parity"
