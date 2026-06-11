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
    Capture,
    Attack,
    FocusFire,
    Repair,
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
            RtsOrderKind::Capture => "capture",
            RtsOrderKind::Attack => "attack",
            RtsOrderKind::FocusFire => "focus_fire",
            RtsOrderKind::Repair => "repair",
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
    let mut order = RtsFrameOrder::new(
        frame,
        player_id,
        subject_actor_ids,
        if mode == "follow" {
            RtsOrderKind::Follow
        } else {
            RtsOrderKind::Move
        },
        RtsOrderSource::LocalInput,
    );
    order.target_tile = Some(tile);
    order.formation_id = Some(mode.to_string());
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
                    RtsOrderKind::Harvest | RtsOrderKind::ReturnCargo => {
                        actor.target_actor_id = order.target_actor_id.clone();
                        actor.target_tile = order.target_tile;
                        actor.harvest_order_count += 1;
                    }
                    RtsOrderKind::Build => {
                        actor.target_rule_id = order.target_rule_id.clone();
                        actor.tile = order.target_tile;
                    }
                    RtsOrderKind::Train => {
                        actor.target_rule_id = order.target_rule_id.clone();
                    }
                    RtsOrderKind::Capture | RtsOrderKind::Repair => {
                        actor.target_actor_id = order.target_actor_id.clone();
                        actor.target_tile = order.target_tile;
                    }
                    RtsOrderKind::Queue => {
                        actor.queue_id = order.queue_id.clone();
                    }
                    RtsOrderKind::Stop | RtsOrderKind::Hold => {}
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
            attack_order_count: 0,
            harvest_order_count: 0,
            command_history: Vec::new(),
        }
    }
}

fn order_target_label(order: &RtsFrameOrder) -> String {
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
}
