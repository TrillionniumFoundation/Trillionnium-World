    #[test]
    fn multi_map_faction_swap_multi_seed_balance_matrix_runs_real_authority() {
        let base = iron_delta_seed();
        let missions = [
            CampaignMission::IronDeltaSkirmish,
            CampaignMission::NightWatchCrossingSkirmish,
            CampaignMission::GlassBasinSkirmish,
            CampaignMission::EmberOrchardSkirmish,
        ];
        let mut seeds = Vec::new();
        for mission in missions {
            for swapped in [false, true] {
                for spawn_swapped in [false, true] {
                    for simulation_seed in 1..=4 {
                        let mut map = authored_map(mission.map_id());
                        if spawn_swapped {
                            let original_party = map.party_start;
                            std::mem::swap(&mut map.party_start, &mut map.objective);
                            for (index, spawn) in map.enemy_spawns.iter_mut().enumerate() {
                                spawn.position = BattleGridPoint::new(
                                    original_party.x + 1 + index as i16 % 3,
                                    original_party.y + index as i16 / 3,
                                );
                            }
                        }
                        let mut seed = base.clone();
                        seed.map_id = mission.map_id().to_string();
                        seed.map = map;
                        seed.mission = MissionDefinition::for_mission(mission, &seed.map);
                        seed.battle_id = format!(
                            "balance-{}-{}-{}-{simulation_seed}",
                            mission.map_id(),
                            u8::from(swapped),
                            u8::from(spawn_swapped),
                        );
                        seed.difficulty = CampaignDifficulty::Standard;
                        seed.skirmish.player_faction = if swapped {
                            RtsFaction::AshenCompact
                        } else {
                            RtsFaction::MirrorCoalition
                        };
                        seed.skirmish.enemy_faction = seed.skirmish.player_faction.opponent();
                        seed.skirmish.simulation_seed = simulation_seed;
                        seed.skirmish.victory_mode = SkirmishVictoryMode::Score;
                        seed.skirmish.score_target = 40;
                        seed.seed_hash = seed.computed_hash().unwrap();
                        seed.validate().unwrap();
                        seeds.push(seed);
                    }
                }
            }
        }
        let matrix = run_skirmish_balance_matrix(&seeds, 500).unwrap();
        assert_eq!(matrix.samples.len(), 64);
        assert_eq!(
            matrix
                .samples
                .iter()
                .map(|sample| sample.map_id.as_str())
                .collect::<BTreeSet<_>>()
                .len(),
            4
        );
        assert!(
            matrix.faction_pressure_delta_permille <= 500,
            "faction pressure delta {} permille exceeds the automated alpha band",
            matrix.faction_pressure_delta_permille
        );
        assert!(
            matrix.terminal_sample_count >= 48,
            "at least 75% of real-map matches must reach a measured terminal outcome: {}/64",
            matrix.terminal_sample_count
        );
        assert!(matrix.mirror_wins > 0 && matrix.ashen_wins > 0);
        assert!(matrix.average_terminal_ticks > 0);
        assert!(matrix.samples.iter().all(|sample| {
            sample.player_resources_spent > 0 && sample.enemy_resources_spent > 0
        }));
        assert!(matrix
            .samples
            .iter()
            .any(|sample| { sample.player_tech_count > 0 && sample.enemy_tech_count > 0 }));
        assert_eq!(matrix.terminal_samples_by_map.len(), 4);
        assert!(matrix.faction_win_delta_permille <= 900);
        assert!(matrix.average_resource_efficiency_delta_permille <= 900);
        assert!(matrix.average_tech_count_delta_permille <= 900);
    }

    #[test]
    fn difficulty_scales_enemy_pressure_and_ai_cadence_deterministically() {
        let base = seed();
        let with_difficulty = |difficulty| {
            let mut value = base.clone();
            value.difficulty = difficulty;
            value.seed_hash = value.computed_hash().unwrap();
            value
        };
        let mut story =
            MissionSimV1::from_seed(with_difficulty(CampaignDifficulty::Story)).unwrap();
        let mut standard =
            MissionSimV1::from_seed(with_difficulty(CampaignDifficulty::Standard)).unwrap();
        let mut veteran =
            MissionSimV1::from_seed(with_difficulty(CampaignDifficulty::Veteran)).unwrap();
        assert!(story.enemies[0].max_hp < standard.enemies[0].max_hp);
        assert!(standard.enemies[0].max_hp < veteran.enemies[0].max_hp);
        assert!(story.relay_guard_max_hp < standard.relay_guard_max_hp);
        assert!(standard.relay_guard_max_hp < veteran.relay_guard_max_hp);
        for _ in 0..105 {
            story.step().unwrap();
            standard.step().unwrap();
            veteran.step().unwrap();
        }
        assert!(story.enemy_ai_decision_index < standard.enemy_ai_decision_index);
        assert!(standard.enemy_ai_decision_index < veteran.enemy_ai_decision_index);

        let checkpoint = SimCheckpointV1::capture(&veteran).unwrap();
        let mut first = checkpoint.sim.clone();
        let mut second = checkpoint.sim;
        for _ in 0..35 {
            first.step().unwrap();
            second.step().unwrap();
        }
        assert_eq!(
            first.snapshot_hash().unwrap(),
            second.snapshot_hash().unwrap()
        );
    }

    #[test]
    fn ranked_pvp_gives_each_human_an_opposing_authority_side() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        let guest_ids = sim
            .party
            .iter()
            .skip(2)
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        sim.enable_human_enemy_authority(&guest_ids).unwrap();
        assert!(sim.human_enemy_authority);
        assert_eq!(sim.party.len(), 2);
        assert_eq!(sim.enemies.len(), 2);
        let guest_target = sim.party[0].unit_id.clone();
        let guest_subjects = sim
            .enemies
            .iter()
            .map(|unit| unit.unit_id.clone())
            .collect::<Vec<_>>();
        let mut guest_order = RtsFrameOrder::new(
            1,
            "enemy-player",
            guest_subjects,
            RtsOrderKind::Attack,
            RtsOrderSource::LocalInput,
        );
        guest_order.target_actor_id = Some(guest_target);
        sim.issue_human_enemy_order(guest_order).unwrap();
        for _ in 0..20 {
            sim.step().unwrap();
        }
        assert!(sim.enemy_last_order_frame.is_some());
        assert!(sim.distinct_order_kinds.contains("enemy:attack"));
        assert!(sim.validate().is_ok());
    }
    fn assert_sim_error_preserves_state<T: std::fmt::Debug>(
        sim: &mut MissionSimV1,
        operation: impl FnOnce(&mut MissionSimV1) -> Result<T, SimError>,
    ) -> SimError {
        let before_bytes = serde_json::to_vec(sim).expect("simulation state serializes");
        let before_hash = sim.snapshot_hash().expect("simulation state hashes");
        let error = operation(sim).expect_err("operation must fail");
        assert_eq!(
            before_bytes,
            serde_json::to_vec(sim).expect("rejected state still serializes"),
            "RTS command returned Err after changing authoritative state"
        );
        assert_eq!(
            before_hash,
            sim.snapshot_hash().expect("rejected state still hashes"),
            "RTS command returned Err after changing the snapshot hash"
        );
        error
    }

    #[test]
    fn rejected_all_party_pvp_partition_preserves_rosters_and_snapshot() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        let all_living_party = sim
            .party
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        let original_party_ids = sim
            .party
            .iter()
            .map(|unit| unit.unit_id.clone())
            .collect::<Vec<_>>();
        let original_enemy_ids = sim
            .enemies
            .iter()
            .map(|unit| unit.unit_id.clone())
            .collect::<Vec<_>>();

        let error = assert_sim_error_preserves_state(&mut sim, |candidate| {
            candidate.enable_human_enemy_authority(&all_living_party)
        });
        assert!(error.to_string().contains("at least one unit on each side"));
        assert_eq!(
            original_party_ids,
            sim.party
                .iter()
                .map(|unit| unit.unit_id.clone())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            original_enemy_ids,
            sim.enemies
                .iter()
                .map(|unit| unit.unit_id.clone())
                .collect::<Vec<_>>()
        );
        assert!(!sim.human_enemy_authority);
    }

    #[test]
    fn insufficient_resources_build_preserves_guard_resources_replay_and_cursor() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.resources_available = 0;
        let target = sim.party[0].position;
        let build = order(&sim, RtsOrderKind::Build, target);
        let guards_before = sim
            .party
            .iter()
            .map(|unit| {
                (
                    unit.unit_id.clone(),
                    unit.guard_ticks,
                    unit.ability_cooldown_ticks,
                )
            })
            .collect::<Vec<_>>();
        let resources_before = (
            sim.resources_available,
            sim.resources_spent,
            sim.resources_generated,
        );
        let replay_before = sim.replay_orders.len();
        let jobs_before = sim.jobs.len();
        let command_records_before = sim.authority_job_commands.len();
        let order_count_before = sim.order_count;
        let event_count_before = sim.event_count;
        let deterministic_cursor_before = sim.enemy_ai_decision_index;

        let error =
            assert_sim_error_preserves_state(&mut sim, |candidate| candidate.issue_order(build));
        assert!(matches!(error, SimError::Order(_)));
        assert_eq!(
            guards_before,
            sim.party
                .iter()
                .map(|unit| (
                    unit.unit_id.clone(),
                    unit.guard_ticks,
                    unit.ability_cooldown_ticks
                ))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            resources_before,
            (
                sim.resources_available,
                sim.resources_spent,
                sim.resources_generated,
            )
        );
        assert_eq!(replay_before, sim.replay_orders.len());
        assert_eq!(jobs_before, sim.jobs.len());
        assert_eq!(command_records_before, sim.authority_job_commands.len());
        assert_eq!(order_count_before, sim.order_count);
        assert_eq!(event_count_before, sim.event_count);
        assert_eq!(deterministic_cursor_before, sim.enemy_ai_decision_index);
    }
