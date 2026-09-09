use serde_json::json;
use sqlx::{executor::Executor, row::Row};
use sqlx_postgres::{PgPool, PgPoolOptions};
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;

const OUTBOX_MIGRATION: &str = include_str!("../migrations/0016_online_settlement_outbox_v1.sql");
const WORKER_MIGRATION: &str =
    include_str!("../migrations/0017_online_settlement_worker_runtime_v1.sql");

fn require_database_url() -> Option<String> {
    match std::env::var("TRNM_SETTLEMENT_TEST_DATABASE_URL") {
        Ok(value) if !value.trim().is_empty() => Some(value),
        _ if std::env::var("TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST").as_deref() == Ok("1") => {
            panic!("TRNM_SETTLEMENT_TEST_DATABASE_URL is required")
        }
        _ => None,
    }
}

async fn reset_schema(pool: &PgPool) {
    pool.execute("drop schema if exists public cascade; create schema public")
        .await
        .expect("reset settlement capture boundary schema");
    pool.execute(
        "create table public.trnm_online_campaigns (
            campaign_id text primary key,
            state_hash text not null
        );
        create table public.trnm_online_matches (
            match_id uuid primary key,
            phase text not null,
            settlement_state text not null,
            terminal_publication_state text not null,
            checkpoint_sequence bigint not null,
            next_sequence bigint not null,
            result_hash text,
            terminal_publication_actor_generation bigint,
            assigned_instance_id text,
            assigned_instance_epoch bigint,
            assigned_physical_host_id text,
            authoritative_tick bigint not null,
            match_revision bigint not null,
            snapshot_hash text not null,
            updated_at timestamptz not null default clock_timestamp()
        );
        create table public.trnm_online_match_members (
            match_id uuid not null references public.trnm_online_matches(match_id),
            player_id text not null,
            campaign_id text not null references public.trnm_online_campaigns(campaign_id),
            next_input_sequence bigint not null,
            primary key (match_id, player_id)
        );
        create table public.trnm_online_terminal_publication_acks (
            match_id uuid primary key references public.trnm_online_matches(match_id),
            local_tombstone_state text not null,
            actor_generation bigint not null,
            instance_id text not null,
            actor_epoch bigint not null,
            physical_host_id text not null,
            authoritative_tick bigint not null,
            next_sequence bigint not null,
            match_revision bigint not null,
            next_input_sequences jsonb not null,
            snapshot_hash text not null,
            phase text not null,
            result_hash text,
            published_settlement_state text not null
        )",
    )
    .await
    .expect("create settlement capture boundary scaffold");
    pool.execute(OUTBOX_MIGRATION)
        .await
        .expect("apply settlement outbox migration");
    pool.execute(WORKER_MIGRATION)
        .await
        .expect("apply settlement worker migration");
}

async fn claim_and_begin_remote(
    pool: &PgPool,
    owner: &str,
    external_start_count: &AtomicUsize,
) -> Option<(String, i64, i32)> {
    let row = sqlx::query::query(
        "select job_id, lease_generation
           from public.trnm_online_claim_settlement_job_v2($1, 30000)",
    )
    .bind(owner)
    .fetch_optional(pool)
    .await
    .expect("claim settlement job");
    let row = row?;
    let job_id = row.get::<String, _>("job_id");
    let generation = row.get::<i64, _>("lease_generation");
    let attempt = sqlx::query_scalar::query_scalar::<_, Option<i32>>(
        "select public.trnm_online_begin_settlement_remote_attempt_v1($1, $2, $3)",
    )
    .bind(&job_id)
    .bind(owner)
    .bind(generation)
    .fetch_one(pool)
    .await
    .expect("begin settlement remote attempt")
    .expect("fresh live lease must authorize one remote attempt");
    external_start_count.fetch_add(1, Ordering::SeqCst);
    Some((job_id, generation, attempt))
}

