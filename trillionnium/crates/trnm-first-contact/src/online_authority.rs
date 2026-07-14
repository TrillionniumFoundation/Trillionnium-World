use crate::online_command_journal::{
    JournalStoreError, OnlineCommandJournal, OnlineCommandJournalScope, PendingExactCommandAttempt,
    MAX_PENDING_EXACT_ATTEMPTS,
};
use bevy::prelude::Resource;
use std::{
    collections::BTreeSet,
    env,
    net::TcpStream,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc::{self, Receiver, SyncSender, TrySendError},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};
use trnm_online_protocol::{
    apply_snapshot_delta, OnlineCommandReceipt, OnlineCommandSubmitRequest,
    OnlineMatchAccessRequest, OnlineSnapshotResponse, OnlineStreamConnectRequest,
    OnlineStreamServerMessage, ONLINE_AUTHORITY_BUILD, ONLINE_AUTHORITY_PROTOCOL,
    ONLINE_STREAM_PROTOCOL,
};
use trnm_rts_protocol::{RtsFrameOrder, RtsOrderSource};
use trnm_rts_sim::MissionSimV1;
use tungstenite::{
    client::IntoClientRequest,
    http::{header::SEC_WEBSOCKET_PROTOCOL, HeaderValue},
    stream::MaybeTlsStream,
    Message, WebSocket,
};

#[derive(Clone)]
struct NetworkConfig {
    base_url: String,
    match_id: String,
    player_id: String,
    account_id: String,
    player_session: String,
}

type CommandJob = PendingExactCommandAttempt;

struct CommandIntent {
    order: RtsFrameOrder,
    label: String,
    intent_id: String,
    legacy_sequence: u64,
    observed_next_input_sequence: u64,
    expected_match_revision: u64,
    client_observed_tick: u64,
}

enum JournalRequest {
    Enqueue(CommandIntent),
    Replace {
        expected: CommandJob,
        replacement: Box<CommandJob>,
        completion: SyncSender<Result<(), String>>,
    },
    Acknowledge {
        expected: CommandJob,
        receipt: OnlineCommandReceipt,
        completion: SyncSender<Result<(), String>>,
    },
    Reject {
        expected: CommandJob,
        status: u16,
        reason: String,
        completion: SyncSender<Result<(), String>>,
    },
    FailStop(String),
}

enum WorkerEvent {
    CommandAccepted(Box<WorkerCommandAccepted>),
    SnapshotRefreshCompleted,
    StreamConnected,
    StreamDisconnected(String),
    StreamResync(String),
    RefreshFailed(String),
    CommandFailed(String),
}

struct LatestWorkerSnapshot {
    view: Box<OnlineSnapshotResponse>,
    mission: Box<MissionSimV1>,
}

struct WorkerCommandAccepted {
    receipt: OnlineCommandReceipt,
    order: RtsFrameOrder,
    label: String,
    round_trip_ms: f64,
}

pub(super) enum OnlineClientEvent {
    Snapshot(Box<MissionSimV1>),
    CommandAccepted(Box<OnlineCommandAcceptedEvent>),
    Connected,
    Disconnected(String),
    Resync(String),
    RefreshFailed(String),
    CommandFailed(String),
}

pub(super) struct OnlineCommandAcceptedEvent {
    pub receipt: OnlineCommandReceipt,
    pub mission: Option<Box<MissionSimV1>>,
    pub order: RtsFrameOrder,
    pub label: String,
    pub round_trip_ms: f64,
}

#[derive(Clone, Resource)]
pub(super) struct OnlineAuthorityClient {
    player_id: String,
    controlled_unit_ids: BTreeSet<String>,
    ranked_pvp_guest: bool,
    view: OnlineSnapshotResponse,
    snapshot_tx: SyncSender<()>,
    command_tx: SyncSender<JournalRequest>,
    events: Arc<Mutex<Receiver<WorkerEvent>>>,
    latest_snapshot: Arc<Mutex<Option<LatestWorkerSnapshot>>>,
    pending_commands: Arc<AtomicUsize>,
    stream_connected: Arc<AtomicBool>,
    snapshot_in_flight: bool,
    smoothed_rtt_ms: Option<f64>,
    rtt_variation_ms: f64,
    pub poll_accumulator: f32,
}

const ONLINE_COMMAND_QUEUE_CAPACITY: usize = MAX_PENDING_EXACT_ATTEMPTS;
const ONLINE_CONTROL_EVENT_CAPACITY: usize = 64;
const ONLINE_COMMAND_RETRY_MIN: Duration = Duration::from_millis(100);
const ONLINE_COMMAND_RETRY_MAX: Duration = Duration::from_secs(2);
const ONLINE_STREAM_RECONNECT_MIN: Duration = Duration::from_millis(250);
const ONLINE_STREAM_RECONNECT_MAX: Duration = Duration::from_secs(5);
const ONLINE_STREAM_READ_TIMEOUT: Duration = Duration::from_secs(5);
const ONLINE_STREAM_MAX_MESSAGE_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone)]
struct StreamCursor {
    actor_generation: Option<String>,
    state_sequence: Option<u64>,
    match_revision: u64,
    next_receipt_sequence: u64,
    snapshot_hash: String,
    authoritative_tick: u64,
    snapshot: serde_json::Value,
}

impl StreamCursor {
    fn from_snapshot(snapshot: &OnlineSnapshotResponse) -> Self {
        Self {
            actor_generation: None,
            state_sequence: None,
            match_revision: snapshot.view.match_revision,
            next_receipt_sequence: snapshot.view.next_sequence,
            snapshot_hash: snapshot.view.snapshot_hash.clone(),
            authoritative_tick: snapshot.view.authoritative_tick,
            snapshot: snapshot.snapshot.clone(),
        }
    }
}

enum StreamApply {
    Snapshot {
        view: Box<OnlineSnapshotResponse>,
        mission: Box<MissionSimV1>,
        full: bool,
        generation_change: Option<String>,
    },
    Resync(String),
}

impl OnlineAuthorityClient {
    pub fn controls(&self, unit_id: &str) -> bool {
        self.controlled_unit_ids.contains(unit_id)
    }

    pub fn ranked_pvp_guest(&self) -> bool {
        self.ranked_pvp_guest
    }

