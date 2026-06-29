use super::*;
use serde_json::json;
use std::collections::HashSet;
use std::time::Instant;

pub(super) fn native_classic_input_frame_budget_evidence_json() -> String {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;
    const SAMPLE_COUNT: usize = 96;
    const GRID_COLS: i32 = 12;
    const GRID_ROWS: i32 = 8;
    let assets = load_classic_runtime_assets();
    let mut world = native_bevy_playable_fixture();
    let mut character = WorldTrillionniumCharacter::default_for("local-player");
    let mut gameplay_log = NativeGameplayLog::default();
    let mut runtime = NativeFirstPlayableRuntime::default();
    let mut player_tile = (5_i32, 4_i32);
    let mut buffer = vec![0_u32; WIDTH * HEIGHT];
    let directions = ["north", "east", "south", "west"];
    let mut frame_micros = Vec::with_capacity(SAMPLE_COUNT);
    let mut accepted_input_count = 0_usize;
    let mut accepted_directions = HashSet::new();
    let mut selected_frame_ids = HashSet::new();
    let mut nonblank_samples = Vec::new();
    let mut sample_summaries = Vec::new();

    for sample_index in 0..SAMPLE_COUNT {
        let direction = directions[sample_index % directions.len()];
        let started = Instant::now();
        let response = apply_live_native_action(
            &mut world,
            &mut character,
            &mut gameplay_log,
            &mut runtime,
            "local-player",
            NativeControlAction::Move {
                direction: direction.to_string(),
            },
        );
        let accepted = response.is_none()
            && gameplay_log.last_action == format!("local_move:{direction}")
            && runtime.facing_direction == direction;
        if accepted {
            accepted_input_count += 1;
            accepted_directions.insert(direction.to_string());
            classic_move_player(&mut player_tile, direction, GRID_COLS, GRID_ROWS);
        }
        classic_draw_scene(&mut buffer, WIDTH, HEIGHT, player_tile, &runtime, &assets);
        let elapsed_micros = started.elapsed().as_micros() as u64;
        frame_micros.push(elapsed_micros);
        let selected_frame_id = classic_player_frame_id(&assets, &runtime);
        selected_frame_ids.insert(selected_frame_id.clone());
        if sample_index % 12 == 0 {
            let nonblank_pixels = buffer
                .iter()
                .filter(|color| **color != 0x101411_u32 && **color != 0x171a1d_u32)
                .count();
            nonblank_samples.push(nonblank_pixels);
            sample_summaries.push(json!({
                "sample_index": sample_index,
                "direction": direction,
                "accepted": accepted,
                "elapsed_micros": elapsed_micros,
                "player_tile": {"x": player_tile.0, "y": player_tile.1},
                "walk_cycle_frame": runtime.walk_cycle_frame,
                "selected_frame_id": selected_frame_id,
                "last_action": gameplay_log.last_action,
                "last_result": gameplay_log.last_result,
                "nonblank_pixels": nonblank_pixels,
            }));
        }
    }

    let mut sorted = frame_micros.clone();
    sorted.sort_unstable();
    let percentile = |values: &[u64], numerator: usize, denominator: usize| -> u64 {
        if values.is_empty() {
            return 0;
        }
        let index = ((values.len() - 1) * numerator) / denominator.max(1);
        values[index]
    };
    let p50_micros = percentile(&sorted, 50, 100);
    let p95_micros = percentile(&sorted, 95, 100);
    let max_micros = sorted.last().copied().unwrap_or_default();
    let avg_micros = if frame_micros.is_empty() {
        0
    } else {
        frame_micros.iter().sum::<u64>() / frame_micros.len() as u64
    };
    let p95_budget_micros = 20_000_u64;
    let max_budget_micros = 50_000_u64;
    let accepted_input_gate = accepted_input_count == SAMPLE_COUNT;
    let direction_coverage_gate = directions
        .iter()
        .all(|direction| accepted_directions.contains(*direction));
    let response_p95_budget_gate = p95_micros <= p95_budget_micros;
    let response_max_budget_gate = max_micros <= max_budget_micros;
    let rendered_frame_gate = nonblank_samples.iter().all(|count| *count > 80_000);
    let selected_frame_manifest_gate = selected_frame_ids
        .iter()
        .all(|frame_id| assets.frame_by_id.contains_key(frame_id));
    let green = assets.loaded_from_manifest
        && assets.atlas_parse_gate
        && accepted_input_gate
        && direction_coverage_gate
        && response_p95_budget_gate
        && response_max_budget_gate
        && rendered_frame_gate
        && selected_frame_manifest_gate
        && !assets.manifest.cex_runtime_player_client_allowed
        && !assets.manifest.wgpu_required;
    serde_json::to_string_pretty(&json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_INPUT_FRAME_BUDGET_CONTRACT,
        "green": green,
        "renderer_path": "classic_cpu_ppm_minifb_low_spec",
        "input_path": "NativeControlAction::Move -> apply_live_native_action -> classic_draw_scene",
        "frame_width": WIDTH,
        "frame_height": HEIGHT,
        "sample_count": SAMPLE_COUNT,
        "accepted_input_count": accepted_input_count,
        "accepted_directions": accepted_directions,
        "selected_frame_ids": selected_frame_ids,
        "p50_micros": p50_micros,
        "p95_micros": p95_micros,
        "max_micros": max_micros,
        "avg_micros": avg_micros,
        "p95_budget_micros": p95_budget_micros,
        "max_budget_micros": max_budget_micros,
        "nonblank_samples": nonblank_samples,
        "samples": sample_summaries,
        "loaded_from_manifest": assets.loaded_from_manifest,
        "atlas_parse_gate": assets.atlas_parse_gate,
        "accepted_input_gate": accepted_input_gate,
        "direction_coverage_gate": direction_coverage_gate,
        "response_p95_budget_gate": response_p95_budget_gate,
        "response_max_budget_gate": response_max_budget_gate,
        "rendered_frame_gate": rendered_frame_gate,
        "selected_frame_manifest_gate": selected_frame_manifest_gate,
        "cex_runtime_player_client_allowed": assets.manifest.cex_runtime_player_client_allowed,
        "wgpu_required": assets.manifest.wgpu_required,
        "source_of_truth": "Classic input-frame budget measures accepted movement input through apply_live_native_action plus the next low-spec classic_draw_scene frame, protecting keyboard responsiveness on the Bevy client path."
    }))
    .expect("classic input-frame budget evidence serializes")
}

