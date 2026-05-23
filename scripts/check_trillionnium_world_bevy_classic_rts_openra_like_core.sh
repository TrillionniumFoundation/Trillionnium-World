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
  and (.rules[] | select(.id == "trnm.worker" and .supply_cost == 1))
  and (.rules[] | select(.id == "trnm.worker" and .vision_radius >= 5))
  and (.rules[] | select(.id == "trnm.horizon.scout" and .vision_radius >= 7))
  and (.rules[] | select(.id == "trnm.flux.relay" and ((.traits | index("refinery")) != null) and .vision_radius >= 5))
  and (.rules[] | select(.id == "trnm.flux.relay" and .supply_provided == 4))
  and (.rules[] | select(.id == "trnm.command.core" and ((.traits | index("producer")) != null) and ((.traits | index("provides_build_radius")) != null)))
  and (.rules[] | select(.id == "trnm.command.core" and .supply_provided == 8))
  and (.rules[] | select(.id == "trnm.flux.beacon" and ((.traits | index("capturable")) != null)))
  and (.rules[] | select(.id == "trnm.striker" and ((.traits | index("attack")) != null)))
  and (.rules[] | select(.id == "trnm.worker" and ((.traits | index("repair")) != null)))
  and ((.orders | index("move")) != null)
  and ((.orders | index("harvest")) != null)
  and ((.orders | index("build")) != null)
  and ((.orders | index("train")) != null)
  and ((.orders | index("capture")) != null)
  and ((.orders | index("attack")) != null)
  and ((.orders | index("repair")) != null)
  and .simulation.tick_count >= 320
  and .simulation.resource_delta > 0
  and .simulation.production_progress_percent > 0
  and .simulation.completed_production_count >= 2
  and .simulation.production_spawn_count >= 2
  and .simulation.production_rally_count >= 2
  and .simulation.multi0_supply_used > .simulation.multi0_initial_supply_used
  and .simulation.multi0_supply_cap > .simulation.multi0_initial_supply_cap
  and .simulation.multi0_supply_used <= .simulation.multi0_supply_cap
  and .simulation.supply_blocked_train_count > 0
  and .simulation.supply_cap_increase_count > 0
  and .simulation.relay_build_progress > 0
  and .simulation.beacon_capture_progress > 0
  and .simulation.combat_damage > 0
  and .simulation.worker_moved == true
  and .simulation.command_accepted_count >= 6
  and .simulation.command_rejected_count >= 6
  and .simulation.command_flux_spent >= 900
  and .simulation.producer_queue_gate == true
  and .simulation.producer_incomplete_gate == true
  and .simulation.tech_train_accept_gate == true
  and .simulation.supply_cap_gate == true
  and .simulation.build_placement_gate == true
  and .simulation.attack_range_gate == true
  and .simulation.attack_visibility_gate == true
  and .simulation.attack_hit_count > 0
  and .simulation.attack_kill_count > 0
  and .simulation.attack_cooldown_wait_count > 0
  and .simulation.multi1_command_core_removed == true
  and .simulation.auto_target_acquire_count > 0
  and .simulation.auto_attack_hit_count > 0
  and .simulation.auto_attack_kill_count > 0
  and .simulation.multi1_auto_raider_removed == true
  and .simulation.repair_command_gate == true
  and .simulation.repair_tick_count > 0
  and .simulation.repair_flux_spent > 0
  and .simulation.repair_complete_count > 0
  and .simulation.repaired_relay_hp == .simulation.repaired_relay_max_hp
  and .simulation.control_group_count >= 2
  and .simulation.queued_order_count >= 3
  and .simulation.queued_order_execute_count >= 3
  and .simulation.completed_queued_order_count >= 3
  and .simulation.path_plan_count >= 2
  and .simulation.move_path_step_count > 0
  and .simulation.blocked_move_count == 0
  and any(.simulation.path_plans[]; .actor_id == "multi0.line.0" and .target_tile.x == 16 and .target_tile.y == 9 and ((.blocked_tile_ids | index("16,16")) != null) and (.path_tile_ids | length) > 0)
  and any(.simulation.path_plans[]; .actor_id == "multi0.worker.0" and .target_tile.x == 12 and .target_tile.y == 16 and (.path_tile_ids | length) > 0)
  and any(.simulation.path_plans[]; .reached == true)
  and any(.simulation.command_log[]; contains("accepted:train:multi0.command.core"))
  and any(.simulation.command_log[]; contains("accepted:build:multi0.worker.1"))
  and any(.simulation.command_log[]; contains("rejected:build:multi0.worker.1:build_tile_blocked"))
  and any(.simulation.command_log[]; contains("accepted:attack:multi0.striker.0"))
  and any(.simulation.command_log[]; contains("accepted:capture:multi0.warden.capture"))
  and any(.simulation.command_log[]; contains("accepted:move:multi0.line.0"))
  and any(.simulation.command_log[]; contains("rejected:build:multi0.scout.intel:trait_missing:buildable"))
  and any(.simulation.command_log[]; contains("rejected:move:multi1.worker.0:owner_mismatch"))
  and any(.simulation.command_log[]; contains("rejected:train:multi0.assembly.pad:producer_incomplete"))
  and any(.simulation.command_log[]; contains("rejected:train:multi0.command.core:producer_queue_mismatch"))
  and any(.simulation.command_log[]; contains("rejected:train:multi0.command.core:supply_cap_reached"))
  and any(.simulation.command_log[]; contains("rejected:attack:multi0.worker.0:target_out_of_range"))
  and any(.simulation.command_log[]; contains("rejected:attack:multi0.line.0:target_not_visible"))
  and any(.simulation.command_log[]; contains("rejected:repair:multi0.worker.repair:repair_target_full"))
  and any(.simulation.command_log[]; contains("accepted:repair:multi0.worker.repair"))
  and any(.simulation.command_log[]; contains("accepted:train:multi0.assembly.pad"))
  and .simulation.multi0_visible_tile_count >= 120
  and .simulation.multi0_explored_tile_count > .simulation.multi0_visible_tile_count
  and ((.simulation.multi0_shroud_memory_actor_ids | index("multi1.command.core@25,25")) != null)
  and .shroud.visible_tile_count >= 120
  and .shroud.explored_tile_count > .shroud.visible_tile_count
  and .shroud.shroud_memory_count > 0
  and .shroud.shroud_memory_core_gate == true
  and .shroud.shroud_event_gate == true
  and any(.simulation.event_log[]; contains("move_step"))
  and any(.simulation.event_log[]; contains("path_step"))
  and any(.simulation.event_log[]; contains("path_plan"))
  and any(.simulation.event_log[]; contains("harvest_deposit"))
  and any(.simulation.event_log[]; contains("build_tick"))
  and any(.simulation.event_log[]; contains("train_tick"))
  and any(.simulation.event_log[]; contains("train_complete:multi0.command.core"))
  and any(.simulation.event_log[]; contains("supply_cap_increase:Multi0:multi0.flux.relay:trnm.flux.relay:+4"))
  and any(.simulation.event_log[]; contains("production_spawn:multi0.trained."))
  and any(.simulation.event_log[]; contains("rally_order:multi0.trained."))
  and any(.simulation.event_log[]; contains("capture_tick"))
  and any(.simulation.event_log[]; contains("attack_hit"))
  and any(.simulation.event_log[]; contains("attack_cooldown:multi0.striker.0"))
  and any(.simulation.event_log[]; contains("attack_kill:multi0.striker.0:multi1.command.core"))
  and any(.simulation.event_log[]; contains("attack_remove:multi1.command.core"))
  and any(.simulation.event_log[]; contains("auto_target_acquire:multi0.guard.sentinel:multi1.auto.raider"))
  and any(.simulation.event_log[]; contains("auto_attack_hit:multi0.guard.sentinel:multi1.auto.raider"))
  and any(.simulation.event_log[]; contains("auto_attack_kill:") and endswith(":multi1.auto.raider"))
  and any(.simulation.event_log[]; contains("attack_remove:multi1.auto.raider"))
  and any(.simulation.event_log[]; contains("repair_tick:multi0.worker.repair:multi0.damaged.relay"))
  and any(.simulation.event_log[]; contains("repair_complete:multi0.worker.repair:multi0.damaged.relay:70000hp"))
  and any(.simulation.event_log[]; contains("control_group_recall:Multi0:1"))
  and any(.simulation.event_log[]; contains("queued_group_order:Multi0:1:move"))
  and ([.simulation.event_log[] | select(contains("queued_order_execute:1:"))] | length) >= 3
  and any(.simulation.event_log[]; contains("vision_reveal:Multi0"))
  and any(.simulation.event_log[]; contains("shroud_memory:Multi0"))
  and .gates.map_gate == true
  and .gates.rule_trait_gate == true
  and .gates.order_gate == true
  and .gates.simulation_gate == true
  and .gates.command_resolution_gate == true
  and .gates.pathfinding_gate == true
  and .gates.move_path_plan_gate == true
  and .gates.harvest_path_plan_gate == true
  and .gates.reached_path_plan_gate == true
  and .gates.production_completion_gate == true
  and .gates.producer_queue_gate == true
  and .gates.producer_incomplete_gate == true
  and .gates.tech_train_accept_gate == true
  and .gates.tech_prerequisite_gate == true
  and .gates.supply_cap_gate == true
  and .gates.build_placement_gate == true
  and .gates.attack_weapon_gate == true
  and .gates.attack_range_gate == true
  and .gates.attack_visibility_gate == true
  and .gates.auto_target_acquisition_gate == true
  and .gates.repair_command_gate == true
  and .gates.repair_gate == true
  and .gates.control_group_gate == true
  and .gates.queued_order_gate == true
  and .gates.shroud_gate == true
  and .gates.source_policy_gate == true
  and .source_policy.no_openra_engine_code_copied == true
  and .source_policy.rust_bevy_owned_runtime == true
  and .source_policy.warcraft_iii_asset_copied == false
  and .source_policy.uses_trillionnium_owned_mod_data == true
' "$OUT" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_LIKE_CORE_GREEN %s\n' "$OUT"
