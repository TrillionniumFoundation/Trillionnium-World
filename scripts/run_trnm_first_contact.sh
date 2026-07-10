#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR/trillionnium"

export PATH="$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin:${PATH:-}"
export WGPU_BACKEND="${WGPU_BACKEND:-vulkan}"
if [[ -n "${DISPLAY:-}" ]]; then
  export WINIT_UNIX_BACKEND="${WINIT_UNIX_BACKEND:-x11}"
  unset WAYLAND_DISPLAY
fi

PROFILE="${TRNM_GAME_PROFILE:-release}"
if [[ "$PROFILE" == "release" ]]; then
  CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" cargo build --release -p trnm-first-contact
  exec "$ROOT_DIR/target/release/trnm-first-contact"
fi

exec cargo run -p trnm-first-contact
