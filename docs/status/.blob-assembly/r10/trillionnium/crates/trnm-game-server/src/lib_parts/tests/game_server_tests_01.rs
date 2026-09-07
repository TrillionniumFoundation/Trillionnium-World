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
__TRNM_SLOT_1__
__TRNM_SLOT_2__
__TRNM_SLOT_3__
