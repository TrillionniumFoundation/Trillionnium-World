#!/usr/bin/bash
set -euo pipefail
builtin umask 077

# Do not let exported functions or operator PATH/configuration choose the
# verifier's shell tools or Git view. The only supported environment input is
# the optional release selector, captured before the environment is cleared.
while IFS= read -r inherited_function_name; do
  builtin unset -f "$inherited_function_name"
done < <(builtin compgen -A function)
unset inherited_function_name
release_dir_override="${TRNM_GAME_SERVER_RELEASE_DIR:-}"
inherited_environment_names=()
while IFS= read -r environment_name; do
  inherited_environment_names+=("$environment_name")
done < <(builtin compgen -e)
for environment_name in "${inherited_environment_names[@]}"; do
  builtin unset "$environment_name" 2>/dev/null \
    || builtin export -n "$environment_name" 2>/dev/null \
    || {
      printf 'TRNM game-server release verification failed: could not clear inherited environment variable: %s\n' \
        "$environment_name" >&2
      exit 1
    }
done
unset inherited_environment_names environment_name

export PATH="/usr/bin:/bin"
export LC_ALL=C
export LANG=C
export TZ=UTC
export GIT_CONFIG_NOSYSTEM=1
export GIT_CONFIG_GLOBAL=/dev/null
export GIT_PAGER=cat
hash -r

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
ALLOW_STAGING=0
if [[ "${1:-}" == "--staging" ]]; then
  ALLOW_STAGING=1
  shift
