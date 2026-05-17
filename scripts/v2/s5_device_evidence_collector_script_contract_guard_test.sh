#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_s5_device_evidence.sh"

required_lines=(
  'trillionnium_world_s5_native_bevy_device_evidence_v1'
  's5-device-evidence.json'
  'TRILLIONNIUM_WORLD_S5_EVIDENCE_DIR'
  '--require-device'
  '--require-apk'
  '--help|-h'
  'adb devices -l'
  'install -r'
  'monkey -p'
  'screencap -p'
  'dumpsys gfxinfo'
  'logcat -d -v time'
  'input keyevent HOME'
  'ANativeActivity_onCreate'
  'android_main'
  'signed_debug_apk_ready'
  'blocked_no_connected_android_device'
  'real_device_evidence_collected'
  'crash_free_logcat_window'
  'TRILLIONNIUM_WORLD_S5_DEVICE_EVIDENCE_READY'
  'TRILLIONNIUM_WORLD_S5_DEVICE_EVIDENCE_BLOCKED'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] S5 device evidence collector script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] S5 device evidence collector script keeps adb real-device capture, APK/native checks, and require-device semantics"
