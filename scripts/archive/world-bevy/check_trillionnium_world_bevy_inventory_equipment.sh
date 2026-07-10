#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-inventory-equipment.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- inventory-equipment >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_inventory_equipment_v1"
  and .quest_journal_contract == "trillionnium_world_bevy_quest_journal_v1"
  and .green == true
  and .event_acceptance_gate == true
  and .inventory_panel_presence_gate == true
  and .initial_bag_gate == true
  and .victory_drop_gate == true
  and .bag_open_gate == true
  and .loot_collection_gate == true
  and .equip_gate == true
  and .rematch_unlock_gate == true
  and .android_s5_real_device_claimed == false
  and (.inventory_texts.initial | contains("BAG HUD | STATUS CLOSED"))
  and (.inventory_texts.initial | contains("small_healing_pill"))
  and (.inventory_texts.after_victory | contains("DROPS: bandit_sash"))
  and (.inventory_texts.after_bag_open | contains("BAG HUD | STATUS OPEN"))
  and (.inventory_texts.after_bag_open | contains("bag_overlay_visible"))
  and (.inventory_texts.after_loot | contains("BAG ITEMS: bandit_sash"))
  and (.inventory_texts.after_loot | contains("accessory bandit_sash:in_bag"))
  and (.inventory_texts.after_equip | contains("accessory bandit_sash:equipped"))
  and (.inventory_texts.after_equip | contains("AFFIX: AFFIX dormant"))
  and (.inventory_texts.after_equip | contains("equipment ready"))
  and (.final_runtime.equipped_items | index("bandit_sash")) != null
  and (.final_runtime.inventory_items | index("bandit_sash")) == null
  and (.final_runtime.unlocked_task_ids | index("task-arena-rematch-route")) != null
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_INVENTORY_EQUIPMENT_GREEN %s\n' "$SUMMARY"
