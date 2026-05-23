#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh"

test -x "$SCRIPT"
grep -q 'trillionnium_world_bevy_desktop_real_machine_readiness_v1' "$SCRIPT"
grep -q 'desktop_real_machine_readiness_green' "$SCRIPT"
grep -q 'check_trillionnium_world_bevy_live_window_screenshot_sequence.sh' "$SCRIPT"
grep -q 'check_trillionnium_world_bevy_live_window_mouse_hit_test_sequence.sh' "$SCRIPT"
grep -q 'check_trillionnium_world_bevy_classic_playtest_runner_status.sh' "$SCRIPT"
grep -q 'keyboard_event_count' "$SCRIPT"
grep -q 'mouse_event_count' "$SCRIPT"
grep -q 'live_window_mouse_hit_test_sequence_gate' "$SCRIPT"
grep -q 'android_s5_real_device_not_required_gate' "$SCRIPT"
grep -q 'local_linux_desktop_x11_window_and_release_runner' "$SCRIPT"

printf 'TRILLIONNIUM_WORLD_BEVY_DESKTOP_REAL_MACHINE_READINESS_SCRIPT_CONTRACT_GUARD_OK %s\n' "$SCRIPT"
