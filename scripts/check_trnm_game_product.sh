#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR/trillionnium"

members="$(cargo metadata --no-deps --format-version 1 | jq -r '.workspace_members[]')"
for required in trnm-economy-protocol trnm-rpg-core trnm-campaign-core trnm-rts-protocol trnm-rts-sim trnm-online-protocol trnm-game-server trnm-first-contact; do
  grep -q "/${required}#" <<<"$members"
done

count="$(cargo metadata --no-deps --format-version 1 | jq '.workspace_members | length')"
[[ "$count" == "8" ]]
[[ "$(cargo metadata --manifest-path crates/platform/Cargo.toml --no-deps --format-version 1 | jq '.workspace_members | length')" == "12" ]]
[[ ! -e crates/legacy-game ]]

tree="$(cargo tree -p trnm-first-contact --depth 2)"
! grep -q 'legacy-game' <<<"$tree"
! grep -q 'crates/platform' <<<"$tree"
! grep -Eq 'trnm-world-(bevy|domain|api|server|projection)|trnm-rts-(core|data|evidence|online|bevy-runtime)' <<<"$tree"
grep -q 'trnm-campaign-core' <<<"$tree"
grep -q 'trnm-rts-sim' <<<"$tree"
grep -q 'trnm-rpg-core' <<<"$tree"
grep -q 'trnm-rts-protocol' <<<"$tree"
grep -q 'trnm-economy-protocol' <<<"$tree"
grep -q 'trnm-online-protocol' <<<"$tree"
! grep -q 'trnm-game-server' <<<"$tree"

echo "TRNM game product boundary: green (8 game / 12 platform / legacy working tree absent)"
