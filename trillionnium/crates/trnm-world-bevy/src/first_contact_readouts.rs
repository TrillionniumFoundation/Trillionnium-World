#![cfg(not(target_os = "android"))]

use crate::{
    NativeFirstPlayableRuntime, CLASSIC_ISO_CONTROL_GROUP_COLOR, CLASSIC_ISO_GOLD_COLOR,
    CLASSIC_RTS_BUILD_PROGRESS_COLOR, CLASSIC_RTS_CAMERA_SYNC_VIEWPORT_COLOR,
    CLASSIC_RTS_MINIMAP_VISION_COLOR, CLASSIC_RTS_QUEUE_PREVIEW_SLOT_COLOR,
    CLASSIC_RTS_RESOURCE_FOOD_COLOR, CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
    CLASSIC_RTS_VISIBILITY_BAR_COLOR,
};
use trnm_rts_bevy_runtime::RtsFirstContactReadoutRuntime;
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
    trnm_rts_bevy_runtime::rts_first_contact_resource_readout_value(
        &readout_runtime(runtime),
        readout,
    )
}

#[cfg(test)]
pub(crate) fn target_label(target_id: &str) -> String {
    trnm_rts_bevy_runtime::rts_first_contact_target_label(target_id)
}

pub(crate) fn tactics_row_value(
    runtime: &NativeFirstPlayableRuntime,
    row: &RtsPlayerScreenTacticsRowProfile,
) -> String {
    trnm_rts_bevy_runtime::rts_first_contact_tactics_row_value(&readout_runtime(runtime), row)
}

pub(crate) fn target_callout_subject(runtime: &NativeFirstPlayableRuntime) -> String {
    trnm_rts_bevy_runtime::rts_first_contact_target_callout_subject(&readout_runtime(runtime))
}

pub(crate) fn target_callout_label(runtime: &NativeFirstPlayableRuntime) -> String {
    trnm_rts_bevy_runtime::rts_first_contact_target_callout_label(&readout_runtime(runtime))
}

pub(crate) fn tactical_header_title(chrome: &RtsFirstContactPlayerScreenChromeProfile) -> String {
    trnm_rts_bevy_runtime::rts_first_contact_tactical_header_title(chrome)
}

pub(crate) fn tactical_header_order_label(
    runtime: &NativeFirstPlayableRuntime,
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> String {
    trnm_rts_bevy_runtime::rts_first_contact_tactical_header_order_label(
        &readout_runtime(runtime),
        chrome,
    )
}

pub(crate) fn tactical_header_camera_label(
    runtime: &NativeFirstPlayableRuntime,
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> String {
    trnm_rts_bevy_runtime::rts_first_contact_tactical_header_camera_label(
        &readout_runtime(runtime),
        chrome,
    )
}

pub(crate) fn build_placement_status_label(runtime: &NativeFirstPlayableRuntime) -> Option<String> {
    trnm_rts_bevy_runtime::rts_first_contact_build_placement_status_label(&readout_runtime(runtime))
}

fn readout_runtime(runtime: &NativeFirstPlayableRuntime) -> RtsFirstContactReadoutRuntime<'_> {
    RtsFirstContactReadoutRuntime {
        coins: runtime.coins,
        resource_spend_log: &runtime.rts_resource_spend_log,
        ai_pressure_percent: runtime.rts_ai_pressure_percent,
        army_supply_used: runtime.rts_army_supply_used,
        army_supply_cap: runtime.rts_army_supply_cap,
        visibility_percent: runtime.rts_visibility_percent,
        group_command_state: &runtime.rts_group_command_state,
        attack_target_id: runtime.rts_attack_target_id.as_deref(),
        camera_focus_tile_id: runtime.rts_camera_focus_tile_id.as_deref(),
        minimap_command_tile_id: runtime.rts_minimap_command_tile_id.as_deref(),
        production_queue: &runtime.rts_production_queue,
        build_queue: &runtime.rts_build_queue,
        training_progress_percent: runtime.rts_training_progress_percent,
        build_progress_percent: runtime.rts_build_progress_percent,
        building_blueprint_id: runtime.rts_building_blueprint_id.as_deref(),
        building_progress_percent: runtime.rts_building_progress_percent,
        build_site_tile_ids: &runtime.rts_build_site_tile_ids,
        target_health_percent: runtime.rts_target_health_percent,
        camera_zoom_percent: runtime.rts_camera_zoom_percent,
    }
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
