#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-equipment-affix-build.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- equipment-affix-build >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_equipment_affix_build_v1"
  and .inventory_equipment_contract == "trillionnium_world_bevy_inventory_equipment_v1"
  and .stat_confirmation_contract == "trillionnium_world_bevy_stat_confirmation_v1"
  and .stat_gameplay_effects_contract == "trillionnium_world_bevy_stat_gameplay_effects_v1"
  and .green == true
  and .event_acceptance_gate == true
  and .dormant_affix_gate == true
  and .pending_affix_gate == true
  and .force_affix_gate == true
  and .agility_affix_gate == true
  and .craft_affix_gate == true
  and (.inventory_texts.dormant_after_equip | contains("AFFIX: AFFIX dormant"))
  and (.inventory_texts.dormant_after_equip | contains("choose force/agility/craft to attune bandit_sash"))
  and (.inventory_texts.pending_force | contains("AFFIX PREVIEW | bandit_sash:force"))
  and (.inventory_texts.pending_force | contains("iron-strike preview -> rematch attack +4 DMG"))
  and (.inventory_texts.force_active | contains("AFFIX ACTIVE | bandit_sash:iron-strike"))
  and (.inventory_texts.agility_active | contains("AFFIX ACTIVE | bandit_sash:wind-guard"))
  and (.inventory_texts.craft_active | contains("AFFIX ACTIVE | bandit_sash:market-thread"))
  and (.effect_texts.force_feedback | contains("FORCE +4 DMG"))
  and (.effect_texts.agility_feedback | contains("AGILITY -3 INCOMING"))
  and (.effect_texts.craft_reward_toast | contains("CRAFT +4 coins +5 XP"))
  and .runtime_summaries.dormant.growth_stat_points == 1
  and (.runtime_summaries.dormant.allocated_stat_points | length) == 0
  and .runtime_summaries.pending_force.pending_stat_allocation == "force"
  and (.runtime_summaries.pending_force.allocated_stat_points | length) == 0
  and .runtime_summaries.force_active.enemy_hp == 34
  and .runtime_summaries.agility_active.player_hp == 97
  and .runtime_summaries.craft_active.coins == 14
  and .runtime_summaries.craft_active.xp == 45
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_EQUIPMENT_AFFIX_BUILD_GREEN %s\n' "$SUMMARY"
