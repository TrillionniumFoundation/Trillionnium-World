#!/usr/bin/bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
BUILD_SCRIPT="$ROOT_DIR/scripts/build-trnm-game-server-release.sh"
CHECK_SCRIPT="$ROOT_DIR/scripts/check-trnm-game-server-release.sh"
TMP_DIR="$(mktemp -d)"

cleanup() {
  chmod -R u+w "$TMP_DIR" 2>/dev/null || true
  rm -rf -- "$TMP_DIR"
}
trap cleanup EXIT

fail() {
  printf 'TRNM game-server release contract test failed: %s\n' "$*" >&2
  exit 1
}

expect_failure() {
  local description="$1"
  shift
  if "$@" >"$TMP_DIR/unexpected-success.out" 2>&1; then
    fail "$description unexpectedly succeeded"
  fi
}

require_literal() {
  local file="$1" literal="$2" description="$3"
  grep -F -- "$literal" "$file" >/dev/null \
    || fail "$description is not enforced by $(basename "$file")"
}

reject_literal() {
  local file="$1" literal="$2" description="$3"
  if grep -F -- "$literal" "$file" >/dev/null; then
    fail "$description is present in $(basename "$file")"
  fi
}

# The formal builder deliberately has no fake-tool launcher. These assertions
# exercise the build recipe without compiling: the contract test constructs a
# sealed fixture below and tests the verifier against it.
[[ "$(head -n 1 "$BUILD_SCRIPT")" == '#!/usr/bin/bash' ]] \
  || fail "formal builder does not use an absolute Bash interpreter"
require_literal "$BUILD_SCRIPT" 'readonly TRUSTED_SYSTEM_PATH="/usr/bin:/bin"' \
  "fixed formal tool PATH"
require_literal "$BUILD_SCRIPT" '/usr/bin/env -i \' \
  "empty build subprocess environment"
require_literal "$BUILD_SCRIPT" 'RUST*|CARGO*|LD|LD_*|DYLD_*|BASH_ENV|ENV|' \
  "compiler/loader/shell injection rejection"
require_literal "$BUILD_SCRIPT" 'target_dir="$work_dir/target"' \
  "fresh per-build Cargo target"
require_literal "$BUILD_SCRIPT" 'CARGO_TARGET_DIR="$target_dir"' \
  "isolated Cargo target selection"
require_literal "$BUILD_SCRIPT" 'TRNM_GAME_SERVER_RELEASE_TARGET_DIR is prohibited' \
  "operator target-cache rejection"
require_literal "$BUILD_SCRIPT" 'repo_git archive --format=tar "$git_commit"' \
  "immutable captured source input"
require_literal "$BUILD_SCRIPT" '--bin trnm-game-server' \
  "game-server binary selection"
require_literal "$BUILD_SCRIPT" '--bin trnm-online-e2e' \
  "online-E2E binary selection"
require_literal "$BUILD_SCRIPT" '--locked' "Cargo lock enforcement"
require_literal "$BUILD_SCRIPT" '--frozen' "offline/frozen Cargo enforcement"
require_literal "$BUILD_SCRIPT" 'install -m 0555 "$target_dir/release/trnm-game-server"' \
  "game-server bundle installation"
require_literal "$BUILD_SCRIPT" 'install -m 0555 "$target_dir/release/trnm-online-e2e"' \
  "online-E2E bundle installation"
require_literal "$BUILD_SCRIPT" 'cargo_sha256="$(sha256sum "$cargo_bin"' \
  "Cargo executable digest binding"
require_literal "$BUILD_SCRIPT" 'rustc_sha256="$(sha256sum "$rustc_bin"' \
  "rustc executable digest binding"
require_literal "$BUILD_SCRIPT" 'toolchain_identity_sha256="$(' \
  "composite toolchain identity binding"
require_literal "$BUILD_SCRIPT" 'release_id="${git_commit:0:12}-${git_tree:0:12}-${toolchain_identity_sha256:0:12}"' \
  "toolchain-qualified release ID"
require_literal "$BUILD_SCRIPT" 'build_recipe_sha256: $build_recipe_sha256' \
  "source-bound build recipe manifest field"
require_literal "$BUILD_SCRIPT" 'fsync_regular_file "$durable_member"' \
  "durable release member publication"
require_literal "$BUILD_SCRIPT" 'fsync_directory "$RELEASE_ROOT"' \
  "durable release parent publication"