    pub fn from_env() -> Result<Option<(Self, MissionSimV1)>, String> {
        let Some(base_url) = env::var("TRNM_ONLINE_AUTHORITY_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
        else {
            return Ok(None);
        };
        validate_authority_base_url(&base_url)?;
        let config = NetworkConfig {
            base_url: base_url.trim_end_matches('/').to_string(),
            match_id: required("TRNM_ONLINE_MATCH_ID")?,
            player_id: required("TRNM_CEX_ACTOR_ID")?,
            account_id: required("TRNM_CEX_ACCOUNT_ID")?,
            player_session: required("TRNM_CEX_PLAYER_SESSION")?,
        };
        // Initial attachment happens before the render loop exists. All traffic
        // after this point is owned by background workers.
        let initial_client = build_http_client()?;
        let view = fetch_snapshot(&initial_client, &config)?;
        let member = view
            .view
            .members
            .iter()
            .find(|member| {
                member.player_id == config.player_id && member.account_id == config.account_id
            })
            .ok_or_else(|| "online match does not contain this player/account".to_string())?;
        let controlled_unit_ids = member.controlled_unit_ids.iter().cloned().collect();
        let ranked_pvp_guest = view.view.match_mode == "ranked_pvp" && member.role == "coop_guest";
        let journal_scope = OnlineCommandJournalScope::new(
            config.match_id.clone(),
            config.player_id.clone(),
            config.account_id.clone(),
        );
        let journal_path = command_journal_path(&config)?;
        let mut command_journal = OnlineCommandJournal::load_or_new(journal_path, journal_scope)?;
        if member.next_input_sequence > command_journal.next_input_sequence {
            command_journal.advance_input_sequence(member.next_input_sequence)?;
        }
        if view.view.next_sequence < command_journal.next_receipt_sequence {
            return Err(format!(
                "online authority receipt cursor {} is behind durable journal cursor {}",
                view.view.next_sequence, command_journal.next_receipt_sequence
            ));
        }
        command_journal
            .update_receipt_cursor(view.view.next_sequence, view.view.snapshot_hash.clone())?;
        command_journal.store().map_err(|error| error.to_string())?;
        let pending_commands = Arc::new(AtomicUsize::new(
            command_journal.pending_exact_attempts.len(),
        ));
        let mission = decode_mission(&view, &config.match_id)?;
        let stream_cursor = StreamCursor::from_snapshot(&view);

        let (snapshot_tx, snapshot_rx) = mpsc::sync_channel(1);
        let (command_tx, command_rx) = mpsc::sync_channel(ONLINE_COMMAND_QUEUE_CAPACITY);
        let (event_tx, event_rx) = mpsc::sync_channel(ONLINE_CONTROL_EVENT_CAPACITY);
        let latest_snapshot = Arc::new(Mutex::new(None));
        let stream_connected = Arc::new(AtomicBool::new(false));
        spawn_snapshot_worker(
            config.clone(),
            build_http_client()?,
            snapshot_rx,
            event_tx.clone(),
            Arc::clone(&latest_snapshot),
        )?;
        spawn_command_workers(
            config.clone(),
            build_http_client()?,
            command_rx,
            command_tx.clone(),
            event_tx.clone(),
            command_journal,
            Arc::clone(&pending_commands),
        )?;
        spawn_stream_worker(
            config.clone(),
            stream_cursor,
            event_tx,
            Arc::clone(&stream_connected),
            Arc::clone(&latest_snapshot),
        )?;

        Ok(Some((
            Self {
                player_id: config.player_id,
                controlled_unit_ids,
                ranked_pvp_guest,
                view,
                snapshot_tx,
                command_tx,
                events: Arc::new(Mutex::new(event_rx)),
                latest_snapshot,
                pending_commands,
                stream_connected,
                snapshot_in_flight: false,
                smoothed_rtt_ms: None,
                rtt_variation_ms: 0.0,
                poll_accumulator: 0.0,
            },
            mission,
        )))
    }

    pub fn request_refresh(&mut self) -> Result<bool, String> {
        if self.snapshot_in_flight {
            return Ok(false);
        }
        match self.snapshot_tx.try_send(()) {
            Ok(()) => {
                self.snapshot_in_flight = true;
                Ok(true)
            }
            Err(TrySendError::Full(())) => Ok(false),
            Err(TrySendError::Disconnected(())) => {
                Err("online snapshot worker disconnected".to_string())
            }
        }
    }

    pub fn is_stream_connected(&self) -> bool {
        self.stream_connected.load(Ordering::Acquire)
    }

    pub fn submit(&mut self, mut order: RtsFrameOrder, label: String) -> Result<(), String> {
        order
            .subject_actor_ids
            .retain(|unit_id| self.controlled_unit_ids.contains(unit_id));
        if order.subject_actor_ids.is_empty() {
            return Err("selection contains no units assigned to this online member".to_string());
        }
        let target_tick = self.view.view.authoritative_tick;
        order.frame = u32::try_from(target_tick)
            .map_err(|_| "online authoritative tick exceeds frame range".to_string())?;
        order.player_id = self.player_id.clone();
        order.source = RtsOrderSource::LocalInput;
        let intent_id = uuid::Uuid::new_v4().to_string();
        if order.queued {
            order.queue_id = Some(format!("native-online:{intent_id}"));
        }
        let intent = CommandIntent {
            order,
            label,
            intent_id,
            legacy_sequence: self.view.view.next_sequence,
            observed_next_input_sequence: member_next_input_sequence(&self.view, &self.player_id)?,
            expected_match_revision: self.view.view.match_revision,
            client_observed_tick: self.view.view.authoritative_tick,
        };
        reserve_pending_slot(&self.pending_commands)?;
        match self.command_tx.try_send(JournalRequest::Enqueue(intent)) {
            Ok(()) => Ok(()),
            Err(error) => {
                release_pending_slot(&self.pending_commands);
                match error {
                    TrySendError::Full(_) => {
                        Err("online command queue is full; input was not accepted".to_string())
                    }
                    TrySendError::Disconnected(_) => {
                        Err("online command worker disconnected; input was not accepted"
                            .to_string())
                    }
                }
            }
        }
    }

    pub fn drain_events(&mut self) -> Vec<OnlineClientEvent> {
        let latest_snapshot = self
            .latest_snapshot
            .lock()
            .ok()
            .and_then(|mut latest| latest.take());
        let mut worker_events = Vec::new();
        if let Ok(events) = self.events.lock() {
            while let Ok(event) = events.try_recv() {
                worker_events.push(event);
            }
        }
        let mut client_events = Vec::with_capacity(worker_events.len().saturating_add(1));
        if let Some(LatestWorkerSnapshot { view, mission }) = latest_snapshot {
            match compare_snapshot_freshness(&view, &self.view) {
                Ok(true) => {
                    self.view = *view;
                    client_events.push(OnlineClientEvent::Snapshot(mission));
                }
                Ok(false) => {}
                Err(error) => client_events.push(OnlineClientEvent::Resync(error)),
            }
        }
        for event in worker_events {
            match event {
                WorkerEvent::CommandAccepted(event) => {
                    self.observe_round_trip(event.round_trip_ms);
                    client_events.push(OnlineClientEvent::CommandAccepted(Box::new(
                        OnlineCommandAcceptedEvent {
                            receipt: event.receipt,
                            mission: None,
                            order: event.order,
                            label: event.label,
                            round_trip_ms: event.round_trip_ms,
                        },
                    )));
                }
                WorkerEvent::StreamConnected => {
                    self.poll_accumulator = 0.0;
                    client_events.push(OnlineClientEvent::Connected);
                }
                WorkerEvent::SnapshotRefreshCompleted => {
                    self.snapshot_in_flight = false;
                }
                WorkerEvent::StreamDisconnected(error) => {
                    client_events.push(OnlineClientEvent::Disconnected(error));
                }
                WorkerEvent::StreamResync(error) => {
                    client_events.push(OnlineClientEvent::Resync(error));
                }
                WorkerEvent::RefreshFailed(error) => {
                    self.snapshot_in_flight = false;
                    client_events.push(OnlineClientEvent::RefreshFailed(error));
                }
                WorkerEvent::CommandFailed(error) => {
                    client_events.push(OnlineClientEvent::CommandFailed(error));
                }
            }
        }
        client_events
    }

    fn observe_round_trip(&mut self, sample_ms: f64) {
        if let Some(smoothed) = self.smoothed_rtt_ms {
            self.rtt_variation_ms =
                0.75 * self.rtt_variation_ms + 0.25 * (smoothed - sample_ms).abs();
            self.smoothed_rtt_ms = Some(0.875 * smoothed + 0.125 * sample_ms);
        } else {
            self.smoothed_rtt_ms = Some(sample_ms);
            self.rtt_variation_ms = sample_ms / 2.0;
        }
    }
}

fn spawn_snapshot_worker(
    config: NetworkConfig,
    client: reqwest::blocking::Client,
    requests: Receiver<()>,
    events: SyncSender<WorkerEvent>,
    latest_snapshot: Arc<Mutex<Option<LatestWorkerSnapshot>>>,
) -> Result<(), String> {
    thread::Builder::new()
        .name("trnm-online-snapshot".to_string())
        .spawn(move || {
            while requests.recv().is_ok() {
                let result = fetch_snapshot(&client, &config).and_then(|view| {
                    decode_mission(&view, &config.match_id).map(|mission| (view, mission))
                });
                match result {
                    Ok((view, mission)) => {
                        let stored = publish_latest_snapshot(
                            &latest_snapshot,
                            LatestWorkerSnapshot {
                                view: Box::new(view),
                                mission: Box::new(mission),
                            },
                        );
                        if let Err(error) = stored {
                            let _ = events.send(WorkerEvent::RefreshFailed(error));
                            break;
                        }
                        if events.send(WorkerEvent::SnapshotRefreshCompleted).is_err() {
                            break;
                        }
                    }
                    Err(error) => {
                        if events.send(WorkerEvent::RefreshFailed(error)).is_err() {
                            break;
                        }
                    }
                }
            }
        })
        .map(|_| ())
        .map_err(|error| format!("spawn online snapshot worker: {error}"))
}

fn spawn_stream_worker(
    config: NetworkConfig,
    mut cursor: StreamCursor,
    events: SyncSender<WorkerEvent>,
    connected: Arc<AtomicBool>,
    latest_snapshot: Arc<Mutex<Option<LatestWorkerSnapshot>>>,
) -> Result<(), String> {
    thread::Builder::new()
        .name("trnm-online-state-stream".to_string())
        .spawn(move || {
            let mut reconnect_delay = ONLINE_STREAM_RECONNECT_MIN;
            let mut disconnected_reported = false;
            loop {
                let mut established = false;
                connected.store(false, Ordering::Release);
                let result = consume_stream_connection(
                    &config,
                    &mut cursor,
                    &events,
                    &connected,
                    &mut established,
                    &latest_snapshot,
                );
                let was_connected = connected.swap(false, Ordering::AcqRel);
                if established {
                    disconnected_reported = false;
                    reconnect_delay = ONLINE_STREAM_RECONNECT_MIN;
                }
                if was_connected || !disconnected_reported {
                    let reason = result
                        .err()
                        .unwrap_or_else(|| "online state stream closed".to_string());
                    if events
                        .send(WorkerEvent::StreamDisconnected(reason))
                        .is_err()
                    {
                        break;
                    }
                    disconnected_reported = true;
                }
                thread::sleep(reconnect_delay);
                if !established {
                    reconnect_delay = reconnect_delay
                        .saturating_mul(2)
                        .min(ONLINE_STREAM_RECONNECT_MAX);
                }
            }
        })
        .map(|_| ())
        .map_err(|error| format!("spawn online state stream worker: {error}"))
}

fn consume_stream_connection(
    config: &NetworkConfig,
    cursor: &mut StreamCursor,
    events: &SyncSender<WorkerEvent>,
    connected: &Arc<AtomicBool>,
    established: &mut bool,
    latest_snapshot: &Arc<Mutex<Option<LatestWorkerSnapshot>>>,
) -> Result<(), String> {
    let stream_url = build_stream_url(config, cursor)?;
    let mut request = stream_url
        .as_str()
        .into_client_request()
        .map_err(|error| format!("online state stream request: {error}"))?;
    request.headers_mut().insert(
        "x-trnm-player-session",
        HeaderValue::from_str(&config.player_session)
            .map_err(|error| format!("online state stream session header: {error}"))?,
    );
    request.headers_mut().insert(
        SEC_WEBSOCKET_PROTOCOL,
        HeaderValue::from_static(ONLINE_STREAM_PROTOCOL),
    );
    let websocket_config = tungstenite::protocol::WebSocketConfig {
        write_buffer_size: 16 * 1024,
        max_write_buffer_size: 2 * 1024 * 1024,
        max_message_size: Some(ONLINE_STREAM_MAX_MESSAGE_BYTES),
        max_frame_size: Some(ONLINE_STREAM_MAX_MESSAGE_BYTES),
        ..Default::default()
    };
    // Redirects are deliberately disabled so the authenticated session header
    // can never be forwarded to a different authority endpoint.
    let (mut socket, response) =
        tungstenite::client::connect_with_config(request, Some(websocket_config), 0)
            .map_err(|error| format!("connect online state stream: {error}"))?;
    let selected_protocol = response
        .headers()
        .get(SEC_WEBSOCKET_PROTOCOL)
        .and_then(|value| value.to_str().ok());
    if selected_protocol != Some(ONLINE_STREAM_PROTOCOL) {
        let _ = socket.close(None);
        return Err("online state stream selected an unexpected subprotocol".to_string());
    }
    configure_stream_socket(&mut socket)?;

    loop {
        let message = socket
            .read()
            .map_err(|error| format!("read online state stream: {error}"))?;
        match message {
            Message::Text(text) => {
                let server_message = serde_json::from_str::<OnlineStreamServerMessage>(&text)
                    .map_err(|error| format!("decode online state stream message: {error}"))?;
                match apply_stream_message(cursor, &config.match_id, server_message) {
                    Ok(StreamApply::Snapshot {
                        view,
                        mission,
                        full,
                        generation_change,
                    }) => {
                        if let Some(reason) = generation_change {
                            events
                                .send(WorkerEvent::StreamResync(reason))
                                .map_err(|_| "online state stream receiver closed".to_string())?;
                        }
                        publish_latest_snapshot(
                            latest_snapshot,
                            LatestWorkerSnapshot { view, mission },
                        )?;
                        if full && !*established {
                            connected.store(true, Ordering::Release);
                            events
                                .send(WorkerEvent::StreamConnected)
                                .map_err(|_| "online state stream receiver closed".to_string())?;
                            *established = true;
                        }
                    }
                    Ok(StreamApply::Resync(reason)) => {
                        connected.store(false, Ordering::Release);
                        events
                            .send(WorkerEvent::StreamResync(reason.clone()))
                            .map_err(|_| "online state stream receiver closed".to_string())?;
                        let _ = socket.close(None);
                        return Err(format!("online authority requested resync: {reason}"));
                    }
                    Err(error) => {
                        connected.store(false, Ordering::Release);
                        events
                            .send(WorkerEvent::StreamResync(error.clone()))
                            .map_err(|_| "online state stream receiver closed".to_string())?;
                        let _ = socket.close(None);
                        return Err(format!("online state stream cursor rejected: {error}"));
                    }
                }
            }
            Message::Ping(_) => {
                // Tungstenite queues the RFC-required pong while reading.
                socket
                    .flush()
                    .map_err(|error| format!("flush online state stream pong: {error}"))?;
            }
            Message::Pong(_) => {}
            Message::Close(_) => return Ok(()),
            Message::Binary(_) | Message::Frame(_) => {
                let reason = "online state stream requires JSON text messages".to_string();
                connected.store(false, Ordering::Release);
                events
                    .send(WorkerEvent::StreamResync(reason.clone()))
                    .map_err(|_| "online state stream receiver closed".to_string())?;
                let _ = socket.close(None);
                return Err(reason);
            }
        }
    }
}

fn configure_stream_socket(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
) -> Result<(), String> {
    let set_timeouts = |stream: &TcpStream| {
        stream
            .set_read_timeout(Some(ONLINE_STREAM_READ_TIMEOUT))
            .and_then(|()| stream.set_write_timeout(Some(ONLINE_STREAM_READ_TIMEOUT)))
            .map_err(|error| format!("configure online state stream timeout: {error}"))
    };
    match socket.get_mut() {
        MaybeTlsStream::Plain(stream) => set_timeouts(stream),
        MaybeTlsStream::Rustls(stream) => set_timeouts(&stream.sock),
        _ => Err("online state stream uses an unsupported TLS transport".to_string()),
    }
}

fn build_stream_url(config: &NetworkConfig, cursor: &StreamCursor) -> Result<reqwest::Url, String> {
    let mut url = reqwest::Url::parse(&config.base_url)
        .map_err(|error| format!("online authority URL: {error}"))?;
    let websocket_scheme = match url.scheme() {
        "http" => "ws",
        "https" => "wss",
        scheme => return Err(format!("unsupported online authority URL scheme: {scheme}")),
    };
    url.set_scheme(websocket_scheme)
        .map_err(|()| "unable to set online state stream URL scheme".to_string())?;
    let match_id = uuid::Uuid::parse_str(&config.match_id)
        .map_err(|error| format!("online match id for state stream: {error}"))?;
    let base_path = url.path().trim_end_matches('/').to_string();
    url.set_path(&format!("{base_path}/v1/online/matches/{match_id}/stream"));
    url.set_fragment(None);
    url.set_query(None);
    let request = OnlineStreamConnectRequest {
        protocol_version: ONLINE_STREAM_PROTOCOL.to_string(),
        build_id: ONLINE_AUTHORITY_BUILD.to_string(),
        player_id: config.player_id.clone(),
        account_id: config.account_id.clone(),
        next_receipt_sequence: cursor.next_receipt_sequence,
        last_snapshot_hash: cursor.snapshot_hash.clone(),
    };
    url.query_pairs_mut()
        .append_pair("protocol_version", &request.protocol_version)
        .append_pair("build_id", &request.build_id)
        .append_pair("player_id", &request.player_id)
        .append_pair("account_id", &request.account_id)
        .append_pair(
            "next_receipt_sequence",
            &request.next_receipt_sequence.to_string(),
        )
        .append_pair("last_snapshot_hash", &request.last_snapshot_hash);
    Ok(url)
}

fn apply_stream_message(
    cursor: &mut StreamCursor,
    expected_match_id: &str,
    message: OnlineStreamServerMessage,
) -> Result<StreamApply, String> {
    match message {
        OnlineStreamServerMessage::FullSnapshot {
            actor_generation,
            state_sequence,
            next_receipt_sequence,
            view,
            snapshot,
        } => {
            validate_actor_generation(&actor_generation)?;
            validate_stream_view(
                &view,
                expected_match_id,
                &view.snapshot_hash,
                view.authoritative_tick,
            )?;
            if next_receipt_sequence != view.next_sequence {
                return Err("online full snapshot receipt cursor mismatch".to_string());
            }
            let generation_change = validate_full_snapshot_progress(
                cursor,
                &actor_generation,
                state_sequence,
                next_receipt_sequence,
                &view,
            )?;
            let mission = validate_stream_snapshot(&view, &snapshot, expected_match_id)?;
            cursor.actor_generation = Some(actor_generation);
            cursor.state_sequence = Some(state_sequence);
            cursor.match_revision = view.match_revision;
            cursor.next_receipt_sequence = next_receipt_sequence;
            cursor.snapshot_hash = view.snapshot_hash.clone();
            cursor.authoritative_tick = view.authoritative_tick;
            cursor.snapshot = snapshot.clone();
            Ok(StreamApply::Snapshot {
                view: Box::new(OnlineSnapshotResponse { view, snapshot }),
                mission: Box::new(mission),
                full: true,
                generation_change,
            })
        }
        OnlineStreamServerMessage::SnapshotDelta {
            actor_generation,
            state_sequence,
            base_state_sequence,
            view,
            delta,
        } => {
            if cursor.actor_generation.as_deref() != Some(actor_generation.as_str()) {
                return Err("online snapshot delta actor generation mismatch".to_string());
            }
            let current_state_sequence = cursor.state_sequence.ok_or_else(|| {
                "online snapshot delta arrived before a full snapshot".to_string()
            })?;
            if base_state_sequence != current_state_sequence {
                return Err("online snapshot delta base state sequence mismatch".to_string());
            }
            if state_sequence <= base_state_sequence {
                return Err("online snapshot delta state sequence is out of order".to_string());
            }
            if view.match_revision < cursor.match_revision
                || delta.authoritative_tick < cursor.authoritative_tick
            {
                return Err("online snapshot delta regressed the authority cursor".to_string());
            }
            validate_stream_view(
                &view,
                expected_match_id,
                &delta.snapshot_hash,
                delta.authoritative_tick,
            )?;
            if view.next_sequence < cursor.next_receipt_sequence {
                return Err("online snapshot delta receipt cursor regressed".to_string());
            }
            let mut snapshot = cursor.snapshot.clone();
            apply_snapshot_delta(
                &mut snapshot,
                &cursor.snapshot_hash,
                cursor.authoritative_tick,
                &delta,
            )?;
            let mission = validate_stream_snapshot(&view, &snapshot, expected_match_id)?;
            cursor.state_sequence = Some(state_sequence);
            cursor.match_revision = view.match_revision;
            cursor.next_receipt_sequence = view.next_sequence;
            cursor.snapshot_hash = delta.snapshot_hash;
            cursor.authoritative_tick = delta.authoritative_tick;
            cursor.snapshot = snapshot.clone();
            Ok(StreamApply::Snapshot {
                view: Box::new(OnlineSnapshotResponse { view, snapshot }),
                mission: Box::new(mission),
                full: false,
                generation_change: None,
            })
        }
        OnlineStreamServerMessage::ResyncRequired {
            actor_generation,
            reason,
        } => {
            if let Some(current_generation) = cursor.actor_generation.as_deref() {
                if actor_generation != current_generation {
                    return Ok(StreamApply::Resync(format!(
                        "actor generation changed from {current_generation} to {actor_generation}: {reason}"
                    )));
                }
            }
            Ok(StreamApply::Resync(reason))
        }
    }
}

fn validate_full_snapshot_progress(
    cursor: &StreamCursor,
    actor_generation: &str,
    state_sequence: u64,
    next_receipt_sequence: u64,
    view: &trnm_online_protocol::OnlineMatchView,
) -> Result<Option<String>, String> {
    let generation_change = cursor
        .actor_generation
        .as_deref()
        .filter(|current| *current != actor_generation)
        .map(|current| {
            format!(
                "actor generation changed from {current} to {actor_generation}; accepted verified full snapshot cursor reset"
            )
        });
    if next_receipt_sequence < cursor.next_receipt_sequence {
        return Err("online full snapshot receipt cursor regressed".to_string());
    }
    if view.match_revision < cursor.match_revision
        || view.authoritative_tick < cursor.authoritative_tick
    {
        return Err("online full snapshot regressed the authority cursor".to_string());
    }
    if view.match_revision == cursor.match_revision
        && view.authoritative_tick == cursor.authoritative_tick
        && next_receipt_sequence == cursor.next_receipt_sequence
        && view.snapshot_hash != cursor.snapshot_hash
    {
        return Err(
            "online full snapshot changed hash without advancing the authority cursor".to_string(),
        );
    }
    if cursor.actor_generation.as_deref() == Some(actor_generation) {
        let current_sequence = cursor
            .state_sequence
            .ok_or_else(|| "online stream generation has no state sequence".to_string())?;
        if state_sequence < current_sequence {
            return Err("online full snapshot state sequence regressed".to_string());
        }
        if state_sequence == current_sequence
            && (view.snapshot_hash != cursor.snapshot_hash
                || view.authoritative_tick != cursor.authoritative_tick
                || view.match_revision != cursor.match_revision
                || next_receipt_sequence != cursor.next_receipt_sequence)
        {
            return Err(
                "online full snapshot changed data without advancing state sequence".to_string(),
            );
        }
    }
    Ok(generation_change)
}

fn validate_actor_generation(actor_generation: &str) -> Result<(), String> {
    uuid::Uuid::parse_str(actor_generation)
        .map(|_| ())
        .map_err(|error| format!("online state stream actor generation: {error}"))
}

fn validate_stream_view(
    view: &trnm_online_protocol::OnlineMatchView,
    expected_match_id: &str,
    expected_snapshot_hash: &str,
    expected_tick: u64,
) -> Result<(), String> {
    if view.protocol_version != ONLINE_AUTHORITY_PROTOCOL || view.build_id != ONLINE_AUTHORITY_BUILD
    {
        return Err("online state stream authority protocol/build mismatch".to_string());
    }
    if view.match_id != expected_match_id {
        return Err("online state stream match id mismatch".to_string());
    }
    if view.snapshot_hash != expected_snapshot_hash || view.authoritative_tick != expected_tick {
        return Err("online state stream view cursor mismatch".to_string());
    }
    if view.snapshot_hash.len() != 64
        || !view
            .snapshot_hash
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("online state stream snapshot hash is malformed".to_string());
    }
    Ok(())
}

