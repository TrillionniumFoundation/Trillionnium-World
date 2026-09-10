#!/usr/bin/env python3
"""Apply the exact operator-policy repairs to a current World candidate clone."""

from __future__ import annotations

from pathlib import Path
import sys

POLICY_SEED_ANCHOR = '''on conflict (policy_revision) do nothing;

create table if not exists public.trnm_online_settlement_operator_replay_requests (
'''
POLICY_SEQUENCE_SYNC = '''on conflict (policy_revision) do nothing;

-- Explicitly seeding an identity column does not consume its backing sequence.
-- Synchronize only forward and preserve an already advanced sequence so the
-- first append after migration receives revision 2 and repeated migrations can
-- never rewind a production sequence.
do $trnm_online_settlement_operator_policy_identity_v1$
declare
    sequence_name text;
    sequence_last bigint;
    sequence_called boolean;
    maximum_revision bigint;
begin
    sequence_name := pg_catalog.pg_get_serial_sequence(
        'public.trnm_online_settlement_operator_policy_revisions',
        'policy_revision'
    );
    if sequence_name is null then
        raise exception using
            errcode = '55000',
            message = 'settlement operator policy identity sequence is missing';
    end if;

    select coalesce(max(policy_revision), 0)
      into maximum_revision
      from public.trnm_online_settlement_operator_policy_revisions;
    execute pg_catalog.format('select last_value, is_called from %s', sequence_name)
       into sequence_last, sequence_called;

    if maximum_revision > 0
       and (
           sequence_last < maximum_revision
           or (sequence_last = maximum_revision and not sequence_called)
       ) then
        perform pg_catalog.setval(
            sequence_name::pg_catalog.regclass,
            maximum_revision,
            true
        );
    end if;
end;
$trnm_online_settlement_operator_policy_identity_v1$;

create table if not exists public.trnm_online_settlement_operator_replay_requests (
'''

OPERATOR_APPLY = '''    pool.execute(OPERATOR_MIGRATION)
        .await
        .expect("apply operator migration");
'''
OPERATOR_REAPPLY = OPERATOR_APPLY + '''    pool.execute(OPERATOR_MIGRATION)
        .await
        .expect("reapply operator migration idempotently");
'''

MUTATION_LOOP = '''    for statement in [
        "update public.trnm_online_settlement_operator_replay_requests set reason = 'tampered evidence'",
        "delete from public.trnm_online_settlement_operator_replay_requests",
        "truncate public.trnm_online_settlement_operator_replay_requests",
        "update public.trnm_online_settlement_operator_policy_revisions set reason = 'tampered policy'",
        "delete from public.trnm_online_settlement_operator_policy_revisions",
        "truncate public.trnm_online_settlement_operator_policy_revisions",
    ] {
        let error = sqlx::query::query(statement)
            .execute(&pool)
            .await
            .unwrap_err();
        assert_sqlstate(error, "55000");
    }
'''
MUTATION_FENCES = '''    for statement in [
        "update public.trnm_online_settlement_operator_replay_requests set reason = 'tampered evidence'",
        "delete from public.trnm_online_settlement_operator_replay_requests",
        "truncate public.trnm_online_settlement_operator_replay_requests",
        "update public.trnm_online_settlement_operator_policy_revisions set reason = 'tampered policy'",
        "delete from public.trnm_online_settlement_operator_policy_revisions",
    ] {
        let error = sqlx::query::query(statement)
            .execute(&pool)
            .await
            .unwrap_err();
        assert_sqlstate(error, "55000");
    }

    // PostgreSQL rejects a parent-only TRUNCATE before relation triggers because
    // replay evidence holds an exact foreign key to the policy revision. That
    // native fence is expected and distinct from the append-only trigger fence.
    let native_fk_fence = sqlx::query::query(
        "truncate public.trnm_online_settlement_operator_policy_revisions",
    )
    .execute(&pool)
    .await
    .unwrap_err();
    assert_sqlstate(native_fk_fence, "0A000");

    // Including dependent rows reaches the reviewed BEFORE TRUNCATE trigger and
    // must still fail with the component-owned append-only SQLSTATE.
    let append_only_fence = sqlx::query::query(
        "truncate public.trnm_online_settlement_operator_policy_revisions cascade",
    )
    .execute(&pool)
    .await
    .unwrap_err();
    assert_sqlstate(append_only_fence, "55000");
'''


def replace_once(path: Path, old: str, new: str, label: str) -> None:
    text = path.read_text(encoding="utf-8")
    if text.count(old) != 1:
        raise SystemExit(f"{label} anchor did not match exactly once in {path}")
    path.write_text(text.replace(old, new), encoding="utf-8", newline="\n")


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: patch-world-operator-only-c3cdd.py ROOT", file=sys.stderr)
        return 64
    root = Path(sys.argv[1]).resolve()
    replace_once(
        root / "trillionnium/crates/trnm-game-server/migrations/0018_online_settlement_operator_controls_v1.sql",
        POLICY_SEED_ANCHOR,
        POLICY_SEQUENCE_SYNC,
        "operator policy identity sequence synchronization",
    )
    test = root / "trillionnium/crates/trnm-game-server/tests/settlement_operator_controls_database.rs"
    replace_once(
        test,
        OPERATOR_APPLY,
        OPERATOR_REAPPLY,
        "operator migration idempotence regression",
    )
    replace_once(
        test,
        MUTATION_LOOP,
        MUTATION_FENCES,
        "operator mutation fence SQLSTATE regression",
    )
    print("WORLD_OPERATOR_ONLY_PATCH=PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
