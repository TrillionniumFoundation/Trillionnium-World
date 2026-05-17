#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

missing=0
ACCEPTANCE_DIR="$ROOT/acceptance/S0_world_dev_environment/latest"
WORLD_PACKAGES=(
  trnm-world-domain
  trnm-world-command
  trnm-world-projection
  trnm-world-map-provider
  trnm-world-ui-fragments
  trnm-world-api
  trnm-world-server
  trnm-world-bevy
  trnm-world-dev-env
)

check_cmd() {
  local name="$1"
  local cmd="$2"
  if bash -lc "$cmd" >/tmp/trillionnium-world-env-check.out 2>&1; then
    printf 'OK   %s: %s\n' "$name" "$(head -n1 /tmp/trillionnium-world-env-check.out)"
  else
    printf 'MISS %s: %s\n' "$name" "$cmd"
    missing=1
  fi
}

check_path() {
  local name="$1"
  local path="$2"
  if [[ -e "$path" ]]; then
    printf 'OK   %s: %s\n' "$name" "$path"
  else
    printf 'MISS %s: %s\n' "$name" "$path"
    missing=1
  fi
}

check_cmd rustc 'rustc --version'
check_cmd cargo 'cargo --version'
check_cmd rustfmt 'rustfmt --version'
check_cmd clippy 'cargo clippy --version'
check_cmd node 'node --version'
check_cmd npm 'npm --version'
check_cmd cmake 'cmake --version'
check_cmd pkg-config 'pkg-config --version'
check_cmd clang 'clang --version'
check_cmd lld 'ld.lld --version'
check_cmd java 'java -version 2>&1'
check_cmd adb 'adb version'
check_cmd godot 'if command -v godot >/dev/null 2>&1; then godot --version; elif command -v godot3 >/dev/null 2>&1; then godot3 --version; else exit 127; fi'

ANDROID_SDK_ROOT="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-/usr/lib/android-sdk}}"
ANDROID_NDK_ROOT="${ANDROID_NDK_ROOT:-${ANDROID_NDK_HOME:-}}"
if [[ -z "$ANDROID_NDK_ROOT" ]]; then
  ANDROID_NDK_ROOT="$(find "$ANDROID_SDK_ROOT" /usr/lib -maxdepth 4 -type f -name ndk-build 2>/dev/null | head -n1 | xargs -r dirname || true)"
fi
if [[ "${ANDROID_NDK_ROOT##*/}" == "build" && -d "$(dirname "$ANDROID_NDK_ROOT")/toolchains" ]]; then
  ANDROID_NDK_ROOT="$(dirname "$ANDROID_NDK_ROOT")"
fi
check_path android-sdk "$ANDROID_SDK_ROOT"
check_path android-platform-tools "$ANDROID_SDK_ROOT/platform-tools"
check_path android-build-tools "$ANDROID_SDK_ROOT/build-tools"
check_path android-cmdline-tools "$ANDROID_SDK_ROOT/cmdline-tools"
if [[ -n "$ANDROID_NDK_ROOT" ]]; then
  check_path android-ndk "$ANDROID_NDK_ROOT"
else
  printf 'MISS android-ndk: set ANDROID_NDK_ROOT/ANDROID_NDK_HOME or install NDK under %s\n' "$ANDROID_SDK_ROOT"
  missing=1
fi

ANDROID_API_LEVEL="${ANDROID_API_LEVEL:-24}"
ANDROID_HOST_TAG="${ANDROID_HOST_TAG:-linux-x86_64}"
ANDROID_TOOLCHAIN_BIN="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$ANDROID_HOST_TAG/bin"
ANDROID_AARCH64_CLANG="$ANDROID_TOOLCHAIN_BIN/aarch64-linux-android${ANDROID_API_LEVEL}-clang"
ANDROID_AARCH64_CLANGXX="$ANDROID_TOOLCHAIN_BIN/aarch64-linux-android${ANDROID_API_LEVEL}-clang++"
ANDROID_LLVM_AR="$ANDROID_TOOLCHAIN_BIN/llvm-ar"
ANDROID_LLVM_RANLIB="$ANDROID_TOOLCHAIN_BIN/llvm-ranlib"
if [[ -n "$ANDROID_NDK_ROOT" ]]; then
  check_path android-aarch64-clang "$ANDROID_AARCH64_CLANG"
  check_path android-aarch64-clang++ "$ANDROID_AARCH64_CLANGXX"
  check_path android-llvm-ar "$ANDROID_LLVM_AR"
  check_path android-llvm-ranlib "$ANDROID_LLVM_RANLIB"
  export ANDROID_NDK_HOME="$ANDROID_NDK_ROOT"
  export CC_aarch64_linux_android="$ANDROID_AARCH64_CLANG"
  export CXX_aarch64_linux_android="$ANDROID_AARCH64_CLANGXX"
  export AR_aarch64_linux_android="$ANDROID_LLVM_AR"
  export RANLIB_aarch64_linux_android="$ANDROID_LLVM_RANLIB"
  export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$ANDROID_AARCH64_CLANG"