fn validate_stream_snapshot(
    view: &trnm_online_protocol::OnlineMatchView,
    snapshot: &serde_json::Value,
    expected_match_id: &str,
) -> Result<MissionSimV1, String> {
    validate_stream_view(
        view,
        expected_match_id,
        &view.snapshot_hash,
        view.authoritative_tick,
    )?;
    if snapshot.is_null() {
        return Err("online state stream has no running simulation snapshot".to_string());
    }
    let mission = serde_json::from_value::<MissionSimV1>(snapshot.clone())
        .map_err(|error| format!("online state stream mission decode: {error}"))?;
    if mission.tick != view.authoritative_tick {
        return Err("online state stream mission tick mismatch".to_string());
    }
    let computed_hash = mission
        .snapshot_hash()
        .map_err(|error| format!("online state stream mission hash: {error}"))?;
    if computed_hash != view.snapshot_hash {
        return Err("online state stream mission snapshot hash mismatch".to_string());
    }
    Ok(mission)
}

fn spawn_command_workers(
    config: NetworkConfig,
    client: reqwest::blocking::Client,
    requests: Receiver<JournalRequest>,
    mutation_tx: SyncSender<JournalRequest>,
    events: SyncSender<WorkerEvent>,
    journal: OnlineCommandJournal,
    pending_commands: Arc<AtomicUsize>,
) -> Result<(), String> {
    let recovered = journal
        .pending_exact_attempts
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    let (network_tx, network_rx) = mpsc::sync_channel(ONLINE_COMMAND_QUEUE_CAPACITY);
    let network_events = events.clone();
    let network_pending = Arc::clone(&pending_commands);
    thread::Builder::new()
        .name("trnm-online-command-network".to_string())
        .spawn(move || {
            while let Ok(job) = network_rx.recv() {
                match submit_command_until_resolved(&client, &config, &mutation_tx, job) {
                    Ok(event) => {
                        release_pending_slot(&network_pending);
                        if network_events.send(event).is_err() {
                            return;
                        }
                    }
                    Err(error) => {
                        let _ = mutation_tx.send(JournalRequest::FailStop(error));
                        return;
                    }
                }
            }
        })
        .map_err(|error| format!("spawn online command network worker: {error}"))?;
    thread::Builder::new()
        .name("trnm-online-command-journal".to_string())
        .spawn(move || run_journal_owner(journal, recovered, requests, network_tx, events))
        .map(|_| ())
        .map_err(|error| format!("spawn online command journal worker: {error}"))
}

