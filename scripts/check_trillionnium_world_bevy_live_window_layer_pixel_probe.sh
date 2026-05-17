#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-live-window-layer-pixel-probe.json"
BRIDGE_SUMMARY="$EVIDENCE_DIR/bevy-authored-live-visual-bridge.json"
LIVE_SUMMARY="$EVIDENCE_DIR/bevy-live-window-screenshot-sequence.json"
RENDER_SUMMARY="$EVIDENCE_DIR/bevy-authored-render-frame.json"
mkdir -p "$EVIDENCE_DIR"

"$ROOT/scripts/check_trillionnium_world_bevy_authored_live_visual_bridge.sh" >/dev/null

test -s "$BRIDGE_SUMMARY"
test -s "$LIVE_SUMMARY"
test -s "$RENDER_SUMMARY"

FINAL_FRAME="$(jq -r '.final_frame_path' "$LIVE_SUMMARY")"
CONTACT_SHEET="$(jq -r '.contact_sheet_path' "$LIVE_SUMMARY")"
test -s "$FINAL_FRAME"
test -s "$CONTACT_SHEET"

python3 - "$FINAL_FRAME" "$CONTACT_SHEET" "$LIVE_SUMMARY" "$RENDER_SUMMARY" "$BRIDGE_SUMMARY" "$SUMMARY" <<'PY'
import json
import sys
from pathlib import Path

from PIL import Image, ImageStat

final_frame = Path(sys.argv[1])
contact_sheet = Path(sys.argv[2])
live_summary_path = Path(sys.argv[3])
render_summary_path = Path(sys.argv[4])
bridge_summary_path = Path(sys.argv[5])
summary_path = Path(sys.argv[6])

live = json.loads(live_summary_path.read_text())
render = json.loads(render_summary_path.read_text())
bridge = json.loads(bridge_summary_path.read_text())
image = Image.open(final_frame).convert("RGB")
width, height = image.size

probe_specs = [
    {
        "probe_id": "map_playfield_pixels",
        "scene_layer": "map",
        "region": [40, 40, 720, 420],
        "min_sampled_colors": 3000,
        "min_stddev": 25.0,
    },
    {
        "probe_id": "hud_pixels",
        "scene_layer": "hud",
        "region": [0, 0, width, 90],
        "min_sampled_colors": 1000,
        "min_stddev": 30.0,
    },
    {
        "probe_id": "actor_activity_pixels",
        "scene_layer": "actor",
        "region": [120, 80, 720, 420],
        "min_sampled_colors": 3000,
        "min_stddev": 25.0,
    },
    {
        "probe_id": "feedback_action_pixels",
        "scene_layer": "feedback",
        "region": [0, 420, width, height],
        "min_sampled_colors": 1000,
        "min_stddev": 20.0,
    },
]

render_layer_pixels = {
    sample.get("scene_layer"): int(sample.get("drawn_pixel_count", 0))
    for sample in render.get("layer_samples", [])
}

