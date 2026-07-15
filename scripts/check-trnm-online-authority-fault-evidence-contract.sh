#!/usr/bin/env bash
set -euo pipefail
umask 077

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
TMP_DIR="$(mktemp -d)"

cleanup() {
  local pid
  if [[ -f "$TMP_DIR/state/prod.pid" ]]; then
    pid="$(<"$TMP_DIR/state/prod.pid")"
    kill "$pid" >/dev/null 2>&1 || true
  fi
  if [[ "${TRNM_FAULT_CONTRACT_KEEP_TMP:-0}" == 1 ]]; then
    echo "preserved fault contract fixture: $TMP_DIR" >&2
    return
  fi
  chmod -R u+w "$TMP_DIR" 2>/dev/null || true
  rm -rf -- "$TMP_DIR"
}
trap cleanup EXIT

fail() {
  echo "TRNM Authority fault-evidence contract test failed: $*" >&2
  exit 1
}

expect_failure() {
  local description="$1"
  shift
  if "$@" >"$TMP_DIR/unexpected-success.out" 2>&1; then
    fail "$description unexpectedly succeeded"
  fi
}

expect_failure "contract mode outside isolated fixture" \
  env TRNM_FAULT_HARNESS_CONTRACT_MODE=1 \
  "$ROOT_DIR/scripts/check-trnm-online-authority-fault-evidence.sh" pg-rtt100

repo="$TMP_DIR/Trillionnium"
cex="$TMP_DIR/CEX"
home="$TMP_DIR/home"
fake_bin="$TMP_DIR/fake-bin"
state="$TMP_DIR/state"
fixture_physical_host="host-$(sha256sum /etc/machine-id | cut -c1-24)"
mkdir -p "$repo/scripts" "$repo/deploy/systemd" "$repo/assets" \
  "$cex/scripts" "$home/.config/systemd/user" "$fake_bin" "$state"

cp "$ROOT_DIR/scripts/check-trnm-online-authority-fault-evidence.sh" "$repo/scripts/"
cp "$ROOT_DIR/deploy/systemd/trnm-game-server.service" "$repo/deploy/systemd/"
cp "$ROOT_DIR/deploy/systemd/trnm-game-server.service" \
  "$home/.config/systemd/user/trnm-game-server.service"
chmod 0755 "$repo/scripts/check-trnm-online-authority-fault-evidence.sh"

cat >"$repo/.gitignore" <<'EOF'
/run/
EOF
printf '%s\n' 'trnm-online-authority-fault-contract-fixture-v1' \
  >"$repo/.trnm-fault-contract-fixture"

cat >"$repo/scripts/check-trnm-game-server-release.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
release="$(realpath -e -- "${1:-$root/run/releases/trnm-game-server/current}")"
commit="$(git -C "$root" rev-parse HEAD)"
tree="$(git -C "$root" rev-parse 'HEAD^{tree}')"
game="$release/trnm-game-server"
e2e="$release/trnm-online-e2e"
game_sha="$(sha256sum "$game" | awk '{print $1}')"
if [[ "${TRNM_FAULT_CHECKER_LEGACY:-0}" == 1 ]]; then
  jq -n --arg release "$release" --arg commit "$commit" --arg tree "$tree" \
    --arg sha "$game_sha" \
    '{contract_version:"trnm_game_server_release_verification_v1",verified:true,
      release_dir:$release,release_contract_version:"trnm_game_server_release_v1",
      fault_harness_capable:false,git_commit:$commit,git_tree:$tree,binary_sha256:$sha,
      binaries:{game_server:{path:($release+"/trnm-game-server"),sha256:$sha},online_e2e:null}}'
  exit 0
