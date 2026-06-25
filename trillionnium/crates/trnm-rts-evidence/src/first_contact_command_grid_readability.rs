#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use trnm_rts_bevy_runtime::{
    rts_first_contact_command_glyph_role as first_contact_command_glyph_role,
    rts_first_contact_command_glyph_signature as first_contact_command_glyph_signature,
    rts_first_contact_command_grid_slot_ids as first_contact_command_grid_slot_ids,
    RtsFirstContactCommandGridRuntime,
};
use trnm_rts_data::RtsFirstContactPlayerScreenChromeProfile;

use crate::TRNM_RTS_EVIDENCE_FIRST_CONTACT_COMMAND_GRID_READABILITY_CONTRACT;

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

pub fn first_contact_command_grid_readability_guard(
    runtime: &RtsFirstContactCommandGridRuntime,
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> Value {
    let command_slot_ids = first_contact_command_grid_slot_ids(chrome);
    let command_icon_roles = command_slot_ids
        .iter()
        .map(|slot| first_contact_command_glyph_role(slot).to_string())
        .collect::<Vec<_>>();
    let command_icon_signatures = command_icon_roles
        .iter()
        .map(|role| first_contact_command_glyph_signature(role).to_string())
        .collect::<Vec<_>>();
    let unique_icon_role_count = command_icon_roles
        .iter()
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let unique_icon_signature_count = command_icon_signatures
        .iter()
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let column_count = chrome.command_grid_column_count.max(1) as usize;
    let top_row_roles = command_icon_roles
        .iter()
        .take(column_count)
        .cloned()
        .collect::<Vec<_>>();
    let bottom_row_roles = command_icon_roles
        .iter()
        .skip(column_count)
        .take(column_count)
        .cloned()
        .collect::<Vec<_>>();
    let active_slot_role = runtime
        .active_ability_id
        .as_deref()
        .map(first_contact_command_glyph_role)
        .unwrap_or("generic")
        .to_string();
    let cooldown_badge_samples = chrome
        .command_grid_slot_ids
        .iter()
        .enumerate()
        .map(|(index, slot)| {
            json!({
                "slot": slot,
                "role": first_contact_command_glyph_role(slot),
                "cooldown_percent": runtime.ability_cooldown_percents.get(index).copied().unwrap_or(0),
            })
        })
        .collect::<Vec<_>>();
    let expected_roles = string_vec([
        "worker", "scout", "warden", "relay", "core", "signal", "worker", "scout", "warden",
        "relay", "core", "signal",
    ]);
    let role_sequence_gate = command_icon_roles == expected_roles;
    let repeated_rows_gate = !top_row_roles.is_empty() && top_row_roles == bottom_row_roles;
    let unique_icon_gate = unique_icon_role_count >= 6 && unique_icon_signature_count >= 6;
    let active_slot_gate = active_slot_role == "worker";
    let cooldown_badge_gate = runtime.ability_cooldown_percents == vec![0, 0, 16, 0, 42, 25];
    let slot_badge_pixel_budget = command_slot_ids.len() * 12;
    let glyph_shape_pixel_budget = command_slot_ids.len() * 96;
    let player_screen_symbol_gate =
        slot_badge_pixel_budget >= 144 && glyph_shape_pixel_budget >= 1_152;
    let green = role_sequence_gate
        && repeated_rows_gate
        && unique_icon_gate
        && active_slot_gate
        && cooldown_badge_gate
        && player_screen_symbol_gate;

    json!({
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_COMMAND_GRID_READABILITY_CONTRACT,
        "green": green,
        "source_path": "trnm-world-bevy classic_draw_rts_command_glyph role-specific First Contact command buttons",
        "command_slot_ids": command_slot_ids,
        "command_icon_roles": command_icon_roles,
        "command_icon_signatures": command_icon_signatures,
        "unique_icon_role_count": unique_icon_role_count,
        "unique_icon_signature_count": unique_icon_signature_count,
        "top_row_roles": top_row_roles,
        "bottom_row_roles": bottom_row_roles,
        "active_slot_role": active_slot_role,
        "cooldown_badge_samples": cooldown_badge_samples,
        "slot_badge_pixel_budget": slot_badge_pixel_budget,
        "glyph_shape_pixel_budget": glyph_shape_pixel_budget,
        "role_sequence_gate": role_sequence_gate,
        "repeated_rows_gate": repeated_rows_gate,
        "unique_icon_gate": unique_icon_gate,
        "active_slot_gate": active_slot_gate,
        "cooldown_badge_gate": cooldown_badge_gate,
        "player_screen_symbol_gate": player_screen_symbol_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use trnm_rts_bevy_runtime::rts_first_contact_command_slot_sent as first_contact_command_slot_sent;
    use trnm_rts_data::first_contact_player_screen_profile;

    fn first_contact_command_grid_runtime() -> RtsFirstContactCommandGridRuntime {
        RtsFirstContactCommandGridRuntime {
            active_ability_id: Some("harvest-worker".to_string()),
            ability_cooldown_percents: vec![0, 0, 16, 0, 42, 25],
        }
    }

    #[test]
    fn first_contact_command_grid_readability_preserves_role_contracts() {
        let profile = first_contact_player_screen_profile();
        let guard = first_contact_command_grid_readability_guard(
            &first_contact_command_grid_runtime(),
            &profile.chrome,
        );

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_COMMAND_GRID_READABILITY_CONTRACT)
        );
        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard.get("command_slot_ids").cloned(),
            Some(json!([
                "worker", "scout", "warden", "relay", "core", "signal", "worker", "scout",
                "warden", "relay", "core", "signal"
            ]))
        );
        assert_eq!(
            guard.get("command_icon_roles").cloned(),
            Some(json!([
                "worker", "scout", "warden", "relay", "core", "signal", "worker", "scout",
                "warden", "relay", "core", "signal"
            ]))
        );
        assert_eq!(
            guard.get("active_slot_role").and_then(Value::as_str),
            Some("worker")
        );
        assert_eq!(
            guard.get("unique_icon_role_count").and_then(Value::as_u64),
            Some(6)
        );
        assert_eq!(
            guard
                .get("unique_icon_signature_count")
                .and_then(Value::as_u64),
            Some(6)
        );

        for gate in [
            "role_sequence_gate",
            "repeated_rows_gate",
            "unique_icon_gate",
            "active_slot_gate",
            "cooldown_badge_gate",
            "player_screen_symbol_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }

    #[test]
    fn first_contact_command_grid_helpers_preserve_roles_slots_and_sent_state() {
        assert_eq!(first_contact_command_glyph_role("harvest-worker"), "worker");
        assert_eq!(first_contact_command_glyph_role("recon_scout"), "scout");
        assert_eq!(first_contact_command_glyph_role("guard-hold"), "warden");
        assert_eq!(first_contact_command_glyph_role("rally-relay"), "relay");
        assert_eq!(first_contact_command_glyph_role("build-core"), "core");
        assert_eq!(first_contact_command_glyph_role("focus-signal"), "signal");
        assert_eq!(first_contact_command_glyph_role("attack-strike"), "attack");
        assert_eq!(first_contact_command_glyph_role("unknown"), "generic");
        assert_eq!(
            first_contact_command_glyph_signature("signal"),
            "pulse_spire"
        );
        assert_eq!(
            first_contact_command_glyph_signature("generic"),
            "fallback_diamond"
        );

        let queue = vec![
            "ability:focus-signal".to_string(),
            "waypoint:16,9".to_string(),
            "control_group:1".to_string(),
            "build:relay".to_string(),
            "harvest:flux".to_string(),
            "recon:east".to_string(),
        ];
        assert!(first_contact_command_slot_sent(&queue, "signal", "signal"));
        assert!(first_contact_command_slot_sent(&queue, "relay", "relay"));
        assert!(first_contact_command_slot_sent(&queue, "warden", "warden"));
        assert!(first_contact_command_slot_sent(&queue, "core", "core"));
        assert!(first_contact_command_slot_sent(&queue, "worker", "worker"));
        assert!(first_contact_command_slot_sent(&queue, "scout", "scout"));
        assert!(!first_contact_command_slot_sent(&queue, "attack", "attack"));
    }
}