require_literal "$ROOT_DIR/scripts/promote-trnm-game-server-release.sh" \
  'rollback_selector' "atomic selector rollback"
reject_literal "$BUILD_SCRIPT" 'TRNM_TEST_' \
  "test-only formal-builder bypass"
reject_literal "$CHECK_SCRIPT" 'TRNM_TEST_' \
  "test-only formal-verifier bypass"

# This invocation exits in the inherited-environment gate, before source or
# toolchain discovery. It proves that the formal entry point rejects a common
# compiler-injection vector rather than silently sanitizing and continuing.
if /usr/bin/env -i PATH=/usr/bin:/bin \
  RUSTFLAGS='--cfg trnm_release_contract_injection' \
  "$BUILD_SCRIPT" >"$TMP_DIR/dangerous-env.out" 2>&1; then
  fail "formal builder accepted inherited RUSTFLAGS"
fi
grep -F 'prohibited inherited build environment variable is set: RUSTFLAGS' \
  "$TMP_DIR/dangerous-env.out" >/dev/null \
  || fail "formal builder did not fail specifically on inherited RUSTFLAGS"

repo="$TMP_DIR/repo"
release_root="$repo/run/releases/trnm-game-server"
mkdir -p \
  "$repo/scripts" \
  "$repo/trillionnium/crates/trnm-game-server" \
  "$release_root"
chmod 0755 "$release_root"
cp \
  "$BUILD_SCRIPT" \
  "$CHECK_SCRIPT" \
  "$ROOT_DIR/scripts/promote-trnm-game-server-release.sh" \
  "$repo/scripts/"
chmod 0755 "$repo/scripts/"*.sh

cat >"$repo/.gitignore" <<'EOF'
/run/
EOF
cat >"$repo/trillionnium/Cargo.toml" <<'EOF'
[workspace]
members = ["crates/trnm-game-server"]
resolver = "2"
EOF
cat >"$repo/trillionnium/Cargo.lock" <<'EOF'
# isolated release contract fixture
version = 3
EOF
cat >"$repo/trillionnium/crates/trnm-game-server/Cargo.toml" <<'EOF'
[package]
name = "trnm-game-server"
version = "0.0.0"
edition = "2021"
EOF

git -C "$repo" init -q
git -C "$repo" config user.email release-contract@example.invalid
git -C "$repo" config user.name release-contract
git -C "$repo" add .
git -C "$repo" commit -qm "release contract fixture"

git_commit="$(git -C "$repo" rev-parse HEAD)"
git_tree="$(git -C "$repo" rev-parse 'HEAD^{tree}')"
source_date_epoch="$(git -C "$repo" show -s --format=%ct HEAD)"
rustc_version='rustc 1.95.0 (release-contract-fixture)'
cargo_version='cargo 1.95.0 (release-contract-fixture)'
target_triple='x86_64-unknown-linux-gnu'
rustc_sha256="$(printf '%s\n' fixture-rustc-binary | sha256sum | awk '{print $1}')"
cargo_sha256="$(printf '%s\n' fixture-cargo-binary | sha256sum | awk '{print $1}')"
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
release_id="${git_commit:0:12}-${git_tree:0:12}-${toolchain_identity_sha256:0:12}"
release_dir="$release_root/$release_id"
mkdir -p "$release_dir"

cat >"$release_dir/trnm-game-server" <<'EOF'
#!/usr/bin/bash
set -euo pipefail
printf '%s\n' release
EOF
cat >"$release_dir/trnm-online-e2e" <<'EOF'
#!/usr/bin/bash
set -euo pipefail
printf '%s\n' mocked-online-e2e
EOF
cp "$repo/trillionnium/Cargo.lock" "$release_dir/Cargo.lock"
cp "$repo/trillionnium/Cargo.toml" "$release_dir/workspace-Cargo.toml"
cp "$repo/trillionnium/crates/trnm-game-server/Cargo.toml" \
  "$release_dir/trnm-game-server-Cargo.toml"
chmod 0555 "$release_dir/trnm-game-server" "$release_dir/trnm-online-e2e"
chmod 0444 \
  "$release_dir/Cargo.lock" \
  "$release_dir/workspace-Cargo.toml" \
  "$release_dir/trnm-game-server-Cargo.toml"

