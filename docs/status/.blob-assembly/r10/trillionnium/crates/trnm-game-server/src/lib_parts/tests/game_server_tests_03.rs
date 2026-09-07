__TRNM_SLOT_0__
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
__TRNM_SLOT_2__
__TRNM_SLOT_3__
