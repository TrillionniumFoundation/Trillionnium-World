#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="$ROOT_DIR/acceptance/online-production-v1-second-host/latest"
mkdir -p "$OUT_DIR"

LOCAL_HOST_ID="host-$(sha256sum /etc/machine-id | cut -c1-24)"
jq -n --arg local_host "$LOCAL_HOST_ID" '{
  contract:"trnm_online_production_v1_second_physical_host_packet",
  status:"pending_external_second_host",
  automation_credit:false,
  local_physical_host_id:$local_host,
  required_remote_evidence:{
    distinct_machine_id_hash:true,
    independent_kernel_and_power_domain:true,
    postgres_connectivity:true,
    signer_not_copied_to_worker:true,
    unique_instance_id_and_monotonic_epoch:true,
    lease_expiry_takeover:true,
    old_epoch_tick_and_terminal_write_rejected:true,
    previous_and_new_physical_host_audited:true,
    terminal_replay_and_season_integrity:true
  },
  forbidden_shortcuts:[
    "two processes on one host",
    "two containers sharing one kernel",
    "manually edited failover rows",
    "claiming regional HA from loopback endpoints"
  ],
  public_or_regional_ha_claimed:false
}' >"$OUT_DIR/packet.json"

printf '%s\n' \
  '# Online Production v1 second physical host packet' '' \
  "Local host identity: $LOCAL_HOST_ID" '' \
  'Status: pending an independently powered second physical machine.' \
  'The remote process must use its own hashed /etc/machine-id as' \
  '`TRNM_FLEET_PHYSICAL_HOST_ID`, a unique instance ID and the same PostgreSQL' \
  'authority store. Stop the owning host, wait for its five-second lease to' \
  'expire, prove takeover on the distinct host, then prove the old epoch cannot' \
  'write a tick or terminal result. Do not copy signer private material to the' \
  'worker host.' >"$OUT_DIR/README.md"

printf '%s\n' "$OUT_DIR"
