#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
ONLINE_URL="${TRNM_GAME_SERVER_URL:-http://127.0.0.1:7005}"
SIGNER_URL="${TRNM_ENTITLEMENT_SIGNER_URL:-http://127.0.0.1:7010}"
ADMIN_TOKEN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?identity admin token required}}"
MODERATOR_TOKEN="${TRNM_MODERATOR_TOKEN:-trnm-moderator-v1:$IDENTITY_ADMIN_TOKEN}"
GAME_TOKEN="${TRNM_GAME_AUTHORITY_TOKEN:-trnm-game-authority-v1:$IDENTITY_ADMIN_TOKEN}"
SIGNER_TOKEN="${TRNM_ENTITLEMENT_SIGNER_TOKEN:-trnm-isolated-signer-v1:$IDENTITY_ADMIN_TOKEN}"
PROTOCOL="trnm_online_production_v2"
BUILD="trnm-online-production-2026.07-v2"
RUN_ID="online-production-v2-$(date +%s)-${RANDOM}"
EVIDENCE="$ROOT_DIR/acceptance/online-production-v2/$RUN_ID"
DISPLAY="${TRNM_PRODUCTION_V2_DISPLAY:-:97}"
RATE_A_PID=""
RATE_B_PID=""
XVFB_PID=""
PRODUCT_PID=""
mkdir -p "$EVIDENCE"

cleanup() {
  local status=$?
  [[ -z "$PRODUCT_PID" ]] || kill "$PRODUCT_PID" >/dev/null 2>&1 || true
  [[ -z "$XVFB_PID" ]] || kill "$XVFB_PID" >/dev/null 2>&1 || true
  [[ -z "$RATE_A_PID" ]] || kill "$RATE_A_PID" >/dev/null 2>&1 || true
  [[ -z "$RATE_B_PID" ]] || kill "$RATE_B_PID" >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT

json_post() {
  local url="$1" body="$2"
  curl -fsS "$url" -H 'content-type: application/json' --data-binary "$body"
}

admin_post() {
  local path="$1" body="$2"
  curl -fsS "$LEDGER_URL$path" -H "x-admin-token: $ADMIN_TOKEN" \
    -H 'content-type: application/json' --data-binary "$body"
}

player_post() {
  local session="$1" path="$2" body="$3"
  curl -fsS "$ONLINE_URL$path" -H "x-trnm-player-session: $session" \
    -H 'content-type: application/json' --data-binary "$body"
}

login() {
  local player="$1" credential="$2" device="$3"
  json_post "$LEDGER_URL/v1/trnm/product/login" "$(jq -cn \
    --arg player "$player" --arg credential "$credential" --arg device "$device" \
    '{player_id:$player,credential:$credential,device_id:$device,lifetime_seconds:3600}')"
}

register_player() {
  local player="$1" credential="$2" invite
  invite="$(admin_post /v1/trnm/product/registration-invites \
    '{"lifetime_seconds":3600,"max_uses":1}' | jq -er .invite_code)"
  json_post "$LEDGER_URL/v1/trnm/product/register" "$(jq -cn \
    --arg player "$player" --arg credential "$credential" --arg invite "$invite" \
    '{player_id:$player,credential:$credential,invite_code:$invite}')"
}

window_for_pid() {
  local pid="$1" id actual name
  while read -r id; do
    [[ -n "$id" ]] || continue
    actual="$(xprop -id "$id" _NET_WM_PID 2>/dev/null | awk -F' = ' '{print $2}')"
    name="$(xprop -id "$id" WM_NAME 2>/dev/null || true)"
    if [[ "$actual" == "$pid" && "$name" == *'Online Product v2'* ]]; then
      printf '%s\n' "$id"
      return 0
    fi
  done < <(xwininfo -root -tree 2>/dev/null | awk '/0x[0-9a-f]+/ {print $1}')
  return 1
}

wait_window() {
  local pid="$1" id=""
  for _ in $(seq 1 60); do
    id="$(window_for_pid "$pid" || true)"
    [[ -z "$id" ]] || { printf '%s\n' "$id"; return 0; }
    sleep 0.5
  done
  return 1
}

wait_state() {
  local state="$1"
  for _ in $(seq 1 120); do
    [[ -s "$EVIDENCE/state.json" ]] &&
      [[ "$(jq -r .state "$EVIDENCE/state.json")" == "$state" ]] && return 0
    sleep 0.25
  done
  [[ -s "$EVIDENCE/state.json" ]] && cat "$EVIDENCE/state.json" >&2
  return 1
}

capture_rendered() {
  local window="$1" output="$2" pixels body border title
  for _ in $(seq 1 20); do
    xwd -silent -id "$window" 2>/dev/null | xwdtopnm 2>/dev/null | pnmtopng >"$output"
    pixels="$(pngtopnm "$output" 2>/dev/null | od -An -v -tu1 |
      awk '{for(i=1;i<=NF;i++) if($i>=16)n++} END{print n+0}')"
    body="$(pngtopnm "$output" 2>/dev/null | ppmhist 2>/dev/null |
      awk '$1==9 && $2==18 && $3==16 {print $5}')"
    border="$(pngtopnm "$output" 2>/dev/null | ppmhist 2>/dev/null |
      awk '$1==64 && $2==133 && $3==107 {print $5}')"
    title="$(pngtopnm "$output" 2>/dev/null | ppmhist 2>/dev/null |
      awk '$1==242 && $2==209 && $3==107 {print $5}')"
    if (( pixels >= 5000 && ${body:-0} >= 250000 && ${border:-0} >= 2500 && ${title:-0} >= 1500 )); then
      return 0
    fi
    sleep 0.5
  done
  return 1
}

