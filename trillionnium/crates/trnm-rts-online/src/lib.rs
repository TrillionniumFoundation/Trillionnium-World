//! Bevy-free RTS online protocol sketches for shared arena state.
//!
//! This crate does not open sockets. It owns deterministic protocol fixtures that
//! future server/client code can replace without changing release-review evidence.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use trnm_rts_bevy_runtime::{
    rts_control_group_command_feedback_rejection_replay_fixtures,
    rts_control_group_command_feedback_replay_fixtures,
};
use trnm_rts_core::{RtsFrameOrder, RtsOrderKind, RtsOrderSource, RtsTile};

pub const TRNM_RTS_ONLINE_CONTRACT: &str = "trnm_rts_online_protocol_v1";
pub const TRNM_RTS_ONLINE_FIRST_CONTACT_FIXTURE_CONTRACT: &str =
    "trnm_rts_online_first_contact_fixture_v1";
pub const TRNM_RTS_ONLINE_AUTHORITY_CONTRACT: &str = "trnm_rts_online_authority_v1";
pub const TRNM_RTS_ONLINE_LOOPBACK_TRANSPORT_CONTRACT: &str =
    "trnm_rts_online_loopback_transport_v1";
pub const TRNM_RTS_ONLINE_LOCAL_HANDOFF_CONTRACT: &str = "trnm_rts_online_local_handoff_v1";
pub const TRNM_RTS_ONLINE_OFFLINE_ADAPTER_CONTRACT: &str = "trnm_rts_online_offline_adapter_v1";
pub const TRNM_RTS_ONLINE_OFFLINE_ADAPTER_LOCAL_REPLAY_CONTRACT: &str =
    "trnm_rts_online_offline_adapter_local_replay_v1";
