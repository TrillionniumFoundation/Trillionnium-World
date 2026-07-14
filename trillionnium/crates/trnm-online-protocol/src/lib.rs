use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use trnm_rts_protocol::RtsFrameOrder;

pub const ONLINE_AUTHORITY_V2_PROTOCOL: &str = "trnm_online_authority_v2";
pub const ONLINE_AUTHORITY_V2_BUILD: &str = "trnm-online-authority-2026.07-v2";
pub const ONLINE_AUTHORITY_PROTOCOL: &str = "trnm_online_authority_v3";
pub const ONLINE_AUTHORITY_BUILD: &str = "trnm-online-authority-2026.07-v3";
pub const ONLINE_STREAM_PROTOCOL: &str = "trnm-online-stream-v1";
pub const ONLINE_PRODUCT_V1_PROTOCOL: &str = "trnm_online_product_v1";
pub const ONLINE_PRODUCT_V1_BUILD: &str = "trnm-online-product-2026.07-v1";
pub const ONLINE_PRODUCT_PROTOCOL: &str = "trnm_online_product_v2";
pub const ONLINE_PRODUCT_BUILD: &str = "trnm-online-product-2026.07-v2";
pub const ONLINE_OPERATIONS_V1_PROTOCOL: &str = "trnm_online_operations_v1";
pub const ONLINE_OPERATIONS_V1_BUILD: &str = "trnm-online-operations-2026.07-v1";
pub const ONLINE_OPERATIONS_V2_PROTOCOL: &str = "trnm_online_operations_v2";
pub const ONLINE_OPERATIONS_V2_BUILD: &str = "trnm-online-operations-2026.07-v2";
pub const ONLINE_PRODUCTION_V1_PROTOCOL: &str = "trnm_online_production_v1";
pub const ONLINE_PRODUCTION_V1_BUILD: &str = "trnm-online-production-2026.07-v1";
pub const ONLINE_OPERATIONS_PROTOCOL: &str = "trnm_online_production_v2";
pub const ONLINE_OPERATIONS_BUILD: &str = "trnm-online-production-2026.07-v2";
pub const ONLINE_AUTHORITY_DEFAULT_RULES: &str = "trnm_first_contact_rules_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineClientIdentity {
    pub player_id: String,
    pub account_id: String,
    pub device_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineCampaignConnectRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub slot_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineCampaignView {
    pub protocol_version: String,
    pub campaign_id: String,
    pub player_id: String,
    pub account_id: String,
    pub slot_key: String,
    pub campaign_revision: u64,
    pub schema_revision: u16,
    pub state_hash: String,
    pub level: u32,
    pub experience: u64,
    pub reputation: i32,
    pub inventory: Vec<OnlineInventoryStack>,
    pub settled_match_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineInventoryStack {
    pub item_id: String,
    pub quantity: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineMatchCreateRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub campaign_id: String,
    pub map_id: String,
    pub expected_campaign_revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineMatchJoinRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub campaign_id: String,
    pub join_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineMatchStartRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub expected_match_revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineMatchAccessRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineReconnectRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub last_acknowledged_sequence: u64,
    pub last_snapshot_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_receipt_sequence: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnlineMatchPhase {
    Waiting,
    Running,
    Complete,
    FailedClosed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineMatchMemberView {
    pub player_id: String,
    pub account_id: String,
    pub campaign_id: String,
    pub role: String,
    pub controlled_unit_ids: Vec<String>,
    pub campaign_revision: u64,
    pub level: u32,
    pub experience: u64,
    pub inventory_count: u64,
    #[serde(default)]
    pub next_input_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineMatchView {
    pub protocol_version: String,
    pub build_id: String,
    pub match_id: String,
    pub join_code: String,
    pub phase: OnlineMatchPhase,
    pub match_revision: u64,
    pub authoritative_tick: u64,
    pub next_sequence: u64,
    pub map_id: String,
    pub match_mode: String,
    pub rules_version: String,
    pub seed_hash: String,
    pub snapshot_hash: String,
    pub members: Vec<OnlineMatchMemberView>,
    pub result_hash: Option<String>,
    pub settlement_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineCommandSubmitRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub command_id: String,
    /// Legacy v2 client-predicted global sequence. V3 authority assigns the
    /// global order and uses `input_sequence` for the per-player cursor.
    pub sequence: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_sequence: Option<u64>,
    pub expected_match_revision: u64,
    /// Legacy v2 order frame. V3 applies immediately at the authoritative
    /// server tick and records the client's observed tick separately.
    pub target_tick: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_observed_tick: Option<u64>,
    pub order: RtsFrameOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineCommandReceipt {
    pub protocol_version: String,
    pub match_id: String,
    #[serde(default)]
    pub player_id: String,
    pub command_id: String,
    /// Server-assigned total order. Retains the v2 field name for replay and
    /// rolling compatibility.
    pub sequence: u64,
    #[serde(default)]
    pub input_sequence: u64,
    pub duplicate: bool,
    /// V3: the authoritative tick at which the command was applied. Legacy
    /// V2 receipts retain their historical requested-target semantics.
    pub accepted_tick: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_observed_tick: Option<u64>,
    pub match_revision: u64,
    pub snapshot_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSnapshotResponse {
    pub view: OnlineMatchView,
    pub snapshot: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineReconnectResponse {
    pub view: OnlineMatchView,
    pub snapshot: Value,
    pub replayed_commands: Vec<OnlineCommandReceipt>,
    pub reconnect_count: u64,
    pub full_snapshot_required: bool,
    #[serde(default)]
    pub replay_from_sequence: u64,
    #[serde(default)]
    pub next_receipt_sequence: u64,
    #[serde(default)]
    pub replay_truncated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineStreamConnectRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    #[serde(default)]
    pub next_receipt_sequence: u64,
    #[serde(default)]
    pub last_snapshot_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSnapshotDelta {
    pub base_snapshot_hash: String,
    pub snapshot_hash: String,
    pub base_tick: u64,
    pub authoritative_tick: u64,
    pub changed_fields: BTreeMap<String, Value>,
    pub removed_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "message_type", rename_all = "snake_case")]
pub enum OnlineStreamServerMessage {
    FullSnapshot {
        actor_generation: String,
        state_sequence: u64,
        next_receipt_sequence: u64,
        view: OnlineMatchView,
        snapshot: Value,
    },
    SnapshotDelta {
        actor_generation: String,
        state_sequence: u64,
        base_state_sequence: u64,
        view: OnlineMatchView,
        delta: OnlineSnapshotDelta,
    },
    ResyncRequired {
        actor_generation: String,
        reason: String,
    },
}

pub fn build_snapshot_delta(
    base: &Value,
    next: &Value,
    base_snapshot_hash: String,
    snapshot_hash: String,
    base_tick: u64,
    authoritative_tick: u64,
) -> Option<OnlineSnapshotDelta> {
    let (Value::Object(base), Value::Object(next)) = (base, next) else {
        return None;
    };
    let changed_fields = next
        .iter()
        .filter(|(key, value)| base.get(*key) != Some(*value))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    let removed_fields = base
        .keys()
        .filter(|key| !next.contains_key(*key))
        .cloned()
        .collect();
    Some(OnlineSnapshotDelta {
        base_snapshot_hash,
        snapshot_hash,
        base_tick,
        authoritative_tick,
        changed_fields,
        removed_fields,
    })
}

pub fn apply_snapshot_delta(
    snapshot: &mut Value,
    current_snapshot_hash: &str,
    current_tick: u64,
    delta: &OnlineSnapshotDelta,
) -> Result<(), String> {
    if current_snapshot_hash != delta.base_snapshot_hash || current_tick != delta.base_tick {
        return Err("online snapshot delta base cursor mismatch".to_string());
    }
    let Value::Object(object) = snapshot else {
        return Err("online snapshot delta requires an object base".to_string());
    };
    for key in &delta.removed_fields {
        object.remove(key);
    }
    for (key, value) in &delta.changed_fields {
        object.insert(key.clone(), value.clone());
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLobbyCreateRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub campaign_id: String,
    pub display_name: String,
    pub map_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLobbyAccessRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLobbyInviteRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub target_player_id: String,
    pub expected_lobby_revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLobbyInviteAcceptRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub campaign_id: String,
    pub invite_token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLobbyReadyRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub ready: bool,
    pub expected_lobby_revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLobbyQueueRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub expected_lobby_revision: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnlineLobbyStatus {
    Open,
    Queued,
    Matched,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLobbyMemberView {
    pub player_id: String,
    pub account_id: String,
    pub campaign_id: String,
    pub role: String,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLobbyView {
    pub protocol_version: String,
    pub build_id: String,
    pub lobby_id: String,
    pub display_name: String,
    pub owner_player_id: String,
    pub status: OnlineLobbyStatus,
    pub lobby_revision: u64,
    pub map_id: String,
    pub queue_mode: String,
    pub members: Vec<OnlineLobbyMemberView>,
    pub match_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLobbyInviteReceipt {
    pub lobby: OnlineLobbyView,
    pub invite_id: String,
    pub invite_token: String,
    pub target_player_id: String,
    pub expires_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineMatchmakingReceipt {
    pub lobby: OnlineLobbyView,
    pub match_view: OnlineMatchView,
    pub queue_mode: String,
    pub allocation_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSoloQueueJoinRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub campaign_id: String,
    pub map_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSoloQueueAccessRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnlineSoloQueueStatus {
    Queued,
    Matched,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSoloQueueView {
    pub protocol_version: String,
    pub build_id: String,
    pub ticket_id: String,
    pub player_id: String,
    pub status: OnlineSoloQueueStatus,
    pub queue_mode: String,
    pub map_id: String,
    pub rating: i32,
    pub matched_lobby_id: Option<String>,
    pub match_id: Option<String>,
    pub opponent_player_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineRatingView {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub rating: i32,
    pub wins: u32,
    pub losses: u32,
    pub provisional_matches: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineFriendRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub target_player_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineFriendResolveRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub requester_player_id: String,
    pub accept: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineBlockRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub target_player_id: String,
    pub blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSocialAccessRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSocialView {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub friends: Vec<String>,
    pub incoming_requests: Vec<String>,
    pub outgoing_requests: Vec<String>,
    pub blocked_players: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineReportCreateRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub target_player_id: String,
    pub match_id: String,
    pub category: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineReportResolveRequest {
    pub report_id: String,
    pub decision: String,
    pub resolution: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineReportView {
    pub report_id: String,
    pub reporter_player_id: String,
    pub target_player_id: String,
    pub match_id: String,
    pub category: String,
    pub status: String,
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineOperationsAccessRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSeasonView {
    pub season_id: String,
    pub display_name: String,
    pub status: String,
    pub rules_version: String,
    pub starts_at_epoch: i64,
    pub ends_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLeaderboardEntry {
    pub rank: u32,
    pub player_id: String,
    pub rating: i32,
    pub wins: u32,
    pub losses: u32,
    pub matches: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineLeaderboardView {
    pub protocol_version: String,
    pub build_id: String,
    pub season: OnlineSeasonView,
    pub entries: Vec<OnlineLeaderboardEntry>,
    pub requester: Option<OnlineLeaderboardEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineReplayAccessRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub match_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineReplayView {
    pub match_id: String,
    pub season_id: Option<String>,
    pub result_hash: String,
    pub replay_hash: String,
    pub command_count: u32,
    pub map_id: String,
    pub build_id: String,
    pub participant_ids: Vec<String>,
    pub final_snapshot_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnlineReplayFrameView {
    pub tick: u64,
    pub snapshot_hash: String,
    pub frame_kind: String,
    pub simulation: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnlineReplayCommandView {
    pub sequence: u64,
    pub player_id: String,
    pub target_tick: u64,
    pub request_hash: String,
    pub accepted_snapshot_hash: String,
    pub order: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnlineReplayPlaybackView {
    pub replay: OnlineReplayView,
    pub commands: Vec<OnlineReplayCommandView>,
    pub frames: Vec<OnlineReplayFrameView>,
    pub result: serde_json::Value,
    pub integrity_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineReplayReportCreateRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub target_player_id: String,
    pub match_id: String,
    pub replay_hash: String,
    pub category: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineIntegritySignalView {
    pub signal_id: String,
    pub match_id: Option<String>,
    pub player_ids: Vec<String>,
    pub signal_kind: String,
    pub severity: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineModerationQueueRequest {
    pub status: String,
    pub limit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineModerationCaseView {
    pub report: OnlineReportView,
    pub replay: Option<OnlineReplayView>,
    pub integrity_signals: Vec<OnlineIntegritySignalView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineModerationQueueView {
    pub cases: Vec<OnlineModerationCaseView>,
    pub open_count: u32,
    pub under_review_signal_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineModerationActionRequest {
    pub report_id: String,
    pub decision: String,
    pub resolution: String,
    pub enforcement_scope: Option<String>,
    pub suspension_hours: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineModerationActionView {
    pub report: OnlineReportView,
    pub audit_id: String,
    pub enforcement_id: Option<String>,
    pub target_player_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineEnforcementAppealCreateRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub enforcement_id: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineEnforcementAppealView {
    pub appeal_id: String,
    pub enforcement_id: String,
    pub player_id: String,
    pub status: String,
    pub detail: String,
    pub resolution: Option<String>,
    pub created_at_epoch: i64,
    pub due_at_epoch: i64,
    pub overdue: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineEnforcementAppealResolveRequest {
    pub appeal_id: String,
    pub decision: String,
    pub resolution: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineEnforcementAppealQueueRequest {
    pub status: String,
    pub limit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineEnforcementAppealQueueView {
    pub appeals: Vec<OnlineEnforcementAppealView>,
    pub pending_count: u32,
    pub overdue_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSeasonAdminRequest {
    pub action: String,
    pub season_id: String,
    pub display_name: Option<String>,
    pub rules_version: Option<String>,
    pub starts_at_epoch: Option<i64>,
    pub ends_at_epoch: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSeasonAdminView {
    pub audit_id: String,
    pub season: OnlineSeasonView,
    pub previous_active_season_id: Option<String>,
    pub archived_entries: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSeasonAutomationRequest {
    pub season_id: String,
    pub automatic_activation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSeasonAutomationView {
    pub season_id: String,
    pub automatic_activation: bool,
    pub automation_state: String,
    pub deferred_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSpectatorInviteCreateRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub match_id: String,
    pub target_player_id: String,
    pub delay_seconds: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSpectatorInviteReceipt {
    pub invite_id: String,
    pub match_id: String,
    pub target_player_id: String,
    pub invite_token: String,
    pub delay_seconds: u32,
    pub expires_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSpectatorInviteAcceptRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub invite_token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSpectatorGrantView {
    pub grant_id: String,
    pub match_id: String,
    pub viewer_player_id: String,
    pub delay_seconds: u32,
    pub expires_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineSpectatorPlaybackRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
    pub grant_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnlineSpectatorPlaybackView {
    pub grant: OnlineSpectatorGrantView,
    pub match_phase: String,
    pub authoritative_tick: u64,
    pub visible_through_tick: u64,
    pub frames: Vec<OnlineReplayFrameView>,
    pub terminal_visible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineProductionStatusView {
    pub protocol_version: String,
    pub build_id: String,
    pub signer_ready: bool,
    pub signer_key_id: String,
    pub signer_custody: String,
    pub request_rate_limit_per_minute: u32,
    pub request_body_limit_bytes: u32,
    pub automatic_season_id: Option<String>,
    pub pending_appeals: u32,
    pub overdue_appeals: u32,
    pub escalated_appeals: u32,
    pub physical_host_id: String,
    pub distinct_healthy_physical_hosts: u32,
    pub public_edge_attested: bool,
    pub distributed_admission: bool,
    pub current_admission_requests: u32,
    pub current_admission_rejections: u32,
    pub recent_capacity_samples: u32,
    pub active_moderation_shifts: u32,
    pub signer_provider_kind: String,
    pub signer_registry_verified: bool,
    pub kms_hsm_attested: bool,
    pub cross_host_failover_attested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineProductionPlayerStatusRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub player_id: String,
    pub account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineProductionPlayerStatusView {
    pub protocol_version: String,
    pub build_id: String,
    pub active_season_id: Option<String>,
    pub active_season_ends_at_epoch: Option<i64>,
    pub automatic_season_id: Option<String>,
    pub region: String,
    pub fleet_capacity: u32,
    pub active_matches: u32,
    pub admission_state: String,
    pub admission_limit_per_minute: u32,
    pub active_spectator_grants: u32,
    pub signer_key_id: String,
    pub signer_provider_kind: String,
    pub signer_registry_verified: bool,
    pub distinct_healthy_physical_hosts: u32,
    pub cross_host_failover_attested: bool,
    pub public_edge_attested: bool,
    pub kms_hsm_attested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineHostAttestationRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub challenge: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineHostAttestationView {
    pub protocol_version: String,
    pub build_id: String,
    pub instance_id: String,
    pub instance_epoch: i64,
    pub physical_host_id: String,
    pub region: String,
    pub challenge: String,
    pub observed_at_epoch: i64,
    pub evidence_hash: String,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineModerationShiftStartRequest {
    pub moderator_id: String,
    pub duration_minutes: u32,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineModerationShiftAccessRequest {
    pub shift_id: String,
    pub moderator_id: String,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineModerationCaseClaimRequest {
    pub shift_id: String,
    pub moderator_id: String,
    pub case_kind: String,
    pub case_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineModerationCaseClaimView {
    pub claim_id: String,
    pub shift_id: String,
    pub case_kind: String,
    pub case_id: String,
    pub status: String,
    pub claimed_at_epoch: i64,
    pub resolved_at_epoch: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineModerationShiftView {
    pub shift_id: String,
    pub moderator_id: String,
    pub status: String,
    pub starts_at_epoch: i64,
    pub ends_at_epoch: i64,
    pub last_heartbeat_epoch: i64,
    pub open_claims: u32,
    pub resolved_claims: u32,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineFleetRouteRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub preferred_region: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineFleetInstanceView {
    pub instance_id: String,
    pub physical_host_id: String,
    pub region: String,
    pub public_endpoint: String,
    pub capacity: u32,
    pub active_matches: u32,
    pub status: String,
    pub heartbeat_age_seconds: i64,
    pub instance_epoch: u64,
    pub lease_remaining_seconds: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineFleetRouteView {
    pub protocol_version: String,
    pub build_id: String,
    pub selected: OnlineFleetInstanceView,
    pub healthy_instances: u32,
    pub cross_region_fallback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineFleetAdminRequest {
    pub instance_id: String,
    pub action: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineFleetAdminView {
    pub audit_id: String,
    pub instance_id: String,
    pub status: String,
    pub active_matches: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineAuthorityError {
    pub error: String,
    pub recoverable: bool,
    pub authoritative_revision: Option<u64>,
}

pub fn validate_client_contract(protocol_version: &str, build_id: &str) -> Result<(), String> {
    let supported = (protocol_version == ONLINE_AUTHORITY_PROTOCOL
        && build_id == ONLINE_AUTHORITY_BUILD)
        || (protocol_version == ONLINE_AUTHORITY_V2_PROTOCOL
            && build_id == ONLINE_AUTHORITY_V2_BUILD);
    if !supported
        && protocol_version != ONLINE_AUTHORITY_PROTOCOL
        && protocol_version != ONLINE_AUTHORITY_V2_PROTOCOL
    {
        return Err(format!("unsupported online protocol {protocol_version}"));
    }
    if !supported {
        return Err(format!("client build {build_id} is not compatible"));
    }
    Ok(())
}

pub fn validate_product_contract(protocol_version: &str, build_id: &str) -> Result<(), String> {
    let supported = (protocol_version == ONLINE_PRODUCT_PROTOCOL
        && build_id == ONLINE_PRODUCT_BUILD)
        || (protocol_version == ONLINE_PRODUCT_V1_PROTOCOL && build_id == ONLINE_PRODUCT_V1_BUILD);
    if !supported
        && protocol_version != ONLINE_PRODUCT_PROTOCOL
        && protocol_version != ONLINE_PRODUCT_V1_PROTOCOL
    {
        return Err(format!(
            "unsupported online product protocol {protocol_version}"
        ));
    }
    if !supported {
        return Err(format!("online product build {build_id} is not compatible"));
    }
    Ok(())
}

pub fn validate_operations_contract(protocol_version: &str, build_id: &str) -> Result<(), String> {
    let supported = (protocol_version == ONLINE_OPERATIONS_PROTOCOL
        && build_id == ONLINE_OPERATIONS_BUILD)
        || (protocol_version == ONLINE_PRODUCTION_V1_PROTOCOL
            && build_id == ONLINE_PRODUCTION_V1_BUILD)
        || (protocol_version == ONLINE_OPERATIONS_V2_PROTOCOL
            && build_id == ONLINE_OPERATIONS_V2_BUILD)
        || (protocol_version == ONLINE_OPERATIONS_V1_PROTOCOL
            && build_id == ONLINE_OPERATIONS_V1_BUILD);
    if !supported
        && protocol_version != ONLINE_OPERATIONS_PROTOCOL
        && protocol_version != ONLINE_PRODUCTION_V1_PROTOCOL
        && protocol_version != ONLINE_OPERATIONS_V2_PROTOCOL
        && protocol_version != ONLINE_OPERATIONS_V1_PROTOCOL
    {
        return Err(format!(
            "unsupported online operations protocol {protocol_version}"
        ));
    }
    if !supported {
        return Err(format!(
            "online operations build {build_id} is not compatible"
        ));
    }
    Ok(())
}

pub fn validate_production_contract(protocol_version: &str, build_id: &str) -> Result<(), String> {
    if (protocol_version == ONLINE_OPERATIONS_PROTOCOL && build_id == ONLINE_OPERATIONS_BUILD)
        || (protocol_version == ONLINE_PRODUCTION_V1_PROTOCOL
            && build_id == ONLINE_PRODUCTION_V1_BUILD)
    {
        Ok(())
    } else {
        Err("unsupported Online Production protocol/build pair".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_and_build_must_match_exactly() {
        assert!(
            validate_client_contract(ONLINE_AUTHORITY_PROTOCOL, ONLINE_AUTHORITY_BUILD).is_ok()
        );
        assert!(
            validate_client_contract(ONLINE_AUTHORITY_V2_PROTOCOL, ONLINE_AUTHORITY_V2_BUILD)
                .is_ok()
        );
        assert!(
            validate_client_contract(ONLINE_AUTHORITY_V2_PROTOCOL, ONLINE_AUTHORITY_BUILD).is_err()
        );
        assert!(validate_client_contract("v0", ONLINE_AUTHORITY_BUILD).is_err());
        assert!(validate_client_contract(ONLINE_AUTHORITY_PROTOCOL, "old-build").is_err());
        assert!(validate_product_contract(ONLINE_PRODUCT_PROTOCOL, ONLINE_PRODUCT_BUILD).is_ok());
        assert!(
            validate_product_contract(ONLINE_PRODUCT_V1_PROTOCOL, ONLINE_PRODUCT_V1_BUILD).is_ok()
        );
        assert!(
            validate_product_contract(ONLINE_PRODUCT_PROTOCOL, ONLINE_PRODUCT_V1_BUILD).is_err()
        );
        assert!(
            validate_operations_contract(ONLINE_OPERATIONS_PROTOCOL, ONLINE_OPERATIONS_BUILD)
                .is_ok()
        );
        assert!(validate_operations_contract(ONLINE_OPERATIONS_PROTOCOL, "old-build").is_err());
        assert!(validate_operations_contract(
            ONLINE_OPERATIONS_V2_PROTOCOL,
            ONLINE_OPERATIONS_V2_BUILD
        )
        .is_ok());
        assert!(validate_operations_contract(
            ONLINE_OPERATIONS_V1_PROTOCOL,
            ONLINE_OPERATIONS_V1_BUILD
        )
        .is_ok());
        assert!(
            validate_production_contract(ONLINE_OPERATIONS_PROTOCOL, ONLINE_OPERATIONS_BUILD)
                .is_ok()
        );
        assert!(validate_production_contract(
            ONLINE_PRODUCTION_V1_PROTOCOL,
            ONLINE_PRODUCTION_V1_BUILD
        )
        .is_ok());
        assert!(validate_production_contract(
            ONLINE_PRODUCTION_V1_PROTOCOL,
            ONLINE_OPERATIONS_BUILD
        )
        .is_err());
    }

    #[test]
    fn wire_contract_round_trips() {
        let request = OnlineCampaignConnectRequest {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            player_id: "player-a".to_string(),
            account_id: "00000000-0000-0000-0000-000000000001".to_string(),
            slot_key: "main".to_string(),
        };
        let encoded = serde_json::to_vec(&request).unwrap();
        assert_eq!(
            serde_json::from_slice::<OnlineCampaignConnectRequest>(&encoded).unwrap(),
            request
        );
    }

    #[test]
    fn top_level_snapshot_delta_round_trips_exactly() {
        let base = serde_json::json!({
            "tick": 10,
            "seed": {"map": "first_contact"},
            "party": [{"id": "hero", "x": 1}],
            "obsolete": true
        });
        let next = serde_json::json!({
            "tick": 11,
            "seed": {"map": "first_contact"},
            "party": [{"id": "hero", "x": 2}],
            "phase": "contact"
        });
        let delta = build_snapshot_delta(
            &base,
            &next,
            "base-hash".to_string(),
            "next-hash".to_string(),
            10,
            11,
        )
        .unwrap();
        assert!(!delta.changed_fields.contains_key("seed"));
        assert_eq!(delta.removed_fields, vec!["obsolete"]);

        let mut applied = base;
        apply_snapshot_delta(&mut applied, "base-hash", 10, &delta).unwrap();
        assert_eq!(applied, next);
        assert!(apply_snapshot_delta(&mut applied, "wrong-hash", 11, &delta).is_err());
    }

    #[test]
    fn stream_messages_are_explicitly_tagged() {
        let message = OnlineStreamServerMessage::ResyncRequired {
            actor_generation: "actor-a".to_string(),
            reason: "base_hash_mismatch".to_string(),
        };
        let encoded = serde_json::to_value(&message).unwrap();
        assert_eq!(encoded["message_type"], "resync_required");
        assert_eq!(
            serde_json::from_value::<OnlineStreamServerMessage>(encoded).unwrap(),
            message
        );
    }
}