V1_REPORT="$($ROOT_DIR/scripts/check-trnm-online-production-v1-e2e.sh)"
PRODUCT_RUN="$(jq -er .after_product.run_id <<<"$V1_REPORT")"
MATCH_ID="$(jq -er .after_rotation_match <<<"$V1_REPORT")"
HOST="$PRODUCT_RUN-host"
HOST_CREDENTIAL="credential-$PRODUCT_RUN-host-012345678901234567890123"
HOST_LOGIN="$(login "$HOST" "$HOST_CREDENTIAL" "$RUN_ID-host")"
HOST_SESSION="$(jq -er .session_token <<<"$HOST_LOGIN")"
HOST_ACCOUNT="$(jq -er .account_id <<<"$HOST_LOGIN")"

CHALLENGE="trnm-production-v2-registry-$RUN_ID"
signer_attestation="$(curl -fsS "$SIGNER_URL/v1/signer/attest" \
  -H "x-trnm-signer-auth: $SIGNER_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg challenge "$CHALLENGE" \
    '{contract_version:"trnm_entitlement_signer_v1",challenge:$challenge}')")"
jq -e --arg challenge "$CHALLENGE" \
  '.challenge == $challenge and .provider_kind == "file_seed" and
   (.public_key_sha256 | length) == 64 and (.signature | length) > 40' \
  <<<"$signer_attestation" >/dev/null
SIGNER_KEY_ID="$(jq -er .key_id <<<"$signer_attestation")"
registry_status="$(curl -fsS "$LEDGER_URL/v1/trnm/economy/issuer-keys/status" \
  -H "x-trnm-game-authority: $GAME_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg key "$SIGNER_KEY_ID" '{key_id:$key}')")"
jq -e --arg key "$SIGNER_KEY_ID" --arg hash "$(jq -er .public_key_sha256 <<<"$signer_attestation")" \
  '.key_id == $key and .status == "active" and .signature_algorithm == "ed25519" and
   .public_key_sha256 == $hash' <<<"$registry_status" >/dev/null

SPECTATOR="$RUN_ID-spectator"
SPECTATOR_CREDENTIAL="credential-$RUN_ID-spectator-012345678901234567890123"
SPECTATOR_IDENTITY="$(register_player "$SPECTATOR" "$SPECTATOR_CREDENTIAL")"
SPECTATOR_ACCOUNT="$(jq -er .account_id <<<"$SPECTATOR_IDENTITY")"
SPECTATOR_LOGIN="$(login "$SPECTATOR" "$SPECTATOR_CREDENTIAL" "$RUN_ID-spectator")"
SPECTATOR_SESSION="$(jq -er .session_token <<<"$SPECTATOR_LOGIN")"
SPECTATOR_SLOT="v2-$(printf '%s' "$RUN_ID" | sha256sum | cut -c1-24)"
player_post "$SPECTATOR_SESSION" /v1/online/campaigns/connect "$(jq -cn \
  --arg player "$SPECTATOR" --arg account "$SPECTATOR_ACCOUNT" --arg slot "$SPECTATOR_SLOT" \
  '{protocol_version:"trnm_online_authority_v2",build_id:"trnm-online-authority-2026.07-v2",
    player_id:$player,account_id:$account,slot_key:$slot}')" >/dev/null
