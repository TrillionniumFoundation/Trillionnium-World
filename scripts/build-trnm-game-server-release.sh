#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RELEASE_ROOT="${TRNM_GAME_SERVER_RELEASE_ROOT:-$ROOT_DIR/run/releases/trnm-game-server}"
TRUSTED_TARGET_DIR="${TRNM_GAME_SERVER_RELEASE_TARGET_DIR:-}"

fail() {
  echo "TRNM game-server release build failed: $*" >&2
  exit 1
}

require_clean_head() {
  local expected_head="$1"
  [[ "$(git -C "$ROOT_DIR" rev-parse HEAD)" == "$expected_head" ]] \
    || fail "HEAD changed while the source-bound release was being built"
  [[ -z "$(git -C "$ROOT_DIR" status --porcelain --untracked-files=all)" ]] \
    || fail "the worktree changed while the source-bound release was being built"
}

if [[ -n "$(git -C "$ROOT_DIR" status --porcelain --untracked-files=all)" ]]; then
  fail "refusing to package a dirty worktree; commit or deliberately partition the WIP first"
fi

git_commit="$(git -C "$ROOT_DIR" rev-parse HEAD)"
git_tree="$(git -C "$ROOT_DIR" rev-parse 'HEAD^{tree}')"
source_date_epoch="$(git -C "$ROOT_DIR" show -s --format=%ct HEAD)"
release_id="${git_commit:0:12}-${git_tree:0:12}"
release_dir="$RELEASE_ROOT/$release_id"
require_clean_head "$git_commit"

mkdir -p "$RELEASE_ROOT"
if [[ -d "$release_dir" ]]; then
  "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_dir" >/dev/null
  printf '%s\n' "$release_dir"
  exit 0
fi

work_dir="$(mktemp -d "$RELEASE_ROOT/.build.XXXXXX")"
source_dir="$work_dir/source"
staging="$(mktemp -d "$RELEASE_ROOT/.staging.XXXXXX")"
if [[ -n "$TRUSTED_TARGET_DIR" ]]; then
  target_dir="$TRUSTED_TARGET_DIR"
else
  # A release build must not reuse build-script output or fingerprints from the
  # mutable development target. Operators may opt into a separately managed,
  # trusted target directory explicitly when they accept that cache boundary.
  target_dir="$work_dir/target"
fi
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
git -C "$ROOT_DIR" archive --format=tar "$git_commit" | tar -xf - -C "$source_dir"

manifest_path="$source_dir/trillionnium/Cargo.toml"
lock_path="$source_dir/trillionnium/Cargo.lock"
package_manifest_path="$source_dir/trillionnium/crates/trnm-game-server/Cargo.toml"

export CARGO_INCREMENTAL=0
export SOURCE_DATE_EPOCH="$source_date_epoch"
CARGO_TARGET_DIR="$target_dir" cargo build \
  --manifest-path "$manifest_path" \
  --package trnm-game-server \
  --bin trnm-game-server \
  --release \
  --locked

require_clean_head "$git_commit"
install -m 0555 "$target_dir/release/trnm-game-server" "$staging/trnm-game-server"
install -m 0444 "$lock_path" "$staging/Cargo.lock"
install -m 0444 "$manifest_path" "$staging/workspace-Cargo.toml"
install -m 0444 "$package_manifest_path" "$staging/trnm-game-server-Cargo.toml"
binary_sha256="$(sha256sum "$staging/trnm-game-server" | awk '{print $1}')"
cargo_lock_sha256="$(sha256sum "$staging/Cargo.lock" | awk '{print $1}')"
workspace_manifest_sha256="$(sha256sum "$staging/workspace-Cargo.toml" | awk '{print $1}')"
package_manifest_sha256="$(sha256sum "$staging/trnm-game-server-Cargo.toml" | awk '{print $1}')"
target_triple="$(rustc -vV | awk -F': ' '$1 == "host" {print $2}')"

jq -n \
  --arg contract_version "trnm_game_server_release_v1" \
  --arg release_id "$release_id" \
  --arg git_commit "$git_commit" \
  --arg git_tree "$git_tree" \
  --arg cargo_lock_sha256 "$cargo_lock_sha256" \
  --arg workspace_manifest_sha256 "$workspace_manifest_sha256" \
  --arg package_manifest_sha256 "$package_manifest_sha256" \
  --arg binary_sha256 "$binary_sha256" \
  --arg rustc_version "$(rustc --version)" \
  --arg cargo_version "$(cargo --version)" \
  --arg target_triple "$target_triple" \
  --arg built_at_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson source_date_epoch "$source_date_epoch" \
  '{
    contract_version: $contract_version,
    release_id: $release_id,
    git_commit: $git_commit,
    git_tree: $git_tree,
    source_dirty: false,
    source_date_epoch: $source_date_epoch,
    cargo_lock_path: "Cargo.lock",
    cargo_lock_sha256: $cargo_lock_sha256,
    workspace_manifest_path: "workspace-Cargo.toml",
    workspace_manifest_sha256: $workspace_manifest_sha256,
    package_manifest_path: "trnm-game-server-Cargo.toml",
    package_manifest_sha256: $package_manifest_sha256,
    binary_path: "trnm-game-server",
    binary_sha256: $binary_sha256,
    rustc_version: $rustc_version,
    cargo_version: $cargo_version,
    target_triple: $target_triple,
    built_at_utc: $built_at_utc
  }' >"$staging/release-manifest.json"
chmod 0444 "$staging/release-manifest.json"
release_manifest_sha256="$(sha256sum "$staging/release-manifest.json" | awk '{print $1}')"
printf '%s  %s\n' \
  "$release_manifest_sha256" \
  "release-manifest.json" >"$staging/release-manifest.sha256"
chmod 0444 "$staging/release-manifest.sha256"

"$ROOT_DIR/scripts/check-trnm-game-server-release.sh" --staging "$staging" >/dev/null
require_clean_head "$git_commit"
chmod 0555 "$staging"
if ! mv -T "$staging" "$release_dir" 2>/dev/null; then
  if [[ -d "$release_dir" ]]; then
    "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_dir" >/dev/null
  else
    fail "could not publish the verified bundle at $release_dir"
  fi
fi
"$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_dir" >/dev/null
printf '%s\n' "$release_dir"
