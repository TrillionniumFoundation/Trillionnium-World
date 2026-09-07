const CEX_WRAPPER: &str = include_str!("../src/cex.rs");
const WORKER_WRAPPER: &str = include_str!("../src/settlement_worker.rs");
const RUNTIME_V2: &str = include_str!("../src/settlement_worker_runtime_v2.rs");
const MIGRATION_V19: &str = include_str!("../migrations/0019_online_settlement_quarantine_v1.sql");

fn normalized(source: &str) -> String {
    source.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn runtime_v2_owns_interruptible_admission_and_bounded_drain() {
    for marker in [
        "settlement_shutdown_signal_v2",
        "SignalKind::terminate",
        "stopped new capture and claim admission",
        "drain_remote_work_v2",
        "tokio::time::timeout",
        "in_flight.abort_all",
        "lease will expire",
    ] {
        assert!(
            RUNTIME_V2.contains(marker),
            "missing shutdown marker {marker}"
        );
    }
}

#[test]
fn unrelated_remote_work_can_run_concurrently() {
    for marker in [
        "TRNM_SETTLEMENT_MAX_IN_FLIGHT",
        "JoinSet::<Result<(), String>>::new",
        "while in_flight.len() < max_in_flight",
        "in_flight.spawn",
        "process_claimed_job",
    ] {
        assert!(
            RUNTIME_V2.contains(marker),
            "missing concurrency marker {marker}"
        );
    }
}

#[test]
fn poison_work_is_quarantined_without_reusing_a_lost_lease() {
    for marker in [
        "capture_pending_matches_isolated_v2",
        "claim_settlement_job_isolated_v2",
        "apply_ready_captures_isolated_v2",
        "trnm_online_quarantine_claimed_settlement_job_v1",
        "lease_generation",
        "trnm_online_record_settlement_quarantine_v1",
        "trnm_online_settlement_scope_quarantined_v1",
    ] {
        assert!(
            RUNTIME_V2.contains(marker),
            "missing quarantine marker {marker}"
        );
    }
}

#[test]
fn migration_enforces_one_campaign_job_per_capture_and_audited_resolution() {
    let sql = normalized(MIGRATION_V19);
    for marker in [
        "unique index if not exists idx_trnm_online_settlement_job_one_campaign_per_capture_v1",
        "on public.trnm_online_settlement_jobs(capture_id, campaign_id)",
        "create table if not exists public.trnm_online_settlement_quarantine_v1",
        "create or replace function public.trnm_online_record_settlement_quarantine_v1",
        "create or replace function public.trnm_online_quarantine_claimed_settlement_job_v1",
        "create or replace function public.trnm_online_resolve_settlement_quarantine_v1",
        "revoke all on function public.trnm_online_resolve_settlement_quarantine_v1",
        "create or replace view public.trnm_online_settlement_quarantine_status_v1",
    ] {
        assert!(sql.contains(marker), "missing migration marker {marker}");
    }
}

#[test]
fn direct_sources_register_migration_19_and_disable_the_old_loop() {
    let migration_marker = "0019_online_settlement_quarantine_v1.sql";
    assert!(
        RUNTIME_V2.contains(migration_marker),
        "missing direct runtime marker {migration_marker}"
    );
    for marker in ["bounded_error_body", "StatusCode::CONFLICT"] {
        assert!(
            CEX_WRAPPER.contains(marker),
            "missing direct CEX marker {marker}"
        );
    }
    assert!(WORKER_WRAPPER.contains("settlement_worker_runtime_v2.rs"));
    assert!(WORKER_WRAPPER.contains("pub use implementation::{run_v2 as run, WorkerConfig};"));
    assert!(!WORKER_WRAPPER.contains("run_legacy as run"));
    assert!(!WORKER_WRAPPER.contains("OUT_DIR"));
    assert!(!CEX_WRAPPER.contains("OUT_DIR"));
    assert!(!CEX_WRAPPER.contains("trnm_cex_generated.rs"));
}

#[test]
fn settlement_cex_transport_never_uses_blocking_http() {
    assert!(!CEX_WRAPPER.contains("reqwest::blocking"));
    assert!(!CEX_WRAPPER.contains("blocking_client"));
}
