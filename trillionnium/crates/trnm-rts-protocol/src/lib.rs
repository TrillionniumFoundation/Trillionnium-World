//! Minimal frame-order protocol used by the current playable RTS simulation.

/// Versioned, bounded JSON intake; legacy serialization remains unchanged.
pub mod strict;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const RTS_ORDER_PROTOCOL: &str = "trnm_rts_order_protocol_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsOrderKind {
    Move,
    AttackMove,
    Harvest,
    Build,
    Capture,
    Extract,
    Attack,
    FocusFire,
    Ability,
    Repair,
    Recon,
    Train,
    Research,
    Upgrade,
    AssignGroup,
    AppendGroup,
    RemoveGroup,
    RecallGroup,
    CancelQueuedOrder,
    CancelJob,
    PauseJob,
    ResumeJob,
    PromoteJob,
    SetRally,
    Patrol,
    Stop,
    SetStance,
    Hold,
}

impl RtsOrderKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Move => "move",
            Self::AttackMove => "attack_move",
            Self::Harvest => "harvest",
            Self::Build => "build",
            Self::Capture => "capture",
            Self::Extract => "extract",
            Self::Attack => "attack",
            Self::FocusFire => "focus_fire",
            Self::Ability => "ability",
            Self::Repair => "repair",
            Self::Recon => "recon",
            Self::Train => "train",
            Self::Research => "research",
            Self::Upgrade => "upgrade",
            Self::AssignGroup => "assign_group",
            Self::AppendGroup => "append_group",
            Self::RemoveGroup => "remove_group",
            Self::RecallGroup => "recall_group",
            Self::CancelQueuedOrder => "cancel_queued_order",
            Self::CancelJob => "cancel_job",
            Self::PauseJob => "pause_job",
            Self::ResumeJob => "resume_job",
            Self::PromoteJob => "promote_job",
            Self::SetRally => "set_rally",
            Self::Patrol => "patrol",
            Self::Stop => "stop",
            Self::SetStance => "set_stance",
            Self::Hold => "hold",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsUnitStance {
    HoldFire,
    #[default]
    Guard,
    Aggressive,
}