fi
e2e_sha="$(sha256sum "$e2e" | awk '{print $1}')"
manifest_sha="$(sha256sum "$release/release-manifest.json" | awk '{print $1}')"
jq -n --arg release "$release" --arg commit "$commit" --arg tree "$tree" \
  --arg game "$game" --arg e2e "$e2e" --arg game_sha "$game_sha" --arg e2e_sha "$e2e_sha" \
  --arg manifest_sha "$manifest_sha" \
  '{contract_version:"trnm_game_server_release_verification_v1",verified:true,
    release_dir:$release,release_contract_version:"trnm_game_server_release_v2",
    fault_harness_capable:true,isolated_target:true,trusted_target_cache_used:false,
    release_manifest_sha256:$manifest_sha,
    git_commit:$commit,git_tree:$tree,binary_sha256:$game_sha,
    binaries:{game_server:{path:$game,sha256:$game_sha},online_e2e:{path:$e2e,sha256:$e2e_sha}}}'
EOF
chmod 0755 "$repo/scripts/check-trnm-game-server-release.sh"

cat >"$cex/scripts/_dev-helpers.sh" <<'EOF'
cex_load_env() {
  export IDENTITY_ADMIN_TOKEN="fixture-admin-secret"
}
cex_effective_database_url() {
  printf '%s\n' 'postgresql://fixture:fixture-db-secret@127.0.0.1:5432/fixture'
}
cex_psql_stdin() {
  local command="$*"
  if [[ "$command" == *"select count(*) from trnm_online_matches where phase = 'running'"* ]]; then
    printf '%s\n' 0
  elif [[ "$command" == *"trnm_authority_terminal_database_evidence_v2"* ]]; then
    jq -cn --arg instance "${TRNM_FAULT_EXPECTED_INSTANCE_ID:?}" \
      --arg host "${TRNM_FAULT_EXPECTED_PHYSICAL_HOST_ID:?}" '{
      contract_version:"trnm_authority_terminal_database_evidence_v2",match_count:1,
      match_id:"11111111-1111-1111-a111-111111111111",phase:"complete",settlement_state:"settled",
      authoritative_tick:3000,next_sequence:54,checkpoint_sequence:54,match_revision:55,
      snapshot_hash:"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      result_hash:"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
      assigned_instance_id:$instance,assigned_instance_epoch:1,assigned_physical_host_id:$host,
      terminal_publication_actor_generation:"44444444-4444-4444-a444-444444444444",
      command_count:54,command_sequences_contiguous:true,player_input_sequences_unique:true,
      missing_post_simulation:0,member_cursors_exact:true,member_cursors:{host:53,guest:1},
      terminal_marker_count:1,terminal_marker_exact:true,
      ack_actor_generation:"44444444-4444-4444-a444-444444444444",
      ack_instance_id:$instance,ack_actor_epoch:1,ack_physical_host_id:$host,
      ack_authoritative_tick:3000,ack_next_sequence:54,ack_match_revision:55,
      ack_next_input_sequences:{host:53,guest:1},
      ack_snapshot_hash:"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      ack_phase:"complete",
      ack_result_hash:"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
      ack_settlement_state:"settled",acknowledged_at_unix_ms:1700000000000,
      database_system_identifier:"72623859790382856",database_timeline_id:7,
      database_current_wal_lsn:"0/16B6D00"}'
  elif [[ "$command" == *"from trnm_online_fleet_instances where instance_id="* ]]; then
    jq -cn --arg instance "${TRNM_FAULT_EXPECTED_INSTANCE_ID:?}" \
      '{instance_id:$instance,instance_epoch:1,physical_host_id:"fixture-host",status:"offline",
        active_matches:0,open_run_match_count:0,lease_expires_at:"2026-07-14T00:00:00Z"}'
  else
    printf '%s\n' 'UPDATE 1'
  fi
}
EOF

git -C "$cex" init -q
git -C "$cex" config user.email fault-contract@example.invalid
git -C "$cex" config user.name fault-contract
git -C "$cex" add .
git -C "$cex" commit -qm "fault contract CEX fixture"

