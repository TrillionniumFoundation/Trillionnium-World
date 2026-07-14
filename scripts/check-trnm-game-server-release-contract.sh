#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_DIR="$(mktemp -d)"
cleanup() {
  chmod -R u+w "$TMP_DIR" 2>/dev/null || true
  rm -rf -- "$TMP_DIR"
}
trap cleanup EXIT

fail() {
  echo "TRNM game-server release contract test failed: $*" >&2
  exit 1
}

expect_failure() {
  local description="$1"
  shift
  if "$@" >"$TMP_DIR/unexpected-success.out" 2>&1; then
    fail "$description unexpectedly succeeded"
  fi
}

repo="$TMP_DIR/repo"
release_root="$repo/run/releases/trnm-game-server"
fake_bin="$TMP_DIR/fake-bin"
fake_cex="$TMP_DIR/fake-cex"
mkdir -p \
  "$repo/scripts" \
  "$repo/trillionnium/crates/trnm-game-server" \
  "$fake_bin" \
  "$fake_cex/scripts"
cp \
  "$ROOT_DIR/scripts/build-trnm-game-server-release.sh" \
  "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" \
  "$ROOT_DIR/scripts/promote-trnm-game-server-release.sh" \
  "$ROOT_DIR/scripts/run-trnm-game-server.sh" \
  "$repo/scripts/"
chmod 0755 "$repo/scripts/"*.sh

cat >"$repo/.gitignore" <<'EOF'
/run/
/target/
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

cat >"$fake_bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--version" ]]; then
  echo "cargo 1.90.0 (release-contract-fixture)"
  exit 0
fi
[[ "${1:-}" == "build" ]] || exit 64
mkdir -p "$CARGO_TARGET_DIR/release"
cat >"$CARGO_TARGET_DIR/release/trnm-game-server" <<'BIN'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' release >"${TRNM_TEST_MARKER:?}"
BIN
chmod 0755 "$CARGO_TARGET_DIR/release/trnm-game-server"
EOF
cat >"$fake_bin/rustc" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "-vV" ]]; then
  printf '%s\n' \
    "rustc 1.90.0 (release-contract-fixture)" \
    "host: x86_64-unknown-linux-gnu"
else
  echo "rustc 1.90.0 (release-contract-fixture)"
fi
EOF
cat >"$fake_bin/curl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
url="${!#}"
if [[ "$url" == *"/v1/signer/readiness" ]]; then
  printf '%s\n' '{"status":"ok","contract_version":"trnm_entitlement_signer_v1","private_key_exported_to_game_server":false,"postgres_receipts":true}'
else
  printf '%s\n' '{"status":"ok","postgres_healthy":true,"fail_fast":true}'
fi
EOF
chmod 0755 "$fake_bin/"*

cat >"$fake_cex/scripts/_dev-helpers.sh" <<'EOF'
cex_load_env() {
  export IDENTITY_ADMIN_TOKEN="release-contract-fixture"
}
cex_effective_database_url() {
  printf '%s\n' "postgresql://release-contract-fixture"
}
EOF

git -C "$repo" init -q
git -C "$repo" config user.email release-contract@example.invalid
git -C "$repo" config user.name release-contract
git -C "$repo" add .
git -C "$repo" commit -qm "release contract fixture"

release_dir="$(
  PATH="$fake_bin:$PATH" \
  TRNM_GAME_SERVER_RELEASE_ROOT="$release_root" \
  "$repo/scripts/build-trnm-game-server-release.sh"
)"
[[ -d "$release_dir" ]] || fail "mocked build did not publish a release directory"
[[ ! -e "$repo/target" ]] \
  || fail "release build reused the mutable development target directory"
"$repo/scripts/check-trnm-game-server-release.sh" "$release_dir" >/dev/null
[[ -f "$release_dir/release-manifest.sha256" ]] \
  || fail "release manifest digest sidecar was not bundled"

"$repo/scripts/promote-trnm-game-server-release.sh" "$release_dir" >/dev/null
[[ "$(realpath -e "$release_root/current")" == "$(realpath -e "$release_dir")" ]] \
  || fail "promotion did not atomically select the verified release"

