#!/usr/bin/bash
set -euo pipefail
builtin umask 077

while IFS= read -r inherited_function_name; do
  builtin unset -f "$inherited_function_name"
done < <(builtin compgen -A function)
unset inherited_function_name

readonly TRUSTED_SYSTEM_PATH="/usr/bin:/bin"

fail() {
  printf 'TRNM game-server release promotion failed: %s\n' "$*" >&2
  exit 1
}

# The checker and selector mutation inherit only a fixed, minimal environment.
release_root_override="${TRNM_GAME_SERVER_RELEASE_ROOT:-}"
inherited_environment_names=()
while IFS= read -r environment_name; do
  inherited_environment_names+=("$environment_name")
  case "$environment_name" in
    LD|LD_*|DYLD_*|BASH_ENV|ENV|BASHOPTS|SHELLOPTS|PYTHON*)
      fail "prohibited inherited promotion environment variable is set: $environment_name"
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
unset release_root_override
RELEASE_DIR="${1:?usage: promote-trnm-game-server-release.sh RELEASE_DIR}"
RUN_ROOT="$ROOT_DIR/run"
LOCK_ROOT="$ROOT_DIR/run/locks"
DEPLOY_LOCK="$LOCK_ROOT/trnm-game-server-deploy.lock"

current_uid="$(id -u)"
if [[ -e "$RUN_ROOT" || -L "$RUN_ROOT" ]]; then
  [[ -d "$RUN_ROOT" && ! -L "$RUN_ROOT" \
      && "$(stat -c '%u' -- "$RUN_ROOT")" == "$current_uid" ]] \
    || fail "runtime root is not a real owner-held directory: $RUN_ROOT"
else
  mkdir -m 0700 -- "$RUN_ROOT" \
    || fail "could not create runtime root: $RUN_ROOT"
fi
if [[ -e "$LOCK_ROOT" || -L "$LOCK_ROOT" ]]; then
  [[ -d "$LOCK_ROOT" && ! -L "$LOCK_ROOT" ]] \
    || fail "deployment lock root is not a real directory: $LOCK_ROOT"
else
  mkdir -m 0700 -- "$LOCK_ROOT" \
    || fail "could not create deployment lock root: $LOCK_ROOT"
fi

validate_lock_root() {
  local owner mode links
  [[ -d "$LOCK_ROOT" && ! -L "$LOCK_ROOT" ]] \
    || fail "deployment lock root is not a real directory: $LOCK_ROOT"
  read -r owner mode links < <(stat -c '%u %a %h' -- "$LOCK_ROOT")
  [[ "$owner" == "$current_uid" && "$mode" == 700 \
      && "$links" =~ ^[0-9]+$ ]] && (( links >= 2 )) \
    || fail "deployment lock root must be owner-only, owner-held, and have a valid link count: $LOCK_ROOT"
}

validate_lock_file_path() {
  local owner mode links
  [[ -f "$DEPLOY_LOCK" && ! -L "$DEPLOY_LOCK" ]] \
    || fail "deployment lock is not a regular non-symlink file: $DEPLOY_LOCK"
  read -r owner mode links < <(stat -c '%u %a %h' -- "$DEPLOY_LOCK")
  [[ "$owner" == "$current_uid" && "$mode" == 600 && "$links" == 1 ]] \
    || fail "deployment lock must be owner-only, owner-held, and singly linked: $DEPLOY_LOCK"
}

validate_lock_root
if [[ -e "$DEPLOY_LOCK" || -L "$DEPLOY_LOCK" ]]; then
  validate_lock_file_path
fi
exec {DEPLOY_LOCK_FD}>>"$DEPLOY_LOCK"
validate_lock_file_path
lock_path_identity="$(stat -c '%d:%i' -- "$DEPLOY_LOCK")"
lock_fd_identity="$(stat -Lc '%d:%i' -- "/proc/self/fd/$DEPLOY_LOCK_FD")"
[[ "$lock_path_identity" == "$lock_fd_identity" ]] \
  || fail "deployment lock path changed while it was being opened"
flock -n "$DEPLOY_LOCK_FD" \
  || fail "capacity/fault evidence or another release promotion is active"
validate_lock_root
validate_lock_file_path
[[ "$(stat -c '%d:%i' -- "$DEPLOY_LOCK")" == \
    "$(stat -Lc '%d:%i' -- "/proc/self/fd/$DEPLOY_LOCK_FD")" ]] \
  || fail "deployment lock path changed after it was acquired"

release_real="$(realpath -e -- "$RELEASE_DIR")"
root_real="$(realpath -e -- "$RELEASE_ROOT")"
[[ -d "$RELEASE_ROOT" && ! -L "$RELEASE_ROOT" \
    && "$(stat -c '%u' -- "$RELEASE_ROOT")" == "$current_uid" ]] \
  || fail "release root must be a real owner-held directory: $RELEASE_ROOT"
chmod 0700 -- "$RELEASE_ROOT"
[[ "$(stat -c '%a' -- "$RELEASE_ROOT")" == 700 ]] \
  || fail "release root could not be made owner-only: $RELEASE_ROOT"
release_root_parent="$(dirname "$root_real")"
[[ -d "$release_root_parent" && ! -L "$release_root_parent" \
    && "$(stat -c '%u' -- "$release_root_parent")" == "$current_uid" ]] \
  || fail "release-root parent must be a real owner-held directory: $release_root_parent"