invite="$(player_post "$HOST_SESSION" /v1/production/spectators/invites "$(jq -cn \
  --arg protocol "$PROTOCOL" --arg build "$BUILD" --arg player "$HOST" \
  --arg account "$HOST_ACCOUNT" --arg match "$MATCH_ID" --arg target "$SPECTATOR" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,
    match_id:$match,target_player_id:$target,delay_seconds:30}')")"
INVITE_TOKEN="$(jq -er .invite_token <<<"$invite")"

export DISPLAY
Xvfb "$DISPLAY" -screen 0 1280x720x24 -nolisten tcp >"$EVIDENCE/xvfb.log" 2>&1 &
XVFB_PID=$!
for _ in $(seq 1 30); do xdpyinfo -display "$DISPLAY" >/dev/null 2>&1 && break; sleep 0.5; done
xdpyinfo -display "$DISPLAY" >/dev/null
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY TRNM_CEX_ENTRY_TOKEN
TRNM_PRODUCT_PLAYER_ID="$SPECTATOR" TRNM_PRODUCT_CREDENTIAL="$SPECTATOR_CREDENTIAL" \
TRNM_PRODUCT_DEVICE_ID="$RUN_ID-native" TRNM_ONLINE_SLOT_KEY="$SPECTATOR_SLOT" \
TRNM_PRODUCT_SPECTATOR_INVITE_TOKEN="$INVITE_TOKEN" \
TRNM_PRODUCT_EVIDENCE_PATH="$EVIDENCE/state.json" \
  "$ROOT_DIR/target/release/trnm-online-product" >"$EVIDENCE/product.log" 2>&1 &
PRODUCT_PID=$!
WINDOW="$(wait_window "$PRODUCT_PID")"
"$ROOT_DIR/scripts/x11_key_inject.py" "$WINDOW" f1 f2 f10
wait_state "SPECTATOR GRANTED"
cex_psql_stdin -c "update trnm_online_replay_frames set created_at=now()-interval '31 seconds'
  where match_id='$MATCH_ID'::uuid" >/dev/null
"$ROOT_DIR/scripts/x11_key_inject.py" "$WINDOW" f11
wait_state "DELAYED SPECTATOR"
jq -e --arg player "$SPECTATOR" --arg match "$MATCH_ID" \
  '.player_id == $player and .production_protocol == "trnm_online_production_v2" and
   .signer_registry_verified == true and .signer_provider_kind == "file_seed" and
   .spectator_match_id == $match and .spectator_frame_count >= 2 and
   .spectator_terminal_visible == true and .spectator_visible_through_tick > 0' \
  "$EVIDENCE/state.json" >/dev/null
! rg -q "$INVITE_TOKEN" "$EVIDENCE/state.json"
sleep 2
capture_rendered "$WINDOW" "$EVIDENCE/delayed-spectator.png"
kill "$PRODUCT_PID" >/dev/null 2>&1 || true
wait "$PRODUCT_PID" 2>/dev/null || true
PRODUCT_PID=""
kill "$XVFB_PID" >/dev/null 2>&1 || true
wait "$XVFB_PID" 2>/dev/null || true
XVFB_PID=""

for port in 7006 7007; do
  instance="$RUN_ID-admission-$port"
  TRNM_GAME_SERVER_BIND_ADDR="127.0.0.1:$port" \
  TRNM_FLEET_INSTANCE_ID="$instance" TRNM_FLEET_REGION=local-x230 \
  TRNM_FLEET_PUBLIC_ENDPOINT="http://127.0.0.1:$port" TRNM_FLEET_CAPACITY=1 \
  TRNM_PRODUCTION_RATE_LIMIT_PER_MINUTE=30 \
    "$ROOT_DIR/scripts/run-trnm-game-server.sh" >"$EVIDENCE/admission-$port.log" 2>&1 &
  if [[ "$port" == "7006" ]]; then RATE_A_PID=$!; else RATE_B_PID=$!; fi
done
for port in 7006 7007; do
  for _ in $(seq 1 80); do
    curl -fsS "http://127.0.0.1:$port/health" >/dev/null 2>&1 && break
    sleep 0.25
  done
  curl -fsS "http://127.0.0.1:$port/health" >/dev/null