fi
(( $# <= 1 )) || {
  echo "usage: check-trnm-game-server-release.sh [--staging] [RELEASE_DIR]" >&2
  exit 64
}
REQUESTED_RELEASE_DIR="${1:-${release_dir_override:-$ROOT_DIR/run/releases/trnm-game-server/current}}"
unset release_dir_override

fail() {
  echo "TRNM game-server release verification failed: $*" >&2
  exit 1
}

RELEASE_DIR="$(realpath -e -- "$REQUESTED_RELEASE_DIR" 2>/dev/null)" \
  || fail "release directory does not exist or has a dangling link: $REQUESTED_RELEASE_DIR"
[[ -d "$RELEASE_DIR" ]] || fail "release path is not a directory: $REQUESTED_RELEASE_DIR"

MANIFEST="$RELEASE_DIR/release-manifest.json"
MANIFEST_DIGEST="$RELEASE_DIR/release-manifest.sha256"
GAME_SERVER_BINARY="$RELEASE_DIR/trnm-game-server"
ONLINE_E2E_BINARY="$RELEASE_DIR/trnm-online-e2e"
LOCKFILE="$RELEASE_DIR/Cargo.lock"
WORKSPACE_MANIFEST="$RELEASE_DIR/workspace-Cargo.toml"
PACKAGE_MANIFEST="$RELEASE_DIR/trnm-game-server-Cargo.toml"

require_regular_member() {
  local release_file="$1"
  [[ -f "$release_file" && ! -L "$release_file" ]] \
    || fail "bundle member must be a regular, non-symbolic-link file: $release_file"
  [[ "$(stat -c '%h' "$release_file")" == "1" ]] \
    || fail "bundle members must not be externally mutable hard links: $release_file"
}

require_regular_member "$MANIFEST"
require_regular_member "$MANIFEST_DIGEST"

manifest_digest_line="$(<"$MANIFEST_DIGEST")"
[[ "$manifest_digest_line" =~ ^[0-9a-f]{64}[[:space:]][[:space:]]release-manifest\.json$ ]] \
  || fail "release manifest digest sidecar has an invalid contract"
expected_manifest_sha="${manifest_digest_line:0:64}"
actual_manifest_sha="$(sha256sum "$MANIFEST" | awk '{print $1}')"
[[ "$actual_manifest_sha" == "$expected_manifest_sha" ]] \
  || fail "release manifest digest mismatch"

release_contract_version="$(jq -er '.contract_version | strings' "$MANIFEST" 2>/dev/null)" \
  || fail "release manifest does not declare a valid contract version"
[[ "$release_contract_version" == "trnm_game_server_release_v2" ]] \
  || fail "formal release consumers require trnm_game_server_release_v2; refusing legacy contract: $release_contract_version"
fault_harness_capable=true
expected_entries="$(printf '%s\n' \
  Cargo.lock \
  release-manifest.json \
  release-manifest.sha256 \
  trnm-game-server \
  trnm-game-server-Cargo.toml \
  trnm-online-e2e \
  workspace-Cargo.toml | LC_ALL=C sort)"

release_files=(
  "$MANIFEST"
  "$MANIFEST_DIGEST"
  "$GAME_SERVER_BINARY"
  "$LOCKFILE"
  "$WORKSPACE_MANIFEST"
  "$PACKAGE_MANIFEST"
)
release_files+=("$ONLINE_E2E_BINARY")
for release_file in "${release_files[@]}"; do
  require_regular_member "$release_file"
done

actual_entries="$(find "$RELEASE_DIR" -mindepth 1 -maxdepth 1 -printf '%f\n' | LC_ALL=C sort)"
[[ "$actual_entries" == "$expected_entries" ]] \
  || fail "release directory members do not match $release_contract_version (found: ${actual_entries//$'\n'/, })"

[[ -x "$GAME_SERVER_BINARY" && "$(stat -c '%a' "$GAME_SERVER_BINARY")" == "555" ]] \
  || fail "game-server binary mode must be 0555"
[[ -x "$ONLINE_E2E_BINARY" && "$(stat -c '%a' "$ONLINE_E2E_BINARY")" == "555" ]] \
  || fail "online E2E binary mode must be 0555"
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

initial_release_dir_stat="$(stat -c '%d:%i:%h:%a' "$RELEASE_DIR")"
declare -A initial_member_stat initial_member_sha
for release_file in "${release_files[@]}"; do
  initial_member_stat["$release_file"]="$(stat -c '%d:%i:%h:%a:%s' "$release_file")"
  initial_member_sha["$release_file"]="$(sha256sum "$release_file" | awk '{print $1}')"
done
[[ "${initial_member_sha[$MANIFEST]}" == "$actual_manifest_sha" && \
    "$(<"$MANIFEST_DIGEST")" == "$manifest_digest_line" && \
    "$(jq -er '.contract_version | strings' "$MANIFEST" 2>/dev/null)" == \
      "$release_contract_version" ]] \
  || fail "release manifest or digest changed during initial verification"

jq -e '
  def sha256: type == "string" and test("^[0-9a-f]{64}$");
  def source_contract:
    .source_dirty == false
    and (.release_id | test("^[0-9a-f]{12}-[0-9a-f]{12}-[0-9a-f]{12}$"))
    and (.git_commit | test("^[0-9a-f]{40}$"))
    and (.git_tree | test("^[0-9a-f]{40}$"))
    and .cargo_lock_path == "Cargo.lock"
    and (.cargo_lock_sha256 | sha256)
    and .workspace_manifest_path == "workspace-Cargo.toml"
    and (.workspace_manifest_sha256 | sha256)
    and .package_manifest_path == "trnm-game-server-Cargo.toml"
    and (.package_manifest_sha256 | sha256)
    and (.source_date_epoch | type == "number")
    and (.built_at_utc | type == "string" and length > 0);
  source_contract
    and .contract_version == "trnm_game_server_release_v2"
    and
    (keys | sort) == [
      "binaries", "build_recipe_path", "build_recipe_sha256", "built_at_utc",
      "cargo_lock_path", "cargo_lock_sha256", "cargo_sha256", "cargo_version",
      "contract_version",
      "git_commit", "git_tree",
      "isolated_target", "package_manifest_path", "package_manifest_sha256",
      "release_id", "rustc_sha256", "rustc_version", "source_date_epoch",
      "source_dirty", "target_triple", "toolchain_identity_sha256",
      "trusted_target_cache_used", "workspace_manifest_path", "workspace_manifest_sha256"
    ]
    and .isolated_target == true
    and .trusted_target_cache_used == false
    and .build_recipe_path == "scripts/build-trnm-game-server-release.sh"
    and (.build_recipe_sha256 | sha256)
    and (.rustc_version | type == "string" and length > 0)
    and (.rustc_sha256 | sha256)
    and (.cargo_version | type == "string" and length > 0)
    and (.cargo_sha256 | sha256)
    and (.target_triple | type == "string" and length > 0)
    and (.toolchain_identity_sha256 | sha256)
    and (.built_at_utc | test("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$"))
    and (.binaries | type == "object" and (keys | sort) == ["game_server", "online_e2e"])
    and (.binaries.game_server | type == "object" and (keys | sort) == ["path", "sha256"])
    and .binaries.game_server.path == "trnm-game-server"
    and (.binaries.game_server.sha256 | sha256)
    and (.binaries.online_e2e | type == "object" and (keys | sort) == ["path", "sha256"])
    and .binaries.online_e2e.path == "trnm-online-e2e"
    and (.binaries.online_e2e.sha256 | sha256)
' "$MANIFEST" >/dev/null || fail "manifest contract or source-clean gate is invalid"

expected_game_server_sha="$(jq -r '.binaries.game_server.sha256' "$MANIFEST")"
expected_online_e2e_sha="$(jq -r '.binaries.online_e2e.sha256' "$MANIFEST")"
manifest_rustc_version="$(jq -r '.rustc_version' "$MANIFEST")"
manifest_cargo_version="$(jq -r '.cargo_version' "$MANIFEST")"
manifest_target_triple="$(jq -r '.target_triple' "$MANIFEST")"
manifest_rustc_sha256="$(jq -r '.rustc_sha256' "$MANIFEST")"
manifest_cargo_sha256="$(jq -r '.cargo_sha256' "$MANIFEST")"
manifest_toolchain_identity_sha256="$(jq -r '.toolchain_identity_sha256' "$MANIFEST")"
manifest_build_recipe_path="$(jq -r '.build_recipe_path' "$MANIFEST")"
manifest_build_recipe_sha256="$(jq -r '.build_recipe_sha256' "$MANIFEST")"
actual_game_server_sha="$(sha256sum "$GAME_SERVER_BINARY" | awk '{print $1}')"
[[ "$actual_game_server_sha" == "$expected_game_server_sha" ]] \
  || fail "game-server digest mismatch: expected $expected_game_server_sha, got $actual_game_server_sha"
actual_online_e2e_sha=""
actual_online_e2e_sha="$(sha256sum "$ONLINE_E2E_BINARY" | awk '{print $1}')"
[[ "$actual_online_e2e_sha" == "$expected_online_e2e_sha" ]] \
  || fail "online E2E digest mismatch: expected $expected_online_e2e_sha, got $actual_online_e2e_sha"

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
calculated_toolchain_identity_sha256="$(
  printf '%s\0%s\0%s\0%s\0%s\0' \
    "$manifest_cargo_version" \
    "$manifest_cargo_sha256" \
    "$manifest_rustc_version" \
    "$manifest_rustc_sha256" \
    "$manifest_target_triple" \
    | sha256sum \
    | awk '{print $1}'
)"
[[ "$calculated_toolchain_identity_sha256" == "$manifest_toolchain_identity_sha256" ]] \
  || fail "toolchain identity digest does not bind the declared Cargo/rustc binaries and versions"
