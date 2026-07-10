#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR/trillionnium"

members="$(cargo metadata --no-deps --format-version 1 | jq -r '.workspace_members[]')"
for required in trnm-world-domain trnm-campaign-core trnm-rts-core trnm-rts-sim trnm-first-contact; do
  grep -q "/${required}#" <<<"$members"
done

count="$(cargo metadata --no-deps --format-version 1 | jq '.workspace_members | length')"
[[ "$count" == "5" ]]
[[ "$(cargo metadata --manifest-path crates/platform/Cargo.toml --no-deps --format-version 1 | jq '.workspace_members | length')" == "12" ]]
[[ "$(cargo metadata --manifest-path crates/legacy-game/Cargo.toml --no-deps --format-version 1 | jq '.workspace_members | length')" == "12" ]]

tree="$(cargo tree -p trnm-first-contact --depth 2)"
! grep -q 'legacy-game' <<<"$tree"
! grep -q 'crates/platform' <<<"$tree"
grep -q 'trnm-campaign-core' <<<"$tree"
grep -q 'trnm-rts-sim' <<<"$tree"

echo "TRNM game product boundary: green (5 game / 12 platform / 12 frozen legacy)"
