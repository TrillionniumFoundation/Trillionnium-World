use super::{
    api_error, apply_published_actor_view, conflict, ensure_match_actor, fetch_match_view,
    internal_db, published_authority_matches_view, published_cursor_is_within_durable_view,
    verify_identity, ApiError, AppState, PublishedMatchState, StreamConnectionRegistry,
};
use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    http::{header::SEC_WEBSOCKET_PROTOCOL, HeaderMap, StatusCode},
    response::Response,
};
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use serde_json::Value;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::watch, time::timeout};
use trnm_online_protocol::{
    build_snapshot_delta, OnlineMatchView, OnlineStreamConnectRequest, OnlineStreamServerMessage,
    ONLINE_AUTHORITY_BUILD, ONLINE_STREAM_PROTOCOL,
};
use uuid::Uuid;

const STREAM_SEND_TIMEOUT: Duration = Duration::from_secs(2);
const STREAM_MAX_INBOUND_BYTES: usize = 64 * 1024;
const STREAM_KEYFRAME_INTERVAL_TICKS: u64 = 100;
const STREAM_DELTA_PERCENT: usize = 80;
const STREAM_REAUTH_INTERVAL: Duration = Duration::from_secs(60);
const STREAM_PING_INTERVAL: Duration = Duration::from_secs(15);
const STREAM_CONNECTIONS_PER_MEMBER_MATCH: usize = 2;

struct StreamConnectionLease {
    registry: Arc<Mutex<StreamConnectionRegistry>>,
    key: String,
}

struct StreamSessionContext {
    state: AppState,
    match_id: Uuid,
    actor_generation: String,
    reauth_headers: HeaderMap,
    reauth_player_id: String,
    reauth_account_id: String,
    _connection_lease: StreamConnectionLease,
}

impl Drop for StreamConnectionLease {
    fn drop(&mut self) {
        if let Ok(mut registry) = self.registry.lock() {
            registry.total = registry.total.saturating_sub(1);
            if let Some(count) = registry.by_member_match.get_mut(&self.key) {
                *count = count.saturating_sub(1);
                if *count == 0 {
                    registry.by_member_match.remove(&self.key);
                }
            }
        }
    }
}

pub(super) async fn stream_match(
    State(state): State<AppState>,
    Path(match_id): Path<Uuid>,
    Query(request): Query<OnlineStreamConnectRequest>,
    headers: HeaderMap,
    websocket: WebSocketUpgrade,
) -> Result<Response, ApiError> {
    if request.protocol_version != ONLINE_STREAM_PROTOCOL
        || request.build_id != ONLINE_AUTHORITY_BUILD
    {
        return Err(api_error(
            StatusCode::UPGRADE_REQUIRED,
            "unsupported online state stream protocol/build pair",
            false,
        ));
    }
    let requested_subprotocol = headers
        .get(SEC_WEBSOCKET_PROTOCOL)
        .and_then(|value| value.to_str().ok());
    if requested_subprotocol != Some(ONLINE_STREAM_PROTOCOL) {
        return Err(api_error(
            StatusCode::UPGRADE_REQUIRED,
            "Sec-WebSocket-Protocol must exactly match the online state stream protocol",
            false,
        ));
    }
    validate_snapshot_hash(&request.last_snapshot_hash)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let member: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_match_members
         where match_id = $1 and player_id = $2 and account_id = $3",
    )
    .bind(match_id)
    .bind(&request.player_id)
    .bind(account_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    if member != 1 {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "identity is not a match member",
            false,
        ));
    }
    let actor = ensure_match_actor(&state, match_id)
        .await
        .map_err(|error| api_error(StatusCode::SERVICE_UNAVAILABLE, error, true))?
        .ok_or_else(|| api_error(StatusCode::CONFLICT, "match is not running", false))?;
    let published = actor.published.clone();
    if request.next_receipt_sequence > published.borrow().next_sequence {
        return Err(conflict(
            "stream receipt cursor is beyond server authority",
            published.borrow().match_revision,
        ));
    }
    let actor_generation = actor.actor_id.to_string();
    let connection_lease =
        acquire_stream_connection(&state, format!("{match_id}:{}", request.player_id))?;
    let reauth_headers = headers.clone();
    let reauth_player_id = request.player_id.clone();
    let reauth_account_id = request.account_id.clone();
    Ok(websocket
        .max_message_size(STREAM_MAX_INBOUND_BYTES)
        .max_frame_size(STREAM_MAX_INBOUND_BYTES)
        .write_buffer_size(16 * 1024)
        .max_write_buffer_size(2 * 1024 * 1024)
        .protocols([ONLINE_STREAM_PROTOCOL])
        .on_upgrade(move |socket| async move {
            let context = StreamSessionContext {
                state,
                match_id,
                actor_generation,
                reauth_headers,
                reauth_player_id,
                reauth_account_id,
                _connection_lease: connection_lease,
            };
            if let Err(error) = serve_state_stream(socket, published, context).await {
                tracing::warn!(%match_id, %error, "online state stream closed");
            }
        }))
}

