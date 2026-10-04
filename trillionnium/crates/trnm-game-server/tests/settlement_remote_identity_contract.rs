//! Static P0 contract coverage for remote settlement identity, lease fencing,
//! account/campaign serialization, evidence retention, and operator-visible
//! two-phase completion semantics.

const OUTBOX_MIGRATION: &str = include_str!("../migrations/0016_online_settlement_outbox_v1.sql");
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
    assert!(sql.contains("references public.trnm_online_matches(match_id) on delete restrict"));
    assert!(sql.contains("references public.trnm_online_campaigns(campaign_id) on delete restrict"));
    assert!(!sql.contains("references public.trnm_online_matches(match_id) on delete cascade"));
    assert!(!sql.contains("references public.trnm_online_campaigns(campaign_id) on delete cascade"));
}

#[test]
fn remote_request_identity_is_database_derived_and_capture_independent() {
    let sql = normalized(WORKER_MIGRATION);
    let identity = normalized(function_body(
        WORKER_MIGRATION,
        "trnm_online_remote_request_id_v1",
        "create or replace function public.trnm_online_set_remote_request_id_v1",
    ));

    assert!(sql.contains("add column if not exists remote_request_id text"));
    assert!(sql.contains("alter column remote_request_id set not null"));
    assert!(sql.contains("remote_request_id must be an ordinary stored column"));
    assert!(sql.contains("set remote_request_id = public.trnm_online_remote_request_id_v1("));

    assert!(identity.contains("pg_catalog.sha256("));
    assert!(identity.contains("pg_catalog.encode("));
    assert!(identity.contains("pg_catalog.convert_to("));
    assert!(identity.matches("pg_catalog.octet_length(").count() >= 4);
    assert!(identity.contains("p_match_id::text"));
    assert!(identity.contains("p_campaign_id"));
    assert!(identity.contains("p_intent_id"));
    assert!(!identity.contains("capture_id"));
    assert!(!identity.contains("capture_generation"));
    assert!(!identity.contains("intent_hash"));
    assert!(!identity.contains("md5("));

    assert!(
        sql.contains("entitlement_nonce = coalesce(job.entitlement_nonce, job.remote_request_id)")
    );
    assert!(sql.contains("authorization_request_id = coalesce("));
    assert!(sql.contains("job.authorization_request_id, job.remote_request_id"));
    assert!(sql.contains("p_authorization_request_id = remote_request_id"));
    assert!(!sql
        .contains("authorization_request_id = coalesce(job.authorization_request_id, job.job_id)"));
}

#[test]
fn settlement_identity_fields_and_remote_aliases_are_immutable() {
    let sql = normalized(WORKER_MIGRATION);
    let trigger = normalized(function_body(
        WORKER_MIGRATION,
        "trnm_online_set_remote_request_id_v1",
        "drop trigger if exists trnm_online_settlement_remote_id_insert_v1",
    ));

    assert!(trigger.contains("tg_op = 'UPDATE'"));
    assert!(trigger.contains("new.match_id is distinct from old.match_id"));
    assert!(trigger.contains("new.campaign_id is distinct from old.campaign_id"));
    assert!(trigger.contains("new.intent_id is distinct from old.intent_id"));
    assert!(trigger.contains(
        "message = 'settlement match, campaign and intent identity fields are immutable'"
    ));
    assert!(trigger.contains("expected := public.trnm_online_remote_request_id_v1("));
    assert!(trigger.contains("new.remote_request_id <> expected"));
    assert!(trigger.matches("errcode = '23514'").count() >= 2);
    assert!(trigger
        .contains("message = 'remote_request_id does not match durable settlement identity'"));
    assert!(trigger.contains("new.remote_request_id := expected"));

    assert!(sql.contains("create trigger trnm_online_settlement_remote_id_insert_v1 before insert"));
    assert!(sql.contains(
        "create trigger trnm_online_settlement_remote_id_update_v1 before update of match_id, campaign_id, intent_id, remote_request_id"
    ));
    assert!(
        sql.matches("execute function public.trnm_online_set_remote_request_id_v1()")
            .count()
            >= 2
    );
    assert!(sql.contains(
        "authorization_request_id is null or authorization_request_id = remote_request_id"
    ));
    assert!(sql.contains("entitlement_nonce is null or entitlement_nonce = remote_request_id"));
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
    assert!(legacy.contains("trnm_online_claim_settlement_job_v1 is retired; use v2"));
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
        assert!(
            body.contains("lease_owner = p_owner"),
            "{name} lost owner fence"
        );
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
fn account_or_campaign_serialization_is_explicit_and_bounded() {
    let serialization = normalized(function_body(
        WORKER_MIGRATION,
        "trnm_online_settlement_serialization_key_v1",
        "create index if not exists idx_trnm_online_settlement_job_serialization",
    ));
    let claim = normalized(function_body(
        WORKER_MIGRATION,
        "trnm_online_claim_settlement_job_v2",
        "create or replace function public.trnm_online_store_settlement_authorization_v1",
    ));

    assert!(serialization.contains("p_intent_json #>> '{actors,0,account_id}'"));
    assert!(serialization.contains("'campaign:' || p_campaign_id"));
    assert!(claim.contains("pg_catalog.pg_try_advisory_xact_lock("));
    assert!(claim.contains("pg_catalog.hashtextextended("));
    assert!(claim.contains("blocker.state = 'succeeded'"));
    assert!(claim.contains("blocker.state = 'leased'"));
    assert!(claim.contains("blocker.lease_expires_at > pg_catalog.clock_timestamp()"));
    assert!(claim.contains("for update of job skip locked"));
    assert!(claim.contains("limit 16"));
}

#[test]
fn remote_success_and_campaign_application_are_never_aliased() {
    let sql = normalized(WORKER_MIGRATION);
    assert!(sql.contains("create or replace view public.trnm_online_settlement_job_status_v1"));
    assert!(sql.contains("when 'succeeded' then 'remote_succeeded'"));
    assert!(sql.contains("when job.state = 'succeeded' then 'pending_apply'"));
    assert!(sql.contains("when job.campaign_applied_at is not null then 'applied'"));
    assert!(sql.contains("when job.state = 'dead_letter' then 'blocked'"));
}

#[test]
fn operator_metrics_expose_backlog_age_and_pending_apply() {
    let sql = normalized(WORKER_MIGRATION);
    for required in [
        "create or replace view public.trnm_online_settlement_metrics_v1",
        "remote_pending",
        "remote_leased",
        "remote_retryable",
        "remote_succeeded",
        "remote_dead_letter",
        "pending_apply",
        "expired_leases",
        "oldest_eligible_age",
        "oldest_pending_apply_age",
        "maximum_remote_attempts",
    ] {
        assert!(
            sql.contains(required),
            "missing operator metric: {required}"
        );
    }
}
