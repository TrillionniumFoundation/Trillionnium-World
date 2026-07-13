use serde::{Deserialize, Serialize};
use serde_json::Value;
use trnm_rts_protocol::RtsFrameOrder;

pub const ONLINE_AUTHORITY_PROTOCOL: &str = "trnm_online_authority_v2";
pub const ONLINE_AUTHORITY_BUILD: &str = "trnm-online-authority-2026.07-v2";
pub const ONLINE_PRODUCT_V1_PROTOCOL: &str = "trnm_online_product_v1";
pub const ONLINE_PRODUCT_V1_BUILD: &str = "trnm-online-product-2026.07-v1";
pub const ONLINE_PRODUCT_PROTOCOL: &str = "trnm_online_product_v2";
pub const ONLINE_PRODUCT_BUILD: &str = "trnm-online-product-2026.07-v2";
pub const ONLINE_OPERATIONS_V1_PROTOCOL: &str = "trnm_online_operations_v1";
pub const ONLINE_OPERATIONS_V1_BUILD: &str = "trnm-online-operations-2026.07-v1";
pub const ONLINE_OPERATIONS_PROTOCOL: &str = "trnm_online_operations_v2";
pub const ONLINE_OPERATIONS_BUILD: &str = "trnm-online-operations-2026.07-v2";
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
    pub sequence: u64,
    pub expected_match_revision: u64,
    pub target_tick: u64,
    pub order: RtsFrameOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineCommandReceipt {
    pub protocol_version: String,
    pub match_id: String,
    pub command_id: String,
    pub sequence: u64,
    pub duplicate: bool,
    pub accepted_tick: u64,
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
pub struct OnlineFleetRouteRequest {
    pub protocol_version: String,
    pub build_id: String,
    pub preferred_region: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnlineFleetInstanceView {
    pub instance_id: String,
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
    if protocol_version != ONLINE_AUTHORITY_PROTOCOL {
        return Err(format!("unsupported online protocol {protocol_version}"));
    }
    if build_id != ONLINE_AUTHORITY_BUILD {
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
        || (protocol_version == ONLINE_OPERATIONS_V1_PROTOCOL
            && build_id == ONLINE_OPERATIONS_V1_BUILD);
    if !supported
        && protocol_version != ONLINE_OPERATIONS_PROTOCOL
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_and_build_must_match_exactly() {
        assert!(
            validate_client_contract(ONLINE_AUTHORITY_PROTOCOL, ONLINE_AUTHORITY_BUILD).is_ok()
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
            ONLINE_OPERATIONS_V1_PROTOCOL,
            ONLINE_OPERATIONS_V1_BUILD
        )
        .is_ok());
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
}
