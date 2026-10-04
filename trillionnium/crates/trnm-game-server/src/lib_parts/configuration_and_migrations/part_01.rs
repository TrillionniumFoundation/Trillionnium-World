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
    pub physical_host_id: String,
    pub match_id: Uuid,
    pub failure_reason: String,
    pub adopt_legacy_pre_v13: bool,
}

#[derive(Debug, serde::Serialize)]
pub struct FailCloseMaintenanceReport {
    pub contract_version: &'static str,
    pub status: &'static str,
    pub match_id: Uuid,
    pub selector: &'static str,
    pub transition_atomic: bool,
    pub previous_phase: String,
    pub final_phase: &'static str,
    pub waiting_db_only: bool,
    pub hot_witness_present_before: bool,
    pub cold_witness_sealed: bool,
    pub local_marker_state: Option<String>,
    pub legacy_adoption: bool,
    pub adoption_contract: Option<&'static str>,
}

async fn configure_database_connection(connection: &mut PgConnection) -> Result<(), sqlx::Error> {
    sqlx::query::query("select set_config('statement_timeout', $1, false)")
        .bind(format!("{}ms", DATABASE_STATEMENT_TIMEOUT.as_millis()))
        .execute(&mut *connection)
        .await?;
    sqlx::query::query("select set_config('lock_timeout', $1, false)")
        .bind(format!("{}ms", DATABASE_LOCK_TIMEOUT.as_millis()))
        .execute(&mut *connection)
        .await?;
    Ok(())
}

fn canonical_database_system_identifier(value: &str) -> bool {
    !value.is_empty() && !value.starts_with('0') && value.bytes().all(|byte| byte.is_ascii_digit())
}

fn migration_checksum_sha256(sql: &str) -> String {
    format!("{:x}", Sha256::digest(sql.as_bytes()))
}

fn migration_ledger_entry_is_applied(
    version: i32,
    expected_name: &str,
    expected_checksum: &str,
    recorded: Option<(&str, &str)>,
) -> Result<bool, String> {
    let Some((recorded_name, recorded_checksum)) = recorded else {
        return Ok(false);
    };
    if recorded_name != expected_name || recorded_checksum != expected_checksum {
        return Err(format!(
            "Online Production migration V{version} ledger drift: expected {expected_name} {expected_checksum}, recorded {recorded_name} {recorded_checksum}"
        ));
    }
    Ok(true)
}

async fn read_database_host_authority_identity(
    connection: &mut PgConnection,
    physical_host_id: &str,
    owner_nonce: Uuid,
    application_name: String,
) -> Result<DatabaseHostAuthorityIdentity, String> {
    let row = sqlx::query::query(
        "select hashtextextended($1, $2) as leader_lock_key,
                hashtextextended($1, $3) as barrier_lock_key,
                pg_backend_pid() as backend_pid,
                activity.backend_start as backend_started_at,
                system.system_identifier::text as database_system_identifier,
                checkpoint.timeline_id::bigint as database_timeline_id,
                pg_postmaster_start_time() as database_postmaster_started_at,
                pg_is_in_recovery() as in_recovery
           from pg_stat_activity activity
           cross join pg_control_system() system
           cross join pg_control_checkpoint() checkpoint
          where activity.pid = pg_backend_pid()",
    )
    .bind(physical_host_id)
    .bind(DATABASE_HOST_LEADER_LOCK_SALT)
    .bind(DATABASE_HOST_BARRIER_LOCK_SALT)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| format!("read PostgreSQL host-authority identity: {error}"))?;
    if row
        .try_get::<bool, _>("in_recovery")
        .map_err(|error| error.to_string())?
    {
        return Err("PostgreSQL host authority requires a writable primary".to_string());
    }
    let database_system_identifier = row
        .try_get::<String, _>("database_system_identifier")
        .map_err(|error| error.to_string())?;
    if !canonical_database_system_identifier(&database_system_identifier) {
        return Err("PostgreSQL system identifier is not canonical".to_string());
    }
    let database_timeline_id = row
        .try_get::<i64, _>("database_timeline_id")
        .map_err(|error| error.to_string())?;
    if database_timeline_id <= 0 {
        return Err("PostgreSQL timeline identifier is not positive".to_string());
    }
    let leader_lock_key = row
        .try_get::<i64, _>("leader_lock_key")
        .map_err(|error| error.to_string())?;
    let barrier_lock_key = row
        .try_get::<i64, _>("barrier_lock_key")
        .map_err(|error| error.to_string())?;
    if leader_lock_key == barrier_lock_key {
        return Err("PostgreSQL host authority lock domains collided".to_string());
    }
    Ok(DatabaseHostAuthorityIdentity {
        physical_host_id: physical_host_id.to_string(),
        owner_nonce,
        application_name,
        backend_pid: row
            .try_get("backend_pid")
            .map_err(|error| error.to_string())?,
        backend_started_at: row
            .try_get("backend_started_at")
            .map_err(|error| error.to_string())?,
        database_system_identifier,
        database_timeline_id,
        database_postmaster_started_at: row
            .try_get("database_postmaster_started_at")
            .map_err(|error| error.to_string())?,
        leader_lock_key,
        barrier_lock_key,
    })
}