expected_release_id="${git_commit:0:12}-${git_tree:0:12}-${manifest_toolchain_identity_sha256:0:12}"
[[ "$(jq -r '.release_id' "$MANIFEST")" == "$expected_release_id" ]] \
  || fail "release ID is not derived from its Git commit, tree, and toolchain identity"
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
[[ "$(jq -r '.build_recipe_sha256' "$MANIFEST")" == \
    "$(git_blob_sha scripts/build-trnm-game-server-release.sh)" ]] \
  || fail "v2 build recipe digest is not from the declared Git commit"

final_entries="$(find "$RELEASE_DIR" -mindepth 1 -maxdepth 1 -printf '%f\n' | LC_ALL=C sort)"
[[ "$final_entries" == "$expected_entries" ]] \
  || fail "release directory members changed during verification"
[[ "$(stat -c '%d:%i:%h:%a' "$RELEASE_DIR")" == "$initial_release_dir_stat" ]] \
  || fail "release directory identity or mode changed during verification"
for release_file in "${release_files[@]}"; do
  require_regular_member "$release_file"
  [[ "$(stat -c '%d:%i:%h:%a:%s' "$release_file")" == \
      "${initial_member_stat[$release_file]}" ]] \
    || fail "bundle member identity, links, mode, or size changed during verification: $release_file"
  [[ "$(sha256sum "$release_file" | awk '{print $1}')" == \
      "${initial_member_sha[$release_file]}" ]] \
    || fail "bundle member contents changed during verification: $release_file"