outside_root="$TMP_DIR/outside"
mkdir -p "$outside_root"
cp -a "$release_dir" "$outside_root/$(basename "$release_dir")"
expect_failure \
  "promotion of a release outside the configured root" \
  env TRNM_GAME_SERVER_RELEASE_ROOT="$release_root" \
  "$repo/scripts/promote-trnm-game-server-release.sh" \
  "$outside_root/$(basename "$release_dir")"

marker="$TMP_DIR/selection.marker"
PATH="$fake_bin:$PATH" \
CEX_PROJECT_ROOT="$fake_cex" \
TRNM_TEST_MARKER="$marker" \
"$repo/scripts/run-trnm-game-server.sh"
[[ "$(<"$marker")" == "release" ]] || fail "promoted release was not selected"

rm -f "$release_root/current" "$marker"
mkdir -p "$repo/target/release"
cat >"$repo/target/release/trnm-game-server" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' legacy >"${TRNM_TEST_MARKER:?}"
EOF
chmod 0755 "$repo/target/release/trnm-game-server"
PATH="$fake_bin:$PATH" \
CEX_PROJECT_ROOT="$fake_cex" \
TRNM_TEST_MARKER="$marker" \
"$repo/scripts/run-trnm-game-server.sh"
[[ "$(<"$marker")" == "legacy" ]] \
  || fail "absent default selector did not preserve the legacy development fallback"

rm -f "$marker"
ln -s missing-release "$release_root/current"
expect_failure \
  "dangling current selector" \
  env PATH="$fake_bin:$PATH" CEX_PROJECT_ROOT="$fake_cex" TRNM_TEST_MARKER="$marker" \
  "$repo/scripts/run-trnm-game-server.sh"
[[ ! -e "$marker" ]] || fail "dangling selector executed the legacy binary"
rm -f "$release_root/current"

expect_failure \
  "explicitly missing release selector" \
  env PATH="$fake_bin:$PATH" CEX_PROJECT_ROOT="$fake_cex" TRNM_TEST_MARKER="$marker" \
  TRNM_GAME_SERVER_RELEASE_DIR="$release_root/does-not-exist" \
  "$repo/scripts/run-trnm-game-server.sh"
[[ ! -e "$marker" ]] || fail "explicitly missing selector executed the legacy binary"

printf '%s\n' dirty >>"$repo/trillionnium/Cargo.lock"
expect_failure \
  "dirty-worktree release build" \
  env PATH="$fake_bin:$PATH" TRNM_GAME_SERVER_RELEASE_ROOT="$release_root" \
  "$repo/scripts/build-trnm-game-server-release.sh"

chmod 0755 "$release_dir"
chmod 0644 "$release_dir/Cargo.lock" "$release_dir/release-manifest.json" \
  "$release_dir/release-manifest.sha256"
printf '%s\n' '# forged lock' >>"$release_dir/Cargo.lock"
forged_lock_sha="$(sha256sum "$release_dir/Cargo.lock" | awk '{print $1}')"
jq --arg digest "$forged_lock_sha" \
  '.cargo_lock_sha256 = $digest' \
  "$release_dir/release-manifest.json" >"$TMP_DIR/forged-manifest.json"
mv "$TMP_DIR/forged-manifest.json" "$release_dir/release-manifest.json"
forged_manifest_sha="$(sha256sum "$release_dir/release-manifest.json" | awk '{print $1}')"
printf '%s  %s\n' "$forged_manifest_sha" release-manifest.json \
  >"$release_dir/release-manifest.sha256"
chmod 0444 "$release_dir/Cargo.lock" "$release_dir/release-manifest.json" \
  "$release_dir/release-manifest.sha256"
chmod 0555 "$release_dir"
expect_failure \
  "self-consistent but source-forged Cargo.lock" \
  "$repo/scripts/check-trnm-game-server-release.sh" "$release_dir"

echo "TRNM game-server immutable release contract: PASS"
