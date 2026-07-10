#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
MANUAL_DIR="$EVIDENCE_DIR/manual_bevy"
mkdir -p "$EVIDENCE_DIR" "$MANUAL_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-first-minute-screenshot-sequence.json"
MANIFEST="$EVIDENCE_DIR/bevy-first-minute-screenshot-manifest.json"
RECORDING="$EVIDENCE_DIR/bevy-first-minute-screenshot-recording.json"
CONTACT_SHEET="$MANUAL_DIR/bevy-first-minute-screenshot-sequence-contact-sheet.png"
SLOT_DIR="$EVIDENCE_DIR/bevy-first-minute-screenshot-sequence-slots"

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- first-minute-screenshot-sequence "$SLOT_DIR" "$MANIFEST" "$RECORDING" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_first_minute_screenshot_sequence_v1"
  and .screenshot_manifest_contract == "trillionnium_world_bevy_first_minute_screenshot_manifest_v1"
  and .input_replay_contract == "trillionnium_world_bevy_first_minute_input_replay_v1"
  and .recording_contract == "trillionnium_world_bevy_first_minute_input_recording_v1"
  and .green == true
  and .manifest_path == "'"$MANIFEST"'"
  and .recording_path == "'"$RECORDING"'"
  and .manifest_bytes > 1024
  and .replay_source_gate == true
  and .manifest_write_gate == true
  and .frame_count_gate == true
  and .frame_sequence_gate == true
  and .frame_next_button_gate == true
  and .frame_visibility_gate == true
  and .frame_highlight_gate == true
  and .final_complete_frame_gate == true
  and .actual_frame_ids == [
    "title",
    "create",
    "talk",
    "train",
    "training_room",
    "arena",
    "fight_result",
    "save_continue",
    "resume_continue",
    "complete"
  ]
  and (.frame_manifest.frames | length) == 10
  and all(.frame_manifest.frames[]; .visible_state_gate == true)
  and all(.frame_manifest.frames[]; .highlight_gate == true)
  and (.frame_manifest.frames[] | select(.frame_id == "title") | .next_button) == "TITLE:NEW"
  and (.frame_manifest.frames[] | select(.frame_id == "create") | .next_button) == "CREATE:CONFIRM"
  and (.frame_manifest.frames[] | select(.frame_id == "talk") | .next_button) == "TALK"
  and (.frame_manifest.frames[] | select(.frame_id == "training_room") | .next_button) == "MOVE:north"
  and (.frame_manifest.frames[] | select(.frame_id == "arena") | .next_button) == "FIGHT"
  and (.frame_manifest.frames[] | select(.frame_id == "fight_result") | .next_button) == "SAVE:SELECTED"
  and (.frame_manifest.frames[] | select(.frame_id == "save_continue") | .next_button) == "TITLE:OPEN"
  and (.frame_manifest.frames[] | select(.frame_id == "resume_continue") | .next_button) == "CONTINUE:SESSION"
  and (.frame_manifest.frames[] | select(.frame_id == "complete") | .next_button) == "FIRST MINUTE COMPLETE"
  and (.frame_manifest.frames[] | select(.frame_id == "complete") | .highlighted_actions | length) == 0
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_first_minute_screenshot_manifest_v1"
  and .capture_kind == "bevy_replay_sample_contact_sheet_manifest"
  and (.frames | length) == 10
  and all(.frames[]; .visible_state_gate == true)
  and all(.frames[]; .highlight_gate == true)
  and .android_s5_real_device_claimed == false
' "$MANIFEST" >/dev/null

python3 - "$MANIFEST" "$CONTACT_SHEET" <<'PY'
import json
import sys
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont, ImageStat

manifest = json.loads(Path(sys.argv[1]).read_text())
out = Path(sys.argv[2])
frames = manifest["frames"]
cols = 2
card_w, card_h = 430, 176
pad = 18
rows = (len(frames) + cols - 1) // cols
img = Image.new("RGB", (cols * card_w + (cols + 1) * pad, rows * card_h + (rows + 1) * pad), (18, 22, 20))
draw = ImageDraw.Draw(img)
font = ImageFont.load_default()
palette = {
    "title": (78, 54, 33),
    "create": (78, 40, 48),
    "talk": (38, 69, 59),
    "train": (46, 74, 54),
    "training_room": (58, 61, 88),
    "arena": (84, 65, 35),
    "fight_result": (86, 46, 38),
    "save_continue": (50, 70, 84),
    "resume_continue": (82, 62, 36),
    "complete": (42, 84, 58),
}

def text_value(v):
    if v is None:
        return ""
    if isinstance(v, str):
        return v
    return json.dumps(v, ensure_ascii=True)

def short(s, limit=74):
    s = text_value(s).replace("\n", " | ")
    return s if len(s) <= limit else s[: limit - 3] + "..."

def draw_line(x, y, label, value, fill=(232, 226, 205)):
    draw.text((x, y), f"{label}: {short(value)}", font=font, fill=fill)

for i, frame in enumerate(frames):
    row, col = divmod(i, cols)
    x = pad + col * (card_w + pad)
    y = pad + row * (card_h + pad)
    fill = palette.get(frame["frame_id"], (50, 50, 50))
    draw.rounded_rectangle((x, y, x + card_w, y + card_h), radius=6, fill=fill, outline=(246, 214, 118), width=2)
    draw.text((x + 12, y + 10), f"{i + 1:02d} {frame['frame_id']}", font=font, fill=(255, 244, 190))
    draw.text((x + card_w - 90, y + 10), "GREEN", font=font, fill=(168, 255, 170))
    draw_line(x + 12, y + 34, "next", frame.get("next_button"), (255, 239, 178))
    draw_line(x + 12, y + 52, "highlight", ",".join(frame.get("highlighted_actions") or ["none"]))
    draw_line(x + 12, y + 70, "node", frame.get("current_node_id"))
    draw_line(x + 12, y + 88, "scene", frame.get("active_scene_layer"))
    draw_line(x + 12, y + 106, "objective", frame.get("objective_status"))
    draw_line(x + 12, y + 124, "quest", frame.get("quest_panel_text"))
    draw_line(x + 12, y + 142, "resume", frame.get("session_resume_text"))

out.parent.mkdir(parents=True, exist_ok=True)
img.save(out)
small = img.resize((120, max(1, int(img.height * 120 / img.width))))
colors = len(small.getcolors(maxcolors=1000000) or [])
means = [round(v, 2) for v in ImageStat.Stat(img).mean]
if colors <= 16 or max(means) <= 8:
    raise SystemExit(f"contact sheet looked blank: colors={colors} means={means}")
print({"path": str(out), "size": img.size, "colors": colors, "mean": means, "nonblank": True})
PY

test -s "$CONTACT_SHEET"

printf 'TRILLIONNIUM_WORLD_BEVY_FIRST_MINUTE_SCREENSHOT_SEQUENCE_GREEN %s manifest=%s contact_sheet=%s slot_dir=%s\n' "$SUMMARY" "$MANIFEST" "$CONTACT_SHEET" "$SLOT_DIR"
