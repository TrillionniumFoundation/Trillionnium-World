use super::*;
use serde_json::json;
use std::collections::HashSet;
use std::fs;

pub(super) fn native_classic_animation_preview_evidence_json(preview_path: &str) -> String {
    const WIDTH: usize = 640;
    const ROW_HEIGHT: usize = 112;
    const SPRITE_SCALE: u32 = 4;
    const FRAME_CELL_WIDTH: i32 = 72;
    let assets = load_classic_runtime_assets();
    let clips = assets
        .manifest
        .actors
        .iter()
        .flat_map(|actor| actor.animation_clips.iter().map(move |clip| (actor, clip)))
        .collect::<Vec<_>>();
    let sheet_height = ROW_HEIGHT * clips.len().max(1);
    let mut pixels = vec![0x0b0d0c_u32; WIDTH * sheet_height];
    let mut clip_summaries = Vec::new();
    let mut rendered_clip_count = 0_usize;
    let mut rendered_frame_slot_count = 0_usize;
    let mut action_set = HashSet::new();
    let mut all_clip_refs_valid = true;
    for (row, (actor, clip)) in clips.iter().enumerate() {
        let row_y = (row * ROW_HEIGHT) as i32;
        action_set.insert(clip.action.clone());
        classic_draw_rect(
            &mut pixels,
            WIDTH,
            sheet_height,
            0,
            row_y,
            WIDTH as i32,
            ROW_HEIGHT as i32 - 2,
            0x121813,
        );
        classic_draw_rect(
            &mut pixels,
            WIDTH,
            sheet_height,
            0,
            row_y,
            WIDTH as i32,
            2,
            0x33483b,
        );
        classic_draw_text(
            &mut pixels,
            WIDTH,
            sheet_height,
            12,
            row_y + 8,
            &classic_catalog_text_label(
                &format!("{} {} FPS {}", actor.id, clip.action, clip.fps),
                34,
            ),
            1,
            CLASSIC_HUD_TEXT_COLOR,
        );
        classic_draw_text(
            &mut pixels,
            WIDTH,
            sheet_height,
            12,
            row_y + 22,
            &classic_catalog_text_label(&clip.id, 36),
            1,
            CLASSIC_HUD_MUTED_TEXT_COLOR,
        );
        let mut clip_refs_valid = true;
        let mut clip_visible_pixels = 0_usize;
        for (index, frame_id) in clip.frame_ids.iter().enumerate() {
            let frame_x = 28 + index as i32 * FRAME_CELL_WIDTH;
            let frame_y = row_y + 40;
            classic_draw_rect(
                &mut pixels,
                WIDTH,
                sheet_height,
                frame_x - 4,
                frame_y - 4,
                68,
                72,
                CLASSIC_HUD_PANEL_COLOR,
            );
            let drawn = classic_blit_frame_scaled(
                &mut pixels,
                WIDTH,
                sheet_height,
                &assets,
                frame_id,
                frame_x,
                frame_y,
                SPRITE_SCALE,
            );
            if drawn {
                rendered_frame_slot_count += 1;
                clip_visible_pixels += classic_frame_visible_pixel_count(&assets, frame_id);
            } else {
                clip_refs_valid = false;
                all_clip_refs_valid = false;
            }
            classic_draw_text(
                &mut pixels,
                WIDTH,
                sheet_height,
                frame_x,
                row_y + 92,
                &format!("{}", index + 1),
                1,
                CLASSIC_HUD_ACCENT_TEXT_COLOR,
            );
        }
        if clip_refs_valid && clip.frame_ids.len() >= 2 && clip_visible_pixels > 24 {
            rendered_clip_count += 1;
        }
        clip_summaries.push(json!({
            "actor_id": actor.id,
            "clip_id": clip.id,
            "action": clip.action,
            "fps": clip.fps,
            "frame_count": clip.frame_ids.len(),
            "frame_ids": clip.frame_ids,
            "refs_valid": clip_refs_valid,
            "visible_pixels": clip_visible_pixels,
        }));
    }
    let write_gate =
        write_classic_rgb_buffer_ppm(preview_path, WIDTH, sheet_height, &pixels).is_ok();
    let preview_bytes = fs::metadata(preview_path)
        .map(|metadata| metadata.len())
        .unwrap_or_default();
    let unique_color_count = pixels.iter().copied().collect::<HashSet<_>>().len();
    let non_background_pixels = pixels
        .iter()
        .filter(|color| **color != 0x0b0d0c_u32 && **color != 0x121813_u32)
        .count();
    let label_pixel_count = pixels
        .iter()
        .filter(|color| {
            **color == CLASSIC_HUD_TEXT_COLOR
                || **color == CLASSIC_HUD_MUTED_TEXT_COLOR
                || **color == CLASSIC_HUD_ACCENT_TEXT_COLOR
        })
        .count();
    let clip_count_gate = clips.len() >= 4;
    let action_coverage_gate = ["walk", "talk", "attack", "hit"]
        .iter()
        .all(|action| action_set.contains(*action));
    let fps_gate = clips.iter().all(|(_, clip)| (4..=12).contains(&clip.fps));
    let rendered_clip_gate = rendered_clip_count == clips.len() && rendered_frame_slot_count >= 15;
    let preview_sheet_gate =
        preview_bytes > 100_000 && unique_color_count >= 32 && non_background_pixels > 35_000;
    let label_gate = label_pixel_count > 2_000;
    let green = write_gate
        && assets.loaded_from_manifest
        && assets.atlas_parse_gate
        && clip_count_gate
        && action_coverage_gate
        && fps_gate
        && all_clip_refs_valid
        && rendered_clip_gate
        && preview_sheet_gate
        && label_gate
        && !assets.manifest.cex_runtime_player_client_allowed
        && !assets.manifest.wgpu_required;
    serde_json::to_string_pretty(&json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_ANIMATION_PREVIEW_CONTRACT,
        "green": green,
        "preview_path": preview_path,
        "preview_format": "ppm_p3_rgb",
        "preview_width": WIDTH,
        "preview_height": sheet_height,
        "preview_bytes": preview_bytes,
        "clip_count": clips.len(),
        "rendered_clip_count": rendered_clip_count,
        "rendered_frame_slot_count": rendered_frame_slot_count,
        "unique_color_count": unique_color_count,
        "non_background_pixels": non_background_pixels,
        "label_pixel_count": label_pixel_count,
        "loaded_from_manifest": assets.loaded_from_manifest,
        "atlas_parse_gate": assets.atlas_parse_gate,
        "clip_count_gate": clip_count_gate,
        "action_coverage_gate": action_coverage_gate,
        "fps_gate": fps_gate,
        "all_clip_refs_valid": all_clip_refs_valid,
        "rendered_clip_gate": rendered_clip_gate,
        "preview_sheet_gate": preview_sheet_gate,
        "label_gate": label_gate,
        "clip_summaries": clip_summaries,
        "cex_runtime_player_client_allowed": assets.manifest.cex_runtime_player_client_allowed,
        "wgpu_required": assets.manifest.wgpu_required,
        "source_of_truth": "Classic animation preview expands manifest actor clips into visible sprite strips through the same PPM atlas blitter used by the low-spec playtest renderer."
    }))
    .expect("classic animation preview evidence serializes")
}

