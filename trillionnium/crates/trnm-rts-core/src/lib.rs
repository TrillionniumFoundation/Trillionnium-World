//! Small deterministic RTS core types for Trillionnium.
//!
//! This crate is intentionally Bevy-free. It is the first landing zone for
//! frame orders emitted by the current playable client before rendering.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub const TRNM_RTS_CORE_CONTRACT: &str = "trnm_rts_core_frame_order_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsOrderKind {
    Move,
    AttackMove,
    Patrol,
    Stop,
    Harvest,
    ReturnCargo,
    Build,
    Train,
    Research,
    Upgrade,
    Unlock,
    Complete,
    Capture,
    Attack,
    FocusFire,
    Ability,
    Repair,
    Cancel,
    Refund,
    Hold,
    Follow,
    Queue,
}

impl RtsOrderKind {
    pub fn as_str(self) -> &'static str {
        match self {
            RtsOrderKind::Move => "move",
            RtsOrderKind::AttackMove => "attack_move",
            RtsOrderKind::Patrol => "patrol",
            RtsOrderKind::Stop => "stop",
            RtsOrderKind::Harvest => "harvest",
            RtsOrderKind::ReturnCargo => "return_cargo",
            RtsOrderKind::Build => "build",
            RtsOrderKind::Train => "train",
            RtsOrderKind::Research => "research",
            RtsOrderKind::Upgrade => "upgrade",
            RtsOrderKind::Unlock => "unlock",
            RtsOrderKind::Complete => "complete",
            RtsOrderKind::Capture => "capture",
            RtsOrderKind::Attack => "attack",
            RtsOrderKind::FocusFire => "focus_fire",
            RtsOrderKind::Ability => "ability",
            RtsOrderKind::Repair => "repair",
            RtsOrderKind::Cancel => "cancel",
            RtsOrderKind::Refund => "refund",
            RtsOrderKind::Hold => "hold",
            RtsOrderKind::Follow => "follow",
            RtsOrderKind::Queue => "queue",
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

    pub fn parse_csv(value: &str) -> Result<Self, String> {
        let (x, y) = value
            .split_once(',')
            .ok_or_else(|| format!("tile_missing_comma:{value}"))?;
        let x = x
            .parse::<i32>()
            .map_err(|_| format!("tile_bad_x:{value}"))?;
        let y = y
            .parse::<i32>()
            .map_err(|_| format!("tile_bad_y:{value}"))?;
        Ok(Self { x, y })
    }

    pub fn label(self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsOrderSource {
    LocalInput,
    Bot,
    Replay,
    Server,
    ImportedFixture,
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
            contract: TRNM_RTS_CORE_CONTRACT.to_string(),
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

    pub fn from_live_command_label(
        frame: u32,
        player_id: impl Into<String>,
        subject_actor_ids: impl Into<Vec<String>>,
        label: &str,
    ) -> Result<Self, String> {
        let player_id = player_id.into();
        let subject_actor_ids = subject_actor_ids.into();
        let mut order = if let Some(rest) = label.strip_prefix("RTS:MOVE:") {
            live_move_order(frame, player_id, subject_actor_ids, rest)?
        } else if let Some(target_actor_id) = label.strip_prefix("RTS:ATTACK:") {
            if target_actor_id.is_empty() {
                return Err("attack_target_missing".to_string());
            }
            let mut order = Self::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Attack,
                RtsOrderSource::LocalInput,
            );
            order.target_actor_id = Some(target_actor_id.to_string());
            order
        } else if let Some(rest) = label.strip_prefix("RTS:QUEUE:") {
            live_queue_order(frame, player_id, subject_actor_ids, rest)?
        } else if let Some(rest) = label.strip_prefix("RTS:ABILITY:") {
            live_ability_order(frame, player_id, subject_actor_ids, rest)?
        } else {
            return Err(format!("unsupported_live_command_label:{label}"));
        };
        order.raw_command_label = Some(label.to_string());
        order.validate()?;
        Ok(order)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.contract != TRNM_RTS_CORE_CONTRACT {
            return Err(format!("contract_mismatch:{}", self.contract));
        }
        if self.player_id.is_empty() {
            return Err("player_id_missing".to_string());
        }
        match self.kind {
            RtsOrderKind::Move | RtsOrderKind::AttackMove | RtsOrderKind::Patrol => {
                if self.target_tile.is_none() {
                    return Err(format!("{}_target_tile_missing", self.kind.as_str()));
                }
            }
            RtsOrderKind::Attack | RtsOrderKind::FocusFire => {
                if self.target_actor_id.is_none() && self.target_tile.is_none() {
                    return Err(format!("{}_target_missing", self.kind.as_str()));
                }
            }
            RtsOrderKind::Ability => {
                if self.target_rule_id.is_none() {
                    return Err("ability_rule_missing".to_string());
                }
            }
            RtsOrderKind::Follow => {
                if self.target_actor_id.is_none() {
                    return Err("follow_target_actor_missing".to_string());
                }
            }
            RtsOrderKind::Harvest | RtsOrderKind::Capture | RtsOrderKind::Repair => {
                if self.target_actor_id.is_none() && self.target_tile.is_none() {
                    return Err(format!("{}_target_missing", self.kind.as_str()));
                }
            }
            RtsOrderKind::Build => {
                if self.target_rule_id.is_none() {
                    return Err("build_rule_missing".to_string());
                }
                if self.target_tile.is_none() {
                    return Err("build_target_tile_missing".to_string());
                }
            }
            RtsOrderKind::Train => {
                if self.target_rule_id.is_none() {
                    return Err("train_rule_missing".to_string());
                }
            }
            RtsOrderKind::Research | RtsOrderKind::Upgrade => {
                if self.target_rule_id.is_none() {
                    return Err(format!("{}_rule_missing", self.kind.as_str()));
                }
                if self.target_actor_id.is_none() {
                    return Err(format!("{}_source_missing", self.kind.as_str()));
                }
            }
            RtsOrderKind::Unlock => {
                if self.target_rule_id.is_none() {
                    return Err("unlock_rule_missing".to_string());
                }
            }
            RtsOrderKind::Complete => {
                if self.target_rule_id.is_none() {
                    return Err("complete_rule_missing".to_string());
                }
                if self.target_tile.is_none() {
                    return Err("complete_target_tile_missing".to_string());
                }
            }
            RtsOrderKind::Cancel => {
                if self.queue_id.is_none() && self.target_rule_id.is_none() {
                    return Err("cancel_target_missing".to_string());
                }
            }
            RtsOrderKind::Refund => {
                if self.target_rule_id.is_none() {
                    return Err("refund_rule_missing".to_string());
                }
                if self.target_tile.is_none() {
                    return Err("refund_target_tile_missing".to_string());
                }
                if self.queue_id.is_none() {
                    return Err("refund_delta_missing".to_string());
                }
            }
            RtsOrderKind::ReturnCargo | RtsOrderKind::Stop | RtsOrderKind::Hold => {}
            RtsOrderKind::Queue => {
                if self.queue_id.is_none() {
                    return Err("queue_id_missing".to_string());
                }
            }
        }
        Ok(())
    }
}

fn live_move_order(
    frame: u32,
    player_id: String,
    subject_actor_ids: Vec<String>,
    rest: &str,
) -> Result<RtsFrameOrder, String> {
    let parts = rest.split(':').collect::<Vec<_>>();
    let tile = RtsTile::parse_csv(
        parts
            .first()
            .copied()
            .ok_or_else(|| "move_tile_missing".to_string())?,
    )?;
    let mode = parts.get(1).copied().unwrap_or("line");
    let kind = match mode {
        "attack_move" => RtsOrderKind::AttackMove,
        "follow" => RtsOrderKind::Follow,
        "hold" => RtsOrderKind::Hold,
        "patrol" => RtsOrderKind::Patrol,
        "stop" => RtsOrderKind::Stop,
        _ => RtsOrderKind::Move,
    };
    let mut order = RtsFrameOrder::new(
        frame,
        player_id,
        subject_actor_ids,
        kind,
        RtsOrderSource::LocalInput,
    );
    order.target_tile = Some(tile);
    order.formation_id = Some(mode.to_string());
    if mode == "shift_waypoint" {
        order.queued = true;
    }
    if mode == "follow" {
        let target_actor_id = parts
            .get(2)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| "follow_target_actor_missing".to_string())?;
        order.target_actor_id = Some((*target_actor_id).to_string());
    }
    Ok(order)
}

fn live_queue_order(
    frame: u32,
    player_id: String,
    subject_actor_ids: Vec<String>,
    rest: &str,
) -> Result<RtsFrameOrder, String> {
    let (queue_kind, payload) = rest
        .split_once(':')
        .ok_or_else(|| format!("queue_payload_missing:{rest}"))?;
    if payload.is_empty() {
        return Err(format!("queue_payload_empty:{queue_kind}"));
    }
    match queue_kind {
        "harvest" => {
            let mut order = RtsFrameOrder::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Harvest,
                RtsOrderSource::LocalInput,
            );
            order.queued = true;
            order.target_actor_id = Some(payload.to_string());
            Ok(order)
        }
        "build" => {
            let (rule_id, tile) = payload
                .split_once('@')
                .ok_or_else(|| format!("build_payload_missing_tile:{payload}"))?;
            let mut order = RtsFrameOrder::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Build,
                RtsOrderSource::LocalInput,
            );
            order.queued = true;
            order.target_rule_id = Some(rule_id.to_string());
            order.target_tile = Some(RtsTile::parse_csv(tile)?);
            Ok(order)
        }
        "train" => {
            let mut order = RtsFrameOrder::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Train,
                RtsOrderSource::LocalInput,
            );
            order.queued = true;
            order.target_rule_id = Some(payload.to_string());
            Ok(order)
        }
        "research" => {
            let (tech_id, source_id) = live_rule_source_payload(payload, "research")?;
            let mut order = RtsFrameOrder::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Research,
                RtsOrderSource::LocalInput,
            );
            order.queued = true;
            order.target_rule_id = Some(tech_id);
            order.target_actor_id = Some(source_id);
            Ok(order)
        }
        "upgrade" => {
            let (upgrade_id, source_id) = live_rule_source_payload(payload, "upgrade")?;
            let mut order = RtsFrameOrder::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Upgrade,
                RtsOrderSource::LocalInput,
            );
            order.queued = true;
            order.target_rule_id = Some(upgrade_id);
            order.target_actor_id = Some(source_id);
            Ok(order)
        }
        "unlock" => {
            let mut order = RtsFrameOrder::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Unlock,
                RtsOrderSource::LocalInput,
            );
            order.queued = true;
            order.target_rule_id = Some(payload.to_string());
            Ok(order)
        }
        "commander" => {
            if let Some(ability_payload) = payload.strip_prefix("ability:") {
                let mut order =
                    live_ability_order(frame, player_id, subject_actor_ids, ability_payload)?;
                order.queued = true;
                order.queue_id = Some(rest.to_string());
                Ok(order)
            } else {
                let mut order = RtsFrameOrder::new(
                    frame,
                    player_id,
                    subject_actor_ids,
                    RtsOrderKind::Queue,
                    RtsOrderSource::LocalInput,
                );
                order.queued = true;
                order.queue_id = Some(rest.to_string());
                Ok(order)
            }
        }
        "complete" => {
            let (rule_id, tile) = payload
                .split_once('@')
                .ok_or_else(|| format!("complete_payload_missing_tile:{payload}"))?;
            let mut order = RtsFrameOrder::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Complete,
                RtsOrderSource::LocalInput,
            );
            order.queued = true;
            order.target_rule_id = Some(rule_id.to_string());
            order.target_tile = Some(RtsTile::parse_csv(tile)?);
            Ok(order)
        }
        "repair" => {
            let (target_actor_id, tile) = payload
                .split_once('@')
                .ok_or_else(|| format!("repair_payload_missing_tile:{payload}"))?;
            let mut order = RtsFrameOrder::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Repair,
                RtsOrderSource::LocalInput,
            );
            order.queued = true;
            order.target_actor_id = Some(target_actor_id.to_string());
            order.target_tile = Some(RtsTile::parse_csv(tile)?);
            Ok(order)
        }
        "cancel" => {
            let mut order = RtsFrameOrder::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Cancel,
                RtsOrderSource::LocalInput,
            );
            order.queued = true;
            order.queue_id = Some(payload.to_string());
            Ok(order)
        }
        "refund" => {
            let (target, delta) = payload
                .split_once(':')
                .ok_or_else(|| format!("refund_payload_missing_delta:{payload}"))?;
            let (rule_id, tile) = target
                .split_once('@')
                .ok_or_else(|| format!("refund_payload_missing_tile:{payload}"))?;
            let mut order = RtsFrameOrder::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Refund,
                RtsOrderSource::LocalInput,
            );
            order.queued = true;
            order.target_rule_id = Some(rule_id.to_string());
            order.target_tile = Some(RtsTile::parse_csv(tile)?);
            order.queue_id = Some(delta.to_string());
            Ok(order)
        }
        _ => {
            let mut order = RtsFrameOrder::new(
                frame,
                player_id,
                subject_actor_ids,
                RtsOrderKind::Queue,
                RtsOrderSource::LocalInput,
            );
            order.queued = true;
            order.queue_id = Some(rest.to_string());
            Ok(order)
        }
    }
}