game_server_sha256="$(sha256sum "$release_dir/trnm-game-server" | awk '{print $1}')"
online_e2e_sha256="$(sha256sum "$release_dir/trnm-online-e2e" | awk '{print $1}')"
cargo_lock_sha256="$(sha256sum "$release_dir/Cargo.lock" | awk '{print $1}')"
workspace_manifest_sha256="$(sha256sum "$release_dir/workspace-Cargo.toml" | awk '{print $1}')"
package_manifest_sha256="$(sha256sum "$release_dir/trnm-game-server-Cargo.toml" | awk '{print $1}')"
build_recipe_sha256="$(git -C "$repo" show \
  "$git_commit:scripts/build-trnm-game-server-release.sh" | sha256sum | awk '{print $1}')"

jq -n \
  --arg contract_version trnm_game_server_release_v2 \
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
  --arg rustc_sha256 "$rustc_sha256" \
  --arg cargo_version "$cargo_version" \
  --arg cargo_sha256 "$cargo_sha256" \
  --arg target_triple "$target_triple" \
  --arg toolchain_identity_sha256 "$toolchain_identity_sha256" \
  --argjson source_date_epoch "$source_date_epoch" '
  {
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
      game_server: {path: "trnm-game-server", sha256: $game_server_sha256},
      online_e2e: {path: "trnm-online-e2e", sha256: $online_e2e_sha256}
    },
    rustc_version: $rustc_version,
    rustc_sha256: $rustc_sha256,
    cargo_version: $cargo_version,
    cargo_sha256: $cargo_sha256,
    target_triple: $target_triple,
    toolchain_identity_sha256: $toolchain_identity_sha256,
    built_at_utc: "2026-07-15T00:00:00Z"
  }
' >"$release_dir/release-manifest.json"
chmod 0444 "$release_dir/release-manifest.json"
release_manifest_sha256="$(sha256sum \
  "$release_dir/release-manifest.json" | awk '{print $1}')"
printf '%s  %s\n' "$release_manifest_sha256" release-manifest.json \
  >"$release_dir/release-manifest.sha256"
chmod 0444 "$release_dir/release-manifest.sha256"
chmod 0555 "$release_dir"

release_verification="$(
  "$repo/scripts/check-trnm-game-server-release.sh" "$release_dir"
)"
jq -e \
  --arg release_id "$release_id" \
  --arg toolchain_identity_sha256 "$toolchain_identity_sha256" '
  .verified == true and
  .release_contract_version == "trnm_game_server_release_v2" and
  .release_id == $release_id and
  .fault_harness_capable == true and
  .isolated_target == true and
  .trusted_target_cache_used == false and
  .build_toolchain.rustc_version == "rustc 1.95.0 (release-contract-fixture)" and
  (.build_toolchain.rustc_sha256 | test("^[0-9a-f]{64}$")) and
  .build_toolchain.cargo_version == "cargo 1.95.0 (release-contract-fixture)" and
  (.build_toolchain.cargo_sha256 | test("^[0-9a-f]{64}$")) and
  .build_toolchain.target_triple == "x86_64-unknown-linux-gnu" and
  .build_toolchain.identity_sha256 == $toolchain_identity_sha256 and
  .build_recipe.path == "scripts/build-trnm-game-server-release.sh" and
  (.build_recipe.sha256 | test("^[0-9a-f]{64}$")) and
  (.binaries.game_server.path | endswith("/trnm-game-server")) and
  (.binaries.online_e2e.path | endswith("/trnm-online-e2e"))
' >/dev/null <<<"$release_verification" \
  || fail "sealed fixture did not produce a verified formal v2 bundle"

# A stable channel can move while commit/tree remain fixed. The release ID
# must therefore change with the actual toolchain identity.
next_cargo_version='cargo 1.96.0 (next-stable-fixture)'
next_toolchain_identity_sha256="$(
  printf '%s\0%s\0%s\0%s\0%s\0' \
    "$next_cargo_version" \
    "$cargo_sha256" \
    "$rustc_version" \
    "$rustc_sha256" \
    "$target_triple" \
    | sha256sum \
    | awk '{print $1}'
)"
next_release_id="${git_commit:0:12}-${git_tree:0:12}-${next_toolchain_identity_sha256:0:12}"
[[ "$next_release_id" != "$release_id" ]] \
  || fail "different stable toolchains collide on one immutable release ID"

clone_release() {
  local name="$1" clone
  clone="$TMP_DIR/$name"
  cp -a "$release_dir" "$clone"
  chmod 0755 "$clone"
  printf '%s\n' "$clone"
}

