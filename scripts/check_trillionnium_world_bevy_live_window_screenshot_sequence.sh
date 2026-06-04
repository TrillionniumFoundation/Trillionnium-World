#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
MANUAL_DIR="$EVIDENCE_DIR/manual_bevy"
FRAME_DIR="$MANUAL_DIR/bevy-live-window-screenshot-sequence-frames"
SUMMARY="$EVIDENCE_DIR/bevy-live-window-screenshot-sequence.json"
CONTACT_SHEET="$MANUAL_DIR/bevy-live-window-screenshot-sequence-contact-sheet.png"
FINAL_FRAME="$MANUAL_DIR/bevy-live-window-screenshot-sequence-final.png"
LOG="$MANUAL_DIR/bevy-live-window-screenshot-sequence-host.log"
RUNTIME_PROBE="$MANUAL_DIR/bevy-live-window-runtime-probe.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-live-window-screenshot-sequence-slots"
RUNTIME_TEXTURE_SUMMARY="$EVIDENCE_DIR/bevy-runtime-texture-asset.json"
RUNTIME_TEXTURE_MANIFEST="$EVIDENCE_DIR/bevy-runtime-texture-asset-manifest.json"
mkdir -p "$EVIDENCE_DIR" "$MANUAL_DIR"
rm -rf "$FRAME_DIR" "$SLOT_DIR"
rm -f "$RUNTIME_PROBE"
mkdir -p "$FRAME_DIR" "$SLOT_DIR"

"$ROOT/scripts/check_trillionnium_world_bevy_runtime_texture_asset.sh" >/dev/null
test -s "$RUNTIME_TEXTURE_SUMMARY"
test -s "$RUNTIME_TEXTURE_MANIFEST"
RUNTIME_TEXTURE_MANIFEST_SHA256="$(sha256sum "$RUNTIME_TEXTURE_MANIFEST" | awk '{print $1}')"
export TRNM_WORLD_BEVY_RUNTIME_TEXTURE_ASSET_MANIFEST="$RUNTIME_TEXTURE_MANIFEST"
export TRNM_WORLD_BEVY_RUNTIME_TEXTURE_ASSET_SHA256="$RUNTIME_TEXTURE_MANIFEST_SHA256"

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
    TRNM_WORLD_BEVY_RUNTIME_PROBE_PATH="$RUNTIME_PROBE" \
    TRNM_WORLD_BEVY_RUNTIME_TEXTURE_ASSET_MANIFEST="$RUNTIME_TEXTURE_MANIFEST" \
    TRNM_WORLD_BEVY_RUNTIME_TEXTURE_ASSET_SHA256="$RUNTIME_TEXTURE_MANIFEST_SHA256" \
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

DISPLAY="$DISPLAY_VALUE" XAUTHORITY="$XAUTH" python3 - "$WINDOW_ID" "$FRAME_DIR" "$CONTACT_SHEET" "$FINAL_FRAME" "$SUMMARY" "$SLOT_DIR" "$HOST_PID" "$RUNTIME_TEXTURE_SUMMARY" "$RUNTIME_TEXTURE_MANIFEST" "$RUNTIME_TEXTURE_MANIFEST_SHA256" "$RUNTIME_PROBE" <<'PY'
import ctypes
import hashlib
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
final_frame = Path(sys.argv[4])
summary_path = Path(sys.argv[5])
slot_dir = Path(sys.argv[6])
host_pid = int(sys.argv[7])
runtime_texture_summary_path = Path(sys.argv[8])
runtime_texture_manifest_path = Path(sys.argv[9])
runtime_texture_manifest_sha256 = sys.argv[10]
runtime_probe_path = Path(sys.argv[11])
runtime_texture_summary = json.loads(runtime_texture_summary_path.read_text())
runtime_texture_manifest_bytes_data = runtime_texture_manifest_path.read_bytes()
runtime_texture_manifest = json.loads(runtime_texture_manifest_bytes_data.decode("utf-8"))
runtime_texture_manifest_computed_sha256 = hashlib.sha256(runtime_texture_manifest_bytes_data).hexdigest()

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
x11.XKeysymToKeycode.argtypes = [ctypes.c_void_p, ctypes.c_ulong]
x11.XKeysymToKeycode.restype = ctypes.c_uint
xtst.XTestFakeMotionEvent.argtypes = [ctypes.c_void_p, ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_ulong]
xtst.XTestFakeButtonEvent.argtypes = [ctypes.c_void_p, ctypes.c_uint, ctypes.c_int, ctypes.c_ulong]
xtst.XTestFakeKeyEvent.argtypes = [ctypes.c_void_p, ctypes.c_uint, ctypes.c_int, ctypes.c_ulong]

