//! Bevy-free RTS evidence summaries and gates.
//!
//! This crate keeps proof contracts separate from `trnm-world-bevy` rendering code.

use serde::{Deserialize, Serialize};
use trnm_rts_bevy_runtime::{
    rts_ability_effect_tiles_for_target, rts_aftermath_debris_tiles_for_id,
    rts_aftermath_smoke_tiles_for_id, rts_ai_counter_tiles_for_pressure,
    rts_ai_pressure_tiles_for_pressure, rts_ai_wave_unit_ids_for_pressure,
    rts_army_rally_tiles_for_id, rts_army_units_for_batch, rts_base_assault_path_tiles_for_target,
    rts_base_assault_targets_for_id, rts_boss_guard_units_for_id,
    rts_central_keep_route_tiles_for_id, rts_central_keep_tile_for_id,
    rts_command_queue_path_preview_stage, rts_commander_aura_tiles_for_id,
    rts_contact_flash_tiles_for_target, rts_counterattack_route_tiles_for_wave,
    rts_counterattack_units_for_wave, rts_creep_camp_tiles_for_id, rts_damage_ticks_for_ability,
    rts_enemy_flank_tile_for_index, rts_enemy_flank_units_for_id,
    rts_enemy_fortification_tile_for_id, rts_enemy_pressure_lane_tiles_for_wave,
    rts_enemy_pressure_wave_units_for_id, rts_enemy_repair_units_for_target,
    rts_enemy_structure_tile_for_id, rts_enemy_structures_for_recon, rts_enemy_unit_tile_for_id,
    rts_enemy_units_for_recon, rts_engagement_tiles_for_target,
    rts_expansion_structure_tile_for_id, rts_expansion_tiles_for_camp, rts_expansion_tiles_for_id,
    rts_expansion_workers_for_line, rts_fog_reveal_tiles_for_recon,
    rts_guardian_counter_units_for_id, rts_inner_core_tile_for_id, rts_inner_defenders_for_id,
    rts_inner_gate_tile_for_id, rts_inner_lane_tiles_for_id, rts_keep_breach_tiles_for_id,
    rts_keep_claim_tiles_for_id, rts_loot_items_for_id, rts_minimap_cell_origin,
    rts_objective_tiles_for_id, rts_player_army_unit_tile_for_id, rts_player_hold_tiles_for_id,
    rts_player_siege_line_tiles_for_id, rts_projectile_id_for_ability,
    rts_projectile_trail_tiles_for_target, rts_runtime_hit_test_grid, rts_runtime_tile_line,
    rts_scout_route_tiles_for_recon, rts_siege_breach_tiles_for_target,
    rts_siege_push_route_tiles_for_target, rts_siege_units_for_id, rts_split_squad_tiles_for_id,
    rts_supply_convoy_for_id, rts_target_priority_ids_for_target, rts_target_tile_for_id,
    rts_terrain_choke_tiles_for_camp, rts_terrain_route_tiles_for_camp,
    rts_threat_levels_for_target, RtsRuntimeGridSpec, RtsRuntimeTileLineStep,
    TRNM_RTS_BEVY_RUNTIME_CONTRACT,
};

