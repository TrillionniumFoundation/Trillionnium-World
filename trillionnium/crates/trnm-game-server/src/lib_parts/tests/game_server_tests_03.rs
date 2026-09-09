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
            "marker.next_input_sequences = coalesce",
        ] {
            assert!(
                sql.contains(exact),
                "atomic fail-close guard omitted {exact}"
            );
        }
        assert!(!sql.contains("old.phase = 'waiting' and new.phase = 'failed_closed'"));
        assert!(sql.contains(
            "create trigger trnm_online_guard_marked_abandonment_match_update before update on \
             trnm_online_matches for each row when (old.phase = 'failed_closed' or new.phase is \
             distinct from old.phase) execute function \
             trnm_online_guard_marked_abandonment_match_update()"
        ));
        assert!(!sql.contains("trnm_online_guard_marked_abandonment_member"));
        assert!(sql.contains("raise exception 'online match phase transition is not monotonic'"));
        assert!(sql.contains("raise exception 'marked failed-closed match authority is immutable'"));
        for exact in [
            "new.simulation_json is distinct from old.simulation_json",
            "new.snapshot_hash is distinct from old.snapshot_hash",
            "new.authoritative_tick is distinct from old.authoritative_tick",
            "new.next_sequence is distinct from old.next_sequence",
            "new.checkpoint_sequence is distinct from old.checkpoint_sequence",
            "new.match_revision is distinct from old.match_revision",
            "new.failure_reason is distinct from old.failure_reason",
        ] {
            assert!(
                sql.contains(exact),
                "marked authority guard omitted {exact}"
            );
        }
    }

    #[test]
    fn v14_command_commit_rpc_keeps_one_statement_atomic_fencing_contract() {
        assert!(MIGRATION_V14
            .contains("create or replace function public.trnm_online_commit_actor_command_v1"));
        assert!(MIGRATION_V14.contains("for share;"));
        assert!(MIGRATION_V14.contains("for update;"));
        assert!(MIGRATION_V14.contains("authority.owner_nonce = p_host_owner_nonce"));
        assert!(MIGRATION_V14.contains("authority_lock.mode = 'ExclusiveLock'"));
        assert!(MIGRATION_V14.contains("insert into public.trnm_online_commands"));
        assert!(MIGRATION_V14.contains("set next_input_sequence = p_input_sequence + 1"));
        assert!(MIGRATION_V14.contains("set next_sequence = p_base_next_sequence + 1"));
        assert!(MIGRATION_V14.contains("result_outcome := 'duplicate'"));
        assert!(MIGRATION_V14.contains("result_outcome := 'command_conflict'"));
        assert!(!MIGRATION_V14.contains("commit;"));
    }

    #[test]
    fn v15_realtime_hot_path_collapses_admission_heartbeat_and_checkpoint_round_trips() {
        let sql = MIGRATION_V15
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            sql.contains("create or replace function public.trnm_online_commit_actor_command_v2")
        );
        assert!(sql.contains("insert into public.trnm_online_admission_windows"));
        assert!(sql.contains("from public.trnm_online_commit_actor_command_v1"));
        assert!(
            sql.find("insert into public.trnm_online_admission_windows")
                .unwrap()
                < sql
                    .find("from public.trnm_online_commit_actor_command_v1")
                    .unwrap()
        );
        assert!(sql.contains("result_outcome := 'rate_limited'"));
        assert!(sql.contains("barrier_lock.pid = pg_catalog.pg_backend_pid()"));
        assert!(sql.contains("barrier_lock.mode = 'ShareLock'"));
        assert!(sql.contains("create or replace function public.trnm_online_heartbeat_fleet_v1"));
        assert!(sql.contains("create or replace function public.trnm_online_checkpoint_actor_v1"));
        assert!(sql.contains("from public.trnm_online_fleet_instances fleet"));
        assert!(sql.contains("from public.trnm_online_matches match"));
        assert!(sql.contains("for share;"));
        assert!(sql.contains("for update;"));
        assert!(!sql.contains("commit;"));

        let command =
            distributed_admission_bucket_key("session-a", "POST", "/v1/online/matches/match-a");
        assert_eq!(command.len(), 64);
        assert_ne!(
            command,
            distributed_admission_bucket_key("session-a", "GET", "/v1/online/matches/match-a")
        );
        assert!(is_operational_probe_path("/health"));
        assert!(is_operational_probe_path("/v1/online/readiness"));
        assert!(!is_operational_probe_path("/v1/production/status"));
        assert!(!is_operational_probe_path(
            "/v1/online/matches/match-a/snapshot"
        ));
    }

    #[test]
    fn v13_cold_witness_history_is_restrictive_immutable_and_monotonic() {
        let sql = MIGRATION_V13
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        assert!(sql.contains(
            "alter table trnm_online_terminal_publication_acks drop constraint if exists \
             trnm_online_terminal_publication_acks_match_id_fkey"
        ));
        assert!(sql.contains(
            "add constraint trnm_online_terminal_publication_acks_match_id_fkey foreign key \
             (match_id) references trnm_online_matches(match_id) on delete restrict"
        ));

        for (trigger, table) in [
            (
                "trnm_online_forbid_terminal_ack_delete",
                "trnm_online_terminal_publication_acks",
            ),
            (
                "trnm_online_forbid_abandonment_delete",
                "trnm_online_failed_closed_abandonment_markers",
            ),
        ] {
            assert!(sql.contains(&format!(
                "create trigger {trigger} before delete on {table} for each row execute \
                 function trnm_online_forbid_cold_witness_delete()"
            )));
        }
        assert!(sql.contains("raise exception 'durable cold witness history cannot be deleted'"));

        for field in [
            "match_id",
            "actor_generation",
            "actor_epoch",
            "authoritative_tick",
            "next_sequence",
            "match_revision",
            "next_input_sequences",
            "snapshot_hash",
            "phase",
            "result_hash",
            "acknowledged_at",
            "instance_id",
            "physical_host_id",
        ] {
            assert!(
                sql.contains(&format!("new.{field} is distinct from old.{field}")),
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
            "match_id",
            "journal_owner_id",
            "actor_generation",
            "instance_id",
            "actor_epoch",
            "physical_host_id",
            "authoritative_tick",
            "next_sequence",
            "match_revision",
            "next_input_sequences",
            "snapshot_hash",
            "failure_reason",
            "abandoned_at",
        ] {
            assert!(
                sql.contains(&format!("new.{field} <> old.{field}")),
                "abandonment immutable tuple omitted {field}"
            );
        }
        assert!(sql.contains(
            "old.local_tombstone_state = 'sealed' and \
             new.local_tombstone_state <> 'sealed'"
        ));
        assert!(sql.contains(
            "create trigger trnm_online_guard_abandonment_marker_update before update on \
             trnm_online_failed_closed_abandonment_markers for each row execute function \
             trnm_online_guard_abandonment_marker_update()"
        ));
    }

    #[test]
    fn v13_cold_witness_summary_has_exact_triggers_and_rebuild() {
        let sql = MIGRATION_V13
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        for column in [
            "terminal_total_count bigint not null default 0",
            "terminal_sealed_count bigint not null default 0",
            "abandonment_total_count bigint not null default 0",
            "abandonment_sealed_count bigint not null default 0",
        ] {
            assert!(sql.contains(column), "summary omitted {column}");
        }
        assert!(sql.contains("check (terminal_sealed_count <= terminal_total_count)"));
        assert!(sql.contains("check (abandonment_sealed_count <= abandonment_total_count)"));
        assert!(sql.contains(
            "create trigger trnm_online_terminal_ack_cold_witness_summary after insert or update \
             of physical_host_id, local_tombstone_state on \
             trnm_online_terminal_publication_acks for each row execute function \
             trnm_online_maintain_cold_witness_summary('terminal')"
        ));
        assert!(sql.contains(
            "create trigger trnm_online_abandonment_cold_witness_summary after insert or update \
             of physical_host_id, local_tombstone_state on \
             trnm_online_failed_closed_abandonment_markers for each row execute function \
             trnm_online_maintain_cold_witness_summary('abandonment')"
        ));
        assert!(sql.contains(
            "terminal_total_count = \
             trnm_online_local_cold_witness_summaries.terminal_total_count + 1"
        ));
        assert!(sql.contains(
            "abandonment_total_count = \
             trnm_online_local_cold_witness_summaries.abandonment_total_count + 1"
        ));
        assert!(sql.contains("set terminal_sealed_count = terminal_sealed_count + 1"));
        assert!(sql.contains("set abandonment_sealed_count = abandonment_sealed_count + 1"));

        assert!(sql.contains("delete from trnm_online_local_cold_witness_summaries"));
        assert!(sql.contains(
            "select physical_host_id from trnm_online_terminal_publication_acks union select \
             physical_host_id from trnm_online_failed_closed_abandonment_markers"
        ));
        assert!(sql.contains(
            "count(*) filter (where local_tombstone_state = 'sealed')::bigint as sealed_count"
        ));
        assert!(sql.contains("coalesce(terminals.total_count, 0)"));
        assert!(sql.contains("coalesce(terminals.sealed_count, 0)"));
        assert!(sql.contains("coalesce(abandonments.total_count, 0)"));
        assert!(sql.contains("coalesce(abandonments.sealed_count, 0)"));
    }

    #[test]
    fn database_system_identifier_must_be_canonical_positive_decimal() {
        assert!(canonical_database_system_identifier("7632232469463367718"));
        assert!(!canonical_database_system_identifier(""));
        assert!(!canonical_database_system_identifier("0"));
        assert!(!canonical_database_system_identifier("0123"));
        assert!(!canonical_database_system_identifier("123x"));
    }

    #[test]
    fn database_host_authority_success_is_monotonic_and_readiness_requires_freshness() {
        let first = Instant::now();
        let second = first + Duration::from_millis(250);
        assert_eq!(
            monotonic_database_host_authority_success(first, second),
            second
        );
        assert_eq!(
            monotonic_database_host_authority_success(second, first),
            second
        );

        assert!(database_host_authority_success_is_fresh(
            true,
            first,
            first + DATABASE_HOST_FENCE_FRESHNESS,
            DATABASE_HOST_FENCE_FRESHNESS,
        ));
        assert!(!database_host_authority_success_is_fresh(
            true,
            first,
            first + DATABASE_HOST_FENCE_FRESHNESS + Duration::from_millis(1),
            DATABASE_HOST_FENCE_FRESHNESS,
        ));
        assert!(!database_host_authority_success_is_fresh(
            false,
            first,
            first,
            DATABASE_HOST_FENCE_FRESHNESS,
        ));
        assert!(!database_host_authority_success_is_fresh(
            true,
            second,
            first,
            DATABASE_HOST_FENCE_FRESHNESS,
        ));
    }

    #[test]
    fn migration_ledger_skips_only_exact_version_name_and_checksum() {
        let checksum = migration_checksum_sha256("select 1");
        assert_eq!(checksum.len(), 64);
        assert_ne!(checksum, migration_checksum_sha256("select 2"));
        assert!(
            !migration_ledger_entry_is_applied(1, "0001_online_authority_v1", &checksum, None,)
                .unwrap()
        );
        assert!(migration_ledger_entry_is_applied(
            1,
            "0001_online_authority_v1",
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

    #[test]
    fn postgres_wal_lsn_and_latest_cold_rollback_boundary_are_canonical() {
        assert_eq!(parse_postgres_wal_lsn("0/0"), Some(0));
        assert_eq!(parse_postgres_wal_lsn("1/ABCDEF"), Some(0x1_00ab_cdef));
        for invalid in ["", "0", "0/00", "01/0", "0/abcdef", "0/G", "0/0/0"] {
            assert_eq!(parse_postgres_wal_lsn(invalid), None, "accepted {invalid}");
        }

        let mut high_water = test_high_water(4, 3, 2, test_input_cursors(), "a".repeat(64));
        high_water.phase = "complete".to_string();
        let tombstone = PublishedTickAckTombstone {
            contract_version: "test".to_string(),
            journal_seal_sequence: 1,
            high_water,
            result_hash: "b".repeat(64),
            settlement_state: "pending".to_string(),
            acknowledged_at_unix_ms: 1,
            database_system_identifier: "7632232469463367718".to_string(),
            database_timeline_id: 7,
            database_wal_lsn: "1/20".to_string(),
        };
        let lineage = |system_identifier: &str, timeline_id, wal_flush_lsn: &str| DatabaseLineage {
            system_identifier: system_identifier.to_string(),
            timeline_id,
            wal_flush_lsn: wal_flush_lsn.to_string(),
        };
        assert!(database_lineage_covers_tombstone(
            &lineage("7632232469463367718", 7, "1/20"),
            &tombstone,
        ));
        assert!(database_lineage_covers_tombstone(
            &lineage("7632232469463367718", 7, "1/21"),
            &tombstone,
        ));
        assert!(!database_lineage_covers_tombstone(
            &lineage("7632232469463367718", 7, "1/1F"),
            &tombstone,
        ));
        assert!(!database_lineage_covers_tombstone(
            &lineage("7632232469463367719", 7, "1/21"),
            &tombstone,
        ));
        assert!(!database_lineage_covers_tombstone(
            &lineage("7632232469463367718", 8, "1/21"),
            &tombstone,
        ));
    }

    #[test]
    fn startup_terminal_ack_sources_never_rebuild_nonlegacy_db_evidence() {
        assert_eq!(
            startup_terminal_ack_seal_source("legacy_bootstrap_pending", false, false),
            Some(StartupTerminalAckSealSource::LegacyDatabaseBootstrap)
        );
        assert_eq!(
            startup_terminal_ack_seal_source("legacy_bootstrap_pending", true, false),
            Some(StartupTerminalAckSealSource::Hot)
        );
        assert_eq!(
            startup_terminal_ack_seal_source("hot_pending", true, false),
            Some(StartupTerminalAckSealSource::Hot)
        );
        assert_eq!(
            startup_terminal_ack_seal_source("hot_pending", false, true),
            Some(StartupTerminalAckSealSource::Cold)
        );
        assert_eq!(
            startup_terminal_ack_seal_source("hot_pending", false, false),
            None
        );
        assert_eq!(
            startup_terminal_ack_seal_source("sealed", true, false),
            None
        );
        assert_eq!(
            startup_terminal_ack_seal_source("sealed", false, true),
            Some(StartupTerminalAckSealSource::Cold)
        );
        assert!(terminal_tombstone_settlement_matches("pending", "settled"));
        assert!(!terminal_tombstone_settlement_matches("settled", "pending"));
        assert!(LOCAL_TERMINAL_ACK_STARTUP_PAGE_SQL
            .contains("a.local_tombstone_state in ('legacy_bootstrap_pending', 'hot_pending')"));
    }

    #[test]
    fn terminal_reads_use_durable_assignment_while_running_fences_use_serving_authority() {
        assert!(EXACT_TERMINAL_PUBLICATION_MARKER_SQL
            .contains("a.instance_id = m.assigned_instance_id"));
        assert!(EXACT_TERMINAL_PUBLICATION_MARKER_SQL
            .contains("a.physical_host_id = m.assigned_physical_host_id"));
        assert!(
            EXACT_TERMINAL_PUBLICATION_MARKER_SQL.contains("a.local_tombstone_state = 'sealed'")
        );
        assert!(!EXACT_TERMINAL_PUBLICATION_MARKER_SQL.contains("$2"));
        assert!(MATCH_ACTOR_FENCE_OWNERSHIP_SQL.contains("and f.physical_host_id = $4"));
    }

    #[test]
    fn retained_terminal_ack_revalidation_survives_runtime_epoch_advance() {
        assert!(terminal_runtime_revalidation_is_allowed(
            "instance-a",
            4,
            "host-a",
            "instance-a",
            5,
            "host-a",
            false,
        ));
        assert!(!terminal_runtime_revalidation_is_allowed(
            "instance-a",
            4,
            "host-a",
            "instance-a",
            5,
            "host-a",
            true,
        ));
        assert!(terminal_runtime_revalidation_is_allowed(
            "instance-a",
            5,
            "host-a",
            "instance-a",
            5,
            "host-a",
            true,
        ));
    }

    #[test]
    fn multi_second_failed_or_duplicate_pending_lane_replays_without_tick_loss() {
        let simulation = published_tick_recovery_simulation();
        let cursors = test_input_cursors();
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

