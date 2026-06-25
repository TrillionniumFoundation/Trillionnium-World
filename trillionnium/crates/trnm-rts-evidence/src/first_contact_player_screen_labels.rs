#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};

use crate::TRNM_RTS_EVIDENCE_FIRST_CONTACT_LABEL_GUARD_CONTRACT;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RtsFirstContactResourceSpacingSnapshot {
    pub label: String,
    pub text_width_px: i32,
    pub value_x_delta_px: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RtsFirstContactBuildPaletteFitSnapshot {
    pub label: String,
    pub label_x: i32,
    pub right_x: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RtsFirstContactPlayerScreenLabelGeometrySnapshot {
    pub tactical_header_title_y_offset_px: i32,
    pub tactical_header_label_gap_px: i32,
    pub tactical_header_status_y_offset_px: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RtsFirstContactPlayerScreenLabelRuntime {
    pub resource_labels: Vec<String>,
    pub resource_spacing_samples: Vec<RtsFirstContactResourceSpacingSnapshot>,
    pub production_slot_labels: Vec<String>,
    pub production_slot_badge_labels: Vec<String>,
    pub production_slot_badge_widths_px: Vec<i32>,
    pub production_slot_status_labels: Vec<String>,
    pub production_slot_status_widths_px: Vec<i32>,
    pub production_empty_slot_status_labels: Vec<String>,
    pub production_empty_slot_status_widths_px: Vec<i32>,
    pub build_palette_labels: Vec<String>,
    pub build_palette_badge_labels: Vec<String>,
    pub build_palette_badge_widths_px: Vec<i32>,
    pub build_palette_state_labels: Vec<String>,
    pub build_palette_state_widths_px: Vec<i32>,
    pub build_palette_fit_samples: Vec<RtsFirstContactBuildPaletteFitSnapshot>,
    pub build_palette_badge_fit_samples: Vec<RtsFirstContactBuildPaletteFitSnapshot>,
    pub order_queue_labels: Vec<String>,
    pub order_queue_label_widths_px: Vec<i32>,
    pub order_queue_badge_labels: Vec<String>,
    pub order_queue_badge_widths_px: Vec<i32>,
    pub completion_event_labels: Vec<String>,
    pub completion_event_label_widths_px: Vec<i32>,
    pub completion_event_badge_labels: Vec<String>,
    pub completion_event_badge_widths_px: Vec<i32>,
    pub tactics_queue_summary: String,
    pub tactics_queue_summary_width_px: i32,
    pub tactics_target_label: String,
    pub tactics_build_label: String,
    pub tactics_detail_labels: Vec<String>,
    pub tactics_detail_widths_px: Vec<i32>,
    pub tactics_compact_badge_labels: Vec<String>,
    pub tactics_compact_badge_widths_px: Vec<i32>,
    pub tactics_queue_fallback_values: Vec<String>,
    pub tactics_queue_fallback_badge_labels: Vec<String>,
    pub tactics_queue_fallback_badge_widths_px: Vec<i32>,
    pub field_status_title: String,
    pub live_status_labels: Vec<String>,
    pub live_status_widths_px: Vec<i32>,
    pub live_state_labels: Vec<String>,
    pub live_state_widths_px: Vec<i32>,
    pub tactical_header_title: String,
    pub tactical_header_order_label: String,
    pub tactical_header_camera_label: String,
    pub tactical_header_title_width_px: i32,
    pub tactical_header_order_width_px: i32,
    pub tactical_header_camera_width_px: i32,
    pub tactical_header_title_order_width_px: i32,
    pub build_placement_status_label: String,
    pub upgrade_placement_status_label: String,
    pub build_placement_status_width_px: i32,
    pub upgrade_placement_status_width_px: i32,
    pub geometry: RtsFirstContactPlayerScreenLabelGeometrySnapshot,
}

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn player_screen_label_has_raw_marker(label: &str) -> bool {
    let upper = label.to_ascii_uppercase();
    upper.contains("TRNM")
        || upper.contains("PRODUCTION COMPLETE")
        || upper.contains("BUILD COMPLETE")
        || upper.contains("UPGRADE COMPLETE")
        || label.contains(':')
        || label.contains('.')
        || label.contains('@')
        || label.contains('_')
        || label.contains("->")
}

fn live_label_has_raw_marker(label: &str) -> bool {
    let upper = label.to_ascii_uppercase();
    label.contains(':')
        || label.contains('_')
        || label.contains("->")
        || upper.contains("LIVE INPUT")
        || upper.contains("LMB")
        || upper.contains("WASD")
        || upper.contains("CTRL")
        || upper.contains("SHIFT")
        || upper.contains("PROD ")
}

fn resource_spacing_sample_json(sample: &RtsFirstContactResourceSpacingSnapshot) -> Value {
    json!({
        "label": sample.label,
        "text_width_px": sample.text_width_px,
        "value_x_delta_px": sample.value_x_delta_px,
        "value_spacing_gate": sample.value_x_delta_px >= sample.text_width_px + 24,
    })
}

fn build_palette_fit_sample_json(sample: &RtsFirstContactBuildPaletteFitSnapshot) -> Value {
    json!({
        "label": sample.label,
        "label_x": sample.label_x,
        "right_x": sample.right_x,
        "fits_tile_gate": sample.label_x >= 102 && sample.right_x <= 146,
    })
}

pub fn first_contact_player_screen_label_guard(
    runtime: &RtsFirstContactPlayerScreenLabelRuntime,
) -> Value {
    let resource_spacing_samples = runtime
        .resource_spacing_samples
        .iter()
        .map(resource_spacing_sample_json)
        .collect::<Vec<_>>();
    let build_palette_fit_samples = runtime
        .build_palette_fit_samples
        .iter()
        .map(build_palette_fit_sample_json)
        .collect::<Vec<_>>();
    let build_palette_badge_fit_samples = runtime
        .build_palette_badge_fit_samples
        .iter()
        .map(build_palette_fit_sample_json)
        .collect::<Vec<_>>();
    let mut all_display_labels = Vec::new();
    all_display_labels.extend(runtime.resource_labels.iter().cloned());
    all_display_labels.extend(runtime.production_slot_labels.iter().cloned());
    all_display_labels.extend(runtime.production_slot_status_labels.iter().cloned());
    all_display_labels.extend(runtime.production_empty_slot_status_labels.iter().cloned());
    all_display_labels.extend(runtime.build_palette_labels.iter().cloned());
    all_display_labels.extend(runtime.build_palette_badge_labels.iter().cloned());
    all_display_labels.extend(runtime.build_palette_state_labels.iter().cloned());
    all_display_labels.extend(runtime.order_queue_labels.iter().cloned());
    all_display_labels.extend(runtime.order_queue_badge_labels.iter().cloned());
    all_display_labels.extend(runtime.completion_event_labels.iter().cloned());
    all_display_labels.extend(runtime.completion_event_badge_labels.iter().cloned());
    all_display_labels.extend(runtime.tactics_detail_labels.iter().cloned());
    all_display_labels.extend(runtime.tactics_compact_badge_labels.iter().cloned());
    all_display_labels.extend(runtime.tactics_queue_fallback_badge_labels.iter().cloned());
    all_display_labels.push(runtime.field_status_title.clone());
    all_display_labels.extend(runtime.live_status_labels.iter().cloned());
    all_display_labels.extend(runtime.live_state_labels.iter().cloned());
    all_display_labels.extend([
        runtime.tactical_header_title.clone(),
        runtime.tactical_header_order_label.clone(),
        runtime.tactical_header_camera_label.clone(),
        runtime.build_placement_status_label.clone(),
        runtime.upgrade_placement_status_label.clone(),
    ]);

    let expected_label_gate = runtime.resource_labels
        == string_vec(["CREDITS", "POWER", "SUPPLY", "VISION"])
        && runtime.production_slot_labels == string_vec(["GUARD", "WORKER", "SIGNAL", "TRAINING"])
        && runtime.production_slot_badge_labels == string_vec(["GRD", "WRK", "SIG", "TRN"])
        && runtime.production_slot_status_labels
            == string_vec(["Q1 64 R", "Q2 42 R", "Q3 64 R", "B2 42 R"])
        && runtime.production_empty_slot_status_labels
            == string_vec(["ADD UNIT", "ADD UNIT", "ADD BUILD", "ADD BUILD"])
        && runtime.build_palette_labels
            == string_vec([
                "POWER", "TRAIN", "REFINE", "TOWER", "COMMAND", "RADAR", "WALL", "SIGNAL",
            ])
        && runtime.build_palette_badge_labels
            == string_vec(["PWR", "TRN", "REF", "TWR", "CMD", "RAD", "WAL", "SIG"])
        && runtime.build_palette_state_labels
            == string_vec([
                "READY", "QUEUE", "READY", "QUEUE", "READY", "READY", "READY", "QUEUE",
            ])
        && runtime.order_queue_labels
            == string_vec(["ATTACK BEACON", "TRAIN WORKER", "BUILD RELAY", "MOVE 16/9"])
        && runtime.order_queue_badge_labels
            == string_vec(["ATK BCN", "TRN WRK", "BLD RLY", "MOV 16/9"])
        && runtime.completion_event_labels
            == string_vec([
                "WORKER READY",
                "SIGNAL READY",
                "TOWER READY",
                "TRAINING READY",
            ])
        && runtime.completion_event_badge_labels
            == string_vec(["WRK RDY", "SIG RDY", "TWR RDY", "TRN RDY"])
        && runtime.tactics_queue_summary == "GUARD 64% TOWER 42%"
        && runtime.tactics_compact_badge_labels
            == string_vec(["SECURE", "BEACON", "16/16", "G64/T42", "IDLE"])
        && runtime.tactics_queue_fallback_badge_labels
            == string_vec(["TRN SIG", "TRN SIG", "BLD RLY", "ATK BCN", "RDY"])
        && runtime.tactics_target_label == "RELAY BEACON"
        && runtime.tactics_build_label == "IDLE"
        && runtime.field_status_title == "FIELD STATUS"
        && runtime.live_status_labels
            == string_vec([
                "SQUAD READY",
                "RALLY 16/9",
                "QUEUE GUARD 64%",
                "SCOUTING 76%",
                "CAMERA 16/16",
                "SUPPLY 12/22",
                "SAVE ROUTE READY",
            ])
        && runtime.live_state_labels
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
        && runtime.tactical_header_title == "TACTICAL VIEW"
        && runtime.tactical_header_order_label == "SECURE RELAY BEACON"
        && runtime.tactical_header_camera_label == "CAM 16/16 Z100"
        && runtime.build_placement_status_label.starts_with("PLACE ")
        && runtime.build_placement_status_label.contains("TOWER")
        && runtime.upgrade_placement_status_label.contains("TRAINING");
    let resource_spacing_gate = runtime
        .resource_spacing_samples
        .iter()
        .all(|sample| sample.value_x_delta_px >= sample.text_width_px + 24);
    let production_slot_width_gate = runtime
        .production_slot_badge_widths_px
        .iter()
        .all(|width| *width <= 18);
    let production_slot_status_width_gate = runtime
        .production_slot_status_widths_px
        .iter()
        .chain(runtime.production_empty_slot_status_widths_px.iter())
        .all(|width| *width <= 52);
    let build_palette_width_gate = runtime
        .build_palette_badge_fit_samples
        .iter()
        .all(|sample| sample.label_x >= 102 && sample.right_x <= 146);
    let build_palette_state_width_gate = runtime
        .build_palette_state_widths_px
        .iter()
        .all(|width| *width <= 42);
    let order_queue_width_gate = runtime
        .order_queue_label_widths_px
        .iter()
        .chain(runtime.completion_event_label_widths_px.iter())
        .all(|width| *width <= 210);
    let order_queue_badge_width_gate = runtime
        .order_queue_badge_widths_px
        .iter()
        .chain(runtime.completion_event_badge_widths_px.iter())
        .all(|width| *width <= 48);
    let tactics_summary_width_gate = runtime.tactics_queue_summary_width_px <= 120;
    let tactics_detail_width_gate = runtime
        .tactics_detail_widths_px
        .iter()
        .all(|width| *width <= 132);
    let tactics_compact_badge_width_gate = runtime
        .tactics_compact_badge_widths_px
        .iter()
        .all(|width| *width <= 48);
    let tactics_queue_fallback_badge_width_gate = runtime
        .tactics_queue_fallback_badge_widths_px
        .iter()
        .all(|width| *width <= 42);
    let live_status_width_gate = runtime
        .live_status_widths_px
        .iter()
        .all(|width| *width <= 132);
    let live_state_width_gate = runtime
        .live_state_widths_px
        .iter()
        .all(|width| *width <= 180);
    let tactical_header_title_order_width_gate =
        runtime.tactical_header_title_order_width_px <= 310;
    let tactical_header_camera_width_gate = runtime.tactical_header_camera_width_px <= 96;
    let build_placement_status_width_gate = runtime.build_placement_status_width_px <= 240
        && runtime.upgrade_placement_status_width_px <= 240;
    let tactical_header_vertical_separation_gate =
        runtime.geometry.tactical_header_title_y_offset_px
            + runtime.geometry.tactical_header_label_gap_px
            <= runtime.geometry.tactical_header_status_y_offset_px;
    let raw_marker_gate = all_display_labels.iter().all(|label| {
        !player_screen_label_has_raw_marker(label) && !live_label_has_raw_marker(label)
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
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_LABEL_GUARD_CONTRACT,
        "green": green,
        "source_path": "trnm-world-bevy classic_draw_openra_style_rts_shell display-label helpers",
        "resource_labels": runtime.resource_labels,
        "resource_spacing_samples": resource_spacing_samples,
        "production_slot_labels": runtime.production_slot_labels,
        "production_slot_badge_labels": runtime.production_slot_badge_labels,
        "production_slot_badge_widths": runtime.production_slot_badge_widths_px,
        "production_slot_status_labels": runtime.production_slot_status_labels,
        "production_empty_slot_status_labels": runtime.production_empty_slot_status_labels,
        "build_palette_labels": runtime.build_palette_labels,
        "build_palette_badge_labels": runtime.build_palette_badge_labels,
        "build_palette_badge_widths": runtime.build_palette_badge_widths_px,
        "build_palette_state_labels": runtime.build_palette_state_labels,
        "build_palette_fit_samples": build_palette_fit_samples,
        "build_palette_badge_fit_samples": build_palette_badge_fit_samples,
        "order_queue_labels": runtime.order_queue_labels,
        "order_queue_badge_labels": runtime.order_queue_badge_labels,
        "completion_event_labels": runtime.completion_event_labels,
        "completion_event_badge_labels": runtime.completion_event_badge_labels,
        "tactics_queue_summary": runtime.tactics_queue_summary,
        "tactics_target_label": runtime.tactics_target_label,
        "tactics_build_label": runtime.tactics_build_label,
        "tactics_detail_labels": runtime.tactics_detail_labels,
        "tactics_compact_badge_labels": runtime.tactics_compact_badge_labels,
        "tactics_compact_badge_widths": runtime.tactics_compact_badge_widths_px,
        "tactics_queue_fallback_values": runtime.tactics_queue_fallback_values,
        "tactics_queue_fallback_badge_labels": runtime.tactics_queue_fallback_badge_labels,
        "tactics_queue_fallback_badge_widths": runtime.tactics_queue_fallback_badge_widths_px,
        "field_status_title": runtime.field_status_title,
        "live_status_labels": runtime.live_status_labels,
        "live_state_labels": runtime.live_state_labels,
        "tactical_header_title": runtime.tactical_header_title,
        "tactical_header_order_label": runtime.tactical_header_order_label,
        "tactical_header_camera_label": runtime.tactical_header_camera_label,
        "tactical_header_title_width_px": runtime.tactical_header_title_width_px,
        "tactical_header_order_width_px": runtime.tactical_header_order_width_px,
        "tactical_header_camera_width_px": runtime.tactical_header_camera_width_px,
        "tactical_header_title_order_width_px": runtime.tactical_header_title_order_width_px,
        "build_placement_status_label": runtime.build_placement_status_label,
        "upgrade_placement_status_label": runtime.upgrade_placement_status_label,
        "build_placement_status_width_px": runtime.build_placement_status_width_px,
        "upgrade_placement_status_width_px": runtime.upgrade_placement_status_width_px,
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

    fn first_contact_text_advance_px(text: &str) -> i32 {
        text.chars().map(|ch| if ch == ' ' { 4 } else { 6 }).sum()
    }

    fn widths(labels: &[String]) -> Vec<i32> {
        labels
            .iter()
            .map(|label| first_contact_text_advance_px(label))
            .collect()
    }

    fn fit_samples(labels: &[String]) -> Vec<RtsFirstContactBuildPaletteFitSnapshot> {
        labels
            .iter()
            .map(|label| RtsFirstContactBuildPaletteFitSnapshot {
                label: label.clone(),
                label_x: 102,
                right_x: 102 + first_contact_text_advance_px(label),
            })
            .collect()
    }

    fn label_runtime() -> RtsFirstContactPlayerScreenLabelRuntime {
        let resource_labels = string_vec(["CREDITS", "POWER", "SUPPLY", "VISION"]);
        let production_slot_labels = string_vec(["GUARD", "WORKER", "SIGNAL", "TRAINING"]);
        let production_slot_badge_labels = string_vec(["GRD", "WRK", "SIG", "TRN"]);
        let production_slot_status_labels =
            string_vec(["Q1 64 R", "Q2 42 R", "Q3 64 R", "B2 42 R"]);
        let production_empty_slot_status_labels =
            string_vec(["ADD UNIT", "ADD UNIT", "ADD BUILD", "ADD BUILD"]);
        let build_palette_labels = string_vec([
            "POWER", "TRAIN", "REFINE", "TOWER", "COMMAND", "RADAR", "WALL", "SIGNAL",
        ]);
        let build_palette_badge_labels =
            string_vec(["PWR", "TRN", "REF", "TWR", "CMD", "RAD", "WAL", "SIG"]);
        let build_palette_state_labels = string_vec([
            "READY", "QUEUE", "READY", "QUEUE", "READY", "READY", "READY", "QUEUE",
        ]);
        let order_queue_labels =
            string_vec(["ATTACK BEACON", "TRAIN WORKER", "BUILD RELAY", "MOVE 16/9"]);
        let order_queue_badge_labels = string_vec(["ATK BCN", "TRN WRK", "BLD RLY", "MOV 16/9"]);
        let completion_event_labels = string_vec([
            "WORKER READY",
            "SIGNAL READY",
            "TOWER READY",
            "TRAINING READY",
        ]);
        let completion_event_badge_labels =
            string_vec(["WRK RDY", "SIG RDY", "TWR RDY", "TRN RDY"]);
        let tactics_detail_labels = string_vec([
            "SECURE RELAY BEACON",
            "RELAY BEACON",
            "16/16",
            "GUARD 64% TOWER 42%",
            "IDLE",
        ]);
        let tactics_compact_badge_labels =
            string_vec(["SECURE", "BEACON", "16/16", "G64/T42", "IDLE"]);
        let tactics_queue_fallback_values = string_vec([
            "TRAIN SIGNAL",
            "TRAIN SI",
            "BUILD RELAY",
            "ATTACK BEACON",
            "READY",
        ]);
        let tactics_queue_fallback_badge_labels =
            string_vec(["TRN SIG", "TRN SIG", "BLD RLY", "ATK BCN", "RDY"]);
        let live_status_labels = string_vec([
            "SQUAD READY",
            "RALLY 16/9",
            "QUEUE GUARD 64%",
            "SCOUTING 76%",
            "CAMERA 16/16",
            "SUPPLY 12/22",
            "SAVE ROUTE READY",
        ]);
        let live_state_labels = string_vec([
            "ORDER SECURE RELAY BEACON",
            "TARGET RELAY BEACON",
            "BUILD IDLE",
            "QUEUE GUARD 64%",
            "CAM 16/16",
            "DRAG NONE",
            "HOVER NONE",
            "RES UPGRADE 210 CREDITS",
        ]);
        let tactical_header_title = "TACTICAL VIEW".to_string();
        let tactical_header_order_label = "SECURE RELAY BEACON".to_string();
        let tactical_header_camera_label = "CAM 16/16 Z100".to_string();
        let build_placement_status_label = "PLACE TOWER READY".to_string();
        let upgrade_placement_status_label = "PLACE TRAINING READY".to_string();

        RtsFirstContactPlayerScreenLabelRuntime {
            resource_spacing_samples: resource_labels
                .iter()
                .map(|label| {
                    let text_width_px = first_contact_text_advance_px(label);
                    RtsFirstContactResourceSpacingSnapshot {
                        label: label.clone(),
                        text_width_px,
                        value_x_delta_px: text_width_px + 28,
                    }
                })
                .collect(),
            production_slot_badge_widths_px: widths(&production_slot_badge_labels),
            production_slot_status_widths_px: widths(&production_slot_status_labels),
            production_empty_slot_status_widths_px: widths(&production_empty_slot_status_labels),
            build_palette_badge_widths_px: widths(&build_palette_badge_labels),
            build_palette_state_widths_px: widths(&build_palette_state_labels),
            build_palette_fit_samples: fit_samples(&build_palette_labels),
            build_palette_badge_fit_samples: fit_samples(&build_palette_badge_labels),
            order_queue_label_widths_px: widths(&order_queue_labels),
            order_queue_badge_widths_px: widths(&order_queue_badge_labels),
            completion_event_label_widths_px: widths(&completion_event_labels),
            completion_event_badge_widths_px: widths(&completion_event_badge_labels),
            tactics_queue_summary_width_px: first_contact_text_advance_px("GUARD 64% TOWER 42%"),
            tactics_detail_widths_px: widths(&tactics_detail_labels),
            tactics_compact_badge_widths_px: widths(&tactics_compact_badge_labels),
            tactics_queue_fallback_badge_widths_px: widths(&tactics_queue_fallback_badge_labels),
            live_status_widths_px: widths(&live_status_labels),
            live_state_widths_px: widths(&live_state_labels),
            tactical_header_title_width_px: first_contact_text_advance_px(&tactical_header_title),
            tactical_header_order_width_px: first_contact_text_advance_px(
                &tactical_header_order_label,
            ),
            tactical_header_camera_width_px: first_contact_text_advance_px(
                &tactical_header_camera_label,
            ),
            tactical_header_title_order_width_px: first_contact_text_advance_px(
                &tactical_header_title,
            ) + 14
                + first_contact_text_advance_px(&tactical_header_order_label),
            build_placement_status_width_px: first_contact_text_advance_px(
                &build_placement_status_label,
            ),
            upgrade_placement_status_width_px: first_contact_text_advance_px(
                &upgrade_placement_status_label,
            ),
            resource_labels,
            production_slot_labels,
            production_slot_badge_labels,
            production_slot_status_labels,
            production_empty_slot_status_labels,
            build_palette_labels,
            build_palette_badge_labels,
            build_palette_state_labels,
            order_queue_labels,
            order_queue_badge_labels,
            completion_event_labels,
            completion_event_badge_labels,
            tactics_queue_summary: "GUARD 64% TOWER 42%".to_string(),
            tactics_target_label: "RELAY BEACON".to_string(),
            tactics_build_label: "IDLE".to_string(),
            tactics_detail_labels,
            tactics_compact_badge_labels,
            tactics_queue_fallback_values,
            tactics_queue_fallback_badge_labels,
            field_status_title: "FIELD STATUS".to_string(),
            live_status_labels,
            live_state_labels,
            tactical_header_title,
            tactical_header_order_label,
            tactical_header_camera_label,
            build_placement_status_label,
            upgrade_placement_status_label,
            geometry: RtsFirstContactPlayerScreenLabelGeometrySnapshot {
                tactical_header_title_y_offset_px: 18,
                tactical_header_label_gap_px: 4,
                tactical_header_status_y_offset_px: 30,
            },
        }
    }

    #[test]
    fn first_contact_player_screen_label_guard_preserves_display_contracts() {
        let guard = first_contact_player_screen_label_guard(&label_runtime());

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_LABEL_GUARD_CONTRACT)
        );
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
