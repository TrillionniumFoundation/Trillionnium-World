-- Mandatory hardening for trnm-world-authority-cutover-v1.sql.
--
-- The base migration is not a supported standalone deployment. The authoritative
-- installer applies this file immediately after the base migration and refuses
-- to continue unless both exact migration markers are present. This file does
-- not grant runtime roles or production authorization.

begin;

DO $guard$
declare
    observed_version text;
begin
    select contract_version
    into observed_version
    from public.trnm_world_authority_schema_migrations_v1
    where migration_id = '0001_world_authority_cutover_v1';

    if not found then
        raise exception using
            errcode = '55000',
            message = 'trnm_world_base_migration_marker_missing';
    end if;
    if observed_version is distinct from 'trillionnium_world_durable_cutover_v1' then
        raise exception using
            errcode = '23505',
            message = 'trnm_world_base_migration_version_conflict',
            detail = pg_catalog.format(
                'expected=%s observed=%s',
                'trillionnium_world_durable_cutover_v1',
                coalesce(observed_version, '<null>')
            );
    end if;
end;
$guard$;

-- A constant-key partial unique index is the database-level final guard: at
-- most one epoch can be both active and write-enabled, including concurrent
-- transactions that target different epoch rows.
create unique index if not exists trnm_world_single_active_writer_epoch_v1
    on public.trnm_world_authority_epochs_v1 ((1))
    where status = 'active' and world_writer_enabled;

create or replace function public.trnm_world_event_id_v1(
    target_epoch_id text,
    target_world_id text,
    target_idempotency_key text,
    target_request_sha256 text
)
returns text
language sql
immutable
strict
set search_path = pg_catalog
as $function$
    select 'sha256:' || pg_catalog.encode(
        pg_catalog.sha256(
            pg_catalog.convert_to(
                target_epoch_id || E'\n'
                || target_world_id || E'\n'
                || target_idempotency_key || E'\n'
                || target_request_sha256,
                'UTF8'
            )
        ),
        'hex'
    )
$function$;

-- Canonical text is exactly PostgreSQL jsonb text serialization. This removes
-- whitespace and key-order aliases before any hash is accepted.
alter table public.trnm_world_states_v1
    drop constraint if exists trnm_world_state_canonical_json_parity_v1;
alter table public.trnm_world_states_v1
    drop constraint if exists trnm_world_state_canonical_hash_v1;
alter table public.trnm_world_states_v1
    add constraint trnm_world_state_canonical_json_parity_v1 check (
        state_canonical = state_json::text
    );
alter table public.trnm_world_states_v1
    add constraint trnm_world_state_canonical_hash_v1 check (
        state_sha256 = 'sha256:' || pg_catalog.encode(
            pg_catalog.sha256(pg_catalog.convert_to(state_json::text, 'UTF8')),
            'hex'
        )
    );

alter table public.trnm_world_events_v1
    drop constraint if exists trnm_world_event_command_parity_v1;
alter table public.trnm_world_events_v1
    drop constraint if exists trnm_world_event_response_parity_v1;
alter table public.trnm_world_events_v1
    drop constraint if exists trnm_world_event_response_hash_v1;
alter table public.trnm_world_events_v1
    add constraint trnm_world_event_command_parity_v1 check (
        command_canonical = command_json::text
    );
alter table public.trnm_world_events_v1
    add constraint trnm_world_event_request_hash_v1 check (
        request_sha256 = 'sha256:' || pg_catalog.encode(
            pg_catalog.sha256(pg_catalog.convert_to(command_json::text, 'UTF8')),
            'hex'
        )
    );
alter table public.trnm_world_events_v1
    add constraint trnm_world_event_identity_v1 check (
        event_id = public.trnm_world_event_id_v1(
            epoch_id,
            world_id,
            idempotency_key,
            request_sha256
        )
    );
alter table public.trnm_world_events_v1
    add constraint trnm_world_event_response_parity_v1 check (
        response_canonical = response_json::text
    );
alter table public.trnm_world_events_v1
    add constraint trnm_world_event_response_hash_v1 check (
        response_sha256 = 'sha256:' || pg_catalog.encode(
            pg_catalog.sha256(pg_catalog.convert_to(response_json::text, 'UTF8')),
            'hex'
        )
    );

create or replace function public.trnm_world_import_snapshot_v1(
    target_batch_id text,
    target_world_id text,
    target_state_contract text,
    target_source_revision text,
    target_state_canonical text,
    target_state_sha256 text
)
returns text
language plpgsql
security definer
set search_path = pg_catalog, public
as $function$
declare
    batch public.trnm_world_migration_batches_v1%rowtype;
    existing public.trnm_world_states_v1%rowtype;
    computed_hash text;
    parsed_state jsonb;
    canonical_state text;