seal_manifest() {
  local clone="$1" digest
  chmod 0644 "$clone/release-manifest.json" \
    "$clone/release-manifest.sha256"
  digest="$(sha256sum "$clone/release-manifest.json" | awk '{print $1}')"
  printf '%s  %s\n' "$digest" release-manifest.json \
    >"$clone/release-manifest.sha256"
  chmod 0444 "$clone/release-manifest.json" \
    "$clone/release-manifest.sha256"
}

missing_e2e="$(clone_release missing-e2e)"
rm -f "$missing_e2e/trnm-online-e2e"
expect_failure "v2 bundle missing online E2E" \
  "$repo/scripts/check-trnm-game-server-release.sh" --staging "$missing_e2e"

tampered_e2e="$(clone_release tampered-e2e)"
chmod 0755 "$tampered_e2e/trnm-online-e2e"
printf '%s\n' tampered >>"$tampered_e2e/trnm-online-e2e"
chmod 0555 "$tampered_e2e/trnm-online-e2e"
expect_failure "v2 bundle with a tampered online E2E" \
  "$repo/scripts/check-trnm-game-server-release.sh" --staging "$tampered_e2e"

wrong_e2e_mode="$(clone_release wrong-e2e-mode)"
chmod 0755 "$wrong_e2e_mode/trnm-online-e2e"
expect_failure "v2 bundle with a writable online E2E" \
  "$repo/scripts/check-trnm-game-server-release.sh" --staging "$wrong_e2e_mode"

symlink_e2e="$(clone_release symlink-e2e)"
rm -f "$symlink_e2e/trnm-online-e2e"
ln -s "$release_dir/trnm-online-e2e" "$symlink_e2e/trnm-online-e2e"
expect_failure "v2 bundle with a symbolic-link online E2E" \
  "$repo/scripts/check-trnm-game-server-release.sh" --staging "$symlink_e2e"

hardlink_e2e="$(clone_release hardlink-e2e)"
cp "$hardlink_e2e/trnm-online-e2e" "$TMP_DIR/hardlink-target"
rm -f "$hardlink_e2e/trnm-online-e2e"
ln "$TMP_DIR/hardlink-target" "$hardlink_e2e/trnm-online-e2e"
expect_failure "v2 bundle with an externally mutable hard-link online E2E" \
  "$repo/scripts/check-trnm-game-server-release.sh" --staging "$hardlink_e2e"

extra_member="$(clone_release extra-member)"
printf '%s\n' unexpected >"$extra_member/unexpected"
chmod 0444 "$extra_member/unexpected"
expect_failure "v2 bundle with an extra member" \
  "$repo/scripts/check-trnm-game-server-release.sh" --staging "$extra_member"

legacy_v1="$(clone_release legacy-v1)"
chmod 0644 "$legacy_v1/release-manifest.json"
jq '
  .contract_version = "trnm_game_server_release_v1"
  | .release_id = (.git_commit[0:12] + "-" + .git_tree[0:12])
  | .binary_path = .binaries.game_server.path
  | .binary_sha256 = .binaries.game_server.sha256
  | del(.binaries, .isolated_target, .trusted_target_cache_used,
      .build_recipe_path, .build_recipe_sha256, .rustc_sha256,
      .cargo_sha256, .toolchain_identity_sha256)
' "$legacy_v1/release-manifest.json" >"$TMP_DIR/legacy-v1-manifest.json"
/bin/mv "$TMP_DIR/legacy-v1-manifest.json" \
  "$legacy_v1/release-manifest.json"
rm -f "$legacy_v1/trnm-online-e2e"
seal_manifest "$legacy_v1"
expect_failure "legacy v1 bundle on a formal verifier" \
  "$repo/scripts/check-trnm-game-server-release.sh" --staging "$legacy_v1"

old_v2="$(clone_release old-v2)"
chmod 0644 "$old_v2/release-manifest.json"
jq '
  .release_id = (.git_commit[0:12] + "-" + .git_tree[0:12])
  | del(.rustc_sha256, .cargo_sha256, .toolchain_identity_sha256)
' "$old_v2/release-manifest.json" >"$TMP_DIR/old-v2-manifest.json"
/bin/mv "$TMP_DIR/old-v2-manifest.json" "$old_v2/release-manifest.json"
seal_manifest "$old_v2"
expect_failure "pre-toolchain-binding v2 bundle" \
  "$repo/scripts/check-trnm-game-server-release.sh" --staging "$old_v2"

