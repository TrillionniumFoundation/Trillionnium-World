#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR/trillionnium"

if [[ ("${TRNM_WORLD_BEVY_FORCE_X11:-0}" == "1" || "${TRNM_WORLD_BEVY_FORCE_X11_OPENGL:-0}" == "1") && -n "${DISPLAY:-}" ]]; then
  export WINIT_UNIX_BACKEND="${WINIT_UNIX_BACKEND:-x11}"
  unset WAYLAND_DISPLAY
fi

if [[ "${TRNM_WORLD_BEVY_FORCE_X11_OPENGL:-0}" == "1" ]]; then
  export WGPU_BACKEND="${WGPU_BACKEND:-gl}"
fi

exec cargo run -p trnm-world-bevy -- run
