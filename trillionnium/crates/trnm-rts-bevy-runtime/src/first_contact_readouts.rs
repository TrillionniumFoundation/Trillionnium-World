use trnm_rts_data::{
    RtsFirstContactPlayerScreenChromeProfile, RtsPlayerScreenResourceReadoutKind,
    RtsPlayerScreenResourceReadoutProfile, RtsPlayerScreenTacticsRowKind,
    RtsPlayerScreenTacticsRowProfile,
};

pub const TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_READOUT_SURFACE_CONTRACT: &str =
    "trnm_rts_bevy_runtime_first_contact_readout_surface_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RtsFirstContactReadoutRuntime<'a> {
    pub coins: u64,
    pub resource_spend_log: &'a [String],
    pub ai_pressure_percent: u8,
    pub army_supply_used: u8,
    pub army_supply_cap: u8,
    pub visibility_percent: u8,
    pub group_command_state: &'a str,
    pub attack_target_id: Option<&'a str>,
    pub camera_focus_tile_id: Option<&'a str>,
    pub minimap_command_tile_id: Option<&'a str>,
    pub production_queue: &'a [String],
    pub build_queue: &'a [String],
    pub training_progress_percent: u8,
    pub build_progress_percent: u8,
    pub building_blueprint_id: Option<&'a str>,
    pub building_progress_percent: u8,
    pub build_site_tile_ids: &'a [String],
    pub target_health_percent: u8,
    pub camera_zoom_percent: u8,
}

impl<'a> Default for RtsFirstContactReadoutRuntime<'a> {
    fn default() -> Self {
        Self {
            coins: 0,
            resource_spend_log: &[],
            ai_pressure_percent: 0,
            army_supply_used: 0,
            army_supply_cap: 0,
            visibility_percent: 0,
            group_command_state: "",
            attack_target_id: None,
            camera_focus_tile_id: None,
            minimap_command_tile_id: None,
            production_queue: &[],
            build_queue: &[],
            training_progress_percent: 0,
            build_progress_percent: 0,
            building_blueprint_id: None,
            building_progress_percent: 0,
            build_site_tile_ids: &[],
            target_health_percent: 0,
            camera_zoom_percent: 0,
        }
    }
}

pub fn rts_first_contact_resource_readout_value(
    runtime: &RtsFirstContactReadoutRuntime<'_>,
    readout: &RtsPlayerScreenResourceReadoutProfile,
) -> String {
    match readout.kind {
        RtsPlayerScreenResourceReadoutKind::Credits => {
            crate::rts_available_gold(runtime.coins, runtime.resource_spend_log).to_string()
        }
        RtsPlayerScreenResourceReadoutKind::Power => {
            let power = 100_i32.saturating_sub((runtime.ai_pressure_percent as i32 / 4).min(20));
            format!("{}%", power.max(0))
        }
        RtsPlayerScreenResourceReadoutKind::Supply => format!(
            "{}/{}",
            runtime.army_supply_used.max(1),
            runtime.army_supply_cap.max(runtime.army_supply_used.max(1))
        ),
        RtsPlayerScreenResourceReadoutKind::Visibility => {
            format!("{}%", runtime.visibility_percent)
        }
    }
}

pub fn rts_first_contact_target_label(target_id: &str) -> String {
    match target_id {
        "trnm.flux.beacon" | "flux.beacon" | "beacon" => "RELAY BEACON".to_string(),
        "trnm.flux.relay" | "flux.relay" | "relay" => "RELAY".to_string(),
        _ => rts_first_contact_order_subject_label(target_id),
    }
}

pub fn rts_first_contact_tactics_row_value(
    runtime: &RtsFirstContactReadoutRuntime<'_>,
    row: &RtsPlayerScreenTacticsRowProfile,
) -> String {
    let max_chars = usize::from(row.max_value_chars.max(1));
    match row.kind {
        RtsPlayerScreenTacticsRowKind::Order => {
            if runtime.group_command_state.is_empty() {
                row.empty_label.clone()
            } else {
                crate::rts_catalog_text_label(runtime.group_command_state, max_chars)
            }
        }
        RtsPlayerScreenTacticsRowKind::Target => runtime
            .attack_target_id
            .map(|target| {
                crate::rts_catalog_text_label(&rts_first_contact_target_label(target), max_chars)
            })
            .unwrap_or_else(|| row.empty_label.clone()),
        RtsPlayerScreenTacticsRowKind::Camera => runtime
            .camera_focus_tile_id
            .or(runtime.minimap_command_tile_id)
            .map(rts_first_contact_hud_tile_label)
            .unwrap_or_else(|| row.empty_label.clone()),
        RtsPlayerScreenTacticsRowKind::Queue => {
            let summary = crate::rts_sidebar_queue_summary(
                runtime.production_queue,
                runtime.build_queue,
                runtime.training_progress_percent,
                runtime.build_progress_percent,
            );
            if summary.is_empty() {
                row.empty_label.clone()
            } else {
                crate::rts_catalog_text_label(&summary, max_chars)
            }
        }
        RtsPlayerScreenTacticsRowKind::Build => runtime
            .building_blueprint_id
            .map(|id| {
                crate::rts_catalog_text_label(
                    &format!(
                        "{} {}%",
                        rts_first_contact_order_completion_subject_label(id),
                        runtime.building_progress_percent.min(100)
                    ),
                    max_chars,
                )
            })
            .unwrap_or_else(|| "IDLE".to_string()),
    }
}

