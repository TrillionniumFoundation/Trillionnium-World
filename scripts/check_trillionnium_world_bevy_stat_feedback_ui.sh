#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-stat-feedback-ui.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- stat-feedback-ui >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_stat_feedback_ui_v1"
  and .stat_gameplay_effects_contract == "trillionnium_world_bevy_stat_gameplay_effects_v1"
  and .game_feedback_contract == "trillionnium_world_bevy_game_feedback_v1"
  and .green == true
  and .gameplay_effects_gate == true
  and .visible_force_feedback_gate == true
  and .visible_agility_feedback_gate == true
  and .visible_craft_settlement_gate == true
  and (.visible_feedback_samples.force_rematch.feedback_banner_text | contains("FORCE +4 DMG"))
  and (.visible_feedback_samples.force_rematch.enemy_text | contains("FLOAT DMG -18"))
  and (.visible_feedback_samples.force_rematch.combat_scene_text | contains("FORCE +4 DMG"))
  and (.visible_feedback_samples.agility_rematch.feedback_banner_text | contains("AGILITY -3 INCOMING"))
  and (.visible_feedback_samples.agility_rematch.enemy_text | contains("FLOAT DMG -14"))
  and (.visible_feedback_samples.agility_rematch.combat_scene_text | contains("AGILITY -3 INCOMING"))
  and (.visible_feedback_samples.craft_reward.reward_toast_text | contains("SETTLEMENT +14 coins +45 XP"))
  and (.visible_feedback_samples.craft_reward.reward_toast_text | contains("CRAFT +4 coins +5 XP"))
  and (.visible_feedback_samples.craft_reward.feedback_banner_text | contains("CRAFT +4 coins +5 XP"))
  and (.visible_feedback_samples.craft_reward.quest_panel_text | contains("CRAFT +4 coins +5 XP"))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_STAT_FEEDBACK_UI_GREEN %s\n' "$SUMMARY"
