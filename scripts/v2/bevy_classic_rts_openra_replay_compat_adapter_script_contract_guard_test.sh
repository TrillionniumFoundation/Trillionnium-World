#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CI_GATE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter_v1'
  'bevy-classic-rts-openra-replay-compat-adapter.json'
  'openra-replay-summary-adapter.json'
  'classic-rts-openra-replay-compat-adapter'
  '.source_paths.owned_replay_file'
  'openra_replay_summary_adapter_v1_json'
  'openra_binary_replay_compatible == false'
  'openra_replay_file_claimed == false'
  'openra_headless_client_match_claimed == false'
  'openra_runtime_parity_claimed == false'
  'openra_replay_compat_adapter_gate == true'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS OpenRA replay compat adapter script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_REPLAY_COMPAT_ADAPTER_CONTRACT'
  'native_classic_rts_openra_replay_compat_adapter_evidence_json'
  'TRNM_OPENRA_PARITY_LANE_SUMMARY'
  'TRNM_OPENRA_PARITY_LANE_DIR'
  'classic-rts-openra-replay-compat-adapter'
  'bevy_owned_replay_to_openra_style_summary_adapter_not_binary_openra_replay'
  'openra_replay_summary_adapter_v1_json'
  'summary_schema_gate'
  'replay_timeline_gate'
  'headless_adapter_gate'
  'terminal_adapter_gate'
  'compatibility_boundary_gate'
  'openra_replay_compat_adapter_gate'
  'bevy_openra_binary_replay_compatible'
  'bevy_openra_runtime_parity_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS OpenRA replay compat adapter source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'bevy_classic_rts_openra_replay_compat_adapter_contract_guard'
  'bevy_classic_rts_openra_replay_compat_adapter_gate'
  'TRNM_OPENRA_PARITY_LANE_SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-parity-lane.json"'
  'TRNM_OPENRA_PARITY_LANE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-parity-lane"'
  'bevy_classic_rts_openra_replay_compat_adapter_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$CI_GATE"; then
    echo "[FAIL] missing release-review CI OpenRA replay compat adapter line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS OpenRA replay compatibility adapter maps owned replay/headless evidence into an OpenRA-style summary without claiming binary/runtime parity"
