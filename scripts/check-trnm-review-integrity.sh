#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$root"

fail() {
  printf 'TRNM review integrity: FAIL: %s\n' "$*" >&2
  exit 1
}

for forbidden in \
  .github/workflows/agent-user-phasea-gate.yml \
  .github/workflows/p1-rust-sidecar.yml \
  .github/workflows/rust-l1-nightly-health.yml \
  .github/workflows/rust-l1-testnet-preflight.yml \
  .github/workflows/trnm-gate-quick-check.yml \
  .github/workflows/trnm-merge-gates.yml \
  .github/workflows/web4-frontend-ci.yml \
  .github/workflows/apply-world-settlement-gap-closure-v1.yml \
  .github/workflows/trnm-world-settlement-self-heal.yml \
  .github/workflows/world-settlement-converge.yml; do
  [[ ! -e "$forbidden" ]] || fail "retired workflow remains active: $forbidden"
done

if rg -n --glob '.github/workflows/*.{yml,yaml}' 'contents:[[:space:]]*write' .github/workflows; then
  fail 'active workflow has contents: write'
fi

if rg -n --glob '.github/workflows/*.{yml,yaml}' \
  'cargo[[:space:]]+(clippy[[:space:]]+--fix|fix)|git[[:space:]]+(commit|push|tag)' \
  .github/workflows; then
  fail 'active workflow can modify, push or tag source'
fi

while IFS= read -r line; do
  ref="${line##*@}"
  [[ "$ref" =~ ^[0-9a-f]{40}$ ]] || fail "workflow action is not immutable: $line"
done < <(rg -N --no-filename --glob '.github/workflows/*.{yml,yaml}' \
  '^[[:space:]]*uses:[[:space:]]*[^#[:space:]]+@[^#[:space:]]+' .github/workflows | sed -E 's/^[[:space:]]*uses:[[:space:]]*//')

for junk in __schema_probe__ nonexistent micro_patch.diff; do
  [[ ! -e "$junk" ]] || fail "temporary probe artifact remains: $junk"
done

printf 'TRNM review integrity: PASS\n'
