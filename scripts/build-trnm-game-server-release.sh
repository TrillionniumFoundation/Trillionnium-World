#!/usr/bin/bash
set -euo pipefail
builtin umask 077

# Bash can import exported functions before it starts a non-interactive script.
# They must not be allowed to shadow commands after PATH is pinned.
while IFS= read -r inherited_function_name; do
  builtin unset -f "$inherited_function_name"
done < <(builtin compgen -A function)
unset inherited_function_name

readonly TRUSTED_SYSTEM_PATH="/usr/bin:/bin"

fail() {
  printf 'TRNM game-server release build failed: %s\n' "$*" >&2
  exit 1
}

# Refuse explicit compiler, loader, shell-startup, and language-runtime
# injection, including RUSTC_WRAPPER, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS,
# LD_PRELOAD, BASH_ENV, ENV, and PYTHONPATH. Everything else inherited from
# the operator environment is cleared before source/toolchain selection.
release_root_override="${TRNM_GAME_SERVER_RELEASE_ROOT:-}"
trusted_target_override="${TRNM_GAME_SERVER_RELEASE_TARGET_DIR:-}"
inherited_environment_names=()
while IFS= read -r environment_name; do
  inherited_environment_names+=("$environment_name")
  case "$environment_name" in
    RUST*|CARGO*|LD|LD_*|DYLD_*|BASH_ENV|ENV|BASHOPTS|SHELLOPTS|PYTHON*|PERL5OPT|PERL5LIB|RUBYOPT|RUBYLIB|NODE_OPTIONS|NODE_PATH|\
      CC|CXX|CPP|AR|AS|NM|OBJCOPY|OBJDUMP|RANLIB|READELF|STRIP|CFLAGS|CXXFLAGS|CPPFLAGS|LDFLAGS|\
      MAKEFLAGS|MFLAGS|PKG_CONFIG|PKG_CONFIG_*|CMAKE_TOOLCHAIN_FILE|CMAKE_PREFIX_PATH|\
      BINDGEN_EXTRA_CLANG_ARGS|CLANG_PATH|LIBCLANG_PATH|LLVM_CONFIG_PATH|GCC_EXEC_PREFIX|COMPILER_PATH|LIBRARY_PATH|CPATH|C_INCLUDE_PATH|CPLUS_INCLUDE_PATH|\
      GLIBC_TUNABLES|GCONV_PATH|GETCONF_DIR|HOSTALIASES|LOCALDOMAIN|LOCPATH|MALLOC_*|NLSPATH|RES_OPTIONS)
      fail "prohibited inherited build environment variable is set: $environment_name"
      ;;
  esac
done < <(builtin compgen -e)
for environment_name in "${inherited_environment_names[@]}"; do
  builtin unset "$environment_name" 2>/dev/null \
    || builtin export -n "$environment_name" 2>/dev/null \
    || fail "could not clear inherited environment variable: $environment_name"
done
unset inherited_environment_names environment_name

export PATH="$TRUSTED_SYSTEM_PATH"
export LC_ALL=C
export LANG=C
export TZ=UTC
export GIT_CONFIG_NOSYSTEM=1
export GIT_CONFIG_GLOBAL=/dev/null
export GIT_PAGER=cat
hash -r

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
RELEASE_ROOT="${release_root_override:-$ROOT_DIR/run/releases/trnm-game-server}"
TRUSTED_TARGET_DIR="$trusted_target_override"
unset release_root_override trusted_target_override

current_uid="$(id -u)"
mkdir -p -- "$RELEASE_ROOT"
[[ -d "$RELEASE_ROOT" && ! -L "$RELEASE_ROOT" ]] \
  || fail "release root must be a real directory: $RELEASE_ROOT"
[[ "$(stat -c '%u' -- "$RELEASE_ROOT")" == "$current_uid" ]] \
  || fail "release root must be held by the current owner: $RELEASE_ROOT"
chmod 0700 -- "$RELEASE_ROOT"
[[ "$(stat -c '%a' -- "$RELEASE_ROOT")" == 700 ]] \
  || fail "release root could not be made owner-only: $RELEASE_ROOT"
