#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger_v1'
  'bevy-classic-rts-openra-imported-replay-audit-ledger.json'
  'openra-imported-replay-audit-ledger.jsonl'
  'openra-imported-replay-audit-ledger.json'
  'openra-imported-replay-audit-ledger-negative-corpus.json'
  'classic-rts-openra-imported-replay-audit-ledger'
  'openra_imported_replay_audit_ledger_v1_json'
  'openra_imported_replay_audit_ledger_gate == true'
  'bevy_openra_imported_replay_audit_ledger_claimed == true'
  'bevy_openra_imported_headless_comparison_harness_claimed == true'
  'bevy_openra_imported_replay_reducer_claimed == true'
  'bevy_openra_order_payload_decoder_claimed == true'
  'bevy_openra_native_order_payload_decoder_claimed == false'
  'bevy_openra_replay_summary_adapter_claimed == true'
  'bevy_openra_network_order_stream_claimed == false'
  'bevy_openra_runtime_parity_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA imported replay audit ledger script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_AUDIT_LEDGER_CONTRACT'
  'native_classic_rts_openra_imported_replay_audit_ledger_evidence_json'
  'classic-rts-openra-imported-replay-audit-ledger'
  'TRNM_OPENRA_IMPORTED_HEADLESS_COMPARISON_SUMMARY'
  'TRNM_OPENRA_IMPORTED_HEADLESS_COMPARISON_DIR'
  'bevy_owned_openra_imported_replay_hash_ledger_not_openra_runtime_parity'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_HEADLESS_COMPARISON_HARNESS_CONTRACT'
  'openra_imported_replay_audit_ledger_v1_json'
  'openra_imported_replay_audit_ledger_entry_v1_json'
  'ledger_validation_gate'
  'headless_alignment_gate'
  'openra_imported_replay_audit_ledger_gate'
  'bevy_openra_imported_replay_audit_ledger_claimed'
  'bevy_openra_order_payload_decoder_claimed'
  'bevy_openra_runtime_parity_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA imported replay audit ledger source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_imported_replay_audit_ledger_contract_guard'
  'bevy_classic_rts_openra_imported_replay_audit_ledger_gate'
  'bevy_classic_rts_openra_imported_replay_audit_ledger_script_contract_guard_test.sh'
  'TRNM_OPENRA_IMPORTED_HEADLESS_COMPARISON_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-headless-comparison-harness.json"'
  'TRNM_OPENRA_IMPORTED_HEADLESS_COMPARISON_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-headless-comparison-harness"'
  'check_trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA imported replay audit ledger line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA imported replay audit ledger hashes decoded records, snapshots, and headless summary without claiming OpenRA runtime/network parity"
