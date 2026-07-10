#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-stat-choice-preview.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- stat-choice-preview >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_stat_choice_preview_v1"
  and .stat_allocation_contract == "trillionnium_world_bevy_stat_allocation_v1"
  and .stat_gameplay_effects_contract == "trillionnium_world_bevy_stat_gameplay_effects_v1"
  and .green == true
  and .event_acceptance_gate == true
  and .preview_visible_gate == true
  and .stat_button_preview_gate == true
  and .force_active_preview_gate == true
  and .agility_active_preview_gate == true
  and .craft_active_preview_gate == true
  and (.preview_texts.before_spend | contains("STAT PREVIEW | choose one before spend"))
  and (.preview_texts.before_spend | contains("FORCE -> rematch attack +4 DMG"))
  and (.preview_texts.before_spend | contains("AGILITY -> incoming -3 / guard +2 HP"))
  and (.preview_texts.before_spend | contains("CRAFT -> reward +4 coins +5 XP"))
  and (.preview_texts.force_active | contains("STAT BUILD ACTIVE | allocated force"))
  and (.preview_texts.force_active | contains("FORCE active -> rematch attack +4 DMG"))
  and (.preview_texts.agility_active | contains("STAT BUILD ACTIVE | allocated agility"))
  and (.preview_texts.agility_active | contains("AGILITY active -> incoming -3 / guard +2 HP"))
  and (.preview_texts.craft_active | contains("STAT BUILD ACTIVE | allocated craft"))
  and (.preview_texts.craft_active | contains("CRAFT active -> reward +4 coins +5 XP"))
  and .runtime_summaries.before_spend.growth_stat_points == 1
  and (.runtime_summaries.before_spend.allocated_stat_points | length) == 0
  and (.stat_button_states.force.enabled == true)
  and (.stat_button_states.agility.enabled == true)
  and (.stat_button_states.craft.enabled == true)
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_STAT_CHOICE_PREVIEW_GREEN %s\n' "$SUMMARY"