cat >"$fake_bin/systemctl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
state="${TRNM_FAULT_FAKE_STATE:?}"
[[ "${1:-}" == --user ]] && shift
action="${1:-}"
case "$action" in
  show)
    shift 2
    [[ "${1:-}" == -p ]] || exit 64
    property="${2:-}"
    active="$(<"$state/service")"
    case "$property" in
      ActiveState) printf '%s\n' "$active" ;;
      SubState) [[ "$active" == active ]] && echo running || echo dead ;;
      MainPID) [[ "$active" == active ]] && echo "${TRNM_FAULT_FAKE_MAIN_PID:-1}" || echo 0 ;;
      UnitFileState) echo enabled ;;
      FragmentPath) printf '%s\n' "$HOME/.config/systemd/user/trnm-game-server.service" ;;
      *) exit 64 ;;
    esac
    ;;
  stop) printf '%s\n' inactive >"$state/service" ;;
  start) printf '%s\n' active >"$state/service" ;;
  *) exit 64 ;;
esac
EOF

cat >"$fake_bin/ss" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
state="${TRNM_FAULT_FAKE_STATE:?}"
args="$*"
if [[ "$args" == *":7005"* && "$(<"$state/service")" == active ]]; then
  echo 'LISTEN 0 4096 127.0.0.1:7005 0.0.0.0:*'
elif [[ "$args" == *":7006"* && -f "$state/standalone" ]]; then
  pid="$(<"$state/standalone.pid")"
  echo "LISTEN 0 4096 127.0.0.1:7006 0.0.0.0:* users:((\"fixture-server\",pid=$pid,fd=3))"
elif [[ "$args" == *":7543"* && -f "$state/proxy" ]]; then
  pid="$(<"$state/proxy.pid")"
  echo "LISTEN 0 4096 127.0.0.1:7543 0.0.0.0:* users:((\"fixture-proxy\",pid=$pid,fd=3))"
fi
EOF

cat >"$fake_bin/pgrep" <<'EOF'
#!/usr/bin/env bash
exit 1
EOF

cat >"$fake_bin/sudo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
[[ "${1:-}" == -n ]] && shift
[[ "${1:-}" == true ]] && exit 0
exec "$@"
EOF

cat >"$fake_bin/tc" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
state="${TRNM_FAULT_FAKE_STATE:?}"
tc_state="$state/qdisc"
stats="$state/tc-stats"
[[ -f "$tc_state" ]] || echo noqueue >"$tc_state"
[[ -f "$stats" ]] || echo 0 >"$stats"
with_stats=0
if [[ "${1:-}" == -s ]]; then with_stats=1; shift; fi
kind="${1:-}" action="${2:-}"
if [[ "$kind" == qdisc && "$action" == show ]]; then
  if [[ "$(<"$tc_state")" == noqueue ]]; then
    echo 'qdisc noqueue 0: root refcnt 2'
  else
    echo 'qdisc prio 1: root refcnt 2 bands 3 priomap 1 2 2 2 1 2 0 0 1 1 1 1 1 1 1 1'
    if (( with_stats == 1 )); then echo ' Sent 100 bytes 1 pkt (dropped 0, overlimits 0 requeues 0)'; fi
    echo 'qdisc netem 30: parent 1:3 limit 1000 delay 50ms'
    if (( with_stats == 1 )); then
      count="$(<"$stats")"
      echo " Sent 1000 bytes $count pkt (dropped 0, overlimits 0 requeues 0)"
      echo $((count + 100)) >"$stats"
    fi
  fi
elif [[ "$kind" == qdisc && "$action" == add ]]; then
  echo configured >"$tc_state"
elif [[ "$kind" == qdisc && "$action" == del ]]; then
  [[ "${TRNM_FAKE_TC_DELETE_FAIL:-0}" != 1 ]] || exit 1
  echo noqueue >"$tc_state"
elif [[ "$kind" == filter && "$action" == add ]]; then
  :
