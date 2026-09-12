-- Trillionnium World durable authority cutover contract v1.
--
-- This schema is intentionally outside the exact historical seven-crate source
-- restoration. It provides the durable migration, single-writer, replay and
-- rollback boundary required before that source can be authorized in production.
-- Applying this migration does not grant production authorization.

begin;

create table if not exists public.trnm_world_authority_schema_migrations_v1 (
    migration_id text primary key,
    contract_version text not null,
    applied_at timestamptz not null default transaction_timestamp(),
    check (length(migration_id) between 1 and 128),
    check (length(contract_version) between 1 and 128)
);

create table if not exists public.trnm_world_authority_epochs_v1 (
    epoch_id text primary key check (epoch_id ~ '^sha256:[0-9a-f]{64}$'),
    cex_source_sha text not null check (cex_source_sha ~ '^[0-9a-f]{40}$'),
    world_source_sha text not null check (world_source_sha ~ '^[0-9a-f]{40}$'),
    status text not null check (status in ('prepared', 'active', 'rolled_back', 'failed')),
    cex_writer_disabled boolean not null default false,
    world_writer_enabled boolean not null default false,
    cex_fence_receipt_sha256 text check (
        cex_fence_receipt_sha256 is null
        or cex_fence_receipt_sha256 ~ '^sha256:[0-9a-f]{64}$'
    ),
    backfill_batch_id text check (
        backfill_batch_id is null
        or backfill_batch_id ~ '^sha256:[0-9a-f]{64}$'
    ),
    rollback_receipt_sha256 text check (
        rollback_receipt_sha256 is null
        or rollback_receipt_sha256 ~ '^sha256:[0-9a-f]{64}$'
    ),
    created_at timestamptz not null default transaction_timestamp(),
    activated_at timestamptz,
    rolled_back_at timestamptz,
    constraint trnm_world_authority_never_dual_writer_v1 check (
        not world_writer_enabled or cex_writer_disabled
    ),
    constraint trnm_world_authority_epoch_shape_v1 check (
        (
            status = 'prepared'
            and not world_writer_enabled
            and activated_at is null
            and rolled_back_at is null
            and rollback_receipt_sha256 is null
        )
        or (
            status = 'active'
            and cex_writer_disabled
            and world_writer_enabled
            and cex_fence_receipt_sha256 is not null
            and backfill_batch_id is not null
            and activated_at is not null
            and rolled_back_at is null
            and rollback_receipt_sha256 is null
        )
        or (
            status = 'rolled_back'
            and cex_writer_disabled
            and not world_writer_enabled
            and cex_fence_receipt_sha256 is not null
            and backfill_batch_id is not null
            and activated_at is not null
            and rolled_back_at is not null
            and rollback_receipt_sha256 is not null
        )
        or (
            status = 'failed'
            and not world_writer_enabled
            and rolled_back_at is null
        )
    )
);

create table if not exists public.trnm_world_migration_batches_v1 (
    batch_id text primary key check (batch_id ~ '^sha256:[0-9a-f]{64}$'),
    epoch_id text not null unique
        references public.trnm_world_authority_epochs_v1(epoch_id),
    expected_state_count bigint not null check (expected_state_count > 0),
    expected_aggregate_sha256 text not null
        check (expected_aggregate_sha256 ~ '^sha256:[0-9a-f]{64}$'),
    imported_state_count bigint,
    imported_aggregate_sha256 text check (
        imported_aggregate_sha256 is null
        or imported_aggregate_sha256 ~ '^sha256:[0-9a-f]{64}$'
    ),
    status text not null check (status in ('staged', 'reconciled', 'failed')),
    created_at timestamptz not null default transaction_timestamp(),
    reconciled_at timestamptz,
    constraint trnm_world_migration_batch_shape_v1 check (
        (
            status = 'staged'
            and imported_state_count is null
            and imported_aggregate_sha256 is null
            and reconciled_at is null
        )
        or (
            status = 'reconciled'
            and imported_state_count = expected_state_count
            and imported_aggregate_sha256 = expected_aggregate_sha256
            and reconciled_at is not null
        )
        or status = 'failed'
    )
);