#[tokio::test]
async fn uncommitted_capture_cannot_start_external_settlement() {
    let Some(database_url) = require_database_url() else {
        eprintln!("capture commit boundary test skipped: no test database URL");
        return;
    };
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await
        .expect("connect settlement capture boundary database");
    reset_schema(&pool).await;

    let match_id = Uuid::parse_str("44444444-4444-4444-4444-444444444444").unwrap();
    let campaign_id = "capture-boundary-campaign";
    sqlx::query::query(
        "insert into public.trnm_online_campaigns (campaign_id, state_hash)
         values ($1, $2)",
    )
    .bind(campaign_id)
    .bind("c".repeat(64))
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query::query(
        "insert into public.trnm_online_matches (
            match_id, phase, settlement_state, terminal_publication_state,
            checkpoint_sequence, next_sequence, result_hash,
            terminal_publication_actor_generation, assigned_instance_id,
            assigned_instance_epoch, assigned_physical_host_id,
            authoritative_tick, match_revision, snapshot_hash
         ) values (
            $1, 'complete', 'pending', 'acknowledged',
            0, 0, $2, 1, 'instance-boundary', 1, 'host-boundary', 0, 0, $3
         )",
    )
    .bind(match_id)
    .bind("d".repeat(64))
    .bind("e".repeat(64))
    .execute(&pool)
    .await
    .unwrap();

    let capture_id = format!("trnm-settlement-capture-v1:{}", "1".repeat(64));
    let job_id = format!("trnm-settlement-outbox-v1:{}", "2".repeat(64));
    let intent_hash = "a".repeat(64);

    let mut capture_transaction = pool.begin().await.expect("begin capture transaction");
    sqlx::query::query(
        "insert into public.trnm_online_settlement_captures (
            capture_id, contract_version, match_id, capture_generation,
            terminal_identity_hash, terminal_identity_json,
            campaign_fences_json, head_intent_ids_json, state
         ) values (
            $1, 'trnm_settlement_capture_v1', $2, 1,
            $3, '{}', '{}', '{}', 'active'
         )",
    )
    .bind(&capture_id)
    .bind(match_id)
    .bind("b".repeat(64))
    .execute(&mut *capture_transaction)
    .await
    .unwrap();
    sqlx::query::query(
        "insert into public.trnm_online_settlement_jobs (
            job_id, contract_version, capture_id, capture_generation,
            match_id, campaign_id, intent_id, intent_hash,
            expected_campaign_revision, expected_campaign_state_hash,
            queue_lane, intent_json
         ) values (
            $1, 'trnm_settlement_outbox_v1', $2, 1,
            $3, $4, 'capture-boundary-intent', $5,
            0, $6, 'ordinary', $7
         )",
    )
    .bind(&job_id)
    .bind(&capture_id)
    .bind(match_id)
    .bind(campaign_id)
    .bind(&intent_hash)
    .bind("c".repeat(64))
    .bind(json!({
        "actors": [{
            "actor_id": "capture-boundary-player",
            "actor_kind": "trnm_player",
            "account_id": "capture-boundary-account"
        }]
    }))
    .execute(&mut *capture_transaction)
    .await
    .unwrap();

    let external_start_count = AtomicUsize::new(0);
    let visible_before_commit = sqlx::query_scalar::query_scalar::<_, bool>(
        "select exists(
            select 1 from public.trnm_online_settlement_captures where capture_id = $1
         )",
    )
    .bind(&capture_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(!visible_before_commit);
    assert_eq!(
        claim_and_begin_remote(&pool, "worker-before-commit", &external_start_count).await,
        None
    );
    assert_eq!(external_start_count.load(Ordering::SeqCst), 0);

    capture_transaction
        .commit()
        .await
        .expect("commit capture and job atomically");

    let (left, right) = tokio::join!(
        claim_and_begin_remote(&pool, "worker-after-commit-a", &external_start_count),
        claim_and_begin_remote(&pool, "worker-after-commit-b", &external_start_count)
    );
    let starts = [left, right].into_iter().flatten().collect::<Vec<_>>();
    assert_eq!(starts.len(), 1);
    assert_eq!(starts[0].0, job_id);
    assert_eq!(starts[0].1, 1);
    assert_eq!(starts[0].2, 1);
    assert_eq!(external_start_count.load(Ordering::SeqCst), 1);

    let durable = sqlx::query::query(
        "select state, remote_attempts, lease_generation
           from public.trnm_online_settlement_jobs
          where job_id = $1",
    )
    .bind(&job_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(durable.get::<String, _>("state"), "leased");
    assert_eq!(durable.get::<i32, _>("remote_attempts"), 1);
    assert_eq!(durable.get::<i64, _>("lease_generation"), 1);
}
