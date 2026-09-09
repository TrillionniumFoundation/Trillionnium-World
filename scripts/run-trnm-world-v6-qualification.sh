#!/usr/bin/env bash
set -euo pipefail

scope="${1:-}"
case "$scope" in
  truth|source|postgres|package) ;;
  *) echo "usage: $0 {truth|source|postgres|package}" >&2; exit 64 ;;
esac

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$root"

required_file() {
  test -f "$1" && test -s "$1" || {
    echo "required qualification input missing or empty: $1" >&2
    exit 1
  }
}

run_python_gate() {
  local path="$1"
  required_file "$path"
  python3 "$path"
}

record_identity() {
  mkdir -p "run/world-v6/${scope}"
  {
    printf 'scope=%s\n' "$scope"
    printf 'commit=%s\n' "$(git rev-parse HEAD)"
    printf 'tree=%s\n' "$(git rev-parse 'HEAD^{tree}')"
    printf 'rustc=%s\n' "$(rustc --version 2>/dev/null || printf unavailable)"
    printf 'cargo=%s\n' "$(cargo --version 2>/dev/null || printf unavailable)"
    printf 'utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf 'production_authorization=not_granted\n'
  } > "run/world-v6/${scope}/identity.txt"
}

case "$scope" in
  truth)
    run_python_gate scripts/test-trnm-world-strict-json.py
    run_python_gate scripts/check-trnm-world-machine-truth.py
    run_python_gate scripts/test-trnm-world-machine-truth.py
    run_python_gate scripts/check-trnm-world-module-documentation.py
    run_python_gate scripts/test-trnm-world-module-documentation-negative.py
    run_python_gate scripts/check-trnm-world-detailed-documentation.py
    run_python_gate scripts/test-trnm-world-detailed-documentation.py
    run_python_gate scripts/check-trnm-world-authority-documentation.py
    run_python_gate scripts/test-trnm-world-authority-documentation.py
    run_python_gate scripts/check-trnm-world-documentation.py
    required_file CURRENT_PLAN.md
    required_file PROJECT_BOUNDARY.json
    required_file docs/catalog.json
    required_file rust-toolchain.toml
    python3 - <<'PY'
import json
from pathlib import Path
boundary=json.loads(Path('PROJECT_BOUNDARY.json').read_text(encoding='utf-8'))
assert boundary['project_id']=='trillionnium-world'
assert boundary['lane']=='game-product'
assert boundary['release']['production_authorization']=='not_granted'
assert boundary['release']['public_online']=='no_go'
assert boundary['release']['public_player_market']=='disabled'
toolchain=Path('rust-toolchain.toml').read_text(encoding='utf-8')
assert '1.98.1' in toolchain and '1.98.0' not in toolchain
PY
    ;;

  source)
    run_python_gate scripts/check-trnm-world-transition-conformance.py
    required_file scripts/check_trnm_game_product.sh
    required_file scripts/check_trnm_authority_boundary.sh
    required_file scripts/check_trnm_runtime_configuration.sh
    required_file scripts/check_trnm_settlement_outbox_contract.sh
    required_file scripts/check_trnm_settlement_transaction_boundary.sh
    bash scripts/check_trnm_game_product.sh
    bash scripts/check_trnm_authority_boundary.sh
    bash scripts/check_trnm_runtime_configuration.sh
    bash scripts/check_trnm_settlement_outbox_contract.sh
    bash scripts/check_trnm_settlement_transaction_boundary.sh
    cargo fmt --manifest-path trillionnium/Cargo.toml --all -- --check
    cargo test --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked
    cargo clippy --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked -- -D warnings
    cargo test --manifest-path trillionnium/contracts/trnm-world-transition-v1/Cargo.toml --locked
    cargo clippy --manifest-path trillionnium/contracts/trnm-world-transition-v1/Cargo.toml --all-targets --locked -- -D warnings
    ;;

  postgres)
    required_file scripts/run-trnm-world-authority-postgres-check.sh
    required_file deploy/postgres/trnm-world-authority-cutover-v1.sql
    required_file deploy/postgres/trnm-world-authority-cutover-v1-hardening.sql
    bash scripts/run-trnm-world-authority-postgres-check.sh
    export TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST=1
    cargo test --manifest-path trillionnium/Cargo.toml -p trnm-game-server --test settlement_database_contract --locked -- --nocapture
    cargo test --manifest-path trillionnium/Cargo.toml -p trnm-game-server --test settlement_operator_controls_database --locked -- --nocapture
    cargo test --manifest-path trillionnium/Cargo.toml -p trnm-game-server --test settlement_fault_model --locked -- --nocapture
    cargo test --manifest-path trillionnium/Cargo.toml -p trnm-game-server --test settlement_worker_contract --locked -- --nocapture
    cargo test --manifest-path trillionnium/Cargo.toml -p trnm-game-server --test settlement_runtime_v2_contract --locked -- --nocapture
    ;;

  package)
    if test -f scripts/check-trnm-world-dependency-exceptions.py; then
      python3 scripts/check-trnm-world-dependency-exceptions.py
    fi
    cargo metadata --manifest-path trillionnium/Cargo.toml --locked --format-version 1 > /tmp/trnm-world-v6-metadata.json
    cargo tree --manifest-path trillionnium/Cargo.toml --locked --edges normal,build > /tmp/trnm-world-v6-tree.txt
    cargo build --manifest-path trillionnium/Cargo.toml --release --locked -p trnm-first-contact -p trnm-game-server --bins
    required_file scripts/package-trnm-game-release.sh
    required_file scripts/check-trnm-game-package.sh
    rm -rf run/world-v6/distribution
    mkdir -p run/world-v6/distribution
    bash scripts/package-trnm-game-release.sh --require-clean --output-dir run/world-v6/distribution
    archive="$(find run/world-v6/distribution -maxdepth 1 -type f -name '*.tar.gz' -print -quit)"
    test -n "$archive" && test -s "$archive"
    TRNM_REQUIRE_CLEAN_PACKAGE=1 bash scripts/check-trnm-game-package.sh "$archive"
    mkdir -p run/world-v6/package
    cp /tmp/trnm-world-v6-metadata.json run/world-v6/package/cargo-metadata.json
    cp /tmp/trnm-world-v6-tree.txt run/world-v6/package/cargo-tree.txt
    sha256sum trillionnium/Cargo.lock rust-toolchain.toml "$archive" > run/world-v6/package/SHA256SUMS
    ;;
esac

record_identity

git diff --exit-code
git diff --cached --exit-code
printf 'TRNM_WORLD_V6_%s=PASS\n' "$(printf '%s' "$scope" | tr '[:lower:]' '[:upper:]')"
