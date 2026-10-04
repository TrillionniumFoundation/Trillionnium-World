    fn published_tick_recovery_simulation() -> MissionSimV1 {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../assets");
        let map = map::load_authoritative_map(&assets, "first_contact").unwrap();
        let mut campaign = CampaignSaveV1::default();
        let seed = prepare_campaign_seed(&mut campaign, "first_contact", map).unwrap();
        MissionSimV1::from_seed(seed).unwrap()
    }

    fn test_input_cursors() -> BTreeMap<String, u64> {
        BTreeMap::from([("player-a".to_string(), 2), ("player-b".to_string(), 2)])
    }

    fn test_high_water(
        tick: u64,
        next_sequence: u64,
        match_revision: u64,
        next_input_sequences: BTreeMap<String, u64>,
        snapshot_hash: String,
    ) -> PublishedTickHighWater {
        PublishedTickHighWater::new(
            Uuid::new_v4(),
            "host-a".to_string(),
            PublishedTickRecordInput {
                instance_id: "instance-a".to_string(),
                match_id: Uuid::new_v4(),
                actor_generation: Uuid::new_v4(),
                actor_epoch: 4,
                tick,
                next_sequence,
                match_revision,
                next_input_sequences,
                phase: "running".to_string(),
                receipts_replayable: true,
                snapshot_hash,
            },
        )
        .unwrap()
    }

    async fn test_terminal_orphan_journal(
        label: &str,
    ) -> (
        PathBuf,
        PublishedTickJournal,
        PublishedTickHighWater,
        PublishedTickHighWater,
    ) {
        let root = std::env::temp_dir().join(format!(
            "trnm-terminal-orphan-{label}-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        let journal = PublishedTickJournal::open(
            root.clone(),
            format!("host-terminal-orphan-{}", Uuid::new_v4()),
        )
        .unwrap();
        let match_id = Uuid::new_v4();
        let actor_generation = Uuid::new_v4();
        let cursors = test_input_cursors();
        let running = journal
            .new_record(PublishedTickRecordInput {
                instance_id: "instance-a".to_string(),
                match_id,
                actor_generation,
                actor_epoch: 4,
                tick: 19,
                next_sequence: 3,
                match_revision: 8,
                next_input_sequences: cursors.clone(),
                phase: "running".to_string(),
                receipts_replayable: true,
                snapshot_hash: "a".repeat(64),
            })
            .unwrap();
        journal
            .record(running.clone(), 3, 8, cursors.clone())
            .await
            .unwrap();
        let terminal = journal
            .new_record(PublishedTickRecordInput {
                instance_id: running.instance_id.clone(),
                match_id,
                actor_generation,
                actor_epoch: running.actor_epoch,
                tick: 20,
                next_sequence: 3,
                match_revision: 8,
                next_input_sequences: cursors,
                phase: "complete".to_string(),
                receipts_replayable: true,
                snapshot_hash: "b".repeat(64),
            })
            .unwrap();
        (root, journal, running, terminal)
    }

    #[test]
    fn online_map_allowlist_is_explicit() {
        assert_eq!(
            mission_for_map("iron_delta").unwrap().map_id(),
            "iron_delta"
        );
        assert_eq!(
            mission_for_map("first_contact").unwrap().map_id(),
            "first_contact"
        );
        assert!(mission_for_map("../../secret").is_err());
    }

    #[test]
    fn start_match_transport_retry_is_idempotent_only_for_current_running_authority() {
        assert!(start_match_retry_is_idempotent("running", 1, 0, true));
        assert!(!start_match_retry_is_idempotent("running", 8, 0, true));
        assert!(!start_match_retry_is_idempotent("waiting", 0, 0, true));
        assert!(!start_match_retry_is_idempotent("complete", 8, 0, true));
        assert!(!start_match_retry_is_idempotent("running", 1, 1, true));
        assert!(!start_match_retry_is_idempotent(
            "running",
            u64::MAX,
            u64::MAX,
            true,
        ));
        assert!(!start_match_retry_is_idempotent("running", 1, 0, false));
    }

    #[test]
    fn authority_materializes_base_and_overlay_maps() {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../assets");
        for map_id in [
            "first_contact",
            "iron_delta",
            "night_watch_crossing",
            "glass_basin",
            "ember_orchard",
            "salt_marsh",
            "cinder_crown",
        ] {
            let map = map::load_authoritative_map(&assets, map_id)
                .unwrap_or_else(|error| panic!("{map_id}: {error}"));
            assert_eq!((map.width, map.height), (40, 24));
            assert!(!map.enemy_spawns.is_empty());
        }
    }

    #[test]
    fn slot_keys_are_bounded_and_portable() {
        assert!(validate_slot_key("main_01").is_ok());
        assert!(validate_slot_key("").is_err());
        assert!(validate_slot_key("bad/slot").is_err());
    }

    #[test]
    fn command_ids_are_bounded_and_portable() {
        assert!(validate_command_id("native:match.player_01-command").is_ok());
        assert!(validate_command_id("").is_err());
        assert!(validate_command_id("bad/command").is_err());
        assert!(validate_command_id(&"x".repeat(161)).is_err());
    }

    #[test]
    fn campaign_hash_changes_with_authoritative_revision() {
        let first = CampaignSaveV1::default();
        let mut second = first.clone();
        second.revision += 1;
        assert_ne!(hash_json(&first).unwrap(), hash_json(&second).unwrap());
    }

    #[test]
    fn online_campaign_starts_from_cex_connected_authority() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .bind_cex_economy_account("player-a", "00000000-0000-0000-0000-000000000001")
            .unwrap();
        assert_eq!(
            campaign.economy_mode,
            trnm_campaign_core::EconomyMode::CexConnected
        );
        campaign.prepare_standalone_skirmish().unwrap();
        assert_eq!(
            campaign.room,
            trnm_campaign_core::CampaignRoom::ExpeditionGate
        );
    }

    #[test]
    fn public_bind_fails_closed_without_production_security_boundary() {
        assert!(validate_operations_bind_addr("127.0.0.1:7005".parse().unwrap()).is_ok());
        assert!(validate_operations_bind_addr("[::1]:7005".parse().unwrap()).is_ok());
        assert!(validate_operations_bind_addr("0.0.0.0:7005".parse().unwrap()).is_err());
        assert!(validate_operations_bind_addr("192.0.2.10:7005".parse().unwrap()).is_err());
    }

    #[test]
    fn sigterm_drain_classifies_commands_as_retryable_service_unavailable() {
        assert!(command_drain_error(false, false).is_none());
        for error in [
            command_drain_error(true, false).unwrap(),
            command_drain_error(false, true).unwrap(),
        ] {
            assert_eq!(error.status, StatusCode::SERVICE_UNAVAILABLE);
            assert!(error.body.recoverable);
            let response = error.into_response();
            assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(
                response.headers().get(header::RETRY_AFTER),
                Some(&HeaderValue::from_static("1"))
            );
        }
    }

    #[test]
    fn production_authority_clock_is_exactly_ten_hz() {
        let interval = production_authority_tick_interval();
        assert_eq!(TICKS_PER_SECOND, 10);
        assert_eq!(interval, Duration::from_millis(100));
        assert_eq!(interval.saturating_mul(1_800), Duration::from_secs(180));
    }

    #[test]
    fn accelerated_clock_is_explicitly_test_only() {
        assert_eq!(
            resolve_authority_tick_interval(None, false).unwrap(),
            Duration::from_millis(100)
        );
        assert!(resolve_authority_tick_interval(Some(50), false).is_err());
        assert_eq!(
            resolve_authority_tick_interval(Some(20), true).unwrap(),
            Duration::from_millis(20)
        );
        assert!(resolve_authority_tick_interval(Some(0), true).is_err());
    }

    #[test]
    fn readiness_fails_closed_on_authority_clock_degradation() {
        let tick_interval = Duration::from_millis(100);
        let snapshot = |drift_ticks: f64,
                        lateness_ticks: f64,
                        last_wake_age_ms: f64,
                        sample_count: usize| AuthorityClockSnapshot {
            elapsed_ms: 1_000.0,
            wake_count: 10,
            cumulative_drift_ticks: 0.0,
            window_sample_count: sample_count,
            window_drift_ticks: Some(drift_ticks),
            latest_lateness_ticks: Some(lateness_ticks),
            max_recent_lateness_ticks: Some(lateness_ticks),
            last_wake_age_ms: Some(last_wake_age_ms),
        };

        assert!(authority_clock_is_operational(
            Some(&snapshot(1.99, 1.99, 199.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(authority_clock_is_operational(
            Some(&snapshot(-1.99, 0.0, 0.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(
            Some(&snapshot(2.0, 0.0, 0.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(
            Some(&snapshot(0.0, 2.0, 0.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(
            Some(&snapshot(0.0, 0.0, 200.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(
            Some(&snapshot(0.0, 0.0, 0.0, AUTHORITY_CLOCK_MIN_SAMPLES - 1)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(
            Some(&snapshot(f64::NAN, 0.0, 0.0, AUTHORITY_CLOCK_MIN_SAMPLES)),
            tick_interval
        ));
        assert!(!authority_clock_is_operational(None, tick_interval));
    }

    #[test]
    fn authority_clock_window_self_heals_after_transient_stall() {
        let telemetry = AuthorityClockTelemetry::default();
        let tick_interval = Duration::from_millis(100);
        let started_at = Instant::now();
        telemetry.reset(started_at);

        for tick in 1..=AUTHORITY_CLOCK_WINDOW_TICKS + 1 {
            let observed_at = started_at + tick_interval.saturating_mul(tick as u32);
            telemetry.record_wake(observed_at, observed_at, tick_interval);
        }
        let healthy = telemetry
            .snapshot(
                started_at
                    + tick_interval.saturating_mul((AUTHORITY_CLOCK_WINDOW_TICKS + 1) as u32),
                tick_interval,
            )
            .unwrap();
        assert!(authority_clock_is_operational(
            Some(&healthy),
            tick_interval
        ));

        let scheduled_at =
            started_at + tick_interval.saturating_mul((AUTHORITY_CLOCK_WINDOW_TICKS + 2) as u32);
        let recovered_epoch = started_at + Duration::from_secs(60);
        telemetry.record_wake(scheduled_at, recovered_epoch, tick_interval);
        let stalled = telemetry.snapshot(recovered_epoch, tick_interval).unwrap();
        assert!(!authority_clock_is_operational(
            Some(&stalled),
            tick_interval
        ));

        for tick in 1..=AUTHORITY_CLOCK_WINDOW_TICKS + 1 {
            let observed_at = recovered_epoch + tick_interval.saturating_mul(tick as u32);
            telemetry.record_wake(observed_at, observed_at, tick_interval);
        }
        let recovered = telemetry
            .snapshot(
                recovered_epoch
                    + tick_interval.saturating_mul((AUTHORITY_CLOCK_WINDOW_TICKS + 1) as u32),
                tick_interval,
            )
            .unwrap();
        assert!(recovered.cumulative_drift_ticks < -500.0);
        assert_eq!(recovered.window_drift_ticks, Some(0.0));
        assert!(authority_clock_is_operational(
            Some(&recovered),
            tick_interval
        ));
    }

    #[test]
    fn authority_clock_window_rejects_sustained_slow_cadence() {
        let telemetry = AuthorityClockTelemetry::default();
        let tick_interval = Duration::from_millis(100);
        let started_at = Instant::now();
        telemetry.reset(started_at);

        for tick in 1..=10_u32 {
            let observed_at = started_at + Duration::from_millis(u64::from(tick) * 150);
            telemetry.record_wake(observed_at, observed_at, tick_interval);
        }
        let snapshot = telemetry
            .snapshot(started_at + Duration::from_millis(1_500), tick_interval)
            .unwrap();
        assert_eq!(snapshot.window_drift_ticks, Some(-4.5));
        assert!(!authority_clock_is_operational(
            Some(&snapshot),
            tick_interval
        ));
    }

    #[test]
    fn match_actor_clock_self_heals_and_allows_one_bounded_command_publication_window() {
        let tick_interval = Duration::from_millis(100);
        let telemetry = AuthorityClockTelemetry::default();
        let started_at = Instant::now();
        telemetry.reset(started_at);
        telemetry.record_wake(
            started_at + tick_interval,
            started_at + Duration::from_secs(3),
            tick_interval,
        );
        for tick in 1..=AUTHORITY_CLOCK_WINDOW_TICKS + 1 {
            let observed_at =
                started_at + Duration::from_secs(3) + tick_interval.saturating_mul(tick as u32);
            telemetry.record_wake(observed_at, observed_at, tick_interval);
        }
        let snapshot = telemetry
            .snapshot(
                started_at
                    + Duration::from_secs(3)
                    + tick_interval.saturating_mul((AUTHORITY_CLOCK_WINDOW_TICKS + 1) as u32),
                tick_interval,
            )
            .unwrap();
        assert!(match_actor_clock_is_operational(
            Some(&snapshot),
            749.0,
            tick_interval
        ));
        assert!(!match_actor_clock_is_operational(
            Some(&snapshot),
            750.0,
            tick_interval
        ));
    }

    #[test]
    fn match_actor_clock_warmup_is_bounded_and_fails_closed_on_bad_samples() {
        let tick_interval = Duration::from_millis(100);
        assert_eq!(
            match_actor_clock_warmup_limit(tick_interval),
            Duration::from_millis(500)
        );
        assert!(match_actor_clock_warmup_is_operational(
            None,
            Duration::from_millis(199),
            0.0,
            tick_interval,
        ));
        assert!(!match_actor_clock_warmup_is_operational(
            None,
            Duration::from_millis(200),
            0.0,
            tick_interval,
        ));

        let mut first_wake = AuthorityClockSnapshot {
            elapsed_ms: 150.0,
            wake_count: 1,
            cumulative_drift_ticks: -0.5,
            window_sample_count: 1,
            window_drift_ticks: None,
            latest_lateness_ticks: Some(0.01),
            max_recent_lateness_ticks: Some(0.01),
            last_wake_age_ms: Some(50.0),
        };
        assert!(match_actor_clock_warmup_is_operational(
            Some(&first_wake),
            Duration::from_millis(150),
            100.0,
            tick_interval,
        ));
        assert!(!match_actor_clock_warmup_is_operational(
            Some(&first_wake),
            Duration::from_millis(500),
            100.0,
            tick_interval,
        ));
        assert!(!match_actor_clock_warmup_is_operational(
            Some(&first_wake),
            Duration::from_millis(150),
            750.0,
            tick_interval,
        ));

        first_wake.latest_lateness_ticks = Some(MAX_AUTHORITY_CLOCK_ABS_DRIFT_TICKS);
        assert!(!match_actor_clock_warmup_is_operational(
            Some(&first_wake),
            Duration::from_millis(150),
            100.0,
            tick_interval,
        ));
        first_wake.latest_lateness_ticks = Some(0.01);
        first_wake.wake_count = 2;
        assert!(!match_actor_clock_warmup_is_operational(
            Some(&first_wake),
            Duration::from_millis(150),
            100.0,
            tick_interval,
        ));

        let mature = AuthorityClockSnapshot {
            elapsed_ms: 300.0,
            wake_count: 3,
            cumulative_drift_ticks: 0.0,
            window_sample_count: AUTHORITY_CLOCK_MIN_SAMPLES,
            window_drift_ticks: Some(0.0),
            latest_lateness_ticks: Some(0.01),
            max_recent_lateness_ticks: Some(0.01),
            last_wake_age_ms: Some(1.0),
        };
        assert!(match_actor_clock_warmup_is_operational(
            Some(&mature),
            Duration::from_millis(300),
            100.0,
            tick_interval,
        ));
    }

    #[test]
    fn actor_registry_coverage_accepts_only_bounded_initializing_or_terminalizing_edges() {
        assert!(match_actor_registry_coverage_is_operational(1, 1, 1, 0));
        assert!(match_actor_registry_coverage_is_operational(1, 0, 0, 1));
        assert!(match_actor_registry_coverage_is_operational(1, 0, 1, 0));
        assert!(match_actor_registry_coverage_is_operational(0, 0, 1, 0));
        assert!(!match_actor_registry_coverage_is_operational(0, 1, 1, 0));
        assert!(!match_actor_registry_coverage_is_operational(2, 1, 1, 0));
        assert!(!match_actor_registry_coverage_is_operational(-1, 0, 0, 0));
    }

    #[test]
    fn readiness_transition_ownership_requires_an_exact_unique_match_set() {
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        let owners = BTreeSet::from([first, second]);
        assert!(match_ids_are_exactly_owned(&[first], 1, &owners));
        assert!(match_ids_are_exactly_owned(&[first, second], 2, &owners));
        assert!(!match_ids_are_exactly_owned(&[first], 2, &owners));
        assert!(!match_ids_are_exactly_owned(&[first, first], 2, &owners));
        assert!(!match_ids_are_exactly_owned(&[Uuid::new_v4()], 1, &owners));
    }

    #[test]
    fn match_actor_clock_rejects_sustained_slow_cadence() {
        let tick_interval = Duration::from_millis(100);
        let telemetry = AuthorityClockTelemetry::default();
        let started_at = Instant::now();
        telemetry.reset(started_at);
        for tick in 1..=10_u32 {
            let observed_at = started_at + Duration::from_millis(u64::from(tick) * 150);
            telemetry.record_wake(observed_at, observed_at, tick_interval);
        }
        let snapshot = telemetry
            .snapshot(started_at + Duration::from_millis(1_500), tick_interval)
            .unwrap();
        assert!(!match_actor_clock_is_operational(
            Some(&snapshot),
            0.0,
            tick_interval
        ));
    }

    #[test]
    fn readiness_fails_closed_on_database_pool_saturation() {
        assert!(database_pool_is_operational(1, 0, 8));
        assert!(database_pool_is_operational(8, 1, 8));
        assert!(!database_pool_is_operational(8, 0, 8));
        assert_eq!(GAME_SERVER_DATABASE_MIN_CONNECTIONS, 12);
        assert_eq!(GAME_SERVER_DATABASE_MAX_CONNECTIONS, 12);
        assert_eq!(READINESS_DATABASE_MIN_CONNECTIONS, 4);
        assert_eq!(READINESS_DATABASE_MAX_CONNECTIONS, 12);
    }

    #[test]
    fn terminal_checkpoint_lock_window_never_aligns_mixed_phase_authority() {
        assert!(authority_metadata_matches(
            OnlineMatchPhase::Running,
            None,
            "not_ready",
            OnlineMatchPhase::Running,
            None,
            "not_ready",
        ));
        assert!(!authority_metadata_matches(
            OnlineMatchPhase::Complete,
            Some("result-a"),
            "pending",
            OnlineMatchPhase::Running,
            None,
            "not_ready",
        ));
        assert!(!authority_metadata_matches(
            OnlineMatchPhase::Running,
            None,
            "not_ready",
            OnlineMatchPhase::Complete,
            Some("result-a"),
            "pending",
        ));
        assert!(authority_metadata_matches(
            OnlineMatchPhase::Complete,
            Some("result-a"),
            "pending",
            OnlineMatchPhase::Complete,
            Some("result-a"),
            "pending",
        ));
        assert!(authority_metadata_matches(
            OnlineMatchPhase::Complete,
            Some("result-a"),
            "pending",
            OnlineMatchPhase::Complete,
            Some("result-a"),
            "settled",
        ));
        assert!(!authority_metadata_matches(
            OnlineMatchPhase::Complete,
            Some("result-a"),
            "settled",
            OnlineMatchPhase::Complete,
            Some("result-a"),
            "pending",
        ));
        assert_eq!(
            effective_published_settlement_state("pending", "settled"),
            "settled"
        );
        assert_eq!(
            effective_published_settlement_state("settled", "pending"),
            "settled"
        );
    }

    #[tokio::test]
    async fn terminal_watch_update_stays_unaligned_until_durable_phase_commits() {
        let (published_tx, mut published_rx) = watch::channel((
            OnlineMatchPhase::Running,
            None::<String>,
            "not_ready".to_string(),
        ));
        published_tx.send_replace((
            OnlineMatchPhase::Complete,
            Some("result-a".to_string()),
            "pending".to_string(),
        ));
        published_rx.changed().await.unwrap();
        let published = published_rx.borrow().clone();
        assert!(!authority_metadata_matches(
            published.0,
            published.1.as_deref(),
            &published.2,
            OnlineMatchPhase::Running,
            None,
            "not_ready",
        ));
        assert!(authority_metadata_matches(
            published.0,
            published.1.as_deref(),
            &published.2,
            OnlineMatchPhase::Complete,
            Some("result-a"),
            "pending",
        ));
    }

    #[test]
    fn published_terminal_authority_overwrites_running_stream_view_metadata() {
        let mut view = OnlineMatchView {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            match_id: Uuid::nil().to_string(),
            join_code: "TEST".to_string(),
            phase: OnlineMatchPhase::Running,
            match_revision: 4,
            authoritative_tick: 10,
            next_sequence: 3,
            map_id: "first_contact".to_string(),
            match_mode: "coop_vs_ai".to_string(),
            rules_version: "test".to_string(),
            seed_hash: "seed".to_string(),
            snapshot_hash: "snapshot".to_string(),
            members: Vec::new(),
            result_hash: None,
            settlement_state: "not_ready".to_string(),
        };
        apply_published_authority_view(
            &mut view,
            OnlineMatchPhase::Complete,
            Some("result-a".to_string()),
            "settled".to_string(),
        );
        assert_eq!(view.phase, OnlineMatchPhase::Complete);
        assert_eq!(view.result_hash.as_deref(), Some("result-a"));
        assert_eq!(view.settlement_state, "settled");
    }

    #[test]
    fn pending_command_barrier_blocks_every_checkpoint_exit() {
        for boundary in [
            ActorCheckpointBoundary::Periodic,
            ActorCheckpointBoundary::Shutdown,
            ActorCheckpointBoundary::Terminal,
            ActorCheckpointBoundary::Fenced,
            ActorCheckpointBoundary::CheckpointFailure,
        ] {
            assert!(!actor_checkpoint_allowed(boundary, true));
        }
        assert!(actor_checkpoint_allowed(
            ActorCheckpointBoundary::Periodic,
            false
        ));
        assert!(actor_checkpoint_allowed(
            ActorCheckpointBoundary::Shutdown,
            false
        ));
        assert!(actor_checkpoint_allowed(
            ActorCheckpointBoundary::Terminal,
            false
        ));
        assert!(!actor_checkpoint_allowed(
            ActorCheckpointBoundary::Fenced,
            false
        ));
        assert!(!actor_checkpoint_allowed(
            ActorCheckpointBoundary::CheckpointFailure,
            false
        ));
    }

    #[test]
    fn different_matches_reserve_independent_initialization_slots() {
        let mut registry = MatchActorRegistry::default();
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        assert!(matches!(
            reserve_match_actor_initialization(&mut registry, first, 4).unwrap(),
            MatchActorEnsureDecision::Initialize(_)
        ));
        assert!(matches!(
            reserve_match_actor_initialization(&mut registry, second, 4).unwrap(),
            MatchActorEnsureDecision::Initialize(_)
        ));
        assert!(matches!(
            reserve_match_actor_initialization(&mut registry, first, 4).unwrap(),
            MatchActorEnsureDecision::Wait(_)
        ));
        assert_eq!(registry.initializing.len(), 2);
    }

    #[test]
    fn bounded_actor_initialization_owns_its_running_high_water_readiness_window() {
        let mut registry = MatchActorRegistry::default();
        let match_id = Uuid::new_v4();
        let initialization =
            match reserve_match_actor_initialization(&mut registry, match_id, 1).unwrap() {
                MatchActorEnsureDecision::Initialize(initialization) => initialization,
                _ => panic!("first reservation must initialize the match"),
            };
        let recorded = [match_id];
        let tracked = actor_tracked_published_tick_match_ids(&registry, Instant::now());
        assert_eq!(
            count_untracked_published_tick_records(&recorded, &tracked),
            0
        );
        assert!(terminal_orphan_recovery_is_operational(true, 0));

        let timed_out_at = initialization.started_at + MATCH_ACTOR_INITIALIZATION_TIMEOUT;
        let tracked = actor_tracked_published_tick_match_ids(&registry, timed_out_at);
        assert_eq!(
            count_untracked_published_tick_records(&recorded, &tracked),
            1
        );
        assert!(!terminal_orphan_recovery_is_operational(true, 1));
    }

    #[tokio::test]
    async fn cancelled_match_initialization_releases_its_capacity_reservation() {
        let registry = Arc::new(RwLock::new(MatchActorRegistry::default()));
        let match_id = Uuid::new_v4();
        let initialization = {
            let mut locked = registry.write().await;
            match reserve_match_actor_initialization(&mut locked, match_id, 1).unwrap() {
                MatchActorEnsureDecision::Initialize(initialization) => initialization,
                _ => panic!("first reservation must initialize the match"),
            }
        };
        let mut ready = initialization.ready.subscribe();
        drop(MatchActorInitializationReservation::new(
            registry.clone(),
            match_id,
            &initialization,
        ));
        tokio::time::timeout(Duration::from_secs(1), ready.changed())
            .await
            .unwrap()
            .unwrap();
        assert!(*ready.borrow());
        assert!(!registry.read().await.initializing.contains_key(&match_id));
    }

