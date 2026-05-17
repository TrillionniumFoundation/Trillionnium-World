#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-progression-loop.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- progression-loop >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_progression_loop_v1"
  and .visible_actor_runtime_contract == "trillionnium_world_bevy_visible_actor_runtime_v1"
  and .green == true
  and .progression_loop_gate == true
  and .required_checkpoints_green == true
  and .rematch_was_enabled_after_equipment == true
  and (.button_events[] | select(.action == "TASK:task-arena-rematch-route") | .accepted) == true
  and (.required_checkpoints | index("combat_victory")) != null
  and (.required_checkpoints | index("loot:bandit_sash")) != null
  and (.required_checkpoints | index("equip:bandit_sash")) != null
  and (.required_checkpoints | index("unlock_next_task:task-arena-rematch-route")) != null
  and (.required_checkpoints | index("start_task:task-arena-rematch-route")) != null
  and .final_runtime.progression_loop_count >= 1
  and (.final_runtime.progression_checkpoint_history | index("start_task:task-arena-rematch-route")) != null
  and (.final_runtime.unlocked_task_ids | index("task-arena-rematch-route")) != null
  and (.final_runtime.active_task_ids | index("task-arena-rematch-route")) != null
  and (.final_runtime.equipped_items | index("bandit_sash")) != null
  and .final_runtime.growth_level >= 2
  and .final_runtime.growth_stat_points >= 1
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_PROGRESSION_LOOP_GREEN %s\n' "$SUMMARY"
