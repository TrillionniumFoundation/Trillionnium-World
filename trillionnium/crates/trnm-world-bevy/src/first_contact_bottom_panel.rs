#![cfg(not(target_os = "android"))]

use crate::{
    classic_catalog_text_label, classic_rts_order_completion_subject_label,
    NativeFirstPlayableRuntime, CLASSIC_HUD_MUTED_TEXT_COLOR, CLASSIC_ISO_UNIT_GUARD_COLOR,
    CLASSIC_ISO_UNIT_PLAYER_COLOR, CLASSIC_RTS_COMMANDER_AURA_COLOR,
    CLASSIC_RTS_HARVEST_NODE_COLOR, CLASSIC_RTS_PRODUCT_UI_ACCENT_COLOR,
    CLASSIC_RTS_SCOUT_REVEAL_COLOR,
};

fn completion_subject_label(subject: &str) -> String {
    let subject = subject.split("->").next().unwrap_or(subject);
    let subject = subject.split('@').next().unwrap_or(subject);
    let subject = subject
        .strip_prefix("train:")
        .or_else(|| subject.strip_prefix("build:"))
        .or_else(|| subject.strip_prefix("upgrade:"))
        .unwrap_or(subject);
    match subject {
        "signal_blade" => "SIGNAL BLADE".to_string(),
        "watch_tower" => "WATCH TOWER".to_string(),
        _ => classic_rts_order_completion_subject_label(subject),
    }
}

pub(crate) fn feedback_label(feedback: &str, max_chars: usize) -> String {
    let trimmed = feedback
        .trim()
        .strip_prefix("RTS ")
        .unwrap_or(feedback.trim())
        .trim();
    let upper = trimmed.to_ascii_uppercase();
    if upper.starts_with("UPGRADE COMPLETE")
        || upper.starts_with("BUILD COMPLETE")
        || upper.starts_with("PRODUCTION COMPLETE")
    {
        let subject_fallback = trimmed
            .split_whitespace()
            .skip(2)
            .collect::<Vec<_>>()
            .join(" ");
        let subject = trimmed
            .split_once(':')
            .map(|(_, subject)| subject)
            .unwrap_or(subject_fallback.as_str());
        let subject = completion_subject_label(subject.trim());
        return classic_catalog_text_label(&format!("{subject} READY"), max_chars);
    }
    if upper.contains("GROUP 1") && upper.contains("SECUR") && upper.contains("RELAY") {
        return classic_catalog_text_label("GROUP 1 SECURING RELAY", max_chars);
    }
    let cleaned = trimmed
        .replace("->", " ")
        .replace([':', '_', '.', '@'], " ");
    classic_catalog_text_label(&cleaned, max_chars)
}

fn role_for_unit(unit_id: &str, index: usize) -> &'static str {
    let normalized = unit_id.to_ascii_lowercase();
    if normalized.contains("player") || normalized.contains("lead") {
        "LEAD"
    } else if normalized.contains("guard") || normalized.contains("warden") {
        "GUARD"
    } else if normalized.contains("worker") || normalized.contains("harvest") {
        "WORKER"
    } else if normalized.contains("scout") || normalized.contains("creep") {
        "SCOUT"
    } else if normalized.contains("relay") {
        "RELAY"
    } else if normalized.contains("signal") {
        "SIGNAL"
    } else {
        ["LEAD", "GUARD", "WORKER", "SCOUT"]
            .get(index)
            .copied()
            .unwrap_or("UNIT")
    }
}

pub(crate) fn squad_roles(
    runtime: &NativeFirstPlayableRuntime,
    selected_unit_display_count: usize,
) -> Vec<String> {
    let fallback = ["LEAD", "GUARD", "WORKER", "SCOUT"];
    let count = selected_unit_display_count
        .max(runtime.rts_selected_unit_ids.len())
        .min(4);
    (0..count)
        .map(|index| {
            runtime
                .rts_selected_unit_ids
                .get(index)
                .map(|unit_id| role_for_unit(unit_id, index))
                .unwrap_or_else(|| fallback.get(index).copied().unwrap_or("UNIT"))
                .to_string()
        })
        .collect()
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