create table if not exists public.trnm_world_states_v1 (
    world_id text primary key check (length(world_id) between 1 and 256),
    state_generation bigint not null check (state_generation >= 0),
    state_contract text not null check (length(state_contract) between 1 and 256),
    state_sha256 text not null check (state_sha256 ~ '^sha256:[0-9a-f]{64}$'),
    state_canonical text not null,
    state_json jsonb not null,
    source_revision text not null check (length(source_revision) between 1 and 256),
    migration_batch_id text not null
        references public.trnm_world_migration_batches_v1(batch_id),
    writer_epoch_id text not null
        references public.trnm_world_authority_epochs_v1(epoch_id),
    created_at timestamptz not null default transaction_timestamp(),
    updated_at timestamptz not null default transaction_timestamp(),
    constraint trnm_world_state_canonical_json_parity_v1 check (
        state_json = state_canonical::jsonb
    ),
    constraint trnm_world_state_canonical_hash_v1 check (
        state_sha256 = 'sha256:' || pg_catalog.encode(
            pg_catalog.sha256(pg_catalog.convert_to(state_canonical, 'UTF8')),
            'hex'
        )
    )
);

create table if not exists public.trnm_world_events_v1 (
    event_id text primary key check (event_id ~ '^sha256:[0-9a-f]{64}$'),
    world_id text not null references public.trnm_world_states_v1(world_id),
    epoch_id text not null references public.trnm_world_authority_epochs_v1(epoch_id),
    idempotency_key text not null check (length(idempotency_key) between 1 and 256),
    request_sha256 text not null check (request_sha256 ~ '^sha256:[0-9a-f]{64}$'),
    command_canonical text not null,
    command_json jsonb not null,
    response_canonical text not null,
    response_json jsonb not null,
    response_sha256 text not null check (response_sha256 ~ '^sha256:[0-9a-f]{64}$'),
    state_before_sha256 text not null check (state_before_sha256 ~ '^sha256:[0-9a-f]{64}$'),
    state_after_sha256 text not null check (state_after_sha256 ~ '^sha256:[0-9a-f]{64}$'),
    generation_after bigint not null check (generation_after > 0),
    created_at timestamptz not null default transaction_timestamp(),
    unique (world_id, idempotency_key),
    constraint trnm_world_event_command_parity_v1 check (
        command_json = command_canonical::jsonb
    ),
    constraint trnm_world_event_response_parity_v1 check (
        response_json = response_canonical::jsonb
    ),
    constraint trnm_world_event_response_hash_v1 check (
        response_sha256 = 'sha256:' || pg_catalog.encode(
            pg_catalog.sha256(pg_catalog.convert_to(response_canonical, 'UTF8')),
            'hex'
        )
    )
);

create table if not exists public.trnm_world_authority_receipts_v1 (
    receipt_id text primary key check (receipt_id ~ '^sha256:[0-9a-f]{64}$'),
    epoch_id text not null references public.trnm_world_authority_epochs_v1(epoch_id),
    receipt_kind text not null check (
        receipt_kind in ('batch_staged', 'batch_reconciled', 'cex_writer_fenced', 'world_writer_activated', 'world_writer_rolled_back')
    ),
    payload_sha256 text not null check (payload_sha256 ~ '^sha256:[0-9a-f]{64}$'),
    payload_json jsonb not null,
    created_at timestamptz not null default transaction_timestamp()
);

create index if not exists trnm_world_events_world_generation_v1
    on public.trnm_world_events_v1(world_id, generation_after);
create index if not exists trnm_world_events_epoch_created_v1
    on public.trnm_world_events_v1(epoch_id, created_at, event_id);
create index if not exists trnm_world_states_epoch_v1
    on public.trnm_world_states_v1(writer_epoch_id, world_id);
create index if not exists trnm_world_receipts_epoch_kind_v1
    on public.trnm_world_authority_receipts_v1(epoch_id, receipt_kind, created_at);

create or replace function public.trnm_world_sha256_text_v1(value text)
returns text
language sql
immutable
strict
set search_path = pg_catalog
as $function$
    select 'sha256:' || pg_catalog.encode(
        pg_catalog.sha256(pg_catalog.convert_to(value, 'UTF8')),
        'hex'
    )
$function$;

create or replace function public.trnm_world_receipt_id_v1(
    target_epoch_id text,
    target_kind text,
    target_payload_sha256 text
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
                target_epoch_id || E'\n' || target_kind || E'\n' || target_payload_sha256,
                'UTF8'
            )
        ),
        'hex'
    )
$function$;

