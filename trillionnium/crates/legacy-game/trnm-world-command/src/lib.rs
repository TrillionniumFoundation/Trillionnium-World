//! Intent-only Trillionnium World commands.

use serde::{Deserialize, Serialize};
use trnm_world_domain::{
    trillionnium_catalog_item_field, trillionnium_skill_definition_by_id,
    trillionnium_training_command_for_skill, TrillionniumCombatResolutionInput, WorldNode,
    WorldPosition, WorldState, WorldTrillionniumCharacter,
    TRILLIONNIUM_TACTICS_COMMAND_OUTCOME_CONTRACT_VERSION, WORLD_RUST_SOURCE_OF_TRUTH,
    WORLD_TRANSITION_SEMANTICS_CONTRACT,
};

pub const WORLD_COMMAND_CONTRACT: &str = "trillionnium_world_command_v1";
pub const WORLD_ACTION_ENGINE_CONTRACT: &str = "trillionnium_world_action_engine_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldActionKindDecision {
    pub contract_version: String,
    pub kind: String,
    pub result: String,
    pub base_impact: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldActionQualitySignal {
    pub score: i64,
    pub missing: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WorldCommand {
    Move {
        actor_id: String,
        direction: String,
    },
    TalkNpc {
        actor_id: String,
        npc_id: String,
    },
    TrainSkill {
        actor_id: String,
        npc_id: String,
        skill_id: String,
    },
    CompleteTask {
        actor_id: String,
        task_id: String,
    },
}

pub fn classify_world_action(body: &str) -> WorldActionKindDecision {
    let lower = body.to_ascii_lowercase();
    let (kind, result, base_impact) = if lower.contains("contract")
        || lower.contains("bounty")
        || lower.contains("commission")
        || body.contains("委托")
        || body.contains("悬赏")
        || body.contains("任务")
    {
        (
            "contract",
            "把现实需求登记成 World Contract，并生成可执行的 CEX 委托任务。",
            20,
        )
    } else if lower.contains("build")
        || lower.contains("craft")
        || lower.contains("fabricate")
        || body.contains("建")
        || body.contains("造")
        || body.contains("工坊")
    {
        (
            "craft",
            "在 Craft District 完成一次建造/创造行动，生成可迭代资产。",
            14,
        )
    } else if lower.contains("market")
        || lower.contains("listing")
        || lower.contains("buy")
        || lower.contains("client")
        || body.contains("客户")
        || body.contains("接单")
        || body.contains("市场")
    {
        (
            "market",
            "进入 Market Bazaar，把现实机会映射为世界委托。",
            16,
        )
    } else if lower.contains("company")
        || lower.contains("shop")
        || lower.contains("studio")
        || body.contains("公司")
        || body.contains("店")
        || body.contains("工作室")
    {
        (
            "venture",
            "创建了一个现实映射经营体，获得资产雏形和声望入口。",
            18,
        )
    } else if lower.contains("hire")
        || lower.contains("agent")
        || body.contains("招募")
        || body.contains("雇佣")
    {
        (
            "recruit",
            "与 Agent 居民建立合作关系，队伍能力获得提升。",
            12,
        )
    } else {
        (
            "explore",
            "完成一次开放世界探索，产生新的线索和关系变化。",
            10,
        )
    };
    WorldActionKindDecision {
        contract_version: WORLD_ACTION_ENGINE_CONTRACT.to_string(),
        kind: kind.to_string(),
        result: result.to_string(),
        base_impact,
    }
}

pub fn world_action_quality_signal(body: &str) -> WorldActionQualitySignal {
    let lower = body.to_ascii_lowercase();
    let mut score = 0;
    let mut missing = Vec::new();
    let signals = [
        (
            "deliverable",
            lower.contains("deliverable") || body.contains("成果") || body.contains("交付"),
        ),
        (
            "evidence",
            lower.contains("evidence")
                || lower.contains("source")
                || lower.contains("proof")
                || body.contains("证据")
                || body.contains("依据"),
        ),
        (
            "risk_control",
            lower.contains("risk") || body.contains("风险") || body.contains("风控"),
        ),
        (
            "next_action",
            lower.contains("next") || body.contains("下一步") || body.contains("计划"),
        ),
        (
            "self_review",
            lower.contains("review") || body.contains("自检") || body.contains("复盘"),
        ),
    ];
    for (signal, present) in signals {
        if present {
            score += 1;
        } else {
            missing.push(signal.to_string());
        }
    }
    if body.chars().count() >= 80 {
        score += 1;
    }
    WorldActionQualitySignal { score, missing }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTacticsCommandRequest {
    pub command: String,
    #[serde(default = "default_tactics_unit_id")]
    pub unit_id: String,
    #[serde(default)]
    pub target_tile: Option<String>,
    #[serde(default)]
    pub skill_id: Option<String>,
    #[serde(default)]
    pub npc_id: Option<String>,
    #[serde(default)]
    pub item_id: Option<String>,
    #[serde(default)]
    pub target_slot: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTacticsCommandOutcome {
    pub contract_version: String,
    pub accepted: bool,
    pub command: String,
    pub unit_id: String,
    pub result: String,
    #[serde(default)]
    pub rejection_reason: Option<String>,
    pub source_of_truth: String,
    pub web_role: String,
    #[serde(default)]
    pub skill_id: Option<String>,
    #[serde(default)]
    pub npc_id: Option<String>,
    #[serde(default)]
    pub item_id: Option<String>,
    #[serde(default)]
    pub item_instance_id: Option<String>,
    #[serde(default)]
    pub equipped_slot: Option<String>,
    #[serde(default)]
    pub target_tile: Option<String>,
    #[serde(default)]
    pub updated_at_epoch: i64,
    pub character: WorldTrillionniumCharacter,
}

fn default_tactics_unit_id() -> String {
    "lord".to_string()
}

pub fn apply_tactics_command(
    character: &mut WorldTrillionniumCharacter,
    request: WorldTacticsCommandRequest,
    now_epoch: i64,
) -> WorldTacticsCommandOutcome {
    character.ensure_defaults(now_epoch);
    let unit_id = if request.unit_id.trim().is_empty() {
        default_tactics_unit_id()
    } else {
        request.unit_id.clone()
    };
    match request.command.trim() {
        "train_skill" => apply_tactics_train_skill(character, request, &unit_id, now_epoch),
        "equip_item" => apply_tactics_equip_item(character, request, &unit_id, now_epoch),
        "attack" => apply_tactics_attack(character, request, &unit_id, now_epoch),
        "talk_npc" | "offer_task" => {
            let mut outcome = accepted_tactics_outcome(
                character,
                &request,
                &unit_id,
                if request.command == "offer_task" {
                    "task_offer_recorded"
                } else {
                    "npc_talk_recorded"
                },
                "rust_trillionnium_npc_interaction_validator",
                now_epoch,
            );
            outcome.npc_id = request.npc_id;
            character.title = if request.command == "offer_task" {
                "受领Trillionnium任务".to_string()
            } else {
                "Trillionnium有约".to_string()
            };
            outcome.character = character.clone();
            outcome
        }
        "complete_task" => {
            character.resource_pressure_state.apply_mutation(
                "tactics_complete_task",
                "complete_task",
                Some("task_completion_validated"),
                now_epoch,
            );
            character.region_story_unlock_state.apply_mutation(
                "tactics_complete_task",
                "complete_task",
                Some("task_completion_validated"),
                Some("delivery-dock"),
                Some("market-bazaar"),
                now_epoch,
            );
            character.title = "任务清账".to_string();
            character.updated_at_epoch = now_epoch;
            accepted_tactics_outcome(
                character,
                &request,
                &unit_id,
                "task_completion_validated",
                "rust_trillionnium_task_completion_handler",
                now_epoch,
            )
        }
        other => rejected_tactics_outcome(
            character,
            &request,
            &unit_id,
            "unknown_tactics_command",
            Some(format!("unsupported_command:{other}")),
            "rust_tactics_command_validator",
            now_epoch,
        ),
    }
}

fn apply_tactics_train_skill(
    character: &mut WorldTrillionniumCharacter,
    request: WorldTacticsCommandRequest,
    unit_id: &str,
    now_epoch: i64,
) -> WorldTacticsCommandOutcome {
    let skill_id = request
        .skill_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("basic_unarmed");
    if trillionnium_skill_definition_by_id(skill_id).is_none() {
        return rejected_tactics_outcome(
            character,
            &request,
            unit_id,
            "unknown_skill_id",
            Some("unknown_skill_id".to_string()),
            "rust_mentor_training_validator",
            now_epoch,
        );
    }
    let Some(training) = trillionnium_training_command_for_skill(skill_id) else {
        return rejected_tactics_outcome(
            character,
            &request,
            unit_id,
            "skill_has_no_training_command",
            Some("skill_has_no_training_command".to_string()),
            "rust_mentor_training_validator",
            now_epoch,
        );
    };
    if let Some(npc_id) = request
        .npc_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        if npc_id != training.mentor_npc_id {
            return rejected_tactics_outcome(
                character,
                &request,
                unit_id,
                "mentor_mismatch",
                Some("mentor_training_requires_matching_npc".to_string()),
                "rust_mentor_training_validator",
                now_epoch,
            );
        }
    }
    if !character.skill_ids.iter().any(|known| known == skill_id) {
        character.skill_ids.push(skill_id.to_string());
    }
    character.title = "得授新艺".to_string();
    character.updated_at_epoch = now_epoch;
    let mut outcome = accepted_tactics_outcome(
        character,
        &request,
        unit_id,
        "skill_training_recorded",
        "rust_mentor_training_validator",
        now_epoch,
    );
    outcome.skill_id = Some(skill_id.to_string());
    outcome.npc_id = Some(training.mentor_npc_id);
    outcome.character = character.clone();
    outcome
}

fn apply_tactics_equip_item(
    character: &mut WorldTrillionniumCharacter,
    request: WorldTacticsCommandRequest,
    unit_id: &str,
    now_epoch: i64,
) -> WorldTacticsCommandOutcome {
    let item_id = request
        .item_id
        .as_deref()
        .or(request.skill_id.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("route-guard-staff");
    let Some(catalog_slot) = trillionnium_catalog_item_field(item_id, "slot") else {
        return rejected_tactics_outcome(
            character,
            &request,
            unit_id,
            "unknown_item_id",
            Some("unknown_item_id".to_string()),
            "rust_trillionnium_item_equipment_runtime_state",
            now_epoch,
        );
    };
    if let Some(target_slot) = request
        .target_slot
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        if target_slot != catalog_slot {
            return rejected_tactics_outcome(
                character,
                &request,
                unit_id,
                "equipment_slot_mismatch",
                Some("equip_requires_matching_rust_catalog_slot".to_string()),
                "rust_trillionnium_item_equipment_runtime_state",
                now_epoch,
            );
        }
    }
    let Some((equipped_slot, item_instance_id)) = character.equip_item_by_id(item_id, now_epoch)
    else {
        return rejected_tactics_outcome(
            character,
            &request,
            unit_id,
            "item_not_in_inventory",
            Some("equip_requires_item_in_rust_owned_inventory".to_string()),
            "rust_trillionnium_item_equipment_runtime_state",
            now_epoch,
        );
    };
    character.title = "整备行囊".to_string();
    character.updated_at_epoch = now_epoch;
    let mut outcome = accepted_tactics_outcome(
        character,
        &request,
        unit_id,
        "item_equipped",
        "rust_trillionnium_item_equipment_runtime_state",
        now_epoch,
    );
    outcome.item_id = Some(item_id.to_string());
    outcome.item_instance_id = Some(item_instance_id);
    outcome.equipped_slot = Some(equipped_slot);
    outcome.character = character.clone();
    outcome
}

fn apply_tactics_attack(
    character: &mut WorldTrillionniumCharacter,
    request: WorldTacticsCommandRequest,
    unit_id: &str,
    now_epoch: i64,
) -> WorldTacticsCommandOutcome {
    let skill_id = request
        .skill_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("basic_unarmed");
    if !character.skill_ids.iter().any(|known| known == skill_id) {
        return rejected_tactics_outcome(
            character,
            &request,
            unit_id,
            "skill_locked",
            Some("required_skill_not_known".to_string()),
            "rust_tactics_command_validator",
            now_epoch,
        );
    }
    let target_tile = request
        .target_tile
        .clone()
        .unwrap_or_else(|| "F5".to_string());
    let resolution = TrillionniumCombatResolutionInput {
        attacker_unit_id: unit_id.to_string(),
        defender_unit_id: "market-bandit".to_string(),
        target_tile: target_tile.clone(),
        skill_id: skill_id.to_string(),
        damage: 30,
        defender_hp_before: 40,
        defender_hp_after: 10,
        result: "hit_landed".to_string(),
    };
    character.combat_numerics_state.apply_attack(
        "tactics_attack",
        "attack",
        &resolution,
        &character.attributes,
        now_epoch,
    );
    character.resource_pressure_state.apply_mutation(
        "tactics_attack",
        "attack",
        Some("tactics_combat_resolved"),
        now_epoch,
    );
    character.title = "街巷交锋".to_string();
    character.updated_at_epoch = now_epoch;
    let mut outcome = accepted_tactics_outcome(
        character,
        &request,
        unit_id,
        "tactics_combat_resolved",
        "rust_tactics_combat_handler",
        now_epoch,
    );
    outcome.skill_id = Some(skill_id.to_string());
    outcome.target_tile = Some(target_tile);
    outcome.character = character.clone();
    outcome
}

fn accepted_tactics_outcome(
    character: &WorldTrillionniumCharacter,
    request: &WorldTacticsCommandRequest,
    unit_id: &str,
    result: &str,
    source_of_truth: &str,
    now_epoch: i64,
) -> WorldTacticsCommandOutcome {
    WorldTacticsCommandOutcome {
        contract_version: TRILLIONNIUM_TACTICS_COMMAND_OUTCOME_CONTRACT_VERSION.to_string(),
        accepted: true,
        command: request.command.clone(),
        unit_id: unit_id.to_string(),
        result: result.to_string(),
        rejection_reason: None,
        source_of_truth: source_of_truth.to_string(),
        web_role: "intent_only_visualization_input".to_string(),
        skill_id: request.skill_id.clone(),
        npc_id: request.npc_id.clone(),
        item_id: request.item_id.clone(),
        item_instance_id: None,
        equipped_slot: None,
        target_tile: request.target_tile.clone(),
        updated_at_epoch: now_epoch,
        character: character.clone(),
    }
}

fn rejected_tactics_outcome(
    character: &WorldTrillionniumCharacter,
    request: &WorldTacticsCommandRequest,
    unit_id: &str,
    result: &str,
    rejection_reason: Option<String>,
    source_of_truth: &str,
    now_epoch: i64,
) -> WorldTacticsCommandOutcome {
    WorldTacticsCommandOutcome {
        contract_version: TRILLIONNIUM_TACTICS_COMMAND_OUTCOME_CONTRACT_VERSION.to_string(),
        accepted: false,
        command: request.command.clone(),
        unit_id: unit_id.to_string(),
        result: result.to_string(),
        rejection_reason,
        source_of_truth: source_of_truth.to_string(),
        web_role: "intent_only_visualization_input".to_string(),
        skill_id: request.skill_id.clone(),
        npc_id: request.npc_id.clone(),
        item_id: request.item_id.clone(),
        item_instance_id: None,
        equipped_slot: None,
        target_tile: request.target_tile.clone(),
        updated_at_epoch: now_epoch,
        character: character.clone(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldCommandDecision {
    pub accepted: bool,
    pub reason: String,
    pub source_of_truth: String,
    #[serde(default)]
    pub transition: Option<WorldTransitionDecision>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTransitionDecision {
    pub contract_version: String,
    pub source_of_truth: String,
    pub web_role: String,
    pub accepted: bool,
    pub result: String,
    pub transition_status: String,
    pub transition_kind: String,
    pub direction: String,
    pub target: String,
    pub current_node_id: String,
    pub target_node_id: Option<String>,
    pub to_node_id: Option<String>,
    pub current_location_id: String,
    pub target_location_id: Option<String>,
    pub current_zone_id: String,
    pub target_zone_id: Option<String>,
    pub changes_location: bool,
    pub changes_zone: bool,
    pub requires_interaction: bool,
    pub blocked_reason: Option<String>,
    pub user_message: String,
}

pub fn apply_command(state: &mut WorldState, command: WorldCommand) -> WorldCommandDecision {
    match command {
        WorldCommand::Move {
            actor_id,
            direction,
        } => move_actor(state, &actor_id, &direction),
        WorldCommand::TalkNpc { actor_id, npc_id } => local_npc_decision(state, &actor_id, &npc_id),
        WorldCommand::TrainSkill {
            actor_id,
            npc_id,
            skill_id,
        } => train_skill_decision(state, &actor_id, &npc_id, &skill_id),
        WorldCommand::CompleteTask { actor_id, task_id } => {
            complete_task_decision(state, &actor_id, &task_id)
        }
    }
}

fn move_actor(state: &mut WorldState, actor_id: &str, direction: &str) -> WorldCommandDecision {
    let Some(current_node_id) = state
        .positions
        .iter()
        .find(|position| position.actor_id == actor_id)
        .map(|position| position.node_id.clone())
    else {
        return reject("actor_position_missing");
    };

    let Some(current_node) = state.node(&current_node_id).cloned() else {
        return reject("current_node_missing");
    };

    let transition = world_transition_decision(state, &current_node, direction);
    if !transition.accepted {
        let reason = transition.result.clone();
        return reject_with_transition(&reason, Some(transition));
    }

    let Some(target_node_id) = transition.to_node_id.clone() else {
        return reject_with_transition("target_node_missing", Some(transition));
    };

    if state.node(&target_node_id).is_none() {
        return reject("target_node_missing");
    }

    if let Some(position) = state
        .positions
        .iter_mut()
        .find(|position| position.actor_id == actor_id)
    {
        position.node_id = target_node_id;
        position.source_of_truth = WORLD_RUST_SOURCE_OF_TRUTH.to_string();
        accept_with_transition("movement_committed_by_rust", Some(transition))
    } else {
        state.positions.push(WorldPosition {
            actor_id: actor_id.to_string(),
            node_id: target_node_id,
            source_of_truth: WORLD_RUST_SOURCE_OF_TRUTH.to_string(),
        });
        accept_with_transition("movement_position_created_by_rust", Some(transition))
    }
}

pub fn world_transition_aliases(target: &str) -> Vec<&'static str> {
    match target.trim().to_ascii_lowercase().as_str() {
        "8" | "n" | "north" => vec!["north", "n"],
        "2" | "s" | "south" => vec!["south", "s"],
        "4" | "w" | "west" => vec!["west", "w"],
        "6" | "e" | "east" => vec!["east", "e"],
        "7" | "nw" | "northwest" | "north-west" => vec!["north-west", "northwest", "nw"],
        "9" | "ne" | "northeast" | "north-east" => vec!["north-east", "northeast", "ne"],
        "1" | "sw" | "southwest" | "south-west" => vec!["south-west", "southwest", "sw"],
        "3" | "se" | "southeast" | "south-east" => vec!["south-east", "southeast", "se"],
        "5" | "wait" | "stay" | "here" => vec!["wait", "stay"],
        _ => Vec::new(),
    }
}

pub fn world_transition_primary_direction(target: &str) -> String {
    world_transition_aliases(target)
        .first()
        .copied()
        .unwrap_or_else(|| target.trim())
        .to_string()
}

fn world_transition_kind(current: &WorldNode, target: &WorldNode) -> String {
    if current.id == target.id {
        "wait".to_string()
    } else if current.region != target.region {
        "zone_transition".to_string()
    } else if current.location_id != target.location_id {
        "room_transition".to_string()
    } else {
        "local_exit".to_string()
    }
}

fn world_transition_status_for_target(target: &WorldNode) -> Option<(&'static str, &'static str)> {
    let status = target.status.trim();
    if status.eq_ignore_ascii_case("open") || status.is_empty() {
        None
    } else if status.eq_ignore_ascii_case("interaction_required")
        || status.starts_with("interaction_required")
        || status.starts_with("requires_")
    {
        Some(("interaction_required", "interaction_required"))
    } else {
        Some(("locked_route", "locked_route"))
    }
}

#[allow(clippy::too_many_arguments)]
fn transition_decision_base(
    current: &WorldNode,
    target: &str,
    accepted: bool,
    result: &str,
    transition_status: &str,
    transition_kind: &str,
    direction: &str,
    target_node: Option<&WorldNode>,
    blocked_reason: Option<String>,
    user_message: String,
) -> WorldTransitionDecision {
    WorldTransitionDecision {
        contract_version: WORLD_TRANSITION_SEMANTICS_CONTRACT.to_string(),
        source_of_truth: "rust_world_map_transition_rules".to_string(),
        web_role: "intent_only_visualization_input".to_string(),
        accepted,
        result: result.to_string(),
        transition_status: transition_status.to_string(),
        transition_kind: transition_kind.to_string(),
        direction: direction.to_string(),
        target: target.to_string(),
        current_node_id: current.id.clone(),
        target_node_id: target_node.map(|node| node.id.clone()),
        to_node_id: target_node.map(|node| node.id.clone()),
        current_location_id: current.location_id.clone(),
        target_location_id: target_node.map(|node| node.location_id.clone()),
        current_zone_id: current.region.clone(),
        target_zone_id: target_node.map(|node| node.region.clone()),
        changes_location: target_node
            .map(|node| node.location_id != current.location_id)
            .unwrap_or(false),
        changes_zone: target_node
            .map(|node| node.region != current.region)
            .unwrap_or(false),
        requires_interaction: result == "interaction_required",
        blocked_reason,
        user_message,
    }
}

pub fn world_transition_decision(
    state: &WorldState,
    current: &WorldNode,
    target: &str,
) -> WorldTransitionDecision {
    let target_trimmed = target.trim();
    let aliases = world_transition_aliases(target_trimmed);
    let direction = world_transition_primary_direction(target_trimmed);

    if aliases.contains(&"wait") || target_trimmed == current.id {
        return transition_decision_base(
            current,
            target_trimmed,
            true,
            "wait",
            "accepted",
            "wait",
            "wait",
            Some(current),
            None,
            "Wait in the current room.".to_string(),
        );
    }

    let mut resolved_direction = direction.clone();
    let mut target_node_id = None;
    for candidate in aliases
        .iter()
        .copied()
        .chain(std::iter::once(target_trimmed))
    {
        if let Some(edge) = state
            .edges
            .iter()
            .find(|edge| edge.from == current.id && edge.direction == candidate)
        {
            resolved_direction = candidate.to_string();
            target_node_id = Some(edge.to.clone());
            break;
        }
    }

    if target_node_id.is_none()
        && state
            .edges
            .iter()
            .any(|edge| edge.from == current.id && edge.to == target_trimmed)
    {
        target_node_id = Some(target_trimmed.to_string());
        resolved_direction = state
            .edges
            .iter()
            .find_map(|edge| {
                if edge.from == current.id && edge.to == target_trimmed {
                    Some(edge.direction.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| target_trimmed.to_string());
    }

    if target_node_id.is_none() && !aliases.is_empty() {
        return transition_decision_base(
            current,
            target_trimmed,
            false,
            "blocked_terrain",
            "blocked",
            "blocked_terrain",
            &direction,
            None,
            Some("no_exit_for_direction".to_string()),
            "That direction has no open exit from the current room.".to_string(),
        );
    }

    let target_node_id = target_node_id.unwrap_or_else(|| target_trimmed.to_string());
    let Some(target_node) = state.node(&target_node_id) else {
        return transition_decision_base(
            current,
            target_trimmed,
            false,
            "unknown_target",
            "blocked",
            "unknown_target",
            &resolved_direction,
            None,
            Some("target_node_missing".to_string()),
            "The target room does not exist in the Rust world graph.".to_string(),
        );
    };

    let is_direct_exit = target_node.id == current.id
        || state
            .edges
            .iter()
            .any(|edge| edge.from == current.id && edge.to == target_node.id);
    if !is_direct_exit {
        return transition_decision_base(
            current,
            target_trimmed,
            false,
            "non_adjacent_route",
            "locked",
            "locked_route",
            &resolved_direction,
            Some(target_node),
            Some("target_not_in_current_exits".to_string()),
            "That room is visible in the world graph but is not adjacent from here.".to_string(),
        );
    }

    if let Some((result, transition_kind)) = world_transition_status_for_target(target_node) {
        return transition_decision_base(
            current,
            target_trimmed,
            false,
            result,
            "locked",
            transition_kind,
            &resolved_direction,
            Some(target_node),
            Some(format!("target_status:{}", target_node.status)),
            if result == "interaction_required" {
                "That route needs a local interaction before the player can enter.".to_string()
            } else {
                "That route is locked by the Rust world state.".to_string()
            },
        );
    }

    let transition_kind = world_transition_kind(current, target_node);
    transition_decision_base(
        current,
        target_trimmed,
        true,
        "open_exit",
        "accepted",
        &transition_kind,
        &resolved_direction,
        Some(target_node),
        None,
        if transition_kind == "zone_transition" || transition_kind == "room_transition" {
            "Move accepted; this crosses into another room/zone projection.".to_string()
        } else {
            "Move accepted through an adjacent local exit.".to_string()
        },
    )
}

fn local_npc_decision(state: &WorldState, actor_id: &str, npc_id: &str) -> WorldCommandDecision {
    let Some(position) = state
        .positions
        .iter()
        .find(|position| position.actor_id == actor_id)
    else {
        return reject("actor_position_missing");
    };
    let Some(npc) = state.npcs.iter().find(|npc| npc.id == npc_id) else {
        return reject("npc_missing");
    };
    if npc.node_id != position.node_id {
        return reject("npc_not_local");
    }
    accept("npc_interaction_available")
}

fn train_skill_decision(
    state: &WorldState,
    actor_id: &str,
    npc_id: &str,
    skill_id: &str,
) -> WorldCommandDecision {
    let npc_decision = local_npc_decision(state, actor_id, npc_id);
    if !npc_decision.accepted {
        return npc_decision;
    }
    let Some(npc) = state.npcs.iter().find(|npc| npc.id == npc_id) else {
        return reject("npc_missing");
    };
    if npc.teaches_skills.iter().any(|skill| skill == skill_id) {
        accept("mentor_training_available")
    } else {
        reject("mentor_skill_mismatch")
    }
}

fn complete_task_decision(
    state: &WorldState,
    actor_id: &str,
    task_id: &str,
) -> WorldCommandDecision {
    let Some(position) = state
        .positions
        .iter()
        .find(|position| position.actor_id == actor_id)
    else {
        return reject("actor_position_missing");
    };
    let Some(task) = state.tasks.iter().find(|task| task.id == task_id) else {
        return reject("task_missing");
    };
    if task.node_id != position.node_id {
        return reject("task_not_local");
    }
    if task.ledger_settlement_required {
        return accept("task_completion_requires_ledger_settlement");
    }
    accept("task_completion_available")
}

fn accept(reason: &str) -> WorldCommandDecision {
    accept_with_transition(reason, None)
}

fn accept_with_transition(
    reason: &str,
    transition: Option<WorldTransitionDecision>,
) -> WorldCommandDecision {
    WorldCommandDecision {
        accepted: true,
        reason: reason.to_string(),
        source_of_truth: WORLD_RUST_SOURCE_OF_TRUTH.to_string(),
        transition,
    }
}

fn reject(reason: &str) -> WorldCommandDecision {
    reject_with_transition(reason, None)
}

fn reject_with_transition(
    reason: &str,
    transition: Option<WorldTransitionDecision>,
) -> WorldCommandDecision {
    WorldCommandDecision {
        accepted: false,
        reason: reason.to_string(),
        source_of_truth: WORLD_RUST_SOURCE_OF_TRUTH.to_string(),
        transition,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trnm_world_domain::{WorldEdge, WorldNode};

    #[test]
    fn move_command_mutates_position_only_through_rust() {
        let mut state = WorldState::fixture();
        let decision = apply_command(
            &mut state,
            WorldCommand::Move {
                actor_id: "local-player".to_string(),
                direction: "east".to_string(),
            },
        );
        assert!(decision.accepted);
        assert_eq!(
            state
                .positions
                .iter()
                .find(|p| p.actor_id == "local-player")
                .unwrap()
                .node_id,
            "league-coliseum"
        );
        state.validate_authority().unwrap();
    }

    #[test]
    fn mentor_training_is_local_and_skill_checked() {
        let state = WorldState::fixture();
        let decision = apply_command(
            &mut state.clone(),
            WorldCommand::TrainSkill {
                actor_id: "local-player".to_string(),
                npc_id: "npc-street-compass-sifu".to_string(),
                skill_id: "basic_unarmed".to_string(),
            },
        );
        assert_eq!(decision.reason, "mentor_training_available");
    }

    #[test]
    fn move_command_uses_extracted_cex_transition_semantics() {
        let mut state = WorldState::fixture();
        let decision = apply_command(
            &mut state,
            WorldCommand::Move {
                actor_id: "local-player".to_string(),
                direction: "6".to_string(),
            },
        );
        assert!(decision.accepted);
        let transition = decision.transition.expect("transition decision");
        assert_eq!(
            transition.contract_version,
            WORLD_TRANSITION_SEMANTICS_CONTRACT
        );
        assert_eq!(
            transition.source_of_truth,
            "rust_world_map_transition_rules"
        );
        assert_eq!(transition.web_role, "intent_only_visualization_input");
        assert_eq!(transition.result, "open_exit");
        assert_eq!(transition.direction, "east");
        assert_eq!(transition.transition_kind, "local_exit");
        assert_eq!(transition.to_node_id.as_deref(), Some("league-coliseum"));
    }

    #[test]
    fn transition_semantics_fail_closed_for_blocked_locked_and_non_adjacent_routes() {
        let mut state = WorldState::fixture();
        state.nodes.push(WorldNode {
            id: "locked-room".to_string(),
            name: "Locked Room".to_string(),
            region: "fixture-osm".to_string(),
            location_id: "mirror-city".to_string(),
            node_kind: "locked".to_string(),
            description: "Locked transition fixture".to_string(),
            status: "locked".to_string(),
            lat_e7: 312_311_000,
            lng_e7: 1_214_741_000,
            tags: vec![],
        });
        state.nodes.push(WorldNode {
            id: "visible-but-non-adjacent".to_string(),
            name: "Visible Non Adjacent".to_string(),
            region: "fixture-osm".to_string(),
            location_id: "mirror-city".to_string(),
            node_kind: "distant".to_string(),
            description: "Visible but not directly connected".to_string(),
            status: "open".to_string(),
            lat_e7: 312_312_000,
            lng_e7: 1_214_742_000,
            tags: vec![],
        });
        state.edges.push(WorldEdge {
            from: "mirror-city-square".to_string(),
            to: "locked-room".to_string(),
            direction: "north".to_string(),
        });
        let current = state.node("mirror-city-square").unwrap().clone();

        let blocked = world_transition_decision(&state, &current, "south");
        assert!(!blocked.accepted);
        assert_eq!(blocked.result, "blocked_terrain");
        assert_eq!(
            blocked.blocked_reason.as_deref(),
            Some("no_exit_for_direction")
        );

        let locked = world_transition_decision(&state, &current, "north");
        assert!(!locked.accepted);
        assert_eq!(locked.result, "locked_route");
        assert_eq!(locked.transition_status, "locked");
        assert_eq!(locked.to_node_id.as_deref(), Some("locked-room"));

        let non_adjacent = world_transition_decision(&state, &current, "visible-but-non-adjacent");
        assert!(!non_adjacent.accepted);
        assert_eq!(non_adjacent.result, "non_adjacent_route");
        assert_eq!(
            non_adjacent.blocked_reason.as_deref(),
            Some("target_not_in_current_exits")
        );
    }

    #[test]
    fn classifies_world_action_kind_with_cex_contract() {
        let contract = classify_world_action("把客户委托拆成 deliverable 和 next action");
        assert_eq!(contract.contract_version, WORLD_ACTION_ENGINE_CONTRACT);
        assert_eq!(contract.kind, "contract");
        assert_eq!(contract.base_impact, 20);

        let venture = classify_world_action("open a studio shop for the team");
        assert_eq!(venture.kind, "venture");
        assert_eq!(venture.base_impact, 18);

        let fallback = classify_world_action("walk around and observe");
        assert_eq!(fallback.kind, "explore");
        assert_eq!(fallback.base_impact, 10);
    }

    #[test]
    fn scores_world_action_quality_signals_without_cex_runtime() {
        let high = world_action_quality_signal(
            "deliverable evidence risk next review 自检 交付 证据 风险 下一步 plus a sufficiently long implementation note for the action gate",
        );
        assert!(high.score >= 6);
        assert!(high.missing.is_empty());

        let low = world_action_quality_signal("quick idea");
        assert_eq!(low.score, 0);
        assert!(low.missing.iter().any(|signal| signal == "deliverable"));
    }
}