fi

for target in wasm32-unknown-unknown aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android; do
  if rustup target list --installed | grep -qx "$target"; then
    printf 'OK   rust-target: %s\n' "$target"
  else
    printf 'MISS rust-target: %s\n' "$target"
    missing=1
  fi
done

if [[ ! -d web4-frontend/node_modules ]]; then
  printf 'MISS web4-frontend/node_modules: run npm ci in web4-frontend\n'
  missing=1
else
  printf 'OK   web4-frontend/node_modules\n'
fi

(
  cd trillionnium
  mkdir -p "$ACCEPTANCE_DIR"
  check_world_server_output() {
    local output_name="$1"
    local expected="$2"
    shift 2
    cargo world-server "$@" >"$ACCEPTANCE_DIR/$output_name"
    grep -q "$expected" "$ACCEPTANCE_DIR/$output_name"
  }
  for package in "${WORLD_PACKAGES[@]}"; do
    cargo fmt -p "$package" -- --check
  done
  cargo world-check
  cargo world-test
  cargo check -p trnm-world-bevy --target aarch64-linux-android >/tmp/trillionnium-world-bevy-android-check.out
  cargo world-env >"$ACCEPTANCE_DIR/world-env-report.json"
  cargo world-bevy >"$ACCEPTANCE_DIR/bevy-bridge-report.json"
  grep -q 'trillionnium_world_bevy_native_client_v1' "$ACCEPTANCE_DIR/bevy-bridge-report.json"
  check_world_server_output home-json.json 'trillionnium_world_api_v1' home-json
  check_world_server_output cex-map-home-json.json '"node_count": 24' cex-map-home-json
  check_world_server_output home-fragment.html 'data-render-owner="rust_world_ui_renderer"' home-fragment
  check_world_server_output move-east.json 'league-coliseum' move-east
  check_world_server_output route-target-reject.json 'world-work-reject-id' route-target '/work reject latest 重试拒收退款'
  check_world_server_output route-artifacts.json 'rejection_chargeback_recovery' route-artifacts
  check_world_server_output map-runtime-budget.json 'trillionnium_world_map_runtime_performance_budget_v1' map-runtime-budget
  check_world_server_output tactics-command-train-skill.json 'skill_training_recorded' tactics-command train_skill
  check_world_server_output tactics-command-equip-item.json 'item_equipped' tactics-command equip_item
  check_world_server_output adapter-readiness.json 'WorldLedgerAdapter' adapter-readiness
  check_world_server_output dev-runtime-smoke.json 'trillionnium_world_dev_runtime_v1' dev-runtime-smoke
  cargo world-server dev-runtime-repository-smoke "$ACCEPTANCE_DIR/world-dev-runtime-state.json" >"$ACCEPTANCE_DIR/dev-runtime-repository-smoke.json"
  grep -q 'file_repository_persistence_green' "$ACCEPTANCE_DIR/dev-runtime-repository-smoke.json"
  cargo world-server full-split-json >"$ACCEPTANCE_DIR/full-split.json"
  grep -q 'trillionnium_world_full_split_response_v1' "$ACCEPTANCE_DIR/full-split.json"
)

if [[ "$missing" -ne 0 ]]; then
  printf 'TRILLIONNIUM_WORLD_DEV_ENV_NOT_READY\n'
  exit 1
fi

printf 'TRILLIONNIUM_WORLD_DEV_ENV_READY\n'
