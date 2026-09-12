#![recursion_limit = "1024"]

//! Rust-owned Trillionnium World read projections.

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use trnm_world_domain::{
    trillionnium_authored_quest_chain_catalog_json, trillionnium_fixture_skill_definitions,
    trillionnium_hash_id, trillionnium_item_equipment_catalog_json, trillionnium_npc_fixtures,
    trillionnium_resource_pressure_loops_json, trillionnium_sect_fixtures,
    trillionnium_story_arc_catalog_json, trillionnium_task_archetype_fixtures,
    trillionnium_training_command_fixtures, WorldState, WorldTacticsGameSession,
    WorldTacticsRewardSettlement, WorldTacticsSimulationTick, WorldTrillionniumCharacter,
    WorldTrillionniumCombatNumericsState, WorldTrillionniumRegionStoryUnlockState,
    WorldTrillionniumResourcePressureState, TRILLIONNIUM_BATTLE_LOG_STYLE_CONTRACT_VERSION,
    TRILLIONNIUM_COMBAT_LOG_CONTRACT_VERSION,
    TRILLIONNIUM_HERO_TAN_FULL_CONTENT_ALIGNMENT_CONTRACT_VERSION,
    TRILLIONNIUM_MAP_OVERLAY_IDENTITY_CONTRACT_VERSION,
    TRILLIONNIUM_MENTOR_TRAINING_TASK_CONTRACT_VERSION,
    TRILLIONNIUM_NPC_COMMAND_DESCRIPTOR_CONTRACT_VERSION, TRILLIONNIUM_NPC_CONTRACT_VERSION,
    TRILLIONNIUM_NPC_RELATIONSHIP_CONTRACT_VERSION, TRILLIONNIUM_NPC_SPAWN_CONTRACT_VERSION,
    TRILLIONNIUM_OSM_OBJECTIVE_CONTRACT_VERSION, TRILLIONNIUM_REWARD_GATE_CONTRACT_VERSION,
    TRILLIONNIUM_SECT_CONTRACT_VERSION, TRILLIONNIUM_SECT_OSM_BINDING_CONTRACT_VERSION,
    TRILLIONNIUM_SKILL_CONTRACT_VERSION, TRILLIONNIUM_TACTICS_ACCESSIBILITY_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_BOARD_CELL_INTERACTION_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_BOARD_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_COMBAT_RESOLUTION_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_COMMAND_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_COMMAND_INTENT_DRAFT_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_COMMAND_OUTCOME_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_GAME_SESSION_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_REPEAT_FARMING_ANTI_CHEESE_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_REWARD_SETTLEMENT_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_SIMULATION_TICK_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_UNIT_CONTRACT_VERSION,
    TRILLIONNIUM_TACTICS_UNIT_SELECTION_CONTRACT_VERSION,
    TRILLIONNIUM_TASK_ARCHETYPE_CONTRACT_VERSION, TRILLIONNIUM_TASK_COMPLETION_CONTRACT_VERSION,
    TRILLIONNIUM_TRAINING_CONTRACT_VERSION,
    TRILLIONNIUM_WORLD_AUTHORED_QUEST_CHAIN_CONTRACT_VERSION,
    TRILLIONNIUM_WORLD_COMBAT_ENCOUNTER_LOOP_CONTRACT_VERSION,
    TRILLIONNIUM_WORLD_COMBAT_NUMERICS_RUNTIME_CONTRACT_VERSION,
    TRILLIONNIUM_WORLD_DYNAMIC_SOCIAL_SIMULATION_CONTRACT_VERSION,
    TRILLIONNIUM_WORLD_FOOD_WATER_AGE_SURVIVAL_CONTRACT_VERSION,
    TRILLIONNIUM_WORLD_ITEM_EQUIPMENT_RUNTIME_CONTRACT_VERSION,
    TRILLIONNIUM_WORLD_OBJECTIVE_TRAVEL_CONTRACT_VERSION,
    TRILLIONNIUM_WORLD_REGION_STORY_UNLOCK_RUNTIME_CONTRACT_VERSION,
    TRILLIONNIUM_WORLD_RESOURCE_PRESSURE_RUNTIME_CONTRACT_VERSION,
    TRILLIONNIUM_WORLD_SKILL_PRACTICE_LOOP_CONTRACT_VERSION, WORLD_RUST_SOURCE_OF_TRUTH,
};

pub const WORLD_PROJECTION_CONTRACT: &str = "trillionnium_world_projection_v1";
pub const WORLD_RUST_UI_FRAGMENT_CONTRACT: &str = "trillionnium_world_rust_ui_fragments_v1";
pub const WORLD_ROUTE_COMMAND_TARGET_CONTRACT: &str = "trillionnium_world_route_command_target_v1";
pub const TRILLIONNIUM_WORLD_ROUTE_RECOMMENDATION_POLICY_CONTRACT_VERSION: &str =
    "trillionnium_world_route_recommendation_policy_v1";
pub const WORLD_ROUTE_UI_CONTRACT_VERSION: u32 = 1;

