use super::*;
use serde_json::json;
use std::collections::HashSet;
use std::fs;

pub(super) fn native_classic_player_motion_probe_evidence_json(probe_path: &str) -> String {
    const PANEL_WIDTH: usize = 160;
    const PANEL_HEIGHT: usize = 96;
    const COLUMNS: usize = 4;
    let assets = load_classic_runtime_assets();
    let motion_cases = [
        ("north_1", "north", 1_u8, "actor_player_walk_north_1"),
        ("north_2", "north", 2_u8, "actor_player_walk_north_2"),
        ("east_1", "east", 1_u8, "actor_player_walk_east_1"),
        ("east_2", "east", 2_u8, "actor_player_walk_east_2"),
        ("south_1", "south", 1_u8, "actor_player_walk_south_1"),
        ("south_2", "south", 2_u8, "actor_player_walk_south_2"),
        ("west_1", "west", 1_u8, "actor_player_walk_west_1"),
        ("west_2", "west", 2_u8, "actor_player_walk_west_2"),
    ];
    let rows = motion_cases.len().div_ceil(COLUMNS);
    let sheet_width = PANEL_WIDTH * COLUMNS;
    let sheet_height = PANEL_HEIGHT * rows;
    let mut sheet_pixels = vec![0x0b0d0c_u32; sheet_width * sheet_height];
    let mut sample_summaries = Vec::new();
    let mut selected_frame_ids = HashSet::new();
    let mut accepted_input_count = 0_usize;
    for (index, (case_id, direction, repeat_count, expected_frame_id)) in
        motion_cases.iter().enumerate()
    {
        let mut world = native_bevy_playable_fixture();
        let mut character = WorldTrillionniumCharacter::default_for("local-player");
        let mut gameplay_log = NativeGameplayLog::default();
        let mut runtime = NativeFirstPlayableRuntime::default();
        let mut accepted = false;
        for _ in 0..*repeat_count {
            let response = apply_live_native_action(
                &mut world,
                &mut character,
                &mut gameplay_log,
                &mut runtime,
                "local-player",
                NativeControlAction::Move {
                    direction: (*direction).to_string(),
                },
            );
            accepted =
                response.is_none() && gameplay_log.last_action == format!("local_move:{direction}");
        }
        if accepted {
            accepted_input_count += 1;
        }
        let selected_frame_id = classic_player_frame_id(&assets, &runtime);
        selected_frame_ids.insert(selected_frame_id.clone());
        let panel_x = ((index % COLUMNS) * PANEL_WIDTH) as i32;
        let panel_y = ((index / COLUMNS) * PANEL_HEIGHT) as i32;
        classic_draw_rect(
            &mut sheet_pixels,
            sheet_width,
            sheet_height,
            panel_x + 2,
            panel_y + 2,
            PANEL_WIDTH as i32 - 4,
            PANEL_HEIGHT as i32 - 4,
            0x121813,
        );
        classic_draw_rect(
            &mut sheet_pixels,
            sheet_width,
            sheet_height,
            panel_x + 50,
            panel_y + 14,
            76,
            72,
            CLASSIC_HUD_PANEL_COLOR,
        );
        classic_draw_frame_at_tile(
            &mut sheet_pixels,
            sheet_width,
            sheet_height,
            &assets,
            &selected_frame_id,
            panel_x + 62,
            panel_y + 20,
            32,
            (0, 0),
            4,
        );
        classic_draw_text(
            &mut sheet_pixels,
            sheet_width,
            sheet_height,
            panel_x + 8,
            panel_y + 8,
            &classic_catalog_text_label(case_id, 16),
            1,
            CLASSIC_HUD_TEXT_COLOR,
        );
        classic_draw_text(
            &mut sheet_pixels,
            sheet_width,
            sheet_height,
            panel_x + 8,
            panel_y + 78,
            &classic_catalog_text_label(&selected_frame_id, 22),
            1,
            CLASSIC_HUD_MUTED_TEXT_COLOR,
        );
        sample_summaries.push(json!({
            "case_id": case_id,
            "direction": direction,
            "repeat_count": repeat_count,
            "accepted_local_input": accepted,
            "walk_cycle_frame": runtime.walk_cycle_frame,
            "facing_direction": runtime.facing_direction,
            "player_sprite_pose": runtime.player_sprite_pose,
            "selected_frame_id": selected_frame_id,
            "expected_frame_id": expected_frame_id,
            "frame_match": selected_frame_id == *expected_frame_id,
            "last_action": gameplay_log.last_action,
            "last_result": gameplay_log.last_result,
        }));
    }
    let write_gate =
        write_classic_rgb_buffer_ppm(probe_path, sheet_width, sheet_height, &sheet_pixels).is_ok();
    let probe_bytes = fs::metadata(probe_path)
        .map(|metadata| metadata.len())
        .unwrap_or_default();
    let unique_color_count = sheet_pixels.iter().copied().collect::<HashSet<_>>().len();
    let non_background_pixels = sheet_pixels
        .iter()
        .filter(|color| **color != 0x0b0d0c_u32 && **color != 0x121813_u32)
        .count();
    let label_pixel_count = sheet_pixels
        .iter()
        .filter(|color| {
            **color == CLASSIC_HUD_TEXT_COLOR || **color == CLASSIC_HUD_MUTED_TEXT_COLOR
        })
        .count();
    let accepted_input_gate = accepted_input_count == motion_cases.len();
    let direction_coverage_gate = ["north", "east", "south", "west"].iter().all(|direction| {
        sample_summaries.iter().any(|sample| {
            sample.get("direction").and_then(|value| value.as_str()) == Some(*direction)
                && sample
                    .get("facing_direction")
                    .and_then(|value| value.as_str())
                    == Some(*direction)
        })
    });
    let frame_match_gate = sample_summaries.iter().all(|sample| {
        sample
            .get("frame_match")
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    });
    let manifest_frame_gate = selected_frame_ids
        .iter()
        .all(|frame_id| assets.frame_by_id.contains_key(frame_id));
    let sheet_gate =
        probe_bytes > 100_000 && unique_color_count >= 16 && non_background_pixels > 45_000;
    let label_gate = label_pixel_count > 800;
    let green = write_gate
        && assets.loaded_from_manifest
        && assets.atlas_parse_gate
        && accepted_input_gate
        && direction_coverage_gate
        && frame_match_gate
        && manifest_frame_gate
        && selected_frame_ids.len() == motion_cases.len()
        && sheet_gate
        && label_gate
        && !assets.manifest.cex_runtime_player_client_allowed
        && !assets.manifest.wgpu_required;
    serde_json::to_string_pretty(&json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYER_MOTION_PROBE_CONTRACT,
        "green": green,
        "probe_path": probe_path,
        "probe_format": "ppm_p3_rgb",
        "probe_width": sheet_width,
        "probe_height": sheet_height,
        "probe_bytes": probe_bytes,
        "sample_count": motion_cases.len(),
        "accepted_input_count": accepted_input_count,
        "selected_frame_ids": selected_frame_ids,
        "unique_color_count": unique_color_count,
        "non_background_pixels": non_background_pixels,
        "label_pixel_count": label_pixel_count,
        "loaded_from_manifest": assets.loaded_from_manifest,
        "atlas_parse_gate": assets.atlas_parse_gate,
        "accepted_input_gate": accepted_input_gate,
        "direction_coverage_gate": direction_coverage_gate,
        "frame_match_gate": frame_match_gate,
        "manifest_frame_gate": manifest_frame_gate,
        "sheet_gate": sheet_gate,
        "label_gate": label_gate,
        "samples": sample_summaries,
        "cex_runtime_player_client_allowed": assets.manifest.cex_runtime_player_client_allowed,
        "wgpu_required": assets.manifest.wgpu_required,
        "source_of_truth": "Classic player motion probe drives real NativeControlAction::Move inputs through apply_live_native_action, then proves runtime direction/walk-cycle state selects the expected low-spec player sprite frames."
    }))
    .expect("classic player motion probe evidence serializes")
}
