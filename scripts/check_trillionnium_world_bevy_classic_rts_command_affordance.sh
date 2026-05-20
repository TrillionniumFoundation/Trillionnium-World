#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-affordance.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-affordance.ppm"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_art_pack.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-command-affordance "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_command_affordance_v1"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 720
  and .input_path == "apply_live_native_action_with_source(classic_rts_command_affordance_input)"
  and .input_action_count == 4
  and .accepted_input_count == 4
  and (.action_labels | index("RTS:SELECT:box:frontline") != null)
  and (.action_labels | index("RTS:MOVE:7,4:diamond") != null)
  and (.action_labels | index("RTS:ATTACK:arena_creep_attack") != null)
  and (.action_labels | index("RTS:ABILITY:guard_break") != null)
  and (.final_selected_unit_ids | length) >= 4
  and (.final_selection_box_tile_ids | length) >= 4
  and .final_attack_target_id == "arena_creep_attack"
  and .final_active_ability_id == "guard_break"
  and (.final_command_queue | map(select(startswith("box_select:"))) | length) >= 1
  and (.final_command_queue | index("attack:arena_creep_attack") != null)
  and (.final_command_queue | index("ability:guard_break") != null)
  and .drag_marquee_pixel_count > 80
  and .right_click_marker_pixel_count > 120
  and .attack_cursor_pixel_count > 120
  and .cursor_arrow_pixel_count > 60
  and .hotkey_pixel_count > 200
  and .command_ack_pixel_count > 160
  and .live_command_affordance_input_gate == true
  and .drag_select_gate == true
  and .right_click_move_gate == true
  and .attack_cursor_gate == true
  and .hotkey_ack_gate == true
  and .original_art_policy_gate == true
  and .warcraft_iii_asset_copied == false
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMMAND_AFFORDANCE_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