create or replace function public.trnm_world_append_receipt_v1(
    target_epoch_id text,
    target_kind text,
    target_payload_json jsonb
)
returns text
language plpgsql
security definer
set search_path = pg_catalog, public
as $function$
declare
    canonical_payload text;
    payload_hash text;
    generated_receipt_id text;
    existing public.trnm_world_authority_receipts_v1%rowtype;
begin
    canonical_payload := target_payload_json::text;
    payload_hash := public.trnm_world_sha256_text_v1(canonical_payload);
    generated_receipt_id := public.trnm_world_receipt_id_v1(
        target_epoch_id,
        target_kind,
        payload_hash
    );

    select * into existing
    from public.trnm_world_authority_receipts_v1
    where receipt_id = generated_receipt_id;

    if found then
        if existing.epoch_id is distinct from target_epoch_id
           or existing.receipt_kind is distinct from target_kind
           or existing.payload_sha256 is distinct from payload_hash
           or existing.payload_json is distinct from target_payload_json then
            raise exception using
                errcode = '23505',
                message = 'trnm_world_authority_receipt_replay_conflict';
        end if;
        return generated_receipt_id;
    end if;

    insert into public.trnm_world_authority_receipts_v1 (
        receipt_id, epoch_id, receipt_kind, payload_sha256, payload_json
    ) values (
        generated_receipt_id,
        target_epoch_id,
        target_kind,
        payload_hash,
        target_payload_json
    );
    return generated_receipt_id;
end;
$function$;

create or replace function public.trnm_world_stage_migration_batch_v1(
    target_epoch_id text,
    target_batch_id text,
    target_cex_source_sha text,
    target_world_source_sha text,
    target_expected_state_count bigint,
    target_expected_aggregate_sha256 text
)
returns text
language plpgsql
security definer
set search_path = pg_catalog, public
as $function$
declare
    existing_epoch public.trnm_world_authority_epochs_v1%rowtype;
    existing_batch public.trnm_world_migration_batches_v1%rowtype;
    receipt_id text;
begin
    if target_epoch_id !~ '^sha256:[0-9a-f]{64}$'
       or target_batch_id !~ '^sha256:[0-9a-f]{64}$'
       or target_cex_source_sha !~ '^[0-9a-f]{40}$'
       or target_world_source_sha !~ '^[0-9a-f]{40}$'
       or target_expected_state_count <= 0
       or target_expected_aggregate_sha256 !~ '^sha256:[0-9a-f]{64}$' then
        raise exception using
            errcode = '22023',
            message = 'trnm_world_migration_batch_arguments_invalid';
    end if;

    select * into existing_epoch
    from public.trnm_world_authority_epochs_v1
    where epoch_id = target_epoch_id
    for update;

    if found then
        if existing_epoch.cex_source_sha is distinct from target_cex_source_sha
           or existing_epoch.world_source_sha is distinct from target_world_source_sha
           or existing_epoch.status is distinct from 'prepared'
           or existing_epoch.world_writer_enabled then
            raise exception using
                errcode = '23505',
                message = 'trnm_world_authority_epoch_replay_conflict';
        end if;
    else
        insert into public.trnm_world_authority_epochs_v1 (
            epoch_id, cex_source_sha, world_source_sha, status,
            cex_writer_disabled, world_writer_enabled
        ) values (
            target_epoch_id, target_cex_source_sha, target_world_source_sha,
            'prepared', false, false
        );
    end if;

    select * into existing_batch
    from public.trnm_world_migration_batches_v1
    where batch_id = target_batch_id
    for update;

    if found then
        if existing_batch.epoch_id is distinct from target_epoch_id
           or existing_batch.expected_state_count is distinct from target_expected_state_count
           or existing_batch.expected_aggregate_sha256 is distinct from target_expected_aggregate_sha256
           or existing_batch.status not in ('staged', 'reconciled') then
            raise exception using
                errcode = '23505',
                message = 'trnm_world_migration_batch_replay_conflict';
        end if;
    else
        if exists (
            select 1 from public.trnm_world_migration_batches_v1
            where epoch_id = target_epoch_id
        ) then
            raise exception using
                errcode = '23505',
                message = 'trnm_world_authority_epoch_has_different_batch';
        end if;
        insert into public.trnm_world_migration_batches_v1 (
            batch_id, epoch_id, expected_state_count,
            expected_aggregate_sha256, status
        ) values (
            target_batch_id, target_epoch_id, target_expected_state_count,
            target_expected_aggregate_sha256, 'staged'
        );
    end if;

    receipt_id := public.trnm_world_append_receipt_v1(
        target_epoch_id,
        'batch_staged',
        pg_catalog.jsonb_build_object(
            'schema', 'trillionnium.world.migration.batch-staged.v1',
            'batch_id', target_batch_id,
            'expected_state_count', target_expected_state_count,
            'expected_aggregate_sha256', target_expected_aggregate_sha256,
            'cex_source_sha', target_cex_source_sha,
            'world_source_sha', target_world_source_sha
        )
    );
    return receipt_id;