pub const TRNM_RTS_EVIDENCE_CONTRACT: &str = "trnm_rts_evidence_v1";
pub const TRNM_RTS_EVIDENCE_BEVY_RUNTIME_ADAPTER_CONTRACT: &str =
    "trnm_rts_evidence_bevy_runtime_adapter_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsEvidencePoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsBevyRuntimeAdapterEvidence {
    pub contract_version: String,
    pub runtime_contract: String,
    pub green: bool,
    pub minimap_cell_sample: RtsEvidencePoint,
    pub path_preview_sample: Option<String>,
    pub command_grid_hit_sample: Option<usize>,
    pub tile_line_sample: Vec<RtsRuntimeTileLineStep>,
    pub combat_engagement_tiles_sample: Vec<String>,
    pub combat_flash_tiles_sample: Vec<String>,
    pub combat_target_tile_sample: RtsEvidencePoint,
    pub combat_target_priority_sample: Vec<String>,
    pub combat_projectile_trail_sample: Vec<String>,
    pub combat_ability_effect_tiles_sample: Vec<String>,
    pub combat_threat_levels_sample: Vec<u8>,
    pub combat_damage_ticks_sample: Vec<u8>,
    pub combat_projectile_id_sample: String,
    pub ai_pressure_wave_units_sample: Vec<String>,
    pub ai_pressure_tiles_sample: Vec<String>,
    pub ai_pressure_counter_tiles_sample: Vec<String>,
    pub enemy_pressure_wave_units_sample: Vec<String>,
    pub enemy_pressure_lane_tiles_sample: Vec<String>,
    pub recon_scout_route_tiles_sample: Vec<String>,
    pub recon_fog_reveal_tiles_sample: Vec<String>,
    pub recon_enemy_structures_sample: Vec<String>,
    pub recon_enemy_units_sample: Vec<String>,
    pub recon_enemy_structure_tile_sample: RtsEvidencePoint,
    pub recon_enemy_unit_tile_sample: RtsEvidencePoint,
    pub base_assault_path_tiles_sample: Vec<String>,
    pub base_assault_targets_sample: Vec<String>,
    pub aftermath_debris_tiles_sample: Vec<String>,
    pub aftermath_smoke_tiles_sample: Vec<String>,
    pub commander_aura_tiles_sample: Vec<String>,
    pub commander_loot_items_sample: Vec<String>,
    pub expansion_claim_tiles_sample: Vec<String>,
    pub expansion_structure_tile_sample: RtsEvidencePoint,
    pub expansion_workers_sample: Vec<String>,
    pub counterattack_units_sample: Vec<String>,
    pub counterattack_route_tiles_sample: Vec<String>,
    pub army_units_sample: Vec<String>,
    pub army_rally_tiles_sample: Vec<String>,
    pub player_army_unit_tile_sample: RtsEvidencePoint,
    pub central_keep_route_tiles_sample: Vec<String>,
    pub central_keep_tile_sample: RtsEvidencePoint,
    pub boss_guard_units_sample: Vec<String>,
    pub player_siege_line_tiles_sample: Vec<String>,
    pub keep_breach_tiles_sample: Vec<String>,
    pub guardian_counter_units_sample: Vec<String>,
    pub keep_claim_tiles_sample: Vec<String>,
    pub objective_tiles_sample: Vec<String>,
    pub creep_camp_tiles_sample: Vec<String>,
    pub terrain_route_tiles_sample: Vec<String>,
    pub terrain_choke_tiles_sample: Vec<String>,
    pub expansion_tiles_sample: Vec<String>,
    pub siege_units_sample: Vec<String>,
    pub siege_push_route_tiles_sample: Vec<String>,
    pub siege_breach_tiles_sample: Vec<String>,
    pub enemy_fortification_tile_sample: RtsEvidencePoint,
    pub enemy_repair_units_sample: Vec<String>,
    pub enemy_flank_units_sample: Vec<String>,
    pub enemy_flank_tile_sample: RtsEvidencePoint,
    pub player_hold_tiles_sample: Vec<String>,
    pub inner_lane_tiles_sample: Vec<String>,
    pub inner_gate_tile_sample: RtsEvidencePoint,
    pub signal_lock_tile_sample: RtsEvidencePoint,
    pub inner_defenders_sample: Vec<String>,
    pub supply_convoy_sample: Vec<String>,
    pub split_squad_tiles_sample: Vec<String>,
    pub inner_core_tile_sample: RtsEvidencePoint,
    pub source_of_truth: String,
}