pub const WORLD_ROUTE_WORK_DELIVER_INPUT_ID: &str = "world-work-deliver-id";
pub const WORLD_ROUTE_WORK_DELIVER_TEXTAREA_ID: &str = "world-work-deliver-body";
pub const WORLD_ROUTE_WORK_ACCEPT_INPUT_ID: &str = "world-work-accept-id";
pub const WORLD_ROUTE_WORK_ACCEPT_TEXTAREA_ID: &str = "world-work-accept-body";
pub const WORLD_ROUTE_WORK_REJECT_INPUT_ID: &str = "world-work-reject-id";
pub const WORLD_ROUTE_WORK_REJECT_TEXTAREA_ID: &str = "world-work-reject-body";
pub const WORLD_ROUTE_WORK_REOPEN_INPUT_ID: &str = "world-work-reopen-id";
pub const WORLD_ROUTE_WORK_REOPEN_TEXTAREA_ID: &str = "world-work-reopen-body";
pub const WORLD_ROUTE_WORK_CANCEL_INPUT_ID: &str = "world-work-cancel-id";
pub const WORLD_ROUTE_WORK_CANCEL_TEXTAREA_ID: &str = "world-work-cancel-body";
pub const WORLD_ROUTE_WORK_LANE_INPUT_IDS: &[&str] = &[
    WORLD_ROUTE_WORK_DELIVER_INPUT_ID,
    WORLD_ROUTE_WORK_ACCEPT_INPUT_ID,
    WORLD_ROUTE_WORK_REJECT_INPUT_ID,
    WORLD_ROUTE_WORK_REOPEN_INPUT_ID,
    WORLD_ROUTE_WORK_CANCEL_INPUT_ID,
];
pub const WORLD_ROUTE_MAP_MOVE_PANEL_ID: &str = "world-map-move-panel";
pub const WORLD_ROUTE_MOVE_TARGET_ID: &str = "world-map-move-target";
pub const WORLD_ROUTE_ACTION_PANEL_ID: &str = "world-action-console";
pub const WORLD_ROUTE_ACTION_LOCATION_ID: &str = "world-action-location";
pub const WORLD_ROUTE_ACTION_TEXTAREA_ID: &str = "world-action-body";
pub const WORLD_ROUTE_ASSETS_PANEL_ID: &str = "world-assets-panel";
pub const WORLD_ROUTE_ASSET_INPUT_ID: &str = "world-asset-id";
pub const WORLD_ROUTE_ASSET_TEXTAREA_ID: &str = "world-asset-body";
pub const WORLD_ROUTE_COMPANIES_PANEL_ID: &str = "world-companies-panel";
pub const WORLD_ROUTE_COMPANY_INPUT_ID: &str = "world-company-asset-id";
pub const WORLD_ROUTE_COMPANY_TEXTAREA_ID: &str = "world-company-body";
pub const WORLD_ROUTE_LISTINGS_PANEL_ID: &str = "world-listings-panel";
pub const WORLD_ROUTE_LISTING_INPUT_ID: &str = "world-listing-company-id";
pub const WORLD_ROUTE_LISTING_TEXTAREA_ID: &str = "world-listing-body";
pub const WORLD_ROUTE_COMMERCE_PANEL_ID: &str = "world-commerce-panel";
pub const WORLD_ROUTE_CONTRACTS_PANEL_ID: &str = "world-contracts-panel";
pub const WORLD_ROUTE_PURCHASE_INPUT_ID: &str = "world-buy-listing-id";
pub const WORLD_ROUTE_PURCHASE_TEXTAREA_ID: &str = "world-buy-body";
pub const WORLD_ROUTE_CONTRACT_INPUT_ID: &str = "world-contract-completion-id";
pub const WORLD_ROUTE_CONTRACT_TEXTAREA_ID: &str = "world-contract-completion-body";
pub const WORLD_ROUTE_EVENT_TIMELINE_ID: &str = "world-event-timeline";
pub const WORLD_ROUTE_LEAGUE_LINK_ID: &str = "world-league-link";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldHomeProjection {
    pub contract_version: String,
    pub source_of_truth: String,
    pub node_count: usize,
    pub route_count: usize,
    pub npc_count: usize,
    pub task_count: usize,
    pub player_node_id: Option<String>,
    pub first_action_prompt: String,
    pub rust_owned_fragment_contract: String,
    pub route_command_target_contract: String,
    pub route_ui_contract_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteCommandTarget {
    pub contract_version: String,
    pub action_label: String,
    pub panel_id: String,
    pub input_id: String,
    pub input_value: String,
    pub textarea_id: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteFocusLaneSpec {
    pub preferred_node_id: Option<String>,
    pub desired_tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteOpportunity {
    pub kind: String,
    pub hint: String,
    pub playbook: String,
    pub command: String,
    pub route_target: WorldRouteCommandTarget,
    pub focus_lane: WorldRouteFocusLaneSpec,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteSuggestedAction {
    pub route_target: WorldRouteCommandTarget,
    pub matrix_command: String,
    pub focus_lane: WorldRouteFocusLaneSpec,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteTaskDescriptor {
    pub task_id: String,
    pub latest_bucket: String,
    pub latest_status: String,
    pub latest_location_id: String,
    pub latest_title: String,
    pub latest_summary: String,
    pub latest_detail: String,
    pub latest_event_title: String,
    pub latest_contract_id: String,
    pub latest_contract_title: String,
    pub latest_completion_id: String,
    pub latest_completion_title: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRoutePreviewItem {
    pub route_bucket: String,
    pub location_id: String,
    pub task_id: String,
    pub route_status: String,
    pub created_at_epoch: i64,
    pub event_id: String,
    pub contract_id: String,
    pub completion_id: String,
    pub purchase_id: String,
    pub listing_id: String,
    pub work_order_id: String,
    pub title: String,
    pub summary: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteStoryOpportunityTarget {
    pub action_label: String,
    pub panel_id: String,
    pub input_id: String,
    pub input_value: String,
    pub textarea_id: String,
    pub body: String,
    pub node_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteStoryView {
    pub preview_item_count: u64,
    pub task_linked_count: u64,
    pub task_graph_count: u64,
    pub next_task_id: String,
    pub next_action_label: String,
    pub next_panel_id: String,
    pub next_command_hint: String,
    pub next_location_id: String,
    pub next_node_id: String,
    pub next_opportunity_node_id: String,
    pub next_stage_summary: String,
    pub next_opportunity_kind: String,
    pub next_outcome_summary: String,
    pub next_feedback_focus: String,
    pub next_opportunity_hint: String,
    pub next_opportunity_playbook: String,
    pub next_opportunity_command: String,
    pub next_opportunity_target: WorldRouteStoryOpportunityTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteTaskGraphView {
    pub task_id: String,
    pub latest_bucket: String,
    pub latest_status: String,
    pub latest_location_id: String,
    pub event_count: u64,
    pub contract_count: u64,
    pub completion_count: u64,
    pub latest_contract_id: String,
    pub latest_created_at_epoch: i64,
    pub route_stage_summary: String,
    pub outcome_summary: String,
    pub feedback_focus: String,
    pub next_opportunity_kind: String,
    pub next_opportunity_hint: String,
    pub next_opportunity_playbook: String,
    pub next_opportunity_command: String,
    pub next_opportunity_action_label: String,
    pub next_opportunity_panel_id: String,
    pub next_opportunity_input_id: String,
    pub next_opportunity_input_value: String,
    pub next_opportunity_textarea_id: String,
    pub next_opportunity_node_id: String,
    pub next_opportunity_body: String,
    pub suggested_action_label: String,
    pub suggested_panel_id: String,
    pub suggested_input_id: String,
    pub suggested_input_value: String,
    pub suggested_textarea_id: String,
    pub suggested_matrix_command: String,
    pub suggested_node_id: String,
    pub suggested_body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteArtifacts {
    pub preview: Value,
    pub task_graph: Value,
    pub task_views: Vec<WorldRouteTaskGraphView>,
    pub story: WorldRouteStoryView,
}

impl WorldRouteCommandTarget {
    fn lane(
        action_label: &str,
        panel_id: &str,
        input_id: &str,
        input_value: String,
        textarea_id: &str,
        body: String,
    ) -> Self {
        Self {
            contract_version: WORLD_ROUTE_COMMAND_TARGET_CONTRACT.to_string(),
            action_label: action_label.to_string(),
            panel_id: panel_id.to_string(),
            input_id: input_id.to_string(),
            input_value,
            textarea_id: textarea_id.to_string(),
            body: world_route_playability_anchor_body(body),
        }
    }
}

pub fn project_home(state: &WorldState, actor_id: &str) -> WorldHomeProjection {
    let player_node_id = state
        .positions
        .iter()
        .find(|position| position.actor_id == actor_id)
        .map(|position| position.node_id.clone());
    let first_action_prompt = player_node_id
        .as_deref()
        .and_then(|node_id| state.nodes.iter().find(|node| node.id == node_id))
        .map(|node| {
            format!(
                "You are at {}. Choose move, talk, train, or task.",
                node.name
            )
        })
        .unwrap_or_else(|| "Choose a spawn point before issuing world intent.".to_string());

    WorldHomeProjection {
        contract_version: WORLD_PROJECTION_CONTRACT.to_string(),
        source_of_truth: WORLD_RUST_SOURCE_OF_TRUTH.to_string(),
        node_count: state.nodes.len(),
        route_count: state.edges.len(),
        npc_count: state.npcs.len(),
        task_count: state.tasks.len(),
        player_node_id,
        first_action_prompt,
        rust_owned_fragment_contract: WORLD_RUST_UI_FRAGMENT_CONTRACT.to_string(),
        route_command_target_contract: WORLD_ROUTE_COMMAND_TARGET_CONTRACT.to_string(),
        route_ui_contract_version: WORLD_ROUTE_UI_CONTRACT_VERSION,
    }
}

pub fn world_route_ui_contract_json() -> Value {
    let mut panel_defaults = Map::new();
    panel_defaults.insert(
        WORLD_ROUTE_MAP_MOVE_PANEL_ID.to_string(),
        json!({ "input_id": WORLD_ROUTE_MOVE_TARGET_ID }),
    );
    panel_defaults.insert(
        WORLD_ROUTE_ACTION_PANEL_ID.to_string(),
        json!({ "textarea_id": WORLD_ROUTE_ACTION_TEXTAREA_ID }),
    );
    panel_defaults.insert(
        WORLD_ROUTE_ASSETS_PANEL_ID.to_string(),
        json!({
            "input_id": WORLD_ROUTE_ASSET_INPUT_ID,
            "textarea_id": WORLD_ROUTE_ASSET_TEXTAREA_ID,
        }),
    );
    panel_defaults.insert(
        WORLD_ROUTE_COMPANIES_PANEL_ID.to_string(),
        json!({
            "input_id": WORLD_ROUTE_COMPANY_INPUT_ID,
            "textarea_id": WORLD_ROUTE_COMPANY_TEXTAREA_ID,
        }),
    );
    panel_defaults.insert(
        WORLD_ROUTE_LISTINGS_PANEL_ID.to_string(),
        json!({
            "input_id": WORLD_ROUTE_LISTING_INPUT_ID,
            "textarea_id": WORLD_ROUTE_LISTING_TEXTAREA_ID,
        }),
    );
    panel_defaults.insert(
        WORLD_ROUTE_COMMERCE_PANEL_ID.to_string(),
        json!({
            "input_id": WORLD_ROUTE_PURCHASE_INPUT_ID,
            "textarea_id": WORLD_ROUTE_PURCHASE_TEXTAREA_ID,
        }),
    );
    panel_defaults.insert(
        WORLD_ROUTE_CONTRACTS_PANEL_ID.to_string(),
        json!({
            "input_id": WORLD_ROUTE_CONTRACT_INPUT_ID,
            "textarea_id": WORLD_ROUTE_CONTRACT_TEXTAREA_ID,
        }),
    );
    Value::Object(Map::from_iter([
        (
            "contract_version".to_string(),
            json!(WORLD_ROUTE_UI_CONTRACT_VERSION),
        ),
        (
            "panels".to_string(),
            json!({
                "map_move": WORLD_ROUTE_MAP_MOVE_PANEL_ID,
                "action": WORLD_ROUTE_ACTION_PANEL_ID,
                "assets": WORLD_ROUTE_ASSETS_PANEL_ID,
                "companies": WORLD_ROUTE_COMPANIES_PANEL_ID,
                "listings": WORLD_ROUTE_LISTINGS_PANEL_ID,
                "commerce": WORLD_ROUTE_COMMERCE_PANEL_ID,
                "contracts": WORLD_ROUTE_CONTRACTS_PANEL_ID,
                "event_timeline": WORLD_ROUTE_EVENT_TIMELINE_ID,
                "league_link": WORLD_ROUTE_LEAGUE_LINK_ID,
            }),
        ),
        (
            "fields".to_string(),
            json!({
                "move_target": WORLD_ROUTE_MOVE_TARGET_ID,
                "action_location": WORLD_ROUTE_ACTION_LOCATION_ID,
                "action_textarea": WORLD_ROUTE_ACTION_TEXTAREA_ID,
                "asset_input": WORLD_ROUTE_ASSET_INPUT_ID,
                "asset_textarea": WORLD_ROUTE_ASSET_TEXTAREA_ID,
                "company_input": WORLD_ROUTE_COMPANY_INPUT_ID,
                "company_textarea": WORLD_ROUTE_COMPANY_TEXTAREA_ID,
                "listing_input": WORLD_ROUTE_LISTING_INPUT_ID,
                "listing_textarea": WORLD_ROUTE_LISTING_TEXTAREA_ID,
                "purchase_input": WORLD_ROUTE_PURCHASE_INPUT_ID,
                "purchase_textarea": WORLD_ROUTE_PURCHASE_TEXTAREA_ID,
                "contract_input": WORLD_ROUTE_CONTRACT_INPUT_ID,
                "contract_textarea": WORLD_ROUTE_CONTRACT_TEXTAREA_ID,
            }),
        ),
        ("panel_defaults".to_string(), Value::Object(panel_defaults)),
        (
            "work_lane_order".to_string(),
            json!([
                "delivery",
                "acceptance",
                "rejection",
                "reopen",
                "cancellation"
            ]),
        ),
        (
            "work_lanes".to_string(),
            json!({
                "delivery": {
                    "input_id": WORLD_ROUTE_WORK_DELIVER_INPUT_ID,
                    "textarea_id": WORLD_ROUTE_WORK_DELIVER_TEXTAREA_ID,
                },
                "acceptance": {
                    "input_id": WORLD_ROUTE_WORK_ACCEPT_INPUT_ID,
                    "textarea_id": WORLD_ROUTE_WORK_ACCEPT_TEXTAREA_ID,
                },
                "rejection": {
                    "input_id": WORLD_ROUTE_WORK_REJECT_INPUT_ID,
                    "textarea_id": WORLD_ROUTE_WORK_REJECT_TEXTAREA_ID,
                },
                "reopen": {
                    "input_id": WORLD_ROUTE_WORK_REOPEN_INPUT_ID,
                    "textarea_id": WORLD_ROUTE_WORK_REOPEN_TEXTAREA_ID,
                },
                "cancellation": {
                    "input_id": WORLD_ROUTE_WORK_CANCEL_INPUT_ID,
                    "textarea_id": WORLD_ROUTE_WORK_CANCEL_TEXTAREA_ID,
                },
            }),
        ),
        (
            "handoff".to_string(),
            json!({
                "storage_key": "trillionnium-world-handoff",
                "saved_at_epoch": "saved_at_epoch",
                "action_label": "action_label",
                "action_id": "action_id",
                "command": "command",
                "location_id": "location_id",
                "node_id": "node_id",
                "panel_id": "web_panel_id",
                "action_body": "web_action_body",
                "target_input_id": "web_target_input_id",
                "target_value": "web_target_value",
                "target_textarea_id": "web_target_textarea_id",
                "move_target": "web_move_target",
                "listing_id": "web_listing_id",
                "work_order_id": "web_work_order_id",
                "contract_id": "web_contract_id",
                "route_task_id": "web_route_task_id",
                "event_id": "web_event_id",
                "event_kind": "web_event_kind",
                "event_body": "web_event_body",
                "event_result": "web_event_result",
            }),
        ),
    ]))
}

fn contains_cjk_text(value: &str) -> bool {
    value.chars().any(|ch| {
        ('\u{3400}'..='\u{9fff}').contains(&ch) || ('\u{f900}'..='\u{faff}').contains(&ch)
    })
}

fn world_route_has_delivery_anchor(value: &str, lower: &str) -> bool {
    lower.contains("deliver")
        || lower.contains("customer")
        || value.contains("客户")
        || value.contains("交付")
        || value.contains("方案")
}

fn world_route_has_evidence_anchor(value: &str, lower: &str) -> bool {
    lower.contains("evidence")
        || lower.contains("source")
        || lower.contains("data")
        || value.contains("证据")
        || value.contains("依据")
}

fn world_route_has_risk_anchor(value: &str, lower: &str) -> bool {
    lower.contains("risk") || value.contains("风险")
}

fn world_route_has_next_anchor(value: &str, lower: &str) -> bool {
    lower.contains("next") || value.contains("下一步") || value.contains("计划")
}

fn world_route_has_review_anchor(value: &str, lower: &str) -> bool {
    lower.contains("review")
        || lower.contains("self-check")
        || lower.contains("self check")
        || value.contains("自评")
        || value.contains("自检")
        || value.contains("复盘")
}

pub fn world_route_playability_anchor_body(body: String) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let lower = trimmed.to_ascii_lowercase();
    let has_cjk = contains_cjk_text(trimmed);
    let mut missing_en = Vec::new();
    let mut missing_zh = Vec::new();
    if !world_route_has_delivery_anchor(trimmed, &lower) {
        missing_en.push("customer deliverable");
        missing_zh.push("客户交付方案");
    }
    if !world_route_has_evidence_anchor(trimmed, &lower) {
        missing_en.push("evidence package");
        missing_zh.push("证据包");
    }
    if !world_route_has_risk_anchor(trimmed, &lower) {
        missing_en.push("risk controls");
        missing_zh.push("风险控制");
    }
    if !world_route_has_next_anchor(trimmed, &lower) {
        missing_en.push("next action");
        missing_zh.push("下一步行动");
    }
    if !world_route_has_review_anchor(trimmed, &lower) {
        missing_en.push("self-review");
        missing_zh.push("自检复盘");
    }
    if missing_en.is_empty() {
        return trimmed.to_string();
    }

    let ends_sentence = trimmed
        .chars()
        .last()
        .map(|ch| matches!(ch, '.' | '。' | '!' | '！' | '?' | '？'))
        .unwrap_or(false);
    let separator = if ends_sentence {
        " "
    } else if has_cjk {
        "；"
    } else {
        "; "
    };
    if has_cjk {
        format!("{}{}补齐{}。", trimmed, separator, missing_zh.join("、"))
    } else {
        format!("{}{}add {}.", trimmed, separator, missing_en.join(", "))
    }
}

pub fn world_route_playability_anchor_command(command: String) -> String {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    for prefix in [
        "/upgrade latest",
        "/company latest",
        "/sell latest",
        "/buy latest",
        "/work deliver latest",
        "/work accept latest",
        "/work reject latest",
        "/work reopen latest",
        "/work cancel latest",
        "/world action",
        "/contract",
    ] {
        if let Some(body) = trimmed.strip_prefix(prefix) {
            let body = world_route_playability_anchor_body(body.trim().to_string());
            return if body.is_empty() {
                trimmed.to_string()
            } else {
                format!("{prefix} {body}")
            };
        }
    }
    if let Some(rest) = trimmed.strip_prefix("/complete ") {
        let rest = rest.trim();
        let (contract_id, body) = rest
            .split_once(' ')
            .map(|(contract_id, body)| (contract_id.trim(), body.trim()))
            .unwrap_or((rest, ""));
        let body = world_route_playability_anchor_body(body.to_string());
        return if body.is_empty() {
            trimmed.to_string()
        } else {
            format!("/complete {contract_id} {body}")
        };
    }
    trimmed.to_string()
}

fn is_world_work_lane_input(input_id: &str) -> bool {
    WORLD_ROUTE_WORK_LANE_INPUT_IDS.contains(&input_id)
}

pub fn world_route_focus_lane_spec(panel_id: &str, input_id: &str) -> WorldRouteFocusLaneSpec {
    let (preferred_node_id, desired_tags): (Option<&str>, &[&str]) = match (panel_id, input_id) {
        (WORLD_ROUTE_ASSETS_PANEL_ID, _) => {
            (Some("asset-yard"), &["asset", "upgrade", "inventory"])
        }
        (WORLD_ROUTE_COMPANIES_PANEL_ID, _) => {
            (Some("starter-studio"), &["company", "craft", "decorate"])
        }
        (WORLD_ROUTE_LISTINGS_PANEL_ID, _) => (
            Some("client-board"),
            &["listing", "brief", "market", "sell"],
        ),
        (WORLD_ROUTE_COMMERCE_PANEL_ID, WORLD_ROUTE_PURCHASE_INPUT_ID) => (
            Some("zbj-market-gate"),
            &["market", "buy", "sell", "listing"],
        ),
        (WORLD_ROUTE_COMMERCE_PANEL_ID, input_id) if is_world_work_lane_input(input_id) => (
            Some("delivery-dock"),
            &["deliver", "accept", "reject", "cancel", "refund", "review"],
        ),
        (WORLD_ROUTE_CONTRACTS_PANEL_ID, _) => {
            (Some("ledger-office"), &["contract", "ledger", "refund"])
        }
        _ => (None, &[]),
    };
    WorldRouteFocusLaneSpec {
        preferred_node_id: preferred_node_id.map(ToString::to_string),
        desired_tags: desired_tags.iter().map(|tag| (*tag).to_string()).collect(),
    }
}

pub fn world_route_action_console_target(
    action_label: &str,
    body: String,
) -> WorldRouteCommandTarget {
    WorldRouteCommandTarget::lane(
        action_label,
        WORLD_ROUTE_ACTION_PANEL_ID,
        "",
        String::new(),
        WORLD_ROUTE_ACTION_TEXTAREA_ID,
        body,
    )
}

pub fn world_route_asset_lane_target(
    action_label: &str,
    asset_id: String,
    body: String,
) -> WorldRouteCommandTarget {
    WorldRouteCommandTarget::lane(
        action_label,
        WORLD_ROUTE_ASSETS_PANEL_ID,
        WORLD_ROUTE_ASSET_INPUT_ID,
        asset_id,
        WORLD_ROUTE_ASSET_TEXTAREA_ID,
        body,
    )
}

pub fn world_route_company_lane_target(
    action_label: &str,
    asset_id: String,
    body: String,
) -> WorldRouteCommandTarget {
    WorldRouteCommandTarget::lane(
        action_label,
        WORLD_ROUTE_COMPANIES_PANEL_ID,
        WORLD_ROUTE_COMPANY_INPUT_ID,
        asset_id,
        WORLD_ROUTE_COMPANY_TEXTAREA_ID,
        body,
    )
}

pub fn world_route_listing_lane_target(
    action_label: &str,
    company_id: String,
    body: String,
) -> WorldRouteCommandTarget {
    WorldRouteCommandTarget::lane(
        action_label,
        WORLD_ROUTE_LISTINGS_PANEL_ID,
        WORLD_ROUTE_LISTING_INPUT_ID,
        company_id,
        WORLD_ROUTE_LISTING_TEXTAREA_ID,
        body,
    )
}

pub fn world_route_purchase_lane_target(
    action_label: &str,
    listing_id: String,
    body: String,
) -> WorldRouteCommandTarget {
    WorldRouteCommandTarget::lane(
        action_label,
        WORLD_ROUTE_COMMERCE_PANEL_ID,
        WORLD_ROUTE_PURCHASE_INPUT_ID,
        listing_id,
        WORLD_ROUTE_PURCHASE_TEXTAREA_ID,
        body,
    )
}

fn world_route_work_lane_ids(lane_kind: &str) -> (&'static str, &'static str) {
    match lane_kind {
        "acceptance" => (
            WORLD_ROUTE_WORK_ACCEPT_INPUT_ID,
            WORLD_ROUTE_WORK_ACCEPT_TEXTAREA_ID,
        ),
        "rejection" => (
            WORLD_ROUTE_WORK_REJECT_INPUT_ID,
            WORLD_ROUTE_WORK_REJECT_TEXTAREA_ID,
        ),
        "reopen" => (
            WORLD_ROUTE_WORK_REOPEN_INPUT_ID,
            WORLD_ROUTE_WORK_REOPEN_TEXTAREA_ID,
        ),
        "cancellation" => (
            WORLD_ROUTE_WORK_CANCEL_INPUT_ID,
            WORLD_ROUTE_WORK_CANCEL_TEXTAREA_ID,
        ),
        _ => (
            WORLD_ROUTE_WORK_DELIVER_INPUT_ID,
            WORLD_ROUTE_WORK_DELIVER_TEXTAREA_ID,
        ),
    }
}

pub fn world_route_work_lane_target(
    action_label: &str,
    input_id: &str,
    textarea_id: &str,
    body: String,
) -> WorldRouteCommandTarget {
    WorldRouteCommandTarget::lane(
        action_label,
        WORLD_ROUTE_COMMERCE_PANEL_ID,
        input_id,
        "latest".to_string(),
        textarea_id,
        body,
    )
}

pub fn world_route_work_lane_target_by_kind(
    action_label: &str,
    lane_kind: &str,
    body: String,
) -> WorldRouteCommandTarget {
    let (input_id, textarea_id) = world_route_work_lane_ids(lane_kind);
    world_route_work_lane_target(action_label, input_id, textarea_id, body)
}

pub fn world_route_contract_lane_target(
    action_label: &str,
    contract_id: String,
    body: String,
) -> WorldRouteCommandTarget {
    WorldRouteCommandTarget::lane(
        action_label,
        WORLD_ROUTE_CONTRACTS_PANEL_ID,
        WORLD_ROUTE_CONTRACT_INPUT_ID,
        contract_id,
        WORLD_ROUTE_CONTRACT_TEXTAREA_ID,
        body,
    )
}

type WorldRouteCommandTargetBuilder = fn(String) -> WorldRouteCommandTarget;

fn world_route_upgrade_latest_command_target(body: String) -> WorldRouteCommandTarget {
    world_route_asset_lane_target("打开道具升级路线", "latest".to_string(), body)
}

fn world_route_company_latest_command_target(body: String) -> WorldRouteCommandTarget {
    world_route_company_lane_target("打开工坊路线", "latest".to_string(), body)
}

fn world_route_sell_latest_command_target(body: String) -> WorldRouteCommandTarget {
    world_route_listing_lane_target("打开任务牌路线", "latest".to_string(), body)
}

fn world_route_buy_latest_command_target(body: String) -> WorldRouteCommandTarget {
    world_route_purchase_lane_target("打开接取路线", "latest".to_string(), body)
}

fn world_route_work_deliver_latest_command_target(body: String) -> WorldRouteCommandTarget {
    world_route_work_lane_target_by_kind("打开成果提交路线", "delivery", body)
}

fn world_route_work_accept_latest_command_target(body: String) -> WorldRouteCommandTarget {
    world_route_work_lane_target_by_kind("打开评级路线", "acceptance", body)
}

fn world_route_work_reject_latest_command_target(body: String) -> WorldRouteCommandTarget {
    world_route_work_lane_target_by_kind("打开返工路线", "rejection", body)
}

fn world_route_work_reopen_latest_command_target(body: String) -> WorldRouteCommandTarget {
    world_route_work_lane_target_by_kind("打开重开路线", "reopen", body)
}

fn world_route_work_cancel_latest_command_target(body: String) -> WorldRouteCommandTarget {
    world_route_work_lane_target_by_kind("打开放弃路线", "cancellation", body)
}

pub const WORLD_ROUTE_PREFIX_COMMAND_BUILDERS: &[(&str, WorldRouteCommandTargetBuilder)] = &[
    ("/upgrade latest", world_route_upgrade_latest_command_target),
    ("/company latest", world_route_company_latest_command_target),
    ("/sell latest", world_route_sell_latest_command_target),
    ("/buy latest", world_route_buy_latest_command_target),
    (
        "/work deliver latest",
        world_route_work_deliver_latest_command_target,
    ),
    (
        "/work accept latest",
        world_route_work_accept_latest_command_target,
    ),
    (
        "/work reject latest",
        world_route_work_reject_latest_command_target,
    ),
    (
        "/work reopen latest",
        world_route_work_reopen_latest_command_target,
    ),
    (
        "/work cancel latest",
        world_route_work_cancel_latest_command_target,
    ),
];

fn finalize_world_route_command_target(
    mut target: WorldRouteCommandTarget,
    fallback_body: &str,
) -> WorldRouteCommandTarget {
    if target.body.trim().is_empty() {
        target.body = fallback_body.trim().to_string();
    }
    target
}

pub fn world_route_command_target(command: &str) -> WorldRouteCommandTarget {
    let trimmed = command.trim();
    let strip_body = |prefix: &str| {
        trimmed
            .strip_prefix(prefix)
            .map(|body| body.trim().to_string())
    };

    for (prefix, builder) in WORLD_ROUTE_PREFIX_COMMAND_BUILDERS {
        if let Some(body) = strip_body(prefix) {
            return finalize_world_route_command_target(builder(body), trimmed);
        }
    }

    if let Some(body) = strip_body("/world action") {
        return finalize_world_route_command_target(
            world_route_action_console_target("打开世界行动路线", body),
            trimmed,
        );
    }

    if let Some(body) = strip_body("/contract") {
        return finalize_world_route_command_target(
            world_route_action_console_target("打开契约捕捉路线", body),
            trimmed,
        );
    }

    if let Some(rest) = trimmed.strip_prefix("/complete ") {
        let rest = rest.trim();
        let (contract_id, body) = rest
            .split_once(' ')
            .map(|(contract_id, body)| (contract_id.trim(), body.trim().to_string()))
            .unwrap_or((rest, String::new()));
        return finalize_world_route_command_target(
            world_route_contract_lane_target("打开契约完成路线", contract_id.to_string(), body),
            trimmed,
        );
    }

    finalize_world_route_command_target(
        world_route_action_console_target("打开世界行动路线", trimmed.to_string()),
        trimmed,
    )
}

pub fn world_route_recommendation_score(item: &Value) -> (i64, Value) {
    let bucket = item
        .get("route_bucket")
        .and_then(Value::as_str)
        .unwrap_or("event");
    let status = item
        .get("route_status")
        .and_then(Value::as_str)
        .unwrap_or("pending")
        .to_ascii_lowercase();
    let summary = item
        .get("summary")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_lowercase();
    let detail = item
        .get("detail")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_lowercase();
    let has_location = item
        .get("location_id")
        .and_then(Value::as_str)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);
    let has_task = item
        .get("task_id")
        .and_then(Value::as_str)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
        || item.get("work_order_id").and_then(Value::as_str).is_some();
    let active_route_relevance = match bucket {
        "tactics_objective" => 38,
        "work_order" | "delivery" | "contract" => 35,
        "purchase" | "event" => 28,
        "completion" | "acceptance" => 20,
        _ => 16,
    } + if has_task { 5 } else { 0 };
    let seller_completion_quality = if matches!(bucket, "acceptance" | "completion")
        || status.contains("completed")
        || status.contains("accepted")
        || status.contains("settled")
    {
        25
    } else if matches!(bucket, "delivery" | "work_order" | "contract") {
        18
    } else {
        10
    };
    let low_dispute_risk = if matches!(bucket, "rejection" | "reopen" | "cancellation")
        || status.contains("refund")
        || status.contains("rejected")
        || status.contains("cancel")
    {
        4
    } else if summary.contains("risk") || detail.contains("risk") || summary.contains("evidence") {
        18
    } else {
        14
    };
    let reward_to_next_route_lift = if bucket == "tactics_objective" && status.contains("settled") {
        18
    } else if status.contains("reward")
        || status.contains("claim")
        || matches!(bucket, "acceptance" | "completion")
    {
        15
    } else if matches!(bucket, "delivery" | "work_order") {
        11
    } else {
        7
    };
    let geographic_nearness = if has_location { 5 } else { 0 };
    let score = active_route_relevance
        + seller_completion_quality
        + low_dispute_risk
        + reward_to_next_route_lift
        + geographic_nearness;
    (
        score,
        json!({
            "active_route_relevance": active_route_relevance,
            "seller_completion_quality": seller_completion_quality,
            "low_dispute_risk": low_dispute_risk,
            "reward_to_next_route_lift": reward_to_next_route_lift,
            "geographic_nearness": geographic_nearness,
            "suppressed_for_dispute_risk": low_dispute_risk < 10,
            "policy": "prefer completable, low-risk, evidence-clear routes before raw recency or map density"
        }),
    )
}

pub fn recommendation_ranked_preview_item(mut item: Value) -> Value {
    let (score, reasons) = world_route_recommendation_score(&item);
    if let Some(object) = item.as_object_mut() {
        object.insert("route_recommendation_score".to_string(), json!(score));
        object.insert(
            "route_recommendation_policy_contract_version".to_string(),
            json!(TRILLIONNIUM_WORLD_ROUTE_RECOMMENDATION_POLICY_CONTRACT_VERSION),
        );
        object.insert("route_recommendation_reasons".to_string(), reasons);
        object.insert(
            "route_recommendation_ranker".to_string(),
            json!("commercial_quality_weighted_route_ranker_v1"),
        );
    }
    item
}

impl WorldRoutePreviewItem {
    pub fn from_value(item: &Value) -> Self {
        Self {
            route_bucket: item
                .get("route_bucket")
                .and_then(Value::as_str)
                .unwrap_or("route")
                .to_string(),
            location_id: item
                .get("location_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            task_id: item
                .get("task_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            route_status: item
                .get("route_status")
                .and_then(Value::as_str)
                .unwrap_or("pending")
                .to_string(),
            created_at_epoch: item
                .get("created_at_epoch")
                .and_then(Value::as_i64)
                .unwrap_or(0),
            event_id: item
                .get("event_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            contract_id: item
                .get("contract_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            completion_id: item
                .get("completion_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            purchase_id: item
                .get("purchase_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            listing_id: item
                .get("listing_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            work_order_id: item
                .get("work_order_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            title: item
                .get("title")
                .and_then(Value::as_str)
                .unwrap_or("路线记录")
                .to_string(),
            summary: item
                .get("summary")
                .and_then(Value::as_str)
                .unwrap_or("waiting")
                .to_string(),
            detail: item
                .get("detail")
                .and_then(Value::as_str)
                .unwrap_or("world flow")
                .to_string(),
        }
    }

    pub fn task_group_id(&self) -> String {
        let candidates = [
            self.task_id.as_str(),
            self.work_order_id.as_str(),
            self.purchase_id.as_str(),
            self.contract_id.as_str(),
            self.listing_id.as_str(),
            self.completion_id.as_str(),
            self.event_id.as_str(),
        ];
        candidates
            .into_iter()
            .find(|value| !value.trim().is_empty())
            .unwrap_or("")
            .to_string()
    }
}

pub fn world_route_preview_items(route_preview: &Value) -> Vec<WorldRoutePreviewItem> {
    route_preview
        .get("items")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|item| WorldRoutePreviewItem::from_value(&item))
        .collect()
}

pub fn world_route_preview_json(raw_items: Vec<Value>) -> Value {
    let mut items = raw_items
        .into_iter()
        .map(recommendation_ranked_preview_item)
        .collect::<Vec<_>>();

    items.sort_by(|left, right| {
        right
            .get("route_recommendation_score")
            .and_then(Value::as_i64)
            .cmp(
                &left
                    .get("route_recommendation_score")
                    .and_then(Value::as_i64),
            )
            .then_with(|| {
                right
                    .get("created_at_epoch")
                    .and_then(Value::as_i64)
                    .cmp(&left.get("created_at_epoch").and_then(Value::as_i64))
            })
    });
    items.truncate(24);

    let task_linked_count = items
        .iter()
        .filter(|item| {
            item.get("task_id")
                .and_then(Value::as_str)
                .map(|task_id| !task_id.trim().is_empty())
                .unwrap_or(false)
        })
        .count();

    json!({
        "projection_layer": "world_route_projection_v1",
        "projection_context": "WorldRouteProjectionContext",
        "index_layer": "WorldIndexes::recent_route_indices_v1",
        "ranker": "commercial_quality_weighted_route_ranker_v1",
        "ranker_contract_version": TRILLIONNIUM_WORLD_ROUTE_RECOMMENDATION_POLICY_CONTRACT_VERSION,
        "ranker_inputs": ["active_route_relevance", "seller_completion_quality", "low_dispute_risk", "reward_to_next_route_lift", "geographic_nearness"],
        "item_count": items.len(),
        "task_linked_count": task_linked_count,
        "items": items,
    })
}

fn default_route_focus_node_id(latest_location_id: &str, panel_id: &str, input_id: &str) -> String {
    world_route_focus_lane_spec(panel_id, input_id)
        .preferred_node_id
        .filter(|node_id| !node_id.trim().is_empty())
        .unwrap_or_else(|| latest_location_id.to_string())
}

pub fn world_route_task_graph_item_with_focus_resolver<F>(
    task_id: String,
    mut items: Vec<WorldRoutePreviewItem>,
    focus_resolver: F,
) -> Value
where
    F: Fn(&str, &str, &str) -> String,
{
    items.sort_by_key(|item| std::cmp::Reverse(item.created_at_epoch));
    let latest = items.first().cloned();
    let latest_bucket = latest
        .as_ref()
        .map(|item| item.route_bucket.as_str())
        .unwrap_or("event");
    let latest_status = latest
        .as_ref()
        .map(|item| item.route_status.as_str())
        .unwrap_or("pending");
    let latest_location_id = latest
        .as_ref()
        .map(|item| item.location_id.as_str())
        .unwrap_or("");
    let latest_created_at_epoch = latest
        .as_ref()
        .map(|item| item.created_at_epoch)
        .unwrap_or(0);
    let event_count = items
        .iter()
        .filter(|item| item.route_bucket == "event")
        .count();
    let contract_count = items
        .iter()
        .filter(|item| item.route_bucket == "contract")
        .count();
    let completion_count = items
        .iter()
        .filter(|item| item.route_bucket == "completion")
        .count();
    let latest_event = items.iter().find(|item| item.route_bucket == "event");
    let latest_contract = items.iter().find(|item| item.route_bucket == "contract");
    let latest_completion = items.iter().find(|item| item.route_bucket == "completion");
    let latest_event_id = latest_event
        .map(|item| item.event_id.as_str())
        .unwrap_or("");
    let latest_event_title = latest_event
        .map(|item| item.title.as_str())
        .unwrap_or("world_event");
    let latest_contract_id = latest_contract
        .map(|item| item.contract_id.as_str())
        .unwrap_or("");
    let latest_contract_title = latest_contract
        .map(|item| item.title.as_str())
        .unwrap_or("世界契约");
    let latest_completion_id = latest_completion
        .map(|item| item.completion_id.as_str())
        .unwrap_or("");
    let latest_completion_title = latest_completion
        .map(|item| item.title.as_str())
        .unwrap_or("契约战报");
    let latest_title = latest
        .as_ref()
        .map(|item| item.title.as_str())
        .unwrap_or(latest_bucket);
    let latest_summary = latest
        .as_ref()
        .map(|item| item.summary.as_str())
        .unwrap_or("");
    let latest_detail = latest
        .as_ref()
        .map(|item| item.detail.as_str())
        .unwrap_or("");
    let route_stage_summary = format!(
        "{} 个事件 → {} 份契约 → {} 条战报 · 最新 {}/{}",
        event_count, contract_count, completion_count, latest_bucket, latest_status
    );
    let descriptor = WorldRouteTaskDescriptor {
        task_id: task_id.clone(),
        latest_bucket: latest_bucket.to_string(),
        latest_status: latest_status.to_string(),
        latest_location_id: latest_location_id.to_string(),
        latest_title: latest_title.to_string(),
        latest_summary: latest_summary.to_string(),
        latest_detail: latest_detail.to_string(),
        latest_event_title: latest_event_title.to_string(),
        latest_contract_id: latest_contract_id.to_string(),
        latest_contract_title: latest_contract_title.to_string(),
        latest_completion_id: latest_completion_id.to_string(),
        latest_completion_title: latest_completion_title.to_string(),
    };
    let outcome_summary = descriptor.outcome_summary();
    let feedback_focus = descriptor.feedback_focus();
    let next_opportunity = descriptor.next_opportunity();
    let next_opportunity_target = next_opportunity.route_target;
    let next_opportunity_node_id = focus_resolver(
        latest_location_id,
        &next_opportunity_target.panel_id,
        &next_opportunity_target.input_id,
    );
    let suggested_action = descriptor.suggested_action();
    let suggested_target = suggested_action.route_target;
    let suggested_node_id = focus_resolver(
        latest_location_id,
        &suggested_target.panel_id,
        &suggested_target.input_id,
    );

    json!({
        "task_id": task_id,
        "latest_bucket": latest_bucket,
        "latest_status": latest_status,
        "latest_location_id": latest_location_id,
        "latest_created_at_epoch": latest_created_at_epoch,
        "event_count": event_count,
        "contract_count": contract_count,
        "completion_count": completion_count,
        "latest_event_id": latest_event_id,
        "latest_event_title": latest_event_title,
        "latest_contract_id": latest_contract_id,
        "latest_contract_title": latest_contract_title,
        "latest_completion_id": latest_completion_id,
        "latest_completion_title": latest_completion_title,
        "route_stage_summary": route_stage_summary,
        "outcome_summary": outcome_summary,
        "feedback_focus": feedback_focus,
        "next_opportunity_hint": next_opportunity.hint,
        "next_opportunity_kind": next_opportunity.kind,
        "next_opportunity_playbook": next_opportunity.playbook,
        "next_opportunity_command": next_opportunity.command,
        "next_opportunity_action_label": next_opportunity_target.action_label,
        "next_opportunity_panel_id": next_opportunity_target.panel_id,
        "next_opportunity_input_id": next_opportunity_target.input_id,
        "next_opportunity_input_value": next_opportunity_target.input_value,
        "next_opportunity_textarea_id": next_opportunity_target.textarea_id,
        "next_opportunity_body": next_opportunity_target.body,
        "next_opportunity_node_id": next_opportunity_node_id,
        "suggested_action_label": suggested_target.action_label,
        "suggested_panel_id": suggested_target.panel_id,
        "suggested_input_id": suggested_target.input_id,
        "suggested_input_value": suggested_target.input_value,
        "suggested_textarea_id": suggested_target.textarea_id,
        "suggested_body": suggested_target.body,
        "suggested_matrix_command": suggested_action.matrix_command,
        "suggested_node_id": suggested_node_id,
    })
}

pub fn world_route_task_graph_item(task_id: String, items: Vec<WorldRoutePreviewItem>) -> Value {
    world_route_task_graph_item_with_focus_resolver(task_id, items, default_route_focus_node_id)
}

pub fn world_route_task_graph_items(preview: &Value) -> Vec<Value> {
    let mut tasks: HashMap<String, Vec<WorldRoutePreviewItem>> = HashMap::new();
    for item in world_route_preview_items(preview) {
        let task_id = item.task_group_id();
        if task_id.is_empty() {
            continue;
        }
        tasks.entry(task_id).or_default().push(item);
    }

    tasks
        .into_iter()
        .map(|(task_id, items)| world_route_task_graph_item(task_id, items))
        .collect()
}

pub fn world_route_task_graph_json(preview: &Value) -> Value {
    let mut graph_tasks = world_route_task_graph_items(preview);

    graph_tasks.sort_by(|left, right| {
        right
            .get("latest_created_at_epoch")
            .and_then(Value::as_i64)
            .cmp(&left.get("latest_created_at_epoch").and_then(Value::as_i64))
    });

    json!({
        "projection_layer": "world_route_task_graph_projection_v1",
        "projection_context": "WorldRouteProjectionContext",
        "task_count": graph_tasks.len(),
        "tasks": graph_tasks,
    })
}

impl WorldRouteTaskGraphView {
    pub fn from_value(task: &Value) -> Self {
        Self {
            task_id: task
                .get("task_id")
                .and_then(Value::as_str)
                .unwrap_or("task")
                .to_string(),
            latest_bucket: task
                .get("latest_bucket")
                .and_then(Value::as_str)
                .unwrap_or("event")
                .to_string(),
            latest_status: task
                .get("latest_status")
                .and_then(Value::as_str)
                .unwrap_or("pending")
                .to_string(),
            latest_location_id: task
                .get("latest_location_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            event_count: task.get("event_count").and_then(Value::as_u64).unwrap_or(0),
            contract_count: task
                .get("contract_count")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            completion_count: task
                .get("completion_count")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            latest_contract_id: task
                .get("latest_contract_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            latest_created_at_epoch: task
                .get("latest_created_at_epoch")
                .and_then(Value::as_i64)
                .unwrap_or(0),
            route_stage_summary: task
                .get("route_stage_summary")
                .and_then(Value::as_str)
                .unwrap_or("路线摘要整理中。")
                .to_string(),
            outcome_summary: task
                .get("outcome_summary")
                .and_then(Value::as_str)
                .unwrap_or("结果摘要整理中。")
                .to_string(),
            feedback_focus: task
                .get("feedback_focus")
                .and_then(Value::as_str)
                .unwrap_or("证据和下一步整理中。")
                .to_string(),
            next_opportunity_kind: task
                .get("next_opportunity_kind")
                .and_then(Value::as_str)
                .unwrap_or("contract_capture")
                .to_string(),
            next_opportunity_hint: task
                .get("next_opportunity_hint")
                .and_then(Value::as_str)
                .unwrap_or("Opportunity hint pending.")
                .to_string(),
            next_opportunity_playbook: task
                .get("next_opportunity_playbook")
                .and_then(Value::as_str)
                .unwrap_or("Opportunity playbook pending.")
                .to_string(),
            next_opportunity_command: world_route_playability_anchor_command(
                task.get("next_opportunity_command")
                    .and_then(Value::as_str)
                    .unwrap_or("/world action 继续推进下一步机会。")
                    .to_string(),
            ),
            next_opportunity_action_label: task
                .get("next_opportunity_action_label")
                .and_then(Value::as_str)
                .unwrap_or("推进下一条支线")
                .to_string(),
            next_opportunity_panel_id: task
                .get("next_opportunity_panel_id")
                .and_then(Value::as_str)
                .unwrap_or(WORLD_ROUTE_ACTION_PANEL_ID)
                .to_string(),
            next_opportunity_input_id: task
                .get("next_opportunity_input_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            next_opportunity_input_value: task
                .get("next_opportunity_input_value")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            next_opportunity_textarea_id: task
                .get("next_opportunity_textarea_id")
                .and_then(Value::as_str)
                .unwrap_or(WORLD_ROUTE_ACTION_TEXTAREA_ID)
                .to_string(),
            next_opportunity_node_id: task
                .get("next_opportunity_node_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            next_opportunity_body: world_route_playability_anchor_body(
                task.get("next_opportunity_body")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
            ),
            suggested_action_label: task
                .get("suggested_action_label")
                .and_then(Value::as_str)
                .unwrap_or("起草任务后续")
                .to_string(),
            suggested_panel_id: task
                .get("suggested_panel_id")
                .and_then(Value::as_str)
                .unwrap_or(WORLD_ROUTE_ACTION_PANEL_ID)
                .to_string(),
            suggested_input_id: task
                .get("suggested_input_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            suggested_input_value: task
                .get("suggested_input_value")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            suggested_textarea_id: task
                .get("suggested_textarea_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            suggested_matrix_command: world_route_playability_anchor_command(
                task.get("suggested_matrix_command")
                    .and_then(Value::as_str)
                    .unwrap_or("/world action 跟进当前任务并记录证据、阻塞和下一步。")
                    .to_string(),
            ),
            suggested_node_id: task
                .get("suggested_node_id")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            suggested_body: world_route_playability_anchor_body(
                task.get("suggested_body")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
            ),
        }
    }

    pub fn matches_focus(&self, task_id: Option<&str>, location_id: Option<&str>) -> bool {
        let task_id = task_id.map(str::trim).filter(|value| !value.is_empty());
        let location_id = location_id.map(str::trim).filter(|value| !value.is_empty());
        if let Some(task_id) = task_id {
            return self.task_id == task_id;
        }
        if let Some(location_id) = location_id {
            return self.latest_location_id.is_empty() || self.latest_location_id == location_id;
        }
        true
    }

    pub fn route_flow_status_text(&self) -> String {
        format!(
            "Adventure route: task {} · {}/{} · branch {}.",
            self.task_id, self.latest_bucket, self.latest_status, self.next_opportunity_kind
        )
    }

    pub fn route_next_step_status_text(&self) -> String {
        if self.next_opportunity_hint.trim().is_empty() {
            format!(
                "Recommended next step: {}.",
                self.next_opportunity_action_label
            )
        } else {
            self.next_opportunity_hint.clone()
        }
    }

    pub fn route_link_status_text(&self) -> String {
        format!(
            "Linked task route: task {} · {} events · {} contracts · {} battle reports.",
            self.task_id, self.event_count, self.contract_count, self.completion_count
        )
    }

    pub fn action_console_status_text(&self) -> String {
        format!(
            "Current world action: task {} · {} · contract {} · next step {}.",
            self.task_id,
            if self.latest_location_id.is_empty() {
                "route"
            } else {
                &self.latest_location_id
            },
            if self.latest_contract_id.is_empty() {
                "no contract yet"
            } else {
                &self.latest_contract_id
            },
            self.next_opportunity_action_label
        )
    }

    pub fn resolved_suggested_input_id(&self) -> &str {
        if !self.suggested_input_id.is_empty() {
            &self.suggested_input_id
        } else if self.suggested_panel_id == WORLD_ROUTE_CONTRACTS_PANEL_ID {
            WORLD_ROUTE_CONTRACT_INPUT_ID
        } else {
            ""
        }
    }

    pub fn resolved_suggested_input_value(&self) -> &str {
        if !self.suggested_input_value.is_empty() {
            &self.suggested_input_value
        } else if self.resolved_suggested_input_id() == WORLD_ROUTE_CONTRACT_INPUT_ID {
            &self.latest_contract_id
        } else {
            ""
        }
    }

    pub fn resolved_suggested_textarea_id(&self) -> &str {
        if !self.suggested_textarea_id.is_empty() {
            &self.suggested_textarea_id
        } else if self.suggested_panel_id == WORLD_ROUTE_CONTRACTS_PANEL_ID {
            WORLD_ROUTE_CONTRACT_TEXTAREA_ID
        } else {
            WORLD_ROUTE_ACTION_TEXTAREA_ID
        }
    }

    pub fn opportunity_body(&self) -> &str {
        if self.next_opportunity_body.is_empty() {
            &self.next_opportunity_command
        } else {
            &self.next_opportunity_body
        }
    }

    pub fn to_feed_item(&self) -> Value {
        json!({
            "feed_kind": "route_task",
            "source": "route_task_graph",
            "task_id": &self.task_id,
            "location_id": &self.latest_location_id,
            "title": format!("Task {}", &self.task_id),
            "summary": &self.route_stage_summary,
            "detail": format!("latest {}/{}", &self.latest_bucket, &self.latest_status),
            "event_count": self.event_count,
            "contract_count": self.contract_count,
            "completion_count": self.completion_count,
            "latest_contract_id": &self.latest_contract_id,
            "feedback_focus": &self.feedback_focus,
            "next_opportunity_kind": &self.next_opportunity_kind,
            "next_opportunity_hint": &self.next_opportunity_hint,
            "next_opportunity_playbook": &self.next_opportunity_playbook,
            "next_opportunity_command": &self.next_opportunity_command,
            "next_opportunity_action_label": &self.next_opportunity_action_label,
            "next_opportunity_panel_id": &self.next_opportunity_panel_id,
            "next_opportunity_input_id": &self.next_opportunity_input_id,
            "next_opportunity_input_value": &self.next_opportunity_input_value,
            "next_opportunity_textarea_id": &self.next_opportunity_textarea_id,
            "next_opportunity_node_id": &self.next_opportunity_node_id,
            "next_opportunity_body": &self.next_opportunity_body,
            "suggested_action_label": &self.suggested_action_label,
            "suggested_panel_id": &self.suggested_panel_id,
            "suggested_matrix_command": &self.suggested_matrix_command,
            "suggested_node_id": &self.suggested_node_id,
            "suggested_body": &self.suggested_body,
            "created_at_epoch": self.latest_created_at_epoch,
        })
    }
}

pub fn world_route_task_graph_views(
    route_task_graph: &Value,
    limit: usize,
) -> Vec<WorldRouteTaskGraphView> {
    route_task_graph
        .get("tasks")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .take(limit)
        .map(|task| WorldRouteTaskGraphView::from_value(&task))
        .collect()
}

impl WorldRouteStoryView {
    pub fn from_route_data(
        route_preview: &Value,
        route_task_graph: &Value,
        task_views: &[WorldRouteTaskGraphView],
    ) -> Self {
        let preview_items = route_preview
            .get("items")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let preview_item_count = route_preview
            .get("item_count")
            .and_then(Value::as_u64)
            .unwrap_or(preview_items.len() as u64);
        let task_linked_count = route_preview
            .get("task_linked_count")
            .and_then(Value::as_u64)
            .unwrap_or_else(|| {
                preview_items
                    .iter()
                    .filter(|item| {
                        item.get("task_id")
                            .and_then(Value::as_str)
                            .map(|task_id| !task_id.trim().is_empty())
                            .unwrap_or(false)
                    })
                    .count() as u64
            });
        let task_graph_count = route_task_graph
            .get("task_count")
            .and_then(Value::as_u64)
            .unwrap_or(task_views.len() as u64);
        let fallback_location_id = preview_items
            .iter()
            .find_map(|item| item.get("location_id").and_then(Value::as_str))
            .unwrap_or("")
            .to_string();
        let fallback_summary = preview_items
            .iter()
            .find_map(|item| item.get("summary").and_then(Value::as_str))
            .unwrap_or("路线摘要整理中。")
            .to_string();
        let fallback_target = world_route_command_target("/world action 继续推进下一步机会。");
        if let Some(task) = task_views.first() {
            return Self {
                preview_item_count,
                task_linked_count,
                task_graph_count,
                next_task_id: task.task_id.clone(),
                next_action_label: task.suggested_action_label.clone(),
                next_panel_id: task.suggested_panel_id.clone(),
                next_command_hint: task.suggested_matrix_command.clone(),
                next_location_id: task.latest_location_id.clone(),
                next_node_id: task.suggested_node_id.clone(),
                next_opportunity_node_id: task.next_opportunity_node_id.clone(),
                next_stage_summary: task.route_stage_summary.clone(),
                next_opportunity_kind: task.next_opportunity_kind.clone(),
                next_outcome_summary: task.outcome_summary.clone(),
                next_feedback_focus: task.feedback_focus.clone(),
                next_opportunity_hint: task.next_opportunity_hint.clone(),
                next_opportunity_playbook: task.next_opportunity_playbook.clone(),
                next_opportunity_command: task.next_opportunity_command.clone(),
                next_opportunity_target: WorldRouteStoryOpportunityTarget {
                    action_label: task.next_opportunity_action_label.clone(),
                    panel_id: task.next_opportunity_panel_id.clone(),
                    input_id: task.next_opportunity_input_id.clone(),
                    input_value: task.next_opportunity_input_value.clone(),
                    textarea_id: task.next_opportunity_textarea_id.clone(),
                    body: task.opportunity_body().to_string(),
                    node_id: task.next_opportunity_node_id.clone(),
                },
            };
        }
        Self {
            preview_item_count,
            task_linked_count,
            task_graph_count,
            next_task_id: String::new(),
            next_action_label: fallback_target.action_label.clone(),
            next_panel_id: fallback_target.panel_id.clone(),
            next_command_hint: world_route_playability_anchor_command(
                "/world action 继续推进下一步机会。".to_string(),
            ),
            next_location_id: fallback_location_id,
            next_node_id: String::new(),
            next_opportunity_node_id: String::new(),
            next_stage_summary: fallback_summary,
            next_opportunity_kind: "contract_capture".to_string(),
            next_outcome_summary: "Outcome summary pending.".to_string(),
            next_feedback_focus: "证据和下一步待补齐。".to_string(),
            next_opportunity_hint: "Opportunity hint pending.".to_string(),
            next_opportunity_playbook: "Opportunity playbook pending.".to_string(),
            next_opportunity_command: world_route_playability_anchor_command(
                "/world action 继续推进下一步机会。".to_string(),
            ),
            next_opportunity_target: WorldRouteStoryOpportunityTarget {
                action_label: fallback_target.action_label,
                panel_id: fallback_target.panel_id,
                input_id: fallback_target.input_id,
                input_value: fallback_target.input_value,
                textarea_id: fallback_target.textarea_id,
                body: fallback_target.body,
                node_id: String::new(),
            },
        }
    }

    pub fn to_value(&self) -> Value {
        json!({
            "preview_item_count": self.preview_item_count,
            "task_linked_count": self.task_linked_count,
            "task_graph_count": self.task_graph_count,
            "next_task_id": &self.next_task_id,
            "next_action_label": &self.next_action_label,
            "next_panel_id": &self.next_panel_id,
            "next_command_hint": &self.next_command_hint,
            "next_location_id": &self.next_location_id,
            "next_node_id": &self.next_node_id,
            "next_opportunity_node_id": &self.next_opportunity_node_id,
            "next_stage_summary": &self.next_stage_summary,
            "next_opportunity_kind": &self.next_opportunity_kind,
            "next_outcome_summary": &self.next_outcome_summary,
            "next_feedback_focus": &self.next_feedback_focus,
            "next_opportunity_hint": &self.next_opportunity_hint,
            "next_opportunity_playbook": &self.next_opportunity_playbook,
            "next_opportunity_command": &self.next_opportunity_command,
            "next_opportunity_target": {
                "action_label": &self.next_opportunity_target.action_label,
                "panel_id": &self.next_opportunity_target.panel_id,
                "input_id": &self.next_opportunity_target.input_id,
                "input_value": &self.next_opportunity_target.input_value,
                "textarea_id": &self.next_opportunity_target.textarea_id,
                "body": &self.next_opportunity_target.body,
                "node_id": &self.next_opportunity_target.node_id,
            },
        })
    }
}

pub fn world_route_artifacts(
    route_preview: Value,
    route_task_graph: Value,
    task_view_limit: usize,
) -> WorldRouteArtifacts {
    let task_views = world_route_task_graph_views(&route_task_graph, task_view_limit);
    let story =
        WorldRouteStoryView::from_route_data(&route_preview, &route_task_graph, &task_views);
    WorldRouteArtifacts {
        preview: route_preview,
        task_graph: route_task_graph,
        task_views,
        story,
    }
}

pub fn world_route_artifacts_from_raw_preview_items(
    raw_items: Vec<Value>,
    task_view_limit: usize,
) -> WorldRouteArtifacts {
    let preview = world_route_preview_json(raw_items);
    let task_graph = world_route_task_graph_json(&preview);
    world_route_artifacts(preview, task_graph, task_view_limit)
}

impl WorldRouteTaskDescriptor {
    fn latest_status_needs_refund_retry(&self) -> bool {
        matches!(
            self.latest_status.as_str(),
            "rejected_refund_hold"
                | "rejected_refund_failed"
                | "rejected_pending_refund"
                | "cancelled_refund_hold"
                | "cancelled_refund_failed"
                | "cancel_pending_refund"
        )
    }

    fn latest_status_needs_chargeback_retry(&self) -> bool {
        matches!(
            self.latest_status.as_str(),
            "rejected_chargeback_failed"
                | "rejected_pending_chargeback"
                | "cancelled_chargeback_failed"
                | "cancel_pending_chargeback"
        )
    }

    fn latest_status_needs_settlement_retry(&self) -> bool {
        self.latest_status_needs_refund_retry() || self.latest_status_needs_chargeback_retry()
    }

    pub fn outcome_summary(&self) -> String {
        match self.latest_bucket.as_str() {
            "completion" if !self.latest_completion_id.is_empty() => format!(
                "{} · {}{}{}",
                self.latest_completion_title,
                self.latest_status,
                if self.latest_detail.is_empty() {
                    ""
                } else {
                    " · "
                },
                self.latest_detail
            ),
            "contract" if !self.latest_contract_id.is_empty() => format!(
                "{} · {}{}{}",
                self.latest_contract_title,
                self.latest_status,
                if self.latest_detail.is_empty() {
                    ""
                } else {
                    " · "
                },
                self.latest_detail
            ),
            _ => format!(
                "{} · {}{}{}",
                self.latest_title,
                self.latest_status,
                if self.latest_detail.is_empty() {
                    ""
                } else {
                    " · "
                },
                self.latest_detail
            ),
        }
    }

    pub fn feedback_focus(&self) -> String {
        match self.latest_bucket.as_str() {
            "completion" => format!(
                "记录 {} 的委托方反馈，并归档最终成果证据、评分和复盘。",
                self.latest_completion_title
            ),
            "acceptance" => format!(
                "记录委托方在 {} 中通过了什么，以及哪些证据提升了信任。",
                self.latest_title
            ),
            "rejection" => format!(
                "记录 {} 的证据缺口、委托方异议和返工范围。",
                self.latest_title
            ),
            "reopen" => format!(
                "记录 {} 为什么重开、评级标准如何变化，以及下一步要修什么。",
                self.latest_title
            ),
            "cancellation" => format!(
                "记录 {} 的放弃原因，以及下次如何提前校准需求来减少流失。",
                self.latest_title
            ),
            "contract" => format!(
                "记录 {} 的成果计划、评级标准和未清风险。",
                self.latest_contract_title
            ),
            "delivery" => format!(
                "记录 {} 的已提交成果、缺失证据和委托方评级线索。",
                self.latest_title
            ),
            "purchase" | "work_order" => {
                format!(
                    "记录 {} 的委托简报、承诺范围和执行风险。",
                    self.latest_title
                )
            }
            _ => format!(
                "记录 {}{}{} 的世界状态证据、阻碍和下一步行动。",
                self.latest_title,
                if self.latest_summary.is_empty() {
                    ""
                } else {
                    " · "
                },
                self.latest_summary
            ),
        }
    }

    pub fn next_opportunity(&self) -> WorldRouteOpportunity {
        let (kind, hint, playbook, command) = match self.latest_bucket.as_str() {
            "completion" => (
                "repeat_order_upsell_referral",
                format!(
                    "把 {} 作为下一条回访委托、升级悬赏或转介绍支线的跳板{}{}。",
                    self.latest_completion_title,
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "归档 {} 的成果证据，索取评价/转介绍，再包装更高价值的回访委托或升级悬赏{}{}。",
                    self.latest_completion_title,
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "/sell latest 回访/升级悬赏：基于 {} 提供下一阶段成果、证据、赏金、评级标准、时间线和推荐理由。",
                    self.latest_completion_title
                ),
            ),
            "acceptance" => (
                "acceptance_upsell",
                format!(
                    "把 {} 转化为下一次协作、评价或高阶悬赏{}{}。",
                    self.latest_title,
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "记录 {} 为什么获得通过，把证据转成评价，再提出更高价值的后续支线{}{}。",
                    self.latest_title,
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "/sell latest 评级后升级悬赏：围绕 {} 输出高阶范围、证据、赏金阶梯、时间线和下一步。",
                    self.latest_title
                ),
            ),
            "rejection" if self.latest_status_needs_chargeback_retry() => (
                "rejection_chargeback_recovery",
                format!(
                    "先恢复卖家扣回/账本清算，再重开修订路线；不要直接提交成果{}{}。",
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "复核 {} 的拒收退款已完成且买家不会二次退款；补足卖家可扣回余额，重试拒收清算，清账后再重开修订。",
                    self.latest_title
                ),
                "/work reject latest 重试拒收卖家扣回：确认买家不二次退款、卖家扣回资金已恢复、账本错误已清理，再决定是否重开修订。".to_string(),
            ),
            "rejection" if self.latest_status_needs_refund_retry() => (
                "rejection_refund_recovery",
                format!(
                    "先恢复买家退款清算，再处理卖家扣回和重开路线；不要直接提交成果{}{}。",
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "复核 {} 的拒收退款阻塞原因，恢复买家预留/退款，再让拒收流程继续到卖家扣回和重开修订。",
                    self.latest_title
                ),
                "/work reject latest 重试拒收退款：确认买家预留资金、退款状态、卖家扣回风险、证据缺口和下一步恢复计划。".to_string(),
            ),
            "rejection" => (
                "revision_reopen",
                format!(
                    "拒收清算完成后，先重开委托，再提交修订成果{}{}。",
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "列出 {} 的异议，确认退款/扣回已清账，重述评级标准，先重开委托，再提交修订成果。",
                    self.latest_title
                ),
                "/work reopen latest 重开修订委托：补齐证据缺口、重述客户交付方案、更新评级标准、风险控制、下一步行动和自检复盘。".to_string(),
            ),
            "reopen" => (
                "reopen_recovery",
                format!(
                    "调整委托方案，收紧评级标准，并重新打开下一条支线{}{}。",
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "用 {} 推动可控的再次提交循环：锁定新评级线，补齐证据缺口，再在通过后打开扩展支线。",
                    self.latest_title
                ),
                "/work deliver latest 重开后修订成果：补齐证据、修复重开要求、更新评级清单和下一步。".to_string(),
            ),
            "cancellation" if self.latest_status_needs_settlement_retry() => (
                "cancellation_settlement_recovery",
                format!(
                    "先恢复取消退款/卖家扣回清算，再重新校准需求；不要直接发布新悬赏{}{}。",
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "复核 {} 的取消清算状态，避免买家二次退款或卖家未扣回，重试取消流程，清账后再缩小范围重新发布。",
                    self.latest_title
                ),
                "/work cancel latest 重试取消清算：确认买家不二次退款、卖家扣回资金已恢复、账本错误已清理、重新校准需求和下一步。".to_string(),
            ),
            "cancellation" => (
                "smaller_scope_requalification",
                format!(
                    "用更小范围或更清晰的委托机会恢复这条路线{}{}。",
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "把 {} 转成重新校准：缩小范围、降低风险、明确证据，再用更小的起步悬赏重启。",
                    self.latest_title
                ),
                "/sell latest 小范围试炼委托：更小范围、明确成果/证据、低风险评级标准、赏金和下一步。".to_string(),
            ),
            "contract" => (
                "delivery_then_upsell",
                format!(
                    "先完成 {}，再打开成果后的跟进支线和下一条悬赏{}{}。",
                    self.latest_contract_title,
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "完成 {}，锁定成果证据和评级标准，再趁上下文新鲜起草下一条支线。",
                    self.latest_contract_title
                ),
                format!(
                    "/complete {} 成果方案：包含成果、证据、风险复盘、评级标准、下一步和自检记录。",
                    self.latest_contract_id
                ),
            ),
            "delivery" => (
                "acceptance_closeout",
                format!(
                    "用 {} 完成评级闭环、沉淀证据，并铺好下一次协作{}{}。",
                    self.latest_title,
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "推动 {} 完成委托方评级：突出证据，快速补齐缺口，在热度消退前埋好下一条支线。",
                    self.latest_title
                ),
                "/work accept latest 评级通过：确认成果、证据、质量、下一次协作和声望奖励。".to_string(),
            ),
            "purchase" | "work_order" => (
                "fulfillment_launch",
                format!(
                    "把 {} 推进成下一份契约、任务牌或世界行动机会{}{}。",
                    self.latest_title,
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "复述 {} 的委托目标，锁定里程碑和证据，尽快提交第一轮成果，让路线进入评级和下一条支线。",
                    self.latest_title
                ),
                "/work deliver latest 首轮成果：成果、证据、评级清单、风险复盘、下一步。".to_string(),
            ),
            _ => (
                "contract_capture",
                format!(
                    "把 {} 转化为下一份契约、任务牌或世界行动机会{}{}。",
                    self.latest_title,
                    if self.latest_location_id.is_empty() { "" } else { " @ " },
                    self.latest_location_id
                ),
                format!(
                    "判断 {} 的可行性，记录证据和风险，再决定转成契约、任务牌或直接世界行动。",
                    self.latest_title
                ),
                format!(
                    "/contract 围绕 {} 整理目标、证据、风险、评级标准和下一步。",
                    self.latest_title
                ),
            ),
        };
        let command = world_route_playability_anchor_command(command);
        let route_target = world_route_command_target(&command);
        let focus_lane =
            world_route_focus_lane_spec(&route_target.panel_id, &route_target.input_id);
        WorldRouteOpportunity {
            kind: kind.to_string(),
            hint,
            playbook,
            command,
            route_target,
            focus_lane,
        }
    }

    pub fn suggested_action(&self) -> WorldRouteSuggestedAction {
        let mut action = if self.latest_bucket == "completion"
            && !self.latest_completion_id.is_empty()
        {
            WorldRouteSuggestedAction {
                route_target: world_route_action_console_target(
                    "起草战报后续",
                    format!(
                        "任务 {}：跟进战报 {}，记录成果证据、委托方反馈和下一条支线。",
                        self.task_id, self.latest_completion_id
                    ),
                ),
                matrix_command: format!(
                    "/world action 跟进已完成任务 {}：围绕 {} 记录成果证据、委托方反馈、复盘和下一条支线。",
                    self.task_id, self.latest_completion_title
                ),
                focus_lane: WorldRouteFocusLaneSpec {
                    preferred_node_id: None,
                    desired_tags: Vec::new(),
                },
            }
        } else if !self.latest_contract_id.is_empty() {
            WorldRouteSuggestedAction {
                route_target: world_route_contract_lane_target(
                    "打开关联契约",
                    self.latest_contract_id.clone(),
                    format!(
                        "任务 {}：完成关联契约 {}，带上证据、评级标准和下一步。",
                        self.task_id, self.latest_contract_id
                    ),
                ),
                matrix_command: format!(
                    "/complete {} 成果方案：包含成果、证据、风险复盘、评级标准、下一步和自检记录。",
                    self.latest_contract_id
                ),
                focus_lane: world_route_focus_lane_spec(
                    WORLD_ROUTE_CONTRACTS_PANEL_ID,
                    WORLD_ROUTE_CONTRACT_INPUT_ID,
                ),
            }
        } else {
            WorldRouteSuggestedAction {
                route_target: world_route_action_console_target(
                    "起草任务后续",
                    format!(
                        "任务 {}：跟进关联事件 {}，记录世界状态证据、阻碍和下一步行动。",
                        self.task_id, self.latest_event_title
                    ),
                ),
                matrix_command: format!(
                    "/world action 跟进任务 {}：记录 {} 的证据、阻塞和下一步。",
                    self.task_id, self.latest_event_title
                ),
                focus_lane: WorldRouteFocusLaneSpec {
                    preferred_node_id: None,
                    desired_tags: Vec::new(),
                },
            }
        };
        action.matrix_command = world_route_playability_anchor_command(action.matrix_command);
        if action.focus_lane.preferred_node_id.is_none() {
            action.focus_lane = world_route_focus_lane_spec(
                &action.route_target.panel_id,
                &action.route_target.input_id,
            );
        }
        action
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorldRouteRecords {
    #[serde(default)]
    pub events: Vec<WorldRouteEventRecord>,
    #[serde(default)]
    pub contracts: Vec<WorldRouteContractRecord>,
    #[serde(default)]
    pub completions: Vec<WorldRouteCompletionRecord>,
    #[serde(default)]
    pub purchases: Vec<WorldRoutePurchaseRecord>,
    #[serde(default)]
    pub work_orders: Vec<WorldRouteWorkOrderRecord>,
    #[serde(default)]
    pub deliveries: Vec<WorldRouteDeliveryRecord>,
    #[serde(default)]
    pub acceptances: Vec<WorldRouteAcceptanceRecord>,
    #[serde(default)]
    pub rejections: Vec<WorldRouteRejectionRecord>,
    #[serde(default)]
    pub reopens: Vec<WorldRouteReopenRecord>,
    #[serde(default)]
    pub cancellations: Vec<WorldRouteCancellationRecord>,
    #[serde(default)]
    pub tactics_sessions: Vec<WorldTacticsGameSession>,
    #[serde(default)]
    pub tactics_ticks: Vec<WorldTacticsSimulationTick>,
    #[serde(default)]
    pub reward_settlements: Vec<WorldTacticsRewardSettlement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteEventRecord {
    pub event_id: String,
    pub event_kind: String,
    pub location_id: String,
    pub task_id: String,
    pub route_status: String,
    pub body: String,
    pub impact_score: i64,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteContractRecord {
    pub contract_id: String,
    pub task_id: String,
    pub title: String,
    pub body: String,
    pub location_id: String,
    pub status: String,
    pub value_score: i64,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldRouteCompletionRecord {
    pub completion_id: String,
    pub contract_id: String,
    pub task_id: String,
    pub location_id: String,
    pub body: String,
    pub ledger_status: Option<String>,
    pub payout_status: String,
    pub score: f64,
    pub reward_amount: f64,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRoutePurchaseRecord {
    pub purchase_id: String,
    pub listing_id: String,
    pub company_id: String,
    pub location_id: String,
    pub status: String,
    pub price_credits: i64,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteWorkOrderRecord {
    pub work_order_id: String,
    pub listing_id: String,
    pub purchase_id: String,
    pub location_id: String,
    pub brief: String,
    pub status: String,
    pub value_score: i64,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldRouteDeliveryRecord {
    pub delivery_id: String,
    pub work_order_id: String,
    pub location_id: String,
    pub body: String,
    pub status: String,
    pub score: f64,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteAcceptanceRecord {
    pub acceptance_id: String,
    pub work_order_id: String,
    pub location_id: String,
    pub body: String,
    pub status: String,
    pub reputation_delta: i64,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteRejectionRecord {
    pub rejection_id: String,
    pub work_order_id: String,
    pub location_id: String,
    pub body: String,
    pub status: String,
    pub refund_status: String,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteReopenRecord {
    pub reopen_id: String,
    pub work_order_id: String,
    pub location_id: String,
    pub body: String,
    pub status: String,
    pub reserve_status: String,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRouteCancellationRecord {
    pub cancellation_id: String,
    pub work_order_id: String,
    pub location_id: String,
    pub body: String,
    pub status: String,
    pub refund_status: String,
    pub created_at_epoch: i64,
}

impl WorldRouteRecords {
    pub fn fixture_for_split(world: &WorldState, actor_id: &str) -> Self {
        let current_node_id = world
            .positions
            .iter()
            .find(|position| position.actor_id == actor_id)
            .map(|position| position.node_id.as_str())
            .unwrap_or("mirror-city-square");
        let session = WorldTacticsGameSession::fixture_for(actor_id, current_node_id);
        Self {
            events: vec![WorldRouteEventRecord {
                event_id: "event-fixture-first-route".to_string(),
                event_kind: "world_action".to_string(),
                location_id: "mirror-city".to_string(),
                task_id: "task-fixture-first-route".to_string(),
                route_status: "pending".to_string(),
                body: "成果 证据 风险 下一步 自检".to_string(),
                impact_score: 10,
                created_at_epoch: 10,
            }],
            contracts: vec![WorldRouteContractRecord {
                contract_id: "contract-fixture-first-route".to_string(),
                task_id: "task-fixture-first-route".to_string(),
                title: "First standalone route contract".to_string(),
                body: "capture route deliverable with evidence and next action".to_string(),
                location_id: "mirror-city".to_string(),
                status: "open".to_string(),
                value_score: 20,
                created_at_epoch: 20,
            }],
            work_orders: vec![WorldRouteWorkOrderRecord {
                work_order_id: "work-order-fixture-first-route".to_string(),
                listing_id: "listing-fixture-first-route".to_string(),
                purchase_id: "purchase-fixture-first-route".to_string(),
                location_id: "mirror-city".to_string(),
                brief: "move east, train, attack, settle reward".to_string(),
                status: "in_progress".to_string(),
                value_score: 18,
                created_at_epoch: 30,
            }],
            deliveries: vec![WorldRouteDeliveryRecord {
                delivery_id: "delivery-fixture-first-route".to_string(),
                work_order_id: "work-order-fixture-first-route".to_string(),
                location_id: "mirror-city".to_string(),
                body: "evidence-ready route report".to_string(),
                status: "submitted".to_string(),
                score: 8.5,
                created_at_epoch: 40,
            }],
            tactics_sessions: vec![session.clone()],
            tactics_ticks: vec![WorldTacticsSimulationTick::fixture_attack_block(&session)],
            reward_settlements: vec![WorldTacticsRewardSettlement::fixture_pending(&session)],
            ..Self::default()
        }
    }

    pub fn preview_items(&self) -> Vec<Value> {
        let mut items = Vec::new();
        items.extend(self.events.iter().map(|event| {
            json!({
                "route_bucket": "event",
                "location_id": event.location_id,
                "task_id": event.task_id,
                "route_status": event.route_status,
                "created_at_epoch": event.created_at_epoch,
                "event_id": event.event_id,
                "title": event.event_kind,
                "summary": event.body,
                "detail": format!("{} · impact +{}", event.location_id, event.impact_score),
            })
        }));
        items.extend(self.contracts.iter().map(|contract| {
            json!({
                "route_bucket": "contract",
                "location_id": contract.location_id,
                "task_id": contract.task_id,
                "route_status": contract.status,
                "created_at_epoch": contract.created_at_epoch,
                "contract_id": contract.contract_id,
                "title": contract.title,
                "summary": contract.body,
                "detail": format!("task {} · value {}", contract.task_id, contract.value_score),
            })
        }));
        items.extend(self.completions.iter().map(|completion| json!({
            "route_bucket": "completion",
            "location_id": completion.location_id,
            "task_id": completion.task_id,
            "route_status": completion.ledger_status.clone().unwrap_or_else(|| completion.payout_status.clone()),
            "created_at_epoch": completion.created_at_epoch,
            "completion_id": completion.completion_id,
            "contract_id": completion.contract_id,
            "title": format!("契约战报 {}", completion.completion_id),
            "summary": completion.body,
            "detail": format!("评分 {:.1} · 奖励 {:.2}", completion.score, completion.reward_amount),
        })));
        items.extend(self.purchases.iter().map(|purchase| {
            json!({
                "route_bucket": "purchase",
                "location_id": purchase.location_id,
                "route_status": purchase.status,
                "created_at_epoch": purchase.created_at_epoch,
                "purchase_id": purchase.purchase_id,
                "listing_id": purchase.listing_id,
                "title": format!("接取契约 {}", purchase.purchase_id),
                "summary": format!("{} 奖励 · {}", purchase.price_credits, purchase.status),
                "detail": format!("任务牌 {}", purchase.listing_id),
            })
        }));
        items.extend(self.work_orders.iter().map(|work_order| {
            json!({
                "route_bucket": "work_order",
                "location_id": work_order.location_id,
                "route_status": work_order.status,
                "created_at_epoch": work_order.created_at_epoch,
                "work_order_id": work_order.work_order_id,
                "listing_id": work_order.listing_id,
                "purchase_id": work_order.purchase_id,
                "title": format!("冒险委托 {}", work_order.work_order_id),
                "summary": work_order.brief,
                "detail": format!("难度 {} · {}", work_order.value_score, work_order.status),
            })
        }));
        items.extend(self.deliveries.iter().map(|delivery| {
            json!({
                "route_bucket": "delivery",
                "location_id": delivery.location_id,
                "route_status": delivery.status,
                "created_at_epoch": delivery.created_at_epoch,
                "work_order_id": delivery.work_order_id,
                "title": format!("成果提交 {}", delivery.delivery_id),
                "summary": delivery.body,
                "detail": format!("评分 {:.1} · {}", delivery.score, delivery.status),
            })
        }));
        items.extend(self.acceptances.iter().map(|acceptance| {
            json!({
                "route_bucket": "acceptance",
                "location_id": acceptance.location_id,
                "route_status": acceptance.status,
                "created_at_epoch": acceptance.created_at_epoch,
                "work_order_id": acceptance.work_order_id,
                "title": format!("评级通过 {}", acceptance.acceptance_id),
                "summary": acceptance.body,
                "detail": format!("声望 +{} · {}", acceptance.reputation_delta, acceptance.status),
            })
        }));
        items.extend(self.rejections.iter().map(|rejection| {
            json!({
                "route_bucket": "rejection",
                "location_id": rejection.location_id,
                "route_status": rejection.status,
                "created_at_epoch": rejection.created_at_epoch,
                "work_order_id": rejection.work_order_id,
                "title": format!("返工要求 {}", rejection.rejection_id),
                "summary": rejection.body,
                "detail": format!("奖励退回 {} · {}", rejection.refund_status, rejection.status),
            })
        }));
        items.extend(self.reopens.iter().map(|reopen| {
            json!({
                "route_bucket": "reopen",
                "location_id": reopen.location_id,
                "route_status": reopen.status,
                "created_at_epoch": reopen.created_at_epoch,
                "work_order_id": reopen.work_order_id,
                "title": format!("委托重开 {}", reopen.reopen_id),
                "summary": reopen.body,
                "detail": format!("再次托管 {} · {}", reopen.reserve_status, reopen.status),
            })
        }));
        items.extend(self.cancellations.iter().map(|cancellation| json!({
            "route_bucket": "cancellation",
            "location_id": cancellation.location_id,
            "route_status": cancellation.status,
            "created_at_epoch": cancellation.created_at_epoch,
            "work_order_id": cancellation.work_order_id,
            "title": format!("委托放弃 {}", cancellation.cancellation_id),
            "summary": cancellation.body,
            "detail": format!("奖励退回 {} · {}", cancellation.refund_status, cancellation.status),
        })));
        items.extend(self.tactics_sessions.iter().map(|session| json!({
            "route_bucket": "tactics_objective",
            "location_id": session.active_node_id,
            "task_id": session.route_task_id(),
            "route_status": format!("{}_{}", session.victory_state, session.reward_status),
            "created_at_epoch": session.updated_at_epoch,
            "event_id": session.reward_event_id,
            "title": format!("Tactics objective {}", session.objective_id),
            "summary": format!("objective {}/{} · victory {} · reward {}", session.objective_progress, session.objective_goal.max(1), session.victory_state, session.reward_status),
            "detail": format!("session {} · overlay {}", session.session_id, session.active_overlay_id),
            "tactics_route_task_binding": tactics_route_task_binding_json(session, &self.tactics_ticks, &self.reward_settlements),
            "tactics_route_task_binding_contract_version": "trillionnium_tactics_route_task_binding_v1",
        })));
        items
    }
}

pub fn tactics_route_task_binding_json(
    session: &WorldTacticsGameSession,
    ticks: &[WorldTacticsSimulationTick],
    settlements: &[WorldTacticsRewardSettlement],
) -> Value {
    let reward_settlement = settlements
        .iter()
        .find(|settlement| settlement.session_id == session.session_id)
        .cloned()
        .unwrap_or_else(|| WorldTacticsRewardSettlement::fixture_pending(session));
    let repeat_block_count = ticks
        .iter()
        .filter(|tick| {
            tick.session_id == session.session_id
                && tick.command == "attack"
                && !tick.outcome_accepted
                && tick.outcome_result == "repeat_farming_blocked"
        })
        .count();
    json!({
        "contract_version": "trillionnium_tactics_route_task_binding_v1",
        "source_of_truth": "rust_world_tactics_sessions",
        "route_task_id": session.route_task_id(),
        "session_id": session.session_id,
        "matrix_user_id": session.matrix_user_id,
        "active_node_id": session.active_node_id,
        "active_overlay_id": session.active_overlay_id,
        "objective_id": session.objective_id,
        "objective_progress": session.objective_progress,
        "objective_goal": session.objective_goal,
        "victory_state": session.victory_state,
        "reward_status": session.reward_status,
        "reward_event_id": session.reward_event_id,
        "reward_credits_awarded": session.reward_credits_awarded,
        "reward_xp_awarded": session.reward_xp_awarded,
        "reward_settlement_contract_version": TRILLIONNIUM_TACTICS_REWARD_SETTLEMENT_CONTRACT_VERSION,
        "reward_settlement": reward_settlement.to_projection_json(),
        "reward_history_contract_version": "trillionnium_tactics_reward_history_v1",
        "reward_history": [
            {"history_id": format!("tactics-reward-history:{}:objective", session.session_id), "stage": "objective_progress", "status": if session.objective_progress >= session.objective_goal.max(1) { "completed" } else { "in_progress" }, "label": "Tactics objective / 战棋目标", "summary": format!("Objective {} progress {}/{}", session.objective_id, session.objective_progress, session.objective_goal.max(1))},
            {"history_id": format!("tactics-reward-history:{}:victory", session.session_id), "stage": "victory_state", "status": session.victory_state, "label": "Victory state / 胜负状态", "summary": format!("Rust tactics session state is {}", session.victory_state)},
            {"history_id": format!("tactics-reward-history:{}:reward", session.session_id), "stage": "reward_settlement", "status": session.reward_status, "label": "Reward settlement / 奖励结算", "summary": if reward_settlement.ledger_receipt_id.is_some() { "Reward event settled into player progression and route history." } else { "Reward waits for victory and server-owned settlement." }, "reward_event_id": session.reward_event_id, "credits_delta": session.reward_credits_awarded, "xp_delta": session.reward_xp_awarded, "ledger_release_gate_status": if reward_settlement.ledger_receipt_id.is_some() { "server_settled" } else { "not_released" }},
        ],
        "reward_history_summary": if reward_settlement.ledger_receipt_id.is_some() { "Tactics reward settled and visible in route-runner history." } else { "Tactics reward history is waiting for victory settlement." },
        "ledger_release_gate_status": if reward_settlement.ledger_receipt_id.is_some() { "server_settled" } else { "not_released" },
        "anti_cheese_contract_version": TRILLIONNIUM_TACTICS_REPEAT_FARMING_ANTI_CHEESE_CONTRACT_VERSION,
        "repeat_farming": {"policy": "one settled reward per tactics objective/session", "repeat_attack_after_settlement": "blocked", "blocked_attempt_count": repeat_block_count, "gate_enforced": true, "source_of_truth": "rust_tactics_repeat_farming_guard"},
        "web_role": "visualization_input_only",
    })
}

pub fn world_route_artifacts_from_records(
    records: &WorldRouteRecords,
    limit: usize,
) -> WorldRouteArtifacts {
    world_route_artifacts_from_raw_preview_items(records.preview_items(), limit)
}

pub fn world_route_artifacts_for_fixture_world(
    world: &WorldState,
    actor_id: &str,
) -> WorldRouteArtifacts {
    world_route_artifacts_from_records(&WorldRouteRecords::fixture_for_split(world, actor_id), 24)
}

fn tactics_terrain_for(row: usize, col: usize) -> &'static str {
    match (row, col) {
        (0, 0) | (0, 1) | (1, 0) => "camp",
        (1, 4) | (2, 4) | (3, 4) => "river",
        (2, 2) | (2, 3) | (3, 2) => "forest",
        (4, 5) | (5, 5) | (5, 6) => "market",
        (6, 6) | (6, 7) | (7, 6) | (7, 7) => "objective",
        _ if row == col => "road",
        _ => "plain",
    }
}

fn tactics_tile_label(row: usize, col: usize) -> String {
    let column = (b'A' + col as u8) as char;
    format!("{column}{}", row + 1)
}

fn world_node_overlay_id(node_id: &str) -> String {
    format!("trillionnium-world-node:{node_id}")
}

fn node_for_semantic_role<'a>(
    world: &'a WorldState,
    role: &str,
) -> Option<&'a trnm_world_domain::WorldNode> {
    world.nodes.iter().find(|node| {
        node.tags.iter().any(|tag| tag == role)
            || node.node_kind == role
            || trillionnium_objective_role_for_node(node) == role
    })
}

fn trillionnium_objective_role_for_node(node: &trnm_world_domain::WorldNode) -> &'static str {
    if node.tags.iter().any(|tag| tag == "arena") || node.id.contains("coliseum") {
        "arena"
    } else if node.tags.iter().any(|tag| tag == "market") || node.id.contains("market") {
        "market"
    } else if node.tags.iter().any(|tag| tag == "mentor") || node.id.contains("mentor") {
        "mentor_home"
    } else if node.tags.iter().any(|tag| tag == "quest_board") || node.id.contains("board") {
        "quest_board"
    } else if node.tags.iter().any(|tag| tag == "raid_hall") || node.id.contains("raid") {
        "raid_hall"
    } else {
        "civic_square"
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TacticsUnitView {
    pub unit_id: String,
    pub owner: String,
    pub side: String,
    pub archetype: String,
    pub label: String,
    pub title: String,
    pub grid_column: i64,
    pub grid_row: i64,
    pub hp: i64,
    pub max_hp: i64,
    pub energy: i64,
    pub move_range: i64,
    pub attack_range: i64,
    pub status_effects: Vec<String>,
    pub osm_game_overlay_id: Option<String>,
    pub actor_matrix_user_id: Option<String>,
    pub character_source: String,
}

impl TacticsUnitView {
    pub fn to_value(&self) -> Value {
        json!({
            "contract_version": TRILLIONNIUM_TACTICS_UNIT_CONTRACT_VERSION,
            "unit_id": self.unit_id,
            "owner": self.owner,
            "side": self.side,
            "class": self.archetype,
            "archetype": self.archetype,
            "label": self.label,
            "title": self.title,
            "grid_column": self.grid_column,
            "grid_row": self.grid_row,
            "hp": self.hp,
            "max_hp": self.max_hp,
            "energy": self.energy,
            "position": {"grid_column": self.grid_column, "grid_row": self.grid_row, "tile_id": tactics_tile_label((self.grid_row - 1).max(0) as usize, (self.grid_column - 1).max(0) as usize)},
            "move": self.move_range,
            "move_range": self.move_range,
            "attack_range": self.attack_range,
            "status_effects": self.status_effects,
            "osm_game_overlay_id": self.osm_game_overlay_id,
            "overlay_identity_ref": self.osm_game_overlay_id,
            "actor_matrix_user_id": self.actor_matrix_user_id,
            "character_source": self.character_source,
            "unit_selection_contract_version": TRILLIONNIUM_TACTICS_UNIT_SELECTION_CONTRACT_VERSION,
            "command_intent_draft_contract_version": TRILLIONNIUM_TACTICS_COMMAND_INTENT_DRAFT_CONTRACT_VERSION,
            "accessibility_contract_version": TRILLIONNIUM_TACTICS_ACCESSIBILITY_CONTRACT_VERSION,
            "selectable_unit": true,
            "selection_role": "active_unit",
            "keyboard_focus_role": "active_unit_button",
            "draft_input_name": "unit_id",
            "aria_role": "button",
            "validation_owner": "rust_tactics_command_validator",
            "web_role": "intent_only_visualization_input",
            "source_of_truth": "rust_tactics_unit_model",
        })
    }
}

fn tactics_units_json(
    matrix_user_id: &str,
    character: &Value,
    arena_overlay_id: Option<String>,
    market_overlay_id: Option<String>,
) -> Value {
    let max_hp = character["attributes"]["derived_stats"]["max_hp"]
        .as_i64()
        .unwrap_or(160);
    let energy = character["attributes"]["derived_stats"]["inner_energy"]
        .as_i64()
        .unwrap_or(100);
    let combat_hp = character["combat_numerics_runtime"]["health"]["current"]
        .as_i64()
        .unwrap_or_else(|| (max_hp / 4).clamp(32, 80));
    let combat_energy = character["combat_numerics_runtime"]["inner_energy"]["current"]
        .as_i64()
        .unwrap_or(energy);
    let move_range = character["attributes"]["derived_stats"]["move_range"]
        .as_i64()
        .unwrap_or(4);
    let unit = |unit_id: &str,
                owner: &str,
                side: &str,
                archetype: &str,
                label: &str,
                title: &str,
                col: i64,
                row: i64,
                hp: i64,
                max_hp: i64,
                energy: i64,
                move_range: i64,
                attack_range: i64,
                status: &[&str],
                overlay: Option<String>,
                actor: Option<String>,
                source: &str| TacticsUnitView {
        unit_id: unit_id.to_string(),
        owner: owner.to_string(),
        side: side.to_string(),
        archetype: archetype.to_string(),
        label: label.to_string(),
        title: title.to_string(),
        grid_column: col,
        grid_row: row,
        hp,
        max_hp,
        energy,
        move_range,
        attack_range,
        status_effects: status.iter().map(|v| (*v).to_string()).collect(),
        osm_game_overlay_id: overlay,
        actor_matrix_user_id: actor,
        character_source: source.to_string(),
    };
    Value::Array(
        vec![
            unit(
                "lord",
                "player",
                "player",
                "trillionnium_lord",
                "主",
                "主公 / Lord",
                2,
                7,
                combat_hp.clamp(0, max_hp),
                max_hp,
                combat_energy.clamp(0, energy),
                move_range,
                1,
                &["ready", "player_controlled"],
                None,
                Some(matrix_user_id.to_string()),
                "trillionnium_character",
            ),
            unit(
                "strategist",
                "ally",
                "ally",
                "route_strategist",
                "策",
                "军师 / Strategist",
                3,
                6,
                24,
                80,
                70,
                3,
                2,
                &["support", "route_reader"],
                None,
                None,
                "route_runner_support",
            ),
            unit(
                "agent-squad",
                "ally",
                "ally",
                "agent_scout",
                "斥",
                "Agent 斥候 / Scout",
                1,
                8,
                28,
                90,
                85,
                5,
                1,
                &["scout", "evidence_runner"],
                None,
                None,
                "agent_party",
            ),
            unit(
                "rival-warlord",
                "enemy",
                "enemy",
                "rival_commander",
                "敌",
                "敌将 / Rival",
                7,
                2,
                30,
                96,
                60,
                3,
                1,
                &["guarding_objective"],
                arena_overlay_id,
                None,
                "rust_fixture_enemy",
            ),
            unit(
                "market-bandit",
                "enemy",
                "enemy",
                "market_bandit",
                "寇",
                "流寇 / Bandit",
                6,
                4,
                18,
                64,
                48,
                4,
                1,
                &["threatens_market_route"],
                market_overlay_id,
                None,
                "rust_fixture_enemy",
            ),
        ]
        .into_iter()
        .map(|unit| unit.to_value())
        .collect(),
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TacticsCommandDescriptor {
    pub command_id: String,
    pub command: String,
    pub label: String,
    pub command_family: String,
    pub validation_owner: String,
    pub web_target: String,
    pub required_skill_id: Option<String>,
    pub action_cost: i64,
}

impl TacticsCommandDescriptor {
    #[allow(clippy::too_many_arguments)]
    fn new(
        command_id: &str,
        command: &str,
        label: &str,
        family: &str,
        owner: &str,
        target: &str,
        required_skill_id: Option<&str>,
        action_cost: i64,
    ) -> Self {
        Self {
            command_id: command_id.to_string(),
            command: command.to_string(),
            label: label.to_string(),
            command_family: family.to_string(),
            validation_owner: owner.to_string(),
            web_target: target.to_string(),
            required_skill_id: required_skill_id.map(ToString::to_string),
            action_cost,
        }
    }

    pub fn to_value(&self) -> Value {
        json!({
            "contract_version": TRILLIONNIUM_TACTICS_COMMAND_CONTRACT_VERSION,
            "command_id": self.command_id,
            "command": self.command,
            "label": self.label,
            "command_family": self.command_family,
            "validation_owner": self.validation_owner,
            "web_target": self.web_target,
            "required_skill_id": self.required_skill_id,
            "action_cost": self.action_cost,
            "command_intent_draft_contract_version": TRILLIONNIUM_TACTICS_COMMAND_INTENT_DRAFT_CONTRACT_VERSION,
            "accessibility_contract_version": TRILLIONNIUM_TACTICS_ACCESSIBILITY_CONTRACT_VERSION,
            "draft_input_name": "command",
            "keyboard_focus_role": "command_button",
            "aria_role": "button",
            "target_tile_required": matches!(self.command.as_str(), "move_unit" | "attack" | "use_skill" | "interact"),
            "unit_selection_required": true,
            "draft_owner": "browser_tactics_intent_builder",
            "source_of_truth": "rust_tactics_command_model",
            "web_role": "intent_only_visualization_input",
        })
    }
}

pub fn tactics_available_commands_json() -> Value {
    let commands = vec![
        TacticsCommandDescriptor::new(
            "select_unit",
            "select_unit",
            "选中单位",
            "selection",
            "rust_tactics_command_validator",
            "#trillionnium-tactics-game-shell",
            None,
            0,
        ),
        TacticsCommandDescriptor::new(
            "move_unit",
            "move_unit",
            "行军路线",
            "movement",
            "rust_tactics_command_validator",
            "#world-map-route-flow-actions",
            Some("basic_lightness"),
            1,
        ),
        TacticsCommandDescriptor::new(
            "attack",
            "attack",
            "发起攻击",
            "combat",
            "rust_tactics_combat_handler",
            "#trillionnium-tactics-game-shell",
            Some("basic_unarmed"),
            1,
        ),
        TacticsCommandDescriptor::new(
            "use_skill",
            "use_skill",
            "施展技能",
            "trillionnium_skill",
            "rust_trillionnium_skill_validator",
            "#trillionnium-status",
            Some("basic_inner_power"),
            1,
        ),
        TacticsCommandDescriptor::new(
            "equip_item",
            "equip_item",
            "装备道具",
            "item_equipment",
            "rust_trillionnium_item_equipment_runtime_state",
            "#trillionnium-equipment",
            None,
            0,
        ),
        TacticsCommandDescriptor::new(
            "train_skill",
            "train_skill",
            "导师修炼",
            "mentor_training",
            "rust_mentor_training_validator",
            "#trillionnium-training",
            None,
            1,
        ),
        TacticsCommandDescriptor::new(
            "talk_npc",
            "talk_npc",
            "交谈问路",
            "npc_society",
            "rust_trillionnium_npc_interaction_validator",
            "#trillionnium-npcs",
            None,
            0,
        ),
        TacticsCommandDescriptor::new(
            "offer_task",
            "offer_task",
            "接取Trillionnium任务",
            "trillionnium_task_offer",
            "rust_trillionnium_task_offer_validator",
            "#trillionnium-npcs",
            Some("reading_and_contracts"),
            1,
        ),
        TacticsCommandDescriptor::new(
            "complete_task",
            "complete_task",
            "提交任务战报",
            "trillionnium_task_completion",
            "rust_trillionnium_task_completion_handler",
            "#trillionnium-task-candidates",
            Some("reading_and_contracts"),
            1,
        ),
        TacticsCommandDescriptor::new(
            "interact",
            "interact",
            "接取悬赏",
            "world_interaction",
            "rust_world_action_handler",
            "#world-commerce-panel",
            Some("reading_and_contracts"),
            1,
        ),
        TacticsCommandDescriptor::new(
            "inspect_osm_underlay",
            "interact",
            "查看底图",
            "map_inspection",
            "rust_openstreetmap_data_provider",
            "#world-real-map",
            Some("streetwise_investigation"),
            0,
        ),
        TacticsCommandDescriptor::new(
            "end_turn",
            "end_turn",
            "结束回合",
            "turn_control",
            "rust_tactics_turn_handler",
            "#trillionnium-tactics-game-shell",
            None,
            0,
        ),
    ];
    Value::Array(
        commands
            .into_iter()
            .map(|command| command.to_value())
            .collect(),
    )
}

pub fn tactics_command_descriptor_json(command: &str) -> Option<Value> {
    tactics_available_commands_json()
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .find(|descriptor| {
            descriptor
                .get("command")
                .and_then(Value::as_str)
                .is_some_and(|value| value == command)
        })
}

pub fn world_trillionnium_character_projection_json(matrix_user_id: &str) -> Value {
    let mut character = WorldTrillionniumCharacter::default_for(matrix_user_id);
    character.ensure_defaults(0);
    character.to_projection_json()
}

pub fn world_tactics_game_session_projection_json(
    world: &WorldState,
    matrix_user_id: &str,
) -> Value {
    let node_id = world
        .positions
        .iter()
        .find(|position| position.actor_id == matrix_user_id)
        .map(|position| position.node_id.as_str())
        .unwrap_or("mirror-city-square");
    WorldTacticsGameSession::fixture_for(matrix_user_id, node_id).to_projection_json()
}

pub fn world_tactics_simulation_tick_log_json(world: &WorldState, matrix_user_id: &str) -> Value {
    let node_id = world
        .positions
        .iter()
        .find(|position| position.actor_id == matrix_user_id)
        .map(|position| position.node_id.as_str())
        .unwrap_or("mirror-city-square");
    let session = WorldTacticsGameSession::fixture_for(matrix_user_id, node_id);
    Value::Array(vec![WorldTacticsSimulationTick::fixture_attack_block(
        &session,
    )
    .to_projection_json()])
}

pub fn trillionnium_npc_command_descriptors_json() -> Value {
    let descriptors = trillionnium_npc_fixtures()
        .into_iter()
        .flat_map(|npc| {
            let mut commands = vec![json!({
                "contract_version": TRILLIONNIUM_NPC_COMMAND_DESCRIPTOR_CONTRACT_VERSION,
                "npc_id": npc.npc_id,
                "command": "talk_npc",
                "label": format!("Talk to {}", npc.display_name),
                "validation_owner": "rust_trillionnium_npc_interaction_validator",
                "web_role": "intent_only_visualization_input",
            })];
            for skill_id in npc.training_skill_ids {
                commands.push(json!({
                    "contract_version": TRILLIONNIUM_NPC_COMMAND_DESCRIPTOR_CONTRACT_VERSION,
                    "npc_id": npc.npc_id,
                    "command": "train_skill",
                    "skill_id": skill_id,
                    "label": format!("Train {}", skill_id),
                    "validation_owner": "rust_mentor_training_validator",
                    "web_role": "intent_only_visualization_input",
                }));
            }
            commands
        })
        .collect::<Vec<_>>();
    Value::Array(descriptors)
}

pub fn trillionnium_task_candidates_json() -> Value {
    Value::Array(
        trillionnium_task_archetype_fixtures()
            .into_iter()
            .enumerate()
            .map(|(index, task)| json!({
                "contract_version": TRILLIONNIUM_TASK_ARCHETYPE_CONTRACT_VERSION,
                "candidate_id": trillionnium_hash_id("trillionnium-task-candidate", &task.task_archetype_id),
                "task_archetype_id": task.task_archetype_id,
                "display_name": task.display_name,
                "source_semantic_roles": task.source_semantic_roles,
                "command": task.command,
                "completion_owner": task.completion_owner,
                "reward_gate": task.reward_gate,
                "log_style_key": task.log_style_key,
                "priority": 100 - index as i64,
                "source_of_truth": "rust_trillionnium_task_archetype_fixtures",
                "web_role": "visualization_input_only",
            }))
            .collect(),
    )
}

pub fn trillionnium_dynamic_social_simulation_json() -> Value {
    let factions = trillionnium_sect_fixtures()
        .into_iter()
        .map(|sect| {
            json!({
                "sect_id": sect.sect_id,
                "display_name": sect.display_name,
                "standing": 0,
                "anchor_semantic_role": sect.anchor_semantic_role,
                "source_of_truth": "rust_world_relationships_persistent_state",
            })
        })
        .collect::<Vec<_>>();
    json!({
        "contract_version": TRILLIONNIUM_WORLD_DYNAMIC_SOCIAL_SIMULATION_CONTRACT_VERSION,
        "source_of_truth": "rust_world_relationships_persistent_state",
        "runtime_status": "fixture_adapter_until_repository_cutover",
        "tracked_domains": ["npc_trust", "faction_standing", "mentor_access", "dispute_heat", "reward_gate_quality"],
        "faction_standings": factions,
        "web_role": "visualization_input_only",
    })
}

fn battle_log_style_json() -> Value {
    json!({
        "contract_version": TRILLIONNIUM_BATTLE_LOG_STYLE_CONTRACT_VERSION,
        "style": "concise_wuxia_route_evidence_log",
        "source_of_truth": "rust_trillionnium_battle_log_style",
        "content_policy": "trillionnium_native_no_copied_hero_tan_text_assets_or_tables",
    })
}

fn combat_log_json(objective_overlay_id: &str, task_candidates: &Value) -> Value {
    json!({
        "contract_version": TRILLIONNIUM_COMBAT_LOG_CONTRACT_VERSION,
        "source_of_truth": "rust_trillionnium_combat_log_projection",
        "objective_overlay_id": objective_overlay_id,
        "task_candidate_count": task_candidates.as_array().map(Vec::len).unwrap_or(0),
        "beats": [
            {"beat_id": "entry", "line": "Street compass points to the objective."},
            {"beat_id": "contact", "line": "Enemy guard blocks the evidence route."},
            {"beat_id": "resolution", "line": "Rust combat handler resolves damage and return state."},
            {"beat_id": "reward", "line": "Reward waits for settlement gate before route progress."}
        ],
        "web_role": "visualization_input_only",
    })
}

fn combat_encounter_projection_json(
    world: &WorldState,
    matrix_user_id: &str,
    current_node_id: &str,
) -> Value {
    let current_node = world.node(current_node_id).or_else(|| world.nodes.first());
    let semantic_role = current_node
        .map(trillionnium_objective_role_for_node)
        .unwrap_or("civic_square");
    let (encounter_kind, target_tile, defender_unit_id, defender_title, recommended_skill_id) =
        match semantic_role {
            "arena" | "raid_hall" => (
                "arena_duel_entry",
                "G7",
                "rival-warlord",
                "Rival Warlord",
                "basic_unarmed",
            ),
            "market" | "quest_board" | "delivery_route" | "arbitration_desk" => (
                "bounty_market_skirmish",
                "F5",
                "market-bandit",
                "Market Bandit",
                "basic_unarmed",
            ),
            _ => (
                "street_encounter_entry",
                "F5",
                "market-bandit",
                "Street Bandit",
                "basic_unarmed",
            ),
        };
    let node_id = current_node
        .map(|node| node.id.as_str())
        .unwrap_or("mirror-city-square");
    json!({
        "contract_version": TRILLIONNIUM_WORLD_COMBAT_ENCOUNTER_LOOP_CONTRACT_VERSION,
        "encounter_id": trillionnium_hash_id("world-combat-encounter", &format!("{matrix_user_id}:{node_id}:{semantic_role}:{target_tile}")),
        "encounter_kind": encounter_kind,
        "current_node_id": node_id,
        "current_node_name": current_node.map(|node| node.name.as_str()).unwrap_or("Mirror City Square"),
        "current_overlay_id": world_node_overlay_id(node_id),
        "semantic_role": semantic_role,
        "target_tile": target_tile,
        "defender_unit_id": defender_unit_id,
        "defender_title": defender_title,
        "recommended_skill_id": recommended_skill_id,
        "available": true,
        "entry_command": "attack",
        "return_anchor": "world-keypad-adventure-shell",
        "validation_owner": "rust_world_combat_encounter_validator",
        "command_handler_owner": "rust_tactics_combat_handler",
        "source_of_truth": "rust_world_combat_encounter_projection",
        "web_role": "intent_only_visualization_input",
    })
}

fn map_overlay_identity_index_json(world: &WorldState) -> Value {
    Value::Array(
        world
            .nodes
            .iter()
            .map(|node| {
                json!({
                    "contract_version": TRILLIONNIUM_MAP_OVERLAY_IDENTITY_CONTRACT_VERSION,
                    "osm_game_overlay_id": world_node_overlay_id(&node.id),
                    "node_id": node.id,
                    "name": node.name,
                    "semantic_role": trillionnium_objective_role_for_node(node),
                    "source_of_truth": "rust_world_map_provider_fixture",
                })
            })
            .collect(),
    )
}

fn osm_objectives_json(world: &WorldState) -> Value {
    Value::Array(
        world
            .nodes
            .iter()
            .take(12)
            .map(|node| {
                json!({
                    "contract_version": TRILLIONNIUM_OSM_OBJECTIVE_CONTRACT_VERSION,
                    "objective_id": trillionnium_hash_id("trillionnium-osm-objective", &node.id),
                    "node_id": node.id,
                    "location_id": node.location_id,
                    "semantic_role": trillionnium_objective_role_for_node(node),
                    "label": node.name,
                    "priority": 100,
                    "osm_game_overlay_id": world_node_overlay_id(&node.id),
                    "rust_command_handler_decides_completion": true,
                    "web_role": "visualization_input_only",
                })
            })
            .collect(),
    )
}

fn world_objective_travel_json(
    world: &WorldState,
    matrix_user_id: &str,
    current_node_id: &str,
) -> Value {
    let targets = world.tasks.iter().map(|task| json!({
        "task_id": task.id,
        "target_node_id": task.node_id,
        "route_status": if task.ledger_settlement_required { "ledger_settlement_required" } else { "open" },
        "reward_units": task.reward_units,
    })).collect::<Vec<_>>();
    json!({
        "contract_version": TRILLIONNIUM_WORLD_OBJECTIVE_TRAVEL_CONTRACT_VERSION,
        "source_of_truth": "rust_world_map_transition_rules",
        "matrix_user_id": matrix_user_id,
        "current_node_id": current_node_id,
        "target_count": targets.len(),
        "targets": targets,
        "web_role": "visualization_only_intent_to_map_move",
    })
}

#[allow(clippy::too_many_arguments)]
fn full_content_alignment_json(
    world: &WorldState,
    character: &Value,
    task_candidates: &Value,
    npc_command_descriptors: &Value,
    map_overlay_identity_index: &Value,
    combat_log: &Value,
    authored_quest_chains: &Value,
    story_arc_catalog: &Value,
    resource_pressure_runtime: &Value,
    dynamic_social_simulation: &Value,
) -> Value {
    let value_array_len = |value: &Value| value.as_array().map(Vec::len).unwrap_or(0);
    let nested_array_len = |value: &Value, field: &str| {
        value
            .get(field)
            .and_then(Value::as_array)
            .map(Vec::len)
            .unwrap_or(0)
    };
    let item_catalog = trillionnium_item_equipment_catalog_json();
    let resource_loops = trillionnium_resource_pressure_loops_json();
    let runtime_inventory_item_count = nested_array_len(character, "inventory_items");
    let runtime_equipped_slot_count = character
        .get("equipment_slots")
        .and_then(Value::as_object)
        .map(Map::len)
        .unwrap_or(0);
    let thresholds_green = trillionnium_fixture_skill_definitions().len() >= 18
        && trillionnium_training_command_fixtures().len() >= 18
        && trillionnium_sect_fixtures().len() >= 8
        && trillionnium_npc_fixtures().len() >= 18
        && value_array_len(npc_command_descriptors) >= 28
        && value_array_len(task_candidates) >= 10
        && world.nodes.len() >= 24
        && value_array_len(map_overlay_identity_index) >= 8
        && nested_array_len(combat_log, "beats") >= 4
        && nested_array_len(&item_catalog, "items") >= 12
        && runtime_inventory_item_count >= 3
        && runtime_equipped_slot_count >= 3
        && nested_array_len(&resource_loops, "loops") >= 6
        && resource_pressure_runtime
            .get("contract_version")
            .and_then(Value::as_str)
            == Some(TRILLIONNIUM_WORLD_RESOURCE_PRESSURE_RUNTIME_CONTRACT_VERSION)
        && dynamic_social_simulation
            .get("contract_version")
            .and_then(Value::as_str)
            == Some(TRILLIONNIUM_WORLD_DYNAMIC_SOCIAL_SIMULATION_CONTRACT_VERSION)
        && authored_quest_chains
            .get("contract_version")
            .and_then(Value::as_str)
            == Some(TRILLIONNIUM_WORLD_AUTHORED_QUEST_CHAIN_CONTRACT_VERSION)
        && nested_array_len(authored_quest_chains, "chains") >= 6
        && nested_array_len(story_arc_catalog, "arcs") >= 6;
    json!({
        "contract_version": TRILLIONNIUM_HERO_TAN_FULL_CONTENT_ALIGNMENT_CONTRACT_VERSION,
        "status": if thresholds_green { "content_volume_catalog_gate_green" } else { "content_volume_catalog_gate_blocked" },
        "scope": "full_content_volume_alignment_manifest",
        "source_of_truth": "rust_trillionnium_full_content_volume_alignment_gate",
        "web_role": "visualization_input_only",
        "coverage_counts": {
            "skill_definitions": trillionnium_fixture_skill_definitions().len(),
            "training_commands": trillionnium_training_command_fixtures().len(),
            "sects": trillionnium_sect_fixtures().len(),
            "npcs": trillionnium_npc_fixtures().len(),
            "npc_command_descriptors": value_array_len(npc_command_descriptors),
            "task_candidates": value_array_len(task_candidates),
            "world_map_nodes": world.nodes.len(),
            "map_overlay_identities": value_array_len(map_overlay_identity_index),
            "combat_log_beats": nested_array_len(combat_log, "beats"),
            "item_equipment_catalog": nested_array_len(&item_catalog, "items"),
            "runtime_inventory_items": runtime_inventory_item_count,
            "runtime_equipped_slots": runtime_equipped_slot_count,
            "resource_pressure_loops": nested_array_len(&resource_loops, "loops"),
            "authored_quest_chains": nested_array_len(authored_quest_chains, "chains"),
            "story_arcs": nested_array_len(story_arc_catalog, "arcs"),
        },
        "thresholds_green": thresholds_green,
        "domains": [
            {"domain": "items_and_equipment", "status": "rust_runtime_backed", "gate_field": "item_equipment_runtime"},
            {"domain": "board_session_projection", "status": "standalone_projection_backed", "gate_field": "game_session"},
            {"domain": "authored_quest_chains", "status": "native_catalog_expanded", "gate_field": "authored_quest_chains"},
            {"domain": "route_records_and_tactics_binding", "status": "adapter_record_backed", "gate_field": "world_route_artifacts"}
        ]
    })
}

pub fn world_tactics_board_projection_json(world: &WorldState, matrix_user_id: &str) -> Value {
    let current_node_id = world
        .positions
        .iter()
        .find(|position| position.actor_id == matrix_user_id)
        .map(|position| position.node_id.as_str())
        .unwrap_or("mirror-city-square");
    let current_overlay_id = world_node_overlay_id(current_node_id);
    let market_overlay_id =
        node_for_semantic_role(world, "market").map(|node| world_node_overlay_id(&node.id));
    let arena_overlay_id =
        node_for_semantic_role(world, "arena").map(|node| world_node_overlay_id(&node.id));
    let mentor_overlay_id =
        node_for_semantic_role(world, "mentor_home").map(|node| world_node_overlay_id(&node.id));
    let objective_overlay_id = market_overlay_id
        .clone()
        .unwrap_or_else(|| current_overlay_id.clone());
    let mut cells = Vec::with_capacity(64);
    for row in 0..8 {
        for col in 0..8 {
            let tile_label = tactics_tile_label(row, col);
            let terrain = tactics_terrain_for(row, col);
            let overlay_id = match terrain {
                "objective" => Some(objective_overlay_id.as_str()),
                "market" => market_overlay_id.as_deref(),
                "camp" => mentor_overlay_id.as_deref(),
                "road" => Some(current_overlay_id.as_str()),
                _ => None,
            };
            cells.push(json!({
                "tile_id": tile_label,
                "row": row + 1,
                "col": col + 1,
                "grid_row": row + 1,
                "grid_column": col + 1,
                "terrain": terrain,
                "source_of_truth": "rust_tactics_board_projection",
                "osm_game_overlay_id": overlay_id,
                "overlay_identity_ref": overlay_id,
                "board_cell_interaction_contract_version": TRILLIONNIUM_TACTICS_BOARD_CELL_INTERACTION_CONTRACT_VERSION,
                "command_intent_draft_contract_version": TRILLIONNIUM_TACTICS_COMMAND_INTENT_DRAFT_CONTRACT_VERSION,
                "accessibility_contract_version": TRILLIONNIUM_TACTICS_ACCESSIBILITY_CONTRACT_VERSION,
                "selectable": true,
                "selection_role": "target_tile",
                "keyboard_focus_role": "target_tile_gridcell",
                "draft_input_name": "target_tile",
                "aria_role": "gridcell",
                "validation_owner": "rust_tactics_command_validator",
                "web_role": "intent_only_visualization_input",
                "movement_cost": match terrain { "forest" => 2, "river" => 3, _ => 1 },
                "blocks_line_of_sight": terrain == "forest",
            }));
        }
    }
    let character = world_trillionnium_character_projection_json(matrix_user_id);
    let units = tactics_units_json(
        matrix_user_id,
        &character,
        arena_overlay_id.clone(),
        market_overlay_id.clone(),
    );
    let available_commands = tactics_available_commands_json();
    let skill_definitions =
        serde_json::to_value(trillionnium_fixture_skill_definitions()).expect("skills serialize");
    let training_commands =
        serde_json::to_value(trillionnium_training_command_fixtures()).expect("training serialize");
    let sects = serde_json::to_value(trillionnium_sect_fixtures()).expect("sects serialize");
    let npcs = serde_json::to_value(trillionnium_npc_fixtures()).expect("npcs serialize");
    let dynamic_social_simulation = trillionnium_dynamic_social_simulation_json();
    let npc_command_descriptors = trillionnium_npc_command_descriptors_json();
    let task_archetypes =
        serde_json::to_value(trillionnium_task_archetype_fixtures()).expect("tasks serialize");
    let task_candidates = trillionnium_task_candidates_json();
    let authored_quest_chains = trillionnium_authored_quest_chain_catalog_json(world);
    let osm_objectives = osm_objectives_json(world);
    let map_overlay_identity_index = map_overlay_identity_index_json(world);
    let game_session = world_tactics_game_session_projection_json(world, matrix_user_id);
    let simulation_ticks = world_tactics_simulation_tick_log_json(world, matrix_user_id);
    let combat_log = combat_log_json(&objective_overlay_id, &task_candidates);
    let combat_numerics_runtime = character
        .get("combat_numerics_runtime")
        .cloned()
        .unwrap_or_else(|| WorldTrillionniumCombatNumericsState::default().to_value());
    let item_equipment_catalog = trillionnium_item_equipment_catalog_json();
    let item_equipment_runtime = character.get("item_equipment_runtime").cloned().unwrap_or_else(|| json!({"contract_version": TRILLIONNIUM_WORLD_ITEM_EQUIPMENT_RUNTIME_CONTRACT_VERSION, "source_of_truth": "rust_trillionnium_item_equipment_runtime_state"}));
    let resource_pressure_runtime = character
        .get("resource_pressure_runtime")
        .cloned()
        .unwrap_or_else(|| WorldTrillionniumResourcePressureState::default().to_value());
    let survival_runtime = character
        .get("survival_runtime")
        .cloned()
        .unwrap_or_else(|| WorldTrillionniumResourcePressureState::default().survival_to_value());
    let resource_pressure_loops = trillionnium_resource_pressure_loops_json();
    let region_story_unlock_state = WorldTrillionniumRegionStoryUnlockState::default();
    let region_story_unlock_runtime = character
        .get("region_story_unlock_runtime")
        .cloned()
        .unwrap_or_else(|| region_story_unlock_state.to_value());
    let story_arc_catalog = trillionnium_story_arc_catalog_json(&region_story_unlock_state);
    let world_objective_travel =
        world_objective_travel_json(world, matrix_user_id, current_node_id);
    let world_combat_encounter =
        combat_encounter_projection_json(world, matrix_user_id, current_node_id);
    let full_content_alignment = full_content_alignment_json(
        world,
        &character,
        &task_candidates,
        &npc_command_descriptors,
        &map_overlay_identity_index,
        &combat_log,
        &authored_quest_chains,
        &story_arc_catalog,
        &resource_pressure_runtime,
        &dynamic_social_simulation,
    );
    let simulation_tick_count = simulation_ticks.as_array().map(Vec::len).unwrap_or(0);
    json!({
        "contract_version": TRILLIONNIUM_TACTICS_BOARD_CONTRACT_VERSION,
        "source_of_truth": "rust_trillionnium_game_state",
        "web_role": "visualization_input_only",
        "unit_contract_version": TRILLIONNIUM_TACTICS_UNIT_CONTRACT_VERSION,
        "command_contract_version": TRILLIONNIUM_TACTICS_COMMAND_CONTRACT_VERSION,
        "command_outcome_contract_version": TRILLIONNIUM_TACTICS_COMMAND_OUTCOME_CONTRACT_VERSION,
        "trillionnium_skill_contract_version": TRILLIONNIUM_SKILL_CONTRACT_VERSION,
        "trillionnium_training_contract_version": TRILLIONNIUM_TRAINING_CONTRACT_VERSION,
        "trillionnium_sect_contract_version": TRILLIONNIUM_SECT_CONTRACT_VERSION,
        "trillionnium_npc_contract_version": TRILLIONNIUM_NPC_CONTRACT_VERSION,
        "trillionnium_sect_osm_binding_contract_version": TRILLIONNIUM_SECT_OSM_BINDING_CONTRACT_VERSION,
        "trillionnium_npc_spawn_contract_version": TRILLIONNIUM_NPC_SPAWN_CONTRACT_VERSION,
        "trillionnium_npc_command_descriptor_contract_version": TRILLIONNIUM_NPC_COMMAND_DESCRIPTOR_CONTRACT_VERSION,
        "mentor_training_task_contract_version": TRILLIONNIUM_MENTOR_TRAINING_TASK_CONTRACT_VERSION,
        "trillionnium_task_archetype_contract_version": TRILLIONNIUM_TASK_ARCHETYPE_CONTRACT_VERSION,
        "trillionnium_task_completion_contract_version": TRILLIONNIUM_TASK_COMPLETION_CONTRACT_VERSION,
        "trillionnium_reward_gate_contract_version": TRILLIONNIUM_REWARD_GATE_CONTRACT_VERSION,
        "trillionnium_battle_log_style_contract_version": TRILLIONNIUM_BATTLE_LOG_STYLE_CONTRACT_VERSION,
        "trillionnium_combat_log_contract_version": TRILLIONNIUM_COMBAT_LOG_CONTRACT_VERSION,
        "trillionnium_resource_pressure_runtime_contract_version": TRILLIONNIUM_WORLD_RESOURCE_PRESSURE_RUNTIME_CONTRACT_VERSION,
        "food_water_age_survival_runtime_contract_version": TRILLIONNIUM_WORLD_FOOD_WATER_AGE_SURVIVAL_CONTRACT_VERSION,
        "dynamic_social_simulation_contract_version": TRILLIONNIUM_WORLD_DYNAMIC_SOCIAL_SIMULATION_CONTRACT_VERSION,
        "authored_quest_chain_contract_version": TRILLIONNIUM_WORLD_AUTHORED_QUEST_CHAIN_CONTRACT_VERSION,
        "trillionnium_region_story_unlock_runtime_contract_version": TRILLIONNIUM_WORLD_REGION_STORY_UNLOCK_RUNTIME_CONTRACT_VERSION,
        "trillionnium_combat_numerics_runtime_contract_version": TRILLIONNIUM_WORLD_COMBAT_NUMERICS_RUNTIME_CONTRACT_VERSION,
        "trillionnium_npc_relationship_contract_version": TRILLIONNIUM_NPC_RELATIONSHIP_CONTRACT_VERSION,
        "trillionnium_osm_objective_contract_version": TRILLIONNIUM_OSM_OBJECTIVE_CONTRACT_VERSION,
        "world_objective_travel_contract_version": TRILLIONNIUM_WORLD_OBJECTIVE_TRAVEL_CONTRACT_VERSION,
        "world_skill_practice_loop_contract_version": TRILLIONNIUM_WORLD_SKILL_PRACTICE_LOOP_CONTRACT_VERSION,
        "world_combat_encounter_loop_contract_version": TRILLIONNIUM_WORLD_COMBAT_ENCOUNTER_LOOP_CONTRACT_VERSION,
        "full_content_alignment_contract_version": TRILLIONNIUM_HERO_TAN_FULL_CONTENT_ALIGNMENT_CONTRACT_VERSION,
        "item_equipment_runtime_contract_version": TRILLIONNIUM_WORLD_ITEM_EQUIPMENT_RUNTIME_CONTRACT_VERSION,
        "tactics_combat_resolution_contract_version": TRILLIONNIUM_TACTICS_COMBAT_RESOLUTION_CONTRACT_VERSION,
        "tactics_game_session_contract_version": TRILLIONNIUM_TACTICS_GAME_SESSION_CONTRACT_VERSION,
        "tactics_simulation_tick_contract_version": TRILLIONNIUM_TACTICS_SIMULATION_TICK_CONTRACT_VERSION,
        "tactics_reward_settlement_contract_version": TRILLIONNIUM_TACTICS_REWARD_SETTLEMENT_CONTRACT_VERSION,
        "tactics_repeat_farming_anti_cheese_contract_version": TRILLIONNIUM_TACTICS_REPEAT_FARMING_ANTI_CHEESE_CONTRACT_VERSION,
        "tactics_board_cell_interaction_contract_version": TRILLIONNIUM_TACTICS_BOARD_CELL_INTERACTION_CONTRACT_VERSION,
        "tactics_unit_selection_contract_version": TRILLIONNIUM_TACTICS_UNIT_SELECTION_CONTRACT_VERSION,
        "tactics_command_intent_draft_contract_version": TRILLIONNIUM_TACTICS_COMMAND_INTENT_DRAFT_CONTRACT_VERSION,
        "tactics_accessibility_contract_version": TRILLIONNIUM_TACTICS_ACCESSIBILITY_CONTRACT_VERSION,
        "map_overlay_identity_contract_version": TRILLIONNIUM_MAP_OVERLAY_IDENTITY_CONTRACT_VERSION,
        "intent_draft_policy": {"contract_version": TRILLIONNIUM_TACTICS_COMMAND_INTENT_DRAFT_CONTRACT_VERSION, "draft_owner": "browser_tactics_intent_builder", "validation_owner": "rust_tactics_command_validator", "browser_may_select": ["unit_id", "target_tile", "command"], "browser_may_not_resolve": ["movement_legality", "combat_result", "reward_status", "objective_completion"], "source_of_truth": "rust_trillionnium_game_state"},
        "accessibility_policy": {"contract_version": TRILLIONNIUM_TACTICS_ACCESSIBILITY_CONTRACT_VERSION, "keyboard_traversal": "roving_grid_focus", "keyboard_keys": ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight", "Home", "End", "Enter", "Space"], "low_motion_support": "prefers_reduced_motion", "source_of_truth": "rust_trillionnium_game_state"},
        "board": {"board_id": "mirror-street-tactics-board-v1", "width": 8, "height": 8, "coordinate_system": "A1_to_H8", "cells": cells},
        "trillionnium_character": character,
        "skill_definitions": skill_definitions,
        "training_commands": training_commands,
        "sects": sects,
        "npcs": npcs,
        "dynamic_social_simulation": dynamic_social_simulation,
        "authored_quest_chains": authored_quest_chains,
        "npc_spawn_anchors": [],
        "npc_command_descriptors": npc_command_descriptors,
        "mentor_training_task_flows": training_commands,
        "task_archetypes": task_archetypes,
        "task_candidates": task_candidates,
        "osm_objectives": osm_objectives.clone(),
        "world_objective_travel": world_objective_travel,
        "world_combat_encounter": world_combat_encounter,
        "map_overlay_identity_index": map_overlay_identity_index,
        "game_session": game_session,
        "simulation_ticks": simulation_ticks,
        "battle_log_style": battle_log_style_json(),
        "combat_log": combat_log,
        "combat_numerics_runtime": combat_numerics_runtime,
        "item_equipment_catalog": item_equipment_catalog,
        "item_equipment_runtime": item_equipment_runtime,
        "resource_pressure_runtime": resource_pressure_runtime,
        "resource_pressure_loops": resource_pressure_loops,
        "survival_runtime": survival_runtime,
        "region_story_unlock_runtime": region_story_unlock_runtime,
        "story_arc_catalog": story_arc_catalog,
        "full_content_alignment": full_content_alignment,
        "npc_relationship_model": {"contract_version": TRILLIONNIUM_NPC_RELATIONSHIP_CONTRACT_VERSION, "source_of_truth": "rust_world_relationships_persistent_state", "web_role": "visualization_input_only"},
        "units": units,
        "objectives": osm_objectives,
        "available_commands": available_commands,
        "turn_state": {"contract_version": "trillionnium_world_tactics_turn_state_v1", "active_side": "player", "active_unit_id": "lord", "round": 1, "action_points_remaining": 2, "source_of_truth": "rust_tactics_turn_handler", "allowed_commands": ["select_unit", "move_unit", "attack", "use_skill", "equip_item", "train_skill", "talk_npc", "offer_task", "complete_task", "interact", "end_turn"]},
        "battle_log": combat_log_json(&objective_overlay_id, &Value::Null)["beats"].clone(),
        "osm_objective_source": {"provider_contract": "OpenStreetMapDataProvider", "objective_overlay_id": objective_overlay_id, "objective_contract_version": TRILLIONNIUM_OSM_OBJECTIVE_CONTRACT_VERSION, "objective_count": osm_objectives.as_array().map(Vec::len).unwrap_or(0), "map_overlay_identity_contract_version": TRILLIONNIUM_MAP_OVERLAY_IDENTITY_CONTRACT_VERSION, "map_overlay_identity_count": map_overlay_identity_index.as_array().map(Vec::len).unwrap_or(0), "osm_can_suggest_objectives": true, "rust_command_handler_decides_completion": true},
        "simulation_tick_source": {"session_contract_version": TRILLIONNIUM_TACTICS_GAME_SESSION_CONTRACT_VERSION, "tick_contract_version": TRILLIONNIUM_TACTICS_SIMULATION_TICK_CONTRACT_VERSION, "tick_count": simulation_tick_count, "source_of_truth": "rust_tactics_simulation_tick", "persistence_owner": "world_state.world_tactics_simulation_ticks"},
        "repeat_farming_anti_cheese_policy": {"contract_version": TRILLIONNIUM_TACTICS_REPEAT_FARMING_ANTI_CHEESE_CONTRACT_VERSION, "gate_owner": "rust_tactics_repeat_farming_guard", "policy": "one objective reward per settled tactics session until a new route/objective is issued", "repeat_attack_after_settlement": "blocked", "reward_history_owner": "route_task_graph_and_route_runner_history", "web_role": "visualization_input_only"}
    })
}

pub fn world_full_split_projection_json(world: &WorldState, actor_id: &str) -> Value {
    let home = project_home(world, actor_id);
    let route_artifacts = world_route_artifacts_for_fixture_world(world, actor_id);
    let tactics_board = world_tactics_board_projection_json(world, actor_id);
    json!({
        "contract_version": "trillionnium_world_full_split_projection_v1",
        "source_of_truth": WORLD_RUST_SOURCE_OF_TRUTH,
        "home": home,
        "route_artifacts": route_artifacts,
        "tactics_board": tactics_board,
        "cutover_status": "standalone_world_server_owns_home_command_route_tactics_projection_fixture_adapters",
        "cex_dependency_status": "no_trnm_world_crate_depends_on_cex_service_internals",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use trnm_world_domain::WorldState;

    #[test]
    fn projection_is_rust_owned() {
        let projection = project_home(&WorldState::fixture(), "local-player");
        assert_eq!(projection.contract_version, WORLD_PROJECTION_CONTRACT);
        assert_eq!(projection.source_of_truth, WORLD_RUST_SOURCE_OF_TRUTH);
        assert_eq!(
            projection.route_command_target_contract,
            WORLD_ROUTE_COMMAND_TARGET_CONTRACT
        );
        assert_eq!(
            projection.route_ui_contract_version,
            WORLD_ROUTE_UI_CONTRACT_VERSION
        );
        assert_eq!(
            projection.player_node_id.as_deref(),
            Some("mirror-city-square")
        );
    }

    #[test]
    fn command_target_maps_work_reject_to_cex_commerce_lane() {
        let target = world_route_command_target("/work reject latest 重试拒收退款");
        assert_eq!(target.contract_version, WORLD_ROUTE_COMMAND_TARGET_CONTRACT);
        assert_eq!(target.panel_id, WORLD_ROUTE_COMMERCE_PANEL_ID);
        assert_eq!(target.input_id, WORLD_ROUTE_WORK_REJECT_INPUT_ID);
        assert_eq!(target.input_value, "latest");
        assert_eq!(target.textarea_id, WORLD_ROUTE_WORK_REJECT_TEXTAREA_ID);
        assert!(target.body.contains("客户交付方案"));
        assert!(target.body.contains("证据包"));
        assert!(target.body.contains("风险控制"));
        assert!(target.body.contains("下一步行动"));
        assert!(target.body.contains("自检复盘"));
    }

    #[test]
    fn command_target_maps_complete_contract_to_contract_lane() {
        let target =
            world_route_command_target("/complete contract-123 成果方案：证据 风险 下一步 自检");
        assert_eq!(target.panel_id, WORLD_ROUTE_CONTRACTS_PANEL_ID);
        assert_eq!(target.input_id, WORLD_ROUTE_CONTRACT_INPUT_ID);
        assert_eq!(target.input_value, "contract-123");
        assert_eq!(target.textarea_id, WORLD_ROUTE_CONTRACT_TEXTAREA_ID);
        assert!(target.body.contains("成果方案"));
    }

    #[test]
    fn focus_lane_specs_preserve_cex_node_preferences() {
        let work = world_route_focus_lane_spec(
            WORLD_ROUTE_COMMERCE_PANEL_ID,
            WORLD_ROUTE_WORK_REJECT_INPUT_ID,
        );
        assert_eq!(work.preferred_node_id.as_deref(), Some("delivery-dock"));
        assert!(work.desired_tags.iter().any(|tag| tag == "refund"));
        assert!(work.desired_tags.iter().any(|tag| tag == "review"));

        let contract = world_route_focus_lane_spec(WORLD_ROUTE_CONTRACTS_PANEL_ID, "anything");
        assert_eq!(contract.preferred_node_id.as_deref(), Some("ledger-office"));
        assert!(contract.desired_tags.iter().any(|tag| tag == "ledger"));
    }

    #[test]
    fn command_playability_anchor_preserves_prefixes() {
        let anchored = world_route_playability_anchor_command("/sell latest next".to_string());
        assert!(anchored.starts_with("/sell latest "));
        assert!(anchored.contains("customer deliverable"));
        assert!(anchored.contains("evidence package"));

        let unknown = world_route_playability_anchor_command("/unknown raw".to_string());
        assert_eq!(unknown, "/unknown raw");
    }

    #[test]
    fn route_ui_contract_exports_cex_panel_defaults() {
        let contract = world_route_ui_contract_json();
        assert_eq!(
            contract["contract_version"],
            WORLD_ROUTE_UI_CONTRACT_VERSION
        );
        assert_eq!(
            contract["panels"]["commerce"],
            WORLD_ROUTE_COMMERCE_PANEL_ID
        );
        assert_eq!(
            contract["work_lanes"]["rejection"]["input_id"],
            WORLD_ROUTE_WORK_REJECT_INPUT_ID
        );
        assert_eq!(
            contract["panel_defaults"][WORLD_ROUTE_CONTRACTS_PANEL_ID]["textarea_id"],
            WORLD_ROUTE_CONTRACT_TEXTAREA_ID
        );
    }

    #[test]
    fn route_recommendation_ranker_prefers_completable_low_risk_routes() {
        let strong = recommendation_ranked_preview_item(json!({
            "route_bucket": "delivery",
            "route_status": "pending",
            "task_id": "task-1",
            "location_id": "delivery-dock",
            "summary": "evidence ready with risk controls",
            "detail": "customer deliverable"
        }));
        let risky = recommendation_ranked_preview_item(json!({
            "route_bucket": "rejection",
            "route_status": "rejected_refund_failed",
            "task_id": "task-2",
            "location_id": "delivery-dock"
        }));
        assert!(
            strong["route_recommendation_score"].as_i64().unwrap()
                > risky["route_recommendation_score"].as_i64().unwrap()
        );
        assert_eq!(
            strong["route_recommendation_policy_contract_version"],
            TRILLIONNIUM_WORLD_ROUTE_RECOMMENDATION_POLICY_CONTRACT_VERSION
        );
        assert_eq!(
            risky["route_recommendation_reasons"]["suppressed_for_dispute_risk"],
            true
        );
    }

    #[test]
    fn route_task_descriptor_derives_settlement_recovery_opportunity() {
        let descriptor = WorldRouteTaskDescriptor {
            task_id: "task-77".to_string(),
            latest_bucket: "rejection".to_string(),
            latest_status: "rejected_chargeback_failed".to_string(),
            latest_location_id: "delivery-dock".to_string(),
            latest_title: "Logo delivery".to_string(),
            latest_summary: "buyer refunded".to_string(),
            latest_detail: "seller chargeback failed".to_string(),
            latest_event_title: "Reject event".to_string(),
            latest_contract_id: String::new(),
            latest_contract_title: String::new(),
            latest_completion_id: String::new(),
            latest_completion_title: String::new(),
        };
        let opportunity = descriptor.next_opportunity();
        assert_eq!(opportunity.kind, "rejection_chargeback_recovery");
        assert_eq!(
            opportunity.route_target.panel_id,
            WORLD_ROUTE_COMMERCE_PANEL_ID
        );
        assert_eq!(
            opportunity.route_target.input_id,
            WORLD_ROUTE_WORK_REJECT_INPUT_ID
        );
        assert_eq!(
            opportunity.focus_lane.preferred_node_id.as_deref(),
            Some("delivery-dock")
        );
        assert!(opportunity.command.contains("/work reject latest"));
        assert!(descriptor.feedback_focus().contains("证据缺口"));
    }

    #[test]
    fn preview_items_preserve_task_group_fallback_order() {
        let preview = json!({
            "items": [
                {"route_bucket": "event", "event_id": "evt-1", "title": "Event"},
                {"route_bucket": "work_order", "work_order_id": "work-1", "contract_id": "contract-1"}
            ]
        });
        let items = world_route_preview_items(&preview);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].task_group_id(), "evt-1");
        assert_eq!(items[1].task_group_id(), "work-1");
    }

    #[test]
    fn preview_json_and_task_graph_builder_preserve_cex_route_shape() {
        let preview = world_route_preview_json(vec![
            json!({
                "route_bucket": "event",
                "event_id": "evt-1",
                "task_id": "task-a",
                "route_status": "pending",
                "location_id": "client-board",
                "created_at_epoch": 10,
                "title": "Board event"
            }),
            json!({
                "route_bucket": "contract",
                "contract_id": "contract-a",
                "task_id": "task-a",
                "route_status": "pending",
                "location_id": "ledger-office",
                "created_at_epoch": 20,
                "title": "Contract A"
            }),
            json!({
                "route_bucket": "rejection",
                "work_order_id": "work-b",
                "route_status": "rejected_chargeback_failed",
                "location_id": "delivery-dock",
                "created_at_epoch": 30,
                "title": "Recovery B"
            }),
        ]);
        assert_eq!(preview["projection_layer"], "world_route_projection_v1");
        assert_eq!(preview["task_linked_count"], 2);

        let graph = world_route_task_graph_json(&preview);
        assert_eq!(
            graph["projection_layer"],
            "world_route_task_graph_projection_v1"
        );
        assert_eq!(graph["task_count"], 2);
        let tasks = graph["tasks"].as_array().unwrap();
        assert_eq!(tasks[0]["task_id"], "work-b");
        assert_eq!(
            tasks[0]["next_opportunity_kind"],
            "rejection_chargeback_recovery"
        );
        assert_eq!(
            tasks[0]["next_opportunity_input_id"],
            WORLD_ROUTE_WORK_REJECT_INPUT_ID
        );
        assert_eq!(tasks[1]["contract_count"], 1);

        let artifacts = world_route_artifacts(preview, graph, 4);
        assert_eq!(artifacts.task_views.len(), 2);
        assert_eq!(artifacts.story.next_task_id, "work-b");
        assert_eq!(
            artifacts.story.next_opportunity_target.input_id,
            WORLD_ROUTE_WORK_REJECT_INPUT_ID
        );
    }
}
