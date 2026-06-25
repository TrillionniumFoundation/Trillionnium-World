#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};

use crate::TRNM_RTS_EVIDENCE_FIRST_CONTACT_SIDEBAR_DENSITY_CONTRACT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RtsFirstContactSidebarDensityGeometrySnapshot {
    pub production_to_palette_gap_px: i32,
    pub build_palette_slot_width_px: i32,
    pub build_palette_slot_height_px: i32,
    pub build_palette_row_gap_px: i32,
    pub build_palette_inter_row_gap_px: i32,
    pub build_palette_to_tactics_gap_px: i32,
    pub build_palette_state_badge_width_px: i32,
    pub build_palette_state_badge_height_px: i32,
    pub tactics_row_gap_px: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RtsFirstContactSidebarDensityRuntime {
    pub production_slot_visible_count: usize,
    pub production_slot_column_count: usize,
    pub production_slot_labels: Vec<String>,
    pub production_slot_badge_labels: Vec<String>,
    pub production_slot_badge_widths_px: Vec<i32>,
    pub production_slot_status_labels: Vec<String>,
    pub production_slot_status_badge_labels: Vec<String>,
    pub production_slot_status_badge_widths_px: Vec<i32>,
    pub production_empty_slot_status_badge_labels: Vec<String>,
    pub production_empty_slot_status_badge_widths_px: Vec<i32>,
    pub production_empty_slot_badge_label: String,
    pub build_palette_visible_count: usize,
    pub build_palette_column_count: usize,
    pub build_palette_labels: Vec<String>,
    pub build_palette_badge_labels: Vec<String>,
    pub build_palette_badge_widths_px: Vec<i32>,
    pub build_palette_state_labels: Vec<String>,
    pub build_palette_state_badge_labels: Vec<String>,
    pub build_palette_state_badge_widths_px: Vec<i32>,
    pub tactics_row_count: usize,
    pub tactics_detail_labels: Vec<String>,
    pub tactics_compact_badge_labels: Vec<String>,
    pub tactics_compact_badge_widths_px: Vec<i32>,
    pub tactics_queue_fallback_values: Vec<String>,
    pub tactics_queue_fallback_badge_labels: Vec<String>,
    pub tactics_queue_fallback_badge_widths_px: Vec<i32>,
    pub geometry: RtsFirstContactSidebarDensityGeometrySnapshot,
}

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

pub fn first_contact_sidebar_density_guard(
    runtime: &RtsFirstContactSidebarDensityRuntime,
) -> Value {
    let production_slot_visible_count = runtime.production_slot_visible_count.max(1);
    let production_slot_column_count = runtime.production_slot_column_count.max(1);
    let production_row_count = (production_slot_visible_count + production_slot_column_count - 1)
        / production_slot_column_count;
    let build_palette_visible_count = runtime.build_palette_visible_count.max(1);
    let build_palette_column_count = runtime.build_palette_column_count.max(1);
    let build_palette_row_count =
        (build_palette_visible_count + build_palette_column_count - 1) / build_palette_column_count;
    let production_density_gate = production_slot_visible_count == 4
        && production_slot_column_count == 2
        && production_row_count == 2
        && runtime.geometry.production_to_palette_gap_px >= 12
        && runtime.production_slot_labels == string_vec(["GUARD", "WORKER", "SIGNAL", "TRAINING"])
        && runtime.production_slot_badge_labels == string_vec(["GRD", "WRK", "SIG", "TRN"])
        && runtime.production_slot_status_labels
            == string_vec(["Q1 64 R", "Q2 42 R", "Q3 64 R", "B2 42 R"])
        && runtime.production_slot_status_badge_labels == string_vec(["Q1", "Q2", "Q3", "B2"])
        && runtime.production_empty_slot_status_badge_labels
            == string_vec(["ADD", "ADD", "ADD", "ADD"])
        && runtime.production_empty_slot_badge_label == "RDY"
        && runtime
            .production_slot_badge_widths_px
            .iter()
            .all(|width| *width <= 18)
        && runtime
            .production_slot_status_badge_widths_px
            .iter()
            .chain(runtime.production_empty_slot_status_badge_widths_px.iter())
            .all(|width| *width <= 18);
    let palette_geometry_gate = build_palette_visible_count == 8
        && build_palette_column_count == 4
        && build_palette_row_count == 2
        && runtime.geometry.build_palette_slot_width_px == 46
        && runtime.geometry.build_palette_slot_height_px == 40
        && runtime.geometry.build_palette_inter_row_gap_px >= 8
        && runtime.geometry.build_palette_to_tactics_gap_px >= 12;
    let palette_state_badge_gate = runtime.build_palette_state_labels
        == string_vec([
            "READY", "QUEUE", "READY", "QUEUE", "READY", "READY", "READY", "QUEUE",
        ])
        && runtime.build_palette_state_badge_labels
            == string_vec(["RDY", "QUE", "RDY", "QUE", "RDY", "RDY", "RDY", "QUE"])
        && runtime
            .build_palette_state_badge_widths_px
            .iter()
            .all(|width| *width <= runtime.geometry.build_palette_state_badge_width_px - 6)
        && runtime.geometry.build_palette_state_badge_height_px >= 9;
    let palette_label_gate = runtime.build_palette_labels
        == string_vec([
            "POWER", "TRAIN", "REFINE", "TOWER", "COMMAND", "RADAR", "WALL", "SIGNAL",
        ])
        && runtime.build_palette_badge_labels
            == string_vec(["PWR", "TRN", "REF", "TWR", "CMD", "RAD", "WAL", "SIG"])
        && runtime
            .build_palette_badge_widths_px
            .iter()
            .all(|width| *width <= 18);
    let tactics_density_gate = runtime.tactics_row_count == 5
        && runtime.geometry.tactics_row_gap_px >= 4
        && runtime.tactics_detail_labels
            == string_vec([
                "SECURE RELAY BEACON",
                "RELAY BEACON",
                "16/16",
                "GUARD 64% TOWER 42%",
                "IDLE",
            ])
        && runtime.tactics_compact_badge_labels
            == string_vec(["SECURE", "BEACON", "16/16", "G64/T42", "IDLE"])
        && runtime.tactics_queue_fallback_badge_labels
            == string_vec(["TRN SIG", "TRN SIG", "BLD RLY", "ATK BCN", "RDY"])
        && runtime
            .tactics_compact_badge_widths_px
            .iter()
            .chain(runtime.tactics_queue_fallback_badge_widths_px.iter())
            .all(|width| *width <= 48);
    let right_sidebar_density_gate = production_density_gate
        && palette_geometry_gate
        && palette_state_badge_gate
        && palette_label_gate
        && tactics_density_gate;

    json!({
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_SIDEBAR_DENSITY_CONTRACT,
        "green": right_sidebar_density_gate,
        "source_path": "trnm-world-bevy classic_draw_openra_style_rts_shell right sidebar production/build palette/tactics density",
        "production_slot_visible_count": production_slot_visible_count,
        "production_slot_column_count": production_slot_column_count,
        "production_row_count": production_row_count,
        "production_slot_labels": runtime.production_slot_labels,
        "production_slot_badge_labels": runtime.production_slot_badge_labels,
        "production_slot_badge_widths": runtime.production_slot_badge_widths_px,
        "production_slot_status_labels": runtime.production_slot_status_labels,
        "production_slot_status_badge_labels": runtime.production_slot_status_badge_labels,
        "production_slot_status_badge_widths": runtime.production_slot_status_badge_widths_px,
        "production_empty_slot_status_badge_labels": runtime.production_empty_slot_status_badge_labels,
        "production_empty_slot_status_badge_widths": runtime.production_empty_slot_status_badge_widths_px,
        "production_empty_slot_badge_label": runtime.production_empty_slot_badge_label,
        "production_to_palette_gap_px": runtime.geometry.production_to_palette_gap_px,
        "build_palette_visible_count": build_palette_visible_count,
        "build_palette_column_count": build_palette_column_count,
        "build_palette_row_count": build_palette_row_count,
        "build_palette_slot_width_px": runtime.geometry.build_palette_slot_width_px,
        "build_palette_slot_height_px": runtime.geometry.build_palette_slot_height_px,
        "build_palette_row_gap_px": runtime.geometry.build_palette_row_gap_px,
        "build_palette_inter_row_gap_px": runtime.geometry.build_palette_inter_row_gap_px,
        "build_palette_to_tactics_gap_px": runtime.geometry.build_palette_to_tactics_gap_px,
        "build_palette_labels": runtime.build_palette_labels,
        "build_palette_badge_labels": runtime.build_palette_badge_labels,
        "build_palette_badge_widths": runtime.build_palette_badge_widths_px,
        "build_palette_state_labels": runtime.build_palette_state_labels,
        "build_palette_state_badge_labels": runtime.build_palette_state_badge_labels,
        "build_palette_state_badge_widths": runtime.build_palette_state_badge_widths_px,
        "build_palette_state_badge_width_px": runtime.geometry.build_palette_state_badge_width_px,
        "build_palette_state_badge_height_px": runtime.geometry.build_palette_state_badge_height_px,
        "tactics_row_count": runtime.tactics_row_count,
        "tactics_row_gap_px": runtime.geometry.tactics_row_gap_px,
        "tactics_detail_labels": runtime.tactics_detail_labels,
        "tactics_compact_badge_labels": runtime.tactics_compact_badge_labels,
        "tactics_compact_badge_widths": runtime.tactics_compact_badge_widths_px,
        "tactics_queue_fallback_values": runtime.tactics_queue_fallback_values,
        "tactics_queue_fallback_badge_labels": runtime.tactics_queue_fallback_badge_labels,
        "tactics_queue_fallback_badge_widths": runtime.tactics_queue_fallback_badge_widths_px,
        "production_density_gate": production_density_gate,
        "palette_geometry_gate": palette_geometry_gate,
        "palette_state_badge_gate": palette_state_badge_gate,
        "palette_label_gate": palette_label_gate,
        "tactics_density_gate": tactics_density_gate,
        "right_sidebar_density_gate": right_sidebar_density_gate,
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

    fn sidebar_runtime() -> RtsFirstContactSidebarDensityRuntime {
        let production_slot_labels = string_vec(["GUARD", "WORKER", "SIGNAL", "TRAINING"]);
        let production_slot_badge_labels = string_vec(["GRD", "WRK", "SIG", "TRN"]);
        let production_slot_status_labels =
            string_vec(["Q1 64 R", "Q2 42 R", "Q3 64 R", "B2 42 R"]);
        let production_slot_status_badge_labels = string_vec(["Q1", "Q2", "Q3", "B2"]);
        let production_empty_slot_status_badge_labels = string_vec(["ADD", "ADD", "ADD", "ADD"]);
        let build_palette_labels = string_vec([
            "POWER", "TRAIN", "REFINE", "TOWER", "COMMAND", "RADAR", "WALL", "SIGNAL",
        ]);
        let build_palette_badge_labels =
            string_vec(["PWR", "TRN", "REF", "TWR", "CMD", "RAD", "WAL", "SIG"]);
        let build_palette_state_labels = string_vec([
            "READY", "QUEUE", "READY", "QUEUE", "READY", "READY", "READY", "QUEUE",
        ]);
        let build_palette_state_badge_labels =
            string_vec(["RDY", "QUE", "RDY", "QUE", "RDY", "RDY", "RDY", "QUE"]);
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

        RtsFirstContactSidebarDensityRuntime {
            production_slot_visible_count: 4,
            production_slot_column_count: 2,
            production_slot_badge_widths_px: widths(&production_slot_badge_labels),
            production_slot_status_badge_widths_px: widths(&production_slot_status_badge_labels),
            production_empty_slot_status_badge_widths_px: widths(
                &production_empty_slot_status_badge_labels,
            ),
            production_slot_labels,
            production_slot_badge_labels,
            production_slot_status_labels,
            production_slot_status_badge_labels,
            production_empty_slot_status_badge_labels,
            production_empty_slot_badge_label: "RDY".to_string(),
            build_palette_visible_count: 8,
            build_palette_column_count: 4,
            build_palette_badge_widths_px: widths(&build_palette_badge_labels),
            build_palette_state_badge_widths_px: widths(&build_palette_state_badge_labels),
            build_palette_labels,
            build_palette_badge_labels,
            build_palette_state_labels,
            build_palette_state_badge_labels,
            tactics_row_count: 5,
            tactics_compact_badge_widths_px: widths(&tactics_compact_badge_labels),
            tactics_queue_fallback_badge_widths_px: widths(&tactics_queue_fallback_badge_labels),
            tactics_detail_labels,
            tactics_compact_badge_labels,
            tactics_queue_fallback_values,
            tactics_queue_fallback_badge_labels,
            geometry: RtsFirstContactSidebarDensityGeometrySnapshot {
                production_to_palette_gap_px: 16,
                build_palette_slot_width_px: 46,
                build_palette_slot_height_px: 40,
                build_palette_row_gap_px: 48,
                build_palette_inter_row_gap_px: 8,
                build_palette_to_tactics_gap_px: 16,
                build_palette_state_badge_width_px: 24,
                build_palette_state_badge_height_px: 10,
                tactics_row_gap_px: 4,
            },
        }
    }

    #[test]
    fn first_contact_sidebar_density_preserves_right_rail_contracts() {
        let guard = first_contact_sidebar_density_guard(&sidebar_runtime());

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_SIDEBAR_DENSITY_CONTRACT)
        );
        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard.get("production_slot_badge_labels").cloned(),
            Some(json!(["GRD", "WRK", "SIG", "TRN"]))
        );
        assert_eq!(
            guard.get("production_slot_status_badge_labels").cloned(),
            Some(json!(["Q1", "Q2", "Q3", "B2"]))
        );
        assert_eq!(
            guard.get("build_palette_badge_labels").cloned(),
            Some(json!([
                "PWR", "TRN", "REF", "TWR", "CMD", "RAD", "WAL", "SIG"
            ]))
        );
        assert_eq!(
            guard.get("build_palette_state_badge_labels").cloned(),
            Some(json!([
                "RDY", "QUE", "RDY", "QUE", "RDY", "RDY", "RDY", "QUE"
            ]))
        );
        assert_eq!(
            guard
                .get("build_palette_to_tactics_gap_px")
                .and_then(Value::as_i64),
            Some(16)
        );
        assert_eq!(
            guard.get("tactics_compact_badge_labels").cloned(),
            Some(json!(["SECURE", "BEACON", "16/16", "G64/T42", "IDLE"]))
        );
        assert_eq!(
            guard.get("tactics_queue_fallback_badge_labels").cloned(),
            Some(json!(["TRN SIG", "TRN SIG", "BLD RLY", "ATK BCN", "RDY"]))
        );

        for gate in [
            "production_density_gate",
            "palette_geometry_gate",
            "palette_state_badge_gate",
            "palette_label_gate",
            "tactics_density_gate",
            "right_sidebar_density_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }
}
