#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_local_obstruction_recovery.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_local_obstruction_recovery_v1'
  'bevy-classic-rts-local-obstruction-recovery.json'
  'bevy-classic-rts-local-obstruction-recovery.ppm'
  'classic-rts-local-obstruction-recovery'
  'detect_block_gate == true'
  'hold_queue_gate == true'
  'side_step_gate == true'
  'gap_claim_gate == true'
  'flow_resume_gate == true'
  'warcraft_iii_asset_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LOCAL_OBSTRUCTION_RECOVERY_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS local obstruction recovery script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_LOCAL_OBSTRUCTION_RECOVERY_CONTRACT'
  'native_classic_rts_local_obstruction_recovery_evidence_json'
  'classic_draw_rts_local_obstruction_recovery_overlay'
  'classic_rts_local_obstruction_recovery_stage'
  'CLASSIC_RTS_OBSTRUCTION_BLOCK_COLOR'
  'CLASSIC_RTS_OBSTRUCTION_QUEUE_COLOR'
  'CLASSIC_RTS_OBSTRUCTION_SIDE_STEP_COLOR'
  'CLASSIC_RTS_OBSTRUCTION_GAP_COLOR'
  'CLASSIC_RTS_OBSTRUCTION_RESUME_COLOR'
  'Original Trillionnium local obstruction recovery overlays'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS local obstruction recovery source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_local_obstruction_recovery.sh'
  'bevy-classic-rts-local-obstruction-recovery.json'
  'classic_rts_local_obstruction_recovery_green'
  'rts_local_obstruction_recovery_detect_block_gate'
  'rts_local_obstruction_recovery_hold_queue_gate'
  'rts_local_obstruction_recovery_side_step_gate'
  'rts_local_obstruction_recovery_gap_claim_gate'
  'rts_local_obstruction_recovery_flow_resume_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS local obstruction recovery readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_local_obstruction_recovery_v1'
  'bevy_classic_rts_local_obstruction_recovery_contract_guard'
  'bevy_classic_rts_local_obstruction_recovery_gate'
  'bevy_classic_rts_local_obstruction_recovery_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_local_obstruction_recovery.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS local obstruction recovery release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS local obstruction recovery evidence remains connected to renderer, CLI, readiness, release-review, accepted input, block detection, queued followers, side steps, gap claims, flow resume, and original art policy"
