#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CEX_ROOT="${CEX_PROJECT_ROOT:-$ROOT_DIR/../CEX}"
# shellcheck source=/dev/null
source "$CEX_ROOT/scripts/_dev-helpers.sh"
cex_load_env

LEDGER_URL="${TRNM_CEX_LEDGER_URL:-http://127.0.0.1:7002}"
ONLINE_URL="${TRNM_GAME_SERVER_URL:-http://127.0.0.1:7005}"
ADMIN_TOKEN="${LEDGER_ADMIN_TOKEN:-${IDENTITY_ADMIN_TOKEN:?identity admin token required}}"
RUN_ID="online-native-$(date +%s)-${RANDOM}"
EVIDENCE="$ROOT_DIR/acceptance/online-native/$RUN_ID"
BIN="${TRNM_NATIVE_CLIENT_BIN:-$ROOT_DIR/target/release/trnm-first-contact}"
REQUIRE_CLEAN_EVIDENCE="${TRNM_REQUIRE_CLEAN_NATIVE_EVIDENCE:-0}"
FRAME_CONTRACT="trnm_online_render_frame_timing_v3"
FRAME_WARMUP_SECONDS=5
FRAME_MIN_MEASUREMENT_SECONDS=10
FRAME_MIN_SAMPLES=300
FRAME_MIN_AVERAGE_FPS=60
FRAME_MIN_ONE_PERCENT_LOW_FPS=30
FRAME_MAX_DELTA_MS=100
FRAME_MIN_COVERAGE_RATIO=0.90
FRAME_MAX_COVERAGE_RATIO=1.10
INPUT_ACK_MAX_MS=1000
HOST_PID=""
GUEST_PID=""
XVFB_PID=""
NETEM_APPLIED=0
CHAOS_RTT_MS="${TRNM_NATIVE_CHAOS_RTT_MS:-}"
CHAOS_LOSS_PERCENT="${TRNM_NATIVE_CHAOS_LOSS_PERCENT:-0}"
MATCH_ID=""

case "$REQUIRE_CLEAN_EVIDENCE" in
  0|1) ;;
  *) echo "TRNM_REQUIRE_CLEAN_NATIVE_EVIDENCE must be 0 or 1" >&2; exit 64 ;;
esac
[[ -x "$BIN" ]] || {
  echo "native client binary is unavailable or not executable: $BIN" >&2
  exit 1
}
SOURCE_COMMIT="$(git -C "$ROOT_DIR" rev-parse HEAD)"
SOURCE_TREE="$(git -C "$ROOT_DIR" rev-parse 'HEAD^{tree}')"
SOURCE_DIRTY=false
[[ -z "$(git -C "$ROOT_DIR" status --porcelain)" ]] || SOURCE_DIRTY=true
if [[ "$REQUIRE_CLEAN_EVIDENCE" == 1 && "$SOURCE_DIRTY" == true ]]; then
  echo "formal native evidence requires a clean source tree" >&2
  exit 1
fi
NEWER_SOURCE="$(
  find "$ROOT_DIR/trillionnium/crates" -type f \
    \( -name '*.rs' -o -name Cargo.toml \) -newer "$BIN" -print -quit
)"
for source_file in "$ROOT_DIR/trillionnium/Cargo.toml" \
  "$ROOT_DIR/trillionnium/Cargo.lock"; do
  if [[ "$source_file" -nt "$BIN" ]]; then
    NEWER_SOURCE="$source_file"
    break
  fi
done
if [[ -n "$NEWER_SOURCE" ]]; then
  echo "native client binary is stale relative to source: $NEWER_SOURCE" >&2
  exit 1
fi
CLIENT_BINARY_SHA256="$(sha256sum "$BIN" | awk '{print $1}')"
mkdir -p "$EVIDENCE/host-save" "$EVIDENCE/guest-save" \
  "$EVIDENCE/host-journal" "$EVIDENCE/guest-journal"

process_environment_value() {
  local pid="$1" name="$2" entry
  while IFS= read -r entry; do
    if [[ "$entry" == "$name="* ]]; then
      printf '%s\n' "${entry#*=}"
      return 0
    fi
  done < <(tr '\0' '\n' <"/proc/$pid/environ")
  return 1
}

