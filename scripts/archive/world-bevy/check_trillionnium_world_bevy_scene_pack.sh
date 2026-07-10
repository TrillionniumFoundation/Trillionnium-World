#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-scene-pack.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- scene-pack >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_scene_pack_v1"
  and .green == true
  and .runtime_uses_scene_pack_helpers == true
  and .room_actor_definitions_complete == true
  and .task_room_definitions_complete == true
  and .component_pack_gate == true
  and .surface_pack_gate == true
  and .progression_pack_gate == true
  and .scene_pack.pack_id == "native_bevy_first_city_scene_pack_v1"
  and (.scene_pack.rooms | length) >= 8
  and (.scene_pack.actors | length) >= 7
  and (.scene_pack.tasks | length) >= 4
  and (.scene_pack.rooms[] | select(.room_id == "league-coliseum") | .task_ids | index("task-arena-rematch-route")) != null
  and (.scene_pack.actors[] | select(.actor_id == "npc-client-runner") | .room_id) == "client-board"
  and (.scene_pack.tasks[] | select(.task_id == "task-arena-rematch-route") | .unlock_rule) == "equip:bandit_sash"
  and (.component_actor_ids | index("npc-client-runner")) != null
  and (.final_runtime.unlocked_task_ids | index("task-arena-rematch-route")) != null
  and (.final_runtime.active_task_ids | index("task-arena-rematch-route")) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_SCENE_PACK_GREEN %s\n' "$SUMMARY"
