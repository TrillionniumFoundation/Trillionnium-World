#![cfg(not(target_os = "android"))]

use serde_json::Value;
use trnm_rts_data::RtsFirstContactPlayerScreenChromeProfile;
use trnm_rts_evidence::{
    RtsFirstContactSidebarDensityGeometrySnapshot, RtsFirstContactSidebarDensityRuntime,
};

use crate::{
    classic_first_contact_empty_production_slot_status_labels,
    classic_first_contact_production_slot_badge_label,
    classic_first_contact_production_status_badge_labels,
    classic_first_contact_rendered_build_palette_badge_labels,
    classic_first_contact_rendered_build_palette_labels,
    classic_first_contact_rendered_build_palette_state_badge_labels,
    classic_first_contact_rendered_build_palette_state_labels,
    classic_first_contact_rendered_production_slot_labels,
    classic_first_contact_rendered_production_slot_status_labels,
    classic_first_contact_tactics_queue_fallback_badge_label,
    classic_first_contact_tactics_row_badge_label, classic_first_contact_tactics_row_value,
    classic_rts_queue_slot_label, classic_text_advance_px, NativeFirstPlayableRuntime,
    CLASSIC_FIRST_CONTACT_BUILD_PALETTE_ROW_GAP_PX, CLASSIC_FIRST_CONTACT_BUILD_PALETTE_SLOT_H_PX,
    CLASSIC_FIRST_CONTACT_BUILD_PALETTE_SLOT_W_PX,
    CLASSIC_FIRST_CONTACT_BUILD_PALETTE_STATE_BADGE_H_PX,
    CLASSIC_FIRST_CONTACT_BUILD_PALETTE_STATE_BADGE_W_PX,
    CLASSIC_FIRST_CONTACT_BUILD_PALETTE_TITLE_TO_SLOT_Y_PX,
    CLASSIC_FIRST_CONTACT_BUILD_PALETTE_TO_TACTICS_Y_PX,
};

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

