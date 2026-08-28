#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

bash scripts/check-trnm-settlement-transaction-boundary.sh full
python3 scripts/check-trnm-settlement-runtime-status.py

python3 - <<'PY'
import json
from pathlib import Path

status = json.loads(Path("docs/status/settlement-runtime-v1.json").read_text(encoding="utf-8"))
expected_gates = {
    "merge_cex_owner_repository_pull_request",
    "bind_exact_cex_build_and_deployment_artifact",
    "prove_deployed_signer_and_cex_response_loss_recovery",
    "prove_process_kill_cancellation_shutdown_and_apply_rollback_matrix",
    "approve_backup_pitr_restore_and_receipt_retention",
    "obtain_exact_commit_github_actions_evidence",
    "obtain_reviewer_signoff",
}
if set(status["open_gates"]) != expected_gates:
    raise SystemExit("final settlement status hid or invented external blockers")
if status["public_online"] != "no_go":
    raise SystemExit("public online was overclaimed")
if status["public_player_market"] != "disabled":
    raise SystemExit("public player market was overclaimed")
if status["release_effect"] != "none" or status["verified_commit"] is not None:
    raise SystemExit("candidate source invented release or verified-commit credit")
PY

printf '%s\n' \
  'TRNM settlement transaction boundary: PASS (stale compatibility caller assumptions retired; final external deployment, governance and review blockers remain explicit)'
