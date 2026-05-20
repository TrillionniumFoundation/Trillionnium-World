#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_entry.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_campaign_entry_v1'
  'trillionnium_world_bevy_classic_rts_campaign_handoff_v1'
  'trillionnium_world_bevy_title_menu_v1'
  'trillionnium_world_bevy_state_snapshot_v1'
  'bevy-classic-rts-campaign-entry.json'
  'classic-rts-campaign-entry'
  'CAMPAIGN:START'
  'CAMPAIGN:CONTINUE'
  'CAMPAIGN:REPLAY'
  'apply_live_native_action_with_source(classic_rts_campaign_entry_title_input)'
  'input_action_count == 73'
  'start_input_count == 73'
  'replay_input_count == 73'
  'continue_unlock_gate == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_ENTRY_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS campaign entry script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_ENTRY_CONTRACT'
  'native_classic_rts_campaign_entry_evidence_json'
  'classic-rts-campaign-entry'
  'StartCampaignFromTitle'
  'ContinueCampaignFromTitle'
  'ReplayCampaignFromTitle'
  'CAMPAIGN:START'
  'CAMPAIGN:CONTINUE'
  'CAMPAIGN:REPLAY'
  'classic_rts_campaign_entry_title_input'
  'native_campaign_entry_slot_path'
  'apply_classic_rts_campaign_handoff_sequence'
  'native_playable_save_snapshot'
  'native_restore_playable_save_snapshot'
  'campaign_entry_snapshot_saved'
  'campaign_entry_snapshot_restored'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS campaign entry source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_campaign_entry.sh'
  'bevy-classic-rts-campaign-entry.json'
  'classic_rts_campaign_entry_green'
  'rts_campaign_entry_title_entry_gate'
  'rts_campaign_entry_start_gate'
  'rts_campaign_entry_slot_snapshot_gate'
  'rts_campaign_entry_continue_gate'
  'rts_campaign_entry_continue_unlock_gate'
  'rts_campaign_entry_replay_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS campaign entry readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS campaign entry evidence remains connected to player-facing title buttons, campaign handoff, snapshot save, and resume"
