#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-stat-allocation-persistence.json"
SLOT_DIR="$EVIDENCE_DIR/bevy-stat-allocation-persistence-slots"
mkdir -p "$EVIDENCE_DIR" "$SLOT_DIR"
rm -f "$SLOT_DIR"/bevy-session-slot-*.snapshot.json

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_SESSION_SLOT_DIR="$SLOT_DIR" \
    cargo run -p trnm-world-bevy -- stat-allocation-persistence "$SLOT_DIR" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_stat_allocation_persistence_v1"
  and .stat_allocation_contract == "trillionnium_world_bevy_stat_allocation_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .expected_slot_dir == .action_slot_dir
  and .slot_a_bytes > 512
  and .event_acceptance_gate == true
  and .multi_stat_option_gate == true
  and .confirmation_pending_gate == true
  and .selected_stat_spend_gate == true
  and .stat_button_lock_after_gate == true
  and .save_after_stat_gate == true
  and .snapshot_persistence_gate == true
  and .load_resume_gate == true
  and .continue_restores_stat_gate == true
  and (.stat_options | index("STAT:force")) != null
  and (.stat_options | index("STAT:agility")) != null
  and (.stat_options | index("STAT:craft")) != null
  and .attrs_before.force == 11
  and .attrs_before.agility == 12
  and .attrs_before.craft == 10
  and .attrs_after_pending.agility == 12
  and .attrs_after_pending.combat_power_hint == 46
  and .attrs_after_stat.force == 11
  and .attrs_after_stat.agility == 13
  and .attrs_after_stat.craft == 10
  and .attrs_after_continue.agility == 13
  and (.stat_button_states.after_equip_force.enabled == true)
  and (.stat_button_states.after_equip_agility.enabled == true)
  and (.stat_button_states.after_equip_craft.enabled == true)
  and (.stat_button_states.after_pending_agility.reason == "stat_confirmation_pending:agility")
  and (.stat_button_states.pending_confirm.visual_state == "stat_confirmation_pending")
  and (.stat_button_states.pending_confirm.enabled == true)
  and (.stat_button_states.pending_cancel.visual_state == "stat_confirmation_pending")
  and (.stat_button_states.pending_cancel.enabled == true)
  and (.stat_button_states.after_stat_force.reason == "stat_point_required")
  and (.stat_button_states.after_stat_agility.reason == "stat_already_allocated:agility")
  and (.stat_button_states.after_stat_craft.reason == "stat_point_required")
  and (.character_texts.after_equip | contains("CHARACTER SHEET | LV 2 | UNSPENT 1"))
  and (.character_texts.after_pending | contains("STAT CONFIRM | pending agility"))
  and (.character_texts.after_pending | contains("AGILITY pending -> incoming -3 / guard +2 HP"))
  and (.character_texts.after_stat | contains("CHARACTER SHEET | LV 2 | UNSPENT 0"))
  and (.character_texts.after_stat | contains("A13"))
  and (.character_texts.after_stat | contains("allocated agility"))
  and (.character_texts.after_continue | contains("A13"))
  and (.character_texts.after_continue | contains("allocated agility"))
  and .snapshot_summary.growth_stat_points == 0
  and (.snapshot_summary.allocated_stat_points | index("agility")) != null
  and (.snapshot_summary.growth_history | index("stat:agility:+1")) != null
  and .snapshot_summary.agility == 13
  and (.final_runtime.allocated_stat_points | index("agility")) != null
  and .final_runtime.growth_stat_points == 0
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_state_snapshot_v1"
  and .actor_id == "local-player"
  and .first_playable.growth_stat_points == 0
  and (.first_playable.allocated_stat_points | index("agility")) != null
  and (.first_playable.growth_history | index("stat:agility:+1")) != null
  and .character.attributes.agility == 13
' "$SLOT_DIR/bevy-session-slot-a.snapshot.json" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_STAT_ALLOCATION_PERSISTENCE_GREEN %s slot_dir=%s\n' "$SUMMARY" "$SLOT_DIR"
