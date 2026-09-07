            &"a".repeat(64),
        let high_water = test_high_water(
            sql.contains("old.phase = 'running' and new.phase = 'failed_closed' and not exists")
        assert!(MIGRATION_V14.contains("result_outcome := 'duplicate'"));
            "alter table trnm_online_terminal_publication_acks drop constraint if exists \
            "raise exception 'terminal publication ACK settlement state cannot regress or change'"
            assert!(sql.contains(column), "summary omitted {column}");
            monotonic_database_host_authority_success(first, second),
            Some(("0001_online_authority_v1", &"0".repeat(64))),
            &lineage("7632232469463367718", 8, "1/21"),
    fn retained_terminal_ack_revalidation_survives_runtime_epoch_advance() {
