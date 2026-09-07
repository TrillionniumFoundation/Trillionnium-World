__TRNM_SLOT_0__
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
__TRNM_SLOT_2__
__TRNM_SLOT_3__
