    #[tokio::test]
    async fn actor_watch_advances_only_after_published_tick_durability_ack() {
        let root = std::env::temp_dir().join(format!(
            "trnm-actor-publication-barrier-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        let journal = PublishedTickJournal::open(
            root.clone(),
            format!("host-publication-{}", Uuid::new_v4()),
        )
        .unwrap();
        let match_id = Uuid::new_v4();
        let actor_id = Uuid::new_v4();
        let mut simulation = published_tick_recovery_simulation();
        let initial_hash = simulation.snapshot_hash().unwrap();
        let started_at = Instant::now();
        let input_cursors = test_input_cursors();
        let initial = PublishedMatchState {
            simulation: Arc::new(simulation.clone()),
            snapshot_hash: Arc::new(initial_hash),
            next_sequence: 2,
            match_revision: 3,
            next_input_sequences: Arc::new(input_cursors.clone()),
            phase: OnlineMatchPhase::Running,
            result_hash: None,
            settlement_state: "not_ready".to_string(),
            state_sequence: 0,
            published_at: started_at,
        };
        simulation.step().unwrap();
        let next_hash = simulation.snapshot_hash().unwrap();
        let candidate = ActorPublicationCandidate {
            state: PublishedMatchState {
                simulation: Arc::new(simulation.clone()),
                snapshot_hash: Arc::new(next_hash.clone()),
                next_sequence: 2,
                match_revision: 3,
                next_input_sequences: Arc::new(input_cursors.clone()),
                phase: OnlineMatchPhase::Running,
                result_hash: None,
                settlement_state: "not_ready".to_string(),
                state_sequence: 1,
                published_at: Instant::now(),
            },
            durable_db_next_sequence: 2,
            durable_db_match_revision: 3,
            durable_db_next_input_sequences: input_cursors.clone(),
            receipts_replayable: true,
            publish_to_watch: true,
        };
        let (published_tx, mut published_rx) = watch::channel(initial);
        let (publication_acked_tx, publication_acked_rx) = watch::channel(ActorPublicationCursor {
            tick: simulation.tick.saturating_sub(1),
            next_sequence: 2,
            match_revision: 3,
            next_input_sequences: input_cursors,
            phase: OnlineMatchPhase::Running,
            receipts_replayable: true,
            snapshot_hash: "0".repeat(64),
        });
        let (candidate_tx, candidate_rx) = watch::channel(None);
        let (permit_tx, permit_rx) = watch::channel(true);
        let (_durable_tick_tx, durable_tick_rx) = watch::channel(simulation.tick);
        let (completion_tx, mut completion_rx) = mpsc::channel(1);
        let worker = tokio::spawn(run_actor_publication_worker(ActorPublicationWorker {
            journal: journal.clone(),
            instance_id: Arc::new("instance-a".to_string()),
            match_id,
            actor_id,
            actor_epoch: 1,
            candidates: candidate_rx,
            permit: permit_rx,
            durable_recovery_tick: durable_tick_rx,
            published: published_tx,
            publication_acked: publication_acked_tx,
            completions: completion_tx,
        }));

        candidate_tx.send_replace(Some(candidate));
        assert_eq!(published_rx.borrow().state_sequence, 0);
        let completion = tokio::time::timeout(Duration::from_secs(2), completion_rx.recv())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(completion.state_sequence, 1);
        completion.result.unwrap();
        published_rx.changed().await.unwrap();
        assert_eq!(published_rx.borrow().state_sequence, 1);
        assert_eq!(publication_acked_rx.borrow().tick, simulation.tick);
        let high_water = journal.high_water(match_id).unwrap().unwrap();
        assert_eq!(high_water.tick, simulation.tick);
        assert_eq!(high_water.snapshot_hash, next_hash);

        permit_tx.send_replace(false);
        drop(candidate_tx);
        worker.await.unwrap();
        drop(journal);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[tokio::test]
    async fn terminal_hwm_waits_for_exact_marker_before_public_watch_and_cursor_release() {
        let root = std::env::temp_dir().join(format!(
            "trnm-terminal-publication-barrier-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        let journal = PublishedTickJournal::open(
            root.clone(),
            format!("host-terminal-publication-{}", Uuid::new_v4()),
        )
        .unwrap();
        let match_id = Uuid::new_v4();
        let actor_id = Uuid::new_v4();
        let mut simulation = published_tick_recovery_simulation();
        let initial_hash = simulation.snapshot_hash().unwrap();
        let input_cursors = test_input_cursors();
        let initial = PublishedMatchState {
            simulation: Arc::new(simulation.clone()),
            snapshot_hash: Arc::new(initial_hash.clone()),
            next_sequence: 2,
            match_revision: 3,
            next_input_sequences: Arc::new(input_cursors.clone()),
            phase: OnlineMatchPhase::Running,
            result_hash: None,
            settlement_state: "not_ready".to_string(),
            state_sequence: 0,
            published_at: Instant::now(),
        };
        simulation.step().unwrap();
        let terminal_hash = simulation.snapshot_hash().unwrap();
        let terminal = PublishedMatchState {
            simulation: Arc::new(simulation.clone()),
            snapshot_hash: Arc::new(terminal_hash.clone()),
            next_sequence: 2,
            match_revision: 3,
            next_input_sequences: Arc::new(input_cursors.clone()),
            phase: OnlineMatchPhase::Complete,
            result_hash: Some("terminal-result".to_string()),
            settlement_state: "staged".to_string(),
            state_sequence: 1,
            published_at: Instant::now(),
        };
        let candidate = ActorPublicationCandidate {
            state: terminal.clone(),
            durable_db_next_sequence: 2,
            durable_db_match_revision: 3,
            durable_db_next_input_sequences: input_cursors.clone(),
            receipts_replayable: true,
            publish_to_watch: true,
        };
        let (published_tx, published_rx) = watch::channel(initial);
        let (publication_acked_tx, publication_acked_rx) = watch::channel(ActorPublicationCursor {
            tick: simulation.tick.saturating_sub(1),
            next_sequence: 2,
            match_revision: 3,
            next_input_sequences: input_cursors,
            phase: OnlineMatchPhase::Running,
            receipts_replayable: true,
            snapshot_hash: initial_hash,
        });
        let (candidate_tx, candidate_rx) = watch::channel(None);
        let (permit_tx, permit_rx) = watch::channel(true);
        let (_durable_tick_tx, durable_tick_rx) = watch::channel(simulation.tick);
        let (completion_tx, mut completion_rx) = mpsc::channel(1);
        let worker = tokio::spawn(run_actor_publication_worker(ActorPublicationWorker {
            journal: journal.clone(),
            instance_id: Arc::new("instance-a".to_string()),
            match_id,
            actor_id,
            actor_epoch: 1,
            candidates: candidate_rx,
            permit: permit_rx,
            durable_recovery_tick: durable_tick_rx,
            published: published_tx.clone(),
            publication_acked: publication_acked_tx.clone(),
            completions: completion_tx,
        }));

        candidate_tx.send_replace(Some(candidate));
        let completion = tokio::time::timeout(Duration::from_secs(2), completion_rx.recv())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(completion.state_sequence, terminal.state_sequence);
        completion.result.unwrap();
        let deferred = completion
            .deferred_terminal
            .expect("terminal completion must retain the public tuple");

        // The host journal is durable, but neither HTTP/WS state nor the
        // receipt cursor may expose terminal authority before the exact DB
        // marker commits.
        let high_water = journal.high_water(match_id).unwrap().unwrap();
        assert_eq!(high_water.phase, "complete");
        assert_eq!(high_water.tick, terminal.simulation.tick);
        assert_eq!(high_water.snapshot_hash, terminal_hash);
        assert_eq!(published_rx.borrow().phase, OnlineMatchPhase::Running);
        assert_eq!(published_rx.borrow().state_sequence, 0);
        assert_eq!(
            publication_acked_rx.borrow().phase,
            OnlineMatchPhase::Running
        );
        assert_eq!(publication_acked_rx.borrow().tick, simulation.tick - 1);

        let mut deferred = deferred;
        bind_deferred_terminal_commit(
            &mut deferred,
            &TerminalPublicationCommit {
                result_hash: "terminal-result".to_string(),
                settlement_state: "pending".to_string(),
                acknowledged_at_unix_ms: 1,
            },
        )
        .unwrap();
        release_deferred_terminal_publication(deferred, &published_tx, &publication_acked_tx)
            .unwrap();
        assert_eq!(published_rx.borrow().phase, OnlineMatchPhase::Complete);
        assert_eq!(published_rx.borrow().settlement_state, "pending");
        assert_eq!(published_rx.borrow().state_sequence, 1);
        assert_eq!(
            publication_acked_rx.borrow().phase,
            OnlineMatchPhase::Complete
        );
        assert!(publication_tuple_matches(
            &published_rx.borrow(),
            &publication_acked_rx.borrow()
        ));

        permit_tx.send_replace(false);
        drop(candidate_tx);
        worker.await.unwrap();
        drop(journal);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn terminal_read_surfaces_require_the_exact_durable_marker() {
        assert!(terminal_read_surface_is_releasable(
            OnlineMatchPhase::Running,
            false
        ));
        assert!(!terminal_read_surface_is_releasable(
            OnlineMatchPhase::Complete,
            false
        ));
        assert!(terminal_read_surface_is_releasable(
            OnlineMatchPhase::Complete,
            true
        ));
    }

    #[tokio::test]
    async fn revoked_publication_permit_blocks_watch_and_ack_updates() {
        let root = std::env::temp_dir().join(format!(
            "trnm-actor-publication-revoke-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        let journal =
            PublishedTickJournal::open(root.clone(), format!("host-revoke-{}", Uuid::new_v4()))
                .unwrap();
        let mut simulation = published_tick_recovery_simulation();
        let initial_hash = simulation.snapshot_hash().unwrap();
        let cursors = test_input_cursors();
        let started_at = Instant::now();
        let initial = PublishedMatchState {
            simulation: Arc::new(simulation.clone()),
            snapshot_hash: Arc::new(initial_hash.clone()),
            next_sequence: 2,
            match_revision: 3,
            next_input_sequences: Arc::new(cursors.clone()),
            phase: OnlineMatchPhase::Running,
            result_hash: None,
            settlement_state: "not_ready".to_string(),
            state_sequence: 0,
            published_at: started_at,
        };
        simulation.step().unwrap();
        let candidate = ActorPublicationCandidate {
            state: PublishedMatchState {
                simulation: Arc::new(simulation.clone()),
                snapshot_hash: Arc::new(simulation.snapshot_hash().unwrap()),
                next_sequence: 2,
                match_revision: 3,
                next_input_sequences: Arc::new(cursors.clone()),
                phase: OnlineMatchPhase::Running,
                result_hash: None,
                settlement_state: "not_ready".to_string(),
                state_sequence: 1,
                published_at: Instant::now(),
            },
            durable_db_next_sequence: 2,
            durable_db_match_revision: 3,
            durable_db_next_input_sequences: cursors.clone(),
            receipts_replayable: true,
            publish_to_watch: true,
        };
        let (published_tx, published_rx) = watch::channel(initial);
        let (acked_tx, acked_rx) = watch::channel(ActorPublicationCursor {
            tick: 0,
            next_sequence: 2,
            match_revision: 3,
            next_input_sequences: cursors,
            phase: OnlineMatchPhase::Running,
            receipts_replayable: true,
            snapshot_hash: initial_hash,
        });
        let (candidate_tx, candidate_rx) = watch::channel(None);
        let (_permit_tx, permit_rx) = watch::channel(false);
        let (_durable_tx, durable_rx) = watch::channel(simulation.tick);
        let (completion_tx, _completion_rx) = mpsc::channel(1);
        let match_id = Uuid::new_v4();
        let worker = tokio::spawn(run_actor_publication_worker(ActorPublicationWorker {
            journal: journal.clone(),
            instance_id: Arc::new("instance-a".to_string()),
            match_id,
            actor_id: Uuid::new_v4(),
            actor_epoch: 1,
            candidates: candidate_rx,
            permit: permit_rx,
            durable_recovery_tick: durable_rx,
            published: published_tx,
            publication_acked: acked_tx,
            completions: completion_tx,
        }));
        candidate_tx.send_replace(Some(candidate));
        tokio::time::timeout(Duration::from_secs(2), worker)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(published_rx.borrow().state_sequence, 0);
        assert_eq!(acked_rx.borrow().tick, 0);
        assert!(journal.high_water(match_id).unwrap().is_none());
        drop(candidate_tx);
        drop(journal);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn duplicate_receipt_waits_for_full_publication_cursor() {
        fn barrier(
            actor_cursor: Option<&ActorPublicationCursor>,
        ) -> CommandReceiptPublicationBarrier<'_> {
            CommandReceiptPublicationBarrier {
                sequence: 3,
                accepted_revision: 9,
                durable_next_sequence: 4,
                checkpoint_sequence: 3,
                player_id: "player-a",
                input_sequence: 2,
                durable_member_input_sequence: 3,
                phase: "running",
                terminal_publication_acked: false,
                actor_cursor,
            }
        }
        let mut cursor = ActorPublicationCursor {
            tick: 20,
            next_sequence: 4,
            match_revision: 9,
            next_input_sequences: BTreeMap::from([
                ("player-a".to_string(), 3),
                ("player-b".to_string(), 2),
            ]),
            phase: OnlineMatchPhase::Running,
            receipts_replayable: false,
            snapshot_hash: "a".repeat(64),
        };
        assert!(!command_receipt_publication_is_acked(barrier(Some(
            &cursor
        ))));
        cursor.receipts_replayable = true;
        assert!(command_receipt_publication_is_acked(barrier(Some(&cursor))));
        cursor
            .next_input_sequences
            .insert("player-a".to_string(), 2);
        assert!(!command_receipt_publication_is_acked(barrier(Some(
            &cursor
        ))));
    }

    #[test]
    fn terminal_duplicate_waits_for_durable_terminal_publication_marker() {
        let barrier = |terminal_publication_acked| CommandReceiptPublicationBarrier {
            sequence: 3,
            accepted_revision: 9,
            durable_next_sequence: 4,
            checkpoint_sequence: 4,
            player_id: "player-a",
            input_sequence: 2,
            durable_member_input_sequence: 3,
            phase: "complete",
            terminal_publication_acked,
            actor_cursor: None,
        };
        assert!(!command_receipt_publication_is_acked(barrier(false)));
        assert!(command_receipt_publication_is_acked(barrier(true)));
    }

    #[test]
    fn terminal_marker_rejects_phase_result_or_settlement_tampering() {
        assert!(terminal_marker_metadata_matches(
            Some("complete"),
            Some("result-a"),
            Some("pending"),
            Some("result-a"),
            "pending",
        ));
        assert!(!terminal_marker_metadata_matches(
            Some("complete"),
            Some("result-a"),
            Some("pending"),
            Some("result-a"),
            "settled",
        ));
        assert!(!terminal_marker_metadata_matches(
            Some("running"),
            Some("result-a"),
            Some("pending"),
            Some("result-a"),
            "pending",
        ));
        assert!(!terminal_marker_metadata_matches(
            Some("complete"),
            Some("result-b"),
            Some("pending"),
            Some("result-a"),
            "pending",
        ));
        assert!(!terminal_marker_metadata_matches(
            Some("complete"),
            Some("result-a"),
            Some("settled"),
            Some("result-a"),
            "pending",
        ));

        let mut evidence = TerminalPublicationEvidence {
            authoritative_tick: 20,
            next_sequence: 4,
            match_revision: 9,
            next_input_sequences: test_input_cursors(),
            snapshot_hash: "a".repeat(64),
            phase: OnlineMatchPhase::Complete,
            result_hash: Some("result-a".to_string()),
            settlement_state: "pending".to_string(),
        };
        assert!(!terminal_publication_metadata_matches_durable(
            &evidence,
            Some("result-a"),
            "settled",
        ));
        // Durable settlement and its marker advance in one transaction; a
        // stale pending marker is not exact publication evidence.
        assert_eq!(evidence.settlement_state, "pending");
        evidence.result_hash = Some("result-b".to_string());
        assert!(!terminal_publication_metadata_matches_durable(
            &evidence,
            Some("result-a"),
            "settled",
        ));
        evidence.result_hash = Some("result-a".to_string());
        evidence.settlement_state = "settled".to_string();
        assert!(!terminal_publication_metadata_matches_durable(
            &evidence,
            Some("result-a"),
            "pending",
        ));
    }

    #[test]
    fn same_instance_and_epoch_on_a_different_physical_host_is_fenced() {
        assert!(match_assignment_matches_local(
            Some("instance-a"),
            4,
            Some("host-a"),
            "instance-a",
            4,
            "host-a",
        ));
        assert!(!match_assignment_matches_local(
            Some("instance-a"),
            4,
            Some("host-b"),
            "instance-a",
            4,
            "host-a",
        ));
        assert!(reconciliation_candidate_is_local(None, None, "host-a"));
        assert!(reconciliation_candidate_is_local(
            Some("instance-a"),
            Some("host-a"),
            "host-a",
        ));
        assert!(!reconciliation_candidate_is_local(
            Some("instance-b"),
            Some("host-b"),
            "host-a",
        ));
    }

    #[test]
    fn terminal_checkpoint_phase_remains_owned_until_publication_finishes() {
        assert!(!MATCH_ACTOR_FENCE_OWNERSHIP_SQL.contains("m.phase"));
        assert!(MATCH_ACTOR_FENCE_OWNERSHIP_SQL.contains("m.assigned_instance_epoch = $3"));
        assert!(MATCH_ACTOR_FENCE_OWNERSHIP_SQL.contains("f.lease_expires_at > now()"));
    }

    #[test]
    fn snapshot_publication_requires_exact_hash_phase_and_member_cursors() {
        let simulation = published_tick_recovery_simulation();
        let hash = simulation.snapshot_hash().unwrap();
        let cursors = test_input_cursors();
        let state = PublishedMatchState {
            simulation: Arc::new(simulation.clone()),
            snapshot_hash: Arc::new(hash.clone()),
            next_sequence: 3,
            match_revision: 8,
            next_input_sequences: Arc::new(cursors.clone()),
            phase: OnlineMatchPhase::Running,
            result_hash: None,
            settlement_state: "not_ready".to_string(),
            state_sequence: 1,
            published_at: Instant::now(),
        };
        let mut acknowledged = ActorPublicationCursor {
            tick: simulation.tick,
            next_sequence: 3,
            match_revision: 8,
            next_input_sequences: cursors,
            phase: OnlineMatchPhase::Running,
            receipts_replayable: true,
            snapshot_hash: hash,
        };
        assert!(publication_tuple_matches(&state, &acknowledged));
        acknowledged.phase = OnlineMatchPhase::Complete;
        assert!(!publication_tuple_matches(&state, &acknowledged));
        acknowledged.phase = OnlineMatchPhase::Running;
        acknowledged
            .next_input_sequences
            .insert("player-a".to_string(), 99);
        assert!(!publication_tuple_matches(&state, &acknowledged));
    }

    #[test]
    fn snapshot_rejects_actor_cursor_ahead_of_durable_database_view() {
        let simulation = published_tick_recovery_simulation();
        let published = PublishedMatchState {
            snapshot_hash: Arc::new(simulation.snapshot_hash().unwrap()),
            simulation: Arc::new(simulation),
            next_sequence: 4,
            match_revision: 9,
            next_input_sequences: Arc::new(BTreeMap::from([
                ("player-a".to_string(), 3),
                ("player-b".to_string(), 2),
            ])),
            phase: OnlineMatchPhase::Running,
            result_hash: None,
            settlement_state: "not_ready".to_string(),
            state_sequence: 1,
            published_at: Instant::now(),
        };
        let member = |player_id: &str, next_input_sequence| OnlineMatchMemberView {
            player_id: player_id.to_string(),
            account_id: Uuid::nil().to_string(),
            campaign_id: Uuid::nil().to_string(),
            role: "host".to_string(),
            controlled_unit_ids: Vec::new(),
            campaign_revision: 1,
            level: 1,
            experience: 0,
            inventory_count: 0,
            next_input_sequence,
        };
        let mut view = OnlineMatchView {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            build_id: ONLINE_AUTHORITY_BUILD.to_string(),
            match_id: Uuid::nil().to_string(),
            join_code: "TEST".to_string(),
            phase: OnlineMatchPhase::Running,
            match_revision: 9,
            authoritative_tick: published.simulation.tick,
            next_sequence: 4,
            map_id: "first_contact".to_string(),
            match_mode: "coop_vs_ai".to_string(),
            rules_version: "test".to_string(),
            seed_hash: "seed".to_string(),
            snapshot_hash: published.snapshot_hash.as_ref().clone(),
            members: vec![member("player-a", 3), member("player-b", 2)],
            result_hash: None,
            settlement_state: "not_ready".to_string(),
        };
        assert!(published_cursor_is_within_durable_view(&published, &view));
        assert!(stream::published_stream_view_is_aligned(&published, &view));
        let mut next = published.clone();
        next.state_sequence = next.state_sequence.saturating_add(1);
        next.next_sequence = next.next_sequence.saturating_add(1);
        next.match_revision = next.match_revision.saturating_add(1);
        next.next_input_sequences = Arc::new(BTreeMap::from([
            ("player-a".to_string(), 4),
            ("player-b".to_string(), 2),
        ]));
        assert!(stream::running_publication_can_reuse_stream_view(
            &published, &next, &view
        ));
        next.phase = OnlineMatchPhase::Complete;
        assert!(!stream::running_publication_can_reuse_stream_view(
            &published, &next, &view
        ));
        view.next_sequence = 3;
        assert!(!published_cursor_is_within_durable_view(&published, &view));
        assert!(!stream::published_stream_view_is_aligned(&published, &view));
        view.next_sequence = 4;
        view.members[0].next_input_sequence = 2;
        assert!(!published_cursor_is_within_durable_view(&published, &view));
        assert!(!stream::published_stream_view_is_aligned(&published, &view));
    }

    #[test]
    fn actor_receipt_cache_preserves_retry_idempotency_and_request_binding() {
        let receipt = OnlineCommandReceipt {
            protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
            match_id: Uuid::nil().to_string(),
            player_id: "player-a".to_string(),
            command_id: "command-a".to_string(),
            sequence: 7,
            input_sequence: 3,
            duplicate: false,
            accepted_tick: 42,
            client_observed_tick: Some(41),
            match_revision: 8,
            snapshot_hash: "a".repeat(64),
        };
        let mut cache = BTreeMap::new();
        let mut cache_order = VecDeque::new();
        cache_actor_receipt(
            &mut cache,
            &mut cache_order,
            "request-a".to_string(),
            &receipt,
        );

        let duplicate =
            cached_actor_command_result(&cache, "command-a", "player-a", "request-a", 8)
                .unwrap()
                .unwrap();
        assert!(duplicate.duplicate);
        assert_eq!(duplicate.sequence, 7);
        assert!(cached_actor_command_result(
            &cache,
            "command-a",
            "player-a",
            "different-request",
            8,
        )
        .unwrap()
        .is_err());
        assert!(committed_receipt_matches_pending(
            &receipt,
            "command-a",
            "player-a",
            7,
            3,
            8,
            &"a".repeat(64),
        ));
        let mut wrong_receipt = receipt;
        wrong_receipt.match_revision = 9;
        assert!(!committed_receipt_matches_pending(
            &wrong_receipt,
            "command-a",
            "player-a",
            7,
            3,
            8,
            &"a".repeat(64),
        ));
    }
