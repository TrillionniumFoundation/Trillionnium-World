#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-build-branch-mastery.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- build-branch-mastery >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_build_branch_mastery_v1"
  and .build_branch_followup_completion_contract == "trillionnium_world_bevy_build_branch_followup_completion_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .all_branch_events_accepted == true
  and .mastery_persisted_gate == true
  and .force_mastery_combat_gate == true
  and .agility_mastery_shortcut_gate == true
  and .craft_mastery_economy_gate == true
  and .mastery_isolation_gate == true
  and .mastery_ids.force == "mastery-force-guard-stance"
  and .mastery_ids.agility == "mastery-agility-relay-step"
  and .mastery_ids.craft == "mastery-craft-forge-batch"
  and .mastery_effects.force == "incoming_damage_reduction_2"
  and .mastery_effects.agility == "delivery_dock_shortcut_to_square"
  and .mastery_effects.craft == "client_order_coin_bonus_3"
  and (.slot_bytes.force > 0)
  and (.slot_bytes.agility > 0)
  and (.slot_bytes.craft > 0)
  and (.slot_snapshots_after_mastery.force.build_mastery_unlocked_ids | index("mastery-force-guard-stance") != null)
  and (.slot_snapshots_after_mastery.force.build_mastery_effects | index("incoming_damage_reduction_2") != null)
  and (.quest_texts_after_mastery_restore.force | contains("force:mastery-force-guard-stance:active"))
  and (.runtime_summaries_after_mastery_effect.force.stat_effect_floating_text | contains("FORCE MASTERY -2 INCOMING"))
  and (.runtime_summaries_after_mastery_effect.force.combat_round_log | map(contains("incoming:4")) | any)
  and (.visible_effects_after_mastery_restore.force_npc | contains("MASTERY guard stance ready"))
  and (.visible_effects_after_mastery_restore.force_enemy | contains("MASTERY incoming -2"))
  and (.slot_snapshots_after_mastery.agility.build_mastery_unlocked_ids | index("mastery-agility-relay-step") != null)
  and (.slot_snapshots_after_mastery.agility.build_mastery_effects | index("delivery_dock_shortcut_to_square") != null)
  and (.slot_snapshots_after_mastery.agility.available_room_exits | index("mirror-city-square") != null)
  and (.quest_texts_after_mastery_restore.agility | contains("agility:mastery-agility-relay-step:active"))
  and .runtime_summaries_after_mastery_effect.agility.current_room_id == "mirror-city-square"
  and (.runtime_summaries_after_mastery_effect.agility.route_director_history | map(contains("build_mastery_route_effect:agility:shortcut_route_opened")) | any)
  and (.visible_effects_after_mastery_restore.agility_npc | contains("MASTERY shortcut route open"))
  and (.slot_snapshots_after_mastery.craft.build_mastery_unlocked_ids | index("mastery-craft-forge-batch") != null)
  and (.slot_snapshots_after_mastery.craft.build_mastery_effects | index("client_order_coin_bonus_3") != null)
  and (.quest_texts_after_mastery_restore.craft | contains("craft:mastery-craft-forge-batch:active"))
  and (.runtime_summaries_after_mastery_effect.craft.reward_settlement_text | contains("CRAFT MASTERY +3 coins"))
  and (.runtime_summaries_after_mastery_effect.craft.coins >= 36)
  and (.visible_effects_after_mastery_restore.craft_npc | contains("MASTERY client orders pay +3 coins"))
  and (.runtime_summaries_after_mastery_effect.force.build_mastery_unlocked_ids | index("mastery-agility-relay-step") == null)
  and (.runtime_summaries_after_mastery_effect.agility.build_mastery_unlocked_ids | index("mastery-craft-forge-batch") == null)
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_BUILD_BRANCH_MASTERY_GREEN %s\n' "$SUMMARY"
