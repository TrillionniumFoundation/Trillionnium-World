#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-title-equip.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-title-equip >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_title_equip_v1"
  and .build_branch_mastery_titles_contract == "trillionnium_world_bevy_build_branch_mastery_titles_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .title_buttons_visible_gate == true
  and .title_equip_accepted_gate == true
  and .title_restore_gate == true
  and .title_visible_gate == true
  and .title_effect_gate == true
  and .wrong_title_blocked_gate == true
  and .title_switch_isolation_gate == true
  and .titles.force.title_id == "title-force-gate-warden"
  and .titles.force.effect == "arena_gate_reputation_anchor"
  and .titles.force.wrong_title_attempt == "title-agility-relay-runner"
  and .titles.agility.title_id == "title-agility-relay-runner"
  and .titles.agility.effect == "relay_route_priority_anchor"
  and .titles.agility.wrong_title_attempt == "title-craft-forge-master"
  and .titles.craft.title_id == "title-craft-forge-master"
  and .titles.craft.effect == "forge_client_trust_anchor"
  and .titles.craft.wrong_title_attempt == "title-force-gate-warden"
  and (.title_button_samples.force[] | select(.action_label == "TITLE:EQUIP:title-force-gate-warden")) != null
  and (.title_button_samples.agility[] | select(.action_label == "TITLE:EQUIP:title-agility-relay-runner")) != null
  and (.title_button_samples.craft[] | select(.action_label == "TITLE:EQUIP:title-craft-forge-master")) != null
  and .slot_snapshots_after_title_equip.force.active_build_title_id == "title-force-gate-warden"
  and .slot_snapshots_after_title_equip.force.active_build_title_effect == "arena_gate_reputation_anchor"
  and (.slot_snapshots_after_title_equip.force.build_title_equip_history | index("equipped:force:title-force-gate-warden:arena_gate_reputation_anchor") != null)
  and (.runtime_summaries_after_title_restore.force.enemy_damage_feedback | contains("ACTIVE TITLE Gate Warden -1 incoming"))
  and (.runtime_summaries_after_title_restore.force.enemy_behavior_history[] | select(. == "active_title_force_gate_warden_threat")) != null
  and .slot_snapshots_after_title_equip.agility.active_build_title_id == "title-agility-relay-runner"
  and .slot_snapshots_after_title_equip.agility.active_build_title_effect == "relay_route_priority_anchor"
  and (.slot_snapshots_after_title_equip.agility.build_title_equip_history | index("equipped:agility:title-agility-relay-runner:relay_route_priority_anchor") != null)
  and (.runtime_summaries_after_title_restore.agility.route_director_history[] | select(. == "active_title_route_effect:agility:relay_priority_anchor")) != null
  and (.runtime_summaries_after_title_restore.agility.npc_bubble_text | contains("ACTIVE TITLE Relay Runner priority"))
  and .slot_snapshots_after_title_equip.craft.active_build_title_id == "title-craft-forge-master"
  and .slot_snapshots_after_title_equip.craft.active_build_title_effect == "forge_client_trust_anchor"
  and (.slot_snapshots_after_title_equip.craft.build_title_equip_history | index("equipped:craft:title-craft-forge-master:forge_client_trust_anchor") != null)
  and (.runtime_summaries_after_title_restore.craft.reward_settlement_text | contains("ACTIVE TITLE Forge Master client +2"))
  and (.runtime_summaries_after_title_restore.craft.loot_history[] | select(. == "active_title_craft_client_trust_anchor")) != null
  and (.quest_texts_after_title_restore.force | contains("ACTIVE TITLE | force:title-force-gate-warden"))
  and (.quest_texts_after_title_restore.agility | contains("ACTIVE TITLE | agility:title-agility-relay-runner"))
  and (.quest_texts_after_title_restore.craft | contains("ACTIVE TITLE | craft:title-craft-forge-master"))
  and (.button_events.force[] | select(.action == "TITLE:EQUIP:title-agility-relay-runner" and .accepted == false and (.availability_before | startswith("build_title_locked:")))) != null
  and (.button_events.agility[] | select(.action == "TITLE:EQUIP:title-craft-forge-master" and .accepted == false and (.availability_before | startswith("build_title_locked:")))) != null
  and (.button_events.craft[] | select(.action == "TITLE:EQUIP:title-force-gate-warden" and .accepted == false and (.availability_before | startswith("build_title_locked:")))) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_TITLE_EQUIP_GREEN %s\n' "$SUMMARY"
