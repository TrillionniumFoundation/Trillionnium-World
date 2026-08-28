//! Static P0 contract coverage for remote settlement identity, lease fencing,
//! evidence retention, and operator-visible two-phase completion semantics.

const OUTBOX_MIGRATION: &str =
    include_str!("../migrations/0016_online_settlement_outbox_v1.sql");
const WORKER_MIGRATION: &str =
    include_str!("../migrations/0017_online_settlement_worker_runtime_v1.sql");

fn normalized(source: &str) -> String {
    source.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn function_body<'a>(source: &'a str, function_name: &str, next_marker: &str) -> &'a str {
    let start_marker = format!("create or replace function public.{function_name}");
    let start = source
        .find(&start_marker)
        .unwrap_or_else(|| panic!("missing function {function_name}"));
    let relative_end = source[start..]
        .find(next_marker)
        .unwrap_or_else(|| panic!("missing end marker for {function_name}"));
    &source[start..start + relative_end]
}

#[test]
fn settlement_evidence_is_not_deleted_by_upstream_cascade() {
    let sql = normalized(OUTBOX_MIGRATION);
    assert!(sql.contains(
        "references public.trnm_online_matches(match_id) on delete restrict"
    ));
    assert!(sql.contains(
        "references public.trnm_online_campaigns(campaign_id) on delete restrict"
    ));
    assert!(!sql.contains(
        "references public.trnm_online_matches(match_id) on delete cascade"
    ));
    assert!(!sql.contains(
        "references public.trnm_online_campaigns(campaign_id) on delete cascade"
    ));
}

#[test]
fn remote_request_identity_is_generated_and_stable_across_capture_generations() {
    let sql = normalized(WORKER_MIGRATION);
    let identity_start = sql
        .find("add column if not exists remote_request_id text generated always as")
        .expect("remote request identity generated column must exist");
    let identity_end = sql[identity_start..]
        .find("alter table public.trnm_online_settlement_jobs add column if not exists authorization_request_id")
        .map(|offset| identity_start + offset)
        .expect("remote request identity definition must be bounded");
    let identity = &sql[identity_start..identity_end];

    assert!(identity.contains("pg_catalog.sha256("));
    assert!(identity.contains("pg_catalog.encode("));
    assert!(identity.contains("pg_catalog.convert_to("));
    assert!(identity.matches("pg_catalog.octet_length(").count() >= 4);
    assert!(identity.contains("match_id::text"));
    assert!(identity.contains("campaign_id"));
    assert!(identity.contains("intent_id"));
    assert!(!identity.contains("capture_id"));
    assert!(!identity.contains("capture_generation"));
    assert!(!identity.contains("intent_hash"));
    assert!(!identity.contains("md5("));

    assert!(sql.contains(
        "entitlement_nonce = coalesce(job.entitlement_nonce, job.remote_request_id)"
    ));
    assert!(sql.contains("authorization_request_id = coalesce("));
    assert!(sql.contains("job.authorization_request_id, job.remote_request_id"));
    assert!(sql.contains("p_authorization_request_id = remote_request_id"));
    assert!(!sql.contains(
        "authorization_request_id = coalesce(job.authorization_request_id, job.job_id)"
    ));
}

#[test]
fn legacy_v1_claim_path_is_fail_closed() {
    let sql = normalized(WORKER_MIGRATION);
    let legacy = function_body(
        WORKER_MIGRATION,
        "trnm_online_claim_settlement_job_v1",
        "create or replace function public.trnm_online_claim_settlement_job_v2",
    );
    let legacy = normalized(legacy);

    assert!(legacy.contains("errcode = '0A000'"));
    assert!(legacy.contains(
        "trnm_online_claim_settlement_job_v1 is retired; use v2"
    ));
    assert!(!legacy.contains("for update skip locked"));
    assert!(sql.contains("create or replace function public.trnm_online_claim_settlement_job_v2"));
}

#[test]
fn every_remote_mutation_requires_a_live_lease() {
    let authorization = function_body(
        WORKER_MIGRATION,
        "trnm_online_store_settlement_authorization_v1",
        "create or replace function public.trnm_online_begin_settlement_remote_attempt_v1",
    );
    let begin_attempt = function_body(
        WORKER_MIGRATION,
        "trnm_online_begin_settlement_remote_attempt_v1",
        "create or replace function public.trnm_online_complete_settlement_job_v1",
    );
    let complete = function_body(
        WORKER_MIGRATION,
        "trnm_online_complete_settlement_job_v1",
        "create or replace function public.trnm_online_retry_settlement_job_v1",
    );
    let retry = function_body(
        WORKER_MIGRATION,
        "trnm_online_retry_settlement_job_v1",
        "create or replace function public.trnm_online_dead_letter_settlement_job_v1",
    );
    let dead_letter = function_body(
        WORKER_MIGRATION,
        "trnm_online_dead_letter_settlement_job_v1",
        "create or replace view public.trnm_online_settlement_job_status_v1",
    );

    for (name, body) in [
        ("authorization", authorization),
        ("begin_attempt", begin_attempt),
        ("complete", complete),
        ("retry", retry),
        ("dead_letter", dead_letter),
    ] {
        let body = normalized(body);
        assert!(body.contains("state = 'leased'"), "{name} lost state fence");
        assert!(body.contains("lease_owner = p_owner"), "{name} lost owner fence");
        assert!(
            body.contains("lease_generation = p_lease_generation"),
            "{name} lost generation fence"
        );
        assert!(
            body.contains("lease_expires_at > pg_catalog.clock_timestamp()"),
            "{name} permits an expired worker to mutate durable state"
        );
    }
}

#[test]
fn remote_success_and_campaign_application_are_never_aliased() {
    let sql = normalized(WORKER_MIGRATION);
    assert!(sql.contains(
        "create or replace view public.trnm_online_settlement_job_status_v1"
    ));
    assert!(sql.contains("when 'succeeded' then 'remote_succeeded'"));
    assert!(sql.contains("when job.state = 'succeeded' then 'pending_apply'"));
    assert!(sql.contains(
        "when job.campaign_applied_at is not null then 'applied'"
    ));
    assert!(sql.contains("when job.state = 'dead_letter' then 'blocked'"));
}
