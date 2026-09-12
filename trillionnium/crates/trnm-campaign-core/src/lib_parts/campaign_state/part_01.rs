#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CampaignSaveV1 {
    pub contract_version: String,
    #[serde(default = "legacy_campaign_schema_revision")]
    pub schema_revision: u16,
    pub campaign_id: String,
    pub revision: u64,
    pub room: CampaignRoom,
    pub phase: CampaignPhase,
    pub character: WorldTrillionniumCharacter,
    #[serde(default)]
    pub character_identity: CharacterIdentity,
    #[serde(default)]
    pub character_origin: CharacterOrigin,
    #[serde(default)]
    pub difficulty: CampaignDifficulty,
    pub progression: CampaignProgression,
    pub party: Vec<PartyMember>,
    pub active_party_ids: Vec<String>,
    #[serde(default)]
    pub selected_training_path: TrainingPath,
    #[serde(default)]
    pub selected_loadout: LoadoutPreset,
    #[serde(default)]
    pub active_mission: CampaignMission,
    #[serde(default)]
    pub skirmish_setup: SkirmishSetup,
    #[serde(default)]
    pub story: StoryProgress,
    #[serde(default)]
    pub npc_relationships: BTreeMap<String, NpcRelationship>,
    #[serde(default)]
    pub faction_rank: FactionRank,
    #[serde(default)]
    pub last_sparring: Option<SparringReport>,
    #[serde(default)]
    pub pending_growth_stat: Option<GrowthStat>,
    #[serde(default)]
    pub build_path: BuildPath,
    #[serde(default)]
    pub unlocked_titles: BTreeSet<BuildTitle>,
    #[serde(default)]
    pub active_title: Option<BuildTitle>,
    #[serde(default)]
    pub active_encounter: Option<RpgEncounterState>,
    #[serde(default)]
    pub last_encounter_outcome: Option<EncounterOutcome>,
    #[serde(default)]
    pub combat_log: Vec<CombatLogBeat>,
    #[serde(default)]
    pub regional_quest_states: BTreeMap<String, QuestState>,
    #[serde(default)]
    pub active_regional_quest_id: Option<String>,
    #[serde(default)]
    pub active_regional_quest_step: usize,
    #[serde(default)]
    pub active_regional_quest_runtime: Option<RegionalQuestRuntime>,
    #[serde(default)]
    pub regional_quest_failure_counts: BTreeMap<String, u8>,
    #[serde(default)]
    pub dialogue_choice: DialogueChoice,
    #[serde(default)]
    pub equipped_technique_slot: u8,
    #[serde(default = "default_secondary_technique_slot")]
    pub secondary_technique_slot: u8,
    #[serde(default)]
    pub technique_mastery: BTreeMap<String, u16>,
    #[serde(default)]
    pub main_story_chapter: MainStoryChapter,
    #[serde(default)]
    pub main_story_choice: MainStoryChoice,
    #[serde(default)]
    pub main_story_decisions: Vec<MainStoryDecisionRecord>,
    #[serde(default)]
    pub main_story_ending: Option<MainStoryEnding>,
    #[serde(default)]
    pub pending_main_story_chapter: Option<MainStoryChapter>,
    #[serde(default)]
    pub main_story_scene_progress: BTreeMap<String, u8>,
    #[serde(default)]
    pub post_ending_world_state: Option<String>,
    #[serde(default)]
    pub ending_epilogue_progress: u8,
    #[serde(default)]
    pub ending_epilogue_complete: bool,
    #[serde(default)]
    pub last_npc_conversation: Option<NpcConversationRecord>,
    #[serde(default)]
    pub conversation_history: Vec<NpcConversationRecord>,
    #[serde(default)]
    pub social_event_history: Vec<NpcSocialEventRecord>,
    #[serde(default)]
    pub npc_memory: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub npc_bonds: BTreeMap<String, i16>,
    #[serde(default)]
    pub npc_work_output: BTreeMap<String, u32>,
    #[serde(default)]
    pub npc_autonomous_goals: BTreeMap<String, NpcAutonomousGoal>,
    #[serde(default)]
    pub npc_goal_rooms: BTreeMap<String, String>,
    #[serde(default)]
    pub selected_shop_item_index: usize,
    #[serde(default)]
    pub selected_recipe_index: usize,
    #[serde(default)]
    pub selected_inventory_index: usize,
    #[serde(default)]
    pub item_conditions: BTreeMap<String, ItemCondition>,
    #[serde(default)]
    pub market_stock: BTreeMap<String, u16>,
    #[serde(default)]
    pub market_demand: BTreeMap<String, i16>,
    #[serde(default = "default_regional_market_stock")]
    pub regional_market_stock: BTreeMap<String, BTreeMap<String, u16>>,
    #[serde(default = "default_regional_market_demand")]
    pub regional_market_demand: BTreeMap<String, BTreeMap<String, i16>>,
    #[serde(default)]
    pub regional_logistics: Vec<RegionalMarketTransfer>,
    #[serde(default)]
    pub active_regional_caravans: Vec<RegionalCaravanState>,
    #[serde(default)]
    pub economy_mode: EconomyMode,
    #[serde(default)]
    pub economy_account_binding: Option<EconomyAccountBinding>,
    #[serde(default)]
    pub wallet_snapshot: WalletSnapshot,
    #[serde(default)]
    pub pending_economic_intents: Vec<EconomicIntent>,
    #[serde(default)]
    pub pending_economic_compensations: Vec<EconomicIntent>,
    #[serde(default)]
    pub verified_economic_receipts: Vec<EconomicReceipt>,
    #[serde(default)]
    pub economic_idempotency_keys: BTreeSet<String>,
    #[serde(default)]
    pub economic_dead_letters: Vec<EconomicIntent>,
    #[serde(default)]
    pub pending_tradeable_purchases: Vec<PendingTradeablePurchase>,
    #[serde(default)]
    pub value_events: Vec<ValueEventRecord>,
    #[serde(default)]
    pub wallet_reward_issued_by_day: BTreeMap<u32, i64>,
    #[serde(default)]
    pub reconciliation_cursor: u64,
    #[serde(default)]
    pub quest_chain: Option<QuestChainProgress>,
    #[serde(default)]
    pub world_clock: WorldClock,
    #[serde(default)]
    pub expedition_supplies: ExpeditionSupplyState,
    #[serde(default)]
    pub selected_expedition_preparation: ExpeditionPreparation,
    pub mentor_met: bool,
    pub trained_with_mentor: bool,
    pub quest_state: QuestState,
    pub pending_battle: Option<PendingBattleV1>,
    pub settled_battle_ids: BTreeSet<String>,
    pub settlement_receipts: Vec<SettlementReceiptV1>,
}

