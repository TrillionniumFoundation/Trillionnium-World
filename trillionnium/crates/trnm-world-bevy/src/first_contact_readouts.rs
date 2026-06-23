#![cfg(not(target_os = "android"))]

use crate::{
    classic_catalog_text_label, classic_first_contact_tile_id, classic_hud_tile_label,
    classic_rts_available_gold, classic_rts_live_subject_label,
    classic_rts_order_completion_subject_label, classic_rts_order_subject_label,
    classic_rts_queue_gold_cost, classic_rts_queue_is_affordable,
    classic_rts_sidebar_queue_summary, NativeFirstPlayableRuntime, CLASSIC_ISO_CONTROL_GROUP_COLOR,
    CLASSIC_ISO_GOLD_COLOR, CLASSIC_RTS_BUILD_PROGRESS_COLOR,
    CLASSIC_RTS_CAMERA_SYNC_VIEWPORT_COLOR, CLASSIC_RTS_MINIMAP_VISION_COLOR,
    CLASSIC_RTS_QUEUE_PREVIEW_SLOT_COLOR, CLASSIC_RTS_RESOURCE_FOOD_COLOR,
    CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR, CLASSIC_RTS_VISIBILITY_BAR_COLOR,
};
use trnm_rts_data::{
    RtsFirstContactPlayerScreenChromeProfile, RtsPlayerScreenResourceReadoutKind,
    RtsPlayerScreenResourceReadoutProfile, RtsPlayerScreenTacticsRowKind,
    RtsPlayerScreenTacticsRowProfile,
};

pub(crate) fn tactics_row_color(kind: RtsPlayerScreenTacticsRowKind) -> u32 {
    match kind {
        RtsPlayerScreenTacticsRowKind::Order => CLASSIC_ISO_CONTROL_GROUP_COLOR,
        RtsPlayerScreenTacticsRowKind::Target => CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
        RtsPlayerScreenTacticsRowKind::Camera => CLASSIC_RTS_CAMERA_SYNC_VIEWPORT_COLOR,
        RtsPlayerScreenTacticsRowKind::Queue => CLASSIC_RTS_QUEUE_PREVIEW_SLOT_COLOR,
        RtsPlayerScreenTacticsRowKind::Build => CLASSIC_RTS_BUILD_PROGRESS_COLOR,
    }
}

pub(crate) fn resource_readout_color(kind: RtsPlayerScreenResourceReadoutKind) -> u32 {
    match kind {
        RtsPlayerScreenResourceReadoutKind::Credits => CLASSIC_ISO_GOLD_COLOR,
        RtsPlayerScreenResourceReadoutKind::Power => CLASSIC_RTS_VISIBILITY_BAR_COLOR,
        RtsPlayerScreenResourceReadoutKind::Supply => CLASSIC_RTS_RESOURCE_FOOD_COLOR,
        RtsPlayerScreenResourceReadoutKind::Visibility => CLASSIC_RTS_MINIMAP_VISION_COLOR,
    }
}

pub(crate) fn resource_readout_value(
    runtime: &NativeFirstPlayableRuntime,
    readout: &RtsPlayerScreenResourceReadoutProfile,
) -> String {
    match readout.kind {
        RtsPlayerScreenResourceReadoutKind::Credits => {
            classic_rts_available_gold(runtime).to_string()
        }
        RtsPlayerScreenResourceReadoutKind::Power => {
            let power =
                100_i32.saturating_sub((runtime.rts_ai_pressure_percent as i32 / 4).min(20));
            format!("{}%", power.max(0))
        }
        RtsPlayerScreenResourceReadoutKind::Supply => format!(
            "{}/{}",
            runtime.rts_army_supply_used.max(1),
            runtime
                .rts_army_supply_cap
                .max(runtime.rts_army_supply_used.max(1))
        ),
        RtsPlayerScreenResourceReadoutKind::Visibility => {
            format!("{}%", runtime.rts_visibility_percent)
        }
    }
}

pub(crate) fn target_label(target_id: &str) -> String {
    match target_id {
        "trnm.flux.beacon" | "flux.beacon" | "beacon" => "RELAY BEACON".to_string(),
        "trnm.flux.relay" | "flux.relay" | "relay" => "RELAY".to_string(),
        _ => classic_rts_order_subject_label(target_id),
    }
}

pub(crate) fn tactics_row_value(
    runtime: &NativeFirstPlayableRuntime,
    row: &RtsPlayerScreenTacticsRowProfile,
) -> String {
    let max_chars = usize::from(row.max_value_chars.max(1));
    match row.kind {
        RtsPlayerScreenTacticsRowKind::Order => {
            if runtime.rts_group_command_state.is_empty() {
                row.empty_label.clone()
            } else {
                classic_catalog_text_label(&runtime.rts_group_command_state, max_chars)
            }
        }
        RtsPlayerScreenTacticsRowKind::Target => runtime
            .rts_attack_target_id
            .as_deref()
            .map(|target| classic_catalog_text_label(&target_label(target), max_chars))
            .unwrap_or_else(|| row.empty_label.clone()),
        RtsPlayerScreenTacticsRowKind::Camera => runtime
            .rts_camera_focus_tile_id
            .as_deref()
            .or(runtime.rts_minimap_command_tile_id.as_deref())
            .map(classic_hud_tile_label)
            .unwrap_or_else(|| row.empty_label.clone()),
        RtsPlayerScreenTacticsRowKind::Queue => {
            let summary = classic_rts_sidebar_queue_summary(runtime);
            if summary.is_empty() {
                row.empty_label.clone()
            } else {
                classic_catalog_text_label(&summary, max_chars)
            }
        }
        RtsPlayerScreenTacticsRowKind::Build => runtime
            .rts_building_blueprint_id
            .as_deref()
            .map(|id| {
                classic_catalog_text_label(
                    &format!(
                        "{} {}%",
                        classic_rts_order_completion_subject_label(id),
                        runtime.rts_building_progress_percent.min(100)
                    ),
                    max_chars,
                )
            })
            .unwrap_or_else(|| "IDLE".to_string()),
    }
}