end;
$function$;

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
begin
    computed_hash := public.trnm_world_sha256_text_v1(target_state_canonical);
    if target_state_sha256 is distinct from computed_hash then
        raise exception using
            errcode = '23514',
            message = 'trnm_world_import_state_hash_mismatch';
    end if;
    parsed_state := target_state_canonical::jsonb;
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
           and existing.state_canonical = target_state_canonical then
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
           and existing.state_canonical = target_state_canonical
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
        target_state_canonical, parsed_state, target_source_revision,
        target_batch_id, batch.epoch_id
    );
    return 'imported';
end;
$function$;

create or replace function public.trnm_world_reconcile_migration_batch_v1(
    target_batch_id text
)
returns text
language plpgsql
security definer
set search_path = pg_catalog, public
as $function$
declare
    batch public.trnm_world_migration_batches_v1%rowtype;
    actual_count bigint;
    actual_aggregate text;
    receipt_id text;
begin
    select * into batch
    from public.trnm_world_migration_batches_v1
    where batch_id = target_batch_id
    for update;
    if not found then
        raise exception using
            errcode = '23503',
            message = 'trnm_world_migration_batch_not_found';
    end if;

    select count(*)::bigint,
           public.trnm_world_sha256_text_v1(
               pg_catalog.string_agg(
                   state.world_id || '|' || state.state_sha256,
                   E'\n' order by state.world_id
               )
           )
    into actual_count, actual_aggregate
    from public.trnm_world_states_v1 as state
    where state.migration_batch_id = target_batch_id;

    if actual_count is distinct from batch.expected_state_count
       or actual_aggregate is distinct from batch.expected_aggregate_sha256 then
        raise exception using
            errcode = '23514',
            message = 'trnm_world_backfill_reconciliation_mismatch',
            detail = pg_catalog.format(
                'expected count=%s aggregate=%s, got count=%s aggregate=%s',
                batch.expected_state_count,
                batch.expected_aggregate_sha256,
                actual_count,
                coalesce(actual_aggregate, '<null>')
            );
    end if;

    if batch.status = 'staged' then
        update public.trnm_world_migration_batches_v1
        set imported_state_count = actual_count,
            imported_aggregate_sha256 = actual_aggregate,
            status = 'reconciled',
            reconciled_at = transaction_timestamp()
        where batch_id = target_batch_id;

        update public.trnm_world_authority_epochs_v1
        set backfill_batch_id = target_batch_id
        where epoch_id = batch.epoch_id
          and status = 'prepared'
          and not world_writer_enabled;
    elsif batch.status <> 'reconciled' then
        raise exception using
            errcode = '55000',
            message = 'trnm_world_migration_batch_not_reconcilable';
    end if;

    receipt_id := public.trnm_world_append_receipt_v1(
        batch.epoch_id,
        'batch_reconciled',
        pg_catalog.jsonb_build_object(
            'schema', 'trillionnium.world.migration.batch-reconciled.v1',
            'batch_id', target_batch_id,
            'state_count', actual_count,
            'aggregate_sha256', actual_aggregate
        )
    );
    return receipt_id;
end;
$function$;

