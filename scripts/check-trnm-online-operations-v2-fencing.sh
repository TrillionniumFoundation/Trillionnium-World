#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

PRIMARY_URL="${TRNM_GAME_SERVER_URL:-http://127.0.0.1:7005}"
DUPLICATE_URL="http://127.0.0.1:7006"
MODERATOR_TOKEN="${TRNM_MODERATOR_TOKEN:-trnm-moderator-v1:$IDENTITY_ADMIN_TOKEN}"
RUN_ID="online-operations-fencing-$(date +%s)-${RANDOM}"
EVIDENCE="$ROOT_DIR/acceptance/online-operations-v2-fencing/$RUN_ID"
DUPLICATE_PID=""
mkdir -p "$EVIDENCE"

cleanup() {
  local status=$?
  [[ -z "$DUPLICATE_PID" ]] || kill "$DUPLICATE_PID" >/dev/null 2>&1 || true
  systemctl --user restart trnm-game-server.service >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT

route_body='{"protocol_version":"trnm_online_operations_v2","build_id":"trnm-online-operations-2026.07-v2","preferred_region":"local-x230"}'
old_readiness="$(curl -fsS "$PRIMARY_URL/v1/online/readiness")"
OLD_EPOCH="$(jq -er .fleet_instance_epoch <<<"$old_readiness")"

TRNM_GAME_SERVER_BIND_ADDR="127.0.0.1:7006" \
TRNM_FLEET_INSTANCE_ID="trnm-local-primary" \
TRNM_FLEET_REGION="local-x230" \
TRNM_FLEET_PUBLIC_ENDPOINT="$DUPLICATE_URL" \
TRNM_FLEET_CAPACITY=2 TRNM_GAME_SERVER_TICK_MS=20 \
  "$ROOT_DIR/scripts/run-trnm-game-server.sh" >"$EVIDENCE/duplicate.log" 2>&1 &
DUPLICATE_PID=$!
for _ in $(seq 1 60); do
  curl -fsS "$DUPLICATE_URL/v1/online/readiness" >/dev/null 2>&1 && break
  sleep 0.25
done
duplicate_readiness="$(curl -fsS "$DUPLICATE_URL/v1/online/readiness")"
NEW_EPOCH="$(jq -er .fleet_instance_epoch <<<"$duplicate_readiness")"
[[ "$NEW_EPOCH" -gt "$OLD_EPOCH" ]]

sleep 1.2
stale_route_status="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$PRIMARY_URL/v1/operations/fleet/route" -H 'content-type: application/json' \
  --data-binary "$route_body")"
[[ "$stale_route_status" == "503" ]]
duplicate_route="$(curl -fsS "$DUPLICATE_URL/v1/operations/fleet/route" \
  -H 'content-type: application/json' --data-binary "$route_body")"
jq -e --argjson epoch "$NEW_EPOCH" \
  '.selected.instance_id == "trnm-local-primary" and
   .selected.instance_epoch == $epoch and .selected.status == "active"' \
  <<<"$duplicate_route" >/dev/null

wrong_admin_status="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$DUPLICATE_URL/v1/operations/fleet/admin" -H 'x-trnm-moderator: wrong' \
  -H 'content-type: application/json' \
  --data-binary '{"instance_id":"trnm-local-primary","action":"drain","reason":"Unauthorized drain must fail closed."}')"
[[ "$wrong_admin_status" == "401" ]]
drained="$(curl -fsS "$DUPLICATE_URL/v1/operations/fleet/admin" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary '{"instance_id":"trnm-local-primary","action":"drain","reason":"Operations v2 acceptance drains new allocations safely."}')"
jq -e '.status == "draining" and .active_matches == 0' <<<"$drained" >/dev/null
drained_route_status="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$DUPLICATE_URL/v1/operations/fleet/route" -H 'content-type: application/json' \
  --data-binary "$route_body")"
[[ "$drained_route_status" == "503" ]]
activated="$(curl -fsS "$DUPLICATE_URL/v1/operations/fleet/admin" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary '{"instance_id":"trnm-local-primary","action":"activate","reason":"Operations v2 acceptance returns the drained instance to service."}')"
jq -e '.status == "active"' <<<"$activated" >/dev/null
curl -fsS "$DUPLICATE_URL/v1/operations/fleet/route" -H 'content-type: application/json' \
  --data-binary "$route_body" >/dev/null
offlined="$(curl -fsS "$DUPLICATE_URL/v1/operations/fleet/admin" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary '{"instance_id":"trnm-local-primary","action":"offline","reason":"Operations v2 acceptance verifies explicit zero-match offline control."}')"
jq -e '.status == "offline" and .active_matches == 0' <<<"$offlined" >/dev/null

kill "$DUPLICATE_PID"
wait "$DUPLICATE_PID" 2>/dev/null || true
DUPLICATE_PID=""
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  restored="$(curl -fsS "$PRIMARY_URL/v1/online/readiness" 2>/dev/null || true)"
  if jq -e --argjson epoch "$NEW_EPOCH" '.fleet_instance_epoch > $epoch' <<<"$restored" >/dev/null 2>&1; then
    break
  fi
  sleep 0.25
done
FINAL_EPOCH="$(jq -er .fleet_instance_epoch <<<"$restored")"
[[ "$FINAL_EPOCH" -gt "$NEW_EPOCH" ]]

database="$(cex_psql_stdin -Atc "select json_build_object(
  'instance_epoch',(select instance_epoch from trnm_online_fleet_instances where instance_id='trnm-local-primary'),
  'status',(select status from trnm_online_fleet_instances where instance_id='trnm-local-primary'),
  'audits',(select count(*) from trnm_online_fleet_admin_audit where instance_id='trnm-local-primary' and created_at > now()-interval '10 minutes')
)" | jq -c .)"
jq -e --argjson final "$FINAL_EPOCH" \
  '.instance_epoch == $final and .status == "active" and .audits >= 3' <<<"$database" >/dev/null

jq -n --arg run_id "$RUN_ID" --argjson old_epoch "$OLD_EPOCH" \
  --argjson duplicate_epoch "$NEW_EPOCH" --argjson final_epoch "$FINAL_EPOCH" \
  --argjson database "$database" \
  '{status:"passed",run_id:$run_id,same_instance_generation_fencing:true,
    stale_route_status:503,old_epoch:$old_epoch,duplicate_epoch:$duplicate_epoch,
    restored_epoch:$final_epoch,drain_route_fail_closed:true,explicit_offline:true,
    fleet_admin_audit:true,database:$database,
    boundary:"same-host duplicate-instance fencing evidence; not cross-host quorum or regional HA"}' \
  | tee "$EVIDENCE/report.json"
