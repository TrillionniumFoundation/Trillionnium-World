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
