#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat >&2 <<'EOF'
usage: run-trnm-world-cross-host-evidence-v1.sh OUTPUT_DIR

Required environment:
  TRNM_HOST_A                       SSH destination for original primary host
  TRNM_HOST_B                       SSH destination for promotion/PITR host
  TRNM_WORLD_PACKAGE                local immutable World package or binary archive
  TRNM_CROSS_HOST_AGENT             local executable copied to both hosts
  TRNM_CROSS_HOST_AUTHORIZATION     local nonempty authorization record
  TRNM_FAULT_PLAN                   local reviewed fault-plan file
  TRNM_REMOTE_EVIDENCE_ROOT         absolute remote evidence root

Optional environment:
  TRNM_SSH_PORT                     default 22
  TRNM_SSH_IDENTITY_FILE            private key path; never copied to evidence
  TRNM_SSH_KNOWN_HOSTS              required known-hosts file
  TRNM_AGENT_TIMEOUT_SECONDS        default 1800 per phase

The remote agent must support:
  identity preflight stop-primary promote pitr old-primary-return reconcile cleanup
and write bounded non-secret evidence below TRNM_REMOTE_EVIDENCE_ROOT.
EOF
  exit 64
}

[[ $# -eq 1 ]] || usage
out="$1"
: "${TRNM_HOST_A:?required}"
: "${TRNM_HOST_B:?required}"
: "${TRNM_WORLD_PACKAGE:?required}"
: "${TRNM_CROSS_HOST_AGENT:?required}"
: "${TRNM_CROSS_HOST_AUTHORIZATION:?required}"
: "${TRNM_FAULT_PLAN:?required}"
: "${TRNM_REMOTE_EVIDENCE_ROOT:?required}"
port="${TRNM_SSH_PORT:-22}"
timeout_seconds="${TRNM_AGENT_TIMEOUT_SECONDS:-1800}"
known_hosts="${TRNM_SSH_KNOWN_HOSTS:-}"

[[ "$TRNM_HOST_A" != "$TRNM_HOST_B" ]] || { echo "hosts must differ" >&2; exit 1; }
[[ "$TRNM_REMOTE_EVIDENCE_ROOT" == /* ]] || { echo "remote evidence root must be absolute" >&2; exit 1; }
[[ "$port" =~ ^[0-9]{1,5}$ ]] && (( port > 0 && port < 65536 )) || { echo "invalid SSH port" >&2; exit 1; }
[[ "$timeout_seconds" =~ ^[0-9]+$ ]] && (( timeout_seconds >= 60 && timeout_seconds <= 7200 )) || { echo "invalid phase timeout" >&2; exit 1; }
for path in "$TRNM_WORLD_PACKAGE" "$TRNM_CROSS_HOST_AGENT" "$TRNM_CROSS_HOST_AUTHORIZATION" "$TRNM_FAULT_PLAN"; do
  [[ -f "$path" && ! -L "$path" && -s "$path" ]] || { echo "missing, linked or empty input: $path" >&2; exit 1; }
done
[[ -x "$TRNM_CROSS_HOST_AGENT" ]] || { echo "cross-host agent must be executable" >&2; exit 1; }
[[ -n "$known_hosts" && -f "$known_hosts" && ! -L "$known_hosts" && -s "$known_hosts" ]] || {
  echo "TRNM_SSH_KNOWN_HOSTS must name a reviewed nonempty file" >&2
  exit 1
}

umask 077
mkdir -p "$out"/{inputs,host-a,host-b,orchestrator}
[[ -z "$(find "$out" -mindepth 1 -maxdepth 1 ! -type d -print -quit)" ]] || {
  echo "output directory contains unexpected files" >&2
  exit 1
}

ssh_args=(-p "$port" -o BatchMode=yes -o IdentitiesOnly=yes -o StrictHostKeyChecking=yes -o "UserKnownHostsFile=$known_hosts" -o ConnectTimeout=15 -o ServerAliveInterval=15 -o ServerAliveCountMax=4)
scp_args=(-P "$port" -o BatchMode=yes -o IdentitiesOnly=yes -o StrictHostKeyChecking=yes -o "UserKnownHostsFile=$known_hosts" -o ConnectTimeout=15)
if [[ -n "${TRNM_SSH_IDENTITY_FILE:-}" ]]; then
  [[ -f "$TRNM_SSH_IDENTITY_FILE" && ! -L "$TRNM_SSH_IDENTITY_FILE" ]] || { echo "unsafe SSH identity file" >&2; exit 1; }
  ssh_args+=(-i "$TRNM_SSH_IDENTITY_FILE")
  scp_args+=(-i "$TRNM_SSH_IDENTITY_FILE")
fi

sha256sum "$TRNM_WORLD_PACKAGE" "$TRNM_CROSS_HOST_AGENT" "$TRNM_CROSS_HOST_AUTHORIZATION" "$TRNM_FAULT_PLAN" > "$out/inputs/SHA256SUMS"
cp "$TRNM_CROSS_HOST_AUTHORIZATION" "$out/inputs/authorization.record"
cp "$TRNM_FAULT_PLAN" "$out/inputs/fault-plan.record"
start="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
printf '%s\n' "$start" > "$out/orchestrator/started-at.txt"

remote_root_q="$(printf '%q' "$TRNM_REMOTE_EVIDENCE_ROOT")"
agent_name="trnm-world-cross-host-agent-v1.sh"
package_name="world-package.bin"

remote() {
  local host="$1"; shift
  ssh "${ssh_args[@]}" "$host" "$@"
}

copy_to() {
  local source="$1" host="$2" destination="$3"
  scp "${scp_args[@]}" "$source" "$host:$destination"
}

prepare_host() {
  local host="$1" label="$2"
  remote "$host" "umask 077; mkdir -p $remote_root_q/{inputs,raw,summary}; test ! -L $remote_root_q"
  copy_to "$TRNM_CROSS_HOST_AGENT" "$host" "$TRNM_REMOTE_EVIDENCE_ROOT/inputs/$agent_name"
  copy_to "$TRNM_WORLD_PACKAGE" "$host" "$TRNM_REMOTE_EVIDENCE_ROOT/inputs/$package_name"
  remote "$host" "chmod 0700 $remote_root_q/inputs/$agent_name; sha256sum $remote_root_q/inputs/$agent_name $remote_root_q/inputs/$package_name > $remote_root_q/inputs/SHA256SUMS"
  timeout "$timeout_seconds" remote "$host" "TRNM_REMOTE_EVIDENCE_ROOT=$remote_root_q $remote_root_q/inputs/$agent_name identity" > "$out/$label/identity.json"
}

prepare_host "$TRNM_HOST_A" host-a
prepare_host "$TRNM_HOST_B" host-b

python3 - "$out/host-a/identity.json" "$out/host-b/identity.json" <<'PY'
import json, pathlib, sys
values=[]
for raw in sys.argv[1:]:
    path=pathlib.Path(raw)
    data=json.loads(path.read_text(encoding='utf-8'))
    for key in ('host_fingerprint','machine_fingerprint','failure_domain','database_system_identifier'):
        value=data.get(key)
        if not isinstance(value,str) or len(value)<4:
            raise SystemExit(f'{path}: missing {key}')
    values.append(data)
for key in ('host_fingerprint','machine_fingerprint','failure_domain'):
    if values[0][key] == values[1][key]:
        raise SystemExit(f'hosts are not independent: identical {key}')
if values[0]['database_system_identifier'] == values[1]['database_system_identifier'] and not values[1].get('declared_replica_of_host_a'):
    raise SystemExit('database identities are crossed without a declared replica relationship')
PY

run_phase() {
  local label="$1" host="$2" phase="$3"
  printf '%s %s %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$label" "$phase" >> "$out/orchestrator/phases.log"
  timeout "$timeout_seconds" remote "$host" "TRNM_REMOTE_EVIDENCE_ROOT=$remote_root_q $remote_root_q/inputs/$agent_name $phase" \
    > "$out/$label/$phase.stdout" 2> "$out/$label/$phase.stderr"
}

cleanup_needed=1
cleanup() {
  local rc=$?
  set +e
  if (( cleanup_needed )); then
    run_phase host-b "$TRNM_HOST_B" cleanup
    run_phase host-a "$TRNM_HOST_A" cleanup
  fi
  printf '%s\n' "$rc" > "$out/orchestrator/orchestrator-exit-code.txt"
  exit "$rc"
}
trap cleanup EXIT INT TERM

run_phase host-a "$TRNM_HOST_A" preflight
run_phase host-b "$TRNM_HOST_B" preflight
run_phase host-a "$TRNM_HOST_A" stop-primary
run_phase host-b "$TRNM_HOST_B" promote
run_phase host-b "$TRNM_HOST_B" pitr
run_phase host-a "$TRNM_HOST_A" old-primary-return
run_phase host-b "$TRNM_HOST_B" reconcile
run_phase host-a "$TRNM_HOST_A" reconcile

for label_host in "host-a:$TRNM_HOST_A" "host-b:$TRNM_HOST_B"; do
  label="${label_host%%:*}"
  host="${label_host#*:}"
  remote "$host" "tar -C $remote_root_q -czf $remote_root_q/summary/evidence.tar.gz inputs raw summary --exclude=summary/evidence.tar.gz"
  scp "${scp_args[@]}" "$host:$TRNM_REMOTE_EVIDENCE_ROOT/summary/evidence.tar.gz" "$out/$label/remote-evidence.tar.gz"
done

run_phase host-b "$TRNM_HOST_B" cleanup
run_phase host-a "$TRNM_HOST_A" cleanup
cleanup_needed=0
trap - EXIT INT TERM
end="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
printf '%s\n' "$end" > "$out/orchestrator/ended-at.txt"
find "$out" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/orchestrator/ARTIFACT-SHA256SUMS"

cat > "$out/orchestrator/RESULT.txt" <<EOF
TRNM_WORLD_CROSS_HOST_ORCHESTRATION=COMPLETE
started_at_utc=$start
ended_at_utc=$end
host_a=$TRNM_HOST_A
host_b=$TRNM_HOST_B
production_authorization=not_granted
independent_review_required=true
strict_evidence_validation_required=true
EOF

echo "Cross-host orchestration completed; this output is not accepted evidence until an independent reviewer signs a strict evidence record." 
