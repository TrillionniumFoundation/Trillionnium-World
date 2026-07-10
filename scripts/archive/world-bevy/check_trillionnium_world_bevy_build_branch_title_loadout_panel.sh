#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-loadout-panel.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-loadout-panel >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_loadout_panel_v1"
  and .build_branch_title_equip_contract == "trillionnium_world_bevy_build_branch_title_equip_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .title_equip_contract_green == true
  and .before_equip_panel_gate == true
  and .after_equip_active_gate == true
  and .locked_title_visibility_gate == true
  and .button_panel_parity_gate == true
  and .slot_snapshot_title_gate == true
  and (.loadout_texts_before_title_equip.force | contains("TITLE LOADOUT | active none") and contains("force:title-force-gate-warden:equippable") and contains("action TITLE:EQUIP:title-force-gate-warden") and contains("reason enabled_build_title_equip:force:title-force-gate-warden"))
  and (.loadout_texts_after_title_restore.force | contains("TITLE LOADOUT | active Gate Warden") and contains("force:title-force-gate-warden:active") and contains("reason build_title_already_active:title-force-gate-warden") and contains("agility:title-agility-relay-runner:locked_requires_agility_mastery_challenge"))
  and (.loadout_texts_before_title_equip.agility | contains("TITLE LOADOUT | active none") and contains("agility:title-agility-relay-runner:equippable") and contains("action TITLE:EQUIP:title-agility-relay-runner") and contains("reason enabled_build_title_equip:agility:title-agility-relay-runner"))
  and (.loadout_texts_after_title_restore.agility | contains("TITLE LOADOUT | active Relay Runner") and contains("agility:title-agility-relay-runner:active") and contains("reason build_title_already_active:title-agility-relay-runner") and contains("craft:title-craft-forge-master:locked_requires_craft_mastery_challenge"))
  and (.loadout_texts_before_title_equip.craft | contains("TITLE LOADOUT | active none") and contains("craft:title-craft-forge-master:equippable") and contains("action TITLE:EQUIP:title-craft-forge-master") and contains("reason enabled_build_title_equip:craft:title-craft-forge-master"))
  and (.loadout_texts_after_title_restore.craft | contains("TITLE LOADOUT | active Forge Master") and contains("craft:title-craft-forge-master:active") and contains("reason build_title_already_active:title-craft-forge-master") and contains("force:title-force-gate-warden:locked_requires_force_mastery_challenge"))
  and (.title_button_samples.force[] | select(.action_label == "TITLE:EQUIP:title-force-gate-warden")) != null
  and (.title_button_samples.agility[] | select(.action_label == "TITLE:EQUIP:title-agility-relay-runner")) != null
  and (.title_button_samples.craft[] | select(.action_label == "TITLE:EQUIP:title-craft-forge-master")) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_LOADOUT_PANEL_GREEN %s\n' "$SUMMARY"