chmod 0700 -- "$release_root_parent"
[[ "$(stat -c '%a' -- "$release_root_parent")" == 700 ]] \
  || fail "release-root parent could not be made owner-only: $release_root_parent"
[[ "$(dirname "$release_real")" == "$root_real" ]] \
  || fail "release must be an immutable child of $root_real"

exec {RELEASE_DIR_FD}<"$release_real"
release_path_identity="$(stat -c '%d:%i' -- "$release_real")"
release_fd_identity="$(stat -Lc '%d:%i' -- "/proc/self/fd/$RELEASE_DIR_FD")"
[[ "$release_path_identity" == "$release_fd_identity" ]] \
  || fail "release path changed while its directory was being opened"

release_verification="$(
  "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_real"
)"
release_id="$(jq -er '.release_id' <<<"$release_verification")"
[[ "$(basename "$release_real")" == "$release_id" ]] \
  || fail "release directory name must match the verified release ID"

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

release_identity() {
  local member
  stat -c 'directory\t%d:%i:%h:%f:%Y:%Z' -- "$release_real"
  for member in Cargo.lock release-manifest.json release-manifest.sha256 \
    trnm-game-server trnm-game-server-Cargo.toml trnm-online-e2e \
    workspace-Cargo.toml; do
    printf '%s\t%s\t%s\n' "$member" \
      "$(stat -c '%d:%i:%h:%f:%s:%Y:%Z' -- "$release_real/$member")" \
      "$(sha256sum "$release_real/$member" | awk '{print $1}')"
  done
}

release_path_still_opened_inode() {
  [[ "$(stat -c '%d:%i' -- "$release_real")" == \
      "$(stat -Lc '%d:%i' -- "/proc/self/fd/$RELEASE_DIR_FD")" ]]
}

for durable_member in Cargo.lock release-manifest.json release-manifest.sha256 \
  trnm-game-server trnm-game-server-Cargo.toml trnm-online-e2e \
  workspace-Cargo.toml; do
  fsync_regular_file "$release_real/$durable_member"
done
fsync_directory "$release_real"
fsync_directory "$root_real"

# Bind the switch to the exact directory inode and member identities accepted
# by a second full verification after durability has been established.
bound_verification="$(
  "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_real"
)"
[[ "$(jq -S -c . <<<"$bound_verification")" == \
    "$(jq -S -c . <<<"$release_verification")" ]] \
  || fail "release verification changed before selector preparation"
bound_release_identity="$(release_identity)"
release_path_still_opened_inode \
  || fail "release path changed before selector preparation"

promotion_tmp="$(mktemp -d "$root_real/.promote.XXXXXX")"
selector_path="$root_real/current"
old_selector_present=0
old_selector_target=""
old_selector_identity=""
if [[ -L "$selector_path" ]]; then
  old_selector_present=1
  old_selector_target="$(readlink -- "$selector_path")"
  old_selector_identity="$(stat -c '%d:%i:%s:%Y:%Z' -- "$selector_path")"
elif [[ -e "$selector_path" ]]; then
  fail "current release selector is not a symbolic link"
fi
selector_switched=0
promotion_committed=0

selector_still_matches_original() {
  if (( old_selector_present == 1 )); then
    [[ -L "$selector_path" \
        && "$(readlink -- "$selector_path")" == "$old_selector_target" \
        && "$(stat -c '%d:%i:%s:%Y:%Z' -- "$selector_path")" == \
          "$old_selector_identity" ]]
  else
    [[ ! -e "$selector_path" && ! -L "$selector_path" ]]
  fi
}

rollback_selector() {
  local rollback_link="$promotion_tmp/rollback-current"
  if (( old_selector_present == 1 )); then
    ln -s -- "$old_selector_target" "$rollback_link" || return 1
    mv -Tf -- "$rollback_link" "$selector_path" || return 1
  else
    rm -f -- "$selector_path" || return 1
  fi
  fsync_directory "$root_real"
}

cleanup() {
  local status=$?
  trap - EXIT
  if (( selector_switched == 1 && promotion_committed == 0 )); then
    rollback_selector \
      || { printf 'TRNM game-server release promotion failed: selector rollback failed\n' >&2; status=1; }
  fi
  rm -rf -- "$promotion_tmp"
  exit "$status"
}
trap cleanup EXIT
temporary_link="$promotion_tmp/current"
ln -s -- "$release_id" "$temporary_link"

pre_switch_verification="$(
  "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_real"
)"
[[ "$(jq -S -c . <<<"$pre_switch_verification")" == \
    "$(jq -S -c . <<<"$bound_verification")" \
    && "$(release_identity)" == "$bound_release_identity" ]] \
  || fail "release changed during promotion verification"
release_path_still_opened_inode \
  || fail "release path changed immediately before selector switch"
selector_still_matches_original \
  || fail "current selector changed while promotion held the deployment lock"

mv -Tf -- "$temporary_link" "$selector_path"
selector_switched=1
fsync_directory "$root_real"

final_verification="$(
  "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$selector_path"
)" || fail "promoted selector failed verification and will be rolled back"
[[ "$(jq -S -c . <<<"$final_verification")" == \
    "$(jq -S -c . <<<"$bound_verification")" \
    && "$(release_identity)" == "$bound_release_identity" ]] \
  || fail "promoted release identity changed and will be rolled back"
release_path_still_opened_inode \
  || fail "promoted release path changed and will be rolled back"
promotion_committed=1
printf '%s\n' "$final_verification"