fn run_journal_owner(
    mut journal: OnlineCommandJournal,
    recovered: Vec<CommandJob>,
    requests: Receiver<JournalRequest>,
    network: SyncSender<CommandJob>,
    events: SyncSender<WorkerEvent>,
) {
    for job in recovered {
        if network.send(job).is_err() {
            fail_stop_journal_owner(
                &events,
                "online command network worker stopped during journal recovery".to_string(),
            );
            return;
        }
    }
    while let Ok(request) = requests.recv() {
        match request {
            JournalRequest::Enqueue(intent) => {
                let before = journal.clone();
                let result = enqueue_command_intent(&mut journal, intent).and_then(|job| {
                    store_journal_mutation(&mut journal, before, "enqueue durable command")?;
                    Ok(job)
                });
                match result {
                    Ok(job) => {
                        if network.send(job).is_err() {
                            fail_stop_journal_owner(
                                &events,
                                "online command network worker stopped after durable enqueue"
                                    .to_string(),
                            );
                            return;
                        }
                    }
                    Err(error) => {
                        fail_stop_journal_owner(&events, error);
                        return;
                    }
                }
            }
            JournalRequest::Replace {
                expected,
                replacement,
                completion,
            } => {
                let before = journal.clone();
                let result = journal
                    .replace_exact_attempt(&expected.intent_id, *replacement)
                    .and_then(|previous| {
                        if previous == expected {
                            Ok(())
                        } else {
                            Err("online command journal replaced a different exact attempt"
                                .to_string())
                        }
                    })
                    .and_then(|()| {
                        store_journal_mutation(
                            &mut journal,
                            before,
                            "replace durable command attempt",
                        )
                    });
                let failed = result.is_err();
                let failure = result.as_ref().err().cloned();
                let _ = completion.send(result);
                if failed {
                    fail_stop_journal_owner(
                        &events,
                        failure.unwrap_or_else(|| "durable replacement failed".to_string()),
                    );
                    return;
                }
            }
            JournalRequest::Acknowledge {
                expected,
                receipt,
                completion,
            } => {
                let before = journal.clone();
                let result = journal
                    .acknowledge(&receipt)
                    .and_then(|acknowledged| {
                        if acknowledged == expected {
                            Ok(())
                        } else {
                            Err(
                                "online command journal acknowledged a different exact attempt"
                                    .to_string(),
                            )
                        }
                    })
                    .and_then(|()| {
                        store_journal_mutation(&mut journal, before, "acknowledge durable command")
                    });
                let failed = result.is_err();
                let failure = result.as_ref().err().cloned();
                let _ = completion.send(result);
                if failed {
                    fail_stop_journal_owner(
                        &events,
                        failure.unwrap_or_else(|| "durable acknowledgement failed".to_string()),
                    );
                    return;
                }
            }
            JournalRequest::Reject {
                expected,
                status,
                reason,
                completion,
            } => {
                let before = journal.clone();
                let result = journal.reject(&expected, status, reason).and_then(|()| {
                    store_journal_mutation(&mut journal, before, "dead-letter durable command")
                });
                let failed = result.is_err();
                let failure = result.as_ref().err().cloned();
                let _ = completion.send(result);
                if failed {
                    fail_stop_journal_owner(
                        &events,
                        failure.unwrap_or_else(|| "durable dead-letter failed".to_string()),
                    );
                    return;
                }
            }
            JournalRequest::FailStop(error) => {
                fail_stop_journal_owner(&events, error);
                return;
            }
        }
    }
}