elif [[ "$kind" == filter && "$action" == show ]]; then
  [[ "$(<"$tc_state")" == configured ]] && {
    echo 'filter parent 1: protocol ip pref 30 u32 chain 0 flowid 1:3 dport 7543'
    echo 'filter parent 1: protocol ip pref 31 u32 chain 0 flowid 1:3 sport 7543'
  }
else
  exit 64
fi
EOF

cat >"$fake_bin/socat" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
state="${TRNM_FAULT_FAKE_STATE:?}"
touch "$state/proxy"
printf '%s\n' "$BASHPID" >"$state/proxy.pid"
trap 'rm -f "$state/proxy" "$state/proxy.pid"; exit 0' TERM INT HUP EXIT
while :; do sleep 1; done
EOF

cat >"$fake_bin/python3" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
state="${TRNM_FAULT_FAKE_STATE:-}"
if [[ "$*" == *"trnm-fault-proxy"* ]]; then
  [[ -n "$state" ]] || exit 64
  touch "$state/proxy"
  printf '%s\n' "$BASHPID" >"$state/proxy.pid"
  trap 'rm -f "$state/proxy" "$state/proxy.pid"; exit 0' TERM INT HUP EXIT
  while :; do sleep 1; done
fi
exec /usr/bin/python3 "$@"
EOF

cat >"$fake_bin/curl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
url=""
for arg in "$@"; do [[ "$arg" == http://* || "$arg" == https://* ]] && url="$arg"; done
case "$url" in
  *:7002/v1/trnm/economy/readiness)
    echo '{"status":"ok","postgres_healthy":true}' ;;
  *:7010/v1/signer/readiness)
    echo '{"status":"ok","postgres_receipts":true}' ;;
  */v1/online/readiness)
    jq -cn --arg instance "${TRNM_FAULT_EXPECTED_INSTANCE_ID:-production-fixture}" '{
      status:"ok",fleet_instance_id:$instance,active_matches:0,active_match_actors:0,
      authority_clock_operational:true,authority_clock_drift_ticks:0.1,
      match_actor_clocks_operational:true,max_actor_clock_abs_drift_ticks:0.2,
      published_tick_journal_operational:true,
      published_tick_terminal_orphan_recovery_operational:true}' ;;
  */v1/accounts)
    echo '{"account_id":"22222222-2222-4222-a222-222222222222"}' ;;
  */v1/trnm/identity/register)
    echo '{}' ;;
  */v1/trnm/identity/session)
    echo '{"session_token":"fixture-session-token"}' ;;
  *) echo '{}' ;;
esac
EOF
chmod 0755 "$fake_bin/"*

git -C "$repo" init -q
git -C "$repo" config user.email fault-contract@example.invalid
git -C "$repo" config user.name fault-contract
git -C "$repo" add .
git -C "$repo" commit -qm "fault harness contract fixture"
commit="$(git -C "$repo" rev-parse HEAD)"
tree="$(git -C "$repo" rev-parse 'HEAD^{tree}')"
fixture_toolchain_identity="$(printf '%s' fault-contract-toolchain | sha256sum | cut -c1-12)"
release="$repo/run/releases/trnm-game-server/${commit:0:12}-${tree:0:12}-${fixture_toolchain_identity}"
mkdir -p "$release" "$repo/run/trnm-game-server/published-ticks"
chmod 0700 "$repo/run/trnm-game-server/published-ticks"
jq -cn --arg host "$fixture_physical_host" '{
  contract_version:"trnm_published_tick_journal_owner_v1",
  journal_owner_id:"33333333-3333-4333-a333-333333333333",
  physical_host_id:$host
}' >"$repo/run/trnm-game-server/published-ticks/.published-tick-owner.json"
jq -cn --arg host "$fixture_physical_host" '{
  contract_version:"trnm_published_tick_ack_manifest_v1",
  journal_owner_id:"33333333-3333-4333-a333-333333333333",
  physical_host_id:$host,tombstone_count:0,committed_seal_sequence:0,
  database_system_identifier:null,database_timeline_id:null,
  latest_tombstone:null,latest_tombstone_sha256:null
}' >"$repo/run/trnm-game-server/published-ticks/.published-tick-ack-manifest.json"
: >"$repo/run/trnm-game-server/published-ticks/.published-tick.lock"
chmod 0600 \
  "$repo/run/trnm-game-server/published-ticks/.published-tick-owner.json" \
  "$repo/run/trnm-game-server/published-ticks/.published-tick-ack-manifest.json" \
  "$repo/run/trnm-game-server/published-ticks/.published-tick.lock"

