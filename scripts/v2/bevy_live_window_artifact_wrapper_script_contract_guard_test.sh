#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

live_window_scripts=(
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_mouse_hit_test_sequence.sh"
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_negative_input_guard.sh"
)

for script in "${live_window_scripts[@]}"; do
  if [[ ! -x "$script" ]]; then
    echo "[FAIL] live-window script missing or not executable: $script" >&2
    exit 1
  fi
  if ! grep -Fq 'run_trillionnium_world_bevy_artifact_command.sh' "$script"; then
    echo "[FAIL] live-window script does not use artifact wrapper: $script" >&2
    exit 1
  fi
  if grep -Fq 'cargo run -p trnm-world-bevy' "$script"; then
    echo "[FAIL] live-window script still invokes cargo directly: $script" >&2
    exit 1
  fi
  if ! grep -Fq 'exec "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" run' "$script"; then
    echo "[FAIL] live-window script does not exec the host runner: $script" >&2
    exit 1
  fi
  if ! grep -Fq 'wait "$HOST_PID"' "$script"; then
    echo "[FAIL] live-window script cleanup does not wait for host runner: $script" >&2
    exit 1
  fi
  if ! grep -Fq '_NET_WM_PID' "$script"; then
    echo "[FAIL] live-window script does not bind window lookup to host pid: $script" >&2
    exit 1
  fi
done

if ! grep -Fq 'visible-button-hit-test-map' "$ROOT/scripts/check_trillionnium_world_bevy_live_window_mouse_hit_test_sequence.sh"; then
  echo "[FAIL] mouse hit-test script lost visible-button-hit-test-map fixture" >&2
  exit 1
fi
if ! grep -Fq 'TRNM_WORLD_BEVY_RUNTIME_PROBE_PATH="$PROBE"' "$ROOT/scripts/check_trillionnium_world_bevy_live_window_mouse_hit_test_sequence.sh"; then
  echo "[FAIL] mouse hit-test script lost runtime probe binding" >&2
  exit 1
fi
if ! grep -Fq 'runtime_feedback_gate' "$ROOT/scripts/check_trillionnium_world_bevy_live_window_mouse_hit_test_sequence.sh"; then
  echo "[FAIL] mouse hit-test script no longer verifies runtime feedback" >&2
  exit 1
fi
if ! grep -Fq 'core_route_actions_reflowed' "$ROOT/scripts/check_trillionnium_world_bevy_live_window_mouse_hit_test_sequence.sh"; then
  echo "[FAIL] mouse hit-test script lost state-specific row reflow coordinates" >&2
  exit 1
fi

if ! grep -Fq 'visible-button-hit-test-map' "$ROOT/scripts/check_trillionnium_world_bevy_live_window_negative_input_guard.sh"; then
  echo "[FAIL] negative input script lost visible-button-hit-test-map fixture" >&2
  exit 1
fi
if ! grep -Fq 'blocked_title_guard_gate' "$ROOT/scripts/check_trillionnium_world_bevy_live_window_negative_input_guard.sh"; then
  echo "[FAIL] negative input script lost blocked title/resume guard" >&2
  exit 1
fi
if ! grep -Fq 'state_specific_visible_button' "$ROOT/scripts/check_trillionnium_world_bevy_live_window_negative_input_guard.sh"; then
  echo "[FAIL] negative input script lost state-specific visible button targets" >&2
  exit 1
fi

printf 'TRILLIONNIUM_BEVY_LIVE_WINDOW_ARTIFACT_WRAPPER_SCRIPT_CONTRACT_GREEN script_count=%s\n' "${#live_window_scripts[@]}"
