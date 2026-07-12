use reqwest::blocking::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::{
    process::Command,
    thread,
    time::{Duration, Instant},
};
use trnm_online_protocol::{
    OnlineCampaignConnectRequest, OnlineCampaignView, OnlineCommandReceipt,
    OnlineCommandSubmitRequest, OnlineMatchAccessRequest, OnlineMatchCreateRequest,
    OnlineMatchJoinRequest, OnlineMatchPhase, OnlineMatchStartRequest, OnlineMatchView,
    OnlineSnapshotResponse, ONLINE_AUTHORITY_BUILD, ONLINE_AUTHORITY_PROTOCOL,
};
use trnm_rts_protocol::{RtsFrameOrder, RtsOrderKind, RtsOrderSource, RtsTile};

#[derive(Clone)]
struct Identity {
    player_id: String,
    account_id: String,
    session: String,
}

struct OnlineClient {
    base_url: String,
    http: Client,
}

struct CommandSpec {
    command_id: String,
    kind: RtsOrderKind,
    subjects: Vec<String>,
    target: RtsTile,
    queued: bool,
}

impl OnlineClient {
    fn new(base_url: String) -> Result<Self, String> {
        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: Client::builder()
                .connect_timeout(Duration::from_secs(2))
                .timeout(Duration::from_secs(10))
                .build()
                .map_err(|error| error.to_string())?,
        })
    }

    fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        identity: &Identity,
        path: &str,
        body: &T,
    ) -> Result<R, String> {
        let response = self
            .http
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

    fn post_status<T: Serialize>(
        &self,
        identity: &Identity,
        path: &str,
        body: &T,
    ) -> Result<(u16, Value), String> {
        let response = self
            .http
            .post(format!("{}{}", self.base_url, path))
            .header("x-trnm-player-session", &identity.session)
            .json(body)
            .send()
            .map_err(|error| error.to_string())?;
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
        self.post(
            identity,
            &format!("/v1/online/matches/{match_id}/snapshot"),
            &OnlineMatchAccessRequest {
                protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
                build_id: ONLINE_AUTHORITY_BUILD.to_string(),
                player_id: identity.player_id.clone(),
                account_id: identity.account_id.clone(),
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
        let last_frame = snapshot
            .snapshot
            .get("last_order_frame")
            .and_then(Value::as_u64)
            .unwrap_or_default();
        let target_tick = snapshot
            .view
            .authoritative_tick
            .saturating_add(40)
            .max(last_frame.saturating_add(1));
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
        let request = OnlineCommandSubmitRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: identity.player_id.clone(),
            account_id: identity.account_id.clone(),
            command_id: spec.command_id,
            sequence: snapshot.view.next_sequence,
            expected_match_revision: snapshot.view.match_revision,
            target_tick,
            order,
        };
        let receipt = self.post(
            identity,
            &format!("/v1/online/matches/{match_id}/commands"),
            &request,
        )?;
        Ok((receipt, request))
    }
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
    let client = OnlineClient::new(base_url.clone())?;
    let run_id = format!("online-e2e-{}", chrono::Utc::now().timestamp_millis());

    let campaign: OnlineCampaignView = client.post(
        &host,
        "/v1/online/campaigns/connect",
        &OnlineCampaignConnectRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: host.player_id.clone(),
            account_id: host.account_id.clone(),
            slot_key: run_id.clone(),
        },
    )?;
    let created: OnlineMatchView = client.post(
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
    let _: OnlineMatchView = client.post(
        &guest,
        "/v1/online/matches/join",
        &OnlineMatchJoinRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: guest.player_id.clone(),
            account_id: guest.account_id.clone(),
            join_code: created.join_code.clone(),
        },
    )?;
    let started: OnlineMatchView = client.post(
        &host,
        &format!("/v1/online/matches/{}/start", created.match_id),
        &OnlineMatchStartRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: host.player_id.clone(),
            account_id: host.account_id.clone(),
            expected_match_revision: 0,
        },
    )?;
    if started.phase != OnlineMatchPhase::Running || started.members.len() != 2 {
        return Err("two-client match did not enter running phase".to_string());
    }
    let initial = client.snapshot(&host, &created.match_id)?;
    let approach = point(&initial, "approach_point")?;
    let objective = point(&initial, "objective")?;
    let host_units = controlled(&initial, &host)?;

    let (first, first_request) = client.submit(
        &host,
        &created.match_id,
        &initial,
        CommandSpec {
            command_id: format!("{run_id}-host-move"),
            kind: RtsOrderKind::Move,
            subjects: host_units.clone(),
            target: approach,
            queued: false,
        },
    )?;
    let duplicate: OnlineCommandReceipt = client.post(
        &host,
        &format!("/v1/online/matches/{}/commands", created.match_id),
        &first_request,
    )?;
    if !duplicate.duplicate || duplicate.sequence != first.sequence {
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
    skipped.sequence = skipped.sequence.saturating_add(2);
    skipped.expected_match_revision = first.match_revision;
    let (status, _) = client.post_status(
        &host,
        &format!("/v1/online/matches/{}/commands", created.match_id),
        &skipped,
    )?;
    if status != 409 {
        return Err(format!(
            "sequence skip returned HTTP {status}, expected 409"
        ));
    }
    let after_first = client.snapshot(&guest, &created.match_id)?;
    let mut theft = skipped;
    theft.player_id = guest.player_id.clone();
    theft.account_id = guest.account_id.clone();
    theft.command_id = format!("{run_id}-control-theft");
    theft.sequence = after_first.view.next_sequence;
    theft.expected_match_revision = after_first.view.match_revision;
    theft.target_tick = after_first.view.authoritative_tick.saturating_add(40);
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
    restart_server(&base_url)?;
    let after_restart = client.snapshot(&guest, &created.match_id)?;
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
        Duration::from_secs(45),
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
        Duration::from_secs(45),
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
            Duration::from_secs(45),
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
            Duration::from_secs(45),
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
            Duration::from_secs(45),
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
                            && ox.is_some_and(|x| {
                                (unit["position"]["x"].as_i64().unwrap_or_default() - x).abs() <= 2
                            })
                            && oy.is_some_and(|y| {
                                (unit["position"]["y"].as_i64().unwrap_or_default() - y).abs() <= 2
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

    let complete = wait_for(
        &client,
        &host,
        &created.match_id,
        Duration::from_secs(60),
        |snapshot| {
            snapshot.view.phase == OnlineMatchPhase::Complete
                && snapshot.view.settlement_state == "settled"
        },
    )?;
    if complete.snapshot["outcome"] != "victory" {
        return Err(format!(
            "authoritative battle completed without victory: {}",
            complete.snapshot["outcome"]
        ));
    }
    Ok(json!({
        "status": "passed",
        "run_id": run_id,
        "campaign_id": campaign.campaign_id,
        "match_id": created.match_id,
        "members": complete.view.members,
        "authoritative_tick": complete.view.authoritative_tick,
        "next_sequence": complete.view.next_sequence,
        "seed_hash": complete.view.seed_hash,
        "snapshot_hash": complete.view.snapshot_hash,
        "result_hash": complete.view.result_hash,
        "settlement_state": complete.view.settlement_state,
        "duplicate_command_exactly_once": true,
        "tampered_duplicate_rejected": true,
        "sequence_regression_rejected": true,
        "cross_member_control_rejected": true,
        "old_build_rejected": true,
        "restart_recovery": true,
        "guest_progression": false,
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
