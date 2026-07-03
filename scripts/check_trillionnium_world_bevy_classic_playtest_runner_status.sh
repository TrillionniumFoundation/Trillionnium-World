#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json"
MANUAL_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/manual_bevy"
PLAYER_SCREEN_SCREENSHOT="$MANUAL_DIR/bevy-classic-player-screen-runner-status.png"
PLAYER_SCREEN_XWD="$MANUAL_DIR/bevy-classic-player-screen-runner-status.xwd"
PLAYER_SCREEN_PROBE="$MANUAL_DIR/bevy-classic-player-screen-runner-status-probe.json"
SERVICE="${TRNM_WORLD_BEVY_PLAYTEST_SERVICE:-trillionnium-bevy-playtest.service}"
EXPECTED_BINARY="$ROOT/target/release/trnm-world-bevy"
EXPECTED_REPO_ROOT="$ROOT"
EXPECTED_CWD="$ROOT/trillionnium"
EXPECTED_MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
EXPECTED_OVERRIDE_DIR="$ROOT/assets/trnm-world/classic/art-pack-v1"

mkdir -p "$(dirname "$SUMMARY")" "$MANUAL_DIR"

systemctl_value() {
  systemctl --user show "$SERVICE" --property="$1" --value 2>/dev/null || true
}

proc_env_value() {
  local pid="$1"
  local key="$2"
  if [[ -r "/proc/$pid/environ" ]]; then
    tr '\0' '\n' <"/proc/$pid/environ" | awk -F= -v key="$key" '
      $1 == key {
        sub(/^[^=]*=/, "")
        print
        exit
      }
    '
  fi
}

ACTIVE_STATE="$(systemctl_value ActiveState)"
SUB_STATE="$(systemctl_value SubState)"
MAIN_PID_RAW="$(systemctl_value MainPID)"
EXEC_MAIN_STATUS="$(systemctl_value ExecMainStatus)"
CPU_WEIGHT="$(systemctl_value CPUWeight)"
CPU_QUOTA_PER_SEC_USEC="$(systemctl_value CPUQuotaPerSecUSec)"

MAIN_PID=0
if [[ "$MAIN_PID_RAW" =~ ^[0-9]+$ ]]; then
  MAIN_PID="$MAIN_PID_RAW"
fi

CMDLINE_JSON='[]'
CMDLINE_JOINED=""
PROCESS_CWD=""
LOW_SPEC_VALUE=""
CLASSIC_RENDERER_VALUE=""
CLASSIC_FPS_VALUE=""
CLASSIC_MANIFEST_VALUE=""
CLASSIC_OVERRIDE_DIR_VALUE=""
PLAYER_SCREEN_VALUE=""
WINIT_UNIX_BACKEND_VALUE=""
WAYLAND_DISPLAY_VALUE=""
DISPLAY_VALUE=""
XAUTHORITY_VALUE=""

if [[ "$MAIN_PID" -gt 0 && -d "/proc/$MAIN_PID" ]]; then
  if [[ -r "/proc/$MAIN_PID/cmdline" ]]; then
    CMDLINE_JSON="$(tr '\0' '\n' <"/proc/$MAIN_PID/cmdline" | awk 'NF > 0' | jq -R . | jq -s .)"
    CMDLINE_JOINED="$(jq -r 'join(" ")' <<<"$CMDLINE_JSON")"
  fi
  PROCESS_CWD="$(readlink -f "/proc/$MAIN_PID/cwd" 2>/dev/null || true)"
  LOW_SPEC_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_LOW_SPEC)"
  CLASSIC_RENDERER_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_CLASSIC_RENDERER)"
  CLASSIC_FPS_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_CLASSIC_FPS)"
  CLASSIC_MANIFEST_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST)"
  CLASSIC_OVERRIDE_DIR_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR)"
  PLAYER_SCREEN_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_CLASSIC_PLAYER_SCREEN)"
  WINIT_UNIX_BACKEND_VALUE="$(proc_env_value "$MAIN_PID" WINIT_UNIX_BACKEND)"
  WAYLAND_DISPLAY_VALUE="$(proc_env_value "$MAIN_PID" WAYLAND_DISPLAY)"
  DISPLAY_VALUE="$(proc_env_value "$MAIN_PID" DISPLAY)"
  XAUTHORITY_VALUE="$(proc_env_value "$MAIN_PID" XAUTHORITY)"
fi

if [[ -z "$DISPLAY_VALUE" ]]; then
  DISPLAY_VALUE="${DISPLAY:-:0}"
fi
if [[ -z "$XAUTHORITY_VALUE" ]]; then
  XAUTHORITY_VALUE="$(
    find "${XDG_RUNTIME_DIR:-/run/user/$(id -u)}" -maxdepth 1 -type f -name '.mutter-Xwaylandauth.*' -printf '%T@ %p\n' 2>/dev/null |
      sort -nr |
      awk 'NR == 1 {print $2}'
  )"
fi