probes = []
for spec in probe_specs:
    x0, y0, x1, y1 = spec["region"]
    x0, y0 = max(0, x0), max(0, y0)
    x1, y1 = min(width, x1), min(height, y1)
    crop = image.crop((x0, y0, x1, y1))
    sampled = crop.resize((max(1, crop.size[0] // 4), max(1, crop.size[1] // 4)))
    sampled_colors = len(sampled.getcolors(maxcolors=1_000_000) or [])
    stat = ImageStat.Stat(crop)
    mean = [round(value, 2) for value in stat.mean]
    stddev = [round(value, 2) for value in stat.stddev]
    avg_stddev = round(sum(stat.stddev) / 3.0, 2)
    nonblank = sum(stat.mean) / 3.0 > 8.0 and avg_stddev >= spec["min_stddev"]
    layer_pixels = render_layer_pixels.get(spec["scene_layer"], 0)
    passes = (
        sampled_colors >= spec["min_sampled_colors"]
        and nonblank
        and layer_pixels > 0
    )
    probes.append(
        {
            "probe_id": spec["probe_id"],
            "scene_layer": spec["scene_layer"],
            "region": [x0, y0, x1, y1],
            "sampled_colors": sampled_colors,
            "min_sampled_colors": spec["min_sampled_colors"],
            "mean": mean,
            "stddev": stddev,
            "avg_stddev": avg_stddev,
            "render_layer_drawn_pixel_count": layer_pixels,
            "nonblank": nonblank,
            "passes": passes,
        }
    )

live_frames = live.get("frames", [])
live_final = live_frames[-1] if live_frames else {}
required_layers = {"map", "hud", "actor", "feedback"}
passed_layers = {probe["scene_layer"] for probe in probes if probe["passes"]}
gates = {
    "bridge_gate": bridge.get("green") is True,
    "final_frame_file_gate": final_frame.exists() and final_frame.stat().st_size > 1024,
    "contact_sheet_file_gate": contact_sheet.exists() and contact_sheet.stat().st_size > 1024,
    "live_window_sequence_gate": (
        live.get("green") is True
        and live.get("screenshot_nonblank_gate") is True
        and live.get("frame_change_gate") is True
        and live.get("contact_sheet_gate") is True
        and live.get("final_frame_gate") is True
    ),
    "final_frame_color_gate": int(live_final.get("colors_96x54", 0)) >= 1000,
    "region_probe_gate": all(probe["passes"] for probe in probes),
    "four_layer_pixel_probe_gate": required_layers.issubset(passed_layers),
    "boundary_gate": (
        bridge.get("gpu_upload_claimed") is False
        and bridge.get("android_s5_real_device_claimed") is False
        and bridge.get("live_osm_ingestion_claimed") is False
    ),
}
summary = {
    "contract_version": "trillionnium_world_bevy_live_window_layer_pixel_probe_v1",
    "green": all(gates.values()),
    "authored_live_visual_bridge_contract": "trillionnium_world_bevy_authored_live_visual_bridge_v1",
    "live_window_screenshot_contract": "trillionnium_world_bevy_live_window_screenshot_sequence_v1",
    "final_frame_path": str(final_frame),
    "contact_sheet_path": str(contact_sheet),
    "live_summary_path": str(live_summary_path),
    "render_summary_path": str(render_summary_path),
    "bridge_summary_path": str(bridge_summary_path),
    "image_size": [width, height],
    "live_frame_count": len(live_frames),
    "live_final_frame_colors_96x54": int(live_final.get("colors_96x54", 0)),
    "live_contact_sheet_colors": int(live.get("contact_sheet_colors", 0)),
    "render_frame_bytes": int(render.get("frame_bytes", 0)),
    "render_frame_unique_color_count": int(render.get("frame_unique_color_count", 0)),
    "probes": probes,
    "passed_layers": sorted(passed_layers),
    "gates": gates,
    "asset_boundary": "live_window_png_region_probe_correlated_with_authored_visual_bridge_not_gpu_texture_claim",
    "source_of_truth": "This gate directly probes the final live Bevy window PNG regions for map, HUD, actor, and feedback pixel complexity, and correlates those regions with the authored live visual bridge evidence.",
    "gpu_upload_claimed": False,
    "android_s5_real_device_claimed": False,
    "live_osm_ingestion_claimed": False,
}
summary_path.write_text(json.dumps(summary, indent=2))
PY

jq -e '
  .contract_version == "trillionnium_world_bevy_live_window_layer_pixel_probe_v1"
  and .green == true
  and .authored_live_visual_bridge_contract == "trillionnium_world_bevy_authored_live_visual_bridge_v1"
  and .live_window_screenshot_contract == "trillionnium_world_bevy_live_window_screenshot_sequence_v1"
  and .live_frame_count >= 11
  and .live_final_frame_colors_96x54 >= 1000
  and .live_contact_sheet_colors > 32
  and .gates.bridge_gate == true
  and .gates.final_frame_file_gate == true
  and .gates.contact_sheet_file_gate == true
  and .gates.live_window_sequence_gate == true
  and .gates.final_frame_color_gate == true
  and .gates.region_probe_gate == true
  and .gates.four_layer_pixel_probe_gate == true
  and .gates.boundary_gate == true
  and ([.probes[] | select(.scene_layer == "map" and .passes == true and .sampled_colors >= 3000)] | length) == 1
  and ([.probes[] | select(.scene_layer == "hud" and .passes == true and .sampled_colors >= 1000)] | length) == 1
  and ([.probes[] | select(.scene_layer == "actor" and .passes == true and .sampled_colors >= 3000)] | length) == 1
  and ([.probes[] | select(.scene_layer == "feedback" and .passes == true and .sampled_colors >= 1000)] | length) == 1
  and .asset_boundary == "live_window_png_region_probe_correlated_with_authored_visual_bridge_not_gpu_texture_claim"
  and .gpu_upload_claimed == false
  and .android_s5_real_device_claimed == false
  and .live_osm_ingestion_claimed == false
' "$SUMMARY" >/dev/null

echo "TRILLIONNIUM_WORLD_BEVY_LIVE_WINDOW_LAYER_PIXEL_PROBE_GREEN $SUMMARY final_frame=$FINAL_FRAME"
