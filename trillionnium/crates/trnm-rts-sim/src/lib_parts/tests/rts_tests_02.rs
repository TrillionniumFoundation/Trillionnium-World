    #[test]
    fn production_jobs_pause_promote_rally_cancel_and_refund_once() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.resources_gathered = 300;
        sim.resources_available = 300;
        let target = sim.seed.map.approach_point;
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Train,
            "field_support_drone",
            target,
        ))
        .unwrap();
        let support_id = sim.jobs[0].job_id.clone();

        let lifecycle = |sim: &MissionSimV1, kind: RtsOrderKind, job_id: &str| {
            let mut order = RtsFrameOrder::new(
                sim.tick as u32,
                "player",
                sim.party
                    .iter()
                    .map(|unit| unit.unit_id.clone())
                    .collect::<Vec<_>>(),
                kind,
                RtsOrderSource::LocalInput,
            );
            order.queue_id = Some(job_id.to_string());
            order
        };
        sim.issue_order(lifecycle(&sim, RtsOrderKind::PauseJob, &support_id))
            .unwrap();
        let remaining = sim.jobs[0].remaining_ticks;
        for _ in 0..10 {
            sim.step().unwrap();
        }
        assert_eq!(sim.jobs[0].remaining_ticks, remaining);
        sim.issue_order(lifecycle(&sim, RtsOrderKind::ResumeJob, &support_id))
            .unwrap();

        let mut blocked_rally = lifecycle(&sim, RtsOrderKind::SetRally, &support_id);
        blocked_rally.target_tile = Some(RtsTile::new(999, 999));
        assert!(sim.issue_order(blocked_rally).is_err());
        let mut rally = lifecycle(&sim, RtsOrderKind::SetRally, &support_id);
        rally.target_tile = Some(RtsTile::new(target.x as i32, target.y as i32));
        sim.issue_order(rally).unwrap();
        assert_eq!(sim.jobs[0].target, target);

        sim.issue_order(job_order(&sim, RtsOrderKind::Train, "field_medic", target))
            .unwrap();
        let medic_id = sim.jobs[1].job_id.clone();
        sim.issue_order(lifecycle(&sim, RtsOrderKind::PromoteJob, &medic_id))
            .unwrap();
        assert_eq!(sim.jobs[0].kind, SimJobKind::TrainMedic);
        let before_cancel = sim.resources_available;
        sim.issue_order(lifecycle(&sim, RtsOrderKind::CancelJob, &medic_id))
            .unwrap();
        assert_eq!(sim.resources_available, before_cancel + 25);
        assert!(sim
            .issue_order(lifecycle(&sim, RtsOrderKind::CancelJob, &medic_id))
            .is_err());
        assert_eq!(sim.resources_available + sim.resources_spent, 300);
    }

    #[test]
    fn fog_hides_targets_until_deterministic_recon_reveals_them() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        let enemy_id = sim.enemies[0].unit_id.clone();
        let enemy_position = sim.enemies[0].position;
        assert!(!sim.is_enemy_visible(&enemy_id));
        let mut hidden_attack = order(&sim, RtsOrderKind::Attack, enemy_position);
        hidden_attack.target_actor_id = Some(enemy_id.clone());
        assert!(sim.issue_order(hidden_attack).is_err());

        sim.resources_gathered = 20;
        sim.resources_available = 20;
        sim.issue_order(order(&sim, RtsOrderKind::Recon, enemy_position))
            .unwrap();
        assert!(sim.is_enemy_visible(&enemy_id));
        assert!(sim.explored_tiles.is_superset(&sim.visible_tiles));
        assert!(sim.visible_percent() > 0);
        sim.issue_order({
            let mut attack = order(&sim, RtsOrderKind::Attack, enemy_position);
            attack.target_actor_id = Some(enemy_id);
            attack
        })
        .unwrap();
    }

    #[test]
    fn expanded_medic_optics_armor_tree_changes_authoritative_combat_state() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.resources_gathered = 600;
        sim.resources_available = 600;
        let target = sim.seed.map.party_start;
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Research,
            "field_logistics",
            target,
        ))
        .unwrap();
        for _ in 0..70 {
            sim.step().unwrap();
        }
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Research,
            "signal_optics",
            target,
        ))
        .unwrap();
        for _ in 0..90 {
            sim.step().unwrap();
        }
        assert!(sim.researched_techs.contains("signal_optics"));

        let armor_before = sim.party[0].armor;
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Upgrade,
            "field_armor",
            target,
        ))
        .unwrap();
        for _ in 0..75 {
            sim.step().unwrap();
        }
        assert_eq!(sim.armor_upgrade_level, 1);
        assert!(sim.party[0].armor > armor_before);

        sim.issue_order(job_order(&sim, RtsOrderKind::Train, "field_medic", target))
            .unwrap();
        for _ in 0..95 {
            sim.step().unwrap();
        }
        assert!(sim.support_units.iter().any(|unit| unit.role == "medic"));
        assert_eq!(sim.resources_available + sim.resources_spent, 600);
    }

    #[test]
    fn workers_carry_depleting_resources_back_to_the_command_post() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        let node = sim.resource_nodes[0].position;
        sim.issue_order(order(&sim, RtsOrderKind::Harvest, node))
            .unwrap();
        step_until(
            &mut sim,
            |sim| sim.party.iter().any(|unit| unit.cargo > 0),
            800,
        );
        assert_eq!(
            sim.resources_gathered, 0,
            "cargo must not teleport into storage"
        );
        step_until(&mut sim, |sim| sim.resources_gathered > 0, 1_400);
        let carried = sim.party.iter().map(|unit| unit.cargo).sum::<u32>();
        assert_eq!(
            sim.resource_nodes[0].remaining + carried + sim.resources_gathered,
            RESOURCE_NODE_CAPACITY
        );
        assert_eq!(
            sim.resources_available + sim.resources_spent,
            sim.resources_gathered
        );
    }

    #[test]
    fn structures_supply_power_prerequisites_repair_and_blocking_are_authoritative() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.resources_gathered = 400;
        sim.resources_available = 400;
        let start = sim.seed.map.party_start;
        let workshop_tile = BattleGridPoint::new(start.x + 4, start.y);
        let mut workshop = order(&sim, RtsOrderKind::Build, workshop_tile);
        workshop.target_rule_id = Some("field_workshop".to_string());
        sim.issue_order(workshop).unwrap();
        for _ in 0..400 {
            advance_side_construction_worker(&sim.seed, &mut sim.party, &sim.enemies, &sim.jobs);
            sim.advance_side_jobs(AuthoritySide::Player);
            if sim.structures.iter().any(|structure| {
                structure.kind == SimStructureKind::FieldWorkshop
                    && structure.position == workshop_tile
            }) {
                break;
            }
        }
        assert!(
            sim.low_power(),
            "workshop construction did not complete: jobs {:?}, structures {:?}, party {:?}",
            sim.jobs,
            sim.structures,
            sim.party
                .iter()
                .map(|unit| (
                    &unit.unit_id,
                    unit.position,
                    unit.move_speed_milli,
                    unit.movement_budget_milli,
                ))
                .collect::<Vec<_>>()
        );

        let job = job_order(&sim, RtsOrderKind::Research, "field_logistics", start);
        sim.issue_order(job).unwrap();
        let remaining = sim.jobs[0].remaining_ticks;
        for _ in 0..5 {
            sim.step().unwrap();
        }
        assert_eq!(sim.jobs[0].remaining_ticks, remaining);

        let generator_tile = BattleGridPoint::new(start.x + 5, start.y);
        let mut generator = order(&sim, RtsOrderKind::Build, generator_tile);
        generator.target_rule_id = Some("relay_generator".to_string());
        sim.issue_order(generator.clone()).unwrap();
        for _ in 0..400 {
            advance_side_construction_worker(&sim.seed, &mut sim.party, &sim.enemies, &sim.jobs);
            sim.advance_side_jobs(AuthoritySide::Player);
            if sim
                .structures
                .iter()
                .any(|structure| structure.kind == SimStructureKind::RelayGenerator)
            {
                break;
            }
        }
        assert!(!sim.low_power());
        sim.step().unwrap();
        assert!(sim.jobs[0].remaining_ticks < remaining);
        assert!(
            sim.issue_order(generator).is_err(),
            "occupied build must fail"
        );

        let generator_id = sim
            .structures
            .iter()
            .find(|structure| structure.kind == SimStructureKind::RelayGenerator)
            .unwrap()
            .structure_id
            .clone();
        let generator_index = sim
            .structures
            .iter()
            .position(|structure| structure.structure_id == generator_id)
            .unwrap();
        sim.structures[generator_index].hp -= 200;
        let mut repair = order(&sim, RtsOrderKind::Repair, generator_tile);
        repair.target_actor_id = Some(generator_id);
        sim.issue_order(repair).unwrap();
        assert!(sim.structures[generator_index].hp > sim.structures[generator_index].max_hp - 200);

        let cap_before = sim.supply_cap();
        let mut supply = order(
            &sim,
            RtsOrderKind::Build,
            BattleGridPoint::new(start.x + 6, start.y),
        );
        supply.target_rule_id = Some("supply_cache".to_string());
        sim.issue_order(supply).unwrap();
        for _ in 0..400 {
            advance_side_construction_worker(&sim.seed, &mut sim.party, &sim.enemies, &sim.jobs);
            sim.advance_side_jobs(AuthoritySide::Player);
            if sim
                .structures
                .iter()
                .any(|structure| structure.kind == SimStructureKind::SupplyCache)
            {
                break;
            }
        }
        assert_eq!(sim.supply_cap(), cap_before + 4);
        assert_eq!(sim.resources_available + sim.resources_spent, 400);
        sim.validate().unwrap();
        let directory = tempdir().unwrap();
        let store = SimCheckpointStore::new(directory.path().join("economy.json"));
        store.save_atomic(&sim).unwrap();
        let resumed = store.load_for_seed(&sim.seed).unwrap().unwrap();
        assert_eq!(resumed, sim);
        assert_eq!(
            resumed.snapshot_hash().unwrap(),
            sim.snapshot_hash().unwrap()
        );
    }

    #[test]
    fn stance_patrol_stop_and_veterancy_survive_the_authoritative_result() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        let hero = sim.party[0].unit_id.clone();
        let subjects = vec![hero.clone()];
        let mut stance = RtsFrameOrder::new(
            0,
            "player",
            subjects.clone(),
            RtsOrderKind::SetStance,
            RtsOrderSource::LocalInput,
        );
        stance.target_rule_id = Some(RtsUnitStance::Aggressive.as_str().to_string());
        sim.issue_order(stance).unwrap();
        assert_eq!(sim.party[0].stance, RtsUnitStance::Aggressive);

        let mut patrol = RtsFrameOrder::new(
            1,
            "player",
            subjects.clone(),
            RtsOrderKind::Patrol,
            RtsOrderSource::LocalInput,
        );
        patrol.target_tile = Some(RtsTile::new(
            sim.seed.map.approach_point.x as i32,
            sim.seed.map.approach_point.y as i32,
        ));
        sim.issue_order(patrol).unwrap();
        assert!(sim.party[0].patrol_target.is_some());
        let stop = RtsFrameOrder::new(
            2,
            "player",
            subjects.clone(),
            RtsOrderKind::Stop,
            RtsOrderSource::LocalInput,
        );
        sim.issue_order(stop).unwrap();
        assert!(sim.active_order.is_none() && sim.party[0].patrol_target.is_none());

        let selected = BTreeSet::from([hero]);
        let interval = sim.party[0].attack_interval_ticks as u64;
        for target_index in 0..2 {
            sim.enemies[target_index].position = sim.party[0].position;
            sim.enemies[target_index].hp = 1;
            sim.enemies[target_index].evasion_permille = 0;
            sim.tick = sim.tick.next_multiple_of(interval);
            let target_id = sim.enemies[target_index].unit_id.clone();
            sim.party_attack(&selected, Some(&target_id));
            sim.tick += 1;
        }
        assert_eq!(sim.party[0].confirmed_kills, 2);
        assert_eq!(sim.party[0].veteran_rank, 1);
        sim.outcome = Some(BattleOutcome::Withdrawal);
        sim.phase = BattlePhase::Complete;
        let result = sim.into_result().unwrap();
        let hero_report = result
            .units
            .iter()
            .find(|unit| unit.unit_id == "hero")
            .unwrap();
        assert_eq!(hero_report.confirmed_kills, 2);
        assert_eq!(hero_report.veteran_rank, 1);
    }

    #[test]
    fn reservation_yield_and_bounded_replan_keep_eight_actors_unique() {
        let mut baseline = MissionSimV1::from_seed(seed()).unwrap();
        for enemy in &mut baseline.enemies {
            enemy.hp = 0;
        }
        baseline.party[0].position = BattleGridPoint::new(5, 5);
        baseline.party[1].position = BattleGridPoint::new(1, 1);
        baseline.party[2].position = BattleGridPoint::new(2, 1);
        baseline.party[3].position = BattleGridPoint::new(3, 1);
        baseline.support_units = [
            BattleGridPoint::new(4, 5),
            BattleGridPoint::new(6, 5),
            BattleGridPoint::new(5, 4),
            BattleGridPoint::new(5, 6),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, position)| SupportUnit {
            unit_id: format!("traffic_support_{index}"),
            archetype_id: "field_support_drone".to_string(),
            role: "support".to_string(),
            position,
            hp: 100,
            damage: 1,
            armor: 0,
            attack_range: 4,
            ability_cooldown_ticks: 0,
            attack_interval_ticks: 20,
            supply: 1,
        })
        .collect();
        let hero = BTreeSet::from([baseline.party[0].unit_id.clone()]);
        let target = BattleGridPoint::new(9, 5);
        for _ in 0..7 {
            baseline.party[0].movement_budget_milli = MOVEMENT_TILE_COST;
            baseline.move_selected_toward(&hero, target, 0, None);
        }
        let intent = baseline.move_intents.get("hero").unwrap();
        assert!(
            intent.replan_count >= 1,
            "blocked actor must enter bounded replan"
        );
        assert_eq!(baseline.party[0].position, BattleGridPoint::new(5, 5));

        baseline.support_units[1].position = BattleGridPoint::new(8, 8);
        baseline.party[0].movement_budget_milli = MOVEMENT_TILE_COST;
        baseline.move_selected_toward(&hero, target, 0, None);
        assert_ne!(baseline.party[0].position, BattleGridPoint::new(5, 5));
        let positions = baseline
            .party
            .iter()
            .map(|unit| unit.position)
            .chain(baseline.support_units.iter().map(|unit| unit.position))
            .collect::<BTreeSet<_>>();
        assert_eq!(positions.len(), 8);
        assert!(baseline.tile_reservations.len() <= 1);

        let first_hash = baseline.snapshot_hash().unwrap();
        let checkpoint = SimCheckpointV1::capture(&baseline).unwrap();
        let resumed: SimCheckpointV1 =
            serde_json::from_slice(&serde_json::to_vec(&checkpoint).unwrap()).unwrap();
        resumed.validate().unwrap();
        assert_eq!(resumed.sim.snapshot_hash().unwrap(), first_hash);
    }

    #[test]
    fn opposing_traffic_uses_a_stable_side_step() {
        let seed = seed();
        let left = BattleGridPoint::new(5, 5);
        let right = BattleGridPoint::new(6, 5);
        let first = deterministic_yield_step(
            &seed,
            left,
            BattleGridPoint::new(9, 5),
            &BTreeSet::from([right]),
            &[],
        )
        .expect("left actor must find a deterministic side step");
        let repeated = deterministic_yield_step(
            &seed,
            left,
            BattleGridPoint::new(9, 5),
            &BTreeSet::from([right]),
            &[],
        )
        .unwrap();
        assert_eq!(first, repeated);
        assert_ne!(first, right);

        let second = deterministic_yield_step(
            &seed,
            right,
            BattleGridPoint::new(2, 5),
            &BTreeSet::from([left, first]),
            &[],
        )
        .expect("right actor must yield without entering the reserved side step");
        assert_ne!(second, left);
        assert_ne!(second, first);
    }

    #[test]
    fn adaptive_ai_observes_budget_selects_goals_and_replays_deterministically() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.step().unwrap();
        assert_eq!(sim.enemy_ai_goal, AiGoal::Scout);
        let history_before_invalid = sim.enemy_ai_history.clone();
        let mut invalid = order(&sim, RtsOrderKind::Move, sim.seed.map.approach_point);
        invalid.player_id = "intruder".to_string();
        assert!(sim.issue_order(invalid).is_err());
        assert_eq!(sim.enemy_ai_history, history_before_invalid);

        sim.resources_gathered = 200;
        sim.resources_available = 200;
        while sim.tick < 50 {
            sim.step().unwrap();
        }
        assert_eq!(sim.enemy_ai_goal, AiGoal::RaidEconomy);
        sim.resources_spent = 200;
        sim.resources_available = 0;
        sim.researched_techs.insert("signal_optics".to_string());
        while sim.tick < 100 {
            sim.step().unwrap();
        }
        assert_eq!(sim.enemy_ai_goal, AiGoal::CounterTech);
        assert!(sim
            .enemy_ai_history
            .iter()
            .any(|decision| decision.goal == AiGoal::RaidEconomy));

        let checkpoint = SimCheckpointV1::capture(&sim).unwrap();
        let mut first = checkpoint.sim.clone();
        let mut second = checkpoint.sim;
        for _ in 0..25 {
            first.step().unwrap();
            second.step().unwrap();
        }
        assert_eq!(first.enemy_ai_history, second.enemy_ai_history);
        assert_eq!(
            first.snapshot_hash().unwrap(),
            second.snapshot_hash().unwrap()
        );
    }

    #[test]
    fn supplied_expedition_resources_enter_authoritative_conservation() {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        campaign.equip_starter_weapon().unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        campaign.accept_first_contact_quest().unwrap();
        campaign.cycle_expedition_preparation().unwrap();
        campaign.cycle_expedition_preparation().unwrap();
        let supplied_seed = campaign.start_first_contact_battle(map()).unwrap();
        let sim = MissionSimV1::from_seed(supplied_seed).unwrap();
        assert_eq!(sim.resources_available, 50);
        assert_eq!(sim.resources_generated, 50);
        assert_eq!(
            sim.resources_available + sim.resources_spent,
            sim.resources_gathered
        );
        sim.validate().unwrap();
    }

    #[test]
    fn enemy_structures_are_explicit_attack_targets_and_never_take_proximity_damage() {
        let mut sim = MissionSimV1::from_seed(iron_delta_seed()).unwrap();
        let target_id = "enemy_supply_cache".to_string();
        let target_index = sim
            .enemy_structures
            .iter()
            .position(|structure| structure.structure_id == target_id)
            .unwrap();
        let target_position = sim.enemy_structures[target_index].position;
        let starting_hp = sim.enemy_structures[target_index].hp;
        for unit in &mut sim.party {
            unit.position = target_position;
            unit.attack_interval_ticks = 1;
        }
        for enemy in &mut sim.enemies {
            enemy.hp = 0;
        }
        sim.tick = 1;
        sim.step().unwrap();
        assert_eq!(sim.enemy_structures[target_index].hp, starting_hp);
        let selected = sim
            .party
            .iter()
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        sim.party_attack(&selected, Some(&target_id));
        assert!(sim.enemy_structures[target_index].hp < starting_hp);
    }

    #[test]
    fn all_twelve_enemy_archetype_abilities_execute_as_typed_authority() {
        let mut sim = MissionSimV1::from_seed(iron_delta_seed()).unwrap();
        for unit in &mut sim.party {
            unit.max_hp = 100_000;
            unit.hp = 100_000;
            unit.max_energy = 1_000;
            unit.energy = 1_000;
        }
        for archetype in UNIT_ROSTER {
            sim.enemies[0].skill_ids = vec![archetype.ability().rule_id().to_string()];
            sim.enemies[0].ability_cooldown_ticks = 0;
            sim.activate_enemy_ability(0, 0, archetype.ability());
        }
        assert_eq!(sim.enemy_ability_activations.len(), 12);
        assert!(UNIT_ROSTER.iter().all(|unit| {
            sim.enemy_ability_activations
                .contains_key(unit.ability().rule_id())
        }));
    }

    #[test]
    fn exported_replay_reconstructs_the_exact_authoritative_snapshot() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.issue_order(order(&sim, RtsOrderKind::Move, sim.seed.map.approach_point))
            .unwrap();
        for _ in 0..80 {
            sim.step().unwrap();
        }
        let replay = sim.export_replay().unwrap();
        let replayed = replay.replay_and_verify().unwrap();
        assert_eq!(replayed.tick, sim.tick);
        assert_eq!(
            replayed.snapshot_hash().unwrap(),
            sim.snapshot_hash().unwrap()
        );
        let directory = tempdir().unwrap();
        let path = directory.path().join("battle-replay.json");
        replay.save_atomic(&path).unwrap();
        let loaded = BattleReplayV1::load_verified(&path).unwrap();
        assert_eq!(loaded.final_snapshot_hash, replay.final_snapshot_hash);
    }

    #[test]
    fn long_match_replay_retains_more_than_the_legacy_512_order_window() {
        let mut durable_seed = seed();
        for unit in &mut durable_seed.party {
            unit.stats.max_hp = 100_000;
            unit.stats.armor = 100;
        }
        durable_seed.seed_hash = durable_seed.computed_hash().unwrap();
        let mut sim = MissionSimV1::from_seed(durable_seed).unwrap();
        for _ in 0..400 {
            let target = if sim.tick.is_multiple_of(2) {
                sim.seed.map.party_start
            } else {
                sim.seed.map.approach_point
            };
            for kind in [RtsOrderKind::Move, RtsOrderKind::Hold, RtsOrderKind::Move] {
                sim.issue_order(order(&sim, kind, target)).unwrap();
            }
            sim.step().unwrap();
        }
        assert_eq!(sim.replay_orders.len(), 1_200);
        let replay = sim.export_replay_v2().unwrap();
        assert_eq!(replay.entry_count(), 1_200);
        assert_eq!(replay.chunks.len(), 3);
        assert!(replay.seek_checkpoints.len() >= 3);
        let checkpoint_seek = replay.replay_until_tick(350).unwrap();
        let mut unindexed = replay.clone();
        unindexed.seek_checkpoints.clear();
        let full_seek = unindexed.replay_until_tick(350).unwrap();
        assert_eq!(
            checkpoint_seek.snapshot_hash().unwrap(),
            full_seek.snapshot_hash().unwrap()
        );
        let replayed = replay.replay_and_verify().unwrap();
        assert_eq!(
            replayed.snapshot_hash().unwrap(),
            sim.snapshot_hash().unwrap()
        );
        let directory = tempdir().unwrap();
        let path = directory.path().join("chunked-long-replay.json");
        replay.save_atomic(&path).unwrap();
        assert_eq!(
            BattleReplayV2::load_verified(&path).unwrap().entry_count(),
            1_200
        );
        let legacy_path = directory.path().join("legacy-auto-migrate.json");
        let mut legacy_sim = MissionSimV1::from_seed(seed()).unwrap();
        for _ in 0..12 {
            let target = legacy_sim.seed.map.approach_point;
            legacy_sim
                .issue_order(order(&legacy_sim, RtsOrderKind::Move, target))
                .unwrap();
            legacy_sim.step().unwrap();
        }
        legacy_sim
            .export_replay()
            .unwrap()
            .save_atomic(&legacy_path)
            .unwrap();
        let auto_migrated = BattleReplayV2::load_verified(&legacy_path).unwrap();
        assert_eq!(auto_migrated.entry_count(), 12);
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&fs::read(&legacy_path).unwrap()).unwrap()
                ["contract_version"],
            "trnm_battle_replay_v2"
        );
        let chunk_directory = directory.path().join("streamed-chunks");
        replay.save_chunk_directory(&chunk_directory).unwrap();
        let streamed = BattleReplayV2::load_chunk_directory_verified(&chunk_directory).unwrap();
        assert_eq!(streamed.final_snapshot_hash, replay.final_snapshot_hash);
    }

    #[test]
    fn enemy_builder_must_reach_the_authored_site_before_construction_progresses() {
        let mut sim = MissionSimV1::from_seed(iron_delta_seed()).unwrap();
        sim.enemy_resources_available = 1_000;
        sim.enemy_resources_generated = 1_000;
        for worker in sim.enemies.iter_mut().filter(|unit| unit.role == "worker") {
            worker.position = sim.seed.map.party_start;
        }
        while !sim
            .enemy_jobs
            .iter()
            .any(|job| job.kind == SimJobKind::BuildStructure)
            && sim.tick < 900
        {
            sim.step().unwrap();
        }
        let job = sim
            .enemy_jobs
            .iter()
            .find(|job| job.kind == SimJobKind::BuildStructure)
            .cloned()
            .unwrap();
        assert_eq!(job.kind, SimJobKind::BuildStructure);
        assert!(job.builder_id.is_some());
        assert!(sim.seed.map.in_bounds(job.target));
        let starting_ticks = job.remaining_ticks;
        sim.step().unwrap();
        if sim
            .enemy_jobs
            .iter()
            .find(|candidate| candidate.kind == SimJobKind::BuildStructure)
            .is_some_and(|candidate| candidate.remaining_ticks == starting_ticks)
        {
            let builder_id = job.builder_id.as_deref().unwrap();
            let target = job.target;
            assert!(sim
                .enemies
                .iter()
                .any(|unit| { unit.unit_id == builder_id && distance(unit.position, target) > 1 }));
        }
        let structures_before = sim.enemy_structures.len();
        while sim.enemy_structures.len() == structures_before && sim.tick < 900 {
            sim.step().unwrap();
        }
        assert!(sim.enemy_structures.len() > structures_before);
    }

