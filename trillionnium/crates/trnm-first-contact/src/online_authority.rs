use bevy::prelude::Resource;
use std::{
    collections::BTreeSet,
    env,
    sync::{
        mpsc::{self, Receiver, SyncSender, TrySendError},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
use trnm_online_protocol::{
    OnlineCommandReceipt, OnlineCommandSubmitRequest, OnlineMatchAccessRequest,
    OnlineSnapshotResponse, ONLINE_AUTHORITY_BUILD, ONLINE_AUTHORITY_PROTOCOL,
};
use trnm_rts_protocol::{RtsFrameOrder, RtsOrderSource};
use trnm_rts_sim::MissionSimV1;

#[derive(Clone)]
struct NetworkConfig {
    base_url: String,
    match_id: String,
    player_id: String,
    account_id: String,
    player_session: String,
}

struct CommandJob {
    request: OnlineCommandSubmitRequest,
    order: RtsFrameOrder,
    label: String,
}

enum WorkerEvent {
    Snapshot {
        view: Box<OnlineSnapshotResponse>,
        mission: Box<MissionSimV1>,
    },
    CommandAccepted(Box<WorkerCommandAccepted>),
    RefreshFailed(String),
    CommandFailed(String),
}

struct WorkerCommandAccepted {
    receipt: OnlineCommandReceipt,
    view: OnlineSnapshotResponse,
    mission: Box<MissionSimV1>,
    order: RtsFrameOrder,
    label: String,
}

pub(super) enum OnlineClientEvent {
    Snapshot(Box<MissionSimV1>),
    CommandAccepted(Box<OnlineCommandAcceptedEvent>),
    RefreshFailed(String),
    CommandFailed(String),
}

pub(super) struct OnlineCommandAcceptedEvent {
    pub receipt: OnlineCommandReceipt,
    pub mission: Option<Box<MissionSimV1>>,
    pub order: RtsFrameOrder,
    pub label: String,
}

#[derive(Clone, Resource)]
pub(super) struct OnlineAuthorityClient {
    match_id: String,
    player_id: String,
    account_id: String,
    controlled_unit_ids: BTreeSet<String>,
    ranked_pvp_guest: bool,
    view: OnlineSnapshotResponse,
    snapshot_tx: SyncSender<()>,
    command_tx: SyncSender<CommandJob>,
    events: Arc<Mutex<Receiver<WorkerEvent>>>,
    snapshot_in_flight: bool,
    command_in_flight: bool,
    pub poll_accumulator: f32,
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
        let config = NetworkConfig {
            base_url: base_url.trim_end_matches('/').to_string(),
            match_id: required("TRNM_ONLINE_MATCH_ID")?,
            player_id: required("TRNM_CEX_ACTOR_ID")?,
            account_id: required("TRNM_CEX_ACCOUNT_ID")?,
            player_session: required("TRNM_CEX_PLAYER_SESSION")?,
        };
        // Initial attachment happens before the render loop exists. All traffic
        // after this point is owned by the two bounded background workers.
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
        let mission = decode_mission(&view)?;

        let (snapshot_tx, snapshot_rx) = mpsc::sync_channel(1);
        let (command_tx, command_rx) = mpsc::sync_channel(1);
        let (event_tx, event_rx) = mpsc::channel();
        spawn_snapshot_worker(
            config.clone(),
            build_http_client()?,
            snapshot_rx,
            event_tx.clone(),
        )?;
        spawn_command_worker(config.clone(), build_http_client()?, command_rx, event_tx)?;

        Ok(Some((
            Self {
                match_id: config.match_id,
                player_id: config.player_id,
                account_id: config.account_id,
                controlled_unit_ids,
                ranked_pvp_guest,
                view,
                snapshot_tx,
                command_tx,
                events: Arc::new(Mutex::new(event_rx)),
                snapshot_in_flight: false,
                command_in_flight: false,
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

    pub fn submit(&mut self, mut order: RtsFrameOrder, label: String) -> Result<(), String> {
        if self.command_in_flight {
            return Err("previous online command is still awaiting ACK".to_string());
        }
        order
            .subject_actor_ids
            .retain(|unit_id| self.controlled_unit_ids.contains(unit_id));
        if order.subject_actor_ids.is_empty() {
            return Err("selection contains no units assigned to this online member".to_string());
        }
        let last_frame = last_order_frame(&self.view);
        let target_tick = self
            .view
            .view
            .authoritative_tick
            .saturating_add(40)
            .max(last_frame.saturating_add(1));
        order.frame = u32::try_from(target_tick)
            .map_err(|_| "online authoritative tick exceeds frame range".to_string())?;
        order.player_id = self.player_id.clone();
        order.source = RtsOrderSource::LocalInput;
        let request = OnlineCommandSubmitRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: self.player_id.clone(),
            account_id: self.account_id.clone(),
            command_id: format!(
                "native:{}:{}:{}",
                self.match_id, self.player_id, self.view.view.next_sequence
            ),
            sequence: self.view.view.next_sequence,
            expected_match_revision: self.view.view.match_revision,
            target_tick,
            order: order.clone(),
        };
        match self.command_tx.try_send(CommandJob {
            request,
            order,
            label,
        }) {
            Ok(()) => {
                self.command_in_flight = true;
                Ok(())
            }
            Err(TrySendError::Full(_)) => {
                Err("online command queue is full; input was not accepted".to_string())
            }
            Err(TrySendError::Disconnected(_)) => {
                Err("online command worker disconnected".to_string())
            }
        }
    }

    pub fn drain_events(&mut self) -> Vec<OnlineClientEvent> {
        let mut worker_events = Vec::new();
        if let Ok(events) = self.events.lock() {
            while let Ok(event) = events.try_recv() {
                worker_events.push(event);
            }
        }
        let mut client_events = Vec::with_capacity(worker_events.len());
        for event in worker_events {
            match event {
                WorkerEvent::Snapshot { view, mission } => {
                    self.snapshot_in_flight = false;
                    if view_is_at_least_as_fresh(&view, &self.view) {
                        self.view = *view;
                        client_events.push(OnlineClientEvent::Snapshot(mission));
                    }
                }
                WorkerEvent::CommandAccepted(event) => {
                    self.command_in_flight = false;
                    let mission = if view_is_at_least_as_fresh(&event.view, &self.view) {
                        self.view = event.view;
                        Some(event.mission)
                    } else {
                        None
                    };
                    client_events.push(OnlineClientEvent::CommandAccepted(Box::new(
                        OnlineCommandAcceptedEvent {
                            receipt: event.receipt,
                            mission,
                            order: event.order,
                            label: event.label,
                        },
                    )));
                }
                WorkerEvent::RefreshFailed(error) => {
                    self.snapshot_in_flight = false;
                    client_events.push(OnlineClientEvent::RefreshFailed(error));
                }
                WorkerEvent::CommandFailed(error) => {
                    self.command_in_flight = false;
                    client_events.push(OnlineClientEvent::CommandFailed(error));
                }
            }
        }
        client_events
    }
}

fn spawn_snapshot_worker(
    config: NetworkConfig,
    client: reqwest::blocking::Client,
    requests: Receiver<()>,
    events: mpsc::Sender<WorkerEvent>,
) -> Result<(), String> {
    thread::Builder::new()
        .name("trnm-online-snapshot".to_string())
        .spawn(move || {
            while requests.recv().is_ok() {
                let event = match fetch_snapshot(&client, &config)
                    .and_then(|view| decode_mission(&view).map(|mission| (view, mission)))
                {
                    Ok((view, mission)) => WorkerEvent::Snapshot {
                        view: Box::new(view),
                        mission: Box::new(mission),
                    },
                    Err(error) => WorkerEvent::RefreshFailed(error),
                };
                if events.send(event).is_err() {
                    break;
                }
            }
        })
        .map(|_| ())
        .map_err(|error| format!("spawn online snapshot worker: {error}"))
}

fn spawn_command_worker(
    config: NetworkConfig,
    client: reqwest::blocking::Client,
    requests: Receiver<CommandJob>,
    events: mpsc::Sender<WorkerEvent>,
) -> Result<(), String> {
    thread::Builder::new()
        .name("trnm-online-command".to_string())
        .spawn(move || {
            while let Ok(job) = requests.recv() {
                let event = match submit_command(&client, &config, job) {
                    Ok(event) => event,
                    Err(error) => WorkerEvent::CommandFailed(error),
                };
                if events.send(event).is_err() {
                    break;
                }
            }
        })
        .map(|_| ())
        .map_err(|error| format!("spawn online command worker: {error}"))
}

fn submit_command(
    client: &reqwest::blocking::Client,
    config: &NetworkConfig,
    mut job: CommandJob,
) -> Result<WorkerEvent, String> {
    let (mut status, mut body) = send_command_request(client, config, &job.request)?;
    if !status.is_success() && body.contains("target_tick is outside the authoritative window") {
        let view = fetch_snapshot(client, config)?;
        job.request.sequence = view.view.next_sequence;
        job.request.expected_match_revision = view.view.match_revision;
        job.request.target_tick = view
            .view
            .authoritative_tick
            .saturating_add(80)
            .max(last_order_frame(&view).saturating_add(1));
        job.request.order.frame = u32::try_from(job.request.target_tick)
            .map_err(|_| "online authoritative tick exceeds frame range".to_string())?;
        job.order.frame = job.request.order.frame;
        (status, body) = send_command_request(client, config, &job.request)?;
    }
    if !status.is_success() {
        return Err(format!(
            "online authority rejected command ({status}): {body}"
        ));
    }
    let receipt = serde_json::from_str::<OnlineCommandReceipt>(&body)
        .map_err(|error| format!("online command receipt: {error}"))?;
    let view = fetch_snapshot(client, config)?;
    let mission = decode_mission(&view)?;
    Ok(WorkerEvent::CommandAccepted(Box::new(
        WorkerCommandAccepted {
            receipt,
            view,
            mission: Box::new(mission),
            order: job.order,
            label: job.label,
        },
    )))
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

fn decode_mission(snapshot: &OnlineSnapshotResponse) -> Result<MissionSimV1, String> {
    if snapshot.snapshot.is_null() {
        return Err("online match has no running simulation snapshot".to_string());
    }
    serde_json::from_value(snapshot.snapshot.clone())
        .map_err(|error| format!("online mission snapshot decode: {error}"))
}

fn last_order_frame(snapshot: &OnlineSnapshotResponse) -> u64 {
    snapshot
        .snapshot
        .get("last_order_frame")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or_default()
}

fn view_is_at_least_as_fresh(
    candidate: &OnlineSnapshotResponse,
    current: &OnlineSnapshotResponse,
) -> bool {
    (
        candidate.view.match_revision,
        candidate.view.authoritative_tick,
    ) >= (current.view.match_revision, current.view.authoritative_tick)
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
        io::{Read, Write},
        net::TcpListener,
        time::Instant,
    };

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
        let (event_tx, event_rx) = mpsc::channel();
        spawn_snapshot_worker(config, build_http_client().unwrap(), request_rx, event_tx).unwrap();

        let started = Instant::now();
        request_tx.try_send(()).unwrap();
        assert!(started.elapsed() < Duration::from_millis(50));
        assert!(matches!(
            event_rx.recv_timeout(Duration::from_secs(2)).unwrap(),
            WorkerEvent::RefreshFailed(_)
        ));
        server.join().unwrap();
    }
}
