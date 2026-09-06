impl CampaignSaveV1 {
    pub fn campaign_journal(&self) -> Vec<CampaignJournalEntry> {
        let signal_state = if self
            .progression
            .world_flags
            .contains("mirror_siege_secured")
        {
            CampaignJournalState::Completed
        } else {
            match self.quest_state {
                QuestState::Locked => CampaignJournalState::Locked,
                QuestState::Available => CampaignJournalState::Available,
                QuestState::Accepted => CampaignJournalState::Active,
                QuestState::Completed => CampaignJournalState::Active,
                QuestState::Failed | QuestState::Withdrawn => CampaignJournalState::Failed,
            }
        };
        let journal_mission = if self.quest_state == QuestState::Accepted {
            self.active_mission
        } else if !self
            .progression
            .world_flags
            .contains("first_contact_secured")
        {
            CampaignMission::FirstContact
        } else if self.progression.aftershock_completions == 0 {
            CampaignMission::AftershockPatrol
        } else if !self
            .progression
            .world_flags
            .contains("convoy_exodus_secured")
        {
            CampaignMission::ConvoyExodus
        } else if !self
            .progression
            .world_flags
            .contains("mirror_siege_secured")
        {
            CampaignMission::MirrorSiege
        } else {
            CampaignMission::AftershockPatrol
        };
        let signal_objective = match journal_mission {
            CampaignMission::FirstContact => "Secure the first relay contact",
            CampaignMission::AftershockPatrol => "Break the repeatable aftershock patrol",
            CampaignMission::ConvoyExodus => "Escort, defend and extract the signal convoy",
            CampaignMission::MirrorSiege => "Break the siege and reclaim Mirror Gate",
            CampaignMission::IronDeltaSkirmish => "Win the Iron Delta score skirmish",
            CampaignMission::NightWatchCrossingSkirmish => "Escort the Night Watch patrol",
            CampaignMission::GlassBasinSkirmish => "Control the Glass Basin relay array",
            CampaignMission::EmberOrchardSkirmish => "Break the Ember Orchard base",
            CampaignMission::SaltMarshSkirmish => "Control the Salt Marsh causeway",
            CampaignMission::CinderCrownSkirmish => "Break the Cinder Crown siege",
        };
        let cistern = match self.quest_chain.as_ref() {
            None => CampaignJournalEntry {
                id: CampaignJournalId::CisternRelief,
                title: "Signal Cistern Relief".to_string(),
                state: if self
                    .progression
                    .world_flags
                    .contains("outer_signal_road_open")
                {
                    CampaignJournalState::Available
                } else {
                    CampaignJournalState::Locked
                },
                objective: "Open the outer Signal Road".to_string(),
                next_room: Some(CampaignRoom::RelayQuarter),
            },
            Some(progress) => CampaignJournalEntry {
                id: CampaignJournalId::CisternRelief,
                title: "Signal Cistern Relief".to_string(),
                state: if progress.complete {
                    CampaignJournalState::Completed
                } else {
                    CampaignJournalState::Active
                },
                objective: match progress.current_node {
                    QuestChainNodeId::SurveyDamage => "Survey the damaged cistern",
                    QuestChainNodeId::GatherSupplies => "Commit 40 credits of relief supplies",
                    QuestChainNodeId::ChooseReliefPlan => "Choose reinforce or evacuate",
                    QuestChainNodeId::ReliefComplete => "Relief plan completed",
                }
                .to_string(),
                next_room: match progress.current_node {
                    QuestChainNodeId::GatherSupplies => Some(CampaignRoom::ExpeditionGate),
                    _ => Some(CampaignRoom::RelayQuarter),
                },
            },
        };
        let mastery_state = if self.active_title.is_some() {
            CampaignJournalState::Completed
        } else if self.build_path == BuildPath::Unformed {
            CampaignJournalState::Locked
        } else {
            CampaignJournalState::Active
        };
        vec![
            CampaignJournalEntry {
                id: CampaignJournalId::SignalRoad,
                title: "Signal Road Campaign".to_string(),
                state: signal_state,
                objective: signal_objective.to_string(),
                next_room: Some(CampaignRoom::ExpeditionGate),
            },
            cistern,
            CampaignJournalEntry {
                id: CampaignJournalId::Mastery,
                title: "Path Mastery".to_string(),
                state: mastery_state,
                objective: if self.active_title.is_some() {
                    "Mastery title earned".to_string()
                } else {
                    "Choose growth, then complete the mentor challenge".to_string()
                },
                next_room: Some(CampaignRoom::MentorHall),
            },
        ]
    }

    pub fn move_to(&mut self, room: CampaignRoom) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| candidate.move_to_atomic_inner(room))
    }

    fn move_to_atomic_inner(&mut self, room: CampaignRoom) -> Result<(), CampaignError> {
        self.require_town()?;
        let mut effective_flags = self.progression.world_flags.clone();
        if self.active_title == Some(BuildTitle::RelayRunner) {
            effective_flags.insert("signal_road_secured".to_string());
        }
        mirror_city_world_graph()
            .transition(self.room.id(), room.id(), &effective_flags)
            .map_err(CampaignError::InvalidState)?;
        self.room = room;
        self.revision += 1;
        Ok(())
    }

    pub fn current_task_route_plan(&self) -> WorldRoutePlan {
        if let Some(quest_id) = self.active_regional_quest_id.as_deref() {
            if REGIONAL_QUEST_CATALOG
                .iter()
                .find(|definition| definition.id == quest_id)
                .is_some()
            {
                let remaining = self.active_regional_quest_ready_rooms();
                let mut flags = self.progression.world_flags.clone();
                if self.active_title == Some(BuildTitle::RelayRunner) {
                    flags.insert("signal_road_secured".to_string());
                }
                if let Some(route) = remaining
                    .iter()
                    .map(|room| {
                        mirror_city_world_graph().shortest_route(self.room.id(), room, &flags)
                    })
                    .filter(|route| route.reachable())
                    .min_by_key(|route| route.path.len())
                {
                    return route;
                }
                if remaining.is_empty() {
                    let runtime = self
                        .active_regional_quest_runtime
                        .as_ref()
                        .expect("active regional quest has runtime");
                    let graph = quest_condition_graph(
                        REGIONAL_QUEST_CATALOG
                            .iter()
                            .find(|definition| definition.id == quest_id)
                            .expect("active quest remains catalog bound"),
                        runtime.approach,
                    );
                    if let Some(settlement) = graph.nodes.iter().find(|node| {
                        node.kind == trnm_rpg_core::QuestConditionKind::ReturnForSettlement
                    }) {
                        return mirror_city_world_graph().shortest_route(
                            self.room.id(),
                            &settlement.subject_id,
                            &flags,
                        );
                    }
                }
                return mirror_city_world_graph().ordered_task_route(
                    self.room.id(),
                    &remaining,
                    &flags,
                );
            }
        }
        let destination = self
            .quest_chain
            .as_ref()
            .filter(|chain| !chain.complete)
            .map(|chain| match chain.current_node {
                QuestChainNodeId::SurveyDamage | QuestChainNodeId::ChooseReliefPlan => {
                    RELAY_QUARTER_ROOM
                }
                QuestChainNodeId::GatherSupplies => EXPEDITION_GATE_ROOM,
                QuestChainNodeId::ReliefComplete => RELAY_QUARTER_ROOM,
            })
            .unwrap_or_else(|| match self.story.current_step {
                StoryStepId::MeetMentor => MENTOR_HALL_ROOM,
                StoryStepId::SecureFirstContact
                | StoryStepId::BreakAftershock
                | StoryStepId::EvacuateConvoy => EXPEDITION_GATE_ROOM,
                StoryStepId::SignalRoadComplete => RELAY_QUARTER_ROOM,
            });
        let mut flags = self.progression.world_flags.clone();
        if self.active_title == Some(BuildTitle::RelayRunner) {
            flags.insert("signal_road_secured".to_string());
        }
        mirror_city_world_graph().shortest_route(self.room.id(), destination, &flags)
    }

    /// Walk one legal world edge toward the nearest currently-ready task
    /// node. This is the same authority used by manual room movement and is
    /// exposed to the client as an accessible route-follow command.
    pub fn advance_toward_current_task(&mut self) -> Result<CampaignRoom, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.advance_toward_current_task_atomic_inner()
        })
    }

    fn advance_toward_current_task_atomic_inner(&mut self) -> Result<CampaignRoom, CampaignError> {
        let route = self.current_task_route_plan();
        let next = route.path.get(1).ok_or_else(|| {
            CampaignError::InvalidState(
                route
                    .blocked_reason
                    .map(|reason| format!("task route blocked: {reason:?}"))
                    .unwrap_or_else(|| "already at the current task node".to_string()),
            )
        })?;
        let room = CampaignRoom::from_id(next).ok_or_else(|| {
            CampaignError::InvalidState(format!("task route returned unknown room {next}"))
        })?;
        self.move_to(room)?;
        Ok(room)
    }

    pub fn start_cistern_relief(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.start_cistern_relief_atomic_inner())
    }

    fn start_cistern_relief_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::RelayQuarter)?;
        if !self
            .progression
            .world_flags
            .contains("outer_signal_road_open")
        {
            return Err(CampaignError::InvalidState(
                "the outer signal road must be open before cistern relief".to_string(),
            ));
        }
        if self
            .quest_chain
            .as_ref()
            .is_some_and(|chain| !chain.complete)
        {
            return Err(CampaignError::InvalidState(
                "another quest-chain step is already active".to_string(),
            ));
        }
        self.quest_chain = Some(QuestChainProgress {
            id: QuestChainId::CisternRelief,
            current_node: QuestChainNodeId::SurveyDamage,
            chosen_branch: None,
            completed_nodes: BTreeSet::new(),
            complete: false,
        });
        self.revision += 1;
        Ok(())
    }

    pub fn advance_cistern_relief(&mut self) -> Result<QuestChainNodeId, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.advance_cistern_relief_atomic_inner())
    }

    fn advance_cistern_relief_atomic_inner(&mut self) -> Result<QuestChainNodeId, CampaignError> {
        self.require_town()?;
        let current = self
            .quest_chain
            .as_ref()
            .filter(|chain| !chain.complete)
            .map(|chain| chain.current_node)
            .ok_or_else(|| {
                CampaignError::InvalidState("cistern relief is not active".to_string())
            })?;
        let required_room = match current {
            QuestChainNodeId::SurveyDamage | QuestChainNodeId::ChooseReliefPlan => {
                CampaignRoom::RelayQuarter
            }
            QuestChainNodeId::GatherSupplies => CampaignRoom::ExpeditionGate,
            QuestChainNodeId::ReliefComplete => CampaignRoom::RelayQuarter,
        };
        if self.room != required_room {
            return Err(CampaignError::InvalidState(format!(
                "cistern relief step requires {}",
                required_room.title()
            )));
        }
        let next = match current {
            QuestChainNodeId::SurveyDamage => QuestChainNodeId::GatherSupplies,
            QuestChainNodeId::GatherSupplies => {
                if self.progression.credits < 40 {
                    return Err(CampaignError::InvalidState(
                        "gathering cistern supplies costs 40 credits".to_string(),
                    ));
                }
                self.progression.credits -= 40;
                QuestChainNodeId::ChooseReliefPlan
            }
            QuestChainNodeId::ChooseReliefPlan => {
                return Err(CampaignError::InvalidState(
                    "choose reinforce or evacuate to complete cistern relief".to_string(),
                ));
            }
            QuestChainNodeId::ReliefComplete => {
                return Err(CampaignError::InvalidState(
                    "cistern relief is already complete".to_string(),
                ));
            }
        };
        let chain = self.quest_chain.as_mut().expect("active chain was checked");
        chain.completed_nodes.insert(current);
        chain.current_node = next;
        self.revision += 1;
        Ok(next)
    }

    pub fn choose_cistern_relief_branch(
        &mut self,
        branch: QuestBranch,
    ) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.choose_cistern_relief_branch_atomic_inner(branch)
        })
    }

    fn choose_cistern_relief_branch_atomic_inner(
        &mut self,
        branch: QuestBranch,
    ) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::RelayQuarter)?;
        let chain = self
            .quest_chain
            .as_ref()
            .filter(|chain| {
                !chain.complete && chain.current_node == QuestChainNodeId::ChooseReliefPlan
            })
            .ok_or_else(|| {
                CampaignError::InvalidState("cistern relief is not awaiting a branch".to_string())
            })?;
        if chain.chosen_branch.is_some() {
            return Err(CampaignError::InvalidState(
                "cistern relief branch has already been chosen".to_string(),
            ));
        }
        let definition = cistern_relief_quest_chain_definition();
        let rewards = definition
            .nodes
            .iter()
            .find(|node| node.id == QuestChainNodeId::ReliefComplete && node.branch == Some(branch))
            .map(|node| node.rewards.clone())
            .ok_or_else(|| {
                CampaignError::InvalidState("missing quest branch rewards".to_string())
            })?;
        let mut local_soft_credit_reward = 0_i64;
        for reward in rewards {
            match reward {
                QuestChainReward::Credits { amount } => {
                    self.progression.credits = self.progression.credits.saturating_add(amount);
                    local_soft_credit_reward = local_soft_credit_reward.saturating_add(amount);
                }
                QuestChainReward::Reputation { amount } => {
                    self.character.attributes.reputation =
                        self.character.attributes.reputation.saturating_add(amount);
                }
                QuestChainReward::WorldFlag { flag } => {
                    self.progression.world_flags.insert(flag);
                }
                QuestChainReward::RelationshipTrust { npc_id, amount } => {
                    let relationship = self
                        .npc_relationships
                        .entry(npc_id.clone())
                        .or_insert_with(|| NpcRelationship::new(npc_id, "relay-quarter"));
                    relationship.trust = relationship.trust.saturating_add(amount);
                }
            }
        }
        let chain = self.quest_chain.as_mut().expect("active chain was checked");
        chain
            .completed_nodes
            .insert(QuestChainNodeId::ChooseReliefPlan);
        chain
            .completed_nodes
            .insert(QuestChainNodeId::ReliefComplete);
        chain.current_node = QuestChainNodeId::ReliefComplete;
        chain.chosen_branch = Some(branch);
        chain.complete = true;
        self.record_value_event(
            format!("quest-chain:cistern-relief:{branch:?}").to_ascii_lowercase(),
            format!("quest-contract:cistern-relief:{branch:?}").to_ascii_lowercase(),
            ValueEventSource::RegionalQuest,
            ValueSettlementPolicy::LocalSoftOnly,
            local_soft_credit_reward,
        )?;
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_expedition_preparation(&mut self) -> Result<ExpeditionPreparation, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.cycle_expedition_preparation_atomic_inner()
        })
    }

    fn cycle_expedition_preparation_atomic_inner(
        &mut self,
    ) -> Result<ExpeditionPreparation, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        self.selected_expedition_preparation = self.selected_expedition_preparation.next();
        self.revision += 1;
        Ok(self.selected_expedition_preparation)
    }

    fn commit_expedition_preparation(&mut self) -> Result<ExpeditionReadiness, CampaignError> {
        let preparation = self.selected_expedition_preparation;
        let mut starting_resources = 0;
        let travel_minutes = match preparation {
            ExpeditionPreparation::Immediate => {
                require_supplies(&self.expedition_supplies, 1, 2)?;
                self.expedition_supplies.stamina =
                    self.expedition_supplies.stamina.saturating_sub(20);
                self.expedition_supplies.rations -= 1;
                self.expedition_supplies.water -= 2;
                20
            }
            ExpeditionPreparation::Rested => {
                if self.progression.credits < 10 {
                    return Err(CampaignError::InvalidState(
                        "resting before departure costs 10 credits".to_string(),
                    ));
                }
                require_supplies(&self.expedition_supplies, 1, 1)?;
                self.progression.credits -= 10;
                self.expedition_supplies.stamina = 100;
                self.expedition_supplies.rations -= 1;
                self.expedition_supplies.water -= 1;
                180
            }
            ExpeditionPreparation::Supplied => {
                if self.progression.credits < 25 {
                    return Err(CampaignError::InvalidState(
                        "stocking the expedition costs 25 credits".to_string(),
                    ));
                }
                self.progression.credits -= 25;
                self.expedition_supplies.rations =
                    self.expedition_supplies.rations.saturating_add(3).min(12);
                self.expedition_supplies.water =
                    self.expedition_supplies.water.saturating_add(4).min(16);
                self.expedition_supplies.stamina =
                    self.expedition_supplies.stamina.saturating_sub(5);
                starting_resources = 50;
                35
            }
            ExpeditionPreparation::Shortcut => {
                if self.character_origin != CharacterOrigin::Scout
                    && self.active_title != Some(BuildTitle::RelayRunner)
                {
                    return Err(CampaignError::InvalidState(
                        "the shortcut requires Scout origin or Relay Runner title".to_string(),
                    ));
                }
                require_supplies(&self.expedition_supplies, 1, 2)?;
                self.expedition_supplies.stamina =
                    self.expedition_supplies.stamina.saturating_sub(10);
                self.expedition_supplies.rations -= 1;
                self.expedition_supplies.water -= 2;
                starting_resources = 20;
                10
            }
        };
        self.world_clock.advance(travel_minutes);
        Ok(ExpeditionReadiness {
            preparation,
            stamina: self.expedition_supplies.stamina,
            rations: self.expedition_supplies.rations,
            water: self.expedition_supplies.water,
            starting_resources,
            travel_minutes,
        })
    }

    pub fn cycle_character_origin(&mut self) -> Result<CharacterOrigin, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_character_origin_atomic_inner())
    }

    fn cycle_character_origin_atomic_inner(&mut self) -> Result<CharacterOrigin, CampaignError> {
        self.require_town()?;
        if self.mentor_met
            || self.progression.mentor_training_sessions > 0
            || !self.progression.growth_allocations.is_empty()
        {
            return Err(CampaignError::InvalidState(
                "character origin is fixed after mentor progression begins".to_string(),
            ));
        }
        let previous = self.character_origin;
        let next = previous.next();
        remove_origin_bonus(previous, &mut self.character.attributes);
        next.apply(&mut self.character.attributes);
        self.character_origin = next;
        self.character.skill_ids.retain(|skill| {
            !["iron_guard", "relay_overcharge", "wind_step"].contains(&skill.as_str())
        });
        self.character
            .skill_ids
            .push(next.starter_skill().to_string());
        if let Some(hero) = self
            .party
            .iter_mut()
            .find(|member| member.unit_id == "hero")
        {
            hero.attributes = self.character.attributes.clone();
            hero.skill_ids = self.character.skill_ids.clone();
        }
        self.revision += 1;
        Ok(next)
    }

    pub fn talk_to_mentor(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.talk_to_mentor_atomic_inner())
    }

    fn talk_to_mentor_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::MentorHall)?;
        self.mentor_met = true;
        self.faction_rank = self.faction_rank.max(FactionRank::Initiate);
        self.npc_relationships
            .entry("street-compass-sifu".to_string())
            .or_insert_with(|| NpcRelationship::new("street-compass-sifu", "signal-road-school"))
            .apply(RelationshipAction::Talk);
        if self.quest_state == QuestState::Locked {
            self.quest_state = QuestState::Available;
        }
        self.complete_story_step(StoryStepId::MeetMentor, StoryStepId::SecureFirstContact)?;
        self.revision += 1;
        Ok(())
    }

    pub fn train_with_mentor(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.train_with_mentor_atomic_inner())
    }

    fn train_with_mentor_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::MentorHall)?;
        if !self.mentor_met {
            return Err(CampaignError::InvalidState(
                "talk to the mentor before training".to_string(),
            ));
        }
        if self.progression.mentor_training_sessions >= MAX_MENTOR_TRAINING_SESSIONS {
            return Err(CampaignError::InvalidState(
                "mentor training cap reached; commit to the skills already learned".to_string(),
            ));
        }
        let session = self.progression.mentor_training_sessions;
        let cost = 50 + i64::from(session) * 40;
        if self.progression.credits < cost {
            return Err(CampaignError::InvalidState(format!(
                "mentor training costs {cost} credits"
            )));
        }
        self.progression.credits -= cost;
        self.progression.mentor_training_sessions += 1;
        self.trained_with_mentor = true;
        self.npc_relationships
            .get_mut("street-compass-sifu")
            .expect("mentor relationship exists")
            .apply(RelationshipAction::Train);
        let skill_id = self.selected_training_path.skill_id().to_string();
        if !self.character.skill_ids.contains(&skill_id) {
            self.character.skill_ids.push(skill_id.clone());
        }
        let progress = self
            .progression
            .skill_progress
            .entry(skill_id)
            .or_insert(SkillProgress {
                rank: 0,
                experience: 0,
            });
        progress.experience += 125;
        progress.rank = (1 + progress.experience / 250) as u16;
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_training_path(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_training_path_atomic_inner())
    }

    fn cycle_training_path_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::MentorHall)?;
        self.selected_training_path = self.selected_training_path.next();
        self.revision += 1;
        Ok(())
    }

    pub fn preview_growth_allocation(&mut self, stat: GrowthStat) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.preview_growth_allocation_atomic_inner(stat)
        })
    }

    fn preview_growth_allocation_atomic_inner(
        &mut self,
        stat: GrowthStat,
    ) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.progression.growth_points_available == 0 {
            return Err(CampaignError::InvalidState(
                "no growth points are available".to_string(),
            ));
        }
        self.pending_growth_stat = Some(stat);
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_growth_preview(&mut self) -> Result<GrowthStat, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_growth_preview_atomic_inner())
    }

    fn cycle_growth_preview_atomic_inner(&mut self) -> Result<GrowthStat, CampaignError> {
        let next = self.pending_growth_stat.unwrap_or_default().next();
        self.preview_growth_allocation(next)?;
        Ok(next)
    }

    pub fn cancel_growth_allocation(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cancel_growth_allocation_atomic_inner())
    }

    fn cancel_growth_allocation_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.pending_growth_stat.take().is_none() {
            return Err(CampaignError::InvalidState(
                "no growth allocation is awaiting confirmation".to_string(),
            ));
        }
        self.revision += 1;
        Ok(())
    }

    pub fn confirm_growth_allocation(&mut self) -> Result<GrowthStat, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.confirm_growth_allocation_atomic_inner()
        })
    }

    fn confirm_growth_allocation_atomic_inner(&mut self) -> Result<GrowthStat, CampaignError> {
        self.require_town()?;
        let stat = self.pending_growth_stat.take().ok_or_else(|| {
            CampaignError::InvalidState(
                "preview a growth allocation before confirming it".to_string(),
            )
        })?;
        if self.progression.growth_points_available == 0 {
            return Err(CampaignError::InvalidState(
                "growth point was already consumed".to_string(),
            ));
        }
        stat.apply(&mut self.character.attributes, 1);
        if let Some(hero) = self
            .party
            .iter_mut()
            .find(|member| member.unit_id == "hero")
        {
            hero.attributes = self.character.attributes.clone();
        }
        self.progression.growth_points_available -= 1;
        *self.progression.growth_allocations.entry(stat).or_default() += 1;
        let path = match stat {
            GrowthStat::Force | GrowthStat::Physique | GrowthStat::Resolve => BuildPath::Vanguard,
            GrowthStat::Agility | GrowthStat::Insight => BuildPath::Windrunner,
            GrowthStat::Craft | GrowthStat::Commerce => BuildPath::Artificer,
        };
        self.build_path = path;
        self.active_title = None;
        self.character.title = format!("{} Aspirant", path.display_name());
        self.progression.world_flags.insert(format!(
            "{}_path_chosen",
            path.display_name().to_ascii_lowercase()
        ));
        self.revision += 1;
        self.validate()?;
        Ok(stat)
    }

    pub fn attempt_mastery_challenge(&mut self) -> Result<BuildTitle, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.attempt_mastery_challenge_atomic_inner()
        })
    }

    fn attempt_mastery_challenge_atomic_inner(&mut self) -> Result<BuildTitle, CampaignError> {
        self.require_room(CampaignRoom::MentorHall)?;
        let challenge = MasteryChallenge::for_path(self.build_path).ok_or_else(|| {
            CampaignError::InvalidState("choose a growth path before mastery".to_string())
        })?;
        if !self.trained_with_mentor {
            return Err(CampaignError::InvalidState(
                "complete mentor training before the mastery challenge".to_string(),
            ));
        }
        let success = match challenge {
            MasteryChallenge::VanguardStand => {
                self.spar_with_mentor()?.outcome == SparringOutcome::Victory
            }
            MasteryChallenge::WindrunnerCircuit => self.character.attributes.agility >= 13,
            MasteryChallenge::ArtificerCommission => {
                if self.progression.credits < 25 {
                    false
                } else {
                    self.progression.credits -= 25;
                    self.character.attributes.craft >= 11
                }
            }
        };
        if !success {
            return Err(CampaignError::InvalidState(
                "mastery challenge requirements were not met".to_string(),
            ));
        }
        let title = challenge.title();
        self.unlocked_titles.insert(title);
        self.active_title = Some(title);
        self.character.title = title.display_name().to_string();
        let flag = match title {
            BuildTitle::GateWarden => "gate_warden_route",
            BuildTitle::RelayRunner => "relay_runner_shortcut",
            BuildTitle::ForgeMaster => "forge_master_prices",
        };
        self.progression.world_flags.insert(flag.to_string());
        if title == BuildTitle::RelayRunner {
            self.story
                .unlocked_room_ids
                .insert(RELAY_QUARTER_ROOM.to_string());
        }
        self.revision += 1;
        Ok(title)
    }

}

