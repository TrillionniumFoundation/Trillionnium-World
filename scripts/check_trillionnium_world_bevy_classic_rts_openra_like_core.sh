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
  and (.rules[] | select(.id == "trnm.flux.relay" and .power_provided == 6))
  and (.rules[] | select(.id == "trnm.command.core" and ((.traits | index("producer")) != null) and ((.traits | index("provides_build_radius")) != null)))
  and (.rules[] | select(.id == "trnm.command.core" and .supply_provided == 8))
  and (.rules[] | select(.id == "trnm.command.core" and .power_provided == 10))
  and (.rules[] | select(.id == "trnm.signal.array" and .power_draw == 12))
  and (.rules[] | select(.id == "trnm.flux.beacon" and ((.traits | index("capturable")) != null)))
  and (.rules[] | select(.id == "trnm.striker" and ((.traits | index("attack")) != null)))
  and (.rules[] | select(.id == "trnm.worker" and ((.traits | index("repair")) != null)))
  and ((.orders | index("move")) != null)
  and ((.orders | index("attack_move")) != null)
  and ((.orders | index("patrol")) != null)
  and ((.orders | index("stop")) != null)
  and ((.orders | index("harvest")) != null)
  and ((.orders | index("return_cargo")) != null)
  and ((.orders | index("build")) != null)
  and ((.orders | index("train")) != null)
  and ((.orders | index("capture")) != null)
  and ((.orders | index("attack")) != null)
  and ((.orders | index("focus_fire")) != null)
  and ((.orders | index("repair")) != null)
  and .simulation.tick_count >= 320
  and .simulation.resource_delta > 0
  and .simulation.harvested_resource_amount > 0
  and .simulation.resource_depleted_count > 0
  and .simulation.harvest_return_trip_count > 0
  and .simulation.harvest_dropoff_count > 0
  and .simulation.harvest_deposited_amount == .simulation.harvested_resource_amount
  and .simulation.resource_depletion_gate == true
  and .simulation.harvest_return_cargo_gate == true
  and .simulation.production_progress_percent > 0
  and .simulation.completed_production_count >= 2
  and .simulation.production_spawn_count >= 2
  and .simulation.production_rally_count >= 2
  and .simulation.production_cancel_count >= 1
  and .simulation.production_refund_amount >= 100
  and .simulation.production_cancel_gate == true
  and .simulation.production_hold_count >= 1
  and .simulation.production_resume_count >= 1
  and .simulation.production_hold_tick_count >= 8
  and .simulation.production_pause_resume_gate == true
  and any(.snapshot.production[]; .owner == "Multi0" and .producer_id == "multi0.command.core" and .rule_id == "trnm.worker" and .canceled == true and .completed == false and .spawned_actor_id == null)
  and any(.snapshot.production[]; .owner == "Multi0" and .producer_id == "multi0.assembly.pad" and .rule_id == "trnm.striker" and .paused == false and .completed == true and .spawned_actor_id != null)
  and .simulation.multi0_supply_used > .simulation.multi0_initial_supply_used
  and .simulation.multi0_supply_cap > .simulation.multi0_initial_supply_cap
  and .simulation.multi0_supply_used <= .simulation.multi0_supply_cap
  and .simulation.supply_blocked_train_count > 0
  and .simulation.supply_cap_increase_count > 0
  and .simulation.multi2_power_used == 12
  and .simulation.multi2_power_provided >= 16
  and .simulation.multi2_low_power_ticks > 0
  and .simulation.low_power_production_pause_count > 0
  and .simulation.power_recovery_count > 0
  and .simulation.relay_build_progress > 0
  and .simulation.beacon_capture_progress == 100
  and .simulation.capture_beacon_owner == "Multi0"
  and .simulation.capture_path_step_count > 0
  and .simulation.capture_contested_tick_count > 0
  and .simulation.capture_resume_count == 1
  and .simulation.capture_complete_count >= 2
  and .simulation.capture_income_tick_count >= 2
  and .simulation.capture_income_amount >= 150
  and .simulation.capture_contested_gate == true
  and .simulation.contested_beacon_capture_progress == 100
  and .simulation.contested_beacon_capture_owner == "Multi0"
  and .simulation.capture_objective_gate == true
  and .simulation.combat_damage > 0
  and .simulation.worker_moved == true
  and .simulation.command_accepted_count >= 6
  and .simulation.command_rejected_count >= 6
  and .simulation.command_flux_spent >= 900
  and .simulation.producer_queue_gate == true
  and .simulation.producer_incomplete_gate == true
  and .simulation.tech_train_accept_gate == true
  and .simulation.supply_cap_gate == true
  and .simulation.power_low_production_gate == true
  and .simulation.build_placement_gate == true
  and .simulation.attack_move_command_gate == true
  and .simulation.attack_move_gate == true
  and .simulation.attack_range_gate == true
  and .simulation.attack_visibility_gate == true
  and .simulation.attack_hit_count > 0
  and .simulation.attack_kill_count > 0
  and .simulation.attack_cooldown_wait_count > 0
  and .simulation.veterancy_kill_credit_count >= 2
  and .simulation.veterancy_rank_up_count >= 2
  and .simulation.veteran_damage_bonus_count >= 1
  and .simulation.veteran_warden_rank >= 3
  and .simulation.veteran_warden_kill_count >= 2
  and .simulation.veteran_first_target_removed == true
  and .simulation.veteran_second_target_removed == true
  and .simulation.veterancy_gate == true
  and .simulation.attack_move_step_count > 0
  and .simulation.attack_move_engage_count > 0
  and .simulation.attack_move_hit_count > 0
  and .simulation.attack_move_kill_count > 0
  and .simulation.patrol_command_gate == true
  and .simulation.patrol_gate == true
  and .simulation.patrol_step_count >= 3
  and .simulation.patrol_turn_count >= 2
  and .simulation.focus_fire_command_gate == true
  and .simulation.focus_fire_gate == true
  and .simulation.focus_fire_hit_count >= 2
  and .simulation.focus_fire_kill_count > 0
  and .simulation.target_priority_gate == true
  and .simulation.target_priority_acquire_count > 0
  and .simulation.stop_command_gate == true
  and .simulation.stop_gate == true
  and .simulation.stop_order_count > 0
  and .simulation.stop_cleared_patrol_count > 0
  and .simulation.stance_behavior_gate == true
  and .simulation.stance_hold_fire_suppressed_count > 0
  and .simulation.stance_guard_leash_hold_count > 0
  and .simulation.stance_aggressive_pursuit_count > 0
  and .simulation.stance_aggressive_hit_count > 0
  and .simulation.hold_fire_target_hp == .simulation.hold_fire_target_initial_hp
  and .simulation.guard_stance_tile.x == 21
  and .simulation.guard_stance_tile.y == 16
  and (.simulation.aggressive_stance_tile.x != 21 or .simulation.aggressive_stance_tile.y != 18)
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
  and .simulation.queued_order_count >= 17
  and .simulation.queued_order_execute_count >= 14
  and .simulation.queued_order_cancel_count >= 2
  and .simulation.queued_order_chain_ready_count >= 1
  and .simulation.queued_order_chain_reach_count >= 2
  and .simulation.queued_order_override_count >= 1
  and .simulation.queued_order_override_cleared_count >= 2
  and .simulation.queued_order_reject_count >= 1
  and .simulation.completed_queued_order_count >= 14
  and .simulation.canceled_queued_order_count >= 4
  and .simulation.queued_order_cancel_gate == true
  and .simulation.queued_order_chain_gate == true
  and .simulation.queued_order_chain_tile.x == 6
  and .simulation.queued_order_chain_tile.y == 31
  and .simulation.queued_order_override_gate == true
  and .simulation.queued_order_override_tile.x == 31
  and .simulation.queued_order_override_tile.y == 29
  and .simulation.queued_order_validation_gate == true
  and .simulation.queued_order_validation_tile.x == 15
  and .simulation.queued_order_validation_tile.y == 31
  and .simulation.formation_move_slot_count >= 6
  and .simulation.formation_move_reached_count >= 6
  and .simulation.formation_move_reassigned_slot_count >= 1
  and (.simulation.formation_move_slot_tiles | length) >= 3
  and (.simulation.formation_move_actor_tiles | length) >= 3
  and (.simulation.formation_move_blocked_slot_tiles | length) >= 3
  and (.simulation.formation_move_blocked_actor_tiles | length) >= 3
  and .simulation.formation_move_group_order_count >= 2
  and .simulation.formation_clean_gate == true
  and .simulation.formation_blocked_reassign_gate == true
  and .simulation.formation_move_gate == true
  and .simulation.local_obstruction_detect_count > 0
  and .simulation.local_obstruction_hold_count > 0
  and .simulation.local_obstruction_side_step_count > 0
  and .simulation.local_obstruction_gap_claim_count > 0
  and .simulation.local_obstruction_resume_count > 0
  and ((.simulation.local_obstruction_actor_tiles | map(select(.x == 6 and .y == 2)) | length) == 1)
  and ((.simulation.local_obstruction_actor_tiles | map(select(.x == 5 and .y == 2)) | length) == 1)
  and .simulation.local_obstruction_blocker_tile.x == 3
  and .simulation.local_obstruction_blocker_tile.y == 3
  and .simulation.local_obstruction_recovery_gate == true
  and .simulation.path_reservation_attempt_count >= 4
  and .simulation.path_reservation_grant_count >= 3
  and .simulation.path_reservation_wait_count >= 1
  and .simulation.path_reservation_collision_avoidance_count >= 1
  and .simulation.path_reservation_lead_tile.x == 2
  and .simulation.path_reservation_lead_tile.y == 5
  and .simulation.path_reservation_wing_tile.x == 3
  and .simulation.path_reservation_wing_tile.y == 4
  and .simulation.path_reservation_gate == true
  and .simulation.traffic_deadlock_detect_count >= 1
  and .simulation.traffic_deadlock_yield_count >= 1
  and .simulation.traffic_deadlock_resume_count >= 1
  and .simulation.traffic_lead_tile.x == 6
  and .simulation.traffic_lead_tile.y == 6
  and .simulation.traffic_yield_tile.x == 4
  and .simulation.traffic_yield_tile.y == 6
  and .simulation.traffic_deadlock_recovery_gate == true
  and .simulation.traffic_stuck_wait_count >= 2
  and .simulation.traffic_stuck_timeout_count >= 1
  and .simulation.traffic_stuck_side_step_count >= 1
  and .simulation.traffic_stuck_resume_count >= 1
  and .simulation.traffic_stuck_runner_tile.x == 12
  and .simulation.traffic_stuck_runner_tile.y == 2
  and .simulation.traffic_stuck_blocker_tile.x == 9
  and .simulation.traffic_stuck_blocker_tile.y == 3
  and .simulation.traffic_stuck_timeout_gate == true
  and .simulation.path_plan_count >= 2
  and .simulation.move_path_step_count > 0
  and .simulation.blocked_move_count == 0
  and any(.simulation.path_plans[]; .actor_id == "multi0.line.0" and .target_tile.x == 16 and .target_tile.y == 9 and ((.blocked_tile_ids | index("16,16")) != null) and (.path_tile_ids | length) > 0)
  and any(.simulation.path_plans[]; .actor_id == "multi0.worker.0" and .target_tile.x == 10 and .target_tile.y == 10 and (.path_tile_ids | length) > 0)
  and any(.simulation.path_plans[]; .reached == true)
  and any(.simulation.command_log[]; contains("accepted:train:multi0.command.core"))
  and any(.simulation.command_log[]; contains("accepted:build:multi0.worker.1"))
  and any(.simulation.command_log[]; contains("accepted:attack_move:multi0.attackmove.warden"))
  and any(.simulation.command_log[]; contains("accepted:attack:multi0.veteran.warden"))
  and any(.simulation.command_log[]; contains("rejected:attack_move:multi0.command.core:trait_missing:mobile"))
  and any(.simulation.command_log[]; contains("accepted:patrol:multi0.patrol.warden"))
  and any(.simulation.command_log[]; contains("rejected:patrol:multi0.command.core:trait_missing:mobile"))
  and any(.simulation.command_log[]; contains("accepted:focus_fire:multi0.focus.warden.a"))
  and any(.simulation.command_log[]; contains("accepted:focus_fire:multi0.focus.warden.b"))
  and any(.simulation.command_log[]; contains("accepted:stop:multi0.stop.warden"))
  and any(.simulation.command_log[]; contains("rejected:stop:multi1.worker.0:owner_mismatch"))
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
  and any(.simulation.event_log[]; contains("resource_harvested:multi0.worker.0:map.actor10"))
  and any(.simulation.event_log[]; contains("resource_depleted:map.actor10:trnm.flux.bloom"))
  and any(.simulation.event_log[]; contains("harvest_return_start:multi0.worker.0:"))
  and any(.simulation.event_log[]; contains("harvest_deposit_at:multi0.worker.0:"))
  and any(.simulation.event_log[]; contains("harvest_resume:multi0.worker.0:map.actor10"))
  and any(.simulation.event_log[]; contains("harvest_cycle_complete:multi0.worker.0:map.actor10"))
  and any(.simulation.event_log[]; contains("harvest_deposit"))
  and any(.simulation.event_log[]; contains("build_tick"))
  and any(.simulation.event_log[]; contains("train_tick"))
  and any(.simulation.event_log[]; contains("production_cancel:Multi0:multi0.command.core:trnm.worker:refund100"))
  and any(.simulation.event_log[]; contains("production_hold:Multi0:multi0.assembly.pad:trnm.striker"))
  and any(.simulation.event_log[]; contains("production_paused_manual:Multi0:multi0.assembly.pad:trnm.striker"))
  and any(.simulation.event_log[]; contains("production_resume:Multi0:multi0.assembly.pad:trnm.striker"))
  and any(.simulation.event_log[]; contains("train_complete:multi0.command.core"))
  and any(.simulation.event_log[]; contains("supply_cap_increase:Multi0:multi0.flux.relay:trnm.flux.relay:+4"))
  and any(.simulation.event_log[]; contains("low_power_tick:Multi2:"))
  and any(.simulation.event_log[]; contains("production_paused_low_power:Multi2:multi2.signal.array:trnm.horizon.scout"))
  and any(.simulation.event_log[]; contains("power_recovered:Multi2:"))
  and any(.simulation.event_log[]; contains("production_spawn:multi2.trained.horizon_scout"))
  and any(.simulation.event_log[]; contains("production_spawn:multi0.trained."))
  and any(.simulation.event_log[]; contains("rally_order:multi0.trained."))
  and any(.simulation.event_log[]; contains("capture_path_step:multi0.warden.capture:15,9->16,9"))
  and any(.simulation.event_log[]; contains("capture_contested:multi0.capture.contested.warden:map.capture.contested.node:multi1.capture.contester:30,2"))
  and any(.simulation.event_log[]; contains("capture_contest_clear:multi1.capture.contester:30,3->32,3"))
  and any(.simulation.event_log[]; contains("capture_resume:multi0.capture.contested.warden:map.capture.contested.node:30,2"))
  and any(.simulation.event_log[]; contains("capture_tick"))
  and any(.simulation.event_log[]; contains("capture_complete:multi0.warden.capture:map.actor15:Multi0:100%"))
  and any(.simulation.event_log[]; contains("capture_complete:multi0.capture.contested.warden:map.capture.contested.node:Multi0:100%"))
  and any(.simulation.event_log[]; contains("capture_income:Multi0:map.actor15:trnm.flux.beacon:+75"))
  and any(.simulation.event_log[]; contains("capture_income:Multi0:map.capture.contested.node:trnm.flux.beacon:+75"))
  and any(.simulation.event_log[]; contains("attack_hit"))
  and any(.simulation.event_log[]; contains("attack_cooldown:multi0.striker.0"))
  and any(.simulation.event_log[]; contains("attack_kill:multi0.striker.0:multi1.command.core"))
  and any(.simulation.event_log[]; contains("attack_remove:multi1.command.core"))
  and any(.simulation.event_log[]; contains("attack_kill:multi0.veteran.warden:multi1.veteran.first"))
  and any(.simulation.event_log[]; contains("veteran_kill_credit:multi0.veteran.warden:multi1.veteran.first:kills1"))
  and any(.simulation.event_log[]; contains("veteran_rank_up:multi0.veteran.warden:rank1->rank2"))
  and any(.simulation.event_log[]; contains("veteran_damage_bonus:multi0.veteran.warden:rank2:700->840"))
  and any(.simulation.event_log[]; contains("attack_kill:multi0.veteran.warden:multi1.veteran.second"))
  and any(.simulation.event_log[]; contains("attack_remove:multi1.veteran.second"))
  and any(.simulation.event_log[]; contains("attack_move_step:multi0.attackmove.warden"))
  and any(.simulation.event_log[]; contains("attack_move_engage:multi0.attackmove.warden:multi1.attackmove.raider"))
  and any(.simulation.event_log[]; contains("attack_move_hit:multi0.attackmove.warden:multi1.attackmove.raider"))
  and any(.simulation.event_log[]; contains("attack_move_kill:multi0.attackmove.warden:multi1.attackmove.raider"))
  and any(.simulation.event_log[]; contains("attack_remove:multi1.attackmove.raider"))
  and any(.simulation.event_log[]; contains("attack_move_reached:multi0.attackmove.warden"))
  and any(.simulation.event_log[]; contains("patrol_step:multi0.patrol.warden"))
  and any(.simulation.event_log[]; contains("patrol_turn:multi0.patrol.warden"))
  and any(.simulation.event_log[]; contains("focus_fire_hit:multi0.focus.warden.a:multi1.focus.target"))
  and any(.simulation.event_log[]; contains("focus_fire_hit:multi0.focus.warden.b:multi1.focus.target"))
  and any(.simulation.event_log[]; contains("focus_fire_kill:") and endswith(":multi1.focus.target"))
  and any(.simulation.event_log[]; contains("target_priority_acquire:multi0.priority.guard:multi1.priority.lowhp"))
  and any(.simulation.event_log[]; contains("stop_order:multi0.stop.warden:patrol->hold"))
  and any(.simulation.event_log[]; contains("stance_hold_fire_suppress:multi0.stance.holdfire:multi1.stance.holdfire.target"))
  and any(.simulation.event_log[]; contains("stance_guard_leash_hold:multi0.stance.guard:multi1.stance.guard.target"))
  and any(.simulation.event_log[]; contains("stance_aggressive_pursue:multi0.stance.aggressive:multi1.stance.aggressive.target"))
  and any(.simulation.event_log[]; contains("stance_auto_attack:aggressive:multi0.stance.aggressive:multi1.stance.aggressive.target"))
  and any(.simulation.event_log[]; contains("auto_target_acquire:multi0.guard.sentinel:multi1.auto.raider"))
  and any(.simulation.event_log[]; contains("auto_attack_hit:multi0.guard.sentinel:multi1.auto.raider"))
  and any(.simulation.event_log[]; contains("auto_attack_kill:") and endswith(":multi1.auto.raider"))
  and any(.simulation.event_log[]; contains("attack_remove:multi1.auto.raider"))
  and any(.simulation.event_log[]; contains("repair_tick:multi0.worker.repair:multi0.damaged.relay"))
  and any(.simulation.event_log[]; contains("repair_complete:multi0.worker.repair:multi0.damaged.relay:70000hp"))
  and any(.simulation.event_log[]; contains("control_group_recall:Multi0:1"))
  and any(.simulation.event_log[]; contains("queued_group_order:Multi0:1:move"))
  and any(.simulation.event_log[]; contains("queued_group_order:Multi0:4:move"))
  and any(.simulation.event_log[]; contains("queued_group_order:Multi0:5:move"))
  and any(.simulation.event_log[]; contains("queued_order_cancel:Multi0:4:2orders"))
  and ([.simulation.event_log[] | select(contains("queued_order_execute:1:"))] | length) >= 3
  and ([.simulation.event_log[] | select(contains("queued_order_execute:4:"))] | length) == 0
  and any(.simulation.event_log[]; contains("queued_order_execute:5:multi0.chain.runner:move:chain0"))
  and any(.simulation.event_log[]; contains("queued_order_chain_ready:5:multi0.chain.runner:chain1"))
  and any(.simulation.event_log[]; contains("queued_order_execute:5:multi0.chain.runner:move:chain1"))
  and any(.simulation.event_log[]; contains("queued_order_reached:5:multi0.chain.runner:chain0:4,31"))
  and any(.simulation.event_log[]; contains("queued_order_reached:5:multi0.chain.runner:chain1:6,31"))
  and any(.simulation.event_log[]; contains("queued_group_order:Multi0:6:move"))
  and any(.simulation.event_log[]; contains("queued_order_execute:6:multi0.override.runner:move:chain0"))
  and any(.simulation.event_log[]; contains("queued_order_override:Multi0:multi0.override.runner:move:cleared2"))
  and ([.simulation.event_log[] | select(contains("queued_order_chain_ready:6:"))] | length) == 0
  and ([.simulation.event_log[] | select(contains("queued_order_execute:6:multi0.override.runner:move:chain1"))] | length) == 0
  and ([.simulation.event_log[] | select(contains("queued_order_reached:6:"))] | length) == 0
  and any(.simulation.event_log[]; contains("queued_actor_order_rejected:Multi0:13:multi0.queue.reject.runner:move:path_unreachable:16,16"))
  and any(.simulation.event_log[]; contains("queued_actor_order:Multi0:13:multi0.queue.reject.runner:move:chain0"))
  and any(.simulation.event_log[]; contains("queued_order_execute:13:multi0.queue.reject.runner:move:chain0"))
  and any(.simulation.event_log[]; contains("queued_order_reached:13:multi0.queue.reject.runner:chain0:15,31"))
  and ([.snapshot.queued_orders[] | select(.group_id == "13" and .actor_id == "multi0.queue.reject.runner" and .target_tile.x == 16 and .target_tile.y == 16)] | length) == 0
  and any(.simulation.event_log[]; contains("formation_group_order:Multi0:7:22,31:3slots:0reassigned"))
  and any(.simulation.event_log[]; contains("formation_move_slot:Multi0:7:multi0.formation.lead:slot0:22,31->22,31"))
  and any(.simulation.event_log[]; contains("formation_move_slot:Multi0:7:multi0.formation.left:slot1:22,31->21,31"))
  and any(.simulation.event_log[]; contains("formation_move_slot:Multi0:7:multi0.formation.right:slot2:22,31->23,31"))
  and ([.simulation.event_log[] | select(contains("queued_order_execute:7:"))] | length) >= 3
  and any(.simulation.event_log[]; contains("formation_move_reached:7:multi0.formation.lead:22,31"))
  and any(.simulation.event_log[]; contains("formation_move_reached:7:multi0.formation.left:21,31"))
  and any(.simulation.event_log[]; contains("formation_move_reached:7:multi0.formation.right:23,31"))
  and any(.simulation.event_log[]; contains("formation_group_order:Multi0:8:10,31:3slots:1reassigned"))
  and any(.simulation.event_log[]; contains("formation_move_slot:Multi0:8:multi0.formation.blocked.lead:slot0:10,31->10,31"))
  and any(.simulation.event_log[]; contains("formation_move_slot:Multi0:8:multi0.formation.blocked.left:slot1:10,31->9,31"))
  and any(.simulation.event_log[]; contains("formation_move_slot:Multi0:8:multi0.formation.blocked.right:slot2:10,31->10,32"))
  and ([.simulation.event_log[] | select(contains("queued_order_execute:8:"))] | length) >= 3
  and any(.simulation.event_log[]; contains("formation_move_reached:8:multi0.formation.blocked.lead:10,31"))
  and any(.simulation.event_log[]; contains("formation_move_reached:8:multi0.formation.blocked.left:9,31"))
  and any(.simulation.event_log[]; contains("formation_move_reached:8:multi0.formation.blocked.right:10,32"))
  and any(.simulation.event_log[]; contains("formation_group_order:Multi0:9:6,2:2slots:0reassigned"))
  and any(.simulation.event_log[]; contains("local_obstruction_detect:multi0.obstruction.leader:multi0.obstruction.blocker:3,2"))
  and any(.simulation.event_log[]; contains("local_obstruction_hold_queue:multi0.obstruction.leader:multi0.obstruction.blocker"))
  and any(.simulation.event_log[]; contains("local_obstruction_side_step:multi0.obstruction.blocker:3,2->3,3"))
  and any(.simulation.event_log[]; contains("local_obstruction_gap_claim:multi0.obstruction.leader:3,2"))
  and any(.simulation.event_log[]; contains("local_obstruction_flow_resume:9:multi0.obstruction.leader:6,2"))
  and any(.simulation.event_log[]; contains("queued_actor_order:Multi0:10:multi0.reservation.lead:move:chain0"))
  and any(.simulation.event_log[]; contains("queued_actor_order:Multi0:10:multi0.reservation.wing:move:chain0"))
  and any(.simulation.event_log[]; contains("path_reservation_grant:multi0.reservation.lead:2,4"))
  and any(.simulation.event_log[]; contains("path_reservation_wait:multi0.reservation.wing:multi0.reservation.lead:2,4"))
  and any(.simulation.event_log[]; contains("path_reservation_grant:multi0.reservation.wing:2,4"))
  and any(.simulation.event_log[]; contains("queued_order_reached:10:multi0.reservation.lead:chain0:2,5"))
  and any(.simulation.event_log[]; contains("queued_order_reached:10:multi0.reservation.wing:chain0:3,4"))
  and any(.simulation.event_log[]; contains("queued_actor_order:Multi0:11:multi0.traffic.lead:move:chain0"))
  and any(.simulation.event_log[]; contains("queued_actor_order:Multi0:11:multi0.traffic.yield:move:chain0"))
  and any(.simulation.event_log[]; contains("traffic_deadlock_detect:multi0.traffic.yield:multi0.traffic.lead:6,6<->5,6"))
  and any(.simulation.event_log[]; contains("traffic_deadlock_yield:multi0.traffic.yield:6,6->6,7"))
  and any(.simulation.event_log[]; contains("traffic_deadlock_resume:11:multi0.traffic.lead:6,6"))
  and any(.simulation.event_log[]; contains("queued_order_reached:11:multi0.traffic.lead:chain0:6,6"))
  and any(.simulation.event_log[]; contains("queued_order_reached:11:multi0.traffic.yield:chain0:4,6"))
  and any(.simulation.event_log[]; contains("queued_actor_order:Multi0:12:multi0.traffic.stuck.runner:move:chain0"))
  and any(.simulation.event_log[]; contains("traffic_stuck_wait:multi0.traffic.stuck.runner:multi0.traffic.stuck.blocker:9,2:wait1"))
  and any(.simulation.event_log[]; contains("traffic_stuck_timeout:multi0.traffic.stuck.runner:multi0.traffic.stuck.blocker:9,2:wait2"))
  and any(.simulation.event_log[]; contains("traffic_stuck_side_step:multi0.traffic.stuck.blocker:9,2->9,3"))
  and any(.simulation.event_log[]; contains("traffic_stuck_resume:12:multi0.traffic.stuck.runner:12,2"))
  and any(.simulation.event_log[]; contains("queued_order_reached:12:multi0.traffic.stuck.runner:chain0:12,2"))
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
  and .gates.production_cancel_gate == true
  and .gates.production_pause_resume_gate == true
  and .gates.producer_queue_gate == true
  and .gates.producer_incomplete_gate == true
  and .gates.tech_train_accept_gate == true
  and .gates.tech_prerequisite_gate == true
  and .gates.supply_cap_gate == true
  and .gates.power_low_production_gate == true
  and .gates.resource_depletion_gate == true
  and .gates.harvest_return_cargo_gate == true
  and .gates.build_placement_gate == true
  and .gates.attack_weapon_gate == true
  and .gates.veterancy_gate == true
  and .gates.attack_range_gate == true
  and .gates.attack_visibility_gate == true
  and .gates.attack_move_command_gate == true
  and .gates.attack_move_gate == true
  and .gates.patrol_command_gate == true
  and .gates.patrol_gate == true
  and .gates.focus_fire_command_gate == true
  and .gates.focus_fire_gate == true
  and .gates.target_priority_gate == true
  and .gates.stop_command_gate == true
  and .gates.stop_gate == true
  and .gates.auto_target_acquisition_gate == true
  and .gates.stance_behavior_gate == true
  and .gates.repair_command_gate == true
  and .gates.repair_gate == true
  and .gates.control_group_gate == true
  and .gates.queued_order_cancel_gate == true
  and .gates.queued_order_chain_gate == true
  and .gates.queued_order_override_gate == true
  and .gates.queued_order_validation_gate == true
  and .gates.queued_order_gate == true
  and .gates.formation_move_gate == true
  and .gates.local_obstruction_recovery_gate == true
  and .gates.path_reservation_gate == true
  and .gates.traffic_deadlock_recovery_gate == true
  and .gates.traffic_stuck_timeout_gate == true
  and .gates.capture_contested_gate == true
  and .gates.capture_objective_gate == true
  and .gates.shroud_gate == true
  and .gates.source_policy_gate == true
  and .source_policy.no_openra_engine_code_copied == true
  and .source_policy.rust_bevy_owned_runtime == true
  and .source_policy.warcraft_iii_asset_copied == false
  and .source_policy.uses_trillionnium_owned_mod_data == true
' "$OUT" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_LIKE_CORE_GREEN %s\n' "$OUT"