cat >"$release/trnm-game-server" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
state="${TRNM_FAULT_FAKE_STATE:?}"
exec {lock_fd}>>"${TRNM_PUBLISHED_TICK_JOURNAL_DIR:?}/.published-tick.lock"
flock -n "$lock_fd"
touch "$state/standalone"
printf '%s\n' "$BASHPID" >"$state/standalone.pid"
match_id="11111111-1111-1111-a111-111111111111"
hot="$TRNM_PUBLISHED_TICK_JOURNAL_DIR/published-$match_id.json"
cold_dir="$TRNM_PUBLISHED_TICK_JOURNAL_DIR/acknowledged/11/11"
cold="$cold_dir/acknowledged-$match_id.json"
mkdir -p -- "$cold_dir"
chmod 0700 \
  "$TRNM_PUBLISHED_TICK_JOURNAL_DIR/acknowledged" \
  "$TRNM_PUBLISHED_TICK_JOURNAL_DIR/acknowledged/11" \
  "$cold_dir"
rm -f -- "$hot" "$cold"
if [[ "${TRNM_FAULT_FAKE_JOURNAL_MODE:-cold}" == hot-only ]]; then
  cat >"$hot" <<JSON
{"contract_version":"trnm_published_tick_high_water_v2","journal_owner_id":"33333333-3333-4333-a333-333333333333","instance_id":"$TRNM_FLEET_INSTANCE_ID","physical_host_id":"$TRNM_FLEET_PHYSICAL_HOST_ID","match_id":"$match_id","actor_generation":"44444444-4444-4444-a444-444444444444","actor_epoch":1,"tick":3000,"next_sequence":54,"match_revision":55,"next_input_sequences":{"host":53,"guest":1},"phase":"complete","receipts_replayable":true,"snapshot_hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","recorded_at_unix_ms":1}
JSON
  chmod 0600 "$hot"
else
  result_hash="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
  database_system_identifier="72623859790382856"
  case "${TRNM_FAULT_FAKE_TOMBSTONE_TAMPER:-none}" in
    none) ;;
    lineage) database_system_identifier="72623859790382857" ;;
    result) result_hash="cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc" ;;
    *) exit 64 ;;
  esac
  cat >"$cold" <<JSON
{"contract_version":"trnm_published_tick_ack_tombstone_v2","journal_seal_sequence":1,"high_water":{"contract_version":"trnm_published_tick_high_water_v2","journal_owner_id":"33333333-3333-4333-a333-333333333333","instance_id":"$TRNM_FLEET_INSTANCE_ID","physical_host_id":"$TRNM_FLEET_PHYSICAL_HOST_ID","match_id":"$match_id","actor_generation":"44444444-4444-4444-a444-444444444444","actor_epoch":1,"tick":3000,"next_sequence":54,"match_revision":55,"next_input_sequences":{"host":53,"guest":1},"phase":"complete","receipts_replayable":true,"snapshot_hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","recorded_at_unix_ms":1},"result_hash":"$result_hash","settlement_state":"settled","acknowledged_at_unix_ms":1700000000000,"database_system_identifier":"$database_system_identifier","database_timeline_id":7,"database_wal_lsn":"0/16B6C50"}
JSON
  chmod 0600 "$cold"
  tombstone_sha="$(sha256sum "$cold" | awk '{print $1}')"
  jq -cn --arg host "$TRNM_FLEET_PHYSICAL_HOST_ID" \
    --arg system "$database_system_identifier" --arg sha "$tombstone_sha" \
    --slurpfile tombstone "$cold" '{
      contract_version:"trnm_published_tick_ack_manifest_v1",
      journal_owner_id:"33333333-3333-4333-a333-333333333333",
      physical_host_id:$host,tombstone_count:1,committed_seal_sequence:1,
      database_system_identifier:$system,database_timeline_id:7,
      latest_tombstone:$tombstone[0],latest_tombstone_sha256:$sha
    }' >"$TRNM_PUBLISHED_TICK_JOURNAL_DIR/.published-tick-ack-manifest.json"
  chmod 0600 "$TRNM_PUBLISHED_TICK_JOURNAL_DIR/.published-tick-ack-manifest.json"
