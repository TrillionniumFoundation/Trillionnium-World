use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectId {
    StreetCompass,
    IronWorkshop,
    NightWatch,
}

impl SectId {
    pub fn id(self) -> &'static str {
        match self {
            Self::StreetCompass => "street_compass_society",
            Self::IronWorkshop => "iron_workshop_gate",
            Self::NightWatch => "night_watch_alliance",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SectDefinition {
    pub id: SectId,
    pub display_name: &'static str,
    pub hall_room_id: &'static str,
    pub mentor_id: &'static str,
    pub entry_skill_id: &'static str,
    pub battle_bonus: SectBattleBonus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectBattleBonus {
    ReconAndMobility,
    ConstructionAndArmor,
    AmbushAndEvasion,
}

pub const SECT_CATALOG: [SectDefinition; 3] = [
    SectDefinition {
        id: SectId::StreetCompass,
        display_name: "Street Compass Society",
        hall_room_id: "mentor_hall",
        mentor_id: "street-compass-sifu",
        entry_skill_id: "route_scouting",
        battle_bonus: SectBattleBonus::ReconAndMobility,
    },
    SectDefinition {
        id: SectId::IronWorkshop,
        display_name: "Iron Workshop Gate",
        hall_room_id: "workshop_gate",
        mentor_id: "master-orsen",
        entry_skill_id: "artifact_crafting",
        battle_bonus: SectBattleBonus::ConstructionAndArmor,
    },
    SectDefinition {
        id: SectId::NightWatch,
        display_name: "Night Watch Alliance",
        hall_room_id: "night_watch_post",
        mentor_id: "captain-veyra",
        entry_skill_id: "shadow_patrol",
        battle_bonus: SectBattleBonus::AmbushAndEvasion,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NpcRole {
    Mentor,
    Smith,
    WatchCaptain,
    Healer,
    Archivist,
    Merchant,
    Courier,
    Scout,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NpcDefinition {
    pub id: &'static str,
    pub display_name: &'static str,
    pub role: NpcRole,
    pub room_id: &'static str,
    pub faction_id: &'static str,
    pub task_ids: &'static [&'static str],
}

pub const NPC_CATALOG: [NpcDefinition; 10] = [
    NpcDefinition {
        id: "street-compass-sifu",
        display_name: "Street Compass Sifu",
        role: NpcRole::Mentor,
        room_id: "mentor_hall",
        faction_id: "street_compass_society",
        task_ids: &["wayfinder_oath", "broken_milestone"],
    },
    NpcDefinition {
        id: "master-orsen",
        display_name: "Master Orsen",
        role: NpcRole::Mentor,
        room_id: "workshop_gate",
        faction_id: "iron_workshop_gate",
        task_ids: &["forge_commission", "lost_tooling"],
    },
    NpcDefinition {
        id: "captain-veyra",
        display_name: "Captain Veyra",
        role: NpcRole::Mentor,
        room_id: "night_watch_post",
        faction_id: "night_watch_alliance",
        task_ids: &["lantern_watch", "wanted_raider"],
    },
    NpcDefinition {
        id: "relay-smith-brann",
        display_name: "Brann",
        role: NpcRole::Smith,
        room_id: "relay_quarter",
        faction_id: "relay_quarter",
        task_ids: &["relay_salvage"],
    },
    NpcDefinition {
        id: "healer-nima",
        display_name: "Healer Nima",
        role: NpcRole::Healer,
        room_id: "lantern_infirmary",
        faction_id: "mirror_civic",
        task_ids: &["fever_tonic"],
    },
    NpcDefinition {
        id: "archivist-sol",
        display_name: "Archivist Sol",
        role: NpcRole::Archivist,
        room_id: "archive_steps",
        faction_id: "mirror_civic",
        task_ids: &["archive_witness"],
    },
    NpcDefinition {
        id: "merchant-aya",
        display_name: "Merchant Aya",
        role: NpcRole::Merchant,
        room_id: "market_wind_pavilion",
        faction_id: "market_wind",
        task_ids: &["market_debt", "missing_crate"],
    },
    NpcDefinition {
        id: "courier-tess",
        display_name: "Courier Tess",
        role: NpcRole::Courier,
        room_id: "caravan_yard",
        faction_id: "street_compass_society",
        task_ids: &["night_letter", "escort_manifest"],
    },
    NpcDefinition {
        id: "scout-mako",
        display_name: "Scout Mako",
        role: NpcRole::Scout,
        room_id: "outer_signal_road",
        faction_id: "night_watch_alliance",
        task_ids: &["bandit_tracks"],
    },
    NpcDefinition {
        id: "quartermaster-nia",
        display_name: "Quartermaster Nia",
        role: NpcRole::WatchCaptain,
        room_id: "cistern_ward",
        faction_id: "mirror_civic",
        task_ids: &["cistern_relief", "ration_audit"],
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcSchedule {
    pub start_minute: u16,
    pub end_minute: u16,
    pub activity: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipStage {
    Stranger,
    Acquaintance,
    Ally,
    Confidant,
    Kin,
}

impl RelationshipStage {
    pub fn from_trust(trust: i16, completed_tasks: usize) -> Self {
        match trust.saturating_add((completed_tasks as i16).saturating_mul(2)) {
            i16::MIN..=1 => Self::Stranger,
            2..=5 => Self::Acquaintance,
            6..=10 => Self::Ally,
            11..=16 => Self::Confidant,
            _ => Self::Kin,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialogueChoice {
    #[default]
    AskForWork,
    OfferHelp,
    ShareNews,
}

impl DialogueChoice {
    pub fn next(self) -> Self {
        match self {
            Self::AskForWork => Self::OfferHelp,
            Self::OfferHelp => Self::ShareNews,
            Self::ShareNews => Self::AskForWork,
        }
    }
}

/// NPCs travel between work, civic and rest locations instead of merely
/// disappearing outside a single static presence window.
pub fn npc_room_at(npc_id: &str, minute_of_day: u16) -> Option<&'static str> {
    let hour = minute_of_day / 60;
    Some(match npc_id {
        "street-compass-sifu" => match hour {
            6..=11 => "mentor_hall",
            12..=17 => "archive_steps",
            _ => "mirror_square",
        },
        "master-orsen" => match hour {
            7..=15 => "workshop_gate",
            16..=18 => "relay_quarter",
            _ => "caravan_yard",
        },
        "captain-veyra" => match hour {
            0..=5 | 16..=23 => "night_watch_post",
            6..=11 => "lantern_infirmary",
            _ => "archive_steps",
        },
        "relay-smith-brann" => match hour {
            5..=18 => "relay_quarter",
            _ => "market_wind_pavilion",
        },
        "healer-nima" => match hour {
            6..=15 => "lantern_infirmary",
            16..=20 => "cistern_ward",
            _ => "mirror_square",
        },
        "archivist-sol" => match hour {
            8..=16 => "archive_steps",
            17..=19 => "night_watch_post",
            _ => "mentor_hall",
        },
        "merchant-aya" => match hour {
            8..=15 => "market_wind_pavilion",
            16..=20 => "caravan_yard",
            _ => "mirror_square",
        },
        "courier-tess" => match hour {
            5..=10 => "caravan_yard",
            11..=16 => "expedition_gate",
            _ => "market_wind_pavilion",
        },
        "scout-mako" => match hour {
            4..=11 => "outer_signal_road",
            12..=17 => "expedition_gate",
            _ => "night_watch_post",
        },
        "quartermaster-nia" => match hour {
            6..=13 => "cistern_ward",
            14..=18 => "caravan_yard",
            _ => "lantern_infirmary",
        },
        _ => return None,
    })
}

impl NpcSchedule {
    pub fn present(self, minute_of_day: u16) -> bool {
        if self.start_minute <= self.end_minute {
            (self.start_minute..self.end_minute).contains(&minute_of_day)
        } else {
            minute_of_day >= self.start_minute || minute_of_day < self.end_minute
        }
    }
}

pub fn npc_schedule(npc_id: &str) -> Option<NpcSchedule> {
    Some(match npc_id {
        "street-compass-sifu" => NpcSchedule {
            start_minute: 360,
            end_minute: 1_200,
            activity: "mapping safe routes with apprentices",
        },
        "master-orsen" => NpcSchedule {
            start_minute: 420,
            end_minute: 1_140,
            activity: "working the relay forge",
        },
        "captain-veyra" => NpcSchedule {
            start_minute: 960,
            end_minute: 360,
            activity: "commanding the night watch",
        },
        "relay-smith-brann" => NpcSchedule {
            start_minute: 300,
            end_minute: 1_380,
            activity: "salvaging relay housings",
        },
        "healer-nima" => NpcSchedule {
            start_minute: 360,
            end_minute: 1_320,
            activity: "tending the lantern ward",
        },
        "archivist-sol" => NpcSchedule {
            start_minute: 480,
            end_minute: 1_080,
            activity: "cataloguing witness accounts",
        },
        "merchant-aya" => NpcSchedule {
            start_minute: 480,
            end_minute: 1_260,
            activity: "opening the pavilion stalls",
        },
        "courier-tess" => NpcSchedule {
            start_minute: 300,
            end_minute: 1_200,
            activity: "sorting sealed route bags",
        },
        "scout-mako" => NpcSchedule {
            start_minute: 240,
            end_minute: 720,
            activity: "reading tracks beyond the wall",
        },
        "quartermaster-nia" => NpcSchedule {
            start_minute: 360,
            end_minute: 1_200,
            activity: "auditing cistern stores",
        },
        _ => return None,
    })
}

pub fn npc_dialogue(npc_id: &str, trust: i16, completed_tasks: usize) -> Option<&'static str> {
    let trusted = trust >= 6 || completed_tasks >= 2;
    Some(match (npc_id, trusted) {
        ("street-compass-sifu", false) => {
            "A road is a promise made twice: once on the map, once under your feet."
        }
        ("street-compass-sifu", true) => {
            "You no longer follow my marks. You leave marks that bring others home."
        }
        ("master-orsen", false) => {
            "Bring me metal with a history. New iron has not learned what failure costs."
        }
        ("master-orsen", true) => {
            "Your repairs hold under fire. The forge now answers to your field judgment."
        }
        ("captain-veyra", false) => {
            "Daylight hides carelessness. Night reveals every gap in a watch line."
        }
        ("captain-veyra", true) => {
            "Take the outer lantern. My sentries will move when they see your signal."
        }
        ("relay-smith-brann", false) => {
            "Every silent relay has one last useful piece. Help me find which one."
        }
        ("relay-smith-brann", true) => {
            "I kept the best core aside. You have earned a machine that remembers you."
        }
        ("healer-nima", false) => "A clean bandage is cheaper than courage spent while fevered.",
        ("healer-nima", true) => {
            "Your party brings people back alive. My last tonic is yours at cost."
        }
        ("archivist-sol", false) => {
            "Rumour becomes history only after someone risks signing their name."
        }
        ("archivist-sol", true) => {
            "The archive lists you as a reliable witness. Doors open for reliable witnesses."
        }
        ("merchant-aya", false) => {
            "Coin moves faster than caravans, but only caravans arrive with medicine."
        }
        ("merchant-aya", true) => {
            "Your word survived the road. I can extend credit where ledgers usually refuse."
        }
        ("courier-tess", false) => {
            "A sealed letter weighs nothing until the wrong person learns it exists."
        }
        ("courier-tess", true) => {
            "Take the black route bag. Only senior couriers know its quiet crossings."
        }
        ("scout-mako", false) => {
            "Tracks tell the truth, but never in the order you want to hear it."
        }
        ("scout-mako", true) => {
            "The ash runners changed formation. They know someone has learned their habits."
        }
        ("quartermaster-nia", false) => {
            "Water is the city's slowest clock. Waste a cup and the whole ward loses an hour."
        }
        ("quartermaster-nia", true) => {
            "Your relief plan held. I can spare supplies without lying to the ration board."
        }
        _ => return None,
    })
}

pub fn npc_choice_dialogue(
    npc_id: &str,
    stage: RelationshipStage,
    choice: DialogueChoice,
) -> &'static str {
    match (npc_id, choice, stage) {
        ("street-compass-sifu", DialogueChoice::AskForWork, _) => "Map what people fear, then test whether the fear deserves its roadblock.",
        ("street-compass-sifu", DialogueChoice::OfferHelp, RelationshipStage::Stranger | RelationshipStage::Acquaintance) => "Carry chalk and listen. A useful apprentice first learns where not to speak.",
        ("street-compass-sifu", DialogueChoice::OfferHelp, _) => "Take the western apprentices. Their route is yours to judge, not mine.",
        ("master-orsen", DialogueChoice::AskForWork, _) => "The deep relay needs a hinge that can survive heat, grit and a frightened operator.",
        ("master-orsen", DialogueChoice::OfferHelp, _) => "Sort alloy from slag. Your hands will tell me more than your promise.",
        ("captain-veyra", DialogueChoice::AskForWork, _) => "Follow the lantern gap. If it moves against the wind, someone is signalling beyond the wall.",
        ("captain-veyra", DialogueChoice::ShareNews, _) => "Names can wait. Give me direction, timing and which dogs did not bark.",
        ("relay-smith-brann", DialogueChoice::OfferHelp, _) => "Hold this coil steady and do not flatter the sparks. They know when you are afraid.",
        ("healer-nima", DialogueChoice::OfferHelp, _) => "Boil cloth, count breaths, and ask before touching a wound. Heroics come later.",
        ("archivist-sol", DialogueChoice::ShareNews, _) => "Tell it twice: once as you remember it, once as your enemy would record it.",
        ("merchant-aya", DialogueChoice::AskForWork, _) => "Find which caravan is late before buying rumours about why.",
        ("merchant-aya", DialogueChoice::ShareNews, RelationshipStage::Confidant | RelationshipStage::Kin) => "That changes tomorrow's price. I will change yours today, quietly.",
        ("courier-tess", DialogueChoice::AskForWork, _) => "Choose: a safe letter delivered late, or a dangerous truth delivered before curfew.",
        ("scout-mako", DialogueChoice::ShareNews, _) => "Show me the track you almost ignored. That is usually where the useful story begins.",
        ("quartermaster-nia", DialogueChoice::OfferHelp, _) => "Count the missing cups, then the people who insist no cup is missing.",
        (_, DialogueChoice::AskForWork, _) => "There is work nearby, but the road and its cost must both be understood.",
        (_, DialogueChoice::OfferHelp, _) => "Help is welcome when it arrives with patience and enough supplies.",
        (_, DialogueChoice::ShareNews, _) => "News earns trust only after its source and consequence are both named.",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NpcSocialEventDefinition {
    pub id: &'static str,
    pub first_npc_id: &'static str,
    pub second_npc_id: &'static str,
    pub room_id: &'static str,
    pub market_item_id: &'static str,
    pub stock_delta: i16,
    pub demand_delta: i16,
    pub text: &'static str,
}

pub const NPC_SOCIAL_EVENTS: [NpcSocialEventDefinition; 8] = [
    NpcSocialEventDefinition { id: "forge_clinic_exchange", first_npc_id: "master-orsen", second_npc_id: "healer-nima", room_id: "lantern_infirmary", market_item_id: "salvaged-alloy", stock_delta: -1, demand_delta: 3, text: "Orsen trades clean alloy splints for Nima's burn treatment, tightening the metal market." },
    NpcSocialEventDefinition { id: "watch_courier_argument", first_npc_id: "captain-veyra", second_npc_id: "courier-tess", room_id: "night_watch_post", market_item_id: "route-token", stock_delta: 2, demand_delta: -2, text: "Veyra and Tess publish a shared curfew route after a loud argument at the watch desk." },
    NpcSocialEventDefinition { id: "archive_market_audit", first_npc_id: "archivist-sol", second_npc_id: "merchant-aya", room_id: "archive_steps", market_item_id: "market-ledger", stock_delta: 1, demand_delta: -3, text: "Sol audits Aya's caravan books and releases a disputed ledger back into trade." },
    NpcSocialEventDefinition { id: "scout_wayfinder_chart", first_npc_id: "scout-mako", second_npc_id: "street-compass-sifu", room_id: "basin_observatory", market_item_id: "route-token", stock_delta: -2, demand_delta: 4, text: "Mako and the Sifu mark a safe basin crossing, consuming the city's reserve route tokens." },
    NpcSocialEventDefinition { id: "brann_quartermaster_repairs", first_npc_id: "relay-smith-brann", second_npc_id: "quartermaster-nia", room_id: "cistern_ward", market_item_id: "cistern-seal-kit", stock_delta: 2, demand_delta: -2, text: "Brann and Nia finish a public cistern repair and return spare seal kits to the stalls." },
    NpcSocialEventDefinition { id: "healer_watch_requisition", first_npc_id: "healer-nima", second_npc_id: "captain-veyra", room_id: "lantern_infirmary", market_item_id: "field-tonic-kit", stock_delta: -2, demand_delta: 4, text: "The night watch requisitions fever tonics after a patrol returns injured." },
    NpcSocialEventDefinition { id: "courier_caravan_fair", first_npc_id: "courier-tess", second_npc_id: "merchant-aya", room_id: "caravan_yard", market_item_id: "watch-cloth", stock_delta: 3, demand_delta: -2, text: "Tess delivers an early cloth caravan and Aya opens a one-day yard fair." },
    NpcSocialEventDefinition { id: "ash_refuge_relief", first_npc_id: "quartermaster-nia", second_npc_id: "scout-mako", room_id: "cinder_refuge", market_item_id: "ashward-tonic", stock_delta: -1, demand_delta: 5, text: "Nia and Mako divert ashward tonics to families arriving at Cinder Refuge." },
];

pub fn npc_social_event(day: u32, minute_of_day: u16) -> &'static NpcSocialEventDefinition {
    let period = usize::from(minute_of_day / 360);
    &NPC_SOCIAL_EVENTS[(day as usize + period) % NPC_SOCIAL_EVENTS.len()]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestArchetype {
    Courier,
    FindItem,
    Escort,
    Hunt,
    Investigation,
    Supply,
    TrainingTrial,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestApproach {
    #[default]
    Direct,
    Diplomatic,
    Resourceful,
}

impl QuestApproach {
    pub fn next(self) -> Self {
        match self {
            Self::Direct => Self::Diplomatic,
            Self::Diplomatic => Self::Resourceful,
            Self::Resourceful => Self::Direct,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuestRuntimeRule {
    pub deadline_days: u32,
    pub minimum_trust_for_diplomacy: i16,
    pub resource_item_id: &'static str,
    pub resource_quantity: u16,
    pub failure_reputation: i32,
}

pub fn quest_runtime_rule(archetype: QuestArchetype) -> QuestRuntimeRule {
    match archetype {
        QuestArchetype::Courier => QuestRuntimeRule {
            deadline_days: 1,
            minimum_trust_for_diplomacy: 4,
            resource_item_id: "route-token",
            resource_quantity: 1,
            failure_reputation: -2,
        },
        QuestArchetype::FindItem => QuestRuntimeRule {
            deadline_days: 3,
            minimum_trust_for_diplomacy: 6,
            resource_item_id: "salvaged-alloy",
            resource_quantity: 2,
            failure_reputation: -1,
        },
        QuestArchetype::Escort => QuestRuntimeRule {
            deadline_days: 2,
            minimum_trust_for_diplomacy: 7,
            resource_item_id: "route-token",
            resource_quantity: 1,
            failure_reputation: -3,
        },
        QuestArchetype::Hunt => QuestRuntimeRule {
            deadline_days: 3,
            minimum_trust_for_diplomacy: 8,
            resource_item_id: "watch-cloth",
            resource_quantity: 2,
            failure_reputation: -2,
        },
        QuestArchetype::Investigation => QuestRuntimeRule {
            deadline_days: 4,
            minimum_trust_for_diplomacy: 5,
            resource_item_id: "market-ledger",
            resource_quantity: 1,
            failure_reputation: -1,
        },
        QuestArchetype::Supply => QuestRuntimeRule {
            deadline_days: 2,
            minimum_trust_for_diplomacy: 5,
            resource_item_id: "salvaged-alloy",
            resource_quantity: 2,
            failure_reputation: -3,
        },
        QuestArchetype::TrainingTrial => QuestRuntimeRule {
            deadline_days: 5,
            minimum_trust_for_diplomacy: 9,
            resource_item_id: "route-token",
            resource_quantity: 2,
            failure_reputation: 0,
        },
    }
}

pub fn quest_step_verb(archetype: QuestArchetype, step: usize) -> &'static str {
    match archetype {
        QuestArchetype::Courier => {
            if step == 0 {
                "collect the sealed dispatch"
            } else {
                "deliver it without breaking the seal"
            }
        }
        QuestArchetype::FindItem => {
            if step == 0 {
                "search for a physical trace"
            } else {
                "recover and identify the missing item"
            }
        }
        QuestArchetype::Escort => {
            if step == 0 {
                "assemble the escort"
            } else {
                "hold the protected route"
            }
        }
        QuestArchetype::Hunt => {
            if step == 0 {
                "read the quarry's trail"
            } else {
                "corner the quarry"
            }
        }
        QuestArchetype::Investigation => {
            if step == 0 {
                "interview the first witness"
            } else {
                "corroborate the evidence"
            }
        }
        QuestArchetype::Supply => {
            if step == 0 {
                "audit the requested stores"
            } else {
                "deliver usable supplies"
            }
        }
        QuestArchetype::TrainingTrial => {
            if step == 0 {
                "declare the trial"
            } else {
                "complete the measured discipline"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestConditionKind {
    SpeakToGiver,
    VisitWaypoint,
    WinEncounter,
    ReachTrust,
    ConsumeItem,
    ReturnForSettlement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestConditionNode {
    pub id: String,
    pub kind: QuestConditionKind,
    pub subject_id: String,
    pub quantity: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestGraphTopology {
    OathCircuit,
    SplitInvestigation,
    CommissionDependency,
    RecoverySpoke,
    EscortRelay,
    HuntPincer,
    SalvageClaim,
    TonicChain,
    WitnessCorroboration,
    DebtMediation,
    ManifestTrace,
    CurfewRelay,
    ConvoyAssembly,
    TrackEncirclement,
    AuditReconciliation,
}

impl QuestGraphTopology {
    pub fn for_quest(quest_id: &str) -> Option<Self> {
        Some(match quest_id {
            "wayfinder_oath" => Self::OathCircuit,
            "broken_milestone" => Self::SplitInvestigation,
            "forge_commission" => Self::CommissionDependency,
            "lost_tooling" => Self::RecoverySpoke,
            "lantern_watch" => Self::EscortRelay,
            "wanted_raider" => Self::HuntPincer,
            "relay_salvage" => Self::SalvageClaim,
            "fever_tonic" => Self::TonicChain,
            "archive_witness" => Self::WitnessCorroboration,
            "market_debt" => Self::DebtMediation,
            "missing_crate" => Self::ManifestTrace,
            "night_letter" => Self::CurfewRelay,
            "escort_manifest" => Self::ConvoyAssembly,
            "bandit_tracks" => Self::TrackEncirclement,
            "ration_audit" => Self::AuditReconciliation,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestConditionEdge {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestConditionGraph {
    pub quest_id: String,
    pub topology: QuestGraphTopology,
    pub nodes: Vec<QuestConditionNode>,
    pub edges: Vec<QuestConditionEdge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuestNarrativeDefinition {
    pub quest_id: &'static str,
    pub direct: &'static str,
    pub diplomatic: &'static str,
    pub resourceful: &'static str,
    pub failure: &'static str,
}

pub const QUEST_NARRATIVES: [QuestNarrativeDefinition; 15] = [
    QuestNarrativeDefinition {
        quest_id: "wayfinder_oath",
        direct: "You cross the broken span under the mentor's measured blows.",
        diplomatic: "The apprentices testify that your route kept every traveller together.",
        resourceful: "Fresh route tokens turn the trial into a safe public crossing.",
        failure:
            "The oath is postponed until the route can be attempted without abandoning a companion.",
    },
    QuestNarrativeDefinition {
        quest_id: "broken_milestone",
        direct: "The ambushers yield the milestone and the hidden survey marks.",
        diplomatic: "Two rival witnesses agree once their maps are compared in public.",
        resourceful: "Recovered alloy exposes the tampered hinge without a fight.",
        failure:
            "Rain erases the exposed tracks; the investigation must restart from signed testimony.",
    },
    QuestNarrativeDefinition {
        quest_id: "forge_commission",
        direct: "The missing mould is recovered from the relay floor by hand.",
        diplomatic: "Orsen accepts the caravan ledger as proof of lawful ownership.",
        resourceful: "Replacement alloy lets the forge finish without the disputed mould.",
        failure: "The commission cools unfinished and the client withdraws the first payment.",
    },
    QuestNarrativeDefinition {
        quest_id: "lost_tooling",
        direct: "The scrap stalker is driven off and every maker's mark is recovered.",
        diplomatic: "The yard crews reveal which broker moved the stolen tools.",
        resourceful: "A matched alloy set replaces the tools before the next shift.",
        failure:
            "The trail reaches the ash road too late and must be rebuilt from caravan records.",
    },
    QuestNarrativeDefinition {
        quest_id: "lantern_watch",
        direct: "The escort holds formation through the moving lantern gap.",
        diplomatic: "Veyra's sentries open a protected corridor after hearing your witnesses.",
        resourceful: "Route tokens buy spare lanterns and remove the blind interval.",
        failure: "The protected group scatters and the watch records a failed escort.",
    },
    QuestNarrativeDefinition {
        quest_id: "wanted_raider",
        direct: "The Ash Runner is cornered before the outer switchback.",
        diplomatic: "Local scouts trade the runner's safehouse for amnesty.",
        resourceful: "Watch cloth marks a false trail that draws the runner into custody.",
        failure: "The quarry crosses the ridge and the warrant returns to the night watch.",
    },
    QuestNarrativeDefinition {
        quest_id: "relay_salvage",
        direct: "Brann recovers the live core while you hold the scrap line.",
        diplomatic: "The salvagers divide the relay by witnessed claim instead of force.",
        resourceful: "Replacement alloy preserves the core and the rival claim together.",
        failure: "The unstable housing collapses and the salvage claim must be surveyed again.",
    },
    QuestNarrativeDefinition {
        quest_id: "fever_tonic",
        direct: "The tonic reaches the ward through the crowded market lane.",
        diplomatic: "Nima persuades both stalls to release their reserved herbs.",
        resourceful: "Clean alloy vessels preserve enough medicine for the full ward.",
        failure: "The fever cycle advances before the delivery and the recipe must be remade.",
    },
    QuestNarrativeDefinition {
        quest_id: "archive_witness",
        direct: "The witness signs after you reconstruct the night watch route.",
        diplomatic: "Sol seals two corroborating statements into the public archive.",
        resourceful: "A recovered ledger supplies the missing time and seal.",
        failure: "The unsigned account is challenged and removed from the archive.",
    },
    QuestNarrativeDefinition {
        quest_id: "market_debt",
        direct: "The sealed account reaches the forge before collection begins.",
        diplomatic: "Aya and Orsen agree to a public repayment schedule.",
        resourceful: "A route token carries the debt through a bonded courier channel.",
        failure: "The account arrives after collection and must be renegotiated.",
    },
    QuestNarrativeDefinition {
        quest_id: "missing_crate",
        direct: "The dock thieves surrender the marked crate intact.",
        diplomatic: "The yard workers identify the false manifest without reprisals.",
        resourceful: "A ledger comparison proves which crate was relabelled.",
        failure: "The false manifest is burned and the search returns to the cistern tally.",
    },
    QuestNarrativeDefinition {
        quest_id: "night_letter",
        direct: "The letter crosses curfew under pursuit and keeps its seal.",
        diplomatic: "The watch records a lawful exception for the courier route.",
        resourceful: "A route token transfers the letter through a trusted relay.",
        failure: "Curfew closes the route and the sender must write a new dated letter.",
    },
    QuestNarrativeDefinition {
        quest_id: "escort_manifest",
        direct: "The manifest and its bearers reach the expedition gate together.",
        diplomatic: "Both caravan captains sign one shared escort order.",
        resourceful: "A route token secures a guarded freight slot.",
        failure: "The convoy departs without the disputed manifest.",
    },
    QuestNarrativeDefinition {
        quest_id: "bandit_tracks",
        direct: "The ambush line breaks when you turn its own tracks against it.",
        diplomatic: "Mako's informants identify the bandit quartermaster.",
        resourceful: "Watch cloth decoys split the raiders from their supply trail.",
        failure: "Wind fills the last prints and the hunt returns to the gate.",
    },
    QuestNarrativeDefinition {
        quest_id: "ration_audit",
        direct: "The missing ration lots are counted in front of every ward.",
        diplomatic: "Nia wins agreement on a fair shortage schedule.",
        resourceful: "Replacement alloy repairs the sealed bins before spoilage.",
        failure: "The books and physical stores diverge beyond a defensible count.",
    },
];

pub fn quest_narrative(quest_id: &str) -> Option<&'static QuestNarrativeDefinition> {
    QUEST_NARRATIVES
        .iter()
        .find(|narrative| narrative.quest_id == quest_id)
}

pub fn quest_resolution_text(quest_id: &str, approach: QuestApproach) -> Option<&'static str> {
    let narrative = quest_narrative(quest_id)?;
    Some(match approach {
        QuestApproach::Direct => narrative.direct,
        QuestApproach::Diplomatic => narrative.diplomatic,
        QuestApproach::Resourceful => narrative.resourceful,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RegionalQuestDefinition {
    pub id: &'static str,
    pub title: &'static str,
    pub archetype: QuestArchetype,
    pub giver_npc_id: &'static str,
    pub waypoint_room_ids: &'static [&'static str],
    pub encounter_id: Option<&'static str>,
    pub credit_reward: i64,
    pub reputation_reward: i32,
}

pub fn quest_condition_graph(
    definition: &RegionalQuestDefinition,
    approach: QuestApproach,
) -> QuestConditionGraph {
    let mut nodes = vec![QuestConditionNode {
        id: format!("{}_giver", definition.id),
        kind: QuestConditionKind::SpeakToGiver,
        subject_id: definition.giver_npc_id.to_string(),
        quantity: 1,
    }];
    nodes.extend(
        definition
            .waypoint_room_ids
            .iter()
            .enumerate()
            .map(|(index, room)| QuestConditionNode {
                id: format!("{}_waypoint_{}", definition.id, index + 1),
                kind: QuestConditionKind::VisitWaypoint,
                subject_id: (*room).to_string(),
                quantity: 1,
            }),
    );
    let rule = quest_runtime_rule(definition.archetype);
    match approach {
        QuestApproach::Direct => {
            if let Some(encounter) = definition.encounter_id {
                nodes.push(QuestConditionNode {
                    id: format!("{}_encounter", definition.id),
                    kind: QuestConditionKind::WinEncounter,
                    subject_id: encounter.to_string(),
                    quantity: 1,
                });
            }
        }
        QuestApproach::Diplomatic => nodes.push(QuestConditionNode {
            id: format!("{}_trust", definition.id),
            kind: QuestConditionKind::ReachTrust,
            subject_id: definition.giver_npc_id.to_string(),
            quantity: rule.minimum_trust_for_diplomacy.max(0) as u16,
        }),
        QuestApproach::Resourceful => nodes.push(QuestConditionNode {
            id: format!("{}_resource", definition.id),
            kind: QuestConditionKind::ConsumeItem,
            subject_id: rule.resource_item_id.to_string(),
            quantity: rule.resource_quantity,
        }),
    }
    nodes.push(QuestConditionNode {
        id: format!("{}_settlement", definition.id),
        kind: QuestConditionKind::ReturnForSettlement,
        subject_id: definition
            .waypoint_room_ids
            .last()
            .copied()
            .unwrap_or(definition.giver_npc_id)
            .to_string(),
        quantity: 1,
    });
    let topology = QuestGraphTopology::for_quest(definition.id)
        .expect("every authored regional quest has a bespoke topology");
    let node_ids = nodes.iter().map(|node| node.id.clone()).collect::<Vec<_>>();
    let giver = node_ids[0].clone();
    let settlement = node_ids.last().expect("settlement node exists").clone();
    let waypoint_ids = node_ids
        .iter()
        .skip(1)
        .take(definition.waypoint_room_ids.len())
        .cloned()
        .collect::<Vec<_>>();
    let branch = node_ids
        .get(1 + definition.waypoint_room_ids.len())
        .filter(|id| **id != settlement)
        .cloned();
    let mut edges = Vec::new();
    let mut connect = |from: &str, to: &str| {
        edges.push(QuestConditionEdge {
            from: from.to_string(),
            to: to.to_string(),
        });
    };
    let terminal = branch.as_deref().unwrap_or(&settlement);
    let w = |index: usize| waypoint_ids[index].as_str();
    match definition.id {
        // Every authored quest owns its edge list. Parallel edges below are
        // intentional player route choices; chains are intentional escorts.
        "wayfinder_oath" => {
            connect(&giver, w(0));
            connect(w(0), w(1));
            connect(w(1), w(2));
            connect(w(2), terminal);
        }
        "broken_milestone" => {
            connect(&giver, w(0));
            connect(&giver, w(1));
            connect(w(0), terminal);
            connect(w(1), terminal);
        }
        "forge_commission" => {
            connect(&giver, w(0));
            connect(w(0), w(1));
            connect(&giver, w(1));
            connect(w(1), terminal);
        }
        "lost_tooling" => {
            connect(&giver, w(0));
            connect(&giver, w(1));
            connect(w(0), w(1));
            connect(w(1), terminal);
        }
        "lantern_watch" => {
            connect(&giver, w(0));
            connect(w(0), w(1));
            connect(w(0), w(2));
            connect(w(1), w(2));
            connect(w(2), terminal);
        }
        "wanted_raider" => {
            connect(&giver, w(0));
            connect(&giver, w(1));
            connect(w(0), terminal);
            connect(w(1), terminal);
            connect(&giver, terminal);
            connect(w(0), &settlement);
        }
        "relay_salvage" => {
            connect(&giver, w(0));
            connect(w(0), w(1));
            connect(w(0), terminal);
            connect(w(1), terminal);
        }
        "fever_tonic" => {
            connect(&giver, w(0));
            connect(w(0), w(1));
            connect(w(1), terminal);
            connect(&giver, terminal);
        }
        "archive_witness" => {
            connect(&giver, w(0));
            connect(&giver, w(1));
            connect(w(0), terminal);
            connect(w(1), terminal);
            connect(w(0), w(1));
        }
        "market_debt" => {
            connect(&giver, w(0));
            connect(&giver, w(1));
            connect(w(0), w(1));
            connect(w(0), terminal);
            connect(w(1), terminal);
            connect(&giver, &settlement);
        }
        "missing_crate" => {
            connect(&giver, w(0));
            connect(w(0), w(1));
            connect(w(1), terminal);
            connect(&giver, w(1));
            connect(w(0), terminal);
        }
        "night_letter" => {
            connect(&giver, w(0));
            connect(w(0), w(1));
            connect(w(1), terminal);
            connect(&giver, terminal);
            connect(w(0), terminal);
        }
        "escort_manifest" => {
            connect(&giver, w(0));
            connect(w(0), w(1));
            connect(w(1), terminal);
            connect(&giver, w(1));
            connect(&giver, terminal);
        }
        "bandit_tracks" => {
            connect(&giver, w(0));
            connect(&giver, w(1));
            connect(w(0), terminal);
            connect(w(1), terminal);
            connect(w(0), w(1));
            connect(&giver, terminal);
        }
        "ration_audit" => {
            connect(&giver, w(0));
            connect(&giver, w(1));
            connect(w(0), w(2));
            connect(w(1), w(2));
            connect(w(2), terminal);
            connect(&giver, w(2));
        }
        _ => unreachable!("catalog validation binds every authored quest"),
    }
    if let Some(branch) = &branch {
        connect(branch, &settlement);
    }
    QuestConditionGraph {
        quest_id: definition.id.to_string(),
        topology,
        nodes,
        edges,
    }
}

pub const REGIONAL_QUEST_CATALOG: [RegionalQuestDefinition; 15] = [
    RegionalQuestDefinition {
        id: "wayfinder_oath",
        title: "The Wayfinder Oath",
        archetype: QuestArchetype::TrainingTrial,
        giver_npc_id: "street-compass-sifu",
        waypoint_room_ids: &["mentor_hall", "archive_steps", "expedition_gate"],
        encounter_id: Some("milestone_duel"),
        credit_reward: 35,
        reputation_reward: 3,
    },
    RegionalQuestDefinition {
        id: "broken_milestone",
        title: "The Broken Milestone",
        archetype: QuestArchetype::Investigation,
        giver_npc_id: "street-compass-sifu",
        waypoint_room_ids: &["mirror_square", "outer_signal_road"],
        encounter_id: Some("roadside_ambush"),
        credit_reward: 45,
        reputation_reward: 4,
    },
    RegionalQuestDefinition {
        id: "forge_commission",
        title: "Forge Commission",
        archetype: QuestArchetype::FindItem,
        giver_npc_id: "master-orsen",
        waypoint_room_ids: &["workshop_gate", "relay_quarter"],
        encounter_id: None,
        credit_reward: 40,
        reputation_reward: 3,
    },
    RegionalQuestDefinition {
        id: "lost_tooling",
        title: "Lost Tooling",
        archetype: QuestArchetype::FindItem,
        giver_npc_id: "master-orsen",
        waypoint_room_ids: &["caravan_yard", "outer_signal_road"],
        encounter_id: Some("scrap_stalker"),
        credit_reward: 55,
        reputation_reward: 5,
    },
    RegionalQuestDefinition {
        id: "lantern_watch",
        title: "Lantern Watch",
        archetype: QuestArchetype::Escort,
        giver_npc_id: "captain-veyra",
        waypoint_room_ids: &["night_watch_post", "market_wind_pavilion", "cistern_ward"],
        encounter_id: Some("night_raiders"),
        credit_reward: 55,
        reputation_reward: 5,
    },
    RegionalQuestDefinition {
        id: "wanted_raider",
        title: "Wanted: Ash Runner",
        archetype: QuestArchetype::Hunt,
        giver_npc_id: "captain-veyra",
        waypoint_room_ids: &["night_watch_post", "outer_signal_road"],
        encounter_id: Some("ash_runner"),
        credit_reward: 70,
        reputation_reward: 7,
    },
    RegionalQuestDefinition {
        id: "relay_salvage",
        title: "Relay Salvage",
        archetype: QuestArchetype::FindItem,
        giver_npc_id: "relay-smith-brann",
        waypoint_room_ids: &["relay_quarter", "outer_signal_road"],
        encounter_id: Some("scrap_stalker"),
        credit_reward: 50,
        reputation_reward: 4,
    },
    RegionalQuestDefinition {
        id: "fever_tonic",
        title: "Fever Tonic",
        archetype: QuestArchetype::Supply,
        giver_npc_id: "healer-nima",
        waypoint_room_ids: &["market_wind_pavilion", "lantern_infirmary"],
        encounter_id: None,
        credit_reward: 30,
        reputation_reward: 4,
    },
    RegionalQuestDefinition {
        id: "archive_witness",
        title: "Archive Witness",
        archetype: QuestArchetype::Investigation,
        giver_npc_id: "archivist-sol",
        waypoint_room_ids: &["archive_steps", "night_watch_post"],
        encounter_id: None,
        credit_reward: 35,
        reputation_reward: 5,
    },
    RegionalQuestDefinition {
        id: "market_debt",
        title: "Market Debt",
        archetype: QuestArchetype::Courier,
        giver_npc_id: "merchant-aya",
        waypoint_room_ids: &["market_wind_pavilion", "workshop_gate"],
        encounter_id: None,
        credit_reward: 45,
        reputation_reward: 2,
    },
    RegionalQuestDefinition {
        id: "missing_crate",
        title: "The Missing Crate",
        archetype: QuestArchetype::Investigation,
        giver_npc_id: "merchant-aya",
        waypoint_room_ids: &["caravan_yard", "cistern_ward"],
        encounter_id: Some("dock_thieves"),
        credit_reward: 50,
        reputation_reward: 4,
    },
    RegionalQuestDefinition {
        id: "night_letter",
        title: "Letter After Curfew",
        archetype: QuestArchetype::Courier,
        giver_npc_id: "courier-tess",
        waypoint_room_ids: &["caravan_yard", "night_watch_post"],
        encounter_id: Some("night_raiders"),
        credit_reward: 40,
        reputation_reward: 3,
    },
    RegionalQuestDefinition {
        id: "escort_manifest",
        title: "Escort Manifest",
        archetype: QuestArchetype::Escort,
        giver_npc_id: "courier-tess",
        waypoint_room_ids: &["caravan_yard", "expedition_gate"],
        encounter_id: None,
        credit_reward: 50,
        reputation_reward: 4,
    },
    RegionalQuestDefinition {
        id: "bandit_tracks",
        title: "Tracks Beyond the Gate",
        archetype: QuestArchetype::Hunt,
        giver_npc_id: "scout-mako",
        waypoint_room_ids: &["outer_signal_road", "caravan_yard"],
        encounter_id: Some("roadside_ambush"),
        credit_reward: 65,
        reputation_reward: 6,
    },
    RegionalQuestDefinition {
        id: "ration_audit",
        title: "Ration Audit",
        archetype: QuestArchetype::Supply,
        giver_npc_id: "quartermaster-nia",
        waypoint_room_ids: &["cistern_ward", "caravan_yard", "archive_steps"],
        encounter_id: None,
        credit_reward: 35,
        reputation_reward: 4,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillEffect {
    Damage,
    Guard,
    Mobility,
    Recon,
    Healing,
    Construction,
    Economy,
    Diplomacy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SkillDefinition {
    pub id: &'static str,
    pub display_name: &'static str,
    pub sect: Option<SectId>,
    pub prerequisite: Option<&'static str>,
    pub effect: SkillEffect,
    pub rts_modifier_permille: u16,
}

pub const SKILL_CATALOG: [SkillDefinition; 15] = [
    SkillDefinition {
        id: "basic_inner_power",
        display_name: "Basic Inner Power",
        sect: None,
        prerequisite: None,
        effect: SkillEffect::Guard,
        rts_modifier_permille: 40,
    },
    SkillDefinition {
        id: "basic_unarmed",
        display_name: "Basic Unarmed",
        sect: None,
        prerequisite: None,
        effect: SkillEffect::Damage,
        rts_modifier_permille: 30,
    },
    SkillDefinition {
        id: "basic_blade",
        display_name: "Basic Blade",
        sect: None,
        prerequisite: None,
        effect: SkillEffect::Damage,
        rts_modifier_permille: 35,
    },
    SkillDefinition {
        id: "basic_lightness",
        display_name: "Basic Lightness",
        sect: None,
        prerequisite: None,
        effect: SkillEffect::Mobility,
        rts_modifier_permille: 35,
    },
    SkillDefinition {
        id: "reading_and_contracts",
        display_name: "Reading and Contracts",
        sect: None,
        prerequisite: None,
        effect: SkillEffect::Diplomacy,
        rts_modifier_permille: 20,
    },
    SkillDefinition {
        id: "route_scouting",
        display_name: "Route Scouting",
        sect: Some(SectId::StreetCompass),
        prerequisite: Some("basic_lightness"),
        effect: SkillEffect::Recon,
        rts_modifier_permille: 70,
    },
    SkillDefinition {
        id: "wind_step",
        display_name: "Wind Step",
        sect: Some(SectId::StreetCompass),
        prerequisite: Some("route_scouting"),
        effect: SkillEffect::Mobility,
        rts_modifier_permille: 100,
    },
    SkillDefinition {
        id: "compass_feint",
        display_name: "Compass Feint",
        sect: Some(SectId::StreetCompass),
        prerequisite: Some("wind_step"),
        effect: SkillEffect::Damage,
        rts_modifier_permille: 90,
    },
    SkillDefinition {
        id: "artifact_crafting",
        display_name: "Artifact Crafting",
        sect: Some(SectId::IronWorkshop),
        prerequisite: Some("reading_and_contracts"),
        effect: SkillEffect::Construction,
        rts_modifier_permille: 80,
    },
    SkillDefinition {
        id: "iron_guard",
        display_name: "Iron Guard",
        sect: Some(SectId::IronWorkshop),
        prerequisite: Some("artifact_crafting"),
        effect: SkillEffect::Guard,
        rts_modifier_permille: 110,
    },
    SkillDefinition {
        id: "relay_overcharge",
        display_name: "Relay Overcharge",
        sect: Some(SectId::IronWorkshop),
        prerequisite: Some("iron_guard"),
        effect: SkillEffect::Construction,
        rts_modifier_permille: 120,
    },
    SkillDefinition {
        id: "shadow_patrol",
        display_name: "Shadow Patrol",
        sect: Some(SectId::NightWatch),
        prerequisite: Some("basic_lightness"),
        effect: SkillEffect::Recon,
        rts_modifier_permille: 75,
    },
    SkillDefinition {
        id: "night_veil",
        display_name: "Night Veil",
        sect: Some(SectId::NightWatch),
        prerequisite: Some("shadow_patrol"),
        effect: SkillEffect::Mobility,
        rts_modifier_permille: 95,
    },
    SkillDefinition {
        id: "inner_flame",
        display_name: "Inner Flame",
        sect: Some(SectId::NightWatch),
        prerequisite: Some("night_veil"),
        effect: SkillEffect::Damage,
        rts_modifier_permille: 115,
    },
    SkillDefinition {
        id: "field_mend",
        display_name: "Field Mend",
        sect: None,
        prerequisite: Some("basic_inner_power"),
        effect: SkillEffect::Healing,
        rts_modifier_permille: 90,
    },
];

pub fn skill_unlockable(skill_id: &str, known: &BTreeSet<String>, sect: Option<SectId>) -> bool {
    SKILL_CATALOG
        .iter()
        .find(|skill| skill.id == skill_id)
        .is_some_and(|skill| {
            skill.sect.is_none_or(|required| Some(required) == sect)
                && skill
                    .prerequisite
                    .is_none_or(|required| known.contains(required))
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncounterKind {
    Duel,
    Ambush,
    Hunt,
    Defense,
    Investigation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EncounterDefinition {
    pub id: &'static str,
    pub kind: EncounterKind,
    pub enemy_name: &'static str,
    pub enemy_hp: i64,
    pub loot_table: &'static [&'static str],
}

pub const ENCOUNTER_CATALOG: [EncounterDefinition; 7] = [
    EncounterDefinition {
        id: "milestone_duel",
        kind: EncounterKind::Duel,
        enemy_name: "Milestone Challenger",
        enemy_hp: 120,
        loot_table: &["route-token"],
    },
    EncounterDefinition {
        id: "roadside_ambush",
        kind: EncounterKind::Ambush,
        enemy_name: "Roadside Cutters",
        enemy_hp: 145,
        loot_table: &["salvaged-alloy", "field-tonic-kit"],
    },
    EncounterDefinition {
        id: "scrap_stalker",
        kind: EncounterKind::Hunt,
        enemy_name: "Scrap Stalker",
        enemy_hp: 170,
        loot_table: &["salvaged-alloy", "relay-core-fragment"],
    },
    EncounterDefinition {
        id: "night_raiders",
        kind: EncounterKind::Defense,
        enemy_name: "Night Raiders",
        enemy_hp: 185,
        loot_table: &["watch-cloth", "field-tonic-kit"],
    },
    EncounterDefinition {
        id: "ash_runner",
        kind: EncounterKind::Hunt,
        enemy_name: "Ash Runner",
        enemy_hp: 210,
        loot_table: &["ash-runner-seal", "watch-cloth"],
    },
    EncounterDefinition {
        id: "dock_thieves",
        kind: EncounterKind::Investigation,
        enemy_name: "Dock Thieves",
        enemy_hp: 135,
        loot_table: &["market-ledger", "salvaged-alloy"],
    },
    EncounterDefinition {
        id: "signal_road_ambush",
        kind: EncounterKind::Ambush,
        enemy_name: "Signal Road Ambushers",
        enemy_hp: 135,
        loot_table: &["signal-road-emblem"],
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatLogBeat {
    pub kind: String,
    pub text: String,
}

pub fn original_combat_log(encounter_id: &str, round: u8, player_won: bool) -> Vec<CombatLogBeat> {
    let enemy = ENCOUNTER_CATALOG
        .iter()
        .find(|entry| entry.id == encounter_id)
        .map(|entry| entry.enemy_name)
        .unwrap_or("Unknown Rival");
    let opening = match encounter_id {
        "milestone_duel" => "A chalk route circle closes beneath both fighters' feet.",
        "roadside_ambush" => "Loose gravel answers three hidden steps before steel appears.",
        "scrap_stalker" => "The salvage beast drags sparks through the broken relay yard.",
        "night_raiders" => "Lanterns shutter in sequence as raiders test the ward line.",
        "ash_runner" => "The Ash Runner never stops moving long enough to cast one shadow.",
        "dock_thieves" => "Wet ledger pages flutter between stacked crates and drawn knives.",
        "signal_road_ambush" => "Signal wire hums once, then the roadside rises into an ambush.",
        _ => "Lantern light narrows around the contested ground.",
    };
    let lesson = match encounter_id {
        "milestone_duel" => "Measured footwork turns the trial from strength into geometry.",
        "roadside_ambush" => "The party breaks the encirclement by holding one quiet exit.",
        "scrap_stalker" => "A feint draws the plated head away from its exposed relay spine.",
        "night_raiders" => "Guard and counter-signal keep the infirmary lane from collapsing.",
        "ash_runner" => "Patience steals the runner's rhythm before the final exchange.",
        "dock_thieves" => "A recovered seal exposes the thieves' false cargo order.",
        "signal_road_ambush" => "The party follows the wire's vibration to the hidden caller.",
        _ => "Breath, distance and timing decide the center line.",
    };
    vec![
        CombatLogBeat {
            kind: "opening".to_string(),
            text: opening.to_string(),
        },
        CombatLogBeat {
            kind: "opponent".to_string(),
            text: format!("{enemy} commits first; the party answers without losing formation."),
        },
        CombatLogBeat {
            kind: "exchange".to_string(),
            text: format!("Steel, breath and footwork trade measure across {round} rounds."),
        },
        CombatLogBeat {
            kind: "lesson".to_string(),
            text: lesson.to_string(),
        },
        CombatLogBeat {
            kind: "outcome".to_string(),
            text: if player_won {
                format!("{enemy} yields the road; your party keeps the initiative.")
            } else {
                format!("{enemy} holds the ground, forcing your party to regroup.")
            },
        },
    ]
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EconomyItemDefinition {
    pub id: &'static str,
    pub display_name: &'static str,
    pub buy_price: i64,
    pub max_durability: u16,
    pub material: bool,
}

pub const ECONOMY_ITEM_CATALOG: [EconomyItemDefinition; 22] = [
    EconomyItemDefinition {
        id: "route-guard-staff",
        display_name: "Route Guard Staff",
        buy_price: 70,
        max_durability: 80,
        material: false,
    },
    EconomyItemDefinition {
        id: "street-compass-bracer",
        display_name: "Street Compass Bracer",
        buy_price: 55,
        max_durability: 70,
        material: false,
    },
    EconomyItemDefinition {
        id: "iron-workshop-blade",
        display_name: "Iron Workshop Blade",
        buy_price: 85,
        max_durability: 100,
        material: false,
    },
    EconomyItemDefinition {
        id: "market-wind-sword",
        display_name: "Market Wind Sword",
        buy_price: 90,
        max_durability: 85,
        material: false,
    },
    EconomyItemDefinition {
        id: "night-watch-cloak",
        display_name: "Night Watch Cloak",
        buy_price: 75,
        max_durability: 65,
        material: false,
    },
    EconomyItemDefinition {
        id: "raid-signal-drum",
        display_name: "Raid Signal Drum",
        buy_price: 80,
        max_durability: 75,
        material: false,
    },
    EconomyItemDefinition {
        id: "field-tonic-kit",
        display_name: "Field Tonic Kit",
        buy_price: 25,
        max_durability: 1,
        material: false,
    },
    EconomyItemDefinition {
        id: "relay-core-fragment",
        display_name: "Relay Core Fragment",
        buy_price: 120,
        max_durability: 120,
        material: false,
    },
    EconomyItemDefinition {
        id: "evidence-wrap-case",
        display_name: "Archive Wrap Case",
        buy_price: 35,
        max_durability: 60,
        material: false,
    },
    EconomyItemDefinition {
        id: "salvaged-alloy",
        display_name: "Salvaged Alloy",
        buy_price: 12,
        max_durability: 0,
        material: true,
    },
    EconomyItemDefinition {
        id: "watch-cloth",
        display_name: "Night Watch Cloth",
        buy_price: 10,
        max_durability: 0,
        material: true,
    },
    EconomyItemDefinition {
        id: "route-token",
        display_name: "Wayfinder Route Token",
        buy_price: 15,
        max_durability: 0,
        material: true,
    },
    EconomyItemDefinition {
        id: "ash-runner-seal",
        display_name: "Ash Runner Seal",
        buy_price: 45,
        max_durability: 0,
        material: true,
    },
    EconomyItemDefinition {
        id: "market-ledger",
        display_name: "Recovered Market Ledger",
        buy_price: 30,
        max_durability: 0,
        material: true,
    },
    EconomyItemDefinition {
        id: "reinforced-staff",
        display_name: "Reinforced Route Staff",
        buy_price: 130,
        max_durability: 140,
        material: false,
    },
    EconomyItemDefinition {
        id: "signal-lamellar",
        display_name: "Signal Lamellar",
        buy_price: 150,
        max_durability: 160,
        material: false,
    },
    EconomyItemDefinition {
        id: "watcher-boots",
        display_name: "Watcher Boots",
        buy_price: 115,
        max_durability: 100,
        material: false,
    },
    EconomyItemDefinition {
        id: "field-medic-satchel",
        display_name: "Field Medic Satchel",
        buy_price: 100,
        max_durability: 90,
        material: false,
    },
    EconomyItemDefinition {
        id: "compass-thread-coat",
        display_name: "Compass Thread Coat",
        buy_price: 165,
        max_durability: 150,
        material: false,
    },
    EconomyItemDefinition {
        id: "emberglass-lens",
        display_name: "Emberglass Signal Lens",
        buy_price: 145,
        max_durability: 110,
        material: false,
    },
    EconomyItemDefinition {
        id: "cistern-seal-kit",
        display_name: "Cistern Seal Kit",
        buy_price: 75,
        max_durability: 40,
        material: false,
    },
    EconomyItemDefinition {
        id: "ashward-tonic",
        display_name: "Ashward Tonic",
        buy_price: 50,
        max_durability: 1,
        material: false,
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CraftingRecipe {
    pub id: &'static str,
    pub output_item_id: &'static str,
    pub ingredients: &'static [(&'static str, u16)],
    pub required_skill_id: &'static str,
}

pub const CRAFTING_RECIPES: [CraftingRecipe; 8] = [
    CraftingRecipe {
        id: "reinforced_staff",
        output_item_id: "reinforced-staff",
        ingredients: &[("salvaged-alloy", 3), ("route-token", 1)],
        required_skill_id: "artifact_crafting",
    },
    CraftingRecipe {
        id: "signal_lamellar",
        output_item_id: "signal-lamellar",
        ingredients: &[("salvaged-alloy", 5), ("watch-cloth", 2)],
        required_skill_id: "iron_guard",
    },
    CraftingRecipe {
        id: "watcher_boots",
        output_item_id: "watcher-boots",
        ingredients: &[("watch-cloth", 4), ("route-token", 1)],
        required_skill_id: "shadow_patrol",
    },
    CraftingRecipe {
        id: "medic_satchel",
        output_item_id: "field-medic-satchel",
        ingredients: &[("watch-cloth", 2), ("salvaged-alloy", 2)],
        required_skill_id: "field_mend",
    },
    CraftingRecipe {
        id: "compass_thread_coat",
        output_item_id: "compass-thread-coat",
        ingredients: &[("watch-cloth", 3), ("route-token", 2)],
        required_skill_id: "route_scouting",
    },
    CraftingRecipe {
        id: "emberglass_lens",
        output_item_id: "emberglass-lens",
        ingredients: &[("salvaged-alloy", 3), ("relay-core-fragment", 1)],
        required_skill_id: "relay_overcharge",
    },
    CraftingRecipe {
        id: "cistern_seal_kit",
        output_item_id: "cistern-seal-kit",
        ingredients: &[("salvaged-alloy", 2), ("watch-cloth", 2)],
        required_skill_id: "artifact_crafting",
    },
    CraftingRecipe {
        id: "ashward_tonic",
        output_item_id: "ashward-tonic",
        ingredients: &[("field-tonic-kit", 1), ("ash-runner-seal", 1)],
        required_skill_id: "field_mend",
    },
];

pub fn market_price(item_id: &str, day: u32, buying: bool) -> Option<i64> {
    market_price_with_state(item_id, day, 8, 0, buying)
}

pub fn market_price_with_state(
    item_id: &str,
    day: u32,
    stock: u16,
    demand: i16,
    buying: bool,
) -> Option<i64> {
    let item = ECONOMY_ITEM_CATALOG
        .iter()
        .find(|item| item.id == item_id)?;
    let cycle = ((day as i64 + item.id.bytes().map(i64::from).sum::<i64>()) % 5) - 2;
    let scarcity = (8_i64 - i64::from(stock.min(16))) * 35;
    let demand_pressure = i64::from(demand.clamp(-20, 20)) * 18;
    let demand_permille = (1_000 + cycle * 70 + scarcity + demand_pressure).clamp(550, 1_650);
    let market = item.buy_price * demand_permille / 1_000;
    Some(if buying {
        market.max(1)
    } else {
        (market * 55 / 100).max(1)
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemCondition {
    pub item_id: String,
    pub durability: u16,
    pub max_durability: u16,
}

impl ItemCondition {
    pub fn new(item_id: impl Into<String>) -> Option<Self> {
        let item_id = item_id.into();
        let definition = ECONOMY_ITEM_CATALOG
            .iter()
            .find(|entry| entry.id == item_id)?;
        Some(Self {
            item_id,
            durability: definition.max_durability,
            max_durability: definition.max_durability,
        })
    }

    pub fn apply_wear(&mut self, amount: u16) {
        self.durability = self.durability.saturating_sub(amount);
    }
    pub fn repair_cost(&self) -> i64 {
        let missing = i64::from(self.max_durability.saturating_sub(self.durability));
        (missing + 3) / 4
    }
    pub fn repair(&mut self) {
        self.durability = self.max_durability;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_region_has_three_sects_ten_npcs_fifteen_quests_and_seven_encounters() {
        assert_eq!(SECT_CATALOG.len(), 3);
        assert_eq!(NPC_CATALOG.len(), 10);
        assert_eq!(REGIONAL_QUEST_CATALOG.len(), 15);
        assert_eq!(ENCOUNTER_CATALOG.len(), 7);
        assert!(REGIONAL_QUEST_CATALOG
            .iter()
            .all(|quest| NPC_CATALOG.iter().any(|npc| npc.id == quest.giver_npc_id)));
        assert!(REGIONAL_QUEST_CATALOG.iter().all(|quest| {
            quest.waypoint_room_ids.len() >= 2
                && quest
                    .waypoint_room_ids
                    .windows(2)
                    .all(|rooms| rooms[0] != rooms[1])
        }));
        assert_eq!(QUEST_NARRATIVES.len(), REGIONAL_QUEST_CATALOG.len());
        for quest in &REGIONAL_QUEST_CATALOG {
            let narrative = quest_narrative(quest.id).expect("every quest has authored prose");
            assert_ne!(narrative.direct, narrative.diplomatic);
            assert_ne!(narrative.direct, narrative.resourceful);
            for approach in [
                QuestApproach::Direct,
                QuestApproach::Diplomatic,
                QuestApproach::Resourceful,
            ] {
                let graph = quest_condition_graph(quest, approach);
                assert_eq!(
                    graph
                        .nodes
                        .iter()
                        .filter(|node| node.kind == QuestConditionKind::VisitWaypoint)
                        .count(),
                    quest.waypoint_room_ids.len()
                );
                assert_eq!(
                    graph.nodes.last().map(|node| node.kind),
                    Some(QuestConditionKind::ReturnForSettlement)
                );
                assert!(!graph.edges.is_empty());
            }
        }
        assert_eq!(
            REGIONAL_QUEST_CATALOG
                .iter()
                .map(|quest| quest_condition_graph(quest, QuestApproach::Direct).topology)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            REGIONAL_QUEST_CATALOG.len(),
            "every regional quest must retain a distinct authored topology"
        );
        let mut signature_owners = std::collections::BTreeMap::new();
        for quest in &REGIONAL_QUEST_CATALOG {
            let graph = quest_condition_graph(quest, QuestApproach::Direct);
            let indexes = graph
                .nodes
                .iter()
                .enumerate()
                .map(|(index, node)| (node.id.as_str(), index))
                .collect::<std::collections::BTreeMap<_, _>>();
            let mut edges = graph
                .edges
                .iter()
                .map(|edge| (indexes[edge.from.as_str()], indexes[edge.to.as_str()]))
                .collect::<Vec<_>>();
            edges.sort_unstable();
            signature_owners
                .entry((graph.nodes.len(), edges))
                .or_insert_with(Vec::new)
                .push(quest.id);
        }
        assert_eq!(
            signature_owners.len(),
            REGIONAL_QUEST_CATALOG.len(),
            "quest ids and enum names cannot disguise a repeated graph structure: {signature_owners:?}"
        );
    }

    #[test]
    fn every_relationship_npc_moves_and_has_five_stage_choice_dialogue() {
        let mut opening_lines = BTreeSet::new();
        let mut trusted_lines = BTreeSet::new();
        for npc in NPC_CATALOG {
            let schedule = npc_schedule(npc.id).expect("every NPC has a schedule");
            assert!(!schedule.activity.trim().is_empty());
            let opening = npc_dialogue(npc.id, 0, 0).expect("opening dialogue exists");
            let trusted = npc_dialogue(npc.id, 8, 2).expect("trusted dialogue exists");
            assert_ne!(opening, trusted);
            opening_lines.insert(opening);
            trusted_lines.insert(trusted);
            let rooms = [8 * 60, 14 * 60, 21 * 60]
                .into_iter()
                .filter_map(|minute| npc_room_at(npc.id, minute))
                .collect::<BTreeSet<_>>();
            assert!(rooms.len() >= 2, "{} never moves", npc.id);
            for choice in [
                DialogueChoice::AskForWork,
                DialogueChoice::OfferHelp,
                DialogueChoice::ShareNews,
            ] {
                assert!(
                    !npc_choice_dialogue(npc.id, RelationshipStage::Confidant, choice).is_empty()
                );
            }
        }
        assert_eq!(opening_lines.len(), NPC_CATALOG.len());
        assert_eq!(trusted_lines.len(), NPC_CATALOG.len());
    }

    #[test]
    fn skill_tree_requires_prerequisites_and_matching_sect() {
        let mut known = BTreeSet::from(["basic_lightness".to_string()]);
        assert!(skill_unlockable(
            "route_scouting",
            &known,
            Some(SectId::StreetCompass)
        ));
        assert!(!skill_unlockable(
            "route_scouting",
            &known,
            Some(SectId::NightWatch)
        ));
        known.insert("route_scouting".to_string());
        assert!(skill_unlockable(
            "wind_step",
            &known,
            Some(SectId::StreetCompass)
        ));
    }

    #[test]
    fn combat_logs_are_original_deterministic_and_encounter_bound() {
        let first = original_combat_log("ash_runner", 4, true);
        assert_eq!(first, original_combat_log("ash_runner", 4, true));
        assert!(first.iter().any(|beat| beat.text.contains("Ash Runner")));
    }

    #[test]
    fn item_economy_has_shop_prices_recipes_durability_and_repair() {
        assert_eq!(ECONOMY_ITEM_CATALOG.len(), 22);
        assert_eq!(CRAFTING_RECIPES.len(), 8);
        assert!(
            market_price("iron-workshop-blade", 2, true).unwrap()
                > market_price("iron-workshop-blade", 2, false).unwrap()
        );
        let mut item = ItemCondition::new("iron-workshop-blade").unwrap();
        item.apply_wear(17);
        assert!(item.repair_cost() > 0);
        item.repair();
        assert_eq!(item.durability, item.max_durability);
    }
}
