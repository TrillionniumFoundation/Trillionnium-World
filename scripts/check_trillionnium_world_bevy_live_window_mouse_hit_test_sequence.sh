#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
MANUAL_DIR="$EVIDENCE_DIR/manual_bevy"
FRAME_DIR="$MANUAL_DIR/bevy-live-window-mouse-hit-test-sequence-frames"
SUMMARY="$EVIDENCE_DIR/bevy-live-window-mouse-hit-test-sequence.json"
HIT_MAP="$EVIDENCE_DIR/bevy-visible-button-hit-test-map.json"
CONTACT_SHEET="$MANUAL_DIR/bevy-live-window-mouse-hit-test-sequence-contact-sheet.png"
LOG="$MANUAL_DIR/bevy-live-window-mouse-hit-test-sequence-host.log"
SLOT_DIR="$EVIDENCE_DIR/bevy-live-window-mouse-hit-test-sequence-slots"
mkdir -p "$EVIDENCE_DIR" "$MANUAL_DIR"
rm -rf "$FRAME_DIR" "$SLOT_DIR"
mkdir -p "$FRAME_DIR" "$SLOT_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- visible-button-hit-test-map >"$HIT_MAP"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_visible_button_hit_test_map_v1"
  and .green == true
  and .target_count_gate == true
  and .target_sequence_gate == true
  and .native_action_parse_gate == true
  and .first_minute_touch_coverage_gate == true
  and .visible_row_gate == true
  and (.targets | length) == 10
  and [.targets[].action_label] == [
    "TITLE:NEW",
    "CREATE:CONFIRM",
    "TALK",
    "TRAIN",
    "MOVE:north",
    "FIGHT",
    "SAVE:SELECTED",
    "TITLE:OPEN",
    "TITLE:CONTINUE",
    "CONTINUE:SESSION"
  ]
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
    cargo run -p trnm-world-bevy -- run >"$LOG" 2>&1
) &
HOST_PID=$!

cleanup() {
  if kill -0 "$HOST_PID" >/dev/null 2>&1; then
    kill "$HOST_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

WINDOW_ID=""
for _ in $(seq 1 160); do
  WINDOW_ID="$(DISPLAY="$DISPLAY_VALUE" XAUTHORITY="$XAUTH" xwininfo -root -tree 2>/dev/null | awk '/"Trillionnium World": \("trnm-world-bevy"/ {print $1; exit}')"
  if [[ -n "$WINDOW_ID" ]]; then
    break
  fi
  sleep 0.25
done
test -n "$WINDOW_ID"

DISPLAY="$DISPLAY_VALUE" XAUTHORITY="$XAUTH" python3 - "$WINDOW_ID" "$FRAME_DIR" "$CONTACT_SHEET" "$SUMMARY" "$SLOT_DIR" "$HOST_PID" "$HIT_MAP" <<'PY'
import ctypes
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw, ImageFont, ImageStat

window_id = int(sys.argv[1], 16)
frame_dir = Path(sys.argv[2])
contact_sheet = Path(sys.argv[3])
summary_path = Path(sys.argv[4])
slot_dir = Path(sys.argv[5])
host_pid = int(sys.argv[6])
hit_map_path = Path(sys.argv[7])
hit_map = json.loads(hit_map_path.read_text(encoding="utf-8"))

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
    raise SystemExit("could not open X11 display for live-window mouse hit-test gate")

def window_origin():
    info = subprocess.check_output(["xwininfo", "-id", hex(window_id)], env=env, text=True, stderr=subprocess.STDOUT)
    w = int(re.search(r"Width:\s+(\d+)", info).group(1))
    h = int(re.search(r"Height:\s+(\d+)", info).group(1))
    x = int(re.search(r"Absolute upper-left X:\s+(-?\d+)", info).group(1))
    y = int(re.search(r"Absolute upper-left Y:\s+(-?\d+)", info).group(1))
    return x, y, w, h

def focus_window():
    x11.XRaiseWindow(display, window_id)
    x11.XSetInputFocus(display, window_id, 1, 0)
    x11.XFlush(display)
    time.sleep(0.08)
    return {"method": "XRaiseWindow+XSetInputFocus", "window_id": hex(window_id)}

def click_target(target, attempt):
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
        "action_label": target["action_label"],
        "step_id": target["step_id"],
        "target_frame_id": target["target_frame_id"],
        "attempt": attempt,
        "relative": [rel_x, rel_y],
        "absolute": [abs_x, abs_y],
        "window_origin": [origin_x, origin_y],
        "window_size": [width, height],
        "source": target["source"],
        "row_id": target["row_id"],
    }