pub(super) fn native_classic_animation_selector_evidence_json() -> String {
    let assets = load_classic_runtime_assets();
    let mentor = ClassicSceneLandmark {
        id: "mentor".to_string(),
        frame_id: "actor_mentor_idle".to_string(),
        tile_x: 4,
        tile_y: 3,
    };
    let enemy = ClassicSceneLandmark {
        id: "enemy".to_string(),
        frame_id: "actor_enemy_idle".to_string(),
        tile_x: 9,
        tile_y: 2,
    };
    let objective = ClassicSceneLandmark {
        id: "objective_gate".to_string(),
        frame_id: "marker_objective".to_string(),
        tile_x: 8,
        tile_y: 2,
    };
    let dialogue_runtime = NativeFirstPlayableRuntime {
        dialogue_overlay_visible: true,
        npc_dialogue_state: "mentor_talk_preview".to_string(),
        ..Default::default()
    };
    let combat_attack_runtime = NativeFirstPlayableRuntime {
        combat_overlay_visible: true,
        combat_overlay_was_visible: true,
        combat_turn: 1,
        enemy_hp: 25,
        enemy_damage_feedback: "turn 1 attack -14 HP".to_string(),
        ..Default::default()
    };
    let combat_hit_runtime = NativeFirstPlayableRuntime {
        combat_overlay_visible: true,
        combat_overlay_was_visible: true,
        combat_turn: 2,
        enemy_hp: 0,
        enemy_damage_feedback: "force route clear: duelist yields".to_string(),
        ..Default::default()
    };
    let marker_pulse_runtime = NativeFirstPlayableRuntime {
        walk_cycle_frame: 1,
        ..Default::default()
    };
    let cases = vec![
        json!({
            "case_id": "mentor_idle",
            "landmark_id": mentor.id.as_str(),
            "selected_frame_id": classic_dynamic_landmark_frame_id(&mentor, &NativeFirstPlayableRuntime::default()),
            "expected_frame_id": "actor_mentor_idle",
        }),
        json!({
            "case_id": "mentor_dialogue_talk",
            "landmark_id": mentor.id.as_str(),
            "selected_frame_id": classic_dynamic_landmark_frame_id(&mentor, &dialogue_runtime),
            "expected_frame_id": "actor_mentor_talk",
        }),
        json!({
            "case_id": "enemy_idle",
            "landmark_id": enemy.id.as_str(),
            "selected_frame_id": classic_dynamic_landmark_frame_id(&enemy, &NativeFirstPlayableRuntime::default()),
            "expected_frame_id": "actor_enemy_idle",
        }),
        json!({
            "case_id": "enemy_combat_attack",
            "landmark_id": enemy.id.as_str(),
            "selected_frame_id": classic_dynamic_landmark_frame_id(&enemy, &combat_attack_runtime),
            "expected_frame_id": "actor_enemy_attack",
        }),
        json!({
            "case_id": "enemy_combat_hit",
            "landmark_id": enemy.id.as_str(),
            "selected_frame_id": classic_dynamic_landmark_frame_id(&enemy, &combat_hit_runtime),
            "expected_frame_id": "actor_enemy_hit",
        }),
        json!({
            "case_id": "objective_marker_pulse",
            "landmark_id": objective.id.as_str(),
            "selected_frame_id": classic_dynamic_landmark_frame_id(&objective, &marker_pulse_runtime),
            "expected_frame_id": "marker_interaction",
        }),
    ];
    let selector_case_gate = cases.iter().all(|case| {
        case.get("selected_frame_id")
            .and_then(|value| value.as_str())
            == case
                .get("expected_frame_id")
                .and_then(|value| value.as_str())
    });
    let selected_frames = cases
        .iter()
        .filter_map(|case| {
            case.get("selected_frame_id")
                .and_then(|value| value.as_str())
                .map(str::to_string)
        })
        .collect::<HashSet<_>>();
    let selected_frame_manifest_gate = selected_frames
        .iter()
        .all(|frame_id| assets.frame_by_id.contains_key(frame_id));
    let animation_transition_gate = selected_frames.contains("actor_mentor_talk")
        && selected_frames.contains("actor_enemy_attack")
        && selected_frames.contains("actor_enemy_hit")
        && selected_frames.contains("marker_interaction");
    let green = assets.loaded_from_manifest
        && assets.atlas_parse_gate
        && cases.len() >= 6
        && selector_case_gate
        && selected_frame_manifest_gate
        && animation_transition_gate
        && !assets.manifest.cex_runtime_player_client_allowed
        && !assets.manifest.wgpu_required;
    serde_json::to_string_pretty(&json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_ANIMATION_SELECTOR_CONTRACT,
        "green": green,
        "case_count": cases.len(),
        "cases": cases,
        "selected_frames": selected_frames,
        "loaded_from_manifest": assets.loaded_from_manifest,
        "atlas_parse_gate": assets.atlas_parse_gate,
        "selector_case_gate": selector_case_gate,
        "selected_frame_manifest_gate": selected_frame_manifest_gate,
        "animation_transition_gate": animation_transition_gate,
        "cex_runtime_player_client_allowed": assets.manifest.cex_runtime_player_client_allowed,
        "wgpu_required": assets.manifest.wgpu_required,
        "source_of_truth": "Classic animation selector evidence locks runtime state-to-frame decisions for dialogue, combat, damage, and marker pulse inside trnm-world-bevy."
    }))
    .expect("classic animation selector evidence serializes")
}
