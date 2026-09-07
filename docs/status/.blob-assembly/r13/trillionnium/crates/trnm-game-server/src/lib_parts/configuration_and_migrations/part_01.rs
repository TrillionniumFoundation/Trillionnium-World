pub struct AppStateConfig {
    pub database_url: String,
    pub cex_base_url: String,
    pub game_authority_token: String,
    pub entitlement_signer_url: String,
    pub entitlement_signer_token: String,
    pub asset_root: PathBuf,
    pub published_tick_journal_dir: PathBuf,
    pub moderator_token: String,
    pub instance_id: String,
    pub region: String,
    pub public_endpoint: String,
    pub physical_host_id: String,
    pub capacity: i32,
    pub rate_limit_per_minute: u32,
    pub request_body_limit_bytes: u32,
    pub tick_interval: Duration,
    pub accelerated_test_clock: bool,
}

pub struct FailCloseMaintenanceConfig {
    pub database_url: String,
    pub published_tick_journal_dir: PathBuf,
    pub instance_id: String,
            13,
            "0013_online_failed_closed_abandonment_v1",
            MIGRATION_V13,
        ),
        (14, "0014_online_command_commit_rpc_v1", MIGRATION_V14),
        (15, "0015_online_realtime_hot_path_v1", MIGRATION_V15),
        (16, "0016_online_settlement_outbox_v1", MIGRATION_V16),
        (
            17,
            "0017_online_settlement_worker_runtime_v1",
            MIGRATION_V17,
        ),
        (
            18,
            "0018_online_settlement_operator_controls_v1",
            MIGRATION_V18,
        ),
        (19, "0019_online_settlement_quarantine_v1", MIGRATION_V19),
    ] {
        let checksum = migration_checksum_sha256(sql);
        let recorded = sqlx::query::query(
            "select migration_name, checksum_sha256
               from public.trnm_online_schema_migrations
              where migration_version = $1",
               and authority_lock.mode = 'ExclusiveLock'
               and authority_lock.granted
               and not pg_is_in_recovery()
               and (select system_identifier::text from pg_control_system()) = $6
               and (select timeline_id::bigint from pg_control_checkpoint()) = $7
               and pg_postmaster_start_time() = $8
        )",
    )
    .bind(&identity.physical_host_id)
    .bind(identity.owner_nonce)
    .bind(&identity.application_name)
    .bind(identity.backend_pid)
    .bind(identity.backend_started_at)
    .bind(&identity.database_system_identifier)
    .bind(identity.database_timeline_id)
    .bind(identity.database_postmaster_started_at)
    .bind(identity.leader_lock_key)
    .bind(identity.barrier_lock_key)
    .fetch_one(&mut *connection)
    .await
}

async fn admit_database_pool_connection(
    connection: &mut PgConnection,
        // 10,000-record recovery bound. Waiting/absent rows are retained so a
        // PITR rollback cannot erase the only evidence of a published tick.
        let recorded_match_ids = published_tick_journal.recorded_match_ids()?;
        let retained_matches = recorded_match_ids.iter().copied().collect::<BTreeSet<_>>();
        let terminal_authority = PostgresTerminalOrphanAuthority {
            pool: &pool,
            journal: &published_tick_journal,
            runtime_state: None,
            database_host_authority: &database_host_authority,
        };
        let mut terminal_acknowledged_high_waters =
            std::mem::take(&mut startup_cold_witnesses.terminal_acknowledged);
        let mut failed_closed_high_waters =
            std::mem::take(&mut startup_cold_witnesses.failed_closed);
        for match_id in recorded_match_ids {
            let Some(high_water) = published_tick_journal.high_water(match_id)? else {
                continue;
            };
            match reconcile_terminal_journal_record(
                &terminal_authority,
                &published_tick_journal,
                &high_water,
            )
            .await?

    pub async fn graceful_shutdown(&self) -> Result<(), String> {
        self.begin_draining().await;
        self.shutdown.send_replace(true);
        if !self.database_host_authority.is_healthy() {
            return Err(
                "PostgreSQL host authority was lost; database flush is blocked".to_string(),
            );
        }
        tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                let registry = self.match_actors.read().await;
                if registry.actors.is_empty() && registry.initializing.is_empty() {
                    return;
                }
                drop(registry);
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .map_err(|_| "timed out flushing online match actors during shutdown".to_string())
    }
}