begin
    parsed_state := target_state_canonical::jsonb;
    canonical_state := parsed_state::text;
    if target_state_canonical is distinct from canonical_state then
        raise exception using
            errcode = '22023',
            message = 'trnm_world_import_state_json_not_canonical';
    end if;

    computed_hash := public.trnm_world_sha256_text_v1(canonical_state);
    if target_state_sha256 is distinct from computed_hash then
        raise exception using
            errcode = '23514',
            message = 'trnm_world_import_state_hash_mismatch';
    end if;
    if target_world_id is null or length(target_world_id) not between 1 and 256
       or target_state_contract is null or length(target_state_contract) not between 1 and 256
       or target_source_revision is null or length(target_source_revision) not between 1 and 256 then
        raise exception using
            errcode = '22023',
            message = 'trnm_world_import_snapshot_identity_invalid';
    end if;

    select * into batch
    from public.trnm_world_migration_batches_v1
    where batch_id = target_batch_id
    for update;
    if not found then
        raise exception using
            errcode = '23503',
            message = 'trnm_world_migration_batch_not_found';
    end if;
    if batch.status = 'reconciled' then
        select * into existing
        from public.trnm_world_states_v1
        where world_id = target_world_id;
        if found
           and existing.migration_batch_id = target_batch_id
           and existing.state_contract = target_state_contract
           and existing.source_revision = target_source_revision
           and existing.state_sha256 = target_state_sha256
           and existing.state_canonical = canonical_state
           and existing.state_json = parsed_state then
            return 'replayed_exact';
        end if;
        raise exception using
            errcode = '55000',
            message = 'trnm_world_reconciled_batch_is_immutable';
    end if;
    if batch.status <> 'staged' then
        raise exception using
            errcode = '55000',
            message = 'trnm_world_migration_batch_not_importable';
    end if;

    select * into existing
    from public.trnm_world_states_v1
    where world_id = target_world_id
    for update;
    if found then
        if existing.migration_batch_id = target_batch_id
           and existing.state_generation = 0
           and existing.state_contract = target_state_contract
           and existing.source_revision = target_source_revision
           and existing.state_sha256 = target_state_sha256
           and existing.state_canonical = canonical_state
           and existing.state_json = parsed_state then
            return 'replayed_exact';
        end if;
        raise exception using
            errcode = '23505',
            message = 'trnm_world_import_snapshot_conflict';
    end if;

    insert into public.trnm_world_states_v1 (
        world_id, state_generation, state_contract, state_sha256,
        state_canonical, state_json, source_revision,
        migration_batch_id, writer_epoch_id
    ) values (
        target_world_id, 0, target_state_contract, target_state_sha256,
        canonical_state, parsed_state, target_source_revision,
        target_batch_id, batch.epoch_id
    );
    return 'imported';
end;
$function$;

create or replace function public.trnm_world_activate_writer_v1(
    target_epoch_id text
)
returns text
language plpgsql
security definer
set search_path = pg_catalog, public
as $function$
declare
    epoch public.trnm_world_authority_epochs_v1%rowtype;
    batch public.trnm_world_migration_batches_v1%rowtype;
    receipt_id text;
begin
    -- Serialize activation across all epoch rows. The partial unique index is a
    -- second independent database constraint rather than the only protection.
    perform pg_catalog.pg_advisory_xact_lock(781400256302915541);

    select * into epoch
    from public.trnm_world_authority_epochs_v1
    where epoch_id = target_epoch_id
    for update;
    if not found then
        raise exception using
            errcode = '23503',
            message = 'trnm_world_authority_epoch_not_found';
    end if;

    if exists (
        select 1
        from public.trnm_world_authority_epochs_v1
        where status = 'active'
          and world_writer_enabled
          and epoch_id <> target_epoch_id
    ) then
        raise exception using
            errcode = '55000',
            message = 'trnm_world_different_active_writer_epoch_exists';
    end if;

    if epoch.status = 'active' then
        if not epoch.cex_writer_disabled or not epoch.world_writer_enabled then
            raise exception using
                errcode = '23514',
                message = 'trnm_world_active_epoch_shape_invalid';
        end if;
    elsif epoch.status = 'prepared' then
        if not epoch.cex_writer_disabled
           or epoch.cex_fence_receipt_sha256 is null
           or epoch.backfill_batch_id is null then
            raise exception using
                errcode = '55000',
                message = 'trnm_world_activation_requires_cex_fence_and_backfill';
        end if;
        select * into strict batch
        from public.trnm_world_migration_batches_v1
        where batch_id = epoch.backfill_batch_id;
        if batch.status <> 'reconciled' then
            raise exception using
                errcode = '55000',
                message = 'trnm_world_activation_requires_reconciled_backfill';
        end if;
        update public.trnm_world_authority_epochs_v1
        set status = 'active',
            world_writer_enabled = true,
            activated_at = transaction_timestamp()
        where epoch_id = target_epoch_id;
    else
        raise exception using
            errcode = '55000',
            message = 'trnm_world_authority_epoch_not_activatable';
    end if;

    receipt_id := public.trnm_world_append_receipt_v1(
        target_epoch_id,
        'world_writer_activated',
        pg_catalog.jsonb_build_object(
            'schema', 'trillionnium.world.cutover.world-writer-activated.v1',
            'epoch_id', target_epoch_id,
            'cex_writer_disabled', true,
            'world_writer_enabled', true
        )
    );
    return receipt_id;