fi
trap 'rm -f "$state/standalone" "$state/standalone.pid"; exit 0' TERM INT HUP EXIT
while :; do sleep 1; done
EOF
cat >"$release/trnm-online-e2e" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
sleep 1.2
acks='[100,200,300]'
[[ "${TRNM_FAULT_FAKE_EMPTY_ACK:-0}" != 1 ]] || acks='[]'
jq -n --argjson acks "$acks" \
  --argjson effects '[200,200,200,200,200,200,200,200,200,200,200,200,200,200,200,200,200,200,200,200]' '{
  status:"passed",run_id:"fixture-e2e",match_id:"11111111-1111-1111-a111-111111111111",
  websocket_authoritative_effect_samples_ms:$effects,websocket_authoritative_effect_p95_ms:200,
  command_ack_ms:$acks,match_tick_drift:0.5}'
EOF
chmod 0555 "$release/trnm-game-server" "$release/trnm-online-e2e"
printf '%s\n' '{"contract_version":"trnm_game_server_release_v2"}' >"$release/release-manifest.json"
ln -s "$(basename "$release")" "$repo/run/releases/trnm-game-server/current"

echo active >"$state/service"
echo noqueue >"$state/qdisc"
echo 0 >"$state/tc-stats"

run_harness() {
  env PATH="$fake_bin:$PATH" HOME="$home" CEX_PROJECT_ROOT="$cex" \
    TRNM_FAULT_FAKE_STATE="$state" TRNM_FAULT_HARNESS_CONTRACT_MODE=1 \
    "$repo/scripts/check-trnm-online-authority-fault-evidence.sh" "$@"
}

clear_fixture_match_journal() {
  rm -f -- \
    "$repo/run/trnm-game-server/published-ticks/published-11111111-1111-1111-a111-111111111111.json" \
    "$repo/run/trnm-game-server/published-ticks/acknowledged/11/11/acknowledged-11111111-1111-1111-a111-111111111111.json"
  jq -cn --arg host "$fixture_physical_host" '{
    contract_version:"trnm_published_tick_ack_manifest_v1",
    journal_owner_id:"33333333-3333-4333-a333-333333333333",
    physical_host_id:$host,tombstone_count:0,committed_seal_sequence:0,
    database_system_identifier:null,database_timeline_id:null,
    latest_tombstone:null,latest_tombstone_sha256:null
  }' >"$repo/run/trnm-game-server/published-ticks/.published-tick-ack-manifest.json"
  chmod 0600 "$repo/run/trnm-game-server/published-ticks/.published-tick-ack-manifest.json"
}

expect_failure "arbitrary fault profile" run_harness arbitrary-profile
expect_failure "legacy release without bundled E2E" env TRNM_FAULT_CHECKER_LEGACY=1 \
  PATH="$fake_bin:$PATH" HOME="$home" CEX_PROJECT_ROOT="$cex" \
  TRNM_FAULT_FAKE_STATE="$state" TRNM_FAULT_HARNESS_CONTRACT_MODE=1 \
  "$repo/scripts/check-trnm-online-authority-fault-evidence.sh" pg-rtt100

