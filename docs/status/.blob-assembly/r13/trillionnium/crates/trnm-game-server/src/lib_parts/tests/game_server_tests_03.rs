    #[test]
    fn abandonment_failure_reason_validation_matches_v13_byte_and_control_contract() {
        assert!(valid_abandonment_failure_reason("actor lease expired"));
        assert!(valid_abandonment_failure_reason(" reason with edges "));
        assert!(valid_abandonment_failure_reason(&"a".repeat(1_024)));
        assert!(!valid_abandonment_failure_reason(&"a".repeat(1_025)));

        // Rust String::len and PostgreSQL octet_length both enforce the bound
        // in UTF-8 bytes, not Unicode scalar values.
        assert!(valid_abandonment_failure_reason(&"界".repeat(341)));
        assert!(!valid_abandonment_failure_reason(&"界".repeat(342)));

        for invalid in [
            "",
            "   ",
            "failure\nreason",
            "failure\treason",
            "failure\0reason",
            "failure\u{7f}reason",
            "failure\u{85}reason",
        ] {
            assert!(
                !valid_abandonment_failure_reason(invalid),
                "accepted invalid abandonment reason: {invalid:?}"
        assert!(sql.contains(
            "create constraint trigger trnm_online_require_atomic_running_abandonment after \
             update on trnm_online_matches deferrable initially deferred for each row when \
             (old.phase = 'running' and new.phase = 'failed_closed')"
        ));
        assert!(sql.contains(
            "create trigger trnm_online_guard_abandonment_marker_insert before insert on \
             trnm_online_failed_closed_abandonment_markers"
        ));
        assert!(sql.contains("new.local_tombstone_state <> 'hot_pending' or not exists"));
        assert!(sql.contains("match_row.phase = 'failed_closed'"));
        assert!(sql.contains("match_row.simulation_json is not null"));
        assert!(
            sql.contains("old.phase = 'running' and new.phase = 'failed_closed' and not exists")
        );
        assert!(sql.contains("marker.local_tombstone_state = 'hot_pending'"));
        for exact in [
            "marker.failure_reason = new.failure_reason",
            "marker.instance_id = new.assigned_instance_id",
            "marker.actor_epoch = new.assigned_instance_epoch",
            "marker.physical_host_id = new.assigned_physical_host_id",
            "marker.authoritative_tick = new.authoritative_tick",
            "marker.next_sequence = new.next_sequence",
            "marker.match_revision = new.match_revision",
                "terminal ACK immutable tuple omitted {field}"
            );
        }
        assert!(sql.contains(
            "old.published_settlement_state = 'pending' and \
             new.published_settlement_state = 'settled'"
        ));
        assert!(sql.contains(
            "old.local_tombstone_state in ('legacy_bootstrap_pending', 'hot_pending') and \
             new.local_tombstone_state = 'sealed'"
        ));
        assert!(sql.contains(
            "raise exception 'terminal publication ACK settlement state cannot regress or change'"
        ));
        assert!(sql.contains(
            "raise exception 'terminal publication ACK cold-seal state cannot regress or change'"
        ));
        assert!(sql.contains(
            "create trigger trnm_online_guard_terminal_ack_update before update on \
             trnm_online_terminal_publication_acks for each row execute function \
             trnm_online_guard_terminal_ack_update()"
        ));

        for field in [
            &checksum,
            Some(("0001_online_authority_v1", &checksum)),
        )
        .unwrap());
        assert!(migration_ledger_entry_is_applied(
            1,
            "0001_online_authority_v1",
            &checksum,
            Some(("renamed", &checksum)),
        )
        .is_err());
        assert!(migration_ledger_entry_is_applied(
            1,
            "0001_online_authority_v1",
            &checksum,
            Some(("0001_online_authority_v1", &"0".repeat(64))),
        )
        .is_err());
        assert!(MIGRATION_LEDGER_DDL.contains("migration_version integer primary key"));
        assert!(MIGRATION_LEDGER_DDL.contains("checksum_sha256 ~ '^[0-9a-f]{64}$'"));
        assert!(DATABASE_MIGRATION_STATEMENT_TIMEOUT > DATABASE_STATEMENT_TIMEOUT);
        assert!(DATABASE_MIGRATION_LOCK_TIMEOUT > DATABASE_LOCK_TIMEOUT);
    }

        let mut durable = LoadedMatchActor {
            simulation: simulation.clone(),
            match_mode: "coop_vs_ai".to_string(),
            members: Arc::new(BTreeMap::new()),
            next_sequence: 3,
            match_revision: 8,
            next_input_sequences: cursors.clone(),
            durable_recovery_tick: simulation.tick,
        };
        let mut expected = simulation;
        for _ in 0..40 {
            expected.step().unwrap();
        }
        replay_durable_actor_after_pending(&mut durable, expected.tick, expected.tick).unwrap();
        assert_eq!(durable.simulation.tick, expected.tick);
        assert_eq!(
            durable.simulation.snapshot_hash().unwrap(),
            expected.snapshot_hash().unwrap()
        );
        assert_eq!(durable.next_sequence, 3);
        assert_eq!(durable.match_revision, 8);
        assert_eq!(durable.next_input_sequences, cursors);
    }
