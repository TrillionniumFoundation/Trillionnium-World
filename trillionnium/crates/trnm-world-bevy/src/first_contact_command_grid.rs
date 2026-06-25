#![cfg(not(target_os = "android"))]

use trnm_rts_data::RtsFirstContactPlayerScreenChromeProfile;

pub(crate) fn command_glyph_role(ability: &str) -> &'static str {
    trnm_rts_bevy_runtime::rts_first_contact_command_glyph_role(ability)
}

pub(crate) fn command_glyph_signature(role: &str) -> &'static str {
    trnm_rts_bevy_runtime::rts_first_contact_command_glyph_signature(role)
}

pub(crate) fn command_role_backdrop(role: &str) -> u32 {
    match role {
        "worker" => 0x2d3b24,
        "scout" => 0x213743,
        "warden" => 0x352d3f,
        "relay" => 0x24383a,
        "core" => 0x3a3325,
        "signal" => 0x2e2944,
        "attack" => 0x3f2626,
        _ => 0x263b2e,
    }
}

pub(crate) fn command_grid_slot_ids(
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> Vec<String> {
    trnm_rts_bevy_runtime::rts_first_contact_command_grid_slot_ids(chrome)
}

pub(crate) fn command_slot_sent(command_queue: &[String], ability: &str, role: &str) -> bool {
    trnm_rts_bevy_runtime::rts_first_contact_command_slot_sent(command_queue, ability, role)
}

#[cfg(test)]
mod tests {
    use super::*;
    use trnm_rts_data::first_contact_player_screen_profile;

    #[test]
    fn first_contact_command_grid_helpers_preserve_roles_and_slots() {
        let profile = first_contact_player_screen_profile();

        assert_eq!(
            command_grid_slot_ids(&profile.chrome),
            vec![
                "worker", "scout", "warden", "relay", "core", "signal", "worker", "scout",
                "warden", "relay", "core", "signal"
            ]
        );
        assert_eq!(command_glyph_role("harvest-worker"), "worker");
        assert_eq!(command_glyph_role("recon_scout"), "scout");
        assert_eq!(command_glyph_role("guard-hold"), "warden");
        assert_eq!(command_glyph_role("rally-relay"), "relay");
        assert_eq!(command_glyph_role("build-core"), "core");
        assert_eq!(command_glyph_role("focus-signal"), "signal");
        assert_eq!(command_glyph_role("attack-strike"), "attack");
        assert_eq!(command_glyph_role("unknown"), "generic");
        assert_eq!(command_glyph_signature("signal"), "pulse_spire");
        assert_eq!(command_glyph_signature("generic"), "fallback_diamond");
        assert_eq!(command_role_backdrop("worker"), 0x2d3b24);
        assert_eq!(command_role_backdrop("generic"), 0x263b2e);

        let queue = vec![
            "ability:focus-signal".to_string(),
            "waypoint:16,9".to_string(),
            "control_group:1".to_string(),
            "build:relay".to_string(),
            "harvest:flux".to_string(),
            "recon:east".to_string(),
        ];
        assert!(command_slot_sent(&queue, "signal", "signal"));
        assert!(command_slot_sent(&queue, "relay", "relay"));
        assert!(command_slot_sent(&queue, "warden", "warden"));
        assert!(command_slot_sent(&queue, "core", "core"));
        assert!(command_slot_sent(&queue, "worker", "worker"));
        assert!(command_slot_sent(&queue, "scout", "scout"));
        assert!(!command_slot_sent(&queue, "attack", "attack"));
    }
}
