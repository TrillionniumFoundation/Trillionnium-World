impl CampaignSaveV1 {
    fn start_regional_quest_atomic_inner(&mut self, quest_id: &str) -> Result<(), CampaignError> {
        self.require_town()?;
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)
            .ok_or_else(|| {
                CampaignError::InvalidState(format!("unknown regional quest: {quest_id}"))
            })?;
        let giver = NPC_CATALOG
            .iter()
            .find(|npc| npc.id == definition.giver_npc_id)
            .expect("quest giver is catalog validated");
        if self.room.id() != giver.room_id {
            return Err(CampaignError::InvalidState(format!(
                "{} is offered in {}",
                definition.title, giver.room_id
            )));
        }
        if self.active_regional_quest_id.is_some() {
            return Err(CampaignError::InvalidState(
                "finish the active regional quest before accepting another".to_string(),
            ));
        }
        if self
            .npc_relationships
            .get(giver.id)
            .is_none_or(|relationship| relationship.interactions == 0)
        {
            return Err(CampaignError::InvalidState(format!(
                "talk to {} before accepting {}",
                giver.display_name, definition.title
            )));
        }
        if !matches!(
            self.regional_quest_states.get(quest_id),
            Some(QuestState::Available | QuestState::Failed | QuestState::Withdrawn)
        ) {
            return Err(CampaignError::InvalidState(format!(
                "{} is not available for acceptance",
                definition.title
            )));
        }
        self.regional_quest_states
            .insert(quest_id.to_string(), QuestState::Accepted);
        self.active_regional_quest_id = Some(quest_id.to_string());
        self.active_regional_quest_step = 0;
        let failures = self
            .regional_quest_failure_counts
            .get(quest_id)
            .copied()
            .unwrap_or_default();
        let rule = quest_runtime_rule(definition.archetype);
        self.active_regional_quest_runtime = Some(RegionalQuestRuntime::new(
            quest_id,
            self.world_clock.day,
            rule.deadline_days,
            failures,
        ));
        self.revision += 1;
        Ok(())
    }

    pub fn start_first_regional_quest_here(&mut self) -> Result<String, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.start_first_regional_quest_here_atomic_inner()
        })
    }

    fn start_first_regional_quest_here_atomic_inner(&mut self) -> Result<String, CampaignError> {
        let quest_id = NPC_CATALOG
            .iter()
            .filter(|npc| npc.room_id == self.room.id())
            .flat_map(|npc| npc.task_ids.iter().copied())
            .find(|quest_id| {
                matches!(
                    self.regional_quest_states.get(*quest_id),
                    Some(QuestState::Available | QuestState::Failed | QuestState::Withdrawn)
                )
            })
            .ok_or_else(|| {
                CampaignError::InvalidState("no available regional quest in this room".to_string())
            })?
            .to_string();
        self.start_regional_quest(&quest_id)?;
        Ok(quest_id)
    }

    pub fn advance_active_regional_quest(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.advance_active_regional_quest_atomic_inner()
        })
    }

    fn advance_active_regional_quest_atomic_inner(&mut self) -> Result<(), CampaignError> {
        let quest_id = self.active_regional_quest_id.clone().ok_or_else(|| {
            CampaignError::InvalidState("no regional quest is active".to_string())
        })?;
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)
            .expect("active regional quest remains catalog bound");
        if self
            .active_regional_quest_runtime
            .as_ref()
            .is_some_and(|runtime| self.world_clock.day > runtime.deadline_day)
        {
            return self.fail_active_regional_quest("the quest deadline expired");
        }
        let ready_rooms = self.active_regional_quest_ready_rooms();
        if !ready_rooms.is_empty() {
            let runtime = self.active_regional_quest_runtime.as_mut().ok_or_else(|| {
                CampaignError::InvalidState("regional quest runtime is missing".to_string())
            })?;
            let condition_graph = quest_condition_graph(definition, runtime.approach);
            let node = condition_graph
                .nodes
                .iter()
                .filter(|node| node.kind == trnm_rpg_core::QuestConditionKind::VisitWaypoint)
                .find(|node| {
                    node.subject_id == self.room.id()
                        && !runtime.completed_condition_node_ids.contains(&node.id)
                        && quest_graph_node_ready(
                            &condition_graph,
                            &node.id,
                            &runtime.completed_condition_node_ids,
                        )
                });
            let Some(node) = node else {
                return Err(CampaignError::InvalidState(format!(
                    "{} requires one of the currently ready authored nodes [{}], current room is {}",
                    definition.title,
                    ready_rooms.join(" / "),
                    self.room.id(),
                )));
            };
            let node_id = node.id.clone();
            let consequence_flag = node.consequence_flag.clone();
            runtime.completed_condition_node_ids.insert(node_id);
            self.active_regional_quest_step = runtime
                .completed_condition_node_ids
                .iter()
                .filter(|node_id| node_id.contains("_waypoint_"))
                .count();
            runtime.evidence_count = runtime.evidence_count.saturating_add(1);
            if let Some(flag) = consequence_flag {
                self.progression.world_flags.insert(flag);
            }
            self.revision += 1;
            return Ok(());
        }
        if self
            .active_regional_quest_runtime
            .as_ref()
            .is_some_and(|runtime| runtime.approach == QuestApproach::Direct)
        {
            if let Some(encounter_id) = definition.encounter_id {
                let cleared = format!("{encounter_id}_cleared");
                if !self.progression.world_flags.contains(&cleared) {
                    return self.begin_regional_encounter(encounter_id);
                }
            }
        }
        self.complete_regional_quest()
    }

    pub fn complete_regional_quest(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| candidate.complete_regional_quest_atomic_inner())
    }

    fn complete_regional_quest_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        let quest_id = self.active_regional_quest_id.clone().ok_or_else(|| {
            CampaignError::InvalidState("no regional quest is active".to_string())
        })?;
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)
            .expect("active regional quest remains catalog bound");
        let runtime = self.active_regional_quest_runtime.clone().ok_or_else(|| {
            CampaignError::InvalidState("regional quest runtime is missing".to_string())
        })?;
        let condition_graph = quest_condition_graph(definition, runtime.approach);
        let unfinished_waypoints = condition_graph
            .nodes
            .iter()
            .filter(|node| node.kind == trnm_rpg_core::QuestConditionKind::VisitWaypoint)
            .filter(|node| {
                !quest_route_node_satisfied(
                    &condition_graph,
                    node,
                    &runtime.completed_condition_node_ids,
                )
            })
            .map(|node| node.subject_id.clone())
            .collect::<Vec<_>>();
        let settlement_room = condition_graph
            .nodes
            .iter()
            .find(|node| node.kind == trnm_rpg_core::QuestConditionKind::ReturnForSettlement)
            .map(|node| node.subject_id.as_str())
            .unwrap_or("unknown");
        if !unfinished_waypoints.is_empty() || settlement_room != self.room.id() {
            return Err(CampaignError::InvalidState(format!(
                "{} still has authored nodes [{}] before settlement in {}",
                definition.title,
                unfinished_waypoints.join(" / "),
                settlement_room,
            )));
        }
        let required_plain = condition_graph
            .nodes
            .iter()
            .filter(|node| node.kind == trnm_rpg_core::QuestConditionKind::VisitWaypoint)
            .filter(|node| !node.optional && node.exclusive_group.is_none())
            .count();
        let required_groups = condition_graph
            .nodes
            .iter()
            .filter(|node| node.kind == trnm_rpg_core::QuestConditionKind::VisitWaypoint)
            .filter_map(|node| node.exclusive_group.as_deref())
            .collect::<BTreeSet<_>>()
            .len();
        let required_evidence = required_plain + required_groups;
        if usize::from(runtime.evidence_count) < required_evidence {
            return Err(CampaignError::InvalidState(format!(
                "{} lacks authored route evidence {}/{}",
                definition.title, runtime.evidence_count, required_evidence
            )));
        }
        let rule = quest_runtime_rule(definition.archetype);
        if runtime.approach == QuestApproach::Diplomatic {
            let trust = self
                .npc_relationships
                .get(definition.giver_npc_id)
                .map(|relationship| relationship.trust)
                .unwrap_or_default();
            if trust < rule.minimum_trust_for_diplomacy {
                return Err(CampaignError::InvalidState(format!(
                    "diplomatic resolution requires trust {} with {}",
                    rule.minimum_trust_for_diplomacy, definition.giver_npc_id
                )));
            }
        }
        if runtime.approach == QuestApproach::Resourceful {
            consume_loot(
                &mut self.progression.inventory,
                rule.resource_item_id,
                rule.resource_quantity,
            )?;
        }
        if runtime.approach == QuestApproach::Direct {
            if let Some(encounter_id) = definition.encounter_id {
                let flag = format!("{encounter_id}_cleared");
                if !self.progression.world_flags.contains(&flag) {
                    return Err(CampaignError::InvalidState(format!(
                        "{} requires winning encounter {}",
                        definition.title, encounter_id
                    )));
                }
            }
        }
        let mut completed_condition_node_ids = runtime.completed_condition_node_ids.clone();
        let branch_node_id = match runtime.approach {
            QuestApproach::Direct => definition
                .encounter_id
                .map(|_| format!("{}_encounter", definition.id)),
            QuestApproach::Diplomatic => Some(format!("{}_trust", definition.id)),
            QuestApproach::Resourceful => Some(format!("{}_resource", definition.id)),
        };
        if let Some(branch_node_id) = branch_node_id {
            if !quest_graph_node_ready(
                &condition_graph,
                &branch_node_id,
                &completed_condition_node_ids,
            ) {
                return Err(CampaignError::InvalidState(format!(
                    "{} branch condition {} is blocked by its authored prerequisites",
                    definition.title, branch_node_id
                )));
            }
            completed_condition_node_ids.insert(branch_node_id);
        }
        let settlement_node_id = format!("{}_settlement", definition.id);
        if !quest_graph_node_ready(
            &condition_graph,
            &settlement_node_id,
            &completed_condition_node_ids,
        ) {
            let incoming = condition_graph
                .edges
                .iter()
                .filter(|edge| edge.to == settlement_node_id)
                .map(|edge| edge.from.as_str())
                .collect::<Vec<_>>();
            return Err(CampaignError::InvalidState(format!(
                "{} settlement is blocked by unfinished authored graph branches; incoming={incoming:?}; completed={completed_condition_node_ids:?}",
                definition.title,
            )));
        }
        completed_condition_node_ids.insert(settlement_node_id);
        let (credit_bonus, reputation_bonus) = match runtime.approach {
            QuestApproach::Direct => (0, 0),
            QuestApproach::Diplomatic => (-definition.credit_reward / 5, 2),
            QuestApproach::Resourceful => (definition.credit_reward / 4, 1),
        };
        self.progression.credits += definition.credit_reward + credit_bonus;
        self.character.attributes.reputation = self
            .character
            .attributes
            .reputation
            .saturating_add(definition.reputation_reward + reputation_bonus);
        self.regional_quest_states
            .insert(quest_id.clone(), QuestState::Completed);
        self.progression
            .world_flags
            .insert(format!("regional_quest_{quest_id}_complete"));
        self.progression.world_flags.insert(
            format!("regional_quest_{quest_id}_{:?}", runtime.approach).to_ascii_lowercase(),
        );
        let region_id = Self::region_id_for_room_id(settlement_room);
        let (stock, demand) = self.regional_market_state(region_id, rule.resource_item_id);
        let (stock_delta, demand_delta) = match runtime.approach {
            QuestApproach::Direct => (1, -1),
            QuestApproach::Diplomatic => (2, -2),
            QuestApproach::Resourceful => (0, 3),
        };
        self.set_regional_market_state(
            region_id,
            rule.resource_item_id,
            stock.saturating_add(stock_delta),
            demand.saturating_add(demand_delta).clamp(-20, 20),
        );
        if let Some(text) = quest_resolution_text(&quest_id, runtime.approach) {
            self.combat_log.push(CombatLogBeat {
                kind: "quest_resolution".to_string(),
                text: text.to_string(),
            });
        }
        if let Some(giver) = NPC_CATALOG
            .iter()
            .find(|npc| npc.id == definition.giver_npc_id)
        {
            self.npc_relationships
                .entry(giver.id.to_string())
                .or_insert_with(|| NpcRelationship::new(giver.id, giver.faction_id))
                .apply(RelationshipAction::CompleteMission);
        }
        let completed = self
            .regional_quest_states
            .values()
            .filter(|state| **state == QuestState::Completed)
            .count();
        self.faction_rank = match completed {
            0..=1 => FactionRank::Outsider,
            2..=4 => FactionRank::Initiate,
            5..=9 => FactionRank::Disciple,
            _ => FactionRank::Envoy,
        };
        self.active_regional_quest_id = None;
        self.active_regional_quest_step = 0;
        self.active_regional_quest_runtime = None;
        if let Some(chapter) = MAIN_STORY_CHAPTERS.iter().find(|chapter| {
            !self
                .main_story_decisions
                .iter()
                .any(|decision| decision.chapter == chapter.chapter)
                && chapter.quest_ids.iter().all(|quest_id| {
                    self.regional_quest_states.get(*quest_id) == Some(&QuestState::Completed)
                })
        }) {
            self.pending_main_story_chapter = Some(chapter.chapter);
            self.progression
                .world_flags
                .insert(format!("main_story_scene_{}_available", chapter.scene_id));
            self.combat_log.push(CombatLogBeat {
                kind: "main_story_scene_available".to_string(),
                text: format!(
                    "{} — meet {} in {} and choose the chapter outcome.",
                    chapter.title, chapter.protagonist_id, chapter.room_id,
                ),
            });
        }
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
        self.record_value_event(
            format!("regional-quest:{quest_id}"),
            format!("quest-contract:{quest_id}"),
            ValueEventSource::RegionalQuest,
            ValueSettlementPolicy::LocalSoftOnly,
            definition.credit_reward + credit_bonus,
        )?;
        self.revision += 1;
        Ok(())
    }

    pub fn advance_pending_main_story_scene(
        &mut self,
    ) -> Result<MainStorySceneAdvance, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.advance_pending_main_story_scene_atomic_inner()
        })
    }

    fn advance_pending_main_story_scene_atomic_inner(
        &mut self,
    ) -> Result<MainStorySceneAdvance, CampaignError> {
        self.require_town()?;
        let pending = self.pending_main_story_chapter.ok_or_else(|| {
            CampaignError::InvalidState(
                "no playable chapter scene is awaiting resolution".to_string(),
            )
        })?;
        let chapter = MAIN_STORY_CHAPTERS
            .iter()
            .find(|chapter| chapter.chapter == pending)
            .expect("pending chapter remains catalog bound");
        if self.room.id() != chapter.room_id {
            return Err(CampaignError::InvalidState(format!(
                "{} must be played in {}",
                chapter.title, chapter.room_id,
            )));
        }
        let step = self
            .main_story_scene_progress
            .get(chapter.scene_id)
            .copied()
            .unwrap_or_default();
        if step < 2 {
            let text = match (chapter.chapter, step) {
                (MainStoryChapter::MirrorCityOaths, 0) => {
                    "The square hears testimony from porters, smiths and ward captains before any oath is proposed."
                }
                (MainStoryChapter::MirrorCityOaths, _) => {
                    "Street Compass Sifu places the three rival charters side by side and asks who will bear their cost."
                }
                (MainStoryChapter::SignalRoadReckoning, 0) => {
                    "Captain Veyra opens the seized route ledger while witnesses identify the hands behind each missing convoy."
                }
                (MainStoryChapter::SignalRoadReckoning, _) => {
                    "The archive doors close; guards, merchants and scouts challenge the evidence before the road is judged."
                }
                (MainStoryChapter::AshenFringeCountermarch, 0) => {
                    "Scout Mako walks the beacon line with the player as refugees and patrols name what the countermarch destroyed."
                }
                (MainStoryChapter::AshenFringeCountermarch, _) => {
                    "At the final assembly every faction commits people and stores, forcing the last choice to carry a visible price."
                }
                (MainStoryChapter::ChapterComplete, _) => unreachable!(),
            }
            .to_string();
            let next = step + 1;
            self.main_story_scene_progress
                .insert(chapter.scene_id.to_string(), next);
            self.progression.world_flags.insert(format!(
                "main_story_scene_{}_beat_{}",
                chapter.scene_id, next
            ));
            self.combat_log.push(CombatLogBeat {
                kind: "main_story_scene_beat".to_string(),
                text: format!("{} — {text}", chapter.title),
            });
            self.revision += 1;
            return Ok(MainStorySceneAdvance::SceneBeat {
                chapter: chapter.chapter,
                step: next,
                text,
            });
        }
        self.finalize_pending_main_story_chapter()
            .map(MainStorySceneAdvance::ChapterResolved)
    }

    fn finalize_pending_main_story_chapter(
        &mut self,
    ) -> Result<MainStoryDecisionRecord, CampaignError> {
        self.require_town()?;
        let pending = self.pending_main_story_chapter.ok_or_else(|| {
            CampaignError::InvalidState(
                "no playable chapter scene is awaiting resolution".to_string(),
            )
        })?;
        let chapter = MAIN_STORY_CHAPTERS
            .iter()
            .find(|chapter| chapter.chapter == pending)
            .expect("pending chapter remains catalog bound");
        if self.room.id() != chapter.room_id {
            return Err(CampaignError::InvalidState(format!(
                "{} must be resolved in {}",
                chapter.title, chapter.room_id,
            )));
        }
        let (flag, credits, reputation, text) =
            main_story_chapter_outcome(chapter.chapter, self.main_story_choice);
        self.progression.world_flags.insert(flag.to_string());
        self.progression
            .world_flags
            .insert(format!("main_story_scene_{}_resolved", chapter.scene_id));
        self.progression.credits += credits;
        self.character.attributes.reputation = self
            .character
            .attributes
            .reputation
            .saturating_add(reputation);
        let decision = MainStoryDecisionRecord {
            chapter: chapter.chapter,
            choice: self.main_story_choice,
            outcome_flag: flag.to_string(),
            day: self.world_clock.day,
        };
        self.main_story_decisions.push(decision.clone());
        self.pending_main_story_chapter = None;
        self.combat_log.push(CombatLogBeat {
            kind: "main_story_chapter".to_string(),
            text: format!(
                "{} — {} and the player resolve the chapter: {}",
                chapter.title, chapter.protagonist_id, text
            ),
        });
        if !self.main_story_decisions.is_empty() {
            self.progression
                .world_flags
                .insert("glass_basin_wayhouse_open".to_string());
            self.story.unlocked_room_ids.extend([
                GLASS_BASIN_WAYHOUSE_ROOM.to_string(),
                DEEP_RELAY_ROOM.to_string(),
            ]);
        }
        if self.main_story_decisions.len() >= 2 {
            self.progression
                .world_flags
                .insert("ashen_fringe_open".to_string());
            self.story.unlocked_room_ids.extend([
                MOON_BRIDGE_ROOM.to_string(),
                EMBER_ORCHARD_EDGE_ROOM.to_string(),
            ]);
        }
        self.main_story_chapter = MAIN_STORY_CHAPTERS
            .iter()
            .find(|candidate| {
                !self
                    .main_story_decisions
                    .iter()
                    .any(|record| record.chapter == candidate.chapter)
            })
            .map(|chapter| chapter.chapter)
            .unwrap_or(MainStoryChapter::ChapterComplete);
        self.main_story_ending = resolve_main_story_ending(&self.main_story_decisions);
        if let Some(ending) = self.main_story_ending {
            let ending_id = ending.label().to_ascii_lowercase().replace([' ', '-'], "_");
            self.progression
                .world_flags
                .insert(format!("main_story_ending_{ending_id}"));
            self.progression
                .world_flags
                .insert(format!("post_ending_world_{ending_id}"));
            let post_state = match ending {
                MainStoryEnding::WayhouseLeague => {
                    "Wayhouses shelter travellers and produce route supplies."
                }
                MainStoryEnding::OpenArchiveRepublic => {
                    "Public ledgers expose shortages and lower information costs."
                }
                MainStoryEnding::FrontierAccord => {
                    "Frontier patrols and caravans share protected crossings."
                }
                MainStoryEnding::ThreeRoadCompact => {
                    "Three institutions remain independent under a rotating compact."
                }
                MainStoryEnding::ContestedMandate => {
                    "Rival mandates persist, opening post-story mediation work."
                }
            };
            self.post_ending_world_state = Some(post_state.to_string());
            self.combat_log.push(CombatLogBeat {
                kind: "main_story_ending_scene".to_string(),
                text: format!("ENDING SCENE — {}: {post_state}", ending.label()),
            });
        }
        self.record_value_event(
            format!("chapter:{:?}", chapter.chapter).to_ascii_lowercase(),
            format!("chapter-contract:{:?}", chapter.chapter).to_ascii_lowercase(),
            ValueEventSource::Chapter,
            ValueSettlementPolicy::LocalSoftOnly,
            credits,
        )?;
        self.revision += 1;
        Ok(decision)
    }

    pub fn resolve_pending_main_story_chapter(
        &mut self,
    ) -> Result<MainStoryDecisionRecord, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.resolve_pending_main_story_chapter_atomic_inner()
        })
    }

    fn resolve_pending_main_story_chapter_atomic_inner(
        &mut self,
    ) -> Result<MainStoryDecisionRecord, CampaignError> {
        loop {
            match self.advance_pending_main_story_scene()? {
                MainStorySceneAdvance::SceneBeat { .. } => {}
                MainStorySceneAdvance::ChapterResolved(decision) => return Ok(decision),
            }
        }
    }

    pub fn advance_ending_epilogue(&mut self) -> Result<String, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.advance_ending_epilogue_atomic_inner())
    }

    fn advance_ending_epilogue_atomic_inner(&mut self) -> Result<String, CampaignError> {
        self.require_town()?;
        let ending = self.main_story_ending.ok_or_else(|| {
            CampaignError::InvalidState("no resolved ending has an epilogue".to_string())
        })?;
        if self.ending_epilogue_complete {
            return Err(CampaignError::InvalidState(
                "the ending epilogue is already complete".to_string(),
            ));
        }
        let required_room = match ending {
            MainStoryEnding::WayhouseLeague => CARAVAN_YARD_ROOM,
            MainStoryEnding::OpenArchiveRepublic => ARCHIVE_STEPS_ROOM,
            MainStoryEnding::FrontierAccord => ASH_BEACON_FIELD_ROOM,
            MainStoryEnding::ThreeRoadCompact => MIRROR_SQUARE_ROOM,
            MainStoryEnding::ContestedMandate => MOON_BRIDGE_ROOM,
        };
        if self.room.id() != required_room {
            return Err(CampaignError::InvalidState(format!(
                "{} epilogue continues in {required_room}",
                ending.label()
            )));
        }
        let text = match (ending, self.ending_epilogue_progress) {
            (MainStoryEnding::WayhouseLeague, 0) => {
                "A winter caravan arrives without losing a traveller; the new league signs its first public shelter ledger."
            }
            (MainStoryEnding::OpenArchiveRepublic, 0) => {
                "Citizens compare the first open ledgers and catch a shortage before a ward goes hungry."
            }
            (MainStoryEnding::FrontierAccord, 0) => {
                "Former enemies relight the Ash Beacon together and exchange patrol routes under witness."
            }
            (MainStoryEnding::ThreeRoadCompact, 0) => {
                "Three delegations take separate seats in Mirror Square and agree to rotate the deciding voice."
            }
            (MainStoryEnding::ContestedMandate, 0) => {
                "Rival envoys meet at Moon Bridge; no banner lowers, but both sides accept the player as mediator."
            }
            (MainStoryEnding::WayhouseLeague, 1) => {
                "Porters elect the league's first route steward while innkeepers publish the cost of every free bed."
            }
            (MainStoryEnding::OpenArchiveRepublic, 1) => {
                "An apprentice challenges an incorrect grain ledger in public and the senior archivists amend it without reprisal."
            }
            (MainStoryEnding::FrontierAccord, 1) => {
                "Mirror and Ashen scouts exchange wounded prisoners, then mark a shared rescue path across the marsh."
            }
            (MainStoryEnding::ThreeRoadCompact, 1) => {
                "The compact's first deadlock ends when market workers demand that all three delegates answer the same shortage."
            }
            (MainStoryEnding::ContestedMandate, 1) => {
                "Two rival tax patrols arrive together; the player forces both to hear the district's accounts before collecting anything."
            }
            (MainStoryEnding::WayhouseLeague, 2) => {
                "A caravan once excluded from the city receives a league seal, shelter and a place on the next route council."
            }
            (MainStoryEnding::OpenArchiveRepublic, 2) => {
                "Citizens copy the corrected ledgers into ward books so no single archive can hide the next crisis."
            }
            (MainStoryEnding::FrontierAccord, 2) => {
                "Children from both frontiers relight a minor beacon and argue cheerfully over whose signal code is clearer."
            }
            (MainStoryEnding::ThreeRoadCompact, 2) => {
                "Independent guilds sign a narrow water pact, proving cooperation need not erase their competing loyalties."
            }
            (MainStoryEnding::ContestedMandate, 2) => {
                "The unresolved banners remain, but a permanent witness bench gives ordinary residents leverage over both courts."
            }
            _ => "The epilogue closes and the post-story world opens for continuing regional work.",
        }
        .to_string();
        self.combat_log.push(CombatLogBeat {
            kind: "main_story_epilogue".to_string(),
            text: format!("{} — {text}", ending.label()),
        });
        self.ending_epilogue_progress = self.ending_epilogue_progress.saturating_add(1);
        if self.ending_epilogue_progress >= 4 {
            self.ending_epilogue_complete = true;
            self.progression
                .world_flags
                .insert("main_story_epilogue_complete".to_string());
            self.progression.credits += 75;
            self.character.attributes.reputation =
                self.character.attributes.reputation.saturating_add(8);
            self.record_value_event(
                format!("ending:{ending:?}").to_ascii_lowercase(),
                format!("ending-contract:{ending:?}").to_ascii_lowercase(),
                ValueEventSource::Ending,
                ValueSettlementPolicy::LocalSoftOnly,
                75,
            )?;
        }
        self.revision += 1;
        Ok(text)
    }

}

