#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-vertical-playable-slice.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- playable-slice >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_vertical_playable_slice_v1"
  and .rust_authority == "client_submits_intent_only_rust_world_command_layer_authoritative"
  and .projection_contract == "trillionnium_world_full_split_projection_v1"
  and (.current_room.node_id == "league-coliseum")
  and (.objective.task_id == "task-fixture-first-route")
  and (.visible_action_panels | length) >= 6
  and (.touch_controls | length) >= 10
  and (.input_contracts | index("bevy_ui_touch_buttons_to_rust_world_command_intent"))
  and (.persistence.round_trip_status == "save_restore_green")
  and (.world_after_scripted_loop.current_node_id == "league-coliseum")
  and (.world_after_scripted_loop.known_skill_ids | index("basic_unarmed"))
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_VERTICAL_SLICE_GREEN %s\n' "$SUMMARY"
