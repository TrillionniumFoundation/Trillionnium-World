const OUTBOX_MIGRATION: &str =
    include_str!("../migrations/0016_online_settlement_outbox_v1.sql");
const WORKER_MIGRATION: &str =
    include_str!("../migrations/0017_online_settlement_worker_runtime_v1.sql");
const WORKER_SOURCE: &str = include_str!("../src/settlement_worker.rs");
const CEX_SOURCE: &str = include_str!("../src/cex.rs");
const WORKER_BINARY: &str = include_str!("../src/bin/trnm-settlement-worker.rs");

fn normalized(source: &str) -> String {
    source.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn capture_claim_and_apply_are_persisted_as_separate_contracts() {
    let sql = normalized(&format!("{OUTBOX_MIGRATION}\n{WORKER_MIGRATION}"));
    for required in [
        "create table if not exists public.trnm_online_settlement_captures",
        "expected_campaign_revision bigint not null",
        "expected_campaign_state_hash text",
        "terminal_identity_hash text not null",
        "campaign_fences_json jsonb not null",
        "head_intent_ids_json jsonb not null",
        "create or replace function public.trnm_online_claim_settlement_job_v2",
        "for update of job skip locked",
        "create or replace function public.trnm_online_complete_settlement_job_v1",
        "campaign_applied_at timestamptz",
    ] {
        assert!(sql.contains(required), "missing settlement contract: {required}");
    }
}

#[test]
fn remote_retries_reuse_stable_authorization_material() {
    let sql = normalized(WORKER_MIGRATION);
    for required in [
        "authorization_request_id = coalesce(job.authorization_request_id, job.job_id)",
        "entitlement_issued_at_epoch = coalesce",
        "entitlement_expires_at_epoch = coalesce",
        "entitlement_nonce = coalesce(job.entitlement_nonce, job.job_id)",
        "authorized_intent_json = p_authorized_intent_json",
        "signer_receipt_hash = p_signer_receipt_hash",
        "remote_attempts = remote_attempts + 1",
    ] {
        assert!(sql.contains(required), "missing stable retry material: {required}");
    }
    assert!(CEX_SOURCE.contains("stable_entitlement_id(authorization_request_id)"));
    assert!(CEX_SOURCE.contains("signed.request_id != authorization_request_id"));
    assert!(CEX_SOURCE.contains("request_hash != signed.request_hash"));
}

#[test]
fn stale_workers_cannot_complete_or_retry_another_lease() {
    let sql = normalized(WORKER_MIGRATION);
    let lease_fence = "state = 'leased' and lease_owner = p_owner and lease_generation = p_lease_generation";
    assert!(
        sql.matches(lease_fence).count() >= 4,
        "completion/retry/dead-letter/authorization must all share the lease fence"
    );
    assert!(sql.contains("lease_expires_at > pg_catalog.clock_timestamp()"));
}

#[test]
fn both_campaign_fences_are_revalidated_before_any_apply_commit() {
    for required in [
        "campaign_fences_json(&campaigns) != expected_campaign_fences",
        "terminal identity hash changed after capture",
        "campaign_revision = $6",
        "state_hash = $7",
        "failed exact revision/state-hash CAS",
        "finalize_match_in_transaction(&mut transaction, match_id).await",
    ] {
        assert!(
            WORKER_SOURCE.contains(required),
            "worker lost exact apply invariant: {required}"
        );
    }
}

#[test]
fn external_requests_are_only_in_the_execute_phase() {
    let capture_start = WORKER_SOURCE.find("async fn capture_match").unwrap();
    let capture_end = WORKER_SOURCE[capture_start..]
        .find("async fn load_terminal_identity")
        .map(|offset| capture_start + offset)
        .unwrap();
    let capture = &WORKER_SOURCE[capture_start..capture_end];
    assert!(!capture.contains("authorize_settlement_intent"));
    assert!(!capture.contains("submit_authorized_settlement_intent"));

    let apply_start = WORKER_SOURCE.find("async fn apply_capture").unwrap();
    let apply_end = WORKER_SOURCE[apply_start..]
        .find("struct CaptureJobRow")
        .map(|offset| apply_start + offset)
        .unwrap();
    let apply = &WORKER_SOURCE[apply_start..apply_end];
    assert!(!apply.contains("authorize_settlement_intent"));
    assert!(!apply.contains("submit_authorized_settlement_intent"));

    let execute_start = WORKER_SOURCE.find("async fn process_claimed_job").unwrap();
    let execute_end = WORKER_SOURCE[execute_start..]
        .find("async fn handle_external_failure")
        .map(|offset| execute_start + offset)
        .unwrap();
    let execute = &WORKER_SOURCE[execute_start..execute_end];
    assert!(!execute.contains(".begin()"));
    assert!(execute.contains("authorize_settlement_intent"));
    assert!(execute.contains("submit_authorized_settlement_intent"));
}

#[test]
fn synchronous_game_server_backend_remains_fail_closed() {
    assert!(CEX_SOURCE.contains("SETTLEMENT_OUTBOX_REQUIRED"));
    assert!(CEX_SOURCE.contains("Err(SETTLEMENT_OUTBOX_REQUIRED.to_string())"));
    assert!(!CEX_SOURCE.contains("blocking_client"));
    assert!(!CEX_SOURCE.contains("reqwest::blocking"));
    assert!(WORKER_BINARY.contains("settlement_worker::run(config).await"));
}
