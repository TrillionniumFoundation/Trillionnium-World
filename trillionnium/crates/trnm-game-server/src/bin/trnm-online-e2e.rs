#![recursion_limit = "256"]

use reqwest::blocking::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::{
    net::TcpStream,
    process::Command,
    sync::{Arc, Barrier, Mutex},
    thread,
    time::{Duration, Instant},
};
use trnm_online_protocol::{
    apply_snapshot_delta, OnlineCampaignConnectRequest, OnlineCampaignView, OnlineCommandReceipt,
    OnlineCommandSubmitRequest, OnlineMatchAccessRequest, OnlineMatchCreateRequest,
    OnlineMatchJoinRequest, OnlineMatchPhase, OnlineMatchStartRequest, OnlineMatchView,
    OnlineReconnectRequest, OnlineReconnectResponse, OnlineSnapshotResponse,
    OnlineStreamConnectRequest, OnlineStreamServerMessage, ONLINE_AUTHORITY_BUILD,
    ONLINE_AUTHORITY_PROTOCOL, ONLINE_STREAM_PROTOCOL,
};
use trnm_rts_protocol::{RtsFrameOrder, RtsOrderKind, RtsOrderSource, RtsTile};
use trnm_rts_sim::MissionSimV1;
use tungstenite::{
    client::IntoClientRequest,
    http::{header::SEC_WEBSOCKET_PROTOCOL, HeaderValue},
    stream::MaybeTlsStream,
    Message, WebSocket,
};

const SNAPSHOT_RECOVERABLE_RETRY_TIMEOUT: Duration = Duration::from_secs(10);
const SNAPSHOT_RECOVERABLE_RETRY_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Clone)]
struct Identity {
    player_id: String,
    account_id: String,
    session: String,
}

struct OnlineClient {
    base_url: String,
    http: Client,
    non_idempotent_http: Client,
    command_ack_ms: Mutex<Vec<u64>>,
}

struct CommandSpec {
    command_id: String,
    kind: RtsOrderKind,
    subjects: Vec<String>,
    target: RtsTile,
    queued: bool,
}

struct StreamSmokeCursor {
    actor_generation: String,
    state_sequence: u64,
    snapshot_hash: String,
    authoritative_tick: u64,
    snapshot: Value,
}

fn connect_stream_smoke(
    client: &OnlineClient,
    identity: &Identity,
    match_id: &str,
    snapshot: &OnlineSnapshotResponse,
) -> Result<(WebSocket<MaybeTlsStream<TcpStream>>, StreamSmokeCursor, u64), String> {
    let mut url = reqwest::Url::parse(&client.base_url)
        .map_err(|error| format!("state stream URL: {error}"))?;
    if url.scheme() != "http" {
        return Err(
            "online E2E state stream currently requires a local HTTP authority".to_string(),
        );
    }
    url.set_scheme("ws")
        .map_err(|()| "state stream URL scheme".to_string())?;
    let base_path = url.path().trim_end_matches('/').to_string();
    url.set_path(&format!("{base_path}/v1/online/matches/{match_id}/stream"));
    url.set_query(None);
    url.set_fragment(None);
    let connect = OnlineStreamConnectRequest {
        protocol_version: ONLINE_STREAM_PROTOCOL.to_string(),
        build_id: ONLINE_AUTHORITY_BUILD.to_string(),
        player_id: identity.player_id.clone(),
        account_id: identity.account_id.clone(),
        next_receipt_sequence: snapshot.view.next_sequence,
        last_snapshot_hash: snapshot.view.snapshot_hash.clone(),
    };
    url.query_pairs_mut()
        .append_pair("protocol_version", &connect.protocol_version)
        .append_pair("build_id", &connect.build_id)
        .append_pair("player_id", &connect.player_id)
        .append_pair("account_id", &connect.account_id)
        .append_pair(
            "next_receipt_sequence",
            &connect.next_receipt_sequence.to_string(),
        )
        .append_pair("last_snapshot_hash", &connect.last_snapshot_hash);
    if url.as_str().contains(&identity.session) {
        return Err("state stream URL leaked the player session".to_string());
    }
    let mut request = url
        .as_str()
        .into_client_request()
        .map_err(|error| format!("state stream request: {error}"))?;
    request.headers_mut().insert(
        "x-trnm-player-session",
        HeaderValue::from_str(&identity.session)
            .map_err(|error| format!("state stream session header: {error}"))?,
    );
    request.headers_mut().insert(
        SEC_WEBSOCKET_PROTOCOL,
        HeaderValue::from_static(ONLINE_STREAM_PROTOCOL),
    );
    let (mut socket, response) = tungstenite::client::connect_with_config(request, None, 0)
        .map_err(|error| format!("connect state stream: {error}"))?;
    if response
        .headers()
        .get(SEC_WEBSOCKET_PROTOCOL)
        .and_then(|value| value.to_str().ok())
        != Some(ONLINE_STREAM_PROTOCOL)
    {
        return Err("state stream server selected an unexpected subprotocol".to_string());
    }
    match socket.get_mut() {
        MaybeTlsStream::Plain(stream) => stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|error| format!("state stream read timeout: {error}"))?,
        _ => return Err("online E2E state stream received an unexpected TLS socket".to_string()),
    }
    let (cursor, next_sequence) = read_initial_stream_snapshot(&mut socket, match_id)?;
    Ok((socket, cursor, next_sequence))
}

fn read_initial_stream_snapshot(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    match_id: &str,
) -> Result<(StreamSmokeCursor, u64), String> {
    loop {
        match socket
            .read()
            .map_err(|error| format!("read initial state stream snapshot: {error}"))?
        {
            Message::Text(text) => {
                let message = serde_json::from_str::<OnlineStreamServerMessage>(&text)
                    .map_err(|error| format!("decode initial state stream snapshot: {error}"))?;
                if let OnlineStreamServerMessage::FullSnapshot {
                    actor_generation,
                    state_sequence,
                    next_receipt_sequence,
                    view,
                    snapshot,
                } = message
                {
                    if view.match_id != match_id || next_receipt_sequence != view.next_sequence {
                        return Err("initial state stream authority cursor mismatch".to_string());
                    }
                    validate_stream_smoke_snapshot(&view, &snapshot)?;
                    return Ok((
                        StreamSmokeCursor {
                            actor_generation,
                            state_sequence,
                            snapshot_hash: view.snapshot_hash,
                            authoritative_tick: view.authoritative_tick,
                            snapshot,
                        },
                        view.next_sequence,
                    ));
                }
                return Err("state stream did not begin with a full snapshot".to_string());
            }
            Message::Ping(_) => socket
                .flush()
                .map_err(|error| format!("flush state stream pong: {error}"))?,
            Message::Pong(_) => {}
            other => return Err(format!("unexpected initial state stream frame: {other:?}")),
        }
    }
}