create or replace function public.trnm_world_record_cex_writer_fence_v1(
    target_epoch_id text,
    target_cex_fence_receipt_sha256 text
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
    if target_cex_fence_receipt_sha256 !~ '^sha256:[0-9a-f]{64}$' then
        raise exception using
            errcode = '22023',
            message = 'trnm_world_cex_fence_receipt_invalid';
    end if;

    select * into epoch
    from public.trnm_world_authority_epochs_v1
    where epoch_id = target_epoch_id
    for update;
    if not found then
        raise exception using
            errcode = '23503',
            message = 'trnm_world_authority_epoch_not_found';
    end if;
    if epoch.status <> 'prepared' or epoch.world_writer_enabled
       or epoch.backfill_batch_id is null then
        raise exception using
            errcode = '55000',
            message = 'trnm_world_cex_fence_before_reconciliation_forbidden';
    end if;

    select * into strict batch
    from public.trnm_world_migration_batches_v1
    where batch_id = epoch.backfill_batch_id;
    if batch.status <> 'reconciled' then
        raise exception using
            errcode = '55000',
            message = 'trnm_world_cex_fence_requires_reconciled_backfill';
    end if;

    if epoch.cex_writer_disabled then
        if epoch.cex_fence_receipt_sha256 is distinct from target_cex_fence_receipt_sha256 then
            raise exception using
                errcode = '23505',
                message = 'trnm_world_cex_fence_replay_conflict';
        end if;
    else
        update public.trnm_world_authority_epochs_v1
        set cex_writer_disabled = true,
            cex_fence_receipt_sha256 = target_cex_fence_receipt_sha256
        where epoch_id = target_epoch_id;
    end if;

    receipt_id := public.trnm_world_append_receipt_v1(
        target_epoch_id,
        'cex_writer_fenced',
        pg_catalog.jsonb_build_object(
            'schema', 'trillionnium.world.cutover.cex-writer-fenced.v1',
            'cex_fence_receipt_sha256', target_cex_fence_receipt_sha256,
            'backfill_batch_id', epoch.backfill_batch_id
        )
    );
    return receipt_id;
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
    select * into epoch
    from public.trnm_world_authority_epochs_v1
    where epoch_id = target_epoch_id
    for update;
    if not found then
        raise exception using
            errcode = '23503',
            message = 'trnm_world_authority_epoch_not_found';
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
    generated_response_hash text;
    generated_state_hash text;
    next_generation bigint;
begin
    if target_event_id !~ '^sha256:[0-9a-f]{64}$'
       or target_request_sha256 !~ '^sha256:[0-9a-f]{64}$'
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
    generated_response_hash := public.trnm_world_sha256_text_v1(target_response_canonical);
    generated_state_hash := public.trnm_world_sha256_text_v1(target_state_after_canonical);
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

    select * into existing
    from public.trnm_world_events_v1
    where world_id = target_world_id
      and idempotency_key = target_idempotency_key;
    if found then
        if existing.event_id = target_event_id
           and existing.epoch_id = target_epoch_id
           and existing.request_sha256 = target_request_sha256
           and existing.command_canonical = target_command_canonical
           and existing.response_canonical = target_response_canonical
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

    select * into state
    from public.trnm_world_states_v1
    where world_id = target_world_id
    for update;
    if not found then
        raise exception using
            errcode = '23503',
            message = 'trnm_world_state_not_found';
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
        target_request_sha256, target_command_canonical, parsed_command,
        target_response_canonical, parsed_response, target_response_sha256,
        target_expected_state_sha256, target_state_after_sha256, next_generation
    );

    update public.trnm_world_states_v1
    set state_generation = next_generation,
        state_sha256 = target_state_after_sha256,
        state_canonical = target_state_after_canonical,
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

create or replace function public.trnm_world_rollback_writer_v1(
    target_epoch_id text,
    target_rollback_receipt_sha256 text
)
returns text
language plpgsql
security definer
set search_path = pg_catalog, public
as $function$
declare
    epoch public.trnm_world_authority_epochs_v1%rowtype;
    receipt_id text;
begin
    if target_rollback_receipt_sha256 !~ '^sha256:[0-9a-f]{64}$' then
        raise exception using
            errcode = '22023',
            message = 'trnm_world_rollback_receipt_invalid';
    end if;

    select * into epoch
    from public.trnm_world_authority_epochs_v1
    where epoch_id = target_epoch_id
    for update;
    if not found then
        raise exception using
            errcode = '23503',
            message = 'trnm_world_authority_epoch_not_found';
    end if;

    if epoch.status = 'rolled_back' then
        if epoch.rollback_receipt_sha256 is distinct from target_rollback_receipt_sha256
           or epoch.world_writer_enabled
           or not epoch.cex_writer_disabled then
            raise exception using
                errcode = '23505',
                message = 'trnm_world_rollback_replay_conflict';
        end if;
    elsif epoch.status = 'active' then
        update public.trnm_world_authority_epochs_v1
        set status = 'rolled_back',
            world_writer_enabled = false,
            rollback_receipt_sha256 = target_rollback_receipt_sha256,
            rolled_back_at = transaction_timestamp()
        where epoch_id = target_epoch_id;
    else
        raise exception using
            errcode = '55000',
            message = 'trnm_world_authority_epoch_not_rollbackable';
    end if;

    receipt_id := public.trnm_world_append_receipt_v1(
        target_epoch_id,
        'world_writer_rolled_back',
        pg_catalog.jsonb_build_object(
            'schema', 'trillionnium.world.cutover.world-writer-rolled-back.v1',
            'epoch_id', target_epoch_id,
            'rollback_receipt_sha256', target_rollback_receipt_sha256,
            'cex_writer_disabled', true,
            'world_writer_enabled', false
        )
    );
    return receipt_id;
