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
