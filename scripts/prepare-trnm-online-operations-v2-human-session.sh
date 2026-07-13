#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${TRNM_HUMAN_SESSION_OUT_DIR:-$ROOT_DIR/acceptance/online-operations-v2-human/latest}"
mkdir -p "$OUT_DIR"

jq -n '{
  contract:"trnm_online_operations_v2_human_session",
  status:"pending_human_participants",
  human_gate_complete:false,
  automation_credit:false,
  planned_duration_minutes:{minimum:10,maximum:15},
  required:{players:2,observers:1,non_developer_players:true,separate_accounts:true,separate_devices:true},
  consent:{player_a:null,player_b:null,observer:null},
  session:{started_at:null,ended_at:null,duration_minutes:null,match_id:null,replay_hash:null,season_id:null},
  checkpoints:[
    {name:"native_login_and_keyring",player_a:null,player_b:null,observer_note:null},
    {name:"ranked_queue_and_authority_match",player_a:null,player_b:null,observer_note:null},
    {name:"disconnect_reconnect",player_a:null,player_b:null,observer_note:null},
    {name:"f9_replay_inspection",player_a:null,player_b:null,observer_note:null},
    {name:"report_and_appeal_comprehension",player_a:null,player_b:null,observer_note:null}
  ],
  observations:[
    {elapsed_seconds:null,player:null,state:null,confusion:null,resolution:null},
    {elapsed_seconds:null,player:null,state:null,confusion:null,resolution:null},
    {elapsed_seconds:null,player:null,state:null,confusion:null,resolution:null}
  ],
  ratings:{
    player_a:{login_clarity:null,queue_clarity:null,battle_control:null,reconnect_confidence:null,replay_clarity:null,safety_clarity:null},
    player_b:{login_clarity:null,queue_clarity:null,battle_control:null,reconnect_confidence:null,replay_clarity:null,safety_clarity:null}
  },
  observer:{verdict:null,top_blocker:null,redacted_media:null},
  secret_capture_forbidden:true,
  completion_rule:"Only human-authored fields, consent, two distinct humans, one observer and a measured 10-15 minute duration may complete this gate."
}' >"$OUT_DIR/session-packet.json"

jq -e '.status == "pending_human_participants" and .human_gate_complete == false and
  .automation_credit == false and .required.players == 2 and .required.observers == 1 and
  (.checkpoints | length) == 5 and (.observations | length) == 3 and
  .secret_capture_forbidden == true' "$OUT_DIR/session-packet.json" >/dev/null

printf '%s\n' \
  '# Online Operations v2 Human Session Packet' '' \
  '- Status: `pending_human_participants`.' \
  '- Required: two non-developer players on separate accounts/devices and one observer.' \
  '- Route: native login/keyring -> ranked queue -> Authority match -> reconnect -> F9 replay -> report/appeal comprehension.' \
  '- Duration: 10-15 measured minutes.' \
  '- Automation credit: none.' \
  '- Secrets in evidence: forbidden.' >"$OUT_DIR/README.md"

printf 'TRNM Online Operations v2 human session packet: pending (%s)\n' "$OUT_DIR"
