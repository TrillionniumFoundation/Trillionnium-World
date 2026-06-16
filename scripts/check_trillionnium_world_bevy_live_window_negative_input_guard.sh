#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
MANUAL_DIR="$EVIDENCE_DIR/manual_bevy"
FRAME_DIR="$MANUAL_DIR/bevy-live-window-negative-input-guard-frames"
SUMMARY="$EVIDENCE_DIR/bevy-live-window-negative-input-guard.json"
HIT_MAP="$EVIDENCE_DIR/bevy-visible-button-hit-test-map.json"
PROBE="$EVIDENCE_DIR/bevy-live-window-negative-input-runtime-probe.json"
CONTACT_SHEET="$MANUAL_DIR/bevy-live-window-negative-input-guard-contact-sheet.png"
LOG="$MANUAL_DIR/bevy-live-window-negative-input-guard-host.log"
SLOT_DIR="$EVIDENCE_DIR/bevy-live-window-negative-input-guard-slots"
mkdir -p "$EVIDENCE_DIR" "$MANUAL_DIR"
rm -rf "$FRAME_DIR" "$SLOT_DIR"
rm -f "$PROBE"
mkdir -p "$FRAME_DIR" "$SLOT_DIR"

(
  cd "$ROOT/trillionnium"
  "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" visible-button-hit-test-map >"$HIT_MAP"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_visible_button_hit_test_map_v1"
  and .green == true
  and (.targets | length) == 10
' "$HIT_MAP" >/dev/null

DEFAULT_XAUTH="$(find "${XDG_RUNTIME_DIR:-/run/user/$(id -u)}" -maxdepth 1 -type f -name '.mutter-Xwaylandauth.*' -printf '%T@ %p\n' 2>/dev/null | sort -nr | awk 'NR == 1 {print $2}')"
XAUTH="${XAUTHORITY:-${DEFAULT_XAUTH:-/run/user/1000/.mutter-Xwaylandauth.BE4HP3}}"
DISPLAY_VALUE="${DISPLAY:-:0}"

pids="$(pgrep -f '(^|/)target/debug/trnm-world-bevy run$' || true)"
if [[ -n "$pids" ]]; then
  kill $pids || true
  sleep 1
fi

(
  cd "$ROOT/trillionnium"
  DISPLAY="$DISPLAY_VALUE" \
    XAUTHORITY="$XAUTH" \
    WAYLAND_DISPLAY="" \
    WINIT_UNIX_BACKEND=x11 \
    TRNM_WORLD_BEVY_FORCE_X11=1 \
    TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    TRNM_WORLD_BEVY_RUNTIME_PROBE_PATH="$PROBE" \
    exec "$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" run >"$LOG" 2>&1
) &
HOST_PID=$!

cleanup() {
  if kill -0 "$HOST_PID" >/dev/null 2>&1; then
    kill "$HOST_PID" >/dev/null 2>&1 || true
    wait "$HOST_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

WINDOW_ID=""
for _ in $(seq 1 160); do
  WINDOW_ID="$(
    DISPLAY="$DISPLAY_VALUE" XAUTHORITY="$XAUTH" xwininfo -root -tree 2>/dev/null |
      awk '/"Trillionnium World": \("trnm-world-bevy"/ {print $1}' |
      while read -r candidate; do
        pid="$(DISPLAY="$DISPLAY_VALUE" XAUTHORITY="$XAUTH" xprop -id "$candidate" _NET_WM_PID 2>/dev/null | awk -F= '{gsub(/[[:space:]]/, "", $2); print $2}')"
        ppid="$(ps -o ppid= -p "$pid" 2>/dev/null | awk '{print $1}')"
        if [[ "$pid" == "$HOST_PID" || "$ppid" == "$HOST_PID" ]]; then
          printf '%s\n' "$candidate"
          break
        fi
      done
  )"
  if [[ -n "$WINDOW_ID" ]]; then
    break
  fi
  sleep 0.25
done
test -n "$WINDOW_ID"

DISPLAY="$DISPLAY_VALUE" XAUTHORITY="$XAUTH" python3 - "$WINDOW_ID" "$FRAME_DIR" "$CONTACT_SHEET" "$SUMMARY" "$SLOT_DIR" "$HOST_PID" "$HIT_MAP" "$PROBE" <<'PY'
import ctypes
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont, ImageStat

window_id = int(sys.argv[1], 16)
frame_dir = Path(sys.argv[2])
contact_sheet = Path(sys.argv[3])
summary_path = Path(sys.argv[4])
slot_dir = Path(sys.argv[5])
host_pid = int(sys.argv[6])
hit_map_path = Path(sys.argv[7])
probe_path = Path(sys.argv[8])
hit_map = json.loads(hit_map_path.read_text(encoding="utf-8"))
targets = {target["action_label"]: target for target in hit_map.get("targets", [])}

display_value = os.environ.get("DISPLAY", ":0")
xauthority = os.environ.get("XAUTHORITY")
env = os.environ.copy()
env["DISPLAY"] = display_value
if xauthority:
    env["XAUTHORITY"] = xauthority

x11 = ctypes.cdll.LoadLibrary("libX11.so.6")
xtst = ctypes.cdll.LoadLibrary("libXtst.so.6")
x11.XOpenDisplay.argtypes = [ctypes.c_char_p]
x11.XOpenDisplay.restype = ctypes.c_void_p
x11.XFlush.argtypes = [ctypes.c_void_p]
x11.XCloseDisplay.argtypes = [ctypes.c_void_p]
x11.XRaiseWindow.argtypes = [ctypes.c_void_p, ctypes.c_ulong]
x11.XSetInputFocus.argtypes = [ctypes.c_void_p, ctypes.c_ulong, ctypes.c_int, ctypes.c_ulong]
xtst.XTestFakeMotionEvent.argtypes = [ctypes.c_void_p, ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_ulong]
xtst.XTestFakeButtonEvent.argtypes = [ctypes.c_void_p, ctypes.c_uint, ctypes.c_int, ctypes.c_ulong]

display = x11.XOpenDisplay(None)
if not display:
    raise SystemExit("could not open X11 display for live-window negative input gate")

def read_probe():
    return json.loads(probe_path.read_text(encoding="utf-8"))

def wait_probe_contract():
    deadline = time.time() + 12
    while time.time() < deadline:
        try:
            probe = read_probe()
        except Exception:
            time.sleep(0.1)
            continue
        if probe.get("contract_version") == "trillionnium_world_bevy_runtime_probe_v1":
            return probe
        time.sleep(0.1)
    raise TimeoutError("runtime probe did not become available")

def wait_last_feedback(action_label, accepted, reason_prefix):
    deadline = time.time() + 6
    last_probe = None
    while time.time() < deadline:
        try:
            probe = read_probe()
        except Exception:
            time.sleep(0.08)
            continue
        last_probe = probe
        feedback = probe.get("last_input_feedback") or {}
        reason = feedback.get("reason") or ""
        if feedback.get("action_label") == action_label and feedback.get("accepted") is accepted and reason.startswith(reason_prefix):
            return probe
        time.sleep(0.08)
    raise TimeoutError(f"expected feedback {action_label} accepted={accepted} reason={reason_prefix}; last={last_probe}")

def core_signature(probe):
    runtime = probe.get("runtime") or {}
    return {
        "current_node_id": probe.get("current_node_id"),
        "next": probe.get("first_minute_next_button"),
        "current_room_id": runtime.get("current_room_id"),
        "tutorial_step": runtime.get("tutorial_step"),
        "objective_status": runtime.get("objective_status"),
        "active_scene_layer": runtime.get("active_scene_layer"),
        "session_title_menu_visible": runtime.get("session_title_menu_visible"),
        "session_character_create_visible": runtime.get("session_character_create_visible"),
        "session_resume_input_locked": runtime.get("session_resume_input_locked"),
        "completed_steps": runtime.get("completed_steps") or [],
        "xp": runtime.get("xp"),
        "coins": runtime.get("coins"),
        "player_hp": runtime.get("player_hp"),
        "enemy_hp": runtime.get("enemy_hp"),
    }

def assert_probe_state(probe, expected):
    runtime = probe.get("runtime") or {}
    checks = []
    if "next" in expected:
        checks.append(("next", probe.get("first_minute_next_button") == expected["next"], probe.get("first_minute_next_button"), expected["next"]))
    if "node" in expected:
        checks.append(("node", probe.get("current_node_id") == expected["node"], probe.get("current_node_id"), expected["node"]))
    if "title_visible" in expected:
        checks.append(("title_visible", runtime.get("session_title_menu_visible") is expected["title_visible"], runtime.get("session_title_menu_visible"), expected["title_visible"]))
    if "create_visible" in expected:
        checks.append(("create_visible", runtime.get("session_character_create_visible") is expected["create_visible"], runtime.get("session_character_create_visible"), expected["create_visible"]))
    if "objective_status" in expected:
        checks.append(("objective_status", runtime.get("objective_status") == expected["objective_status"], runtime.get("objective_status"), expected["objective_status"]))
    failed = [check for check in checks if not check[1]]
    return {
        "ok": not failed,
        "checks": [{"name": name, "ok": ok, "actual": actual, "expected": expected_value} for name, ok, actual, expected_value in checks],
    }

def window_origin():
    info = subprocess.check_output(["xwininfo", "-id", hex(window_id)], env=env, text=True, stderr=subprocess.STDOUT)
    w = int(re.search(r"Width:\s+(\d+)", info).group(1))
    h = int(re.search(r"Height:\s+(\d+)", info).group(1))
    x = int(re.search(r"Absolute upper-left X:\s+(-?\d+)", info).group(1))
    y = int(re.search(r"Absolute upper-left Y:\s+(-?\d+)", info).group(1))
    return x, y, w, h

def target_for_step(step):
    action_label = step["action"]
    target = dict(targets.get(action_label, {}))
    if not target and "client_x" not in step:
        raise KeyError(f"missing hit-test target for {action_label}")
    if "client_x" in step:
        target.update({
            "action_label": action_label,
            "client_x": step["client_x"],
            "client_y": step["client_y"],
            "source": step.get("source", "state_specific_visible_button"),
            "row_id": step.get("row_id", "state_specific"),
        })
    return target

def click_action(step, action_label, attempt=1):
    target = target_for_step(step)
    rel_x = int(target["client_x"])
    rel_y = int(target["client_y"])
    origin_x, origin_y, width, height = window_origin()
    abs_x = origin_x + rel_x
    abs_y = origin_y + rel_y
    x11.XRaiseWindow(display, window_id)
    x11.XSetInputFocus(display, window_id, 1, 0)
    x11.XFlush(display)
    time.sleep(0.05)
    xtst.XTestFakeMotionEvent(display, -1, abs_x, abs_y, 0)
    x11.XFlush(display)
    time.sleep(0.05)
    xtst.XTestFakeButtonEvent(display, 1, 1, 0)
    x11.XFlush(display)
    time.sleep(0.07)
    xtst.XTestFakeButtonEvent(display, 1, 0, 0)
    x11.XFlush(display)
    return {
        "action_label": action_label,
        "attempt": attempt,
        "relative": [rel_x, rel_y],
        "absolute": [abs_x, abs_y],
        "window_origin": [origin_x, origin_y],
        "window_size": [width, height],
        "source": target["source"],
        "row_id": target["row_id"],
    }

def capture(frame_id, index, after_action=None):
    xwd_path = frame_dir / f"{index:02d}-{frame_id}.xwd"
    png_path = frame_dir / f"{index:02d}-{frame_id}.png"
    image = None
    stat = None
    colors = 0
    for _ in range(20):
        subprocess.check_call(["xwd", "-silent", "-id", hex(window_id), "-out", str(xwd_path)], env=env)
        subprocess.check_call(["ffmpeg", "-y", "-hide_banner", "-loglevel", "error", "-i", str(xwd_path), str(png_path)], env=env)
        image = Image.open(png_path).convert("RGB")
        stat = ImageStat.Stat(image)
        colors = len(image.resize((96, 54)).getcolors(maxcolors=1000000) or [])
        if colors > 32 and max(stat.mean) > 8:
            break
        time.sleep(0.2)
    return {
        "frame_index": index,
        "frame_id": frame_id,
        "after_action": after_action,
        "path": str(png_path),
        "size": list(image.size),
        "mean": [round(v, 2) for v in stat.mean],
        "colors_96x54": colors,
        "nonblank": colors > 32 and max(stat.mean) > 8,
    }

steps = [
    {"id": "blocked_title_continue_missing_slot", "action": "TITLE:CONTINUE", "accepted": False, "reason": "title_continue_slot_missing", "expect": {"next": "TITLE:NEW", "node": "mirror-city-square", "title_visible": True, "create_visible": False}},
    {"id": "blocked_title_load_missing_slot", "action": "TITLE:LOAD", "client_x": 558, "client_y": 473, "row_id": "title", "accepted": False, "reason": "title_load_slot_missing", "expect": {"next": "TITLE:NEW", "node": "mirror-city-square", "title_visible": True, "create_visible": False}},
    {"id": "accepted_title_new", "action": "TITLE:NEW", "accepted": True, "reason": "enabled_title_new_game", "expect": {"next": "CREATE:CONFIRM", "node": "mirror-city-square", "title_visible": False, "create_visible": True}},
    {"id": "accepted_create_name_cycle", "action": "CREATE:NAME", "client_x": 313, "client_y": 473, "row_id": "character_create", "accepted": True, "reason": "enabled_character_name_cycle", "expect": {"next": "CREATE:CONFIRM", "node": "mirror-city-square", "title_visible": False, "create_visible": True}},
    {"id": "accepted_create_confirm", "action": "CREATE:CONFIRM", "accepted": True, "reason": "enabled_character_create_confirm", "expect": {"next": "TALK", "node": "mirror-city-square", "title_visible": False, "create_visible": False}},
    {"id": "accepted_spawn_local_move_before_train", "action": "MOVE:north", "accepted": True, "reason": "enabled_local_map_step_north", "expect": {"next": "TALK", "node": "mirror-city-square"}},
    {"id": "accepted_talk", "action": "TALK", "accepted": True, "reason": "enabled_at_mentor_tile", "expect": {"next": "TRAIN", "node": "mirror-city-square"}},
    {"id": "accepted_train", "action": "TRAIN", "client_x": 332, "client_y": 473, "row_id": "core_route_actions_reflowed", "accepted": True, "reason": "enabled_after_dialogue_choice", "expect": {"next": "MOVE:north", "node": "mirror-city-square"}},
    {"id": "accepted_move_to_arena", "action": "MOVE:north", "accepted": True, "reason": "enabled_route_step_north", "expect": {"next": "FIGHT", "node": "league-coliseum"}},
    {"id": "accepted_fight", "action": "FIGHT", "client_x": 332, "client_y": 473, "row_id": "core_route_actions_reflowed", "accepted": True, "reason": "enabled_enemy_adjacent", "expect": {"next": "SAVE:SELECTED", "node": "league-coliseum", "objective_status": "combat_resolved"}},
]

time.sleep(2.0)
initial_probe = wait_probe_contract()
frames = [capture("initial_title", 0, None)]
click_events = []
step_results = []

for index, step in enumerate(steps, start=1):
    click_action_label = step["action"]
    feedback_action_label = step.get("feedback_action", click_action_label)
    before_probe = read_probe()
    before_core = core_signature(before_probe)
    click_event = click_action(step, click_action_label)
    click_event["expected_feedback_action_label"] = feedback_action_label
    click_events.append(click_event)
    after_probe = wait_last_feedback(feedback_action_label, step["accepted"], step["reason"])
    # Wait one rendered frame before the next click. The runtime probe can update before
    # Bevy has redrawn/relaid out the contextual control row on the live X11 window.
    time.sleep(0.6)
    state_check = assert_probe_state(after_probe, step["expect"])
    after_core = core_signature(after_probe)
    toast_text = after_probe.get("input_feedback_toast") or ""
    disabled_core_unchanged = None
    if not step["accepted"]:
        disabled_core_unchanged = before_core == after_core
    frame = capture(step["id"], index, step["action"])
    frames.append(frame)
    feedback = after_probe.get("last_input_feedback") or {}
    step_results.append({
        "step_index": index,
        "step_id": step["id"],
        "click_action_label": click_action_label,
        "feedback_action_label": feedback_action_label,
        "action_label": feedback_action_label,
        "expected_accepted": step["accepted"],
        "actual_accepted": feedback.get("accepted"),
        "reason": feedback.get("reason"),
        "expected_reason_prefix": step["reason"],
        "expected_state": step["expect"],
        "state_check": state_check,
        "input_feedback_toast": toast_text,
        "disabled_core_unchanged": disabled_core_unchanged,
        "before_core": before_core,
        "after_core": after_core,
        "probe": after_probe,
        "frame": frame,
    })

x11.XCloseDisplay(display)

thumb_w, thumb_h = 256, 144
label_h = 42
cols = 3
pad = 14
rows = (len(frames) + cols - 1) // cols
sheet = Image.new("RGB", (cols * thumb_w + (cols + 1) * pad, rows * (thumb_h + label_h) + (rows + 1) * pad), (18, 22, 20))
draw = ImageDraw.Draw(sheet)
font = ImageFont.load_default()
for frame in frames:
    idx = frame["frame_index"]
    row, col = divmod(idx, cols)
    x = pad + col * (thumb_w + pad)
    y = pad + row * (thumb_h + label_h + pad)
    image = Image.open(frame["path"]).convert("RGB").resize((thumb_w, thumb_h))
    sheet.paste(image, (x, y + label_h))
    draw.rectangle((x, y, x + thumb_w, y + label_h - 2), fill=(36, 44, 38), outline=(246, 214, 118))
    draw.text((x + 8, y + 6), f"{idx:02d} {frame['frame_id'][:30]}", font=font, fill=(255, 244, 190))
    draw.text((x + 8, y + 22), f"after {frame.get('after_action') or 'boot'}", font=font, fill=(190, 224, 204))

contact_sheet.parent.mkdir(parents=True, exist_ok=True)
sheet.save(contact_sheet)
sheet_colors = len(sheet.resize((120, max(1, int(sheet.height * 120 / sheet.width)))).getcolors(maxcolors=1000000) or [])
sheet_mean = [round(v, 2) for v in ImageStat.Stat(sheet).mean]

slot_path = slot_dir / "bevy-session-slot-a.snapshot.json"
slot_bytes = slot_path.stat().st_size if slot_path.exists() else 0
disabled_results = [step for step in step_results if step["expected_accepted"] is False]
accepted_results = [step for step in step_results if step["expected_accepted"] is True]
runtime_probe_gate = initial_probe.get("contract_version") == "trillionnium_world_bevy_runtime_probe_v1"
hit_test_map_gate = hit_map.get("green") is True and hit_map.get("contract_version") == "trillionnium_world_bevy_visible_button_hit_test_map_v1"
host_window_gate = window_id > 0 and frames[0]["size"] == [960, 540]
mouse_event_count_gate = len(click_events) == len(steps)
disabled_rejection_gate = len(disabled_results) == 2 and all(step["actual_accepted"] is False for step in disabled_results)
disabled_state_gate = all(step["state_check"]["ok"] and step["disabled_core_unchanged"] is True for step in disabled_results)
accepted_progression_gate = len(accepted_results) == 8 and all(step["actual_accepted"] is True and step["state_check"]["ok"] for step in accepted_results)
blocked_title_guard_gate = all(step_id in {step["step_id"] for step in disabled_results} for step_id in ["blocked_title_continue_missing_slot", "blocked_title_load_missing_slot"])
toast_feedback_gate = all(step["input_feedback_toast"].startswith(f"TOAST BLOCKED | {step['feedback_action_label']}") and f"NEXT {step['after_core']['next']}" in step["input_feedback_toast"] for step in disabled_results) and all(step["input_feedback_toast"].startswith(f"TOAST OK | {step['feedback_action_label']}") and f"NEXT {step['after_core']['next']}" in step["input_feedback_toast"] for step in accepted_results)
slot_write_blocked_gate = slot_bytes == 0
screenshot_nonblank_gate = all(frame["nonblank"] for frame in frames)
contact_sheet_gate = contact_sheet.exists() and contact_sheet.stat().st_size > 1024 and sheet_colors > 32
green = all([
    runtime_probe_gate,
    hit_test_map_gate,
    host_window_gate,
    mouse_event_count_gate,
    disabled_rejection_gate,
    disabled_state_gate,
    accepted_progression_gate,
    blocked_title_guard_gate,
    toast_feedback_gate,
    slot_write_blocked_gate,
    screenshot_nonblank_gate,
    contact_sheet_gate,
])

evidence = {
    "contract_version": "trillionnium_world_bevy_live_window_negative_input_guard_v1",
    "runtime_probe_contract": "trillionnium_world_bevy_runtime_probe_v1",
    "hit_test_map_contract": "trillionnium_world_bevy_visible_button_hit_test_map_v1",
    "source_of_truth": "XTest mouse events click visible Bevy controls while TRNM_WORLD_BEVY_RUNTIME_PROBE_PATH samples the running runtime after each accepted or rejected input",
    "host_pid": host_pid,
    "window_id": hex(window_id),
    "display": display_value,
    "slot_dir": str(slot_dir),
    "slot_a_path": str(slot_path),
    "slot_a_bytes": slot_bytes,
    "probe_path": str(probe_path),
    "hit_map_path": str(hit_map_path),
    "contact_sheet_path": str(contact_sheet),
    "contact_sheet_size": list(sheet.size),
    "contact_sheet_colors": sheet_colors,
    "contact_sheet_mean": sheet_mean,
    "click_events": click_events,
    "step_results": step_results,
    "frames": frames,
    "runtime_probe_gate": runtime_probe_gate,
    "hit_test_map_gate": hit_test_map_gate,
    "host_window_gate": host_window_gate,
    "mouse_event_count_gate": mouse_event_count_gate,
    "disabled_rejection_gate": disabled_rejection_gate,
    "disabled_state_gate": disabled_state_gate,
    "accepted_progression_gate": accepted_progression_gate,
    "blocked_title_guard_gate": blocked_title_guard_gate,
    "toast_feedback_gate": toast_feedback_gate,
    "slot_write_blocked_gate": slot_write_blocked_gate,
    "screenshot_nonblank_gate": screenshot_nonblank_gate,
    "contact_sheet_gate": contact_sheet_gate,
    "green": green,
    "android_s5_real_device_claimed": False,
}
summary_path.write_text(json.dumps(evidence, indent=2, sort_keys=True), encoding="utf-8")
PY

jq -e '
  .contract_version == "trillionnium_world_bevy_live_window_negative_input_guard_v1"
  and .runtime_probe_contract == "trillionnium_world_bevy_runtime_probe_v1"
  and .green == true
  and .runtime_probe_gate == true
  and .hit_test_map_gate == true
  and .host_window_gate == true
  and .mouse_event_count_gate == true
  and .disabled_rejection_gate == true
  and .disabled_state_gate == true
  and .accepted_progression_gate == true
  and .blocked_title_guard_gate == true
  and .toast_feedback_gate == true
  and .slot_write_blocked_gate == true
  and .screenshot_nonblank_gate == true
  and .contact_sheet_gate == true
  and .slot_a_bytes == 0
  and .android_s5_real_device_claimed == false
  and (.click_events | length) == 10
  and ([.step_results[] | select(.expected_accepted == false)] | length) == 2
  and ([.step_results[] | select(.expected_accepted == true)] | length) == 8
  and all(.step_results[] | select(.expected_accepted == false); .actual_accepted == false and .disabled_core_unchanged == true)
  and all(.step_results[] | select(.expected_accepted == false); .input_feedback_toast | startswith("TOAST BLOCKED | "))
  and all(.step_results[] | select(.expected_accepted == true); .input_feedback_toast | startswith("TOAST OK | "))
  and all(.step_results[]; .state_check.ok == true)
  and all(.frames[]; .nonblank == true)
' "$SUMMARY" >/dev/null

test -s "$CONTACT_SHEET"

printf 'TRILLIONNIUM_WORLD_BEVY_LIVE_WINDOW_NEGATIVE_INPUT_GUARD_GREEN %s probe=%s contact_sheet=%s frame_dir=%s slot_dir=%s\n' "$SUMMARY" "$PROBE" "$CONTACT_SHEET" "$FRAME_DIR" "$SLOT_DIR"
