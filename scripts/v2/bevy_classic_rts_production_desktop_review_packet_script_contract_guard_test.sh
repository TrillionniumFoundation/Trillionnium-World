#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_desktop_review_packet.sh"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

test -x "$SCRIPT"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_production_desktop_review_packet_v1'
  'classic_rts_production_desktop_review_packet_green'
  'bevy-classic-rts-production-desktop-review-packet.json'
  'bevy-classic-rts-production-desktop-review-packet.md'
  'check_trillionnium_world_bevy_classic_rts_production_interaction_polish.sh'
  'check_trillionnium_world_bevy_desktop_playtest_review_packet.sh'
  'trillionnium_world_bevy_classic_rts_production_interaction_polish_v1'
  'trillionnium_world_bevy_desktop_playtest_review_packet_v1'
  'trillionnium_world_bevy_desktop_real_machine_readiness_v1'
  'production_interaction_polish_gate'
  'desktop_playtest_review_packet_gate'
  'production_to_desktop_review_gate'
  'keyboard_visual_review_gate'
  'mouse_visual_review_gate'
  'DRAG SELECT'
  'RIGHT CLICK MOVE'
  'ATTACK LOCK'
  'BUILD GHOST'
  'QUEUE PATH'
  'SCROLL MINIMAP'
  'production_ready_desktop_review_shipped'
  'local_linux_desktop_x11_window_keyboard_mouse_with_production_interaction_polish'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_DESKTOP_REVIEW_PACKET_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing production desktop review packet script line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_production_desktop_review_packet.sh'
  'bevy_classic_rts_production_desktop_review_packet_script_contract_guard_test.sh'
  'bevy_classic_rts_production_desktop_review_packet_contract_guard'
  'bevy_classic_rts_production_desktop_review_packet_gate'
  'trillionnium_world_bevy_classic_rts_production_desktop_review_packet_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing production desktop review packet release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS production desktop review packet remains connected to production interaction polish, desktop keyboard/mouse evidence, boundaries, and release-review CI"
