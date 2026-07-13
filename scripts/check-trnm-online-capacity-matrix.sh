#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROFILES_CSV="${TRNM_CAPACITY_PROFILES:-1,4,8,16,32}"
PROFILE_DURATION_SECONDS="${TRNM_CAPACITY_PROFILE_DURATION_SECONDS:-300}"
RUN_ID="capacity-matrix-$(date +%s)-${RANDOM}"
EVIDENCE="$ROOT_DIR/run/online-capacity/$RUN_ID"
mkdir -p "$EVIDENCE"

IFS=',' read -r -a profiles <<<"$PROFILES_CSV"
for profile in "${profiles[@]}"; do
  if ! [[ "$profile" =~ ^(1|4|8|16|32)$ ]]; then
    echo "capacity profiles must be selected from 1,4,8,16,32" >&2
    exit 2
  fi
  if ! TRNM_CAPACITY_CONCURRENCY="$profile" \
    TRNM_CAPACITY_DURATION_SECONDS="$PROFILE_DURATION_SECONDS" \
    "$ROOT_DIR/scripts/check-trnm-online-capacity-soak.sh" \
      >"$EVIDENCE/profile-${profile}.json" \
      2>"$EVIDENCE/profile-${profile}.stderr"; then
    jq -n --arg contract_version trnm_online_capacity_matrix_v1 \
      --arg run_id "$RUN_ID" --argjson first_failed_profile "$profile" \
      --arg evidence "$EVIDENCE/profile-${profile}.json" \
      '{contract_version:$contract_version,run_id:$run_id,status:"capacity_limit_found",
        first_failed_profile:$first_failed_profile,evidence:$evidence}' \
      >"$EVIDENCE/summary.json"
    cat "$EVIDENCE/summary.json"
    exit 1
  fi
done

jq -s --arg contract_version trnm_online_capacity_matrix_v1 --arg run_id "$RUN_ID" '
  {
    contract_version: $contract_version,
    run_id: $run_id,
    status: "all_requested_profiles_passed",
    profiles: map({concurrency,completed_matches,command_ack_p95_ms,
      max_absolute_match_tick_drift,server_restarts,passed}),
    measured_capacity: ([.[].concurrency] | max)
  }' "$EVIDENCE"/profile-*.json >"$EVIDENCE/summary.json"
cat "$EVIDENCE/summary.json"
