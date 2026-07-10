#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-stat-confirmation.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- stat-confirmation >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_stat_confirmation_v1"
  and .stat_choice_preview_contract == "trillionnium_world_bevy_stat_choice_preview_v1"
  and .stat_allocation_contract == "trillionnium_world_bevy_stat_allocation_v1"
  and .green == true
  and .event_acceptance_gate == true
  and .preview_is_non_mutating_gate == true
  and .confirm_cancel_button_gate == true
  and .cancel_restores_preview_gate == true
  and .confirm_spends_point_gate == true
  and (.character_texts.pending_force | contains("STAT CONFIRM | pending force"))
  and (.character_texts.pending_force | contains("Cost: 1 stat point"))
  and (.character_texts.pending_force | contains("FORCE pending -> rematch attack +4 DMG"))
  and (.character_texts.after_cancel | contains("STAT PREVIEW | choose one before spend"))
  and (.character_texts.after_confirm | contains("STAT BUILD ACTIVE | allocated craft"))
  and (.character_texts.after_confirm | contains("CRAFT active -> reward +4 coins +5 XP"))
  and .attrs.force_before == 11
  and .attrs.force_after_pending == 11
  and .attrs.agility_after_cancel == 12
  and .attrs.craft_before_confirm == 10
  and .attrs.craft_after_confirm == 11
  and .runtime_summaries.pending_force.growth_stat_points == 1
  and .runtime_summaries.pending_force.pending_stat_allocation == "force"
  and (.runtime_summaries.pending_force.allocated_stat_points | length) == 0
  and .runtime_summaries.after_cancel.growth_stat_points == 1
  and .runtime_summaries.after_cancel.pending_stat_allocation == null
  and (.runtime_summaries.after_cancel.allocated_stat_points | length) == 0
  and (.runtime_summaries.after_cancel.stat_confirmation_history | index("cancelled:agility")) != null
  and .runtime_summaries.after_confirm.growth_stat_points == 0
  and .runtime_summaries.after_confirm.pending_stat_allocation == null
  and (.runtime_summaries.after_confirm.allocated_stat_points | index("craft")) != null
  and (.runtime_summaries.after_confirm.stat_confirmation_history | index("confirmed:craft")) != null
  and .stat_button_states.pending_confirm.visual_state == "stat_confirmation_pending"
  and .stat_button_states.pending_confirm.enabled == true
  and .stat_button_states.pending_cancel.visual_state == "stat_confirmation_pending"
  and .stat_button_states.pending_cancel.enabled == true
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_STAT_CONFIRMATION_GREEN %s\n' "$SUMMARY"
