#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-entry.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/run_trillionnium_world_bevy_artifact_command.sh" classic-rts-campaign-entry >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_campaign_entry_v1"
  and .campaign_handoff_contract == "trillionnium_world_bevy_classic_rts_campaign_handoff_v1"
  and .title_menu_contract == "trillionnium_world_bevy_title_menu_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and (.title_actions | index("CAMPAIGN:START") != null)
  and (.title_actions | index("CAMPAIGN:CONTINUE") != null)
  and (.title_actions | index("CAMPAIGN:REPLAY") != null)
  and .input_path == "apply_live_native_action_with_source(classic_rts_campaign_entry_title_input)"
  and .input_action_count == 73
  and .start_input_count == 73
  and .replay_input_count == 73
  and .campaign_slot_bytes > 20000
  and .final_current_room_id == "league-coliseum"
  and .final_map_scene == "arena_outdoor"
  and .final_open_world_handoff_state == "resumed:league-coliseum"
  and .final_contextual_primary_action_label == "COMBAT:attack"
  and .title_entry_gate == true
  and .start_gate == true
  and .slot_snapshot_gate == true
  and .continue_gate == true
  and .continue_unlock_gate == true
  and .replay_gate == true
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_ENTRY_GREEN %s\n' "$SUMMARY"
