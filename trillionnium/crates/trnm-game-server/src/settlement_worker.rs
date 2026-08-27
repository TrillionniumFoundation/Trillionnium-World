use crate::cex::{AuthorizedSettlementIntent, CexClient, ExternalSettlementError};
use serde::Serialize;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use sqlx::{connection::Connection, executor::Executor, row::Row};
use sqlx_postgres::{PgConnection, PgPool, PgPoolOptions, Postgres};
use std::{collections::BTreeMap, env, time::Duration};
use tokio::time::sleep;
use trnm_campaign_core::{CampaignSaveV1, EconomyBackend};
use trnm_economy_protocol::{
    EconomicIntent, EconomicReceipt, EconomyAccountBinding, ReceiptProgressionClass,
    WalletSnapshot,
};
use uuid::Uuid;

const OUTBOX_CONTRACT: &str = "trnm_settlement_outbox_v1";
const CAPTURE_CONTRACT: &str = "trnm_settlement_capture_v1";
const MIGRATION_V16: &str = include_str!("../migrations/0016_online_settlement_outbox_v1.sql");
const MIGRATION_V17: &str = include_str!("../migrations/0017_online_settlement_worker_runtime_v1.sql");
const MIGRATION_LEDGER_DDL: &str =
    "create table if not exists public.trnm_online_schema_migrations (
        migration_version integer primary key check (migration_version > 0),
        migration_name text not null unique check (btrim(migration_name) <> ''),
        checksum_sha256 text not null check (checksum_sha256 ~ '^[0-9a-f]{64}$'),
        applied_at timestamptz not null default now()
    )";
const MIGRATION_ADVISORY_LOCK: i64 = 0x5452_4e4d_4f4e_4c49;
const MAX_REMOTE_ATTEMPTS: i32 = 16;
const DEFAULT_BATCH_SIZE: usize = 8;
const DEFAULT_LEASE_MILLISECONDS: i64 = 120_000;
const DEFAULT_POLL_MILLISECONDS: u64 = 250;
const DEFAULT_POOL_MAX_CONNECTIONS: u32 = 4;

#[derive(Clone, Debug)]
pub struct WorkerConfig {
    pub database_url: String,
    pub cex_url: String,
    pub game_authority_token: String,
    pub signer_url: String,
    pub signer_token: String,
    pub worker_id: String,
    pub batch_size: usize,
    pub lease_milliseconds: i64,
    pub poll_interval: Duration,
    pub pool_max_connections: u32,
}

impl WorkerConfig {
    pub fn from_env() -> Result<Self, String> {
        let database_url = required_env("DATABASE_URL")?;
        let cex_url = env::var("TRNM_CEX_LEDGER_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:7002".to_string());
        let signer_url = env::var("TRNM_ENTITLEMENT_SIGNER_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:7010".to_string());
        let game_authority_token = required_env("TRNM_GAME_AUTHORITY_TOKEN")?;
        let signer_token = required_env("TRNM_ENTITLEMENT_SIGNER_TOKEN")?;
        if game_authority_token == signer_token {
            return Err(
                "game-authority and signer-service credentials must be independently generated"
                    .to_string(),
            );
        }
        let worker_id = env::var("TRNM_SETTLEMENT_WORKER_ID")
            .unwrap_or_else(|_| format!("settlement-worker:{}", Uuid::new_v4()));
        if worker_id.trim().is_empty() || worker_id.len() > 256 {
            return Err("TRNM_SETTLEMENT_WORKER_ID must contain 1..=256 characters".to_string());
        }
        let batch_size = parse_env_range(
            "TRNM_SETTLEMENT_BATCH_SIZE",
            DEFAULT_BATCH_SIZE,
            1,
            64,
        )?;
        let lease_milliseconds = parse_env_range(
            "TRNM_SETTLEMENT_LEASE_MILLISECONDS",
            DEFAULT_LEASE_MILLISECONDS,
            1_000,
            300_000,
        )?;
        let poll_milliseconds = parse_env_range(
            "TRNM_SETTLEMENT_POLL_MILLISECONDS",
            DEFAULT_POLL_MILLISECONDS,
            25,
            60_000,
        )?;
        let pool_max_connections = parse_env_range(
            "TRNM_SETTLEMENT_DATABASE_MAX_CONNECTIONS",
            DEFAULT_POOL_MAX_CONNECTIONS,
            2,
            16,
        )?;
        Ok(Self {
            database_url,
            cex_url,
            game_authority_token,
            signer_url,
            signer_token,
            worker_id,
            batch_size,
            lease_milliseconds,
            poll_interval: Duration::from_millis(poll_milliseconds),
            pool_max_connections,
        })
    }
}

fn required_env(name: &str) -> Result<String, String> {
    env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("{name} is required"))
}

fn parse_env_range<T>(name: &str, default: T, minimum: T, maximum: T) -> Result<T, String>
where
    T: Copy + Ord + std::str::FromStr + std::fmt::Display,
{
    let value = match env::var(name) {
        Ok(value) => value
            .parse::<T>()
            .map_err(|_| format!("{name} must be a valid integer"))?,
        Err(_) => default,
    };
    if value < minimum || value > maximum {
        return Err(format!(
            "{name} must be between {minimum} and {maximum}"
        ));
    }
    Ok(value)
}

pub async fn run(config: WorkerConfig) -> Result<(), String> {
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(config.pool_max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&config.database_url)
        .await
        .map_err(|error| format!("connect settlement worker PostgreSQL pool: {error}"))?;
    apply_worker_migrations(&pool).await?;

    let cex = CexClient::new(
        config.cex_url.clone(),
        config.game_authority_token.clone(),
        config.signer_url.clone(),
        config.signer_token.clone(),
    )?;
    cex.readiness().await?;

    tracing::info!(
        worker_id = %config.worker_id,
        batch_size = config.batch_size,
        lease_milliseconds = config.lease_milliseconds,
        "transaction-free settlement worker is ready"
    );

    loop {
        let mut work_count = 0_u64;
        match capture_pending_matches(&pool, config.batch_size).await {
            Ok(captured) => work_count = work_count.saturating_add(captured),
            Err(error) => tracing::error!(%error, "settlement capture scan failed"),
        }

        for _ in 0..config.batch_size {
            match claim_settlement_job(
                &pool,
                &config.worker_id,
                config.lease_milliseconds,
            )
            .await
            {
                Ok(Some(job)) => {
                    work_count = work_count.saturating_add(1);
                    if let Err(error) = process_claimed_job(&pool, &cex, job).await {
                        tracing::error!(%error, "claimed settlement job failed");
                    }
                }
                Ok(None) => break,
                Err(error) => {
                    tracing::error!(%error, "settlement claim failed");
                    break;
                }
            }
        }

        match apply_ready_captures(&pool, config.batch_size).await {
            Ok(applied) => work_count = work_count.saturating_add(applied),
            Err(error) => tracing::error!(%error, "settlement apply scan failed"),
        }

        if work_count == 0 {
            tokio::select! {
                signal = tokio::signal::ctrl_c() => {
                    signal.map_err(|error| format!("install settlement shutdown signal: {error}"))?;
                    tracing::info!("settlement worker received shutdown");
                    return Ok(());
                }
                _ = sleep(config.poll_interval) => {}
            }
        }
    }
}