pub(crate) fn sidebar_density_guard(
    runtime: &NativeFirstPlayableRuntime,
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> Value {
    let production_slot_visible_count = chrome.production_slot_visible_count.max(1) as usize;
    let production_slot_column_count = chrome.production_slot_column_count.max(1) as usize;
    let production_row_count = (production_slot_visible_count + production_slot_column_count - 1)
        / production_slot_column_count;
    let production_slot_status_labels =
        classic_first_contact_rendered_production_slot_status_labels(runtime, chrome);
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
    let production_slot_status_badge_labels =
        classic_first_contact_production_status_badge_labels(&production_slot_status_labels);
    let production_empty_slot_status_labels =
        classic_first_contact_empty_production_slot_status_labels(chrome);
    let production_empty_slot_status_badge_labels =
        classic_first_contact_production_status_badge_labels(&production_empty_slot_status_labels);
    let production_empty_slot_badge_label = classic_first_contact_production_slot_badge_label(
        &classic_rts_queue_slot_label(&chrome.production_empty_label),
    );
    let production_slot_status_badge_widths = production_slot_status_badge_labels
        .iter()
        .map(|label| classic_text_advance_px(label, 1))
        .collect::<Vec<_>>();
    let production_empty_slot_status_badge_widths = production_empty_slot_status_badge_labels
        .iter()
        .map(|label| classic_text_advance_px(label, 1))
        .collect::<Vec<_>>();
    let production_slot_y_offset_px = 18_i32;
    let production_slot_row_gap_px = 34_i32;
    let production_status_bottom_offset_px = 30_i32;
    let production_to_palette_y_px = 98_i32;
    let production_to_palette_gap_px = production_to_palette_y_px
        - (production_slot_y_offset_px
            + (production_row_count.saturating_sub(1) as i32 * production_slot_row_gap_px)
            + production_status_bottom_offset_px);

    let build_palette_visible_count = chrome.build_palette_visible_count.max(1) as usize;
    let build_palette_column_count = chrome.build_palette_column_count.max(1) as usize;
    let build_palette_row_count =
        (build_palette_visible_count + build_palette_column_count - 1) / build_palette_column_count;
    let build_palette_labels = classic_first_contact_rendered_build_palette_labels(chrome);
    let build_palette_badge_labels =
        classic_first_contact_rendered_build_palette_badge_labels(chrome);
    let build_palette_state_labels =
        classic_first_contact_rendered_build_palette_state_labels(runtime, chrome);
    let build_palette_state_badge_labels =
        classic_first_contact_rendered_build_palette_state_badge_labels(runtime, chrome);
    let palette_row_gap_px = CLASSIC_FIRST_CONTACT_BUILD_PALETTE_ROW_GAP_PX
        - CLASSIC_FIRST_CONTACT_BUILD_PALETTE_SLOT_H_PX;
    let palette_to_tactics_gap_px = CLASSIC_FIRST_CONTACT_BUILD_PALETTE_TO_TACTICS_Y_PX
        - (CLASSIC_FIRST_CONTACT_BUILD_PALETTE_TITLE_TO_SLOT_Y_PX
            + (build_palette_row_count.saturating_sub(1) as i32
                * CLASSIC_FIRST_CONTACT_BUILD_PALETTE_ROW_GAP_PX)
            + CLASSIC_FIRST_CONTACT_BUILD_PALETTE_SLOT_H_PX);
    let build_palette_state_badge_widths = build_palette_state_badge_labels
        .iter()
        .map(|label| classic_text_advance_px(label, 1))
        .collect::<Vec<_>>();
    let build_palette_badge_widths = build_palette_badge_labels
        .iter()
        .map(|label| classic_text_advance_px(label, 1))
        .collect::<Vec<_>>();
    let tactics_row_count = chrome.tactics_rows.len();
    let tactics_row_gap_px = 22_i32 - 18_i32;
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

    let sidebar_runtime = RtsFirstContactSidebarDensityRuntime {
        production_slot_visible_count,
        production_slot_column_count,
        production_slot_labels,
        production_slot_badge_labels,
        production_slot_badge_widths_px: production_slot_badge_widths,
        production_slot_status_labels,
        production_slot_status_badge_labels,
        production_slot_status_badge_widths_px: production_slot_status_badge_widths,
        production_empty_slot_status_badge_labels,
        production_empty_slot_status_badge_widths_px: production_empty_slot_status_badge_widths,
        production_empty_slot_badge_label,
        build_palette_visible_count,
        build_palette_column_count,
        build_palette_labels,
        build_palette_badge_labels,
        build_palette_badge_widths_px: build_palette_badge_widths,
        build_palette_state_labels,
        build_palette_state_badge_labels,
        build_palette_state_badge_widths_px: build_palette_state_badge_widths,
        tactics_row_count,
        tactics_detail_labels,
        tactics_compact_badge_labels,
        tactics_compact_badge_widths_px: tactics_compact_badge_widths,
        tactics_queue_fallback_values,
        tactics_queue_fallback_badge_labels,
        tactics_queue_fallback_badge_widths_px: tactics_queue_fallback_badge_widths,
        geometry: RtsFirstContactSidebarDensityGeometrySnapshot {
            production_to_palette_gap_px,
            build_palette_slot_width_px: CLASSIC_FIRST_CONTACT_BUILD_PALETTE_SLOT_W_PX,
            build_palette_slot_height_px: CLASSIC_FIRST_CONTACT_BUILD_PALETTE_SLOT_H_PX,
            build_palette_row_gap_px: CLASSIC_FIRST_CONTACT_BUILD_PALETTE_ROW_GAP_PX,
            build_palette_inter_row_gap_px: palette_row_gap_px,
            build_palette_to_tactics_gap_px: palette_to_tactics_gap_px,
            build_palette_state_badge_width_px:
                CLASSIC_FIRST_CONTACT_BUILD_PALETTE_STATE_BADGE_W_PX,
            build_palette_state_badge_height_px:
                CLASSIC_FIRST_CONTACT_BUILD_PALETTE_STATE_BADGE_H_PX,
            tactics_row_gap_px,
        },
    };
    trnm_rts_evidence::first_contact_sidebar_density_guard(&sidebar_runtime)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn first_contact_sidebar_density_helpers_preserve_right_rail_contracts() {
        let runtime = crate::classic_first_contact_player_screen_runtime();
        let profile = trnm_rts_data::first_contact_player_screen_profile();
        let guard = sidebar_density_guard(&runtime, &profile.chrome);

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
