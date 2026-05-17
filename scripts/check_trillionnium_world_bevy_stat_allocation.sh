#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-stat-allocation.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- stat-allocation >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_stat_allocation_v1"
  and .inventory_equipment_contract == "trillionnium_world_bevy_inventory_equipment_v1"
  and .green == true
  and .event_acceptance_gate == true
  and .character_sheet_presence_gate == true
  and .point_award_gate == true
  and .stat_button_enabled_gate == true
  and .stat_confirmation_pending_gate == true
  and .stat_spend_gate == true
  and .character_attribute_mutation_gate == true
  and .stat_button_lock_after_gate == true
  and .force_before == 11
  and .force_pending == 11
  and .force_after == 12
  and .combat_power_hint_before == 46
  and .combat_power_hint_pending == 46
  and .combat_power_hint_after == 48
  and .android_s5_real_device_claimed == false
  and (.character_texts.after_equip | contains("CHARACTER SHEET | LV 2 | UNSPENT 1"))
  and (.character_texts.after_equip | contains("F11"))
  and (.character_texts.after_stat_pending | contains("STAT CONFIRM | pending force"))
  and (.character_texts.after_stat_pending | contains("Cost: 1 stat point"))
  and (.character_texts.after_stat_pending | contains("FORCE pending -> rematch attack +4 DMG"))
  and (.character_texts.after_stat | contains("CHARACTER SHEET | LV 2 | UNSPENT 0"))
  and (.character_texts.after_stat | contains("F12"))
  and (.character_texts.after_stat | contains("allocated force"))
  and (.final_runtime.allocated_stat_points | index("force")) != null
  and (.final_runtime.growth_history | index("stat:force:+1")) != null
  and (.final_runtime.progression_checkpoint_history | index("stat_allocated:force")) != null
  and .stat_button_states.pending_confirm.visual_state == "stat_confirmation_pending"
  and .stat_button_states.pending_cancel.visual_state == "stat_confirmation_pending"
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_STAT_ALLOCATION_GREEN %s\n' "$SUMMARY"