pub fn first_contact_bevy_runtime_adapter_evidence() -> RtsBevyRuntimeAdapterEvidence {
    let minimap_cell = rts_minimap_cell_origin(10, 20, 4, 5, (32, 32));
    let preview_queue = vec!["command_queue_path_preview:queue_stack".to_string()];
    let path_preview =
        rts_command_queue_path_preview_stage(&[], &preview_queue, 0).map(str::to_string);
    let command_grid_hit = rts_runtime_hit_test_grid(
        RtsRuntimeGridSpec {
            origin_x: 360,
            origin_y: 572,
            columns: 6,
            count: 12,
            stride_x: 58,
            stride_y: 46,
            slot_width: 48,
            slot_height: 38,
        },
        363,
        575,
    );
    let tile_line = rts_runtime_tile_line((8, 8), (12, 16));
    let combat_engagement_tiles = rts_engagement_tiles_for_target("enemy_barracks");
    let combat_flash_tiles = rts_contact_flash_tiles_for_target("arena_creep_attack");
    let combat_target_tile = rts_target_tile_for_id("forest_shaman_support", 0);
    let combat_target_priority = rts_target_priority_ids_for_target("arena_creep_attack");
    let combat_projectile_trail = rts_projectile_trail_tiles_for_target("forest_creep_camp");
    let combat_ability_effect_tiles =
        rts_ability_effect_tiles_for_target("enemy_barracks", "guard_break");
    let combat_threat_levels = rts_threat_levels_for_target("enemy_barracks");
    let combat_damage_ticks = rts_damage_ticks_for_ability("guard_break");
    let combat_projectile_id = rts_projectile_id_for_ability("guard_break");
    let ai_pressure_wave_units = rts_ai_wave_unit_ids_for_pressure("skirmish_wave");
    let ai_pressure_tiles = rts_ai_pressure_tiles_for_pressure("skirmish_wave");
    let ai_pressure_counter_tiles = rts_ai_counter_tiles_for_pressure("skirmish_wave");
    let enemy_pressure_wave_units = rts_enemy_pressure_wave_units_for_id("raider_wave");
    let enemy_pressure_lane_tiles = rts_enemy_pressure_lane_tiles_for_wave("raider_wave");
    let recon_scout_route_tiles = rts_scout_route_tiles_for_recon("enemy_base");
    let recon_fog_reveal_tiles = rts_fog_reveal_tiles_for_recon("enemy_base", "mark");
    let recon_enemy_structures = rts_enemy_structures_for_recon("enemy_base", "mark");
    let recon_enemy_units = rts_enemy_units_for_recon("enemy_base", "mark");
    let recon_enemy_structure_tile = rts_enemy_structure_tile_for_id("enemy_resource_vault", 2);
    let recon_enemy_unit_tile = rts_enemy_unit_tile_for_id("enemy_guard", 2);
    let base_assault_path_tiles = rts_base_assault_path_tiles_for_target("enemy_barracks", "10,3");
    let base_assault_targets = rts_base_assault_targets_for_id("enemy_barracks");
    let aftermath_debris_tiles = rts_aftermath_debris_tiles_for_id("enemy_barracks", "10,3");
    let aftermath_smoke_tiles = rts_aftermath_smoke_tiles_for_id("enemy_barracks", "10,3");
    let commander_aura_tiles = rts_commander_aura_tiles_for_id("mirror_captain");
    let commander_loot_items = rts_loot_items_for_id("enemy_barracks");
    let expansion_claim_tiles = rts_expansion_tiles_for_id("forest_relay", "9,2");
    let expansion_structure_tile = rts_expansion_structure_tile_for_id("watch_lantern");
    let expansion_workers = rts_expansion_workers_for_line("gold_line");
    let counterattack_units = rts_counterattack_units_for_wave("counter_wave");
    let counterattack_route_tiles = rts_counterattack_route_tiles_for_wave("counter_wave", "8,3");
    let army_units = rts_army_units_for_batch("mixed_vanguard");
    let army_rally_tiles = rts_army_rally_tiles_for_id("forward_watch");
    let player_army_unit_tile = rts_player_army_unit_tile_for_id("field_mender", 3);
    let central_keep_route_tiles = rts_central_keep_route_tiles_for_id("central_keep", "13,3");
    let central_keep_tile = rts_central_keep_tile_for_id("central_keep");
    let boss_guard_units = rts_boss_guard_units_for_id("warden_line");
    let player_siege_line_tiles = rts_player_siege_line_tiles_for_id("final_line", "12,4");
    let keep_breach_tiles = rts_keep_breach_tiles_for_id("central_keep", "13,3");
    let guardian_counter_units = rts_guardian_counter_units_for_id("high_warden");
    let keep_claim_tiles = rts_keep_claim_tiles_for_id("central_keep", "13,3");
    let objective_tiles = rts_objective_tiles_for_id("relay_beacon", "6,5");
    let creep_camp_tiles = rts_creep_camp_tiles_for_id("forest_creep_camp", "8,3");
    let terrain_route_tiles = rts_terrain_route_tiles_for_camp("forest_creep_camp");
    let terrain_choke_tiles = rts_terrain_choke_tiles_for_camp("forest_creep_camp");
    let expansion_tiles = rts_expansion_tiles_for_camp("forest_creep_camp");
    let siege_units = rts_siege_units_for_id("stonebreak_cart");
    let siege_push_route_tiles = rts_siege_push_route_tiles_for_target("gate_bulwark", "10,3");
    let siege_breach_tiles = rts_siege_breach_tiles_for_target("gate_bulwark", "10,3");
    let enemy_fortification_tile = rts_enemy_fortification_tile_for_id("gate_bulwark");
    let enemy_repair_units = rts_enemy_repair_units_for_target("gate_bulwark");
    let enemy_flank_units = rts_enemy_flank_units_for_id("ridge_sentries");
    let enemy_flank_tile = rts_enemy_flank_tile_for_index(2);
    let player_hold_tiles = rts_player_hold_tiles_for_id("shield_line", "9,3");
    let inner_lane_tiles = rts_inner_lane_tiles_for_id("inner_lane", "11,2");
    let inner_gate_tile = rts_inner_gate_tile_for_id("inner_latch");
    let signal_lock_tile = rts_inner_gate_tile_for_id("signal_lock");
    let inner_defenders = rts_inner_defenders_for_id("second_line");
    let supply_convoy = rts_supply_convoy_for_id("relay_convoy");
    let split_squad_tiles = rts_split_squad_tiles_for_id("flank_team", "10,4");
    let inner_core_tile = rts_inner_core_tile_for_id("signal_core");
    let green = TRNM_RTS_BEVY_RUNTIME_CONTRACT == "trnm_rts_bevy_runtime_adapter_v1"
        && minimap_cell == (134, 175)
        && path_preview.as_deref() == Some("queue_stack")
        && command_grid_hit == Some(0)
        && tile_line.len() == 9
        && tile_line.first().is_some_and(|step| {
            step.step_index == 0 && step.step_count == 8 && step.tile_x == 8 && step.tile_y == 8
        })
        && tile_line.get(4).is_some_and(|step| {
            step.step_index == 4 && step.step_count == 8 && step.tile_x == 10 && step.tile_y == 12
        })
        && tile_line.last().is_some_and(|step| {
            step.step_index == 8 && step.step_count == 8 && step.tile_x == 12 && step.tile_y == 16
        })
        && combat_engagement_tiles == vec!["9,3", "10,3", "10,2", "11,2"]
        && combat_flash_tiles == vec!["6,5", "6,4"]
        && combat_target_tile == (9, 3)
        && combat_target_priority
            == vec![
                "arena_creep_attack",
                "arena_guard_support",
                "arena_worker_support",
            ]
        && combat_projectile_trail == vec!["5,5", "6,5", "7,4", "8,3"]
        && combat_ability_effect_tiles == vec!["10,3", "10,2", "11,2", "9,3"]
        && combat_threat_levels == vec![88, 66, 41]
        && combat_damage_ticks == vec![16, 21, 35]
        && combat_projectile_id == "guard_break_bolt"
        && ai_pressure_wave_units == vec!["lane_scout", "mirror_raider", "siege_runner"]
        && ai_pressure_tiles == vec!["9,3", "8,4", "7,4", "6,5"]
        && ai_pressure_counter_tiles == vec!["5,5", "6,5", "6,4", "7,5"]
        && enemy_pressure_wave_units == vec!["enemy_raider", "enemy_signal_guard", "enemy_sapper"]
        && enemy_pressure_lane_tiles == vec!["10,2", "9,3", "8,4", "7,4", "6,5"]
        && recon_scout_route_tiles == vec!["5,5", "6,4", "7,4", "8,3", "9,2", "10,2"]
        && recon_fog_reveal_tiles
            == vec![
                "7,4", "8,3", "8,2", "9,2", "9,3", "10,2", "10,3", "11,1", "11,2",
            ]
        && recon_enemy_structures
            == vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        && recon_enemy_units == vec!["enemy_scout", "enemy_worker", "enemy_guard"]
        && recon_enemy_structure_tile == (11, 2)
        && recon_enemy_unit_tile == (11, 2)
        && base_assault_path_tiles == vec!["5,5", "6,5", "7,4", "8,4", "9,3", "10,3"]
        && base_assault_targets
            == vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        && aftermath_debris_tiles == vec!["9,3", "10,3", "10,4", "11,3"]
        && aftermath_smoke_tiles == vec!["10,2", "10,3", "11,3"]
        && commander_aura_tiles == vec!["6,5", "7,4", "8,4", "9,3", "10,3"]
        && commander_loot_items
            == vec![
                "barracks_map_cache",
                "field_banner_relic",
                "repair_kit_crate",
            ]
        && expansion_claim_tiles == vec!["8,2", "9,2", "10,2", "9,3", "10,3"]
        && expansion_structure_tile == (8, 3)
        && expansion_workers
            == vec![
                "expansion_worker_alpha",
                "expansion_worker_beta",
                "expansion_worker_gamma",
            ]
        && counterattack_units
            == vec![
                "counter_raider_alpha",
                "counter_raider_beta",
                "counter_sapper",
            ]
        && counterattack_route_tiles == vec!["11,2", "10,2", "9,3", "8,3", "7,4", "9,2"]
        && army_units
            == vec![
                "relay_guard_alpha",
                "relay_guard_beta",
                "wayfinder_scout",
                "field_mender",
            ]
        && army_rally_tiles == vec!["5,5", "6,5", "7,4", "8,4", "8,3"]
        && player_army_unit_tile == (6, 4)
        && central_keep_route_tiles == vec!["12,3", "12,4", "13,4", "13,3", "14,3"]
        && central_keep_tile == (13, 3)
        && boss_guard_units == vec!["keep_warden_alpha", "keep_warden_beta", "ward_sentinel"]
        && player_siege_line_tiles == vec!["11,4", "12,4", "13,4", "12,3"]
        && keep_breach_tiles == vec!["13,3", "13,4", "14,3", "14,4"]
        && guardian_counter_units == vec!["high_warden", "ward_lancer", "last_mirror_guard"]
        && keep_claim_tiles == vec!["12,3", "13,3", "14,3", "13,4"]
        && objective_tiles == vec!["6,5", "6,4", "7,5", "9,2"]
        && creep_camp_tiles == vec!["8,3", "8,2", "9,3", "9,2"]
        && terrain_route_tiles == vec!["5,5", "6,5", "7,4", "8,3"]
        && terrain_choke_tiles == vec!["7,4", "7,3", "8,4"]
        && expansion_tiles == vec!["9,2", "10,2", "10,3"]
        && siege_units == vec!["stonebreak_cart"]
        && siege_push_route_tiles == vec!["9,2", "9,3", "10,3", "10,2", "11,2", "10,3"]
        && siege_breach_tiles == vec!["9,3", "10,3", "10,2", "11,2", "10,3"]
        && enemy_fortification_tile == (10, 3)
        && enemy_repair_units == vec!["repair_adept_alpha", "repair_adept_beta"]
        && enemy_flank_units == vec!["ridge_sentry_left", "ridge_sentry_right", "ridge_sapper"]
        && enemy_flank_tile == (8, 4)
        && player_hold_tiles == vec!["8,3", "9,3", "9,4", "10,3"]
        && inner_lane_tiles == vec!["10,3", "11,2", "11,3", "12,3", "12,4"]
        && inner_gate_tile == (11, 3)
        && signal_lock_tile == (12, 3)
        && inner_defenders == vec!["inner_guard_alpha", "inner_guard_beta", "signal_lancer"]
        && supply_convoy == vec!["convoy_cart", "field_medic", "ammo_runner"]
        && split_squad_tiles == vec!["10,4", "11,4", "12,4", "12,3"]
        && inner_core_tile == (12, 3);

    RtsBevyRuntimeAdapterEvidence {
        contract_version: TRNM_RTS_EVIDENCE_BEVY_RUNTIME_ADAPTER_CONTRACT.to_string(),
        runtime_contract: TRNM_RTS_BEVY_RUNTIME_CONTRACT.to_string(),
        green,
        minimap_cell_sample: RtsEvidencePoint {
            x: minimap_cell.0,
            y: minimap_cell.1,
        },
        path_preview_sample: path_preview,
        command_grid_hit_sample: command_grid_hit,
        tile_line_sample: tile_line,
        combat_engagement_tiles_sample: combat_engagement_tiles,
        combat_flash_tiles_sample: combat_flash_tiles,
        combat_target_tile_sample: RtsEvidencePoint {
            x: combat_target_tile.0,
            y: combat_target_tile.1,
        },
        combat_target_priority_sample: combat_target_priority,
        combat_projectile_trail_sample: combat_projectile_trail,
        combat_ability_effect_tiles_sample: combat_ability_effect_tiles,
        combat_threat_levels_sample: combat_threat_levels,
        combat_damage_ticks_sample: combat_damage_ticks,
        combat_projectile_id_sample: combat_projectile_id.to_string(),
        ai_pressure_wave_units_sample: ai_pressure_wave_units,
        ai_pressure_tiles_sample: ai_pressure_tiles,
        ai_pressure_counter_tiles_sample: ai_pressure_counter_tiles,
        enemy_pressure_wave_units_sample: enemy_pressure_wave_units,
        enemy_pressure_lane_tiles_sample: enemy_pressure_lane_tiles,
        recon_scout_route_tiles_sample: recon_scout_route_tiles,
        recon_fog_reveal_tiles_sample: recon_fog_reveal_tiles,
        recon_enemy_structures_sample: recon_enemy_structures,
        recon_enemy_units_sample: recon_enemy_units,
        recon_enemy_structure_tile_sample: RtsEvidencePoint {
            x: recon_enemy_structure_tile.0,
            y: recon_enemy_structure_tile.1,
        },
        recon_enemy_unit_tile_sample: RtsEvidencePoint {
            x: recon_enemy_unit_tile.0,
            y: recon_enemy_unit_tile.1,
        },
        base_assault_path_tiles_sample: base_assault_path_tiles,
        base_assault_targets_sample: base_assault_targets,
        aftermath_debris_tiles_sample: aftermath_debris_tiles,
        aftermath_smoke_tiles_sample: aftermath_smoke_tiles,
        commander_aura_tiles_sample: commander_aura_tiles,
        commander_loot_items_sample: commander_loot_items,
        expansion_claim_tiles_sample: expansion_claim_tiles,
        expansion_structure_tile_sample: RtsEvidencePoint {
            x: expansion_structure_tile.0,
            y: expansion_structure_tile.1,
        },
        expansion_workers_sample: expansion_workers,
        counterattack_units_sample: counterattack_units,
        counterattack_route_tiles_sample: counterattack_route_tiles,
        army_units_sample: army_units,
        army_rally_tiles_sample: army_rally_tiles,
        player_army_unit_tile_sample: RtsEvidencePoint {
            x: player_army_unit_tile.0,
            y: player_army_unit_tile.1,
        },
        central_keep_route_tiles_sample: central_keep_route_tiles,
        central_keep_tile_sample: RtsEvidencePoint {
            x: central_keep_tile.0,
            y: central_keep_tile.1,
        },
        boss_guard_units_sample: boss_guard_units,
        player_siege_line_tiles_sample: player_siege_line_tiles,
        keep_breach_tiles_sample: keep_breach_tiles,
        guardian_counter_units_sample: guardian_counter_units,
        keep_claim_tiles_sample: keep_claim_tiles,
        objective_tiles_sample: objective_tiles,
        creep_camp_tiles_sample: creep_camp_tiles,
        terrain_route_tiles_sample: terrain_route_tiles,
        terrain_choke_tiles_sample: terrain_choke_tiles,
        expansion_tiles_sample: expansion_tiles,
        siege_units_sample: siege_units,
        siege_push_route_tiles_sample: siege_push_route_tiles,
        siege_breach_tiles_sample: siege_breach_tiles,
        enemy_fortification_tile_sample: RtsEvidencePoint {
            x: enemy_fortification_tile.0,
            y: enemy_fortification_tile.1,
        },
        enemy_repair_units_sample: enemy_repair_units,
        enemy_flank_units_sample: enemy_flank_units,
        enemy_flank_tile_sample: RtsEvidencePoint {
            x: enemy_flank_tile.0,
            y: enemy_flank_tile.1,
        },
        player_hold_tiles_sample: player_hold_tiles,
        inner_lane_tiles_sample: inner_lane_tiles,
        inner_gate_tile_sample: RtsEvidencePoint {
            x: inner_gate_tile.0,
            y: inner_gate_tile.1,
        },
        signal_lock_tile_sample: RtsEvidencePoint {
            x: signal_lock_tile.0,
            y: signal_lock_tile.1,
        },
        inner_defenders_sample: inner_defenders,
        supply_convoy_sample: supply_convoy,
        split_squad_tiles_sample: split_squad_tiles,
        inner_core_tile_sample: RtsEvidencePoint {
            x: inner_core_tile.0,
            y: inner_core_tile.1,
        },
        source_of_truth: "The RTS evidence crate verifies the Bevy-free runtime adapter contract using deterministic First Contact minimap, path preview, command-grid, tile-line raster, combat-target, ability-effect, AI-pressure, recon-intel, base-assault, aftermath, commander-progression, expansion-counterattack, army-production/rally, siege breach counterplay, inner-lane breakthrough, central-keep, objective, terrain-route, and siege-route samples before trnm-world-bevy includes the proof in release-review evidence.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_runtime_adapter_evidence_is_green() {
        let evidence = first_contact_bevy_runtime_adapter_evidence();

        assert_eq!(
            evidence.contract_version,
            TRNM_RTS_EVIDENCE_BEVY_RUNTIME_ADAPTER_CONTRACT
        );
        assert_eq!(evidence.runtime_contract, TRNM_RTS_BEVY_RUNTIME_CONTRACT);
        assert!(evidence.green);
        assert_eq!(
            evidence.minimap_cell_sample,
            RtsEvidencePoint { x: 134, y: 175 }
        );
        assert_eq!(evidence.path_preview_sample.as_deref(), Some("queue_stack"));
        assert_eq!(evidence.command_grid_hit_sample, Some(0));
        assert_eq!(evidence.tile_line_sample.len(), 9);
        assert_eq!(evidence.tile_line_sample[4].tile_x, 10);
        assert_eq!(evidence.tile_line_sample[4].tile_y, 12);
        assert_eq!(evidence.tile_line_sample[8].tile_x, 12);
        assert_eq!(evidence.tile_line_sample[8].tile_y, 16);
        assert_eq!(
            evidence.combat_engagement_tiles_sample,
            vec!["9,3", "10,3", "10,2", "11,2"]
        );
        assert_eq!(evidence.combat_flash_tiles_sample, vec!["6,5", "6,4"]);
        assert_eq!(
            evidence.combat_target_tile_sample,
            RtsEvidencePoint { x: 9, y: 3 }
        );
        assert_eq!(
            evidence.combat_target_priority_sample,
            vec![
                "arena_creep_attack",
                "arena_guard_support",
                "arena_worker_support"
            ]
        );
        assert_eq!(
            evidence.combat_projectile_trail_sample,
            vec!["5,5", "6,5", "7,4", "8,3"]
        );
        assert_eq!(
            evidence.combat_ability_effect_tiles_sample,
            vec!["10,3", "10,2", "11,2", "9,3"]
        );
        assert_eq!(evidence.combat_threat_levels_sample, vec![88, 66, 41]);
        assert_eq!(evidence.combat_damage_ticks_sample, vec![16, 21, 35]);
        assert_eq!(evidence.combat_projectile_id_sample, "guard_break_bolt");
        assert_eq!(
            evidence.ai_pressure_wave_units_sample,
            vec!["lane_scout", "mirror_raider", "siege_runner"]
        );
        assert_eq!(
            evidence.ai_pressure_tiles_sample,
            vec!["9,3", "8,4", "7,4", "6,5"]
        );
        assert_eq!(
            evidence.ai_pressure_counter_tiles_sample,
            vec!["5,5", "6,5", "6,4", "7,5"]
        );
        assert_eq!(
            evidence.enemy_pressure_wave_units_sample,
            vec!["enemy_raider", "enemy_signal_guard", "enemy_sapper"]
        );
        assert_eq!(
            evidence.enemy_pressure_lane_tiles_sample,
            vec!["10,2", "9,3", "8,4", "7,4", "6,5"]
        );
        assert_eq!(
            evidence.recon_scout_route_tiles_sample,
            vec!["5,5", "6,4", "7,4", "8,3", "9,2", "10,2"]
        );
        assert_eq!(
            evidence.recon_fog_reveal_tiles_sample,
            vec!["7,4", "8,3", "8,2", "9,2", "9,3", "10,2", "10,3", "11,1", "11,2"]
        );
        assert_eq!(
            evidence.recon_enemy_structures_sample,
            vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        );
        assert_eq!(
            evidence.recon_enemy_units_sample,
            vec!["enemy_scout", "enemy_worker", "enemy_guard"]
        );
        assert_eq!(
            evidence.recon_enemy_structure_tile_sample,
            RtsEvidencePoint { x: 11, y: 2 }
        );
        assert_eq!(
            evidence.recon_enemy_unit_tile_sample,
            RtsEvidencePoint { x: 11, y: 2 }
        );
        assert_eq!(
            evidence.base_assault_path_tiles_sample,
            vec!["5,5", "6,5", "7,4", "8,4", "9,3", "10,3"]
        );
        assert_eq!(
            evidence.base_assault_targets_sample,
            vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        );
        assert_eq!(
            evidence.aftermath_debris_tiles_sample,
            vec!["9,3", "10,3", "10,4", "11,3"]
        );
        assert_eq!(
            evidence.aftermath_smoke_tiles_sample,
            vec!["10,2", "10,3", "11,3"]
        );
        assert_eq!(
            evidence.commander_aura_tiles_sample,
            vec!["6,5", "7,4", "8,4", "9,3", "10,3"]
        );
        assert_eq!(
            evidence.commander_loot_items_sample,
            vec![
                "barracks_map_cache",
                "field_banner_relic",
                "repair_kit_crate"
            ]
        );
        assert_eq!(
            evidence.expansion_claim_tiles_sample,
            vec!["8,2", "9,2", "10,2", "9,3", "10,3"]
        );
        assert_eq!(
            evidence.expansion_structure_tile_sample,
            RtsEvidencePoint { x: 8, y: 3 }
        );
        assert_eq!(
            evidence.expansion_workers_sample,
            vec![
                "expansion_worker_alpha",
                "expansion_worker_beta",
                "expansion_worker_gamma"
            ]
        );
        assert_eq!(
            evidence.counterattack_units_sample,
            vec![
                "counter_raider_alpha",
                "counter_raider_beta",
                "counter_sapper"
            ]
        );
        assert_eq!(
            evidence.counterattack_route_tiles_sample,
            vec!["11,2", "10,2", "9,3", "8,3", "7,4", "9,2"]
        );
        assert_eq!(
            evidence.army_units_sample,
            vec![
                "relay_guard_alpha",
                "relay_guard_beta",
                "wayfinder_scout",
                "field_mender"
            ]
        );
        assert_eq!(
            evidence.army_rally_tiles_sample,
            vec!["5,5", "6,5", "7,4", "8,4", "8,3"]
        );
        assert_eq!(
            evidence.player_army_unit_tile_sample,
            RtsEvidencePoint { x: 6, y: 4 }
        );
        assert_eq!(
            evidence.central_keep_route_tiles_sample,
            vec!["12,3", "12,4", "13,4", "13,3", "14,3"]
        );
        assert_eq!(
            evidence.central_keep_tile_sample,
            RtsEvidencePoint { x: 13, y: 3 }
        );
        assert_eq!(
            evidence.boss_guard_units_sample,
            vec!["keep_warden_alpha", "keep_warden_beta", "ward_sentinel"]
        );
        assert_eq!(
            evidence.player_siege_line_tiles_sample,
            vec!["11,4", "12,4", "13,4", "12,3"]
        );
        assert_eq!(
            evidence.keep_breach_tiles_sample,
            vec!["13,3", "13,4", "14,3", "14,4"]
        );
        assert_eq!(
            evidence.guardian_counter_units_sample,
            vec!["high_warden", "ward_lancer", "last_mirror_guard"]
        );
        assert_eq!(
            evidence.keep_claim_tiles_sample,
            vec!["12,3", "13,3", "14,3", "13,4"]
        );
        assert_eq!(
            evidence.objective_tiles_sample,
            vec!["6,5", "6,4", "7,5", "9,2"]
        );
        assert_eq!(
            evidence.creep_camp_tiles_sample,
            vec!["8,3", "8,2", "9,3", "9,2"]
        );
        assert_eq!(
            evidence.terrain_route_tiles_sample,
            vec!["5,5", "6,5", "7,4", "8,3"]
        );
        assert_eq!(
            evidence.terrain_choke_tiles_sample,
            vec!["7,4", "7,3", "8,4"]
        );
        assert_eq!(evidence.expansion_tiles_sample, vec!["9,2", "10,2", "10,3"]);
        assert_eq!(evidence.siege_units_sample, vec!["stonebreak_cart"]);
        assert_eq!(
            evidence.siege_push_route_tiles_sample,
            vec!["9,2", "9,3", "10,3", "10,2", "11,2", "10,3"]
        );
        assert_eq!(
            evidence.siege_breach_tiles_sample,
            vec!["9,3", "10,3", "10,2", "11,2", "10,3"]
        );
        assert_eq!(
            evidence.enemy_fortification_tile_sample,
            RtsEvidencePoint { x: 10, y: 3 }
        );
        assert_eq!(
            evidence.enemy_repair_units_sample,
            vec!["repair_adept_alpha", "repair_adept_beta"]
        );
        assert_eq!(
            evidence.enemy_flank_units_sample,
            vec!["ridge_sentry_left", "ridge_sentry_right", "ridge_sapper"]
        );
        assert_eq!(
            evidence.enemy_flank_tile_sample,
            RtsEvidencePoint { x: 8, y: 4 }
        );
        assert_eq!(
            evidence.player_hold_tiles_sample,
            vec!["8,3", "9,3", "9,4", "10,3"]
        );
        assert_eq!(
            evidence.inner_lane_tiles_sample,
            vec!["10,3", "11,2", "11,3", "12,3", "12,4"]
        );
        assert_eq!(
            evidence.inner_gate_tile_sample,
            RtsEvidencePoint { x: 11, y: 3 }
        );
        assert_eq!(
            evidence.signal_lock_tile_sample,
            RtsEvidencePoint { x: 12, y: 3 }
        );
        assert_eq!(
            evidence.inner_defenders_sample,
            vec!["inner_guard_alpha", "inner_guard_beta", "signal_lancer"]
        );
        assert_eq!(
            evidence.supply_convoy_sample,
            vec!["convoy_cart", "field_medic", "ammo_runner"]
        );
        assert_eq!(
            evidence.split_squad_tiles_sample,
            vec!["10,4", "11,4", "12,4", "12,3"]
        );
        assert_eq!(
            evidence.inner_core_tile_sample,
            RtsEvidencePoint { x: 12, y: 3 }
        );
    }
}
