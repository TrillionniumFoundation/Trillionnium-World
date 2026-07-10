#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR/trillionnium"

export PATH="$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin:${PATH:-}"
export TRNM_WORLD_BEVY_LOW_SPEC="${TRNM_WORLD_BEVY_LOW_SPEC:-0}"
export TRNM_WORLD_BEVY_CLASSIC_RENDERER="${TRNM_WORLD_BEVY_CLASSIC_RENDERER:-0}"

if [[ "${TRNM_WORLD_BEVY_CLASSIC_RENDERER:-0}" == "1" ]]; then
  export TRNM_WORLD_BEVY_CLASSIC_PLAYER_SCREEN="${TRNM_WORLD_BEVY_CLASSIC_PLAYER_SCREEN:-1}"
  export TRNM_WORLD_BEVY_CLASSIC_FPS="${TRNM_WORLD_BEVY_CLASSIC_FPS:-30}"
  export TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="${TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST:-$ROOT_DIR/assets/trnm-world/classic/manifest.json}"
  export TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR="${TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR:-$ROOT_DIR/assets/trnm-world/classic/art-pack-v1}"
fi

if [[ "${TRNM_WORLD_BEVY_CLASSIC_RENDERER:-0}" != "1" ]]; then
  # Ivy Bridge OpenGL does not expose a Bevy-compatible adapter on this host.
  # Vulkan selects Mesa/llvmpipe when hardware Vulkan is unavailable, keeping
  # the real Bevy product path runnable without silently falling back to minifb.
  export WGPU_BACKEND="${WGPU_BACKEND:-vulkan}"
  if [[ -n "${DISPLAY:-}" ]]; then
    export WINIT_UNIX_BACKEND="${WINIT_UNIX_BACKEND:-x11}"
    unset WAYLAND_DISPLAY
  fi
fi

if [[ (
  "${TRNM_WORLD_BEVY_FORCE_X11:-0}" == "1" ||
  "${TRNM_WORLD_BEVY_FORCE_X11_OPENGL:-0}" == "1" ||
  ("${TRNM_WORLD_BEVY_CLASSIC_RENDERER:-0}" == "1" && "${TRNM_WORLD_BEVY_ALLOW_WAYLAND:-0}" != "1")
) && -n "${DISPLAY:-}" ]]; then
  export WINIT_UNIX_BACKEND="${WINIT_UNIX_BACKEND:-x11}"
  export XMODIFIERS="${TRNM_WORLD_BEVY_XMODIFIERS:-@im=none}"
  unset WAYLAND_DISPLAY
fi

if [[ "${TRNM_WORLD_BEVY_FORCE_X11_OPENGL:-0}" == "1" ]]; then
  export WGPU_BACKEND="${WGPU_BACKEND:-gl}"
fi

PROFILE="${TRNM_WORLD_BEVY_PROFILE:-release}"
if [[ "${TRNM_WORLD_BEVY_CLASSIC_RENDERER:-0}" == "1" ]]; then
  PLAYER_PACKAGE="trnm-world-bevy"
  PLAYER_BINARY="trnm-world-bevy"
else
  PLAYER_PACKAGE="trnm-first-contact"
  PLAYER_BINARY="trnm-first-contact"
fi
if [[ "$PROFILE" == "release" ]]; then
  CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" cargo build --release -p "$PLAYER_PACKAGE" --bin "$PLAYER_BINARY"
  exec "$ROOT_DIR/target/release/$PLAYER_BINARY"
fi

exec cargo run -p "$PLAYER_PACKAGE"
