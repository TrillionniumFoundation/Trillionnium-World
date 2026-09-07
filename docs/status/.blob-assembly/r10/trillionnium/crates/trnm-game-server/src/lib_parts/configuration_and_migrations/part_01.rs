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
__TRNM_SLOT_1__
__TRNM_SLOT_2__
__TRNM_SLOT_3__
