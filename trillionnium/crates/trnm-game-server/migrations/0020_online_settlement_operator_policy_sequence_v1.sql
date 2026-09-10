-- Synchronize the settlement operator policy identity sequence after the
-- migration-default row was inserted with an explicit policy_revision.
--
-- PostgreSQL does not advance an identity sequence for an explicit value.
-- Without this repair the first appended policy revision asks nextval() for 1
-- and collides with the immutable seed row. This migration grants no
-- production authorization.

do $block$
declare
    sequence_name text;
    max_revision bigint;
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

    select pg_catalog.max(policy_revision)
      into max_revision
      from public.trnm_online_settlement_operator_policy_revisions;
    if max_revision is null or max_revision < 1 then
        raise exception using
            errcode = '55000',
            message = 'settlement operator policy seed revision is missing';
    end if;

    perform pg_catalog.setval(
        sequence_name::pg_catalog.regclass,
        max_revision,
        true
    );
end
$block$;
