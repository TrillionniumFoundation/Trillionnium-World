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
fn remote_request_identity_is_stable_across_capture_generations() {
    let sql = normalized(WORKER_MIGRATION);
    let identity_start = sql
        .find("set remote_request_id = 'trnm-settlement-remote-v1:'")
        .expect("remote request identity backfill must exist");
    let identity_end = sql[identity_start..]
        .find("where remote_request_id is null")
        .map(|offset| identity_start + offset)
        .expect("remote request identity backfill must be bounded");
    let identity = &sql[identity_start..identity_end];

    assert!(identity.contains("match_id::text"));
    assert!(identity.contains("md5(campaign_id)"));
    assert!(identity.contains("intent_hash"));
    assert!(!identity.contains("capture_id"));
    assert!(!identity.contains("capture_generation"));

    assert!(sql.contains(
        "entitlement_nonce = coalesce(job.entitlement_nonce, job.remote_request_id)"
    ));
    assert!(sql.contains(
        "authorization_request_id = coalesce( job.authorization_request_id, job.remote_request_id )"
    ));
    assert!(sql.contains("p_authorization_request_id = remote_request_id"));
    assert!(!sql.contains(
        "authorization_request_id = coalesce(job.authorization_request_id, job.job_id)"
    ));
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