done
PROBE_PATH="/v1/production/distributed-probe/$RUN_ID"
for _ in $(seq 1 15); do
  [[ "$(curl -sS -o /dev/null -w '%{http_code}' "http://127.0.0.1:7006$PROBE_PATH")" == "404" ]]
  [[ "$(curl -sS -o /dev/null -w '%{http_code}' "http://127.0.0.1:7007$PROBE_PATH")" == "404" ]]
done
DISTRIBUTED_LIMIT_STATUS="$(curl -sS -o /dev/null -w '%{http_code}' \
  "http://127.0.0.1:7006$PROBE_PATH")"
[[ "$DISTRIBUTED_LIMIT_STATUS" == "429" ]]
sleep 6
CAPACITY_SAMPLES="$(cex_psql_stdin -Atc "select count(distinct instance_id)
  from trnm_online_capacity_samples where instance_id like '$RUN_ID-admission-%'")"
[[ "$CAPACITY_SAMPLES" == "2" ]]

HOST_CHALLENGE="trnm-host-challenge-$RUN_ID-0123456789"
host_attestation="$(curl -fsS "$ONLINE_URL/v1/production/host-attestation" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg protocol "$PROTOCOL" --arg build "$BUILD" \
    --arg challenge "$HOST_CHALLENGE" \
    '{protocol_version:$protocol,build_id:$build,challenge:$challenge}')")"
jq -e --arg challenge "$HOST_CHALLENGE" \
  '.challenge == $challenge and (.physical_host_id | startswith("host-")) and
   (.evidence_hash | length) == 64 and (.boundary | contains("not hardware identity"))' \
  <<<"$host_attestation" >/dev/null
HOST_EVIDENCE_HASH="$(jq -er .evidence_hash <<<"$host_attestation")"

ENFORCEMENT_ID="$(tr -d '\n' </proc/sys/kernel/random/uuid)"
cex_psql_stdin -c "insert into trnm_online_enforcements (
  enforcement_id,player_id,scope,reason,expires_at
) values ('$ENFORCEMENT_ID'::uuid,'$SPECTATOR','ranked',
  'Production v2 staffed-shift ownership acceptance fixture.',now()+interval '2 hours')" >/dev/null
appeal="$(player_post "$SPECTATOR_SESSION" /v1/operations/enforcements/appeals "$(jq -cn \
  --arg protocol "$PROTOCOL" --arg build "$BUILD" --arg player "$SPECTATOR" \
  --arg account "$SPECTATOR_ACCOUNT" --arg enforcement "$ENFORCEMENT_ID" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,
    enforcement_id:$enforcement,detail:"Production v2 shift claim and handoff acceptance fixture."}')")"
APPEAL_ID="$(jq -er .appeal_id <<<"$appeal")"
MODERATOR_ID="$RUN_ID-operator"
shift="$(curl -fsS "$ONLINE_URL/v1/production/moderation/shifts/start" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg moderator "$MODERATOR_ID" \
    '{moderator_id:$moderator,duration_minutes:30,note:"Production v2 automated no-staff drill."}')")"
SHIFT_ID="$(jq -er .shift_id <<<"$shift")"
heartbeat="$(curl -fsS "$ONLINE_URL/v1/production/moderation/shifts/heartbeat" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg shift "$SHIFT_ID" --arg moderator "$MODERATOR_ID" \
    '{shift_id:$shift,moderator_id:$moderator,note:"Queue reviewed; claiming one appeal."}')")"
jq -e '.status == "active" and .open_claims == 0' <<<"$heartbeat" >/dev/null
claim="$(curl -fsS "$ONLINE_URL/v1/production/moderation/claims" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg shift "$SHIFT_ID" --arg moderator "$MODERATOR_ID" \
    --arg appeal "$APPEAL_ID" \
    '{shift_id:$shift,moderator_id:$moderator,case_kind:"appeal",case_id:$appeal}')")"
jq -e '.status == "claimed" and .case_kind == "appeal"' <<<"$claim" >/dev/null
DUPLICATE_CLAIM_STATUS="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$ONLINE_URL/v1/production/moderation/claims" -H "x-trnm-moderator: $MODERATOR_TOKEN" \
  -H 'content-type: application/json' --data-binary "$(jq -cn --arg shift "$SHIFT_ID" \
    --arg moderator "$MODERATOR_ID" --arg appeal "$APPEAL_ID" \
    '{shift_id:$shift,moderator_id:$moderator,case_kind:"appeal",case_id:$appeal}')")"
