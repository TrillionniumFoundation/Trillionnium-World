#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${TRNM_HUMAN_SESSION_OUT_DIR:-$ROOT_DIR/acceptance/online-operations-v1-human/latest}"
mkdir -p "$OUT_DIR"

jq -n '{
  contract:"trnm_online_operations_v1_human_session",
  status:"pending_human_participants",
  human_gate_complete:false,
  automation_credit:false,
  planned_duration_minutes:{minimum:10,maximum:15},
  required:{players:2,observers:1,non_developer_players:true,separate_accounts:true,separate_devices:true},
  consent:{player_a:null,player_b:null,observer:null},
  session:{started_at:null,ended_at:null,duration_minutes:null,match_id:null,replay_hash:null},
  observations:[
    {elapsed_seconds:null,player:null,state:null,confusion:null,resolution:null},
    {elapsed_seconds:null,player:null,state:null,confusion:null,resolution:null},
    {elapsed_seconds:null,player:null,state:null,confusion:null,resolution:null}
  ],
  ratings:{
    player_a:{login_clarity:null,queue_clarity:null,battle_control:null,reconnect_confidence:null},
    player_b:{login_clarity:null,queue_clarity:null,battle_control:null,reconnect_confidence:null}
  },
  observer:{verdict:null,top_blocker:null,redacted_media:null},
  secret_capture_forbidden:true,
  completion_rule:"Only human-authored fields plus consent and a 10-15 minute duration may complete this gate."
}' >"$OUT_DIR/session-packet.json"

jq -e '.status == "pending_human_participants" and .human_gate_complete == false and
  .automation_credit == false and .required.players == 2 and .required.observers == 1 and
  (.observations | length) == 3 and .secret_capture_forbidden == true' \
  "$OUT_DIR/session-packet.json" >/dev/null

printf '%s\n' \
  '# Online Operations v1 Human Session Packet' '' \
  '- Status: `pending_human_participants`' \
  '- Required: two non-developer players, separate accounts/devices, one observer.' \
  '- Route: native text/keyring login -> ranked queue -> Authority match -> reconnect -> replay report.' \
  '- Duration: 10-15 minutes.' \
  '- Automation credit: none.' \
  '- Secrets in evidence: forbidden.' >"$OUT_DIR/README.md"

printf 'TRNM Online Operations v1 human session packet: pending (%s)\n' "$OUT_DIR"