def capture(frame_id, after_action=None, index=0, previous_image=None):
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
        small = image.resize((96, 54))
        colors = len(small.getcolors(maxcolors=1000000) or [])
        if colors > 32 and max(stat.mean) > 8:
            break
        time.sleep(0.25)
    diff_mean = None
    diff_bbox = None
    if previous_image is not None:
        diff = ImageChops.difference(image, previous_image)
        diff_stat = ImageStat.Stat(diff)
        diff_mean = round(sum(diff_stat.mean) / 3.0, 4)
        diff_bbox = list(diff.getbbox()) if diff.getbbox() else None
    nonblank = colors > 32 and max(stat.mean) > 8
    return {
        "frame_index": index,
        "frame_id": frame_id,
        "after_action": after_action,
        "path": str(png_path),
        "size": list(image.size),
        "mean": [round(v, 2) for v in stat.mean],
        "colors_96x54": colors,
        "nonblank": nonblank,
        "diff_mean_from_previous": diff_mean,
        "diff_bbox_from_previous": diff_bbox,
    }, image

targets = hit_map.get("targets", [])
expected_frame_ids = ["title"] + [target["target_frame_id"] for target in targets]
expected_action_labels = [target["action_label"] for target in targets]

time.sleep(3.0)
focus_event = focus_window()
time.sleep(0.5)
frames = []
mouse_events = []
frame, previous = capture("title", None, 0, None)
frames.append(frame)
for index, target in enumerate(targets, start=1):
    frame = None
    candidate_image = None
    for attempt in range(1, 7):
        event = click_target(target, attempt)
        mouse_events.append(event)
        time.sleep(0.55)
        frame, candidate_image = capture(target["target_frame_id"], target["action_label"], index, previous)
        if frame["diff_mean_from_previous"] is not None and frame["diff_mean_from_previous"] >= 0.35:
            break
    previous = candidate_image
    frames.append(frame)

x11.XCloseDisplay(display)

thumb_w, thumb_h = 256, 144
label_h = 42
cols = 2
pad = 16
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
    draw.text((x + 8, y + 6), f"{idx:02d} {frame['frame_id']}", font=font, fill=(255, 244, 190))
    draw.text((x + 8, y + 22), f"after {frame.get('after_action') or 'boot'}", font=font, fill=(190, 224, 204))

contact_sheet.parent.mkdir(parents=True, exist_ok=True)
sheet.save(contact_sheet)
sheet_colors = len(sheet.resize((120, max(1, int(sheet.height * 120 / sheet.width)))).getcolors(maxcolors=1000000) or [])
sheet_mean = [round(v, 2) for v in ImageStat.Stat(sheet).mean]

slot_path = slot_dir / "bevy-session-slot-a.snapshot.json"
slot_bytes = slot_path.stat().st_size if slot_path.exists() else 0
actual_frame_ids = [frame["frame_id"] for frame in frames]
changed_frames = [frame for frame in frames[1:] if frame["diff_mean_from_previous"] is not None and frame["diff_mean_from_previous"] >= 0.35]
hit_test_map_gate = (
    hit_map.get("contract_version") == "trillionnium_world_bevy_visible_button_hit_test_map_v1"
    and hit_map.get("green") is True
    and hit_map.get("live_mouse_sequence_contract") == "trillionnium_world_bevy_live_window_mouse_hit_test_sequence_v1"
)
host_window_gate = window_id > 0 and frames[0]["size"] == [960, 540]
mouse_event_count_gate = len(mouse_events) >= len(targets) and len(mouse_events) <= len(targets) * 6
frame_count_gate = len(frames) == len(expected_frame_ids)
frame_sequence_gate = actual_frame_ids == expected_frame_ids
screenshot_nonblank_gate = all(frame["nonblank"] for frame in frames)
frame_change_gate = len(changed_frames) == len(frames) - 1
slot_write_gate = slot_bytes > 512
contact_sheet_gate = contact_sheet.exists() and contact_sheet.stat().st_size > 1024 and sheet_colors > 32
green = all([
    hit_test_map_gate,
    host_window_gate,
    mouse_event_count_gate,
    frame_count_gate,
    frame_sequence_gate,
    screenshot_nonblank_gate,
    frame_change_gate,
    slot_write_gate,
    contact_sheet_gate,
])