RELEASE_ROOT="$(realpath -e -- "$RELEASE_ROOT")"
release_root_parent="$(dirname "$RELEASE_ROOT")"
[[ -d "$release_root_parent" && ! -L "$release_root_parent" \
    && "$(stat -c '%u' -- "$release_root_parent")" == "$current_uid" ]] \
  || fail "release-root parent must be a real owner-held directory: $release_root_parent"
chmod 0700 -- "$release_root_parent"
[[ "$(stat -c '%a' -- "$release_root_parent")" == 700 ]] \
  || fail "release-root parent could not be made owner-only: $release_root_parent"
account_record="$(getent passwd "$current_uid")" \
  || fail "could not resolve the current account"
IFS=: read -r account_name _ account_uid _ _ account_home _ <<<"$account_record"
[[ "$account_uid" == "$current_uid" && "$account_home" == /* \
    && -d "$account_home" && ! -L "$account_home" ]] \
  || fail "current account has no trusted absolute home directory"
account_home="$(realpath -e -- "$account_home")"
rustup_home="$account_home/.rustup"
cargo_home="$account_home/.cargo"
rustup_bin="$cargo_home/bin/rustup"

require_trusted_executable() {
  local executable="$1" owner mode links
  [[ -f "$executable" && ! -L "$executable" && -x "$executable" ]] \
    || fail "trusted tool is not a regular executable: $executable"
  read -r owner mode links < <(stat -c '%u %a %h' -- "$executable")
  [[ "$owner" == "$current_uid" && "$links" == 1 ]] \
    || fail "trusted tool has unsafe ownership or link count: $executable"
  (( (8#$mode & 8#022) == 0 )) \
    || fail "trusted tool is group- or world-writable: $executable"
}

require_trusted_executable "$rustup_bin"
for cargo_config in "$cargo_home/config" "$cargo_home/config.toml"; do
  [[ ! -e "$cargo_config" && ! -L "$cargo_config" ]] \
    || fail "formal builds do not accept user-level Cargo configuration: $cargo_config"
done

resolve_toolchain_command() {
  local command_name="$1"
  (
    cd "$ROOT_DIR"
    /usr/bin/env -i \
      HOME="$account_home" \
      PATH="$TRUSTED_SYSTEM_PATH" \
      LC_ALL=C \
      LANG=C \
      RUSTUP_HOME="$rustup_home" \
      "$rustup_bin" which "$command_name"
  )
}

cargo_bin="$(realpath -e -- "$(resolve_toolchain_command cargo)")"
rustc_bin="$(realpath -e -- "$(resolve_toolchain_command rustc)")"
[[ "$cargo_bin" == "$rustup_home"/toolchains/*/bin/cargo \
    && "$rustc_bin" == "$(dirname "$cargo_bin")/rustc" ]] \
  || fail "active Cargo and rustc are not a co-located rustup toolchain"
require_trusted_executable "$cargo_bin"
require_trusted_executable "$rustc_bin"
readonly TRUSTED_BUILD_PATH="$(dirname "$cargo_bin"):$TRUSTED_SYSTEM_PATH"
cargo_sha256="$(sha256sum "$cargo_bin" | awk '{print $1}')"
rustc_sha256="$(sha256sum "$rustc_bin" | awk '{print $1}')"
cargo_tool_identity="$(stat -c '%d:%i:%s:%Y' -- "$cargo_bin"):$cargo_sha256"
rustc_tool_identity="$(stat -c '%d:%i:%s:%Y' -- "$rustc_bin"):$rustc_sha256"

require_toolchain_unchanged() {
  require_trusted_executable "$cargo_bin"
  require_trusted_executable "$rustc_bin"
  [[ "$(stat -c '%d:%i:%s:%Y' -- "$cargo_bin"):$(sha256sum "$cargo_bin" | awk '{print $1}')" == \
      "$cargo_tool_identity" ]] \
    || fail "Cargo changed during the source-bound release build"
  [[ "$(stat -c '%d:%i:%s:%Y' -- "$rustc_bin"):$(sha256sum "$rustc_bin" | awk '{print $1}')" == \
      "$rustc_tool_identity" ]] \
    || fail "rustc changed during the source-bound release build"
}