display = x11.XOpenDisplay(None)
if not display:
    raise SystemExit("could not open X11 display for live-window screenshot gate")

def window_origin():
    info = subprocess.check_output(["xwininfo", "-id", hex(window_id)], env=env, text=True, stderr=subprocess.STDOUT)
    w = int(re.search(r"Width:\s+(\d+)", info).group(1))
    h = int(re.search(r"Height:\s+(\d+)", info).group(1))
    geometry = re.search(r"-geometry\s+\d+x\d+([+-]\d+)([+-]\d+)", info)
    if geometry:
        x = int(geometry.group(1))
        y = int(geometry.group(2))
    else:
        x = int(re.search(r"Absolute upper-left X:\s+(-?\d+)", info).group(1))
        y = int(re.search(r"Absolute upper-left Y:\s+(-?\d+)", info).group(1))
    return x, y, w, h

def click_relative(rel_x, rel_y):
    x, y, _w, _h = window_origin()
    abs_x = x + int(rel_x)
    abs_y = y + int(rel_y)
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
    return {"relative": [int(rel_x), int(rel_y)], "absolute": [abs_x, abs_y]}

def focus_window():
    x11.XRaiseWindow(display, window_id)
    x11.XSetInputFocus(display, window_id, 1, 0)
    x11.XFlush(display)
    time.sleep(0.08)
    return {"method": "XRaiseWindow+XSetInputFocus", "window_id": hex(window_id)}

def press_return(action_label):
    focus_window()
    keycode = int(x11.XKeysymToKeycode(display, 0xFF0D))
    xtst.XTestFakeKeyEvent(display, keycode, 1, 0)
    x11.XFlush(display)
    time.sleep(0.07)
    xtst.XTestFakeKeyEvent(display, keycode, 0, 0)
    x11.XFlush(display)
    return {"key": "Return", "keycode": keycode, "action_label": action_label}

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

steps = [
    ("title_new", "TITLE:NEW", "create"),
    ("character_confirm", "CREATE:CONFIRM", "talk"),
    ("mentor_talk", "TALK", "train"),
    ("mentor_train", "TRAIN", "training_room"),
    ("move_north", "MOVE:north", "arena"),
    ("fight", "FIGHT", "fight_result"),
    ("save_selected", "SAVE:SELECTED", "save_continue"),
    ("title_open", "TITLE:OPEN", "title_continue"),
    ("title_continue", "TITLE:CONTINUE", "resume_continue"),
    ("continue_session", "CONTINUE:SESSION", "complete"),
]

time.sleep(3.0)
focus_event = focus_window()
time.sleep(0.5)
frames = []
key_events = []
frame, previous = capture("title", None, 0, None)
frames.append(frame)
for index, (step_id, action, frame_id) in enumerate(steps, start=1):
    frame = None
    candidate_image = None
    for attempt in range(1, 7):
        key_event = press_return(action)
        key_event.update({"step_id": step_id, "target_frame_id": frame_id, "attempt": attempt})
        key_events.append(key_event)
        time.sleep(0.55)
        frame, candidate_image = capture(frame_id, action, index, previous)
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
if frames:
    final_frame.parent.mkdir(parents=True, exist_ok=True)
    Image.open(frames[-1]["path"]).convert("RGB").save(final_frame)
sheet_colors = len(sheet.resize((120, max(1, int(sheet.height * 120 / sheet.width)))).getcolors(maxcolors=1000000) or [])
sheet_mean = [round(v, 2) for v in ImageStat.Stat(sheet).mean]
final_frame_bytes = final_frame.stat().st_size if final_frame.exists() else 0
runtime_probe = {}
for _ in range(80):
    if runtime_probe_path.exists() and runtime_probe_path.stat().st_size > 512:
        try:
            runtime_probe = json.loads(runtime_probe_path.read_text(encoding="utf-8"))
            break
        except json.JSONDecodeError:
            pass
    time.sleep(0.1)