fn enqueue_command_intent(
    journal: &mut OnlineCommandJournal,
    intent: CommandIntent,
) -> Result<CommandJob, String> {
    if intent.observed_next_input_sequence > journal.next_input_sequence {
        journal.advance_input_sequence(intent.observed_next_input_sequence)?;
    }
    let input_sequence = journal.next_input_sequence;
    let request = OnlineCommandSubmitRequest {
        protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
        build_id: ONLINE_AUTHORITY_BUILD.to_string(),
        player_id: journal.scope.player_id.clone(),
        account_id: journal.scope.account_id.clone(),
        command_id: native_command_id(&intent.intent_id, 0),
        sequence: intent.legacy_sequence,
        input_sequence: Some(input_sequence),
        expected_match_revision: intent.expected_match_revision,
        target_tick: intent.client_observed_tick,
        client_observed_tick: Some(intent.client_observed_tick),
        order: intent.order.clone(),
    };
    let job = CommandJob {
        request,
        order: intent.order,
        label: intent.label,
        intent_id: intent.intent_id,
        attempt: 0,
    };
    journal.enqueue_exact_attempt(job.clone())?;
    Ok(job)
}

fn store_journal_mutation(
    journal: &mut OnlineCommandJournal,
    before: OnlineCommandJournal,
    context: &str,
) -> Result<(), String> {
    let store_result = journal.store();
    resolve_journal_store_result(journal, before, context, store_result)
}

fn resolve_journal_store_result(
    journal: &mut OnlineCommandJournal,
    before: OnlineCommandJournal,
    context: &str,
    store_result: Result<(), JournalStoreError>,
) -> Result<(), String> {
    match store_result {
        Ok(()) => Ok(()),
        Err(error) if error.durability_uncertain() => Err(format!(
            "{context} failed after installation; forward state retained and command pipeline fail-stopped: {error}"
        )),
        Err(error) => {
            *journal = before;
            Err(format!(
                "{context} was not installed; prior state restored and command pipeline fail-stopped: {error}"
            ))
        }
    }
}

fn fail_stop_journal_owner(events: &SyncSender<WorkerEvent>, error: String) {
    let _ = events.try_send(WorkerEvent::CommandFailed(format!(
        "online command journal fail-stopped: {error}"
    )));
}

fn submit_command_until_resolved(
    client: &reqwest::blocking::Client,
    config: &NetworkConfig,
    journal: &SyncSender<JournalRequest>,
    mut job: CommandJob,
) -> Result<WorkerEvent, String> {
    let started = Instant::now();
    let mut retry_delay = ONLINE_COMMAND_RETRY_MIN;
    loop {
        let (status, body) = match send_command_request(client, config, &job.request) {
            Ok(response) => response,
            Err(_) => {
                thread::sleep(retry_delay);
                retry_delay = retry_delay.saturating_mul(2).min(ONLINE_COMMAND_RETRY_MAX);
                continue;
            }
        };
        if status.is_success() {
            let receipt = serde_json::from_str::<OnlineCommandReceipt>(&body)
                .map_err(|error| format!("online command receipt: {error}"))?;
            if receipt.protocol_version != ONLINE_AUTHORITY_PROTOCOL
                || receipt.match_id != config.match_id
                || receipt.player_id != config.player_id
                || receipt.command_id != job.request.command_id
            {
                return Err("online command receipt identity/contract mismatch".to_string());
            }
            request_journal_acknowledgement(journal, &job, &receipt)?;
            return Ok(WorkerEvent::CommandAccepted(Box::new(
                WorkerCommandAccepted {
                    receipt,
                    order: job.order,
                    label: job.label,
                    round_trip_ms: started.elapsed().as_secs_f64() * 1_000.0,
                },
            )));
        }
        if body.contains("expected player input sequence") {
            let view = match fetch_snapshot(client, config) {
                Ok(view) => view,
                Err(_) => {
                    thread::sleep(retry_delay);
                    retry_delay = retry_delay.saturating_mul(2).min(ONLINE_COMMAND_RETRY_MAX);
                    continue;
                }
            };
            let mut replacement = job.clone();
            replacement.request.sequence = view.view.next_sequence;
            replacement.request.input_sequence =
                Some(member_next_input_sequence(&view, &config.player_id)?);
            replacement.attempt = replacement
                .attempt
                .checked_add(1)
                .ok_or_else(|| "online command attempt sequence exhausted".to_string())?;
            replacement.request.command_id =
                native_command_id(&replacement.intent_id, replacement.attempt);
            replacement.request.expected_match_revision = view.view.match_revision;
            replacement.request.target_tick = view.view.authoritative_tick;
            replacement.request.client_observed_tick = Some(view.view.authoritative_tick);
            replacement.request.order.frame = u32::try_from(replacement.request.target_tick)
                .map_err(|_| "online authoritative tick exceeds frame range".to_string())?;
            replacement.order.frame = replacement.request.order.frame;
            request_journal_replacement(journal, &job, &replacement)?;
            job = replacement;
            retry_delay = ONLINE_COMMAND_RETRY_MIN;
            continue;
        }
        if command_response_is_recoverable(status, &body) {
            thread::sleep(retry_delay);
            retry_delay = retry_delay.saturating_mul(2).min(ONLINE_COMMAND_RETRY_MAX);
            continue;
        }
        if status.is_client_error() {
            request_journal_rejection(journal, &job, status.as_u16(), &body)?;
            return Ok(WorkerEvent::CommandFailed(format!(
                "online authority rejected queued command ({status}); the exact attempt was durably dead-lettered"
            )));
        }
        return Err(format!(
            "online authority returned an unexpected non-recoverable status ({status}); pending command retained"
        ));
    }
}

