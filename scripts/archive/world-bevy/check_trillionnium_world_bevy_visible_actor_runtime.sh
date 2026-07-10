#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
mkdir -p "$EVIDENCE_DIR"

SUMMARY="$EVIDENCE_DIR/bevy-visible-actor-runtime.json"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- visible-actor-runtime >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_visible_actor_runtime_v1"
  and .green == true
  and .bevy_component == "BevyWorldVisibleActorRuntime"
  and .all_required_actor_components_visible == true
  and .drop_visible_before_loot == true
  and .drop_hidden_after_loot == true
  and .enemy_hp_zero_after_victory == true
  and .patrol_frames_advanced == true
  and (.required_actor_ids | index("npc-street-compass-sifu")) != null
  and (.required_actor_ids | index("npc-starter-artisan")) != null
  and (.required_actor_ids | index("npc-market-clerk")) != null
  and (.required_actor_ids | index("npc-client-runner")) != null
  and (.required_actor_ids | index("npc-dock-courier")) != null
  and (.required_actor_ids | index("enemy-market-bandit")) != null
  and (.required_actor_ids | index("drop-bandit-sash")) != null
  and (.button_events[] | select(.action == "NPC:npc-dock-courier") | .accepted) == true
  and (.samples[] | .visible_actor_runtimes | length >= 6)
  and .final_runtime.combat_result_state == "victory"
  and .final_runtime.enemy_hp == 0
  and (.final_runtime.loot_history | index("loot:bandit_sash")) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_VISIBLE_ACTOR_RUNTIME_GREEN %s\n' "$SUMMARY"