done

jq -n \
  --arg contract_version "trnm_game_server_release_verification_v1" \
  --arg release_contract_version "$release_contract_version" \
  --arg release_id "$expected_release_id" \
  --arg release_dir "$RELEASE_DIR" \
  --arg git_commit "$git_commit" \
  --arg git_tree "$git_tree" \
  --arg game_server_path "$GAME_SERVER_BINARY" \
  --arg game_server_sha256 "$actual_game_server_sha" \
  --arg online_e2e_path "$ONLINE_E2E_BINARY" \
  --arg online_e2e_sha256 "$actual_online_e2e_sha" \
  --arg release_manifest_sha256 "$actual_manifest_sha" \
  --arg rustc_version "$manifest_rustc_version" \
  --arg rustc_sha256 "$manifest_rustc_sha256" \
  --arg cargo_version "$manifest_cargo_version" \
  --arg cargo_sha256 "$manifest_cargo_sha256" \
  --arg target_triple "$manifest_target_triple" \
  --arg toolchain_identity_sha256 "$manifest_toolchain_identity_sha256" \
  --arg build_recipe_path "$manifest_build_recipe_path" \
  --arg build_recipe_sha256 "$manifest_build_recipe_sha256" \
  --argjson fault_harness_capable "$fault_harness_capable" \
  --argjson isolated_target "$(jq -r '.isolated_target' "$MANIFEST")" \
  --argjson trusted_target_cache_used "$(jq -r '.trusted_target_cache_used' "$MANIFEST")" \
  '{
    contract_version: $contract_version,
    release_contract_version: $release_contract_version,
    release_id: $release_id,
    verified: true,
    fault_harness_capable: $fault_harness_capable,
    isolated_target: $isolated_target,
    trusted_target_cache_used: $trusted_target_cache_used,
    release_dir: $release_dir,
    release_manifest_sha256: $release_manifest_sha256,
    git_commit: $git_commit,
    git_tree: $git_tree,
    binary_sha256: $game_server_sha256,
    build_toolchain: {
      rustc_version: $rustc_version,
      rustc_sha256: $rustc_sha256,
      cargo_version: $cargo_version,
      cargo_sha256: $cargo_sha256,
      target_triple: $target_triple,
      identity_sha256: $toolchain_identity_sha256
    },
    build_recipe: (
      if $fault_harness_capable then
        {path: $build_recipe_path, sha256: $build_recipe_sha256}
      else
        null
      end
    ),
    binaries: {
      game_server: {
        path: $game_server_path,
        sha256: $game_server_sha256
      },
      online_e2e: (
        if $fault_harness_capable then
          {path: $online_e2e_path, sha256: $online_e2e_sha256}
        else
          null
        end
      )
    }
  }'
