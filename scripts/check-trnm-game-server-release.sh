#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ALLOW_STAGING=0
if [[ "${1:-}" == "--staging" ]]; then
  ALLOW_STAGING=1
  shift
fi
(( $# <= 1 )) || {
  echo "usage: check-trnm-game-server-release.sh [--staging] [RELEASE_DIR]" >&2
  exit 64
}
REQUESTED_RELEASE_DIR="${1:-${TRNM_GAME_SERVER_RELEASE_DIR:-$ROOT_DIR/run/releases/trnm-game-server/current}}"
RELEASE_DIR=""
MANIFEST="$RELEASE_DIR/release-manifest.json"
MANIFEST_DIGEST="$RELEASE_DIR/release-manifest.sha256"
BINARY="$RELEASE_DIR/trnm-game-server"
LOCKFILE="$RELEASE_DIR/Cargo.lock"
WORKSPACE_MANIFEST="$RELEASE_DIR/workspace-Cargo.toml"
PACKAGE_MANIFEST="$RELEASE_DIR/trnm-game-server-Cargo.toml"

fail() {
  echo "TRNM game-server release verification failed: $*" >&2
  exit 1
}

RELEASE_DIR="$(realpath -e -- "$REQUESTED_RELEASE_DIR" 2>/dev/null)" \
  || fail "release directory does not exist or has a dangling link: $REQUESTED_RELEASE_DIR"
[[ -d "$RELEASE_DIR" ]] || fail "release path is not a directory: $REQUESTED_RELEASE_DIR"
MANIFEST="$RELEASE_DIR/release-manifest.json"
MANIFEST_DIGEST="$RELEASE_DIR/release-manifest.sha256"
BINARY="$RELEASE_DIR/trnm-game-server"
LOCKFILE="$RELEASE_DIR/Cargo.lock"
WORKSPACE_MANIFEST="$RELEASE_DIR/workspace-Cargo.toml"
PACKAGE_MANIFEST="$RELEASE_DIR/trnm-game-server-Cargo.toml"

[[ -f "$MANIFEST" ]] || fail "release manifest is missing: $MANIFEST"
[[ -f "$MANIFEST_DIGEST" ]] || fail "release manifest digest is missing: $MANIFEST_DIGEST"
[[ -f "$BINARY" && -x "$BINARY" ]] || fail "release binary is missing or not executable: $BINARY"
[[ -f "$LOCKFILE" ]] || fail "release Cargo.lock is missing: $LOCKFILE"
[[ -f "$WORKSPACE_MANIFEST" ]] || fail "bundled workspace Cargo.toml is missing"
[[ -f "$PACKAGE_MANIFEST" ]] || fail "bundled game-server Cargo.toml is missing"

for release_file in \
  "$MANIFEST" \
  "$MANIFEST_DIGEST" \
  "$BINARY" \
  "$LOCKFILE" \
  "$WORKSPACE_MANIFEST" \
  "$PACKAGE_MANIFEST"; do
  [[ ! -L "$release_file" ]] || fail "bundle members must not be symbolic links: $release_file"
  [[ "$(stat -c '%h' "$release_file")" == "1" ]] \
    || fail "bundle members must not be externally mutable hard links: $release_file"
done

actual_entries="$(find "$RELEASE_DIR" -mindepth 1 -maxdepth 1 -printf '%f\n' | LC_ALL=C sort)"
expected_entries="$(printf '%s\n' \
  Cargo.lock \
  release-manifest.json \
  release-manifest.sha256 \
  trnm-game-server \
  trnm-game-server-Cargo.toml \
  workspace-Cargo.toml | LC_ALL=C sort)"
[[ "$actual_entries" == "$expected_entries" ]] \
  || fail "release directory members do not match the contract (found: ${actual_entries//$'\n'/, })"

[[ "$(stat -c '%a' "$BINARY")" == "555" ]] \
  || fail "release binary mode must be 0555"
for readonly_file in \
  "$MANIFEST" \
  "$MANIFEST_DIGEST" \
  "$LOCKFILE" \
  "$WORKSPACE_MANIFEST" \
  "$PACKAGE_MANIFEST"; do
  [[ "$(stat -c '%a' "$readonly_file")" == "444" ]] \
    || fail "release metadata mode must be 0444: $readonly_file"
done
if (( ALLOW_STAGING == 0 )); then
  [[ "$(stat -c '%a' "$RELEASE_DIR")" == "555" ]] \
    || fail "published release directory mode must be 0555"
fi

manifest_digest_line="$(<"$MANIFEST_DIGEST")"
[[ "$manifest_digest_line" =~ ^[0-9a-f]{64}[[:space:]][[:space:]]release-manifest\.json$ ]] \
  || fail "release manifest digest sidecar has an invalid contract"
expected_manifest_sha="${manifest_digest_line:0:64}"
actual_manifest_sha="$(sha256sum "$MANIFEST" | awk '{print $1}')"
[[ "$actual_manifest_sha" == "$expected_manifest_sha" ]] \
  || fail "release manifest digest mismatch"

jq -e '
  .contract_version == "trnm_game_server_release_v1"
  and .source_dirty == false
  and (.release_id | test("^[0-9a-f]{12}-[0-9a-f]{12}$"))
  and (.git_commit | test("^[0-9a-f]{40}$"))
  and (.git_tree | test("^[0-9a-f]{40}$"))
  and .cargo_lock_path == "Cargo.lock"
  and (.cargo_lock_sha256 | test("^[0-9a-f]{64}$"))
  and .workspace_manifest_path == "workspace-Cargo.toml"
  and (.workspace_manifest_sha256 | test("^[0-9a-f]{64}$"))
  and .package_manifest_path == "trnm-game-server-Cargo.toml"
  and (.package_manifest_sha256 | test("^[0-9a-f]{64}$"))
  and .binary_path == "trnm-game-server"
  and (.binary_sha256 | test("^[0-9a-f]{64}$"))
  and (.source_date_epoch | type == "number")
  and (.rustc_version | type == "string" and length > 0)
  and (.cargo_version | type == "string" and length > 0)
  and (.target_triple | type == "string" and length > 0)
  and (.built_at_utc | type == "string" and length > 0)
' "$MANIFEST" >/dev/null || fail "manifest contract or source-clean gate is invalid"

expected_binary_sha="$(jq -r '.binary_sha256' "$MANIFEST")"
actual_binary_sha="$(sha256sum "$BINARY" | awk '{print $1}')"
[[ "$actual_binary_sha" == "$expected_binary_sha" ]] \
  || fail "binary digest mismatch: expected $expected_binary_sha, got $actual_binary_sha"

expected_lock_sha="$(jq -r '.cargo_lock_sha256' "$MANIFEST")"
actual_lock_sha="$(sha256sum "$LOCKFILE" | awk '{print $1}')"
[[ "$actual_lock_sha" == "$expected_lock_sha" ]] \
  || fail "bundled Cargo.lock digest does not match the manifest"

expected_workspace_manifest_sha="$(jq -r '.workspace_manifest_sha256' "$MANIFEST")"
actual_workspace_manifest_sha="$(sha256sum "$WORKSPACE_MANIFEST" | awk '{print $1}')"
[[ "$actual_workspace_manifest_sha" == "$expected_workspace_manifest_sha" ]] \
  || fail "bundled workspace Cargo.toml digest does not match the manifest"

expected_package_manifest_sha="$(jq -r '.package_manifest_sha256' "$MANIFEST")"
actual_package_manifest_sha="$(sha256sum "$PACKAGE_MANIFEST" | awk '{print $1}')"
[[ "$actual_package_manifest_sha" == "$expected_package_manifest_sha" ]] \
  || fail "bundled game-server Cargo.toml digest does not match the manifest"

git_commit="$(jq -r '.git_commit' "$MANIFEST")"
git_tree="$(jq -r '.git_tree' "$MANIFEST")"
git -C "$ROOT_DIR" cat-file -e "$git_commit^{commit}" 2>/dev/null \
  || fail "manifest Git commit is not available in the local source repository"
local_tree="$(git -C "$ROOT_DIR" rev-parse "$git_commit^{tree}")"
[[ "$local_tree" == "$git_tree" ]] \
  || fail "manifest tree does not match its Git commit"
expected_release_id="${git_commit:0:12}-${git_tree:0:12}"
[[ "$(jq -r '.release_id' "$MANIFEST")" == "$expected_release_id" ]] \
  || fail "release ID is not derived from its Git commit and tree"
if (( ALLOW_STAGING == 0 )); then
  [[ "$(basename "$RELEASE_DIR")" == "$expected_release_id" ]] \
    || fail "published release directory name does not match release ID"
fi

expected_source_date_epoch="$(git -C "$ROOT_DIR" show -s --format=%ct "$git_commit")"
[[ "$(jq -r '.source_date_epoch' "$MANIFEST")" == "$expected_source_date_epoch" ]] \
  || fail "source date epoch does not match the Git commit"

git_blob_sha() {
  git -C "$ROOT_DIR" show "$git_commit:$1" | sha256sum | awk '{print $1}'
}
[[ "$actual_lock_sha" == "$(git_blob_sha trillionnium/Cargo.lock)" ]] \
  || fail "bundled Cargo.lock is not from the declared Git commit"
[[ "$actual_workspace_manifest_sha" == "$(git_blob_sha trillionnium/Cargo.toml)" ]] \
  || fail "bundled workspace Cargo.toml is not from the declared Git commit"
[[ "$actual_package_manifest_sha" == \
  "$(git_blob_sha trillionnium/crates/trnm-game-server/Cargo.toml)" ]] \
  || fail "bundled game-server Cargo.toml is not from the declared Git commit"

jq -n \
  --arg contract_version "trnm_game_server_release_verification_v1" \
  --arg release_dir "$RELEASE_DIR" \
  --arg git_commit "$git_commit" \
  --arg git_tree "$git_tree" \
  --arg binary_sha256 "$actual_binary_sha" \
  '{
    contract_version: $contract_version,
    verified: true,
    release_dir: $release_dir,
    git_commit: $git_commit,
    git_tree: $git_tree,
    binary_sha256: $binary_sha256
  }'