end;
$function$;

create or replace function public.trnm_world_reject_append_only_mutation_v1()
returns trigger
language plpgsql
set search_path = pg_catalog
as $function$
begin
    raise exception using
        errcode = '55000',
        message = 'trnm_world_append_only_authority_mutation_forbidden',
        detail = pg_catalog.format('%s is append-only authority evidence', tg_table_name);
end;
$function$;

drop trigger if exists trnm_world_events_immutable_v1 on public.trnm_world_events_v1;
create trigger trnm_world_events_immutable_v1
before update or delete on public.trnm_world_events_v1
for each row execute function public.trnm_world_reject_append_only_mutation_v1();

drop trigger if exists trnm_world_events_truncate_guard_v1 on public.trnm_world_events_v1;
create trigger trnm_world_events_truncate_guard_v1
before truncate on public.trnm_world_events_v1
for each statement execute function public.trnm_world_reject_append_only_mutation_v1();

drop trigger if exists trnm_world_receipts_immutable_v1 on public.trnm_world_authority_receipts_v1;
create trigger trnm_world_receipts_immutable_v1
before update or delete on public.trnm_world_authority_receipts_v1
for each row execute function public.trnm_world_reject_append_only_mutation_v1();

drop trigger if exists trnm_world_receipts_truncate_guard_v1 on public.trnm_world_authority_receipts_v1;
create trigger trnm_world_receipts_truncate_guard_v1
before truncate on public.trnm_world_authority_receipts_v1
for each statement execute function public.trnm_world_reject_append_only_mutation_v1();

alter table public.trnm_world_events_v1 enable always trigger trnm_world_events_immutable_v1;
alter table public.trnm_world_events_v1 enable always trigger trnm_world_events_truncate_guard_v1;
alter table public.trnm_world_authority_receipts_v1 enable always trigger trnm_world_receipts_immutable_v1;
alter table public.trnm_world_authority_receipts_v1 enable always trigger trnm_world_receipts_truncate_guard_v1;

revoke all on table public.trnm_world_authority_schema_migrations_v1 from public;
revoke all on table public.trnm_world_authority_epochs_v1 from public;
revoke all on table public.trnm_world_migration_batches_v1 from public;
revoke all on table public.trnm_world_states_v1 from public;
revoke all on table public.trnm_world_events_v1 from public;
revoke all on table public.trnm_world_authority_receipts_v1 from public;
revoke all on function public.trnm_world_append_receipt_v1(text, text, jsonb) from public;
revoke all on function public.trnm_world_stage_migration_batch_v1(text, text, text, text, bigint, text) from public;
revoke all on function public.trnm_world_import_snapshot_v1(text, text, text, text, text, text) from public;
revoke all on function public.trnm_world_reconcile_migration_batch_v1(text) from public;
revoke all on function public.trnm_world_record_cex_writer_fence_v1(text, text) from public;
revoke all on function public.trnm_world_activate_writer_v1(text) from public;
revoke all on function public.trnm_world_apply_event_v1(text, text, text, text, text, text, text, text, text, text, text) from public;
revoke all on function public.trnm_world_rollback_writer_v1(text, text) from public;

insert into public.trnm_world_authority_schema_migrations_v1 (
    migration_id, contract_version
) values (
    '0001_world_authority_cutover_v1',
    'trillionnium_world_durable_cutover_v1'
)
on conflict (migration_id) do update
set contract_version = excluded.contract_version
where public.trnm_world_authority_schema_migrations_v1.contract_version = excluded.contract_version;

commit;
