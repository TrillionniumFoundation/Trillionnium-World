#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-selector.json"
MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh" >/dev/null

(
  cd "$ROOT/trillionnium"
  TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST="$MANIFEST" \
    cargo run -p trnm-world-bevy -- classic-animation-selector >"$SUMMARY"
)

test -s "$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_animation_selector_v1"
  and .green == true
  and .case_count >= 6
  and .loaded_from_manifest == true
  and .atlas_parse_gate == true
  and .selector_case_gate == true
  and .selected_frame_manifest_gate == true
  and .animation_transition_gate == true
  and ([.cases[] | select(.case_id == "mentor_idle") | .selected_frame_id] | first) == "actor_mentor_idle"
  and ([.cases[] | select(.case_id == "mentor_dialogue_talk") | .selected_frame_id] | first) == "actor_mentor_talk"
  and ([.cases[] | select(.case_id == "enemy_idle") | .selected_frame_id] | first) == "actor_enemy_idle"
  and ([.cases[] | select(.case_id == "enemy_combat_attack") | .selected_frame_id] | first) == "actor_enemy_attack"
  and ([.cases[] | select(.case_id == "enemy_combat_hit") | .selected_frame_id] | first) == "actor_enemy_hit"
  and ([.cases[] | select(.case_id == "objective_marker_pulse") | .selected_frame_id] | first) == "marker_interaction"
  and ([.selected_frames[]] | index("actor_mentor_talk") != null)
  and ([.selected_frames[]] | index("actor_enemy_attack") != null)
  and ([.selected_frames[]] | index("actor_enemy_hit") != null)
  and ([.selected_frames[]] | index("marker_interaction") != null)
  and .cex_runtime_player_client_allowed == false
  and .wgpu_required == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_ANIMATION_SELECTOR_GREEN %s\n' "$SUMMARY"