runtime_probe_contract = runtime_probe.get("contract_version")
runtime_probe_sprite_binding = runtime_probe.get("runtime_texture_sprite_asset_binding", {})
runtime_probe_sprite_binding_contract = runtime_probe.get("runtime_texture_sprite_asset_binding_contract")
runtime_probe_sprite_binding_count = runtime_probe_sprite_binding.get("bound_sprite_surface_count", 0)
runtime_probe_sprite_scene_layers = runtime_probe_sprite_binding.get("scene_layers", [])
runtime_probe_sprite_material_slots = runtime_probe_sprite_binding.get("material_slots", [])
runtime_texture_sprite_asset_binding_gate = (
    runtime_probe_contract == "trillionnium_world_bevy_runtime_probe_v1"
    and runtime_probe_sprite_binding_contract == "trillionnium_world_bevy_sprite_asset_binding_v1"
    and runtime_probe.get("runtime_texture_sprite_asset_binding_gate") is True
    and runtime_probe_sprite_binding_count >= 24
    and all(layer in runtime_probe_sprite_scene_layers for layer in ["map", "hud", "actor", "feedback"])
    and all(slot in runtime_probe_sprite_material_slots for slot in ["world_tile_material", "hud_icon_material", "actor_sprite_material", "feedback_glyph_material"])
)
runtime_texture_manifest_bytes = runtime_texture_manifest_path.stat().st_size if runtime_texture_manifest_path.exists() else 0
runtime_texture_image_handle = runtime_texture_summary.get("image_asset_handle_id")
runtime_texture_layout_handle = runtime_texture_summary.get("texture_atlas_layout_handle_id")
runtime_texture_launch_env = {
    "manifest_path": os.environ.get("TRNM_WORLD_BEVY_RUNTIME_TEXTURE_ASSET_MANIFEST"),
    "manifest_sha256": os.environ.get("TRNM_WORLD_BEVY_RUNTIME_TEXTURE_ASSET_SHA256"),
    "image_asset_handle_id": runtime_texture_image_handle,
    "texture_atlas_layout_handle_id": runtime_texture_layout_handle,
}

slot_path = slot_dir / "bevy-session-slot-a.snapshot.json"
slot_bytes = slot_path.stat().st_size if slot_path.exists() else 0
expected_frame_ids = ["title", "create", "talk", "train", "training_room", "arena", "fight_result", "save_continue", "title_continue", "resume_continue", "complete"]
actual_frame_ids = [frame["frame_id"] for frame in frames]
changed_frames = [frame for frame in frames[1:] if frame["diff_mean_from_previous"] is not None and frame["diff_mean_from_previous"] >= 0.35]
host_window_gate = window_id > 0 and frames[0]["size"] == [960, 540]
key_count_gate = len(key_events) >= len(steps) and len(key_events) <= len(steps) * 6
frame_count_gate = len(frames) == len(expected_frame_ids)
frame_sequence_gate = actual_frame_ids == expected_frame_ids
screenshot_nonblank_gate = all(frame["nonblank"] for frame in frames)
frame_change_gate = len(changed_frames) == len(frames) - 1
slot_write_gate = slot_bytes > 512
contact_sheet_gate = contact_sheet.exists() and contact_sheet.stat().st_size > 1024 and sheet_colors > 32
final_frame_gate = final_frame_bytes > 1024
runtime_texture_asset_gate = runtime_texture_summary.get("green") is True
runtime_texture_manifest_file_gate = runtime_texture_manifest_path.exists() and runtime_texture_manifest_bytes > 8192
runtime_texture_manifest_hash_gate = runtime_texture_manifest_sha256 == runtime_texture_manifest_computed_sha256 and len(runtime_texture_manifest_sha256) == 64
runtime_texture_launch_env_gate = (
    runtime_texture_launch_env["manifest_path"] == str(runtime_texture_manifest_path)
    and runtime_texture_launch_env["manifest_sha256"] == runtime_texture_manifest_sha256
)
runtime_texture_handle_gate = (
    runtime_texture_image_handle == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1"
    and runtime_texture_layout_handle == "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1"
    and runtime_texture_manifest.get("image_asset_descriptor", {}).get("image_asset_handle_id") == runtime_texture_image_handle
    and runtime_texture_manifest.get("texture_atlas_layout_descriptor", {}).get("texture_atlas_layout_handle_id") == runtime_texture_layout_handle
)
green = all([host_window_gate, key_count_gate, frame_count_gate, frame_sequence_gate, screenshot_nonblank_gate, frame_change_gate, slot_write_gate, contact_sheet_gate, final_frame_gate, runtime_texture_asset_gate, runtime_texture_manifest_file_gate, runtime_texture_manifest_hash_gate, runtime_texture_launch_env_gate, runtime_texture_handle_gate, runtime_texture_sprite_asset_binding_gate])

