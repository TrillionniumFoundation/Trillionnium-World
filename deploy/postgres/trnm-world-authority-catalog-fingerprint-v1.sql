-- Canonical catalog fingerprint for the supported World authority schema.
-- Emits one jsonb document. It deliberately excludes row data and OIDs while
-- binding relation/column/constraint/index/trigger/function/ACL semantics.
with
relations as (
    select c.oid, n.nspname, c.relname, c.relkind, c.relpersistence,
           pg_catalog.pg_get_userbyid(c.relowner) as owner,
           c.relrowsecurity, c.relforcerowsecurity, c.relreplident,
           c.relacl, c.relowner
    from pg_catalog.pg_class c
    join pg_catalog.pg_namespace n on n.oid = c.relnamespace
    where n.nspname = 'public'
      and c.relname like 'trnm_world_%'
),
relation_json as (
    select coalesce(jsonb_agg(jsonb_build_object(
        'schema', nspname,
        'name', relname,
        'kind', relkind,
        'persistence', relpersistence,
        'owner', owner,
        'row_security', relrowsecurity,
        'force_row_security', relforcerowsecurity,
        'replica_identity', relreplident
    ) order by nspname, relname), '[]'::jsonb) as value
    from relations
),
column_json as (
    select coalesce(jsonb_agg(jsonb_build_object(
        'relation', r.relname,
        'position', a.attnum,
        'name', a.attname,
        'type', pg_catalog.format_type(a.atttypid, a.atttypmod),
        'not_null', a.attnotnull,
        'identity', a.attidentity,
        'generated', a.attgenerated,
        'default', pg_catalog.pg_get_expr(d.adbin, d.adrelid, true),
        'collation', case when a.attcollation = 0 then null else a.attcollation::pg_catalog.regcollation::text end
    ) order by r.relname, a.attnum), '[]'::jsonb) as value
    from relations r
    join pg_catalog.pg_attribute a on a.attrelid = r.oid
    left join pg_catalog.pg_attrdef d on d.adrelid = a.attrelid and d.adnum = a.attnum
    where a.attnum > 0 and not a.attisdropped
),
constraint_json as (
    select coalesce(jsonb_agg(jsonb_build_object(
        'relation', r.relname,
        'name', con.conname,
        'type', con.contype,
        'deferrable', con.condeferrable,
        'deferred', con.condeferred,
        'validated', con.convalidated,
        'definition', pg_catalog.pg_get_constraintdef(con.oid, true)
    ) order by r.relname, con.conname), '[]'::jsonb) as value
    from relations r
    join pg_catalog.pg_constraint con on con.conrelid = r.oid
),
index_json as (
    select coalesce(jsonb_agg(jsonb_build_object(
        'relation', r.relname,
        'name', ic.relname,
        'unique', i.indisunique,
        'primary', i.indisprimary,
        'exclusion', i.indisexclusion,
        'immediate', i.indimmediate,
        'valid', i.indisvalid,
        'ready', i.indisready,
        'live', i.indislive,
        'check_xmin', i.indcheckxmin,
        'clustered', i.indisclustered,
        'replica_identity', i.indisreplident,
        'nulls_not_distinct', i.indnullsnotdistinct,
        'definition', pg_catalog.pg_get_indexdef(i.indexrelid)
    ) order by r.relname, ic.relname), '[]'::jsonb) as value
    from relations r
    join pg_catalog.pg_index i on i.indrelid = r.oid
    join pg_catalog.pg_class ic on ic.oid = i.indexrelid
),
trigger_json as (
    select coalesce(jsonb_agg(jsonb_build_object(
        'relation', r.relname,
        'name', t.tgname,
        'enabled', t.tgenabled,
        'definition', pg_catalog.pg_get_triggerdef(t.oid, true)
    ) order by r.relname, t.tgname), '[]'::jsonb) as value
    from relations r
    join pg_catalog.pg_trigger t on t.tgrelid = r.oid
    where not t.tgisinternal
),
functions as (
    select p.oid, n.nspname, p.proname,
           pg_catalog.pg_get_userbyid(p.proowner) as owner,
           p.prosecdef, p.proleakproof, p.proisstrict, p.provolatile, p.proparallel,
           p.proconfig, p.proacl, p.proowner,
           pg_catalog.pg_get_function_identity_arguments(p.oid) as identity_arguments,
           pg_catalog.pg_get_function_result(p.oid) as result_type,
           pg_catalog.pg_get_functiondef(p.oid) as definition
    from pg_catalog.pg_proc p
    join pg_catalog.pg_namespace n on n.oid = p.pronamespace
    where n.nspname = 'public'
      and p.proname like 'trnm_world_%'
),
function_json as (
    select coalesce(jsonb_agg(jsonb_build_object(
        'schema', nspname,
        'name', proname,
        'identity_arguments', identity_arguments,
        'result_type', result_type,
        'owner', owner,
        'security_definer', prosecdef,
        'leakproof', proleakproof,
        'strict', proisstrict,
        'volatility', provolatile,
        'parallel', proparallel,
        'config', coalesce(to_jsonb(proconfig), '[]'::jsonb),
        'definition', definition
    ) order by nspname, proname, identity_arguments), '[]'::jsonb) as value
    from functions
),
relation_acl_json as (
    select coalesce(jsonb_agg(jsonb_build_object(
        'relation', r.relname,
        'grantor', pg_catalog.pg_get_userbyid(x.grantor),
        'grantee', case when x.grantee = 0 then 'PUBLIC' else pg_catalog.pg_get_userbyid(x.grantee) end,
        'privilege', x.privilege_type,
        'grantable', x.is_grantable
    ) order by r.relname, x.grantee, x.privilege_type, x.is_grantable), '[]'::jsonb) as value
    from relations r
    cross join lateral pg_catalog.aclexplode(coalesce(r.relacl, pg_catalog.acldefault('r', r.relowner))) x
),
function_acl_json as (
    select coalesce(jsonb_agg(jsonb_build_object(
        'function', f.proname,
        'identity_arguments', f.identity_arguments,
        'grantor', pg_catalog.pg_get_userbyid(x.grantor),
        'grantee', case when x.grantee = 0 then 'PUBLIC' else pg_catalog.pg_get_userbyid(x.grantee) end,
        'privilege', x.privilege_type,
        'grantable', x.is_grantable
    ) order by f.proname, f.identity_arguments, x.grantee, x.privilege_type, x.is_grantable), '[]'::jsonb) as value
    from functions f
    cross join lateral pg_catalog.aclexplode(coalesce(f.proacl, pg_catalog.acldefault('f', f.proowner))) x
)
select jsonb_build_object(
    'schema', 'trillionnium.world.authority-catalog.v1',
    'relations', relation_json.value,
    'columns', column_json.value,
    'constraints', constraint_json.value,
    'indexes', index_json.value,
    'triggers', trigger_json.value,
    'functions', function_json.value,
    'relation_privileges', relation_acl_json.value,
    'function_privileges', function_acl_json.value
)
from relation_json, column_json, constraint_json, index_json, trigger_json,
     function_json, relation_acl_json, function_acl_json;