evidence = {
    "contract_version": "trillionnium_world_bevy_live_window_mouse_hit_test_sequence_v1",
    "hit_test_map_contract": "trillionnium_world_bevy_visible_button_hit_test_map_v1",
    "source_of_truth": "XTest mouse button events click Bevy-exposed client hit centers on the visible X11 window and xwd captures each post-action frame",
    "host_pid": host_pid,
    "window_id": hex(window_id),
    "display": display_value,
    "slot_dir": str(slot_dir),
    "slot_a_path": str(slot_path),
    "slot_a_bytes": slot_bytes,
    "hit_map_path": str(hit_map_path),
    "contact_sheet_path": str(contact_sheet),
    "contact_sheet_size": list(sheet.size),
    "contact_sheet_colors": sheet_colors,
    "contact_sheet_mean": sheet_mean,
    "expected_frame_ids": expected_frame_ids,
    "actual_frame_ids": actual_frame_ids,
    "expected_action_labels": expected_action_labels,
    "actions": [
        {
            "step_index": target["step_index"],
            "step_id": target["step_id"],
            "action_label": target["action_label"],
            "target_frame_id": target["target_frame_id"],
            "client_x": target["client_x"],
            "client_y": target["client_y"],
            "row_id": target["row_id"],
            "source": target["source"],
        }
        for target in targets
    ],
    "focus_event": focus_event,
    "mouse_events": mouse_events,
    "frames": frames,
    "hit_test_map_gate": hit_test_map_gate,
    "host_window_gate": host_window_gate,
    "mouse_event_count_gate": mouse_event_count_gate,
    "frame_count_gate": frame_count_gate,
    "frame_sequence_gate": frame_sequence_gate,
    "screenshot_nonblank_gate": screenshot_nonblank_gate,
    "frame_change_gate": frame_change_gate,
    "slot_write_gate": slot_write_gate,
    "contact_sheet_gate": contact_sheet_gate,
    "green": green,
    "android_s5_real_device_claimed": False,
}
summary_path.write_text(json.dumps(evidence, indent=2, sort_keys=True), encoding="utf-8")
PY

jq -e '
  .contract_version == "trillionnium_world_bevy_live_window_mouse_hit_test_sequence_v1"
  and .hit_test_map_contract == "trillionnium_world_bevy_visible_button_hit_test_map_v1"
  and .green == true
  and .hit_test_map_gate == true
  and .host_window_gate == true
  and .mouse_event_count_gate == true
  and .frame_count_gate == true
  and .frame_sequence_gate == true
  and .screenshot_nonblank_gate == true
  and .frame_change_gate == true
  and .slot_write_gate == true
  and .contact_sheet_gate == true
  and .slot_a_bytes > 512
  and .android_s5_real_device_claimed == false
  and .actual_frame_ids == [
    "title",
    "create",
    "talk",
    "train",
    "training_room",
    "arena",
    "fight_result",
    "save_continue",
    "title_continue",
    "resume_continue",
    "complete"
  ]
  and [.actions[].action_label] == [
    "TITLE:NEW",
    "CREATE:CONFIRM",
    "TALK",
    "TRAIN",
    "MOVE:north",
    "FIGHT",
    "SAVE:SELECTED",
    "TITLE:OPEN",
    "TITLE:CONTINUE",
    "CONTINUE:SESSION"
  ]
  and (.mouse_events | length) >= 10
  and (.mouse_events | length) <= 60
  and all(.mouse_events[]; .relative | length == 2)
  and all(.frames[]; .nonblank == true)
  and all(.frames[1:][]; .diff_mean_from_previous >= 0.35)
' "$SUMMARY" >/dev/null

test -s "$CONTACT_SHEET"

printf 'TRILLIONNIUM_WORLD_BEVY_LIVE_WINDOW_MOUSE_HIT_TEST_SEQUENCE_GREEN %s hit_map=%s contact_sheet=%s frame_dir=%s slot_dir=%s\n' "$SUMMARY" "$HIT_MAP" "$CONTACT_SHEET" "$FRAME_DIR" "$SLOT_DIR"