fn request_journal_replacement(
    journal: &SyncSender<JournalRequest>,
    expected: &CommandJob,
    replacement: &CommandJob,
) -> Result<(), String> {
    let (completion, result) = mpsc::sync_channel(0);
    journal
        .send(JournalRequest::Replace {
            expected: expected.clone(),
            replacement: Box::new(replacement.clone()),
            completion,
        })
        .map_err(|_| "online command journal owner stopped before replacement".to_string())?;
    result
        .recv()
        .map_err(|_| "online command journal owner dropped replacement result".to_string())?
}

fn request_journal_acknowledgement(
    journal: &SyncSender<JournalRequest>,
    expected: &CommandJob,
    receipt: &OnlineCommandReceipt,
) -> Result<(), String> {
    let (completion, result) = mpsc::sync_channel(0);
    journal
        .send(JournalRequest::Acknowledge {
            expected: expected.clone(),
            receipt: receipt.clone(),
            completion,
        })
        .map_err(|_| "online command journal owner stopped before acknowledgement".to_string())?;
    result
        .recv()
        .map_err(|_| "online command journal owner dropped acknowledgement result".to_string())?
}

fn request_journal_rejection(
    journal: &SyncSender<JournalRequest>,
    expected: &CommandJob,
    status: u16,
    reason: &str,
) -> Result<(), String> {
    let (completion, result) = mpsc::sync_channel(0);
    journal
        .send(JournalRequest::Reject {
            expected: expected.clone(),
            status,
            reason: reason.to_string(),
            completion,
        })
        .map_err(|_| "online command journal owner stopped before dead-letter".to_string())?;
    result
        .recv()
        .map_err(|_| "online command journal owner dropped dead-letter result".to_string())?
}

fn command_response_is_recoverable(status: reqwest::StatusCode, body: &str) -> bool {
    if status == reqwest::StatusCode::REQUEST_TIMEOUT
        || status == reqwest::StatusCode::TOO_MANY_REQUESTS
        || status.is_server_error()
    {
        return true;
    }
    serde_json::from_str::<trnm_online_protocol::OnlineAuthorityError>(body)
        .is_ok_and(|error| error.recoverable)
}

fn send_command_request(
    client: &reqwest::blocking::Client,
    config: &NetworkConfig,
    request: &OnlineCommandSubmitRequest,
) -> Result<(reqwest::StatusCode, String), String> {
    let response = send_with_retry(
        client
            .post(format!(
                "{}/v1/online/matches/{}/commands",
                config.base_url, config.match_id
            ))
            .header("x-trnm-player-session", &config.player_session)
            .json(request),
        "online command transport",
    )?;
    let status = response.status();
    let body = response
        .text()
        .map_err(|error| format!("online command response: {error}"))?;
    Ok((status, body))
}

fn fetch_snapshot(
    client: &reqwest::blocking::Client,
    config: &NetworkConfig,
) -> Result<OnlineSnapshotResponse, String> {
    let response = send_with_retry(
        client
            .post(format!(
                "{}/v1/online/matches/{}/snapshot",
                config.base_url, config.match_id
            ))
            .header("x-trnm-player-session", &config.player_session)
            .json(&OnlineMatchAccessRequest {
                protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
                build_id: ONLINE_AUTHORITY_BUILD.to_string(),
                player_id: config.player_id.clone(),
                account_id: config.account_id.clone(),
            }),
        "online snapshot transport",
    )?;
    let status = response.status();
    let body = response
        .text()
        .map_err(|error| format!("online snapshot response: {error}"))?;
    if !status.is_success() {
        return Err(format!("online snapshot rejected ({status}): {body}"));
    }
    serde_json::from_str(&body).map_err(|error| format!("online snapshot decode: {error}"))
}

fn build_http_client() -> Result<reqwest::blocking::Client, String> {
    reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(2))
        .timeout(Duration::from_secs(5))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|error| format!("online authority HTTP client: {error}"))
}

fn send_with_retry(
    request: reqwest::blocking::RequestBuilder,
    context: &str,
) -> Result<reqwest::blocking::Response, String> {
    let mut last_error = None;
    for attempt in 0..3 {
        let retry = request
            .try_clone()
            .ok_or_else(|| format!("{context}: request cannot be retried safely"))?;
        match retry.send() {
            Ok(response) => return Ok(response),
            Err(error) => {
                last_error = Some(error);
                if attempt < 2 {
                    thread::sleep(Duration::from_millis(100 * (attempt + 1) as u64));
                }
            }
        }
    }
    Err(format!(
        "{context}: {}",
        last_error
            .map(|error| error.to_string())
            .unwrap_or_else(|| "request failed".to_string())
    ))
}

fn decode_mission(
    snapshot: &OnlineSnapshotResponse,
    expected_match_id: &str,
) -> Result<MissionSimV1, String> {
    validate_stream_view(
        &snapshot.view,
        expected_match_id,
        &snapshot.view.snapshot_hash,
        snapshot.view.authoritative_tick,
    )?;
    if snapshot.snapshot.is_null() {
        return Err("online match has no running simulation snapshot".to_string());
    }
    let mission = serde_json::from_value::<MissionSimV1>(snapshot.snapshot.clone())
        .map_err(|error| format!("online mission snapshot decode: {error}"))?;
    if mission.tick != snapshot.view.authoritative_tick {
        return Err("online mission snapshot tick does not match its authority view".to_string());
    }
    if mission
        .snapshot_hash()
        .map_err(|error| format!("online mission snapshot hash: {error}"))?
        != snapshot.view.snapshot_hash
    {
        return Err("online mission snapshot hash does not match its authority view".to_string());
    }
    Ok(mission)
}

fn member_next_input_sequence(
    snapshot: &OnlineSnapshotResponse,
    player_id: &str,
) -> Result<u64, String> {
    snapshot
        .view
        .members
        .iter()
        .find(|member| member.player_id == player_id)
        .map(|member| member.next_input_sequence)
        .ok_or_else(|| "online snapshot is missing the submitting member".to_string())
}

fn native_command_id(intent_id: &str, attempt: u32) -> String {
    format!("native:{intent_id}:a{attempt}")
}

fn command_journal_path(config: &NetworkConfig) -> Result<PathBuf, String> {
    if let Some(path) =
        env::var_os("TRNM_ONLINE_COMMAND_JOURNAL_PATH").filter(|value| !value.is_empty())
    {
        return Ok(PathBuf::from(path));
    }
    let state_root =
        if let Some(path) = env::var_os("XDG_STATE_HOME").filter(|value| !value.is_empty()) {
            PathBuf::from(path)
        } else {
            let home = env::var_os("HOME")
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    "HOME or XDG_STATE_HOME is required for the online command journal".to_string()
                })?;
            PathBuf::from(home).join(".local/state")
        };
    Ok(state_root
        .join("trillionnium/online")
        .join(format!("{}-{}.json", config.match_id, config.account_id)))
}

fn compare_snapshot_freshness(
    candidate: &OnlineSnapshotResponse,
    current: &OnlineSnapshotResponse,
) -> Result<bool, String> {
    if candidate.view.match_revision < current.view.match_revision
        || candidate.view.authoritative_tick < current.view.authoritative_tick
        || candidate.view.next_sequence < current.view.next_sequence
    {
        return Ok(false);
    }
    if candidate.view.match_revision == current.view.match_revision
        && candidate.view.authoritative_tick == current.view.authoritative_tick
        && candidate.view.next_sequence == current.view.next_sequence
        && candidate.view.snapshot_hash != current.view.snapshot_hash
    {
        return Err("online snapshot changed hash without advancing revision or tick".to_string());
    }
    Ok(true)
}

fn publish_latest_snapshot(
    slot: &Arc<Mutex<Option<LatestWorkerSnapshot>>>,
    candidate: LatestWorkerSnapshot,
) -> Result<bool, String> {
    let mut latest = slot
        .lock()
        .map_err(|_| "online latest-snapshot slot is poisoned".to_string())?;
    if let Some(current) = latest.as_ref() {
        if !compare_snapshot_freshness(&candidate.view, &current.view)? {
            return Ok(false);
        }
    }
    *latest = Some(candidate);
    Ok(true)
}