fn wait_for_stream_total_order(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    cursor: &mut StreamSmokeCursor,
    match_id: &str,
    expected_next_sequence: u64,
) -> Result<(), String> {
    let mut saw_delta = false;
    for _ in 0..256 {
        match socket
            .read()
            .map_err(|error| format!("read state stream update: {error}"))?
        {
            Message::Text(text) => {
                let message = serde_json::from_str::<OnlineStreamServerMessage>(&text)
                    .map_err(|error| format!("decode state stream update: {error}"))?;
                let next_sequence = match message {
                    OnlineStreamServerMessage::FullSnapshot {
                        actor_generation,
                        state_sequence,
                        next_receipt_sequence,
                        view,
                        snapshot,
                    } => {
                        if actor_generation != cursor.actor_generation
                            || state_sequence < cursor.state_sequence
                            || view.match_id != match_id
                            || next_receipt_sequence != view.next_sequence
                        {
                            return Err("state stream full snapshot cursor regressed".to_string());
                        }
                        validate_stream_smoke_snapshot(&view, &snapshot)?;
                        cursor.state_sequence = state_sequence;
                        cursor.snapshot_hash = view.snapshot_hash.clone();
                        cursor.authoritative_tick = view.authoritative_tick;
                        cursor.snapshot = snapshot;
                        view.next_sequence
                    }
                    OnlineStreamServerMessage::SnapshotDelta {
                        actor_generation,
                        state_sequence,
                        base_state_sequence,
                        view,
                        delta,
                    } => {
                        saw_delta = true;
                        if actor_generation != cursor.actor_generation
                            || base_state_sequence != cursor.state_sequence
                            || state_sequence <= base_state_sequence
                            || view.match_id != match_id
                        {
                            return Err("state stream delta cursor mismatch".to_string());
                        }
                        apply_snapshot_delta(
                            &mut cursor.snapshot,
                            &cursor.snapshot_hash,
                            cursor.authoritative_tick,
                            &delta,
                        )?;
                        validate_stream_smoke_snapshot(&view, &cursor.snapshot)?;
                        cursor.state_sequence = state_sequence;
                        cursor.snapshot_hash = delta.snapshot_hash;
                        cursor.authoritative_tick = delta.authoritative_tick;
                        view.next_sequence
                    }
                    OnlineStreamServerMessage::ResyncRequired { reason, .. } => {
                        return Err(format!("state stream requested resync: {reason}"));
                    }
                };
                if next_sequence >= expected_next_sequence && saw_delta {
                    return Ok(());
                }
            }
            Message::Ping(_) => socket
                .flush()
                .map_err(|error| format!("flush state stream pong: {error}"))?,
            Message::Pong(_) => {}
            Message::Close(frame) => {
                return Err(format!(
                    "state stream closed before command update: {frame:?}"
                ));
            }
            other => return Err(format!("unexpected state stream frame: {other:?}")),
        }
    }
    Err("state stream did not reach the expected total-order cursor".to_string())
}

fn validate_stream_smoke_snapshot(view: &OnlineMatchView, snapshot: &Value) -> Result<(), String> {
    if view.protocol_version != ONLINE_AUTHORITY_PROTOCOL || view.build_id != ONLINE_AUTHORITY_BUILD
    {
        return Err("state stream authority contract mismatch".to_string());
    }
    let simulation = serde_json::from_value::<MissionSimV1>(snapshot.clone())
        .map_err(|error| format!("state stream mission decode: {error}"))?;
    if simulation.tick != view.authoritative_tick
        || simulation
            .snapshot_hash()
            .map_err(|error| format!("state stream mission hash: {error}"))?
            != view.snapshot_hash
    {
        return Err("state stream mission snapshot hash/tick mismatch".to_string());
    }
    Ok(())
}

fn summarize_milliseconds(mut samples: Vec<u64>) -> (Vec<u64>, u64, u64) {
    samples.sort_unstable();
    if samples.is_empty() {
        return (samples, 0, 0);
    }
    let p95_index = (samples.len() * 95).div_ceil(100).saturating_sub(1);
    let p95_ms = samples[p95_index];
    let max_ms = *samples.last().unwrap_or(&0);
    (samples, p95_ms, max_ms)
}

