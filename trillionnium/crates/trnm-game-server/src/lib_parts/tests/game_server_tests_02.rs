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
            Some(bridge),
            &high_water,
        )
        .unwrap();
        assert_eq!(new_lane.tick, high_water.tick);
    }

    #[test]
    fn publication_budget_stops_before_recovery_becomes_unbounded() {
        assert!(publication_within_recovery_budget(10_000, 0));
        assert!(!publication_within_recovery_budget(10_001, 0));
        assert!(publication_within_recovery_budget(20_000, 10_000));
    }

    #[test]
    fn physical_host_mismatch_is_rejected_even_when_instance_identity_matches() {
        assert!(match_assignment_uses_local_physical_host(
            Some("instance-a"),
            Some("host-a"),
            "host-a"
        ));
        assert!(!match_assignment_uses_local_physical_host(
            Some("instance-a"),
            Some("host-b"),
            "host-a"
        ));
        assert!(!match_assignment_uses_local_physical_host(
            Some("instance-a"),
            None,
            "host-a"
        ));
        assert!(match_assignment_uses_local_physical_host(
            None, None, "host-a"
        ));
    }

    #[test]
    fn startup_terminal_compaction_rejects_tampered_tick_hash_or_member_cursor() {
        let cursors = test_input_cursors();
        let mut high_water = test_high_water(20, 4, 9, cursors.clone(), "a".repeat(64));
        high_water.phase = "complete".to_string();
        let mut view = DurableTerminalCompactionView {
            phase: "complete".to_string(),
            simulation_tick: 20,
            simulation_terminal: true,
            simulation_hash: "a".repeat(64),
            durable_snapshot_hash: "a".repeat(64),
            authoritative_tick: Some(20),
            next_sequence: Some(4),
            checkpoint_sequence: Some(4),
            match_revision: Some(9),
            result_valid: true,
            settlement_state: "pending".to_string(),
            next_input_sequences: cursors,
            assigned_instance_id: Some("instance-a".to_string()),
            assigned_instance_epoch: Some(4),
            assigned_physical_host_id: Some("host-a".to_string()),
        };
        assert!(terminal_authority_matches_high_water(&view, &high_water));
        view.simulation_tick = 19;
        assert!(!terminal_authority_matches_high_water(&view, &high_water));
        view.simulation_tick = 20;
        view.durable_snapshot_hash = "b".repeat(64);
        assert!(!terminal_authority_matches_high_water(&view, &high_water));
        view.durable_snapshot_hash = "a".repeat(64);
        view.next_input_sequences.insert("player-a".to_string(), 3);
        assert!(!terminal_authority_matches_high_water(&view, &high_water));
        view.next_input_sequences.insert("player-a".to_string(), 2);
        view.assigned_instance_epoch = Some(5);
        assert!(!terminal_authority_matches_high_water(&view, &high_water));
        view.assigned_instance_epoch = Some(4);
        view.assigned_physical_host_id = Some("host-b".to_string());
        assert!(!terminal_authority_matches_high_water(&view, &high_water));
        view.assigned_physical_host_id = Some("host-a".to_string());
        view.result_valid = false;
        assert!(!terminal_authority_matches_high_water(&view, &high_water));
    }

    #[test]
    fn terminal_running_high_water_crash_windows_preserve_owner_and_exact_successor() {
        let source = published_tick_recovery_simulation();
        let mut running = source.clone();
        for _ in 0..5 {
            running.step().unwrap();
        }
        let cursors = test_input_cursors();
        let high_water = test_high_water(
            running.tick,
            3,
            8,
            cursors.clone(),
            running.snapshot_hash().unwrap(),
        );
        let mut terminal = running.clone();
        for _ in 0..MAX_PUBLISHED_TICK_RECOVERY_STEPS {
            if terminal.terminal() {
                break;
            }
            terminal.step().unwrap();
        }
        assert!(terminal.terminal());
        let terminal_hash = terminal.snapshot_hash().unwrap();
        let mut view = DurableTerminalCompactionView {
            phase: "complete".to_string(),
            simulation_tick: terminal.tick,
            simulation_terminal: true,
            simulation_hash: terminal_hash.clone(),
            durable_snapshot_hash: terminal_hash.clone(),
            authoritative_tick: Some(terminal.tick),
            next_sequence: Some(high_water.next_sequence),
            checkpoint_sequence: Some(high_water.next_sequence),
            match_revision: Some(high_water.match_revision),
            result_valid: true,
            settlement_state: "pending".to_string(),
            next_input_sequences: cursors.clone(),
            assigned_instance_id: Some(high_water.instance_id.clone()),
            assigned_instance_epoch: Some(high_water.actor_epoch),
            assigned_physical_host_id: Some(high_water.physical_host_id.clone()),
        };
        assert!(terminal_authority_succeeds_running_high_water(
            &view,
            &high_water
        ));
        view.next_sequence = Some(4);
        view.match_revision = Some(9);
        view.checkpoint_sequence = Some(4);
        view.next_input_sequences.insert("player-a".to_string(), 3);
        assert!(terminal_authority_succeeds_running_high_water(
            &view,
            &high_water
        ));
        view.next_input_sequences.insert("player-b".to_string(), 3);
        assert!(!terminal_authority_succeeds_running_high_water(
            &view,
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
            "coop_vs_ai",
        )
        .unwrap();
        let mut tampered_prefix = high_water.clone();
        tampered_prefix.snapshot_hash = "b".repeat(64);
        assert!(replay_running_high_water_to_terminal(
            published_tick_recovery_simulation(),
            &tampered_prefix,
            None,
            &terminal,
            &terminal_hash,
            "coop_vs_ai",
        )
        .is_err());

        // Window 1 (DB complete, disk still running) is converted to a
        // terminal record without changing owner/generation/epoch.
        let terminal_high_water = PublishedTickHighWater::new(
            high_water.journal_owner_id,
            high_water.physical_host_id.clone(),
            PublishedTickRecordInput {
                instance_id: high_water.instance_id.clone(),
                match_id: high_water.match_id,
                actor_generation: high_water.actor_generation,
                actor_epoch: high_water.actor_epoch,
                tick: terminal.tick,
                next_sequence: high_water.next_sequence,
                match_revision: high_water.match_revision,
                next_input_sequences: cursors,
                phase: "complete".to_string(),
                receipts_replayable: true,
                snapshot_hash: terminal_hash,
            },
        )
        .unwrap();
        assert_eq!(
            terminal_high_water.journal_owner_id,
            high_water.journal_owner_id
        );
        assert_eq!(
            terminal_high_water.actor_generation,
            high_water.actor_generation
        );
        assert_eq!(terminal_high_water.actor_epoch, high_water.actor_epoch);
        assert!(terminal_authority_matches_high_water(
            &view,
            &terminal_high_water
        ));

        // Window 2 is terminal HWM without marker; the exact predicate above
        // is the only state allowed to create it. Window 3 is marker plus the
        // retained PITR witness; duplicate ACK remains explicitly marker-gated.
        let barrier = |terminal_publication_acked| CommandReceiptPublicationBarrier {
            sequence: 2,
            accepted_revision: 8,
            durable_next_sequence: 3,
            checkpoint_sequence: 3,
            player_id: "player-a",
            input_sequence: 1,
            durable_member_input_sequence: 2,
            phase: "complete",
            terminal_publication_acked,
            actor_cursor: None,
        };
        assert!(!command_receipt_publication_is_acked(barrier(false)));
        assert!(command_receipt_publication_is_acked(barrier(true)));
        assert!(!command_receipt_publication_is_acked(
            CommandReceiptPublicationBarrier {
                sequence: 3,
                accepted_revision: 9,
                durable_next_sequence: 4,
                checkpoint_sequence: 4,
                player_id: "player-a",
                input_sequence: 2,
                durable_member_input_sequence: 3,
                phase: "waiting",
                terminal_publication_acked: true,
                actor_cursor: None,
            }
        ));
    }

    #[tokio::test]
    async fn in_process_terminal_orphan_recovery_retains_revalidated_pitr_witness() {
        let (root, journal, running, terminal) = test_terminal_orphan_journal("success").await;
        let authority = FakeTerminalOrphanAuthority {
            recovered: Some(terminal.clone()),
            marker: Mutex::new(None),
            fail_acknowledgement: false,
            durably_failed_closed: false,
        };
        let actor_tracked = BTreeSet::new();
        assert_eq!(
            count_untracked_published_tick_records(&[running.match_id], &actor_tracked),
            1
        );
        assert!(!terminal_orphan_recovery_is_operational(true, 1));

        assert_eq!(
            reconcile_terminal_journal_record(&authority, &journal, &running)
                .await
                .unwrap(),
            TerminalJournalReconciliationOutcome::Recovered
        );
        assert_eq!(
            authority.marker.lock().expect("fake marker lock").as_ref(),
            Some(&terminal)
        );
        assert_eq!(
            journal.high_water(running.match_id).unwrap().as_ref(),
            Some(&terminal)
        );
        let actor_tracked = BTreeSet::from([running.match_id]);
        assert!(terminal_orphan_recovery_is_operational(
            true,
            count_untracked_published_tick_records(
                &journal.recorded_match_ids().unwrap(),
                &actor_tracked,
            ),
        ));

        drop(journal);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn terminal_orphan_recovery_failure_keeps_record_and_readiness_blocked() {
        let (root, journal, running, terminal) = test_terminal_orphan_journal("failure").await;
        let authority = FakeTerminalOrphanAuthority {
            recovered: Some(terminal.clone()),
            marker: Mutex::new(None),
            fail_acknowledgement: true,
            durably_failed_closed: false,
        };
        assert!(
            reconcile_terminal_journal_record(&authority, &journal, &running)
                .await
                .unwrap_err()
                .contains("injected terminal marker failure")
        );
        assert_eq!(
            journal.high_water(running.match_id).unwrap().as_ref(),
            Some(&terminal)
        );
        let mut registry = MatchActorRegistry::default();
        let initialization =
            reserve_terminal_orphan_recovery(&mut registry, running.match_id).unwrap();
        assert_eq!(
            initialization.kind,
            MatchActorInitializationKind::TerminalRecovery
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
        assert_eq!(
            reconcile_terminal_journal_record(&authority, &journal, &running)
                .await
                .unwrap(),
            TerminalJournalReconciliationOutcome::FailedClosed
        );
        assert_eq!(
            journal.high_water(running.match_id).unwrap().as_ref(),
            Some(&running)
        );
        let tracked = BTreeSet::from([running.match_id]);
        assert_eq!(
            count_untracked_published_tick_records(
                &journal.recorded_match_ids().unwrap(),
                &tracked,
            ),
            0
        );

        drop(journal);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn terminal_ack_gap_detection_is_host_wide_exact_bounded_and_fails_closed() {
        for predicate in [
            "m.phase = 'complete'",
            "m.assigned_physical_host_id = $1",
            "m.terminal_publication_state <> 'legacy_quarantined'",
            "m.terminal_publication_state = 'acknowledged'",
            "a.local_tombstone_state = 'sealed'",
            "m.checkpoint_sequence = m.next_sequence",
            "a.instance_id = m.assigned_instance_id",
            "a.actor_epoch = m.assigned_instance_epoch",
            "a.physical_host_id = m.assigned_physical_host_id",
            "a.authoritative_tick = m.authoritative_tick",
            "a.next_sequence = m.next_sequence",
            "a.match_revision = m.match_revision",
            "a.next_input_sequences =",
            "a.snapshot_hash = m.snapshot_hash",
            "a.result_hash = m.result_hash",
            "a.published_settlement_state = m.settlement_state",
            "limit $2",
        ] {
            assert!(
                LOCAL_TERMINAL_ACK_GAPS_SQL.contains(predicate),
                "missing terminal ACK gap predicate: {predicate}"
            );
        }
        assert!(terminal_ack_gap_recovery_is_operational(true, 0, false));
        assert!(terminal_ack_gap_recovery_is_operational(true, 1, true));
        assert!(!terminal_ack_gap_recovery_is_operational(true, 1, false));
        assert!(!terminal_ack_gap_recovery_is_operational(false, 0, false));
        assert!(!LOCAL_TERMINAL_ACK_GAPS_SQL.contains("trnm_online_fleet_instances"));
        assert!(!terminal_ack_gap_scan_is_saturated(
            usize::try_from(TERMINAL_ACK_GAP_SCAN_LIMIT).unwrap()
        ));
        assert!(terminal_ack_gap_scan_is_saturated(
            usize::try_from(TERMINAL_ACK_GAP_SCAN_LIMIT).unwrap() + 1
        ));

        let recoverable = Uuid::new_v4();
        let unrecoverable = Uuid::new_v4();
        assert_eq!(
            terminal_ack_gaps_without_high_water(
                &[recoverable, unrecoverable],
                &BTreeSet::from([recoverable]),
            ),
            vec![unrecoverable]
        );
    }

    #[test]
    fn readiness_database_summary_collapses_exact_host_checks_and_fails_closed() {
        for predicate in [
            "m.phase = 'complete'",
            "m.assigned_physical_host_id = $3",
            "m.terminal_publication_state <> 'legacy_quarantined'",
            "m.terminal_publication_state = 'acknowledged'",
            "a.local_tombstone_state = 'sealed'",
            "a.actor_generation = m.terminal_publication_actor_generation",
            "a.instance_id = m.assigned_instance_id",
            "a.actor_epoch = m.assigned_instance_epoch",
            "a.physical_host_id = m.assigned_physical_host_id",
            "a.authoritative_tick = m.authoritative_tick",
            "a.next_sequence = m.next_sequence",
            "a.match_revision = m.match_revision",
            "a.next_input_sequences =",
            "a.snapshot_hash = m.snapshot_hash",
            "a.result_hash = m.result_hash",
            "a.published_settlement_state = m.settlement_state",
            "limit $4",
            "instance_id = $1 and instance_epoch = $2",
            "status in ('active', 'draining') and lease_expires_at > now()",
            "phase = 'running' and assigned_instance_id = $1",
            "terminal_publication_state = 'legacy_quarantined'",
            "left join trnm_online_local_cold_witness_summaries cold",
            "cold.physical_host_id = $3",
            "pending_terminal_seals as materialized",
            "trnm_online_failed_closed_abandonment_markers marker",
            "a.local_tombstone_state <> 'sealed'",
            "marker.local_tombstone_state <> 'sealed'",
            "terminal_ack_gap_match_ids",
            "pending_terminal_seal_match_ids",
            "pending_abandonment_seal_match_ids",
        ] {
            assert!(
                READINESS_DATABASE_SUMMARY_SQL.contains(predicate),
                "missing readiness database summary predicate: {predicate}"
            );
        }
        let blocked = ReadinessDatabaseSummary::blocked();
        assert!(!blocked.postgres_healthy);
        assert!(!blocked.fleet_epoch_current);
        assert_eq!(blocked.active_matches, -1);
        assert_eq!(blocked.terminal_ack_gap_count, usize::MAX);
        assert!(blocked.terminal_ack_gap_match_ids.is_empty());
        assert!(blocked.pending_terminal_seal_match_ids.is_empty());
        assert!(blocked.pending_abandonment_seal_match_ids.is_empty());
        assert!(!blocked.historical_projection.public_credit_is_clean());
        assert!(blocked.cold_witness.all_sealed());
    }

    #[test]
    fn legacy_terminal_rows_are_quarantined_without_forging_acknowledgements() {
        assert!(MIGRATION_V12
            .contains("create table if not exists trnm_online_physical_host_authorities"));
        assert!(MIGRATION_V12.contains("database_system_identifier ~ '^[1-9][0-9]*$'"));
        assert!(MIGRATION_V12.contains("leader_lock_key bigint not null"));
        assert!(MIGRATION_V12.contains("barrier_lock_key bigint not null"));
        assert!(MIGRATION_V12.contains("else 'legacy_quarantined'"));
        assert!(MIGRATION_V12.contains("'legacy_bootstrap_pending'"));
        assert!(
            MIGRATION_V12.contains("alter column local_tombstone_state set default 'hot_pending'")
        );
        assert!(MIGRATION_V12.contains(
            "local_tombstone_state in ('legacy_bootstrap_pending', 'hot_pending', 'sealed')"
        ));
        assert!(!MIGRATION_V12.contains("acknowledged_at = now()"));
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

