pub const TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_BOTTOM_PANEL_SURFACE_CONTRACT: &str =
    "trnm_rts_bevy_runtime_first_contact_bottom_panel_surface_v1";

pub fn rts_first_contact_bottom_panel_feedback_label(feedback: &str, max_chars: usize) -> String {
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
        let subject = rts_first_contact_completion_subject_label(subject.trim());
        return crate::rts_catalog_text_label(&format!("{subject} READY"), max_chars);
    }
    if upper.contains("GROUP 1") && upper.contains("SECUR") && upper.contains("RELAY") {
        return crate::rts_catalog_text_label("GROUP 1 SECURING RELAY", max_chars);
    }
    let cleaned = trimmed
        .replace("->", " ")
        .replace([':', '_', '.', '@'], " ");
    crate::rts_catalog_text_label(&cleaned, max_chars)
}

pub fn rts_first_contact_bottom_panel_squad_roles(
    selected_unit_ids: &[String],
    selected_unit_display_count: usize,
) -> Vec<String> {
    let fallback = ["LEAD", "GUARD", "WORKER", "SCOUT"];
    let count = selected_unit_display_count
        .max(selected_unit_ids.len())
        .min(4);
    (0..count)
        .map(|index| {
            selected_unit_ids
                .get(index)
                .map(|unit_id| rts_first_contact_bottom_panel_role_for_unit(unit_id, index))
                .unwrap_or_else(|| fallback.get(index).copied().unwrap_or("UNIT"))
                .to_string()
        })
        .collect()
}

fn rts_first_contact_bottom_panel_role_for_unit(unit_id: &str, index: usize) -> &'static str {
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

fn rts_first_contact_completion_subject_label(subject: &str) -> String {
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
        _ => rts_first_contact_order_completion_subject_label(subject),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
        values.into_iter().map(str::to_string).collect()
    }

    #[test]
    fn first_contact_bottom_panel_surface_preserves_feedback_and_roles() {
        assert_eq!(
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_BOTTOM_PANEL_SURFACE_CONTRACT,
            "trnm_rts_bevy_runtime_first_contact_bottom_panel_surface_v1"
        );
        assert_eq!(
            rts_first_contact_bottom_panel_feedback_label("RTS UPGRADE COMPLETE: SIGNAL BLADE", 62),
            "SIGNAL BLADE READY"
        );
        assert_eq!(
            rts_first_contact_bottom_panel_feedback_label(
                "RTS BUILD COMPLETE: build:watch_tower@7,4->watch_tower",
                62
            ),
            "WATCH TOWER READY"
        );
        assert_eq!(
            rts_first_contact_bottom_panel_feedback_label("RTS GROUP 1 SECURING RELAY", 62),
            "GROUP 1 SECURING RELAY"
        );
        assert_eq!(
            rts_first_contact_bottom_panel_squad_roles(
                &string_vec(["worker_03", "horizon_scout", "forge_warden", "flux_relay"]),
                4,
            ),
            string_vec(["WORKER", "SCOUT", "GUARD", "RELAY"])
        );
    }
}