evidence = {
    "contract_version": "trillionnium_world_bevy_live_window_screenshot_sequence_v1",
    "status": "live_window_screenshot_sequence_green",
    "source_of_truth": "XTest Return key events drive the visible Bevy X11 window through the current NEXT action and xwd captures each post-action frame. Before launch, the gate loads the runtime texture asset manifest and passes its path/hash into the Bevy host process launch environment so screenshot evidence remains tied to the host-side texture handle chain.",
    "host_pid": host_pid,
    "window_id": hex(window_id),
    "display": display_value,
    "slot_dir": str(slot_dir),
    "slot_a_path": str(slot_path),
    "slot_a_bytes": slot_bytes,
    "contact_sheet_path": str(contact_sheet),
    "final_frame_path": str(final_frame),
    "final_frame_bytes": final_frame_bytes,
    "runtime_probe_path": str(runtime_probe_path),
    "runtime_probe_contract": runtime_probe_contract,
    "runtime_texture_sprite_asset_binding_contract": runtime_probe_sprite_binding_contract,
    "runtime_texture_sprite_asset_binding_gate": runtime_texture_sprite_asset_binding_gate,
    "runtime_texture_sprite_bound_surface_count": runtime_probe_sprite_binding_count,
    "runtime_texture_sprite_scene_layers": runtime_probe_sprite_scene_layers,
    "runtime_texture_sprite_material_slots": runtime_probe_sprite_material_slots,
    "runtime_texture_asset_contract": "trillionnium_world_bevy_runtime_texture_asset_v1",
    "runtime_texture_summary_path": str(runtime_texture_summary_path),
    "runtime_texture_manifest_path": str(runtime_texture_manifest_path),
    "runtime_texture_manifest_sha256": runtime_texture_manifest_sha256,
    "runtime_texture_manifest_bytes": runtime_texture_manifest_bytes,
    "runtime_texture_image_asset_handle_id": runtime_texture_image_handle,
    "runtime_texture_atlas_layout_handle_id": runtime_texture_layout_handle,
    "runtime_texture_launch_env": runtime_texture_launch_env,
    "contact_sheet_size": list(sheet.size),
    "contact_sheet_colors": sheet_colors,
    "contact_sheet_mean": sheet_mean,
    "expected_frame_ids": expected_frame_ids,
    "actual_frame_ids": actual_frame_ids,
    "actions": [{"step_id": step[0], "action_label": step[1], "target_frame_id": step[2]} for step in steps],
    "focus_event": focus_event,
    "key_events": key_events,
    "frames": frames,
    "host_window_gate": host_window_gate,
    "key_count_gate": key_count_gate,
    "frame_count_gate": frame_count_gate,
    "frame_sequence_gate": frame_sequence_gate,
    "screenshot_nonblank_gate": screenshot_nonblank_gate,
    "frame_change_gate": frame_change_gate,
    "slot_write_gate": slot_write_gate,
    "contact_sheet_gate": contact_sheet_gate,
    "final_frame_gate": final_frame_gate,
    "runtime_texture_asset_gate": runtime_texture_asset_gate,
    "runtime_texture_manifest_file_gate": runtime_texture_manifest_file_gate,
    "runtime_texture_manifest_hash_gate": runtime_texture_manifest_hash_gate,
    "runtime_texture_launch_env_gate": runtime_texture_launch_env_gate,
    "runtime_texture_handle_gate": runtime_texture_handle_gate,
    "runtime_probe_sprite_binding_sample": runtime_probe_sprite_binding.get("sample", [])[:4],
    "green": green,
    "internal_live_window_screenshot_sequence_claimed": True,
    "external_evidence_ignored_for_current_live_window_pass": True,
    "gpu_upload_claimed": False,
    "android_s5_real_device_claimed": False,
    "public_launch_ready": False,
    "production_ready_ui_claimed": False,
    "screen_for_screen_openra_ui_claimed": False,
    "openra_engine_port_claimed": False,
    "warcraft_iii_asset_copied": False,
    "openra_asset_copied": False,
    "third_party_asset_copied": False,
    "live_osm_ingestion_claimed": False,
}
summary_path.write_text(json.dumps(evidence, indent=2, sort_keys=True), encoding="utf-8")
PY

