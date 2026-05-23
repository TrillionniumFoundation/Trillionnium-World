#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-like-core.json"
mkdir -p "$(dirname "$OUT")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-openra-like-core >"$OUT"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_like_core_v1"
  and .green == true
  and .runtime_model == "rust_bevy_owned_openra_like_rts_core"
  and .map.width == 34
  and .map.height == 34
  and .map.bounds.x == 1
  and .map.bounds.y == 1
  and .map.bounds.width == 32
  and .map.bounds.height == 32
  and .map.actor_template_count == 39
  and .map.runtime_actor_count >= 48
  and .map.player_count == 4
  and (.rules[] | select(.id == "trnm.worker" and ((.traits | index("harvester")) != null)))
  and (.rules[] | select(.id == "trnm.flux.relay" and ((.traits | index("refinery")) != null)))
  and (.rules[] | select(.id == "trnm.command.core" and ((.traits | index("producer")) != null) and ((.traits | index("provides_build_radius")) != null)))
  and (.rules[] | select(.id == "trnm.flux.beacon" and ((.traits | index("capturable")) != null)))
  and (.rules[] | select(.id == "trnm.striker" and ((.traits | index("attack")) != null)))
  and ((.orders | index("move")) != null)
  and ((.orders | index("harvest")) != null)
  and ((.orders | index("build")) != null)
  and ((.orders | index("train")) != null)
  and ((.orders | index("capture")) != null)
  and ((.orders | index("attack")) != null)
  and .simulation.tick_count >= 32
  and .simulation.resource_delta > 0
  and .simulation.production_progress_percent > 0
  and .simulation.relay_build_progress > 0
  and .simulation.beacon_capture_progress > 0
  and .simulation.combat_damage > 0
  and .simulation.worker_moved == true
  and any(.simulation.event_log[]; contains("move_step"))
  and any(.simulation.event_log[]; contains("harvest_deposit"))
  and any(.simulation.event_log[]; contains("build_tick"))
  and any(.simulation.event_log[]; contains("train_tick"))
  and any(.simulation.event_log[]; contains("capture_tick"))
  and any(.simulation.event_log[]; contains("attack_hit"))
  and .gates.map_gate == true
  and .gates.rule_trait_gate == true
  and .gates.order_gate == true
  and .gates.simulation_gate == true
  and .gates.source_policy_gate == true
  and .source_policy.no_openra_engine_code_copied == true
  and .source_policy.rust_bevy_owned_runtime == true
  and .source_policy.warcraft_iii_asset_copied == false
  and .source_policy.uses_trillionnium_owned_mod_data == true
' "$OUT" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_LIKE_CORE_GREEN %s\n' "$OUT"
