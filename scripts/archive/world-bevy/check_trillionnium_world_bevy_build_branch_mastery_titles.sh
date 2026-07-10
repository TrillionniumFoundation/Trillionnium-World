#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-mastery-titles.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-mastery-titles >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_mastery_titles_v1"
  and .build_branch_mastery_challenges_contract == "trillionnium_world_bevy_build_branch_mastery_challenges_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .challenge_contract_green == true
  and .title_persisted_gate == true
  and .quest_title_text_gate == true
  and .settlement_title_gate == true
  and .visible_title_gate == true
  and .title_isolation_gate == true
  and .titles.force.title_id == "title-force-gate-warden"
  and .titles.force.effect == "arena_gate_reputation_anchor"
  and .titles.agility.title_id == "title-agility-relay-runner"
  and .titles.agility.effect == "relay_route_priority_anchor"
  and .titles.craft.title_id == "title-craft-forge-master"
  and .titles.craft.effect == "forge_client_trust_anchor"
  and (.slot_snapshots_after_title_unlock.force.build_title_unlocked_ids | index("title-force-gate-warden") != null)
  and (.slot_snapshots_after_title_unlock.force.build_title_effects | index("arena_gate_reputation_anchor") != null)
  and (.runtime_summaries_after_title_unlock.force.reward_settlement_text | contains("TITLE Gate Warden"))
  and (.quest_texts_after_title_unlock.force | contains("force:title-force-gate-warden:unlocked"))
  and (.visible_texts_after_title_unlock.force_npc | contains("TITLE Gate Warden recognized"))
  and (.runtime_summaries_after_title_unlock.force.visible_behavior_history | index("build_mastery_title_force_visible") != null)
  and (.slot_snapshots_after_title_unlock.agility.build_title_unlocked_ids | index("title-agility-relay-runner") != null)
  and (.slot_snapshots_after_title_unlock.agility.build_title_effects | index("relay_route_priority_anchor") != null)
  and (.runtime_summaries_after_title_unlock.agility.reward_settlement_text | contains("TITLE Relay Runner"))
  and (.quest_texts_after_title_unlock.agility | contains("agility:title-agility-relay-runner:unlocked"))
  and (.visible_texts_after_title_unlock.agility_npc | contains("TITLE Relay Runner recognized"))
  and (.runtime_summaries_after_title_unlock.agility.visible_behavior_history | index("build_mastery_title_agility_visible") != null)
  and (.slot_snapshots_after_title_unlock.craft.build_title_unlocked_ids | index("title-craft-forge-master") != null)
  and (.slot_snapshots_after_title_unlock.craft.build_title_effects | index("forge_client_trust_anchor") != null)
  and (.runtime_summaries_after_title_unlock.craft.reward_settlement_text | contains("TITLE Forge Master"))
  and (.quest_texts_after_title_unlock.craft | contains("craft:title-craft-forge-master:unlocked"))
  and (.visible_texts_after_title_unlock.craft_npc | contains("TITLE Forge Master recognized"))
  and (.runtime_summaries_after_title_unlock.craft.visible_behavior_history | index("build_mastery_title_craft_visible") != null)
  and (.runtime_summaries_after_title_unlock.force.build_title_unlocked_ids | index("title-agility-relay-runner") == null)
  and (.runtime_summaries_after_title_unlock.agility.build_title_unlocked_ids | index("title-craft-forge-master") == null)
  and (.runtime_summaries_after_title_unlock.craft.build_title_unlocked_ids | index("title-force-gate-warden") == null)
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_MASTERY_TITLES_GREEN %s\n' "$SUMMARY"