const TRNM_RTS_ONLINE_WIRE_MAGIC: &[u8; 8] = b"TRNMRTS1";

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
pub struct RtsOnlineClientRequest {
    pub request_id: String,
    pub player_id: String,
    pub client_tick: u32,
    pub acknowledged_update_sha256: String,
    pub orders: Vec<RtsFrameOrder>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineRejectedOrder {
    pub player_id: String,
    pub raw_command_label: Option<String>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineAuthorityResolution {
    pub contract_version: String,
    pub arena_id: String,
    pub map_id: String,
    pub authority_tick: u32,
    pub client_requests: Vec<RtsOnlineClientRequest>,
    pub accepted_orders: Vec<RtsFrameOrder>,
    pub rejected_orders: Vec<RtsOnlineRejectedOrder>,
    pub scoped_updates: Vec<RtsOnlineUpdateEnvelope>,
    pub authority_sha256: String,
    pub green: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsOnlineTransportDirection {
    ClientToServer,
    ServerToClient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsOnlineTransportPayloadKind {
    ClientRequest,
    ScopedUpdate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineTransportFrame {
    pub contract_version: String,
    pub direction: RtsOnlineTransportDirection,
    pub payload_kind: RtsOnlineTransportPayloadKind,
    pub sequence: u32,
    pub arena_id: String,
    pub player_id: String,
    pub wire_magic: String,
    pub encoded_len: usize,
    pub payload_sha256: String,
    pub frame_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineLoopbackTransportFixture {
    pub contract_version: String,
    pub session_id: String,
    pub arena_id: String,
    pub map_id: String,
    pub request_frame: RtsOnlineTransportFrame,
    pub response_frame: RtsOnlineTransportFrame,
    pub request_ack_matches_envelope: bool,
    pub response_matches_authority: bool,
    pub server_authoritative: bool,
    pub visibility_scoped_response: bool,
    pub socket_opened: bool,
    pub hosted_service_claimed: bool,
    pub public_launch_ready: bool,
    pub green: bool,
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
    pub authority: RtsOnlineAuthorityResolution,
    pub transport: RtsOnlineLoopbackTransportFixture,
    pub bot_plan: RtsOnlineBotPlan,
    pub green: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineLocalHandoff {
    pub contract_version: String,
    pub handoff_id: String,
    pub arena_id: String,
    pub map_id: String,
    pub player_id: String,
    pub phase: RtsOnlineArenaPhase,
    pub authority_tick: u32,
    pub accepted_order_count: usize,
    pub rejected_order_count: usize,
    pub scoped_update_count: usize,
    pub bot_count: usize,
    pub visible_chunk_count: usize,
    pub visible_actor_count: usize,
    pub loopback_session_id: String,
    pub request_frame_sha256: String,
    pub response_frame_sha256: String,
    pub bevy_client_role: String,
    pub authority_role: String,
    pub server_authoritative: bool,
    pub visibility_scoped_response: bool,
    pub socket_opened: bool,
    pub hosted_service_claimed: bool,
    pub public_launch_ready: bool,
    pub handoff_ready: bool,
    pub green: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineOfflineAdapterLocalReplay {
    pub contract_version: String,
    pub replay_mode: String,
    pub accepted_action_labels: Vec<String>,
    pub accepted_preview_stages: Vec<String>,
    pub blocked_action_labels: Vec<String>,
    pub blocked_input_sources: Vec<String>,
    pub blocked_reasons: Vec<String>,
    pub blocked_preview_stages: Vec<String>,
    pub retained_history_group_ids: Vec<String>,
    pub pruned_history_group_ids: Vec<String>,
    pub command_history_capacity: usize,
    pub local_input_sources_ready: bool,
    pub command_history_ready: bool,
    pub green: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOnlineOfflineAdapterSummary {
    pub contract_version: String,
    pub adapter_id: String,
    pub handoff_id: String,
    pub arena_id: String,
    pub map_id: String,
    pub adapter_mode: String,
    pub bevy_client_role: String,
    pub authority_role: String,
    pub connected_player_ids: Vec<String>,
    pub bot_player_ids: Vec<String>,
    pub input_queue_labels: Vec<String>,
    pub accepted_server_order_labels: Vec<String>,
    pub rejected_client_order_reasons: Vec<String>,
    pub scoped_update_actor_ids: Vec<String>,
    pub scoped_update_order_count: usize,
    pub frame_sha256s: Vec<String>,
    pub local_action_replay: RtsOnlineOfflineAdapterLocalReplay,
    pub local_multiplayer_ready: bool,
    pub offline_bot_ready: bool,
    pub bevy_adapter_ready: bool,
    pub server_authoritative: bool,
    pub visibility_scoped_response: bool,
    pub client_prediction_claimed: bool,
    pub rollback_netcode_claimed: bool,
    pub socket_opened: bool,
    pub hosted_service_claimed: bool,
    pub public_launch_ready: bool,
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

fn chunk_for_tile(tile: RtsTile, chunk_size: i32) -> RtsOnlineChunkId {
    RtsOnlineChunkId::new(tile.x.div_euclid(chunk_size), tile.y.div_euclid(chunk_size))
}

fn scope_contains_actor(scope: &RtsOnlineVisibilityScope, actor_id: &str) -> bool {
    scope
        .visible_actor_ids
        .iter()
        .any(|visible_actor_id| visible_actor_id == actor_id)
}

fn scope_contains_tile(scope: &RtsOnlineVisibilityScope, tile: RtsTile) -> bool {
    let tile_chunk = chunk_for_tile(tile, 16);
    scope
        .visible_chunks
        .iter()
        .any(|chunk| *chunk == tile_chunk)
}

fn authority_rejection_reason(
    order: &RtsFrameOrder,
    scope: &RtsOnlineVisibilityScope,
) -> Option<String> {
    for subject_actor_id in &order.subject_actor_ids {
        if !scope_contains_actor(scope, subject_actor_id) {
            return Some("subject_actor_not_visible".to_string());
        }
    }
    if let Some(target_actor_id) = order.target_actor_id.as_deref() {
        if !scope_contains_actor(scope, target_actor_id) {
            return Some("target_actor_not_visible".to_string());
        }
    }
    if let Some(target_tile) = order.target_tile {
        if !scope_contains_tile(scope, target_tile) {
            return Some("target_tile_not_visible".to_string());
        }
    }
    None
}

fn authority_hash_input(resolution: &RtsOnlineAuthorityResolution) -> String {
    serde_json::to_string(&serde_json::json!({
        "contract_version": resolution.contract_version,
        "arena_id": resolution.arena_id,
        "map_id": resolution.map_id,
        "authority_tick": resolution.authority_tick,
        "client_requests": resolution.client_requests,
        "accepted_orders": resolution.accepted_orders,
        "rejected_orders": resolution.rejected_orders,
        "scoped_update_hashes": resolution.scoped_updates
            .iter()
            .map(|update| update.update_sha256.as_str())
            .collect::<Vec<_>>(),
    }))
    .expect("RTS online authority hash input serializes")
}

pub fn rts_online_authority_sha256(resolution: &RtsOnlineAuthorityResolution) -> String {
    let mut hasher = Sha256::new();
    hasher.update(authority_hash_input(resolution));
    format!("{:x}", hasher.finalize())
}

pub fn rts_online_authority_resolve(
    arena_id: &str,
    map_id: &str,
    authority_tick: u32,
    scopes: &[RtsOnlineVisibilityScope],
    client_requests: Vec<RtsOnlineClientRequest>,
) -> RtsOnlineAuthorityResolution {
    let mut accepted_orders = Vec::new();
    let mut rejected_orders = Vec::new();

    for request in &client_requests {
        if let Some(scope) = scopes
            .iter()
            .find(|scope| scope.player_id == request.player_id)
        {
            for order in &request.orders {
                if let Some(reason) = authority_rejection_reason(order, scope) {
                    rejected_orders.push(RtsOnlineRejectedOrder {
                        player_id: request.player_id.clone(),
                        raw_command_label: order.raw_command_label.clone(),
                        reason,
                    });
                } else {
                    let mut accepted = order.clone();
                    accepted.source = RtsOrderSource::Server;
                    accepted.frame = authority_tick;
                    accepted_orders.push(accepted);
                }
            }
        } else {
            for order in &request.orders {
                rejected_orders.push(RtsOnlineRejectedOrder {
                    player_id: request.player_id.clone(),
                    raw_command_label: order.raw_command_label.clone(),
                    reason: "player_scope_missing".to_string(),
                });
            }
        }
    }

    let scoped_updates = scopes
        .iter()
        .map(|scope| {
            let visible_actor_set = scope
                .visible_actor_ids
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>();
            let scoped_orders = accepted_orders
                .iter()
                .filter(|order| {
                    order
                        .subject_actor_ids
                        .iter()
                        .all(|actor_id| visible_actor_set.contains(actor_id))
                        && order
                            .target_actor_id
                            .as_deref()
                            .map(|actor_id| visible_actor_set.contains(actor_id))
                            .unwrap_or(true)
                })
                .cloned()
                .collect::<Vec<_>>();
            let update_sha256 =
                rts_online_update_sha256(arena_id, map_id, authority_tick, scope, &scoped_orders);
            RtsOnlineUpdateEnvelope {
                contract_version: TRNM_RTS_ONLINE_CONTRACT.to_string(),
                arena_id: arena_id.to_string(),
                map_id: map_id.to_string(),
                tick: authority_tick,
                scope: scope.clone(),
                orders: scoped_orders,
                update_sha256,
            }
        })
        .collect::<Vec<_>>();

    let mut resolution = RtsOnlineAuthorityResolution {
        contract_version: TRNM_RTS_ONLINE_AUTHORITY_CONTRACT.to_string(),
        arena_id: arena_id.to_string(),
        map_id: map_id.to_string(),
        authority_tick,
        client_requests,
        accepted_orders,
        rejected_orders,
        scoped_updates,
        authority_sha256: String::new(),
        green: false,
    };
    resolution.authority_sha256 = rts_online_authority_sha256(&resolution);
    resolution.green = resolution.authority_sha256.len() == 64
        && !resolution.client_requests.is_empty()
        && resolution
            .scoped_updates
            .iter()
            .all(|update| update.update_sha256.len() == 64)
        && (!resolution.scoped_updates.is_empty() || resolution.accepted_orders.is_empty())
        && resolution
            .rejected_orders
            .iter()
            .all(|rejection| !rejection.reason.is_empty());
    resolution
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn transport_payload_bytes<T: Serialize>(payload: &T) -> Vec<u8> {
    serde_json::to_vec(payload).expect("RTS online transport payload serializes")
}

pub fn rts_online_transport_frame<T: Serialize>(
    arena_id: &str,
    player_id: &str,
    sequence: u32,
    direction: RtsOnlineTransportDirection,
    payload_kind: RtsOnlineTransportPayloadKind,
    payload: &T,
) -> RtsOnlineTransportFrame {
    let payload_bytes = transport_payload_bytes(payload);
    let payload_sha256 = sha256_bytes(&payload_bytes);
    let mut wire_bytes = Vec::with_capacity(
        TRNM_RTS_ONLINE_WIRE_MAGIC.len()
            + std::mem::size_of::<u32>() * 2
            + payload_sha256.len()
            + payload_bytes.len(),
    );
    wire_bytes.extend_from_slice(TRNM_RTS_ONLINE_WIRE_MAGIC);
    wire_bytes.extend_from_slice(&sequence.to_be_bytes());
    wire_bytes.extend_from_slice(&(payload_bytes.len() as u32).to_be_bytes());
    wire_bytes.extend_from_slice(payload_sha256.as_bytes());
    wire_bytes.extend_from_slice(&payload_bytes);
    let frame_sha256 = sha256_bytes(&wire_bytes);

    RtsOnlineTransportFrame {
        contract_version: TRNM_RTS_ONLINE_LOOPBACK_TRANSPORT_CONTRACT.to_string(),
        direction,
        payload_kind,
        sequence,
        arena_id: arena_id.to_string(),
        player_id: player_id.to_string(),
        wire_magic: String::from_utf8_lossy(TRNM_RTS_ONLINE_WIRE_MAGIC).into_owned(),
        encoded_len: wire_bytes.len(),
        payload_sha256,
        frame_sha256,
    }
}

pub fn rts_online_loopback_transport_fixture(
    session_id: &str,
    baseline_envelope: &RtsOnlineUpdateEnvelope,
    authority: &RtsOnlineAuthorityResolution,
) -> RtsOnlineLoopbackTransportFixture {
    let request = authority
        .client_requests
        .first()
        .expect("first contact authority fixture includes a client request");
    let scoped_update = authority
        .scoped_updates
        .first()
        .expect("first contact authority fixture includes a scoped update");
    let request_frame = rts_online_transport_frame(
        &authority.arena_id,
        &request.player_id,
        1,
        RtsOnlineTransportDirection::ClientToServer,
        RtsOnlineTransportPayloadKind::ClientRequest,
        request,
    );
    let response_frame = rts_online_transport_frame(
        &authority.arena_id,
        &request.player_id,
        2,
        RtsOnlineTransportDirection::ServerToClient,
        RtsOnlineTransportPayloadKind::ScopedUpdate,
        scoped_update,
    );
    let request_ack_matches_envelope =
        request.acknowledged_update_sha256 == baseline_envelope.update_sha256;
    let response_matches_authority = scoped_update.arena_id == authority.arena_id
        && scoped_update.map_id == authority.map_id
        && scoped_update.tick == authority.authority_tick
        && authority
            .scoped_updates
            .iter()
            .any(|update| update.update_sha256 == scoped_update.update_sha256);
    let server_authoritative = !authority.accepted_orders.is_empty()
        && authority
            .accepted_orders
            .iter()
            .all(|order| order.source == RtsOrderSource::Server);
    let visible_actor_set = scoped_update
        .scope
        .visible_actor_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let visibility_scoped_response = scoped_update.scope.player_id == request.player_id
        && scoped_update.orders.iter().all(|order| {
            order
                .subject_actor_ids
                .iter()
                .all(|actor_id| visible_actor_set.contains(actor_id))
                && order
                    .target_actor_id
                    .as_deref()
                    .map(|actor_id| visible_actor_set.contains(actor_id))
                    .unwrap_or(true)
        });
    let socket_opened = false;
    let hosted_service_claimed = false;
    let public_launch_ready = false;
    let green = request_frame.contract_version == TRNM_RTS_ONLINE_LOOPBACK_TRANSPORT_CONTRACT
        && response_frame.contract_version == TRNM_RTS_ONLINE_LOOPBACK_TRANSPORT_CONTRACT
        && request_frame.wire_magic == "TRNMRTS1"
        && response_frame.wire_magic == "TRNMRTS1"
        && request_frame.payload_sha256.len() == 64
        && request_frame.frame_sha256.len() == 64
        && response_frame.payload_sha256.len() == 64
        && response_frame.frame_sha256.len() == 64
        && request_frame.encoded_len > 96
        && response_frame.encoded_len > 96
        && request_ack_matches_envelope
        && response_matches_authority
        && server_authoritative
        && visibility_scoped_response
        && !socket_opened
        && !hosted_service_claimed
        && !public_launch_ready;

    RtsOnlineLoopbackTransportFixture {
        contract_version: TRNM_RTS_ONLINE_LOOPBACK_TRANSPORT_CONTRACT.to_string(),
        session_id: session_id.to_string(),
        arena_id: authority.arena_id.clone(),
        map_id: authority.map_id.clone(),
        request_frame,
        response_frame,
        request_ack_matches_envelope,
        response_matches_authority,
        server_authoritative,
        visibility_scoped_response,
        socket_opened,
        hosted_service_claimed,
        public_launch_ready,
        green,
    }
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
        scope: scope.clone(),
        orders,
        update_sha256: update_sha256.clone(),
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
    let mut local_move_order = RtsFrameOrder::new(
        tick + 1,
        player_id,
        vec!["trnm.worker.alpha".to_string()],
        RtsOrderKind::Move,
        RtsOrderSource::LocalInput,
    );
    local_move_order.target_tile = Some(RtsTile::new(8, 4));
    local_move_order.raw_command_label = Some("client:move_worker@8,4".to_string());

    let mut fogged_attack_order = RtsFrameOrder::new(
        tick + 1,
        player_id,
        vec!["trnm.horizon.scout.alpha".to_string()],
        RtsOrderKind::Attack,
        RtsOrderSource::LocalInput,
    );
    fogged_attack_order.target_actor_id = Some("trnm.enemy.keep.fogged".to_string());
    fogged_attack_order.raw_command_label = Some("client:attack_fogged_keep".to_string());

    let client_request = RtsOnlineClientRequest {
        request_id: "first-contact-client-request-43".to_string(),
        player_id: player_id.to_string(),
        client_tick: tick + 1,
        acknowledged_update_sha256: update_sha256.clone(),
        orders: vec![local_move_order, fogged_attack_order],
    };
    let authority = rts_online_authority_resolve(
        arena_id,
        map_id,
        tick + 1,
        &[scope.clone()],
        vec![client_request],
    );
    let transport = rts_online_loopback_transport_fixture(
        "first-contact-loopback-session",
        &envelope,
        &authority,
    );
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
        && authority.green
        && authority.accepted_orders.len() == 1
        && authority.rejected_orders.len() == 1
        && authority
            .rejected_orders
            .iter()
            .any(|rejection| rejection.reason == "target_actor_not_visible")
        && authority.scoped_updates.iter().all(|update| {
            update
                .scope
                .visible_actor_ids
                .iter()
                .all(|actor_id| actor_id != "trnm.enemy.keep.fogged")
        })
        && transport.green
        && transport.server_authoritative
        && transport.visibility_scoped_response
        && !transport.socket_opened
        && !transport.hosted_service_claimed
        && !transport.public_launch_ready
        && lifecycle.phase == RtsOnlineArenaPhase::Playing
        && lifecycle.bot_count == 1
        && bot_plan.visible_chunks == envelope.scope.visible_chunks;

    RtsOnlineProtocolFixture {
        contract_version: TRNM_RTS_ONLINE_FIRST_CONTACT_FIXTURE_CONTRACT.to_string(),
        lifecycle,
        envelope,
        authority,
        transport,
        bot_plan,
        green,
    }
}

pub fn rts_online_local_handoff_from_fixture(
    fixture: &RtsOnlineProtocolFixture,
) -> RtsOnlineLocalHandoff {
    let scoped_update = fixture.authority.scoped_updates.first();
    let player_id = scoped_update
        .map(|update| update.scope.player_id.clone())
        .or_else(|| {
            fixture
                .authority
                .client_requests
                .first()
                .map(|request| request.player_id.clone())
        })
        .unwrap_or_else(|| "unknown".to_string());
    let visible_chunk_count = scoped_update
        .map(|update| update.scope.visible_chunks.len())
        .unwrap_or(0);
    let visible_actor_count = scoped_update
        .map(|update| update.scope.visible_actor_ids.len())
        .unwrap_or(0);
    let bevy_client_role = "visualization_and_local_input_submitter".to_string();
    let authority_role = "trnm_rts_online_fixture_authority_no_socket".to_string();
    let handoff_ready = fixture.green
        && fixture.authority.green
        && fixture.transport.green
        && fixture.transport.server_authoritative
        && fixture.transport.visibility_scoped_response
        && !fixture.transport.socket_opened
        && !fixture.transport.hosted_service_claimed
        && !fixture.transport.public_launch_ready
        && fixture.lifecycle.phase == RtsOnlineArenaPhase::Playing
        && fixture.lifecycle.bot_count >= 1
        && fixture.authority.accepted_orders.len() == 1
        && fixture.authority.rejected_orders.len() == 1
        && scoped_update.is_some();
    let green = handoff_ready
        && fixture.contract_version == TRNM_RTS_ONLINE_FIRST_CONTACT_FIXTURE_CONTRACT
        && fixture.lifecycle.map_id == fixture.envelope.map_id
        && fixture.envelope.update_sha256.len() == 64
        && fixture.authority.authority_sha256.len() == 64
        && fixture.transport.request_frame.frame_sha256.len() == 64
        && fixture.transport.response_frame.frame_sha256.len() == 64;

    RtsOnlineLocalHandoff {
        contract_version: TRNM_RTS_ONLINE_LOCAL_HANDOFF_CONTRACT.to_string(),
        handoff_id: "first-contact-local-loopback-handoff".to_string(),
        arena_id: fixture.lifecycle.arena_id.clone(),
        map_id: fixture.lifecycle.map_id.clone(),
        player_id,
        phase: fixture.lifecycle.phase,
        authority_tick: fixture.authority.authority_tick,
        accepted_order_count: fixture.authority.accepted_orders.len(),
        rejected_order_count: fixture.authority.rejected_orders.len(),
        scoped_update_count: fixture.authority.scoped_updates.len(),
        bot_count: fixture.lifecycle.bot_count,
        visible_chunk_count,
        visible_actor_count,
        loopback_session_id: fixture.transport.session_id.clone(),
        request_frame_sha256: fixture.transport.request_frame.frame_sha256.clone(),
        response_frame_sha256: fixture.transport.response_frame.frame_sha256.clone(),
        bevy_client_role,
        authority_role,
        server_authoritative: fixture.transport.server_authoritative,
        visibility_scoped_response: fixture.transport.visibility_scoped_response,
        socket_opened: fixture.transport.socket_opened,
        hosted_service_claimed: fixture.transport.hosted_service_claimed,
        public_launch_ready: fixture.transport.public_launch_ready,
        handoff_ready,
        green,
    }
}

pub fn first_contact_online_local_handoff() -> RtsOnlineLocalHandoff {
    rts_online_local_handoff_from_fixture(&first_contact_online_protocol_fixture())
}

pub fn rts_online_offline_adapter_local_replay() -> RtsOnlineOfflineAdapterLocalReplay {
    let replay_fixtures = rts_control_group_command_feedback_replay_fixtures();
    let rejection_fixtures = rts_control_group_command_feedback_rejection_replay_fixtures();
    let accepted_action_labels = replay_fixtures
        .command_steps
        .iter()
        .map(|step| step.action_label.clone())
        .collect::<Vec<_>>();
    let accepted_preview_stages = replay_fixtures
        .command_steps
        .iter()
        .filter_map(|step| step.preview_stage.clone())
        .collect::<Vec<_>>();
    let blocked_action_labels = rejection_fixtures
        .rejection_steps
        .iter()
        .filter(|step| !step.expected_accepted)
        .map(|step| step.action_label.clone())
        .collect::<Vec<_>>();
    let blocked_input_sources = rejection_fixtures
        .rejection_steps
        .iter()
        .filter(|step| !step.expected_accepted)
        .map(|step| step.input_source.clone())
        .collect::<Vec<_>>();
    let blocked_reasons = rejection_fixtures
        .rejection_steps
        .iter()
        .filter(|step| !step.expected_accepted)
        .map(|step| step.expected_reason.clone())
        .collect::<Vec<_>>();
    let blocked_preview_stages = rejection_fixtures
        .rejection_steps
        .iter()
        .filter(|step| !step.expected_accepted)
        .filter_map(|step| step.preview_stage.clone())
        .collect::<Vec<_>>();
    let local_input_sources_ready = blocked_input_sources
        == vec![
            "classic_rts_mouse_viewport".to_string(),
            "classic_rts_mouse_viewport".to_string(),
            "classic_rts_mouse_viewport".to_string(),
            "classic_rts_hotkey".to_string(),
            "classic_rts_mouse_sidebar".to_string(),
            "classic_rts_mouse_sidebar".to_string(),
            "classic_rts_hotkey".to_string(),
        ]
        && blocked_preview_stages
            == vec![
                "group_selection_required".to_string(),
                "invalid_tile".to_string(),
                "attack_target_required".to_string(),
                "history_preserved_after_rejections".to_string(),
            ];
    let command_history_ready = replay_fixtures.retained_history_group_ids
        == vec!["26".to_string(), "27".to_string(), "28".to_string()]
        && replay_fixtures.pruned_history_group_ids == vec!["25".to_string(), "24".to_string()]
        && replay_fixtures.history_entries.len() == 3
        && replay_fixtures.pruned_history_entries.len() == 2
        && rejection_fixtures.retained_history_group_ids
            == vec!["26".to_string(), "27".to_string(), "28".to_string()]
        && rejection_fixtures.pruned_history_group_ids == vec!["25".to_string(), "24".to_string()];
    let green = accepted_action_labels
        == vec![
            "RTS:SELECT:26".to_string(),
            "RTS:MOVE:18,31:line".to_string(),
            "RTS:SELECT:27".to_string(),
            "RTS:MOVE:21,25:line".to_string(),
            "RTS:SELECT:28".to_string(),
            "RTS:MOVE:1,31:line".to_string(),
            "RTS:SELECT:26".to_string(),
        ]
        && accepted_preview_stages
            == vec![
                "group_26_queued".to_string(),
                "group_27_override".to_string(),
                "group_28_formation".to_string(),
                "cleared_history_bounded".to_string(),
            ]
        && blocked_action_labels
            == vec![
                "RTS:MOVE:18,31:line".to_string(),
                "RTS:MOVE:bad-tile:line".to_string(),
                "RTS:ATTACK:".to_string(),
                "RTS:ABILITY:guard_break".to_string(),
                "RTS:QUEUE:".to_string(),
                "RTS:QUEUE:build:watch_tower@7,4".to_string(),
                "RTS:SELECT:".to_string(),
            ]
        && blocked_reasons == rejection_fixtures.expected_blocked_reasons
        && local_input_sources_ready
        && command_history_ready;

    RtsOnlineOfflineAdapterLocalReplay {
        contract_version: TRNM_RTS_ONLINE_OFFLINE_ADAPTER_LOCAL_REPLAY_CONTRACT.to_string(),
        replay_mode: "bevy_local_ui_action_replay".to_string(),
        accepted_action_labels,
        accepted_preview_stages,
        blocked_action_labels,
        blocked_input_sources,
        blocked_reasons,
        blocked_preview_stages,
        retained_history_group_ids: replay_fixtures.retained_history_group_ids,
        pruned_history_group_ids: replay_fixtures.pruned_history_group_ids,
        command_history_capacity: 3,
        local_input_sources_ready,
        command_history_ready,
        green,
    }
}

pub fn rts_online_offline_adapter_from_fixture(
    fixture: &RtsOnlineProtocolFixture,
) -> RtsOnlineOfflineAdapterSummary {
    let handoff = rts_online_local_handoff_from_fixture(fixture);
    let local_action_replay = rts_online_offline_adapter_local_replay();
    let scoped_update = fixture.authority.scoped_updates.first();
    let input_queue_labels = fixture
        .authority
        .client_requests
        .iter()
        .flat_map(|request| request.orders.iter())
        .filter_map(|order| order.raw_command_label.clone())
        .collect::<Vec<_>>();
    let accepted_server_order_labels = fixture
        .authority
        .accepted_orders
        .iter()
        .filter_map(|order| order.raw_command_label.clone())
        .collect::<Vec<_>>();
    let rejected_client_order_reasons = fixture
        .authority
        .rejected_orders
        .iter()
        .map(|rejection| rejection.reason.clone())
        .collect::<Vec<_>>();
    let scoped_update_actor_ids = scoped_update
        .map(|update| update.scope.visible_actor_ids.clone())
        .unwrap_or_default();
    let scoped_update_order_count = scoped_update
        .map(|update| update.orders.len())
        .unwrap_or_default();
    let bot_player_ids = vec![fixture.bot_plan.player_id.clone()];
    let frame_sha256s = vec![
        fixture.authority.authority_sha256.clone(),
        fixture.transport.request_frame.frame_sha256.clone(),
        fixture.transport.response_frame.frame_sha256.clone(),
    ];
    let local_multiplayer_ready = handoff.handoff_ready
        && fixture.lifecycle.connected_player_ids.len() >= 2
        && !fixture.transport.socket_opened
        && !fixture.transport.hosted_service_claimed;
    let offline_bot_ready = fixture.lifecycle.bot_count == bot_player_ids.len()
        && fixture.bot_plan.tick == fixture.envelope.tick
        && !fixture.bot_plan.order_labels.is_empty();
    let bevy_adapter_ready = handoff.green
        && handoff.bevy_client_role == "visualization_and_local_input_submitter"
        && handoff.authority_role == "trnm_rts_online_fixture_authority_no_socket"
        && local_action_replay.green;
    let client_prediction_claimed = false;
    let rollback_netcode_claimed = false;
    let green = fixture.green
        && handoff.green
        && fixture.contract_version == TRNM_RTS_ONLINE_FIRST_CONTACT_FIXTURE_CONTRACT
        && local_multiplayer_ready
        && offline_bot_ready
        && bevy_adapter_ready
        && local_action_replay.green
        && input_queue_labels.len() == 2
        && accepted_server_order_labels == vec!["client:move_worker@8,4".to_string()]
        && rejected_client_order_reasons == vec!["target_actor_not_visible".to_string()]
        && scoped_update_actor_ids.len() == handoff.visible_actor_count
        && scoped_update_order_count == handoff.accepted_order_count
        && frame_sha256s.iter().all(|sha| sha.len() == 64)
        && fixture.transport.server_authoritative
        && fixture.transport.visibility_scoped_response
        && !client_prediction_claimed
        && !rollback_netcode_claimed
        && !fixture.transport.socket_opened
        && !fixture.transport.hosted_service_claimed
        && !fixture.transport.public_launch_ready;

    RtsOnlineOfflineAdapterSummary {
        contract_version: TRNM_RTS_ONLINE_OFFLINE_ADAPTER_CONTRACT.to_string(),
        adapter_id: "first-contact-offline-loopback-adapter".to_string(),
        handoff_id: handoff.handoff_id,
        arena_id: fixture.lifecycle.arena_id.clone(),
        map_id: fixture.lifecycle.map_id.clone(),
        adapter_mode: "offline_loopback_authority".to_string(),
        bevy_client_role: handoff.bevy_client_role,
        authority_role: handoff.authority_role,
        connected_player_ids: fixture.lifecycle.connected_player_ids.clone(),
        bot_player_ids,
        input_queue_labels,
        accepted_server_order_labels,
        rejected_client_order_reasons,
        scoped_update_actor_ids,
        scoped_update_order_count,
        frame_sha256s,
        local_action_replay,
        local_multiplayer_ready,
        offline_bot_ready,
        bevy_adapter_ready,
        server_authoritative: fixture.transport.server_authoritative,
        visibility_scoped_response: fixture.transport.visibility_scoped_response,
        client_prediction_claimed,
        rollback_netcode_claimed,
        socket_opened: fixture.transport.socket_opened,
        hosted_service_claimed: fixture.transport.hosted_service_claimed,
        public_launch_ready: fixture.transport.public_launch_ready,
        green,
    }
}

pub fn first_contact_online_offline_adapter() -> RtsOnlineOfflineAdapterSummary {
    rts_online_offline_adapter_from_fixture(&first_contact_online_protocol_fixture())
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
        assert!(fixture.authority.green);
        assert_eq!(
            fixture.authority.contract_version,
            TRNM_RTS_ONLINE_AUTHORITY_CONTRACT
        );
        assert_eq!(fixture.authority.accepted_orders.len(), 1);
        assert_eq!(fixture.authority.rejected_orders.len(), 1);
        assert_eq!(
            fixture.authority.rejected_orders[0].reason,
            "target_actor_not_visible"
        );
        assert_eq!(fixture.authority.scoped_updates.len(), 1);
        assert!(fixture.authority.scoped_updates[0]
            .scope
            .visible_actor_ids
            .iter()
            .all(|actor_id| actor_id != "trnm.enemy.keep.fogged"));
        assert!(fixture.transport.green);
        assert_eq!(
            fixture.transport.contract_version,
            TRNM_RTS_ONLINE_LOOPBACK_TRANSPORT_CONTRACT
        );
        assert_eq!(
            fixture.transport.request_frame.direction,
            RtsOnlineTransportDirection::ClientToServer
        );
        assert_eq!(
            fixture.transport.response_frame.direction,
            RtsOnlineTransportDirection::ServerToClient
        );
        assert_eq!(
            fixture.transport.request_frame.payload_kind,
            RtsOnlineTransportPayloadKind::ClientRequest
        );
        assert_eq!(
            fixture.transport.response_frame.payload_kind,
            RtsOnlineTransportPayloadKind::ScopedUpdate
        );
        assert_eq!(fixture.transport.request_frame.frame_sha256.len(), 64);
        assert_eq!(fixture.transport.response_frame.frame_sha256.len(), 64);
        assert!(fixture.transport.request_ack_matches_envelope);
        assert!(fixture.transport.response_matches_authority);
        assert!(fixture.transport.server_authoritative);
        assert!(fixture.transport.visibility_scoped_response);
        assert!(!fixture.transport.socket_opened);
        assert!(!fixture.transport.hosted_service_claimed);
        assert!(!fixture.transport.public_launch_ready);
        assert_eq!(fixture.bot_plan.order_labels, vec!["move:rally@8,4"]);
    }

    #[test]
    fn first_contact_online_local_handoff_is_no_socket_and_bevy_facing() {
        let handoff = first_contact_online_local_handoff();

        assert_eq!(
            handoff.contract_version,
            TRNM_RTS_ONLINE_LOCAL_HANDOFF_CONTRACT
        );
        assert!(handoff.green);
        assert!(handoff.handoff_ready);
        assert_eq!(handoff.handoff_id, "first-contact-local-loopback-handoff");
        assert_eq!(handoff.map_id, "first_contact_basin");
        assert_eq!(handoff.player_id, "mirror_guard");
        assert_eq!(handoff.phase, RtsOnlineArenaPhase::Playing);
        assert_eq!(handoff.authority_tick, 43);
        assert_eq!(handoff.accepted_order_count, 1);
        assert_eq!(handoff.rejected_order_count, 1);
        assert_eq!(handoff.scoped_update_count, 1);
        assert_eq!(handoff.bot_count, 1);
        assert_eq!(handoff.visible_chunk_count, 3);
        assert_eq!(handoff.visible_actor_count, 4);
        assert_eq!(
            handoff.loopback_session_id,
            "first-contact-loopback-session"
        );
        assert_eq!(handoff.request_frame_sha256.len(), 64);
        assert_eq!(handoff.response_frame_sha256.len(), 64);
        assert_eq!(
            handoff.bevy_client_role,
            "visualization_and_local_input_submitter"
        );
        assert_eq!(
            handoff.authority_role,
            "trnm_rts_online_fixture_authority_no_socket"
        );
        assert!(handoff.server_authoritative);
        assert!(handoff.visibility_scoped_response);
        assert!(!handoff.socket_opened);
        assert!(!handoff.hosted_service_claimed);
        assert!(!handoff.public_launch_ready);
    }

    #[test]
    fn first_contact_online_offline_adapter_is_local_and_no_socket() {
        let adapter = first_contact_online_offline_adapter();

        assert_eq!(
            adapter.contract_version,
            TRNM_RTS_ONLINE_OFFLINE_ADAPTER_CONTRACT
        );
        assert!(adapter.green);
        assert_eq!(adapter.adapter_id, "first-contact-offline-loopback-adapter");
        assert_eq!(adapter.handoff_id, "first-contact-local-loopback-handoff");
        assert_eq!(adapter.map_id, "first_contact_basin");
        assert_eq!(adapter.adapter_mode, "offline_loopback_authority");
        assert_eq!(
            adapter.connected_player_ids,
            vec!["local-player".to_string(), "mirror_guard".to_string()]
        );
        assert_eq!(adapter.bot_player_ids, vec!["mirror_guard".to_string()]);
        assert_eq!(
            adapter.input_queue_labels,
            vec![
                "client:move_worker@8,4".to_string(),
                "client:attack_fogged_keep".to_string(),
            ]
        );
        assert_eq!(
            adapter.accepted_server_order_labels,
            vec!["client:move_worker@8,4".to_string()]
        );
        assert_eq!(
            adapter.rejected_client_order_reasons,
            vec!["target_actor_not_visible".to_string()]
        );
        assert_eq!(adapter.scoped_update_actor_ids.len(), 4);
        assert_eq!(adapter.scoped_update_order_count, 1);
        assert_eq!(adapter.frame_sha256s.len(), 3);
        assert!(adapter.frame_sha256s.iter().all(|sha| sha.len() == 64));
        assert_eq!(
            adapter.local_action_replay.contract_version,
            TRNM_RTS_ONLINE_OFFLINE_ADAPTER_LOCAL_REPLAY_CONTRACT
        );
        assert_eq!(
            adapter.local_action_replay.replay_mode,
            "bevy_local_ui_action_replay"
        );
        assert_eq!(
            adapter.local_action_replay.accepted_action_labels,
            vec![
                "RTS:SELECT:26".to_string(),
                "RTS:MOVE:18,31:line".to_string(),
                "RTS:SELECT:27".to_string(),
                "RTS:MOVE:21,25:line".to_string(),
                "RTS:SELECT:28".to_string(),
                "RTS:MOVE:1,31:line".to_string(),
                "RTS:SELECT:26".to_string(),
            ]
        );
        assert_eq!(
            adapter.local_action_replay.blocked_reasons,
            vec![
                "rts_group_selection_required".to_string(),
                "rts_invalid_tile:bad-tile".to_string(),
                "rts_attack_target_required".to_string(),
                "rts_attack_required_before_ability".to_string(),
                "rts_queue_id_required".to_string(),
                "rts_queue_unaffordable:build:watch_tower@7,4".to_string(),
                "rts_group_id_required".to_string(),
            ]
        );
        assert_eq!(
            adapter.local_action_replay.retained_history_group_ids,
            vec!["26".to_string(), "27".to_string(), "28".to_string()]
        );
        assert_eq!(
            adapter.local_action_replay.pruned_history_group_ids,
            vec!["25".to_string(), "24".to_string()]
        );
        assert_eq!(adapter.local_action_replay.command_history_capacity, 3);
        assert!(adapter.local_action_replay.local_input_sources_ready);
        assert!(adapter.local_action_replay.command_history_ready);
        assert!(adapter.local_action_replay.green);
        assert!(adapter.local_multiplayer_ready);
        assert!(adapter.offline_bot_ready);
        assert!(adapter.bevy_adapter_ready);
        assert!(adapter.server_authoritative);
        assert!(adapter.visibility_scoped_response);
        assert!(!adapter.client_prediction_claimed);
        assert!(!adapter.rollback_netcode_claimed);
        assert!(!adapter.socket_opened);
        assert!(!adapter.hosted_service_claimed);
        assert!(!adapter.public_launch_ready);
    }

    #[test]
    fn transport_frame_hash_changes_by_sequence() {
        let fixture = first_contact_online_protocol_fixture();
        let request = &fixture.authority.client_requests[0];
        let first = rts_online_transport_frame(
            &fixture.lifecycle.arena_id,
            &request.player_id,
            1,
            RtsOnlineTransportDirection::ClientToServer,
            RtsOnlineTransportPayloadKind::ClientRequest,
            request,
        );
        let second = rts_online_transport_frame(
            &fixture.lifecycle.arena_id,
            &request.player_id,
            2,
            RtsOnlineTransportDirection::ClientToServer,
            RtsOnlineTransportPayloadKind::ClientRequest,
            request,
        );

        assert_eq!(first.payload_sha256, second.payload_sha256);
        assert_ne!(first.frame_sha256, second.frame_sha256);
    }

    #[test]
    fn authority_rejects_missing_visibility_scope() {
        let mut order = RtsFrameOrder::new(
            7,
            "rogue-client",
            vec!["trnm.worker.alpha".to_string()],
            RtsOrderKind::Move,
            RtsOrderSource::LocalInput,
        );
        order.target_tile = Some(RtsTile::new(1, 1));
        order.raw_command_label = Some("client:move_without_scope".to_string());
        let request = RtsOnlineClientRequest {
            request_id: "missing-scope".to_string(),
            player_id: "rogue-client".to_string(),
            client_tick: 7,
            acknowledged_update_sha256: "0".repeat(64),
            orders: vec![order],
        };

        let resolution =
            rts_online_authority_resolve("arena", "first_contact_basin", 8, &[], vec![request]);

        assert!(resolution.green);
        assert!(resolution.accepted_orders.is_empty());
        assert_eq!(resolution.rejected_orders.len(), 1);
        assert_eq!(resolution.rejected_orders[0].reason, "player_scope_missing");
    }
}