async fn apply_worker_migrations(pool: &PgPool) -> Result<(), String> {
    let mut connection = pool
        .acquire()
        .await
        .map_err(|error| format!("acquire migration connection: {error}"))?;
    connection
        .execute(MIGRATION_LEDGER_DDL)
        .await
        .map_err(|error| format!("create migration ledger: {error}"))?;
    sqlx::query::query("select pg_advisory_lock($1)")
        .bind(MIGRATION_ADVISORY_LOCK)
        .execute(&mut *connection)
        .await
        .map_err(|error| format!("acquire migration advisory lock: {error}"))?;

    let result = apply_worker_migrations_locked(&mut connection).await;
    let unlock = sqlx::query_scalar::query_scalar::<_, bool>("select pg_advisory_unlock($1)")
        .bind(MIGRATION_ADVISORY_LOCK)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| format!("release migration advisory lock: {error}"));
    match (result, unlock) {
        (Err(error), _) => Err(error),
        (Ok(()), Err(error)) => Err(error),
        (Ok(()), Ok(true)) => Ok(()),
        (Ok(()), Ok(false)) => Err("migration advisory lock was not held".to_string()),
    }
}

async fn apply_worker_migrations_locked(connection: &mut PgConnection) -> Result<(), String> {
    for (version, name, sql) in [
        (16_i32, "0016_online_settlement_outbox_v1", MIGRATION_V16),
        (
            17_i32,
            "0017_online_settlement_worker_runtime_v1",
            MIGRATION_V17,
        ),
    ] {
        let checksum = hash_bytes(sql.as_bytes());
        let recorded = sqlx::query::query(
            "select migration_name, checksum_sha256
               from public.trnm_online_schema_migrations
              where migration_version = $1",
        )
        .bind(version)
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| format!("read settlement migration {version}: {error}"))?;
        if let Some(recorded) = recorded {
            let recorded_name: String = recorded
                .try_get("migration_name")
                .map_err(|error| error.to_string())?;
            let recorded_checksum: String = recorded
                .try_get("checksum_sha256")
                .map_err(|error| error.to_string())?;
            if recorded_name != name || recorded_checksum != checksum {
                return Err(format!(
                    "settlement migration {version} checksum/name drift: {recorded_name} {recorded_checksum}"
                ));
            }
            continue;
        }
        let mut transaction = connection
            .begin()
            .await
            .map_err(|error| format!("begin settlement migration {version}: {error}"))?;
        transaction
            .execute(sql)
            .await
            .map_err(|error| format!("execute settlement migration {version}: {error}"))?;
        sqlx::query::query(
            "insert into public.trnm_online_schema_migrations (
                migration_version, migration_name, checksum_sha256
             ) values ($1, $2, $3)",
        )
        .bind(version)
        .bind(name)
        .bind(checksum)
        .execute(&mut *transaction)
        .await
        .map_err(|error| format!("record settlement migration {version}: {error}"))?;
        transaction
            .commit()
            .await
            .map_err(|error| format!("commit settlement migration {version}: {error}"))?;
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QueueLane {
    Compensation,
    Ordinary,
}

impl QueueLane {
    fn as_str(self) -> &'static str {
        match self {
            Self::Compensation => "compensation",
            Self::Ordinary => "ordinary",
        }
    }

    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "compensation" => Ok(Self::Compensation),
            "ordinary" => Ok(Self::Ordinary),
            _ => Err(format!("unknown settlement queue lane {value}")),
        }
    }
}

#[derive(Clone)]
struct CampaignRow {
    campaign_id: String,
    campaign_revision: u64,
    state_hash: String,
    campaign: CampaignSaveV1,
}

#[derive(Clone)]
struct SettlementJob {
    job_id: String,
    capture_id: String,
    match_id: Uuid,
    campaign_id: String,
    intent_id: String,
    intent_hash: String,
    expected_campaign_revision: u64,
    expected_campaign_state_hash: String,
    queue_lane: QueueLane,
    intent: EconomicIntent,
    authorization_request_id: String,
    entitlement_issued_at_epoch: i64,
    entitlement_expires_at_epoch: i64,
    entitlement_nonce: String,
    authorized_intent: Option<EconomicIntent>,
    signer_receipt_hash: Option<String>,
    remote_attempts: i32,
    lease_owner: String,
    lease_generation: i64,
}

fn campaign_head(campaign: &CampaignSaveV1) -> Option<(QueueLane, &EconomicIntent)> {
    campaign
        .pending_economic_compensations
        .first()
        .map(|intent| (QueueLane::Compensation, intent))
        .or_else(|| {
            campaign
                .pending_economic_intents
                .first()
                .map(|intent| (QueueLane::Ordinary, intent))
        })
}

pub async fn capture_pending_matches(pool: &PgPool, limit: usize) -> Result<u64, String> {
    let limit = i64::try_from(limit).map_err(|_| "capture limit is too large".to_string())?;
    let match_ids = sqlx::query_scalar::query_scalar::<_, Uuid>(
        "select match_row.match_id
           from public.trnm_online_matches match_row
          where public.trnm_online_settlement_match_ready_v1(match_row.match_id)
            and not exists (
                select 1
                  from public.trnm_online_settlement_captures capture
                 where capture.match_id = match_row.match_id
                   and capture.state in ('active', 'dead_letter')
            )
          order by match_row.updated_at, match_row.match_id
          limit $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|error| format!("scan settlement capture candidates: {error}"))?;

    let mut captured = 0_u64;
    for match_id in match_ids {
        captured = captured.saturating_add(capture_match(pool, match_id).await?);
    }
    Ok(captured)
}