impl OnlineClient {
    fn new(base_url: String) -> Result<Self, String> {
        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: Client::builder()
                .connect_timeout(Duration::from_secs(2))
                .timeout(Duration::from_secs(4))
                .build()
                .map_err(|error| error.to_string())?,
            non_idempotent_http: Client::builder()
                .connect_timeout(Duration::from_secs(2))
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|error| error.to_string())?,
            command_ack_ms: Mutex::new(Vec::new()),
        })
    }

    fn post_one_shot_non_idempotent<T: Serialize, R: DeserializeOwned>(
        &self,
        identity: &Identity,
        path: &str,
        body: &T,
    ) -> Result<R, String> {
        // Create and join have no request ID, so a timeout cannot be retried
        // without risking a second mutation. Keep them on a long, one-shot
        // request path until the protocol exposes an idempotency key.
        let response = self
            .non_idempotent_http
            .post(format!("{}{}", self.base_url, path))
            .header("x-trnm-player-session", &identity.session)
            .json(body)
            .send()
            .map_err(|error| error.to_string())?;
        let status = response.status();
        let bytes = response.bytes().map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!(
                "POST {path} returned {status}: {}",
                String::from_utf8_lossy(&bytes)
            ));
        }
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())
    }

    fn start_with_lost_response_retry(
        &self,
        identity: &Identity,
        match_id: &str,
        body: &OnlineMatchStartRequest,
    ) -> Result<OnlineMatchView, String> {
        let path = format!("/v1/online/matches/{match_id}/start");
        let request = self
            .http
            .post(format!("{}{}", self.base_url, path))
            .header("x-trnm-player-session", &identity.session)
            .json(body);
        let response = send_with_lost_response_retry(request)?;
        let status = response.status();
        let bytes = response.bytes().map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!(
                "POST {path} returned {status}: {}",
                String::from_utf8_lossy(&bytes)
            ));
        }
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())
    }

    fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        identity: &Identity,
        path: &str,
        body: &T,
    ) -> Result<R, String> {
        let request = self
            .http
            .post(format!("{}{}", self.base_url, path))
            .header("x-trnm-player-session", &identity.session)
            .json(body);
        let started = Instant::now();
        let response = send_with_retry(request)?;
        let status = response.status();
        let bytes = response.bytes().map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!(
                "POST {path} returned {status}: {}",
                String::from_utf8_lossy(&bytes)
            ));
        }
        if path.ends_with("/commands") {
            let elapsed_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
            if let Ok(mut samples) = self.command_ack_ms.lock() {
                samples.push(elapsed_ms);
            }
        }
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())
    }

    fn tick_interval_ms(&self) -> Result<f64, String> {
        let response = send_with_retry(
            self.http
                .get(format!("{}/v1/online/readiness", self.base_url)),
        )?;
        let status = response.status();
        let value = response
            .json::<Value>()
            .map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!("readiness returned {status}: {value}"));
        }
        value["tick_interval_ms"]
            .as_f64()
            .filter(|interval| *interval > 0.0)
            .ok_or_else(|| "readiness did not expose a valid tick_interval_ms".to_string())
    }

    fn command_ack_summary(&self) -> (Vec<u64>, u64, u64) {
        let samples = self
            .command_ack_ms
            .lock()
            .map(|samples| samples.clone())
            .unwrap_or_default();
        summarize_milliseconds(samples)
    }

    fn post_status<T: Serialize>(
        &self,
        identity: &Identity,
        path: &str,
        body: &T,
    ) -> Result<(u16, Value), String> {
        let request = self
            .http
            .post(format!("{}{}", self.base_url, path))
            .header("x-trnm-player-session", &identity.session)
            .json(body);
        let response = send_with_retry(request)?;
        let status = response.status().as_u16();
        let value = response
            .json::<Value>()
            .map_err(|error| error.to_string())?;
        Ok((status, value))
    }

    fn snapshot(
        &self,
        identity: &Identity,
        match_id: &str,
    ) -> Result<OnlineSnapshotResponse, String> {
        let path = format!("/v1/online/matches/{match_id}/snapshot");
        let request = OnlineMatchAccessRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: identity.player_id.clone(),
            account_id: identity.account_id.clone(),
        };
        let retry_started = Instant::now();
        let response = loop {
            let (status, body) = self.post_status(identity, &path, &request)?;
            if (200..300).contains(&status) {
                break serde_json::from_value::<OnlineSnapshotResponse>(body)
                    .map_err(|error| error.to_string())?;
            }
            if snapshot_response_is_recoverable(status, &body)
                && retry_started.elapsed() < SNAPSHOT_RECOVERABLE_RETRY_TIMEOUT
            {
                // The Authority advertises Retry-After: 1 while a running
                // tuple crosses its durable terminal publication barrier.
                // Honor that contract without hiding non-recoverable errors or
                // allowing an unbounded terminal wait.
                thread::sleep(SNAPSHOT_RECOVERABLE_RETRY_INTERVAL);
                continue;
            }
            let status = reqwest::StatusCode::from_u16(status)
                .map(|status| status.to_string())
                .unwrap_or_else(|_| status.to_string());
            return Err(format!("POST {path} returned {status}: {body}"));
        };
        if let Some(snapshot_tick) = response.snapshot.get("tick").and_then(Value::as_u64) {
            if response.view.authoritative_tick != snapshot_tick {
                return Err(format!(
                    "snapshot/view authoritative tick mismatch: view={} snapshot={snapshot_tick}",
                    response.view.authoritative_tick
                ));
            }
        }
        Ok(response)
    }

    fn reconnect(
        &self,
        identity: &Identity,
        match_id: &str,
        last_acknowledged_sequence: u64,
        last_snapshot_hash: String,
    ) -> Result<OnlineReconnectResponse, String> {
        self.post(
            identity,
            &format!("/v1/online/matches/{match_id}/reconnect"),
            &OnlineReconnectRequest {
                protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
                build_id: ONLINE_AUTHORITY_BUILD.to_string(),
                player_id: identity.player_id.clone(),
                account_id: identity.account_id.clone(),
                last_acknowledged_sequence,
                last_snapshot_hash,
                next_receipt_sequence: Some(last_acknowledged_sequence),
            },
        )
    }

    fn submit(
        &self,
        identity: &Identity,
        match_id: &str,
        snapshot: &OnlineSnapshotResponse,
        spec: CommandSpec,
    ) -> Result<(OnlineCommandReceipt, OnlineCommandSubmitRequest), String> {
        let target_tick = snapshot.view.authoritative_tick;
        let input_sequence = snapshot
            .view
            .members
            .iter()
            .find(|member| member.player_id == identity.player_id)
            .map(|member| member.next_input_sequence)
            .ok_or_else(|| "snapshot is missing the submitting member".to_string())?;
        let mut order = RtsFrameOrder::new(
            u32::try_from(target_tick).map_err(|_| "target tick overflow".to_string())?,
            &identity.player_id,
            spec.subjects,
            spec.kind,
            RtsOrderSource::LocalInput,
        );
        order.target_tile = Some(spec.target);
        order.queued = spec.queued;
        if spec.queued {
            order.queue_id = Some(format!("queue-{}", spec.command_id));
        }
        match spec.kind {
            RtsOrderKind::Attack | RtsOrderKind::FocusFire => {
                order.target_actor_id = Some("relay_beacon".to_string());
            }
            RtsOrderKind::Ability => {
                order.target_rule_id = Some("party_signature".to_string());
            }
            _ => {}
        }
        let mut request = OnlineCommandSubmitRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: identity.player_id.clone(),
            account_id: identity.account_id.clone(),
            command_id: spec.command_id,
            sequence: snapshot.view.next_sequence,
            input_sequence: Some(input_sequence),
            expected_match_revision: snapshot.view.match_revision,
            target_tick,
            client_observed_tick: Some(snapshot.view.authoritative_tick),
            order,
        };
        let path = format!("/v1/online/matches/{match_id}/commands");
        let receipt = match self.post(identity, &path, &request) {
            Ok(receipt) => receipt,
            Err(error) if error.contains("expected player input sequence") => {
                let fresh = self.snapshot(identity, match_id)?;
                request.sequence = fresh.view.next_sequence;
                request.input_sequence = fresh
                    .view
                    .members
                    .iter()
                    .find(|member| member.player_id == identity.player_id)
                    .map(|member| member.next_input_sequence);
                request.expected_match_revision = fresh.view.match_revision;
                request.target_tick = fresh.view.authoritative_tick;
                request.client_observed_tick = Some(fresh.view.authoritative_tick);
                request.order.frame = u32::try_from(request.target_tick)
                    .map_err(|_| "target tick overflow".to_string())?;
                self.post(identity, &path, &request)?
            }
            Err(error) => return Err(error),
        };
        Ok((receipt, request))
    }
}

