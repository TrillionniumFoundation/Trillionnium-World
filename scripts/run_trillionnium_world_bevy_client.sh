#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR/trillionnium"

export TRNM_WORLD_BEVY_LOW_SPEC="${TRNM_WORLD_BEVY_LOW_SPEC:-1}"

if [[ "${TRNM_WORLD_BEVY_LOW_SPEC}" == "1" && -z "${TRNM_WORLD_BEVY_CLASSIC_RENDERER:-}" ]]; then
  export TRNM_WORLD_BEVY_CLASSIC_RENDERER=1
fi

if [[ "${TRNM_WORLD_BEVY_CLASSIC_RENDERER:-0}" == "1" ]]; then
  export TRNM_WORLD_BEVY_CLASSIC_FPS="${TRNM_WORLD_BEVY_CLASSIC_FPS:-30}"
  export TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="${TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST:-$ROOT_DIR/assets/trnm-world/classic/manifest.json}"
  export TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR="${TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR:-$ROOT_DIR/assets/trnm-world/classic/art-pack-v1}"
fi

if [[ "${TRNM_WORLD_BEVY_LOW_SPEC}" == "1" && "${TRNM_WORLD_BEVY_CLASSIC_RENDERER:-0}" != "1" && "${TRNM_WORLD_BEVY_ALLOW_CPU_VULKAN:-0}" != "1" ]]; then
  export WGPU_BACKEND="${WGPU_BACKEND:-gl}"
fi

if [[ ("${TRNM_WORLD_BEVY_FORCE_X11:-0}" == "1" || "${TRNM_WORLD_BEVY_FORCE_X11_OPENGL:-0}" == "1") && -n "${DISPLAY:-}" ]]; then
  export WINIT_UNIX_BACKEND="${WINIT_UNIX_BACKEND:-x11}"
  unset WAYLAND_DISPLAY
fi

if [[ "${TRNM_WORLD_BEVY_FORCE_X11_OPENGL:-0}" == "1" ]]; then
  export WGPU_BACKEND="${WGPU_BACKEND:-gl}"
fi

PROFILE="${TRNM_WORLD_BEVY_PROFILE:-release}"
if [[ "$PROFILE" == "release" ]]; then
  CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" cargo build -p trnm-world-bevy --release
  exec "$ROOT_DIR/target/release/trnm-world-bevy" run
fi

exec cargo run -p trnm-world-bevy -- run