CMD0="$(jq -r '.[0] // ""' <<<"$CMDLINE_JSON")"
HAS_RUN_ARG="$(jq -r 'index("run") != null' <<<"$CMDLINE_JSON")"
ENV_JSON="$(jq -n \
  --arg low_spec "$LOW_SPEC_VALUE" \
  --arg classic_renderer "$CLASSIC_RENDERER_VALUE" \
  --arg classic_fps "$CLASSIC_FPS_VALUE" \
  --arg classic_asset_manifest "$CLASSIC_MANIFEST_VALUE" \
  --arg classic_asset_override_dir "$CLASSIC_OVERRIDE_DIR_VALUE" \
  --arg classic_player_screen "$PLAYER_SCREEN_VALUE" \
  --arg winit_unix_backend "$WINIT_UNIX_BACKEND_VALUE" \
  --arg wayland_display "$WAYLAND_DISPLAY_VALUE" \
  --arg display "$DISPLAY_VALUE" \
  --arg xauthority "$XAUTHORITY_VALUE" \
  '{
    TRNM_WORLD_BEVY_LOW_SPEC: $low_spec,
    TRNM_WORLD_BEVY_CLASSIC_RENDERER: $classic_renderer,
    TRNM_WORLD_BEVY_CLASSIC_FPS: $classic_fps,
    TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST: $classic_asset_manifest,
    TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR: $classic_asset_override_dir,
    TRNM_WORLD_BEVY_CLASSIC_PLAYER_SCREEN: $classic_player_screen,
    WINIT_UNIX_BACKEND: $winit_unix_backend,
    WAYLAND_DISPLAY: $wayland_display,
    DISPLAY: $display,
    XAUTHORITY: $xauthority
  }')"

SERVICE_PROCESS_GATE=false
if [[ "$ACTIVE_STATE" == "active" && "$SUB_STATE" == "running" && "$MAIN_PID" -gt 0 && -d "/proc/$MAIN_PID" ]]; then
  SERVICE_PROCESS_GATE=true
fi

RELEASE_BINARY_GATE=false
if [[ "$CMD0" == "$EXPECTED_BINARY" && -x "$EXPECTED_BINARY" && "$HAS_RUN_ARG" == "true" ]]; then
  RELEASE_BINARY_GATE=true
fi

CLASSIC_ENV_GATE=false
if [[ "$LOW_SPEC_VALUE" == "1" && "$CLASSIC_RENDERER_VALUE" == "1" && "$CLASSIC_FPS_VALUE" == "30" ]]; then
  CLASSIC_ENV_GATE=true
fi

PLAYER_SCREEN_ENV_GATE=false
if [[ "$PLAYER_SCREEN_VALUE" == "1" ]]; then
  PLAYER_SCREEN_ENV_GATE=true
fi

X11_BACKEND_GATE=false
if [[ "$WINIT_UNIX_BACKEND_VALUE" == "x11" && -z "$WAYLAND_DISPLAY_VALUE" ]]; then
  X11_BACKEND_GATE=true
fi

MANIFEST_GATE=false
if [[ "$CLASSIC_MANIFEST_VALUE" == "$EXPECTED_MANIFEST" && -f "$EXPECTED_MANIFEST" ]]; then
  MANIFEST_GATE=true
fi

OVERRIDE_DIR_GATE=false
if [[ "$CLASSIC_OVERRIDE_DIR_VALUE" == "$EXPECTED_OVERRIDE_DIR" && -d "$EXPECTED_OVERRIDE_DIR" ]]; then
  OVERRIDE_DIR_GATE=true
fi

WORKDIR_GATE=false
if [[ "$PROCESS_CWD" == "$EXPECTED_CWD" ]]; then
  WORKDIR_GATE=true
fi

CPU_BUDGET_GATE=false
if [[ "$CPU_WEIGHT" == "50" && "$CPU_QUOTA_PER_SEC_USEC" == "500ms" ]]; then
  CPU_BUDGET_GATE=true
fi

COMBINED_RUNTIME_PATHS="$(printf '%s %s %s %s %s' "$CMDLINE_JOINED" "$PROCESS_CWD" "$CLASSIC_MANIFEST_VALUE" "$CLASSIC_OVERRIDE_DIR_VALUE" "$CMD0")"
CEX_PATH_GATE=true
if grep -qiE '(^|[[:space:]])/[^[:space:]]*/CEX(/|[[:space:]]|$)|(^|[[:space:]])/[^[:space:]]*/cex(/|[[:space:]]|$)' <<<"$COMBINED_RUNTIME_PATHS"; then
  CEX_PATH_GATE=false
fi

PLAYER_SCREEN_WINDOW_ID=""
PLAYER_SCREEN_WINDOW_TITLE=""
PLAYER_SCREEN_WINDOW_WIDTH=0
PLAYER_SCREEN_WINDOW_HEIGHT=0
PLAYER_SCREEN_WINDOW_TITLE_CHARS=0
PLAYER_SCREEN_WINDOW_GATE=false
PLAYER_SCREEN_TITLE_GATE=false
PLAYER_SCREEN_PROOF_DEBUG_ABSENT_GATE=false
PLAYER_SCREEN_TITLE_CONCISE_GATE=false
PLAYER_SCREEN_SCREENSHOT_GATE=false
PLAYER_SCREEN_REGION_GATE=false
PLAYER_SCREEN_DEAD_PANEL_GATE=false
PLAYER_SCREEN_CLIPPED_LABEL_GATE=false
PLAYER_SCREEN_GAMEPLAY_SCENE_GATE=false
PLAYER_SCREEN_DEBUG_TITLE_ABSENT_GATE=false
PLAYER_SCREEN_VISUAL_GATE=false
PLAYER_SCREEN_SCREENSHOT_BYTES=0