fn snapshot_response_is_recoverable(status: u16, body: &Value) -> bool {
    status == reqwest::StatusCode::SERVICE_UNAVAILABLE.as_u16()
        && body["recoverable"].as_bool() == Some(true)
}

fn send_with_retry(
    request: reqwest::blocking::RequestBuilder,
) -> Result<reqwest::blocking::Response, String> {
    send_with_retry_inner(request, false)
}

fn send_with_lost_response_retry(
    request: reqwest::blocking::RequestBuilder,
) -> Result<reqwest::blocking::Response, String> {
    send_with_retry_inner(request, true)
}

fn send_with_retry_inner(
    request: reqwest::blocking::RequestBuilder,
    mut lose_first_successful_response: bool,
) -> Result<reqwest::blocking::Response, String> {
    let mut transport_failures = 0_u64;
    let last_error = loop {
        let retry = request
            .try_clone()
            .ok_or_else(|| "request cannot be safely retried".to_string())?;
        match retry.send() {
            Ok(response) if lose_first_successful_response && response.status().is_success() => {
                // The server has committed the mutation, but the simulated
                // client never observes the response. Draining then discarding
                // it keeps the connection healthy before resending the exact
                // cloned request and exercising the server's idempotent branch.
                let _ = response.bytes();
                lose_first_successful_response = false;
            }
            Ok(response) => return Ok(response),
            Err(error) => {
                if transport_failures >= 3 {
                    break error;
                }
                transport_failures += 1;
                thread::sleep(Duration::from_millis(100 * transport_failures));
            }
        }
    };
    Err(last_error.to_string())
}

fn env_identity(prefix: &str) -> Result<Identity, String> {
    Ok(Identity {
        player_id: std::env::var(format!("{prefix}_PLAYER_ID"))
            .map_err(|_| format!("{prefix}_PLAYER_ID is required"))?,
        account_id: std::env::var(format!("{prefix}_ACCOUNT_ID"))
            .map_err(|_| format!("{prefix}_ACCOUNT_ID is required"))?,
        session: std::env::var(format!("{prefix}_SESSION"))
            .map_err(|_| format!("{prefix}_SESSION is required"))?,
    })
}

fn wait_for(
    client: &OnlineClient,
    identity: &Identity,
    match_id: &str,
    timeout: Duration,
    predicate: impl Fn(&OnlineSnapshotResponse) -> bool,
) -> Result<OnlineSnapshotResponse, String> {
    let started = Instant::now();
    while started.elapsed() < timeout {
        let snapshot = client.snapshot(identity, match_id)?;
        if predicate(&snapshot) {
            return Ok(snapshot);
        }
        thread::sleep(Duration::from_millis(20));
    }
    Err(format!("online condition timed out after {timeout:?}"))
}

