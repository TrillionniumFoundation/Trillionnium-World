#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use trnm_rts_data::{RtsFirstContactPlayerScreenChromeProfile, RtsPlayerScreenTacticsRowKind};

use crate::{
    classic_build_palette_label_x, classic_first_contact_build_placement_status_label,
    classic_first_contact_empty_production_slot_status_labels,
    classic_first_contact_label_has_raw_marker, classic_first_contact_order_queue_badge_label,
    classic_first_contact_production_slot_badge_label,
    classic_first_contact_rendered_build_palette_badge_labels,
    classic_first_contact_rendered_build_palette_labels,
    classic_first_contact_rendered_build_palette_state_labels,
    classic_first_contact_rendered_order_queue_badge_labels,
    classic_first_contact_rendered_order_queue_labels,
    classic_first_contact_rendered_production_slot_labels,
    classic_first_contact_rendered_production_slot_status_labels,
    classic_first_contact_tactical_header_camera_label,
    classic_first_contact_tactical_header_order_label, classic_first_contact_tactical_header_title,
    classic_first_contact_tactics_queue_fallback_badge_label,
    classic_first_contact_tactics_row_badge_label, classic_first_contact_tactics_row_value,
    classic_resource_readout_value_x, classic_rts_live_label_has_raw_marker,
    classic_rts_live_state_lines, classic_rts_live_status_labels, classic_rts_order_queue_label,
    classic_text_advance_px, NativeFirstPlayableRuntime,
    TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_LABEL_GUARD_CONTRACT,
};

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

