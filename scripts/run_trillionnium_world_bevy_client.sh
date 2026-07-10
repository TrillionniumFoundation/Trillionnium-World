#!/usr/bin/env bash
set -euo pipefail

# Compatibility alias retained for existing local service units. The only
# player product is trnm-first-contact; classic/legacy runners are archived.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec "$ROOT_DIR/scripts/run_trnm_first_contact.sh" "$@"
