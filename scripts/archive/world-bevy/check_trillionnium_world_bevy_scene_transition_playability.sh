#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-scene-transition-playability.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- scene-transition-playability >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_scene_transition_playability_v1"
  and .green == true
  and (.scene_history | index("dialogue_overlay_opened"))
  and (.scene_history | index("mentor_training_room_indoor_tilemap"))
  and (.scene_history | index("tile_to_tile_interpolation"))
  and (.scene_history | index("arena_outdoor_tilemap"))
  and (.scene_history | index("combat_encounter_overlay"))
  and (.scene_history | index("combat_returned_to_map"))
  and (.movement_gate.from_tile == "mentor_room_exit_tile")
  and (.movement_gate.to_tile == "arena_gate_tile")
  and (.movement_gate.progress_percent == 100)
  and (.movement_gate.input_lock_observed == true)
  and (.movement_gate.input_locked_final == false)
  and (.movement_gate.facing_direction == "north")
  and (.movement_gate.player_sprite_pose | startswith("walk_north_frame_"))
  and (.movement_gate.interpolation_samples | length) >= 3
  and (.npc_behavior_gate.mentor_history | index("dialogue_choice_opened"))
  and (.npc_behavior_gate.mentor_history | index("training_feedback_visible"))
  and (.npc_behavior_gate.enemy_history | index("enemy_visible_at_arena"))
  and (.npc_behavior_gate.enemy_history | index("enemy_damage_feedback_visible"))
  and (.npc_behavior_gate.enemy_history | index("enemy_hp_changed"))
  and (.npc_behavior_gate.enemy_hp < 40)
  and (.visible_scene_contracts | index("talk_dialogue_overlay"))
  and (.visible_scene_contracts | index("train_indoor_training_room_tilemap"))
  and (.visible_scene_contracts | index("move_tile_to_tile_interpolation"))
  and (.visible_scene_contracts | index("move_input_lock_observed_and_released"))
  and (.visible_scene_contracts | index("fight_combat_overlay"))
  and (.visible_scene_contracts | index("fight_return_to_map"))
  and (.visible_scene_contracts | index("mentor_dialogue_bubble"))
  and (.visible_scene_contracts | index("enemy_damage_feedback"))
  and (.screenshot_manifest.town | endswith("bevy-scene-town.png"))
  and (.screenshot_manifest.dialogue | endswith("bevy-scene-dialogue.png"))
  and (.screenshot_manifest.training_room | endswith("bevy-scene-training-room.png"))
  and (.screenshot_manifest.combat | endswith("bevy-scene-combat.png"))
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_SCENE_TRANSITION_PLAYABILITY_GREEN %s\n' "$SUMMARY"