touch "$repo/untracked-dirty-file"
expect_failure "dirty Trillionnium worktree" run_harness pg-rtt100
rm -f "$repo/untracked-dirty-file"

mkdir -p "$repo/run/locks"
rm -f "$repo/run/locks/trnm-game-server-deploy.lock"
ln -s "$TMP_DIR/dangling-lock-target" "$repo/run/locks/trnm-game-server-deploy.lock"
expect_failure "dangling deployment lock symlink" run_harness pg-rtt100
[[ ! -e "$TMP_DIR/dangling-lock-target" ]] \
  || fail "dangling deployment lock check created or modified its target"
rm -f "$repo/run/locks/trnm-game-server-deploy.lock"

: >"$repo/run/locks/trnm-game-server-deploy.lock"
chmod 0600 "$repo/run/locks/trnm-game-server-deploy.lock"
ln "$repo/run/locks/trnm-game-server-deploy.lock" "$repo/run/locks/deploy-hardlink"
expect_failure "hard-linked deployment lock" run_harness pg-rtt100
rm -f "$repo/run/locks/deploy-hardlink" "$repo/run/locks/trnm-game-server-deploy.lock"

printf '%s\n' 'lock-content-must-survive' >"$repo/run/locks/trnm-game-server-deploy.lock"
chmod 0600 "$repo/run/locks/trnm-game-server-deploy.lock"
exec 7>>"$repo/run/locks/trnm-game-server-deploy.lock"
flock -n 7 || fail "could not hold deployment lock for contract test"
expect_failure "held canonical deployment lock" run_harness pg-rtt100
[[ "$(<"$state/service")" == active && "$(<"$state/qdisc")" == noqueue ]] \
  || fail "lock contention mutated service or qdisc state"
flock -u 7
exec 7>&-

run_harness pg-rtt100 >/dev/null
[[ "$(<"$repo/run/locks/trnm-game-server-deploy.lock")" == lock-content-must-survive ]] \
  || fail "deployment lock was truncated"
success_dir="$(find "$repo/run/online-faults" -mindepth 1 -maxdepth 1 -type d | sort | tail -n1)"
jq -e '.contract_version=="trnm_online_authority_fault_decision_v2"
  and .status=="contract_test_passed" and .passed==false and .contract_test_passed==true
  and .local_only==true and .public_launch_credit==false
  and (.checks|to_entries|all(.value==true))' "$success_dir/decision.json" >/dev/null \
  || fail "mocked success did not produce a non-credit contract decision"
jq -e '.cleanup_failed==0 and .qdisc_default_noqueue==true
  and .restored_active_state=="active"' "$success_dir/cleanup.json" >/dev/null \
  || fail "mocked success did not restore service/qdisc state"
[[ "$(<"$state/qdisc")" == noqueue && "$(<"$state/service")" == active ]] \
  || fail "mocked success left external state changed"
if grep -R -F -e fixture-admin-secret -e fixture-db-secret -e fixture-session-token \
    "$success_dir" >/dev/null 2>&1; then
  fail "mocked evidence persisted a secret"
fi
clear_fixture_match_journal

sleep 1
expect_failure "legacy hot-only terminal journal evidence" env \
  TRNM_FAULT_FAKE_JOURNAL_MODE=hot-only \
  PATH="$fake_bin:$PATH" HOME="$home" CEX_PROJECT_ROOT="$cex" \
  TRNM_FAULT_FAKE_STATE="$state" TRNM_FAULT_HARNESS_CONTRACT_MODE=1 \
  "$repo/scripts/check-trnm-online-authority-fault-evidence.sh" pg-rtt100
hot_only_dir="$(find "$repo/run/online-faults" -mindepth 1 -maxdepth 1 -type d | sort | tail -n1)"
jq -e '.status=="failed" and .checks.terminal_journal_cold_ack_tombstone==false' \
  "$hot_only_dir/decision.json" >/dev/null \
  || fail "legacy hot-only evidence was accepted as final PITR evidence"
