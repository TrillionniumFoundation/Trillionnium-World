pub const TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_SUBJECT_SURFACE_CONTRACT: &str =
    "trnm_rts_bevy_runtime_first_contact_subject_surface_v1";

pub fn rts_first_contact_subject_label(subject: &str) -> String {
    let subject = subject.strip_prefix("trnm.").unwrap_or(subject);
    let subject = subject.strip_prefix("flux.").unwrap_or(subject);
    crate::rts_catalog_text_label(&subject.replace(['_', '.', ':', '-'], " "), 18)
}

pub fn rts_first_contact_order_completion_subject_label(subject: &str) -> String {
    let subject = rts_first_contact_order_subject_id(subject);
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
        _ => rts_first_contact_subject_label(subject),
    }
}

pub fn rts_first_contact_feedback_completion_subject_label(subject: &str) -> String {
    let subject = rts_first_contact_order_subject_id(subject);
    match subject {
        "signal_blade" => "SIGNAL BLADE".to_string(),
        "watch_tower" => "WATCH TOWER".to_string(),
        _ => rts_first_contact_order_completion_subject_label(subject),
    }
}

pub fn rts_first_contact_live_subject_label(subject: &str, max_chars: usize) -> String {
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

fn rts_first_contact_order_subject_id(subject: &str) -> &str {
    let subject = subject.split("->").next().unwrap_or(subject);
    let subject = subject.split('@').next().unwrap_or(subject);
    let subject = subject
        .strip_prefix("train:")
        .or_else(|| subject.strip_prefix("build:"))
        .or_else(|| subject.strip_prefix("upgrade:"))
        .unwrap_or(subject);
    subject.strip_prefix("trnm.").unwrap_or(subject)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_subject_surface_preserves_player_facing_subject_labels() {
        assert_eq!(
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_SUBJECT_SURFACE_CONTRACT,
            "trnm_rts_bevy_runtime_first_contact_subject_surface_v1"
        );
        assert_eq!(
            rts_first_contact_subject_label("trnm.flux.beacon"),
            "BEACON"
        );
        assert_eq!(
            rts_first_contact_order_completion_subject_label("build:watch_tower@7,4->watch_tower"),
            "TOWER"
        );
        assert_eq!(
            rts_first_contact_feedback_completion_subject_label(
                "build:watch_tower@7,4->watch_tower"
            ),
            "WATCH TOWER"
        );
        assert_eq!(
            rts_first_contact_feedback_completion_subject_label("signal_blade"),
            "SIGNAL BLADE"
        );
        assert_eq!(
            rts_first_contact_live_subject_label("objective:claim:ridge_sentries", 16),
            "RIDGE SENTRIES"
        );
        assert_eq!(
            rts_first_contact_live_subject_label("trnm.flux.beacon", 8),
            "RELAY BE"
        );
    }
}
