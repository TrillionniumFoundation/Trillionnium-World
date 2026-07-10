//! Trillionnium World standalone domain contracts.
//!
//! These types are intentionally independent of CEX service internals. They are
//! the first landing zone for the CEX `/world` incubator split.

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

pub const WORLD_DOMAIN_CONTRACT: &str = "trillionnium_world_domain_v1";
pub const WORLD_RUST_SOURCE_OF_TRUTH: &str = "rust_world_state_projection";
pub const WORLD_CEX_INCUBATOR_SOURCE: &str = "cex_incubator_source_of_evidence";
pub const WORLD_TRANSITION_SEMANTICS_CONTRACT: &str = "trillionnium_world_transition_semantics_v1";
pub const TRILLIONNIUM_CHARACTER_CONTRACT_VERSION: &str = "trillionnium_character_v1";
pub const TRILLIONNIUM_TACTICS_BOARD_CONTRACT_VERSION: &str = "trillionnium_tactics_board_v1";
pub const TRILLIONNIUM_TACTICS_UNIT_CONTRACT_VERSION: &str = "trillionnium_tactics_unit_v1";
pub const TRILLIONNIUM_TACTICS_COMMAND_CONTRACT_VERSION: &str = "trillionnium_tactics_command_v1";
pub const TRILLIONNIUM_TACTICS_COMMAND_OUTCOME_CONTRACT_VERSION: &str =
    "trillionnium_tactics_command_outcome_v1";
pub const TRILLIONNIUM_SKILL_CONTRACT_VERSION: &str = "trillionnium_skill_v1";
pub const TRILLIONNIUM_TRAINING_CONTRACT_VERSION: &str = "trillionnium_training_command_v1";
pub const TRILLIONNIUM_SECT_OSM_BINDING_CONTRACT_VERSION: &str = "trillionnium_sect_osm_binding_v1";
pub const TRILLIONNIUM_TASK_ARCHETYPE_CONTRACT_VERSION: &str = "trillionnium_task_archetype_v1";
pub const TRILLIONNIUM_MENTOR_TRAINING_TASK_CONTRACT_VERSION: &str =
    "trillionnium_mentor_training_task_v1";
pub const TRILLIONNIUM_TASK_COMPLETION_CONTRACT_VERSION: &str = "trillionnium_task_completion_v1";
pub const TRILLIONNIUM_REWARD_GATE_CONTRACT_VERSION: &str = "trillionnium_reward_gate_v1";
pub const TRILLIONNIUM_BATTLE_LOG_STYLE_CONTRACT_VERSION: &str = "trillionnium_battle_log_style_v1";
pub const TRILLIONNIUM_COMBAT_LOG_CONTRACT_VERSION: &str = "trillionnium_combat_log_v1";
pub const TRILLIONNIUM_SECT_CONTRACT_VERSION: &str = "trillionnium_sect_v1";
pub const TRILLIONNIUM_NPC_CONTRACT_VERSION: &str = "trillionnium_npc_v1";
pub const TRILLIONNIUM_NPC_SPAWN_CONTRACT_VERSION: &str = "trillionnium_npc_spawn_anchor_v1";
pub const TRILLIONNIUM_NPC_COMMAND_DESCRIPTOR_CONTRACT_VERSION: &str =
    "trillionnium_npc_command_descriptor_v1";
pub const TRILLIONNIUM_NPC_RELATIONSHIP_CONTRACT_VERSION: &str = "trillionnium_npc_relationship_v1";
pub const TRILLIONNIUM_OSM_OBJECTIVE_CONTRACT_VERSION: &str =
    "trillionnium_osm_objective_binding_v1";
pub const TRILLIONNIUM_TACTICS_GAME_SESSION_CONTRACT_VERSION: &str =
    "trillionnium_tactics_game_session_v1";
pub const TRILLIONNIUM_TACTICS_SIMULATION_TICK_CONTRACT_VERSION: &str =
    "trillionnium_tactics_simulation_tick_v1";
pub const TRILLIONNIUM_TACTICS_REWARD_SETTLEMENT_CONTRACT_VERSION: &str =
    "trillionnium_tactics_reward_settlement_v1";
pub const TRILLIONNIUM_TACTICS_REPEAT_FARMING_ANTI_CHEESE_CONTRACT_VERSION: &str =
    "trillionnium_tactics_repeat_farming_anti_cheese_v1";
pub const TRILLIONNIUM_TACTICS_BOARD_CELL_INTERACTION_CONTRACT_VERSION: &str =
    "trillionnium_tactics_board_cell_interaction_v1";
pub const TRILLIONNIUM_TACTICS_UNIT_SELECTION_CONTRACT_VERSION: &str =
    "trillionnium_tactics_unit_selection_v1";
pub const TRILLIONNIUM_TACTICS_COMMAND_INTENT_DRAFT_CONTRACT_VERSION: &str =
    "trillionnium_tactics_command_intent_draft_v1";
pub const TRILLIONNIUM_TACTICS_ACCESSIBILITY_CONTRACT_VERSION: &str =
    "trillionnium_tactics_accessibility_v1";
pub const TRILLIONNIUM_MAP_OVERLAY_IDENTITY_CONTRACT_VERSION: &str =
    "trillionnium_map_overlay_identity_v1";
pub const TRILLIONNIUM_WORLD_OBJECTIVE_TRAVEL_CONTRACT_VERSION: &str =
    "trillionnium_world_objective_travel_v1";
pub const TRILLIONNIUM_WORLD_SKILL_PRACTICE_LOOP_CONTRACT_VERSION: &str =
    "trillionnium_world_skill_practice_loop_v1";
pub const TRILLIONNIUM_WORLD_COMBAT_ENCOUNTER_LOOP_CONTRACT_VERSION: &str =
    "trillionnium_world_combat_encounter_loop_v1";
pub const TRILLIONNIUM_HERO_TAN_FULL_CONTENT_ALIGNMENT_CONTRACT_VERSION: &str =
    "trillionnium_hero_tan_full_content_alignment_v1";
pub const TRILLIONNIUM_WORLD_ITEM_EQUIPMENT_RUNTIME_CONTRACT_VERSION: &str =
    "trillionnium_world_item_equipment_runtime_v1";
pub const TRILLIONNIUM_WORLD_RESOURCE_PRESSURE_RUNTIME_CONTRACT_VERSION: &str =
    "trillionnium_world_resource_pressure_runtime_v1";
pub const TRILLIONNIUM_WORLD_FOOD_WATER_AGE_SURVIVAL_CONTRACT_VERSION: &str =
    "trillionnium_world_food_water_age_survival_v1";
pub const TRILLIONNIUM_WORLD_DYNAMIC_SOCIAL_SIMULATION_CONTRACT_VERSION: &str =
    "trillionnium_world_dynamic_social_simulation_v1";
pub const TRILLIONNIUM_WORLD_AUTHORED_QUEST_CHAIN_CONTRACT_VERSION: &str =
    "trillionnium_world_authored_quest_chain_v1";
pub const TRILLIONNIUM_WORLD_REGION_STORY_UNLOCK_RUNTIME_CONTRACT_VERSION: &str =
    "trillionnium_world_region_story_unlock_runtime_v1";
pub const TRILLIONNIUM_WORLD_COMBAT_NUMERICS_RUNTIME_CONTRACT_VERSION: &str =
    "trillionnium_world_combat_numerics_runtime_v1";
