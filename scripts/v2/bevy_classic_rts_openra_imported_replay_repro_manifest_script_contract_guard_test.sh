#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest_v1'
  'bevy-classic-rts-openra-imported-replay-repro-manifest.json'
  'openra-imported-replay-repro-manifest.json'
  'openra-imported-replay-repro-diff.json'
  'openra-imported-replay-repro-manifest-negative-corpus.json'
  'classic-rts-openra-imported-replay-repro-manifest'
  'openra_imported_replay_repro_manifest_v1_json'
  'openra_imported_replay_repro_manifest_gate == true'
  'bevy_openra_imported_replay_repro_manifest_claimed == true'
  'bevy_openra_imported_replay_audit_ledger_claimed == true'
  'bevy_openra_imported_headless_comparison_harness_claimed == true'
  'bevy_openra_order_payload_decoder_claimed == true'
  'bevy_openra_native_order_payload_decoder_claimed == false'
  'bevy_openra_network_order_stream_claimed == false'
  'bevy_openra_runtime_parity_claimed == false'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA imported replay repro manifest script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_REPRO_MANIFEST_CONTRACT'
  'native_classic_rts_openra_imported_replay_repro_manifest_evidence_json'
  'classic-rts-openra-imported-replay-repro-manifest'
  'bevy_owned_openra_imported_replay_repro_manifest_not_openra_runtime_parity'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_IMPORTED_REPLAY_AUDIT_LEDGER_CONTRACT'
  'openra_imported_replay_repro_manifest_v1_json'
  'openra_imported_replay_repro_diff_v1_json'
  'stable_summary_gate'
  'artifact_read_gate'
  'openra_imported_replay_repro_manifest_gate'
  'bevy_openra_imported_replay_repro_manifest_claimed'
  'bevy_openra_imported_replay_audit_ledger_claimed'
  'bevy_openra_order_payload_decoder_claimed'
  'bevy_openra_runtime_parity_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA imported replay repro manifest source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_imported_replay_repro_manifest_contract_guard'
  'bevy_classic_rts_openra_imported_replay_repro_manifest_gate'
  'bevy_classic_rts_openra_imported_replay_repro_manifest_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA imported replay repro manifest line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA imported replay repro manifest proves stable ledger regeneration without claiming OpenRA runtime/network parity"
