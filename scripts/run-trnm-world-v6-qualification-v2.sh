#!/usr/bin/env bash
set -euo pipefail

scope="${1:-}"
case "$scope" in
  truth|source|postgres|package) ;;
  *) echo "usage: $0 {truth|source|postgres|package}" >&2; exit 64 ;;
esac

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$root"

need() {
  test -f "$1" && test -s "$1" || {
    echo "missing or empty qualification input: $1" >&2
    exit 1
  }
}

py() {
  need "$1"
  python3 "$1"
}

mkdir -p "run/world-v6-v2/${scope}"

case "$scope" in
  truth)
    py scripts/test-trnm-world-strict-json.py
    py scripts/check-trnm-world-machine-truth.py
    py scripts/test-trnm-world-machine-truth.py
    py scripts/check-trnm-world-module-documentation.py
    py scripts/test-trnm-world-module-documentation-negative.py
    py scripts/check-trnm-world-detailed-documentation.py
    py scripts/test-trnm-world-detailed-documentation.py
    py scripts/check-trnm-world-authority-documentation.py
    py scripts/test-trnm-world-authority-documentation.py
    python3 - <<'PY'
from pathlib import Path
import json

required = (
    'CURRENT_PLAN.md',
    'PROJECT_BOUNDARY.json',
    'docs/catalog.json',
    'rust-toolchain.toml',
    'RELEASE_READINESS.md',
    'docs/development/trnm-world-gap-closure-ledger-v6.json',
    'docs/integration/trnm-world-cex-current-pending-lock-v2.json',
    'docs/release/trnm-world-external-execution-board-v1.json',
    'docs/status/world-plan-v4-execution-truth-2026-09-08.json',
)
for path in required:
    target = Path(path)
    assert target.is_file() and target.stat().st_size > 0, path

plan = Path('CURRENT_PLAN.md').read_text(encoding='utf-8')
assert 'docs/development/TRILLIONNIUM_WORLD_CLOSURE_EXECUTION_BOARD_V5.md' not in plan
for path in required[4:]:
    assert f'`{path}`' in plan, path

boundary=json.loads(Path('PROJECT_BOUNDARY.json').read_text(encoding='utf-8'))
assert boundary['project_id']=='trillionnium-world'
assert boundary['lane']=='game-product'
assert boundary['authority']['online_match_authority']=='Trillionnium-Nakama'
assert boundary['authority']['chain_finality_authority']=='Trillionnium-Chain'
assert boundary['authority']['wallet_settlement_authority']=='CEX'
assert boundary['release']['public_online']=='no_go'
assert boundary['release']['public_player_market']=='disabled'
assert boundary['release']['production_authorization']=='not_granted'
toolchain=Path('rust-toolchain.toml').read_text(encoding='utf-8')
assert '1.98.1' in toolchain and '1.98.0' not in toolchain
catalog=json.loads(Path('docs/catalog.json').read_text(encoding='utf-8'))
assert catalog['schema']=='trnm_world_document_catalog_v1'
assert catalog['truth_order'][:3]==['PROJECT_BOUNDARY.md','CURRENT_PLAN.md','docs/catalog.json']
PY
    ;;
  source)
    py scripts/check-trnm-world-transition-conformance.py
    for gate in \
      scripts/check_trnm_game_product.sh \
      scripts/check_trnm_authority_boundary.sh \
      scripts/check_trnm_runtime_configuration.sh \
      scripts/check_trnm_settlement_outbox_contract.sh \
      scripts/check_trnm_settlement_transaction_boundary.sh; do
      need "$gate"
      bash "$gate"
    done
    cargo fmt --manifest-path trillionnium/Cargo.toml --all -- --check
    cargo test --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked
    cargo clippy --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked -- -D warnings
    cargo test --manifest-path trillionnium/contracts/trnm-world-transition-v1/Cargo.toml --locked
    cargo clippy --manifest-path trillionnium/contracts/trnm-world-transition-v1/Cargo.toml --all-targets --locked -- -D warnings
    ;;
  postgres)
    need scripts/run-trnm-world-authority-postgres-check.sh
    need deploy/postgres/trnm-world-authority-cutover-v1.sql
    need deploy/postgres/trnm-world-authority-cutover-v1-hardening.sql
    bash scripts/run-trnm-world-authority-postgres-check.sh
    export TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST=1
    for test_name in \
      settlement_database_contract \
      settlement_operator_controls_database \
      settlement_fault_model \
      settlement_worker_contract \
      settlement_runtime_v2_contract; do
      cargo test --manifest-path trillionnium/Cargo.toml -p trnm-game-server --test "$test_name" --locked -- --nocapture
    done
    ;;
  package)
    if test -s scripts/check-trnm-world-dependency-exceptions.py; then
      python3 scripts/check-trnm-world-dependency-exceptions.py
    fi
    cargo metadata --manifest-path trillionnium/Cargo.toml --locked --format-version 1 > "run/world-v6-v2/${scope}/cargo-metadata.json"
    cargo tree --manifest-path trillionnium/Cargo.toml --locked --edges normal,build > "run/world-v6-v2/${scope}/cargo-tree.txt"
    cargo build --manifest-path trillionnium/Cargo.toml --release --locked -p trnm-first-contact -p trnm-game-server --bins
    need scripts/package-trnm-game-release.sh
    need scripts/check-trnm-game-package.sh
    rm -rf run/world-v6-v2/distribution
    mkdir -p run/world-v6-v2/distribution
    bash scripts/package-trnm-game-release.sh --require-clean --output-dir run/world-v6-v2/distribution
    archive="$(find run/world-v6-v2/distribution -maxdepth 1 -type f -name '*.tar.gz' -print -quit)"
    test -n "$archive" && test -s "$archive"
    TRNM_REQUIRE_CLEAN_PACKAGE=1 bash scripts/check-trnm-game-package.sh "$archive"
    sha256sum trillionnium/Cargo.lock rust-toolchain.toml "$archive" > "run/world-v6-v2/${scope}/SHA256SUMS"
    ;;
esac

{
  printf 'scope=%s\n' "$scope"
  printf 'commit=%s\n' "$(git rev-parse HEAD)"
  printf 'tree=%s\n' "$(git rev-parse 'HEAD^{tree}')"
  printf 'rustc=%s\n' "$(rustc --version 2>/dev/null || printf unavailable)"
  printf 'cargo=%s\n' "$(cargo --version 2>/dev/null || printf unavailable)"
  printf 'completed_at_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  printf 'production_authorization=not_granted\n'
} > "run/world-v6-v2/${scope}/identity.txt"

git diff --exit-code
git diff --cached --exit-code
printf 'TRNM_WORLD_V6_V2_%s=PASS\n' "$(printf '%s' "$scope" | tr '[:lower:]' '[:upper:]')"