async fn capture_match(pool: &PgPool, match_id: Uuid) -> Result<u64, String> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(|error| format!("begin settlement capture: {error}"))?;
    let locked = sqlx::query_scalar::query_scalar::<_, Uuid>(
        "select match_id
           from public.trnm_online_matches
          where match_id = $1
            and public.trnm_online_settlement_match_ready_v1(match_id)
          for update skip locked",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| format!("lock settlement capture match: {error}"))?;
    if locked.is_none() {
        transaction
            .commit()
            .await
            .map_err(|error| format!("commit skipped settlement capture: {error}"))?;
        return Ok(0);
    }
    let active_capture = sqlx::query_scalar::query_scalar::<_, bool>(
        "select exists(
            select 1 from public.trnm_online_settlement_captures
             where match_id = $1 and state in ('active', 'dead_letter')
         )",
    )
    .bind(match_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(|error| format!("inspect active settlement capture: {error}"))?;
    if active_capture {
        transaction
            .commit()
            .await
            .map_err(|error| format!("commit duplicate settlement capture skip: {error}"))?;
        return Ok(0);
    }

    let terminal_identity = load_terminal_identity(&mut transaction, match_id).await?;
    let terminal_identity_hash = hash_json(&terminal_identity)?;
    let campaigns = load_campaign_rows(&mut transaction, match_id).await?;
    if campaigns.len() != 2 {
        return Err(format!(
            "pending settlement match {match_id} has {} campaigns instead of exactly two",
            campaigns.len()
        ));
    }
    let campaign_fences = campaign_fences_json(&campaigns);
    let head_intents = head_intents_json(&campaigns)?;
    let generation = sqlx::query_scalar::query_scalar::<_, i64>(
        "select coalesce(max(capture_generation), 0) + 1
           from public.trnm_online_settlement_captures
          where match_id = $1",
    )
    .bind(match_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(|error| format!("allocate settlement capture generation: {error}"))?;
    let capture_id = deterministic_capture_id(
        match_id,
        generation,
        &terminal_identity_hash,
        &campaign_fences,
        &head_intents,
    )?;

    sqlx::query::query(
        "insert into public.trnm_online_settlement_captures (
            capture_id, contract_version, match_id, capture_generation,
            terminal_identity_hash, terminal_identity_json,
            campaign_fences_json, head_intent_ids_json
         ) values ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(&capture_id)
    .bind(CAPTURE_CONTRACT)
    .bind(match_id)
    .bind(generation)
    .bind(&terminal_identity_hash)
    .bind(&terminal_identity)
    .bind(&campaign_fences)
    .bind(&head_intents)
    .execute(&mut *transaction)
    .await
    .map_err(|error| format!("insert settlement capture: {error}"))?;

    let mut jobs = 0_u64;
    for campaign in &campaigns {
        let Some((lane, intent)) = campaign_head(&campaign.campaign) else {
            continue;
        };
        intent.validate()?;
        let intent_hash = hash_json(intent)?;
        let job_id = deterministic_job_id(
            &capture_id,
            match_id,
            &campaign.campaign_id,
            &intent.intent_id,
            &intent_hash,
        );
        sqlx::query::query(
            "insert into public.trnm_online_settlement_jobs (
                job_id, contract_version, capture_id, capture_generation,
                match_id, campaign_id, intent_id, intent_hash,
                expected_campaign_revision, expected_campaign_state_hash,
                queue_lane, intent_json
             ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        )
        .bind(job_id)
        .bind(OUTBOX_CONTRACT)
        .bind(&capture_id)
        .bind(generation)
        .bind(match_id)
        .bind(&campaign.campaign_id)
        .bind(&intent.intent_id)
        .bind(intent_hash)
        .bind(i64::try_from(campaign.campaign_revision).map_err(|_| {
            "campaign revision exceeds PostgreSQL bigint range".to_string()
        })?)
        .bind(&campaign.state_hash)
        .bind(lane.as_str())
        .bind(serde_json::to_value(intent).map_err(|error| error.to_string())?)
        .execute(&mut *transaction)
        .await
        .map_err(|error| format!("insert settlement job: {error}"))?;
        jobs = jobs.saturating_add(1);
    }

    if jobs == 0 {
        finalize_match_in_transaction(&mut transaction, match_id).await?;
        let capture = sqlx::query::query(
            "update public.trnm_online_settlement_captures
                set state = 'finalized', applied_at = clock_timestamp(),
                    updated_at = clock_timestamp()
              where capture_id = $1 and state = 'active'",
        )
        .bind(&capture_id)
        .execute(&mut *transaction)
        .await
        .map_err(|error| format!("finalize empty settlement capture: {error}"))?;
        if capture.rows_affected() != 1 {
            return Err("empty settlement capture lost its active fence".to_string());
        }
    }

    transaction
        .commit()
        .await
        .map_err(|error| format!("commit settlement capture: {error}"))?;
    Ok(1)
}

async fn load_terminal_identity(
    transaction: &mut sqlx::transaction::Transaction<'_, Postgres>,
    match_id: Uuid,
) -> Result<Value, String> {
    sqlx::query_scalar::query_scalar::<_, Value>(
        "select jsonb_build_object(
            'contract_version', 'trnm_terminal_settlement_identity_v1',
            'match_id', match_row.match_id::text,
            'phase', match_row.phase,
            'settlement_state', match_row.settlement_state,
            'terminal_publication_state', match_row.terminal_publication_state,
            'checkpoint_sequence', match_row.checkpoint_sequence,
            'next_sequence', match_row.next_sequence,
            'result_hash', match_row.result_hash,
            'terminal_publication_actor_generation',
                match_row.terminal_publication_actor_generation::text,
            'assigned_instance_id', match_row.assigned_instance_id,
            'assigned_instance_epoch', match_row.assigned_instance_epoch,
            'assigned_physical_host_id', match_row.assigned_physical_host_id,
            'authoritative_tick', match_row.authoritative_tick,
            'match_revision', match_row.match_revision,
            'snapshot_hash', match_row.snapshot_hash,
            'ack_actor_generation', ack.actor_generation::text,
            'ack_instance_id', ack.instance_id,
            'ack_actor_epoch', ack.actor_epoch,
            'ack_physical_host_id', ack.physical_host_id,
            'ack_authoritative_tick', ack.authoritative_tick,
            'ack_next_sequence', ack.next_sequence,
            'ack_match_revision', ack.match_revision,
            'ack_snapshot_hash', ack.snapshot_hash,
            'ack_result_hash', ack.result_hash,
            'ack_phase', ack.phase,
            'ack_settlement_state', ack.published_settlement_state,
            'ack_tombstone_state', ack.local_tombstone_state,
            'member_sequences', coalesce(
                (select jsonb_object_agg(
                    member.player_id,
                    to_jsonb(member.next_input_sequence)
                    order by member.player_id
                 )
                   from public.trnm_online_match_members member
                  where member.match_id = match_row.match_id),
                '{}'::jsonb
            )
        )
        from public.trnm_online_matches match_row
        join public.trnm_online_terminal_publication_acks ack
          on ack.match_id = match_row.match_id
        where match_row.match_id = $1
          and public.trnm_online_settlement_match_ready_v1(match_row.match_id)",
    )
    .bind(match_id)
    .fetch_one(&mut **transaction)
    .await
    .map_err(|error| format!("load terminal settlement identity: {error}"))
}

async fn load_campaign_rows(
    transaction: &mut sqlx::transaction::Transaction<'_, Postgres>,
    match_id: Uuid,
) -> Result<Vec<CampaignRow>, String> {
    let rows = sqlx::query::query(
        "select campaign.campaign_id, campaign.campaign_revision,
                campaign.state_hash, campaign.campaign_json
           from public.trnm_online_match_members member
           join public.trnm_online_campaigns campaign
             on campaign.campaign_id = member.campaign_id
          where member.match_id = $1
          order by member.player_id
          for update of campaign",
    )
    .bind(match_id)
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| format!("lock settlement campaigns: {error}"))?;
    let mut campaigns = Vec::with_capacity(rows.len());
    for row in rows {
        let campaign_id: String = row
            .try_get("campaign_id")
            .map_err(|error| error.to_string())?;
        let campaign_revision = u64::try_from(
            row.try_get::<i64, _>("campaign_revision")
                .map_err(|error| error.to_string())?,
        )
        .map_err(|_| "campaign revision is negative".to_string())?;
        let state_hash: String = row
            .try_get("state_hash")
            .map_err(|error| error.to_string())?;
        let campaign: CampaignSaveV1 = serde_json::from_value(
            row.try_get::<Value, _>("campaign_json")
                .map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("decode campaign {campaign_id}: {error}"))?;
        campaign
            .validate()
            .map_err(|error| format!("validate campaign {campaign_id}: {error}"))?;
        if campaign.campaign_id != campaign_id
            || campaign.revision != campaign_revision
            || hash_json(&campaign)? != state_hash
        {
            return Err(format!(
                "campaign {campaign_id} failed revision/state-hash binding"
            ));
        }
        campaigns.push(CampaignRow {
            campaign_id,
            campaign_revision,
            state_hash,
            campaign,
        });
    }
    Ok(campaigns)
}

