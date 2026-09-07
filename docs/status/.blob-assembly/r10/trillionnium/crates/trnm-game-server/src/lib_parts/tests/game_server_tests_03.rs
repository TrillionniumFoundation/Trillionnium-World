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
            );
        }

        assert!(MIGRATION_V13.contains("btrim(failure_reason) <> ''"));
        assert!(MIGRATION_V13.contains("octet_length(failure_reason) <= 1024"));
        assert!(MIGRATION_V13.contains("failure_reason !~ '[[:cntrl:]]'"));
    }

    #[test]
    fn legacy_abandonment_adoption_is_explicit_and_strictly_pre_v13() {
        let applied_at = Utc::now();
        let older = applied_at - chrono::Duration::milliseconds(1);
        let newer = applied_at + chrono::Duration::milliseconds(1);
        assert!(legacy_abandonment_adoption_allowed(true, older, applied_at));
        assert!(!legacy_abandonment_adoption_allowed(
            false, older, applied_at
        ));
        assert!(!legacy_abandonment_adoption_allowed(
            true, applied_at, applied_at
        ));
        assert!(!legacy_abandonment_adoption_allowed(
            true, newer, applied_at
        ));
    }

    #[test]
    fn maintenance_final_high_water_accepts_only_exact_monotonic_db_successors() {
        let cursors = BTreeMap::from([("player-a".to_string(), 4), ("player-b".to_string(), 7)]);
        let high_water = test_high_water(20, 5, 9, cursors.clone(), "a".repeat(64));
        assert!(running_maintenance_successor_is_monotonic(
            &high_water,
            20,
            5,
            9,
            &cursors,
            &"a".repeat(64),
        ));
        assert!(!running_maintenance_successor_is_monotonic(
            &high_water,
            20,
            5,
            9,
            &cursors,
            &"b".repeat(64),
        ));

        let successor_cursors =
            BTreeMap::from([("player-a".to_string(), 5), ("player-b".to_string(), 7)]);
        assert!(running_maintenance_successor_is_monotonic(
            &high_water,
            21,
            6,
            10,
            &successor_cursors,
            &"b".repeat(64),
        ));
        for (tick, sequence, revision, candidate) in [
            (19, 6, 10, successor_cursors.clone()),
            (21, 4, 10, successor_cursors.clone()),
            (21, 6, 8, successor_cursors.clone()),
            (
                21,
                6,
                10,
                BTreeMap::from([("player-a".to_string(), 3), ("player-b".to_string(), 7)]),
            ),
            (
                21,
                6,
                10,
                BTreeMap::from([("player-a".to_string(), 5), ("player-c".to_string(), 7)]),
            ),
        ] {
            assert!(!running_maintenance_successor_is_monotonic(
                &high_water,
                tick,
                sequence,
                revision,
                &candidate,
                &"b".repeat(64),
            ));
        }
    }

    #[test]
    fn legacy_adoption_expectation_rejects_identity_or_reason_before_insert() {
        let high_water = test_high_water(
            20,
            5,
            9,
            BTreeMap::from([("player-a".to_string(), 4)]),
            "a".repeat(64),
        );
        let exact = FailedClosedMaintenanceExpectation {
            instance_id: "instance-a",
            physical_host_id: "host-a",
            failure_reason: "operator fail-close",
        };
        assert!(failed_closed_maintenance_expectation_matches(
            &high_water,
            "operator fail-close",
            exact,
        ));
        for mismatch in [
            FailedClosedMaintenanceExpectation {
                instance_id: "instance-b",
                ..exact
            },
            FailedClosedMaintenanceExpectation {
                physical_host_id: "host-b",
                ..exact
            },
            FailedClosedMaintenanceExpectation {
                failure_reason: "different reason",
                ..exact
            },
        ] {
            assert!(!failed_closed_maintenance_expectation_matches(
                &high_water,
                "operator fail-close",
                mismatch,
            ));
        }
    }

    #[test]
    fn v13_running_fail_close_requires_deferred_exact_hot_pending_marker() {
        let sql = MIGRATION_V13
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
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
            "marker.snapshot_hash = new.snapshot_hash",
__TRNM_SLOT_1__
__TRNM_SLOT_2__
__TRNM_SLOT_3__