end;
$function$;

create or replace function public.trnm_world_apply_event_v1(
    target_event_id text,
    target_epoch_id text,
    target_world_id text,
    target_idempotency_key text,
    target_request_sha256 text,
    target_command_canonical text,
    target_response_canonical text,
    target_response_sha256 text,
    target_expected_state_sha256 text,
    target_state_after_canonical text,
    target_state_after_sha256 text
)
returns jsonb
language plpgsql
security definer
set search_path = pg_catalog, public
as $function$
declare
    epoch public.trnm_world_authority_epochs_v1%rowtype;
    state public.trnm_world_states_v1%rowtype;
    existing public.trnm_world_events_v1%rowtype;
    parsed_command jsonb;
    parsed_response jsonb;
    parsed_state_after jsonb;
    canonical_command text;
    canonical_response text;
    canonical_state_after text;
    generated_request_hash text;
    generated_response_hash text;
    generated_state_hash text;
    generated_event_id text;
    next_generation bigint;
begin
    if target_epoch_id !~ '^sha256:[0-9a-f]{64}$'
       or target_world_id is null
       or length(target_world_id) not between 1 and 256
       or target_event_id !~ '^sha256:[0-9a-f]{64}$'
       or target_request_sha256 !~ '^sha256:[0-9a-f]{64}$'
       or target_response_sha256 !~ '^sha256:[0-9a-f]{64}$'
       or target_state_after_sha256 !~ '^sha256:[0-9a-f]{64}$'
       or target_expected_state_sha256 !~ '^sha256:[0-9a-f]{64}$'
       or target_idempotency_key is null
       or length(target_idempotency_key) not between 1 and 256 then
        raise exception using
            errcode = '22023',
            message = 'trnm_world_event_arguments_invalid';
    end if;

    parsed_command := target_command_canonical::jsonb;
    parsed_response := target_response_canonical::jsonb;
    parsed_state_after := target_state_after_canonical::jsonb;
    canonical_command := parsed_command::text;
    canonical_response := parsed_response::text;
    canonical_state_after := parsed_state_after::text;

    if target_command_canonical is distinct from canonical_command then
        raise exception using
            errcode = '22023',
            message = 'trnm_world_event_command_json_not_canonical';
    end if;
    if target_response_canonical is distinct from canonical_response then
        raise exception using
            errcode = '22023',
            message = 'trnm_world_event_response_json_not_canonical';
    end if;
    if target_state_after_canonical is distinct from canonical_state_after then
        raise exception using
            errcode = '22023',
            message = 'trnm_world_event_state_json_not_canonical';
    end if;

    generated_request_hash := public.trnm_world_sha256_text_v1(canonical_command);
    generated_response_hash := public.trnm_world_sha256_text_v1(canonical_response);
    generated_state_hash := public.trnm_world_sha256_text_v1(canonical_state_after);
    generated_event_id := public.trnm_world_event_id_v1(
        target_epoch_id,
        target_world_id,
        target_idempotency_key,
        generated_request_hash
    );

    if target_request_sha256 is distinct from generated_request_hash then
        raise exception using
            errcode = '23514',
            message = 'trnm_world_event_request_hash_mismatch';
    end if;
    if target_event_id is distinct from generated_event_id then
        raise exception using
            errcode = '23514',
            message = 'trnm_world_event_id_mismatch';
    end if;
    if target_response_sha256 is distinct from generated_response_hash then
        raise exception using
            errcode = '23514',
            message = 'trnm_world_event_response_hash_mismatch';
    end if;
    if target_state_after_sha256 is distinct from generated_state_hash then
        raise exception using
            errcode = '23514',
            message = 'trnm_world_event_state_hash_mismatch';
    end if;

    -- Serialize a logical idempotency key before reading its receipt. This makes
    -- the post-wait re-read authoritative for concurrent exact replays.
    perform pg_catalog.pg_advisory_xact_lock(
        pg_catalog.hashtextextended(
            target_world_id || E'\n' || target_idempotency_key,
            0
        )
    );

    select * into epoch
    from public.trnm_world_authority_epochs_v1
    where epoch_id = target_epoch_id
    for share;
    if not found
       or epoch.status <> 'active'
       or not epoch.cex_writer_disabled
       or not epoch.world_writer_enabled then
        raise exception using
            errcode = '55000',
            message = 'trnm_world_writer_not_active_or_dual_writer_risk';
    end if;

    select * into state
    from public.trnm_world_states_v1
    where world_id = target_world_id
    for update;
    if not found then
        raise exception using
            errcode = '23503',
            message = 'trnm_world_state_not_found';
    end if;

    -- Re-read only after both the logical replay lock and state row lock. A
    -- waiter therefore sees the committed first receipt and returns exact replay
    -- instead of leaking a CAS or unique-constraint error.
    select * into existing
    from public.trnm_world_events_v1
    where world_id = target_world_id
      and idempotency_key = target_idempotency_key;
    if found then
        if existing.event_id = target_event_id
           and existing.epoch_id = target_epoch_id
           and existing.request_sha256 = target_request_sha256
           and existing.command_canonical = canonical_command
           and existing.command_json = parsed_command
           and existing.response_canonical = canonical_response
           and existing.response_json = parsed_response
           and existing.response_sha256 = target_response_sha256
           and existing.state_before_sha256 = target_expected_state_sha256
           and existing.state_after_sha256 = target_state_after_sha256 then
            return pg_catalog.jsonb_build_object(
                'schema', 'trillionnium.world.event-application.v1',
                'status', 'replayed_exact',
                'event_id', existing.event_id,
                'world_id', existing.world_id,
                'generation_after', existing.generation_after,
                'state_after_sha256', existing.state_after_sha256
            );
        end if;
        raise exception using
            errcode = '23505',
            message = 'trnm_world_event_idempotency_conflict';
    end if;

    if state.writer_epoch_id is distinct from target_epoch_id
       or state.state_sha256 is distinct from target_expected_state_sha256 then
        raise exception using
            errcode = '40001',
            message = 'trnm_world_state_compare_and_swap_conflict';
    end if;
    if state.state_generation >= 9007199254740991 then
        raise exception using
            errcode = '22003',
            message = 'trnm_world_state_generation_overflow';
    end if;
    next_generation := state.state_generation + 1;

    insert into public.trnm_world_events_v1 (
        event_id, world_id, epoch_id, idempotency_key,
        request_sha256, command_canonical, command_json,
        response_canonical, response_json, response_sha256,
        state_before_sha256, state_after_sha256, generation_after
    ) values (
        target_event_id, target_world_id, target_epoch_id, target_idempotency_key,
        target_request_sha256, canonical_command, parsed_command,
        canonical_response, parsed_response, target_response_sha256,
        target_expected_state_sha256, target_state_after_sha256, next_generation
    );

    update public.trnm_world_states_v1
    set state_generation = next_generation,
        state_sha256 = target_state_after_sha256,
        state_canonical = canonical_state_after,
        state_json = parsed_state_after,
        writer_epoch_id = target_epoch_id,
        updated_at = transaction_timestamp()
    where world_id = target_world_id;

    return pg_catalog.jsonb_build_object(
        'schema', 'trillionnium.world.event-application.v1',
        'status', 'applied',
        'event_id', target_event_id,
        'world_id', target_world_id,
        'generation_after', next_generation,
        'state_after_sha256', target_state_after_sha256
    );
