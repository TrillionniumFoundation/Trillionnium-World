#!/usr/bin/bash
set -euo pipefail

readonly TRUSTED_PATH="/usr/sbin:/usr/bin"
ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
canonical_home="$(getent passwd "$UID" | awk -F: -v uid="$UID" \
  '$3 == uid {print $6; exit}')"
[[ -n "$canonical_home" && -d "$canonical_home" ]] || {
  printf 'could not resolve the canonical home directory\n' >&2
  exit 2
}

concurrency="${TRNM_CAPACITY_CONCURRENCY:-4}"
duration_seconds="${TRNM_CAPACITY_DURATION_SECONDS:-86400}"
monitor_interval_seconds="${TRNM_CAPACITY_MONITOR_INTERVAL_SECONDS:-5}"
for value in "$concurrency" "$duration_seconds" "$monitor_interval_seconds"; do
  [[ "$value" =~ ^[1-9][0-9]*$ ]] || {
    printf 'capacity launcher values must be positive integers\n' >&2
    exit 2
  }
done

if systemctl --user list-units --state=running --plain --no-legend \
    'trnm-capacity-*.service' 'trnm-capacity-*.scope' | grep -q .; then
  printf 'another TRNM capacity harness is already running\n' >&2
  exit 2
fi

started_at="$(date -u +%Y%m%dT%H%M%S)"
unit="trnm-capacity-$started_at-$$"
nonce="$(od -An -N4 -tu4 /dev/urandom | tr -d ' ')"
run_id="capacity-$(date +%s)-$nonce"
commit="$(git -C "$ROOT_DIR" rev-parse --short=12 HEAD)"

systemd-run --user --collect --unit="$unit" \
  --description="TRNM durable online capacity soak $commit" \
  --property=WorkingDirectory="$ROOT_DIR" \
  --property=TimeoutStopSec=300s \
  --property=KillMode=mixed \
  --property=CPUAccounting=true \
  --property=CPUWeight=100 \
  --property=CPUQuota=150% \
  --property=MemoryAccounting=true \
  --property=MemoryHigh=1536M \
  --property=MemoryMax=2048M \
  --property=MemorySwapMax=512M \
  --property=IOAccounting=true \
  --property=IOWeight=100 \
  --property=TasksAccounting=true \
  --property=TasksMax=512 \
  /usr/bin/env -i \
  PATH="$TRUSTED_PATH" \
  HOME="$canonical_home" \
  XDG_RUNTIME_DIR="/run/user/$UID" \
  TRNM_CAPACITY_RESOURCE_SCOPE_ACTIVE=1 \
  TRNM_CAPACITY_CONCURRENCY="$concurrency" \
  TRNM_CAPACITY_DURATION_SECONDS="$duration_seconds" \
  TRNM_CAPACITY_MONITOR_INTERVAL_SECONDS="$monitor_interval_seconds" \
  TRNM_CAPACITY_RUN_ID="$run_id" \
  "$ROOT_DIR/scripts/check-trnm-online-capacity-soak.sh"

jq -n --arg unit "$unit.service" --arg run_id "$run_id" \
  --arg evidence "$ROOT_DIR/run/online-capacity/$run_id" \
  --arg commit "$commit" \
  --argjson concurrency "$concurrency" \
  --argjson duration_seconds "$duration_seconds" \
  --argjson monitor_interval_seconds "$monitor_interval_seconds" \
  '{status:"started",unit:$unit,run_id:$run_id,evidence_dir:$evidence,
    git_commit:$commit,concurrency:$concurrency,
    duration_seconds:$duration_seconds,
    monitor_interval_seconds:$monitor_interval_seconds,
    completion_pending:true}'