async fn run_database_migrations(connection: &mut PgConnection) -> Result<(), String> {
    let mut migrations = Connection::begin(connection)
        .await
        .map_err(|error| format!("begin Online Production migrations: {error}"))?;
    sqlx::query::query(
        "select set_config('statement_timeout', $1, true),
                set_config('lock_timeout', $2, true)",
    )
    .bind(format!(
        "{}ms",
        DATABASE_MIGRATION_STATEMENT_TIMEOUT.as_millis()
    ))
    .bind(format!("{}ms", DATABASE_MIGRATION_LOCK_TIMEOUT.as_millis()))
    .execute(&mut *migrations)
    .await
    .map_err(|error| format!("configure Online Production migration transaction: {error}"))?;
    sqlx::query::query("select pg_advisory_xact_lock($1)")
        .bind(MIGRATION_ADVISORY_LOCK)
        .execute(&mut *migrations)
        .await
        .map_err(|error| format!("lock Online Production migrations: {error}"))?;
    let migration_ledger_exists =
        sqlx::query_scalar::query_scalar::<_, bool>("select to_regclass($1) is not null")
            .bind("public.trnm_online_schema_migrations")
            .fetch_one(&mut *migrations)
            .await
            .map_err(|error| format!("inspect Online Production migration ledger: {error}"))?;
    if !migration_ledger_exists {
        sqlx::raw_sql::raw_sql(MIGRATION_LEDGER_DDL)
            .execute(&mut *migrations)
            .await
            .map_err(|error| format!("create Online Production migration ledger: {error}"))?;
    }
    for (version, label, sql) in [
        (1_i32, "0001_online_authority_v1", MIGRATION_V1),
        (2, "0002_online_authority_v2", MIGRATION_V2),
        (3, "0003_online_product_v1", MIGRATION_V3),
        (4, "0004_online_product_v2", MIGRATION_V4),
        (5, "0005_online_operations_v1", MIGRATION_V5),
        (6, "0006_online_operations_v2", MIGRATION_V6),
        (7, "0007_online_production_v1", MIGRATION_V7),
        (8, "0008_online_production_v2", MIGRATION_V8),
        (9, "0009_online_realtime_actor_v1", MIGRATION_V9),
        (10, "0010_online_realtime_input_v1", MIGRATION_V10),
        (11, "0011_online_terminal_publication_ack_v1", MIGRATION_V11),
        (12, "0012_online_terminal_staging_v1", MIGRATION_V12),
        (
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
        )
        .bind(version)
        .fetch_optional(&mut *migrations)
        .await
        .map_err(|error| format!("read migration V{version} ledger entry: {error}"))?;
        if let Some(recorded) = recorded {
            let recorded_name = recorded
                .try_get::<String, _>("migration_name")
                .map_err(|error| error.to_string())?;
            let recorded_checksum = recorded
                .try_get::<String, _>("checksum_sha256")
                .map_err(|error| error.to_string())?;
            migration_ledger_entry_is_applied(
                version,
                label,
                &checksum,
                Some((&recorded_name, &recorded_checksum)),
            )?;
            continue;
        }
        sqlx::raw_sql::raw_sql(sql)
            .execute(&mut *migrations)
            .await
            .map_err(|error| format!("migrate V{version} {label} PostgreSQL: {error}"))?;
        sqlx::query::query(
            "insert into public.trnm_online_schema_migrations (
                migration_version, migration_name, checksum_sha256
             ) values ($1, $2, $3)",
        )
        .bind(version)
        .bind(label)
        .bind(&checksum)
        .execute(&mut *migrations)
        .await
        .map_err(|error| format!("record migration V{version} {label}: {error}"))?;
    }
    migrations
        .commit()
        .await
        .map_err(|error| format!("commit Online Production migrations: {error}"))
}

