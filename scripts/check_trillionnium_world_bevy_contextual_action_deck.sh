#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-contextual-action-deck.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- contextual-action-deck >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_contextual_action_deck_v1"
  and .route_director_contract == "trillionnium_world_bevy_scene_pack_route_director_v1"
  and .green == true
  and .initial_gate == true
  and .forge_gate == true
  and .delivery_gate == true
  and .arena_gate == true
  and .rematch_gate == true
  and .runtime_history_gate == true
  and .all_pressed_accepted == true
  and (.input_samples[] | select(.stage == "initial") | .contextual_action_deck.primary_action_label) == "TALK"
  and (.input_samples[] | select(.stage == "after_ROOM:forge-workbench") | .contextual_action_deck.primary_action_label) == "TASK:task-starter-forge-intro"
  and (.input_samples[] | select(.stage == "after_ROOM:league-coliseum") | .contextual_action_deck.actions[] | select(.label == "TASK:task-arena-rematch-route") | .enabled) == false
  and (.input_samples[] | select(.stage == "after_EQUIP:bandit_sash") | .contextual_action_deck.primary_action_label) == "TASK:task-arena-rematch-route"
  and (.final_runtime.contextual_action_history | index("contextual_deck:league-coliseum:TASK:task-arena-rematch-route")) != null
  and (.final_runtime.blocked_action_history | length) == 0
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CONTEXTUAL_ACTION_DECK_GREEN %s\n' "$SUMMARY"