fn campaign_fences_json(campaigns: &[CampaignRow]) -> Value {
    let mut fences = Map::new();
    for campaign in campaigns {
        fences.insert(
            campaign.campaign_id.clone(),
            json!({
                "campaign_revision": campaign.campaign_revision,
                "state_hash": campaign.state_hash.clone(),
            }),
        );
    }
    Value::Object(fences)
}

fn head_intents_json(campaigns: &[CampaignRow]) -> Result<Value, String> {
    let mut heads = Map::new();
    for campaign in campaigns {
        let value = match campaign_head(&campaign.campaign) {
            Some((lane, intent)) => {
                intent.validate()?;
                json!({
                    "queue_lane": lane.as_str(),
                    "intent_id": intent.intent_id.clone(),
                    "intent_hash": hash_json(intent)?,
                })
            }
            None => Value::Null,
        };
        heads.insert(campaign.campaign_id.clone(), value);
    }
    Ok(Value::Object(heads))
}

fn deterministic_capture_id(
    match_id: Uuid,
    generation: i64,
    terminal_identity_hash: &str,
    campaign_fences: &Value,
    head_intents: &Value,
) -> Result<String, String> {
    let digest = hash_json(&json!({
        "contract_version": CAPTURE_CONTRACT,
        "match_id": match_id,
        "capture_generation": generation,
        "terminal_identity_hash": terminal_identity_hash,
        "campaign_fences": campaign_fences,
        "head_intents": head_intents,
    }))?;
    Ok(format!("trnm-settlement-capture-v1:{digest}"))
}

fn deterministic_job_id(
    capture_id: &str,
    match_id: Uuid,
    campaign_id: &str,
    intent_id: &str,
    intent_hash: &str,
) -> String {
    let digest = Sha256::digest(
        format!(
            "{OUTBOX_CONTRACT}\0{capture_id}\0{match_id}\0{campaign_id}\0{intent_id}\0{intent_hash}"
        )
        .as_bytes(),
    );
    format!("trnm-settlement-outbox-v1:{digest:x}")
}

async fn finalize_match_in_transaction(
    transaction: &mut sqlx::transaction::Transaction<'_, Postgres>,
    match_id: Uuid,
) -> Result<(), String> {
    let marker = sqlx::query::query(
        "update public.trnm_online_terminal_publication_acks
            set published_settlement_state = 'settled'
          where match_id = $1
            and published_settlement_state = 'pending'
            and local_tombstone_state = 'sealed'",
    )
    .bind(match_id)
    .execute(&mut **transaction)
    .await
    .map_err(|error| format!("advance terminal settlement marker: {error}"))?;
    if marker.rows_affected() != 1 {
        return Err("terminal settlement marker lost its pending/sealed fence".to_string());
    }
    let durable_match = sqlx::query::query(
        "update public.trnm_online_matches
            set settlement_state = 'settled', updated_at = clock_timestamp()
          where match_id = $1
            and phase = 'complete'
            and terminal_publication_state = 'acknowledged'
            and settlement_state = 'pending'",
    )
    .bind(match_id)
    .execute(&mut **transaction)
    .await
    .map_err(|error| format!("advance durable match settlement: {error}"))?;
    if durable_match.rows_affected() != 1 {
        return Err("terminal settlement lost its acknowledged match fence".to_string());
    }
    Ok(())
}

async fn claim_settlement_job(
    pool: &PgPool,
    owner: &str,
    lease_milliseconds: i64,
) -> Result<Option<SettlementJob>, String> {
    let row = sqlx::query::query(
        "select * from public.trnm_online_claim_settlement_job_v2($1, $2)",
    )
    .bind(owner)
    .bind(lease_milliseconds)
    .fetch_optional(pool)
    .await
    .map_err(|error| format!("claim settlement job: {error}"))?;
    row.as_ref().map(settlement_job_from_row).transpose()
}

fn settlement_job_from_row(row: &sqlx_postgres::PgRow) -> Result<SettlementJob, String> {
    let intent: EconomicIntent = serde_json::from_value(
        row.try_get::<Value, _>("intent_json")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|error| format!("decode claimed settlement intent: {error}"))?;
    let authorized_intent = row
        .try_get::<Option<Value>, _>("authorized_intent_json")
        .map_err(|error| error.to_string())?
        .map(serde_json::from_value::<EconomicIntent>)
        .transpose()
        .map_err(|error| format!("decode authorized settlement intent: {error}"))?;
    let capture_id = row
        .try_get::<Option<String>, _>("capture_id")
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "claimed settlement job is not bound to a capture".to_string())?;
    let authorization_request_id = row
        .try_get::<Option<String>, _>("authorization_request_id")
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "claimed settlement job has no authorization request id".to_string())?;
    let entitlement_issued_at_epoch = row
        .try_get::<Option<i64>, _>("entitlement_issued_at_epoch")
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "claimed settlement job has no issued_at".to_string())?;
    let entitlement_expires_at_epoch = row
        .try_get::<Option<i64>, _>("entitlement_expires_at_epoch")
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "claimed settlement job has no expires_at".to_string())?;
    let entitlement_nonce = row
        .try_get::<Option<String>, _>("entitlement_nonce")
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "claimed settlement job has no entitlement nonce".to_string())?;
    let expected_campaign_revision = u64::try_from(
        row.try_get::<i64, _>("expected_campaign_revision")
            .map_err(|error| error.to_string())?,
    )
    .map_err(|_| "expected campaign revision is negative".to_string())?;
    let job = SettlementJob {
        job_id: row.try_get("job_id").map_err(|error| error.to_string())?,
        capture_id,
        match_id: row.try_get("match_id").map_err(|error| error.to_string())?,
        campaign_id: row
            .try_get("campaign_id")
            .map_err(|error| error.to_string())?,
        intent_id: row
            .try_get("intent_id")
            .map_err(|error| error.to_string())?,
        intent_hash: row
            .try_get("intent_hash")
            .map_err(|error| error.to_string())?,
        expected_campaign_revision,
        expected_campaign_state_hash: row
            .try_get("expected_campaign_state_hash")
            .map_err(|error| error.to_string())?,
        queue_lane: QueueLane::parse(
            &row.try_get::<String, _>("queue_lane")
                .map_err(|error| error.to_string())?,
        )?,
        intent,
        authorization_request_id,
        entitlement_issued_at_epoch,
        entitlement_expires_at_epoch,
        entitlement_nonce,
        authorized_intent,
        signer_receipt_hash: row
            .try_get("signer_receipt_hash")
            .map_err(|error| error.to_string())?,
        remote_attempts: row
            .try_get("remote_attempts")
            .map_err(|error| error.to_string())?,
        lease_owner: row
            .try_get::<Option<String>, _>("lease_owner")
            .map_err(|error| error.to_string())?
            .ok_or_else(|| "claimed settlement job has no lease owner".to_string())?,
        lease_generation: row
            .try_get("lease_generation")
            .map_err(|error| error.to_string())?,
    };
    job.intent.validate()?;
    if job.intent.intent_id != job.intent_id
        || hash_json(&job.intent)? != job.intent_hash
        || job.lease_generation <= 0
        || job.remote_attempts < 0
        || job.remote_attempts >= MAX_REMOTE_ATTEMPTS
        || job.expected_campaign_state_hash.len() != 64
    {
        return Err("claimed settlement job failed durable binding validation".to_string());
    }
    if let Some(authorized) = job.authorized_intent.as_ref() {
        authorized.validate()?;
        if authorized.intent_id != job.intent_id
            || authorized.term_id != job.intent.term_id
            || authorized.idempotency_key != job.intent.idempotency_key
        {
            return Err("authorized settlement intent changed durable identity".to_string());
        }
    }
    Ok(job)
}