[[ -f "$repo/run/trnm-game-server/published-ticks/published-11111111-1111-1111-a111-111111111111.json" \
  && ! -e "$repo/run/trnm-game-server/published-ticks/acknowledged/11/11/acknowledged-11111111-1111-1111-a111-111111111111.json" ]] \
  || fail "hot-only negative fixture did not produce the intended journal state"
clear_fixture_match_journal

sleep 1
expect_failure "tampered ACK tombstone database lineage" env \
  TRNM_FAULT_FAKE_TOMBSTONE_TAMPER=lineage \
  PATH="$fake_bin:$PATH" HOME="$home" CEX_PROJECT_ROOT="$cex" \
  TRNM_FAULT_FAKE_STATE="$state" TRNM_FAULT_HARNESS_CONTRACT_MODE=1 \
  "$repo/scripts/check-trnm-online-authority-fault-evidence.sh" pg-rtt100
lineage_dir="$(find "$repo/run/online-faults" -mindepth 1 -maxdepth 1 -type d | sort | tail -n1)"
jq -e '.status=="failed" and .checks.terminal_journal_cold_ack_tombstone==false' \
  "$lineage_dir/decision.json" >/dev/null \
  || fail "tampered ACK tombstone lineage did not fail closed"
clear_fixture_match_journal

sleep 1
expect_failure "tampered ACK tombstone result" env \
  TRNM_FAULT_FAKE_TOMBSTONE_TAMPER=result \
  PATH="$fake_bin:$PATH" HOME="$home" CEX_PROJECT_ROOT="$cex" \
  TRNM_FAULT_FAKE_STATE="$state" TRNM_FAULT_HARNESS_CONTRACT_MODE=1 \
  "$repo/scripts/check-trnm-online-authority-fault-evidence.sh" pg-rtt100
result_dir="$(find "$repo/run/online-faults" -mindepth 1 -maxdepth 1 -type d | sort | tail -n1)"
jq -e '.status=="failed" and .checks.terminal_journal_cold_ack_tombstone==false' \
  "$result_dir/decision.json" >/dev/null \
  || fail "tampered ACK tombstone result did not fail closed"
clear_fixture_match_journal

sleep 1
expect_failure "empty ACK evidence" env TRNM_FAULT_FAKE_EMPTY_ACK=1 \
  PATH="$fake_bin:$PATH" HOME="$home" CEX_PROJECT_ROOT="$cex" \
  TRNM_FAULT_FAKE_STATE="$state" TRNM_FAULT_HARNESS_CONTRACT_MODE=1 \
  "$repo/scripts/check-trnm-online-authority-fault-evidence.sh" pg-rtt100
empty_ack_dir="$(find "$repo/run/online-faults" -mindepth 1 -maxdepth 1 -type d | sort | tail -n1)"
jq -e '.status=="failed" and .checks.command_ack_p99_within_budget==false' \
  "$empty_ack_dir/decision.json" >/dev/null \
  || fail "empty ACK evidence did not fail closed"
clear_fixture_match_journal

sleep 1
expect_failure "cleanup failure" env TRNM_FAKE_TC_DELETE_FAIL=1 \
  PATH="$fake_bin:$PATH" HOME="$home" CEX_PROJECT_ROOT="$cex" \
  TRNM_FAULT_FAKE_STATE="$state" TRNM_FAULT_HARNESS_CONTRACT_MODE=1 \
  "$repo/scripts/check-trnm-online-authority-fault-evidence.sh" pg-rtt100
failed_dir="$(find "$repo/run/online-faults" -mindepth 1 -maxdepth 1 -type d | sort | tail -n1)"
jq -e '.status=="failed" and .passed==false and .contract_test_passed==false
  and .checks.cleanup_and_restore_complete==false' "$failed_dir/decision.json" >/dev/null \
  || fail "cleanup failure did not fail the final decision"

printf '%s\n' '{"status":"passed","contract":"trnm_online_authority_fault_evidence_shell_contract_v2"}'