cleanup() {
  local status=$? cleanup_failed=0 maintenance_report=""
  local maintenance_pid="" maintenance_exe="" maintenance_sha=""
  local maintenance_release="" maintenance_verification=""
  local maintenance_journal="" maintenance_instance="" maintenance_host=""
  local maintenance_identity_valid=0
  [[ -z "$HOST_PID" ]] || kill "$HOST_PID" >/dev/null 2>&1 || true
  [[ -z "$GUEST_PID" ]] || kill "$GUEST_PID" >/dev/null 2>&1 || true
  [[ -z "$XVFB_PID" ]] || kill "$XVFB_PID" >/dev/null 2>&1 || true
  if [[ "$NETEM_APPLIED" == "1" ]]; then
    sudo -n "${TC:-/usr/sbin/tc}" qdisc del dev lo root >/dev/null 2>&1 || true
  fi
  if [[ -n "$MATCH_ID" ]]; then
    maintenance_pid="$(systemctl --user show trnm-game-server.service \
      -p MainPID --value 2>/dev/null || true)"
    if [[ "$maintenance_pid" =~ ^[1-9][0-9]*$ \
        && -r "/proc/$maintenance_pid/stat" ]]; then
      maintenance_exe="$(readlink -e "/proc/$maintenance_pid/exe" 2>/dev/null || true)"
      maintenance_sha="$(sha256sum "/proc/$maintenance_pid/exe" 2>/dev/null \
        | awk '{print $1}')"
      maintenance_release="$(dirname -- "$maintenance_exe")"
      maintenance_verification="$(
        "$ROOT_DIR/scripts/check-trnm-game-server-release.sh" \
          "$maintenance_release" 2>/dev/null || true
      )"
      maintenance_journal="$(process_environment_value "$maintenance_pid" \
        TRNM_PUBLISHED_TICK_JOURNAL_DIR 2>/dev/null || true)"
      maintenance_instance="$(process_environment_value "$maintenance_pid" \
        TRNM_FLEET_INSTANCE_ID 2>/dev/null || true)"
      maintenance_host="$(process_environment_value "$maintenance_pid" \
        TRNM_FLEET_PHYSICAL_HOST_ID 2>/dev/null || true)"
      if [[ -n "$maintenance_exe" && -n "$maintenance_sha" \
          && -n "$maintenance_journal" && "$maintenance_journal" == /* \
          && -n "$maintenance_instance" && -n "$maintenance_host" ]] \
          && jq -e --arg exe "$maintenance_exe" --arg sha "$maintenance_sha" \
            --arg release "$maintenance_release" '
              .verified == true
              and .release_dir == $release
              and .binaries.game_server.path == $exe
              and .binaries.game_server.sha256 == $sha
            ' >/dev/null 2>&1 <<<"$maintenance_verification"; then
        maintenance_identity_valid=1
        jq -n --argjson pid "$maintenance_pid" \
          --arg executable "$maintenance_exe" --arg sha256 "$maintenance_sha" \
          --arg release_dir "$maintenance_release" --arg journal "$maintenance_journal" \
          --arg instance "$maintenance_instance" --arg host "$maintenance_host" '{
            contract_version:"trnm_online_native_maintenance_runtime_identity_v1",
            pid:$pid,executable:$executable,sha256:$sha256,release_dir:$release_dir,
            journal_dir:$journal,instance_id:$instance,physical_host_id:$host
          }' >"$EVIDENCE/maintenance-runtime-identity.json" \
          || maintenance_identity_valid=0
      fi
    fi
    systemctl --user stop trnm-game-server.service >/dev/null 2>&1 \
      || cleanup_failed=1
    if [[ "$(systemctl --user show trnm-game-server.service \
          -p ActiveState --value 2>/dev/null || true)" != inactive \
        || -e "/proc/$maintenance_pid/stat" \
        || "$maintenance_identity_valid" != 1 \
        || "$(sha256sum "$maintenance_exe" 2>/dev/null | awk '{print $1}')" \
          != "$maintenance_sha" ]]; then
      cleanup_failed=1
      maintenance_identity_valid=0
    fi
    maintenance_report="$EVIDENCE/maintenance-fail-close.json"
    if (( maintenance_identity_valid == 0 )); then
      cleanup_failed=1
    elif ! TRNM_GAME_SERVER_RELEASE_DIR="$maintenance_release" \
        TRNM_PUBLISHED_TICK_JOURNAL_DIR="$maintenance_journal" \
        TRNM_FLEET_INSTANCE_ID="$maintenance_instance" \
        TRNM_FLEET_PHYSICAL_HOST_ID="$maintenance_host" \
        TRNM_MAINTENANCE_FAILURE_REASON="native render/network smoke completed without settlement" \
        timeout --foreground --signal=TERM --kill-after=5s 120s \
        "$ROOT_DIR/scripts/run-trnm-game-server.sh" \
        --maintenance-fail-close "$MATCH_ID" \
        >"$maintenance_report" 2>"$EVIDENCE/maintenance-fail-close.log"; then
      cleanup_failed=1
    elif ! jq -e --arg match_id "$MATCH_ID" '
        .contract_version == "trnm_online_maintenance_fail_close_v1"
        and .status == "completed"
        and .match_id == $match_id
        and .selector == "exact_match_id"
        and .transition_atomic == true
        and .legacy_adoption == false
        and .adoption_contract == null
        and (.previous_phase == "waiting" or .previous_phase == "running"
          or .previous_phase == "failed_closed")
        and .final_phase == "failed_closed"
        and (
          if .waiting_db_only then
            .hot_witness_present_before == false
            and .cold_witness_sealed == false
            and .local_marker_state == null
          else
            .cold_witness_sealed == true
            and .local_marker_state == "sealed"
            and (if .previous_phase == "running" then
              .hot_witness_present_before == true
            else
              (.hot_witness_present_before | type) == "boolean"
            end)
          end
        )' "$maintenance_report" >/dev/null 2>&1; then
      cleanup_failed=1
    fi
  fi
  systemctl --user unset-environment TRNM_GAME_SERVER_TICK_MS TRNM_ALLOW_ACCELERATED_TEST_CLOCK >/dev/null 2>&1 || true
  systemctl --user reset-failed trnm-game-server.service >/dev/null 2>&1 || true
  systemctl --user restart trnm-game-server.service >/dev/null 2>&1 \
    || cleanup_failed=1
  if (( status == 0 && cleanup_failed != 0 )); then
    status=1
  fi
  exit "$status"
}
trap cleanup EXIT

admin_post() {
  curl -fsS "$LEDGER_URL$1" -H "x-admin-token: $ADMIN_TOKEN" \
    -H 'content-type: application/json' --data-binary "$2"
}

create_identity() {
  local role="$1" account player credential session invite identity
  player="$RUN_ID-$role"
  credential="credential-$RUN_ID-$role-012345678901234567890123"
  invite="$(admin_post /v1/trnm/product/registration-invites \
    '{"lifetime_seconds":3600,"max_uses":1}' | jq -er .invite_code)"
  identity="$(curl -fsS "$LEDGER_URL/v1/trnm/product/register" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg credential "$credential" --arg invite "$invite" \
      '{player_id:$player,credential:$credential,invite_code:$invite}')")"
  account="$(jq -er .account_id <<<"$identity")"
  session="$(curl -fsS "$LEDGER_URL/v1/trnm/product/login" \
    -H 'content-type: application/json' --data-binary "$(jq -cn \
      --arg player "$player" --arg credential "$credential" --arg device "$RUN_ID-$role-device" \
      '{player_id:$player,credential:$credential,device_id:$device,lifetime_seconds:3600}')" \
    | jq -er .session_token)"
  printf '%s\t%s\t%s\n' "$player" "$account" "$session"
}

player_post() {
  local session="$1" path="$2" body="$3"
  curl -fsS "$ONLINE_URL$path" -H "x-trnm-player-session: $session" \
    -H 'content-type: application/json' --data-binary "$body"
}

window_for_pid() {
  local pid="$1" id actual
  while read -r id; do
    [[ -n "$id" ]] || continue
    actual="$(xprop -id "$id" _NET_WM_PID 2>/dev/null | awk -F' = ' '{print $2}')"
    if [[ "$actual" == "$pid" ]]; then
      printf '%s\n' "$id"
      return 0
    fi
  done < <(xwininfo -root -tree 2>/dev/null | awk \
    '/"Trillionnium — First Contact": \("trnm-first-contact" "trnm-first-contact"\)/ {print $1}')
  return 1
}

wait_for_window() {
  local pid="$1" id=""
  for _ in $(seq 1 90); do
    id="$(window_for_pid "$pid" || true)"
    if [[ -n "$id" ]]; then
      printf '%s\n' "$id"
      return 0
    fi
    sleep 1
  done
  return 1
}

capture() {
  local window_id="$1" output="$2"
  local rendered_pixels
  for _ in $(seq 1 10); do
    if xwd -silent -id "$window_id" 2>/dev/null \
      | xwdtopnm 2>/dev/null | pnmtopng >"$output" \
      && [[ -s "$output" ]]; then
      rendered_pixels="$(pngtopnm "$output" 2>/dev/null \
        | od -An -v -tu1 \
        | awk '{ for (i = 1; i <= NF; i++) if ($i >= 16) count++ } END { print count + 0 }')"
      if (( rendered_pixels >= 5000 )); then
        return 0
      fi
    fi
    sleep 1
  done
  echo "native window $window_id never produced a non-black rendered frame" >&2
  return 1
}

wait_for_frame_timing() {
  local path="$1"
  for _ in $(seq 1 180); do
    if [[ -s "$path" ]] && jq -e '
        .contract_version == "trnm_online_render_frame_timing_v3"
        and .clock == "bevy_time_real"
        and .write_mode == "same_directory_atomic_rename"
        and .measurement_valid == true
        and .frame_count >= .minimum_frame_samples
        and .network_thread_instrumentation.instrumented_io_calls_after_render_start > 0
        and .native_input_to_durable_ack.samples >=
          .native_input_to_durable_ack.minimum_samples
      ' "$path" >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.25
  done
  echo "native client did not produce a complete real-clock frame/input measurement: $path" >&2
  return 1
}

wait_for_frame_measurement_start() {
  local path="$1"
  for _ in $(seq 1 80); do
    if [[ -s "$path" ]] && jq -e '
        .contract_version == "trnm_online_render_frame_timing_v3"
        and .clock == "bevy_time_real"
        and .measurement_elapsed_ms >= 500
      ' "$path" >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.25
  done
  echo "native client did not enter the real-clock measurement window: $path" >&2
  return 1
}

wait_for_bound_input_command() {
  local player="$1" injected_at_ms="$2"
  local row=""
  for _ in $(seq 1 40); do
    row="$(cex_psql_stdin -Atc "select json_build_object(
      'command_id',command_id,
      'sequence',sequence,
      'input_sequence',input_sequence,
      'created_at_unix_ms',floor(extract(epoch from created_at) * 1000)::bigint,
      'source',order_json->>'source',
      'kind',order_json->>'kind',
      'raw_command_label',order_json->>'raw_command_label'
    ) from trnm_online_commands
      where match_id = '$MATCH_ID'::uuid and player_id = '$player'
      order by sequence desc limit 1")"
    if [[ -n "$row" ]] && jq -e '
        (.command_id | startswith("native:"))
        and .input_sequence == 0
        and .source == "local_input"
        and .kind == "move"
        and .raw_command_label == "FIRST_CONTACT:MOVE"
      ' >/dev/null 2>&1 <<<"$row"; then
      jq -cn --argjson row "$row" --argjson injected_at_ms "$injected_at_ms" \
        '$row + {
          injected_at_unix_ms:$injected_at_ms,
          database_commit_after_injection_ms:
            ($row.created_at_unix_ms - $injected_at_ms)
        }'
      return 0
    fi
    sleep 0.25
  done
  echo "native input did not produce one bound durable local-input command for $player" >&2
  return 1
}

netem_packet_count() {
  "$TC" -s qdisc show dev lo | awk '
    $1 == "qdisc" && $2 == "netem" && $3 == "30:" { found = 1; next }
    found && $1 == "Sent" { print $4 + 0; exit }
  '
}

IFS=$'\t' read -r HOST_PLAYER HOST_ACCOUNT HOST_SESSION < <(create_identity host)
IFS=$'\t' read -r GUEST_PLAYER GUEST_ACCOUNT GUEST_SESSION < <(create_identity guest)

systemctl --user reset-failed trnm-game-server.service
systemctl --user set-environment TRNM_GAME_SERVER_TICK_MS=200 \
  TRNM_ALLOW_ACCELERATED_TEST_CLOCK=1
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  curl -fsS "$ONLINE_URL/v1/online/readiness" >/dev/null 2>&1 && break
  sleep 1
done
curl -fsS "$ONLINE_URL/v1/online/readiness" | jq -e '.status == "ok"' >/dev/null

contract="trnm_online_authority_v3"
build="trnm-online-authority-2026.07-v3"
product_contract="trnm_online_product_v2"
product_build="trnm-online-product-2026.07-v2"
campaign="$(player_post "$HOST_SESSION" /v1/online/campaigns/connect "$(jq -cn \
  --arg protocol "$contract" --arg build "$build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" --arg slot "$RUN_ID" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,slot_key:$slot}')")"
guest_campaign="$(player_post "$GUEST_SESSION" /v1/online/campaigns/connect "$(jq -cn \
  --arg protocol "$contract" --arg build "$build" --arg player "$GUEST_PLAYER" \
  --arg account "$GUEST_ACCOUNT" --arg slot "$RUN_ID" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,slot_key:$slot}')")"
lobby="$(player_post "$HOST_SESSION" /v1/product/lobbies "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" --arg campaign "$(jq -er .campaign_id <<<"$campaign")" \
  --arg name "$RUN_ID party" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,campaign_id:$campaign,display_name:$name,map_id:"first_contact"}')")"
LOBBY_ID="$(jq -er .lobby_id <<<"$lobby")"
invite="$(player_post "$HOST_SESSION" "/v1/product/lobbies/$LOBBY_ID/invites" "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" --arg target "$GUEST_PLAYER" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,target_player_id:$target,expected_lobby_revision:0}')")"
lobby="$(player_post "$GUEST_SESSION" /v1/product/lobbies/invites/accept "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$GUEST_PLAYER" \
  --arg account "$GUEST_ACCOUNT" --arg campaign "$(jq -er .campaign_id <<<"$guest_campaign")" \
  --arg token "$(jq -er .invite_token <<<"$invite")" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,campaign_id:$campaign,invite_token:$token}')")"
lobby="$(player_post "$HOST_SESSION" "/v1/product/lobbies/$LOBBY_ID/ready" "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,ready:true,expected_lobby_revision:1}')")"
lobby="$(player_post "$GUEST_SESSION" "/v1/product/lobbies/$LOBBY_ID/ready" "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$GUEST_PLAYER" \
  --arg account "$GUEST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,ready:true,expected_lobby_revision:2}')")"
allocation="$(player_post "$HOST_SESSION" "/v1/product/lobbies/$LOBBY_ID/queue" "$(jq -cn \
  --arg protocol "$product_contract" --arg build "$product_build" --arg player "$HOST_PLAYER" \
  --arg account "$HOST_ACCOUNT" \
  '{protocol_version:$protocol,build_id:$build,player_id:$player,account_id:$account,expected_lobby_revision:3}')")"
MATCH_ID="$(jq -er .match_view.match_id <<<"$allocation")"

if [[ -n "$CHAOS_RTT_MS" && -n "${TRNM_NATIVE_CHAOS_LATENCY_MS:-}" ]]; then
  echo "set only TRNM_NATIVE_CHAOS_RTT_MS; the legacy one-way latency variable is ambiguous" >&2
  exit 64
fi
if [[ -n "${TRNM_NATIVE_CHAOS_LATENCY_MS:-}" ]]; then
  [[ "${TRNM_NATIVE_CHAOS_LATENCY_MS}" =~ ^[0-9]+$ ]] \
    && (( TRNM_NATIVE_CHAOS_LATENCY_MS > 0 ))
  CHAOS_RTT_MS=$(( TRNM_NATIVE_CHAOS_LATENCY_MS * 2 ))
fi
if [[ -n "$CHAOS_RTT_MS" ]]; then
  TC="/usr/sbin/tc"
  [[ "$ONLINE_URL" == "http://127.0.0.1:7005" ]] || {
    echo "native loopback chaos requires the canonical IPv4 authority URL on port 7005" >&2
    exit 64
  }
  jq -en --arg value "$CHAOS_LOSS_PERCENT" \
    '($value | tonumber) >= 0 and ($value | tonumber) <= 100' >/dev/null
  [[ "$CHAOS_RTT_MS" =~ ^[0-9]+$ ]] && (( CHAOS_RTT_MS > 0 ))
  one_way_delay_ms=$(( (CHAOS_RTT_MS + 1) / 2 ))
  sudo -n true
  [[ -x "$TC" ]]
  "$TC" qdisc show dev lo | grep -q '^qdisc noqueue'
  sudo -n "$TC" qdisc add dev lo root handle 1: prio bands 3
  NETEM_APPLIED=1
  sudo -n "$TC" qdisc add dev lo parent 1:3 handle 30: netem \
    delay "${one_way_delay_ms}ms" loss "${CHAOS_LOSS_PERCENT}%"
  sudo -n "$TC" filter add dev lo protocol ip parent 1:0 prio 3 u32 \
    match ip dport 7005 0xffff flowid 1:3
  sudo -n "$TC" filter add dev lo protocol ip parent 1:0 prio 4 u32 \
    match ip sport 7005 0xffff flowid 1:3
fi

export DISPLAY="${TRNM_ONLINE_NATIVE_DISPLAY:-:97}"
Xvfb "$DISPLAY" -screen 0 2560x720x24 -nolisten tcp >"$EVIDENCE/xvfb.log" 2>&1 &
XVFB_PID=$!
for _ in $(seq 1 30); do
  xdpyinfo -display "$DISPLAY" >/dev/null 2>&1 && break
  sleep 1
done
xdpyinfo -display "$DISPLAY" >/dev/null
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY TRNM_CEX_ENTRY_TOKEN

TRNM_CAMPAIGN_SAVE_PATH="$EVIDENCE/host-save/campaign.json" \
TRNM_ONLINE_AUTHORITY_URL="$ONLINE_URL" TRNM_ONLINE_MATCH_ID="$MATCH_ID" \
TRNM_CEX_ACTOR_ID="$HOST_PLAYER" TRNM_CEX_ACCOUNT_ID="$HOST_ACCOUNT" \
TRNM_CEX_PLAYER_SESSION="$HOST_SESSION" \
TRNM_ONLINE_COMMAND_JOURNAL_PATH="$EVIDENCE/host-journal/journal.json" \
TRNM_ONLINE_FRAME_TIMING_PATH="$EVIDENCE/host-frame-timing.json" \
TRNM_ONLINE_FRAME_TIMING_WARMUP_SECONDS="$FRAME_WARMUP_SECONDS" \
TRNM_ONLINE_FRAME_TIMING_MIN_SECONDS="$FRAME_MIN_MEASUREMENT_SECONDS" \
TRNM_ONLINE_FRAME_TIMING_MIN_SAMPLES="$FRAME_MIN_SAMPLES" \
TRNM_ONLINE_MIN_AVERAGE_FPS="$FRAME_MIN_AVERAGE_FPS" \
TRNM_ONLINE_MIN_ONE_PERCENT_LOW_FPS="$FRAME_MIN_ONE_PERCENT_LOW_FPS" \
TRNM_ONLINE_MAX_FRAME_DELTA_MS="$FRAME_MAX_DELTA_MS" \
TRNM_ONLINE_MIN_REAL_TIME_COVERAGE_RATIO="$FRAME_MIN_COVERAGE_RATIO" \
TRNM_ONLINE_MAX_REAL_TIME_COVERAGE_RATIO="$FRAME_MAX_COVERAGE_RATIO" \
TRNM_ONLINE_REQUIRE_NETWORK_THREAD_EVIDENCE=1 \
TRNM_ONLINE_REQUIRE_INPUT_ACK_EVIDENCE=1 \
TRNM_ONLINE_MIN_INPUT_ACK_SAMPLES=1 \
TRNM_ONLINE_MAX_INPUT_ACK_MS="$INPUT_ACK_MAX_MS" \
  "$BIN" >"$EVIDENCE/host.log" 2>&1 &
HOST_PID=$!
HOST_WINDOW="$(wait_for_window "$HOST_PID")"
"$ROOT_DIR/scripts/x11_window_move.py" "$HOST_WINDOW" 0 0
sleep 1

capture "$HOST_WINDOW" "$EVIDENCE/host-attached.png"
wait_for_frame_measurement_start "$EVIDENCE/host-frame-timing.json"
host_input_injected_at_ms="$(date +%s%3N)"
"$ROOT_DIR/scripts/x11_key_inject.py" "$HOST_WINDOW" q
host_input_command="$(wait_for_bound_input_command "$HOST_PLAYER" "$host_input_injected_at_ms")"
jq -e --argjson maximum_ms "$INPUT_ACK_MAX_MS" '
  .database_commit_after_injection_ms >= 0
  and .database_commit_after_injection_ms <= $maximum_ms
' >/dev/null <<<"$host_input_command"
wait_for_frame_timing "$EVIDENCE/host-frame-timing.json"
kill -STOP "$HOST_PID"
host_frame_timing="$(jq -c . "$EVIDENCE/host-frame-timing.json")"
capture "$HOST_WINDOW" "$EVIDENCE/host-command-ack.png"
kill -CONT "$HOST_PID" >/dev/null 2>&1 || true
kill "$HOST_PID" >/dev/null 2>&1 || true
wait "$HOST_PID" 2>/dev/null || true
HOST_PID=""

TRNM_CAMPAIGN_SAVE_PATH="$EVIDENCE/guest-save/campaign.json" \
TRNM_ONLINE_AUTHORITY_URL="$ONLINE_URL" TRNM_ONLINE_MATCH_ID="$MATCH_ID" \
TRNM_CEX_ACTOR_ID="$GUEST_PLAYER" TRNM_CEX_ACCOUNT_ID="$GUEST_ACCOUNT" \
TRNM_CEX_PLAYER_SESSION="$GUEST_SESSION" \
TRNM_ONLINE_COMMAND_JOURNAL_PATH="$EVIDENCE/guest-journal/journal.json" \
TRNM_ONLINE_FRAME_TIMING_PATH="$EVIDENCE/guest-frame-timing.json" \
TRNM_ONLINE_FRAME_TIMING_WARMUP_SECONDS="$FRAME_WARMUP_SECONDS" \
TRNM_ONLINE_FRAME_TIMING_MIN_SECONDS="$FRAME_MIN_MEASUREMENT_SECONDS" \
TRNM_ONLINE_FRAME_TIMING_MIN_SAMPLES="$FRAME_MIN_SAMPLES" \
TRNM_ONLINE_MIN_AVERAGE_FPS="$FRAME_MIN_AVERAGE_FPS" \
TRNM_ONLINE_MIN_ONE_PERCENT_LOW_FPS="$FRAME_MIN_ONE_PERCENT_LOW_FPS" \
TRNM_ONLINE_MAX_FRAME_DELTA_MS="$FRAME_MAX_DELTA_MS" \
TRNM_ONLINE_MIN_REAL_TIME_COVERAGE_RATIO="$FRAME_MIN_COVERAGE_RATIO" \
TRNM_ONLINE_MAX_REAL_TIME_COVERAGE_RATIO="$FRAME_MAX_COVERAGE_RATIO" \
TRNM_ONLINE_REQUIRE_NETWORK_THREAD_EVIDENCE=1 \
TRNM_ONLINE_REQUIRE_INPUT_ACK_EVIDENCE=1 \
TRNM_ONLINE_MIN_INPUT_ACK_SAMPLES=1 \
TRNM_ONLINE_MAX_INPUT_ACK_MS="$INPUT_ACK_MAX_MS" \
  "$BIN" >"$EVIDENCE/guest.log" 2>&1 &
GUEST_PID=$!
GUEST_WINDOW="$(wait_for_window "$GUEST_PID")"
"$ROOT_DIR/scripts/x11_window_move.py" "$GUEST_WINDOW" 0 0
sleep 1
capture "$GUEST_WINDOW" "$EVIDENCE/guest-attached.png"
wait_for_frame_measurement_start "$EVIDENCE/guest-frame-timing.json"
guest_input_injected_at_ms="$(date +%s%3N)"
"$ROOT_DIR/scripts/x11_key_inject.py" "$GUEST_WINDOW" q
guest_input_command="$(wait_for_bound_input_command "$GUEST_PLAYER" "$guest_input_injected_at_ms")"
jq -e --argjson maximum_ms "$INPUT_ACK_MAX_MS" '
  .database_commit_after_injection_ms >= 0
  and .database_commit_after_injection_ms <= $maximum_ms
' >/dev/null <<<"$guest_input_command"
wait_for_frame_timing "$EVIDENCE/guest-frame-timing.json"
kill -STOP "$GUEST_PID"
guest_frame_timing="$(jq -c . "$EVIDENCE/guest-frame-timing.json")"
capture "$GUEST_WINDOW" "$EVIDENCE/guest-command-ack.png"
kill -CONT "$GUEST_PID" >/dev/null 2>&1 || true
kill "$GUEST_PID" >/dev/null 2>&1 || true
wait "$GUEST_PID" 2>/dev/null || true
GUEST_PID=""

jq -e '.contract_version == "trnm_online_render_frame_timing_v3" and
  .clock == "bevy_time_real" and
  .write_mode == "same_directory_atomic_rename" and
  .warmup_seconds == 5 and .minimum_measurement_seconds == 10 and
  .minimum_frame_samples == 300 and
  .targets.minimum_average_fps == 60 and
  .targets.minimum_one_percent_low_fps == 30 and
  .targets.maximum_frame_delta_ms == 100 and
  .targets.minimum_real_time_coverage_ratio == 0.9 and
  .targets.maximum_real_time_coverage_ratio == 1.1 and
  .measurement_valid == true and .frame_count >= .minimum_frame_samples and
  .average_fps >= .targets.minimum_average_fps and
  .one_percent_low_fps >= .targets.minimum_one_percent_low_fps and
  .max_frame_delta_ms <= .targets.maximum_frame_delta_ms and
  .frames_over_100ms == 0 and
  .main_thread_updates_over_100ms == 0 and .max_main_thread_update_ms <= 100 and
  .network_requests_on_render_thread == false and
  .network_thread_instrumentation.required == true and
  .network_thread_instrumentation.instrumented_io_calls_after_render_start > 0 and
  .network_thread_instrumentation.io_calls_on_render_thread == 0 and
  .network_thread_instrumentation.render_update_thread_changes == 0 and
  .network_thread_instrumentation.passed == true and
  .network_command_round_trip.samples >= 1 and
  .native_input_to_durable_ack.required == true and
  .native_input_to_durable_ack.minimum_samples == 1 and
  .native_input_to_durable_ack.maximum_ms == 1000 and
  .native_input_to_durable_ack.samples >= 1 and
  .native_input_to_durable_ack.max_ms <= 1000 and
  .native_input_to_durable_ack.passed == true and
  .network_main_thread_passed == true and .frame_cadence_passed == true and
  .passed == true' \
  >/dev/null <<<"$host_frame_timing"
jq -e '.contract_version == "trnm_online_render_frame_timing_v3" and
  .clock == "bevy_time_real" and
  .write_mode == "same_directory_atomic_rename" and
  .warmup_seconds == 5 and .minimum_measurement_seconds == 10 and
  .minimum_frame_samples == 300 and
  .targets.minimum_average_fps == 60 and
  .targets.minimum_one_percent_low_fps == 30 and
  .targets.maximum_frame_delta_ms == 100 and
  .targets.minimum_real_time_coverage_ratio == 0.9 and
  .targets.maximum_real_time_coverage_ratio == 1.1 and
  .measurement_valid == true and .frame_count >= .minimum_frame_samples and
  .average_fps >= .targets.minimum_average_fps and
  .one_percent_low_fps >= .targets.minimum_one_percent_low_fps and
  .max_frame_delta_ms <= .targets.maximum_frame_delta_ms and
  .frames_over_100ms == 0 and
  .main_thread_updates_over_100ms == 0 and .max_main_thread_update_ms <= 100 and
  .network_requests_on_render_thread == false and
  .network_thread_instrumentation.required == true and
  .network_thread_instrumentation.instrumented_io_calls_after_render_start > 0 and
  .network_thread_instrumentation.io_calls_on_render_thread == 0 and
  .network_thread_instrumentation.render_update_thread_changes == 0 and
  .network_thread_instrumentation.passed == true and
  .network_command_round_trip.samples >= 1 and
  .native_input_to_durable_ack.required == true and
  .native_input_to_durable_ack.minimum_samples == 1 and
  .native_input_to_durable_ack.maximum_ms == 1000 and
  .native_input_to_durable_ack.samples >= 1 and
  .native_input_to_durable_ack.max_ms <= 1000 and
  .native_input_to_durable_ack.passed == true and
  .network_main_thread_passed == true and .frame_cadence_passed == true and
  .passed == true' \
  >/dev/null <<<"$guest_frame_timing"

for journal in "$EVIDENCE/host-journal/journal.json" \
  "$EVIDENCE/guest-journal/journal.json"; do
  for _ in $(seq 1 40); do
    [[ -s "$journal" ]] \
      && jq -e '.pending_exact_attempts | length == 0' "$journal" >/dev/null 2>&1 \
      && break
    sleep 0.25
  done
  [[ -s "$journal" ]]
  [[ "$(stat -c '%a' "$(dirname "$journal")")" == "700" ]]
  [[ "$(stat -c '%a' "$journal")" == "600" ]]
  [[ "$(stat -c '%a' "$(dirname "$journal")/.$(basename "$journal").lock")" == "600" ]]
  jq -e '.contract_version == "trnm_online_command_journal_v1" and
    (.pending_exact_attempts | length == 0) and
    (.rejected_exact_attempts | length == 0)' "$journal" >/dev/null
done
if grep -Fq -- "$HOST_SESSION" "$EVIDENCE/host-journal/journal.json"; then
  echo "host command journal leaked the player session" >&2
  exit 1
fi
if grep -Fq -- "$GUEST_SESSION" "$EVIDENCE/guest-journal/journal.json"; then
  echo "guest command journal leaked the player session" >&2
  exit 1
fi
journal_evidence="$(jq -cn \
  --arg host_directory_mode "$(stat -c '%a' "$EVIDENCE/host-journal")" \
  --arg host_file_mode "$(stat -c '%a' "$EVIDENCE/host-journal/journal.json")" \
  --arg guest_directory_mode "$(stat -c '%a' "$EVIDENCE/guest-journal")" \
  --arg guest_file_mode "$(stat -c '%a' "$EVIDENCE/guest-journal/journal.json")" \
  '{host_directory_mode:$host_directory_mode,host_file_mode:$host_file_mode,
    guest_directory_mode:$guest_directory_mode,guest_file_mode:$guest_file_mode,
    pending_after_ack:0,rejected_after_ack:0,credentials_absent:true}')"

database="$(cex_psql_stdin -Atc "select json_build_object(
  'lobby_status',(select status from trnm_online_lobbies where lobby_id = '$LOBBY_ID'::uuid),
  'allocations',(select count(*) from trnm_online_matchmaking_allocations where lobby_id = '$LOBBY_ID'::uuid and match_id = '$MATCH_ID'::uuid),
  'members',(select count(*) from trnm_online_match_members where match_id = '$MATCH_ID'::uuid),
  'host_commands',(select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and player_id = '$HOST_PLAYER'),
  'guest_commands',(select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and player_id = '$GUEST_PLAYER'),
  'fingerprinted_commands',(select count(*) from trnm_online_commands where match_id = '$MATCH_ID'::uuid and length(request_hash) = 64),
  'distinct_control_sets',(select count(distinct controlled_unit_ids) from trnm_online_match_members where match_id = '$MATCH_ID'::uuid)
)" | jq -c .)"
jq -e '.lobby_status == "matched" and .allocations == 1 and
  .members == 2 and .host_commands == 1 and .guest_commands == 1 and
  .fingerprinted_commands == (.host_commands + .guest_commands) and
  .distinct_control_sets == 2' <<<"$database" >/dev/null

NETEM_PACKETS=0
if [[ "$NETEM_APPLIED" == 1 ]]; then
  NETEM_PACKETS="$(netem_packet_count)"
  if [[ ! "$NETEM_PACKETS" =~ ^[0-9]+$ ]] || (( NETEM_PACKETS <= 0 )); then
    echo "loopback netem did not observe authority traffic" >&2
    exit 1
  fi
fi

jq -n --arg run_id "$RUN_ID" --arg match_id "$MATCH_ID" --arg evidence "$EVIDENCE" \
  --arg host_window "$HOST_WINDOW" --arg guest_window "$GUEST_WINDOW" \
  --arg authority_protocol "$contract" --arg authority_build "$build" \
  --arg product_protocol "$product_contract" --arg product_build "$product_build" \
  --arg frame_contract "$FRAME_CONTRACT" \
  --arg chaos_rtt_ms "$CHAOS_RTT_MS" --arg chaos_loss_percent "$CHAOS_LOSS_PERCENT" \
  --argjson netem_packets "$NETEM_PACKETS" \
  --arg client_binary "$BIN" --arg client_binary_sha256 "$CLIENT_BINARY_SHA256" \
  --arg source_commit "$SOURCE_COMMIT" --arg source_tree "$SOURCE_TREE" \
  --argjson source_dirty "$SOURCE_DIRTY" \
  --argjson clean_evidence_required "$REQUIRE_CLEAN_EVIDENCE" \
  --argjson database "$database" --argjson host_frame_timing "$host_frame_timing" \
  --argjson guest_frame_timing "$guest_frame_timing" \
  --argjson host_input_command "$host_input_command" \
  --argjson guest_input_command "$guest_input_command" \
  --argjson journal_evidence "$journal_evidence" \
  '{status:"passed",run_id:$run_id,match_id:$match_id,evidence:$evidence,
    authority_protocol:$authority_protocol,authority_build:$authority_build,
    product_protocol:$product_protocol,product_build:$product_build,
    frame_contract:$frame_contract,
    client_binary:{path:$client_binary,sha256:$client_binary_sha256,
      source_commit:$source_commit,source_tree:$source_tree,source_dirty:$source_dirty,
      clean_evidence_required:($clean_evidence_required == 1),build_fresh:true},
    network_chaos:{
      configured:($chaos_rtt_ms | length > 0),
      rtt_ms:($chaos_rtt_ms | if length == 0 then 0 else tonumber end),
      one_way_delay_ms:($chaos_rtt_ms | if length == 0 then 0 else ((tonumber + 1) / 2 | floor) end),
      loss_percent:($chaos_loss_percent|tonumber),
      matched_transport:"ipv4_loopback_tcp_7005",
      netem_packets:$netem_packets
    },
    native_x11_clients:2,distinct_windows:($host_window != $guest_window),
    client_execution_model:"sequential_on_single_evidence_host_models_separate_player_devices",
    closed_alpha_product_lobby_flow:true,
    server_authoritative_commands:true,database:$database,
    native_input_database:{host:$host_input_command,guest:$guest_input_command,
      maximum_commit_after_injection_ms:1000},
    durable_command_journal:$journal_evidence,
    host_frame_timing:$host_frame_timing,guest_frame_timing:$guest_frame_timing,
    boundary:"release-bound real-clock 60-average/30-one-percent-low native input-to-durable-ACK and instrumented worker-network laboratory evidence on one host; not a human multiplayer, public-network, GPU-fleet, or regional result"}' \
  | tee "$EVIDENCE/report.json"