fn phase_timeout() -> Duration {
    Duration::from_secs(
        std::env::var("TRNM_ONLINE_E2E_PHASE_TIMEOUT_SECONDS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(240)
            .clamp(45, 900),
    )
}

fn completion_timeout() -> Duration {
    Duration::from_secs(
        std::env::var("TRNM_ONLINE_E2E_COMPLETION_TIMEOUT_SECONDS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(360)
            .clamp(60, 1_200),
    )
}

fn point(snapshot: &OnlineSnapshotResponse, key: &str) -> Result<RtsTile, String> {
    let value = snapshot
        .snapshot
        .pointer(&format!("/seed/map/{key}"))
        .ok_or_else(|| format!("snapshot is missing map point {key}"))?;
    Ok(RtsTile::new(
        value["x"]
            .as_i64()
            .ok_or_else(|| "point.x missing".to_string())? as i32,
        value["y"]
            .as_i64()
            .ok_or_else(|| "point.y missing".to_string())? as i32,
    ))
}

fn controlled(
    snapshot: &OnlineSnapshotResponse,
    identity: &Identity,
) -> Result<Vec<String>, String> {
    let assigned = snapshot
        .view
        .members
        .iter()
        .find(|member| member.player_id == identity.player_id)
        .map(|member| member.controlled_unit_ids.clone())
        .ok_or_else(|| "identity is missing from match view".to_string())?;
    let living = snapshot.snapshot["party"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|unit| unit["hp"].as_i64().unwrap_or_default() > 0)
        .filter_map(|unit| unit["unit_id"].as_str())
        .collect::<std::collections::BTreeSet<_>>();
    Ok(assigned
        .into_iter()
        .filter(|unit_id| living.contains(unit_id.as_str()))
        .collect())
}

fn restart_server(base_url: &str) -> Result<(), String> {
    let status = Command::new("systemctl")
        .args(["--user", "restart", "trnm-game-server.service"])
        .status()
        .map_err(|error| format!("restart game server: {error}"))?;
    if !status.success() {
        return Err("systemd rejected game-server restart".to_string());
    }
    let http = Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .map_err(|error| error.to_string())?;
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(30) {
        if http
            .get(format!(
                "{}/v1/online/readiness",
                base_url.trim_end_matches('/')
            ))
            .send()
            .is_ok_and(|response| response.status().is_success())
        {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }
    Err("game server did not recover after restart".to_string())
}

fn run() -> Result<Value, String> {
    let base_url = std::env::var("TRNM_GAME_SERVER_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:7005".to_string());
    let host = env_identity("TRNM_ONLINE_HOST")?;
    let guest = env_identity("TRNM_ONLINE_GUEST")?;
    let client = Arc::new(OnlineClient::new(base_url.clone())?);
    let tick_interval_ms = client.tick_interval_ms()?;
    let run_id = format!("online-e2e-{}", chrono::Utc::now().timestamp_millis());
    let slot_key = std::env::var("TRNM_ONLINE_SLOT_KEY").unwrap_or_else(|_| run_id.clone());

    let campaign: OnlineCampaignView = client.post(
        &host,
        "/v1/online/campaigns/connect",
        &OnlineCampaignConnectRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: host.player_id.clone(),
            account_id: host.account_id.clone(),
            slot_key: slot_key.clone(),
        },
    )?;
    let guest_campaign: OnlineCampaignView = client.post(
        &guest,
        "/v1/online/campaigns/connect",
        &OnlineCampaignConnectRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: guest.player_id.clone(),
            account_id: guest.account_id.clone(),
            slot_key,
        },
    )?;
    let (created, started, start_lost_response_retry_verified) =
        if let Ok(match_id) = std::env::var("TRNM_ONLINE_EXISTING_MATCH_ID") {
            let snapshot = client.snapshot(&host, &match_id)?;
            (snapshot.view.clone(), snapshot.view, false)
        } else {
            let created: OnlineMatchView = client.post_one_shot_non_idempotent(
                &host,
                "/v1/online/matches",
                &OnlineMatchCreateRequest {
                    protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
                    build_id: ONLINE_AUTHORITY_BUILD.to_string(),
                    campaign_id: campaign.campaign_id.clone(),
                    map_id: "first_contact".to_string(),
                    expected_campaign_revision: campaign.campaign_revision,
                },
            )?;
            let _: OnlineMatchView = client.post_one_shot_non_idempotent(
                &guest,
                "/v1/online/matches/join",
                &OnlineMatchJoinRequest {
                    protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
                    build_id: ONLINE_AUTHORITY_BUILD.to_string(),
                    player_id: guest.player_id.clone(),
                    account_id: guest.account_id.clone(),
                    campaign_id: guest_campaign.campaign_id.clone(),
                    join_code: created.join_code.clone(),
                },
            )?;
            let started = client.start_with_lost_response_retry(
                &host,
                &created.match_id,
                &OnlineMatchStartRequest {
                    protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
                    build_id: ONLINE_AUTHORITY_BUILD.to_string(),
                    player_id: host.player_id.clone(),
                    account_id: host.account_id.clone(),
                    expected_match_revision: 0,
                },
            )?;
            (created, started, true)
        };
    if started.phase != OnlineMatchPhase::Running || started.members.len() != 2 {
        return Err("two-client match did not enter running phase".to_string());
    }
    let initial = client.snapshot(&host, &created.match_id)?;
    let initial_guest = client.snapshot(&guest, &created.match_id)?;
    if initial.view.match_revision != initial_guest.view.match_revision {
        return Err("initial two-player snapshots did not share one match revision".to_string());
    }
    let match_clock_started = Instant::now();
    let initial_authoritative_tick = initial.view.authoritative_tick;
    let approach = point(&initial, "approach_point")?;
    let objective = point(&initial, "objective")?;
    let host_units = controlled(&initial, &host)?;
    let guest_units = controlled(&initial_guest, &guest)?;
    let (mut state_stream, mut state_stream_cursor, streamed_initial_sequence) =
        connect_stream_smoke(&client, &host, &created.match_id, &initial)?;
    if streamed_initial_sequence != initial.view.next_sequence {
        return Err(
            "state stream initial total-order cursor diverged from HTTP snapshot".to_string(),
        );
    }

    let websocket_authoritative_effect_started = Instant::now();
    let host_submit = {
        let client = Arc::clone(&client);
        let host = host.clone();
        let match_id = created.match_id.clone();
        let snapshot = initial.clone();
        let command_id = format!("{run_id}-host-move");
        let subjects = host_units.clone();
        thread::spawn(move || {
            client.submit(
                &host,
                &match_id,
                &snapshot,
                CommandSpec {
                    command_id,
                    kind: RtsOrderKind::Move,
                    subjects,
                    target: approach,
                    queued: true,
                },
            )
        })
    };
    let guest_submit = {
        let client = Arc::clone(&client);
        let guest = guest.clone();
        let match_id = created.match_id.clone();
        let snapshot = initial_guest;
        let command_id = format!("{run_id}-guest-concurrent-move");
        thread::spawn(move || {
            client.submit(
                &guest,
                &match_id,
                &snapshot,
                CommandSpec {
                    command_id,
                    kind: RtsOrderKind::Move,
                    subjects: guest_units,
                    target: approach,
                    queued: true,
                },
            )
        })
    };
    let (first, first_request) = host_submit
        .join()
        .map_err(|_| "host concurrent submit panicked".to_string())??;
    let (guest_concurrent, _) = guest_submit
        .join()
        .map_err(|_| "guest concurrent submit panicked".to_string())??;
    let mut total_orders = [first.sequence, guest_concurrent.sequence];
    total_orders.sort_unstable();
    if total_orders[1] != total_orders[0].saturating_add(1)
        || first.input_sequence != 0
        || guest_concurrent.input_sequence != 0
    {
        return Err("two-player concurrent input did not receive independent input cursors and contiguous server total order".to_string());
    }
    wait_for_stream_total_order(
        &mut state_stream,
        &mut state_stream_cursor,
        &created.match_id,
        total_orders[1].saturating_add(1),
    )?;
    let websocket_authoritative_effect_ms =
        u64::try_from(websocket_authoritative_effect_started.elapsed().as_millis())
            .unwrap_or(u64::MAX);
    let websocket_authoritative_effect_sample_count =
        std::env::var("TRNM_ONLINE_E2E_EFFECT_SAMPLES")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(20);
    let mut websocket_authoritative_effect_samples_ms =
        Vec::with_capacity(websocket_authoritative_effect_sample_count);
    for sample in 0..websocket_authoritative_effect_sample_count {
        let effect_snapshot = client.snapshot(&host, &created.match_id)?;
        let effect_started = Instant::now();
        let (receipt, _) = client.submit(
            &host,
            &created.match_id,
            &effect_snapshot,
            CommandSpec {
                command_id: format!("{run_id}-effect-sample-{sample}"),
                kind: RtsOrderKind::Move,
                subjects: host_units.clone(),
                target: approach,
                queued: false,
            },
        )?;
        wait_for_stream_total_order(
            &mut state_stream,
            &mut state_stream_cursor,
            &created.match_id,
            receipt.sequence.saturating_add(1),
        )?;
        websocket_authoritative_effect_samples_ms
            .push(u64::try_from(effect_started.elapsed().as_millis()).unwrap_or(u64::MAX));
    }
    let (
        websocket_authoritative_effect_samples_ms,
        websocket_authoritative_effect_p95_ms,
        websocket_authoritative_effect_max_ms,
    ) = summarize_milliseconds(websocket_authoritative_effect_samples_ms);
    if let Some(maximum_p95_ms) = std::env::var("TRNM_ONLINE_E2E_MAX_EFFECT_P95_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
    {
        if websocket_authoritative_effect_p95_ms > maximum_p95_ms {
            let (_, command_ack_p95_ms, command_ack_max_ms) = client.command_ack_summary();
            return Err(format!(
                "WebSocket authoritative effect p95 {}ms exceeded {}ms; samples_ms={:?}; command_ack_p95_ms={}; command_ack_max_ms={}",
                websocket_authoritative_effect_p95_ms,
                maximum_p95_ms,
                websocket_authoritative_effect_samples_ms,
                command_ack_p95_ms,
                command_ack_max_ms,
            ));
        }
    }
    let _ = state_stream.close(None);
    let reconnect_command_race_rounds = 32_u64;
    let mut reconnect_cursor = 0_u64;
    let mut reconnect_hash = "stale-race-snapshot".to_string();
    for round in 0..reconnect_command_race_rounds {
        let command_snapshot = client.snapshot(&host, &created.match_id)?;
        let barrier = Arc::new(Barrier::new(2));
        let race_command_id = format!("{run_id}-reconnect-race-{round}");
        let command_thread = {
            let client = Arc::clone(&client);
            let host = host.clone();
            let match_id = created.match_id.clone();
            let barrier = Arc::clone(&barrier);
            let subjects = host_units.clone();
            thread::spawn(move || {
                barrier.wait();
                client.submit(
                    &host,
                    &match_id,
                    &command_snapshot,
                    CommandSpec {
                        command_id: race_command_id,
                        kind: RtsOrderKind::Move,
                        subjects,
                        target: approach,
                        queued: true,
                    },
                )
            })
        };
        let reconnect_thread = {
            let client = Arc::clone(&client);
            let guest = guest.clone();
            let match_id = created.match_id.clone();
            let barrier = Arc::clone(&barrier);
            let last_snapshot_hash = reconnect_hash.clone();
            thread::spawn(move || {
                barrier.wait();
                client.reconnect(&guest, &match_id, reconnect_cursor, last_snapshot_hash)
            })
        };
        command_thread
            .join()
            .map_err(|_| "reconnect race command panicked".to_string())??;
        let raced_reconnect = reconnect_thread
            .join()
            .map_err(|_| "reconnect race request panicked".to_string())??;
        if raced_reconnect.next_receipt_sequence < reconnect_cursor
            || raced_reconnect.view.next_sequence < raced_reconnect.next_receipt_sequence
        {
            return Err(
                "reconnect race returned a regressed or impossible replay cursor".to_string(),
            );
        }
        reconnect_cursor = raced_reconnect.next_receipt_sequence;
        reconnect_hash = raced_reconnect.view.snapshot_hash;
    }
    let duplicate: OnlineCommandReceipt = client.post(
        &host,
        &format!("/v1/online/matches/{}/commands", created.match_id),
        &first_request,
    )?;
    if !duplicate.duplicate
        || duplicate.sequence != first.sequence
        || duplicate.input_sequence != first.input_sequence
    {
        return Err("duplicate command did not return the stored receipt".to_string());
    }
    let mut tampered_duplicate = first_request.clone();
    tampered_duplicate.target_tick = tampered_duplicate.target_tick.saturating_add(1);
    let (status, _) = client.post_status(
        &host,
        &format!("/v1/online/matches/{}/commands", created.match_id),
        &tampered_duplicate,
    )?;
    if status != 409 {
        return Err(format!(
            "tampered duplicate returned HTTP {status}, expected 409"
        ));
    }
    let mut skipped = first_request.clone();
    skipped.command_id = format!("{run_id}-sequence-skip");
    skipped.input_sequence = skipped.input_sequence.map(|value| value.saturating_add(2));
    skipped.expected_match_revision = first.match_revision;
    let (status, _) = client.post_status(
        &host,
        &format!("/v1/online/matches/{}/commands", created.match_id),
        &skipped,
    )?;
    if status != 409 {
        return Err(format!(
            "input sequence skip returned HTTP {status}, expected 409"
        ));
    }
    let after_first = client.snapshot(&guest, &created.match_id)?;
    let mut theft = skipped;
    theft.player_id = guest.player_id.clone();
    theft.account_id = guest.account_id.clone();
    theft.command_id = format!("{run_id}-control-theft");
    theft.sequence = after_first.view.next_sequence;
    theft.input_sequence = after_first
        .view
        .members
        .iter()
        .find(|member| member.player_id == guest.player_id)
        .map(|member| member.next_input_sequence);
    theft.expected_match_revision = after_first.view.match_revision;
    theft.target_tick = after_first.view.authoritative_tick;
    theft.client_observed_tick = Some(after_first.view.authoritative_tick);
    theft.order.frame = u32::try_from(theft.target_tick)
        .map_err(|_| "control-theft target tick overflow".to_string())?;
    theft.order.subject_actor_ids = host_units.clone();
    let (status, _) = client.post_status(
        &guest,
        &format!("/v1/online/matches/{}/commands", created.match_id),
        &theft,
    )?;
    if status != 403 {
        return Err(format!(
            "control theft returned HTTP {status}, expected 403"
        ));
    }
    let mut old_build = theft;
    old_build.command_id = format!("{run_id}-old-build");
    old_build.build_id = "trnm-online-old-build".to_string();
    let (status, _) = client.post_status(
        &guest,
        &format!("/v1/online/matches/{}/commands", created.match_id),
        &old_build,
    )?;
    if status != 426 {
        return Err(format!("old build returned HTTP {status}, expected 426"));
    }

    let before_restart = client.snapshot(&host, &created.match_id)?;
    let order_count_before = before_restart.snapshot["order_count"]
        .as_u64()
        .unwrap_or_default();
    let restart_recovery = std::env::var("TRNM_ONLINE_E2E_RESTART_SERVER")
        .map(|value| value != "0")
        .unwrap_or(true);
    if restart_recovery {
        restart_server(&base_url)?;
    }
    let reconnected = client.reconnect(
        &guest,
        &created.match_id,
        0,
        "stale-client-snapshot".to_string(),
    )?;
    if reconnected.reconnect_count < reconnect_command_race_rounds.saturating_add(1)
        || !reconnected.full_snapshot_required
        || reconnected.replayed_commands.len() != before_restart.view.next_sequence as usize
        || reconnected.replay_truncated
        || reconnected.next_receipt_sequence != before_restart.view.next_sequence
    {
        return Err(
            "authenticated reconnect did not replay the authoritative command gap".to_string(),
        );
    }
    let after_restart = OnlineSnapshotResponse {
        view: reconnected.view,
        snapshot: reconnected.snapshot,
    };
    if after_restart.view.seed_hash != before_restart.view.seed_hash
        || after_restart.view.next_sequence != before_restart.view.next_sequence
        || after_restart.snapshot["order_count"]
            .as_u64()
            .unwrap_or_default()
            != order_count_before
        || after_restart.view.authoritative_tick < before_restart.view.authoritative_tick
    {
        return Err("restart did not preserve authoritative match/command state".to_string());
    }

    let contact = wait_for(
        &client,
        &host,
        &created.match_id,
        phase_timeout(),
        |snapshot| snapshot.snapshot["phase"] == "contact",
    )?;
    let _ = client.submit(
        &guest,
        &created.match_id,
        &contact,
        CommandSpec {
            command_id: format!("{run_id}-guest-ability"),
            kind: RtsOrderKind::Ability,
            subjects: controlled(&contact, &guest)?,
            target: objective,
            queued: false,
        },
    )?;
    let after_ability = client.snapshot(&host, &created.match_id)?;
    let _ = client.submit(
        &host,
        &created.match_id,
        &after_ability,
        CommandSpec {
            command_id: format!("{run_id}-host-attack-relay"),
            kind: RtsOrderKind::Attack,
            subjects: controlled(&after_ability, &host)?,
            target: objective,
            queued: false,
        },
    )?;
    let after_attack = client.snapshot(&guest, &created.match_id)?;
    let _ = client.submit(
        &guest,
        &created.match_id,
        &after_attack,
        CommandSpec {
            command_id: format!("{run_id}-guest-attack-relay"),
            kind: RtsOrderKind::Attack,
            subjects: controlled(&after_attack, &guest)?,
            target: objective,
            queued: true,
        },
    )?;
    let relay = wait_for(
        &client,
        &host,
        &created.match_id,
        phase_timeout(),
        |snapshot| {
            snapshot.snapshot["phase"] == "relay"
                && snapshot.snapshot["relay_guard_hp"].as_i64().unwrap_or(1) <= 0
        },
    )?;
    let _ = client.submit(
        &host,
        &created.match_id,
        &relay,
        CommandSpec {
            command_id: format!("{run_id}-host-hold-relay"),
            kind: RtsOrderKind::Hold,
            subjects: controlled(&relay, &host)?,
            target: objective,
            queued: false,
        },
    )?;

    for wave in 1..=2u64 {
        let wave_snapshot = wait_for(
            &client,
            &host,
            &created.match_id,
            phase_timeout(),
            |snapshot| {
                snapshot.snapshot["reinforcement_wave"]
                    .as_u64()
                    .unwrap_or_default()
                    >= wave
            },
        )?;
        let guest_wave_units = controlled(&wave_snapshot, &guest)?;
        let ability_identity = if guest_wave_units.is_empty() {
            &host
        } else {
            &guest
        };
        let ability_units = if guest_wave_units.is_empty() {
            controlled(&wave_snapshot, &host)?
        } else {
            guest_wave_units
        };
        let _ = client.submit(
            ability_identity,
            &created.match_id,
            &wave_snapshot,
            CommandSpec {
                command_id: format!("{run_id}-wave-{wave}-ability"),
                kind: RtsOrderKind::Ability,
                subjects: ability_units,
                target: objective,
                queued: false,
            },
        )?;
        let wave_snapshot = client.snapshot(&host, &created.match_id)?;
        let _ = client.submit(
            &host,
            &created.match_id,
            &wave_snapshot,
            CommandSpec {
                command_id: format!("{run_id}-host-attack-wave-{wave}"),
                kind: RtsOrderKind::Attack,
                subjects: controlled(&wave_snapshot, &host)?,
                target: objective,
                queued: false,
            },
        )?;
        let guest_snapshot = client.snapshot(&guest, &created.match_id)?;
        let living_guest = controlled(&guest_snapshot, &guest)?;
        if !living_guest.is_empty() {
            let _ = client.submit(
                &guest,
                &created.match_id,
                &guest_snapshot,
                CommandSpec {
                    command_id: format!("{run_id}-guest-attack-wave-{wave}"),
                    kind: RtsOrderKind::Attack,
                    subjects: living_guest,
                    target: objective,
                    queued: true,
                },
            )?;
        }
        let cleared = wait_for(
            &client,
            &host,
            &created.match_id,
            phase_timeout(),
            |snapshot| {
                snapshot.snapshot["enemies"]
                    .as_array()
                    .is_some_and(|enemies| {
                        enemies
                            .iter()
                            .all(|enemy| enemy["hp"].as_i64().unwrap_or_default() <= 0)
                    })
            },
        )?;
        let host_move_units = controlled(&cleared, &host)?;
        let _ = client.submit(
            &host,
            &created.match_id,
            &cleared,
            CommandSpec {
                command_id: format!("{run_id}-host-move-objective-{wave}"),
                kind: RtsOrderKind::Move,
                subjects: host_move_units.clone(),
                target: objective,
                queued: false,
            },
        )?;
        let host_move_units = host_move_units
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        let moved = wait_for(
            &client,
            &host,
            &created.match_id,
            phase_timeout(),
            |snapshot| {
                let ox = snapshot
                    .snapshot
                    .pointer("/seed/map/objective/x")
                    .and_then(Value::as_i64);
                let oy = snapshot
                    .snapshot
                    .pointer("/seed/map/objective/y")
                    .and_then(Value::as_i64);
                snapshot.snapshot["party"].as_array().is_some_and(|party| {
                    party.iter().any(|unit| {
                        unit["hp"].as_i64().unwrap_or_default() > 0
                            && unit["unit_id"]
                                .as_str()
                                .is_some_and(|unit_id| host_move_units.contains(unit_id))
                            && ox.zip(oy).is_some_and(|(x, y)| {
                                (unit["position"]["x"].as_i64().unwrap_or_default() - x).abs()
                                    + (unit["position"]["y"].as_i64().unwrap_or_default() - y).abs()
                                    <= 2
                            })
                    })
                })
            },
        )?;
        let _ = client.submit(
            &host,
            &created.match_id,
            &moved,
            CommandSpec {
                command_id: format!("{run_id}-host-hold-objective-{wave}"),
                kind: RtsOrderKind::Hold,
                subjects: controlled(&moved, &host)?,
                target: objective,
                queued: false,
            },
        )?;
    }

    // Keep the client-observed terminal skew as diagnostic evidence. It includes
    // the terminal publication barrier plus the final snapshot request's
    // database RTT, so the formal actor-clock gate uses the server's cumulative
    // clock telemetry instead of misclassifying read-surface latency as dropped
    // simulation ticks.
    let terminal = wait_for(
        &client,
        &host,
        &created.match_id,
        completion_timeout(),
        |snapshot| snapshot.snapshot["phase"] == "complete",
    )?;
    let match_wall_elapsed_ms =
        u64::try_from(match_clock_started.elapsed().as_millis()).unwrap_or(u64::MAX);
    let observed_match_ticks = terminal
        .view
        .authoritative_tick
        .saturating_sub(initial_authoritative_tick);
    let match_tick_drift =
        observed_match_ticks as f64 - match_wall_elapsed_ms as f64 / tick_interval_ms;
    let complete = if terminal.view.phase == OnlineMatchPhase::Complete
        && terminal.view.settlement_state == "settled"
    {
        terminal
    } else {
        wait_for(
            &client,
            &host,
            &created.match_id,
            completion_timeout(),
            |snapshot| {
                snapshot.view.phase == OnlineMatchPhase::Complete
                    && snapshot.view.settlement_state == "settled"
            },
        )?
    };
    let terminal_duplicate: OnlineCommandReceipt = client.post(
        &host,
        &format!("/v1/online/matches/{}/commands", created.match_id),
        &first_request,
    )?;
    if !terminal_duplicate.duplicate || terminal_duplicate.sequence != first.sequence {
        return Err("terminal exact duplicate did not return the durable receipt".to_string());
    }
    if complete.snapshot["outcome"] != "victory" {
        return Err(format!(
            "authoritative battle completed without victory: {}",
            complete.snapshot["outcome"]
        ));
    }
    for (identity, initial_campaign) in [(&host, &campaign), (&guest, &guest_campaign)] {
        let member = complete
            .view
            .members
            .iter()
            .find(|member| member.player_id == identity.player_id)
            .ok_or_else(|| "completed view lost one member progression".to_string())?;
        if member.experience <= initial_campaign.experience
            || member.inventory_count
                <= initial_campaign
                    .inventory
                    .iter()
                    .map(|stack| u64::from(stack.quantity))
                    .sum::<u64>()
        {
            return Err(format!(
                "online member {} did not receive independent progression/inventory",
                identity.player_id
            ));
        }
    }
    let (command_ack_ms, command_ack_p95_ms, command_ack_max_ms) = client.command_ack_summary();
    let settlement_observation_wall_elapsed_ms =
        u64::try_from(match_clock_started.elapsed().as_millis()).unwrap_or(u64::MAX);
    Ok(json!({
        "status": "passed",
        "run_id": run_id,
        "campaign_id": campaign.campaign_id,
        "match_id": created.match_id,
        "members": complete.view.members,
        "authoritative_tick": complete.view.authoritative_tick,
        "initial_authoritative_tick": initial_authoritative_tick,
        "observed_match_ticks": observed_match_ticks,
        "match_wall_elapsed_ms": match_wall_elapsed_ms,
        "tick_interval_ms": tick_interval_ms,
        "match_tick_drift": match_tick_drift,
        "match_tick_drift_scope": "client_wall_to_terminal_read_surface_includes_publication_barrier_and_snapshot_database_latency_not_formal_actor_clock_gate",
        "settlement_observation_wall_elapsed_ms": settlement_observation_wall_elapsed_ms,
        "next_sequence": complete.view.next_sequence,
        "seed_hash": complete.view.seed_hash,
        "snapshot_hash": complete.view.snapshot_hash,
        "result_hash": complete.view.result_hash,
        "settlement_state": complete.view.settlement_state,
        "start_lost_response_retry_verified": start_lost_response_retry_verified,
        "duplicate_command_exactly_once": true,
        "terminal_duplicate_command_exactly_once": true,
        "tampered_duplicate_rejected": true,
        "sequence_regression_rejected": true,
        "concurrent_player_input_sequences": true,
        "concurrent_server_total_orders": total_orders,
        "websocket_full_delta_verified": true,
        "websocket_authoritative_effect_ms": websocket_authoritative_effect_ms,
        "websocket_authoritative_effect_sample_scope": "command_submit_start_to_hash_verified_stream_total_order",
        "websocket_authoritative_effect_samples_ms": websocket_authoritative_effect_samples_ms,
        "websocket_authoritative_effect_p95_ms": websocket_authoritative_effect_p95_ms,
        "websocket_authoritative_effect_max_ms": websocket_authoritative_effect_max_ms,
        "reconnect_command_race_rounds": reconnect_command_race_rounds,
        "cross_member_control_rejected": true,
        "old_build_rejected": true,
        "restart_recovery": restart_recovery,
        "authenticated_reconnect": true,
        "replayed_commands": before_restart.view.next_sequence,
        "guest_progression": true,
        "independent_cloud_campaigns": [campaign.campaign_id, guest_campaign.campaign_id],
        "command_ack_samples": command_ack_ms.len(),
        "command_ack_ms": command_ack_ms,
        "command_ack_p95_ms": command_ack_p95_ms,
        "command_ack_max_ms": command_ack_max_ms,
    }))
}

fn main() {
    match run() {
        Ok(report) => println!("{}", serde_json::to_string_pretty(&report).unwrap()),
        Err(error) => {
            eprintln!("TRNM Online Authority E2E failed: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod snapshot_retry_tests {
    use super::*;

    #[test]
    fn retries_only_explicitly_recoverable_service_unavailability() {
        assert!(snapshot_response_is_recoverable(
            503,
            &json!({"error":"publication barrier","recoverable":true})
        ));
        assert!(!snapshot_response_is_recoverable(
            503,
            &json!({"error":"publication barrier","recoverable":false})
        ));
        assert!(!snapshot_response_is_recoverable(
            500,
            &json!({"error":"internal","recoverable":true})
        ));
        assert!(!snapshot_response_is_recoverable(
            200,
            &json!({"recoverable":true})
        ));
    }
}
