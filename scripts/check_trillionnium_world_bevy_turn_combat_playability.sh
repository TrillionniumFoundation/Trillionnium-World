#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-turn-combat-playability.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- turn-combat-playability >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_turn_combat_playability_v1"
  and .green == true
  and .running_state_sampling == true
  and (.bevy_window_button_path | contains("Interaction_Pressed"))
  and .combat_events_accepted == true
  and (.button_events[] | select(.action == "COMBAT:attack") | .accepted) == true
  and (.button_events[] | select(.action == "COMBAT:defend") | .accepted) == true
  and (.button_events[] | select(.action == "COMBAT:potion") | .accepted) == true
  and .final_runtime.combat_turn >= 5
  and .final_runtime.combat_result_state == "victory"
  and .final_runtime.enemy_hp == 0
  and .final_runtime.player_hp > 0
  and (.final_runtime.inventory_items | index("small_healing_pill")) == null
  and (.final_runtime.completed_steps | index("turn_combat_victory")) != null
  and (.final_runtime.completed_steps | index("resolve_first_combat")) != null
  and (.final_runtime.scene_history | index("turn_combat_scene_loaded")) != null
  and (.final_runtime.scene_history | index("turn_combat_returned_to_map")) != null
  and (.final_runtime.combat_round_log | length) >= 5
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_TURN_COMBAT_PLAYABILITY_GREEN %s\n' "$SUMMARY"