async fn process_claimed_job(
    pool: &PgPool,
    cex: &CexClient,
    job: SettlementJob,
) -> Result<(), String> {
    let remote_attempt = sqlx::query_scalar::query_scalar::<_, Option<i32>>(
        "select public.trnm_online_begin_settlement_remote_attempt_v1($1, $2, $3)",
    )
    .bind(&job.job_id)
    .bind(&job.lease_owner)
    .bind(job.lease_generation)
    .fetch_one(pool)
    .await
    .map_err(|error| format!("begin settlement remote attempt: {error}"))?
    .ok_or_else(|| "settlement job lost its lease before remote execution".to_string())?;

    let authorized = match job.authorized_intent.clone() {
        Some(intent) => AuthorizedSettlementIntent {
            intent,
            authorization_request_id: job.authorization_request_id.clone(),
            signer_receipt_hash: job.signer_receipt_hash.clone(),
        },
        None => match cex
            .authorize_settlement_intent(
                &job.intent,
                &job.authorization_request_id,
                job.entitlement_issued_at_epoch,
                job.entitlement_expires_at_epoch,
                &job.entitlement_nonce,
            )
            .await
        {
            Ok(authorized) => {
                let stored = sqlx::query_scalar::query_scalar::<_, bool>(
                    "select public.trnm_online_store_settlement_authorization_v1(
                        $1, $2, $3, $4, $5, $6
                     )",
                )
                .bind(&job.job_id)
                .bind(&job.lease_owner)
                .bind(job.lease_generation)
                .bind(&authorized.authorization_request_id)
                .bind(
                    serde_json::to_value(&authorized.intent)
                        .map_err(|error| error.to_string())?,
                )
                .bind(&authorized.signer_receipt_hash)
                .fetch_one(pool)
                .await
                .map_err(|error| format!("persist settlement authorization: {error}"))?;
                if !stored {
                    return Err(
                        "settlement authorization finished after its lease was lost".to_string(),
                    );
                }
                authorized
            }
            Err(error) => {
                handle_external_failure(pool, &job, remote_attempt, error).await?;
                return Ok(());
            }
        },
    };

    let receipt = match cex
        .submit_authorized_settlement_intent(&authorized.intent)
        .await
    {
        Ok(receipt) => receipt,
        Err(error) => {
            handle_external_failure(pool, &job, remote_attempt, error).await?;
            return Ok(());
        }
    };
    if receipt.progression_class == ReceiptProgressionClass::RecoverableHold {
        handle_external_failure(
            pool,
            &job,
            remote_attempt,
            ExternalSettlementError::Retryable(
                receipt
                    .reason
                    .clone()
                    .unwrap_or_else(|| "CEX returned a recoverable settlement hold".to_string()),
            ),
        )
        .await?;
        return Ok(());
    }
    let receipt_hash = hash_json(&receipt)?;
    let completed = sqlx::query_scalar::query_scalar::<_, bool>(
        "select public.trnm_online_complete_settlement_job_v1(
            $1, $2, $3, $4, $5, $6, $7
         )",
    )
    .bind(&job.job_id)
    .bind(&job.lease_owner)
    .bind(job.lease_generation)
    .bind(&receipt.receipt_id)
    .bind(receipt_hash)
    .bind(serde_json::to_value(&receipt).map_err(|error| error.to_string())?)
    .bind(Option::<Value>::None)
    .fetch_one(pool)
    .await
    .map_err(|error| format!("complete settlement job: {error}"))?;
    if !completed {
        return Err("remote settlement completed after its lease was lost".to_string());
    }
    Ok(())
}

async fn handle_external_failure(
    pool: &PgPool,
    job: &SettlementJob,
    remote_attempt: i32,
    error: ExternalSettlementError,
) -> Result<(), String> {
    match error {
        ExternalSettlementError::Retryable(message) => {
            let delay = retry_delay_milliseconds(&job.job_id, remote_attempt);
            let state = sqlx::query_scalar::query_scalar::<_, Option<String>>(
                "select public.trnm_online_retry_settlement_job_v1($1, $2, $3, $4, $5)",
            )
            .bind(&job.job_id)
            .bind(&job.lease_owner)
            .bind(job.lease_generation)
            .bind(message)
            .bind(delay)
            .fetch_one(pool)
            .await
            .map_err(|error| format!("mark settlement retry: {error}"))?;
            if state.as_deref() == Some("dead_letter") {
                mark_capture_state(
                    pool,
                    &job.capture_id,
                    "dead_letter",
                    "settlement remote retry budget exhausted",
                )
                .await?;
            } else if state.as_deref() != Some("retryable") {
                return Err("settlement retry lost its lease fence".to_string());
            }
        }
        ExternalSettlementError::Permanent(message) => {
            let dead = sqlx::query_scalar::query_scalar::<_, bool>(
                "select public.trnm_online_dead_letter_settlement_job_v1($1, $2, $3, $4)",
            )
            .bind(&job.job_id)
            .bind(&job.lease_owner)
            .bind(job.lease_generation)
            .bind(&message)
            .fetch_one(pool)
            .await
            .map_err(|error| format!("dead-letter settlement job: {error}"))?;
            if !dead {
                return Err("permanent settlement failure lost its lease fence".to_string());
            }
            mark_capture_state(pool, &job.capture_id, "dead_letter", &message).await?;
        }
    }
    Ok(())
}

fn retry_delay_milliseconds(job_id: &str, remote_attempt: i32) -> i64 {
    let exponent = u32::try_from(remote_attempt.saturating_sub(1).clamp(0, 8)).unwrap_or(0);
    let base = 1_000_i64.saturating_mul(1_i64 << exponent).min(300_000);
    let digest = Sha256::digest(job_id.as_bytes());
    let jitter = i64::from(u16::from_be_bytes([digest[0], digest[1]]) % 251);
    (base + jitter).min(300_000)
}

