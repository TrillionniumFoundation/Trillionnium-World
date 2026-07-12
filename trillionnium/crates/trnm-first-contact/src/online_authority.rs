use bevy::prelude::Resource;
use std::{collections::BTreeSet, env, thread, time::Duration};
use trnm_online_protocol::{
    OnlineCommandReceipt, OnlineCommandSubmitRequest, OnlineMatchAccessRequest,
    OnlineSnapshotResponse, ONLINE_AUTHORITY_BUILD, ONLINE_AUTHORITY_PROTOCOL,
};
use trnm_rts_protocol::{RtsFrameOrder, RtsOrderSource};
use trnm_rts_sim::MissionSimV1;

#[derive(Clone, Resource)]
pub(super) struct OnlineAuthorityClient {
    base_url: String,
    match_id: String,
    player_id: String,
    account_id: String,
    player_session: String,
    controlled_unit_ids: BTreeSet<String>,
    view: OnlineSnapshotResponse,
    client: reqwest::blocking::Client,
    pub poll_accumulator: f32,
}

impl OnlineAuthorityClient {
    pub fn from_env() -> Result<Option<(Self, MissionSimV1)>, String> {
        let Some(base_url) = env::var("TRNM_ONLINE_AUTHORITY_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
        else {
            return Ok(None);
        };
        let match_id = required("TRNM_ONLINE_MATCH_ID")?;
        let player_id = required("TRNM_CEX_ACTOR_ID")?;
        let account_id = required("TRNM_CEX_ACCOUNT_ID")?;
        let player_session = required("TRNM_CEX_PLAYER_SESSION")?;
        let client = reqwest::blocking::Client::builder()
            .connect_timeout(Duration::from_secs(2))
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|error| format!("online authority HTTP client: {error}"))?;
        let view = fetch_snapshot(
            &client,
            &base_url,
            &match_id,
            &player_id,
            &account_id,
            &player_session,
        )?;
        let controlled_unit_ids = view
            .view
            .members
            .iter()
            .find(|member| member.player_id == player_id && member.account_id == account_id)
            .ok_or_else(|| "online match does not contain this player/account".to_string())?
            .controlled_unit_ids
            .iter()
            .cloned()
            .collect();
        let mission = decode_mission(&view)?;
        Ok(Some((
            Self {
                base_url: base_url.trim_end_matches('/').to_string(),
                match_id,
                player_id,
                account_id,
                player_session,
                controlled_unit_ids,
                view,
                client,
                poll_accumulator: 0.0,
            },
            mission,
        )))
    }

    pub fn refresh(&mut self) -> Result<MissionSimV1, String> {
        self.view = fetch_snapshot(
            &self.client,
            &self.base_url,
            &self.match_id,
            &self.player_id,
            &self.account_id,
            &self.player_session,
        )?;
        decode_mission(&self.view)
    }

    pub fn submit(
        &mut self,
        mut order: RtsFrameOrder,
    ) -> Result<(OnlineCommandReceipt, MissionSimV1), String> {
        order
            .subject_actor_ids
            .retain(|unit_id| self.controlled_unit_ids.contains(unit_id));
        if order.subject_actor_ids.is_empty() {
            return Err("selection contains no units assigned to this online member".to_string());
        }
        let last_frame = self
            .view
            .snapshot
            .get("last_order_frame")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or_default();
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
        let mut request = OnlineCommandSubmitRequest {
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
            order,
        };
        let (mut status, mut body) = send_command_request(self, &request)?;
        if !status.is_success() && body.contains("target_tick is outside the authoritative window")
        {
            self.refresh()?;
            let last_frame = self
                .view
                .snapshot
                .get("last_order_frame")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or_default();
            request.sequence = self.view.view.next_sequence;
            request.expected_match_revision = self.view.view.match_revision;
            request.target_tick = self
                .view
                .view
                .authoritative_tick
                .saturating_add(80)
                .max(last_frame.saturating_add(1));
            request.order.frame = u32::try_from(request.target_tick)
                .map_err(|_| "online authoritative tick exceeds frame range".to_string())?;
            (status, body) = send_command_request(self, &request)?;
        }
        if !status.is_success() {
            return Err(format!(
                "online authority rejected command ({status}): {body}"
            ));
        }
        let receipt = serde_json::from_str::<OnlineCommandReceipt>(&body)
            .map_err(|error| format!("online command receipt: {error}"))?;
        let mission = self.refresh()?;
        Ok((receipt, mission))
    }
}

fn send_command_request(
    authority: &OnlineAuthorityClient,
    request: &OnlineCommandSubmitRequest,
) -> Result<(reqwest::StatusCode, String), String> {
    let response = send_with_retry(
        authority
            .client
            .post(format!(
                "{}/v1/online/matches/{}/commands",
                authority.base_url, authority.match_id
            ))
            .header("x-trnm-player-session", &authority.player_session)
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
    base_url: &str,
    match_id: &str,
    player_id: &str,
    account_id: &str,
    player_session: &str,
) -> Result<OnlineSnapshotResponse, String> {
    let response = send_with_retry(
        client
            .post(format!(
                "{}/v1/online/matches/{match_id}/snapshot",
                base_url.trim_end_matches('/')
            ))
            .header("x-trnm-player-session", player_session)
            .json(&OnlineMatchAccessRequest {
                protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
                build_id: ONLINE_AUTHORITY_BUILD.to_string(),
                player_id: player_id.to_string(),
                account_id: account_id.to_string(),
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

fn required(name: &str) -> Result<String, String> {
    env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("{name} is required in online authority mode"))
}
