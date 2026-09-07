use axum::extract::DefaultBodyLimit;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use cex::CexClient;
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use published_tick_journal::{
    PublishedTickAbandonmentTombstone, PublishedTickAbandonmentTombstoneInput,
    PublishedTickAckTombstone, PublishedTickAckTombstoneInput, PublishedTickColdWitness,
    PublishedTickHighWater, PublishedTickJournal, PublishedTickRecordInput,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{connection::Connection, executor::Executor, row::Row};
use sqlx_postgres::{PgConnection, PgPool, PgPoolOptions, Postgres};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
}

struct AuthorityClockSnapshot {
    elapsed_ms: f64,
    wake_count: u64,
    cumulative_drift_ticks: f64,
    window_sample_count: usize,
    window_drift_ticks: Option<f64>,
    latest_lateness_ticks: Option<f64>,
    max_recent_lateness_ticks: Option<f64>,
    last_wake_age_ms: Option<f64>,
}

const MATCH_ACTOR_COMMAND_QUEUE: usize = 64;
const MATCH_COMMAND_PERSISTENCE_QUEUE: usize = 1;
const MATCH_ACTOR_PUBLICATION_COMPLETION_QUEUE: usize = 64;
const MATCH_ACTOR_RECEIPT_CACHE: usize = 256;
const MATCH_CHECKPOINT_QUEUE: usize = 2;
const MATCH_CHECKPOINT_INTERVAL_TICKS: u64 = 100;
const MATCH_ACTOR_FENCE_INTERVAL: Duration = Duration::from_secs(1);
const MATCH_ACTOR_WORKER_OPERATION_TIMEOUT: Duration = Duration::from_secs(6);
const MATCH_ACTOR_WORKER_JOIN_TIMEOUT: Duration = Duration::from_secs(2);
const MATCH_ACTOR_WORKER_ABORT_TIMEOUT: Duration = Duration::from_secs(1);
const MATCH_ACTOR_INITIALIZATION_TIMEOUT: Duration = Duration::from_secs(10);
            and a.result_hash = m.result_hash
            and a.published_settlement_state = m.settlement_state,
            false
        ) as tuple_exact
      from trnm_online_terminal_publication_acks a
      left join trnm_online_matches m on m.match_id = a.match_id
     where a.physical_host_id = $1
       and a.local_tombstone_state in ('legacy_bootstrap_pending', 'hot_pending')
       and ($2::uuid is null or a.match_id > $2)
     order by a.match_id
     limit $3";
const TERMINAL_ACK_DATABASE_EVIDENCE_BY_MATCH_SQL: &str = "select
        a.match_id, a.actor_generation, a.instance_id, a.actor_epoch,
        a.physical_host_id, a.authoritative_tick, a.next_sequence,
        a.match_revision, a.next_input_sequences, a.snapshot_hash,
        a.result_hash, a.published_settlement_state, a.local_tombstone_state,
        floor(extract(epoch from a.acknowledged_at) * 1000)::bigint
            as acknowledged_at_unix_ms,
        coalesce(
            m.phase = 'complete'
            and m.checkpoint_sequence = m.next_sequence
            and m.result_hash is not null
            and m.settlement_state in ('pending', 'settled')
            and m.terminal_publication_state = 'acknowledged'
    response: oneshot::Sender<Result<OnlineCommandReceipt, ApiError>>,
}

#[derive(Clone)]
struct ActorReceiptCacheEntry {
    player_id: String,
    request_hash: String,
    receipt: OnlineCommandReceipt,
}

struct MatchCheckpointJob {
    simulation: MissionSimV1,
    snapshot_hash: String,
    next_sequence: u64,
    match_revision: u64,
    terminal: bool,
    completion: Option<oneshot::Sender<Result<(), String>>>,
}

#[derive(Clone)]
struct ActorPublicationCandidate {
    state: PublishedMatchState,
    durable_db_next_sequence: u64,
    durable_db_match_revision: u64,
    publish_to_watch: bool,
}

struct ActorPublicationWorker {
    journal: PublishedTickJournal,
    instance_id: Arc<String>,
    match_id: Uuid,
    actor_id: Uuid,
    actor_epoch: i64,
    candidates: watch::Receiver<Option<ActorPublicationCandidate>>,
    permit: watch::Receiver<bool>,
    durable_recovery_tick: watch::Receiver<u64>,
    published: watch::Sender<PublishedMatchState>,
    publication_acked: watch::Sender<ActorPublicationCursor>,
    completions: mpsc::Sender<ActorPublicationCompletion>,
}

struct PendingPublishedCommandReceipt {
    state_sequence: u64,
    request_hash: String,
    receipt: OnlineCommandReceipt,
    response: oneshot::Sender<Result<OnlineCommandReceipt, ApiError>>,
}
