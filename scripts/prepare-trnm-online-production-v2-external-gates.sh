#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${TRNM_PRODUCTION_V2_EXTERNAL_OUT_DIR:-$ROOT_DIR/acceptance/online-production-v2-external/latest}"
mkdir -p "$OUT_DIR"

LOCAL_HOST_ID="host-$(sha256sum /etc/machine-id | cut -c1-24)"
jq -n --arg local_host "$LOCAL_HOST_ID" '{
  contract:"trnm_online_production_v2_external_gates",
  status:"pending_external_actors_and_infrastructure",
  automation_credit:false,
  local_physical_host_id:$local_host,
  gates:{
    human_session:{complete:false,players:2,observers:1,duration_minutes:"10-15",separate_devices:true},
    second_physical_host:{complete:false,distinct_host_ids_required:2,independent_kernel_power_domain:true,epoch_fencing:true,terminal_integrity:true},
    kms_hsm:{complete:false,provider_attestation_required:true,non_exportable_key_required:true,rotation_and_revocation_required:true,cex_registry_convergence:true},
    public_edge:{complete:false,waf_rate_limit_required:true,ddos_attestation_required:true,capacity_soak_required:true,approved_non_loopback_ingress_required:true},
    staffed_moderation:{complete:false,named_shift_staff_required:true,real_case_handoff_required:true,appeal_sla_measurement_required:true}
  },
  forbidden_shortcuts:[
    "automated users counted as human participants",
    "two processes or containers on one kernel counted as two hosts",
    "file-backed seed described as KMS or HSM custody",
    "loopback 429 or synthetic payload rejection described as public WAF or DDoS proof",
    "software queue described as a staffed moderation team"
  ],
  public_launch_claimed:false,
  player_market_claimed:false
}' >"$OUT_DIR/packet.json"

jq -e '.status == "pending_external_actors_and_infrastructure" and
  .automation_credit == false and .gates.human_session.complete == false and
  .gates.second_physical_host.complete == false and .gates.kms_hsm.complete == false and
  .gates.public_edge.complete == false and .gates.staffed_moderation.complete == false and
  .public_launch_claimed == false and .player_market_claimed == false' \
  "$OUT_DIR/packet.json" >/dev/null

printf '%s\n' \
  '# Online Production v2 external gate packet' '' \
  "Local host identity: $LOCAL_HOST_ID" '' \
  '- Status: pending real people and independently controlled infrastructure.' \
  '- Human: two non-developer players, separate devices/accounts, one observer, measured 10-15 minutes.' \
  '- Host: a second independently powered machine must take over after lease expiry and fence the old epoch.' \
  '- Key: a provider-backed non-exportable signer must prove possession, rotate/revoke and converge with the CEX registry.' \
  '- Edge: approved ingress, WAF/rate-limit, DDoS evidence and a measured capacity soak are required.' \
  '- Safety: named staff must run a shift, claim/handoff a real case and measure appeal response.' \
  '- This packet grants no release, HA, KMS/HSM, public-edge, staffing or market credit.' \
  >"$OUT_DIR/README.md"

printf 'TRNM Online Production v2 external packet: pending (%s)\n' "$OUT_DIR"
