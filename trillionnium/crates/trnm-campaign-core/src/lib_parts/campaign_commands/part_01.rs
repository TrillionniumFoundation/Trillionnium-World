impl CampaignSaveV1 {
    /// Executes one public campaign command against a private candidate and
    /// publishes it only after both the command and aggregate validation pass.
    /// Thus every `Err` return preserves the exact caller-visible state.
    fn apply_command_atomically<T>(
        &mut self,
        command: impl FnOnce(&mut Self) -> Result<T, CampaignError>,
    ) -> Result<T, CampaignError> {
        let mut candidate = self.clone();
        let output = command(&mut candidate)?;
        candidate.validate()?;
        *self = candidate;
        Ok(output)
    }

    fn cex_scoped_campaign_id(account_id: &str, campaign_id: &str) -> String {
        let digest = Sha256::digest(
            format!("trnm-cex-campaign-scope-v1\0{account_id}\0{campaign_id}").as_bytes(),
        );
        format!("cex-campaign-{digest:x}")
    }

    fn scoped_economic_intent_id(&self, intent_id: &str) -> String {
        let prefix = format!("{}:", self.campaign_id);
        if intent_id.starts_with(&prefix) {
            intent_id.to_string()
        } else {
            format!("{prefix}{intent_id}")
        }
    }

    pub fn ensure_gameplay_defaults(&mut self) {
        let previous_schema_revision = self.schema_revision;
        self.schema_revision = 12;
        if previous_schema_revision < 11
            && self.ending_epilogue_complete
            && self.ending_epilogue_progress == 3
        {
            self.ending_epilogue_progress = 4;
        }
        if self.active_regional_quest_id.is_none() {
            self.active_regional_quest_step = 0;
            self.active_regional_quest_runtime = None;
        }
        if self.conversation_history.len() > 24 {
            self.conversation_history = self
                .conversation_history
                .split_off(self.conversation_history.len() - 24);
        }
        if self.social_event_history.len() > 16 {
            self.social_event_history = self
                .social_event_history
                .split_off(self.social_event_history.len() - 16);
        }
        for memory in self.npc_memory.values_mut() {
            if memory.len() > 8 {
                *memory = memory.split_off(memory.len() - 8);
            }
        }
        self.main_story_decisions
            .sort_by_key(|decision| decision.chapter);
        self.main_story_decisions
            .dedup_by_key(|decision| decision.chapter);
        self.main_story_ending = resolve_main_story_ending(&self.main_story_decisions);
        self.main_story_chapter = MAIN_STORY_CHAPTERS
            .iter()
            .find(|chapter| {
                !self
                    .main_story_decisions
                    .iter()
                    .any(|decision| decision.chapter == chapter.chapter)
            })
            .map(|chapter| chapter.chapter)
            .unwrap_or(MainStoryChapter::ChapterComplete);
        self.selected_shop_item_index %= ECONOMY_ITEM_CATALOG.len();
        self.selected_recipe_index %= CRAFTING_RECIPES.len();
        if !self.character.inventory_items.is_empty() {
            self.selected_inventory_index %= self.character.inventory_items.len();
        }
        let defaults = Self::default();
        for member in defaults.party {
            if let Some(existing) = self
                .party
                .iter_mut()
                .find(|existing| existing.unit_id == member.unit_id)
            {
                for skill_id in member.skill_ids {
                    if !existing.skill_ids.contains(&skill_id) {
                        existing.skill_ids.push(skill_id);
                    }
                }
            } else {
                self.party.push(member);
            }
        }
        for npc in NPC_CATALOG {
            self.npc_relationships
                .entry(npc.id.to_string())
                .or_insert_with(|| NpcRelationship::new(npc.id, npc.faction_id));
        }
        for quest in REGIONAL_QUEST_CATALOG {
            self.regional_quest_states
                .entry(quest.id.to_string())
                .or_insert(QuestState::Available);
        }
        if self.progression.growth_points_awarded == 0
            && self.progression.growth_allocations.is_empty()
        {
            self.progression.growth_points_available = 1;
            self.progression.growth_points_awarded = 1;
        }
        for item in defaults.character.inventory_items {
            if !self
                .character
                .inventory_items
                .iter()
                .any(|existing| existing.item_id == item.item_id)
            {
                self.character.inventory_items.push(item);
            }
        }
        for (instance_id, condition) in character_item_conditions(&self.character) {
            self.item_conditions.entry(instance_id).or_insert(condition);
        }
        for item in ECONOMY_ITEM_CATALOG {
            self.market_stock
                .entry(item.id.to_string())
                .or_insert(if item.material { 12 } else { 4 });
            self.market_demand.entry(item.id.to_string()).or_insert(0);
        }
        let default_stock = default_regional_market_stock();
        let default_demand = default_regional_market_demand();
        for region in MARKET_REGION_IDS {
            let stock = self
                .regional_market_stock
                .entry(region.to_string())
                .or_default();
            let demand = self
                .regional_market_demand
                .entry(region.to_string())
                .or_default();
            for item in ECONOMY_ITEM_CATALOG {
                stock.entry(item.id.to_string()).or_insert_with(|| {
                    default_stock[region]
                        .get(item.id)
                        .copied()
                        .unwrap_or_default()
                });
                demand.entry(item.id.to_string()).or_insert_with(|| {
                    default_demand[region]
                        .get(item.id)
                        .copied()
                        .unwrap_or_default()
                });
            }
        }
        if let Some(mirror_stock) = self.regional_market_stock.get_mut("mirror_city") {
            for (item_id, stock) in &self.market_stock {
                mirror_stock.insert(item_id.clone(), *stock);
            }
        }
        if let Some(mirror_demand) = self.regional_market_demand.get_mut("mirror_city") {
            for (item_id, demand) in &self.market_demand {
                mirror_demand.insert(item_id.clone(), *demand);
            }
        }
        for caravan in &mut self.active_regional_caravans {
            if caravan.route_room_ids.is_empty() {
                caravan.route_room_ids =
                    Self::caravan_route(&caravan.from_region_id, &caravan.to_region_id);
            }
            caravan.route_index = caravan.route_index.min(caravan.route_room_ids.len() - 1);
            caravan.integrity = caravan.integrity.min(100);
            caravan.risk = caravan.risk.min(9);
        }
        if self.economy_mode == EconomyMode::CexConnected && self.economy_account_binding.is_none()
        {
            self.economy_mode = EconomyMode::OfflineLocal;
        }
        if let Some(binding) = &self.economy_account_binding {
            if self.wallet_snapshot.account_id.is_empty() {
                self.wallet_snapshot.account_id = binding.account_id.clone();
            }
            if !self.campaign_id.starts_with("cex-campaign-") {
                self.campaign_id =
                    Self::cex_scoped_campaign_id(&binding.account_id, &self.campaign_id);
            }
        }
        if self.pending_economic_intents.len() > 128 {
            self.pending_economic_intents.truncate(128);
        }
        if self.pending_economic_compensations.len() > 64 {
            self.pending_economic_compensations.truncate(64);
        }
        if self.verified_economic_receipts.len() > 256 {
            let keep_from = self.verified_economic_receipts.len() - 256;
            self.verified_economic_receipts.drain(..keep_from);
        }
        if self.economic_dead_letters.len() > 64 {
            let keep_from = self.economic_dead_letters.len() - 64;
            self.economic_dead_letters.drain(..keep_from);
        }
        self.economic_idempotency_keys.extend(
            self.pending_economic_intents
                .iter()
                .map(|intent| intent.idempotency_key.key.clone()),
        );
        self.economic_idempotency_keys.extend(
            self.pending_economic_compensations
                .iter()
                .map(|intent| intent.idempotency_key.key.clone()),
        );
        if self.value_events.len() > 256 {
            let keep_from = self.value_events.len() - 256;
            self.value_events.drain(..keep_from);
        }
        if self.wallet_reward_issued_by_day.len() > 400 {
            let first_day_to_keep = self
                .wallet_reward_issued_by_day
                .keys()
                .rev()
                .nth(399)
                .copied()
                .unwrap_or_default();
            self.wallet_reward_issued_by_day
                .retain(|day, _| *day >= first_day_to_keep);
        }
        if self.character.display_name.trim().is_empty() {
            self.apply_character_identity_name();
        }
        self.story.unlocked_room_ids.extend([
            MIRROR_SQUARE_ROOM.to_string(),
            MENTOR_HALL_ROOM.to_string(),
            CISTERN_WARD_ROOM.to_string(),
            NIGHT_WATCH_POST_ROOM.to_string(),
            WORKSHOP_GATE_ROOM.to_string(),
            MARKET_WIND_PAVILION_ROOM.to_string(),
            LANTERN_INFIRMARY_ROOM.to_string(),
            ARCHIVE_STEPS_ROOM.to_string(),
            CARAVAN_YARD_ROOM.to_string(),
        ]);
        if self.mentor_met {
            self.progression
                .world_flags
                .insert("expedition_gate_open".to_string());
            self.story
                .unlocked_room_ids
                .insert(EXPEDITION_GATE_ROOM.to_string());
        }
        if self.progression.world_flags.contains("signal_road_secured") {
            self.story
                .unlocked_room_ids
                .insert(RELAY_QUARTER_ROOM.to_string());
            self.story.current_step = StoryStepId::SignalRoadComplete;
        }
        if self
            .progression
            .world_flags
            .contains("glass_basin_wayhouse_open")
        {
            self.story.unlocked_room_ids.extend([
                GLASS_BASIN_WAYHOUSE_ROOM.to_string(),
                DEEP_RELAY_ROOM.to_string(),
                GLASS_REED_MARSH_ROOM.to_string(),
                BASIN_OBSERVATORY_ROOM.to_string(),
            ]);
        }
        if self.progression.world_flags.contains("ashen_fringe_open") {
            self.story.unlocked_room_ids.extend([
                MOON_BRIDGE_ROOM.to_string(),
                EMBER_ORCHARD_EDGE_ROOM.to_string(),
                ASH_BEACON_FIELD_ROOM.to_string(),
                CINDER_REFUGE_ROOM.to_string(),
            ]);
        }
        let mut effective_flags = self.progression.world_flags.clone();
        if self.active_title == Some(BuildTitle::RelayRunner) {
            effective_flags.insert("signal_road_secured".to_string());
        }
        if mirror_city_world_graph()
            .can_enter(self.room.id(), &effective_flags)
            .is_err()
        {
            self.room = CampaignRoom::MirrorSquare;
        }
    }

    pub fn validate(&self) -> Result<(), CampaignError> {
        if self.contract_version != CAMPAIGN_SAVE_CONTRACT {
            return Err(CampaignError::InvalidContract(
                self.contract_version.clone(),
            ));
        }
        if self.schema_revision != 12 {
            return Err(CampaignError::InvalidContract(format!(
                "unsupported campaign schema revision {}",
                self.schema_revision
            )));
        }
        if self.active_regional_quest_id.is_none() && self.active_regional_quest_step != 0 {
            return Err(CampaignError::InvalidState(
                "regional quest step exists without an active quest".to_string(),
            ));
        }
        if self.active_regional_quest_id.as_deref()
            != self
                .active_regional_quest_runtime
                .as_ref()
                .map(|runtime| runtime.quest_id.as_str())
        {
            return Err(CampaignError::InvalidState(
                "regional quest runtime does not match the active quest".to_string(),
            ));
        }
        if self.conversation_history.len() > 24 {
            return Err(CampaignError::InvalidState(
                "NPC conversation history exceeds its bounded save budget".to_string(),
            ));
        }
        if self.social_event_history.len() > 16
            || self.npc_memory.values().any(|memory| memory.len() > 8)
            || self
                .npc_bonds
                .values()
                .any(|bond| bond.unsigned_abs() > 100)
            || self.main_story_decisions.len() > 3
            || self
                .main_story_decisions
                .iter()
                .map(|decision| decision.chapter)
                .collect::<BTreeSet<_>>()
                .len()
                != self.main_story_decisions.len()
            || self.main_story_ending != resolve_main_story_ending(&self.main_story_decisions)
            || self
                .main_story_scene_progress
                .values()
                .any(|step| *step > 2)
            || self.ending_epilogue_progress > 4
            || self.ending_epilogue_complete != (self.ending_epilogue_progress >= 4)
        {
            return Err(CampaignError::InvalidState(
                "NPC social or main-story history is inconsistent or exceeds its bound".to_string(),
            ));
        }
        if self
            .technique_mastery
            .values()
            .any(|mastery| *mastery > 100)
        {
            return Err(CampaignError::InvalidState(
                "sect technique mastery exceeds its persistent cap".to_string(),
            ));
        }
        if ECONOMY_ITEM_CATALOG.iter().any(|item| {
            !self.market_stock.contains_key(item.id)
                || !self.market_demand.contains_key(item.id)
                || self.market_demand[item.id].unsigned_abs() > 20
        }) {
            return Err(CampaignError::InvalidState(
                "market stock/demand state is incomplete or out of bounds".to_string(),
            ));
        }
        if self.regional_logistics.len() > 64
            || self.active_regional_caravans.iter().any(|caravan| {
                caravan.route_room_ids.is_empty()
                    || caravan.route_index >= caravan.route_room_ids.len()
                    || caravan.integrity > 100
                    || caravan.risk > 9
            })
            || MARKET_REGION_IDS.into_iter().any(|region| {
                ECONOMY_ITEM_CATALOG.iter().any(|item| {
                    !self
                        .regional_market_stock
                        .get(region)
                        .is_some_and(|state| state.contains_key(item.id))
                        || !self
                            .regional_market_demand
                            .get(region)
                            .is_some_and(|state| {
                                state
                                    .get(item.id)
                                    .is_some_and(|demand| demand.unsigned_abs() <= 20)
                            })
                })
            })
        {
            return Err(CampaignError::InvalidState(
                "regional market or logistics state is incomplete or out of bounds".to_string(),
            ));
        }
        if self.pending_economic_intents.len() > 128
            || self.pending_economic_compensations.len() > 64
            || self.verified_economic_receipts.len() > 256
            || self.economic_dead_letters.len() > 64
            || self.pending_tradeable_purchases.len() > 32
            || self
                .pending_economic_intents
                .iter()
                .any(|intent| intent.validate().is_err())
            || self
                .pending_economic_compensations
                .iter()
                .any(|intent| intent.validate().is_err())
            || self.value_events.len() > 256
            || self.wallet_reward_issued_by_day.len() > 400
            || self
                .wallet_reward_issued_by_day
                .values()
                .any(|amount| *amount < 0 || *amount > BATTLE_WALLET_REWARD_DAILY_CAP)
            || self.value_events.iter().any(|event| {
                event.event_id.trim().is_empty()
                    || event.economic_intent_id.trim().is_empty()
                    || event.local_soft_credit_delta < 0
                    || event.wallet_credit_delta < 0
                    || match event.policy {
                        ValueSettlementPolicy::LocalSoftOnly => event.wallet_credit_delta != 0,
                        ValueSettlementPolicy::WalletOnly => event.local_soft_credit_delta != 0,
                        ValueSettlementPolicy::DualTrack => {
                            event.wallet_credit_delta > event.local_soft_credit_delta
                                || event.wallet_credit_delta > BATTLE_WALLET_REWARD_PER_EVENT_CAP
                        }
                    }
                    || event
                        .economic_receipt_id
                        .as_ref()
                        .is_some_and(|receipt_id| {
                            !self.verified_economic_receipts.iter().any(|receipt| {
                                receipt.receipt_id == *receipt_id
                                    && receipt.intent_id == event.economic_intent_id
                            })
                        })
            })
            || self.verified_economic_receipts.iter().any(|receipt| {
                receipt.protocol_version != TERM_EXCHANGE_PROTOCOL_VERSION
                    || receipt.progression_class != receipt.status.progression_class()
            })
            || self.pending_tradeable_purchases.iter().any(|purchase| {
                purchase.quantity == 0
                    || purchase.price_wallet_credits <= 0
                    || purchase.item_id.trim().is_empty()
                    || purchase.buyer.account_id.trim().is_empty()
                    || purchase.seller.account_id.trim().is_empty()
            })
            || self.settlement_receipts.iter().any(|settlement| {
                settlement
                    .economic_receipt_id
                    .as_ref()
                    .is_some_and(|receipt_id| {
                        let Some(intent_id) = settlement.economic_intent_id.as_deref() else {
                            return true;
                        };
                        !self.verified_economic_receipts.iter().any(|receipt| {
                            receipt.receipt_id == *receipt_id && receipt.intent_id == intent_id
                        })
                    })
            })
            || self.economy_mode == EconomyMode::CexConnected
                && (self.economy_account_binding.is_none()
                    || self.wallet_snapshot.available_credits < 0
                    || self.wallet_snapshot.reserved_credits < 0
                    || self
                        .economy_account_binding
                        .as_ref()
                        .is_some_and(|binding| {
                            self.wallet_snapshot.account_id != binding.account_id
                        }))
        {
            return Err(CampaignError::InvalidState(
                "economy outbox, account binding or receipt state is invalid".to_string(),
            ));
        }
        let linked_economic_intent_ids = self
            .settlement_receipts
            .iter()
            .filter_map(|settlement| settlement.economic_intent_id.as_deref())
            .collect::<BTreeSet<_>>();
        if linked_economic_intent_ids.len()
            != self
                .settlement_receipts
                .iter()
                .filter(|settlement| settlement.economic_intent_id.is_some())
                .count()
        {
            return Err(CampaignError::InvalidState(
                "battle settlement economic intent links must be one-to-one".to_string(),
            ));
        }
        if self.active_party_ids.len() != 4 {
            return Err(CampaignError::InvalidState(
                "exactly four active party members are required".to_string(),
            ));
        }
        let hero_name = self
            .party
            .iter()
            .find(|member| member.unit_id == "hero")
            .map(|member| member.display_name.as_str());
        if self.character.display_name.trim().is_empty()
            || self.character.display_name.len() > 32
            || hero_name != Some(self.character.display_name.as_str())
            || self.character.display_name != self.character_identity.name.display_name()
        {
            return Err(CampaignError::InvalidState(
                "character identity and persistent hero name disagree".to_string(),
            ));
        }
        if self.progression.credits < 0
            || self.progression.mentor_training_sessions > MAX_MENTOR_TRAINING_SESSIONS
        {
            return Err(CampaignError::InvalidState(
                "campaign credits or mentor training count is invalid".to_string(),
            ));
        }
        if self.world_clock.day == 0
            || self.world_clock.minute_of_day >= 24 * 60
            || self.expedition_supplies.stamina > 100
            || self.expedition_supplies.rations > 12
            || self.expedition_supplies.water > 16
        {
            return Err(CampaignError::InvalidState(
                "world clock or expedition supplies are invalid".to_string(),
            ));
        }
        if let Some(chain) = &self.quest_chain {
            if chain.complete && chain.current_node != QuestChainNodeId::ReliefComplete {
                return Err(CampaignError::InvalidState(
                    "completed quest chain is not at its terminal node".to_string(),
                ));
            }
            if chain.chosen_branch.is_some()
                && chain.current_node != QuestChainNodeId::ReliefComplete
            {
                return Err(CampaignError::InvalidState(
                    "quest branch is set before the terminal decision".to_string(),
                ));
            }
        }
        let spent_growth = self
            .progression
            .growth_allocations
            .values()
            .copied()
            .sum::<u16>();
        if spent_growth.saturating_add(self.progression.growth_points_available)
            != self.progression.growth_points_awarded
            || (self.pending_growth_stat.is_some() && self.progression.growth_points_available == 0)
        {
            return Err(CampaignError::InvalidState(
                "growth point accounting is inconsistent".to_string(),
            ));
        }
        let party_ids = self
            .party
            .iter()
            .map(|member| member.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let active = self
            .active_party_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let available = self
            .party
            .iter()
            .filter(|member| member.available)
            .map(|member| member.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        if party_ids.len() != self.party.len()
            || active.len() != self.active_party_ids.len()
            || !active.is_subset(&party_ids)
            || !active.is_subset(&available)
        {
            return Err(CampaignError::InvalidState(
                "party ids must be unique and active ids must exist".to_string(),
            ));
        }
        match (self.phase, self.pending_battle.as_ref()) {
            (CampaignPhase::Town, None) => {}
            (CampaignPhase::BattlePending, Some(pending)) if pending.result.is_none() => {
                pending.seed.validate()?;
            }
            (CampaignPhase::PostBattlePending, Some(pending)) if pending.result.is_some() => {
                pending.seed.validate()?;
                pending
                    .result
                    .as_ref()
                    .expect("guarded result")
                    .validate_against(&pending.seed)?;
            }
            _ => {
                return Err(CampaignError::InvalidState(
                    "campaign phase and pending battle disagree".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn apply_character_identity_name(&mut self) {
        let display_name = self.character_identity.name.display_name().to_string();
        self.character.display_name = display_name.clone();
        if let Some(hero) = self
            .party
            .iter_mut()
            .find(|member| member.unit_id == "hero")
        {
            hero.display_name = display_name;
        }
    }

    pub fn cycle_character_identity(&mut self) -> Result<CharacterNamePreset, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_character_identity_atomic_inner())
    }

    fn cycle_character_identity_atomic_inner(
        &mut self,
    ) -> Result<CharacterNamePreset, CampaignError> {
        self.require_town()?;
        if self.character_identity.confirmed {
            return Err(CampaignError::InvalidState(
                "confirmed character identity cannot be changed".to_string(),
            ));
        }
        self.character_identity.name = self.character_identity.name.next();
        self.apply_character_identity_name();
        self.revision += 1;
        Ok(self.character_identity.name)
    }

    pub fn confirm_character_identity(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.confirm_character_identity_atomic_inner()
        })
    }

    fn confirm_character_identity_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.character_identity.confirmed {
            return Err(CampaignError::InvalidState(
                "character identity is already confirmed".to_string(),
            ));
        }
        self.apply_character_identity_name();
        self.character_identity.confirmed = true;
        self.revision += 1;
        self.validate()
    }

    pub fn cycle_difficulty(&mut self) -> Result<CampaignDifficulty, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_difficulty_atomic_inner())
    }

    fn cycle_difficulty_atomic_inner(&mut self) -> Result<CampaignDifficulty, CampaignError> {
        self.require_town()?;
        if self.quest_state == QuestState::Accepted {
            return Err(CampaignError::InvalidState(
                "difficulty is locked after accepting a mission".to_string(),
            ));
        }
        self.difficulty = self.difficulty.next();
        self.revision += 1;
        Ok(self.difficulty)
    }

    pub fn current_guide_step(&self) -> CampaignGuideStep {
        if !self.mentor_met {
            CampaignGuideStep::MeetMentor
        } else if !self.trained_with_mentor {
            CampaignGuideStep::TrainWithMentor
        } else if !self.character.equipment_slots.contains_key("weapon") {
            CampaignGuideStep::EquipWeapon
        } else if self.room != CampaignRoom::ExpeditionGate {
            CampaignGuideStep::ReachExpeditionGate
        } else if self.quest_state == QuestState::Accepted {
            CampaignGuideStep::DeployMission
        } else if self
            .progression
            .world_flags
            .contains("mirror_siege_secured")
        {
            CampaignGuideStep::ReadJournal
        } else {
            CampaignGuideStep::AcceptMission
        }
    }

}