impl RtsUnitStance {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldFire => "hold_fire",
            Self::Guard => "guard",
            Self::Aggressive => "aggressive",
        }
    }

    pub fn from_rule_id(rule_id: &str) -> Option<Self> {
        match rule_id {
            "hold_fire" => Some(Self::HoldFire),
            "guard" => Some(Self::Guard),
            "aggressive" => Some(Self::Aggressive),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RtsTile {
    pub x: i32,
    pub y: i32,
}

impl RtsTile {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsOrderSource {
    LocalInput,
    Replay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFrameOrder {
    pub contract: String,
    pub frame: u32,
    pub player_id: String,
    pub subject_actor_ids: Vec<String>,
    pub kind: RtsOrderKind,
    #[serde(default)]
    pub queued: bool,
    #[serde(default)]
    pub target_tile: Option<RtsTile>,
    #[serde(default)]
    pub target_actor_id: Option<String>,
    #[serde(default)]
    pub target_rule_id: Option<String>,
    #[serde(default)]
    pub queue_id: Option<String>,
    #[serde(default)]
    pub formation_id: Option<String>,
    pub source: RtsOrderSource,
    #[serde(default)]
    pub raw_command_label: Option<String>,
}

impl RtsFrameOrder {
    pub fn new(
        frame: u32,
        player_id: impl Into<String>,
        subject_actor_ids: impl Into<Vec<String>>,
        kind: RtsOrderKind,
        source: RtsOrderSource,
    ) -> Self {
        Self {
            contract: RTS_ORDER_PROTOCOL.to_string(),
            frame,
            player_id: player_id.into(),
            subject_actor_ids: subject_actor_ids.into(),
            kind,
            queued: false,
            target_tile: None,
            target_actor_id: None,
            target_rule_id: None,
            queue_id: None,
            formation_id: None,
            source,
            raw_command_label: None,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.contract != RTS_ORDER_PROTOCOL {
            return Err(format!("order_contract_mismatch:{}", self.contract));
        }
        if self.player_id.is_empty() || self.subject_actor_ids.is_empty() {
            return Err("order_player_or_subject_missing".to_string());
        }
        if self.queued && self.queue_id.as_deref().is_none_or(str::is_empty) {
            return Err("queued_order_id_missing".to_string());
        }
        match self.kind {
            RtsOrderKind::Move | RtsOrderKind::AttackMove | RtsOrderKind::Patrol => {
                if self.target_tile.is_none() {
                    return Err("move_target_missing".to_string());
                }
            }
            RtsOrderKind::Attack | RtsOrderKind::FocusFire => {
                if self.target_actor_id.is_none() && self.target_tile.is_none() {
                    return Err("attack_target_missing".to_string());
                }
            }
            RtsOrderKind::Harvest
            | RtsOrderKind::Capture
            | RtsOrderKind::Extract
            | RtsOrderKind::Repair => {
                if self.target_actor_id.is_none() && self.target_tile.is_none() {
                    return Err(format!("{}_target_missing", self.kind.as_str()));
                }
            }
            RtsOrderKind::Ability => {
                if self.target_rule_id.is_none() {
                    return Err("ability_rule_missing".to_string());
                }
            }
            RtsOrderKind::Build => {
                if self.target_rule_id.is_none() || self.target_tile.is_none() {
                    return Err("build_rule_or_tile_missing".to_string());
                }
            }
            RtsOrderKind::Recon => {
                if self.target_tile.is_none() {
                    return Err("recon_target_missing".to_string());
                }
            }
            RtsOrderKind::Train | RtsOrderKind::Research | RtsOrderKind::Upgrade => {
                if self.target_rule_id.is_none() || self.queue_id.is_none() {
                    return Err(format!("{}_rule_or_queue_missing", self.kind.as_str()));
                }
            }
            RtsOrderKind::AssignGroup
            | RtsOrderKind::AppendGroup
            | RtsOrderKind::RemoveGroup
            | RtsOrderKind::RecallGroup => {
                if self.target_rule_id.is_none() {
                    return Err("control_group_id_missing".to_string());
                }
            }
            RtsOrderKind::CancelQueuedOrder
            | RtsOrderKind::CancelJob
            | RtsOrderKind::PauseJob
            | RtsOrderKind::ResumeJob
            | RtsOrderKind::PromoteJob => {
                if self.queue_id.is_none() {
                    return Err(format!("{}_queue_id_missing", self.kind.as_str()));
                }
            }
            RtsOrderKind::SetRally => {
                if self.queue_id.is_none() || self.target_tile.is_none() {
                    return Err("set_rally_queue_or_tile_missing".to_string());
                }
            }
            RtsOrderKind::SetStance => {
                if self
                    .target_rule_id
                    .as_deref()
                    .and_then(RtsUnitStance::from_rule_id)
                    .is_none()
                {
                    return Err("unit_stance_missing_or_invalid".to_string());
                }
            }
            RtsOrderKind::Stop | RtsOrderKind::Hold => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFrameOrderStream {
    pub contract: String,
    pub map_id: String,
    pub rules_id: String,
    pub orders: Vec<RtsFrameOrder>,
}

impl RtsFrameOrderStream {
    pub fn new(
        map_id: impl Into<String>,
        rules_id: impl Into<String>,
        orders: Vec<RtsFrameOrder>,
    ) -> Self {
        Self {
            contract: RTS_ORDER_PROTOCOL.to_string(),
            map_id: map_id.into(),
            rules_id: rules_id.into(),
            orders,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.contract != RTS_ORDER_PROTOCOL || self.map_id.is_empty() || self.rules_id.is_empty()
        {
            return Err("invalid_order_stream_header".to_string());
        }
        let mut previous = None;
        for order in &self.orders {
            order.validate()?;
            if previous.is_some_and(|frame| order.frame < frame) {
                return Err("frame_regression".to_string());
            }
            previous = Some(order.frame);
        }
        Ok(())
    }

    pub fn sha256_hex(&self) -> String {
        let bytes = serde_json::to_vec(self).expect("order stream serializes");
        Sha256::digest(bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_rejects_regression_and_missing_targets() {
        let mut first = RtsFrameOrder::new(
            2,
            "player",
            vec!["hero".to_string()],
            RtsOrderKind::Move,
            RtsOrderSource::LocalInput,
        );
        first.target_tile = Some(RtsTile::new(1, 2));
        let mut second = first.clone();
        second.frame = 1;
        assert!(
            RtsFrameOrderStream::new("map", "rules", vec![first, second])
                .validate()
                .is_err()
        );
    }

    #[test]
    fn gameplay_orders_require_typed_rules_and_queues() {
        for kind in [
            RtsOrderKind::Train,
            RtsOrderKind::Research,
            RtsOrderKind::Upgrade,
        ] {
            let mut order = RtsFrameOrder::new(
                1,
                "player",
                vec!["hero".to_string()],
                kind,
                RtsOrderSource::LocalInput,
            );
            assert!(order.validate().is_err());
            order.target_rule_id = Some("field_support".to_string());
            order.queue_id = Some("expedition_queue".to_string());
            assert!(order.validate().is_ok());
        }
    }

    #[test]
    fn control_group_queue_and_job_lifecycle_orders_are_typed() {
        let mut group = RtsFrameOrder::new(
            1,
            "player",
            vec!["hero".to_string()],
            RtsOrderKind::AssignGroup,
            RtsOrderSource::LocalInput,
        );
        assert!(group.validate().is_err());
        group.target_rule_id = Some("1".to_string());
        assert!(group.validate().is_ok());

        let mut cancel = group.clone();
        cancel.kind = RtsOrderKind::CancelJob;
        cancel.target_rule_id = None;
        assert!(cancel.validate().is_err());
        cancel.queue_id = Some("expedition_production-42".to_string());
        assert!(cancel.validate().is_ok());

        let mut rally = cancel;
        rally.kind = RtsOrderKind::SetRally;
        assert!(rally.validate().is_err());
        rally.target_tile = Some(RtsTile::new(4, 5));
        assert!(rally.validate().is_ok());
    }

    #[test]
    fn patrol_stop_and_stance_are_typed() {
        let mut patrol = RtsFrameOrder::new(
            1,
            "player",
            vec!["hero".to_string()],
            RtsOrderKind::Patrol,
            RtsOrderSource::LocalInput,
        );
        assert!(patrol.validate().is_err());
        patrol.target_tile = Some(RtsTile::new(4, 5));
        patrol.validate().unwrap();

        let stop = RtsFrameOrder::new(
            2,
            "player",
            vec!["hero".to_string()],
            RtsOrderKind::Stop,
            RtsOrderSource::LocalInput,
        );
        stop.validate().unwrap();

        let mut stance = RtsFrameOrder::new(
            3,
            "player",
            vec!["hero".to_string()],
            RtsOrderKind::SetStance,
            RtsOrderSource::LocalInput,
        );
        stance.target_rule_id = Some(RtsUnitStance::Aggressive.as_str().to_string());
        stance.validate().unwrap();
    }
}