wrong_toolchain_identity="$(clone_release wrong-toolchain-identity)"
chmod 0644 "$wrong_toolchain_identity/release-manifest.json"
jq --arg digest "$(printf '%064d' 0)" \
  '.toolchain_identity_sha256 = $digest' \
  "$wrong_toolchain_identity/release-manifest.json" \
  >"$TMP_DIR/wrong-toolchain-manifest.json"
/bin/mv "$TMP_DIR/wrong-toolchain-manifest.json" \
  "$wrong_toolchain_identity/release-manifest.json"
seal_manifest "$wrong_toolchain_identity"
expect_failure "self-sealed manifest with an unbound toolchain identity" \
  "$repo/scripts/check-trnm-game-server-release.sh" --staging \
  "$wrong_toolchain_identity"

wrong_recipe="$(clone_release wrong-recipe)"
chmod 0644 "$wrong_recipe/release-manifest.json"
jq --arg digest "$(printf '%064d' 1)" \
  '.build_recipe_sha256 = $digest' \
  "$wrong_recipe/release-manifest.json" >"$TMP_DIR/wrong-recipe-manifest.json"
/bin/mv "$TMP_DIR/wrong-recipe-manifest.json" \
  "$wrong_recipe/release-manifest.json"
seal_manifest "$wrong_recipe"
expect_failure "self-sealed manifest with a non-source build recipe" \
  "$repo/scripts/check-trnm-game-server-release.sh" --staging "$wrong_recipe"

forged_lock="$(clone_release forged-lock)"
chmod 0644 "$forged_lock/Cargo.lock" "$forged_lock/release-manifest.json"
printf '%s\n' '# forged lock' >>"$forged_lock/Cargo.lock"
forged_lock_sha256="$(sha256sum "$forged_lock/Cargo.lock" | awk '{print $1}')"
jq --arg digest "$forged_lock_sha256" \
  '.cargo_lock_sha256 = $digest' \
  "$forged_lock/release-manifest.json" >"$TMP_DIR/forged-lock-manifest.json"
/bin/mv "$TMP_DIR/forged-lock-manifest.json" \
  "$forged_lock/release-manifest.json"
chmod 0444 "$forged_lock/Cargo.lock"
seal_manifest "$forged_lock"
expect_failure "self-consistent but source-forged Cargo.lock" \
  "$repo/scripts/check-trnm-game-server-release.sh" --staging "$forged_lock"

"$repo/scripts/promote-trnm-game-server-release.sh" "$release_dir" >/dev/null
[[ "$(realpath -e "$release_root/current")" == "$(realpath -e "$release_dir")" ]] \
  || fail "promotion did not atomically select the verified formal release"

# A post-switch verification failure must put back the exact prior selector,
# even when that prior selector was stale.  The copied verifier is wrapped only
# inside this isolated fixture so the formal promoter itself has no test hook.
ln -sfn stale-prior-selector "$release_root/current"
mv "$repo/scripts/check-trnm-game-server-release.sh" \
  "$repo/scripts/check-trnm-game-server-release.real.sh"
cat >"$repo/scripts/check-trnm-game-server-release.sh" <<'EOF'
#!/usr/bin/bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
if [[ "${1:-}" == "$root/run/releases/trnm-game-server/current" ]]; then
  exit 91
fi
exec "$root/scripts/check-trnm-game-server-release.real.sh" "$@"
EOF
chmod 0755 "$repo/scripts/check-trnm-game-server-release.sh"
expect_failure "promotion with a failing post-switch verifier" \
  "$repo/scripts/promote-trnm-game-server-release.sh" "$release_dir"
[[ "$(readlink "$release_root/current")" == stale-prior-selector ]] \
  || fail "failed promotion did not roll back the prior selector"
mv "$repo/scripts/check-trnm-game-server-release.real.sh" \
  "$repo/scripts/check-trnm-game-server-release.sh"
ln -sfn "$release_id" "$release_root/current"

outside_root="$TMP_DIR/outside"
mkdir -p "$outside_root"
cp -a "$release_dir" "$outside_root/$release_id"
expect_failure "promotion of a release outside the configured root" \
  env TRNM_GAME_SERVER_RELEASE_ROOT="$release_root" \
  "$repo/scripts/promote-trnm-game-server-release.sh" \
  "$outside_root/$release_id"

printf '%s\n' 'TRNM game-server immutable release contract: PASS'