repo_git() {
  command git -c core.fsmonitor=false -C "$ROOT_DIR" "$@"
}

require_clean_head() {
  local expected_head="$1"
  [[ "$(repo_git rev-parse HEAD)" == "$expected_head" ]] \
    || fail "HEAD changed while the source-bound release was being built"
  [[ -z "$(repo_git status --porcelain --untracked-files=all)" ]] \
    || fail "the worktree changed while the source-bound release was being built"
}

if [[ -n "$(repo_git status --porcelain --untracked-files=all)" ]]; then
  fail "refusing to package a dirty worktree; commit or deliberately partition the WIP first"
fi
[[ -z "$TRUSTED_TARGET_DIR" ]] \
  || fail "formal v2 bundles require a fresh isolated Cargo target; TRNM_GAME_SERVER_RELEASE_TARGET_DIR is prohibited"

git_commit="$(repo_git rev-parse HEAD)"
git_tree="$(repo_git rev-parse 'HEAD^{tree}')"
source_date_epoch="$(repo_git show -s --format=%ct HEAD)"
rustc_version="$("$rustc_bin" --version)"
cargo_version="$("$cargo_bin" --version)"
target_triple="$("$rustc_bin" -vV | awk -F': ' '$1 == "host" {print $2}')"
[[ -n "$target_triple" ]] || fail "rustc did not report a host target triple"
toolchain_identity_sha256="$(
  printf '%s\0%s\0%s\0%s\0%s\0' \
    "$cargo_version" \
    "$cargo_sha256" \
    "$rustc_version" \
    "$rustc_sha256" \
    "$target_triple" \
    | sha256sum \
    | awk '{print $1}'
)"
build_recipe_sha256="$(sha256sum \
  "$ROOT_DIR/scripts/build-trnm-game-server-release.sh" | awk '{print $1}')"
release_id="${git_commit:0:12}-${git_tree:0:12}-${toolchain_identity_sha256:0:12}"
release_dir="$RELEASE_ROOT/$release_id"
require_clean_head "$git_commit"

verification_matches_current_builder() {
  jq -e \
    --arg rustc_version "$rustc_version" \
    --arg cargo_version "$cargo_version" \
    --arg target_triple "$target_triple" \
    --arg cargo_sha256 "$cargo_sha256" \
    --arg rustc_sha256 "$rustc_sha256" \
    --arg toolchain_identity_sha256 "$toolchain_identity_sha256" \
    --arg release_id "$release_id" \
    --arg build_recipe_sha256 "$build_recipe_sha256" '
      .verified == true and
      .release_contract_version == "trnm_game_server_release_v2" and
      .fault_harness_capable == true and
      .isolated_target == true and
      .trusted_target_cache_used == false and
      .build_toolchain.rustc_version == $rustc_version and
      .build_toolchain.cargo_version == $cargo_version and
      .build_toolchain.target_triple == $target_triple and
      .build_toolchain.cargo_sha256 == $cargo_sha256 and
      .build_toolchain.rustc_sha256 == $rustc_sha256 and
      .build_toolchain.identity_sha256 == $toolchain_identity_sha256 and
      .release_id == $release_id and
      .build_recipe.sha256 == $build_recipe_sha256
    ' >/dev/null
}

if [[ -d "$release_dir" ]]; then
  existing_verification="$(
    "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_dir"
  )"
  if verification_matches_current_builder <<<"$existing_verification"; then
    printf '%s\n' "$release_dir"
    exit 0
  fi
  fail "release ID already exists under an older immutable bundle contract: $release_dir"
fi