pub fn rts_first_contact_target_callout_subject(
    runtime: &RtsFirstContactReadoutRuntime<'_>,
) -> String {
    let target_id = runtime.attack_target_id.unwrap_or("trnm.flux.beacon");
    let normalized = target_id.to_ascii_lowercase();
    if normalized.contains("beacon") {
        "BEACON".to_string()
    } else if normalized.contains("relay") {
        "RELAY".to_string()
    } else {
        rts_first_contact_live_subject_label(target_id, 8)
    }
}

pub fn rts_first_contact_target_callout_label(
    runtime: &RtsFirstContactReadoutRuntime<'_>,
) -> String {
    format!(
        "{} {}%",
        rts_first_contact_target_callout_subject(runtime),
        runtime.target_health_percent.min(100)
    )
}

pub fn rts_first_contact_tactical_status_label(
    runtime: &RtsFirstContactReadoutRuntime<'_>,
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> String {
    let status = if runtime.group_command_state.is_empty() {
        chrome.tactical_view_status_fallback.as_str()
    } else {
        runtime.group_command_state
    };
    crate::rts_catalog_text_label(
        &status.replace('_', " "),
        usize::from(chrome.tactical_view_status_max_chars.max(1)),
    )
}

pub fn rts_first_contact_tactical_header_title(
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> String {
    crate::rts_catalog_text_label(&chrome.tactical_view_title, 16)
}

pub fn rts_first_contact_tactical_header_order_label(
    runtime: &RtsFirstContactReadoutRuntime<'_>,
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> String {
    crate::rts_catalog_text_label(
        &rts_first_contact_tactical_status_label(runtime, chrome),
        24,
    )
}

pub fn rts_first_contact_tactical_header_camera_label(
    runtime: &RtsFirstContactReadoutRuntime<'_>,
    chrome: &RtsFirstContactPlayerScreenChromeProfile,
) -> String {
    let camera_tile = runtime
        .camera_focus_tile_id
        .map(rts_first_contact_hud_tile_label)
        .unwrap_or_else(|| {
            rts_first_contact_hud_tile_label(&crate::rts_first_contact_tile_id(
                chrome.tactical_view_default_camera_tile,
            ))
        });
    crate::rts_catalog_text_label(
        &format!(
            "{} {} {}{}",
            chrome.tactical_view_camera_prefix,
            camera_tile,
            chrome.tactical_view_zoom_prefix,
            runtime.camera_zoom_percent.max(1)
        ),
        16,
    )
}

pub fn rts_first_contact_build_placement_status_label(
    runtime: &RtsFirstContactReadoutRuntime<'_>,
) -> Option<String> {
    let blueprint_id = runtime.building_blueprint_id?;
    let primary_tile_id = runtime.build_site_tile_ids.first()?;
    let placement_queue_id = format!("build:{blueprint_id}@{primary_tile_id}");
    let subject = rts_first_contact_order_completion_subject_label(blueprint_id);
    let state = if crate::rts_queue_is_affordable(
        runtime.coins,
        runtime.resource_spend_log,
        &placement_queue_id,
    ) {
        "READY"
    } else {
        "LOW CRED"
    };
    Some(crate::rts_catalog_text_label(
        &format!(
            "PLACE {} {}C {}",
            subject,
            crate::rts_queue_gold_cost(&placement_queue_id),
            state
        ),
        28,
    ))
}

pub fn rts_first_contact_hud_tile_label(tile_id: &str) -> String {
    parse_tile_id(tile_id)
        .map(|(x, y)| format!("{x}/{y}"))
        .unwrap_or_else(|| tile_id.replace(',', "/"))
}

fn rts_first_contact_order_subject_label(subject: &str) -> String {
    let subject = subject.strip_prefix("trnm.").unwrap_or(subject);
    let subject = subject.strip_prefix("flux.").unwrap_or(subject);
    crate::rts_catalog_text_label(&subject.replace(['_', '.', ':', '-'], " "), 18)
}

fn rts_first_contact_order_completion_subject_label(subject: &str) -> String {
    let subject = subject.split("->").next().unwrap_or(subject);
    let subject = subject.split('@').next().unwrap_or(subject);
    let subject = subject
        .strip_prefix("train:")
        .or_else(|| subject.strip_prefix("build:"))
        .or_else(|| subject.strip_prefix("upgrade:"))
        .unwrap_or(subject);
    let subject = subject.strip_prefix("trnm.").unwrap_or(subject);
    match subject {
        "guard" => "GUARD".to_string(),
        "worker" => "WORKER".to_string(),
        "signal_blade" => "SIGNAL".to_string(),
        "training_hall" => "TRAINING".to_string(),
        "watch_tower" => "TOWER".to_string(),
        "power_node" => "POWER".to_string(),
        "refinery" => "REFINE".to_string(),
        "command_post" => "COMMAND".to_string(),
        "radar_spire" => "RADAR".to_string(),
        "wall" => "WALL".to_string(),
        _ => rts_first_contact_order_subject_label(subject),
    }
}

fn rts_first_contact_live_subject_label(subject: &str, max_chars: usize) -> String {
    let subject = subject.split("->").next().unwrap_or(subject);
    let subject = subject.split('@').next().unwrap_or(subject);
    let subject = subject
        .strip_prefix("train:")
        .or_else(|| subject.strip_prefix("build:"))
        .or_else(|| subject.strip_prefix("upgrade:"))
        .or_else(|| subject.strip_prefix("attack:"))
        .or_else(|| subject.strip_prefix("objective:claim:"))
        .or_else(|| subject.strip_prefix("objective:extract:"))
        .unwrap_or(subject);
    let subject = subject.strip_prefix("trnm.").unwrap_or(subject);
    let normalized = subject.to_ascii_lowercase();
    let label = if normalized.contains("flux.beacon")
        || normalized.contains("relay_beacon")
        || normalized == "beacon"
    {
        "RELAY BEACON".to_string()
    } else if normalized.contains("flux.relay") || normalized.contains("relay_outpost") {
        "RELAY".to_string()
    } else if normalized.contains("ai_skirmish") || normalized.contains("skirmish_wave") {
        "SKIRMISH".to_string()
    } else if normalized.contains("ridge_sentries") {
        "RIDGE SENTRIES".to_string()
    } else if normalized.contains("scout") {
        "SCOUT CREW".to_string()
    } else if normalized.contains("tier_two") || normalized.contains("tier2") {
        "TIER2".to_string()
    } else if normalized.contains("open_world") || normalized.contains("resume") {
        "RESUME".to_string()
    } else {
        rts_first_contact_order_completion_subject_label(subject)
    };
    crate::rts_catalog_text_label(&label, max_chars)
}

fn parse_tile_id(value: &str) -> Option<(i32, i32)> {
    let (x, y) = value.split_once(',')?;
    Some((x.parse().ok()?, y.parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use trnm_rts_data::first_contact_player_screen_profile;

    #[test]
    fn first_contact_readout_surface_preserves_player_facing_labels() {
        let profile = first_contact_player_screen_profile();
        let production_queue = vec!["train:worker".to_string()];
        let build_queue = vec!["build:watch_tower@7,4".to_string()];
        let build_site_tile_ids = vec!["7,4".to_string()];

        let runtime = RtsFirstContactReadoutRuntime {
            coins: 1_000,
            attack_target_id: Some("trnm.flux.beacon"),
            group_command_state: "secure_relay_beacon",
            target_health_percent: 38,
            camera_focus_tile_id: Some("16,16"),
            camera_zoom_percent: 100,
            production_queue: &production_queue,
            build_queue: &build_queue,
            training_progress_percent: 64,
            build_progress_percent: 42,
            army_supply_used: 12,
            army_supply_cap: 22,
            visibility_percent: 76,
            building_blueprint_id: Some("watch_tower"),
            building_progress_percent: 42,
            build_site_tile_ids: &build_site_tile_ids,
            ..Default::default()
        };

        assert_eq!(
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_READOUT_SURFACE_CONTRACT,
            "trnm_rts_bevy_runtime_first_contact_readout_surface_v1"
        );
        assert_eq!(
            rts_first_contact_target_label("trnm.flux.beacon"),
            "RELAY BEACON"
        );
        assert_eq!(rts_first_contact_target_callout_subject(&runtime), "BEACON");
        assert_eq!(
            rts_first_contact_target_callout_label(&runtime),
            "BEACON 38%"
        );
        assert_eq!(
            rts_first_contact_tactical_header_order_label(
                &RtsFirstContactReadoutRuntime::default(),
                &profile.chrome
            ),
            "GROUP 1 ATTACK QUEUED"
        );
        assert_eq!(
            rts_first_contact_tactical_header_title(&profile.chrome),
            "TACTICAL VIEW"
        );
        assert_eq!(
            rts_first_contact_tactical_header_order_label(&runtime, &profile.chrome),
            "SECURE RELAY BEACON"
        );
        assert_eq!(
            rts_first_contact_tactical_header_camera_label(&runtime, &profile.chrome),
            "CAM 16/16 Z100"
        );

        let queue_row = profile
            .chrome
            .tactics_rows
            .iter()
            .find(|row| row.kind == RtsPlayerScreenTacticsRowKind::Queue)
            .expect("queue tactics row");
        assert_eq!(
            rts_first_contact_tactics_row_value(&runtime, queue_row),
            "WORKER 64% TOWER 42%"
        );
        assert_eq!(
            rts_first_contact_build_placement_status_label(&runtime),
            Some("PLACE TOWER 210C READY".to_string())
        );
        assert_eq!(rts_first_contact_hud_tile_label("16,9"), "16/9");
    }
}