pub(super) fn native_classic_render_budget_evidence_json() -> String {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;
    const FRAME_COUNT: usize = 180;
    let assets = load_classic_runtime_assets();
    let mut buffer = vec![0_u32; WIDTH * HEIGHT];
    let mut frame_micros = Vec::with_capacity(FRAME_COUNT);
    let mut nonblank_samples = Vec::new();
    let directions = ["south", "east", "north", "west"];
    let scenes = [
        "mirror_city_square",
        "mentor_training_room",
        "league_arena",
        "mirror_city_square",
    ];
    for frame_index in 0..FRAME_COUNT {
        let mut runtime = classic_preview_runtime(
            directions[frame_index % directions.len()],
            (frame_index % 4) as u8,
            scenes[(frame_index / 15) % scenes.len()],
        );
        runtime.xp = (frame_index % 100) as u64;
        runtime.coins = (frame_index % 37) as u64;
        if frame_index % 24 >= 12 {
            runtime.dialogue_overlay_visible = true;
            runtime.npc_dialogue_state = "mentor_talk_budget_probe".to_string();
        }
        if runtime.map_scene.contains("arena") {
            runtime.combat_overlay_visible = true;
            runtime.combat_overlay_was_visible = true;
            runtime.combat_turn = (frame_index % 5) as u8;
            runtime.enemy_damage_feedback = if frame_index % 2 == 0 {
                "turn budget attack -14 HP".to_string()
            } else {
                "force route clear: duelist yields".to_string()
            };
            runtime.enemy_hp = if frame_index % 2 == 0 { 25 } else { 0 };
        }
        let player_tile = (
            4 + (frame_index as i32 % 4),
            3 + ((frame_index / 4) as i32 % 3),
        );
        let started = Instant::now();
        classic_draw_scene(&mut buffer, WIDTH, HEIGHT, player_tile, &runtime, &assets);
        frame_micros.push(started.elapsed().as_micros() as u64);
        if frame_index % 45 == 0 {
            nonblank_samples.push(
                buffer
                    .iter()
                    .filter(|color| **color != 0x101411_u32 && **color != 0x171a1d_u32)
                    .count(),
            );
        }
    }
    let mut sorted = frame_micros.clone();
    sorted.sort_unstable();
    let percentile = |values: &[u64], numerator: usize, denominator: usize| -> u64 {
        if values.is_empty() {
            return 0;
        }
        let index = ((values.len() - 1) * numerator) / denominator.max(1);
        values[index]
    };
    let p50_micros = percentile(&sorted, 50, 100);
    let p95_micros = percentile(&sorted, 95, 100);
    let max_micros = sorted.last().copied().unwrap_or_default();
    let avg_micros = if frame_micros.is_empty() {
        0
    } else {
        frame_micros.iter().sum::<u64>() / frame_micros.len() as u64
    };
    let p95_budget_micros = 16_000_u64;
    let max_budget_micros = 40_000_u64;
    let p95_budget_gate = p95_micros <= p95_budget_micros;
    let max_budget_gate = max_micros <= max_budget_micros;
    let frame_count_gate = frame_micros.len() == FRAME_COUNT;
    let nonblank_gate = nonblank_samples.iter().all(|count| *count > 80_000);
    let green = assets.loaded_from_manifest
        && assets.atlas_parse_gate
        && frame_count_gate
        && p95_budget_gate
        && max_budget_gate
        && nonblank_gate
        && !assets.manifest.cex_runtime_player_client_allowed
        && !assets.manifest.wgpu_required;
    serde_json::to_string_pretty(&json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_RENDER_BUDGET_CONTRACT,
        "green": green,
        "renderer_path": "classic_cpu_ppm_minifb_low_spec",
        "frame_width": WIDTH,
        "frame_height": HEIGHT,
        "frame_count": frame_micros.len(),
        "p50_micros": p50_micros,
        "p95_micros": p95_micros,
        "max_micros": max_micros,
        "avg_micros": avg_micros,
        "p95_budget_micros": p95_budget_micros,
        "max_budget_micros": max_budget_micros,
        "nonblank_samples": nonblank_samples,
        "loaded_from_manifest": assets.loaded_from_manifest,
        "atlas_parse_gate": assets.atlas_parse_gate,
        "frame_count_gate": frame_count_gate,
        "p95_budget_gate": p95_budget_gate,
        "max_budget_gate": max_budget_gate,
        "nonblank_gate": nonblank_gate,
        "cex_runtime_player_client_allowed": assets.manifest.cex_runtime_player_client_allowed,
        "wgpu_required": assets.manifest.wgpu_required,
        "source_of_truth": "Classic render budget measures repeated low-spec classic_draw_scene CPU frames without Bevy/wgpu, protecting the X230 playtest path from renderer regressions."
    }))
    .expect("classic render budget evidence serializes")
}