work_dir="$(mktemp -d "$RELEASE_ROOT/.build.XXXXXX")"
source_dir="$work_dir/source"
staging="$(mktemp -d "$RELEASE_ROOT/.staging.XXXXXX")"
# Formal evidence must never reuse build-script output or fingerprints from a
# mutable development or operator-managed Cargo target.
target_dir="$work_dir/target"
cleanup() {
  if [[ -d "$staging" ]]; then
    chmod u+w "$staging" 2>/dev/null || true
    rm -rf -- "$staging"
  fi
  rm -rf -- "$work_dir"
}
trap cleanup EXIT
mkdir -p "$source_dir"

# Compile an archive of the captured commit rather than the mutable checkout.
# The clean-worktree gates still ensure operators do not omit uncommitted
# release changes, while the archive closes the build-time source race.
repo_git archive --format=tar "$git_commit" | tar -xf - -C "$source_dir"

manifest_path="$source_dir/trillionnium/Cargo.toml"
lock_path="$source_dir/trillionnium/Cargo.lock"
package_manifest_path="$source_dir/trillionnium/crates/trnm-game-server/Cargo.toml"

(
  cd "$source_dir"
  /usr/bin/env -i \
    HOME="$account_home" \
    USER="$account_name" \
    LOGNAME="$account_name" \
    PATH="$TRUSTED_BUILD_PATH" \
    LC_ALL=C \
    LANG=C \
    TZ=UTC \
    CARGO_HOME="$cargo_home" \
    RUSTUP_HOME="$rustup_home" \
    CARGO_TARGET_DIR="$target_dir" \
    CARGO_INCREMENTAL=0 \
    CARGO_NET_OFFLINE=true \
    SOURCE_DATE_EPOCH="$source_date_epoch" \
    RUSTC="$rustc_bin" \
    "$cargo_bin" build \
      --manifest-path "$manifest_path" \
      --package trnm-game-server \
      --bin trnm-game-server \
      --bin trnm-online-e2e \
      --release \
      --locked \
      --frozen
)

require_toolchain_unchanged
require_clean_head "$git_commit"
install -m 0555 "$target_dir/release/trnm-game-server" "$staging/trnm-game-server"
install -m 0555 "$target_dir/release/trnm-online-e2e" "$staging/trnm-online-e2e"
install -m 0444 "$lock_path" "$staging/Cargo.lock"
install -m 0444 "$manifest_path" "$staging/workspace-Cargo.toml"
install -m 0444 "$package_manifest_path" "$staging/trnm-game-server-Cargo.toml"
game_server_sha256="$(sha256sum "$staging/trnm-game-server" | awk '{print $1}')"
online_e2e_sha256="$(sha256sum "$staging/trnm-online-e2e" | awk '{print $1}')"
cargo_lock_sha256="$(sha256sum "$staging/Cargo.lock" | awk '{print $1}')"
workspace_manifest_sha256="$(sha256sum "$staging/workspace-Cargo.toml" | awk '{print $1}')"
package_manifest_sha256="$(sha256sum "$staging/trnm-game-server-Cargo.toml" | awk '{print $1}')"
jq -n \
  --arg contract_version "trnm_game_server_release_v2" \
  --arg release_id "$release_id" \
  --arg git_commit "$git_commit" \
  --arg git_tree "$git_tree" \
  --arg cargo_lock_sha256 "$cargo_lock_sha256" \
  --arg workspace_manifest_sha256 "$workspace_manifest_sha256" \
  --arg package_manifest_sha256 "$package_manifest_sha256" \
  --arg game_server_sha256 "$game_server_sha256" \
  --arg online_e2e_sha256 "$online_e2e_sha256" \
  --arg build_recipe_sha256 "$build_recipe_sha256" \
  --arg rustc_version "$rustc_version" \
  --arg cargo_version "$cargo_version" \
  --arg target_triple "$target_triple" \
  --arg cargo_sha256 "$cargo_sha256" \
  --arg rustc_sha256 "$rustc_sha256" \
  --arg toolchain_identity_sha256 "$toolchain_identity_sha256" \
  --arg built_at_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson source_date_epoch "$source_date_epoch" \
  '{
    contract_version: $contract_version,
    release_id: $release_id,
    git_commit: $git_commit,
    git_tree: $git_tree,
    source_dirty: false,
    isolated_target: true,
    trusted_target_cache_used: false,
    build_recipe_path: "scripts/build-trnm-game-server-release.sh",
    build_recipe_sha256: $build_recipe_sha256,
    source_date_epoch: $source_date_epoch,
    cargo_lock_path: "Cargo.lock",
    cargo_lock_sha256: $cargo_lock_sha256,
    workspace_manifest_path: "workspace-Cargo.toml",
    workspace_manifest_sha256: $workspace_manifest_sha256,
    package_manifest_path: "trnm-game-server-Cargo.toml",
    package_manifest_sha256: $package_manifest_sha256,
    binaries: {
      game_server: {
        path: "trnm-game-server",
        sha256: $game_server_sha256
      },
      online_e2e: {
        path: "trnm-online-e2e",
        sha256: $online_e2e_sha256
      }
    },
    rustc_version: $rustc_version,
    rustc_sha256: $rustc_sha256,
    cargo_version: $cargo_version,
    cargo_sha256: $cargo_sha256,
    target_triple: $target_triple,
    toolchain_identity_sha256: $toolchain_identity_sha256,
    built_at_utc: $built_at_utc
  }' >"$staging/release-manifest.json"