if [[ "$SERVICE_PROCESS_GATE" == "true" && "$PLAYER_SCREEN_ENV_GATE" == "true" && "$X11_BACKEND_GATE" == "true" && -n "$DISPLAY_VALUE" && -n "$XAUTHORITY_VALUE" ]]; then
  if command -v xwininfo >/dev/null 2>&1 && command -v xprop >/dev/null 2>&1; then
    WINDOW_TREE="$(
      DISPLAY="$DISPLAY_VALUE" XAUTHORITY="$XAUTHORITY_VALUE" xwininfo -root -tree 2>/dev/null || true
    )"
    PLAYER_SCREEN_WINDOW_ID="$(
      awk '/"Trillionnium RTS/ && /room=first-contact-basin/ && /1280x720/ {print $1; exit}' <<<"$WINDOW_TREE"
    )"
    if [[ -z "$PLAYER_SCREEN_WINDOW_ID" ]]; then
      PLAYER_SCREEN_WINDOW_ID="$(
        awk '/"Trillionnium RTS/ && /room=first-contact-basin/ {print $1; exit}' <<<"$WINDOW_TREE"
      )"
    fi
    if [[ -n "$PLAYER_SCREEN_WINDOW_ID" ]]; then
      PLAYER_SCREEN_WINDOW_TITLE="$(
        DISPLAY="$DISPLAY_VALUE" XAUTHORITY="$XAUTHORITY_VALUE" xprop -id "$PLAYER_SCREEN_WINDOW_ID" WM_NAME 2>/dev/null |
          sed -n 's/^WM_NAME([^)]*) = "\(.*\)"$/\1/p' |
          head -1
      )"
      WINDOW_INFO="$(
        DISPLAY="$DISPLAY_VALUE" XAUTHORITY="$XAUTHORITY_VALUE" xwininfo -id "$PLAYER_SCREEN_WINDOW_ID" 2>/dev/null || true
      )"
      PLAYER_SCREEN_WINDOW_WIDTH="$(
        awk '/Width:/ {print $2; exit}' <<<"$WINDOW_INFO"
      )"
      PLAYER_SCREEN_WINDOW_HEIGHT="$(
        awk '/Height:/ {print $2; exit}' <<<"$WINDOW_INFO"
      )"
      PLAYER_SCREEN_WINDOW_TITLE_CHARS="${#PLAYER_SCREEN_WINDOW_TITLE}"
      if [[ "$PLAYER_SCREEN_WINDOW_WIDTH" =~ ^[0-9]+$ && "$PLAYER_SCREEN_WINDOW_HEIGHT" =~ ^[0-9]+$ && "$PLAYER_SCREEN_WINDOW_WIDTH" -ge 1200 && "$PLAYER_SCREEN_WINDOW_HEIGHT" -ge 690 ]]; then
        PLAYER_SCREEN_WINDOW_GATE=true
      fi
      if [[ "$PLAYER_SCREEN_WINDOW_TITLE" == *"Trillionnium RTS"* && "$PLAYER_SCREEN_WINDOW_TITLE" == *"room=first-contact-basin"* && "$PLAYER_SCREEN_WINDOW_TITLE" == *"owned-assets-v1"* ]]; then
        PLAYER_SCREEN_TITLE_GATE=true
      fi
      if ! grep -Eiq 'DESKTOP PRODUCT ALIGNMENT|MAP-FIRST ALIGNMENT|proof|debug|mirror-city-square|title menu|engineering dashboard|LMB|RMB|Ctrl|Shift|WASD|wheel zoom|attack-move|shortcut' <<<"$PLAYER_SCREEN_WINDOW_TITLE"; then
        PLAYER_SCREEN_PROOF_DEBUG_ABSENT_GATE=true
      fi
      if [[ "$PLAYER_SCREEN_WINDOW_TITLE_CHARS" -le 96 ]]; then
        PLAYER_SCREEN_TITLE_CONCISE_GATE=true
      fi
    fi
  fi
fi

if [[ "$PLAYER_SCREEN_WINDOW_GATE" == "true" && "$PLAYER_SCREEN_TITLE_GATE" == "true" ]] &&
  command -v xwd >/dev/null 2>&1 && command -v ffmpeg >/dev/null 2>&1 && command -v python3 >/dev/null 2>&1; then
  if DISPLAY="$DISPLAY_VALUE" XAUTHORITY="$XAUTHORITY_VALUE" xwd -silent -id "$PLAYER_SCREEN_WINDOW_ID" -out "$PLAYER_SCREEN_XWD" >/dev/null 2>&1 &&
    DISPLAY="$DISPLAY_VALUE" XAUTHORITY="$XAUTHORITY_VALUE" ffmpeg -y -hide_banner -loglevel error -i "$PLAYER_SCREEN_XWD" "$PLAYER_SCREEN_SCREENSHOT" >/dev/null 2>&1; then
    python3 - "$PLAYER_SCREEN_SCREENSHOT" "$PLAYER_SCREEN_PROBE" "$PLAYER_SCREEN_WINDOW_TITLE" <<'PY'
import json
import sys
from pathlib import Path

from PIL import Image, ImageStat