fn reserve_pending_slot(pending: &AtomicUsize) -> Result<(), String> {
    pending
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
            (current < MAX_PENDING_EXACT_ATTEMPTS).then_some(current + 1)
        })
        .map(|_| ())
        .map_err(|_| {
            format!(
                "online command queue is full ({MAX_PENDING_EXACT_ATTEMPTS}); input was not accepted"
            )
        })
}

fn release_pending_slot(pending: &AtomicUsize) {
    let _ = pending.fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
        Some(current.saturating_sub(1))
    });
}

fn validate_authority_base_url(value: &str) -> Result<(), String> {
    let url = reqwest::Url::parse(value.trim_end_matches('/'))
        .map_err(|error| format!("online authority URL: {error}"))?;
    if url.username() != "" || url.password().is_some() {
        return Err("online authority URL must not contain credentials".to_string());
    }
    match url.scheme() {
        "https" => Ok(()),
        "http" if authority_host_is_loopback(&url) => Ok(()),
        "http" => Err(
            "non-loopback online authority endpoints require HTTPS to protect the player session"
                .to_string(),
        ),
        scheme => Err(format!("unsupported online authority URL scheme: {scheme}")),
    }
}

fn authority_host_is_loopback(url: &reqwest::Url) -> bool {
    url.host_str().is_some_and(|host| {
        let host = host
            .strip_prefix('[')
            .and_then(|value| value.strip_suffix(']'))
            .unwrap_or(host);
        host.eq_ignore_ascii_case("localhost")
            || host
                .parse::<std::net::IpAddr>()
                .is_ok_and(|address| address.is_loopback())
    })
}