pub const TRILLIONNIUM_TACTICS_COMBAT_RESOLUTION_CONTRACT_VERSION: &str =
    "trillionnium_tactics_combat_resolution_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldNode {
    pub id: String,
    pub name: String,
    pub region: String,
    #[serde(default)]
    pub location_id: String,
    #[serde(default)]
    pub node_kind: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub status: String,
    pub lat_e7: i32,
    pub lng_e7: i32,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldEdge {
    pub from: String,
    pub to: String,
    pub direction: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldPosition {
    pub actor_id: String,
    pub node_id: String,
    pub source_of_truth: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldSkill {
    pub id: String,
    pub level: u32,
    pub mentor_npc_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldNpc {
    pub id: String,
    pub name: String,
    pub node_id: String,
    #[serde(default)]
    pub teaches_skills: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTask {
    pub id: String,
    pub title: String,
    pub node_id: String,
    pub reward_units: u64,
    pub ledger_settlement_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldReceipt {
    pub id: String,
    pub progression_class: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldState {
    pub contract_version: String,
    pub source: String,
    pub nodes: Vec<WorldNode>,
    pub edges: Vec<WorldEdge>,
    pub positions: Vec<WorldPosition>,
    pub npcs: Vec<WorldNpc>,
    pub tasks: Vec<WorldTask>,
    #[serde(default)]
    pub receipts: Vec<WorldReceipt>,
}

impl WorldState {
    pub fn fixture() -> Self {
        Self {
            contract_version: WORLD_DOMAIN_CONTRACT.to_string(),
            source: WORLD_CEX_INCUBATOR_SOURCE.to_string(),
            nodes: vec![
                WorldNode {
                    id: "mirror-city-square".to_string(),
                    name: "Mirror City Square".to_string(),
                    region: "fixture-osm".to_string(),
                    location_id: "mirror-city".to_string(),
                    node_kind: "mentor".to_string(),
                    description: "Spawn square with a local mentor.".to_string(),
                    status: "open".to_string(),
                    lat_e7: 312_304_160,
                    lng_e7: 1_214_737_010,
                    tags: vec!["spawn".to_string(), "mentor".to_string()],
                },
                WorldNode {
                    id: "league-coliseum".to_string(),
                    name: "League Coliseum".to_string(),
                    region: "fixture-osm".to_string(),
                    location_id: "mirror-city".to_string(),
                    node_kind: "combat".to_string(),
                    description: "First objective and combat entry.".to_string(),
                    status: "open".to_string(),
                    lat_e7: 312_310_000,
                    lng_e7: 1_214_740_000,
                    tags: vec!["combat".to_string(), "objective".to_string()],
                },
            ],
            edges: vec![WorldEdge {
                from: "mirror-city-square".to_string(),
                to: "league-coliseum".to_string(),
                direction: "east".to_string(),
            }],
            positions: vec![WorldPosition {
                actor_id: "local-player".to_string(),
                node_id: "mirror-city-square".to_string(),
                source_of_truth: WORLD_RUST_SOURCE_OF_TRUTH.to_string(),
            }],
            npcs: vec![WorldNpc {
                id: "npc-street-compass-sifu".to_string(),
                name: "Street Compass Sifu".to_string(),
                node_id: "mirror-city-square".to_string(),
                teaches_skills: vec!["basic_unarmed".to_string()],
            }],
            tasks: vec![WorldTask {
                id: "task-fixture-first-route".to_string(),
                title: "Reach the first objective".to_string(),
                node_id: "league-coliseum".to_string(),
                reward_units: 10,
                ledger_settlement_required: true,
            }],
            receipts: vec![],
        }
    }

    pub fn trillionnium_default_map_fixture() -> Self {
        Self::cex_default_map_fixture()
    }

    pub fn cex_default_map_fixture() -> Self {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "mirror-city-square",
            "mirror-city-square",
            "reality-mirror-city",
            "镜像城市广场",
            "hub_square",
            "经典文字冒险式的开局主广场：公告牌、Ledger Clerk、Agent 居民和现实任务入口都在附近。",
            0,
            0,
            &[
                ("east", "starter-studio"),
                ("south", "zbj-market-gate"),
                ("north", "league-coliseum"),
                ("west", "agent-dormitory"),
            ],
            &["talk", "notice_board", "ledger", "meet_agents"],
            &[
                "和 NPC 打听任务",
                "贴公告招募 Agent",
                "把现实想法登记成 World action",
            ],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "agent-dormitory",
            "mirror-city-square",
            "reality-mirror-city",
            "Agent 宿舍巷",
            "agent_home",
            "自由招募、聊天、派遣 Agent 的小巷，适合做关系和队伍系统。",
            -1,
            0,
            &[("east", "mirror-city-square"), ("south", "ledger-office")],
            &["recruit", "relationship", "party"],
            &["拜访 Agent", "组队出门", "训练专长"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "ledger-office",
            "mirror-city-square",
            "reality-mirror-city",
            "Ledger 办事处",
            "ledger_office",
            "余额、预留款、合同与退款的城市窗口。",
            -1,
            1,
            &[("north", "agent-dormitory"), ("east", "zbj-market-gate")],
            &["wallet", "refund", "contract"],
            &["查账", "发起争议", "登记合约"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "starter-studio",
            "starter-studio",
            "craft-district",
            "新手工坊",
            "workshop_room",
            "第一间可布置工坊，Gather 风格的冒险房间，也是公会据点起点。",
            1,
            0,
            &[
                ("west", "mirror-city-square"),
                ("east", "forge-workbench"),
                ("south", "asset-yard"),
            ],
            &["craft", "guild", "decorate"],
            &["摆放道具", "升级工坊", "创建据点"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "forge-workbench",
            "starter-studio",
            "craft-district",
            "锻造工坊",
            "craft_station",
            "把想法打磨成方案、素材、道具和成果包的工作台。",
            2,
            0,
            &[("west", "starter-studio"), ("south", "asset-yard")],
            &["craft", "upgrade", "review"],
            &["生成道具", "审稿", "做成果包"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "asset-yard",
            "starter-studio",
            "craft-district",
            "道具庭院",
            "asset_yard",
            "收纳道具、素材和展示件的庭院，后续可自由布置。",
            1,
            1,
            &[("north", "starter-studio"), ("east", "zbj-market-gate")],
            &["asset", "inventory", "display"],
            &["查看道具", "升级道具", "挂到摊位"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "zbj-market-gate",
            "zbj-market-gate",
            "market-bazaar",
            "悬赏集市门",
            "market_gate",
            "现实机会映射成世界悬赏的入口。",
            0,
            2,
            &[
                ("north", "mirror-city-square"),
                ("west", "asset-yard"),
                ("east", "client-board"),
                ("south", "dispute-desk"),
            ],
            &["market", "contract", "bounty", "service"],
            &["接悬赏", "发布服务", "招募队友"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "client-board",
            "zbj-market-gate",
            "market-bazaar",
            "悬赏任务牌",
            "client_board",
            "像文字 MUD 的公告栏：任务、悬赏、提示和线索都贴在这里。",
            1,
            2,
            &[("west", "zbj-market-gate"), ("east", "delivery-dock")],
            &["listing", "bounty", "brief"],
            &["浏览悬赏", "接取挑战", "发布服务"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "delivery-dock",
            "zbj-market-gate",
            "market-bazaar",
            "成果评定台",
            "delivery_dock",
            "成果提交、评级、返工和放弃都在这里形成冒险路线。",
            2,
            2,
            &[("west", "client-board"), ("north", "league-coliseum")],
            &["deliver", "accept", "reject", "cancel"],
            &["提交成果", "评级", "发起返工或放弃"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "dispute-desk",
            "zbj-market-gate",
            "market-bazaar",
            "争议柜台",
            "dispute_desk",
            "后续 dispute / review hold / 仲裁规则的入口。",
            0,
            3,
            &[("north", "zbj-market-gate"), ("south", "witness-archive")],
            &["dispute", "refund", "review"],
            &["申请仲裁", "查看退款", "提交证据"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "league-coliseum",
            "league-coliseum",
            "league-arena",
            "League 竞技场",
            "arena_gate",
            "League 入口，任务可以从悬赏集市带进竞技场评级。",
            0,
            -1,
            &[
                ("south", "mirror-city-square"),
                ("east", "raid-hall"),
                ("south-east", "delivery-dock"),
            ],
            &["arena", "ranking", "judge"],
            &["加入赛场", "提交比赛", "查看排行"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "raid-hall",
            "league-coliseum",
            "league-arena",
            "公会团本厅",
            "raid_hall",
            "多人协作任务和 Agent 阵容站位的大厅。",
            1,
            -1,
            &[("west", "league-coliseum"), ("east", "guild-vault")],
            &["guild", "raid", "team"],
            &["认领职责", "组队打本", "分配 Agent"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "witness-archive",
            "witness-archive",
            "civic-watch",
            "见证档案馆",
            "witness_archive",
            "保存任务证词、争议记录和关系事件的档案馆，用原创证据链替代任何外部剧情表。",
            0,
            4,
            &[
                ("north", "dispute-desk"),
                ("east", "night-watch-yard"),
                ("south", "elder-step"),
            ],
            &["archive", "witness", "relationship", "proof"],
            &["查阅证词", "整理关系事件", "补全争议证据"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "night-watch-yard",
            "night-watch-yard",
            "civic-watch",
            "夜巡校场",
            "night_watch_yard",
            "训练夜间移动、暗线传信和巡逻路线的校场，承担长期行动风险教学。",
            1,
            4,
            &[
                ("west", "witness-archive"),
                ("east", "river-cistern"),
                ("south", "courier-yard"),
            ],
            &["patrol", "stealth", "messaging", "night"],
            &["练夜巡步", "接暗线信使任务", "检查巡逻风险"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "river-cistern",
            "river-cistern",
            "survival-belt",
            "河湾水仓",
            "river_cistern",
            "管理饮水、补给和长线行军压力的水仓，服务 food/water/age 生存循环。",
            2,
            4,
            &[("west", "night-watch-yard"), ("south", "ration-kitchen")],
            &["water", "survival", "restock", "weather"],
            &["补水", "检查脱水风险", "安排雨季路线"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "ration-kitchen",
            "ration-kitchen",
            "survival-belt",
            "行灶补给棚",
            "ration_kitchen",
            "烹饪干粮、分配队伍补给和恢复远行体力的原创生存节点。",
            2,
            5,
            &[("north", "river-cistern"), ("west", "field-infirmary")],
            &["food", "cooking", "party_supply", "rest"],
            &["做行粮", "分配队伍补给", "降低饥饿风险"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "field-infirmary",
            "field-infirmary",
            "survival-belt",
            "野外医棚",
            "field_infirmary",
            "处理轻伤、疲劳和长期年龄压力的医棚，连接战斗失败后的恢复路线。",
            1,
            5,
            &[
                ("east", "ration-kitchen"),
                ("west", "elder-step"),
                ("south", "caravan-rest-camp"),
            ],
            &["medicine", "injury", "recovery", "age"],
            &["处理轻伤", "调配补剂", "评估长期体能"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "elder-step",
            "elder-step",
            "civic-watch",
            "长者石阶",
            "elder_step",
            "长者和居民调解公共关系的石阶，推动 NPC 信任、冲突热度和派系站位变化。",
            0,
            5,
            &[
                ("north", "witness-archive"),
                ("east", "field-infirmary"),
                ("south", "mentor-cloister"),
            ],
            &["elder", "mediation", "social", "trust"],
            &["听取传闻", "调停关系", "恢复导师信任"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "mentor-cloister",
            "mentor-cloister",
            "civic-watch",
            "导师回廊",
            "mentor_cloister",
            "原创导师回廊，承载门派称号、训练试炼和长期成长路线，而不是引用外部门派表。",
            0,
            6,
            &[("north", "elder-step"), ("east", "caravan-rest-camp")],
            &["mentor", "sect", "training", "title_ladder"],
            &["拜访导师", "进行门籍试炼", "规划长期成长"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "courier-yard",
            "courier-yard",
            "civic-watch",
            "信使马厩",
            "courier_yard",
            "处理远程送达、信使队伍和路线接力的马厩，连接地图探索和任务物流。",
            1,
            5,
            &[
                ("north", "night-watch-yard"),
                ("east", "survey-tower"),
                ("south", "auction-arcade"),
            ],
            &["courier", "route", "handoff", "party"],
            &["安排信使", "接力路线", "追踪队伍位置"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "survey-tower",
            "survey-tower",
            "survey-ridge",
            "地势测绘塔",
            "survey_tower",
            "观察地形、记录阻挡规则和规划探索路线的高塔。",
            2,
            5,
            &[("west", "courier-yard"), ("south", "guild-vault")],
            &["terrain", "survey", "map", "blocked_path"],
            &["测绘地形", "标注阻挡", "优化路线"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "guild-vault",
            "guild-vault",
            "league-arena",
            "公会库房",
            "guild_vault",
            "保管团队装备、团本凭证和结算证据的公会库房。",
            2,
            -1,
            &[
                ("west", "raid-hall"),
                ("north", "survey-tower"),
                ("south-west", "auction-arcade"),
            ],
            &["guild", "inventory", "raid", "evidence"],
            &["整理团本凭证", "保管装备", "复核结算证据"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "auction-arcade",
            "auction-arcade",
            "market-bazaar",
            "拍卖廊",
            "auction_arcade",
            "原创市场拍卖廊，承载鉴定、竞价、补给和商路选择。",
            1,
            6,
            &[
                ("north", "courier-yard"),
                ("north-east", "guild-vault"),
                ("south-east", "caravan-rest-camp"),
            ],
            &["auction", "appraisal", "commerce", "supply"],
            &["鉴定物件", "竞价补给", "选择商路"],
        );
        push_cex_default_map_node(
            &mut nodes,
            &mut edges,
            "caravan-rest-camp",
            "caravan-rest-camp",
            "survival-belt",
            "商队歇脚营",
            "caravan_rest_camp",
            "长途路线中的休整营地，压力来自补给、时间、队伍疲劳和事件等待。",
            1,
            7,
            &[
                ("north", "field-infirmary"),
                ("west", "mentor-cloister"),
                ("north-west", "auction-arcade"),
            ],
            &["camp", "time", "fatigue", "caravan"],
            &["扎营休整", "计算行程天数", "处理队伍疲劳"],
        );

        Self {
            contract_version: WORLD_DOMAIN_CONTRACT.to_string(),
            source: WORLD_CEX_INCUBATOR_SOURCE.to_string(),
            nodes,
            edges,
            positions: vec![WorldPosition {
                actor_id: "local-player".to_string(),
                node_id: "mirror-city-square".to_string(),
                source_of_truth: WORLD_RUST_SOURCE_OF_TRUTH.to_string(),
            }],
            npcs: vec![],
            tasks: vec![],
            receipts: vec![],
        }
    }

    pub fn validate_authority(&self) -> Result<(), String> {
        if self.contract_version != WORLD_DOMAIN_CONTRACT {
            return Err(format!(
                "unexpected world contract: {}",
                self.contract_version
            ));
        }
        for position in &self.positions {
            if position.source_of_truth != WORLD_RUST_SOURCE_OF_TRUTH {
                return Err(format!(
                    "position {} is not Rust-owned: {}",
                    position.actor_id, position.source_of_truth
                ));
            }
        }
        Ok(())
    }

    pub fn node(&self, id: &str) -> Option<&WorldNode> {
        self.nodes.iter().find(|node| node.id == id)
    }
}

#[allow(clippy::too_many_arguments)]
fn push_cex_default_map_node(
    nodes: &mut Vec<WorldNode>,
    edges: &mut Vec<WorldEdge>,
    id: &str,
    location_id: &str,
    region: &str,
    name: &str,
    node_kind: &str,
    description: &str,
    x: i32,
    y: i32,
    exits: &[(&str, &str)],
    tags: &[&str],
    hooks: &[&str],
) {
    nodes.push(WorldNode {
        id: id.to_string(),
        name: name.to_string(),
        region: region.to_string(),
        location_id: location_id.to_string(),
        node_kind: node_kind.to_string(),
        description: description.to_string(),
        status: "open".to_string(),
        lat_e7: y,
        lng_e7: x,
        tags: tags
            .iter()
            .chain(hooks.iter())
            .chain(std::iter::once(&node_kind))
            .map(|value| (*value).to_string())
            .collect(),
    });
    edges.extend(exits.iter().map(|(direction, target)| WorldEdge {
        from: id.to_string(),
        to: (*target).to_string(),
        direction: (*direction).to_string(),
    }));
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CexWorldHandoffManifest {
    pub cex_head: String,
    pub trillionnium_head: String,
    pub source_paths: Vec<String>,
    pub target_crates: Vec<String>,
    pub migration_rule: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldIndexes {
    pub sorted_node_ids: Vec<String>,
    pub sorted_node_ids_by_id: Vec<String>,
    pub node_ids_by_location: HashMap<String, Vec<String>>,
    pub sorted_position_actor_ids: Vec<String>,
    pub recent_task_indices: Vec<usize>,
    pub recent_receipt_indices: Vec<usize>,
}

pub fn recent_tail_indices(len: usize, limit: usize) -> Vec<usize> {
    (0..len).rev().take(limit).collect()
}

pub fn sorted_indices_by<T, F>(items: &[T], mut compare: F) -> Vec<usize>
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let mut indices: Vec<usize> = (0..items.len()).collect();
    indices.sort_by(|left, right| compare(&items[*left], &items[*right]));
    indices
}

pub fn indexed_sorted<'a, T>(items: &'a [T], indices: &'a [usize]) -> Vec<&'a T> {
    indices
        .iter()
        .filter_map(move |index| items.get(*index))
        .collect()
}

pub fn indexed_recent<'a, T>(
    items: &'a [T],
    indices: &'a [usize],
    limit: usize,
) -> impl Iterator<Item = &'a T> + 'a {
    indices
        .iter()
        .take(limit)
        .filter_map(move |index| items.get(*index))
}

pub fn build_world_indexes(world: &WorldState) -> WorldIndexes {
    let mut indexes = WorldIndexes::default();

    indexes.sorted_node_ids = world.nodes.iter().map(|node| node.id.clone()).collect();
    indexes.sorted_node_ids_by_id = indexes.sorted_node_ids.clone();
    indexes.sorted_node_ids_by_id.sort();
    indexes
        .sorted_node_ids
        .sort_by(|left, right| match (world.node(left), world.node(right)) {
            (Some(left_node), Some(right_node)) => left_node
                .lat_e7
                .cmp(&right_node.lat_e7)
                .then(left_node.lng_e7.cmp(&right_node.lng_e7))
                .then(left_node.id.cmp(&right_node.id)),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => left.cmp(right),
        });

    for node in indexes
        .sorted_node_ids_by_id
        .iter()
        .filter_map(|node_id| world.node(node_id))
    {
        indexes
            .node_ids_by_location
            .entry(node.location_id.clone())
            .or_default()
            .push(node.id.clone());
    }
    for node_ids in indexes.node_ids_by_location.values_mut() {
        node_ids.sort();
    }

    indexes.sorted_position_actor_ids = world
        .positions
        .iter()
        .map(|position| position.actor_id.clone())
        .collect();
    indexes.sorted_position_actor_ids.sort();
    indexes.recent_task_indices = recent_tail_indices(world.tasks.len(), 10);
    indexes.recent_receipt_indices = recent_tail_indices(world.receipts.len(), 10);

    indexes
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumSkillDefinition {
    pub contract_version: String,
    pub skill_id: String,
    pub family: String,
    pub name: String,
    pub level: u16,
    pub xp: u32,
    pub unlock_condition: String,
    pub combat_effect: String,
    pub world_effect: String,
    pub training_anchor_role: String,
    pub source_of_truth: String,
    pub content_policy: String,
}

impl TrillionniumSkillDefinition {
    #[allow(clippy::too_many_arguments)]
    fn fixture(
        skill_id: &str,
        family: &str,
        name: &str,
        xp: u32,
        unlock_condition: &str,
        combat_effect: &str,
        world_effect: &str,
        training_anchor_role: &str,
    ) -> Self {
        Self {
            contract_version: TRILLIONNIUM_SKILL_CONTRACT_VERSION.to_string(),
            skill_id: skill_id.to_string(),
            family: family.to_string(),
            name: name.to_string(),
            level: 1,
            xp,
            unlock_condition: unlock_condition.to_string(),
            combat_effect: combat_effect.to_string(),
            world_effect: world_effect.to_string(),
            training_anchor_role: training_anchor_role.to_string(),
            source_of_truth: "rust_trillionnium_skill_definition".to_string(),
            content_policy: "trillionnium_native_no_copied_hero_tan_text_assets_or_tables"
                .to_string(),
        }
    }
}

pub fn trillionnium_fixture_skill_definitions() -> Vec<TrillionniumSkillDefinition> {
    vec![
        TrillionniumSkillDefinition::fixture(
            "basic_inner_power",
            "inner_power",
            "Cloud Ledger Breathing / 云账吐纳",
            120,
            "default_character_seed",
            "raise_inner_energy_and_guard",
            "improve_settlement_recovery_focus",
            "mentor_home",
        ),
        TrillionniumSkillDefinition::fixture(
            "basic_unarmed",
            "unarmed",
            "Street Compass Palm / 街指南掌",
            60,
            "inspect_civic_square",
            "enable_melee_attack",
            "resolve_minor_street_encounters",
            "civic_square",
        ),
        TrillionniumSkillDefinition::fixture(
            "basic_blade",
            "blade",
            "Iron Workshop Blade / 铁坊刀法",
            40,
            "visit_workshop_anchor",
            "increase_attack_against_armored_targets",
            "improve_artifact_repair_tasks",
            "workshop",
        ),
        TrillionniumSkillDefinition::fixture(
            "basic_sword",
            "sword",
            "Market Wind Sword / 集风剑式",
            40,
            "market_route_training",
            "increase_precision_attack",
            "improve_negotiation_opening_move",
            "market",
        ),
        TrillionniumSkillDefinition::fixture(
            "basic_lightness",
            "lightness",
            "Night Watch Steps / 夜巡步",
            100,
            "default_character_seed",
            "increase_move_range_and_evade",
            "reduce_route_travel_friction",
            "delivery_route",
        ),
        TrillionniumSkillDefinition::fixture(
            "reading_and_contracts",
            "civil",
            "Contract Reading / 契约读法",
            140,
            "default_character_seed",
            "reveal_risk_marker_before_attack",
            "improve_contract_capture_and_review_hold_recovery",
            "ledger_hall",
        ),
        TrillionniumSkillDefinition::fixture(
            "merchant_routecraft",
            "commerce",
            "Merchant Routecraft / 商路术",
            80,
            "accept_market_bounty",
            "convert_market_tile_to_supply_bonus",
            "improve_bounty_pricing_and_route_choice",
            "market",
        ),
        TrillionniumSkillDefinition::fixture(
            "artifact_crafting",
            "craft",
            "Artifact Crafting / 器作法",
            80,
            "visit_workshop_anchor",
            "improve_item_use_quality",
            "increase_asset_upgrade_quality_bonus",
            "workshop",
        ),
        TrillionniumSkillDefinition::fixture(
            "streetwise_investigation",
            "investigation",
            "Streetwise Investigation / 街察术",
            90,
            "inspect_quest_board_or_dispute_desk",
            "reveal_hidden_enemy_intent",
            "improve_evidence_gathering_and_dispute_routes",
            "arbitration_desk",
        ),
        TrillionniumSkillDefinition::fixture(
            "staff_and_polearm",
            "staff",
            "Route Guard Staff / 护路棍法",
            70,
            "escort_route_training",
            "extend_melee_zone_control",
            "improve_escort_and_patrol_task_safety",
            "delivery_route",
        ),
        TrillionniumSkillDefinition::fixture(
            "evidence_packaging",
            "evidence",
            "Evidence Packaging / 证据封装",
            110,
            "submit_first_task_report",
            "preserve_objective_proof_under_pressure",
            "improve_review_hold_release_quality",
            "quest_board",
        ),
        TrillionniumSkillDefinition::fixture(
            "healing_tonic_craft",
            "medicine",
            "Tonic Craft / 补剂调配",
            85,
            "meet_field_apothecary",
            "recover_minor_wounds_after_encounter",
            "reduce_party_downtime_after_failed_tasks",
            "mentor_home",
        ),
        TrillionniumSkillDefinition::fixture(
            "route_scouting",
            "scouting",
            "Route Scouting / 路线侦察",
            95,
            "complete_first_delivery_route",
            "preview_enemy_position_before_entry",
            "improve_objective_travel_next_step_quality",
            "delivery_route",
        ),
        TrillionniumSkillDefinition::fixture(
            "dispute_mediation",
            "mediation",
            "Dispute Mediation / 纠纷调停",
            130,
            "inspect_arbitration_desk",
            "convert_some_hostile_events_to_negotiation",
            "increase_npc_trust_and_reputation_recovery",
            "arbitration_desk",
        ),
        TrillionniumSkillDefinition::fixture(
            "raid_coordination",
            "raid_command",
            "Raid Coordination / 会战号令",
            160,
            "enter_raid_hall",
            "improve_party_focus_on_objective_tiles",
            "unlock_group_route_and_raid_hall_tasks",
            "raid_hall",
        ),
        TrillionniumSkillDefinition::fixture(
            "artifact_appraisal",
            "appraisal",
            "Artifact Appraisal / 器物鉴定",
            105,
            "inspect_workshop_or_market_item",
            "identify_item_quality_before_use",
            "improve_market_listing_quality_and_repair_routes",
            "workshop",
        ),
        TrillionniumSkillDefinition::fixture(
            "terrain_reading",
            "terrain",
            "Terrain Reading / 地势辨读",
            100,
            "move_across_three_world_nodes",
            "reduce_forest_and_river_movement_penalty",
            "improve_map_transition_and_blocked_terrain_guidance",
            "civic_square",
        ),
        TrillionniumSkillDefinition::fixture(
            "auction_sense",
            "auction",
            "Auction Sense / 拍卖眼力",
            115,
            "visit_market_listing_board",
            "turn_supply_tiles_into_temporary_focus",
            "improve_bounty_pricing_and_seller_selection",
            "market",
        ),
        TrillionniumSkillDefinition::fixture(
            "camp_cooking",
            "survival",
            "Camp Cooking / 行灶术",
            75,
            "rest_after_long_route",
            "restore_focus_before_next_encounter",
            "reduce_stamina_pressure_on_long_routes",
            "mentor_home",
        ),
        TrillionniumSkillDefinition::fixture(
            "shadow_messaging",
            "messaging",
            "Shadow Messaging / 暗信步",
            125,
            "complete_night_watch_patrol",
            "delay_enemy_reinforcement_signal",
            "unlock_discreet_courier_and_witness_routes",
            "quest_board",
        ),
    ]
}

pub fn trillionnium_skill_definition_by_id(skill_id: &str) -> Option<TrillionniumSkillDefinition> {
    trillionnium_fixture_skill_definitions()
        .into_iter()
        .find(|skill| skill.skill_id == skill_id)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumTrainingCommand {
    pub contract_version: String,
    pub command: String,
    pub skill_id: String,
    pub mentor_npc_id: String,
    pub required_semantic_role: String,
    pub cost_xp: i64,
    pub cooldown_seconds: i64,
    pub validation_owner: String,
    pub state_mutation_owner: String,
    pub web_role: String,
}

impl TrillionniumTrainingCommand {
    fn fixture(
        skill_id: &str,
        mentor_npc_id: &str,
        required_semantic_role: &str,
        cost_xp: i64,
        cooldown_seconds: i64,
    ) -> Self {
        Self {
            contract_version: TRILLIONNIUM_TRAINING_CONTRACT_VERSION.to_string(),
            command: "train_skill".to_string(),
            skill_id: skill_id.to_string(),
            mentor_npc_id: mentor_npc_id.to_string(),
            required_semantic_role: required_semantic_role.to_string(),
            cost_xp,
            cooldown_seconds,
            validation_owner: "rust_mentor_training_validator".to_string(),
            state_mutation_owner: "rust_trillionnium_game_state".to_string(),
            web_role: "intent_only_visualization_input".to_string(),
        }
    }
}

pub fn trillionnium_training_command_fixtures() -> Vec<TrillionniumTrainingCommand> {
    vec![
        TrillionniumTrainingCommand::fixture(
            "basic_inner_power",
            "npc-cloud-ledger-mentor",
            "mentor_home",
            12,
            600,
        ),
        TrillionniumTrainingCommand::fixture(
            "basic_unarmed",
            "npc-street-compass-sifu",
            "civic_square",
            8,
            420,
        ),
        TrillionniumTrainingCommand::fixture(
            "basic_blade",
            "npc-iron-workshop-smith",
            "workshop",
            10,
            480,
        ),
        TrillionniumTrainingCommand::fixture(
            "basic_sword",
            "npc-market-wind-adviser",
            "market",
            10,
            480,
        ),
        TrillionniumTrainingCommand::fixture(
            "merchant_routecraft",
            "npc-market-wind-adviser",
            "market",
            16,
            900,
        ),
        TrillionniumTrainingCommand::fixture(
            "artifact_crafting",
            "npc-iron-workshop-smith",
            "workshop",
            16,
            900,
        ),
        TrillionniumTrainingCommand::fixture(
            "streetwise_investigation",
            "npc-night-watch-arbiter",
            "arbitration_desk",
            14,
            720,
        ),
        TrillionniumTrainingCommand::fixture(
            "staff_and_polearm",
            "npc-escort-captain-han",
            "delivery_route",
            12,
            600,
        ),
        TrillionniumTrainingCommand::fixture(
            "evidence_packaging",
            "npc-bounty-board-clerk",
            "quest_board",
            15,
            840,
        ),
        TrillionniumTrainingCommand::fixture(
            "healing_tonic_craft",
            "npc-field-apothecary",
            "mentor_home",
            13,
            720,
        ),
        TrillionniumTrainingCommand::fixture(
            "route_scouting",
            "npc-jade-route-scout",
            "delivery_route",
            15,
            780,
        ),
        TrillionniumTrainingCommand::fixture(
            "dispute_mediation",
            "npc-dispute-witness-lu",
            "arbitration_desk",
            18,
            960,
        ),
        TrillionniumTrainingCommand::fixture(
            "raid_coordination",
            "npc-raid-drum-sergeant",
            "raid_hall",
            20,
            1200,
        ),
        TrillionniumTrainingCommand::fixture(
            "artifact_appraisal",
            "npc-artifact-appraiser",
            "workshop",
            14,
            780,
        ),
        TrillionniumTrainingCommand::fixture(
            "terrain_reading",
            "npc-map-tile-surveyor",
            "civic_square",
            12,
            600,
        ),
        TrillionniumTrainingCommand::fixture(
            "auction_sense",
            "npc-warehouse-broker-xu",
            "market",
            16,
            900,
        ),
        TrillionniumTrainingCommand::fixture(
            "camp_cooking",
            "npc-camp-cook-lin",
            "mentor_home",
            10,
            600,
        ),
        TrillionniumTrainingCommand::fixture(
            "shadow_messaging",
            "npc-shadow-message-runner",
            "quest_board",
            17,
            960,
        ),
    ]
}

pub fn trillionnium_training_command_for_skill(
    skill_id: &str,
) -> Option<TrillionniumTrainingCommand> {
    trillionnium_training_command_fixtures()
        .into_iter()
        .find(|command| command.skill_id == skill_id)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumTaskArchetype {
    pub contract_version: String,
    pub task_archetype_id: String,
    pub display_name: String,
    pub source_semantic_roles: Vec<String>,
    pub command: String,
    pub completion_owner: String,
    pub reward_gate: String,
    pub log_style_key: String,
    pub content_policy: String,
}

impl TrillionniumTaskArchetype {
    fn fixture(
        task_archetype_id: &str,
        display_name: &str,
        source_semantic_roles: &[&str],
        command: &str,
        completion_owner: &str,
        reward_gate: &str,
        log_style_key: &str,
    ) -> Self {
        Self {
            contract_version: TRILLIONNIUM_TASK_ARCHETYPE_CONTRACT_VERSION.to_string(),
            task_archetype_id: task_archetype_id.to_string(),
            display_name: display_name.to_string(),
            source_semantic_roles: source_semantic_roles
                .iter()
                .map(|role| (*role).to_string())
                .collect(),
            command: command.to_string(),
            completion_owner: completion_owner.to_string(),
            reward_gate: reward_gate.to_string(),
            log_style_key: log_style_key.to_string(),
            content_policy: "trillionnium_native_no_copied_hero_tan_text_assets_or_tables"
                .to_string(),
        }
    }
}

pub fn trillionnium_task_archetype_fixtures() -> Vec<TrillionniumTaskArchetype> {
    vec![
        TrillionniumTaskArchetype::fixture(
            "courier_letter",
            "Courier Letter / 飞笺传书",
            &["delivery_route", "civic_square"],
            "offer_task",
            "rust_command_handler_ledger_progression",
            "ledger_settlement_review_hold_anti_cheese",
            "street_courier",
        ),
        TrillionniumTaskArchetype::fixture(
            "find_item",
            "Find Item / 寻物探查",
            &["workshop", "market"],
            "offer_task",
            "rust_world_action_handler",
            "evidence_required_before_reward",
            "street_investigation",
        ),
        TrillionniumTaskArchetype::fixture(
            "escort_route",
            "Escort Route / 护送路线",
            &["delivery_route", "arbitration_desk"],
            "offer_task",
            "rust_tactics_turn_handler",
            "proof_gated_route_completion",
            "escort_clash",
        ),
        TrillionniumTaskArchetype::fixture(
            "market_settlement",
            "Market Settlement / 市集结算",
            &["market", "ledger_hall"],
            "offer_task",
            "rust_market_command_handler",
            "ledger_release_required",
            "market_parley",
        ),
        TrillionniumTaskArchetype::fixture(
            "defeat_bandit",
            "Defeat Bandit / 平定流寇",
            &["arena", "market"],
            "attack",
            "rust_tactics_combat_handler",
            "combat_resolution_then_ledger_review_gate",
            "escort_clash",
        ),
        TrillionniumTaskArchetype::fixture(
            "sect_training_trial",
            "Sect Training Trial / 门内试炼",
            &[
                "mentor_home",
                "civic_square",
                "workshop",
                "market",
                "arbitration_desk",
            ],
            "train_skill",
            "rust_mentor_training_validator",
            "mentor_place_cost_cooldown_required",
            "mentor_trial",
        ),
        TrillionniumTaskArchetype::fixture(
            "street_patrol",
            "Street Patrol / 街巡护路",
            &["civic_square", "delivery_route"],
            "offer_task",
            "rust_world_action_handler",
            "patrol_report_review_hold_anti_cheese",
            "street_patrol",
        ),
        TrillionniumTaskArchetype::fixture(
            "debt_recovery",
            "Debt Recovery / 清账追索",
            &["ledger_hall", "market"],
            "offer_task",
            "rust_command_handler_ledger_progression",
            "ledger_settlement_and_dispute_evidence_required",
            "debt_recovery",
        ),
        TrillionniumTaskArchetype::fixture(
            "map_survey",
            "Map Survey / 地图踏勘",
            &["civic_square", "quest_board", "delivery_route"],
            "offer_task",
            "rust_world_graph_objective_travel",
            "route_evidence_required_before_reward",
            "map_survey",
        ),
        TrillionniumTaskArchetype::fixture(
            "healing_supply",
            "Healing Supply / 行药补给",
            &["mentor_home", "workshop"],
            "offer_task",
            "rust_world_action_handler",
            "supply_quality_review_required",
            "healing_supply",
        ),
        TrillionniumTaskArchetype::fixture(
            "arbitrate_dispute",
            "Arbitrate Dispute / 调停纠纷",
            &["arbitration_desk", "market"],
            "offer_task",
            "rust_trillionnium_task_completion_handler",
            "relationship_evidence_and_review_hold_gate",
            "arbitrate_dispute",
        ),
        TrillionniumTaskArchetype::fixture(
            "raid_signal",
            "Raid Signal / 会战号令",
            &["raid_hall", "arena"],
            "offer_task",
            "rust_tactics_turn_handler",
            "party_raid_resolution_then_ledger_gate",
            "raid_signal",
        ),
        TrillionniumTaskArchetype::fixture(
            "witness_archive_case",
            "Witness Archive Case / 见证档案案卷",
            &["archive", "mediation_steps", "arbitration_desk"],
            "offer_task",
            "rust_trillionnium_task_completion_handler",
            "relationship_evidence_and_review_hold_gate",
            "arbitrate_dispute",
        ),
        TrillionniumTaskArchetype::fixture(
            "night_watch_message",
            "Night Watch Message / 夜巡暗信",
            &["patrol_yard", "courier_yard", "quest_board"],
            "offer_task",
            "rust_world_graph_objective_travel",
            "route_evidence_required_before_reward",
            "street_patrol",
        ),
        TrillionniumTaskArchetype::fixture(
            "cistern_ration_run",
            "Cistern Ration Run / 水仓行粮",
            &["water_supply", "ration_kitchen", "caravan_camp"],
            "offer_task",
            "rust_trillionnium_food_water_age_survival_state",
            "survival_supply_quality_review_required",
            "healing_supply",
        ),
        TrillionniumTaskArchetype::fixture(
            "field_infirmary_round",
            "Field Infirmary Round / 野外医棚巡诊",
            &["infirmary", "caravan_camp", "mentor_home"],
            "offer_task",
            "rust_trillionnium_resource_pressure_runtime_state",
            "recovery_evidence_required_before_reward",
            "healing_supply",
        ),
        TrillionniumTaskArchetype::fixture(
            "survey_tower_chart",
            "Survey Tower Chart / 测绘塔图记",
            &["survey_tower", "courier_yard", "delivery_route"],
            "offer_task",
            "rust_world_graph_objective_travel",
            "route_evidence_required_before_reward",
            "map_survey",
        ),
        TrillionniumTaskArchetype::fixture(
            "guild_vault_audit",
            "Guild Vault Audit / 公会库房稽核",
            &["guild_vault", "raid_hall", "ledger_hall"],
            "offer_task",
            "rust_review_hold_gate",
            "guild_evidence_review_hold_required",
            "raid_signal",
        ),
        TrillionniumTaskArchetype::fixture(
            "auction_appraisal",
            "Auction Appraisal / 拍卖廊鉴定",
            &["auction_arcade", "workshop", "market"],
            "offer_task",
            "rust_inventory_crafting_gate",
            "appraisal_evidence_and_settlement_required",
            "market_parley",
        ),
        TrillionniumTaskArchetype::fixture(
            "mentor_cloister_oath",
            "Mentor Cloister Oath / 导师回廊誓约",
            &["mentor_cloister", "mediation_steps", "civic_square"],
            "train_skill",
            "rust_mentor_training_validator",
            "mentor_place_cost_cooldown_required",
            "mentor_trial",
        ),
    ]
}

pub fn trillionnium_task_archetype_by_id(
    task_archetype_id: &str,
) -> Option<TrillionniumTaskArchetype> {
    trillionnium_task_archetype_fixtures()
        .into_iter()
        .find(|task| task.task_archetype_id == task_archetype_id)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumSectFixture {
    pub contract_version: String,
    pub sect_id: String,
    pub display_name: String,
    pub specialization: String,
    pub anchor_semantic_role: String,
    pub mentor_npc_ids: Vec<String>,
    pub entry_requirement: String,
    pub benefits: Vec<String>,
    pub title_ladder: Vec<String>,
    pub source_of_truth: String,
    pub content_policy: String,
}

impl TrillionniumSectFixture {
    #[allow(clippy::too_many_arguments)]
    fn fixture(
        sect_id: &str,
        display_name: &str,
        specialization: &str,
        anchor_semantic_role: &str,
        mentor_npc_ids: &[&str],
        entry_requirement: &str,
        benefits: &[&str],
        title_ladder: &[&str],
    ) -> Self {
        Self {
            contract_version: TRILLIONNIUM_SECT_CONTRACT_VERSION.to_string(),
            sect_id: sect_id.to_string(),
            display_name: display_name.to_string(),
            specialization: specialization.to_string(),
            anchor_semantic_role: anchor_semantic_role.to_string(),
            mentor_npc_ids: mentor_npc_ids
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            entry_requirement: entry_requirement.to_string(),
            benefits: benefits.iter().map(|value| (*value).to_string()).collect(),
            title_ladder: title_ladder
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            source_of_truth: "rust_trillionnium_sect_model".to_string(),
            content_policy: "trillionnium_native_no_copied_hero_tan_text_assets_or_tables"
                .to_string(),
        }
    }
}

pub fn trillionnium_sect_fixtures() -> Vec<TrillionniumSectFixture> {
    vec![
        TrillionniumSectFixture::fixture(
            "cloud-ledger-hall",
            "Cloud Ledger Hall / 云账堂",
            "inner_power_contracts_and_settlement",
            "ledger_hall",
            &["npc-cloud-ledger-mentor"],
            "reading_and_contracts_known",
            &["settlement_recovery_bonus", "contract_risk_preview"],
            &["outer_clerk", "ledger_runner", "cloud_ledger_keeper"],
        ),
        TrillionniumSectFixture::fixture(
            "street-compass-society",
            "Street Compass Society / 街指南社",
            "movement_investigation_and_routecraft",
            "civic_square",
            &["npc-street-compass-sifu"],
            "basic_lightness_known",
            &["movement_range_hint", "street_event_preview"],
            &["street_walker", "route_scout", "compass_pathfinder"],
        ),
        TrillionniumSectFixture::fixture(
            "iron-workshop-gate",
            "Iron Workshop Gate / 铁坊门",
            "craft_blade_and_artifact_repair",
            "workshop",
            &["npc-iron-workshop-smith"],
            "artifact_crafting_or_basic_blade_training",
            &["craft_quality_bonus", "item_repair_discount"],
            &["apprentice_smith", "artifact_mender", "iron_gate_master"],
        ),
        TrillionniumSectFixture::fixture(
            "market-wind-pavilion",
            "Market Wind Pavilion / 集风阁",
            "commerce_negotiation_and_bounty_quality",
            "market",
            &["npc-market-wind-adviser"],
            "merchant_routecraft_training_available",
            &["listing_quality_hint", "negotiation_bonus"],
            &["stall_runner", "wind_broker", "market_pavilion_master"],
        ),
        TrillionniumSectFixture::fixture(
            "night-watch-alliance",
            "Night Watch Alliance / 夜巡盟",
            "risk_control_disputes_and_escort_tasks",
            "arbitration_desk",
            &["npc-night-watch-arbiter"],
            "streetwise_investigation_training_available",
            &["dispute_evidence_bonus", "escort_risk_reduction"],
            &["watch_runner", "risk_warden", "night_watch_captain"],
        ),
        TrillionniumSectFixture::fixture(
            "jade-route-agency",
            "Jade Route Agency / 玉路局",
            "route_scouting_patrol_and_caravan_escort",
            "delivery_route",
            &["npc-jade-route-scout", "npc-escort-captain-han"],
            "complete_first_delivery_or_patrol_task",
            &["route_scouting_bonus", "escort_party_readiness"],
            &["route_runner", "jade_scout", "caravan_path_master"],
        ),
        TrillionniumSectFixture::fixture(
            "dispute-mirror-court",
            "Dispute Mirror Court / 明镜庭",
            "mediation_witness_handling_and_reputation_repair",
            "arbitration_desk",
            &["npc-dispute-witness-lu", "npc-sect-registrar-qin"],
            "relationship_trust_or_dispute_mediation_training",
            &["npc_trust_recovery", "review_hold_release_hint"],
            &["case_listener", "mirror_clerk", "court_mediator"],
        ),
        TrillionniumSectFixture::fixture(
            "raid-signal-lodge",
            "Raid Signal Lodge / 号令楼",
            "combat_entry_party_coordination_and_raid_tasks",
            "raid_hall",
            &["npc-raid-drum-sergeant", "npc-arena-referee-du"],
            "win_first_lightweight_encounter",
            &["raid_coordination_bonus", "combat_return_state_clarity"],
            &["signal_runner", "drum_captain", "raid_lodge_commander"],
        ),
        TrillionniumSectFixture::fixture(
            "field-remedy-garden",
            "Field Remedy Garden / 行药园",
            "medicine_recovery_and_failed_task_downtime_control",
            "mentor_home",
            &["npc-field-apothecary"],
            "meet_field_apothecary_or_failed_encounter_recovery",
            &["minor_wound_recovery", "party_downtime_reduction"],
            &["herb_runner", "field_tonic_maker", "garden_healer"],
        ),
    ]
}

pub fn trillionnium_sect_by_id(sect_id: &str) -> Option<TrillionniumSectFixture> {
    trillionnium_sect_fixtures()
        .into_iter()
        .find(|sect| sect.sect_id == sect_id)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumNpcFixture {
    pub contract_version: String,
    pub npc_id: String,
    pub display_name: String,
    pub role: String,
    pub sect_id: String,
    pub anchor_semantic_role: String,
    pub relationship_seed: i64,
    pub schedule: String,
    pub task_capabilities: Vec<String>,
    pub training_skill_ids: Vec<String>,
    pub task_archetype_ids: Vec<String>,
    pub command_ids: Vec<String>,
    pub source_of_truth: String,
    pub content_policy: String,
}

impl TrillionniumNpcFixture {
    #[allow(clippy::too_many_arguments)]
    fn fixture(
        npc_id: &str,
        display_name: &str,
        role: &str,
        sect_id: &str,
        anchor_semantic_role: &str,
        relationship_seed: i64,
        schedule: &str,
        task_capabilities: &[&str],
    ) -> Self {
        let training_skill_ids = trillionnium_npc_training_skill_ids(npc_id);
        let mut task_archetype_ids = task_capabilities
            .iter()
            .flat_map(|capability| trillionnium_task_archetype_ids_for_capability(capability))
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        task_archetype_ids.sort();
        task_archetype_ids.dedup();
        Self {
            contract_version: TRILLIONNIUM_NPC_CONTRACT_VERSION.to_string(),
            npc_id: npc_id.to_string(),
            display_name: display_name.to_string(),
            role: role.to_string(),
            sect_id: sect_id.to_string(),
            anchor_semantic_role: anchor_semantic_role.to_string(),
            relationship_seed,
            schedule: schedule.to_string(),
            task_capabilities: task_capabilities
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            training_skill_ids,
            task_archetype_ids,
            command_ids: vec![
                "talk_npc".to_string(),
                "train_skill".to_string(),
                "offer_task".to_string(),
            ],
            source_of_truth: "rust_trillionnium_npc_model".to_string(),
            content_policy: "trillionnium_native_no_copied_hero_tan_text_assets_or_tables"
                .to_string(),
        }
    }
}

pub fn trillionnium_task_archetype_ids_for_capability(capability: &str) -> Vec<&'static str> {
    match capability {
        "offer_patrol_task" => vec!["courier_letter", "escort_route"],
        "train_unarmed"
        | "train_lightness"
        | "train_blade"
        | "train_sword"
        | "train_routecraft"
        | "train_artifact_crafting"
        | "train_investigation"
        | "train_inner_power"
        | "train_staff"
        | "train_evidence_packaging"
        | "train_medicine"
        | "train_route_scouting"
        | "train_mediation"
        | "train_raid_coordination"
        | "train_appraisal"
        | "train_terrain_reading"
        | "train_auction_sense"
        | "train_camp_cooking"
        | "train_shadow_messaging" => {
            vec!["sect_training_trial"]
        }
        "review_contract_risk" | "review_evidence" => {
            vec!["market_settlement", "find_item", "witness_archive_case"]
        }
        "repair_item" => vec!["find_item", "auction_appraisal"],
        "price_bounty" => vec!["market_settlement", "auction_appraisal"],
        "offer_escort_task" => vec!["escort_route"],
        "offer_patrol_loop" => vec!["street_patrol", "map_survey", "night_watch_message"],
        "recover_debt" => vec!["debt_recovery", "market_settlement", "witness_archive_case"],
        "supply_medicine" => vec![
            "healing_supply",
            "find_item",
            "cistern_ration_run",
            "field_infirmary_round",
        ],
        "survey_map" => vec!["map_survey", "courier_letter", "survey_tower_chart"],
        "mediate_dispute" => vec![
            "arbitrate_dispute",
            "market_settlement",
            "witness_archive_case",
            "mentor_cloister_oath",
        ],
        "coordinate_raid" => vec![
            "raid_signal",
            "escort_route",
            "defeat_bandit",
            "guild_vault_audit",
        ],
        "run_arena_duel" => vec!["defeat_bandit", "raid_signal"],
        "register_sect_case" => vec![
            "sect_training_trial",
            "arbitrate_dispute",
            "mentor_cloister_oath",
        ],
        "appraise_artifact" => vec!["find_item", "healing_supply", "auction_appraisal"],
        _ => Vec::new(),
    }
}

pub fn trillionnium_npc_training_skill_ids(npc_id: &str) -> Vec<String> {
    trillionnium_training_command_fixtures()
        .into_iter()
        .filter(|command| command.mentor_npc_id == npc_id)
        .map(|command| command.skill_id)
        .collect::<Vec<_>>()
}

pub fn trillionnium_npc_fixtures() -> Vec<TrillionniumNpcFixture> {
    vec![
        TrillionniumNpcFixture::fixture(
            "npc-cloud-ledger-mentor",
            "Ledger Mentor Wen / 温账师",
            "mentor_contracts_inner_power",
            "cloud-ledger-hall",
            "ledger_hall",
            12,
            "morning_ledger_evening_training",
            &["train_inner_power", "review_contract_risk"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-street-compass-sifu",
            "Compass Sifu Luo / 罗街师",
            "mentor_movement_unarmed",
            "street-compass-society",
            "civic_square",
            9,
            "daytime_square_patrol",
            &["train_unarmed", "train_lightness", "offer_patrol_task"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-iron-workshop-smith",
            "Iron Smith Qiao / 乔铁匠",
            "mentor_blade_crafting",
            "iron-workshop-gate",
            "workshop",
            7,
            "workshop_day_shift",
            &["train_blade", "train_artifact_crafting", "repair_item"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-market-wind-adviser",
            "Market Adviser Lin / 林集风",
            "mentor_commerce_sword",
            "market-wind-pavilion",
            "market",
            10,
            "market_open_hours",
            &["train_sword", "train_routecraft", "price_bounty"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-night-watch-arbiter",
            "Night Arbiter Shen / 沈夜判",
            "mentor_investigation_disputes",
            "night-watch-alliance",
            "arbitration_desk",
            8,
            "evening_dispute_watch",
            &[
                "train_investigation",
                "offer_escort_task",
                "review_evidence",
            ],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-contract-runner-mei",
            "Contract Runner Mei / 梅契跑",
            "courier_contract_runner",
            "cloud-ledger-hall",
            "delivery_route",
            6,
            "route_morning_contract_evening_return",
            &["offer_patrol_loop", "recover_debt"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-jade-route-scout",
            "Jade Route Scout / 玉路探",
            "mentor_route_scouting",
            "jade-route-agency",
            "delivery_route",
            11,
            "dawn_route_scout_noon_report",
            &["train_route_scouting", "survey_map", "offer_patrol_loop"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-bounty-board-clerk",
            "Bounty Clerk Bao / 鲍榜吏",
            "mentor_evidence_and_bounty_board",
            "street-compass-society",
            "quest_board",
            5,
            "quest_board_open_hours",
            &[
                "train_evidence_packaging",
                "review_evidence",
                "offer_patrol_loop",
            ],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-escort-captain-han",
            "Escort Captain Han / 韩镖头",
            "mentor_staff_and_escort",
            "jade-route-agency",
            "delivery_route",
            9,
            "caravan_departure_and_evening_drill",
            &["train_staff", "offer_escort_task", "offer_patrol_loop"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-arena-referee-du",
            "Arena Referee Du / 杜校场",
            "arena_duel_referee",
            "raid-signal-lodge",
            "arena",
            4,
            "arena_challenge_windows",
            &["run_arena_duel", "coordinate_raid"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-field-apothecary",
            "Field Apothecary Yi / 易行药",
            "mentor_field_medicine",
            "field-remedy-garden",
            "mentor_home",
            8,
            "midday_tonic_prep_night_recovery",
            &["train_medicine", "supply_medicine", "review_evidence"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-dispute-witness-lu",
            "Witness Lu / 卢见证",
            "mentor_mediation_and_witness",
            "dispute-mirror-court",
            "arbitration_desk",
            7,
            "case_hearing_and_witness_route",
            &["train_mediation", "mediate_dispute", "review_evidence"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-warehouse-broker-xu",
            "Warehouse Broker Xu / 徐仓牙",
            "mentor_auction_and_inventory",
            "market-wind-pavilion",
            "market",
            6,
            "market_auction_and_warehouse_close",
            &["train_auction_sense", "price_bounty", "recover_debt"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-raid-drum-sergeant",
            "Raid Drum Sergeant / 鼓令军曹",
            "mentor_raid_coordination",
            "raid-signal-lodge",
            "raid_hall",
            10,
            "raid_drill_and_signal_watch",
            &[
                "train_raid_coordination",
                "coordinate_raid",
                "offer_escort_task",
            ],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-sect-registrar-qin",
            "Sect Registrar Qin / 秦录事",
            "sect_registry_and_title_ladder",
            "dispute-mirror-court",
            "sect_hall",
            3,
            "registry_open_midday",
            &["register_sect_case", "mediate_dispute"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-map-tile-surveyor",
            "Map Surveyor Gao / 高图工",
            "mentor_terrain_and_map_survey",
            "street-compass-society",
            "civic_square",
            8,
            "square_survey_and_evening_grid_notes",
            &["train_terrain_reading", "survey_map", "offer_patrol_loop"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-artifact-appraiser",
            "Artifact Appraiser Yan / 颜鉴器",
            "mentor_artifact_appraisal",
            "iron-workshop-gate",
            "workshop",
            7,
            "workshop_appraisal_and_market_walk",
            &["train_appraisal", "appraise_artifact", "repair_item"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-camp-cook-lin",
            "Camp Cook Lin / 林行灶",
            "mentor_survival_cooking",
            "field-remedy-garden",
            "mentor_home",
            5,
            "dawn_meal_prep_evening_recovery",
            &["train_camp_cooking", "supply_medicine", "offer_patrol_loop"],
        ),
        TrillionniumNpcFixture::fixture(
            "npc-shadow-message-runner",
            "Shadow Message Runner / 暗信行者",
            "mentor_discreet_courier",
            "night-watch-alliance",
            "quest_board",
            9,
            "night_message_route_and_board_drop",
            &[
                "train_shadow_messaging",
                "offer_patrol_loop",
                "review_evidence",
            ],
        ),
    ]
}

pub fn trillionnium_npc_by_id(npc_id: &str) -> Option<TrillionniumNpcFixture> {
    trillionnium_npc_fixtures()
        .into_iter()
        .find(|npc| npc.npc_id == npc_id)
}

pub fn trillionnium_npc_relationship_delta(relation_kind: &str, strength: i64) -> i64 {
    match relation_kind {
        "trillionnium_npc_talk_npc" | "tactics_talk_npc" => 3,
        "trillionnium_npc_offer_task" | "tactics_offer_task" => 5,
        "trillionnium_npc_train_skill" | "tactics_train_skill" => 4,
        "trillionnium_npc_complete_task" | "tactics_complete_task" => 2,
        _ => strength.clamp(-3, 3),
    }
}

pub fn trillionnium_objective_task_for_semantic_role(role: &str) -> Option<&'static str> {
    match role {
        "civic_square" | "mentor_home" | "sect_hall" => Some("sect_training_trial"),
        "ledger_hall" | "market" => Some("market_settlement"),
        "quest_board" | "workshop" | "arbitration_desk" => Some("find_item"),
        "delivery_route" => Some("courier_letter"),
        "arena" => Some("defeat_bandit"),
        "raid_hall" => Some("escort_route"),
        "archive" | "mediation_steps" => Some("witness_archive_case"),
        "patrol_yard" | "courier_yard" => Some("night_watch_message"),
        "water_supply" | "ration_kitchen" => Some("cistern_ration_run"),
        "infirmary" | "caravan_camp" => Some("field_infirmary_round"),
        "mentor_cloister" => Some("mentor_cloister_oath"),
        "survey_tower" => Some("survey_tower_chart"),
        "guild_vault" => Some("guild_vault_audit"),
        "auction_arcade" => Some("auction_appraisal"),
        _ => None,
    }
}

pub fn trillionnium_objective_label_for_role(role: &str) -> &'static str {
    match role {
        "civic_square" => "集",
        "mentor_home" => "师",
        "ledger_hall" => "账",
        "sect_hall" => "门",
        "workshop" => "器",
        "market" => "市",
        "quest_board" => "榜",
        "delivery_route" => "路",
        "arbitration_desk" => "判",
        "arena" => "战",
        "raid_hall" => "盟",
        "archive" => "档",
        "patrol_yard" => "巡",
        "water_supply" => "水",
        "ration_kitchen" => "粮",
        "infirmary" => "医",
        "mediation_steps" => "和",
        "mentor_cloister" => "师",
        "courier_yard" => "信",
        "survey_tower" => "图",
        "guild_vault" => "库",
        "auction_arcade" => "拍",
        "caravan_camp" => "营",
        _ => "遇",
    }
}

pub fn trillionnium_objective_priority_for_role(role: &str) -> i64 {
    match role {
        "market" => 100,
        "arena" => 96,
        "delivery_route" => 92,
        "quest_board" => 88,
        "civic_square" => 84,
        "mentor_home" => 80,
        "ledger_hall" => 76,
        "workshop" => 72,
        "arbitration_desk" => 68,
        "sect_hall" => 64,
        "raid_hall" => 60,
        "archive" => 58,
        "patrol_yard" => 57,
        "water_supply" => 56,
        "ration_kitchen" => 55,
        "infirmary" => 54,
        "mediation_steps" => 53,
        "mentor_cloister" => 52,
        "courier_yard" => 51,
        "survey_tower" => 50,
        "guild_vault" => 49,
        "auction_arcade" => 48,
        "caravan_camp" => 47,
        _ => 10,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumDerivedStats {
    pub max_hp: i64,
    pub inner_energy: i64,
    pub move_range: u16,
    pub learning_speed: i64,
    pub negotiation_bonus: i64,
    pub craft_quality_bonus: i64,
    pub combat_power_hint: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumAttributes {
    pub physique: u16,
    pub force: u16,
    pub agility: u16,
    pub insight: u16,
    pub resolve: u16,
    pub craft: u16,
    pub commerce: u16,
    pub reputation: i32,
}

impl Default for TrillionniumAttributes {
    fn default() -> Self {
        Self {
            physique: 12,
            force: 11,
            agility: 12,
            insight: 13,
            resolve: 12,
            craft: 10,
            commerce: 10,
            reputation: 0,
        }
    }
}

impl TrillionniumAttributes {
    pub fn derived_stats(&self) -> TrillionniumDerivedStats {
        TrillionniumDerivedStats {
            max_hp: (80 + self.physique as i64 * 6 + self.resolve as i64 * 2).clamp(80, 260),
            inner_energy: (40 + self.resolve as i64 * 5 + self.insight as i64 * 2).clamp(40, 220),
            move_range: 3 + (self.agility / 8).clamp(0, 3),
            learning_speed: (100 + self.insight as i64 * 4).clamp(100, 220),
            negotiation_bonus: (self.commerce as i64 + (self.reputation / 10) as i64)
                .clamp(-25, 80),
            craft_quality_bonus: (self.craft as i64 / 2).clamp(0, 50),
            combat_power_hint: (self.force as i64 * 2 + self.agility as i64 + self.resolve as i64)
                .clamp(0, 160),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTrillionniumResourcePressureMutation {
    pub event_kind: String,
    pub command: String,
    pub time_delta_minutes: i64,
    pub stamina_delta: i64,
    pub injury_delta: i64,
    pub evidence_integrity_delta: i64,
    pub evidence_fragment_delta: i64,
    pub food_delta: i64,
    pub water_delta: i64,
    pub age_delta_days: i64,
    pub source_of_truth: String,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTrillionniumResourcePressureState {
    pub day_index: i64,
    pub minute_of_day: i64,
    pub stamina_current: i64,
    pub stamina_max: i64,
    pub injury_level: i64,
    pub evidence_integrity: i64,
    pub evidence_fragments: i64,
    pub food_current: i64,
    pub food_max: i64,
    pub water_current: i64,
    pub water_max: i64,
    pub age_days: i64,
    pub mutation_count: i64,
    pub last_mutation_command: Option<String>,
    pub last_mutation_event: Option<String>,
    pub last_mutation_result: Option<String>,
    pub updated_at_epoch: i64,
    #[serde(default)]
    pub recent_mutations: Vec<WorldTrillionniumResourcePressureMutation>,
}

impl Default for WorldTrillionniumResourcePressureState {
    fn default() -> Self {
        Self {
            day_index: 1,
            minute_of_day: 8 * 60,
            stamina_current: 100,
            stamina_max: 100,
            injury_level: 0,
            evidence_integrity: 72,
            evidence_fragments: 0,
            food_current: 76,
            food_max: 100,
            water_current: 82,
            water_max: 100,
            age_days: 19 * 360,
            mutation_count: 0,
            last_mutation_command: None,
            last_mutation_event: None,
            last_mutation_result: None,
            updated_at_epoch: 0,
            recent_mutations: Vec::new(),
        }
    }
}

impl WorldTrillionniumResourcePressureState {
    pub fn ensure_defaults(&mut self) {
        if self.day_index <= 0 {
            self.day_index = 1;
        }
        if !(0..(24 * 60)).contains(&self.minute_of_day) {
            self.minute_of_day = 8 * 60;
        }
        if self.stamina_max <= 0 {
            self.stamina_max = 100;
        }
        self.stamina_current = self.stamina_current.clamp(0, self.stamina_max);
        self.injury_level = self.injury_level.clamp(0, 4);
        self.evidence_integrity = self.evidence_integrity.max(72).clamp(0, 100);
        self.evidence_fragments = self.evidence_fragments.max(0);
        if self.food_max <= 0 {
            self.food_max = 100;
        }
        if self.water_max <= 0 {
            self.water_max = 100;
        }
        self.food_current = self.food_current.clamp(0, self.food_max);
        self.water_current = self.water_current.clamp(0, self.water_max);
        if self.age_days <= 0 {
            self.age_days = 19 * 360;
        }
    }

    pub fn stamina_status(&self) -> &'static str {
        if self.stamina_current <= 20 {
            "exhausted_risk"
        } else if self.stamina_current <= 45 {
            "strained"
        } else {
            "route_ready"
        }
    }

    pub fn survival_status(&self) -> &'static str {
        if self.food_current <= 12 || self.water_current <= 12 {
            "critical_survival_pressure"
        } else if self.food_current <= 34 || self.water_current <= 34 {
            "survival_pressure_visible"
        } else {
            "stable_survival_loop"
        }
    }

    pub fn apply_mutation(
        &mut self,
        event_kind: &str,
        command: &str,
        result: Option<&str>,
        now_epoch: i64,
    ) -> WorldTrillionniumResourcePressureMutation {
        self.ensure_defaults();
        let (
            time_delta_minutes,
            stamina_delta,
            injury_delta,
            evidence_integrity_delta,
            evidence_fragment_delta,
            food_delta,
            water_delta,
        ) = match event_kind {
            "world_map_move" => (12, -4, 0, 1, 1, -2, -4),
            "tactics_attack" => {
                let injury_delta = if result == Some("defender_routed") {
                    0
                } else {
                    1
                };
                (8, -14, injury_delta, -2, 0, -3, -5)
            }
            "tactics_complete_task" => (18, -6, -1, 12, 3, -2, -3),
            _ => (4, -1, 0, 0, 0, -1, -1),
        };
        let absolute_minute = self.minute_of_day + time_delta_minutes;
        let age_delta_days = absolute_minute.div_euclid(24 * 60).max(0);
        self.day_index += age_delta_days;
        self.minute_of_day = absolute_minute.rem_euclid(24 * 60);
        self.stamina_current = (self.stamina_current + stamina_delta).clamp(0, self.stamina_max);
        self.injury_level = (self.injury_level + injury_delta).clamp(0, 4);
        self.evidence_integrity =
            (self.evidence_integrity + evidence_integrity_delta).clamp(0, 100);
        self.evidence_fragments = (self.evidence_fragments + evidence_fragment_delta).max(0);
        self.food_current = (self.food_current + food_delta).clamp(0, self.food_max);
        self.water_current = (self.water_current + water_delta).clamp(0, self.water_max);
        self.age_days = (self.age_days + age_delta_days).max(0);
        if self.food_current <= 12 || self.water_current <= 12 {
            self.injury_level = (self.injury_level + 1).clamp(0, 4);
            self.stamina_current = self.stamina_current.min(20);
        } else if self.food_current <= 34 || self.water_current <= 34 {
            self.stamina_current = self.stamina_current.min(45);
        }
        self.mutation_count += 1;
        self.last_mutation_command = Some(command.to_string());
        self.last_mutation_event = Some(event_kind.to_string());
        self.last_mutation_result = result.map(ToString::to_string);
        self.updated_at_epoch = now_epoch;
        let mutation = WorldTrillionniumResourcePressureMutation {
            event_kind: event_kind.to_string(),
            command: command.to_string(),
            time_delta_minutes,
            stamina_delta,
            injury_delta,
            evidence_integrity_delta,
            evidence_fragment_delta,
            food_delta,
            water_delta,
            age_delta_days,
            source_of_truth: "rust_trillionnium_resource_pressure_runtime_state".to_string(),
            created_at_epoch: now_epoch,
        };
        self.recent_mutations.push(mutation.clone());
        if self.recent_mutations.len() > 8 {
            let overflow = self.recent_mutations.len() - 8;
            self.recent_mutations.drain(0..overflow);
        }
        mutation
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTrillionniumRegionStoryUnlockEvent {
    pub event_kind: String,
    pub command: String,
    pub node_id: Option<String>,
    pub zone_id: Option<String>,
    pub unlocked_region_ids: Vec<String>,
    pub unlocked_story_arc_ids: Vec<String>,
    pub source_of_truth: String,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTrillionniumRegionStoryUnlockState {
    pub unlocked_region_ids: Vec<String>,
    pub unlocked_story_arc_ids: Vec<String>,
    pub visited_node_ids: Vec<String>,
    pub visited_zone_ids: Vec<String>,
    pub mutation_count: i64,
    pub last_mutation_command: Option<String>,
    pub last_mutation_event: Option<String>,
    pub last_mutation_result: Option<String>,
    pub updated_at_epoch: i64,
    #[serde(default)]
    pub recent_unlock_events: Vec<WorldTrillionniumRegionStoryUnlockEvent>,
}

impl Default for WorldTrillionniumRegionStoryUnlockState {
    fn default() -> Self {
        Self {
            unlocked_region_ids: vec!["reality-mirror-city".to_string()],
            unlocked_story_arc_ids: vec!["mirror_city_arrival".to_string()],
            visited_node_ids: vec!["mirror-city-square".to_string()],
            visited_zone_ids: vec!["reality-mirror-city".to_string()],
            mutation_count: 0,
            last_mutation_command: None,
            last_mutation_event: None,
            last_mutation_result: None,
            updated_at_epoch: 0,
            recent_unlock_events: Vec::new(),
        }
    }
}

fn push_unique_unlock_id(values: &mut Vec<String>, value: &str) -> bool {
    let value = value.trim();
    if value.is_empty() || values.iter().any(|existing| existing == value) {
        false
    } else {
        values.push(value.to_string());
        true
    }
}

pub fn story_arc_ids_for_region_story_signal(
    event_kind: &str,
    command: &str,
    node_id: Option<&str>,
    zone_id: Option<&str>,
    result: Option<&str>,
) -> Vec<&'static str> {
    let mut arcs = Vec::new();
    let mut add = |arc_id: &'static str| {
        if !arcs.contains(&arc_id) {
            arcs.push(arc_id);
        }
    };
    if matches!(node_id, Some("mirror-city-square"))
        || matches!(zone_id, Some("reality-mirror-city"))
    {
        add("mirror_city_arrival");
    }
    match zone_id.unwrap_or_default() {
        "craft-district" => add("field_remedy_supply"),
        "market-bazaar" => {
            add("ledger_debt_storm");
            add("jade_route_patrol");
        }
        "league-arena" => add("raid_signal_return"),
        _ => {}
    }
    match node_id.unwrap_or_default() {
        "client-board" | "delivery-dock" => add("jade_route_patrol"),
        "ledger-office" => add("ledger_debt_storm"),
        "dispute-desk" => add("night_watch_dispute"),
        "forge-workbench" | "asset-yard" | "starter-studio" => add("field_remedy_supply"),
        "league-coliseum" | "raid-hall" => add("raid_signal_return"),
        _ => {}
    }
    if event_kind == "tactics_attack" || command == "attack" {
        add("raid_signal_return");
    }
    if event_kind == "tactics_complete_task" || command == "complete_task" {
        add("jade_route_patrol");
        add("ledger_debt_storm");
        if result == Some("task_completion_validated") {
            add("night_watch_dispute");
        }
    }
    arcs
}

impl WorldTrillionniumRegionStoryUnlockState {
    pub fn ensure_defaults(&mut self) {
        push_unique_unlock_id(&mut self.unlocked_region_ids, "reality-mirror-city");
        push_unique_unlock_id(&mut self.unlocked_story_arc_ids, "mirror_city_arrival");
        push_unique_unlock_id(&mut self.visited_node_ids, "mirror-city-square");
        push_unique_unlock_id(&mut self.visited_zone_ids, "reality-mirror-city");
    }

    pub fn apply_mutation(
        &mut self,
        event_kind: &str,
        command: &str,
        result: Option<&str>,
        node_id: Option<&str>,
        zone_id: Option<&str>,
        now_epoch: i64,
    ) -> WorldTrillionniumRegionStoryUnlockEvent {
        self.ensure_defaults();
        let mut newly_unlocked_regions = Vec::new();
        let mut newly_unlocked_arcs = Vec::new();
        if let Some(node_id) = node_id.map(str::trim).filter(|value| !value.is_empty()) {
            push_unique_unlock_id(&mut self.visited_node_ids, node_id);
        }
        if let Some(zone_id) = zone_id.map(str::trim).filter(|value| !value.is_empty()) {
            push_unique_unlock_id(&mut self.visited_zone_ids, zone_id);
            if push_unique_unlock_id(&mut self.unlocked_region_ids, zone_id) {
                newly_unlocked_regions.push(zone_id.to_string());
            }
        }
        for arc_id in
            story_arc_ids_for_region_story_signal(event_kind, command, node_id, zone_id, result)
        {
            if push_unique_unlock_id(&mut self.unlocked_story_arc_ids, arc_id) {
                newly_unlocked_arcs.push(arc_id.to_string());
            }
        }
        self.mutation_count += 1;
        self.last_mutation_command = Some(command.to_string());
        self.last_mutation_event = Some(event_kind.to_string());
        self.last_mutation_result = result.map(ToString::to_string);
        self.updated_at_epoch = now_epoch;
        let unlock_event = WorldTrillionniumRegionStoryUnlockEvent {
            event_kind: event_kind.to_string(),
            command: command.to_string(),
            node_id: node_id.map(ToString::to_string),
            zone_id: zone_id.map(ToString::to_string),
            unlocked_region_ids: newly_unlocked_regions,
            unlocked_story_arc_ids: newly_unlocked_arcs,
            source_of_truth: "rust_trillionnium_region_story_unlock_runtime_state".to_string(),
            created_at_epoch: now_epoch,
        };
        self.recent_unlock_events.push(unlock_event.clone());
        if self.recent_unlock_events.len() > 8 {
            let overflow = self.recent_unlock_events.len() - 8;
            self.recent_unlock_events.drain(0..overflow);
        }
        unlock_event
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTrillionniumCombatExchange {
    pub event_kind: String,
    pub command: String,
    pub attacker_unit_id: String,
    pub defender_unit_id: String,
    pub target_tile: String,
    pub skill_id: String,
    pub damage_dealt: i64,
    pub defender_hp_before: i64,
    pub defender_hp_after: i64,
    pub player_hp_delta: i64,
    pub inner_energy_delta: i64,
    pub guard_delta: i64,
    pub focus_delta: i64,
    pub mitigation_applied: i64,
    pub hit_quality: String,
    pub critical: bool,
    pub stance_after: String,
    pub tempo_after: String,
    pub result: String,
    pub source_of_truth: String,
    pub created_at_epoch: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTrillionniumCombatNumericsState {
    pub hp_current: i64,
    pub hp_max: i64,
    pub inner_energy_current: i64,
    pub inner_energy_max: i64,
    pub guard_current: i64,
    pub guard_max: i64,
    pub focus_current: i64,
    pub focus_max: i64,
    pub injury_level: i64,
    pub stance: String,
    pub tempo: String,
    pub hit_chance: i64,
    pub critical_chance: i64,
    pub mitigation_rating: i64,
    pub mutation_count: i64,
    pub last_mutation_command: Option<String>,
    pub last_mutation_event: Option<String>,
    pub last_mutation_result: Option<String>,
    pub updated_at_epoch: i64,
    #[serde(default)]
    pub recent_exchanges: Vec<WorldTrillionniumCombatExchange>,
}

impl Default for WorldTrillionniumCombatNumericsState {
    fn default() -> Self {
        Self {
            hp_current: 176,
            hp_max: 176,
            inner_energy_current: 126,
            inner_energy_max: 126,
            guard_current: 24,
            guard_max: 24,
            focus_current: 103,
            focus_max: 103,
            injury_level: 0,
            stance: "balanced_guard".to_string(),
            tempo: "steady".to_string(),
            hit_chance: 73,
            critical_chance: 11,
            mitigation_rating: 18,
            mutation_count: 0,
            last_mutation_command: None,
            last_mutation_event: None,
            last_mutation_result: None,
            updated_at_epoch: 0,
            recent_exchanges: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumCombatResolutionInput {
    pub attacker_unit_id: String,
    pub defender_unit_id: String,
    pub target_tile: String,
    pub skill_id: String,
    pub damage: i64,
    pub defender_hp_before: i64,
    pub defender_hp_after: i64,
    pub result: String,
}

impl WorldTrillionniumCombatNumericsState {
    fn derived_max_hp(attributes: &TrillionniumAttributes) -> i64 {
        (80 + attributes.physique as i64 * 6 + attributes.resolve as i64 * 2).clamp(80, 260)
    }

    fn derived_inner_energy_max(attributes: &TrillionniumAttributes) -> i64 {
        (40 + attributes.resolve as i64 * 5 + attributes.insight as i64 * 2).clamp(40, 220)
    }

    fn derived_guard_max(attributes: &TrillionniumAttributes) -> i64 {
        (8 + attributes.resolve as i64 + attributes.physique as i64 / 2).clamp(8, 64)
    }

    fn derived_focus_max(attributes: &TrillionniumAttributes) -> i64 {
        (40 + attributes.insight as i64 * 3 + attributes.agility as i64 * 2).clamp(40, 160)
    }

    pub fn ensure_defaults(&mut self, attributes: &TrillionniumAttributes) {
        let hp_max = Self::derived_max_hp(attributes);
        let inner_energy_max = Self::derived_inner_energy_max(attributes);
        let guard_max = Self::derived_guard_max(attributes);
        let focus_max = Self::derived_focus_max(attributes);
        self.hp_max = self.hp_max.max(hp_max).clamp(80, 320);
        self.hp_current = self.hp_current.clamp(0, self.hp_max);
        self.inner_energy_max = self.inner_energy_max.max(inner_energy_max).clamp(40, 260);
        self.inner_energy_current = self.inner_energy_current.clamp(0, self.inner_energy_max);
        self.guard_max = self.guard_max.max(guard_max).clamp(8, 80);
        self.guard_current = self.guard_current.clamp(0, self.guard_max);
        self.focus_max = self.focus_max.max(focus_max).clamp(40, 180);
        self.focus_current = self.focus_current.clamp(0, self.focus_max);
        self.injury_level = self.injury_level.clamp(0, 5);
        if self.stance.trim().is_empty() {
            self.stance = "balanced_guard".to_string();
        }
        if self.tempo.trim().is_empty() {
            self.tempo = "steady".to_string();
        }
        self.hit_chance =
            (55 + attributes.agility as i64 + attributes.insight as i64 / 2).clamp(40, 95);
        self.critical_chance =
            (5 + attributes.insight as i64 / 3 + attributes.force as i64 / 4).clamp(5, 40);
        self.mitigation_rating =
            (self.guard_current / 2 + attributes.resolve as i64 / 2).clamp(0, 80);
    }

    pub fn health_status(&self) -> &'static str {
        if self.hp_current <= 0 {
            "routed_recovery_required"
        } else if self.hp_current * 4 <= self.hp_max {
            "critical"
        } else if self.hp_current * 2 <= self.hp_max {
            "wounded"
        } else {
            "combat_ready"
        }
    }

    pub fn apply_attack(
        &mut self,
        event_kind: &str,
        command: &str,
        combat_resolution: &TrillionniumCombatResolutionInput,
        attributes: &TrillionniumAttributes,
        now_epoch: i64,
    ) -> WorldTrillionniumCombatExchange {
        self.ensure_defaults(attributes);
        let result = combat_resolution.result.as_str();
        let damage_dealt = combat_resolution.damage.max(0);
        let critical = damage_dealt >= 28 || (result == "defender_routed" && damage_dealt >= 20);
        let hit_quality = if critical {
            "critical_route_break"
        } else if damage_dealt >= 18 {
            "solid_hit"
        } else {
            "glancing_hit"
        };
        let inner_energy_delta = match combat_resolution.skill_id.as_str() {
            "basic_blade" | "basic_sword" => -10,
            "basic_inner_power" => -7,
            "basic_unarmed" => -8,
            _ => -6,
        };
        let guard_delta = if result == "defender_routed" { -1 } else { -3 };
        let focus_delta = if result == "defender_routed" { 4 } else { -6 };
        let incoming_pressure: i64 = if result == "defender_routed" { 4 } else { 16 };
        let mitigation_applied = (self.mitigation_rating / 4).clamp(0, 12);
        let player_hp_delta = -(incoming_pressure.saturating_sub(mitigation_applied).max(1));
        self.hp_current = (self.hp_current + player_hp_delta).clamp(0, self.hp_max);
        self.inner_energy_current =
            (self.inner_energy_current + inner_energy_delta).clamp(0, self.inner_energy_max);
        self.guard_current = (self.guard_current + guard_delta).clamp(0, self.guard_max);
        self.focus_current = (self.focus_current + focus_delta).clamp(0, self.focus_max);
        if self.hp_current * 3 <= self.hp_max || result != "defender_routed" {
            self.injury_level = (self.injury_level + 1).clamp(0, 5);
        }
        self.stance = if self.guard_current * 3 <= self.guard_max {
            "open_guard".to_string()
        } else if self.focus_current * 3 >= self.focus_max * 2 {
            "pressing_guard".to_string()
        } else {
            "balanced_guard".to_string()
        };
        self.tempo = if result == "defender_routed" {
            "initiative".to_string()
        } else if self.focus_current * 3 <= self.focus_max {
            "under_pressure".to_string()
        } else {
            "contested".to_string()
        };
        self.mutation_count += 1;
        self.last_mutation_command = Some(command.to_string());
        self.last_mutation_event = Some(event_kind.to_string());
        self.last_mutation_result = Some(result.to_string());
        self.updated_at_epoch = now_epoch;
        self.ensure_defaults(attributes);
        let exchange = WorldTrillionniumCombatExchange {
            event_kind: event_kind.to_string(),
            command: command.to_string(),
            attacker_unit_id: combat_resolution.attacker_unit_id.clone(),
            defender_unit_id: combat_resolution.defender_unit_id.clone(),
            target_tile: combat_resolution.target_tile.clone(),
            skill_id: combat_resolution.skill_id.clone(),
            damage_dealt,
            defender_hp_before: combat_resolution.defender_hp_before.max(0),
            defender_hp_after: combat_resolution.defender_hp_after.max(0),
            player_hp_delta,
            inner_energy_delta,
            guard_delta,
            focus_delta,
            mitigation_applied,
            hit_quality: hit_quality.to_string(),
            critical,
            stance_after: self.stance.clone(),
            tempo_after: self.tempo.clone(),
            result: result.to_string(),
            source_of_truth: "rust_trillionnium_combat_numerics_runtime_state".to_string(),
            created_at_epoch: now_epoch,
        };
        self.recent_exchanges.push(exchange.clone());
        if self.recent_exchanges.len() > 8 {
            let overflow = self.recent_exchanges.len() - 8;
            self.recent_exchanges.drain(0..overflow);
        }
        exchange
    }
}

/// CEX-compatible short id helper used by extracted world runtime records.
pub fn trillionnium_hash_id(prefix: &str, value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    let encoded = BASE64_URL_SAFE_NO_PAD.encode(hasher.finalize());
    format!("{prefix}-{}", &encoded[..16])
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumItemEquipmentCatalogItem {
    pub item_id: String,
    pub slot: String,
    pub family: String,
    pub display_name: String,
    pub use_case: String,
    pub combat_effect: String,
    pub world_effect: String,
}

fn item_equipment_catalog_item(
    item_id: &str,
    slot: &str,
    family: &str,
    display_name: &str,
    use_case: &str,
    combat_effect: &str,
    world_effect: &str,
) -> TrillionniumItemEquipmentCatalogItem {
    TrillionniumItemEquipmentCatalogItem {
        item_id: item_id.to_string(),
        slot: slot.to_string(),
        family: family.to_string(),
        display_name: display_name.to_string(),
        use_case: use_case.to_string(),
        combat_effect: combat_effect.to_string(),
        world_effect: world_effect.to_string(),
    }
}

pub fn trillionnium_item_equipment_catalog_items() -> Vec<TrillionniumItemEquipmentCatalogItem> {
    vec![
        item_equipment_catalog_item(
            "ledger-seal-token",
            "quest_proof",
            "evidence",
            "Ledger Seal Token / 账印令",
            "binds submitted proof to settlement review",
            "protects one objective proof bundle from interruption",
            "improves review-hold release quality",
        ),
        item_equipment_catalog_item(
            "street-compass-bracer",
            "wrist",
            "navigation",
            "Street Compass Bracer / 街指南护腕",
            "marks nearby exits and mentor anchors",
            "adds one focus point on objective-entry turns",
            "reduces wrong-node route attempts",
        ),
        item_equipment_catalog_item(
            "route-guard-staff",
            "weapon",
            "staff",
            "Route Guard Staff / 护路棍",
            "escort and patrol starter weapon",
            "extends melee zone control",
            "improves escort-route safety",
        ),
        item_equipment_catalog_item(
            "iron-workshop-blade",
            "weapon",
            "blade",
            "Iron Workshop Blade / 铁坊短刀",
            "workshop repair and armored encounter starter",
            "improves attack against armored targets",
            "improves artifact repair task quality",
        ),
        item_equipment_catalog_item(
            "market-wind-sword",
            "weapon",
            "sword",
            "Market Wind Sword / 集风剑",
            "market negotiation duel starter",
            "improves precision attack",
            "raises opening negotiation quality",
        ),
        item_equipment_catalog_item(
            "night-watch-cloak",
            "cloak",
            "lightness",
            "Night Watch Cloak / 夜巡披风",
            "patrol movement and evasion",
            "raises evade on low-light tiles",
            "reduces travel friction on night routes",
        ),
        item_equipment_catalog_item(
            "field-tonic-kit",
            "consumable",
            "medicine",
            "Field Tonic Kit / 行药包",
            "recover from failed encounters and long patrols",
            "recovers minor wounds after combat",
            "reduces party downtime",
        ),
        item_equipment_catalog_item(
            "relay-core-fragment",
            "relic",
            "relay_salvage",
            "Relay Core Fragment / 中继芯片",
            "a recovered First Contact relay component",
            "raises ability energy, protection and reach",
            "records that the relay was secured and salvaged",
        ),
        item_equipment_catalog_item(
            "evidence-wrap-case",
            "pack",
            "evidence",
            "Evidence Wrap Case / 证据匣",
            "holds task photos, reports, and review notes",
            "prevents proof loss on objective tiles",
            "improves proof completeness scoring",
        ),
        item_equipment_catalog_item(
            "auction-eye-lens",
            "tool",
            "auction",
            "Auction Eye Lens / 拍卖镜",
            "inspect bounty price and item quality",
            "reveals supply tile quality",
            "improves bounty pricing",
        ),
        item_equipment_catalog_item(
            "raid-signal-drum",
            "party_tool",
            "raid_command",
            "Raid Signal Drum / 会战鼓",
            "coordinate party entry and return state",
            "focuses party objective attacks",
            "unlocks group route coordination",
        ),
        item_equipment_catalog_item(
            "map-tile-rubbing",
            "map_note",
            "terrain",
            "Map Tile Rubbing / 地格拓片",
            "records blocked terrain and transition rules",
            "reduces movement penalty on known terrain",
            "improves route explanation",
        ),
        item_equipment_catalog_item(
            "sect-registry-tag",
            "identity",
            "sect",
            "Sect Registry Tag / 门籍牌",
            "tracks sect title ladder and mentor trust",
            "adds morale on sect trial objectives",
            "improves relationship recovery",
        ),
    ]
}

pub fn trillionnium_item_equipment_catalog_json() -> Value {
    json!({
        "contract_version": "trillionnium_native_item_equipment_catalog_v1",
        "source_of_truth": "rust_trillionnium_item_equipment_catalog",
        "content_policy": "trillionnium_native_no_copied_hero_tan_text_assets_or_tables",
        "runtime_status": "catalog_drives_rust_owned_inventory_and_equip_slots",
        "items": trillionnium_item_equipment_catalog_items(),
    })
}

pub fn trillionnium_catalog_item_field(item_id: &str, field: &str) -> Option<String> {
    let item = trillionnium_item_equipment_catalog_items()
        .into_iter()
        .find(|item| item.item_id == item_id)?;
    match field {
        "slot" => Some(item.slot),
        "family" => Some(item.family),
        "display_name" => Some(item.display_name),
        "use_case" => Some(item.use_case),
        "combat_effect" => Some(item.combat_effect),
        "world_effect" => Some(item.world_effect),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTrillionniumInventoryItem {
    pub item_instance_id: String,
    pub item_id: String,
    pub slot: String,
    pub family: String,
    pub display_name: String,
    pub quantity: u16,
    pub quality: String,
    pub equipped_slot: Option<String>,
    pub acquired_from: String,
    pub acquired_at_epoch: i64,
    pub updated_at_epoch: i64,
}

pub fn trillionnium_inventory_item_for(
    matrix_user_id: &str,
    item_id: &str,
    acquired_from: &str,
    equipped_slot: Option<&str>,
    now_epoch: i64,
) -> Option<WorldTrillionniumInventoryItem> {
    let slot = trillionnium_catalog_item_field(item_id, "slot")?;
    let family = trillionnium_catalog_item_field(item_id, "family")?;
    let display_name = trillionnium_catalog_item_field(item_id, "display_name")?;
    Some(WorldTrillionniumInventoryItem {
        item_instance_id: trillionnium_hash_id(
            "world-trillionnium-inventory-item",
            &format!("{matrix_user_id}:{item_id}"),
        ),
        item_id: item_id.to_string(),
        slot,
        family,
        display_name,
        quantity: 1,
        quality: "starter".to_string(),
        equipped_slot: equipped_slot.map(ToString::to_string),
        acquired_from: acquired_from.to_string(),
        acquired_at_epoch: now_epoch,
        updated_at_epoch: now_epoch,
    })
}

pub fn default_trillionnium_inventory_items(
    matrix_user_id: &str,
    now_epoch: i64,
) -> Vec<WorldTrillionniumInventoryItem> {
    [
        ("route-guard-staff", Some("weapon")),
        ("street-compass-bracer", Some("wrist")),
        ("evidence-wrap-case", Some("pack")),
    ]
    .into_iter()
    .filter_map(|(item_id, equipped_slot)| {
        trillionnium_inventory_item_for(
            matrix_user_id,
            item_id,
            "rust_default_trillionnium_starter_loadout",
            equipped_slot,
            now_epoch,
        )
    })
    .collect()
}

pub fn default_trillionnium_equipment_slots(
    inventory_items: &[WorldTrillionniumInventoryItem],
) -> HashMap<String, String> {
    inventory_items
        .iter()
        .filter_map(|item| {
            item.equipped_slot
                .as_ref()
                .map(|slot| (slot.clone(), item.item_instance_id.clone()))
        })
        .collect()
}

impl TrillionniumAttributes {
    pub fn to_value(&self) -> Value {
        json!({
            "physique": self.physique,
            "force": self.force,
            "agility": self.agility,
            "insight": self.insight,
            "resolve": self.resolve,
            "craft": self.craft,
            "commerce": self.commerce,
            "reputation": self.reputation,
            "derived_stats": self.derived_stats(),
        })
    }
}

impl WorldTrillionniumResourcePressureMutation {
    pub fn to_value(&self) -> Value {
        json!({
            "event_kind": self.event_kind,
            "command": self.command,
            "time_delta_minutes": self.time_delta_minutes,
            "stamina_delta": self.stamina_delta,
            "injury_delta": self.injury_delta,
            "evidence_integrity_delta": self.evidence_integrity_delta,
            "evidence_fragment_delta": self.evidence_fragment_delta,
            "food_delta": self.food_delta,
            "water_delta": self.water_delta,
            "age_delta_days": self.age_delta_days,
            "source_of_truth": self.source_of_truth,
            "created_at_epoch": self.created_at_epoch,
        })
    }

    pub fn to_survival_value(&self) -> Value {
        json!({
            "contract_version": TRILLIONNIUM_WORLD_FOOD_WATER_AGE_SURVIVAL_CONTRACT_VERSION,
            "source_of_truth": "rust_trillionnium_food_water_age_survival_state",
            "event_kind": self.event_kind,
            "command": self.command,
            "time_delta_minutes": self.time_delta_minutes,
            "food_delta": self.food_delta,
            "water_delta": self.water_delta,
            "age_delta_days": self.age_delta_days,
            "created_at_epoch": self.created_at_epoch,
            "web_role": "visualization_input_only",
        })
    }
}

impl WorldTrillionniumResourcePressureState {
    pub fn clock_label(&self) -> String {
        format!(
            "{:02}:{:02}",
            self.minute_of_day / 60,
            self.minute_of_day % 60
        )
    }

    pub fn injury_status(&self) -> &'static str {
        match self.injury_level {
            0 => "clear",
            1 => "bruised",
            2 => "wounded",
            3 => "downtime_recommended",
            _ => "must_recover_before_risk_route",
        }
    }

    pub fn evidence_status(&self) -> &'static str {
        if self.evidence_integrity >= 82 && self.evidence_fragments >= 3 {
            "review_ready"
        } else if self.evidence_integrity >= 62 {
            "draft_evidence_bundle"
        } else {
            "review_hold_risk"
        }
    }

    pub fn food_status(&self) -> &'static str {
        if self.food_current <= 12 {
            "starvation_risk"
        } else if self.food_current <= 34 {
            "hungry"
        } else {
            "fed"
        }
    }

    pub fn water_status(&self) -> &'static str {
        if self.water_current <= 12 {
            "dehydration_risk"
        } else if self.water_current <= 34 {
            "thirsty"
        } else {
            "hydrated"
        }
    }

    pub fn age_years(&self) -> i64 {
        (self.age_days / 360).max(1)
    }

    pub fn age_stage(&self) -> &'static str {
        match self.age_years() {
            0..=17 => "apprentice",
            18..=29 => "young_adult",
            30..=49 => "seasoned",
            50..=69 => "elder",
            _ => "ancient_legend",
        }
    }

    pub fn survival_to_value(&self) -> Value {
        json!({
            "contract_version": TRILLIONNIUM_WORLD_FOOD_WATER_AGE_SURVIVAL_CONTRACT_VERSION,
            "source_of_truth": "rust_trillionnium_food_water_age_survival_state",
            "persistence_owner": "world_state.world_trillionnium_characters.resource_pressure_state.food_water_age",
            "runtime_status": "rust_owned_food_water_age_decay_live",
            "tracked_domains": ["food", "water", "age", "stamina_consequences", "injury_consequences"],
            "mutation_sources": ["world_map_move", "tactics_attack", "tactics_complete_task"],
            "food": {"current": self.food_current, "max": self.food_max, "status": self.food_status()},
            "water": {"current": self.water_current, "max": self.water_max, "status": self.water_status()},
            "age": {"days": self.age_days, "years": self.age_years(), "stage": self.age_stage()},
            "survival_pressure_status": self.survival_status(),
            "consequences": {
                "low_food_caps_stamina": true,
                "low_water_caps_stamina": true,
                "critical_food_or_water_adds_injury": true,
                "age_advances_on_world_day_rollover": true,
            },
            "mutation_count": self.mutation_count,
            "last_mutation_command": self.last_mutation_command,
            "last_mutation_event": self.last_mutation_event,
            "recent_mutations": self.recent_mutations.iter().map(WorldTrillionniumResourcePressureMutation::to_survival_value).collect::<Vec<_>>(),
            "updated_at_epoch": self.updated_at_epoch,
            "web_role": "visualization_input_only",
        })
    }

    pub fn to_value(&self) -> Value {
        json!({
            "contract_version": TRILLIONNIUM_WORLD_RESOURCE_PRESSURE_RUNTIME_CONTRACT_VERSION,
            "source_of_truth": "rust_trillionnium_resource_pressure_runtime_state",
            "persistence_owner": "world_state.world_trillionnium_characters.resource_pressure_state",
            "runtime_status": "rust_owned_time_stamina_injury_evidence_food_water_age_live",
            "tracked_domains": ["time", "stamina", "injury", "evidence_integrity", "food", "water", "age"],
            "mutation_sources": ["world_map_move", "tactics_attack", "tactics_complete_task"],
            "time": {"day_index": self.day_index, "minute_of_day": self.minute_of_day, "clock_label": self.clock_label()},
            "stamina": {"current": self.stamina_current, "max": self.stamina_max, "status": self.stamina_status()},
            "injury": {"level": self.injury_level, "status": self.injury_status()},
            "evidence_integrity": {"score": self.evidence_integrity, "fragments": self.evidence_fragments, "status": self.evidence_status()},
            "survival_runtime_contract_version": TRILLIONNIUM_WORLD_FOOD_WATER_AGE_SURVIVAL_CONTRACT_VERSION,
            "survival": self.survival_to_value(),
            "mutation_count": self.mutation_count,
            "last_mutation_command": self.last_mutation_command,
            "last_mutation_event": self.last_mutation_event,
            "last_mutation_result": self.last_mutation_result,
            "recent_mutations": self.recent_mutations.iter().map(WorldTrillionniumResourcePressureMutation::to_value).collect::<Vec<_>>(),
            "updated_at_epoch": self.updated_at_epoch,
            "web_role": "visualization_input_only",
        })
    }
}

impl WorldTrillionniumRegionStoryUnlockEvent {
    pub fn to_value(&self) -> Value {
        json!({
            "event_kind": self.event_kind,
            "command": self.command,
            "node_id": self.node_id,
            "zone_id": self.zone_id,
            "unlocked_region_ids": self.unlocked_region_ids,
            "unlocked_story_arc_ids": self.unlocked_story_arc_ids,
            "source_of_truth": self.source_of_truth,
            "created_at_epoch": self.created_at_epoch,
        })
    }
}

impl WorldTrillionniumRegionStoryUnlockState {
    pub fn to_value(&self) -> Value {
        let region_graph = [
            ("reality-mirror-city", "mirror-city-square", "first_hub_and_identity"),
            ("craft-district", "starter-studio", "crafting_recovery_and_item_routes"),
            ("market-bazaar", "zbj-market-gate", "bounty_contracts_and_review_hold_routes"),
            ("league-arena", "league-coliseum", "combat_raid_and_return_routes"),
        ]
        .into_iter()
        .map(|(region_id, entry_node_id, role)| {
            json!({
                "region_id": region_id,
                "entry_node_id": entry_node_id,
                "story_role": role,
                "unlock_status": if self.unlocked_region_ids.iter().any(|id| id == region_id) { "unlocked" } else { "locked" },
                "source_of_truth": "rust_world_map_nodes_and_region_story_unlock_state",
            })
        })
        .collect::<Vec<_>>();
        json!({
            "contract_version": TRILLIONNIUM_WORLD_REGION_STORY_UNLOCK_RUNTIME_CONTRACT_VERSION,
            "source_of_truth": "rust_trillionnium_region_story_unlock_runtime_state",
            "persistence_owner": "world_state.world_trillionnium_characters.region_story_unlock_state",
            "runtime_status": "rust_owned_region_graph_story_arc_unlocks_live",
            "tracked_domains": ["regions", "story_arcs", "visited_nodes", "visited_zones"],
            "mutation_sources": ["world_map_move", "tactics_attack", "tactics_complete_task"],
            "region_graph": region_graph,
            "unlocked_region_ids": self.unlocked_region_ids,
            "unlocked_story_arc_ids": self.unlocked_story_arc_ids,
            "visited_node_ids": self.visited_node_ids,
            "visited_zone_ids": self.visited_zone_ids,
            "unlocked_region_count": self.unlocked_region_ids.len(),
            "unlocked_story_arc_count": self.unlocked_story_arc_ids.len(),
            "visited_node_count": self.visited_node_ids.len(),
            "visited_zone_count": self.visited_zone_ids.len(),
            "mutation_count": self.mutation_count,
            "last_mutation_command": self.last_mutation_command,
            "last_mutation_event": self.last_mutation_event,
            "last_mutation_result": self.last_mutation_result,
            "recent_unlock_events": self.recent_unlock_events.iter().map(WorldTrillionniumRegionStoryUnlockEvent::to_value).collect::<Vec<_>>(),
            "updated_at_epoch": self.updated_at_epoch,
            "web_role": "visualization_input_only",
        })
    }
}

impl WorldTrillionniumCombatExchange {
    pub fn to_value(&self) -> Value {
        json!({
            "event_kind": self.event_kind,
            "command": self.command,
            "attacker_unit_id": self.attacker_unit_id,
            "defender_unit_id": self.defender_unit_id,
            "target_tile": self.target_tile,
            "skill_id": self.skill_id,
            "damage_dealt": self.damage_dealt,
            "defender_hp_before": self.defender_hp_before,
            "defender_hp_after": self.defender_hp_after,
            "player_hp_delta": self.player_hp_delta,
            "inner_energy_delta": self.inner_energy_delta,
            "guard_delta": self.guard_delta,
            "focus_delta": self.focus_delta,
            "mitigation_applied": self.mitigation_applied,
            "hit_quality": self.hit_quality,
            "critical": self.critical,
            "stance_after": self.stance_after,
            "tempo_after": self.tempo_after,
            "result": self.result,
            "source_of_truth": self.source_of_truth,
            "created_at_epoch": self.created_at_epoch,
        })
    }
}

impl WorldTrillionniumCombatNumericsState {
    pub fn energy_status(&self) -> &'static str {
        if self.inner_energy_current * 4 <= self.inner_energy_max {
            "low_inner_energy"
        } else if self.inner_energy_current * 2 <= self.inner_energy_max {
            "managed_breath"
        } else {
            "flowing"
        }
    }

    pub fn focus_status(&self) -> &'static str {
        if self.focus_current * 4 <= self.focus_max {
            "shaken"
        } else if self.focus_current * 2 <= self.focus_max {
            "contested"
        } else {
            "focused"
        }
    }

    pub fn to_value(&self) -> Value {
        let latest_hit_quality = self
            .recent_exchanges
            .last()
            .map(|exchange| exchange.hit_quality.as_str())
            .unwrap_or("none");
        json!({
            "contract_version": TRILLIONNIUM_WORLD_COMBAT_NUMERICS_RUNTIME_CONTRACT_VERSION,
            "source_of_truth": "rust_trillionnium_combat_numerics_runtime_state",
            "persistence_owner": "world_state.world_trillionnium_characters.combat_numerics_state",
            "runtime_status": "rust_owned_hp_energy_guard_focus_hitcrit_live",
            "tracked_domains": ["hp", "inner_energy", "guard", "focus", "injury", "hit_quality", "critical", "mitigation", "stance", "tempo"],
            "mutation_sources": ["tactics_attack"],
            "health": {"current": self.hp_current, "max": self.hp_max, "status": self.health_status()},
            "inner_energy": {"current": self.inner_energy_current, "max": self.inner_energy_max, "status": self.energy_status()},
            "guard": {"current": self.guard_current, "max": self.guard_max, "stance": self.stance},
            "focus": {"current": self.focus_current, "max": self.focus_max, "status": self.focus_status()},
            "injury": {"level": self.injury_level, "status": self.health_status()},
            "offense": {"hit_chance": self.hit_chance, "critical_chance": self.critical_chance, "latest_hit_quality": latest_hit_quality},
            "defense": {"mitigation_rating": self.mitigation_rating, "guard_current": self.guard_current},
            "stance": self.stance,
            "tempo": self.tempo,
            "mutation_count": self.mutation_count,
            "last_mutation_command": self.last_mutation_command,
            "last_mutation_event": self.last_mutation_event,
            "last_mutation_result": self.last_mutation_result,
            "recent_exchanges": self.recent_exchanges.iter().map(WorldTrillionniumCombatExchange::to_value).collect::<Vec<_>>(),
            "updated_at_epoch": self.updated_at_epoch,
            "web_role": "visualization_input_only",
        })
    }
}

pub fn trillionnium_known_skill_definitions_json(skill_ids: &[String]) -> Value {
    Value::Array(
        trillionnium_fixture_skill_definitions()
            .into_iter()
            .filter(|skill| skill_ids.iter().any(|skill_id| skill_id == &skill.skill_id))
            .map(|skill| serde_json::to_value(skill).expect("skill definition serializes"))
            .collect(),
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTrillionniumCharacter {
    pub matrix_user_id: String,
    pub character_id: String,
    pub display_name: String,
    pub attributes: TrillionniumAttributes,
    pub sect_id: Option<String>,
    pub title: String,
    pub skill_ids: Vec<String>,
    #[serde(default)]
    pub inventory_items: Vec<WorldTrillionniumInventoryItem>,
    #[serde(default)]
    pub equipment_slots: HashMap<String, String>,
    #[serde(default)]
    pub resource_pressure_state: WorldTrillionniumResourcePressureState,
    #[serde(default)]
    pub region_story_unlock_state: WorldTrillionniumRegionStoryUnlockState,
    #[serde(default)]
    pub combat_numerics_state: WorldTrillionniumCombatNumericsState,
    pub updated_at_epoch: i64,
}

impl WorldTrillionniumCharacter {
    pub fn default_for(matrix_user_id: &str) -> Self {
        let inventory_items = default_trillionnium_inventory_items(matrix_user_id, 0);
        let equipment_slots = default_trillionnium_equipment_slots(&inventory_items);
        Self {
            matrix_user_id: matrix_user_id.to_string(),
            character_id: trillionnium_hash_id("trillionnium-character", matrix_user_id),
            display_name: "镜城游侠".to_string(),
            attributes: TrillionniumAttributes::default(),
            sect_id: None,
            title: "初入Trillionnium".to_string(),
            skill_ids: vec![
                "basic_inner_power".to_string(),
                "basic_lightness".to_string(),
                "reading_and_contracts".to_string(),
            ],
            inventory_items,
            equipment_slots,
            resource_pressure_state: WorldTrillionniumResourcePressureState::default(),
            region_story_unlock_state: WorldTrillionniumRegionStoryUnlockState::default(),
            combat_numerics_state: WorldTrillionniumCombatNumericsState::default(),
            updated_at_epoch: 0,
        }
    }

    pub fn ensure_defaults(&mut self, now_epoch: i64) {
        if self.inventory_items.is_empty() {
            self.inventory_items =
                default_trillionnium_inventory_items(&self.matrix_user_id, now_epoch);
        }
        if self.equipment_slots.is_empty() {
            self.equipment_slots = default_trillionnium_equipment_slots(&self.inventory_items);
        }
        self.resource_pressure_state.ensure_defaults();
        self.region_story_unlock_state.ensure_defaults();
        self.combat_numerics_state.ensure_defaults(&self.attributes);
    }

    pub fn equip_item_by_id(&mut self, item_id: &str, now_epoch: i64) -> Option<(String, String)> {
        self.ensure_defaults(now_epoch);
        let (slot, item_instance_id) = {
            let item = self
                .inventory_items
                .iter_mut()
                .find(|candidate| candidate.item_id == item_id)?;
            item.updated_at_epoch = now_epoch;
            item.equipped_slot = Some(item.slot.clone());
            (item.slot.clone(), item.item_instance_id.clone())
        };
        self.equipment_slots
            .insert(slot.clone(), item_instance_id.clone());
        self.updated_at_epoch = now_epoch;
        Some((slot, item_instance_id))
    }

    pub fn item_equipment_runtime_json(&self) -> Value {
        json!({
            "contract_version": TRILLIONNIUM_WORLD_ITEM_EQUIPMENT_RUNTIME_CONTRACT_VERSION,
            "source_of_truth": "rust_trillionnium_item_equipment_runtime_state",
            "persistence_owner": "world_state.world_trillionnium_characters.inventory_items_and_equipment_slots",
            "runtime_status": "rust_owned_inventory_and_equip_slots_live",
            "content_policy": "trillionnium_native_no_copied_hero_tan_text_assets_or_tables",
            "inventory_count": self.inventory_items.len(),
            "equipped_slot_count": self.equipment_slots.len(),
            "inventory_items": self.inventory_items,
            "equipment_slots": self.equipment_slots,
            "allowed_mutation_commands": ["equip_item", "attack", "complete_task"],
            "web_role": "visualization_input_only",
        })
    }

    pub fn to_projection_json(&self) -> Value {
        json!({
            "contract_version": TRILLIONNIUM_CHARACTER_CONTRACT_VERSION,
            "source_of_truth": "rust_trillionnium_game_state",
            "mechanics_reference_layer": "gmud_rmxp_hero_yxts_llm_reference_only",
            "content_policy": "trillionnium_native_no_copied_hero_tan_text_assets_or_tables",
            "matrix_user_id": self.matrix_user_id,
            "character_id": self.character_id,
            "display_name": self.display_name,
            "title": self.title,
            "sect_id": self.sect_id,
            "attributes": self.attributes.to_value(),
            "skill_ids": self.skill_ids,
            "known_skills": trillionnium_known_skill_definitions_json(&self.skill_ids),
            "item_equipment_runtime_contract_version": TRILLIONNIUM_WORLD_ITEM_EQUIPMENT_RUNTIME_CONTRACT_VERSION,
            "inventory_items": self.inventory_items,
            "equipment_slots": self.equipment_slots,
            "item_equipment_runtime": self.item_equipment_runtime_json(),
            "resource_pressure_runtime_contract_version": TRILLIONNIUM_WORLD_RESOURCE_PRESSURE_RUNTIME_CONTRACT_VERSION,
            "resource_pressure_state": self.resource_pressure_state.to_value(),
            "resource_pressure_runtime": self.resource_pressure_state.to_value(),
            "food_water_age_survival_runtime_contract_version": TRILLIONNIUM_WORLD_FOOD_WATER_AGE_SURVIVAL_CONTRACT_VERSION,
            "survival_pressure_state": self.resource_pressure_state.survival_to_value(),
            "survival_runtime": self.resource_pressure_state.survival_to_value(),
            "region_story_unlock_runtime_contract_version": TRILLIONNIUM_WORLD_REGION_STORY_UNLOCK_RUNTIME_CONTRACT_VERSION,
            "region_story_unlock_state": self.region_story_unlock_state.to_value(),
            "region_story_unlock_runtime": self.region_story_unlock_state.to_value(),
            "combat_numerics_runtime_contract_version": TRILLIONNIUM_WORLD_COMBAT_NUMERICS_RUNTIME_CONTRACT_VERSION,
            "combat_numerics_state": self.combat_numerics_state.to_value(),
            "combat_numerics_runtime": self.combat_numerics_state.to_value(),
            "skill_definition_contract": TRILLIONNIUM_SKILL_CONTRACT_VERSION,
            "skill_families": [
                "basic_inner_power", "basic_unarmed", "basic_blade", "basic_sword", "basic_lightness",
                "reading_and_contracts", "merchant_routecraft", "artifact_crafting", "streetwise_investigation",
                "staff_and_polearm", "evidence_packaging", "healing_tonic_craft", "route_scouting",
                "dispute_mediation", "raid_coordination", "artifact_appraisal", "terrain_reading",
                "auction_sense", "camp_cooking", "shadow_messaging"
            ],
            "next_development_hooks": [
                "sect_hall_osm_overlay_binding",
                "mentor_training_command",
                "npc_relationship_model",
                "wuxia_combat_log_generator"
            ],
            "updated_at_epoch": self.updated_at_epoch,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumStoryArcCatalogItem {
    pub arc_id: String,
    pub theme: String,
    pub entry_task_archetypes: Vec<String>,
    pub unlock_signal: String,
    pub unlock_status: String,
    pub runtime_contract_version: String,
    pub runtime_source_of_truth: String,
}

pub fn trillionnium_story_arc_catalog_json(
    region_story_unlock_runtime: &WorldTrillionniumRegionStoryUnlockState,
) -> Value {
    let unlocked_arc_ids = region_story_unlock_runtime
        .unlocked_story_arc_ids
        .iter()
        .cloned()
        .collect::<HashSet<_>>();
    let arc = |arc_id: &str, theme: &str, entry_task_archetypes: Vec<&str>, unlock_signal: &str| {
        json!({
            "arc_id": arc_id,
            "theme": theme,
            "entry_task_archetypes": entry_task_archetypes,
            "unlock_signal": unlock_signal,
            "unlock_status": if unlocked_arc_ids.contains(arc_id) { "unlocked" } else { "locked" },
            "runtime_contract_version": TRILLIONNIUM_WORLD_REGION_STORY_UNLOCK_RUNTIME_CONTRACT_VERSION,
            "runtime_source_of_truth": "rust_trillionnium_region_story_unlock_runtime_state",
        })
    };
    json!({
        "contract_version": "trillionnium_native_story_arc_catalog_v1",
        "source_of_truth": "rust_trillionnium_story_arc_catalog",
        "content_policy": "trillionnium_native_no_copied_hero_tan_text_assets_or_tables",
        "runtime_status": "rust_runtime_backed_by_region_story_unlock_state",
        "runtime_contract_version": TRILLIONNIUM_WORLD_REGION_STORY_UNLOCK_RUNTIME_CONTRACT_VERSION,
        "runtime_source_of_truth": "rust_trillionnium_region_story_unlock_runtime_state",
        "unlocked_story_arc_count": unlocked_arc_ids.len(),
        "arcs": [
            arc("mirror_city_arrival", "first_route_first_mentor_first_reward", vec!["courier_letter", "sect_training_trial"], "first_human_session_complete"),
            arc("ledger_debt_storm", "contracts_debt_recovery_and_review_hold", vec!["debt_recovery", "market_settlement"], "ledger_settlement_dispute_seen"),
            arc("jade_route_patrol", "escort_patrol_and_map_survey", vec!["street_patrol", "map_survey", "escort_route"], "route_scouting_known"),
            arc("night_watch_dispute", "npc_relationship_witness_and_mediation", vec!["arbitrate_dispute", "find_item"], "streetwise_investigation_or_mediation_known"),
            arc("raid_signal_return", "combat_entry_party_raid_and_return_to_map", vec!["raid_signal", "defeat_bandit"], "world_combat_encounter_return_loop_green"),
            arc("field_remedy_supply", "medicine_supplies_recovery_and_failed_task_repair", vec!["healing_supply", "find_item"], "healing_tonic_craft_known"),
        ]
    })
}

pub fn trillionnium_resource_pressure_loops_json() -> Value {
    json!({
        "contract_version": "trillionnium_native_resource_pressure_loop_v1",
        "source_of_truth": "rust_trillionnium_resource_pressure_catalog",
        "content_policy": "trillionnium_native_no_copied_hero_tan_text_assets_or_tables",
        "runtime_status": "rust_runtime_backed_time_stamina_injury_evidence_food_water_age",
        "survival_runtime_contract_version": TRILLIONNIUM_WORLD_FOOD_WATER_AGE_SURVIVAL_CONTRACT_VERSION,
        "loops": [
            {"loop_id": "daylight_route_window", "domain": "time", "pressure": "daylight_windows_change_patrol_and_delivery_risk", "player_choice": "depart_now_or_wait_for_lower_risk", "failure_mode": "late_report_review_hold"},
            {"loop_id": "stamina_travel_budget", "domain": "stamina", "pressure": "long_routes_reduce_combat_entry_focus", "player_choice": "rest_train_or_push_route", "failure_mode": "low_focus_encounter_penalty"},
            {"loop_id": "evidence_integrity", "domain": "proof", "pressure": "proof_bundle_can_be_incomplete_or_interrupted", "player_choice": "collect_more_evidence_or_submit_fast", "failure_mode": "review_hold_or_reward_delay"},
            {"loop_id": "injury_recovery", "domain": "health", "pressure": "failed_encounters_increase_downtime", "player_choice": "use_tonic_seek_mentor_or_continue", "failure_mode": "party_downtime_and_task_risk"},
            {"loop_id": "food_supply", "domain": "food", "pressure": "travel_and_combat_consume_food_until_stamina_caps_apply", "player_choice": "restock_rations_finish_route_or_risk_exhaustion", "failure_mode": "starvation_risk_caps_stamina_and_adds_injury"},
            {"loop_id": "water_supply", "domain": "water", "pressure": "movement_and_combat_consume_water_faster_than_food", "player_choice": "refill_water_take_shortcut_or_delay_combat", "failure_mode": "dehydration_risk_caps_stamina_and_adds_injury"},
            {"loop_id": "age_pressure", "domain": "age", "pressure": "world_day_rollovers_increment_age_and_long_term_stage", "player_choice": "spend_days_training_building_trust_or_pushing_routes", "failure_mode": "age_stage_changes_long_term_identity_and_recovery_pressure"},
            {"loop_id": "reputation_trust", "domain": "relationship", "pressure": "npc_trust_changes_task_access_and_dispute_outcomes", "player_choice": "mediate_dispute_pay_debt_or_train", "failure_mode": "locked_mentor_or_worse_reward_gate"},
            {"loop_id": "ledger_settlement_risk", "domain": "economy", "pressure": "rewards_are_held_until_settlement_review_passes", "player_choice": "improve_deliverable_or_accept_delay", "failure_mode": "reward_not_released"},
        ]
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumAuthoredQuestChainFixture {
    pub chain_id: String,
    pub title: String,
    pub theme: String,
    pub node_ids: Vec<String>,
    pub task_archetype_ids: Vec<String>,
    pub relationship_consequence: String,
    pub survival_pressure: String,
    pub encounter_hook: String,
    pub reward_gate: String,
}

#[allow(clippy::too_many_arguments)]
fn authored_chain(
    chain_id: &str,
    title: &str,
    theme: &str,
    node_ids: &[&str],
    task_archetype_ids: &[&str],
    relationship_consequence: &str,
    survival_pressure: &str,
    encounter_hook: &str,
    reward_gate: &str,
) -> TrillionniumAuthoredQuestChainFixture {
    TrillionniumAuthoredQuestChainFixture {
        chain_id: chain_id.to_string(),
        title: title.to_string(),
        theme: theme.to_string(),
        node_ids: node_ids.iter().map(|value| (*value).to_string()).collect(),
        task_archetype_ids: task_archetype_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        relationship_consequence: relationship_consequence.to_string(),
        survival_pressure: survival_pressure.to_string(),
        encounter_hook: encounter_hook.to_string(),
        reward_gate: reward_gate.to_string(),
    }
}

pub fn trillionnium_authored_quest_chain_fixtures() -> Vec<TrillionniumAuthoredQuestChainFixture> {
    vec![
        authored_chain(
            "witness_archive_reconciliation",
            "Witness Archive Reconciliation / 见证档案调停",
            "restore_trust_by_rebuilding_a_clean_evidence_chain",
            &["witness-archive", "elder-step", "dispute-desk"],
            &["witness_archive_case", "arbitrate_dispute"],
            "conflict_heat_can_shift_into_trust_recovery",
            "low",
            "social_dispute_without_combat_required",
            "relationship_evidence_and_review_hold_gate",
        ),
        authored_chain(
            "cistern_ration_relief",
            "Cistern Ration Relief / 水仓行粮救援",
            "food_water_and_recovery_pressure_across_a_short_supply_route",
            &[
                "river-cistern",
                "ration-kitchen",
                "field-infirmary",
                "caravan-rest-camp",
            ],
            &["cistern_ration_run", "field_infirmary_round"],
            "field_remedy_garden_trust_rises_when_supplies_arrive",
            "food_water_decay_visible",
            "failed_combat_recovery_can_redirect_here",
            "survival_supply_quality_review_required",
        ),
        authored_chain(
            "night_courier_watch",
            "Night Courier Watch / 夜巡信使接力",
            "patrol_message_handoff_and_terrain_scouting",
            &[
                "night-watch-yard",
                "courier-yard",
                "survey-tower",
                "client-board",
            ],
            &[
                "night_watch_message",
                "survey_tower_chart",
                "courier_letter",
            ],
            "night_watch_alliance_trust_grows_with_clean_handoffs",
            "water_and_fatigue_risk_on_longer_routes",
            "optional_street_patrol_interruption",
            "route_evidence_required_before_reward",
        ),
        authored_chain(
            "guild_vault_signal",
            "Guild Vault Signal / 公会库房号令",
            "raid_evidence_inventory_and_return_to_arena",
            &["raid-hall", "guild-vault", "league-coliseum"],
            &["guild_vault_audit", "raid_signal", "defeat_bandit"],
            "raid_signal_lodge_standing_depends_on_evidence_custody",
            "combat_energy_and_guard_pressure",
            "lightweight_combat_entry_then_map_return",
            "guild_evidence_review_hold_required",
        ),
        authored_chain(
            "auction_arcade_appraisal",
            "Auction Arcade Appraisal / 拍卖廊鉴定",
            "appraise_repair_and_price_original_market_goods",
            &[
                "auction-arcade",
                "guild-vault",
                "forge-workbench",
                "zbj-market-gate",
            ],
            &["auction_appraisal", "find_item", "market_settlement"],
            "market_wind_pavilion_trust_tracks_fair_appraisal",
            "medium_supply_cost_before_market_reward",
            "no_reference_items_or_names_copied",
            "appraisal_evidence_and_settlement_required",
        ),
        authored_chain(
            "mentor_cloister_title_oath",
            "Mentor Cloister Title Oath / 导师回廊誓约",
            "long_term_title_ladder_and_social_repair_route",
            &["mentor-cloister", "elder-step", "mirror-city-square"],
            &["mentor_cloister_oath", "sect_training_trial"],
            "mentor_trust_unlocks_clean_room_title_ladder_progress",
            "age_days_make_training_time_a_visible_choice",
            "mentor_trial_not_external_sect_table",
            "mentor_place_cost_cooldown_required",
        ),
    ]
}

impl TrillionniumAuthoredQuestChainFixture {
    pub fn to_value(&self, world: &WorldState, task_candidate_ids: &[String]) -> Value {
        let route_nodes = self
            .node_ids
            .iter()
            .filter_map(|node_id| world.node(node_id))
            .map(|node| {
                json!({
                    "node_id": node.id,
                    "name": node.name,
                    "zone_id": node.region,
                    "location_id": node.location_id,
                    "node_kind": node.node_kind,
                    "interaction_tags": node.tags,
                    "osm_game_overlay_id": format!("trillionnium-world-node:{}", node.id),
                })
            })
            .collect::<Vec<_>>();
        let missing_node_ids = self
            .node_ids
            .iter()
            .filter(|node_id| world.node(node_id).is_none())
            .cloned()
            .collect::<Vec<_>>();
        let available_task_candidate_count = task_candidate_ids
            .iter()
            .filter(|task_id| self.task_archetype_ids.iter().any(|id| id == *task_id))
            .count();
        json!({
            "contract_version": TRILLIONNIUM_WORLD_AUTHORED_QUEST_CHAIN_CONTRACT_VERSION,
            "chain_id": self.chain_id,
            "title": self.title,
            "theme": self.theme,
            "node_ids": self.node_ids,
            "route_nodes": route_nodes,
            "task_archetype_ids": self.task_archetype_ids,
            "available_task_candidate_count": available_task_candidate_count,
            "missing_node_ids": missing_node_ids,
            "relationship_consequence": self.relationship_consequence,
            "survival_pressure": self.survival_pressure,
            "encounter_hook": self.encounter_hook,
            "reward_gate": self.reward_gate,
            "graph_owner": "world_state.world_map_nodes.exits",
            "task_candidate_owner": "rust_trillionnium_task_archetype_fixtures",
            "source_of_truth": "rust_trillionnium_authored_quest_chain_catalog",
            "content_policy": "trillionnium_native_no_copied_hero_tan_text_assets_or_tables",
            "copy_policy": "no_copied_hero_tan_text_assets_code_tables_or_data",
            "web_role": "visualization_input_only",
        })
    }
}

pub fn trillionnium_authored_quest_chain_catalog_json(world: &WorldState) -> Value {
    let task_candidate_ids = trillionnium_task_archetype_fixtures()
        .into_iter()
        .map(|task| task.task_archetype_id)
        .collect::<Vec<_>>();
    let chains = trillionnium_authored_quest_chain_fixtures()
        .into_iter()
        .map(|chain| chain.to_value(world, &task_candidate_ids))
        .collect::<Vec<_>>();
    let mut covered_node_ids = chains
        .iter()
        .flat_map(|chain| {
            chain
                .get("node_ids")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter_map(|value| value.as_str().map(ToString::to_string))
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    covered_node_ids.sort();
    let total_step_count = chains
        .iter()
        .map(|chain| {
            chain
                .get("node_ids")
                .and_then(Value::as_array)
                .map(Vec::len)
                .unwrap_or(0)
        })
        .sum::<usize>();
    json!({
        "contract_version": TRILLIONNIUM_WORLD_AUTHORED_QUEST_CHAIN_CONTRACT_VERSION,
        "source_of_truth": "rust_trillionnium_authored_quest_chain_catalog",
        "content_policy": "trillionnium_native_no_copied_hero_tan_text_assets_or_tables",
        "copy_policy": "no_copied_hero_tan_text_assets_code_tables_or_data",
        "architecture_rule": "build_reference_shape_equivalent_with_original_trillionnium_content_first",
        "forbidden_intermediate": "no_full_hero_tan_replica_then_replace_workflow",
        "graph_owner": "world_state.world_map_nodes.exits",
        "task_candidate_owner": "rust_trillionnium_task_archetype_fixtures",
        "relationship_owner": "world_state.world_relationships",
        "survival_owner": "world_state.world_trillionnium_characters.resource_pressure_state.food_water_age",
        "chain_count": chains.len(),
        "total_step_count": total_step_count,
        "covered_node_count": covered_node_ids.len(),
        "covered_node_ids": covered_node_ids,
        "chains": chains,
        "web_role": "visualization_input_only",
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTacticsGameSession {
    pub session_id: String,
    pub matrix_user_id: String,
    pub room_id: Option<String>,
    pub active_node_id: String,
    pub active_overlay_id: String,
    pub objective_id: String,
    pub objective_progress: i64,
    pub objective_goal: i64,
    pub victory_state: String,
    pub reward_status: String,
    pub reward_event_id: Option<String>,
    pub reward_credits_awarded: i64,
    pub reward_xp_awarded: i64,
    pub created_at_epoch: i64,
    pub updated_at_epoch: i64,
}

impl WorldTacticsGameSession {
    pub fn fixture_for(matrix_user_id: &str, current_node_id: &str) -> Self {
        Self {
            session_id: trillionnium_hash_id(
                "world-tactics-session",
                &format!("{matrix_user_id}:{current_node_id}:fixture"),
            ),
            matrix_user_id: matrix_user_id.to_string(),
            room_id: None,
            active_node_id: current_node_id.to_string(),
            active_overlay_id: format!("trillionnium-world-node:{current_node_id}"),
            objective_id: "first_objective".to_string(),
            objective_progress: 0,
            objective_goal: 1,
            victory_state: "in_progress".to_string(),
            reward_status: "not_released".to_string(),
            reward_event_id: None,
            reward_credits_awarded: 0,
            reward_xp_awarded: 0,
            created_at_epoch: 0,
            updated_at_epoch: 0,
        }
    }

    pub fn route_task_id(&self) -> String {
        format!(
            "tactics-objective:{}:{}",
            self.matrix_user_id, self.objective_id
        )
    }

    pub fn to_projection_json(&self) -> Value {
        json!({
            "contract_version": TRILLIONNIUM_TACTICS_GAME_SESSION_CONTRACT_VERSION,
            "session_id": self.session_id,
            "matrix_user_id": self.matrix_user_id,
            "room_id": self.room_id,
            "active_node_id": self.active_node_id,
            "active_overlay_id": self.active_overlay_id,
            "objective_id": self.objective_id,
            "objective_progress": self.objective_progress,
            "objective_goal": self.objective_goal,
            "victory_state": self.victory_state,
            "reward_status": self.reward_status,
            "reward_event_id": self.reward_event_id,
            "reward_credits_awarded": self.reward_credits_awarded,
            "reward_xp_awarded": self.reward_xp_awarded,
            "route_task_id": self.route_task_id(),
            "source_of_truth": "rust_world_tactics_sessions",
            "persistence_owner": "world_state.world_tactics_sessions",
            "web_role": "visualization_input_only",
            "created_at_epoch": self.created_at_epoch,
            "updated_at_epoch": self.updated_at_epoch,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTacticsSimulationTick {
    pub tick_id: String,
    pub session_id: String,
    pub matrix_user_id: String,
    pub command: String,
    pub outcome_accepted: bool,
    pub outcome_result: String,
    pub action_cost: i64,
    pub effect_summary: String,
    pub created_at_epoch: i64,
}

impl WorldTacticsSimulationTick {
    pub fn fixture_attack_block(session: &WorldTacticsGameSession) -> Self {
        Self {
            tick_id: trillionnium_hash_id(
                "world-tactics-tick",
                &format!("{}:attack:0", session.session_id),
            ),
            session_id: session.session_id.clone(),
            matrix_user_id: session.matrix_user_id.clone(),
            command: "attack".to_string(),
            outcome_accepted: false,
            outcome_result: "repeat_farming_blocked".to_string(),
            action_cost: 1,
            effect_summary: "repeat farming guard blocks already-settled reward route".to_string(),
            created_at_epoch: 0,
        }
    }

    pub fn to_projection_json(&self) -> Value {
        json!({
            "contract_version": TRILLIONNIUM_TACTICS_SIMULATION_TICK_CONTRACT_VERSION,
            "tick_id": self.tick_id,
            "session_id": self.session_id,
            "matrix_user_id": self.matrix_user_id,
            "command": self.command,
            "outcome_accepted": self.outcome_accepted,
            "outcome_result": self.outcome_result,
            "action_cost": self.action_cost,
            "effect_summary": self.effect_summary,
            "source_of_truth": "rust_tactics_simulation_tick",
            "web_role": "visualization_input_only",
            "created_at_epoch": self.created_at_epoch,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTacticsRewardSettlement {
    pub settlement_id: String,
    pub session_id: String,
    pub route_task_id: String,
    pub ledger_receipt_id: Option<String>,
    pub reward_status: String,
    pub credits_delta: i64,
    pub xp_delta: i64,
    pub source_of_truth: String,
}

impl WorldTacticsRewardSettlement {
    pub fn fixture_pending(session: &WorldTacticsGameSession) -> Self {
        Self {
            settlement_id: trillionnium_hash_id(
                "world-tactics-reward-settlement",
                &session.session_id,
            ),
            session_id: session.session_id.clone(),
            route_task_id: session.route_task_id(),
            ledger_receipt_id: None,
            reward_status: session.reward_status.clone(),
            credits_delta: session.reward_credits_awarded,
            xp_delta: session.reward_xp_awarded,
            source_of_truth: "rust_tactics_reward_settlement_adapter".to_string(),
        }
    }

    pub fn to_projection_json(&self) -> Value {
        json!({
            "contract_version": TRILLIONNIUM_TACTICS_REWARD_SETTLEMENT_CONTRACT_VERSION,
            "settlement_id": self.settlement_id,
            "session_id": self.session_id,
            "route_task_id": self.route_task_id,
            "ledger_receipt_id": self.ledger_receipt_id,
            "reward_status": self.reward_status,
            "credits_delta": self.credits_delta,
            "xp_delta": self.xp_delta,
            "ledger_release_gate_status": if self.ledger_receipt_id.is_some() { "server_settled" } else { "not_released" },
            "source_of_truth": self.source_of_truth,
            "web_role": "visualization_input_only",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_world_is_rust_owned() {
        let world = WorldState::fixture();
        world.validate_authority().unwrap();
        assert!(world.node("mirror-city-square").is_some());
    }

    #[test]
    fn fixture_world_serializes_contract() {
        let json = serde_json::to_string(&WorldState::fixture()).unwrap();
        assert!(json.contains(WORLD_DOMAIN_CONTRACT));
        assert!(json.contains(WORLD_RUST_SOURCE_OF_TRUTH));
    }

    #[test]
    fn cex_default_map_fixture_preserves_incubator_topology() {
        let world = WorldState::cex_default_map_fixture();
        world.validate_authority().unwrap();
        assert_eq!(world.nodes.len(), 24);
        assert!(world.node("mirror-city-square").is_some());
        assert!(world.node("caravan-rest-camp").is_some());
        assert!(world.edges.iter().any(|edge| {
            edge.from == "mirror-city-square"
                && edge.direction == "north"
                && edge.to == "league-coliseum"
        }));
        assert!(world.edges.iter().any(|edge| {
            edge.from == "mirror-city-square"
                && edge.direction == "east"
                && edge.to == "starter-studio"
        }));
        assert!(world
            .node("mentor-cloister")
            .unwrap()
            .tags
            .iter()
            .any(|tag| tag == "title_ladder"));
    }

    #[test]
    fn builds_extracted_world_indexes_without_cex_runtime() {
        let indexes = build_world_indexes(&WorldState::fixture());
        assert_eq!(indexes.sorted_node_ids_by_id.len(), 2);
        assert_eq!(
            indexes
                .node_ids_by_location
                .get("mirror-city")
                .expect("mirror city location"),
            &vec![
                "league-coliseum".to_string(),
                "mirror-city-square".to_string()
            ]
        );
        assert_eq!(indexes.sorted_position_actor_ids, vec!["local-player"]);
    }

    #[test]
    fn extracted_index_helpers_match_cex_tail_and_sort_contract() {
        assert_eq!(recent_tail_indices(5, 3), vec![4, 3, 2]);
        let values = vec!["b", "a", "c"];
        let sorted = sorted_indices_by(&values, |left, right| left.cmp(right));
        assert_eq!(sorted, vec![1, 0, 2]);
        let sorted_values: Vec<&str> = indexed_sorted(&values, &sorted)
            .into_iter()
            .copied()
            .collect();
        assert_eq!(sorted_values, vec!["a", "b", "c"]);
        let recent: Vec<&str> = indexed_recent(&values, &[2, 1, 0], 2).copied().collect();
        assert_eq!(recent, vec!["c", "a"]);
    }

    #[test]
    fn extracted_tactics_catalogs_preserve_core_cex_content() {
        let skills = trillionnium_fixture_skill_definitions();
        assert_eq!(skills.len(), 20);
        let unarmed = trillionnium_skill_definition_by_id("basic_unarmed").unwrap();
        assert_eq!(
            unarmed.contract_version,
            TRILLIONNIUM_SKILL_CONTRACT_VERSION
        );
        assert_eq!(unarmed.training_anchor_role, "civic_square");
        assert!(unarmed.content_policy.contains("no_copied_hero_tan"));

        let training = trillionnium_training_command_for_skill("basic_unarmed").unwrap();
        assert_eq!(training.mentor_npc_id, "npc-street-compass-sifu");
        assert_eq!(training.required_semantic_role, "civic_square");

        let archetypes = trillionnium_task_archetype_fixtures();
        assert_eq!(archetypes.len(), 20);
        assert_eq!(
            trillionnium_objective_task_for_semantic_role("auction_arcade"),
            Some("auction_appraisal")
        );
        assert_eq!(trillionnium_objective_label_for_role("raid_hall"), "盟");
        assert_eq!(trillionnium_objective_priority_for_role("market"), 100);

        let sects = trillionnium_sect_fixtures();
        assert_eq!(sects.len(), 9);
        assert_eq!(
            trillionnium_sect_by_id("raid-signal-lodge")
                .unwrap()
                .anchor_semantic_role,
            "raid_hall"
        );

        let npcs = trillionnium_npc_fixtures();
        assert_eq!(npcs.len(), 19);
        let scout = trillionnium_npc_by_id("npc-jade-route-scout").unwrap();
        assert!(scout
            .training_skill_ids
            .iter()
            .any(|skill| skill == "route_scouting"));
        assert!(scout
            .task_archetype_ids
            .iter()
            .any(|task| task == "map_survey"));
        assert_eq!(
            trillionnium_npc_relationship_delta("tactics_train_skill", 0),
            4
        );
    }

    #[test]
    fn extracted_tactics_runtime_states_mutate_without_cex_runtime() {
        let mut resources = WorldTrillionniumResourcePressureState::default();
        let mutation = resources.apply_mutation("world_map_move", "move east", None, 123);
        assert_eq!(
            mutation.source_of_truth,
            "rust_trillionnium_resource_pressure_runtime_state"
        );
        assert_eq!(resources.mutation_count, 1);
        assert_eq!(resources.stamina_status(), "route_ready");

        let mut story = WorldTrillionniumRegionStoryUnlockState::default();
        let unlock = story.apply_mutation(
            "tactics_complete_task",
            "complete_task",
            Some("task_completion_validated"),
            Some("delivery-dock"),
            Some("market-bazaar"),
            456,
        );
        assert!(unlock
            .unlocked_story_arc_ids
            .iter()
            .any(|arc| arc == "night_watch_dispute"));
        assert!(story
            .unlocked_story_arc_ids
            .iter()
            .any(|arc| arc == "jade_route_patrol"));
    }

    #[test]
    fn extracted_combat_numerics_apply_attack_like_cex_runtime() {
        let attributes = TrillionniumAttributes::default();
        let mut combat = WorldTrillionniumCombatNumericsState::default();
        combat.ensure_defaults(&attributes);
        assert_eq!(attributes.derived_stats().max_hp, 176);
        let exchange = combat.apply_attack(
            "tactics_attack",
            "attack F5",
            &TrillionniumCombatResolutionInput {
                attacker_unit_id: "lord".to_string(),
                defender_unit_id: "market-bandit".to_string(),
                target_tile: "F5".to_string(),
                skill_id: "basic_unarmed".to_string(),
                damage: 30,
                defender_hp_before: 40,
                defender_hp_after: 10,
                result: "hit_landed".to_string(),
            },
            &attributes,
            789,
        );
        assert!(exchange.critical);
        assert_eq!(exchange.hit_quality, "critical_route_break");
        assert_eq!(combat.mutation_count, 1);
        assert_ne!(combat.health_status(), "routed_recovery_required");
    }
}

pub mod cex_compat {
    //! Compatibility structs for reading CEX-incubator world snapshots without
    //! depending on CEX service crates.

    use super::{
        WorldEdge, WorldNode, WorldPosition, WorldState, WORLD_CEX_INCUBATOR_SOURCE,
        WORLD_DOMAIN_CONTRACT, WORLD_RUST_SOURCE_OF_TRUTH,
    };
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    pub const CEX_WORLD_SNAPSHOT_CONTRACT: &str = "cex_world_state_snapshot_v1";

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CexWorldMapNode {
        pub node_id: String,
        pub location_id: String,
        pub zone_id: String,
        pub name: String,
        pub node_kind: String,
        pub description: String,
        pub x: i64,
        pub y: i64,
        #[serde(default)]
        pub exits: HashMap<String, String>,
        #[serde(default)]
        pub interaction_tags: Vec<String>,
        #[serde(default)]
        pub freedom_hooks: Vec<String>,
        pub status: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CexWorldPlayerPosition {
        pub matrix_user_id: String,
        pub node_id: String,
        pub location_id: String,
        pub updated_at_epoch: i64,
    }

    #[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CexWorldStateSnapshot {
        #[serde(default)]
        pub world_map_nodes: HashMap<String, CexWorldMapNode>,
        #[serde(default)]
        pub world_player_positions: HashMap<String, CexWorldPlayerPosition>,
    }

    impl From<&CexWorldStateSnapshot> for WorldState {
        fn from(snapshot: &CexWorldStateSnapshot) -> Self {
            let mut nodes: Vec<WorldNode> = snapshot
                .world_map_nodes
                .values()
                .map(|node| WorldNode {
                    id: node.node_id.clone(),
                    name: node.name.clone(),
                    region: node.zone_id.clone(),
                    location_id: node.location_id.clone(),
                    node_kind: node.node_kind.clone(),
                    description: node.description.clone(),
                    status: node.status.clone(),
                    lat_e7: node.y as i32,
                    lng_e7: node.x as i32,
                    tags: node
                        .interaction_tags
                        .iter()
                        .chain(node.freedom_hooks.iter())
                        .chain(std::iter::once(&node.node_kind))
                        .cloned()
                        .collect(),
                })
                .collect();
            nodes.sort_by(|a, b| a.id.cmp(&b.id));

            let mut edges: Vec<WorldEdge> = snapshot
                .world_map_nodes
                .values()
                .flat_map(|node| {
                    node.exits.iter().map(move |(direction, target)| WorldEdge {
                        from: node.node_id.clone(),
                        to: target.clone(),
                        direction: direction.clone(),
                    })
                })
                .collect();
            edges.sort_by(|a, b| {
                (&a.from, &a.direction, &a.to).cmp(&(&b.from, &b.direction, &b.to))
            });

            let mut positions: Vec<WorldPosition> = snapshot
                .world_player_positions
                .values()
                .map(|position| WorldPosition {
                    actor_id: position.matrix_user_id.clone(),
                    node_id: position.node_id.clone(),
                    source_of_truth: WORLD_RUST_SOURCE_OF_TRUTH.to_string(),
                })
                .collect();
            positions.sort_by(|a, b| a.actor_id.cmp(&b.actor_id));

            WorldState {
                contract_version: WORLD_DOMAIN_CONTRACT.to_string(),
                source: WORLD_CEX_INCUBATOR_SOURCE.to_string(),
                nodes,
                edges,
                positions,
                npcs: vec![],
                tasks: vec![],
                receipts: vec![],
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn converts_cex_map_nodes_into_standalone_world_state() {
            let snapshot: CexWorldStateSnapshot = serde_json::from_value(serde_json::json!({
                "world_map_nodes": {
                    "mirror-city-square": {
                        "node_id": "mirror-city-square",
                        "location_id": "mirror-city",
                        "zone_id": "fixture-osm",
                        "name": "Mirror City Square",
                        "node_kind": "mentor",
                        "description": "spawn",
                        "x": 1214737010,
                        "y": 312304160,
                        "exits": { "east": "league-coliseum" },
                        "interaction_tags": ["spawn"],
                        "freedom_hooks": ["train"],
                        "status": "active"
                    },
                    "league-coliseum": {
                        "node_id": "league-coliseum",
                        "location_id": "mirror-city",
                        "zone_id": "fixture-osm",
                        "name": "League Coliseum",
                        "node_kind": "combat",
                        "description": "objective",
                        "x": 1214740000,
                        "y": 312310000,
                        "exits": {},
                        "interaction_tags": ["combat"],
                        "freedom_hooks": [],
                        "status": "active"
                    }
                },
                "world_player_positions": {
                    "@player:matrix": {
                        "matrix_user_id": "@player:matrix",
                        "node_id": "mirror-city-square",
                        "location_id": "mirror-city",
                        "updated_at_epoch": 1778660000
                    }
                }
            }))
            .unwrap();
            let world = WorldState::from(&snapshot);
            world.validate_authority().unwrap();
            assert_eq!(world.nodes.len(), 2);
            assert_eq!(world.edges.len(), 1);
            assert_eq!(world.positions[0].actor_id, "@player:matrix");
            assert!(world
                .nodes
                .iter()
                .any(|node| node.tags.iter().any(|tag| tag == "train")));
        }
    }
}
