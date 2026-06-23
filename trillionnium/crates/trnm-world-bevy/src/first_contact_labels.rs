use trnm_rts_data::RtsPlayerScreenTacticsRowKind;

fn catalog_text_label(text: &str, max_chars: usize) -> String {
    text.replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_uppercase()
        .chars()
        .take(max_chars)
        .collect()
}

pub(crate) fn palette_state_badge_label(state_label: &str) -> String {
    match state_label.to_ascii_uppercase().as_str() {
        "ACTIVE" => "ACT".to_string(),
        "QUEUE" => "QUE".to_string(),
        "READY" => "RDY".to_string(),
        "LOCK" => "LCK".to_string(),
        state => catalog_text_label(state, 3),
    }
}

pub(crate) fn build_palette_badge_label(slot_label: &str) -> String {
    match slot_label.to_ascii_uppercase().as_str() {
        "COMMAND" => "CMD".to_string(),
        "POWER" => "PWR".to_string(),
        "RADAR" => "RAD".to_string(),
        "REFINE" => "REF".to_string(),
        "SIGNAL" => "SIG".to_string(),
        "TOWER" => "TWR".to_string(),
        "TRAIN" | "TRAINING" => "TRN".to_string(),
        "WALL" => "WAL".to_string(),
        label => catalog_text_label(label, 3),
    }
}

pub(crate) fn production_slot_badge_label(slot_label: &str) -> String {
    match slot_label.to_ascii_uppercase().as_str() {
        "GUARD" => "GRD".to_string(),
        "READY" => "RDY".to_string(),
        "SIGNAL" => "SIG".to_string(),
        "TRAINING" => "TRN".to_string(),
        "WORKER" => "WRK".to_string(),
        "COMMAND" => "CMD".to_string(),
        "RADAR" => "RAD".to_string(),
        "RELAY" => "RLY".to_string(),
        "TOWER" => "TWR".to_string(),
        label => catalog_text_label(label, 3),
    }
}

pub(crate) fn production_status_badge_label(status_label: &str) -> String {
    let upper = status_label.to_ascii_uppercase();
    if let Some(queue_id) = upper
        .strip_prefix('Q')
        .and_then(|rest| rest.split_whitespace().next())
    {
        return catalog_text_label(&format!("Q{queue_id}"), 3);
    }
    if let Some(build_id) = upper
        .strip_prefix('B')
        .and_then(|rest| rest.split_whitespace().next())
    {
        return catalog_text_label(&format!("B{build_id}"), 3);
    }
    match upper.as_str() {
        "ADD UNIT" | "ADD BUILD" => "ADD".to_string(),
        "LOCK" => "LCK".to_string(),
        "READY" => "RDY".to_string(),
        "QUEUE" => "QUE".to_string(),
        label => catalog_text_label(label, 3),
    }
}

pub(crate) fn tactics_queue_badge_label(value: &str) -> String {
    let tokens = value.split_whitespace().collect::<Vec<_>>();
    let mut badges = Vec::new();
    for pair in tokens.chunks(2) {
        if pair.len() != 2 || !pair[1].ends_with('%') {
            continue;
        }
        let code = match pair[0] {
            "GUARD" => "G",
            "TOWER" => "T",
            "WORKER" => "W",
            "TRAIN" | "TRAINING" => "TRN",
            "RELAY" => "R",
            "SIGNAL" => "S",
            other => other.get(0..1).unwrap_or("Q"),
        };
        badges.push(format!("{code}{}", pair[1].trim_end_matches('%')));
    }
    if badges.is_empty() {
        tactics_queue_fallback_badge_label(value)
    } else {
        catalog_text_label(&badges.join("/"), 9)
    }
}

fn tactics_queue_word_badge(token: &str) -> Option<&'static str> {
    let normalized = token
        .trim_matches(|ch: char| !ch.is_ascii_alphanumeric())
        .to_ascii_uppercase();
    match normalized.as_str() {
        "ATTACK" | "ATK" => Some("ATK"),
        "BEACON" | "BEAC" | "BCN" => Some("BCN"),
        "BUILD" | "BLD" => Some("BLD"),
        "COMMAND" | "CMD" => Some("CMD"),
        "GUARD" | "GRD" => Some("GRD"),
        "MOVE" | "MOV" => Some("MOV"),
        "RADAR" | "RAD" => Some("RAD"),
        "READY" | "RDY" => Some("RDY"),
        "RELAY" | "RLY" => Some("RLY"),
        "SIGNAL" | "SIG" | "SI" => Some("SIG"),
        "TOWER" | "TWR" | "TO" => Some("TWR"),
        "TRAIN" | "TRAINING" | "TRN" => Some("TRN"),
        "WORKER" | "WRK" | "WO" => Some("WRK"),
        _ => None,
    }
}

