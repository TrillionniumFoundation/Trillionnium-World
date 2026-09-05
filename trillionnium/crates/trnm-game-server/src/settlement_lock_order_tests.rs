//! PostgreSQL regressions for the actual private settlement functions.
//! Each run creates and drops only its own randomly named database. It never
//! resets the database named by TRNM_SETTLEMENT_TEST_DATABASE_URL.

use super::{apply_capture, load_campaign_rows};
use sqlx::executor::Executor;
use sqlx_postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::{str::FromStr, time::Duration};
use uuid::Uuid;

type TestResult = Result<(), String>;

async fn pool(options: PgConnectOptions) -> Result<PgPool, String> {
    PgPoolOptions::new()
        .max_connections(4)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .map_err(|error| error.to_string())
}

async fn wait_for_row_lock(observer: &PgPool, application: &str) -> TestResult {
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let waiting = sqlx::query_scalar::query_scalar::<_, bool>(
                "select exists(select 1 from pg_catalog.pg_stat_activity
                  where application_name = $1 and datname = current_database()
                    and wait_event_type = 'Lock')",
            )
            .bind(application)
            .fetch_one(observer)
            .await
            .map_err(|error| error.to_string())?;
            if waiting {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .map_err(|_| "actual settlement function did not reach the expected row wait".to_string())?
}

async fn apply_does_not_hold_capture_before_match(
    observer: &PgPool,
    options: &PgConnectOptions,
    match_id: Uuid,
) -> TestResult {
    let application = "trnm-lock-test-apply";
    let worker_pool = pool(options.clone().application_name(application)).await?;
    let mut holder = observer.begin().await.map_err(|error| error.to_string())?;
    sqlx::query::query(
        "select match_id from public.trnm_online_matches where match_id = $1 for update",
    )
    .bind(match_id)
    .execute(&mut *holder)
    .await
    .map_err(|error| error.to_string())?;
    let task_pool = worker_pool.clone();
    let mut task = tokio::task::JoinSet::new();
    task.spawn(async move { apply_capture(&task_pool, "capture-lock-test").await });
    let result = async {
        wait_for_row_lock(observer, application).await?;
        // The old implementation holds capture while blocked on match, making
        // this NOWAIT fail with 55P03. The corrected implementation holds none.
        sqlx::query::query(
            "select capture_id from public.trnm_online_settlement_captures
              where capture_id = 'capture-lock-test' for update nowait",
        )
        .execute(&mut *holder)
        .await
        .map_err(|error| format!("apply acquired capture before match: {error}"))?;
        Ok(())
    }
    .await;
    task.abort_all();
    holder.rollback().await.map_err(|error| error.to_string())?;
    while task.join_next().await.is_some() {}
    worker_pool.close().await;
    result
}

async fn campaigns_lock_by_campaign_not_player(
    observer: &PgPool,
    options: &PgConnectOptions,
    match_id: Uuid,
) -> TestResult {
    let application = "trnm-lock-test-campaign";
    let worker_pool = pool(options.clone().application_name(application)).await?;
    let mut holder = observer.begin().await.map_err(|error| error.to_string())?;
    (&mut *holder)
        .execute(
            "select campaign_id from public.trnm_online_campaigns
              where campaign_id = 'campaign-z' for update",
        )
        .await
        .map_err(|error| error.to_string())?;
    let task_pool = worker_pool.clone();
    let mut task = tokio::task::JoinSet::new();
    task.spawn(async move {
        let mut transaction = task_pool
            .begin()
            .await
            .map_err(|error| error.to_string())?;
        load_campaign_rows(&mut transaction, match_id)
            .await
            .map(|_| ())
    });
    let result = async {
        wait_for_row_lock(observer, application).await?;
        let mut probe = observer.begin().await.map_err(|error| error.to_string())?;
        let lock = (&mut *probe)
            .execute(
                "select campaign_id from public.trnm_online_campaigns
                  where campaign_id = 'campaign-a' for update nowait",
            )
            .await;
        probe.rollback().await.map_err(|error| error.to_string())?;
        // player-a owns campaign-z. Ordering by player reaches z first and
        // leaves a unlocked. Ordering by campaign must already hold a here.
        match lock {
            Err(sqlx::error::Error::Database(error))
                if error.code().as_deref() == Some("55P03") => Ok(()),
            Err(error) => Err(format!("unexpected campaign probe error: {error}")),
            Ok(_) => Err("campaign-a was not locked before campaign-z".to_string()),
        }
    }
    .await;
    task.abort_all();
    holder.rollback().await.map_err(|error| error.to_string())?;
    while task.join_next().await.is_some() {}
    worker_pool.close().await;
    result
}