end;
$function$;

revoke all on function public.trnm_world_event_id_v1(text, text, text, text) from public;
revoke all on function public.trnm_world_import_snapshot_v1(text, text, text, text, text, text) from public;
revoke all on function public.trnm_world_activate_writer_v1(text) from public;
revoke all on function public.trnm_world_apply_event_v1(text, text, text, text, text, text, text, text, text, text, text) from public;

DO $marker$
declare
    observed_version text;
begin
    insert into public.trnm_world_authority_schema_migrations_v1 (
        migration_id,
        contract_version
    ) values (
        '0002_world_authority_cutover_v1_hardening',
        'trillionnium_world_durable_cutover_v1_hardening_1'
    )
    on conflict (migration_id) do nothing;

    select contract_version
    into observed_version
    from public.trnm_world_authority_schema_migrations_v1
    where migration_id = '0002_world_authority_cutover_v1_hardening';

    if observed_version is distinct from 'trillionnium_world_durable_cutover_v1_hardening_1' then
        raise exception using
            errcode = '23505',
            message = 'trnm_world_hardening_migration_version_conflict',
            detail = pg_catalog.format(
                'expected=%s observed=%s',
                'trillionnium_world_durable_cutover_v1_hardening_1',
                coalesce(observed_version, '<null>')
            );
    end if;
end;
$marker$;

commit;