chmod 0444 "$staging/release-manifest.json"
release_manifest_sha256="$(sha256sum "$staging/release-manifest.json" | awk '{print $1}')"
printf '%s  %s\n' \
  "$release_manifest_sha256" \
  "release-manifest.json" >"$staging/release-manifest.sha256"
chmod 0444 "$staging/release-manifest.sha256"

fsync_regular_file() {
  /usr/bin/python3 - "$1" <<'PY'
import os
import stat
import sys

flags = os.O_RDONLY | getattr(os, "O_CLOEXEC", 0) | getattr(os, "O_NOFOLLOW", 0)
fd = os.open(sys.argv[1], flags)
try:
    if not stat.S_ISREG(os.fstat(fd).st_mode):
        raise SystemExit("release member is not a regular file")
    os.fsync(fd)
finally:
    os.close(fd)
PY
}

fsync_directory() {
  /usr/bin/python3 - "$1" <<'PY'
import os
import sys

flags = os.O_RDONLY | getattr(os, "O_DIRECTORY", 0) | getattr(os, "O_CLOEXEC", 0)
fd = os.open(sys.argv[1], flags)
try:
    os.fsync(fd)
finally:
    os.close(fd)
PY
}

"$ROOT_DIR/scripts/check-trnm-game-server-release.sh" --staging "$staging" >/dev/null
require_clean_head "$git_commit"
for durable_member in \
  "$staging/trnm-game-server" \
  "$staging/trnm-online-e2e" \
  "$staging/Cargo.lock" \
  "$staging/workspace-Cargo.toml" \
  "$staging/trnm-game-server-Cargo.toml" \
  "$staging/release-manifest.json" \
  "$staging/release-manifest.sha256"; do
  fsync_regular_file "$durable_member"
done
fsync_directory "$staging"
chmod 0555 "$staging"
fsync_directory "$staging"
if ! mv -T "$staging" "$release_dir" 2>/dev/null; then
  if [[ -d "$release_dir" ]]; then
    racing_verification="$(
      "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_dir"
    )"
    verification_matches_current_builder <<<"$racing_verification" \
      || fail "a racing publisher installed a release with a different bundle recipe or toolchain"
  else
    fail "could not publish the verified bundle at $release_dir"
  fi
else
  # The selector must never be allowed to outlive an uncommitted rename of its
  # immutable target.  Persist the target directory and the publishing parent
  # before returning it to a promoter.
  fsync_directory "$release_dir"
  fsync_directory "$RELEASE_ROOT"
fi
published_verification="$(
  "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_dir"
)"
verification_matches_current_builder <<<"$published_verification" \
  || fail "published release does not match the current isolated build recipe and toolchain"
printf '%s\n' "$release_dir"