pub(crate) fn target_callout_subject(runtime: &NativeFirstPlayableRuntime) -> String {
    let target_id = runtime
        .rts_attack_target_id
        .as_deref()
        .unwrap_or("trnm.flux.beacon");
    let normalized = target_id.to_ascii_lowercase();
    if normalized.contains("beacon") {
        "BEACON".to_string()
    } else if normalized.contains("relay") {
        "RELAY".to_string()
    } else {
        classic_rts_live_subject_label(target_id, 8)
    }
}

pub(crate) fn target_callout_label(runtime: &NativeFirstPlayableRuntime) -> String {
    format!(
        "{} {}%",
        target_callout_subject(runtime),
        runtime.rts_target_health_percent.min(100)
    )
}

pub(crate) fn tactical_status_label(
    runtime: &NativeFirstPlayableRuntime,
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> String {
    let status = if runtime.rts_group_command_state.is_empty() {
        chrome.tactical_view_status_fallback.clone()
    } else {
        runtime.rts_group_command_state.replace('_', " ")
    };
    classic_catalog_text_label(
        &status,
        usize::from(chrome.tactical_view_status_max_chars.max(1)),
    )
}

pub(crate) fn tactical_header_title(chrome: &RtsFirstContactPlayerScreenChromeProfile) -> String {
    classic_catalog_text_label(&chrome.tactical_view_title, 16)
}

pub(crate) fn tactical_header_order_label(
    runtime: &NativeFirstPlayableRuntime,
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> String {
    classic_catalog_text_label(&tactical_status_label(runtime, chrome), 24)
}

pub(crate) fn tactical_header_camera_label(
    runtime: &NativeFirstPlayableRuntime,
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> String {
    let camera_tile = runtime
        .rts_camera_focus_tile_id
        .as_deref()
        .map(classic_hud_tile_label)
        .unwrap_or_else(|| {
            classic_hud_tile_label(&classic_first_contact_tile_id(
                chrome.tactical_view_default_camera_tile,
            ))
        });
    classic_catalog_text_label(
        &format!(
            "{} {} {}{}",
            chrome.tactical_view_camera_prefix,
            camera_tile,
            chrome.tactical_view_zoom_prefix,
            runtime.rts_camera_zoom_percent.max(1)
        ),
        16,
    )
}

pub(crate) fn build_placement_status_label(runtime: &NativeFirstPlayableRuntime) -> Option<String> {
    let blueprint_id = runtime.rts_building_blueprint_id.as_deref()?;
    let primary_tile_id = runtime.rts_build_site_tile_ids.first()?;
    let placement_queue_id = format!("build:{blueprint_id}@{primary_tile_id}");
    let subject = classic_rts_order_completion_subject_label(blueprint_id);
    let state = if classic_rts_queue_is_affordable(runtime, &placement_queue_id) {
        "READY"
    } else {
        "LOW CRED"
    };
    Some(classic_catalog_text_label(
        &format!(
            "PLACE {} {}C {}",
            subject,
            classic_rts_queue_gold_cost(&placement_queue_id),
            state
        ),
        28,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use trnm_rts_data::first_contact_player_screen_profile;

    #[test]
    fn first_contact_readouts_preserve_player_facing_labels() {
        let profile = first_contact_player_screen_profile();
        let mut runtime = NativeFirstPlayableRuntime {
            rts_attack_target_id: Some("trnm.flux.beacon".to_string()),
            rts_group_command_state: "secure_relay_beacon".to_string(),
            rts_target_health_percent: 38,
            rts_camera_focus_tile_id: Some("16,16".to_string()),
            rts_camera_zoom_percent: 100,
            rts_production_queue: vec!["train:worker".to_string()],
            rts_build_queue: vec!["build:watch_tower@7,4".to_string()],
            rts_training_progress_percent: 64,
            rts_build_progress_percent: 42,
            rts_army_supply_used: 12,
            rts_army_supply_cap: 22,
            rts_visibility_percent: 76,
            ..Default::default()
        };

        assert_eq!(target_label("trnm.flux.beacon"), "RELAY BEACON");
        assert_eq!(target_callout_subject(&runtime), "BEACON");
        assert_eq!(target_callout_label(&runtime), "BEACON 38%");
        assert_eq!(
            tactical_header_order_label(&NativeFirstPlayableRuntime::default(), &profile.chrome),
            "GROUP 1 ATTACK QUEUED"
        );
        assert_eq!(tactical_header_title(&profile.chrome), "TACTICAL VIEW");
        assert_eq!(
            tactical_header_order_label(&runtime, &profile.chrome),
            "SECURE RELAY BEACON"
        );
        assert_eq!(
            tactical_header_camera_label(&runtime, &profile.chrome),
            "CAM 16/16 Z100"
        );

        let queue_row = profile
            .chrome
            .tactics_rows
            .iter()
            .find(|row| row.kind == RtsPlayerScreenTacticsRowKind::Queue)
            .expect("queue tactics row");
        assert_eq!(
            tactics_row_value(&runtime, queue_row),
            "WORKER 64% TOWER 42%"
        );

        runtime.rts_building_blueprint_id = Some("watch_tower".to_string());
        runtime.rts_build_site_tile_ids = vec!["7,4".to_string()];
        runtime.coins = 1_000;
        assert_eq!(
            build_placement_status_label(&runtime),
            Some("PLACE TOWER 210C READY".to_string())
        );
    }
}
