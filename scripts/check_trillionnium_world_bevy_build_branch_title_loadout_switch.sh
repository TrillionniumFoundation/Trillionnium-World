#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-loadout-switch.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-loadout-switch >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_loadout_switch_v1"
  and .build_branch_title_loadout_panel_contract == "trillionnium_world_bevy_build_branch_title_loadout_panel_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .multi_title_seed_gate == true
  and .initial_button_state_gate == true
  and .agility_switch_gate == true
  and .craft_switch_gate == true
  and .feedback_gate == true
  and .history_gate == true
  and .save_load_gate == true
  and .restore_ui_gate == true
  and .button_state_after_switch_gate == true
  and .button_events.switch_agility.accepted == true
  and .button_events.switch_agility.availability_before == "enabled_build_title_equip:agility:title-agility-relay-runner"
  and .button_events.switch_craft.accepted == true
  and .button_events.switch_craft.availability_before == "enabled_build_title_equip:craft:title-craft-forge-master"
  and .button_events.save_selected.availability_before == "enabled_save_selected_slot:A"
  and .button_events.load_selected.availability_before == "enabled_session_slot_found:A"
  and .button_events.continue_after_load.availability_before == "enabled_session_resume_continue"
  and (.ui_texts.initial.character_detail_text | contains("TITLE LOADOUT | active Gate Warden") and contains("agility:title-agility-relay-runner:equippable") and contains("craft:title-craft-forge-master:equippable"))
  and (.ui_texts.after_switch_agility.character_status_text | contains("Title Relay Runner"))
  and (.ui_texts.after_switch_agility.character_detail_text | contains("TITLE LOADOUT | active Relay Runner") and contains("force:title-force-gate-warden:equippable") and contains("craft:title-craft-forge-master:equippable"))
  and (.ui_texts.after_switch_agility.event_log_text | contains("Build title equipped: Relay Runner"))
  and (.ui_texts.after_switch_craft.character_status_text | contains("Title Forge Master"))
  and (.ui_texts.after_switch_craft.character_detail_text | contains("TITLE LOADOUT | active Forge Master") and contains("force:title-force-gate-warden:equippable") and contains("agility:title-agility-relay-runner:equippable"))
  and (.ui_texts.after_switch_craft.event_log_text | contains("Build title equipped: Forge Master"))
  and (.ui_texts.after_continue_craft_title.character_status_text | contains("Title Forge Master"))
  and (.ui_texts.after_continue_craft_title.character_detail_text | contains("TITLE LOADOUT | active Forge Master"))
  and (.ui_texts.after_continue_craft_title.session_slot_text | contains("TITLE Forge Master"))
  and .title_button_states.initial_force.enabled == false
  and .title_button_states.initial_force.reason == "build_title_already_active:title-force-gate-warden"
  and .title_button_states.initial_agility.enabled == true
  and .title_button_states.initial_craft.enabled == true
  and .title_button_states.after_agility_agility.enabled == false
  and .title_button_states.after_agility_craft.enabled == true
  and .title_button_states.after_craft_force.enabled == true
  and .title_button_states.after_craft_craft.enabled == false
  and .slot_snapshot_after_switch_save.present == true
  and .slot_snapshot_after_switch_save.contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .slot_snapshot_after_switch_save.active_build_title_id == "title-craft-forge-master"
  and .slot_snapshot_after_switch_save.active_build_title_effect == "forge_client_trust_anchor"
  and .final_runtime.active_build_title_id == "title-craft-forge-master"
  and .final_runtime.active_build_title_effect == "forge_client_trust_anchor"
  and (.final_runtime.build_title_equip_history | index("equipped:force:title-force-gate-warden:arena_gate_reputation_anchor") != null)
  and (.final_runtime.build_title_equip_history | index("equipped:agility:title-agility-relay-runner:relay_route_priority_anchor") != null)
  and (.final_runtime.build_title_equip_history | index("equipped:craft:title-craft-forge-master:forge_client_trust_anchor") != null)
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_LOADOUT_SWITCH_GREEN %s\n' "$SUMMARY"
