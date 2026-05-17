#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-active-title-ui.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-active-title-ui >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_active_title_ui_v1"
  and .build_branch_title_equip_contract == "trillionnium_world_bevy_build_branch_title_equip_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .title_equip_contract_green == true
  and .top_hud_title_gate == true
  and .character_panel_title_gate == true
  and .session_slot_title_gate == true
  and .slot_snapshot_title_gate == true
  and (.ui_texts_after_title_restore.force.character_status_text | contains("Title Gate Warden") and contains("arena_gate_reputation_anchor"))
  and (.ui_texts_after_title_restore.force.character_detail_text | contains("TITLE Gate Warden") and contains("title-force-gate-warden") and contains("arena_gate_reputation_anchor"))
  and (.ui_texts_after_title_restore.force.session_slot_text | contains("A:saved") and contains("TITLE Gate Warden") and contains("enabled_session_slot_found:A"))
  and (.ui_texts_after_title_restore.agility.character_status_text | contains("Title Relay Runner") and contains("relay_route_priority_anchor"))
  and (.ui_texts_after_title_restore.agility.character_detail_text | contains("TITLE Relay Runner") and contains("title-agility-relay-runner") and contains("relay_route_priority_anchor"))
  and (.ui_texts_after_title_restore.agility.session_slot_text | contains("A:saved") and contains("TITLE Relay Runner") and contains("enabled_session_slot_found:A"))
  and (.ui_texts_after_title_restore.craft.character_status_text | contains("Title Forge Master") and contains("forge_client_trust_anchor"))
  and (.ui_texts_after_title_restore.craft.character_detail_text | contains("TITLE Forge Master") and contains("title-craft-forge-master") and contains("forge_client_trust_anchor"))
  and (.ui_texts_after_title_restore.craft.session_slot_text | contains("A:saved") and contains("TITLE Forge Master") and contains("enabled_session_slot_found:A"))
  and .slot_snapshots_after_title_equip.force.active_build_title_id == "title-force-gate-warden"
  and .slot_snapshots_after_title_equip.force.active_build_title_effect == "arena_gate_reputation_anchor"
  and .slot_snapshots_after_title_equip.agility.active_build_title_id == "title-agility-relay-runner"
  and .slot_snapshots_after_title_equip.agility.active_build_title_effect == "relay_route_priority_anchor"
  and .slot_snapshots_after_title_equip.craft.active_build_title_id == "title-craft-forge-master"
  and .slot_snapshots_after_title_equip.craft.active_build_title_effect == "forge_client_trust_anchor"
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_ACTIVE_TITLE_UI_GREEN %s\n' "$SUMMARY"