async fn apply_ready_captures(pool: &PgPool, limit: usize) -> Result<u64, String> {
    let limit = i64::try_from(limit).map_err(|_| "apply limit is too large".to_string())?;
    let capture_ids = sqlx::query_scalar::query_scalar::<_, String>(
        "select capture.capture_id
           from public.trnm_online_settlement_captures capture
          where capture.state = 'active'
            and exists (
                select 1 from public.trnm_online_settlement_jobs job
                 where job.capture_id = capture.capture_id
            )
            and not exists (
                select 1 from public.trnm_online_settlement_jobs job
                 where job.capture_id = capture.capture_id
                   and (job.state <> 'succeeded' or job.campaign_applied_at is not null)
            )
          order by capture.created_at, capture.capture_id
          limit $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|error| format!("scan ready settlement captures: {error}"))?;

    let mut applied = 0_u64;
    for capture_id in capture_ids {
        match apply_capture(pool, &capture_id).await? {
            ApplyCaptureResult::NotReady => {}
            ApplyCaptureResult::Applied { finalized } => {
                applied = applied.saturating_add(1);
                tracing::info!(%capture_id, finalized, "settlement capture applied");
            }
            ApplyCaptureResult::Stale(reason) => {
                mark_capture_state(pool, &capture_id, "stale", &reason).await?;
                tracing::warn!(%capture_id, %reason, "settlement capture became stale");
            }
            ApplyCaptureResult::DeadLetter(reason) => {
                mark_capture_state(pool, &capture_id, "dead_letter", &reason).await?;
                tracing::error!(%capture_id, %reason, "settlement capture dead-lettered");
            }
        }
    }
    Ok(applied)
}

enum ApplyCaptureResult {
    NotReady,
    Applied { finalized: bool },
    Stale(String),
    DeadLetter(String),
}

