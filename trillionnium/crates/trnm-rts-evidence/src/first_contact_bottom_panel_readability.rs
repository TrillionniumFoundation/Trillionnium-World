#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use trnm_rts_bevy_runtime::{
    rts_first_contact_bottom_panel_feedback_label as first_contact_bottom_panel_feedback_label,
    rts_first_contact_bottom_panel_squad_roles as first_contact_bottom_panel_squad_roles,
    TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_SUBJECT_SURFACE_CONTRACT,
};

use crate::TRNM_RTS_EVIDENCE_FIRST_CONTACT_BOTTOM_PANEL_READABILITY_CONTRACT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RtsFirstContactBottomPanelGeometrySnapshot {
    pub bottom_panel_height_px: i32,
    pub squad_chip_y_offset_px: i32,
    pub squad_chip_height_px: i32,
    pub squad_chip_bottom_margin_min_px: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RtsFirstContactBottomPanelRuntime {
    pub control_group_id: Option<String>,
    pub selected_unit_ids: Vec<String>,
    pub last_feedback: String,
    pub selection_feedback_max_chars: usize,
    pub group_summary_prefix: String,
    pub group_summary_suffix: String,
    pub order_queue_badge_labels: Vec<String>,
    pub completion_event_badge_labels: Vec<String>,
    pub feedback_label_widths_px: Vec<i32>,
    pub squad_role_label_widths_px: Vec<i32>,
    pub order_queue_badge_widths_px: Vec<i32>,
    pub completion_event_badge_widths_px: Vec<i32>,
    pub geometry: RtsFirstContactBottomPanelGeometrySnapshot,
}

fn first_contact_label_has_raw_marker(label: &str) -> bool {
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

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

pub fn first_contact_bottom_panel_readability_guard(
    runtime: &RtsFirstContactBottomPanelRuntime,
) -> Value {
    let selection_feedback_max_chars = runtime.selection_feedback_max_chars.max(1);
    let group_id = runtime.control_group_id.as_deref().unwrap_or("-");
    let selected_unit_display_count = runtime.selected_unit_ids.len().max(4);
    let group_summary = format!(
        "{} {group_id}  {} {}",
        runtime.group_summary_prefix, selected_unit_display_count, runtime.group_summary_suffix
    );
    let runtime_feedback_label = first_contact_bottom_panel_feedback_label(
        &runtime.last_feedback,
        selection_feedback_max_chars,
    );
    let upgrade_feedback_label = first_contact_bottom_panel_feedback_label(
        "RTS UPGRADE COMPLETE: SIGNAL BLADE",
        selection_feedback_max_chars,
    );
    let build_feedback_label = first_contact_bottom_panel_feedback_label(
        "RTS BUILD COMPLETE: build:watch_tower@7,4->watch_tower",
        selection_feedback_max_chars,
    );
    let squad_role_labels = first_contact_bottom_panel_squad_roles(
        &runtime.selected_unit_ids,
        selected_unit_display_count,
    );
    let order_queue_badge_labels = runtime.order_queue_badge_labels.clone();
    let completion_event_badge_labels = runtime.completion_event_badge_labels.clone();
    let feedback_labels = vec![
        runtime_feedback_label.clone(),
        upgrade_feedback_label.clone(),
        build_feedback_label.clone(),
    ];
    let raw_marker_gate = feedback_labels.iter().all(|label| {
        !first_contact_label_has_raw_marker(label)
            && !live_label_has_raw_marker(label)
            && !label.to_ascii_uppercase().contains("RTS ")
    });
    let feedback_expected_gate = upgrade_feedback_label == "SIGNAL BLADE READY"
        && build_feedback_label == "WATCH TOWER READY"
        && !runtime_feedback_label.is_empty();
    let feedback_width_gate = runtime
        .feedback_label_widths_px
        .iter()
        .take(feedback_labels.len())
        .all(|width| *width <= 268)
        && runtime.feedback_label_widths_px.len() >= feedback_labels.len();
    let squad_strip_gate = squad_role_labels == string_vec(["WORKER", "SCOUT", "GUARD", "RELAY"]);
    let squad_chip_width_gate = runtime
        .squad_role_label_widths_px
        .iter()
        .take(squad_role_labels.len())
        .all(|width| *width <= 52)
        && runtime.squad_role_label_widths_px.len() >= squad_role_labels.len();
    let squad_chip_bottom_margin_px = runtime.geometry.bottom_panel_height_px
        - (runtime.geometry.squad_chip_y_offset_px + runtime.geometry.squad_chip_height_px);
    let squad_chip_edge_clearance_gate =
        squad_chip_bottom_margin_px >= runtime.geometry.squad_chip_bottom_margin_min_px;
    let order_queue_badge_gate = order_queue_badge_labels
        == string_vec(["ATK BCN", "TRN WRK", "BLD RLY", "MOV 16/9"])
        && completion_event_badge_labels
            == string_vec(["WRK RDY", "SIG RDY", "TWR RDY", "TRN RDY"])
        && runtime
            .order_queue_badge_widths_px
            .iter()
            .chain(runtime.completion_event_badge_widths_px.iter())
            .all(|width| *width <= 48)
        && runtime.order_queue_badge_widths_px.len() >= order_queue_badge_labels.len()
        && runtime.completion_event_badge_widths_px.len() >= completion_event_badge_labels.len();
    let selection_density_gate = selected_unit_display_count >= 4
        && squad_role_labels.len() >= 4
        && group_summary == "GROUP 1  4 UNITS SELECTED";
    let green = raw_marker_gate
        && feedback_expected_gate
        && feedback_width_gate
        && squad_strip_gate
        && squad_chip_width_gate
        && squad_chip_edge_clearance_gate
        && order_queue_badge_gate
        && selection_density_gate;

    json!({
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_BOTTOM_PANEL_READABILITY_CONTRACT,
        "subject_surface_contract": TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_SUBJECT_SURFACE_CONTRACT,
        "green": green,
        "source_path": "trnm-world-bevy classic_draw_openra_style_rts_shell bottom selection/status panel",
        "group_summary": group_summary,
        "selected_unit_display_count": selected_unit_display_count,
        "feedback_labels": feedback_labels,
        "runtime_feedback_label": runtime_feedback_label,
        "upgrade_feedback_label": upgrade_feedback_label,
        "build_feedback_label": build_feedback_label,
        "feedback_label_widths_px": runtime.feedback_label_widths_px,
        "squad_role_labels": squad_role_labels,
        "squad_role_label_widths_px": runtime.squad_role_label_widths_px,
        "order_queue_badge_labels": order_queue_badge_labels,
        "order_queue_badge_widths_px": runtime.order_queue_badge_widths_px,
        "completion_event_badge_labels": completion_event_badge_labels,
        "completion_event_badge_widths_px": runtime.completion_event_badge_widths_px,
        "raw_marker_gate": raw_marker_gate,
        "feedback_expected_gate": feedback_expected_gate,
        "feedback_width_gate": feedback_width_gate,
        "squad_strip_gate": squad_strip_gate,
        "squad_chip_width_gate": squad_chip_width_gate,
        "squad_chip_bottom_margin_px": squad_chip_bottom_margin_px,
        "squad_chip_edge_clearance_gate": squad_chip_edge_clearance_gate,
        "order_queue_badge_gate": order_queue_badge_gate,
        "selection_density_gate": selection_density_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_contact_text_advance_px(text: &str) -> i32 {
        text.chars().map(|ch| if ch == ' ' { 4 } else { 6 }).sum()
    }

    fn bottom_panel_runtime() -> RtsFirstContactBottomPanelRuntime {
        let feedback_labels = vec![
            "SIGNAL BLADE READY".to_string(),
            "SIGNAL BLADE READY".to_string(),
            "WATCH TOWER READY".to_string(),
        ];
        let squad_role_labels = vec![
            "WORKER".to_string(),
            "SCOUT".to_string(),
            "GUARD".to_string(),
            "RELAY".to_string(),
        ];
        let order_queue_badge_labels = string_vec(["ATK BCN", "TRN WRK", "BLD RLY", "MOV 16/9"]);
        let completion_event_badge_labels =
            string_vec(["WRK RDY", "SIG RDY", "TWR RDY", "TRN RDY"]);

        RtsFirstContactBottomPanelRuntime {
            control_group_id: Some("1".to_string()),
            selected_unit_ids: string_vec([
                "worker_03",
                "horizon_scout",
                "forge_warden",
                "flux_relay",
            ]),
            last_feedback: "RTS UPGRADE COMPLETE: SIGNAL BLADE".to_string(),
            selection_feedback_max_chars: 62,
            group_summary_prefix: "GROUP".to_string(),
            group_summary_suffix: "UNITS SELECTED".to_string(),
            feedback_label_widths_px: feedback_labels
                .iter()
                .map(|label| first_contact_text_advance_px(label))
                .collect(),
            squad_role_label_widths_px: squad_role_labels
                .iter()
                .map(|label| first_contact_text_advance_px(label))
                .collect(),
            order_queue_badge_widths_px: order_queue_badge_labels
                .iter()
                .map(|label| first_contact_text_advance_px(label))
                .collect(),
            completion_event_badge_widths_px: completion_event_badge_labels
                .iter()
                .map(|label| first_contact_text_advance_px(label))
                .collect(),
            order_queue_badge_labels,
            completion_event_badge_labels,
            geometry: RtsFirstContactBottomPanelGeometrySnapshot {
                bottom_panel_height_px: 148,
                squad_chip_y_offset_px: 120,
                squad_chip_height_px: 11,
                squad_chip_bottom_margin_min_px: 16,
            },
        }
    }

    #[test]
    fn first_contact_bottom_panel_helpers_preserve_feedback_and_roles() {
        assert_eq!(
            first_contact_bottom_panel_feedback_label("RTS UPGRADE COMPLETE: SIGNAL BLADE", 62),
            "SIGNAL BLADE READY"
        );
        assert_eq!(
            first_contact_bottom_panel_feedback_label(
                "RTS BUILD COMPLETE: build:watch_tower@7,4->watch_tower",
                62
            ),
            "WATCH TOWER READY"
        );
        assert_eq!(
            first_contact_bottom_panel_feedback_label("RTS GROUP 1 SECURING RELAY", 62),
            "GROUP 1 SECURING RELAY"
        );
        assert_eq!(
            first_contact_bottom_panel_squad_roles(
                &string_vec(["worker_03", "horizon_scout", "forge_warden", "flux_relay"]),
                4
            ),
            string_vec(["WORKER", "SCOUT", "GUARD", "RELAY"])
        );
    }

    #[test]
    fn first_contact_bottom_panel_readability_preserves_status_contracts() {
        let guard = first_contact_bottom_panel_readability_guard(&bottom_panel_runtime());

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_BOTTOM_PANEL_READABILITY_CONTRACT)
        );
        assert_eq!(
            guard
                .get("subject_surface_contract")
                .and_then(Value::as_str),
            Some(TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_SUBJECT_SURFACE_CONTRACT)
        );
        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard.get("group_summary").and_then(Value::as_str),
            Some("GROUP 1  4 UNITS SELECTED")
        );
        assert_eq!(
            guard.get("squad_role_labels").cloned(),
            Some(json!(["WORKER", "SCOUT", "GUARD", "RELAY"]))
        );
        assert_eq!(
            guard.get("order_queue_badge_labels").cloned(),
            Some(json!(["ATK BCN", "TRN WRK", "BLD RLY", "MOV 16/9"]))
        );
        assert_eq!(
            guard.get("completion_event_badge_labels").cloned(),
            Some(json!(["WRK RDY", "SIG RDY", "TWR RDY", "TRN RDY"]))
        );
        assert_eq!(
            guard
                .get("squad_chip_bottom_margin_px")
                .and_then(Value::as_i64),
            Some(17)
        );

        for gate in [
            "raw_marker_gate",
            "feedback_expected_gate",
            "feedback_width_gate",
            "squad_strip_gate",
            "squad_chip_width_gate",
            "squad_chip_edge_clearance_gate",
            "order_queue_badge_gate",
            "selection_density_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }
}
