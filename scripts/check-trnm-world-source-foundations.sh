#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"
export PYTHONDONTWRITEBYTECODE=1

python3 scripts/world_source_foundations/check_http.py
python3 scripts/world_source_foundations/check_stream.py
python3 scripts/world_source_foundations/check_database_and_install.py
python3 scripts/world_source_foundations/check_status.py

printf 'world_source_foundations=passed\n'
