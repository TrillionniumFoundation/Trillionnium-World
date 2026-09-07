    #[test]
    fn sigterm_during_match_initialization_prevents_actor_installation() {
        assert!(match_actor_install_is_allowed(false, false));
        assert!(!match_actor_install_is_allowed(true, false));
        assert!(!match_actor_install_is_allowed(false, true));
        assert!(!match_actor_install_is_allowed(true, true));
    }

    #[tokio::test]
    async fn actor_started_after_shutdown_observes_the_current_watch_value() {
        let (shutdown, mut receiver) = watch::channel(false);
        shutdown.send_replace(true);
        tokio::time::timeout(
            Duration::from_millis(10),
            shutdown_changed_or_current(&mut receiver),
        )
        .await
        .unwrap()
        .unwrap();
    }

    #[test]
    fn pending_command_lane_requires_durable_commit_before_publication() {
        assert!(!actor_command_lane_publish_allowed(true, false));
        validate_recovered_command_timing(
            &simulation,
            scheduled_tick as i64,
            None,
            serde_json::to_value(&scheduled_order).unwrap(),
            "scheduled",
        )
        .unwrap();
        assert!(validate_recovered_command_timing(
            &simulation,
            scheduled_tick.saturating_add(1) as i64,
            None,
            serde_json::to_value(&scheduled_order).unwrap(),
            "scheduled",
        )
        .is_err());
    }

    #[test]
    fn published_tick_recovery_bridges_one_committed_command_without_tick_regression() {
        let bridge = published_tick_recovery_simulation();
        let mut expected_old_lane = bridge.clone();
        for _ in 0..5 {
            expected_old_lane.step().unwrap();
            &high_water
        ));
        view.next_sequence = Some(3);
        view.match_revision = Some(8);
        view.checkpoint_sequence = Some(3);
        view.next_input_sequences = test_input_cursors();
        view.assigned_instance_epoch = Some(high_water.actor_epoch + 1);
        assert!(!terminal_authority_succeeds_running_high_water(
            &view,
            &high_water
        ));
        view.assigned_instance_epoch = Some(high_water.actor_epoch);
        view.assigned_physical_host_id = Some("host-b".to_string());
        assert!(!terminal_authority_succeeds_running_high_water(
            &view,
            &high_water
        ));
        view.assigned_physical_host_id = Some(high_water.physical_host_id.clone());
        replay_running_high_water_to_terminal(
            source,
            &high_water,
            None,
            &terminal,
            &terminal_hash,
        );
        let actor_tracked = actor_tracked_published_tick_match_ids(&registry, Instant::now());
        let untracked = count_untracked_published_tick_records(
            &journal.recorded_match_ids().unwrap(),
            &actor_tracked,
        );
        assert_eq!(untracked, 1);
        assert!(!terminal_orphan_recovery_is_operational(true, untracked));
        assert!(!terminal_orphan_recovery_is_operational(false, 0));

        drop(journal);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn failed_closed_running_high_water_remains_a_tracked_rollback_witness() {
        let (root, journal, running, _terminal) =
            test_terminal_orphan_journal("failed-closed").await;
        let authority = FakeTerminalOrphanAuthority {
            recovered: None,
            marker: Mutex::new(None),
            fail_acknowledgement: false,
            durably_failed_closed: true,
        };
        assert!(MIGRATION_V12.contains("where m.terminal_publication_state is null"));
        assert!(MIGRATION_V12.contains("phase = 'complete'"));
        assert!(MIGRATION_V12
            .contains("terminal_publication_state in ('acknowledged', 'legacy_quarantined')"));
        assert!(!MIGRATION_V12.contains("insert into trnm_online_terminal_publication_acks"));
        assert!(LOCAL_TERMINAL_ACK_GAPS_SQL
            .contains("m.terminal_publication_state <> 'legacy_quarantined'"));
        assert!(CAMPAIGN_HAS_UNACKNOWLEDGED_PROGRESSION_SQL
            .contains("m.terminal_publication_state = 'acknowledged'"));
        assert!(CAMPAIGN_HAS_UNACKNOWLEDGED_PROGRESSION_SQL
            .contains("a.local_tombstone_state = 'sealed'"));
        assert!(CAMPAIGN_HAS_UNACKNOWLEDGED_PROGRESSION_SQL
            .contains("a.actor_epoch = m.assigned_instance_epoch"));
        assert!(MIGRATION_V12.contains("trnm_online_rating_events_publication_gate_idx"));
        assert!(MIGRATION_V12.contains("trnm_online_progression_events_publication_gate_idx"));

        assert!(HistoricalTerminalProjectionQuarantine::default().public_credit_is_clean());
        let polluted = HistoricalTerminalProjectionQuarantine {
            legacy_terminal_match_count: 1,
            ..HistoricalTerminalProjectionQuarantine::default()
        };
        assert!(!polluted.public_credit_is_clean());
    }
