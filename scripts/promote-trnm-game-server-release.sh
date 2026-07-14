#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RELEASE_ROOT="${TRNM_GAME_SERVER_RELEASE_ROOT:-$ROOT_DIR/run/releases/trnm-game-server}"
RELEASE_DIR="${1:?usage: promote-trnm-game-server-release.sh RELEASE_DIR}"

release_real="$(realpath -e -- "$RELEASE_DIR")"
root_real="$(realpath -e -- "$RELEASE_ROOT")"
[[ "$(dirname "$release_real")" == "$root_real" ]] || {
  echo "Release must be an immutable child of $root_real" >&2
  exit 1
}

"$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$release_real" >/dev/null
release_id="$(jq -r '.release_id' "$release_real/release-manifest.json")"
[[ "$(basename "$release_real")" == "$release_id" ]] || {
  echo "Release directory name must match the verified release ID" >&2
  exit 1
}

promotion_tmp="$(mktemp -d "$root_real/.promote.XXXXXX")"
cleanup() {
  rm -rf -- "$promotion_tmp"
}
trap cleanup EXIT
temporary_link="$promotion_tmp/current"
ln -s "$release_id" "$temporary_link"
mv -Tf "$temporary_link" "$root_real/current"

"$ROOT_DIR/scripts/check-trnm-game-server-release.sh" "$root_real/current"