async fn exercise_database(options: &PgConnectOptions) -> TestResult {
    let observer = pool(options.clone().application_name("trnm-lock-test-observer")).await?;
    let result = async {
        observer
            .execute(
                "create table public.trnm_online_matches (match_id uuid primary key);
                 create table public.trnm_online_settlement_captures (
                   capture_id text primary key, match_id uuid not null,
                   state text not null, terminal_identity_hash text not null,
                   campaign_fences_json jsonb not null, head_intent_ids_json jsonb not null);
                 create table public.trnm_online_campaigns (
                   campaign_id text primary key, campaign_revision bigint not null,
                   state_hash text not null, campaign_json jsonb not null);
                 create table public.trnm_online_match_members (
                   match_id uuid not null, player_id text not null, campaign_id text not null,
                   primary key (match_id, player_id));",
            )
            .await
            .map_err(|error| error.to_string())?;
        let match_id = Uuid::new_v4();
        sqlx::query::query("insert into public.trnm_online_matches values ($1)")
            .bind(match_id)
            .execute(&observer)
            .await
            .map_err(|error| error.to_string())?;
        sqlx::query::query(
            "insert into public.trnm_online_settlement_captures
             values ('capture-lock-test', $1, 'active', 'test-only', '{}', '{}')",
        )
        .bind(match_id)
        .execute(&observer)
        .await
        .map_err(|error| error.to_string())?;
        observer
            .execute(
                "insert into public.trnm_online_campaigns values
                 ('campaign-a', 1, 'test-only', '{}'), ('campaign-z', 1, 'test-only', '{}')",
            )
            .await
            .map_err(|error| error.to_string())?;
        sqlx::query::query(
            "insert into public.trnm_online_match_members values
             ($1, 'player-a', 'campaign-z'), ($1, 'player-z', 'campaign-a')",
        )
        .bind(match_id)
        .execute(&observer)
        .await
        .map_err(|error| error.to_string())?;
        apply_does_not_hold_capture_before_match(&observer, options, match_id).await?;
        campaigns_lock_by_campaign_not_player(&observer, options, match_id).await
    }
    .await;
    observer.close().await;
    result
}

#[tokio::test]
async fn actual_settlement_functions_obey_lock_order() {
    let database_url = match std::env::var("TRNM_SETTLEMENT_TEST_DATABASE_URL") {
        Ok(value) if !value.trim().is_empty() => value,
        _ if std::env::var("TRNM_REQUIRE_SETTLEMENT_DATABASE_TEST").as_deref() == Ok("1") => {
            panic!("TRNM_SETTLEMENT_TEST_DATABASE_URL is required for lock-order evidence")
        }
        _ => {
            eprintln!("lock-order PostgreSQL regression NOT EXECUTED: no test database URL");
            return;
        }
    };
    let base_options = PgConnectOptions::from_str(&database_url).expect("valid test database URL");
    let admin = pool(base_options.clone())
        .await
        .expect("connect test database creator");
    let database = format!("trnm_world_lock_{}", Uuid::new_v4().simple());
    admin
        .execute(format!("create database \"{database}\"").as_str())
        .await
        .expect("test role must be allowed to create an isolated regression database");
    let options = base_options.database(&database);
    let result = tokio::time::timeout(Duration::from_secs(30), exercise_database(&options))
        .await
        .map_err(|_| "isolated lock-order regression exceeded its deadline".to_string())
        .and_then(|result| result);
    // Only this UUID-named database, created by this test, is ever dropped.
    let cleanup = admin
        .execute(format!("drop database \"{database}\" with (force)").as_str())
        .await;
    admin.close().await;
    cleanup.expect("drop isolated lock-order regression database");
    result.expect("actual settlement lock-order regression");
}