fn live_ability_order(
    frame: u32,
    player_id: String,
    subject_actor_ids: Vec<String>,
    rest: &str,
) -> Result<RtsFrameOrder, String> {
    if rest.is_empty() {
        return Err("ability_id_missing".to_string());
    }
    let (ability_id, target_actor_id) = match rest.split_once('@') {
        Some((ability_id, target_actor_id)) => {
            if ability_id.is_empty() {
                return Err(format!("ability_id_empty:{rest}"));
            }
            if target_actor_id.is_empty() {
                return Err(format!("ability_target_empty:{rest}"));
            }
            (ability_id, Some(target_actor_id))
        }
        None => (rest, None),
    };
    let mut order = RtsFrameOrder::new(
        frame,
        player_id,
        subject_actor_ids,
        RtsOrderKind::Ability,
        RtsOrderSource::LocalInput,
    );
    order.target_rule_id = Some(ability_id.to_string());
    order.target_actor_id = target_actor_id.map(ToString::to_string);
    Ok(order)
}

fn live_rule_source_payload(payload: &str, kind: &str) -> Result<(String, String), String> {
    let (rule_id, source_id) = payload
        .split_once('@')
        .ok_or_else(|| format!("{kind}_payload_missing_source:{payload}"))?;
    if rule_id.is_empty() {
        return Err(format!("{kind}_payload_rule_empty:{payload}"));
    }
    if source_id.is_empty() {
        return Err(format!("{kind}_payload_source_empty:{payload}"));
    }
    Ok((rule_id.to_string(), source_id.to_string()))
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
            contract: TRNM_RTS_CORE_CONTRACT.to_string(),
            map_id: map_id.into(),
            rules_id: rules_id.into(),
            orders,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.contract != TRNM_RTS_CORE_CONTRACT {
            return Err(format!("stream_contract_mismatch:{}", self.contract));
        }
        if self.map_id.is_empty() {
            return Err("map_id_missing".to_string());
        }
        if self.rules_id.is_empty() {
            return Err("rules_id_missing".to_string());
        }
        let mut previous_frame = None;
        for order in &self.orders {
            order.validate()?;
            if let Some(previous_frame) = previous_frame {
                if order.frame < previous_frame {
                    return Err(format!(
                        "frame_regression:{previous_frame}->{}",
                        order.frame
                    ));
                }
            }
            previous_frame = Some(order.frame);
        }
        Ok(())
    }

    pub fn canonical_json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("rts frame order stream serializes")
    }

    pub fn sha256_hex(&self) -> String {
        sha256_hex(&self.canonical_json_bytes())
    }

    pub fn replay_headless(&self) -> Result<RtsHeadlessReplayReport, String> {
        self.validate()?;
        let stream_sha256 = self.sha256_hex();
        let mut players = BTreeMap::<String, RtsPlayerCheckpoint>::new();
        let mut actors = BTreeMap::<String, RtsActorCheckpoint>::new();
        let mut production_lifecycle = RtsProductionLifecycleCheckpoint::default();
        let mut tech_tree = RtsTechTreeCheckpoint::default();
        let mut abilities = RtsAbilityCheckpoint::default();
        let mut event_log = Vec::new();
        let mut final_frame = 0_u32;

        for order in &self.orders {
            final_frame = final_frame.max(order.frame);
            let subject_count = order.subject_actor_ids.len() as u32;
            let player = players
                .entry(order.player_id.clone())
                .or_insert_with(|| RtsPlayerCheckpoint::new(order.player_id.clone()));
            player.issued_order_count += 1;
            player.subject_order_count += subject_count;
            if order.queued {
                player.queued_order_count += 1;
            }

            let target_label = order_target_label(order);
            event_log.push(format!(
                "frame:{}:player:{}:kind:{}:subjects:{}:target:{}",
                order.frame,
                order.player_id,
                order.kind.as_str(),
                subject_count,
                target_label
            ));
            production_lifecycle.record_order(order);
            tech_tree.record_order(order);
            abilities.record_order(order);

            for actor_id in &order.subject_actor_ids {
                let actor = actors.entry(actor_id.clone()).or_insert_with(|| {
                    RtsActorCheckpoint::new(actor_id.clone(), order.player_id.clone())
                });
                actor.player_id = order.player_id.clone();
                actor.last_frame = Some(order.frame);
                actor.last_order_kind = Some(order.kind);
                actor.last_raw_command_label = order.raw_command_label.clone();
                if order.queued {
                    actor.queued_order_count += 1;
                }
                actor.command_history.push(format!(
                    "{}:{}:{}",
                    order.frame,
                    order.kind.as_str(),
                    target_label
                ));

                match order.kind {
                    RtsOrderKind::Move | RtsOrderKind::AttackMove | RtsOrderKind::Patrol => {
                        actor.tile = order.target_tile;
                        actor.formation_id = order.formation_id.clone();
                    }
                    RtsOrderKind::Follow => {
                        actor.tile = order.target_tile;
                        actor.target_actor_id = order.target_actor_id.clone();
                        actor.formation_id = order.formation_id.clone();
                    }
                    RtsOrderKind::Attack | RtsOrderKind::FocusFire => {
                        actor.target_actor_id = order.target_actor_id.clone();
                        actor.target_tile = order.target_tile;
                        actor.attack_order_count += 1;
                    }
                    RtsOrderKind::Ability => {
                        actor.target_rule_id = order.target_rule_id.clone();
                        actor.target_actor_id = order.target_actor_id.clone();
                        actor.target_tile = order.target_tile;
                        actor.ability_order_count += 1;
                    }
                    RtsOrderKind::Harvest | RtsOrderKind::ReturnCargo => {
                        actor.target_actor_id = order.target_actor_id.clone();
                        actor.target_tile = order.target_tile;
                        actor.harvest_order_count += 1;
                    }
                    RtsOrderKind::Build => {
                        actor.target_rule_id = order.target_rule_id.clone();
                        actor.tile = order.target_tile;
                        actor.build_order_count += 1;
                    }
                    RtsOrderKind::Train => {
                        actor.target_rule_id = order.target_rule_id.clone();
                        actor.train_order_count += 1;
                    }
                    RtsOrderKind::Research => {
                        actor.target_rule_id = order.target_rule_id.clone();
                        actor.target_actor_id = order.target_actor_id.clone();
                        actor.research_order_count += 1;
                    }
                    RtsOrderKind::Upgrade => {
                        actor.target_rule_id = order.target_rule_id.clone();
                        actor.target_actor_id = order.target_actor_id.clone();
                        actor.upgrade_order_count += 1;
                    }
                    RtsOrderKind::Unlock => {
                        actor.target_rule_id = order.target_rule_id.clone();
                        actor.unlock_order_count += 1;
                    }
                    RtsOrderKind::Complete => {
                        actor.target_rule_id = order.target_rule_id.clone();
                        actor.tile = order.target_tile;
                        actor.complete_order_count += 1;
                    }
                    RtsOrderKind::Capture | RtsOrderKind::Repair => {
                        actor.target_actor_id = order.target_actor_id.clone();
                        actor.target_tile = order.target_tile;
                        if order.kind == RtsOrderKind::Repair {
                            actor.repair_order_count += 1;
                        }
                    }
                    RtsOrderKind::Cancel => {
                        actor.target_rule_id = order.target_rule_id.clone();
                        actor.target_tile = order.target_tile;
                        actor.queue_id = order.queue_id.clone();
                        actor.cancel_order_count += 1;
                    }
                    RtsOrderKind::Refund => {
                        actor.target_rule_id = order.target_rule_id.clone();
                        actor.target_tile = order.target_tile;
                        actor.queue_id = order.queue_id.clone();
                        actor.refund_order_count += 1;
                    }
                    RtsOrderKind::Queue => {
                        actor.queue_id = order.queue_id.clone();
                    }
                    RtsOrderKind::Stop | RtsOrderKind::Hold => {
                        actor.target_tile = order.target_tile;
                        actor.formation_id = order.formation_id.clone();
                    }
                }
            }
        }

        let actors = actors.into_values().collect::<Vec<_>>();
        let players = players.into_values().collect::<Vec<_>>();
        let checkpoint = RtsHeadlessReplayCheckpoint {
            contract: TRNM_RTS_CORE_CONTRACT.to_string(),
            map_id: self.map_id.clone(),
            rules_id: self.rules_id.clone(),
            stream_sha256,
            final_frame,
            applied_order_count: self.orders.len() as u32,
            player_count: players.len() as u32,
            actor_count: actors.len() as u32,
            production_lifecycle,
            tech_tree,
            abilities,
            players,
            actors,
            event_log,
        };
        let checkpoint_sha256 = checkpoint.sha256_hex();
        Ok(RtsHeadlessReplayReport {
            contract: TRNM_RTS_CORE_CONTRACT.to_string(),
            checkpoint_sha256,
            checkpoint,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsHeadlessReplayReport {
    pub contract: String,
    pub checkpoint_sha256: String,
    pub checkpoint: RtsHeadlessReplayCheckpoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsHeadlessReplayCheckpoint {
    pub contract: String,
    pub map_id: String,
    pub rules_id: String,
    pub stream_sha256: String,
    pub final_frame: u32,
    pub applied_order_count: u32,
    pub player_count: u32,
    pub actor_count: u32,
    pub production_lifecycle: RtsProductionLifecycleCheckpoint,
    pub tech_tree: RtsTechTreeCheckpoint,
    pub abilities: RtsAbilityCheckpoint,
    pub players: Vec<RtsPlayerCheckpoint>,
    pub actors: Vec<RtsActorCheckpoint>,
    pub event_log: Vec<String>,
}

impl RtsHeadlessReplayCheckpoint {
    pub fn canonical_json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("rts headless checkpoint serializes")
    }

    pub fn sha256_hex(&self) -> String {
        sha256_hex(&self.canonical_json_bytes())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsPlayerCheckpoint {
    pub player_id: String,
    pub issued_order_count: u32,
    pub subject_order_count: u32,
    pub queued_order_count: u32,
}

impl RtsPlayerCheckpoint {
    fn new(player_id: String) -> Self {
        Self {
            player_id,
            issued_order_count: 0,
            subject_order_count: 0,
            queued_order_count: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsActorCheckpoint {
    pub actor_id: String,
    pub player_id: String,
    #[serde(default)]
    pub last_frame: Option<u32>,
    #[serde(default)]
    pub last_order_kind: Option<RtsOrderKind>,
    #[serde(default)]
    pub last_raw_command_label: Option<String>,
    #[serde(default)]
    pub tile: Option<RtsTile>,
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
    pub queued_order_count: u32,
    pub build_order_count: u32,
    pub train_order_count: u32,
    pub research_order_count: u32,
    pub upgrade_order_count: u32,
    pub unlock_order_count: u32,
    pub complete_order_count: u32,
    pub repair_order_count: u32,
    pub cancel_order_count: u32,
    pub refund_order_count: u32,
    pub ability_order_count: u32,
    pub attack_order_count: u32,
    pub harvest_order_count: u32,
    pub command_history: Vec<String>,
}

impl RtsActorCheckpoint {
    fn new(actor_id: String, player_id: String) -> Self {
        Self {
            actor_id,
            player_id,
            last_frame: None,
            last_order_kind: None,
            last_raw_command_label: None,
            tile: None,
            target_tile: None,
            target_actor_id: None,
            target_rule_id: None,
            queue_id: None,
            formation_id: None,
            queued_order_count: 0,
            build_order_count: 0,
            train_order_count: 0,
            research_order_count: 0,
            upgrade_order_count: 0,
            unlock_order_count: 0,
            complete_order_count: 0,
            repair_order_count: 0,
            cancel_order_count: 0,
            refund_order_count: 0,
            ability_order_count: 0,
            attack_order_count: 0,
            harvest_order_count: 0,
            command_history: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsAbilityCheckpoint {
    pub ability_order_count: u32,
    pub ability_rule_ids: Vec<String>,
    pub target_actor_ids: Vec<String>,
    pub target_tile_ids: Vec<String>,
    pub queued_ability_count: u32,
}

impl RtsAbilityCheckpoint {
    fn record_order(&mut self, order: &RtsFrameOrder) {
        if order.kind != RtsOrderKind::Ability {
            return;
        }
        self.ability_order_count += 1;
        if order.queued {
            self.queued_ability_count += 1;
        }
        push_if_present(&mut self.ability_rule_ids, order.target_rule_id.as_deref());
        push_if_present(&mut self.target_actor_ids, order.target_actor_id.as_deref());
        if let Some(tile) = order.target_tile {
            self.target_tile_ids.push(tile.label());
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsTechTreeCheckpoint {
    pub tech_order_count: u32,
    pub research_order_count: u32,
    pub upgrade_order_count: u32,
    pub unlock_order_count: u32,
    pub researched_rule_ids: Vec<String>,
    pub upgraded_rule_ids: Vec<String>,
    pub unlocked_rule_ids: Vec<String>,
    pub source_actor_ids: Vec<String>,
}

impl RtsTechTreeCheckpoint {
    fn record_order(&mut self, order: &RtsFrameOrder) {
        match order.kind {
            RtsOrderKind::Research => {
                self.tech_order_count += 1;
                self.research_order_count += 1;
                push_if_present(
                    &mut self.researched_rule_ids,
                    order.target_rule_id.as_deref(),
                );
                push_if_present(&mut self.source_actor_ids, order.target_actor_id.as_deref());
            }
            RtsOrderKind::Upgrade => {
                self.tech_order_count += 1;
                self.upgrade_order_count += 1;
                push_if_present(&mut self.upgraded_rule_ids, order.target_rule_id.as_deref());
                push_if_present(&mut self.source_actor_ids, order.target_actor_id.as_deref());
            }
            RtsOrderKind::Unlock => {
                self.tech_order_count += 1;
                self.unlock_order_count += 1;
                push_if_present(&mut self.unlocked_rule_ids, order.target_rule_id.as_deref());
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsProductionLifecycleCheckpoint {
    pub lifecycle_order_count: u32,
    pub build_order_count: u32,
    pub train_order_count: u32,
    pub complete_order_count: u32,
    pub repair_order_count: u32,
    pub cancel_order_count: u32,
    pub refund_order_count: u32,
    pub build_rule_ids: Vec<String>,
    pub train_rule_ids: Vec<String>,
    pub completed_rule_ids: Vec<String>,
    pub repair_target_ids: Vec<String>,
    pub cancelled_queue_ids: Vec<String>,
    pub refund_rule_ids: Vec<String>,
    pub refund_delta_labels: Vec<String>,
}

impl RtsProductionLifecycleCheckpoint {
    fn record_order(&mut self, order: &RtsFrameOrder) {
        match order.kind {
            RtsOrderKind::Build => {
                self.lifecycle_order_count += 1;
                self.build_order_count += 1;
                push_if_present(&mut self.build_rule_ids, order.target_rule_id.as_deref());
            }
            RtsOrderKind::Train => {
                self.lifecycle_order_count += 1;
                self.train_order_count += 1;
                push_if_present(&mut self.train_rule_ids, order.target_rule_id.as_deref());
            }
            RtsOrderKind::Complete => {
                self.lifecycle_order_count += 1;
                self.complete_order_count += 1;
                push_if_present(
                    &mut self.completed_rule_ids,
                    order.target_rule_id.as_deref(),
                );
            }
            RtsOrderKind::Repair => {
                self.lifecycle_order_count += 1;
                self.repair_order_count += 1;
                push_if_present(
                    &mut self.repair_target_ids,
                    order.target_actor_id.as_deref(),
                );
            }
            RtsOrderKind::Cancel => {
                self.lifecycle_order_count += 1;
                self.cancel_order_count += 1;
                push_if_present(&mut self.cancelled_queue_ids, order.queue_id.as_deref());
            }
            RtsOrderKind::Refund => {
                self.lifecycle_order_count += 1;
                self.refund_order_count += 1;
                push_if_present(&mut self.refund_rule_ids, order.target_rule_id.as_deref());
                push_if_present(&mut self.refund_delta_labels, order.queue_id.as_deref());
            }
            _ => {}
        }
    }
}

fn push_if_present(values: &mut Vec<String>, value: Option<&str>) {
    if let Some(value) = value {
        if !value.is_empty() {
            values.push(value.to_string());
        }
    }
}

fn order_target_label(order: &RtsFrameOrder) -> String {
    match order.kind {
        RtsOrderKind::Research | RtsOrderKind::Upgrade => {
            if let (Some(rule_id), Some(source_id)) = (
                order.target_rule_id.as_deref(),
                order.target_actor_id.as_deref(),
            ) {
                return format!("{rule_id}@{source_id}");
            }
        }
        RtsOrderKind::Unlock => {
            if let Some(rule_id) = order.target_rule_id.as_deref() {
                return rule_id.to_string();
            }
        }
        RtsOrderKind::Ability => {
            if let (Some(ability_id), Some(target_id)) = (
                order.target_rule_id.as_deref(),
                order.target_actor_id.as_deref(),
            ) {
                return format!("{ability_id}@{target_id}");
            }
        }
        _ => {}
    }
    order
        .target_tile
        .map(RtsTile::label)
        .or_else(|| order.target_actor_id.clone())
        .or_else(|| order.target_rule_id.clone())
        .or_else(|| order.queue_id.clone())
        .unwrap_or_else(|| "none".to_string())
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn selected_subjects() -> Vec<String> {
        vec!["player".to_string(), "square_worker_harvest".to_string()]
    }

    fn live_right_click_stream() -> RtsFrameOrderStream {
        let labels = [
            "RTS:MOVE:4,3:line",
            "RTS:ATTACK:square_creep_wander",
            "RTS:MOVE:5,4:follow:player",
            "RTS:QUEUE:harvest:gold_vein",
        ];
        let orders = labels
            .iter()
            .enumerate()
            .map(|(index, label)| {
                RtsFrameOrder::from_live_command_label(
                    420 + index as u32,
                    "Multi0",
                    selected_subjects(),
                    label,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        RtsFrameOrderStream::new("first-contact-basin-live-input", "trnm-rules-v1", orders)
    }

    #[test]
    fn live_right_click_labels_become_frame_orders() {
        let move_order = RtsFrameOrder::from_live_command_label(
            120,
            "Multi0",
            selected_subjects(),
            "RTS:MOVE:4,3:line",
        )
        .unwrap();
        assert_eq!(move_order.kind, RtsOrderKind::Move);
        assert_eq!(move_order.target_tile, Some(RtsTile::new(4, 3)));
        assert_eq!(move_order.formation_id.as_deref(), Some("line"));

        let attack_order = RtsFrameOrder::from_live_command_label(
            121,
            "Multi0",
            selected_subjects(),
            "RTS:ATTACK:square_creep_wander",
        )
        .unwrap();
        assert_eq!(attack_order.kind, RtsOrderKind::Attack);
        assert_eq!(
            attack_order.target_actor_id.as_deref(),
            Some("square_creep_wander")
        );

        let follow_order = RtsFrameOrder::from_live_command_label(
            122,
            "Multi0",
            selected_subjects(),
            "RTS:MOVE:5,4:follow:player",
        )
        .unwrap();
        assert_eq!(follow_order.kind, RtsOrderKind::Follow);
        assert_eq!(follow_order.target_tile, Some(RtsTile::new(5, 4)));
        assert_eq!(follow_order.target_actor_id.as_deref(), Some("player"));

        let harvest_order = RtsFrameOrder::from_live_command_label(
            123,
            "Multi0",
            selected_subjects(),
            "RTS:QUEUE:harvest:gold_vein",
        )
        .unwrap();
        assert_eq!(harvest_order.kind, RtsOrderKind::Harvest);
        assert!(harvest_order.queued);
        assert_eq!(harvest_order.target_actor_id.as_deref(), Some("gold_vein"));
    }

    #[test]
    fn build_and_train_queue_labels_keep_rule_identity() {
        let build_order = RtsFrameOrder::from_live_command_label(
            10,
            "Multi0",
            vec!["builder".to_string()],
            "RTS:QUEUE:build:watch_tower@7,4",
        )
        .unwrap();
        assert_eq!(build_order.kind, RtsOrderKind::Build);
        assert_eq!(build_order.target_rule_id.as_deref(), Some("watch_tower"));
        assert_eq!(build_order.target_tile, Some(RtsTile::new(7, 4)));

        let train_order = RtsFrameOrder::from_live_command_label(
            11,
            "Multi0",
            vec!["barracks".to_string()],
            "RTS:QUEUE:train:guard",
        )
        .unwrap();
        assert_eq!(train_order.kind, RtsOrderKind::Train);
        assert_eq!(train_order.target_rule_id.as_deref(), Some("guard"));
    }

    #[test]
    fn tech_tree_queue_labels_keep_tech_identity() {
        let research_order = RtsFrameOrder::from_live_command_label(
            16,
            "Multi0",
            vec!["town_hall".to_string()],
            "RTS:QUEUE:research:wayfinder_code@town_hall",
        )
        .unwrap();
        assert_eq!(research_order.kind, RtsOrderKind::Research);
        assert_eq!(
            research_order.target_rule_id.as_deref(),
            Some("wayfinder_code")
        );
        assert_eq!(research_order.target_actor_id.as_deref(), Some("town_hall"));

        let upgrade_order = RtsFrameOrder::from_live_command_label(
            17,
            "Multi0",
            vec!["training_hall".to_string()],
            "RTS:QUEUE:upgrade:iron_lacing@training_hall",
        )
        .unwrap();
        assert_eq!(upgrade_order.kind, RtsOrderKind::Upgrade);
        assert_eq!(upgrade_order.target_rule_id.as_deref(), Some("iron_lacing"));
        assert_eq!(
            upgrade_order.target_actor_id.as_deref(),
            Some("training_hall")
        );

        let unlock_order = RtsFrameOrder::from_live_command_label(
            18,
            "Multi0",
            vec!["signal_spire".to_string()],
            "RTS:QUEUE:unlock:relay_guard",
        )
        .unwrap();
        assert_eq!(unlock_order.kind, RtsOrderKind::Unlock);
        assert_eq!(unlock_order.target_rule_id.as_deref(), Some("relay_guard"));
        assert!(unlock_order.target_actor_id.is_none());
    }

    #[test]
    fn ability_labels_keep_ability_identity() {
        let ability_order = RtsFrameOrder::from_live_command_label(
            19,
            "Multi0",
            selected_subjects(),
            "RTS:ABILITY:focus_fire",
        )
        .unwrap();
        assert_eq!(ability_order.kind, RtsOrderKind::Ability);
        assert_eq!(ability_order.target_rule_id.as_deref(), Some("focus_fire"));
        assert!(ability_order.target_actor_id.is_none());
        assert!(!ability_order.queued);

        let commander_ability_order = RtsFrameOrder::from_live_command_label(
            20,
            "Multi0",
            vec!["mirror_captain".to_string()],
            "RTS:QUEUE:commander:ability:rally_aura@mirror_captain",
        )
        .unwrap();
        assert_eq!(commander_ability_order.kind, RtsOrderKind::Ability);
        assert_eq!(
            commander_ability_order.target_rule_id.as_deref(),
            Some("rally_aura")
        );
        assert_eq!(
            commander_ability_order.target_actor_id.as_deref(),
            Some("mirror_captain")
        );
        assert!(commander_ability_order.queued);
        assert_eq!(
            commander_ability_order.queue_id.as_deref(),
            Some("commander:ability:rally_aura@mirror_captain")
        );
    }

    #[test]
    fn build_lifecycle_queue_labels_keep_lifecycle_identity() {
        let complete_order = RtsFrameOrder::from_live_command_label(
            12,
            "Multi0",
            vec!["builder".to_string()],
            "RTS:QUEUE:complete:watch_tower@7,4",
        )
        .unwrap();
        assert_eq!(complete_order.kind, RtsOrderKind::Complete);
        assert_eq!(
            complete_order.target_rule_id.as_deref(),
            Some("watch_tower")
        );
        assert_eq!(complete_order.target_tile, Some(RtsTile::new(7, 4)));

        let repair_order = RtsFrameOrder::from_live_command_label(
            13,
            "Multi0",
            vec!["builder".to_string()],
            "RTS:QUEUE:repair:watch_tower@7,4",
        )
        .unwrap();
        assert_eq!(repair_order.kind, RtsOrderKind::Repair);
        assert_eq!(repair_order.target_actor_id.as_deref(), Some("watch_tower"));
        assert_eq!(repair_order.target_tile, Some(RtsTile::new(7, 4)));

        let cancel_order = RtsFrameOrder::from_live_command_label(
            14,
            "Multi0",
            vec!["builder".to_string()],
            "RTS:QUEUE:cancel:build:1",
        )
        .unwrap();
        assert_eq!(cancel_order.kind, RtsOrderKind::Cancel);
        assert_eq!(cancel_order.queue_id.as_deref(), Some("build:1"));

        let refund_order = RtsFrameOrder::from_live_command_label(
            15,
            "Multi0",
            vec!["builder".to_string()],
            "RTS:QUEUE:refund:scout_tower@8,4:gold:+180",
        )
        .unwrap();
        assert_eq!(refund_order.kind, RtsOrderKind::Refund);
        assert_eq!(refund_order.target_rule_id.as_deref(), Some("scout_tower"));
        assert_eq!(refund_order.target_tile, Some(RtsTile::new(8, 4)));
        assert_eq!(refund_order.queue_id.as_deref(), Some("gold:+180"));
    }

    #[test]
    fn live_command_modes_keep_order_kind_identity() {
        for (label, expected_kind, queued) in [
            ("RTS:MOVE:9,4:shift_waypoint", RtsOrderKind::Move, true),
            ("RTS:MOVE:6,5:hold", RtsOrderKind::Hold, false),
            ("RTS:MOVE:9,4:patrol", RtsOrderKind::Patrol, false),
            ("RTS:MOVE:10,3:attack_move", RtsOrderKind::AttackMove, false),
            ("RTS:MOVE:10,3:stop", RtsOrderKind::Stop, false),
        ] {
            let order =
                RtsFrameOrder::from_live_command_label(20, "Multi0", selected_subjects(), label)
                    .unwrap();
            assert_eq!(order.kind, expected_kind);
            assert_eq!(order.queued, queued);
            assert!(order.target_tile.is_some());
        }
    }

    #[test]
    fn frame_stream_digest_is_stable_and_order_sensitive() {
        let order = RtsFrameOrder::from_live_command_label(
            7,
            "Multi0",
            selected_subjects(),
            "RTS:MOVE:4,3:line",
        )
        .unwrap();
        let stream = RtsFrameOrderStream::new("first-contact-basin", "trnm-rules-v1", vec![order]);
        stream.validate().unwrap();
        let digest = stream.sha256_hex();
        assert_eq!(digest.len(), 64);
        assert_eq!(digest, stream.sha256_hex());

        let mut changed = stream.clone();
        changed.orders[0].frame = 8;
        assert_ne!(digest, changed.sha256_hex());
    }

    #[test]
    fn validation_rejects_missing_move_tile_and_frame_regression() {
        let bad_order = RtsFrameOrder::new(
            1,
            "Multi0",
            vec!["player".to_string()],
            RtsOrderKind::Move,
            RtsOrderSource::LocalInput,
        );
        assert_eq!(
            bad_order.validate().unwrap_err(),
            "move_target_tile_missing"
        );

        let order_a = RtsFrameOrder::from_live_command_label(
            3,
            "Multi0",
            selected_subjects(),
            "RTS:MOVE:4,3:line",
        )
        .unwrap();
        let order_b = RtsFrameOrder::from_live_command_label(
            2,
            "Multi0",
            selected_subjects(),
            "RTS:ATTACK:square_creep_wander",
        )
        .unwrap();
        let stream = RtsFrameOrderStream::new(
            "first-contact-basin",
            "trnm-rules-v1",
            vec![order_a, order_b],
        );
        assert_eq!(stream.validate().unwrap_err(), "frame_regression:3->2");
    }

    #[test]
    fn headless_replay_checkpoint_tracks_actor_and_player_state() {
        let report = live_right_click_stream().replay_headless().unwrap();
        assert_eq!(report.contract, TRNM_RTS_CORE_CONTRACT);
        assert_eq!(report.checkpoint.applied_order_count, 4);
        assert_eq!(report.checkpoint.final_frame, 423);
        assert_eq!(report.checkpoint.player_count, 1);
        assert_eq!(report.checkpoint.actor_count, 2);
        assert_eq!(report.checkpoint.players[0].issued_order_count, 4);
        assert_eq!(report.checkpoint.players[0].queued_order_count, 1);
        assert_eq!(report.checkpoint.event_log.len(), 4);
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:attack:")));

        let player = report
            .checkpoint
            .actors
            .iter()
            .find(|actor| actor.actor_id == "player")
            .unwrap();
        assert_eq!(player.last_order_kind, Some(RtsOrderKind::Harvest));
        assert_eq!(player.target_actor_id.as_deref(), Some("gold_vein"));
        assert_eq!(player.tile, Some(RtsTile::new(5, 4)));
        assert_eq!(player.queued_order_count, 1);
        assert_eq!(player.attack_order_count, 1);
        assert_eq!(player.harvest_order_count, 1);
        assert_eq!(player.command_history.len(), 4);
        assert_eq!(report.checkpoint.sha256_hex(), report.checkpoint_sha256);
        assert_eq!(report.checkpoint_sha256.len(), 64);
    }

    #[test]
    fn headless_replay_checkpoint_hash_is_deterministic_and_order_sensitive() {
        let stream = live_right_click_stream();
        let first = stream.replay_headless().unwrap();
        let second = stream.replay_headless().unwrap();
        assert_eq!(first.checkpoint_sha256, second.checkpoint_sha256);

        let mut changed = stream.clone();
        changed.orders[3].target_actor_id = Some("blue_crystal".to_string());
        let changed = changed.replay_headless().unwrap();
        assert_ne!(first.checkpoint_sha256, changed.checkpoint_sha256);
    }

    #[test]
    fn headless_replay_tracks_expanded_live_command_stream() {
        let labels = [
            "RTS:QUEUE:train:guard",
            "RTS:MOVE:7,4:diamond",
            "RTS:MOVE:9,4:shift_waypoint",
            "RTS:MOVE:6,5:hold",
            "RTS:MOVE:9,4:patrol",
            "RTS:MOVE:10,3:attack_move",
            "RTS:MOVE:10,3:stop",
            "RTS:ATTACK:arena_creep_attack",
            "RTS:ABILITY:focus_fire",
        ];
        let mut orders = labels
            .iter()
            .enumerate()
            .map(|(index, label)| {
                RtsFrameOrder::from_live_command_label(
                    300 + index as u32,
                    "Multi0",
                    selected_subjects(),
                    label,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let focus_fire = orders
            .iter_mut()
            .find(|order| order.kind == RtsOrderKind::Ability)
            .unwrap();
        focus_fire.target_actor_id = Some("arena_creep_attack".to_string());
        focus_fire.validate().unwrap();
        orders.extend(live_right_click_stream().orders);
        let stream =
            RtsFrameOrderStream::new("first-contact-basin-live-input", "trnm-rules-v1", orders);
        let report = stream.replay_headless().unwrap();
        assert_eq!(report.checkpoint.applied_order_count, 13);
        assert_eq!(report.checkpoint.final_frame, 423);
        assert_eq!(report.checkpoint.abilities.ability_order_count, 1);
        assert!(report
            .checkpoint
            .abilities
            .ability_rule_ids
            .iter()
            .any(|rule| rule == "focus_fire"));
        assert!(report
            .checkpoint
            .abilities
            .target_actor_ids
            .iter()
            .any(|target| target == "arena_creep_attack"));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:train:")));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:hold:")));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:patrol:")));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:attack_move:")));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:stop:")));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:ability:")));
    }

    #[test]
    fn headless_replay_tracks_build_lifecycle_stream() {
        let labels = [
            "RTS:QUEUE:build:watch_tower@7,4",
            "RTS:QUEUE:complete:watch_tower@7,4",
            "RTS:QUEUE:repair:watch_tower@7,4",
            "RTS:QUEUE:build:scout_tower@8,4",
            "RTS:QUEUE:cancel:build:1",
            "RTS:QUEUE:refund:scout_tower@8,4:gold:+180",
        ];
        let mut orders = labels
            .iter()
            .enumerate()
            .map(|(index, label)| {
                RtsFrameOrder::from_live_command_label(
                    500 + index as u32,
                    "Multi0",
                    vec!["builder".to_string()],
                    label,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let cancel = orders
            .iter_mut()
            .find(|order| order.kind == RtsOrderKind::Cancel)
            .unwrap();
        cancel.queue_id = Some("cancel:build:scout_tower@8,4".to_string());
        cancel.target_rule_id = Some("scout_tower".to_string());
        cancel.target_tile = Some(RtsTile::new(8, 4));
        cancel.validate().unwrap();

        let stream = RtsFrameOrderStream::new(
            "first-contact-basin-build-lifecycle",
            "trnm-rules-v1",
            orders,
        );
        let report = stream.replay_headless().unwrap();
        let lifecycle = &report.checkpoint.production_lifecycle;
        assert_eq!(report.checkpoint.applied_order_count, 6);
        assert_eq!(report.checkpoint.final_frame, 505);
        assert_eq!(lifecycle.lifecycle_order_count, 6);
        assert_eq!(lifecycle.build_order_count, 2);
        assert_eq!(lifecycle.complete_order_count, 1);
        assert_eq!(lifecycle.repair_order_count, 1);
        assert_eq!(lifecycle.cancel_order_count, 1);
        assert_eq!(lifecycle.refund_order_count, 1);
        assert!(lifecycle
            .build_rule_ids
            .iter()
            .any(|rule| rule == "watch_tower"));
        assert!(lifecycle
            .refund_rule_ids
            .iter()
            .any(|rule| rule == "scout_tower"));
        assert!(lifecycle
            .refund_delta_labels
            .iter()
            .any(|delta| delta == "gold:+180"));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:build:")));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:complete:")));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:repair:")));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:cancel:")));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:refund:")));

        let builder = report
            .checkpoint
            .actors
            .iter()
            .find(|actor| actor.actor_id == "builder")
            .unwrap();
        assert_eq!(builder.build_order_count, 2);
        assert_eq!(builder.complete_order_count, 1);
        assert_eq!(builder.repair_order_count, 1);
        assert_eq!(builder.cancel_order_count, 1);
        assert_eq!(builder.refund_order_count, 1);
        assert_eq!(builder.target_rule_id.as_deref(), Some("scout_tower"));
        assert_eq!(builder.target_tile, Some(RtsTile::new(8, 4)));
        assert_eq!(builder.queue_id.as_deref(), Some("gold:+180"));
    }

    #[test]
    fn headless_replay_tracks_tech_tree_stream() {
        let labels = [
            "RTS:QUEUE:build:training_hall@4,3",
            "RTS:QUEUE:research:wayfinder_code@town_hall",
            "RTS:QUEUE:upgrade:iron_lacing@training_hall",
            "RTS:QUEUE:unlock:relay_guard",
        ];
        let orders = labels
            .iter()
            .enumerate()
            .map(|(index, label)| {
                RtsFrameOrder::from_live_command_label(
                    600 + index as u32,
                    "Multi0",
                    vec!["tech_lane".to_string()],
                    label,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let stream =
            RtsFrameOrderStream::new("first-contact-basin-tech-tree", "trnm-rules-v1", orders);
        let report = stream.replay_headless().unwrap();
        let tech_tree = &report.checkpoint.tech_tree;
        assert_eq!(report.checkpoint.applied_order_count, 4);
        assert_eq!(report.checkpoint.final_frame, 603);
        assert_eq!(tech_tree.tech_order_count, 3);
        assert_eq!(tech_tree.research_order_count, 1);
        assert_eq!(tech_tree.upgrade_order_count, 1);
        assert_eq!(tech_tree.unlock_order_count, 1);
        assert!(tech_tree
            .researched_rule_ids
            .iter()
            .any(|rule| rule == "wayfinder_code"));
        assert!(tech_tree
            .upgraded_rule_ids
            .iter()
            .any(|rule| rule == "iron_lacing"));
        assert!(tech_tree
            .unlocked_rule_ids
            .iter()
            .any(|rule| rule == "relay_guard"));
        assert!(tech_tree
            .source_actor_ids
            .iter()
            .any(|source| source == "town_hall"));
        assert!(tech_tree
            .source_actor_ids
            .iter()
            .any(|source| source == "training_hall"));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:research:")));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:upgrade:")));
        assert!(report
            .checkpoint
            .event_log
            .iter()
            .any(|event| event.contains(":kind:unlock:")));

        let tech_lane = report
            .checkpoint
            .actors
            .iter()
            .find(|actor| actor.actor_id == "tech_lane")
            .unwrap();
        assert_eq!(tech_lane.build_order_count, 1);
        assert_eq!(tech_lane.research_order_count, 1);
        assert_eq!(tech_lane.upgrade_order_count, 1);
        assert_eq!(tech_lane.unlock_order_count, 1);
        assert_eq!(tech_lane.target_rule_id.as_deref(), Some("relay_guard"));
    }
}
