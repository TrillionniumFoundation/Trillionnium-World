#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "usage: run-world-exact-surface-v1.sh REPOSITORY SURFACE" >&2
  exit 64
fi

repository="$1"
surface="$2"
case "$surface" in
  head|prospective-merge) ;;
  *) echo "unsupported surface: $surface" >&2; exit 64 ;;
esac

test -d "$repository/.git"
cd "$repository"
test -z "$(git status --porcelain=v1 --untracked-files=no)"
printf 'WORLD_SURFACE_BEGIN=%s\n' "$surface"
printf 'WORLD_SURFACE_COMMIT=%s\n' "$(git rev-parse HEAD)"
printf 'WORLD_SURFACE_TREE=%s\n' "$(git rev-parse 'HEAD^{tree}')"

git diff --check
python3 scripts/check-trnm-world-component-catalog.py
python3 scripts/test-trnm-world-component-catalog.py
python3 scripts/check-trnm-world-include-boundaries.py
python3 scripts/test-trnm-world-include-boundaries.py
cargo fmt --manifest-path trillionnium/Cargo.toml --all -- --check
cargo test --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked -- -D warnings
cargo build --manifest-path trillionnium/Cargo.toml --workspace --release --locked

psql -h 127.0.0.1 -U postgres -d postgres -v ON_ERROR_STOP=1 \
  -c 'drop database if exists trillionnium_world_test with (force);'
psql -h 127.0.0.1 -U postgres -d postgres -v ON_ERROR_STOP=1 \
  -c 'create database trillionnium_world_test;'
DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/trillionnium_world_test' \
TEST_DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/trillionnium_world_test' \
  cargo test --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked -- --test-threads=1

package_list="${RUNNER_TEMP:-/tmp}/world-${surface}-publishable-packages.txt"
python3 - <<'PY' > "$package_list"
import json
import subprocess
metadata = json.loads(subprocess.check_output([
    'cargo', 'metadata', '--manifest-path', 'trillionnium/Cargo.toml',
    '--format-version', '1', '--no-deps', '--locked'
]))
members = set(metadata['workspace_members'])
names = []
for package in metadata['packages']:
    if package['id'] not in members or package.get('publish') == []:
        continue
    names.append(package['name'])
for name in sorted(set(names)):
    print(name)
PY
test -s "$package_list"
while IFS= read -r package; do
  cargo package --manifest-path trillionnium/Cargo.toml \
    -p "$package" --locked --allow-dirty
done < "$package_list"

printf 'WORLD_SURFACE_PASS=%s\n' "$surface"
printf 'PRODUCTION_AUTHORIZATION=not_granted\n'