async fn apply_capture(pool: &PgPool, capture_id: &str) -> Result<ApplyCaptureResult, String> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(|error| format!("begin settlement apply: {error}"))?;
    let capture = sqlx::query::query(
        "select match_id, terminal_identity_hash, campaign_fences_json,
                head_intent_ids_json
           from public.trnm_online_settlement_captures
          where capture_id = $1 and state = 'active'
          for update skip locked",
    )
    .bind(capture_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| format!("lock settlement capture: {error}"))?;
    let Some(capture) = capture else {
        transaction
            .commit()
            .await
            .map_err(|error| format!("commit skipped settlement apply: {error}"))?;
        return Ok(ApplyCaptureResult::NotReady);
    };
    let match_id: Uuid = capture
        .try_get("match_id")
        .map_err(|error| error.to_string())?;
    let expected_terminal_hash: String = capture
        .try_get("terminal_identity_hash")
        .map_err(|error| error.to_string())?;
    let expected_campaign_fences: Value = capture
        .try_get("campaign_fences_json")
        .map_err(|error| error.to_string())?;
    let expected_heads: Value = capture
        .try_get("head_intent_ids_json")
        .map_err(|error| error.to_string())?;

    let match_locked = sqlx::query_scalar::query_scalar::<_, Uuid>(
        "select match_id from public.trnm_online_matches
          where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| format!("lock settlement match during apply: {error}"))?;
    if match_locked.is_none() {
        transaction
            .rollback()
            .await
            .map_err(|error| format!("rollback missing settlement match: {error}"))?;
        return Ok(ApplyCaptureResult::Stale(
            "captured match no longer exists".to_string(),
        ));
    }
    let ready = sqlx::query_scalar::query_scalar::<_, bool>(
        "select public.trnm_online_settlement_match_ready_v1($1)",
    )
    .bind(match_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(|error| format!("revalidate terminal settlement marker: {error}"))?;
    if !ready {
        transaction
            .rollback()
            .await
            .map_err(|error| format!("rollback stale terminal marker: {error}"))?;
        return Ok(ApplyCaptureResult::Stale(
            "terminal publication identity changed after capture".to_string(),
        ));
    }
    let terminal_identity = load_terminal_identity(&mut transaction, match_id).await?;
    if hash_json(&terminal_identity)? != expected_terminal_hash {
        transaction
            .rollback()
            .await
            .map_err(|error| format!("rollback terminal identity mismatch: {error}"))?;
        return Ok(ApplyCaptureResult::Stale(
            "terminal identity hash changed after capture".to_string(),
        ));
    }

    let mut campaigns = load_campaign_rows(&mut transaction, match_id).await?;
    if campaigns.len() != 2 || campaign_fences_json(&campaigns) != expected_campaign_fences {
        transaction
            .rollback()
            .await
            .map_err(|error| format!("rollback campaign fence mismatch: {error}"))?;
        return Ok(ApplyCaptureResult::Stale(
            "one or more campaign revision/state hashes changed after capture".to_string(),
        ));
    }

    let jobs = load_capture_jobs(&mut transaction, capture_id).await?;
    if jobs.is_empty() || jobs.iter().any(|job| job.receipt.is_none()) {
        transaction
            .commit()
            .await
            .map_err(|error| format!("commit not-ready settlement apply: {error}"))?;
        return Ok(ApplyCaptureResult::NotReady);
    }
    let expected_heads = expected_heads
        .as_object()
        .ok_or_else(|| "settlement capture head-intent map is not an object".to_string())?;
    let jobs_by_campaign = jobs
        .iter()
        .map(|job| (job.campaign_id.as_str(), job))
        .collect::<BTreeMap<_, _>>();

    for campaign in &mut campaigns {
        let expected = expected_heads.get(&campaign.campaign_id).ok_or_else(|| {
            format!(
                "capture omitted campaign head descriptor for {}",
                campaign.campaign_id
            )
        })?;
        if expected.is_null() {
            if campaign_head(&campaign.campaign).is_some() {
                transaction
                    .rollback()
                    .await
                    .map_err(|error| format!("rollback unexpected campaign head: {error}"))?;
                return Ok(ApplyCaptureResult::Stale(format!(
                    "campaign {} gained a settlement head after capture",
                    campaign.campaign_id
                )));
            }
            continue;
        }
        let expected = expected
            .as_object()
            .ok_or_else(|| "capture campaign head descriptor is not an object".to_string())?;
        let expected_intent_id = expected
            .get("intent_id")
            .and_then(Value::as_str)
            .ok_or_else(|| "capture head has no intent_id".to_string())?;
        let expected_intent_hash = expected
            .get("intent_hash")
            .and_then(Value::as_str)
            .ok_or_else(|| "capture head has no intent_hash".to_string())?;
        let expected_lane = QueueLane::parse(
            expected
                .get("queue_lane")
                .and_then(Value::as_str)
                .ok_or_else(|| "capture head has no queue_lane".to_string())?,
        )?;
        let Some((current_lane, current_intent)) = campaign_head(&campaign.campaign) else {
            transaction
                .rollback()
                .await
                .map_err(|error| format!("rollback missing campaign head: {error}"))?;
            return Ok(ApplyCaptureResult::Stale(format!(
                "campaign {} lost its captured settlement head",
                campaign.campaign_id
            )));
        };
        if current_lane != expected_lane
            || current_intent.intent_id != expected_intent_id
            || hash_json(current_intent)? != expected_intent_hash
        {
            transaction
                .rollback()
                .await
                .map_err(|error| format!("rollback changed campaign head: {error}"))?;
            return Ok(ApplyCaptureResult::Stale(format!(
                "campaign {} settlement head changed after capture",
                campaign.campaign_id
            )));
        }
        let Some(job) = jobs_by_campaign.get(campaign.campaign_id.as_str()) else {
            transaction
                .rollback()
                .await
                .map_err(|error| format!("rollback missing settlement job: {error}"))?;
            return Ok(ApplyCaptureResult::DeadLetter(format!(
                "capture has no succeeded job for campaign {}",
                campaign.campaign_id
            )));
        };
        if job.intent_id != expected_intent_id
            || job.intent_hash != expected_intent_hash
            || job.queue_lane != expected_lane
            || job.expected_campaign_revision != campaign.campaign_revision
            || job.expected_campaign_state_hash != campaign.state_hash
        {
            transaction
                .rollback()
                .await
                .map_err(|error| format!("rollback job/campaign binding mismatch: {error}"))?;
            return Ok(ApplyCaptureResult::DeadLetter(format!(
                "settlement job is not bound to campaign {} capture fence",
                campaign.campaign_id
            )));
        }
        let receipt = job.receipt.clone().expect("guarded receipt");
        if let Err(error) = receipt.validate_for(current_intent) {
            transaction
                .rollback()
                .await
                .map_err(|rollback| format!("rollback invalid receipt binding: {rollback}"))?;
            return Ok(ApplyCaptureResult::DeadLetter(format!(
                "receipt/campaign intent binding failed: {error}"
            )));
        }
        if receipt.progression_class == ReceiptProgressionClass::RecoverableHold {
            transaction
                .rollback()
                .await
                .map_err(|error| format!("rollback recoverable receipt: {error}"))?;
            return Ok(ApplyCaptureResult::DeadLetter(
                "recoverable receipt was incorrectly persisted as terminal success".to_string(),
            ));
        }
        let backend = CapturedReceiptBackend {
            receipt,
            wallet_snapshot: job.wallet_snapshot.clone(),
        };
        let report = match campaign.campaign.reconcile_economy(&backend, 1) {
            Ok(report) => report,
            Err(error) => {
                transaction
                    .rollback()
                    .await
                    .map_err(|rollback| format!("rollback receipt apply failure: {rollback}"))?;
                return Ok(ApplyCaptureResult::DeadLetter(format!(
                    "apply captured economic receipt failed: {error}"
                )));
            }
        };
        if report.attempted != 1 || report.recoverable_holds != 0 {
            transaction
                .rollback()
                .await
                .map_err(|error| format!("rollback non-terminal receipt apply: {error}"))?;
            return Ok(ApplyCaptureResult::DeadLetter(format!(
                "captured receipt did not terminally consume campaign {} head",
                campaign.campaign_id
            )));
        }
        campaign.campaign.revision = campaign.campaign.revision.saturating_add(1);
        if let Err(error) = campaign.campaign.validate() {
            transaction
                .rollback()
                .await
                .map_err(|rollback| format!("rollback invalid applied campaign: {rollback}"))?;
            return Ok(ApplyCaptureResult::DeadLetter(format!(
                "validate receipt-applied campaign failed: {error}"
            )));
        }
    }

    for campaign in &campaigns {
        let next_state_hash = hash_json(&campaign.campaign)?;
        let updated = sqlx::query::query(
            "update public.trnm_online_campaigns
                set campaign_revision = $2, schema_revision = $3,
                    state_hash = $4, campaign_json = $5,
                    updated_at = clock_timestamp()
              where campaign_id = $1
                and campaign_revision = $6
                and state_hash = $7",
        )
        .bind(&campaign.campaign_id)
        .bind(i64::try_from(campaign.campaign.revision).map_err(|_| {
            "updated campaign revision exceeds PostgreSQL bigint range".to_string()
        })?)
        .bind(i32::from(campaign.campaign.schema_revision))
        .bind(next_state_hash)
        .bind(
            serde_json::to_value(&campaign.campaign).map_err(|error| error.to_string())?,
        )
        .bind(i64::try_from(campaign.campaign_revision).map_err(|_| {
            "captured campaign revision exceeds PostgreSQL bigint range".to_string()
        })?)
        .bind(&campaign.state_hash)
        .execute(&mut *transaction)
        .await
        .map_err(|error| format!("persist receipt-applied campaign: {error}"))?;
        if updated.rows_affected() != 1 {
            transaction
                .rollback()
                .await
                .map_err(|error| format!("rollback failed campaign CAS: {error}"))?;
            return Ok(ApplyCaptureResult::Stale(format!(
                "campaign {} failed exact revision/state-hash CAS",
                campaign.campaign_id
            )));
        }
    }

    for job in &jobs {
        let marked = sqlx::query::query(
            "update public.trnm_online_settlement_jobs
                set campaign_applied_at = clock_timestamp(),
                    updated_at = clock_timestamp()
              where job_id = $1
                and capture_id = $2
                and state = 'succeeded'
                and campaign_applied_at is null",
        )
        .bind(&job.job_id)
        .bind(capture_id)
        .execute(&mut *transaction)
        .await
        .map_err(|error| format!("mark settlement job campaign-applied: {error}"))?;
        if marked.rows_affected() != 1 {
            transaction
                .rollback()
                .await
                .map_err(|error| format!("rollback lost apply marker: {error}"))?;
            return Ok(ApplyCaptureResult::DeadLetter(format!(
                "settlement job {} lost its unapplied-success fence",
                job.job_id
            )));
        }
    }

    let finalized = campaigns
        .iter()
        .all(|campaign| campaign_head(&campaign.campaign).is_none());
    if finalized {
        finalize_match_in_transaction(&mut transaction, match_id).await?;
    }
    let next_capture_state = if finalized { "finalized" } else { "applied" };
    let capture_update = sqlx::query::query(
        "update public.trnm_online_settlement_captures
            set state = $2, applied_at = clock_timestamp(),
                updated_at = clock_timestamp()
          where capture_id = $1 and state = 'active'",
    )
    .bind(capture_id)
    .bind(next_capture_state)
    .execute(&mut *transaction)
    .await
    .map_err(|error| format!("advance settlement capture state: {error}"))?;
    if capture_update.rows_affected() != 1 {
        return Err("settlement capture lost its active apply fence".to_string());
    }
    transaction
        .commit()
        .await
        .map_err(|error| format!("commit settlement apply: {error}"))?;
    Ok(ApplyCaptureResult::Applied { finalized })
}

#[derive(Clone)]
struct CaptureJobRow {
    job_id: String,
    campaign_id: String,
    intent_id: String,
    intent_hash: String,
    expected_campaign_revision: u64,
    expected_campaign_state_hash: String,
    queue_lane: QueueLane,
    receipt: Option<EconomicReceipt>,
    wallet_snapshot: Option<WalletSnapshot>,
}