impl Default for CampaignSaveV1 {
    fn default() -> Self {
        let mut character = WorldTrillionniumCharacter::default_for("local-player");
        CharacterOrigin::Balanced.apply(&mut character.attributes);
        character
            .skill_ids
            .push(CharacterOrigin::Balanced.starter_skill().to_string());
        for item_id in [
            "iron-workshop-blade",
            "market-wind-sword",
            "night-watch-cloak",
            "raid-signal-drum",
        ] {
            if let Some(item) = trillionnium_inventory_item_for(
                "local-player",
                item_id,
                "first_contact_loadout_choice",
                None,
                0,
            ) {
                character.inventory_items.push(item);
            }
        }
        character.equipment_slots.clear();
        for item in &mut character.inventory_items {
            item.equipped_slot = None;
        }
        let mut skill_progress = BTreeMap::new();
        for skill_id in &character.skill_ids {
            skill_progress.insert(
                skill_id.clone(),
                SkillProgress {
                    rank: 1,
                    experience: 0,
                },
            );
        }
        let base = character.attributes.clone();
        let mut scout = base.clone();
        scout.agility += 4;
        scout.insight += 2;
        let mut warden = base.clone();
        warden.physique += 5;
        warden.resolve += 4;
        let mut striker = base.clone();
        striker.force += 5;
        striker.agility += 2;
        let item_conditions = character_item_conditions(&character);
        Self {
            contract_version: CAMPAIGN_SAVE_CONTRACT.to_string(),
            schema_revision: 12,
            campaign_id: "local-campaign".to_string(),
            revision: 0,
            room: CampaignRoom::MirrorSquare,
            phase: CampaignPhase::Town,
            character,
            character_identity: CharacterIdentity::default(),
            character_origin: CharacterOrigin::Balanced,
            difficulty: CampaignDifficulty::Standard,
            progression: CampaignProgression {
                level: 1,
                experience: 0,
                credits: default_campaign_credits(),
                mentor_training_sessions: 0,
                aftershock_completions: 0,
                growth_points_available: 1,
                growth_points_awarded: 1,
                growth_allocations: BTreeMap::new(),
                skill_progress,
                inventory: Vec::new(),
                world_flags: BTreeSet::new(),
            },
            party: vec![
                PartyMember {
                    unit_id: "hero".to_string(),
                    display_name: "Mirror Ranger".to_string(),
                    role: "worker".to_string(),
                    attributes: base,
                    skill_ids: vec!["basic_inner_power".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "aya".to_string(),
                    display_name: "Aya".to_string(),
                    role: "scout".to_string(),
                    attributes: scout,
                    skill_ids: vec![
                        "basic_lightness".to_string(),
                        "route_scouting".to_string(),
                        "wind_step".to_string(),
                    ],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "mako".to_string(),
                    display_name: "Mako".to_string(),
                    role: "warden".to_string(),
                    attributes: warden,
                    skill_ids: vec!["basic_unarmed".to_string(), "iron_guard".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "tess".to_string(),
                    display_name: "Tess".to_string(),
                    role: "striker".to_string(),
                    attributes: striker,
                    skill_ids: vec!["basic_blade".to_string(), "inner_flame".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "nia".to_string(),
                    display_name: "Nia".to_string(),
                    role: "medic".to_string(),
                    attributes: {
                        let mut attributes = TrillionniumAttributes::default();
                        attributes.resolve += 4;
                        attributes.insight += 5;
                        attributes
                    },
                    skill_ids: vec!["field_mend".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "brann".to_string(),
                    display_name: "Brann".to_string(),
                    role: "engineer".to_string(),
                    attributes: {
                        let mut attributes = TrillionniumAttributes::default();
                        attributes.craft += 6;
                        attributes.physique += 3;
                        attributes
                    },
                    skill_ids: vec!["relay_overcharge".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: false,
                },
                PartyMember {
                    unit_id: "sol".to_string(),
                    display_name: "Sol".to_string(),
                    role: "mystic".to_string(),
                    attributes: {
                        let mut attributes = TrillionniumAttributes::default();
                        attributes.insight += 6;
                        attributes.resolve += 2;
                        attributes
                    },
                    skill_ids: vec!["inner_flame".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
            ],
            active_party_ids: vec![
                "hero".to_string(),
                "aya".to_string(),
                "mako".to_string(),
                "tess".to_string(),
            ],
            selected_training_path: TrainingPath::default(),
            selected_loadout: LoadoutPreset::default(),
            active_mission: CampaignMission::default(),
            skirmish_setup: SkirmishSetup::default(),
            story: StoryProgress::default(),
            npc_relationships: NPC_CATALOG
                .iter()
                .map(|npc| {
                    (
                        npc.id.to_string(),
                        NpcRelationship::new(npc.id, npc.faction_id),
                    )
                })
                .collect(),
            faction_rank: FactionRank::Outsider,
            last_sparring: None,
            pending_growth_stat: None,
            build_path: BuildPath::Unformed,
            unlocked_titles: BTreeSet::new(),
            active_title: None,
            active_encounter: None,
            last_encounter_outcome: None,
            combat_log: Vec::new(),
            regional_quest_states: REGIONAL_QUEST_CATALOG
                .iter()
                .map(|quest| (quest.id.to_string(), QuestState::Available))
                .collect(),
            active_regional_quest_id: None,
            active_regional_quest_step: 0,
            active_regional_quest_runtime: None,
            regional_quest_failure_counts: BTreeMap::new(),
            dialogue_choice: DialogueChoice::AskForWork,
            equipped_technique_slot: 0,
            secondary_technique_slot: default_secondary_technique_slot(),
            technique_mastery: BTreeMap::new(),
            main_story_chapter: MainStoryChapter::MirrorCityOaths,
            main_story_choice: MainStoryChoice::ProtectWayhouses,
            main_story_decisions: Vec::new(),
            main_story_ending: None,
            pending_main_story_chapter: None,
            main_story_scene_progress: BTreeMap::new(),
            post_ending_world_state: None,
            ending_epilogue_progress: 0,
            ending_epilogue_complete: false,
            last_npc_conversation: None,
            conversation_history: Vec::new(),
            social_event_history: Vec::new(),
            npc_memory: BTreeMap::new(),
            npc_bonds: BTreeMap::new(),
            npc_work_output: BTreeMap::new(),
            npc_autonomous_goals: BTreeMap::new(),
            npc_goal_rooms: BTreeMap::new(),
            selected_shop_item_index: 0,
            selected_recipe_index: 0,
            selected_inventory_index: 0,
            item_conditions,
            market_stock: ECONOMY_ITEM_CATALOG
                .iter()
                .map(|item| (item.id.to_string(), if item.material { 12 } else { 4 }))
                .collect(),
            market_demand: ECONOMY_ITEM_CATALOG
                .iter()
                .map(|item| (item.id.to_string(), 0))
                .collect(),
            regional_market_stock: default_regional_market_stock(),
            regional_market_demand: default_regional_market_demand(),
            regional_logistics: Vec::new(),
            active_regional_caravans: Vec::new(),
            economy_mode: EconomyMode::OfflineLocal,
            economy_account_binding: None,
            wallet_snapshot: WalletSnapshot::default(),
            pending_economic_intents: Vec::new(),
            pending_economic_compensations: Vec::new(),
            verified_economic_receipts: Vec::new(),
            economic_idempotency_keys: BTreeSet::new(),
            economic_dead_letters: Vec::new(),
            pending_tradeable_purchases: Vec::new(),
            value_events: Vec::new(),
            wallet_reward_issued_by_day: BTreeMap::new(),
            reconciliation_cursor: 0,
            quest_chain: None,
            world_clock: WorldClock::default(),
            expedition_supplies: ExpeditionSupplyState::default(),
            selected_expedition_preparation: ExpeditionPreparation::Immediate,
            mentor_met: false,
            trained_with_mentor: false,
            quest_state: QuestState::Locked,
            pending_battle: None,
            settled_battle_ids: BTreeSet::new(),
            settlement_receipts: Vec::new(),
        }
    }
}