async fn publish_database_host_authority_identity(
    connection: &mut PgConnection,
    identity: &DatabaseHostAuthorityIdentity,
) -> Result<(), String> {
    sqlx::query::query(
        "insert into trnm_online_physical_host_authorities (
            physical_host_id, owner_nonce, application_name, backend_pid,
            backend_started_at, database_system_identifier, database_timeline_id,
            database_postmaster_started_at, leader_lock_key, barrier_lock_key, claimed_at
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, now())
         on conflict (physical_host_id) do update set
            owner_nonce = excluded.owner_nonce,
            application_name = excluded.application_name,
            backend_pid = excluded.backend_pid,
            backend_started_at = excluded.backend_started_at,
            database_system_identifier = excluded.database_system_identifier,
            database_timeline_id = excluded.database_timeline_id,
            database_postmaster_started_at = excluded.database_postmaster_started_at,
            leader_lock_key = excluded.leader_lock_key,
            barrier_lock_key = excluded.barrier_lock_key,
            claimed_at = now()",
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
    .execute(&mut *connection)
    .await
    .map_err(|error| format!("publish PostgreSQL host-authority identity: {error}"))?;
    Ok(())
}

async fn bootstrap_database_host_authority(
    database_url: &str,
    physical_host_id: &str,
) -> Result<Arc<DatabaseHostAuthorityFence>, String> {
    let mut connection = PgConnection::connect(database_url)
        .await
        .map_err(|error| format!("connect PostgreSQL host-authority session: {error}"))?;
    configure_database_connection(&mut connection)
        .await
        .map_err(|error| format!("configure PostgreSQL host-authority session: {error}"))?;
    let owner_nonce = Uuid::new_v4();
    let application_name = format!("trnm-host-fence:{owner_nonce}");
    sqlx::query::query("select set_config('application_name', $1, false)")
        .bind(&application_name)
        .execute(&mut connection)
        .await
        .map_err(|error| format!("name PostgreSQL host-authority session: {error}"))?;
    let identity = read_database_host_authority_identity(
        &mut connection,
        physical_host_id,
        owner_nonce,
        application_name,
    )
    .await?;
    let acquired_leader =
        sqlx::query_scalar::query_scalar::<_, bool>("select pg_try_advisory_lock($1)")
            .bind(identity.leader_lock_key)
            .fetch_one(&mut connection)
            .await
            .map_err(|error| format!("acquire PostgreSQL host leader lock: {error}"))?;
    if !acquired_leader {
        return Err(format!(
            "physical host {} already has a PostgreSQL authority leader",
            identity.physical_host_id
        ));
    }
    tokio::time::timeout(DATABASE_HOST_HANDOFF_TIMEOUT, async {
        loop {
            let acquired =
                sqlx::query_scalar::query_scalar::<_, bool>("select pg_try_advisory_lock($1)")
                    .bind(identity.barrier_lock_key)
                    .fetch_one(&mut connection)
                    .await
                    .map_err(|error| format!("acquire PostgreSQL host handoff barrier: {error}"))?;
            if acquired {
                return Ok::<(), String>(());
            }
            tokio::time::sleep(DATABASE_HOST_HANDOFF_POLL_INTERVAL).await;
        }
    })
    .await
    .map_err(|_| {
        format!(
            "PostgreSQL host handoff for {} exceeded {} seconds",
            identity.physical_host_id,
            DATABASE_HOST_HANDOFF_TIMEOUT.as_secs()
        )
    })??;
    run_database_migrations(&mut connection).await?;
    publish_database_host_authority_identity(&mut connection, &identity).await?;
    let released_barrier =
        sqlx::query_scalar::query_scalar::<_, bool>("select pg_advisory_unlock($1)")
            .bind(identity.barrier_lock_key)
            .fetch_one(&mut connection)
            .await
            .map_err(|error| format!("release PostgreSQL host handoff barrier: {error}"))?;
    if !released_barrier {
        return Err(
            "PostgreSQL host handoff barrier was not owned by bootstrap session".to_string(),
        );
    }
    Ok(Arc::new(DatabaseHostAuthorityFence {
        identity,
        connection: AsyncMutex::new(connection),
        healthy: AtomicBool::new(true),
        last_success: Mutex::new(Instant::now()),
    }))
}

