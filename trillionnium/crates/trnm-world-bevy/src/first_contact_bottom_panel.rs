#![cfg(not(target_os = "android"))]

use crate::{
    NativeFirstPlayableRuntime, CLASSIC_HUD_MUTED_TEXT_COLOR, CLASSIC_ISO_UNIT_GUARD_COLOR,
    CLASSIC_ISO_UNIT_PLAYER_COLOR, CLASSIC_RTS_COMMANDER_AURA_COLOR,
    CLASSIC_RTS_HARVEST_NODE_COLOR, CLASSIC_RTS_PRODUCT_UI_ACCENT_COLOR,
    CLASSIC_RTS_SCOUT_REVEAL_COLOR,
};

pub(crate) fn feedback_label(feedback: &str, max_chars: usize) -> String {
    trnm_rts_evidence::first_contact_bottom_panel_feedback_label(feedback, max_chars)
}

pub(crate) fn squad_roles(
    runtime: &NativeFirstPlayableRuntime,
    selected_unit_display_count: usize,
) -> Vec<String> {
    trnm_rts_evidence::first_contact_bottom_panel_squad_roles(
        &runtime.rts_selected_unit_ids,
        selected_unit_display_count,
    )
}

pub(crate) fn role_color(role: &str) -> u32 {
    match role {
        "LEAD" => CLASSIC_ISO_UNIT_PLAYER_COLOR,
        "GUARD" => CLASSIC_ISO_UNIT_GUARD_COLOR,
        "WORKER" => CLASSIC_RTS_HARVEST_NODE_COLOR,
        "SCOUT" => CLASSIC_RTS_SCOUT_REVEAL_COLOR,
        "RELAY" => CLASSIC_RTS_COMMANDER_AURA_COLOR,
        "SIGNAL" => CLASSIC_RTS_PRODUCT_UI_ACCENT_COLOR,
        _ => CLASSIC_HUD_MUTED_TEXT_COLOR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_bottom_panel_helpers_preserve_feedback_roles_and_colors() {
        let runtime = NativeFirstPlayableRuntime {
            rts_selected_unit_ids: vec![
                "worker_03".to_string(),
                "horizon_scout".to_string(),
                "forge_warden".to_string(),
                "flux_relay".to_string(),
            ],
            ..Default::default()
        };

        assert_eq!(
            feedback_label("RTS UPGRADE COMPLETE: SIGNAL BLADE", 62),
            "SIGNAL BLADE READY"
        );
        assert_eq!(
            feedback_label("RTS BUILD COMPLETE: build:watch_tower@7,4->watch_tower", 62),
            "WATCH TOWER READY"
        );
        assert_eq!(
            feedback_label("RTS GROUP 1 SECURING RELAY", 62),
            "GROUP 1 SECURING RELAY"
        );
        assert_eq!(
            squad_roles(&runtime, 4),
            vec!["WORKER", "SCOUT", "GUARD", "RELAY"]
        );
        assert_eq!(role_color("LEAD"), CLASSIC_ISO_UNIT_PLAYER_COLOR);
        assert_eq!(role_color("SIGNAL"), CLASSIC_RTS_PRODUCT_UI_ACCENT_COLOR);
        assert_eq!(role_color("UNKNOWN"), CLASSIC_HUD_MUTED_TEXT_COLOR);
    }
}
