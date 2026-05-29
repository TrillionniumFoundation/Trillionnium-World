fn main() {
    let world = trnm_world_bevy::native_bevy_playable_fixture();
    let args: Vec<String> = std::env::args().skip(1).filter(|arg| arg != "--").collect();
    if matches!(
        args.first().map(String::as_str),
        Some("playable-slice" | "--playable-slice" | "evidence")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_playable_slice_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("first-playable" | "--first-playable")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_first_playable_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("account-client-boundary" | "--account-client-boundary" | "account-client")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_account_client_boundary_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("account-title-flow" | "--account-title-flow")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_account_title_flow_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-campaign-entry" | "--classic-rts-campaign-entry" | "campaign-entry")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_campaign_entry_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-visual-fidelity" | "--classic-rts-visual-fidelity" | "visual-fidelity")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-visual-fidelity.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_visual_fidelity_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-first-contact-basin-spec"
                | "--classic-rts-first-contact-basin-spec"
                | "first-contact-basin-spec"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_first_contact_basin_spec_evidence_json()
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-first-contact-opening-loop"
                | "--classic-rts-first-contact-opening-loop"
                | "first-contact-opening-loop"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_first_contact_opening_loop_evidence_json()
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-openra-like-core" | "--classic-rts-openra-like-core" | "openra-like-core"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_openra_like_core_evidence_json()
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-command-affordance"
                | "--classic-rts-command-affordance"
                | "command-affordance"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-affordance.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_command_affordance_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-action-cadence" | "--classic-rts-action-cadence" | "action-cadence")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-cadence.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_action_cadence_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-unit-model-depth" | "--classic-rts-unit-model-depth" | "unit-model-depth"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-model-depth.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_unit_model_depth_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-action-sequence" | "--classic-rts-action-sequence" | "action-sequence")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-action-sequence.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_action_sequence_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-npc-behavior" | "--classic-rts-npc-behavior" | "npc-behavior")
    ) {
        let preview_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-behavior.ppm");
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_npc_behavior_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-combat-impact" | "--classic-rts-combat-impact" | "combat-impact")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-impact.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_combat_impact_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-locomotion-blend" | "--classic-rts-locomotion-blend" | "locomotion-blend"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-locomotion-blend.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_locomotion_blend_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-npc-transition" | "--classic-rts-npc-transition" | "npc-transition")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-npc-transition.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_npc_transition_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-depth-readability"
                | "--classic-rts-depth-readability"
                | "depth-readability"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-depth-readability.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_depth_readability_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-command-surface" | "--classic-rts-command-surface" | "command-surface")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-surface.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_command_surface_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-structure-modeling"
                | "--classic-rts-structure-modeling"
                | "structure-modeling"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-structure-modeling.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_structure_modeling_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-environment-life" | "--classic-rts-environment-life" | "environment-life"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-environment-life.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_environment_life_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-map-model-gap" | "--classic-rts-map-model-gap" | "map-model-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-model-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_map_model_gap_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-worker-harvest-animation"
                | "--classic-rts-worker-harvest-animation"
                | "worker-harvest-animation"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-worker-harvest-animation.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_worker_harvest_animation_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-production-spawn-animation"
                | "--classic-rts-production-spawn-animation"
                | "production-spawn-animation"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-spawn-animation.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_production_spawn_animation_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-unit-status-portrait"
                | "--classic-rts-unit-status-portrait"
                | "unit-status-portrait"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-unit-status-portrait.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_unit_status_portrait_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-selection-command-feedback"
                | "--classic-rts-selection-command-feedback"
                | "selection-command-feedback"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-command-feedback.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_selection_command_feedback_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-ability-tooltip-telegraph"
                | "--classic-rts-ability-tooltip-telegraph"
                | "ability-tooltip-telegraph"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ability-tooltip-telegraph.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_ability_tooltip_telegraph_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-control-group-hotkey-feedback"
                | "--classic-rts-control-group-hotkey-feedback"
                | "control-group-hotkey-feedback"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-hotkey-feedback.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_control_group_hotkey_feedback_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-control-group-recall-formation-preview"
                | "--classic-rts-control-group-recall-formation-preview"
                | "control-group-recall-formation-preview"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-recall-formation-preview.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_control_group_recall_formation_preview_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-control-group-recall-override-preview"
                | "--classic-rts-control-group-recall-override-preview"
                | "control-group-recall-override-preview"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-recall-override-preview.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_control_group_recall_override_preview_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-control-group-command-feedback-strip"
                | "--classic-rts-control-group-command-feedback-strip"
                | "control-group-command-feedback-strip"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-command-feedback-strip.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_control_group_command_feedback_strip_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-control-group-command-feedback-lifecycle"
                | "--classic-rts-control-group-command-feedback-lifecycle"
                | "control-group-command-feedback-lifecycle"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-command-feedback-lifecycle.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_control_group_command_feedback_lifecycle_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-control-group-command-history"
                | "--classic-rts-control-group-command-history"
                | "control-group-command-history"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-command-history.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_control_group_command_history_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-control-group-command-history-prune"
                | "--classic-rts-control-group-command-history-prune"
                | "control-group-command-history-prune"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-group-command-history-prune.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_control_group_command_history_prune_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-scrollable-map" | "--classic-rts-scrollable-map" | "scrollable-map")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-scrollable-map.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_scrollable_map_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-camera-minimap-sync"
                | "--classic-rts-camera-minimap-sync"
                | "camera-minimap-sync"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-camera-minimap-sync.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_camera_minimap_sync_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-command-queue-path-preview"
                | "--classic-rts-command-queue-path-preview"
                | "command-queue-path-preview"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-command-queue-path-preview.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_command_queue_path_preview_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-formation-move-preview"
                | "--classic-rts-formation-move-preview"
                | "formation-move-preview"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-preview.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_formation_move_preview_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-formation-move-execution"
                | "--classic-rts-formation-move-execution"
                | "formation-move-execution"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-formation-move-execution.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_formation_move_execution_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-local-obstruction-recovery"
                | "--classic-rts-local-obstruction-recovery"
                | "local-obstruction-recovery"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-local-obstruction-recovery.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_local_obstruction_recovery_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("scene-transition-playability" | "--scene-transition-playability")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_scene_transition_playability_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("live-play-session" | "--live-play-session")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_live_play_session_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("live-input-sampling" | "--live-input-sampling")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_live_input_sampling_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("multi-room-playability" | "--multi-room-playability")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_multi_room_playability_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("turn-combat-playability" | "--turn-combat-playability")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_turn_combat_playability_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("four-lane-playability" | "--four-lane-playability")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_four_lane_playability_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("visible-actor-runtime" | "--visible-actor-runtime")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_visible_actor_runtime_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("progression-loop" | "--progression-loop")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_progression_loop_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("scene-pack" | "--scene-pack")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_scene_pack_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("scene-pack-route-director" | "--scene-pack-route-director")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_scene_pack_route_director_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("contextual-action-deck" | "--contextual-action-deck")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_contextual_action_deck_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("contextual-button-state" | "--contextual-button-state")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_contextual_button_state_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("contextual-button-guard" | "--contextual-button-guard")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_contextual_button_guard_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("keyboard-input-guard" | "--keyboard-input-guard")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_keyboard_input_guard_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("input-feedback-loop" | "--input-feedback-loop")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_input_feedback_loop_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("input-replay-telemetry" | "--input-replay-telemetry")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_input_replay_telemetry_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("input-telemetry-summary" | "--input-telemetry-summary")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_input_telemetry_summary_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("input-telemetry-hud" | "--input-telemetry-hud")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_input_telemetry_hud_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("player-hud-debug-layer" | "--player-hud-debug-layer")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_player_hud_debug_layer_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("player-ui-rescue" | "--player-ui-rescue")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_player_ui_rescue_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-asset-pack" | "--classic-asset-pack" | "classic-assets")
    ) {
        let manifest_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("../assets/trnm-world/classic/manifest.json");
        let atlas_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("../assets/trnm-world/classic/atlas.ppm");
        println!(
            "{}",
            trnm_world_bevy::native_classic_asset_pack_evidence_json(manifest_path, atlas_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-scene-preview" | "--classic-scene-preview" | "classic-preview")
    ) {
        let preview_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("../acceptance/S5_native_bevy_device/latest/bevy-classic-scene-preview.ppm");
        println!(
            "{}",
            trnm_world_bevy::native_classic_scene_preview_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-model-catalog" | "--classic-model-catalog" | "classic-catalog")
    ) {
        let catalog_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("../acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.ppm");
        println!(
            "{}",
            trnm_world_bevy::native_classic_model_catalog_evidence_json(catalog_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-asset-slot-map" | "--classic-asset-slot-map" | "classic-slots")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_classic_asset_slot_map_evidence_json()
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-asset-override-probe"
                | "--classic-asset-override-probe"
                | "classic-override-probe"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-asset-override-probe.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_asset_override_probe_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-art-pack" | "--classic-art-pack" | "classic-artpack")
    ) {
        let override_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("../assets/trnm-world/classic/art-pack-v1");
        let preview_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("../acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack.ppm");
        println!(
            "{}",
            trnm_world_bevy::native_classic_art_pack_evidence_json(override_dir, preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-art-pack-scene-probe"
                | "--classic-art-pack-scene-probe"
                | "classic-artpack-scene"
        )
    ) {
        let override_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("../assets/trnm-world/classic/art-pack-v1");
        let preview_path = args.get(2).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-art-pack-scene-probe.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_art_pack_scene_probe_evidence_json(
                override_dir,
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-renderer-probe" | "--classic-renderer-probe" | "classic-probe")
    ) {
        let frame_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_renderer_probe_evidence_json(frame_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-isometric-modeling" | "--classic-isometric-modeling" | "classic-iso-modeling"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-isometric-modeling.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_isometric_modeling_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-manifest-lint" | "--classic-manifest-lint" | "classic-lint")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_classic_manifest_lint_evidence_json()
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-animation-preview" | "--classic-animation-preview" | "classic-animation")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_animation_preview_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-animation-selector" | "--classic-animation-selector" | "classic-selector")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_classic_animation_selector_evidence_json()
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-player-motion-probe" | "--classic-player-motion-probe" | "classic-motion")
    ) {
        let probe_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-player-motion-probe.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_player_motion_probe_evidence_json(probe_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-render-budget" | "--classic-render-budget" | "classic-budget")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_classic_render_budget_evidence_json()
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-control-loop" | "--classic-rts-control-loop" | "classic-rts-control")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_control_loop_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-live-input-sequence"
                | "--classic-rts-live-input-sequence"
                | "classic-rts-live-input"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-input-sequence.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_live_input_sequence_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-pathing-formation" | "--classic-rts-pathing-formation")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-pathing-formation.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_pathing_formation_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-collision-engagement" | "--classic-rts-collision-engagement")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-collision-engagement.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_collision_engagement_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-target-aggro-focus" | "--classic-rts-target-aggro-focus")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-target-aggro-focus.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_target_aggro_focus_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-economy-build" | "--classic-rts-economy-build")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-economy-build.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_economy_build_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-selection-minimap" | "--classic-rts-selection-minimap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-minimap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_selection_minimap_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-build-lifecycle" | "--classic-rts-build-lifecycle")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-build-lifecycle.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_build_lifecycle_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-tech-tree" | "--classic-rts-tech-tree")
    ) {
        let preview_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tech-tree.ppm");
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_tech_tree_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-projectile-ability" | "--classic-rts-projectile-ability")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-projectile-ability.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_projectile_ability_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-ai-skirmish-pressure" | "--classic-rts-ai-skirmish-pressure")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-ai-skirmish-pressure.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_ai_skirmish_pressure_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-objective-victory-loop" | "--classic-rts-objective-victory-loop")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-objective-victory-loop.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_objective_victory_loop_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-bot-terminal-loop" | "--classic-rts-bot-terminal-loop")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-terminal-loop.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_bot_terminal_loop_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-autonomous-bot-skirmish" | "--classic-rts-autonomous-bot-skirmish")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-autonomous-bot-skirmish.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_autonomous_bot_skirmish_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-organic-terminal-gap" | "--classic-rts-organic-terminal-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-organic-terminal-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_organic_terminal_gap_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-terminal-observation-gap" | "--classic-rts-terminal-observation-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-terminal-observation-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_terminal_observation_gap_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-replay-metrics-gap" | "--classic-rts-replay-metrics-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-replay-metrics-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_replay_metrics_gap_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-endurance-skirmish-gap" | "--classic-rts-endurance-skirmish-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-endurance-skirmish-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_endurance_skirmish_gap_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-openra-parity-bridge" | "--classic-rts-openra-parity-bridge")
    ) {
        let preview_dir = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-parity-bridge",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_openra_parity_bridge_evidence_json(preview_dir)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-owned-replay-file" | "--classic-rts-owned-replay-file")
    ) {
        let replay_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-owned-replay-file.trnm-replay.json",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_owned_replay_file_evidence_json(replay_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-headless-replay-playback" | "--classic-rts-headless-replay-playback")
    ) {
        let replay_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-owned-replay-file.trnm-replay.json",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_headless_replay_playback_evidence_json(replay_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-playtest-observability-readiness"
                | "--classic-rts-playtest-observability-readiness"
        )
    ) {
        let preview_dir = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-playtest-observability-readiness",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_playtest_observability_readiness_evidence_json(
                preview_dir
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-bot-decision-state-gap" | "--classic-rts-bot-decision-state-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-decision-state-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_bot_decision_state_gap_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-bot-adaptive-build-order-gap"
                | "--classic-rts-bot-adaptive-build-order-gap"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-adaptive-build-order-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_bot_adaptive_build_order_gap_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-bot-tactical-micro-gap" | "--classic-rts-bot-tactical-micro-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tactical-micro-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_bot_tactical_micro_gap_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-bot-map-intel-gap" | "--classic-rts-bot-map-intel-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-map-intel-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_bot_map_intel_gap_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-bot-macro-economy-gap" | "--classic-rts-bot-macro-economy-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-macro-economy-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_bot_macro_economy_gap_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-bot-harassment-defense-gap" | "--classic-rts-bot-harassment-defense-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-harassment-defense-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_bot_harassment_defense_gap_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-bot-multi-front-pressure-gap"
                | "--classic-rts-bot-multi-front-pressure-gap"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-multi-front-pressure-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_bot_multi_front_pressure_gap_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-bot-expansion-control-gap" | "--classic-rts-bot-expansion-control-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-expansion-control-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_bot_expansion_control_gap_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-bot-tech-transition-gap" | "--classic-rts-bot-tech-transition-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tech-transition-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_bot_tech_transition_gap_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-bot-army-composition-gap" | "--classic-rts-bot-army-composition-gap")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-army-composition-gap.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_bot_army_composition_gap_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-creep-camp-terrain-route" | "--classic-rts-creep-camp-terrain-route")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-creep-camp-terrain-route.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_creep_camp_terrain_route_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-fog-scouting-intel" | "--classic-rts-fog-scouting-intel")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-fog-scouting-intel.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_fog_scouting_intel_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-enemy-base-tech-pressure" | "--classic-rts-enemy-base-tech-pressure")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-enemy-base-tech-pressure.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_enemy_base_tech_pressure_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-army-production-rally" | "--classic-rts-army-production-rally")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-army-production-rally.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_army_production_rally_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-base-assault-resolution" | "--classic-rts-base-assault-resolution")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-base-assault-resolution.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_base_assault_resolution_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-battle-aftermath" | "--classic-rts-battle-aftermath")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-battle-aftermath.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_battle_aftermath_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-commander-progression" | "--classic-rts-commander-progression")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-commander-progression.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_commander_progression_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-expansion-counterattack" | "--classic-rts-expansion-counterattack")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-expansion-counterattack.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_expansion_counterattack_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-tier-two-siege-push" | "--classic-rts-tier-two-siege-push")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tier-two-siege-push.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_tier_two_siege_push_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-siege-breach-counterplay" | "--classic-rts-siege-breach-counterplay")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-siege-breach-counterplay.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_siege_breach_counterplay_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-inner-lane-breakthrough" | "--classic-rts-inner-lane-breakthrough")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-inner-lane-breakthrough.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_inner_lane_breakthrough_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-central-keep-pressure" | "--classic-rts-central-keep-pressure")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-pressure.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_central_keep_pressure_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-central-keep-breakthrough" | "--classic-rts-central-keep-breakthrough")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-breakthrough.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_central_keep_breakthrough_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-mirror-city-restoration" | "--classic-rts-mirror-city-restoration")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-mirror-city-restoration.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_mirror_city_restoration_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-open-world-after-action" | "--classic-rts-open-world-after-action")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-open-world-after-action.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_open_world_after_action_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-campaign-handoff" | "--classic-rts-campaign-handoff")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-handoff.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_campaign_handoff_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-campaign-ui-continuity" | "--classic-rts-campaign-ui-continuity")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-ui-continuity.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_campaign_ui_continuity_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-objective-minimap-breadcrumbs"
                | "--classic-rts-objective-minimap-breadcrumbs"
        )
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-objective-minimap-breadcrumbs.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_objective_minimap_breadcrumbs_evidence_json(
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-first-minute-readiness" | "--classic-rts-first-minute-readiness")
    ) {
        let preview_path = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-minute-readiness.ppm",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_first_minute_readiness_evidence_json(preview_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("classic-rts-map-ui-modeling-readiness" | "--classic-rts-map-ui-modeling-readiness")
    ) {
        let preview_dir = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-map-ui-modeling-readiness",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_map_ui_modeling_readiness_evidence_json(
                preview_dir
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-campaign-outcome-ui-readiness"
                | "--classic-rts-campaign-outcome-ui-readiness"
        )
    ) {
        let preview_dir = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-outcome-ui-readiness",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_campaign_outcome_ui_readiness_evidence_json(
                preview_dir
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-rts-combat-readability-pressure-readiness"
                | "--classic-rts-combat-readability-pressure-readiness"
        )
    ) {
        let preview_dir = args.get(1).map(String::as_str).unwrap_or(
            "../acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-readability-pressure-readiness",
        );
        println!(
            "{}",
            trnm_world_bevy::native_classic_rts_combat_readability_pressure_readiness_evidence_json(
                preview_dir
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "classic-input-frame-budget" | "--classic-input-frame-budget" | "classic-input-budget"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_classic_input_frame_budget_evidence_json()
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("authored-art-pack" | "--authored-art-pack" | "art-pack")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_authored_art_pack_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("authored-sprite-sheet" | "--authored-sprite-sheet" | "sprite-sheet-artifact")
    ) {
        let atlas_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet.ppm");
        let manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet-manifest.json");
        println!(
            "{}",
            trnm_world_bevy::native_authored_sprite_sheet_artifact_evidence_json(
                "local-player",
                atlas_path,
                manifest_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "authored-texture-atlas-binding"
                | "--authored-texture-atlas-binding"
                | "texture-atlas-binding"
        )
    ) {
        let atlas_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet.ppm");
        let manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet-manifest.json");
        let binding_path = args
            .get(3)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-texture-atlas-binding.json");
        println!(
            "{}",
            trnm_world_bevy::native_authored_texture_atlas_binding_evidence_json(
                "local-player",
                atlas_path,
                manifest_path,
                binding_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "authored-material-consumption"
                | "--authored-material-consumption"
                | "material-consumption"
        )
    ) {
        let atlas_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet.ppm");
        let manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet-manifest.json");
        let binding_path = args
            .get(3)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-texture-atlas-binding.json");
        let consumption_path = args
            .get(4)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-material-consumption.json");
        println!(
            "{}",
            trnm_world_bevy::native_authored_material_consumption_evidence_json(
                "local-player",
                atlas_path,
                manifest_path,
                binding_path,
                consumption_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "authored-material-application"
                | "--authored-material-application"
                | "material-application"
        )
    ) {
        let atlas_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet.ppm");
        let manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet-manifest.json");
        let binding_path = args
            .get(3)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-texture-atlas-binding.json");
        let consumption_path = args
            .get(4)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-material-consumption.json");
        let application_path = args
            .get(5)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-material-application.json");
        println!(
            "{}",
            trnm_world_bevy::native_authored_material_application_evidence_json(
                "local-player",
                atlas_path,
                manifest_path,
                binding_path,
                consumption_path,
                application_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("authored-render-frame" | "--authored-render-frame" | "render-frame")
    ) {
        let atlas_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet.ppm");
        let manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet-manifest.json");
        let binding_path = args
            .get(3)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-texture-atlas-binding.json");
        let consumption_path = args
            .get(4)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-material-consumption.json");
        let application_path = args
            .get(5)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-material-application.json");
        let frame_path = args
            .get(6)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-render-frame.ppm");
        let frame_manifest_path = args
            .get(7)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-render-frame-manifest.json");
        println!(
            "{}",
            trnm_world_bevy::native_authored_render_frame_evidence_json(
                "local-player",
                atlas_path,
                manifest_path,
                binding_path,
                consumption_path,
                application_path,
                frame_path,
                frame_manifest_path,
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "runtime-texture-asset" | "--runtime-texture-asset" | "authored-runtime-texture-asset"
        )
    ) {
        let atlas_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet.ppm");
        let manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-sprite-sheet-manifest.json");
        let binding_path = args
            .get(3)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-texture-atlas-binding.json");
        let consumption_path = args
            .get(4)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-material-consumption.json");
        let application_path = args
            .get(5)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-authored-material-application.json");
        let runtime_asset_path = args
            .get(6)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-runtime-texture-asset.json");
        println!(
            "{}",
            trnm_world_bevy::native_runtime_texture_asset_evidence_json(
                "local-player",
                atlas_path,
                manifest_path,
                binding_path,
                consumption_path,
                application_path,
                runtime_asset_path,
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "runtime-texture-manifest-probe"
                | "--runtime-texture-manifest-probe"
                | "runtime-texture-probe"
        )
    ) {
        let runtime_summary_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-runtime-texture-asset.json");
        let runtime_manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-runtime-texture-asset-manifest.json");
        println!(
            "{}",
            trnm_world_bevy::native_runtime_texture_manifest_probe_evidence_json(
                "local-player",
                runtime_summary_path,
                runtime_manifest_path,
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "asset-store-registration"
                | "--asset-store-registration"
                | "runtime-texture-asset-store-registration"
        )
    ) {
        let runtime_summary_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-runtime-texture-asset.json");
        let runtime_manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-runtime-texture-asset-manifest.json");
        println!(
            "{}",
            trnm_world_bevy::native_runtime_texture_asset_store_registration_evidence_json(
                "local-player",
                runtime_summary_path,
                runtime_manifest_path,
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("sprite-asset-binding" | "--sprite-asset-binding" | "runtime-texture-sprite-binding")
    ) {
        let runtime_summary_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-runtime-texture-asset.json");
        let runtime_manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-runtime-texture-asset-manifest.json");
        println!(
            "{}",
            trnm_world_bevy::native_runtime_texture_sprite_asset_binding_evidence_json(
                "local-player",
                runtime_summary_path,
                runtime_manifest_path,
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "sprite-texture-sampling"
                | "--sprite-texture-sampling"
                | "runtime-texture-sprite-sampling"
        )
    ) {
        let runtime_summary_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-runtime-texture-asset.json");
        let runtime_manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-runtime-texture-asset-manifest.json");
        println!(
            "{}",
            trnm_world_bevy::native_runtime_texture_sprite_texture_sampling_evidence_json(
                "local-player",
                runtime_summary_path,
                runtime_manifest_path,
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "render-asset-eligibility"
                | "--render-asset-eligibility"
                | "runtime-texture-render-eligibility"
        )
    ) {
        let runtime_summary_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-runtime-texture-asset.json");
        let runtime_manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-runtime-texture-asset-manifest.json");
        let sampled_live_correlation_path = args
            .get(3)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-live-window-sampled-texture-correlation.json");
        println!(
            "{}",
            trnm_world_bevy::native_runtime_texture_render_asset_eligibility_evidence_json(
                "local-player",
                runtime_summary_path,
                runtime_manifest_path,
                sampled_live_correlation_path,
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("action-coach" | "--action-coach")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_action_coach_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("session-recovery" | "--session-recovery")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_session_recovery_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("session-recovery-ui" | "--session-recovery-ui")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_session_recovery_ui_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("session-save-slot" | "--session-save-slot")
    ) {
        let slot_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-save-slot.json");
        println!(
            "{}",
            trnm_world_bevy::native_session_save_slot_evidence_json("local-player", slot_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("session-slot-buttons" | "--session-slot-buttons")
    ) {
        let slot_path = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-action-slot.json");
        println!(
            "{}",
            trnm_world_bevy::native_session_slot_buttons_evidence_json("local-player", slot_path)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("session-slot-menu" | "--session-slot-menu")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        println!(
            "{}",
            trnm_world_bevy::native_session_slot_menu_evidence_json("local-player", slot_dir)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("session-slot-confirm" | "--session-slot-confirm")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        println!(
            "{}",
            trnm_world_bevy::native_session_slot_confirm_evidence_json("local-player", slot_dir)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("session-load-resume" | "--session-load-resume")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        println!(
            "{}",
            trnm_world_bevy::native_session_load_resume_evidence_json("local-player", slot_dir)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("pause-menu" | "--pause-menu")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        println!(
            "{}",
            trnm_world_bevy::native_pause_menu_evidence_json("local-player", slot_dir)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("settings-menu" | "--settings-menu")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        println!(
            "{}",
            trnm_world_bevy::native_settings_menu_evidence_json("local-player", slot_dir)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("title-menu" | "--title-menu")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        println!(
            "{}",
            trnm_world_bevy::native_title_menu_evidence_json("local-player", slot_dir)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("character-create" | "--character-create")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        println!(
            "{}",
            trnm_world_bevy::native_character_create_evidence_json("local-player", slot_dir)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("first-minute-onboarding" | "--first-minute-onboarding")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        println!(
            "{}",
            trnm_world_bevy::native_first_minute_onboarding_evidence_json("local-player", slot_dir)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("onboarding-objective-hud" | "--onboarding-objective-hud")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        println!(
            "{}",
            trnm_world_bevy::native_onboarding_objective_hud_evidence_json(
                "local-player",
                slot_dir
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("next-button-highlight" | "--next-button-highlight")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        println!(
            "{}",
            trnm_world_bevy::native_next_button_highlight_evidence_json("local-player", slot_dir)
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("first-minute-interaction-timeline" | "--first-minute-interaction-timeline")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        println!(
            "{}",
            trnm_world_bevy::native_first_minute_interaction_timeline_evidence_json(
                "local-player",
                slot_dir
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("first-minute-input-replay" | "--first-minute-input-replay")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        let recording_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-first-minute-recording.json");
        println!(
            "{}",
            trnm_world_bevy::native_first_minute_input_replay_evidence_json(
                "local-player",
                slot_dir,
                recording_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("first-minute-command-feedback-replay" | "--first-minute-command-feedback-replay")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        let first_minute_recording_path = args.get(2).map(String::as_str).unwrap_or(
            "target/trnm-world-bevy-first-minute-command-feedback-source-recording.json",
        );
        let command_recording_path = args
            .get(3)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-first-minute-command-feedback-recording.json");
        let preview_path = args
            .get(4)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-first-minute-command-feedback-replay.ppm");
        println!(
            "{}",
            trnm_world_bevy::native_first_minute_command_feedback_replay_evidence_json(
                "local-player",
                slot_dir,
                first_minute_recording_path,
                command_recording_path,
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "first-minute-command-feedback-rejection-replay"
                | "--first-minute-command-feedback-rejection-replay"
        )
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        let first_minute_recording_path = args.get(2).map(String::as_str).unwrap_or(
            "target/trnm-world-bevy-first-minute-command-feedback-rejection-source-recording.json",
        );
        let rejection_recording_path = args.get(3).map(String::as_str).unwrap_or(
            "target/trnm-world-bevy-first-minute-command-feedback-rejection-recording.json",
        );
        let preview_path = args
            .get(4)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-first-minute-command-feedback-rejection-replay.ppm");
        println!(
            "{}",
            trnm_world_bevy::native_first_minute_command_feedback_rejection_replay_evidence_json(
                "local-player",
                slot_dir,
                first_minute_recording_path,
                rejection_recording_path,
                preview_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("first-minute-screenshot-sequence" | "--first-minute-screenshot-sequence")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        let manifest_path = args
            .get(2)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-first-minute-screenshot-manifest.json");
        let recording_path = args
            .get(3)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-first-minute-recording.json");
        println!(
            "{}",
            trnm_world_bevy::native_first_minute_screenshot_sequence_evidence_json(
                "local-player",
                slot_dir,
                manifest_path,
                recording_path
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("visible-button-hit-test-map" | "--visible-button-hit-test-map" | "hit-test-map")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_visible_button_hit_test_map_evidence_json()
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("input-affordance-feedback" | "--input-affordance-feedback")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_input_affordance_feedback_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("quest-journal" | "--quest-journal")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_quest_journal_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("inventory-equipment" | "--inventory-equipment")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_inventory_equipment_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("stat-allocation" | "--stat-allocation")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_stat_allocation_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("stat-allocation-persistence" | "--stat-allocation-persistence")
    ) {
        let slot_dir = args
            .get(1)
            .map(String::as_str)
            .unwrap_or("target/trnm-world-bevy-session-slots");
        std::env::set_var("TRNM_WORLD_BEVY_SESSION_SLOT_DIR", slot_dir);
        println!(
            "{}",
            trnm_world_bevy::native_stat_allocation_persistence_evidence_json(
                "local-player",
                slot_dir
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("stat-gameplay-effects" | "--stat-gameplay-effects")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_stat_gameplay_effects_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("stat-feedback-ui" | "--stat-feedback-ui")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_stat_feedback_ui_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("stat-choice-preview" | "--stat-choice-preview")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_stat_choice_preview_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("stat-confirmation" | "--stat-confirmation")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_stat_confirmation_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("equipment-affix-build" | "--equipment-affix-build")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_equipment_affix_build_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-routes" | "--build-branch-routes")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_routes_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-outcomes" | "--build-branch-outcomes")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_outcomes_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-world-reactions" | "--build-branch-world-reactions")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_world_reactions_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-persistence" | "--build-branch-persistence")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_persistence_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-followup-unlocks" | "--build-branch-followup-unlocks")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_followup_unlocks_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-followup-completion" | "--build-branch-followup-completion")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_followup_completion_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-mastery" | "--build-branch-mastery")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_mastery_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-mastery-challenges" | "--build-branch-mastery-challenges")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_mastery_challenges_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-mastery-titles" | "--build-branch-mastery-titles")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_mastery_titles_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-title-equip" | "--build-branch-title-equip")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_equip_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-active-title-ui" | "--build-branch-active-title-ui")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_active_title_ui_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-title-loadout-panel" | "--build-branch-title-loadout-panel")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_loadout_panel_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-title-loadout-switch" | "--build-branch-title-loadout-switch")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_loadout_switch_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "build-branch-title-route-recommendation" | "--build-branch-title-route-recommendation"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_recommendation_evidence_json(
                "local-player"
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-title-route-accept" | "--build-branch-title-route-accept")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_accept_evidence_json("local-player")
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "build-branch-title-route-completion-handoff"
                | "--build-branch-title-route-completion-handoff"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_completion_handoff_evidence_json(
                "local-player"
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "build-branch-title-route-mastery-completion"
                | "--build-branch-title-route-mastery-completion"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_mastery_completion_evidence_json(
                "local-player"
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "build-branch-title-route-all-branch-completion"
                | "--build-branch-title-route-all-branch-completion"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_all_branch_completion_evidence_json(
                "local-player"
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "build-branch-title-route-progress-summary"
                | "--build-branch-title-route-progress-summary"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_progress_summary_evidence_json(
                "local-player"
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "build-branch-title-route-action-dashboard"
                | "--build-branch-title-route-action-dashboard"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_action_dashboard_evidence_json(
                "local-player"
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-title-route-action-focus" | "--build-branch-title-route-action-focus")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_action_focus_evidence_json(
                "local-player"
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "build-branch-title-route-action-focus-input"
                | "--build-branch-title-route-action-focus-input"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_action_focus_input_evidence_json(
                "local-player"
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-title-route-action-hint" | "--build-branch-title-route-action-hint")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_action_hint_evidence_json(
                "local-player"
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("build-branch-title-route-keyboard-loop" | "--build-branch-title-route-keyboard-loop")
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_keyboard_loop_evidence_json(
                "local-player"
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "build-branch-title-route-all-branch-keyboard-loop"
                | "--build-branch-title-route-all-branch-keyboard-loop"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_all_branch_keyboard_loop_evidence_json(
                "local-player",
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some(
            "build-branch-title-route-all-branch-keyboard-replay"
                | "--build-branch-title-route-all-branch-keyboard-replay"
        )
    ) {
        println!(
            "{}",
            trnm_world_bevy::native_build_branch_title_route_all_branch_keyboard_replay_evidence_json(
                "local-player",
            )
        );
        return;
    }
    if matches!(
        args.first().map(String::as_str),
        Some("run" | "--run" | "play")
    ) {
        trnm_world_bevy::run_native_bevy_client(world, "local-player");
        return;
    }

    let (_app, report) = trnm_world_bevy::build_native_bevy_app(world, "local-player");
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("Bevy bridge report serializes")
    );
}