async fn load_capture_jobs(
    transaction: &mut sqlx::transaction::Transaction<'_, Postgres>,
    capture_id: &str,
) -> Result<Vec<CaptureJobRow>, String> {
    let rows = sqlx::query::query(
        "select job_id, campaign_id, intent_id, intent_hash,
                expected_campaign_revision, expected_campaign_state_hash,
                queue_lane, state, receipt_json, wallet_snapshot_json,
                campaign_applied_at
           from public.trnm_online_settlement_jobs
          where capture_id = $1
          order by campaign_id
          for update",
    )
    .bind(capture_id)
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| format!("lock captured settlement jobs: {error}"))?;
    let mut jobs = Vec::with_capacity(rows.len());
    for row in rows {
        let state: String = row.try_get("state").map_err(|error| error.to_string())?;
        let applied: Option<chrono::DateTime<chrono::Utc>> = row
            .try_get("campaign_applied_at")
            .map_err(|error| error.to_string())?;
        if state != "succeeded" || applied.is_some() {
            return Ok(Vec::new());
        }
        let receipt = row
            .try_get::<Option<Value>, _>("receipt_json")
            .map_err(|error| error.to_string())?
            .map(serde_json::from_value::<EconomicReceipt>)
            .transpose()
            .map_err(|error| format!("decode captured receipt: {error}"))?;
        let wallet_snapshot = row
            .try_get::<Option<Value>, _>("wallet_snapshot_json")
            .map_err(|error| error.to_string())?
            .map(serde_json::from_value::<WalletSnapshot>)
            .transpose()
            .map_err(|error| format!("decode captured wallet snapshot: {error}"))?;
        jobs.push(CaptureJobRow {
            job_id: row.try_get("job_id").map_err(|error| error.to_string())?,
            campaign_id: row
                .try_get("campaign_id")
                .map_err(|error| error.to_string())?,
            intent_id: row
                .try_get("intent_id")
                .map_err(|error| error.to_string())?,
            intent_hash: row
                .try_get("intent_hash")
                .map_err(|error| error.to_string())?,
            expected_campaign_revision: u64::try_from(
                row.try_get::<i64, _>("expected_campaign_revision")
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|_| "captured expected revision is negative".to_string())?,
            expected_campaign_state_hash: row
                .try_get("expected_campaign_state_hash")
                .map_err(|error| error.to_string())?,
            queue_lane: QueueLane::parse(
                &row.try_get::<String, _>("queue_lane")
                    .map_err(|error| error.to_string())?,
            )?,
            receipt,
            wallet_snapshot,
        });
    }
    Ok(jobs)
}

struct CapturedReceiptBackend {
    receipt: EconomicReceipt,
    wallet_snapshot: Option<WalletSnapshot>,
}

impl EconomyBackend for CapturedReceiptBackend {
    fn backend_id(&self) -> &str {
        &self.receipt.backend_id
    }

    fn execute(&self, intent: &EconomicIntent) -> Result<EconomicReceipt, String> {
        self.receipt.validate_for(intent)?;
        Ok(self.receipt.clone())
    }

    fn wallet_snapshot(
        &self,
        binding: &EconomyAccountBinding,
        _cursor: u64,
    ) -> Result<Option<WalletSnapshot>, String> {
        if self
            .wallet_snapshot
            .as_ref()
            .is_some_and(|snapshot| snapshot.account_id != binding.account_id)
        {
            return Err("captured wallet snapshot belongs to another account".to_string());
        }
        Ok(self.wallet_snapshot.clone())
    }
}

async fn mark_capture_state(
    pool: &PgPool,
    capture_id: &str,
    state: &str,
    error: &str,
) -> Result<(), String> {
    if !matches!(state, "stale" | "dead_letter") {
        return Err(format!("unsupported capture terminal state {state}"));
    }
    sqlx::query::query(
        "update public.trnm_online_settlement_captures
            set state = $2, last_error = left($3, 1024),
                updated_at = clock_timestamp()
          where capture_id = $1 and state = 'active'",
    )
    .bind(capture_id)
    .bind(state)
    .bind(error)
    .execute(pool)
    .await
    .map_err(|db_error| format!("mark settlement capture {state}: {db_error}"))?;
    Ok(())
}

fn hash_json<T: Serialize>(value: &T) -> Result<String, String> {
    let bytes = serde_json::to_vec(value).map_err(|error| error.to_string())?;
    Ok(hash_bytes(&bytes))
}

fn hash_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use trnm_economy_protocol::{
        ActorRef, EconomicIntentKind, IdempotencyKey, TERM_EXCHANGE_PROTOCOL_VERSION,
    };

    fn intent(id: &str) -> EconomicIntent {
        EconomicIntent {
            protocol_version: TERM_EXCHANGE_PROTOCOL_VERSION.to_string(),
            intent_id: id.to_string(),
            term_id: "settlement-test".to_string(),
            term_version: "v1".to_string(),
            domain: "trnm_game".to_string(),
            kind: EconomicIntentKind::CompleteContract,
            idempotency_key: IdempotencyKey {
                scope: "campaign-a".to_string(),
                key: id.to_string(),
            },
            actors: vec![ActorRef {
                actor_id: "player-a".to_string(),
                actor_kind: "trnm_player".to_string(),
                account_id: Some("account-a".to_string()),
            }],
            assets: Vec::new(),
            amount_credits: Some(0),
            currency: None,
            metadata: json!({}),
            created_at_epoch: 1,
        }
    }

    #[test]
    fn job_identity_is_stable_and_capture_bound() {
        let match_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
        let hash = hash_json(&intent("intent-a")).unwrap();
        let first = deterministic_job_id(
            "trnm-settlement-capture-v1:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            match_id,
            "campaign-a",
            "intent-a",
            &hash,
        );
        let second = deterministic_job_id(
            "trnm-settlement-capture-v1:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            match_id,
            "campaign-a",
            "intent-a",
            &hash,
        );
        let other_capture = deterministic_job_id(
            "trnm-settlement-capture-v1:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            match_id,
            "campaign-a",
            "intent-a",
            &hash,
        );
        assert_eq!(first, second);
        assert_ne!(first, other_capture);
        assert!(first.starts_with("trnm-settlement-outbox-v1:"));
        assert_eq!(first.len(), "trnm-settlement-outbox-v1:".len() + 64);
    }

    #[test]
    fn compensation_lane_is_selected_before_ordinary_intents() {
        let mut campaign = CampaignSaveV1::default();
        campaign.pending_economic_intents.push(intent("ordinary"));
        campaign
            .pending_economic_compensations
            .push(intent("compensation"));
        let (lane, head) = campaign_head(&campaign).unwrap();
        assert_eq!(lane, QueueLane::Compensation);
        assert_eq!(head.intent_id, "compensation");
    }

    #[test]
    fn retry_delay_is_bounded_deterministic_and_monotonic_before_cap() {
        let first = retry_delay_milliseconds("job-a", 1);
        let second = retry_delay_milliseconds("job-a", 2);
        let repeated = retry_delay_milliseconds("job-a", 2);
        let capped = retry_delay_milliseconds("job-a", 16);
        assert!(first >= 1_000);
        assert!(second > first);
        assert_eq!(second, repeated);
        assert!(capped <= 300_000);
    }

    #[test]
    fn migration_contract_contains_lease_fenced_remote_and_apply_markers() {
        let normalized = MIGRATION_V17.split_whitespace().collect::<Vec<_>>().join(" ");
        for required in [
            "trnm_online_settlement_captures",
            "expected_campaign_state_hash",
            "trnm_online_claim_settlement_job_v2",
            "trnm_online_store_settlement_authorization_v1",
            "trnm_online_complete_settlement_job_v1",
            "campaign_applied_at",
        ] {
            assert!(normalized.contains(required), "missing {required}");
        }
    }
}
