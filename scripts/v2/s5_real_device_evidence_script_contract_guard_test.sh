#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_s5_real_device_evidence.sh"

required_lines=(
  'trillionnium_world_s5_real_device_evidence_gate_v1'
  's5-real-device-evidence-validation.json'
  's5-device-evidence.template.json'
  'check_trillionnium_world_s5_device_evidence.sh --require-device'
  'TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH'
  'TRILLIONNIUM_WORLD_S5_REAL_DEVICE_VALIDATION_SUMMARY'
  'trillionnium_world_s5_native_bevy_device_evidence_v1'
  's5_real_device_evidence_green'
  'blocked_missing_s5_real_device_evidence'
  'android_native_cdylib_ready'
  'signed_debug_apk_ready'
  'real_device_evidence_collected'
  'real_device_screenshot'
  'real_device_gfxinfo_or_frame_stats'
  'real_device_logcat'
  'real_device_lifecycle'
  'crash_free_logcat_window'
  'host_side_replay_credit: false'
  'template_requires_real_s5_device_evidence'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] S5 real-device evidence script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] S5 real-device evidence script requires real device screenshot/gfxinfo/logcat/lifecycle/crash-free evidence and rejects host-side replay credit"
