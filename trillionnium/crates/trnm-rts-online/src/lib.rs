//! Bevy-free RTS online protocol sketches for shared arena state.
//!
//! This crate does not open sockets. It owns deterministic protocol fixtures that
//! future server/client code can replace without changing release-review evidence.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use trnm_rts_core::{RtsFrameOrder, RtsOrderKind, RtsOrderSource, RtsTile};

pub const TRNM_RTS_ONLINE_CONTRACT: &str = "trnm_rts_online_protocol_v1";
pub const TRNM_RTS_ONLINE_FIRST_CONTACT_FIXTURE_CONTRACT: &str =
    "trnm_rts_online_first_contact_fixture_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RtsOnlineChunkId {
    pub x: i32,
    pub y: i32,
}

impl RtsOnlineChunkId {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsOnlineArenaPhase {
    Lobby,
    Loading,
    Playing,
    Paused,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineVisibilityScope {
    pub player_id: String,
    pub tick: u32,
    pub visible_chunks: Vec<RtsOnlineChunkId>,
    pub visible_actor_ids: Vec<String>,
    pub fogged_chunks: Vec<RtsOnlineChunkId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineUpdateEnvelope {
    pub contract_version: String,
    pub arena_id: String,
    pub map_id: String,
    pub tick: u32,
    pub scope: RtsOnlineVisibilityScope,
    pub orders: Vec<RtsFrameOrder>,
    pub update_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineBotPlan {
    pub bot_id: String,
    pub player_id: String,
    pub tick: u32,
    pub visible_chunks: Vec<RtsOnlineChunkId>,
    pub order_labels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineArenaLifecycle {
    pub arena_id: String,
    pub map_id: String,
    pub phase: RtsOnlineArenaPhase,
    pub connected_player_ids: Vec<String>,
    pub bot_count: usize,
    pub source_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineProtocolFixture {
    pub contract_version: String,
    pub lifecycle: RtsOnlineArenaLifecycle,
    pub envelope: RtsOnlineUpdateEnvelope,
    pub bot_plan: RtsOnlineBotPlan,
    pub green: bool,
}

fn update_hash_input(
    arena_id: &str,
    map_id: &str,
    tick: u32,
    scope: &RtsOnlineVisibilityScope,
    orders: &[RtsFrameOrder],
) -> String {
    serde_json::to_string(&serde_json::json!({
        "contract_version": TRNM_RTS_ONLINE_CONTRACT,
        "arena_id": arena_id,
        "map_id": map_id,
        "tick": tick,
        "scope": scope,
        "orders": orders,
    }))
    .expect("RTS online hash input serializes")
}

pub fn rts_online_update_sha256(
    arena_id: &str,
    map_id: &str,
    tick: u32,
    scope: &RtsOnlineVisibilityScope,
    orders: &[RtsFrameOrder],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(update_hash_input(arena_id, map_id, tick, scope, orders));
    format!("{:x}", hasher.finalize())
}

pub fn first_contact_online_protocol_fixture() -> RtsOnlineProtocolFixture {
    let arena_id = "first-contact-local-arena";
    let map_id = "first_contact_basin";
    let tick = 42_u32;
    let player_id = "mirror_guard";
    let visible_chunks = vec![
        RtsOnlineChunkId::new(0, 0),
        RtsOnlineChunkId::new(1, 0),
        RtsOnlineChunkId::new(0, 1),
    ];
    let fogged_chunks = vec![RtsOnlineChunkId::new(1, 1), RtsOnlineChunkId::new(2, 1)];
    let visible_actor_ids = vec![
        "trnm.worker.alpha".to_string(),
        "trnm.horizon.scout.alpha".to_string(),
        "trnm.command.core.alpha".to_string(),
        "trnm.flux.beacon.center".to_string(),
    ];

    let mut bot_order = RtsFrameOrder::new(
        tick,
        player_id,
        vec!["trnm.worker.alpha".to_string()],
        RtsOrderKind::Move,
        RtsOrderSource::Bot,
    );
    bot_order.queued = true;
    bot_order.target_tile = Some(RtsTile::new(8, 4));
    bot_order.formation_id = Some("rally".to_string());
    bot_order.raw_command_label = Some("bot:worker_rally_to_beacon".to_string());

    let scope = RtsOnlineVisibilityScope {
        player_id: player_id.to_string(),
        tick,
        visible_chunks: visible_chunks.clone(),
        visible_actor_ids: visible_actor_ids.clone(),
        fogged_chunks,
    };
    let orders = vec![bot_order];
    let update_sha256 = rts_online_update_sha256(arena_id, map_id, tick, &scope, &orders);
    let envelope = RtsOnlineUpdateEnvelope {
        contract_version: TRNM_RTS_ONLINE_CONTRACT.to_string(),
        arena_id: arena_id.to_string(),
        map_id: map_id.to_string(),
        tick,
        scope,
        orders,
        update_sha256,
    };
    let lifecycle = RtsOnlineArenaLifecycle {
        arena_id: arena_id.to_string(),
        map_id: map_id.to_string(),
        phase: RtsOnlineArenaPhase::Playing,
        connected_player_ids: vec!["local-player".to_string(), player_id.to_string()],
        bot_count: 1,
        source_policy:
            "project_owned_protocol_sketch_no_socket_no_hosted_service_public_launch_false"
                .to_string(),
    };
    let bot_plan = RtsOnlineBotPlan {
        bot_id: "first-contact-baseline-bot".to_string(),
        player_id: player_id.to_string(),
        tick,
        visible_chunks,
        order_labels: vec!["move:rally@8,4".to_string()],
    };
    let green = envelope.contract_version == TRNM_RTS_ONLINE_CONTRACT
        && envelope.map_id == map_id
        && envelope.tick == tick
        && envelope.update_sha256.len() == 64
        && envelope.scope.visible_chunks.len() == 3
        && envelope.scope.fogged_chunks.len() == 2
        && envelope
            .scope
            .visible_actor_ids
            .iter()
            .any(|actor_id| actor_id == "trnm.flux.beacon.center")
        && envelope.orders.iter().any(|order| {
            order.source == RtsOrderSource::Bot && order.target_tile == Some(RtsTile::new(8, 4))
        })
        && lifecycle.phase == RtsOnlineArenaPhase::Playing
        && lifecycle.bot_count == 1
        && bot_plan.visible_chunks == envelope.scope.visible_chunks;

    RtsOnlineProtocolFixture {
        contract_version: TRNM_RTS_ONLINE_FIRST_CONTACT_FIXTURE_CONTRACT.to_string(),
        lifecycle,
        envelope,
        bot_plan,
        green,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_online_protocol_fixture_is_green() {
        let fixture = first_contact_online_protocol_fixture();

        assert_eq!(
            fixture.contract_version,
            TRNM_RTS_ONLINE_FIRST_CONTACT_FIXTURE_CONTRACT
        );
        assert!(fixture.green);
        assert_eq!(fixture.envelope.contract_version, TRNM_RTS_ONLINE_CONTRACT);
        assert_eq!(fixture.envelope.map_id, "first_contact_basin");
        assert_eq!(fixture.envelope.scope.visible_chunks.len(), 3);
        assert_eq!(fixture.envelope.scope.fogged_chunks.len(), 2);
        assert_eq!(fixture.envelope.update_sha256.len(), 64);
        assert_eq!(fixture.lifecycle.phase, RtsOnlineArenaPhase::Playing);
        assert_eq!(fixture.bot_plan.order_labels, vec!["move:rally@8,4"]);
    }
}
