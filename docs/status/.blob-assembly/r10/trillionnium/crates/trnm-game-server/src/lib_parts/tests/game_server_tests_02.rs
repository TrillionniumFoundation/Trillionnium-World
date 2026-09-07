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
        assert!(actor_command_lane_publish_allowed(true, true));
        assert!(actor_command_lane_publish_allowed(false, false));
    }

    #[tokio::test]
    async fn checkpoint_barrier_has_a_hard_deadline_when_worker_never_acknowledges() {
        let simulation = published_tick_recovery_simulation();
        let (checkpoint_tx, mut checkpoint_rx) = mpsc::channel(1);
        let holder = tokio::spawn(async move {
            let _held = checkpoint_rx.recv().await;
            std::future::pending::<()>().await;
        });
        let error = checkpoint_barrier_with_deadline(
            &checkpoint_tx,
            MatchCheckpointJob {
                snapshot_hash: simulation.snapshot_hash().unwrap(),
                simulation,
                next_sequence: 2,
                match_revision: 3,
                terminal: false,
                completion: None,
            },
            "test",
            Duration::from_millis(10),
        )
        .await
        .unwrap_err();
        assert!(error.contains("hard timeout"));
        holder.abort();
        let _ = holder.await;
    }

    #[test]
    fn committed_command_publication_never_rolls_back_visible_tick() {
        assert!(committed_publication_tick_is_monotonic(40, 40));
        assert!(committed_publication_tick_is_monotonic(40, 41));
        assert!(!committed_publication_tick_is_monotonic(41, 40));
    }

    #[test]
    fn published_tick_recovery_replays_autonomous_ticks_without_regression() {
        let mut recovered = published_tick_recovery_simulation();
        let mut expected = recovered.clone();
        for _ in 0..7 {
            expected.step().unwrap();
        }
        let cursors = test_input_cursors();
        let high_water = test_high_water(
            expected.tick,
            3,
            8,
            cursors.clone(),
            expected.snapshot_hash().unwrap(),
        );

        recover_to_published_high_water(&mut recovered, 3, 8, &cursors, None, &high_water).unwrap();
        assert_eq!(recovered.tick, expected.tick);
        assert_eq!(
            recovered.snapshot_hash().unwrap(),
            expected.snapshot_hash().unwrap()
        );
    }

    #[test]
    fn published_tick_recovery_fails_closed_on_hash_or_cursor_rollback() {
        let mut simulation = published_tick_recovery_simulation();
        let mut expected = simulation.clone();
        expected.step().unwrap();
        let cursors = test_input_cursors();
        let mut high_water = test_high_water(
            expected.tick,
            6,
            9,
            cursors.clone(),
            expected.snapshot_hash().unwrap(),
        );
        high_water.snapshot_hash = "0".repeat(64);
        assert!(recover_to_published_high_water(
            &mut simulation,
            6,
            9,
            &cursors,
            None,
            &high_water,
        )
        .unwrap_err()
        .contains("hash mismatched"));

        let mut simulation = published_tick_recovery_simulation();
        high_water.snapshot_hash = expected.snapshot_hash().unwrap();
        assert!(recover_to_published_high_water(
            &mut simulation,
            5,
            9,
            &cursors,
            None,
            &high_water,
        )
        .unwrap_err()
        .contains("ahead of durable database command cursors"));
    }

    #[test]
    fn recovered_command_timing_rejects_tampered_post_tick_or_order_frame() {
        let simulation = published_tick_recovery_simulation();
        let realtime_order = trnm_rts_protocol::RtsFrameOrder::new(
            simulation.tick as u32,
            "player",
            vec!["player-leader".to_string()],
            trnm_rts_protocol::RtsOrderKind::Hold,
            RtsOrderSource::LocalInput,
        );
        let realtime_json = serde_json::to_value(&realtime_order).unwrap();
        validate_recovered_command_timing(
            &simulation,
            simulation.tick as i64,
            Some(simulation.tick as i64),
            realtime_json.clone(),
            "realtime",
        )
        .unwrap();
        assert!(validate_recovered_command_timing(
            &simulation,
            simulation.tick.saturating_add(1) as i64,
            Some(simulation.tick as i64),
            realtime_json,
            "realtime",
        )
        .is_err());

        let scheduled_tick = simulation.tick.saturating_add(10);
        let scheduled_order = trnm_rts_protocol::RtsFrameOrder::new(
            scheduled_tick as u32,
            "player",
            vec!["player-leader".to_string()],
            trnm_rts_protocol::RtsOrderKind::Hold,
            RtsOrderSource::LocalInput,
        );
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
        }
        let old_cursors = test_input_cursors();
        let high_water = test_high_water(
            expected_old_lane.tick,
            3,
            8,
            old_cursors.clone(),
            expected_old_lane.snapshot_hash().unwrap(),
        );
        let mut durable_cursors = old_cursors;
        *durable_cursors.get_mut("player-a").unwrap() += 1;
        let mut new_lane = published_tick_recovery_simulation();
        recover_to_published_high_water(
            &mut new_lane,
            4,
            9,
            &durable_cursors,
__TRNM_SLOT_1__
__TRNM_SLOT_2__
__TRNM_SLOT_3__
