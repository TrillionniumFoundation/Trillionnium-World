#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_desktop_playtest_review_packet.sh"

test -x "$SCRIPT"
grep -q 'trillionnium_world_bevy_desktop_playtest_review_packet_v1' "$SCRIPT"
grep -q 'desktop_playtest_review_packet_green' "$SCRIPT"
grep -q 'check_trillionnium_world_bevy_desktop_real_machine_readiness.sh' "$SCRIPT"
grep -q 'bevy-live-window-screenshot-sequence.json' "$SCRIPT"
grep -q 'bevy-live-window-mouse-hit-test-sequence.json' "$SCRIPT"
grep -q 'manual_review_checklist' "$SCRIPT"
grep -q 'keyboard_visual_review_gate' "$SCRIPT"
grep -q 'mouse_visual_review_gate' "$SCRIPT"
grep -q 'local_linux_desktop_x11_window_keyboard_mouse_visual_review_packet' "$SCRIPT"
grep -q 'android_s5_real_device_not_claimed_gate' "$SCRIPT"
grep -q 'public_launch_not_claimed_gate' "$SCRIPT"

printf 'TRILLIONNIUM_WORLD_BEVY_DESKTOP_PLAYTEST_REVIEW_PACKET_SCRIPT_CONTRACT_GUARD_OK %s\n' "$SCRIPT"
