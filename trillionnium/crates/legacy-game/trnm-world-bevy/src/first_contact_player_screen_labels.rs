#![cfg(not(target_os = "android"))]

use serde_json::Value;
use trnm_rts_data::{RtsFirstContactPlayerScreenChromeProfile, RtsPlayerScreenTacticsRowKind};
use trnm_rts_evidence::{
    RtsFirstContactBuildPaletteFitSnapshot, RtsFirstContactPlayerScreenLabelGeometrySnapshot,
    RtsFirstContactPlayerScreenLabelRuntime, RtsFirstContactResourceSpacingSnapshot,
};

use crate::{
    classic_build_palette_label_x, classic_first_contact_build_placement_status_label,
    classic_first_contact_empty_production_slot_status_labels,
    classic_first_contact_order_queue_badge_label,
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
    classic_resource_readout_value_x, classic_rts_live_state_lines, classic_rts_live_status_labels,
    classic_rts_order_queue_label, classic_text_advance_px, NativeFirstPlayableRuntime,
};

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn label_widths(labels: &[String]) -> Vec<i32> {
    labels
        .iter()
        .map(|label| classic_text_advance_px(label, 1))
        .collect()
}

fn build_palette_fit_snapshots(labels: &[String]) -> Vec<RtsFirstContactBuildPaletteFitSnapshot> {
    labels
        .iter()
        .map(|label| {
            let label_x = classic_build_palette_label_x(100, 46, label);
            RtsFirstContactBuildPaletteFitSnapshot {
                label: label.clone(),
                label_x,
                right_x: label_x + classic_text_advance_px(label, 1),
            }
        })
        .collect()
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
            RtsFirstContactResourceSpacingSnapshot {
                label: label.clone(),
                text_width_px,
                value_x_delta_px,
            }
        })
        .collect::<Vec<_>>();
    let production_slot_labels =
        classic_first_contact_rendered_production_slot_labels(runtime, chrome);
    let production_slot_badge_labels = production_slot_labels
        .iter()
        .map(|label| classic_first_contact_production_slot_badge_label(label))
        .collect::<Vec<_>>();
    let production_slot_badge_widths = label_widths(&production_slot_badge_labels);
    let production_slot_status_labels =
        classic_first_contact_rendered_production_slot_status_labels(runtime, chrome);
    let production_slot_status_widths = label_widths(&production_slot_status_labels);
    let production_empty_slot_status_labels =
        classic_first_contact_empty_production_slot_status_labels(chrome);
    let production_empty_slot_status_widths = label_widths(&production_empty_slot_status_labels);
    let build_palette_labels = classic_first_contact_rendered_build_palette_labels(chrome);
    let build_palette_badge_labels =
        classic_first_contact_rendered_build_palette_badge_labels(chrome);
    let build_palette_badge_widths = label_widths(&build_palette_badge_labels);
    let build_palette_state_labels =
        classic_first_contact_rendered_build_palette_state_labels(runtime, chrome);
    let build_palette_state_widths = label_widths(&build_palette_state_labels);
    let build_palette_fit_samples = build_palette_fit_snapshots(&build_palette_labels);
    let build_palette_badge_fit_samples = build_palette_fit_snapshots(&build_palette_badge_labels);
    let order_queue_labels = classic_first_contact_rendered_order_queue_labels(runtime, chrome);
    let order_queue_label_widths = label_widths(&order_queue_labels);
    let order_queue_badge_labels =
        classic_first_contact_rendered_order_queue_badge_labels(runtime, chrome);
    let order_queue_badge_widths = label_widths(&order_queue_badge_labels);
    let completion_event_labels = vec![
        classic_rts_order_queue_label("production_complete:train:worker->worker_03"),
        classic_rts_order_queue_label("upgrade_complete:signal_blade"),
        classic_rts_order_queue_label("build_complete:build:watch_tower@7,4->watch_tower"),
        classic_rts_order_queue_label("build_complete:upgrade:training_hall->training_hall"),
    ];
    let completion_event_label_widths = label_widths(&completion_event_labels);
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
    let completion_event_badge_widths = label_widths(&completion_event_badge_labels);
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
    let tactics_detail_widths = label_widths(&tactics_detail_labels);
    let tactics_compact_badge_widths = label_widths(&tactics_compact_badge_labels);
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
    let tactics_queue_fallback_badge_widths = label_widths(&tactics_queue_fallback_badge_labels);
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
    let live_status_widths = label_widths(&live_status_labels);
    let live_state_labels = classic_rts_live_state_lines(runtime);
    let live_state_widths = label_widths(&live_state_labels);
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
    let label_runtime = RtsFirstContactPlayerScreenLabelRuntime {
        resource_labels,
        resource_spacing_samples,
        production_slot_labels,
        production_slot_badge_labels,
        production_slot_badge_widths_px: production_slot_badge_widths,
        production_slot_status_labels,
        production_slot_status_widths_px: production_slot_status_widths,
        production_empty_slot_status_labels,
        production_empty_slot_status_widths_px: production_empty_slot_status_widths,
        build_palette_labels,
        build_palette_badge_labels,
        build_palette_badge_widths_px: build_palette_badge_widths,
        build_palette_state_labels,
        build_palette_state_widths_px: build_palette_state_widths,
        build_palette_fit_samples,
        build_palette_badge_fit_samples,
        order_queue_labels,
        order_queue_label_widths_px: order_queue_label_widths,
        order_queue_badge_labels,
        order_queue_badge_widths_px: order_queue_badge_widths,
        completion_event_labels,
        completion_event_label_widths_px: completion_event_label_widths,
        completion_event_badge_labels,
        completion_event_badge_widths_px: completion_event_badge_widths,
        tactics_queue_summary_width_px: classic_text_advance_px(&tactics_queue_summary, 1),
        tactics_queue_summary,
        tactics_target_label,
        tactics_build_label,
        tactics_detail_labels,
        tactics_detail_widths_px: tactics_detail_widths,
        tactics_compact_badge_labels,
        tactics_compact_badge_widths_px: tactics_compact_badge_widths,
        tactics_queue_fallback_values,
        tactics_queue_fallback_badge_labels,
        tactics_queue_fallback_badge_widths_px: tactics_queue_fallback_badge_widths,
        field_status_title,
        live_status_labels,
        live_status_widths_px: live_status_widths,
        live_state_labels,
        live_state_widths_px: live_state_widths,
        tactical_header_title,
        tactical_header_order_label,
        tactical_header_camera_label,
        tactical_header_title_width_px,
        tactical_header_order_width_px,
        tactical_header_camera_width_px,
        tactical_header_title_order_width_px,
        build_placement_status_label,
        upgrade_placement_status_label,
        build_placement_status_width_px,
        upgrade_placement_status_width_px,
        geometry: RtsFirstContactPlayerScreenLabelGeometrySnapshot {
            tactical_header_title_y_offset_px: 18,
            tactical_header_label_gap_px: 4,
            tactical_header_status_y_offset_px: 30,
        },
    };
    trnm_rts_evidence::first_contact_player_screen_label_guard(&label_runtime)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
