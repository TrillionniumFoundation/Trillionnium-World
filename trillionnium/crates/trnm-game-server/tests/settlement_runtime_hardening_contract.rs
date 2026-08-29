const BUILD_SCRIPT: &str = include_str!("../build.rs");
const MIGRATION_V19: &str =
    include_str!("../migrations/0019_online_settlement_runtime_hardening_v1.sql");
const GAME_CI: &str = include_str!("../../../../.github/workflows/trnm-game-ci.yml");

fn normalized(source: &str) -> String {
    source.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn migration_19_is_registered_by_both_runtime_entrypoints() {
    for marker in [
        "0019_online_settlement_runtime_hardening_v1.sql",
        "0019_online_settlement_runtime_hardening_v1",
        "MIGRATION_V19",
    ] {
        assert!(BUILD_SCRIPT.contains(marker), "missing migration marker {marker}");
    }
}

#[test]
fn runtime_lifecycle_handles_sigterm_and_drains_bounded_inflight_work() {
    for marker in [
        "SignalKind::terminate()",
        "settlement worker stopped after bounded in-flight drain",
        "tokio::task::JoinSet::new()",
        "claimed settlement job failed in isolation",
        "settlement task join failed; durable lease recovery remains active",
    ] {
        assert!(BUILD_SCRIPT.contains(marker), "missing lifecycle control {marker}");
    }
}

#[test]
fn poison_capture_and_apply_work_are_isolated_and_auditable() {
    let sql = normalized(MIGRATION_V19);
    for marker in [
        "create table if not exists public.trnm_online_settlement_runtime_failures",
        "subject_kind in ('capture', 'apply')",
        "consecutive_failures between 1 and 1000000",
        "create or replace function public.trnm_online_release_settlement_quarantine_v1",
        "settlement operator evidence is append-only",
        "create or replace view public.trnm_online_settlement_runtime_failure_metrics_v1",
    ] {
        assert!(sql.contains(marker), "missing poison-isolation contract {marker}");
    }
    for marker in [
        "record_runtime_failure",
        "clear_runtime_failure",
        "settlement capture candidate failed in isolation",
        "settlement apply failed in isolation",
    ] {
        assert!(BUILD_SCRIPT.contains(marker), "missing runtime isolation marker {marker}");
    }
}

#[test]
fn one_capture_has_at_most_one_job_per_campaign() {
    let sql = normalized(MIGRATION_V19);
    assert!(sql.contains(
        "create unique index if not exists idx_trnm_online_settlement_job_capture_campaign_v1 on public.trnm_online_settlement_jobs(capture_id, campaign_id) where capture_id is not null"
    ));
    assert!(BUILD_SCRIPT.contains("jobs_by_campaign.len() != jobs.len()"));
    assert!(BUILD_SCRIPT.contains("capture contains multiple settlement jobs for one campaign"));
}

#[test]
fn malformed_success_and_conflict_are_recovered_as_ambiguous() {
    for marker in [
        "normalize_external_failure",
        "decode isolated signer response:",
        "decode CEX receipt:",
        "(409 Conflict)",
        "exact receipt lookup is required before retry",
    ] {
        assert!(BUILD_SCRIPT.contains(marker), "missing ambiguity recovery marker {marker}");
    }
}

#[test]
fn revision_overflow_fails_closed() {
    assert!(BUILD_SCRIPT.contains("campaign.campaign.revision.checked_add(1)"));
    assert!(BUILD_SCRIPT.contains("revision exhausted"));
    assert!(!BUILD_SCRIPT.contains(
        "campaign.campaign.revision = campaign.campaign.revision.saturating_add(1)"
    ));
}

#[test]
fn exact_ci_runs_the_hardening_contract_without_source_write_permission() {
    assert!(GAME_CI.contains("trnm-game-ci/settlement-postgres"));
    assert!(GAME_CI.contains("cargo test --workspace --all-targets --locked"));
    assert!(GAME_CI.contains("contents: read"));
    assert!(!GAME_CI.contains("contents: write"));
    assert!(!GAME_CI.contains("cargo clippy --fix"));
    assert!(!GAME_CI.contains("git push"));
}