fn required(name: &str) -> Result<String, String> {
    env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("{name} is required in online authority mode"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::BTreeMap,
        fs,
        io::{Read, Write},
        net::TcpListener,
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering as AtomicOrdering},
        time::Instant,
    };
    use trnm_rts_protocol::{RtsOrderKind, RtsTile};

    static TEST_JOURNAL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn snapshot_transport_never_blocks_the_render_caller() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 4096];
            let _ = stream.read(&mut request).unwrap();
            thread::sleep(Duration::from_millis(300));
            stream
                .write_all(
                    b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 4\r\nConnection: close\r\n\r\nslow",
                )
                .unwrap();
        });
        let config = NetworkConfig {
            base_url: format!("http://{address}"),
            match_id: "00000000-0000-0000-0000-000000000001".to_string(),
            player_id: "player".to_string(),
            account_id: "00000000-0000-0000-0000-000000000002".to_string(),
            player_session: "session".to_string(),
        };
        let (request_tx, request_rx) = mpsc::sync_channel(1);
        let (event_tx, event_rx) = mpsc::sync_channel(4);
        spawn_snapshot_worker(
            config,
            build_http_client().unwrap(),
            request_rx,
            event_tx,
            Arc::new(Mutex::new(None)),
        )
        .unwrap();

        let started = Instant::now();
        request_tx.try_send(()).unwrap();
        assert!(started.elapsed() < Duration::from_millis(50));
        assert!(matches!(
            event_rx.recv_timeout(Duration::from_secs(2)).unwrap(),
            WorkerEvent::RefreshFailed(_)
        ));
        server.join().unwrap();
    }

    #[test]
    fn stream_delta_rejects_a_bad_snapshot_base() {
        let mut cursor = test_stream_cursor();
        let error = apply_stream_message(
            &mut cursor,
            TEST_MATCH_ID,
            test_delta_message(8, 7, "b".repeat(64)),
        )
        .err()
        .unwrap();
        assert!(error.contains("base cursor mismatch"), "{error}");
        assert_eq!(cursor.state_sequence, Some(7));
        assert_eq!(cursor.snapshot_hash, "a".repeat(64));
    }

    #[test]
    fn stream_delta_rejects_out_of_order_state_sequence() {
        let mut cursor = test_stream_cursor();
        let error = apply_stream_message(
            &mut cursor,
            TEST_MATCH_ID,
            test_delta_message(7, 7, "a".repeat(64)),
        )
        .err()
        .unwrap();
        assert!(error.contains("out of order"), "{error}");
        assert_eq!(cursor.state_sequence, Some(7));
    }

    #[test]
    fn stream_url_converts_scheme_and_never_contains_the_session() {
        let mut config = NetworkConfig {
            base_url: "http://127.0.0.1:9090/authority".to_string(),
            match_id: TEST_MATCH_ID.to_string(),
            player_id: "player one".to_string(),
            account_id: "00000000-0000-0000-0000-000000000002".to_string(),
            player_session: "session-secret-must-stay-in-header".to_string(),
        };
        let cursor = test_stream_cursor();
        let url = build_stream_url(&config, &cursor).unwrap();
        assert_eq!(url.scheme(), "ws");
        assert_eq!(
            url.path(),
            format!("/authority/v1/online/matches/{TEST_MATCH_ID}/stream")
        );
        assert!(!url.as_str().contains(&config.player_session));
        let query = url
            .query_pairs()
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect::<BTreeMap<_, _>>();
        assert_eq!(
            query.get("protocol_version").map(String::as_str),
            Some(ONLINE_STREAM_PROTOCOL)
        );
        assert_eq!(
            query.get("build_id").map(String::as_str),
            Some(ONLINE_AUTHORITY_BUILD)
        );
        assert_eq!(
            query.get("player_id").map(String::as_str),
            Some("player one")
        );
        assert_eq!(
            query.get("next_receipt_sequence").map(String::as_str),
            Some("11")
        );
        assert_eq!(
            query.get("last_snapshot_hash").map(String::as_str),
            Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
        );
        assert!(!query.contains_key("player_session"));

        config.base_url = "https://authority.example.test".to_string();
        assert_eq!(build_stream_url(&config, &cursor).unwrap().scheme(), "wss");
    }

    #[test]
    fn authority_url_requires_https_except_for_literal_loopback() {
        assert!(validate_authority_base_url("http://127.0.0.1:8080").is_ok());
        assert!(validate_authority_base_url("http://[::1]:8080").is_ok());
        assert!(validate_authority_base_url("http://localhost:8080").is_ok());
        assert!(validate_authority_base_url("https://authority.example.test").is_ok());
        assert!(validate_authority_base_url("http://192.0.2.10:8080").is_err());
        assert!(validate_authority_base_url("http://localhost.example.test").is_err());
        assert!(validate_authority_base_url("https://user@authority.example.test").is_err());
    }

    #[test]
    fn http_client_does_not_follow_redirects_with_the_session_header() {
        let origin = TcpListener::bind("127.0.0.1:0").unwrap();
        let sink = TcpListener::bind("127.0.0.1:0").unwrap();
        sink.set_nonblocking(true).unwrap();
        let sink_address = sink.local_addr().unwrap();
        let origin_address = origin.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = origin.accept().unwrap();
            let mut request = [0_u8; 4096];
            let _ = stream.read(&mut request).unwrap();
            write!(
                stream,
                "HTTP/1.1 302 Found\r\nLocation: http://{sink_address}/capture\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            )
            .unwrap();
        });

        let response = build_http_client()
            .unwrap()
            .post(format!("http://{origin_address}/command"))
            .header("x-trnm-player-session", "must-not-cross-origin")
            .send()
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::FOUND);
        server.join().unwrap();
        assert!(matches!(
            sink.accept(),
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock
        ));
    }

    #[test]
    fn latest_snapshot_freshness_rejects_each_regressing_cursor() {
        let current = test_snapshot(5, 100, 12, "a".repeat(64));
        assert!(
            !compare_snapshot_freshness(&test_snapshot(4, 101, 13, "b".repeat(64)), &current,)
                .unwrap()
        );
        assert!(
            !compare_snapshot_freshness(&test_snapshot(6, 99, 13, "b".repeat(64)), &current,)
                .unwrap()
        );
        assert!(
            !compare_snapshot_freshness(&test_snapshot(6, 101, 11, "b".repeat(64)), &current,)
                .unwrap()
        );
        assert!(
            compare_snapshot_freshness(&test_snapshot(6, 101, 13, "b".repeat(64)), &current,)
                .unwrap()
        );
        let error =
            compare_snapshot_freshness(&test_snapshot(5, 100, 12, "b".repeat(64)), &current)
                .unwrap_err();
        assert!(error.contains("changed hash"), "{error}");
    }

    #[test]
    fn actor_generation_change_cannot_reset_authority_cursors() {
        let cursor = test_stream_cursor();
        let next_generation = "00000000-0000-0000-0000-000000000004";

        let revision_error = validate_full_snapshot_progress(
            &cursor,
            next_generation,
            0,
            11,
            &test_online_view(2, 43, 11, "b".repeat(64)),
        )
        .unwrap_err();
        assert!(
            revision_error.contains("authority cursor"),
            "{revision_error}"
        );
        let tick_error = validate_full_snapshot_progress(
            &cursor,
            next_generation,
            0,
            11,
            &test_online_view(4, 41, 11, "b".repeat(64)),
        )
        .unwrap_err();
        assert!(tick_error.contains("authority cursor"), "{tick_error}");
        let receipt_error = validate_full_snapshot_progress(
            &cursor,
            next_generation,
            0,
            10,
            &test_online_view(4, 43, 10, "b".repeat(64)),
        )
        .unwrap_err();
        assert!(receipt_error.contains("receipt cursor"), "{receipt_error}");

        let generation_change = validate_full_snapshot_progress(
            &cursor,
            next_generation,
            0,
            11,
            &test_online_view(3, 42, 11, "a".repeat(64)),
        )
        .unwrap();
        assert!(generation_change.is_some());
    }

    #[test]
    fn sixteen_rapid_intents_receive_contiguous_durable_input_sequences() {
        let path = test_journal_path("rapid-intents");
        let scope = test_journal_scope();
        let mut journal = OnlineCommandJournal::load_or_new(&path, scope.clone()).unwrap();
        let mut allocated = Vec::new();
        for sequence in 0..MAX_PENDING_EXACT_ATTEMPTS as u64 {
            let before = journal.clone();
            let job = enqueue_command_intent(&mut journal, test_command_intent(sequence)).unwrap();
            allocated.push(job.request.input_sequence.unwrap());
            store_journal_mutation(&mut journal, before, "test durable enqueue").unwrap();
        }
        assert_eq!(
            allocated,
            (0..MAX_PENDING_EXACT_ATTEMPTS as u64).collect::<Vec<_>>()
        );
        assert_eq!(
            journal.next_input_sequence,
            MAX_PENDING_EXACT_ATTEMPTS as u64
        );
        drop(journal);

        let loaded = OnlineCommandJournal::load_or_new(&path, scope).unwrap();
        assert_eq!(
            loaded.pending_exact_attempts.len(),
            MAX_PENDING_EXACT_ATTEMPTS
        );
        assert_eq!(
            loaded
                .pending_exact_attempts
                .iter()
                .map(|pending| pending.request.input_sequence.unwrap())
                .collect::<Vec<_>>(),
            (0..MAX_PENDING_EXACT_ATTEMPTS as u64).collect::<Vec<_>>()
        );
        drop(loaded);
        cleanup_test_journal(&path);
    }

    #[test]
    fn journal_store_failure_rolls_back_only_before_install() {
        let path = test_journal_path("store-error-boundary");
        let mut journal = OnlineCommandJournal::load_or_new(&path, test_journal_scope()).unwrap();
        let before = journal.clone();
        enqueue_command_intent(&mut journal, test_command_intent(0)).unwrap();
        let error = resolve_journal_store_result(
            &mut journal,
            before,
            "injected enqueue",
            Err(JournalStoreError::BeforeInstall("injected".to_string())),
        )
        .unwrap_err();
        assert!(error.contains("prior state restored"), "{error}");
        assert_eq!(journal.next_input_sequence, 0);
        assert!(journal.pending_exact_attempts.is_empty());

        let before = journal.clone();
        enqueue_command_intent(&mut journal, test_command_intent(1)).unwrap();
        let error = resolve_journal_store_result(
            &mut journal,
            before,
            "injected enqueue",
            Err(JournalStoreError::DurabilityUncertain(
                "injected".to_string(),
            )),
        )
        .unwrap_err();
        assert!(error.contains("forward state retained"), "{error}");
        assert_eq!(journal.next_input_sequence, 1);
        assert_eq!(journal.pending_exact_attempts.len(), 1);
        drop(journal);
        cleanup_test_journal(&path);
    }

    #[test]
    fn only_retryable_authority_responses_keep_the_fifo_head_pending() {
        assert!(command_response_is_recoverable(
            reqwest::StatusCode::REQUEST_TIMEOUT,
            "timeout"
        ));
        assert!(command_response_is_recoverable(
            reqwest::StatusCode::TOO_MANY_REQUESTS,
            "rate limited"
        ));
        assert!(command_response_is_recoverable(
            reqwest::StatusCode::SERVICE_UNAVAILABLE,
            "offline"
        ));
        assert!(command_response_is_recoverable(
            reqwest::StatusCode::UNPROCESSABLE_ENTITY,
            r#"{"error":"retry","recoverable":true,"authoritative_revision":null}"#,
        ));
        assert!(!command_response_is_recoverable(
            reqwest::StatusCode::UNPROCESSABLE_ENTITY,
            r#"{"error":"bad","recoverable":false,"authoritative_revision":null}"#,
        ));
    }

    const TEST_MATCH_ID: &str = "00000000-0000-0000-0000-000000000001";
    const TEST_ACCOUNT_ID: &str = "00000000-0000-0000-0000-000000000002";
    const TEST_GENERATION: &str = "00000000-0000-0000-0000-000000000003";

    fn test_stream_cursor() -> StreamCursor {
        StreamCursor {
            actor_generation: Some(TEST_GENERATION.to_string()),
            state_sequence: Some(7),
            match_revision: 3,
            next_receipt_sequence: 11,
            snapshot_hash: "a".repeat(64),
            authoritative_tick: 42,
            snapshot: serde_json::json!({}),
        }
    }

    fn test_online_view(
        match_revision: u64,
        authoritative_tick: u64,
        next_sequence: u64,
        snapshot_hash: String,
    ) -> trnm_online_protocol::OnlineMatchView {
        trnm_online_protocol::OnlineMatchView {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            match_id: TEST_MATCH_ID.to_string(),
            join_code: "TEST".to_string(),
            phase: trnm_online_protocol::OnlineMatchPhase::Running,
            match_revision,
            authoritative_tick,
            next_sequence,
            map_id: "first_contact".to_string(),
            match_mode: "coop".to_string(),
            rules_version: "test".to_string(),
            seed_hash: "d".repeat(64),
            snapshot_hash,
            members: Vec::new(),
            result_hash: None,
            settlement_state: "pending".to_string(),
        }
    }

    fn test_snapshot(
        match_revision: u64,
        authoritative_tick: u64,
        next_sequence: u64,
        snapshot_hash: String,
    ) -> OnlineSnapshotResponse {
        OnlineSnapshotResponse {
            view: test_online_view(
                match_revision,
                authoritative_tick,
                next_sequence,
                snapshot_hash,
            ),
            snapshot: serde_json::Value::Null,
        }
    }

    fn test_journal_path(label: &str) -> PathBuf {
        let sequence = TEST_JOURNAL_SEQUENCE.fetch_add(1, AtomicOrdering::Relaxed);
        std::env::temp_dir()
            .join(format!(
                "trnm-online-authority-{}-{label}-{sequence}",
                std::process::id()
            ))
            .join("journal.json")
    }

    fn test_journal_scope() -> OnlineCommandJournalScope {
        OnlineCommandJournalScope::new(TEST_MATCH_ID, "player-one", TEST_ACCOUNT_ID)
    }

    fn test_command_intent(sequence: u64) -> CommandIntent {
        let mut order = RtsFrameOrder::new(
            42,
            "player-one",
            vec!["host:hero".to_string()],
            RtsOrderKind::Move,
            RtsOrderSource::LocalInput,
        );
        order.target_tile = Some(RtsTile { x: 3, y: 4 });
        CommandIntent {
            order,
            label: "Move".to_string(),
            intent_id: format!("intent-{sequence}"),
            legacy_sequence: 0,
            observed_next_input_sequence: 0,
            expected_match_revision: 3,
            client_observed_tick: 42,
        }
    }

    fn cleanup_test_journal(path: &Path) {
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }

    fn test_delta_message(
        state_sequence: u64,
        base_state_sequence: u64,
        base_snapshot_hash: String,
    ) -> OnlineStreamServerMessage {
        let next_hash = "c".repeat(64);
        OnlineStreamServerMessage::SnapshotDelta {
            actor_generation: TEST_GENERATION.to_string(),
            state_sequence,
            base_state_sequence,
            view: test_online_view(3, 43, 11, next_hash.clone()),
            delta: trnm_online_protocol::OnlineSnapshotDelta {
                base_snapshot_hash,
                snapshot_hash: next_hash,
                base_tick: 42,
                authoritative_tick: 43,
                changed_fields: BTreeMap::new(),
                removed_fields: Vec::new(),
            },
        }
    }
}