[[ "$DUPLICATE_CLAIM_STATUS" == "409" ]]
EARLY_CLOSE_STATUS="$(curl -sS -o /dev/null -w '%{http_code}' \
  "$ONLINE_URL/v1/production/moderation/shifts/close" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg shift "$SHIFT_ID" --arg moderator "$MODERATOR_ID" \
    '{shift_id:$shift,moderator_id:$moderator,note:"Must reject unresolved claim."}')")"
[[ "$EARLY_CLOSE_STATUS" == "409" ]]
curl -fsS "$ONLINE_URL/v1/operations/moderation/appeals/resolve" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg appeal "$APPEAL_ID" \
    '{appeal_id:$appeal,decision:"approved",
      resolution:"Production v2 claimed appeal was reviewed and the fixture enforcement revoked."}')" >/dev/null
closed_shift="$(curl -fsS "$ONLINE_URL/v1/production/moderation/shifts/close" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN" -H 'content-type: application/json' \
  --data-binary "$(jq -cn --arg shift "$SHIFT_ID" --arg moderator "$MODERATOR_ID" \
    '{shift_id:$shift,moderator_id:$moderator,note:"Claim resolved; automated drill closed."}')")"
jq -e '.status == "closed" and .open_claims == 0 and .resolved_claims == 1' \
  <<<"$closed_shift" >/dev/null

production_status="$(curl -fsS "$ONLINE_URL/v1/production/status" \
  -H "x-trnm-moderator: $MODERATOR_TOKEN")"
jq -e --arg key "$SIGNER_KEY_ID" \
  '.protocol_version == "trnm_online_production_v2" and .signer_key_id == $key and
   .distributed_admission == true and .current_admission_rejections >= 1 and
   .recent_capacity_samples >= 2 and .signer_provider_kind == "file_seed" and
   .signer_registry_verified == true and .kms_hsm_attested == false and
   .cross_host_failover_attested == false and .public_edge_attested == false' \
  <<<"$production_status" >/dev/null

database="$(cex_psql_stdin -Atc "select json_build_object(
  'admission_windows',(select count(*) from trnm_online_admission_windows
    where window_started_at > now()-interval '10 minutes'),
  'capacity_instances',(select count(distinct instance_id) from trnm_online_capacity_samples
    where instance_id like '$RUN_ID-admission-%'),
  'host_challenges',(select count(*) from trnm_online_host_attestation_audit
    where evidence_hash='$HOST_EVIDENCE_HASH'),
  'shift_status',(select status from trnm_online_moderation_shifts where shift_id='$SHIFT_ID'::uuid),
  'resolved_claims',(select count(*) from trnm_online_moderation_case_claims
    where shift_id='$SHIFT_ID'::uuid and status='resolved'),
  'distinct_hosts',(select count(distinct physical_host_id) from trnm_online_fleet_instances
    where lease_expires_at > now())
)" | jq -c .)"
jq -e '.admission_windows >= 1 and .capacity_instances == 2 and .host_challenges == 1 and
  .shift_status == "closed" and .resolved_claims == 1 and .distinct_hosts == 1' \
  <<<"$database" >/dev/null

jq -n --arg run_id "$RUN_ID" --argjson v1 "$V1_REPORT" --arg match "$MATCH_ID" \
  --arg signer_key "$SIGNER_KEY_ID" --arg spectator "$SPECTATOR" \
  --arg grant "$(jq -er .spectator_grant_id "$EVIDENCE/state.json")" \
  --arg shift "$SHIFT_ID" --arg appeal "$APPEAL_ID" --arg evidence "$EVIDENCE" \
  --argjson status_view "$production_status" --argjson database "$database" \
  '{status:"passed",run_id:$run_id,production_v1_compatibility:$v1,
    match_id:$match,signer_key_id:$signer_key,signer_key_possession_and_registry:true,
    distributed_admission_across_two_instances:true,distributed_limit_status:429,
    capacity_sampling:true,host_challenge_evidence:true,distinct_physical_hosts:1,
    cross_host_failover_claimed:false,native_delayed_spectator:true,spectator_player:$spectator,
    spectator_grant_id:$grant,moderation_shift_claim_resolution:true,shift_id:$shift,
    appeal_id:$appeal,public_edge_attested:false,kms_hsm_attested:false,
    production_status:$status_view,database:$database,evidence:$evidence,
    boundary:"automated local Production v2 evidence; no humans, second host, KMS/HSM, public edge or staffed shift"}' \
  | tee "$EVIDENCE/report.json"