pub(crate) fn tactics_queue_fallback_badge_label(value: &str) -> String {
    match value {
        "READY" => return "RDY".to_string(),
        "IDLE" => return "IDLE".to_string(),
        _ => {}
    }
    let compact = value
        .split_whitespace()
        .filter_map(tactics_queue_word_badge)
        .take(2)
        .collect::<Vec<_>>();
    if compact.is_empty() {
        catalog_text_label(value, 8)
    } else {
        catalog_text_label(&compact.join(" "), 8)
    }
}

pub(crate) fn tactics_row_badge_label(kind: RtsPlayerScreenTacticsRowKind, value: &str) -> String {
    match kind {
        RtsPlayerScreenTacticsRowKind::Order => {
            if value.contains("SECURE") {
                "SECURE".to_string()
            } else if value == "READY" {
                "RDY".to_string()
            } else {
                catalog_text_label(value, 6)
            }
        }
        RtsPlayerScreenTacticsRowKind::Target => {
            if value.contains("BEACON") {
                "BEACON".to_string()
            } else if value == "NONE" {
                "NONE".to_string()
            } else {
                catalog_text_label(value, 6)
            }
        }
        RtsPlayerScreenTacticsRowKind::Camera => value.to_string(),
        RtsPlayerScreenTacticsRowKind::Queue => tactics_queue_badge_label(value),
        RtsPlayerScreenTacticsRowKind::Build => {
            if value == "IDLE" {
                "IDLE".to_string()
            } else {
                catalog_text_label(value, 8)
            }
        }
    }
}

pub(crate) fn order_queue_badge_code_from_display_label(label: &str) -> String {
    let upper = label.to_ascii_uppercase();
    if upper.contains("BLOCKED") {
        return "BLK".to_string();
    }
    let head = upper.split_whitespace().next().unwrap_or("ORDER");
    match head {
        "ATTACK" => "ATK",
        "BUILD" => "BLD",
        "GUARD" => "GRD",
        "MOVE" => "MOV",
        "SIGNAL" => "SIG",
        "TOWER" => "TWR",
        "TRAIN" | "TRAINING" => "TRN",
        "UPGRADE" => "UPG",
        "WORKER" => "WRK",
        "READY" => "RDY",
        _ => head.get(..head.len().min(3)).unwrap_or("ORD"),
    }
    .to_string()
}

pub(crate) fn order_queue_badge_detail_from_display_label(label: &str) -> String {
    let upper = label.to_ascii_uppercase();
    if upper.ends_with(" READY") {
        return "RDY".to_string();
    }
    if upper.contains("BLOCKED") {
        let blocked_subject = upper
            .replace("BLOCKED", "")
            .split_whitespace()
            .take(2)
            .collect::<Vec<_>>()
            .join(" ");
        return catalog_text_label(
            if blocked_subject.is_empty() {
                "PATH"
            } else {
                blocked_subject.as_str()
            },
            10,
        );
    }
    let detail = upper
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");
    let compact_detail = match detail.as_str() {
        "" => "READY",
        "BEACON" | "RELAY BEACON" => "BCN",
        "GUARD" => "GRD",
        "RELAY" => "RLY",
        "SIGNAL" | "SIGNAL BLADE" => "SIG",
        "TOWER" | "WATCH TOWER" => "TWR",
        "TRAINING" | "TRAINING HALL" => "TRN",
        "WORKER" => "WRK",
        _ => detail.as_str(),
    };
    catalog_text_label(compact_detail, 12)
}

pub(crate) fn order_queue_badge_label_from_display_label(label: &str) -> String {
    format!(
        "{} {}",
        order_queue_badge_code_from_display_label(label),
        order_queue_badge_detail_from_display_label(label)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_badges_stay_compact_and_player_facing() {
        assert_eq!(palette_state_badge_label("READY"), "RDY");
        assert_eq!(build_palette_badge_label("COMMAND"), "CMD");
        assert_eq!(production_slot_badge_label("TRAINING"), "TRN");
        assert_eq!(production_status_badge_label("Q3 64 R"), "Q3");
        assert_eq!(tactics_queue_badge_label("WORKER 42% TOWER 66%"), "W42/T66");
        assert_eq!(tactics_queue_badge_label("TRAIN SI"), "TRN SIG");
        assert_eq!(
            tactics_row_badge_label(RtsPlayerScreenTacticsRowKind::Target, "RELAY BEACON"),
            "BEACON"
        );
        assert_eq!(
            order_queue_badge_label_from_display_label("ATTACK BEACON"),
            "ATK BCN"
        );
        assert_eq!(
            order_queue_badge_label_from_display_label("TRAINING READY"),
            "TRN RDY"
        );
    }
}