async fn database_host_authority_is_exact(
    connection: &mut PgConnection,
    identity: &DatabaseHostAuthorityIdentity,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::query_scalar(
        "select exists (
            select 1
              from trnm_online_physical_host_authorities authority
              join pg_stat_activity activity on activity.pid = authority.backend_pid
              join pg_locks authority_lock on authority_lock.pid = authority.backend_pid
             where authority.physical_host_id = $1
               and authority.owner_nonce = $2
               and authority.application_name = $3
               and authority.backend_pid = $4
               and authority.backend_started_at = $5
               and authority.database_system_identifier = $6
               and authority.database_timeline_id = $7
               and authority.database_postmaster_started_at = $8
               and authority.leader_lock_key = $9
               and authority.barrier_lock_key = $10
               and activity.datname = current_database()
               and activity.backend_start = $5
               and activity.application_name = $3
               and authority_lock.locktype = 'advisory'
               and authority_lock.database = (
                   select oid from pg_database where datname = current_database()
               )
               and authority_lock.classid::bigint = (
                   ($9::bigint >> 32) & 4294967295::bigint
               )
               and authority_lock.objid::bigint = (
                   $9::bigint & 4294967295::bigint
               )
               and authority_lock.objsubid = 1
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
    fence: &DatabaseHostAuthorityFence,
) -> Result<(), sqlx::Error> {
    configure_database_connection(connection).await?;
    if !fence.is_healthy() || !database_host_authority_is_exact(connection, &fence.identity).await?
    {
        return Err(sqlx::Error::Protocol(
            "PostgreSQL host authority is not exact before pool admission".to_string(),
        ));
    }
    sqlx::query::query("select pg_advisory_lock_shared($1)")
        .bind(fence.identity.barrier_lock_key)
        .execute(&mut *connection)
        .await?;
    if !fence.is_healthy() || !database_host_authority_is_exact(connection, &fence.identity).await?
    {
        return Err(sqlx::Error::Protocol(
            "PostgreSQL host authority changed across pool barrier admission".to_string(),
        ));
    }
    Ok(())
}

async fn configure_database_pool_connection(
    connection: &mut PgConnection,
    fence: &DatabaseHostAuthorityFence,
) -> Result<(), sqlx::Error> {
    admit_database_pool_connection(connection, fence).await?;
    // SQLx prepares dynamic PostgreSQL queries on first use per physical
    // connection. Under WAN-like RTT that cache miss adds a full Parse/Describe
    // round trip to an otherwise one-round-trip command. Prepare every
    // real-time statement only after this connection owns the lifetime K2
    // barrier, so no admitted command, heartbeat, checkpoint, or actor-fence
    // query can pay that cold-start tax.
    for statement in [
        ONLINE_COMMAND_COMMIT_V2_SQL,
        ONLINE_HEARTBEAT_FLEET_V1_SQL,
        ONLINE_CHECKPOINT_ACTOR_V1_SQL,
        MATCH_ACTOR_FENCE_OWNERSHIP_SQL,
        DUPLICATE_COMMAND_RECEIPT_SQL,
    ] {
        let _ = connection.prepare(statement).await?;
    }
    Ok(())
}

async fn configure_readiness_database_pool_connection(
    connection: &mut PgConnection,
    fence: &DatabaseHostAuthorityFence,
) -> Result<(), sqlx::Error> {
    admit_database_pool_connection(connection, fence).await?;
    // The control-plane pool is deliberately independent of player traffic.
    // Prepare every query used by the two concurrent readiness branches before
    // the listener becomes available so WAN RTT cannot create a first-sample
    // Parse/Describe spike on either reserved connection.
    for statement in [
        READINESS_DATABASE_SUMMARY_SQL,
        DATABASE_LINEAGE_SQL,
        TERMINAL_ACK_DATABASE_EVIDENCE_BY_MATCH_SQL,
        EXACT_TERMINAL_PUBLICATION_MARKER_SQL,
        ABANDONMENT_DATABASE_EVIDENCE_BY_MATCH_SQL,
    ] {
        let _ = connection.prepare(statement).await?;
    }
    Ok(())
}

async fn require_database_host_authority(
    transaction: &mut sqlx::transaction::Transaction<'_, Postgres>,
    fence: &DatabaseHostAuthorityFence,
) -> Result<(), String> {
    if !fence.is_healthy() {
        return Err("PostgreSQL host authority is fail-closed".to_string());
    }
    let exact = database_host_authority_is_exact(transaction, &fence.identity)
        .await
        .map_err(|error| error.to_string())?;
    if !exact {
        return Err("PostgreSQL host authority leader or incarnation changed".to_string());
    }
    Ok(())
}

impl AppState {
    pub async fn connect(config: AppStateConfig) -> Result<Self, String> {
        if config.instance_id.trim().is_empty()
            || config.region.trim().is_empty()
            || config.public_endpoint.trim().is_empty()
            || config.physical_host_id.trim().is_empty()
            || !(1..=10_000).contains(&config.capacity)
            || !(30..=100_000).contains(&config.rate_limit_per_minute)
            || !(16_384..=1_048_576).contains(&config.request_body_limit_bytes)
        {
            return Err(
                "fleet identity, capacity and production ingress limits must be valid".to_string(),
            );
        }

        // Acquire and validate the host-local publication journal before any
        // database fencing mutation. A second process on the same physical
        // host must fail without incrementing the healthy process epoch.
        let published_tick_journal = PublishedTickJournal::open(
            config.published_tick_journal_dir.clone(),
            config.physical_host_id.clone(),
        )?;
        let database_host_authority =
            bootstrap_database_host_authority(&config.database_url, config.physical_host_id.trim())
                .await?;
        let after_connect_authority = database_host_authority.clone();
        let before_acquire_authority = database_host_authority.clone();
        let pool = PgPoolOptions::new()
            .max_connections(GAME_SERVER_DATABASE_MAX_CONNECTIONS)
            .min_connections(GAME_SERVER_DATABASE_MIN_CONNECTIONS)
            .acquire_timeout(Duration::from_secs(5))
            .after_connect(move |connection, _metadata| {
                let authority = after_connect_authority.clone();
                Box::pin(
                    async move { configure_database_pool_connection(connection, &authority).await },
                )
            })
            .before_acquire(move |_connection, _metadata| {
                let authority = before_acquire_authority.clone();
                Box::pin(async move {
                    if authority.is_healthy() {
                        Ok(true)
                    } else {
                        Err(sqlx::Error::Protocol(
                            "PostgreSQL host authority is fail-closed".to_string(),
                        ))
                    }
                })
            })
            .connect(&config.database_url)
            .await
            .map_err(|error| format!("connect Online Authority PostgreSQL: {error}"))?;

        // Reserve a small control-plane pool for the two concurrent database
        // branches in each readiness request. Four warm connections cover the
        // steady probe; elastic headroom covers the operations monitor plus
        // four workers performing their startup probe together. Player
        // admission, snapshots, checkpoints and command commits may fully
        // occupy the data-plane pool under RTT or a burst, but they must not
        // make the local orchestrator blind and turn transient probe overlap
        // into restart feedback.
        let readiness_after_connect_authority = database_host_authority.clone();
        let readiness_before_acquire_authority = database_host_authority.clone();
        let readiness_pool = PgPoolOptions::new()
            .min_connections(READINESS_DATABASE_MIN_CONNECTIONS)
            .max_connections(READINESS_DATABASE_MAX_CONNECTIONS)
            .acquire_timeout(Duration::from_secs(5))
            .after_connect(move |connection, _metadata| {
                let authority = readiness_after_connect_authority.clone();
                Box::pin(async move {
                    configure_readiness_database_pool_connection(connection, &authority).await
                })
            })
            .before_acquire(move |_connection, _metadata| {
                let authority = readiness_before_acquire_authority.clone();
                Box::pin(async move {
                    if authority.is_healthy() {
                        Ok(true)
                    } else {
                        Err(sqlx::Error::Protocol(
                            "PostgreSQL host authority is fail-closed".to_string(),
                        ))
                    }
                })
            })
            .connect(&config.database_url)
            .await
            .map_err(|error| format!("connect Online Authority readiness PostgreSQL: {error}"))?;

        let cex = CexClient::new(
            config.cex_base_url,
            config.game_authority_token,
            config.entitlement_signer_url,
            config.entitlement_signer_token,
        )?;
        cex.readiness().await?;

        // Validate the manifest's O(1) rollback sentinel before claiming a
        // new fleet epoch, then close only the explicitly pending DB seal
        // states. A non-legacy ACK may never be reconstructed from DB alone.
        let mut startup_cold_witnesses = reconcile_startup_cold_witnesses(
            &pool,
            &published_tick_journal,
            config.physical_host_id.trim(),
            &database_host_authority,
        )
        .await
        .map_err(|error| format!("reconcile pre-claim terminal ACK tombstones: {error}"))?;

        // Compaction is scoped to records that actually exist on this host;
        // the size of the global running fleet must not exhaust a local
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
            {
                TerminalJournalReconciliationOutcome::Recovered => {
                    terminal_acknowledged_high_waters.insert(match_id);
                }
                TerminalJournalReconciliationOutcome::FailedClosed => {
                    failed_closed_high_waters.insert(match_id);
                }
                TerminalJournalReconciliationOutcome::NotTerminal => {}
            }
        }
        let post_recovery_cold_witnesses = reconcile_startup_cold_witnesses(
            &pool,
            &published_tick_journal,
            config.physical_host_id.trim(),
            &database_host_authority,
        )
        .await
        .map_err(|error| format!("revalidate post-recovery cold witnesses: {error}"))?;
        startup_cold_witnesses.extend(post_recovery_cold_witnesses);
        terminal_acknowledged_high_waters.extend(std::mem::take(
            &mut startup_cold_witnesses.terminal_acknowledged,
        ));
        failed_closed_high_waters.extend(std::mem::take(&mut startup_cold_witnesses.failed_closed));
        compact_published_tick_journal_with_timeout(&published_tick_journal, retained_matches)
            .await
            .map_err(|error| format!("compact published-tick journal: {error}"))?;

        // The startup journal pass can prove and repair A/B/C terminal crash
        // windows. Scan every historical assignment on this physical host,
        // independent of the current instance name, fleet row or epoch. Any
        // remaining complete row without an exact marker is shape D and must
        // be quarantined before a new epoch is claimed.
        let terminal_ack_gaps =
            local_terminal_publication_ack_gaps(&pool, config.physical_host_id.trim())
                .await
                .map_err(|error| format!("scan pre-claim terminal publication gaps: {error}"))?;
        let recorded_match_ids = published_tick_journal
            .recorded_match_ids()?
            .into_iter()
            .collect::<BTreeSet<_>>();
        let unrecoverable =
            terminal_ack_gaps_without_high_water(&terminal_ack_gaps, &recorded_match_ids);
        if let Some(match_id) = unrecoverable.first() {
            return Err(format!(
                "terminal publication quarantine before epoch claim: match {match_id} on physical host {} lacks both an exact ACK marker and a recoverable host journal record",
                config.physical_host_id.trim(),
            ));
        }
        if let Some(match_id) = terminal_ack_gaps.first() {
            return Err(format!(
                "terminal publication quarantine before epoch claim: match {match_id} on physical host {} still lacks an exact ACK marker after host journal recovery",
                config.physical_host_id.trim(),
            ));
        }

        // Epoch registration is deliberately the final fallible startup
        // operation. Once this fences an older instance, this process is
        // already able to serve with a bound listener and valid dependencies.
        let mut fleet_claim = pool
            .begin()
            .await
            .map_err(|error| format!("begin Online Operations fleet claim: {error}"))?;
        require_database_host_authority(&mut fleet_claim, &database_host_authority).await?;
        let instance_epoch: i64 = sqlx::query_scalar::query_scalar(
            "insert into trnm_online_fleet_instances (
                instance_id, region, public_endpoint, build_id, capacity, status,
                instance_epoch, lease_expires_at, physical_host_id
             ) values ($1, $2, $3, $4, $5, 'active', 1, now() + interval '5 seconds', $6)
             on conflict (instance_id) do update set region = excluded.region,
                public_endpoint = excluded.public_endpoint, build_id = excluded.build_id,
                capacity = excluded.capacity, status = 'active', heartbeat_at = now(),
                lease_expires_at = now() + interval '5 seconds',
                instance_epoch = trnm_online_fleet_instances.instance_epoch + 1,
                physical_host_id = excluded.physical_host_id, drain_reason = null
             returning instance_epoch",
        )
        .bind(config.instance_id.trim())
        .bind(config.region.trim())
        .bind(config.public_endpoint.trim())
        .bind(trnm_online_protocol::ONLINE_OPERATIONS_BUILD)
        .bind(config.capacity)
        .bind(config.physical_host_id.trim())
        .fetch_one(&mut *fleet_claim)
        .await
        .map_err(|error| format!("register Online Operations fleet instance: {error}"))?;
        require_database_host_authority(&mut fleet_claim, &database_host_authority).await?;
        fleet_claim
            .commit()
            .await
            .map_err(|error| format!("commit Online Operations fleet claim: {error}"))?;
        let (draining, _) = watch::channel(false);
        let (shutdown, _) = watch::channel(false);
        let (fatal_shutdown, _) = watch::channel(false);
        let mut journal_fatal = published_tick_journal.fatal_shutdown();
        let combined_fatal = fatal_shutdown.clone();
        tokio::spawn(async move {
            if !*journal_fatal.borrow() {
                let _ = journal_fatal.changed().await;
            }
            combined_fatal.send_replace(true);
        });
        Ok(Self {
            pool,
            readiness_pool,
            cex,
            asset_root: Arc::new(config.asset_root),
            moderator_token: Arc::new(config.moderator_token),
            instance_id: Arc::new(config.instance_id),
            region: Arc::new(config.region),
            public_endpoint: Arc::new(config.public_endpoint),
            physical_host_id: Arc::new(config.physical_host_id),
            capacity: config.capacity,
            instance_epoch,
            rate_limit_per_minute: config.rate_limit_per_minute,
            request_body_limit_bytes: config.request_body_limit_bytes,
            tick_interval: config.tick_interval,
            accelerated_test_clock: config.accelerated_test_clock,
            authority_clock: Arc::new(AuthorityClockTelemetry::default()),
            published_tick_journal,
            terminal_acknowledged_high_waters: Arc::new(RwLock::new(
                terminal_acknowledged_high_waters,
            )),
            failed_closed_high_waters: Arc::new(RwLock::new(failed_closed_high_waters)),
            match_actors: Arc::new(RwLock::new(MatchActorRegistry::default())),
            stream_connections: Arc::new(Mutex::new(StreamConnectionRegistry::default())),
            database_host_authority,
            fatal_shutdown,
            draining,
            shutdown,
        })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn journal_fatal_shutdown(&self) -> watch::Receiver<bool> {
        self.fatal_shutdown.subscribe()
    }

    fn database_host_authority_is_healthy(&self) -> bool {
        self.database_host_authority.is_healthy()
    }

    async fn verify_database_host_authority_session(&self) -> Result<(), String> {
        if !self.database_host_authority.is_healthy() {
            return Err("PostgreSQL host authority is fail-closed".to_string());
        }
        let mut connection = self.database_host_authority.connection.lock().await;
        let exact = database_host_authority_is_exact(
            &mut connection,
            &self.database_host_authority.identity,
        )
        .await
        .map_err(|error| error.to_string())?;
        if !exact {
            return Err("PostgreSQL host authority session or incarnation changed".to_string());
        }
        self.database_host_authority.record_success(Instant::now());
        Ok(())
    }

    fn fail_database_host_authority(&self) {
        if self
            .database_host_authority
            .healthy
            .swap(false, Ordering::AcqRel)
        {
            self.draining.send_replace(true);
            self.shutdown.send_replace(true);
            self.fatal_shutdown.send_replace(true);
        }
    }

    pub async fn begin_draining(&self) {
        // Synchronize the drain transition with the actor-install CAS. Once
        // this returns, no initializer can pass its final check and install a
        // new actor behind the shutdown boundary.
        let _registry = self.match_actors.write().await;
        self.draining.send_replace(true);
    }

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

