use serde_json::{json, Value};
use std::collections::BTreeSet;
use trnm_rts_bevy_runtime::rts_runtime_tile_id;
use trnm_rts_core::RtsTile;
use trnm_rts_data::{
    first_contact_command_feedback_player_labels, first_contact_samples, RtsCommandFeedbackProfile,
    RtsFirstContactVisualTelemetryProfile, RtsOpeningLoopProfile, RtsTacticalTrackProfile,
    RtsVisualTelemetryColorRole,
};

use crate::{
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_MOTION_READABILITY_CONTRACT,
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_SECONDARY_TRACK_DARKEN_DENOMINATOR,
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_SECONDARY_TRACK_DARKEN_NUMERATOR,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtsFirstContactMotionReadabilityRuntime {
    pub walk_cycle_frame: u8,
    pub combat_turn: u8,
    pub training_progress_percent: u8,
    pub build_progress_percent: u8,
    pub command_destination_tile: Option<String>,
    pub route_tile_ids: Vec<String>,
    pub attack_target_id: Option<String>,
    pub combat_event_log: Vec<String>,
    pub feedback_move_trail_origin_count: usize,
    pub feedback_move_trail_step_count_per_origin: usize,
    pub feedback_move_trail_tick_width_px: usize,
    pub feedback_move_trail_tick_height_px: usize,
    pub opening_action_path_count: usize,
    pub opening_action_path_step_count: usize,
    pub opening_action_path_dot_width_px: usize,
    pub opening_action_path_dot_height_px: usize,
    pub warden_attack_arm_count: usize,
    pub warden_attack_arm_width_px: usize,
    pub warden_attack_arm_height_px: usize,
    pub production_training_spark_count: usize,
    pub production_training_spark_width_px: usize,
    pub production_training_spark_height_px: usize,
    pub animation_training_tick_count: usize,
    pub animation_training_tick_width_px: usize,
    pub animation_training_tick_height_px: usize,
    pub shield_charge_arc_count: usize,
    pub shield_charge_arc_width_px: usize,
    pub shield_charge_arc_height_px: usize,
    pub sensor_sweep_tick_count: usize,
    pub sensor_sweep_tick_width_px: usize,
    pub sensor_sweep_tick_height_px: usize,
    pub carry_load_pip_count: usize,
    pub carry_load_pip_width_px: usize,
    pub carry_load_pip_height_px: usize,
    pub combat_hit_flash_site_count: usize,
    pub combat_hit_flash_sparks_per_site: usize,
    pub combat_hit_flash_spark_width_px: usize,
    pub combat_hit_flash_spark_height_px: usize,
}

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn first_contact_tile_id(tile: RtsTile) -> String {
    rts_runtime_tile_id((tile.x, tile.y))
}

fn tile_id(tile: (i32, i32)) -> String {
    rts_runtime_tile_id(tile)
}

fn feedback_labels_without_raw_markers(
    feedback: &RtsCommandFeedbackProfile,
    labels: &[String],
) -> bool {
    labels.iter().all(|label| {
        !label.contains("->")
            && !label.contains("ACK")
            && !label.contains(" CD ")
            && !label.starts_with("Q ")
            && label != &feedback.blocked_reason
    })
}

fn primary_tactical_track(
    track: &RtsTacticalTrackProfile,
    opening: &RtsOpeningLoopProfile,
) -> bool {
    track.from_tile == opening.active_relay_tile
        && track.to_tile == opening.active_beacon_tile
        && track.color_role == RtsVisualTelemetryColorRole::ActionTrail
}

pub fn first_contact_motion_readability_guard(
    opening: &RtsOpeningLoopProfile,
    feedback: &RtsCommandFeedbackProfile,
    telemetry: &RtsFirstContactVisualTelemetryProfile,
    runtime: &RtsFirstContactMotionReadabilityRuntime,
) -> Value {
    let animation_samples = first_contact_samples::animation_cycle_samples();
    let active_beacon_tile_id = first_contact_tile_id(opening.active_beacon_tile);
    let active_relay_tile_id = first_contact_tile_id(opening.active_relay_tile);
    let opening_action_ids = opening.opening_actions.clone();
    let action_verbs = opening_action_ids
        .iter()
        .map(|action| action.split('_').next().unwrap_or_default().to_string())
        .collect::<Vec<_>>();
    let unit_status_badges = telemetry
        .unit_statuses
        .iter()
        .map(|status| status.role_badge.clone())
        .collect::<Vec<_>>();
    let unit_status_color_roles = telemetry
        .unit_statuses
        .iter()
        .map(|status| status.role_color.as_str().to_string())
        .collect::<Vec<_>>();
    let track_roles = telemetry
        .tactical_tracks
        .iter()
        .map(|track| track.color_role.as_str().to_string())
        .collect::<Vec<_>>();
    let track_sample_objects = telemetry
        .tactical_tracks
        .iter()
        .map(|track| {
            json!({
                "from_tile": first_contact_tile_id(track.from_tile),
                "to_tile": first_contact_tile_id(track.to_tile),
                "role": track.color_role.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let animation_sample_tiles = animation_samples
        .iter()
        .map(|(tile, _, _)| tile_id(*tile))
        .collect::<Vec<_>>();
    let animation_roles = animation_samples
        .iter()
        .map(|(_, role, _)| (*role).to_string())
        .collect::<Vec<_>>();
    let animation_signatures = animation_samples
        .iter()
        .map(|(_, _, signature)| (*signature).to_string())
        .collect::<Vec<_>>();
    let animation_frame_richness_signatures =
        first_contact_samples::animation_frame_richness_signatures()
            .iter()
            .map(|signature| (*signature).to_string())
            .collect::<Vec<_>>();
    let animation_sample_objects = animation_samples
        .iter()
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let unique_animation_signature_count =
        animation_signatures.iter().collect::<BTreeSet<_>>().len();
    let unit_animation_frame_count = animation_roles
        .iter()
        .filter(|role| first_contact_samples::unit_animation_role(role))
        .count();
    let building_animation_frame_count = animation_roles
        .iter()
        .filter(|role| first_contact_samples::structure_animation_role(role))
        .count();
    let objective_animation_frame_count = animation_roles
        .iter()
        .filter(|role| role.as_str() == "beacon")
        .count();
    let action_trail_count = telemetry
        .tactical_tracks
        .iter()
        .filter(|track| track.color_role.as_str() == "action_trail")
        .count();
    let npc_action_count = telemetry
        .tactical_tracks
        .iter()
        .filter(|track| track.color_role.as_str() == "npc_action")
        .count();
    let primary_tactical_track_count = telemetry
        .tactical_tracks
        .iter()
        .filter(|track| primary_tactical_track(track, opening))
        .count();
    let secondary_tactical_track_count = telemetry
        .tactical_tracks
        .len()
        .saturating_sub(primary_tactical_track_count);
    let primary_tactical_track_pixel_budget = primary_tactical_track_count * 48;
    let secondary_tactical_track_pixel_budget = secondary_tactical_track_count * 16;
    let secondary_tactical_track_height_px = 1_usize;
    let secondary_tactical_track_darken_numerator =
        TRNM_RTS_EVIDENCE_FIRST_CONTACT_SECONDARY_TRACK_DARKEN_NUMERATOR;
    let secondary_tactical_track_darken_denominator =
        TRNM_RTS_EVIDENCE_FIRST_CONTACT_SECONDARY_TRACK_DARKEN_DENOMINATOR;
    let tactical_track_density_signatures = string_vec([
        "primary_relay_beacon_track_kept_hot",
        "secondary_tracks_dimmed",
        "secondary_tracks_faded_into_terrain",
        "secondary_tracks_one_pixel",
    ]);
    let route_tile_ids = runtime.route_tile_ids.clone();
    let combat_event_log = runtime.combat_event_log.clone();
    let feedback_player_labels = first_contact_command_feedback_player_labels(feedback)
        .into_iter()
        .collect::<Vec<_>>();
    let expected_feedback_player_labels = string_vec([
        "GROUP 1 SECURING BEACON",
        "QUEUE ADDED  ORDER READY",
        "ROUTE BLOCKED MID VENT",
    ]);
    let feedback_raw_marker_gate =
        feedback_labels_without_raw_markers(feedback, &feedback_player_labels);
    let feedback_player_label_gate =
        feedback_player_labels == expected_feedback_player_labels && feedback_raw_marker_gate;
    let unit_status_pixel_budget = telemetry.unit_statuses.len() * 64;
    let tactical_track_pixel_budget =
        primary_tactical_track_pixel_budget + secondary_tactical_track_pixel_budget;
    let progress_meter_pixel_budget = usize::from(opening.worker_train_progress)
        + usize::from(opening.scout_train_progress)
        + usize::from(opening.relay_build_progress)
        + usize::from(opening.beacon_capture_progress);
    let feedback_pixel_budget = usize::from(feedback.command_ack_progress)
        + usize::from(feedback.cooldown_progress)
        + usize::from(feedback.queued_after) * 24;
    let feedback_move_trail_tick_count = runtime.feedback_move_trail_origin_count
        * runtime.feedback_move_trail_step_count_per_origin;
    let feedback_move_trail_pixel_budget = feedback_move_trail_tick_count
        * runtime.feedback_move_trail_tick_width_px
        * runtime.feedback_move_trail_tick_height_px;
    let opening_action_path_pixel_budget = runtime.opening_action_path_step_count
        * runtime.opening_action_path_dot_width_px
        * runtime.opening_action_path_dot_height_px;
    let warden_attack_arm_pixel_budget = runtime.warden_attack_arm_count
        * runtime.warden_attack_arm_width_px
        * runtime.warden_attack_arm_height_px;
    let production_training_spark_pixel_budget = runtime.production_training_spark_count
        * runtime.production_training_spark_width_px
        * runtime.production_training_spark_height_px;
    let animation_training_tick_pixel_budget = runtime.animation_training_tick_count
        * runtime.animation_training_tick_width_px
        * runtime.animation_training_tick_height_px;
    let shield_charge_arc_pixel_budget = runtime.shield_charge_arc_count
        * runtime.shield_charge_arc_width_px
        * runtime.shield_charge_arc_height_px;
    let sensor_sweep_tick_pixel_budget = runtime.sensor_sweep_tick_count
        * runtime.sensor_sweep_tick_width_px
        * runtime.sensor_sweep_tick_height_px;
    let carry_load_pip_pixel_budget = runtime.carry_load_pip_count
        * runtime.carry_load_pip_width_px
        * runtime.carry_load_pip_height_px;
    let combat_hit_flash_spark_count =
        runtime.combat_hit_flash_site_count * runtime.combat_hit_flash_sparks_per_site;
    let combat_hit_flash_spark_pixel_budget = combat_hit_flash_spark_count
        * runtime.combat_hit_flash_spark_width_px
        * runtime.combat_hit_flash_spark_height_px;
    let unit_state_motion_signatures = string_vec([
        "unit_status_badges",
        "player_screen_production_training_micro_sparks",
        "player_screen_warden_attack_micro_sparks",
    ]);
    let player_screen_animation_signatures = string_vec([
        "player_screen_animation_training_lane_micro_ticks",
        "player_screen_worker_carry_load_micro_pips",
        "player_screen_shield_charge_micro_arcs",
        "player_screen_sensor_sweep_micro_ticks",
    ]);
    let animation_training_tick_gate = runtime.animation_training_tick_count == 3
        && runtime.animation_training_tick_width_px == 4
        && runtime.animation_training_tick_height_px == 2
        && animation_training_tick_pixel_budget <= 24
        && player_screen_animation_signatures.iter().any(|signature| {
            signature.as_str() == "player_screen_animation_training_lane_micro_ticks"
        });
    let carry_load_pip_gate = runtime.carry_load_pip_count == 4
        && runtime.carry_load_pip_width_px == 4
        && runtime.carry_load_pip_height_px == 2
        && carry_load_pip_pixel_budget <= 32
        && player_screen_animation_signatures
            .iter()
            .any(|signature| signature.as_str() == "player_screen_worker_carry_load_micro_pips");
    let production_training_spark_gate = runtime.production_training_spark_count == 3
        && runtime.production_training_spark_width_px == 4
        && runtime.production_training_spark_height_px == 2
        && production_training_spark_pixel_budget <= 24
        && unit_state_motion_signatures.iter().any(|signature| {
            signature.as_str() == "player_screen_production_training_micro_sparks"
        });
    let warden_attack_arm_gate = runtime.warden_attack_arm_count == 3
        && runtime.warden_attack_arm_width_px == 14
        && runtime.warden_attack_arm_height_px == 2
        && warden_attack_arm_pixel_budget <= 84
        && unit_state_motion_signatures
            .iter()
            .any(|signature| signature.as_str() == "player_screen_warden_attack_micro_sparks");
    let shield_charge_arc_gate = runtime.shield_charge_arc_count == 3
        && runtime.shield_charge_arc_width_px == 10
        && runtime.shield_charge_arc_height_px == 2
        && shield_charge_arc_pixel_budget <= 60
        && player_screen_animation_signatures
            .iter()
            .any(|signature| signature.as_str() == "player_screen_shield_charge_micro_arcs");
    let sensor_sweep_tick_gate = runtime.sensor_sweep_tick_count == 3
        && runtime.sensor_sweep_tick_width_px == 8
        && runtime.sensor_sweep_tick_height_px == 2
        && sensor_sweep_tick_pixel_budget <= 48
        && player_screen_animation_signatures
            .iter()
            .any(|signature| signature.as_str() == "player_screen_sensor_sweep_micro_ticks");
    let combat_phase_motion_signatures = string_vec(["player_screen_combat_hit_micro_sparks"]);
    let combat_hit_flash_spark_gate = runtime.combat_hit_flash_site_count == 4
        && runtime.combat_hit_flash_sparks_per_site == 2
        && combat_hit_flash_spark_count == 8
        && runtime.combat_hit_flash_spark_width_px == 6
        && runtime.combat_hit_flash_spark_height_px == 2
        && combat_hit_flash_spark_pixel_budget <= 96
        && combat_phase_motion_signatures
            .iter()
            .any(|signature| signature.as_str() == "player_screen_combat_hit_micro_sparks");
    let command_feedback_motion_signatures = string_vec([
        "feedback_player_labels",
        "battlefield_command_feedback_micro_trail",
    ]);
    let feedback_battlefield_trail_gate = runtime.feedback_move_trail_origin_count == 2
        && runtime.feedback_move_trail_step_count_per_origin == 10
        && feedback_move_trail_tick_count == 20
        && runtime.feedback_move_trail_tick_width_px == 2
        && runtime.feedback_move_trail_tick_height_px == 2
        && feedback_move_trail_pixel_budget <= 80
        && command_feedback_motion_signatures
            .iter()
            .any(|signature| signature.as_str() == "battlefield_command_feedback_micro_trail");
    let opening_action_motion_signatures = string_vec(["opening_action_path_micro_dots"]);
    let opening_action_path_gate = runtime.opening_action_path_count == 3
        && runtime.opening_action_path_step_count == 24
        && runtime.opening_action_path_dot_width_px == 2
        && runtime.opening_action_path_dot_height_px == 2
        && opening_action_path_pixel_budget <= 96
        && opening_action_motion_signatures
            .iter()
            .any(|signature| signature.as_str() == "opening_action_path_micro_dots");
    let animation_frame_pixel_budget = animation_samples.len() * 88;
    let animation_frame_richness_sample_count = animation_samples.len();
    let animation_frame_richness_pixel_budget = animation_frame_richness_sample_count * 40;
    let opening_action_gate = opening_action_ids
        == string_vec([
            "worker_harvest_flux",
            "build_flux_relay",
            "train_worker",
            "train_horizon_scout",
            "secure_flux_beacon",
        ])
        && action_verbs == string_vec(["worker", "build", "train", "train", "secure"])
        && progress_meter_pixel_budget >= 200
        && opening_action_path_gate;
    let unit_state_motion_gate = unit_status_badges == string_vec(["W", "S", "R", "G"])
        && unit_status_color_roles == string_vec(["health", "mana", "attack", "confirm"])
        && telemetry
            .unit_statuses
            .iter()
            .all(|status| status.health_percent >= 60 && status.shield_percent > 0)
        && unit_status_pixel_budget >= 256
        && production_training_spark_gate
        && warden_attack_arm_gate
        && animation_training_tick_gate;
    let tactical_track_motion_gate = telemetry.tactical_tracks.len() == 6
        && action_trail_count == 3
        && npc_action_count == 3
        && primary_tactical_track_count == 1
        && secondary_tactical_track_count == 5
        && telemetry.tactical_tracks.iter().any(|track| {
            track.from_tile == opening.active_relay_tile
                && track.to_tile == opening.active_beacon_tile
                && track.color_role.as_str() == "action_trail"
        })
        && primary_tactical_track_pixel_budget >= 48
        && secondary_tactical_track_pixel_budget <= 80
        && tactical_track_pixel_budget <= 128
        && secondary_tactical_track_height_px == 1
        && secondary_tactical_track_darken_numerator == 3
        && secondary_tactical_track_darken_denominator == 4
        && tactical_track_density_signatures
            .iter()
            .any(|signature| signature.as_str() == "primary_relay_beacon_track_kept_hot")
        && tactical_track_density_signatures
            .iter()
            .any(|signature| signature.as_str() == "secondary_tracks_dimmed")
        && tactical_track_density_signatures
            .iter()
            .any(|signature| signature.as_str() == "secondary_tracks_faded_into_terrain")
        && tactical_track_density_signatures
            .iter()
            .any(|signature| signature.as_str() == "secondary_tracks_one_pixel");
    let command_feedback_motion_gate = feedback.selected_group.as_str() == "GROUP 1"
        && feedback.active_order.as_str() == "SECURE BEACON"
        && feedback.target_tile == opening.active_beacon_tile
        && feedback.queued_after > feedback.queued_before
        && feedback.command_ack_progress > feedback.cooldown_progress
        && feedback_pixel_budget >= 190
        && feedback_player_label_gate
        && feedback_battlefield_trail_gate;
    let runtime_motion_gate = runtime.walk_cycle_frame >= 2
        && runtime.combat_turn >= 3
        && runtime.training_progress_percent >= 60
        && runtime.build_progress_percent >= 40
        && runtime.training_progress_percent >= runtime.build_progress_percent
        && runtime.command_destination_tile.as_deref() == Some(active_beacon_tile_id.as_str())
        && route_tile_ids.len() >= 4
        && route_tile_ids.last().map(|tile| tile.as_str()) == Some(active_beacon_tile_id.as_str())
        && runtime.attack_target_id.as_deref() == Some("trnm.flux.beacon")
        && combat_event_log
            .iter()
            .any(|event| event == "worker_carry_supply")
        && combat_event_log
            .iter()
            .any(|event| event == "secure_beacon:16,9");
    let green = opening_action_gate
        && unit_state_motion_gate
        && tactical_track_motion_gate
        && command_feedback_motion_gate
        && runtime_motion_gate
        && animation_training_tick_gate
        && carry_load_pip_gate
        && shield_charge_arc_gate
        && sensor_sweep_tick_gate
        && combat_hit_flash_spark_gate;
    let unit_animation_frame_gate = unit_animation_frame_count >= 8
        && animation_roles.iter().any(|role| role.as_str() == "worker")
        && animation_roles.iter().any(|role| role.as_str() == "scout")
        && animation_roles.iter().any(|role| role.as_str() == "warden")
        && animation_roles.iter().any(|role| role.as_str() == "relay")
        && animation_signatures
            .iter()
            .any(|signature| signature.as_str() == "harvest_tool_swing_frame")
        && animation_signatures
            .iter()
            .any(|signature| signature.as_str() == "sensor_sweep_arc")
        && animation_signatures
            .iter()
            .any(|signature| signature.as_str() == "attack_recoil_ticks");
    let building_animation_frame_gate = building_animation_frame_count >= 3
        && animation_signatures
            .iter()
            .any(|signature| signature.as_str() == "training_tick_lane")
        && animation_signatures
            .iter()
            .any(|signature| signature.as_str() == "spawn_door_open_frame")
        && animation_signatures
            .iter()
            .any(|signature| signature.as_str() == "construction_spark_ladder");
    let objective_animation_frame_gate = objective_animation_frame_count >= 2
        && animation_signatures
            .iter()
            .any(|signature| signature.as_str() == "capture_pulse_frame")
        && animation_signatures
            .iter()
            .any(|signature| signature.as_str() == "rally_flag_flutter");
    let animation_cycle_detail_gate = unit_animation_frame_gate
        && building_animation_frame_gate
        && objective_animation_frame_gate
        && unique_animation_signature_count >= 13
        && animation_frame_pixel_budget >= 1_144;
    let animation_frame_richness_gate = animation_frame_richness_signatures
        == string_vec([
            "animation_secondary_pose_offsets",
            "animation_contact_smear_ticks",
            "animation_structure_shutter_frames",
            "animation_objective_afterglow_frames",
        ])
        && animation_frame_richness_sample_count >= 13
        && animation_frame_richness_pixel_budget >= 520;
    let green = green && animation_cycle_detail_gate && animation_frame_richness_gate;

    json!({
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_MOTION_READABILITY_CONTRACT,
        "green": green,
        "source_path": "trnm-world-bevy classic_draw_first_contact_opening_actions + classic_draw_first_contact_unit_state_layers + classic_draw_first_contact_combat_phase_layers + classic_draw_first_contact_command_feedback_layers + classic_draw_first_contact_animation_readability_layer",
        "active_relay_tile": active_relay_tile_id,
        "active_beacon_tile": active_beacon_tile_id,
        "opening_action_ids": opening_action_ids,
        "action_verbs": action_verbs,
        "progress_meter_pixel_budget": progress_meter_pixel_budget,
        "opening_action_motion_signatures": opening_action_motion_signatures,
        "opening_action_path_count": runtime.opening_action_path_count,
        "opening_action_path_step_count": runtime.opening_action_path_step_count,
        "opening_action_path_dot_width_px": runtime.opening_action_path_dot_width_px,
        "opening_action_path_dot_height_px": runtime.opening_action_path_dot_height_px,
        "opening_action_path_pixel_budget": opening_action_path_pixel_budget,
        "opening_action_path_gate": opening_action_path_gate,
        "opening_action_gate": opening_action_gate,
        "unit_status_badges": unit_status_badges,
        "unit_status_color_roles": unit_status_color_roles,
        "unit_status_pixel_budget": unit_status_pixel_budget,
        "unit_state_motion_signatures": unit_state_motion_signatures,
        "production_training_spark_count": runtime.production_training_spark_count,
        "production_training_spark_width_px": runtime.production_training_spark_width_px,
        "production_training_spark_height_px": runtime.production_training_spark_height_px,
        "production_training_spark_pixel_budget": production_training_spark_pixel_budget,
        "production_training_spark_gate": production_training_spark_gate,
        "warden_attack_arm_count": runtime.warden_attack_arm_count,
        "warden_attack_arm_width_px": runtime.warden_attack_arm_width_px,
        "warden_attack_arm_height_px": runtime.warden_attack_arm_height_px,
        "warden_attack_arm_pixel_budget": warden_attack_arm_pixel_budget,
        "warden_attack_arm_gate": warden_attack_arm_gate,
        "player_screen_animation_signatures": player_screen_animation_signatures,
        "animation_training_tick_count": runtime.animation_training_tick_count,
        "animation_training_tick_width_px": runtime.animation_training_tick_width_px,
        "animation_training_tick_height_px": runtime.animation_training_tick_height_px,
        "animation_training_tick_pixel_budget": animation_training_tick_pixel_budget,
        "animation_training_tick_gate": animation_training_tick_gate,
        "carry_load_pip_count": runtime.carry_load_pip_count,
        "carry_load_pip_width_px": runtime.carry_load_pip_width_px,
        "carry_load_pip_height_px": runtime.carry_load_pip_height_px,
        "carry_load_pip_pixel_budget": carry_load_pip_pixel_budget,
        "carry_load_pip_gate": carry_load_pip_gate,
        "shield_charge_arc_count": runtime.shield_charge_arc_count,
        "shield_charge_arc_width_px": runtime.shield_charge_arc_width_px,
        "shield_charge_arc_height_px": runtime.shield_charge_arc_height_px,
        "shield_charge_arc_pixel_budget": shield_charge_arc_pixel_budget,
        "shield_charge_arc_gate": shield_charge_arc_gate,
        "sensor_sweep_tick_count": runtime.sensor_sweep_tick_count,
        "sensor_sweep_tick_width_px": runtime.sensor_sweep_tick_width_px,
        "sensor_sweep_tick_height_px": runtime.sensor_sweep_tick_height_px,
        "sensor_sweep_tick_pixel_budget": sensor_sweep_tick_pixel_budget,
        "sensor_sweep_tick_gate": sensor_sweep_tick_gate,
        "combat_phase_motion_signatures": combat_phase_motion_signatures,
        "combat_hit_flash_site_count": runtime.combat_hit_flash_site_count,
        "combat_hit_flash_sparks_per_site": runtime.combat_hit_flash_sparks_per_site,
        "combat_hit_flash_spark_count": combat_hit_flash_spark_count,
        "combat_hit_flash_spark_width_px": runtime.combat_hit_flash_spark_width_px,
        "combat_hit_flash_spark_height_px": runtime.combat_hit_flash_spark_height_px,
        "combat_hit_flash_spark_pixel_budget": combat_hit_flash_spark_pixel_budget,
        "combat_hit_flash_spark_gate": combat_hit_flash_spark_gate,
        "unit_state_motion_gate": unit_state_motion_gate,
        "track_roles": track_roles,
        "track_samples": track_sample_objects,
        "action_trail_count": action_trail_count,
        "npc_action_count": npc_action_count,
        "primary_tactical_track_count": primary_tactical_track_count,
        "secondary_tactical_track_count": secondary_tactical_track_count,
        "primary_tactical_track_pixel_budget": primary_tactical_track_pixel_budget,
        "secondary_tactical_track_pixel_budget": secondary_tactical_track_pixel_budget,
        "secondary_tactical_track_height_px": secondary_tactical_track_height_px,
        "secondary_tactical_track_darken_numerator": secondary_tactical_track_darken_numerator,
        "secondary_tactical_track_darken_denominator": secondary_tactical_track_darken_denominator,
        "tactical_track_density_signatures": tactical_track_density_signatures,
        "tactical_track_pixel_budget": tactical_track_pixel_budget,
        "tactical_track_motion_gate": tactical_track_motion_gate,
        "feedback_selected_group": feedback.selected_group.clone(),
        "feedback_active_order": feedback.active_order.clone(),
        "feedback_target_tile": first_contact_tile_id(feedback.target_tile),
        "feedback_queued_before": feedback.queued_before,
        "feedback_queued_after": feedback.queued_after,
        "feedback_command_ack_progress": feedback.command_ack_progress,
        "feedback_cooldown_progress": feedback.cooldown_progress,
        "feedback_player_labels": feedback_player_labels,
        "feedback_raw_marker_gate": feedback_raw_marker_gate,
        "feedback_player_label_gate": feedback_player_label_gate,
        "feedback_pixel_budget": feedback_pixel_budget,
        "command_feedback_motion_signatures": command_feedback_motion_signatures,
        "feedback_move_trail_origin_count": runtime.feedback_move_trail_origin_count,
        "feedback_move_trail_step_count_per_origin": runtime.feedback_move_trail_step_count_per_origin,
        "feedback_move_trail_tick_count": feedback_move_trail_tick_count,
        "feedback_move_trail_tick_width_px": runtime.feedback_move_trail_tick_width_px,
        "feedback_move_trail_tick_height_px": runtime.feedback_move_trail_tick_height_px,
        "feedback_move_trail_pixel_budget": feedback_move_trail_pixel_budget,
        "feedback_battlefield_trail_gate": feedback_battlefield_trail_gate,
        "command_feedback_motion_gate": command_feedback_motion_gate,
        "walk_cycle_frame": runtime.walk_cycle_frame,
        "combat_turn": runtime.combat_turn,
        "route_tile_ids": route_tile_ids,
        "command_destination_tile": runtime.command_destination_tile.clone(),
        "attack_target_id": runtime.attack_target_id.clone(),
        "combat_event_log": combat_event_log,
        "training_progress_percent": runtime.training_progress_percent,
        "build_progress_percent": runtime.build_progress_percent,
        "runtime_motion_gate": runtime_motion_gate,
        "animation_sample_tiles": animation_sample_tiles,
        "animation_roles": animation_roles,
        "animation_signatures": animation_signatures,
        "animation_frame_richness_source_path": "trnm-world-bevy classic_draw_first_contact_animation_frame_richness_detail",
        "animation_frame_richness_signatures": animation_frame_richness_signatures,
        "animation_frame_richness_sample_count": animation_frame_richness_sample_count,
        "animation_frame_richness_pixel_budget": animation_frame_richness_pixel_budget,
        "animation_samples": animation_sample_objects,
        "unit_animation_frame_count": unit_animation_frame_count,
        "building_animation_frame_count": building_animation_frame_count,
        "objective_animation_frame_count": objective_animation_frame_count,
        "unique_animation_signature_count": unique_animation_signature_count,
        "animation_frame_pixel_budget": animation_frame_pixel_budget,
        "unit_animation_frame_gate": unit_animation_frame_gate,
        "building_animation_frame_gate": building_animation_frame_gate,
        "objective_animation_frame_gate": objective_animation_frame_gate,
        "animation_cycle_detail_gate": animation_cycle_detail_gate,
        "animation_frame_richness_gate": animation_frame_richness_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_motion_readability_helpers_preserve_activity_contracts() {
        let runtime = RtsFirstContactMotionReadabilityRuntime {
            walk_cycle_frame: 2,
            combat_turn: 3,
            training_progress_percent: 60,
            build_progress_percent: 40,
            command_destination_tile: Some("16,9".to_string()),
            route_tile_ids: vec![
                "14,11".to_string(),
                "15,11".to_string(),
                "16,10".to_string(),
                "16,9".to_string(),
            ],
            attack_target_id: Some("trnm.flux.beacon".to_string()),
            combat_event_log: vec![
                "worker_carry_supply".to_string(),
                "secure_beacon:16,9".to_string(),
            ],
            feedback_move_trail_origin_count: 2,
            feedback_move_trail_step_count_per_origin: 10,
            feedback_move_trail_tick_width_px: 2,
            feedback_move_trail_tick_height_px: 2,
            opening_action_path_count: 3,
            opening_action_path_step_count: 24,
            opening_action_path_dot_width_px: 2,
            opening_action_path_dot_height_px: 2,
            warden_attack_arm_count: 3,
            warden_attack_arm_width_px: 14,
            warden_attack_arm_height_px: 2,
            production_training_spark_count: 3,
            production_training_spark_width_px: 4,
            production_training_spark_height_px: 2,
            animation_training_tick_count: 3,
            animation_training_tick_width_px: 4,
            animation_training_tick_height_px: 2,
            shield_charge_arc_count: 3,
            shield_charge_arc_width_px: 10,
            shield_charge_arc_height_px: 2,
            sensor_sweep_tick_count: 3,
            sensor_sweep_tick_width_px: 8,
            sensor_sweep_tick_height_px: 2,
            carry_load_pip_count: 4,
            carry_load_pip_width_px: 4,
            carry_load_pip_height_px: 2,
            combat_hit_flash_site_count: 4,
            combat_hit_flash_sparks_per_site: 2,
            combat_hit_flash_spark_width_px: 6,
            combat_hit_flash_spark_height_px: 2,
        };
        let guard = first_contact_motion_readability_guard(
            &trnm_rts_data::first_contact_opening_loop_profile(),
            &trnm_rts_data::first_contact_command_feedback_profile(),
            &trnm_rts_data::first_contact_visual_telemetry_profile(),
            &runtime,
        );

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_MOTION_READABILITY_CONTRACT)
        );
        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard.get("opening_action_ids").cloned(),
            Some(json!([
                "worker_harvest_flux",
                "build_flux_relay",
                "train_worker",
                "train_horizon_scout",
                "secure_flux_beacon"
            ]))
        );
        assert_eq!(
            guard.get("track_roles").cloned(),
            Some(json!([
                "action_trail",
                "npc_action",
                "action_trail",
                "npc_action",
                "action_trail",
                "npc_action"
            ]))
        );
        assert_eq!(
            guard
                .get("opening_action_motion_signatures")
                .and_then(Value::as_array)
                .map(|signatures| {
                    signatures
                        .iter()
                        .any(|value| value.as_str() == Some("opening_action_path_micro_dots"))
                }),
            Some(true)
        );
        assert_eq!(
            guard
                .get("opening_action_path_count")
                .and_then(Value::as_u64),
            Some(3)
        );
        assert_eq!(
            guard
                .get("opening_action_path_step_count")
                .and_then(Value::as_u64),
            Some(24)
        );
        assert_eq!(
            guard
                .get("opening_action_path_dot_width_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("opening_action_path_dot_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("opening_action_path_pixel_budget")
                .and_then(Value::as_u64),
            Some(96)
        );
        assert_eq!(
            guard
                .get("opening_action_path_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard
                .get("primary_tactical_track_count")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            guard
                .get("secondary_tactical_track_count")
                .and_then(Value::as_u64),
            Some(5)
        );
        assert_eq!(
            guard
                .get("secondary_tactical_track_pixel_budget")
                .and_then(Value::as_u64),
            Some(80)
        );
        assert_eq!(
            guard
                .get("tactical_track_pixel_budget")
                .and_then(Value::as_u64),
            Some(128)
        );
        assert_eq!(
            guard
                .get("tactical_track_density_signatures")
                .and_then(Value::as_array)
                .map(|signatures| {
                    signatures
                        .iter()
                        .any(|value| value.as_str() == Some("primary_relay_beacon_track_kept_hot"))
                        && signatures
                            .iter()
                            .any(|value| value.as_str() == Some("secondary_tracks_dimmed"))
                        && signatures.iter().any(|value| {
                            value.as_str() == Some("secondary_tracks_faded_into_terrain")
                        })
                        && signatures
                            .iter()
                            .any(|value| value.as_str() == Some("secondary_tracks_one_pixel"))
                }),
            Some(true)
        );
        assert_eq!(
            guard
                .get("animation_frame_richness_signatures")
                .and_then(Value::as_array)
                .map(|signatures| signatures.len()),
            Some(4)
        );
        assert_eq!(
            guard
                .get("animation_frame_richness_pixel_budget")
                .and_then(Value::as_u64),
            Some(520)
        );
        assert_eq!(
            guard
                .get("unit_animation_frame_count")
                .and_then(Value::as_u64),
            Some(8)
        );
        assert_eq!(
            guard
                .get("unit_state_motion_signatures")
                .and_then(Value::as_array)
                .map(|signatures| {
                    signatures
                        .iter()
                        .any(|value| value.as_str() == Some("unit_status_badges"))
                        && signatures.iter().any(|value| {
                            value.as_str() == Some("player_screen_production_training_micro_sparks")
                        })
                        && signatures.iter().any(|value| {
                            value.as_str() == Some("player_screen_warden_attack_micro_sparks")
                        })
                }),
            Some(true)
        );
        assert_eq!(
            guard
                .get("production_training_spark_pixel_budget")
                .and_then(Value::as_u64),
            Some(24)
        );
        assert_eq!(
            guard
                .get("production_training_spark_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard
                .get("warden_attack_arm_pixel_budget")
                .and_then(Value::as_u64),
            Some(84)
        );
        assert_eq!(
            guard.get("warden_attack_arm_gate").and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard
                .get("player_screen_animation_signatures")
                .and_then(Value::as_array)
                .map(|signatures| {
                    signatures.iter().any(|value| {
                        value.as_str() == Some("player_screen_animation_training_lane_micro_ticks")
                    }) && signatures.iter().any(|value| {
                        value.as_str() == Some("player_screen_worker_carry_load_micro_pips")
                    }) && signatures.iter().any(|value| {
                        value.as_str() == Some("player_screen_shield_charge_micro_arcs")
                    }) && signatures.iter().any(|value| {
                        value.as_str() == Some("player_screen_sensor_sweep_micro_ticks")
                    })
                }),
            Some(true)
        );
        assert_eq!(
            guard
                .get("animation_training_tick_pixel_budget")
                .and_then(Value::as_u64),
            Some(24)
        );
        assert_eq!(
            guard
                .get("animation_training_tick_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard.get("carry_load_pip_count").and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard.get("carry_load_pip_width_px").and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("carry_load_pip_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("carry_load_pip_pixel_budget")
                .and_then(Value::as_u64),
            Some(32)
        );
        assert_eq!(
            guard.get("carry_load_pip_gate").and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard
                .get("shield_charge_arc_pixel_budget")
                .and_then(Value::as_u64),
            Some(60)
        );
        assert_eq!(
            guard.get("shield_charge_arc_gate").and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard
                .get("sensor_sweep_tick_pixel_budget")
                .and_then(Value::as_u64),
            Some(48)
        );
        assert_eq!(
            guard.get("sensor_sweep_tick_gate").and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard
                .get("combat_phase_motion_signatures")
                .and_then(Value::as_array)
                .map(|signatures| {
                    signatures.iter().any(|value| {
                        value.as_str() == Some("player_screen_combat_hit_micro_sparks")
                    })
                }),
            Some(true)
        );
        assert_eq!(
            guard
                .get("combat_hit_flash_spark_pixel_budget")
                .and_then(Value::as_u64),
            Some(96)
        );
        assert_eq!(
            guard
                .get("combat_hit_flash_spark_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard.get("feedback_player_labels").cloned(),
            Some(json!([
                "GROUP 1 SECURING BEACON",
                "QUEUE ADDED  ORDER READY",
                "ROUTE BLOCKED MID VENT"
            ]))
        );
        assert_eq!(
            guard
                .get("feedback_raw_marker_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard
                .get("feedback_player_label_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard
                .get("command_feedback_motion_signatures")
                .and_then(Value::as_array)
                .map(|signatures| {
                    signatures
                        .iter()
                        .any(|value| value.as_str() == Some("feedback_player_labels"))
                        && signatures.iter().any(|value| {
                            value.as_str() == Some("battlefield_command_feedback_micro_trail")
                        })
                }),
            Some(true)
        );
        assert_eq!(
            guard
                .get("feedback_move_trail_tick_count")
                .and_then(Value::as_u64),
            Some(20)
        );
        assert_eq!(
            guard
                .get("feedback_move_trail_tick_width_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("feedback_move_trail_tick_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("feedback_move_trail_pixel_budget")
                .and_then(Value::as_u64),
            Some(80)
        );
        assert_eq!(
            guard
                .get("feedback_battlefield_trail_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard
                .get("building_animation_frame_count")
                .and_then(Value::as_u64),
            Some(3)
        );
        assert_eq!(
            guard
                .get("objective_animation_frame_count")
                .and_then(Value::as_u64),
            Some(2)
        );
        for gate in [
            "opening_action_gate",
            "opening_action_path_gate",
            "unit_state_motion_gate",
            "production_training_spark_gate",
            "warden_attack_arm_gate",
            "shield_charge_arc_gate",
            "sensor_sweep_tick_gate",
            "combat_hit_flash_spark_gate",
            "tactical_track_motion_gate",
            "command_feedback_motion_gate",
            "feedback_raw_marker_gate",
            "feedback_player_label_gate",
            "feedback_battlefield_trail_gate",
            "runtime_motion_gate",
            "unit_animation_frame_gate",
            "building_animation_frame_gate",
            "objective_animation_frame_gate",
            "animation_cycle_detail_gate",
            "animation_frame_richness_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }
}
