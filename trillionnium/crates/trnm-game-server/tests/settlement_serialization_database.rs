use serde_json::json;
use sqlx::{executor::Executor, row::Row};
use sqlx_postgres::{PgPool, PgPoolOptions};
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
        .expect("reset settlement serialization schema");
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
    .expect("create settlement serialization scaffold");
    pool.execute(OUTBOX_MIGRATION)
        .await
        .expect("apply outbox migration");
    pool.execute(WORKER_MIGRATION)
        .await
        .expect("apply worker migration");
}

async fn insert_match_campaign_job(
    pool: &PgPool,
    ordinal: u64,
    campaign_id: &str,
    account_id: &str,
    intent_id: &str,
) -> String {
    let match_id = Uuid::from_u128(ordinal as u128 + 1);
    let capture_id = format!("trnm-settlement-capture-v1:{:064x}", ordinal + 1);
    let job_id = format!("trnm-settlement-outbox-v1:{:064x}", ordinal + 1);
    let hash = format!("{:064x}", ordinal + 100);

    sqlx::query::query(
        "insert into public.trnm_online_campaigns (campaign_id, state_hash)
         values ($1, $2)",
    )
    .bind(campaign_id)
    .bind("c".repeat(64))
    .execute(pool)
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
            0, 0, $2, 1, 'instance-a', 1, 'host-a', 0, 0, $3
         )",
    )
    .bind(match_id)
    .bind("d".repeat(64))
    .bind("e".repeat(64))
    .execute(pool)
    .await
    .unwrap();
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
    .bind(&hash)
    .execute(pool)
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
            $3, $4, $5, $6, 0, $7, 'ordinary', $8
         )",
    )
    .bind(&job_id)
    .bind(&capture_id)
    .bind(match_id)
    .bind(campaign_id)
    .bind(intent_id)
    .bind(&hash)
    .bind("c".repeat(64))
    .bind(json!({
        "actors": [{
            "actor_id": format!("actor-{ordinal}"),
            "actor_kind": "player",
            "account_id": account_id
        }]
    }))
    .execute(pool)
    .await
    .unwrap();

    job_id
}

async fn claim(pool: &PgPool, owner: &str) -> Option<(String, i64, String)> {
    sqlx::query::query(
        "select job_id, lease_generation,
                public.trnm_online_settlement_serialization_key_v1(
                    campaign_id, intent_json
                ) as serialization_key
           from public.trnm_online_claim_settlement_job_v2($1, 30000)",
    )
    .bind(owner)
    .fetch_optional(pool)
    .await
    .unwrap()
    .map(|row| {
        (
            row.get::<String, _>("job_id"),
            row.get::<i64, _>("lease_generation"),
            row.get::<String, _>("serialization_key"),
        )
    })
}

#[tokio::test]
async fn account_serialization_does_not_block_unrelated_work() {
    let Some(database_url) = require_database_url() else {
        eprintln!("settlement serialization database test skipped: no test database URL");
        return;
    };
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await
        .expect("connect settlement serialization database");
    reset_schema(&pool).await;

    let account_a_first =
        insert_match_campaign_job(&pool, 1, "campaign-a1", "account-a", "intent-a1").await;
    let account_a_second =
        insert_match_campaign_job(&pool, 2, "campaign-a2", "account-a", "intent-a2").await;
    let account_b =
        insert_match_campaign_job(&pool, 3, "campaign-b", "account-b", "intent-b").await;

    let account_a_lease = claim(&pool, "worker-a")
        .await
        .expect("oldest account-a job must be claimable");
    assert_eq!(account_a_lease.0, account_a_first);
    assert_eq!(account_a_lease.2, "account-a");

    let (second, third) = tokio::join!(claim(&pool, "worker-b"), claim(&pool, "worker-c"));
    let claimed_unrelated = [second, third].into_iter().flatten().collect::<Vec<_>>();
    assert_eq!(claimed_unrelated.len(), 1);
    assert_eq!(claimed_unrelated[0].0, account_b);
    assert_eq!(claimed_unrelated[0].2, "account-b");
    assert_ne!(claimed_unrelated[0].0, account_a_second);

    assert_eq!(claim(&pool, "worker-d").await, None);

    sqlx::query::query(
        "update public.trnm_online_settlement_jobs
            set lease_expires_at = clock_timestamp() - interval '1 second'
          where job_id = $1",
    )
    .bind(&account_a_lease.0)
    .execute(&pool)
    .await
    .unwrap();

    let reclaimed = claim(&pool, "worker-d")
        .await
        .expect("expired account lease must become recoverable");
    assert_eq!(reclaimed.0, account_a_lease.0);
    assert_eq!(reclaimed.2, "account-a");
    assert!(reclaimed.1 > account_a_lease.1);

    let completed = sqlx::query_scalar::query_scalar::<_, bool>(
        "select public.trnm_online_complete_settlement_job_v1(
            $1, 'worker-d', $2, 'receipt-a', $3, $4, null
         )",
    )
    .bind(&reclaimed.0)
    .bind(reclaimed.1)
    .bind("9".repeat(64))
    .bind(json!({"receipt_id": "receipt-a"}))
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(completed);
    assert_eq!(claim(&pool, "worker-e").await, None);

    let metrics = sqlx::query::query(
        "select remote_succeeded, pending_apply, remote_leased,
                maximum_remote_attempts
           from public.trnm_online_settlement_metrics_v1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(metrics.get::<i64, _>("remote_succeeded"), 1);
    assert_eq!(metrics.get::<i64, _>("pending_apply"), 1);
    assert_eq!(metrics.get::<i64, _>("remote_leased"), 1);
    assert_eq!(metrics.get::<i32, _>("maximum_remote_attempts"), 0);

    sqlx::query::query(
        "update public.trnm_online_settlement_jobs
            set campaign_applied_at = clock_timestamp()
          where job_id = $1",
    )
    .bind(&reclaimed.0)
    .execute(&pool)
    .await
    .unwrap();

    let next_account_a = claim(&pool, "worker-e")
        .await
        .expect("next account job must become eligible after application");
    assert_eq!(next_account_a.0, account_a_second);
    assert_eq!(next_account_a.2, "account-a");
}