jq -e '
  .contract_version == "trillionnium_world_bevy_live_window_screenshot_sequence_v1"
  and .status == "live_window_screenshot_sequence_green"
  and .green == true
  and .host_window_gate == true
  and .key_count_gate == true
  and .frame_count_gate == true
  and .frame_sequence_gate == true
  and .screenshot_nonblank_gate == true
  and .frame_change_gate == true
  and .slot_write_gate == true
  and .contact_sheet_gate == true
  and .final_frame_gate == true
  and .runtime_texture_asset_contract == "trillionnium_world_bevy_runtime_texture_asset_v1"
  and .runtime_texture_asset_gate == true
  and .runtime_texture_manifest_file_gate == true
  and .runtime_texture_manifest_hash_gate == true
  and .runtime_texture_launch_env_gate == true
  and .runtime_texture_handle_gate == true
  and .runtime_probe_contract == "trillionnium_world_bevy_runtime_probe_v1"
  and .runtime_texture_sprite_asset_binding_contract == "trillionnium_world_bevy_sprite_asset_binding_v1"
  and .runtime_texture_sprite_asset_binding_gate == true
  and .runtime_texture_sprite_bound_surface_count >= 24
  and (.runtime_texture_sprite_scene_layers | index("map"))
  and (.runtime_texture_sprite_scene_layers | index("hud"))
  and (.runtime_texture_sprite_scene_layers | index("actor"))
  and (.runtime_texture_sprite_scene_layers | index("feedback"))
  and (.runtime_texture_sprite_material_slots | index("world_tile_material"))
  and (.runtime_texture_sprite_material_slots | index("hud_icon_material"))
  and (.runtime_texture_sprite_material_slots | index("actor_sprite_material"))
  and (.runtime_texture_sprite_material_slots | index("feedback_glyph_material"))
  and .runtime_texture_manifest_bytes > 8192
  and (.runtime_texture_manifest_sha256 | length) == 64
  and .runtime_texture_image_asset_handle_id == "bevy_image_handle::trnm_world_authored_sprite_sheet_v1"
  and .runtime_texture_atlas_layout_handle_id == "bevy_texture_atlas_layout_handle::trnm_world_authored_sprite_sheet_layout_v1"
  and .slot_a_bytes > 512
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
  and .internal_live_window_screenshot_sequence_claimed == true
  and .external_evidence_ignored_for_current_live_window_pass == true
  and .public_launch_ready == false
  and .production_ready_ui_claimed == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
  and .live_osm_ingestion_claimed == false
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
  and (.key_events | length) >= 10
  and (.key_events | length) <= 60
  and all(.key_events[]; .key == "Return")
  and all(.frames[]; .nonblank == true)
  and all(.frames[1:][]; .diff_mean_from_previous >= 0.35)
' "$SUMMARY" >/dev/null

test -s "$CONTACT_SHEET"

printf 'TRILLIONNIUM_WORLD_BEVY_LIVE_WINDOW_SCREENSHOT_SEQUENCE_GREEN %s contact_sheet=%s final_frame=%s frame_dir=%s slot_dir=%s\n' "$SUMMARY" "$CONTACT_SHEET" "$FINAL_FRAME" "$FRAME_DIR" "$SLOT_DIR"