fn acquire_stream_connection(
    state: &AppState,
    key: String,
) -> Result<StreamConnectionLease, ApiError> {
    let mut registry = state.stream_connections.lock().map_err(|_| {
        api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "online state stream registry is unavailable",
            true,
        )
    })?;
    let global_limit = usize::try_from(state.capacity)
        .unwrap_or(usize::MAX / 4)
        .saturating_mul(4)
        .max(8);
    let member_connections = registry.by_member_match.get(&key).copied().unwrap_or(0);
    if registry.total >= global_limit || member_connections >= STREAM_CONNECTIONS_PER_MEMBER_MATCH {
        return Err(api_error(
            StatusCode::TOO_MANY_REQUESTS,
            "online state stream connection capacity is exhausted",
            true,
        ));
    }
    registry.total = registry.total.saturating_add(1);
    registry
        .by_member_match
        .insert(key.clone(), member_connections.saturating_add(1));
    drop(registry);
    Ok(StreamConnectionLease {
        registry: Arc::clone(&state.stream_connections),
        key,
    })
}

fn validate_snapshot_hash(value: &str) -> Result<(), ApiError> {
    if !value.is_empty()
        && (value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()))
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "last_snapshot_hash must be empty or a 64-character hexadecimal hash",
            false,
        ));
    }
    Ok(())
}

async fn serve_state_stream(
    socket: WebSocket,
    mut published: watch::Receiver<PublishedMatchState>,
    context: StreamSessionContext,
) -> Result<(), String> {
    let StreamSessionContext {
        state,
        match_id,
        actor_generation,
        reauth_headers,
        reauth_player_id,
        reauth_account_id,
        _connection_lease,
    } = context;
    let (mut outbound, mut inbound) = socket.split();
    let (mut current, mut view) =
        aligned_published_stream_view(&state, match_id, &mut published).await?;
    let mut snapshot = snapshot_value(&current)?;
    send_server_message(
        &mut outbound,
        &OnlineStreamServerMessage::FullSnapshot {
            actor_generation: actor_generation.clone(),
            state_sequence: current.state_sequence,
            next_receipt_sequence: current.next_sequence,
            view: view.clone(),
            snapshot: snapshot.clone(),
        },
    )
    .await?;
    let mut reauth = tokio::time::interval(STREAM_REAUTH_INTERVAL);
    reauth.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    reauth.tick().await;
    let mut ping = tokio::time::interval(STREAM_PING_INTERVAL);
    ping.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    ping.tick().await;

    loop {
        tokio::select! {
            _ = reauth.tick() => {
                verify_identity(
                    &state,
                    &reauth_headers,
                    &reauth_player_id,
                    &reauth_account_id,
                )
                .await
                .map_err(|_| "online state stream periodic identity revalidation failed".to_string())?;
            }
            _ = ping.tick() => {
                send_message(&mut outbound, Message::Ping(Vec::new())).await?;
            }
            changed = published.changed() => {
                if changed.is_err() {
                    let _ = send_server_message(
                        &mut outbound,
                        &OnlineStreamServerMessage::ResyncRequired {
                            actor_generation: actor_generation.clone(),
                            reason: "actor_generation_ended".to_string(),
                        },
                    ).await;
                    break;
                }
                let mut next = published.borrow_and_update().clone();
                let mut next_view = if next.match_revision != view.match_revision
                    || !published_authority_matches_view(&next, &view)
                {
                    let aligned = aligned_published_stream_view(
                        &state,
                        match_id,
                        &mut published,
                    ).await?;
                    next = aligned.0;
                    aligned.1
                } else {
                    view.clone()
                };
                apply_published_actor_view(&mut next_view, &next);
                let next_snapshot = snapshot_value(&next)?;
                send_state_update(
                    &mut outbound,
                    &actor_generation,
                    &current,
                    &next,
                    &next_view,
                    &snapshot,
                    &next_snapshot,
                ).await?;
                current = next;
                view = next_view;
                snapshot = next_snapshot;
            }
            incoming = inbound.next() => {
                match incoming {
                    Some(Ok(Message::Ping(payload))) => {
                        send_message(&mut outbound, Message::Pong(payload)).await?;
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Pong(_))) => {}
                    Some(Ok(Message::Text(_))) | Some(Ok(Message::Binary(_))) => {
                        let _ = send_message(
                            &mut outbound,
                            Message::Close(Some(CloseFrame {
                                code: 1008,
                                reason: "state-only stream rejects client data frames".into(),
                            })),
                        ).await;
                        return Err("online state stream received an unexpected client data frame".to_string());
                    }
                    Some(Err(error)) => return Err(format!("receive online state stream: {error}")),
                }
            }
        }
    }
    let _ = timeout(STREAM_SEND_TIMEOUT, outbound.close()).await;
    Ok(())
}

