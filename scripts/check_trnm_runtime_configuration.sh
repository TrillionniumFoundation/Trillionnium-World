#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

fail() {
  printf 'runtime-configuration: %s\n' "$*" >&2
  exit 1
}

files=(
  scripts/run-trnm-game-server.sh
  scripts/run-trnm-entitlement-signer.sh
  scripts/install-trnm-game-server-systemd.sh
  deploy/systemd/trnm-game-server.service
  deploy/systemd/trnm-entitlement-signer.service
  config/trnm-game-server.env.example
  config/trnm-entitlement-signer.env.example
)
for file in "${files[@]}"; do
  [[ -s "$file" ]] || fail "missing runtime configuration file: $file"
done

for script in \
  scripts/run-trnm-game-server.sh \
  scripts/run-trnm-entitlement-signer.sh \
  scripts/install-trnm-game-server-systemd.sh; do
  bash -n "$script" || fail "bash syntax failed: $script"
done

forbidden="$(
  grep -nE \
    '/home/[A-Za-z0-9_.-]+|\.openclaw/workspace|\.\./CEX|CEX_PROJECT_ROOT|IDENTITY_ADMIN_TOKEN' \
    "${files[@]}" 2>/dev/null || true
)"
[[ -z "$forbidden" ]] || {
  printf '%s\n' "$forbidden" >&2
  fail "personal path, sibling repository, or shared administrator-token fallback detected"
}

for unit in deploy/systemd/trnm-game-server.service deploy/systemd/trnm-entitlement-signer.service; do
  grep -Fq '@TRNM_WORLD_ROOT@' "$unit" || fail "$unit is not an install-time template"
  grep -Fq '@TRNM_CONFIG_HOME@' "$unit" || fail "$unit has no rendered config boundary"
  grep -Fq '@TRNM_STATE_HOME@' "$unit" || fail "$unit has no rendered state boundary"
  grep -Fq 'UMask=0077' "$unit" || fail "$unit does not enforce a private umask"
  grep -Fq 'NoNewPrivileges=true' "$unit" || fail "$unit does not set NoNewPrivileges"
done

grep -Fq 'require_distinct_secret' scripts/run-trnm-game-server.sh \
  || fail "game-server runner does not validate role-secret separation"
grep -Fq 'TRNM_ALLOW_DEV_BINARY' scripts/run-trnm-game-server.sh \
  || fail "game-server runner has no explicit development fallback gate"
grep -Fq 'TRNM_ALLOW_DEV_BINARY' scripts/run-trnm-entitlement-signer.sh \
  || fail "signer runner has no explicit development fallback gate"
grep -Fq 'check-trnm-game-server-release.sh' scripts/run-trnm-game-server.sh \
  || fail "game-server runner does not verify its selected release"
grep -Fq 'check-trnm-game-server-release.sh' scripts/run-trnm-entitlement-signer.sh \
  || fail "signer runner does not verify its selected release"

for variable in \
  TRNM_GAME_AUTHORITY_TOKEN \
  TRNM_MODERATOR_TOKEN \
  TRNM_ENTITLEMENT_SIGNER_TOKEN; do
  grep -Fq "$variable=" config/trnm-game-server.env.example \
    || fail "game-server environment example is missing $variable"
done

grep -Fq 'TRNM_ENTITLEMENT_ED25519_PRIVATE_KEY_FILE=' \
  config/trnm-entitlement-signer.env.example \
  || fail "signer environment example is missing its private-key file"

if grep -Eq 'TRNM_ALLOW_DEV_BINARY[[:space:]]*=[[:space:]]*1' \
  config/*.env.example; then
  fail "production environment examples must not enable development binary fallback"
fi

grep -Fq 'install -m 0600' scripts/install-trnm-game-server-systemd.sh \
  || fail "installer does not create private environment files"
grep -Fq -- '--start' scripts/install-trnm-game-server-systemd.sh \
  || fail "installer does not separate installation from explicit service start"

printf '%s\n' \
  'TRNM runtime configuration: green (portable paths, explicit config, distinct secrets, verified-release production policy)'