screenshot = Path(sys.argv[1])
probe_path = Path(sys.argv[2])
window_title = sys.argv[3] if len(sys.argv) > 3 else ""
image = Image.open(screenshot).convert("RGB")
width, height = image.size

def region(name, box, min_colors, min_stddev):
    x0, y0, x1, y1 = box
    x0 = max(0, min(width - 1, x0))
    y0 = max(0, min(height - 1, y0))
    x1 = max(x0 + 1, min(width, x1))
    y1 = max(y0 + 1, min(height, y1))
    crop = image.crop((x0, y0, x1, y1))
    sampled = crop.resize((max(1, crop.size[0] // 4), max(1, crop.size[1] // 4)))
    sampled_colors = len(sampled.getcolors(maxcolors=1_000_000) or [])
    stat = ImageStat.Stat(crop)
    avg_stddev = round(sum(stat.stddev) / 3.0, 2)
    passes = sampled_colors >= min_colors and avg_stddev >= min_stddev
    return {
        "id": name,
        "region": [x0, y0, x1, y1],
        "sampled_colors": sampled_colors,
        "min_sampled_colors": min_colors,
        "mean": [round(value, 2) for value in stat.mean],
        "stddev": [round(value, 2) for value in stat.stddev],
        "avg_stddev": avg_stddev,
        "min_avg_stddev": min_stddev,
        "passes": passes,
    }

def edge_safety_sample(name, box, max_high_contrast_pixels, max_foreground_pixels):
    x0, y0, x1, y1 = box
    x0 = max(0, min(width - 1, x0))
    y0 = max(0, min(height - 1, y0))
    x1 = max(x0 + 1, min(width, x1))
    y1 = max(y0 + 1, min(height, y1))
    crop = image.crop((x0, y0, x1, y1))
    high_contrast_pixels = 0
    foreground_pixels = 0
    for red, green, blue in crop.getdata():
        luma = (red + green + blue) / 3.0
        chroma = max(red, green, blue) - min(red, green, blue)
        if chroma >= 70 and luma >= 140:
            high_contrast_pixels += 1
        if chroma >= 25 and luma >= 95:
            foreground_pixels += 1
    return {
        "id": name,
        "region": [x0, y0, x1, y1],
        "high_contrast_pixels": high_contrast_pixels,
        "max_high_contrast_pixels": max_high_contrast_pixels,
        "foreground_pixels": foreground_pixels,
        "max_foreground_pixels": max_foreground_pixels,
        "passes": (
            high_contrast_pixels <= max_high_contrast_pixels
            and foreground_pixels <= max_foreground_pixels
        ),
    }

def exact_color_component_summary(name, rgb, max_component_width, max_component_height):
    pixels = image.load()
    pending = set()
    for sample_y in range(height):
        for sample_x in range(width):
            if pixels[sample_x, sample_y] == rgb:
                pending.add((sample_x, sample_y))

    components = []
    while pending:
        start = pending.pop()
        stack = [start]
        xs = []
        ys = []
        count = 0
        while stack:
            x, y = stack.pop()
            xs.append(x)
            ys.append(y)
            count += 1
            for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                if (nx, ny) in pending:
                    pending.remove((nx, ny))
                    stack.append((nx, ny))
        x0 = min(xs)
        y0 = min(ys)
        x1 = max(xs)
        y1 = max(ys)
        components.append(
            {
                "pixels": count,
                "width": x1 - x0 + 1,
                "height": y1 - y0 + 1,
                "bounds": [x0, y0, x1, y1],
            }
        )

    components.sort(key=lambda item: (item["pixels"], item["width"], item["height"]), reverse=True)
    broad_components = [
        item
        for item in components
        if item["width"] > max_component_width or item["height"] > max_component_height
    ]
    return {
        "id": name,
        "rgb": list(rgb),
        "pixel_count": sum(item["pixels"] for item in components),
        "component_count": len(components),
        "max_component_width": max([item["width"] for item in components] or [0]),
        "max_component_height": max([item["height"] for item in components] or [0]),
        "max_allowed_component_width": max_component_width,
        "max_allowed_component_height": max_component_height,
        "broad_component_count": len(broad_components),
        "broad_components": broad_components[:12],
        "top_components": components[:12],
        "passes": not broad_components,
    }

regions = [
    region("map_playfield", [32, 72, min(940, width), min(610, height)], 2500, 30.0),
    region("top_hud", [0, 0, width, min(90, height)], 250, 18.0),
    region("right_hud", [max(0, width - 320), 72, width, min(610, height)], 500, 20.0),
    region("bottom_command", [0, max(0, height - 110), width, height], 500, 20.0),
    region("minimap", [max(0, width - 300), 90, max(1, width - 24), min(300, height)], 100, 14.0),
    region("center_map", [260, 140, min(820, width), min(520, height)], 2000, 35.0),
]
regions_by_id = {item["id"]: item for item in regions}
full = region("full", [0, 0, width, height], 3000, 25.0)
dead_panel_regions = []
for panel_id in ["top_hud", "right_hud", "bottom_command", "minimap"]:
    item = regions_by_id[panel_id]
    min_sampled_colors = int(item["min_sampled_colors"] * 1.5)
    min_avg_stddev = round(item["min_avg_stddev"] + 4.0, 2)
    dead_panel_regions.append(
        {
            "id": panel_id,
            "sampled_colors": item["sampled_colors"],
            "min_sampled_colors": min_sampled_colors,
            "avg_stddev": item["avg_stddev"],
            "min_avg_stddev": min_avg_stddev,
            "passes": item["sampled_colors"] >= min_sampled_colors
            and item["avg_stddev"] >= min_avg_stddev,
        }
    )
edge_safety_samples = [
    edge_safety_sample("right_hud_right_edge", [width - 10, 72, width, min(610, height)], 120, 240),
    edge_safety_sample("bottom_command_bottom_edge", [0, height - 10, width, height], 500, 900),
    edge_safety_sample("bottom_command_bottom_quiet_band", [0, height - 4, width, height], 24, 40),
    edge_safety_sample("top_hud_top_edge", [0, 0, width, 6], 100, 180),
]
exact_red_component_sample = exact_color_component_summary(
    "attack_feedback_exact_red",
    (255, 114, 114),
    16,
    16,
)
exact_green_component_sample = exact_color_component_summary(
    "ability_range_exact_green",
    (140, 255, 143),
    16,
    16,
)
exact_blueprint_component_sample = exact_color_component_summary(
    "build_blueprint_exact_cyan",
    (200, 232, 255),
    12,
    10,
)
exact_scaffold_component_sample = exact_color_component_summary(
    "relay_scaffold_exact_tan",
    (200, 157, 98),
    28,
    14,
)
exact_color_component_samples = [
    exact_red_component_sample,
    exact_green_component_sample,
    exact_blueprint_component_sample,
    exact_scaffold_component_sample,
]
forbidden_title_fragments = [
    "desktop product alignment",
    "map-first alignment",
    "proof",
    "debug",
    "mirror-city-square",
    "title menu",
    "engineering dashboard",
    "lmb",
    "rmb",
    "ctrl",
    "shift",
    "wasd",
    "wheel zoom",
    "attack-move",
    "shortcut",
]
window_title_lc = window_title.lower()
gates = {
    "screenshot_file_gate": screenshot.exists() and screenshot.stat().st_size > 8192,
    "image_size_gate": width >= 1200 and height >= 690,
    "full_nonblank_gate": full["passes"],
    "region_complexity_gate": all(item["passes"] for item in regions),
    "first_contact_visual_balance_gate": (
        regions_by_id["map_playfield"]["sampled_colors"] > regions_by_id["right_hud"]["sampled_colors"]
        and regions_by_id["center_map"]["sampled_colors"] > regions_by_id["minimap"]["sampled_colors"]
    ),
    "dead_panel_gate": all(item["passes"] for item in dead_panel_regions),
    "clipped_label_edge_gate": all(item["passes"] for item in edge_safety_samples),
    "exact_red_micro_component_gate": exact_red_component_sample["passes"],
    "exact_green_micro_component_gate": exact_green_component_sample["passes"],
    "exact_blueprint_micro_component_gate": exact_blueprint_component_sample["passes"],
    "exact_scaffold_thin_component_gate": exact_scaffold_component_sample["passes"],
    "gameplay_scene_gate": (
        regions_by_id["map_playfield"]["sampled_colors"] >= regions_by_id["map_playfield"]["min_sampled_colors"] * 2
        and regions_by_id["center_map"]["sampled_colors"] >= regions_by_id["center_map"]["min_sampled_colors"] * 1.8
        and regions_by_id["map_playfield"]["avg_stddev"] >= regions_by_id["map_playfield"]["min_avg_stddev"] + 10.0
        and regions_by_id["center_map"]["avg_stddev"] >= regions_by_id["center_map"]["min_avg_stddev"] + 10.0
        and regions_by_id["minimap"]["sampled_colors"] >= regions_by_id["minimap"]["min_sampled_colors"] * 1.8
        and regions_by_id["map_playfield"]["sampled_colors"] > regions_by_id["right_hud"]["sampled_colors"] * 2
    ),
    "debug_title_text_absent_gate": bool(window_title.strip())
    and not any(fragment in window_title_lc for fragment in forbidden_title_fragments),
}
summary = {
    "contract_version": "trillionnium_world_bevy_classic_player_screen_runner_visual_v1",
    "screenshot_path": str(screenshot),
    "screenshot_bytes": screenshot.stat().st_size if screenshot.exists() else 0,
    "image_size": [width, height],
    "window_title": window_title,
    "full_probe": full,
    "regions": regions,
    "dead_panel_regions": dead_panel_regions,
    "edge_safety_samples": edge_safety_samples,
    "exact_color_component_samples": exact_color_component_samples,
    "forbidden_title_fragments": forbidden_title_fragments,
    "gates": gates,
    "green": all(gates.values()),
    "source_of_truth": "The live classic playtest runner must show the First Contact Basin player screen as a real gameplay map/HUD/command surface, not merely a running process, proof/debug panel, empty/dead panel layout, clipped-label frame, bottom-edge HUD strip, or non-gameplay-looking screenshot.",
}
probe_path.write_text(json.dumps(summary, indent=2), encoding="utf-8")
PY
  fi
fi

if [[ -s "$PLAYER_SCREEN_PROBE" ]]; then
  PLAYER_SCREEN_SCREENSHOT_BYTES="$(jq -r '.screenshot_bytes // 0' "$PLAYER_SCREEN_PROBE")"
  if jq -e '.gates.screenshot_file_gate == true and .gates.image_size_gate == true and .gates.full_nonblank_gate == true' "$PLAYER_SCREEN_PROBE" >/dev/null; then
    PLAYER_SCREEN_SCREENSHOT_GATE=true
  fi
  if jq -e '.gates.region_complexity_gate == true and .gates.first_contact_visual_balance_gate == true' "$PLAYER_SCREEN_PROBE" >/dev/null; then
    PLAYER_SCREEN_REGION_GATE=true
  fi
  if jq -e '.gates.dead_panel_gate == true' "$PLAYER_SCREEN_PROBE" >/dev/null; then
    PLAYER_SCREEN_DEAD_PANEL_GATE=true
  fi
  if jq -e '.gates.clipped_label_edge_gate == true' "$PLAYER_SCREEN_PROBE" >/dev/null; then
    PLAYER_SCREEN_CLIPPED_LABEL_GATE=true
  fi
  if jq -e '.gates.gameplay_scene_gate == true' "$PLAYER_SCREEN_PROBE" >/dev/null; then
    PLAYER_SCREEN_GAMEPLAY_SCENE_GATE=true
  fi
  if jq -e '.gates.debug_title_text_absent_gate == true' "$PLAYER_SCREEN_PROBE" >/dev/null; then
    PLAYER_SCREEN_DEBUG_TITLE_ABSENT_GATE=true
  fi
  if jq -e '.green == true' "$PLAYER_SCREEN_PROBE" >/dev/null; then
    PLAYER_SCREEN_VISUAL_GATE=true
  fi
fi

GREEN=false
if [[ "$SERVICE_PROCESS_GATE" == "true" && "$RELEASE_BINARY_GATE" == "true" && "$CLASSIC_ENV_GATE" == "true" && "$PLAYER_SCREEN_ENV_GATE" == "true" && "$X11_BACKEND_GATE" == "true" && "$MANIFEST_GATE" == "true" && "$OVERRIDE_DIR_GATE" == "true" && "$WORKDIR_GATE" == "true" && "$CPU_BUDGET_GATE" == "true" && "$CEX_PATH_GATE" == "true" && "$PLAYER_SCREEN_WINDOW_GATE" == "true" && "$PLAYER_SCREEN_TITLE_GATE" == "true" && "$PLAYER_SCREEN_PROOF_DEBUG_ABSENT_GATE" == "true" && "$PLAYER_SCREEN_TITLE_CONCISE_GATE" == "true" && "$PLAYER_SCREEN_SCREENSHOT_GATE" == "true" && "$PLAYER_SCREEN_REGION_GATE" == "true" && "$PLAYER_SCREEN_DEAD_PANEL_GATE" == "true" && "$PLAYER_SCREEN_CLIPPED_LABEL_GATE" == "true" && "$PLAYER_SCREEN_GAMEPLAY_SCENE_GATE" == "true" && "$PLAYER_SCREEN_DEBUG_TITLE_ABSENT_GATE" == "true" && "$PLAYER_SCREEN_VISUAL_GATE" == "true" ]]; then
  GREEN=true
fi

STATUS=blocked
if [[ "$GREEN" == "true" ]]; then
  STATUS=green
fi

MANIFEST_SHA256=""
if [[ -f "$EXPECTED_MANIFEST" ]]; then
  MANIFEST_SHA256="$(sha256sum "$EXPECTED_MANIFEST" | awk '{print $1}')"
fi

jq -n \
  --arg contract_version "trillionnium_world_bevy_classic_playtest_runner_status_v1" \
  --arg status "$STATUS" \
  --arg service "$SERVICE" \
  --arg active_state "$ACTIVE_STATE" \
  --arg sub_state "$SUB_STATE" \
  --arg main_pid "$MAIN_PID" \
  --arg exec_main_status "$EXEC_MAIN_STATUS" \
  --arg cpu_weight "$CPU_WEIGHT" \
  --arg cpu_quota_per_sec_usec "$CPU_QUOTA_PER_SEC_USEC" \
  --arg expected_binary "$EXPECTED_BINARY" \
  --arg expected_repo_root "$EXPECTED_REPO_ROOT" \
  --arg expected_cwd "$EXPECTED_CWD" \
  --arg process_cwd "$PROCESS_CWD" \
  --arg expected_manifest "$EXPECTED_MANIFEST" \
  --arg expected_override_dir "$EXPECTED_OVERRIDE_DIR" \
  --arg manifest_sha256 "$MANIFEST_SHA256" \
  --arg player_screen_window_id "$PLAYER_SCREEN_WINDOW_ID" \
  --arg player_screen_window_title "$PLAYER_SCREEN_WINDOW_TITLE" \
  --arg player_screen_window_width "$PLAYER_SCREEN_WINDOW_WIDTH" \
  --arg player_screen_window_height "$PLAYER_SCREEN_WINDOW_HEIGHT" \
  --arg player_screen_window_title_chars "$PLAYER_SCREEN_WINDOW_TITLE_CHARS" \
  --arg player_screen_screenshot "$PLAYER_SCREEN_SCREENSHOT" \
  --arg player_screen_probe "$PLAYER_SCREEN_PROBE" \
  --arg player_screen_screenshot_bytes "$PLAYER_SCREEN_SCREENSHOT_BYTES" \
  --argjson green "$GREEN" \
  --argjson cmdline "$CMDLINE_JSON" \
  --argjson selected_environment "$ENV_JSON" \
  --argjson service_process_gate "$SERVICE_PROCESS_GATE" \
  --argjson release_binary_gate "$RELEASE_BINARY_GATE" \
  --argjson classic_env_gate "$CLASSIC_ENV_GATE" \
  --argjson player_screen_env_gate "$PLAYER_SCREEN_ENV_GATE" \
  --argjson x11_backend_gate "$X11_BACKEND_GATE" \
  --argjson manifest_gate "$MANIFEST_GATE" \
  --argjson override_dir_gate "$OVERRIDE_DIR_GATE" \
  --argjson workdir_gate "$WORKDIR_GATE" \
  --argjson cpu_budget_gate "$CPU_BUDGET_GATE" \
  --argjson cex_path_gate "$CEX_PATH_GATE" \
  --argjson player_screen_window_gate "$PLAYER_SCREEN_WINDOW_GATE" \
  --argjson player_screen_title_gate "$PLAYER_SCREEN_TITLE_GATE" \
  --argjson player_screen_proof_debug_absent_gate "$PLAYER_SCREEN_PROOF_DEBUG_ABSENT_GATE" \
  --argjson player_screen_title_concise_gate "$PLAYER_SCREEN_TITLE_CONCISE_GATE" \
  --argjson player_screen_screenshot_gate "$PLAYER_SCREEN_SCREENSHOT_GATE" \
  --argjson player_screen_region_gate "$PLAYER_SCREEN_REGION_GATE" \
  --argjson player_screen_dead_panel_gate "$PLAYER_SCREEN_DEAD_PANEL_GATE" \
  --argjson player_screen_clipped_label_gate "$PLAYER_SCREEN_CLIPPED_LABEL_GATE" \
  --argjson player_screen_gameplay_scene_gate "$PLAYER_SCREEN_GAMEPLAY_SCENE_GATE" \
  --argjson player_screen_debug_title_absent_gate "$PLAYER_SCREEN_DEBUG_TITLE_ABSENT_GATE" \
  --argjson player_screen_visual_gate "$PLAYER_SCREEN_VISUAL_GATE" \
  '{
    contract_version: $contract_version,
    status: $status,
    green: $green,
    service: {
      unit: $service,
      active_state: $active_state,
      sub_state: $sub_state,
      main_pid: ($main_pid | tonumber),
      exec_main_status: $exec_main_status,
      cpu_weight: $cpu_weight,
      cpu_quota_per_sec_usec: $cpu_quota_per_sec_usec,
      expected_cpu_weight: "50",
      expected_cpu_quota_per_sec_usec: "500ms"
    },
    runtime: {
      expected_binary: $expected_binary,
      expected_repo_root: $expected_repo_root,
      expected_cwd: $expected_cwd,
      process_cwd: $process_cwd,
      expected_manifest: $expected_manifest,
      expected_override_dir: $expected_override_dir,
      manifest_sha256: (if $manifest_sha256 == "" then null else $manifest_sha256 end),
      cmdline: $cmdline,
      selected_environment: $selected_environment
    },
    live_player_screen: {
      window_id: (if $player_screen_window_id == "" then null else $player_screen_window_id end),
      window_title: $player_screen_window_title,
      window_title_chars: ($player_screen_window_title_chars | tonumber? // 0),
      window_width: ($player_screen_window_width | tonumber? // 0),
      window_height: ($player_screen_window_height | tonumber? // 0),
      screenshot_path: $player_screen_screenshot,
      probe_path: $player_screen_probe,
      screenshot_bytes: ($player_screen_screenshot_bytes | tonumber? // 0),
      contract_version: "trillionnium_world_bevy_classic_player_screen_runner_visual_v1",
      title_rule: "concise player-facing title keeps room and owned-art-pack identity while excluding debug/proof strings and shortcut manuals"
    },
    gates: {
      service_process_gate: $service_process_gate,
      release_binary_gate: $release_binary_gate,
      classic_env_gate: $classic_env_gate,
      player_screen_env_gate: $player_screen_env_gate,
      x11_backend_gate: $x11_backend_gate,
      manifest_gate: $manifest_gate,
      override_dir_gate: $override_dir_gate,
      workdir_gate: $workdir_gate,
      cpu_budget_gate: $cpu_budget_gate,
      cex_path_gate: $cex_path_gate,
      player_screen_window_gate: $player_screen_window_gate,
      player_screen_title_gate: $player_screen_title_gate,
      player_screen_proof_debug_absent_gate: $player_screen_proof_debug_absent_gate,
      player_screen_title_concise_gate: $player_screen_title_concise_gate,
      player_screen_screenshot_gate: $player_screen_screenshot_gate,
      player_screen_region_gate: $player_screen_region_gate,
      player_screen_dead_panel_gate: $player_screen_dead_panel_gate,
      player_screen_clipped_label_gate: $player_screen_clipped_label_gate,
      player_screen_gameplay_scene_gate: $player_screen_gameplay_scene_gate,
      player_screen_debug_title_absent_gate: $player_screen_debug_title_absent_gate,
      player_screen_visual_gate: $player_screen_visual_gate
    },
    ready_for_release_review: true,
    public_launch_ready: false,
    android_s5_real_device_claimed: false,
    source_of_truth: "The live playtest runner must be the release trnm-world-bevy binary with the low-spec classic player screen, X11 backend, classic renderer manifest, a concise player-facing First Contact Basin window title, a bounded CPUQuota/CPUWeight budget, and a visible First Contact Basin player screen with real map/HUD/command pixels, non-dead HUD panels, clipped-label edge safety, and gameplay-scene balance; CEX paths are explicitly rejected, and proof/debug/shortcut-manual default title strings are explicitly rejected."
  }
  | .runtime_cmdline_arg_count = (.runtime.cmdline | length)
  | .selected_environment_count = (.runtime.selected_environment | keys | length)
  | .live_player_screen_evidence_path_count = ([.live_player_screen.screenshot_path, .live_player_screen.probe_path] | map(select(. != null and . != "")) | length)
  | .live_player_screen_pixel_count = (.live_player_screen.window_width * .live_player_screen.window_height)
  | .gate_count = (.gates | keys | length)
  | .passed_gate_count = ([.gates[] | select(. == true)] | length)
  | .failed_gate_count = ([.gates[] | select(. != true)] | length)' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_playtest_runner_status_v1"
  and .green == true
  and .ready_for_release_review == true
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .runtime_cmdline_arg_count == (.runtime.cmdline | length)
  and .runtime_cmdline_arg_count == 2
  and .selected_environment_count == (.runtime.selected_environment | keys | length)
  and .selected_environment_count >= 8
  and .live_player_screen_evidence_path_count == ([.live_player_screen.screenshot_path, .live_player_screen.probe_path] | map(select(. != null and . != "")) | length)
  and .live_player_screen_evidence_path_count == 2
  and .live_player_screen_pixel_count == (.live_player_screen.window_width * .live_player_screen.window_height)
  and .live_player_screen_pixel_count >= 828000
  and .gate_count == (.gates | keys | length)
  and .passed_gate_count == ([.gates[] | select(. == true)] | length)
  and .failed_gate_count == ([.gates[] | select(. != true)] | length)
  and .failed_gate_count == 0
  and .service.unit == "trillionnium-bevy-playtest.service"
  and .service.active_state == "active"
  and .service.sub_state == "running"
  and .service.main_pid > 0
  and (.runtime.cmdline[0] | contains("/target/release/trnm-world-bevy"))
  and (.runtime.cmdline | index("run") != null)
  and .runtime.selected_environment.TRNM_WORLD_BEVY_LOW_SPEC == "1"
  and .runtime.selected_environment.TRNM_WORLD_BEVY_CLASSIC_RENDERER == "1"
  and .runtime.selected_environment.TRNM_WORLD_BEVY_CLASSIC_FPS == "30"
  and .runtime.selected_environment.TRNM_WORLD_BEVY_CLASSIC_PLAYER_SCREEN == "1"
  and .runtime.selected_environment.WINIT_UNIX_BACKEND == "x11"
  and .runtime.selected_environment.WAYLAND_DISPLAY == ""
  and .service.cpu_weight == "50"
  and .service.cpu_quota_per_sec_usec == "500ms"
  and (.runtime.selected_environment.TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST | contains("/assets/trnm-world/classic/manifest.json"))
  and (.runtime.selected_environment.TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR | contains("/assets/trnm-world/classic/art-pack-v1"))
  and .gates.service_process_gate == true
  and .gates.release_binary_gate == true
  and .gates.classic_env_gate == true
  and .gates.player_screen_env_gate == true
  and .gates.x11_backend_gate == true
  and .gates.manifest_gate == true
  and .gates.override_dir_gate == true
  and .gates.workdir_gate == true
  and .gates.cpu_budget_gate == true
  and .gates.cex_path_gate == true
  and .live_player_screen.contract_version == "trillionnium_world_bevy_classic_player_screen_runner_visual_v1"
  and (.live_player_screen.window_title | contains("Trillionnium RTS"))
  and (.live_player_screen.window_title | contains("room=first-contact-basin"))
  and (.live_player_screen.window_title | contains("owned-assets-v1"))
  and (.live_player_screen.window_title | contains("mirror-city-square") | not)
  and (.live_player_screen.window_title | contains("LMB") | not)
  and (.live_player_screen.window_title | contains("Ctrl") | not)
  and (.live_player_screen.window_title | contains("Shift") | not)
  and (.live_player_screen.window_title_chars <= 96)
  and .live_player_screen.window_width >= 1200
  and .live_player_screen.window_height >= 690
  and .live_player_screen.screenshot_bytes > 8192
  and .gates.player_screen_window_gate == true
  and .gates.player_screen_title_gate == true
  and .gates.player_screen_proof_debug_absent_gate == true
  and .gates.player_screen_title_concise_gate == true
  and .gates.player_screen_screenshot_gate == true
  and .gates.player_screen_region_gate == true
  and .gates.player_screen_dead_panel_gate == true
  and .gates.player_screen_clipped_label_gate == true
  and .gates.player_screen_gameplay_scene_gate == true
  and .gates.player_screen_debug_title_absent_gate == true
  and .gates.player_screen_visual_gate == true
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYTEST_RUNNER_STATUS_GREEN %s\n' "$SUMMARY"