async fn send_state_update(
    outbound: &mut SplitSink<WebSocket, Message>,
    actor_generation: &str,
    base: &PublishedMatchState,
    next: &PublishedMatchState,
    view: &OnlineMatchView,
    base_snapshot: &Value,
    next_snapshot: &Value,
) -> Result<(), String> {
    let full = OnlineStreamServerMessage::FullSnapshot {
        actor_generation: actor_generation.to_string(),
        state_sequence: next.state_sequence,
        next_receipt_sequence: next.next_sequence,
        view: view.clone(),
        snapshot: next_snapshot.clone(),
    };
    let full_text = serde_json::to_string(&full)
        .map_err(|error| format!("encode online full snapshot: {error}"))?;
    let crossed_keyframe_boundary = next.simulation.tick / STREAM_KEYFRAME_INTERVAL_TICKS
        > base.simulation.tick / STREAM_KEYFRAME_INTERVAL_TICKS;
    if crossed_keyframe_boundary {
        return send_text(outbound, full_text).await;
    }
    let Some(delta) = build_snapshot_delta(
        base_snapshot,
        next_snapshot,
        base.snapshot_hash.as_ref().clone(),
        next.snapshot_hash.as_ref().clone(),
        base.simulation.tick,
        next.simulation.tick,
    ) else {
        return send_text(outbound, full_text).await;
    };
    let delta = OnlineStreamServerMessage::SnapshotDelta {
        actor_generation: actor_generation.to_string(),
        state_sequence: next.state_sequence,
        base_state_sequence: base.state_sequence,
        view: view.clone(),
        delta,
    };
    let delta_text = serde_json::to_string(&delta)
        .map_err(|error| format!("encode online snapshot delta: {error}"))?;
    if delta_text.len().saturating_mul(100) >= full_text.len().saturating_mul(STREAM_DELTA_PERCENT)
    {
        send_text(outbound, full_text).await
    } else {
        send_text(outbound, delta_text).await
    }
}

async fn stream_view(state: &AppState, match_id: Uuid) -> Result<OnlineMatchView, String> {
    fetch_match_view(&state.pool, match_id)
        .await
        .map_err(|error| error.body.error)
}

async fn aligned_published_stream_view(
    state: &AppState,
    match_id: Uuid,
    published: &mut watch::Receiver<PublishedMatchState>,
) -> Result<(PublishedMatchState, OnlineMatchView), String> {
    for _ in 0..20 {
        let mut view = stream_view(state, match_id).await?;
        let current = published.borrow_and_update().clone();
        if published_stream_view_is_aligned(&current, &view) {
            apply_published_actor_view(&mut view, &current);
            return Ok((current, view));
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    Err("online state stream could not align durable and published authority cursors".to_string())
}

pub(super) fn published_stream_view_is_aligned(
    published: &PublishedMatchState,
    view: &OnlineMatchView,
) -> bool {
    view.next_sequence == published.next_sequence
        && view.match_revision == published.match_revision
        && published_authority_matches_view(published, view)
        && published_cursor_is_within_durable_view(published, view)
}

fn snapshot_value(published: &PublishedMatchState) -> Result<Value, String> {
    serde_json::to_value(published.simulation.as_ref())
        .map_err(|error| format!("encode online state stream snapshot: {error}"))
}

async fn send_server_message(
    outbound: &mut SplitSink<WebSocket, Message>,
    message: &OnlineStreamServerMessage,
) -> Result<(), String> {
    let text = serde_json::to_string(message)
        .map_err(|error| format!("encode online state stream message: {error}"))?;
    send_text(outbound, text).await
}

async fn send_text(
    outbound: &mut SplitSink<WebSocket, Message>,
    text: String,
) -> Result<(), String> {
    send_message(outbound, Message::Text(text)).await
}

async fn send_message(
    outbound: &mut SplitSink<WebSocket, Message>,
    message: Message,
) -> Result<(), String> {
    timeout(STREAM_SEND_TIMEOUT, outbound.send(message))
        .await
        .map_err(|_| "online state stream send timed out".to_string())?
        .map_err(|error| format!("send online state stream: {error}"))
}
