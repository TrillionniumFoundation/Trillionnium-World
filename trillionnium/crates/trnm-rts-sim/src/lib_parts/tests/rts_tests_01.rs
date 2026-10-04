    fn map() -> BattleMapSeedV1 {
        BattleMapSeedV1 {
            width: 20,
            height: 10,
            terrain_rows: vec!["gggggggggggggggggggg".to_string(); 10],
            party_start: BattleGridPoint::new(1, 8),
            approach_point: BattleGridPoint::new(7, 6),
            objective: BattleGridPoint::new(18, 1),
            resource_nodes: vec![BattleMapNodeV1 {
                id: "amber_mid".to_string(),
                position: BattleGridPoint::new(8, 7),
            }],
            enemy_spawns: vec![
                BattleMapNodeV1 {
                    id: "contact_scout".to_string(),
                    position: BattleGridPoint::new(10, 5),
                },
                BattleMapNodeV1 {
                    id: "contact_warden".to_string(),
                    position: BattleGridPoint::new(12, 4),
                },
                BattleMapNodeV1 {
                    id: "contact_striker".to_string(),
                    position: BattleGridPoint::new(14, 3),
                },
                BattleMapNodeV1 {
                    id: "relay_guard".to_string(),
                    position: BattleGridPoint::new(16, 2),
                },
            ],
        }
    }

    fn merge_yaml(base: &mut serde_yaml::Value, overlay: serde_yaml::Value) {
        match (base, overlay) {
            (serde_yaml::Value::Mapping(base), serde_yaml::Value::Mapping(overlay)) => {
                for (key, value) in overlay {
                    if let Some(base_value) = base.get_mut(&key) {
                        merge_yaml(base_value, value);
                    } else {
                        base.insert(key, value);
                    }
                }
            }
            (base, overlay) => *base = overlay,
        }
    }

    fn authored_map(name: &str) -> BattleMapSeedV1 {
        let root =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../assets/first_contact/maps");
        let base_source = fs::read_to_string(root.join("first_contact.yaml")).unwrap();
        let overlay_source = fs::read_to_string(root.join(format!("{name}.yaml"))).unwrap();
        let mut value: serde_yaml::Value = serde_yaml::from_str(&base_source).unwrap();
        let overlay: serde_yaml::Value = serde_yaml::from_str(&overlay_source).unwrap();
        let transform = overlay["terrain_transform"].as_str().map(str::to_string);
        merge_yaml(&mut value, overlay);
        let width = value["width"].as_u64().unwrap() as u16;
        let height = value["height"].as_u64().unwrap() as u16;
        let mut terrain_rows = value["terrain_rows"]
            .as_sequence()
            .unwrap()
            .iter()
            .map(|row| row.as_str().unwrap().to_string())
            .collect::<Vec<_>>();
        match transform.as_deref() {
            Some("mirror_x") => {
                for row in &mut terrain_rows {
                    *row = row.chars().rev().collect();
                }
            }
            Some("rotate_180") => {
                terrain_rows.reverse();
                for row in &mut terrain_rows {
                    *row = row.chars().rev().collect();
                }
            }
            Some("shift_5") | Some("shift_11") => {
                let amount = if transform.as_deref() == Some("shift_5") {
                    5
                } else {
                    11
                };
                for row in &mut terrain_rows {
                    let mut chars = row.chars().collect::<Vec<_>>();
                    chars.rotate_left(amount);
                    *row = chars.into_iter().collect();
                }
            }
            _ => {}
        }
        let point = |value: &serde_yaml::Value| {
            BattleGridPoint::new(
                value["x"].as_i64().unwrap() as i16,
                value["y"].as_i64().unwrap() as i16,
            )
        };
        let player_start = point(&value["player_start"]);
        let objective = point(&value["objective"]);
        let south_pass = value["chokepoints"]
            .as_sequence()
            .unwrap()
            .iter()
            .find(|choke| choke["id"].as_str() == Some("south_pass"))
            .unwrap();
        let approach_point = BattleGridPoint::new(
            south_pass["x"].as_i64().unwrap() as i16
                + south_pass["width"].as_u64().unwrap() as i16 / 2,
            south_pass["y"].as_i64().unwrap() as i16
                + south_pass["height"].as_u64().unwrap() as i16 / 2,
        );
        let nodes = |key: &str, owner: Option<&str>| {
            value[key]
                .as_sequence()
                .unwrap()
                .iter()
                .filter(|entry| owner.is_none_or(|owner| entry["owner"].as_str() == Some(owner)))
                .map(|entry| BattleMapNodeV1 {
                    id: entry["id"].as_str().unwrap().to_string(),
                    position: point(entry),
                })
                .collect::<Vec<_>>()
        };
        let map = BattleMapSeedV1 {
            width,
            height,
            terrain_rows,
            party_start: player_start,
            approach_point,
            objective,
            resource_nodes: nodes("resources", None),
            enemy_spawns: nodes("units", Some("contact")),
        };
        map.validate().unwrap();
        map
    }

    fn seed() -> BattleSeedV1 {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        campaign.equip_starter_weapon().unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        campaign.accept_first_contact_quest().unwrap();
        campaign.start_first_contact_battle(map()).unwrap()
    }

    fn iron_delta_seed() -> BattleSeedV1 {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        campaign.equip_starter_weapon().unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        campaign.quest_state = QuestState::Completed;
        campaign.progression.aftershock_completions = 1;
        campaign.active_mission = CampaignMission::AftershockPatrol;
        for flag in [
            "first_contact_secured",
            "convoy_exodus_secured",
            "mirror_siege_secured",
        ] {
            campaign.progression.world_flags.insert(flag.to_string());
        }
        assert_eq!(
            campaign.cycle_endgame_mission().unwrap(),
            CampaignMission::IronDeltaSkirmish
        );
        campaign.accept_first_contact_quest().unwrap();
        let mut authored_map = map();
        authored_map.enemy_spawns = [
            "ash_runner",
            "ash_bulwark",
            "ash_lancer",
            "ash_sapper",
            "ash_commander",
        ]
        .into_iter()
        .enumerate()
        .map(|(index, id)| BattleMapNodeV1 {
            id: id.to_string(),
            position: BattleGridPoint::new(10 + index as i16, 5),
        })
        .collect();
        campaign.start_first_contact_battle(authored_map).unwrap()
    }

    #[test]
    fn iron_delta_instantiates_the_authored_ashen_compact_roster() {
        let sim = MissionSimV1::from_seed(iron_delta_seed()).unwrap();
        let runner = sim
            .enemies
            .iter()
            .find(|enemy| enemy.unit_id == "ash_runner")
            .unwrap();
        let commander = sim
            .enemies
            .iter()
            .find(|enemy| enemy.unit_id == "ash_commander")
            .unwrap();
        assert_eq!(runner.role, "raider");
        assert_eq!(commander.role, "heavy");
        assert!(commander.max_hp > runner.max_hp);
        assert!(commander.damage > runner.damage);
    }

    #[test]
    fn configurable_skirmish_drives_faction_roster_structures_tech_and_terminal_rules() {
        let mut seed = iron_delta_seed();
        seed.skirmish.player_faction = RtsFaction::AshenCompact;
        seed.skirmish.enemy_faction = RtsFaction::MirrorCoalition;
        seed.skirmish.starting_resources = 1_000;
        seed.skirmish.victory_mode = SkirmishVictoryMode::Score;
        seed.skirmish.score_target = 500;
        seed.seed_hash = seed.computed_hash().unwrap();
        seed.validate().unwrap();

        let mut sim = MissionSimV1::from_seed(seed.clone()).unwrap();
        assert!(
            sim.resources_available >= 1_000,
            "skirmish resources {} readiness {} configured {}",
            sim.resources_available,
            sim.seed.expedition_readiness.starting_resources,
            sim.seed.skirmish.starting_resources
        );
        assert!(sim
            .enemies
            .iter()
            .filter(|enemy| enemy.role != "worker")
            .all(|enemy| {
                UNIT_ROSTER.iter().any(|unit| {
                    unit.faction == RtsFaction::MirrorCoalition && unit.role == enemy.role
                })
            }));
        let start = sim.seed.map.party_start;
        let mut workshop = order(
            &sim,
            RtsOrderKind::Build,
            BattleGridPoint::new(start.x + 4, start.y),
        );
        workshop.target_rule_id = Some("field_workshop".to_string());
        sim.issue_order(workshop).unwrap();
        let mut beacon = order(
            &sim,
            RtsOrderKind::Build,
            BattleGridPoint::new(start.x + 5, start.y),
        );
        beacon.target_rule_id = Some("ash_beacon".to_string());
        sim.issue_order(beacon).unwrap();
        let mut opposing_tower = order(
            &sim,
            RtsOrderKind::Build,
            BattleGridPoint::new(start.x + 6, start.y),
        );
        opposing_tower.target_rule_id = Some("sensor_tower".to_string());
        assert!(sim.issue_order(opposing_tower).is_err());

        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Research,
            "rapid_mustering",
            start,
        ))
        .unwrap();
        for _ in 0..80 {
            sim.advance_side_jobs(AuthoritySide::Player);
        }
        assert!(sim.researched_techs.contains("rapid_mustering"));
        sim.issue_order(job_order(&sim, RtsOrderKind::Train, "ash_runner", start))
            .unwrap();
        for _ in 0..60 {
            sim.advance_side_jobs(AuthoritySide::Player);
        }
        assert!(sim.party.iter().any(|unit| {
            !unit.persistent && unit.role == "raider" && unit.unit_id.starts_with("ash_runner_")
        }));
        assert!(sim
            .issue_order(job_order(
                &sim,
                RtsOrderKind::Train,
                "mirror_wayfinder",
                start,
            ))
            .is_err());

        let mut score_sim = MissionSimV1::from_seed(seed.clone()).unwrap();
        score_sim.player_score = 500;
        score_sim.step().unwrap();
        assert_eq!(score_sim.outcome, Some(BattleOutcome::Victory));

        seed.skirmish.victory_mode = SkirmishVictoryMode::Annihilation;
        seed.seed_hash = seed.computed_hash().unwrap();
        let mut annihilation = MissionSimV1::from_seed(seed).unwrap();
        for enemy in &mut annihilation.enemies {
            enemy.hp = 0;
        }
        for structure in &mut annihilation.enemy_structures {
            structure.hp = 0;
        }
        annihilation.step().unwrap();
        assert_eq!(annihilation.outcome, Some(BattleOutcome::Victory));
    }

    #[test]
    fn skirmish_enemy_runs_a_deterministic_resource_build_research_and_production_loop() {
        let seed = iron_delta_seed();
        let mut first = MissionSimV1::from_seed(seed.clone()).unwrap();
        let mut second = MissionSimV1::from_seed(seed).unwrap();
        for sim in [&mut first, &mut second] {
            assert_eq!(
                sim.enemies
                    .iter()
                    .filter(|unit| unit.role == "worker" && unit.alive())
                    .count(),
                usize::from(sim.enemy_workers)
            );
            assert!(sim.enemy_supply_used() <= sim.enemy_supply_cap());
            for unit in &mut sim.party {
                unit.max_hp = 100_000;
                unit.hp = 100_000;
            }
            for _ in 0..500 {
                if !sim.terminal() {
                    sim.step().unwrap();
                }
            }
            assert!(sim.enemy_resources_generated > sim.seed.skirmish.starting_resources);
            assert!(sim.enemy_resources_spent > 0);
            assert!(sim.authority_job_commands.iter().any(|command| {
                command.source == AuthorityCommandSource::AdaptiveAi
                    && command.side == AuthoritySide::Enemy
            }));
            assert_eq!(
                sim.enemy_resources_available + sim.enemy_resources_spent,
                sim.enemy_resources_generated
            );
            assert!(sim.enemy_structures.len() >= 3);
            assert!(
                !sim.enemy_researched_techs.is_empty()
                    || sim.enemies.len() > sim.seed.map.enemy_spawns.len()
            );
        }
        assert_eq!(
            first.enemy_resources_generated,
            second.enemy_resources_generated
        );
        assert_eq!(first.enemy_resources_spent, second.enemy_resources_spent);
        assert_eq!(first.enemy_structures, second.enemy_structures);
        assert_eq!(first.enemy_researched_techs, second.enemy_researched_techs);
        assert_eq!(first.enemies, second.enemies);
    }

    fn order(sim: &MissionSimV1, kind: RtsOrderKind, target: BattleGridPoint) -> RtsFrameOrder {
        let mut order = RtsFrameOrder::new(
            sim.tick as u32,
            "player",
            sim.party
                .iter()
                .filter(|unit| unit.alive())
                .map(|unit| unit.unit_id.clone())
                .collect::<Vec<_>>(),
            kind,
            RtsOrderSource::LocalInput,
        );
        order.target_tile = Some(RtsTile::new(target.x as i32, target.y as i32));
        if kind == RtsOrderKind::Harvest {
            order.target_actor_id = Some("amber_mid".to_string());
        } else if kind == RtsOrderKind::Attack {
            order.target_actor_id = Some("relay_beacon".to_string());
        } else if kind == RtsOrderKind::Ability {
            order.target_rule_id = Some("party_signature".to_string());
        } else if kind == RtsOrderKind::Repair {
            order.target_actor_id = Some("party_field_aid".to_string());
        } else if kind == RtsOrderKind::Build {
            order.target_rule_id = Some("field_barricade".to_string());
        }
        order
    }

    fn job_order(
        sim: &MissionSimV1,
        kind: RtsOrderKind,
        rule: &str,
        target: BattleGridPoint,
    ) -> RtsFrameOrder {
        let mut order = order(sim, kind, target);
        order.target_rule_id = Some(rule.to_string());
        order.queue_id = Some("test_queue".to_string());
        order
    }

    fn step_until(sim: &mut MissionSimV1, predicate: impl Fn(&MissionSimV1) -> bool, limit: u64) {
        while !sim.terminal() && !predicate(sim) && sim.tick < limit {
            sim.step().unwrap();
        }
        assert!(
            predicate(sim),
            "condition not reached by tick {} phase {:?} outcome {:?} guard {} capture {} wave {} alive_enemies {} order {:?}",
            sim.tick,
            sim.phase,
            sim.outcome,
            sim.relay_guard_hp,
            sim.relay_capture_ticks,
            sim.reinforcement_wave,
            sim.enemies.iter().filter(|enemy| enemy.alive()).count(),
            sim.current_order_kind(),
        );
    }

    fn run_decision_dense_victory(mut sim: MissionSimV1, harvest: bool) -> MissionSimV1 {
        let approach = sim.seed.map.approach_point;
        sim.issue_order(order(&sim, RtsOrderKind::Move, approach))
            .unwrap();
        step_until(&mut sim, |sim| sim.phase == BattlePhase::Contact, 900);
        if harvest {
            let resource = sim.seed.map.resource_nodes[0].position;
            sim.issue_order(order(&sim, RtsOrderKind::Harvest, resource))
                .unwrap();
            step_until(&mut sim, |sim| sim.resources_available >= 100, 1_300);
        } else {
            let objective = sim.seed.map.objective;
            sim.issue_order(order(&sim, RtsOrderKind::Ability, objective))
                .unwrap();
        }
        let objective = sim.seed.map.objective;
        sim.issue_order(order(&sim, RtsOrderKind::Attack, objective))
            .unwrap();
        step_until(
            &mut sim,
            |sim| sim.phase == BattlePhase::Relay && sim.relay_guard_hp <= 0,
            2_700,
        );
        sim.issue_order(order(&sim, RtsOrderKind::Hold, objective))
            .unwrap();
        for wave in 1..=2 {
            step_until(
                &mut sim,
                |sim| sim.reinforcement_wave >= wave,
                FIVE_MINUTE_TICKS,
            );
            if harvest {
                let resource_order = if wave == 1 {
                    RtsOrderKind::Repair
                } else {
                    RtsOrderKind::Build
                };
                sim.issue_order(order(&sim, resource_order, objective))
                    .unwrap();
            } else {
                sim.issue_order(order(&sim, RtsOrderKind::Ability, objective))
                    .unwrap();
            }
            sim.issue_order(order(&sim, RtsOrderKind::Attack, objective))
                .unwrap();
            step_until(
                &mut sim,
                |sim| sim.enemies.iter().all(|enemy| !enemy.alive()),
                FIVE_MINUTE_TICKS,
            );
            sim.issue_order(order(&sim, RtsOrderKind::Move, objective))
                .unwrap();
            step_until(
                &mut sim,
                |sim| {
                    sim.party.iter().any(|unit| {
                        unit.alive() && distance(unit.position, sim.seed.map.objective) <= 2
                    })
                },
                FIVE_MINUTE_TICKS,
            );
            sim.issue_order(order(&sim, RtsOrderKind::Hold, objective))
                .unwrap();
        }
        while !sim.terminal() {
            sim.step().unwrap();
        }
        sim
    }

    #[test]
    fn three_phase_orders_produce_a_real_three_to_five_minute_victory() {
        let sim = run_decision_dense_victory(MissionSimV1::from_seed(seed()).unwrap(), true);
        assert_eq!(sim.outcome, Some(BattleOutcome::Victory));
        assert!(
            (THREE_MINUTE_TICKS..=FIVE_MINUTE_TICKS).contains(&sim.tick),
            "victory tick {} is outside the 3-5 minute target",
            sim.tick
        );
        assert!((8..=12).contains(&sim.order_count));
        let barricade_cost = SimStructureKind::FieldBarricade.definition().cost;
        assert!(sim.resources_spent >= FIELD_AID_COST + barricade_cost);
        assert_eq!(sim.enemy_tactics_level, 3);
        assert!(!sim.enemy_ai_history.is_empty());
        let result = sim.into_result().unwrap();
        assert!(!result.loot.is_empty());
        assert!(result.resource_delta > 0);
    }

    #[test]
    fn one_order_cannot_win_the_mission() {
        for kind in [
            RtsOrderKind::Move,
            RtsOrderKind::Attack,
            RtsOrderKind::Harvest,
            RtsOrderKind::Hold,
        ] {
            let mut sim = MissionSimV1::from_seed(seed()).unwrap();
            let target = match kind {
                RtsOrderKind::Harvest => sim.seed.map.resource_nodes[0].position,
                RtsOrderKind::Move => sim.seed.map.approach_point,
                _ => sim.seed.map.objective,
            };
            sim.issue_order(order(&sim, kind, target)).unwrap();
            while !sim.terminal() {
                sim.step().unwrap();
            }
            assert_ne!(
                sim.outcome,
                Some(BattleOutcome::Victory),
                "{kind:?} won alone"
            );
        }
    }

    #[test]
    fn ability_rush_is_a_second_viable_route_without_resource_payout() {
        let sim = run_decision_dense_victory(MissionSimV1::from_seed(seed()).unwrap(), false);
        assert_eq!(sim.outcome, Some(BattleOutcome::Victory));
        assert_eq!(sim.resources_gathered, 0);
        assert!((8..=12).contains(&sim.order_count));
        assert!(sim.distinct_order_kinds.contains("ability"));
        assert!((THREE_MINUTE_TICKS..=FIVE_MINUTE_TICKS).contains(&sim.tick));
    }

    #[test]
    fn withdrawal_has_zero_progression_reward() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.issue_order(order(&sim, RtsOrderKind::Move, sim.seed.map.approach_point))
            .unwrap();
        for _ in 0..WITHDRAWAL_MIN_TICKS {
            sim.step().unwrap();
        }
        let mut retreat = order(&sim, RtsOrderKind::Extract, sim.seed.map.party_start);
        retreat.target_actor_id = Some("expedition_gate".to_string());
        sim.issue_order(retreat).unwrap();
        let result = sim.into_result().unwrap();
        assert_eq!(result.outcome, BattleOutcome::Withdrawal);
        assert_eq!(
            result
                .units
                .iter()
                .map(|unit| unit.experience_gained)
                .sum::<u64>(),
            0
        );
        assert_eq!(result.resource_delta, 0);
    }

    #[test]
    fn checkpoint_resume_is_bit_deterministic() {
        let directory = tempdir().unwrap();
        let store = SimCheckpointStore::new(directory.path().join("battle.json"));
        let mut baseline = MissionSimV1::from_seed(seed()).unwrap();
        baseline
            .issue_order(order(
                &baseline,
                RtsOrderKind::Move,
                baseline.seed.map.approach_point,
            ))
            .unwrap();
        for _ in 0..200 {
            baseline.step().unwrap();
        }
        store.save_atomic(&baseline).unwrap();
        let resumed = store.load_for_seed(&baseline.seed).unwrap().unwrap();
        assert_eq!(resumed, baseline);
        assert_eq!(
            resumed.snapshot_hash().unwrap(),
            baseline.snapshot_hash().unwrap()
        );
    }

    #[test]
    fn tampered_checkpoint_is_rejected() {
        let mut checkpoint =
            SimCheckpointV1::capture(&MissionSimV1::from_seed(seed()).unwrap()).unwrap();
        checkpoint.sim.resources_gathered += 1;
        assert!(matches!(checkpoint.validate(), Err(SimError::Integrity(_))));
    }

    #[test]
    fn recon_production_research_upgrade_and_formations_change_authoritative_state() {
        let seed = seed();
        let mut sim = MissionSimV1::from_seed(seed.clone()).unwrap();
        sim.resources_gathered = 200;
        sim.resources_available = 200;
        let target = seed.map.approach_point;

        sim.issue_order(order(&sim, RtsOrderKind::Recon, target))
            .unwrap();
        assert_eq!(sim.intel_level, 1);
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Train,
            "field_support_drone",
            target,
        ))
        .unwrap();
        assert!(sim.authority_job_commands.iter().any(|command| {
            command.source == AuthorityCommandSource::PlayerOrder
                && command.side == AuthoritySide::Player
        }));
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Research,
            "field_logistics",
            target,
        ))
        .unwrap();
        for _ in 0..150 {
            sim.step().unwrap();
        }
        assert_eq!(sim.support_units.len(), 1);
        assert!(sim.researched_techs.contains("field_logistics"));
        let damage_before = sim.party[0].damage;
        sim.issue_order(job_order(&sim, RtsOrderKind::Upgrade, "relay_arms", target))
            .unwrap();
        for _ in 0..60 {
            sim.step().unwrap();
        }
        assert_eq!(sim.upgrade_level, 1);
        assert!(sim.party[0].damage > damage_before);
        assert_eq!(sim.resources_available + sim.resources_spent, 200);

        let mut line = MissionSimV1::from_seed(seed.clone()).unwrap();
        let mut line_order = order(&line, RtsOrderKind::Move, target);
        line_order.formation_id = Some("party_line".to_string());
        line.issue_order(line_order).unwrap();
        let mut column = MissionSimV1::from_seed(seed).unwrap();
        let mut column_order = order(&column, RtsOrderKind::Move, target);
        column_order.formation_id = Some("party_column".to_string());
        column.issue_order(column_order).unwrap();
        for _ in 0..180 {
            line.step().unwrap();
            column.step().unwrap();
        }
        assert_ne!(
            line.party
                .iter()
                .map(|unit| unit.position)
                .collect::<Vec<_>>(),
            column
                .party
                .iter()
                .map(|unit| unit.position)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn control_groups_and_shift_queue_are_authoritative_and_cancelable() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        let subjects = sim.party[0..2]
            .iter()
            .map(|unit| unit.unit_id.clone())
            .collect::<Vec<_>>();
        let mut assign = RtsFrameOrder::new(
            1,
            "player",
            subjects.clone(),
            RtsOrderKind::AssignGroup,
            RtsOrderSource::LocalInput,
        );
        assign.target_rule_id = Some("2".to_string());
        sim.issue_order(assign).unwrap();
        assert_eq!(
            sim.control_group_members("2")
                .into_iter()
                .collect::<BTreeSet<_>>(),
            subjects.into_iter().collect::<BTreeSet<_>>()
        );

        let mut first = order(&sim, RtsOrderKind::Move, sim.seed.map.approach_point);
        first.frame = 2;
        first.queued = true;
        first.queue_id = Some("route-1".to_string());
        sim.issue_order(first).unwrap();
        let mut second = order(&sim, RtsOrderKind::Attack, sim.seed.map.objective);
        second.frame = 2;
        second.queued = true;
        second.queue_id = Some("route-2".to_string());
        sim.issue_order(second).unwrap();
        assert_eq!(sim.queued_orders.len(), 1);

        let mut cancel = RtsFrameOrder::new(
            2,
            "player",
            sim.party
                .iter()
                .map(|unit| unit.unit_id.clone())
                .collect::<Vec<_>>(),
            RtsOrderKind::CancelQueuedOrder,
            RtsOrderSource::LocalInput,
        );
        cancel.queue_id = Some("route-2".to_string());
        sim.issue_order(cancel).unwrap();
        assert!(sim.queued_orders.is_empty());
        sim.party[0].hp = 0;
        let dead_id = sim.party[0].unit_id.clone();
        sim.step().unwrap();
        assert!(!sim.control_group_members("2").contains(&dead_id));
        assert!(sim.validate().is_ok());
    }

