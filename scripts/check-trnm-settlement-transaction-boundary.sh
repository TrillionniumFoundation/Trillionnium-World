#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# Preserve full/scan-only, but actually forward the optional fixture directory.
# Python rejects unknown modes, extra arguments and fixture paths in full mode.
exec python3 "$ROOT_DIR/scripts/check-trnm-settlement-transaction-boundary.py" "$@"