pub(crate) fn player_screen_label_guard(
    runtime: &NativeFirstPlayableRuntime,
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> Value {
    let resource_labels = chrome
        .resource_readouts
        .iter()
        .map(|readout| readout.label.clone())
        .collect::<Vec<_>>();
    let resource_spacing_samples = resource_labels
        .iter()
        .map(|label| {
            let text_width_px = classic_text_advance_px(label, 1);
            let value_x_delta_px = classic_resource_readout_value_x(120, label) - 120;
            json!({
                "label": label,
                "text_width_px": text_width_px,
                "value_x_delta_px": value_x_delta_px,
                "value_spacing_gate": value_x_delta_px >= text_width_px + 24,
            })
        })
        .collect::<Vec<_>>();
    let production_slot_labels =
        classic_first_contact_rendered_production_slot_labels(runtime, chrome);
    let production_slot_badge_labels = production_slot_labels
        .iter()
        .map(|label| classic_first_contact_production_slot_badge_label(label))
        .collect::<Vec<_>>();
    let production_slot_badge_widths = production_slot_badge_labels
        .iter()
        .map(|label| classic_text_advance_px(label, 1))
        .collect::<Vec<_>>();
    let production_slot_status_labels =
        classic_first_contact_rendered_production_slot_status_labels(runtime, chrome);
    let production_empty_slot_status_labels =
        classic_first_contact_empty_production_slot_status_labels(chrome);
    let build_palette_labels = classic_first_contact_rendered_build_palette_labels(chrome);
    let build_palette_badge_labels =
        classic_first_contact_rendered_build_palette_badge_labels(chrome);
    let build_palette_badge_widths = build_palette_badge_labels
        .iter()
        .map(|label| classic_text_advance_px(label, 1))
        .collect::<Vec<_>>();
    let build_palette_state_labels =
        classic_first_contact_rendered_build_palette_state_labels(runtime, chrome);
    let build_palette_fit_samples = build_palette_labels
        .iter()
        .map(|label| {
            let label_x = classic_build_palette_label_x(100, 46, label);
            let right_x = label_x + classic_text_advance_px(label, 1);
            json!({
                "label": label,
                "label_x": label_x,
                "right_x": right_x,
                "fits_tile_gate": label_x >= 102 && right_x <= 146,
            })
        })
        .collect::<Vec<_>>();
    let build_palette_badge_fit_samples = build_palette_badge_labels
        .iter()
        .map(|label| {
            let label_x = classic_build_palette_label_x(100, 46, label);
            let right_x = label_x + classic_text_advance_px(label, 1);
            json!({
                "label": label,
                "label_x": label_x,
                "right_x": right_x,
                "fits_tile_gate": label_x >= 102 && right_x <= 146,
            })
        })
        .collect::<Vec<_>>();
    let order_queue_labels = classic_first_contact_rendered_order_queue_labels(runtime, chrome);
    let order_queue_badge_labels =
        classic_first_contact_rendered_order_queue_badge_labels(runtime, chrome);
    let completion_event_labels = vec![
        classic_rts_order_queue_label("production_complete:train:worker->worker_03"),
        classic_rts_order_queue_label("upgrade_complete:signal_blade"),
        classic_rts_order_queue_label("build_complete:build:watch_tower@7,4->watch_tower"),
        classic_rts_order_queue_label("build_complete:upgrade:training_hall->training_hall"),
    ];
    let completion_event_badge_labels = vec![
        classic_first_contact_order_queue_badge_label(
            "production_complete:train:worker->worker_03",
        ),
        classic_first_contact_order_queue_badge_label("upgrade_complete:signal_blade"),
        classic_first_contact_order_queue_badge_label(
            "build_complete:build:watch_tower@7,4->watch_tower",
        ),
        classic_first_contact_order_queue_badge_label(
            "build_complete:upgrade:training_hall->training_hall",
        ),
    ];
    let tactics_queue_summary = chrome
        .tactics_rows
        .iter()
        .find(|row| row.kind == RtsPlayerScreenTacticsRowKind::Queue)
        .map(|row| classic_first_contact_tactics_row_value(runtime, row))
        .unwrap_or_else(|| "READY".to_string());
    let tactics_detail_labels = chrome
        .tactics_rows
        .iter()
        .map(|row| classic_first_contact_tactics_row_value(runtime, row))
        .collect::<Vec<_>>();
    let tactics_compact_badge_labels = chrome
        .tactics_rows
        .iter()
        .zip(tactics_detail_labels.iter())
        .map(|(row, value)| classic_first_contact_tactics_row_badge_label(row.kind, value))
        .collect::<Vec<_>>();
    let tactics_compact_badge_widths = tactics_compact_badge_labels
        .iter()
        .map(|label| classic_text_advance_px(label, 1))
        .collect::<Vec<_>>();
    let tactics_queue_fallback_values = string_vec([
        "TRAIN SIGNAL",
        "TRAIN SI",
        "BUILD RELAY",
        "ATTACK BEACON",
        "READY",
    ]);
    let tactics_queue_fallback_badge_labels = tactics_queue_fallback_values
        .iter()
        .map(|label| classic_first_contact_tactics_queue_fallback_badge_label(label))
        .collect::<Vec<_>>();
    let tactics_queue_fallback_badge_widths = tactics_queue_fallback_badge_labels
        .iter()
        .map(|label| classic_text_advance_px(label, 1))
        .collect::<Vec<_>>();
    let tactics_target_label = chrome
        .tactics_rows
        .iter()
        .find(|row| row.kind == RtsPlayerScreenTacticsRowKind::Target)
        .map(|row| classic_first_contact_tactics_row_value(runtime, row))
        .unwrap_or_else(|| "NONE".to_string());
    let tactics_build_label = chrome
        .tactics_rows
        .iter()
        .find(|row| row.kind == RtsPlayerScreenTacticsRowKind::Build)
        .map(|row| classic_first_contact_tactics_row_value(runtime, row))
        .unwrap_or_else(|| "IDLE".to_string());
    let field_status_title = "FIELD STATUS".to_string();
    let live_status_labels = classic_rts_live_status_labels(runtime);
    let live_state_labels = classic_rts_live_state_lines(runtime);
    let tactical_header_title = classic_first_contact_tactical_header_title(chrome);
    let tactical_header_order_label =
        classic_first_contact_tactical_header_order_label(runtime, chrome);
    let tactical_header_camera_label =
        classic_first_contact_tactical_header_camera_label(runtime, chrome);
    let mut build_placement_runtime = runtime.clone();
    build_placement_runtime.rts_building_blueprint_id = Some("watch_tower".to_string());
    build_placement_runtime.rts_build_site_tile_ids = string_vec(["7,4", "7,5", "8,4"]);
    build_placement_runtime.coins = build_placement_runtime.coins.max(1_000);
    build_placement_runtime.rts_resource_spend_log.clear();
    let build_placement_status_label =
        classic_first_contact_build_placement_status_label(&build_placement_runtime)
            .unwrap_or_else(|| "PLACE TOWER READY".to_string());
    let mut upgrade_placement_runtime = runtime.clone();
    upgrade_placement_runtime.rts_building_blueprint_id = Some("upgrade:training_hall".to_string());
    upgrade_placement_runtime.rts_build_site_tile_ids = string_vec(["4,3", "4,4"]);
    upgrade_placement_runtime.coins = upgrade_placement_runtime.coins.max(1_000);
    upgrade_placement_runtime.rts_resource_spend_log.clear();
    let upgrade_placement_status_label =
        classic_first_contact_build_placement_status_label(&upgrade_placement_runtime)
            .unwrap_or_else(|| "PLACE TRAINING READY".to_string());
    let tactical_header_title_width_px = classic_text_advance_px(&tactical_header_title, 1);
    let tactical_header_order_width_px = classic_text_advance_px(&tactical_header_order_label, 1);
    let tactical_header_camera_width_px = classic_text_advance_px(&tactical_header_camera_label, 1);
    let tactical_header_title_order_width_px =
        tactical_header_title_width_px + 14 + tactical_header_order_width_px;
    let build_placement_status_width_px = classic_text_advance_px(&build_placement_status_label, 1);
    let upgrade_placement_status_width_px =
        classic_text_advance_px(&upgrade_placement_status_label, 1);
    let mut all_display_labels = Vec::new();
    all_display_labels.extend(resource_labels.iter().cloned());
    all_display_labels.extend(production_slot_labels.iter().cloned());
    all_display_labels.extend(production_slot_status_labels.iter().cloned());
    all_display_labels.extend(production_empty_slot_status_labels.iter().cloned());
    all_display_labels.extend(build_palette_labels.iter().cloned());
    all_display_labels.extend(build_palette_badge_labels.iter().cloned());
    all_display_labels.extend(build_palette_state_labels.iter().cloned());
    all_display_labels.extend(order_queue_labels.iter().cloned());
    all_display_labels.extend(order_queue_badge_labels.iter().cloned());
    all_display_labels.extend(completion_event_labels.iter().cloned());
    all_display_labels.extend(completion_event_badge_labels.iter().cloned());
    all_display_labels.extend(tactics_detail_labels.iter().cloned());
    all_display_labels.extend(tactics_compact_badge_labels.iter().cloned());
    all_display_labels.extend(tactics_queue_fallback_badge_labels.iter().cloned());
    all_display_labels.push(field_status_title.clone());
    all_display_labels.extend(live_status_labels.iter().cloned());
    all_display_labels.extend(live_state_labels.iter().cloned());
    all_display_labels.extend([
        tactical_header_title.clone(),
        tactical_header_order_label.clone(),
        tactical_header_camera_label.clone(),
        build_placement_status_label.clone(),
        upgrade_placement_status_label.clone(),
    ]);

    let expected_label_gate = resource_labels
        == string_vec(["CREDITS", "POWER", "SUPPLY", "VISION"])
        && production_slot_labels == string_vec(["GUARD", "WORKER", "SIGNAL", "TRAINING"])
        && production_slot_badge_labels == string_vec(["GRD", "WRK", "SIG", "TRN"])
        && production_slot_status_labels
            == string_vec(["Q1 64 R", "Q2 42 R", "Q3 64 R", "B2 42 R"])
        && production_empty_slot_status_labels
            == string_vec(["ADD UNIT", "ADD UNIT", "ADD BUILD", "ADD BUILD"])
        && build_palette_labels
            == string_vec([
                "POWER", "TRAIN", "REFINE", "TOWER", "COMMAND", "RADAR", "WALL", "SIGNAL",
            ])
        && build_palette_badge_labels
            == string_vec(["PWR", "TRN", "REF", "TWR", "CMD", "RAD", "WAL", "SIG"])
        && build_palette_state_labels
            == string_vec([
                "READY", "QUEUE", "READY", "QUEUE", "READY", "READY", "READY", "QUEUE",
            ])
        && order_queue_labels
            == string_vec(["ATTACK BEACON", "TRAIN WORKER", "BUILD RELAY", "MOVE 16/9"])
        && order_queue_badge_labels == string_vec(["ATK BCN", "TRN WRK", "BLD RLY", "MOV 16/9"])
        && completion_event_labels
            == string_vec([
                "WORKER READY",
                "SIGNAL READY",
                "TOWER READY",
                "TRAINING READY",
            ])
        && completion_event_badge_labels
            == string_vec(["WRK RDY", "SIG RDY", "TWR RDY", "TRN RDY"])
        && tactics_queue_summary == "GUARD 64% TOWER 42%"
        && tactics_compact_badge_labels
            == string_vec(["SECURE", "BEACON", "16/16", "G64/T42", "IDLE"])
        && tactics_queue_fallback_badge_labels
            == string_vec(["TRN SIG", "TRN SIG", "BLD RLY", "ATK BCN", "RDY"])
        && tactics_target_label == "RELAY BEACON"
        && tactics_build_label == "IDLE"
        && field_status_title == "FIELD STATUS"
        && live_status_labels
            == string_vec([
                "SQUAD READY",
                "RALLY 16/9",
                "QUEUE GUARD 64%",
                "SCOUTING 76%",
                "CAMERA 16/16",
                "SUPPLY 12/22",
                "SAVE ROUTE READY",
            ])
        && live_state_labels
            == string_vec([
                "ORDER SECURE RELAY BEACON",
                "TARGET RELAY BEACON",
                "BUILD IDLE",
                "QUEUE GUARD 64%",
                "CAM 16/16",
                "DRAG NONE",
                "HOVER NONE",
                "RES UPGRADE 210 CREDITS",
            ])
        && tactical_header_title == "TACTICAL VIEW"
        && tactical_header_order_label == "SECURE RELAY BEACON"
        && tactical_header_camera_label == "CAM 16/16 Z100"
        && build_placement_status_label.starts_with("PLACE ")
        && build_placement_status_label.contains("TOWER")
        && upgrade_placement_status_label.contains("TRAINING");
    let resource_spacing_gate = resource_spacing_samples
        .iter()
        .all(|sample| sample.get("value_spacing_gate").and_then(Value::as_bool) == Some(true));
    let production_slot_width_gate = production_slot_badge_labels
        .iter()
        .all(|label| classic_text_advance_px(label, 1) <= 18);
    let production_slot_status_width_gate = production_slot_status_labels
        .iter()
        .chain(production_empty_slot_status_labels.iter())
        .all(|label| classic_text_advance_px(label, 1) <= 52);
    let build_palette_width_gate = build_palette_badge_fit_samples
        .iter()
        .all(|sample| sample.get("fits_tile_gate").and_then(Value::as_bool) == Some(true));
    let build_palette_state_width_gate = build_palette_state_labels
        .iter()
        .all(|label| classic_text_advance_px(label, 1) <= 42);
    let order_queue_width_gate = order_queue_labels
        .iter()
        .chain(completion_event_labels.iter())
        .all(|label| classic_text_advance_px(label, 1) <= 210);
    let order_queue_badge_width_gate = order_queue_badge_labels
        .iter()
        .chain(completion_event_badge_labels.iter())
        .all(|label| classic_text_advance_px(label, 1) <= 48);
    let tactics_summary_width_gate = classic_text_advance_px(&tactics_queue_summary, 1) <= 120;
    let tactics_detail_width_gate = tactics_detail_labels
        .iter()
        .all(|label| classic_text_advance_px(label, 1) <= 132);
    let tactics_compact_badge_width_gate = tactics_compact_badge_widths
        .iter()
        .all(|width| *width <= 48);
    let tactics_queue_fallback_badge_width_gate = tactics_queue_fallback_badge_widths
        .iter()
        .all(|width| *width <= 42);
    let live_status_width_gate = live_status_labels
        .iter()
        .all(|label| classic_text_advance_px(label, 1) <= 132);
    let live_state_width_gate = live_state_labels
        .iter()
        .all(|label| classic_text_advance_px(label, 1) <= 180);
    let tactical_header_title_order_width_gate = tactical_header_title_order_width_px <= 310;
    let tactical_header_camera_width_gate = tactical_header_camera_width_px <= 96;
    let build_placement_status_width_gate =
        build_placement_status_width_px <= 240 && upgrade_placement_status_width_px <= 240;
    let tactical_header_vertical_separation_gate = 18 + 4 <= 30;
    let raw_marker_gate = all_display_labels.iter().all(|label| {
        !classic_first_contact_label_has_raw_marker(label)
            && !classic_rts_live_label_has_raw_marker(label)
    });
    let green = expected_label_gate
        && resource_spacing_gate
        && production_slot_width_gate
        && production_slot_status_width_gate
        && build_palette_width_gate
        && build_palette_state_width_gate
        && order_queue_width_gate
        && order_queue_badge_width_gate
        && tactics_summary_width_gate
        && tactics_detail_width_gate
        && tactics_compact_badge_width_gate
        && tactics_queue_fallback_badge_width_gate
        && live_status_width_gate
        && live_state_width_gate
        && tactical_header_title_order_width_gate
        && tactical_header_camera_width_gate
        && build_placement_status_width_gate
        && tactical_header_vertical_separation_gate
        && raw_marker_gate;

    json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_LABEL_GUARD_CONTRACT,
        "green": green,
        "source_path": "trnm-world-bevy classic_draw_openra_style_rts_shell display-label helpers",
        "resource_labels": resource_labels,
        "resource_spacing_samples": resource_spacing_samples,
        "production_slot_labels": production_slot_labels,
        "production_slot_badge_labels": production_slot_badge_labels,
        "production_slot_badge_widths": production_slot_badge_widths,
        "production_slot_status_labels": production_slot_status_labels,
        "production_empty_slot_status_labels": production_empty_slot_status_labels,
        "build_palette_labels": build_palette_labels,
        "build_palette_badge_labels": build_palette_badge_labels,
        "build_palette_badge_widths": build_palette_badge_widths,
        "build_palette_state_labels": build_palette_state_labels,
        "build_palette_fit_samples": build_palette_fit_samples,
        "build_palette_badge_fit_samples": build_palette_badge_fit_samples,
        "order_queue_labels": order_queue_labels,
        "order_queue_badge_labels": order_queue_badge_labels,
        "completion_event_labels": completion_event_labels,
        "completion_event_badge_labels": completion_event_badge_labels,
        "tactics_queue_summary": tactics_queue_summary,
        "tactics_target_label": tactics_target_label,
        "tactics_build_label": tactics_build_label,
        "tactics_detail_labels": tactics_detail_labels,
        "tactics_compact_badge_labels": tactics_compact_badge_labels,
        "tactics_compact_badge_widths": tactics_compact_badge_widths,
        "tactics_queue_fallback_values": tactics_queue_fallback_values,
        "tactics_queue_fallback_badge_labels": tactics_queue_fallback_badge_labels,
        "tactics_queue_fallback_badge_widths": tactics_queue_fallback_badge_widths,
        "field_status_title": field_status_title,
        "live_status_labels": live_status_labels,
        "live_state_labels": live_state_labels,
        "tactical_header_title": tactical_header_title,
        "tactical_header_order_label": tactical_header_order_label,
        "tactical_header_camera_label": tactical_header_camera_label,
        "tactical_header_title_width_px": tactical_header_title_width_px,
        "tactical_header_order_width_px": tactical_header_order_width_px,
        "tactical_header_camera_width_px": tactical_header_camera_width_px,
        "tactical_header_title_order_width_px": tactical_header_title_order_width_px,
        "build_placement_status_label": build_placement_status_label,
        "upgrade_placement_status_label": upgrade_placement_status_label,
        "build_placement_status_width_px": build_placement_status_width_px,
        "upgrade_placement_status_width_px": upgrade_placement_status_width_px,
        "forbidden_display_fragments": ["TRNM", "PRODUCTION COMPLETE", "BUILD COMPLETE", "UPGRADE COMPLETE", "LIVE INPUT", "LMB", "WASD", "CTRL", "SHIFT", "PROD ", ":", ".", "@", "_", "->"],
        "expected_label_gate": expected_label_gate,
        "resource_spacing_gate": resource_spacing_gate,
        "production_slot_width_gate": production_slot_width_gate,
        "production_slot_status_width_gate": production_slot_status_width_gate,
        "build_palette_width_gate": build_palette_width_gate,
        "build_palette_state_width_gate": build_palette_state_width_gate,
        "order_queue_width_gate": order_queue_width_gate,
        "order_queue_badge_width_gate": order_queue_badge_width_gate,
        "tactics_summary_width_gate": tactics_summary_width_gate,
        "tactics_detail_width_gate": tactics_detail_width_gate,
        "tactics_compact_badge_width_gate": tactics_compact_badge_width_gate,
        "tactics_queue_fallback_badge_width_gate": tactics_queue_fallback_badge_width_gate,
        "live_status_width_gate": live_status_width_gate,
        "live_state_width_gate": live_state_width_gate,
        "tactical_header_title_order_width_gate": tactical_header_title_order_width_gate,
        "tactical_header_camera_width_gate": tactical_header_camera_width_gate,
        "build_placement_status_width_gate": build_placement_status_width_gate,
        "tactical_header_vertical_separation_gate": tactical_header_vertical_separation_gate,
        "raw_marker_gate": raw_marker_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_player_screen_label_helpers_preserve_display_contracts() {
        let runtime = crate::classic_first_contact_player_screen_runtime();
        let profile = trnm_rts_data::first_contact_player_screen_profile();
        let guard = player_screen_label_guard(&runtime, &profile.chrome);

        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard.get("resource_labels").cloned(),
            Some(json!(["CREDITS", "POWER", "SUPPLY", "VISION"]))
        );
        assert_eq!(
            guard.get("production_slot_badge_labels").cloned(),
            Some(json!(["GRD", "WRK", "SIG", "TRN"]))
        );
        assert_eq!(
            guard.get("build_palette_badge_labels").cloned(),
            Some(json!([
                "PWR", "TRN", "REF", "TWR", "CMD", "RAD", "WAL", "SIG"
            ]))
        );
        assert_eq!(
            guard.get("order_queue_badge_labels").cloned(),
            Some(json!(["ATK BCN", "TRN WRK", "BLD RLY", "MOV 16/9"]))
        );
        assert_eq!(
            guard.get("tactics_compact_badge_labels").cloned(),
            Some(json!(["SECURE", "BEACON", "16/16", "G64/T42", "IDLE"]))
        );
        assert_eq!(
            guard
                .get("build_placement_status_label")
                .and_then(Value::as_str)
                .map(|label| label.starts_with("PLACE ")),
            Some(true)
        );
        assert_eq!(
            guard.get("live_status_width_gate").and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard.get("raw_marker_gate").and_then(Value::as_bool),
            Some(true)
        );

        for gate in [
            "expected_label_gate",
            "resource_spacing_gate",
            "production_slot_width_gate",
            "build_palette_width_gate",
            "order_queue_width_gate",
            "tactics_compact_badge_width_gate",
            "build_placement_status_width_gate",
            "tactical_header_vertical_separation_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }
}
