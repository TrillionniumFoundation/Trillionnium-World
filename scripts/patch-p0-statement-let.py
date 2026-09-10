#!/usr/bin/env python3
"""Apply the exact World P0 and operator-policy repairs to a temporary clone."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys

KEYWORD = '''fn keyword_at(bytes: &[u8], start: usize, keyword: &[u8]) -> bool {
    bytes.get(start..start + keyword.len()) == Some(keyword)
        && (start == 0 || !identifier_byte(bytes[start - 1]))
        && bytes
            .get(start + keyword.len())
            .is_none_or(|byte| !identifier_byte(*byte))
}
'''
KEYWORD_PATCHED = KEYWORD + '''
fn statement_let_at(bytes: &[u8], start: usize) -> bool {
    if !keyword_at(bytes, start, b"let") {
        return false;
    }
    let mut cursor = start;
    while cursor > 0 && bytes[cursor - 1].is_ascii_whitespace() {
        cursor -= 1;
    }
    cursor == 0 || matches!(bytes[cursor - 1], b'{' | b'}' | b';')
}
'''
OLD_GUARD = '''            _ if keyword_at(bytes, cursor, b"let") => {
'''
NEW_GUARD = '''            _ if statement_let_at(bytes, cursor) => {
'''
INLINE_FIXTURE = '''        r#"
            async fn inline_constructor() {
                let mut transaction = pool.begin().await?;
'''
PATTERN_FIXTURE = '''        r#"
            async fn pattern_let_before_captured_backend() {
                let mut transaction = pool.begin().await?;
                if let Err(error) = validate_receipt(receipt) {
                    return Err(error);
                }
                let backend = CapturedReceiptBackend {
                    receipt,
                    wallet_snapshot,
                };
                campaign.reconcile_economy(&backend, 1)?;
                transaction.commit().await?;
            }
        "#,
''' + INLINE_FIXTURE

CONFIG_HELPER_ANCHOR = '''async fn require_database_host_authority(
'''
CONFIG_HELPER = '''async fn require_cex_readiness(cex: &CexClient) -> Result<(), String> {
    cex.readiness().await
}

async fn require_database_host_authority(
'''
CONFIG_CALL_OLD = '''        cex.readiness().await?;
'''
CONFIG_CALL_NEW = '''        require_cex_readiness(&cex).await?;
'''

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


def replace_once(text: str, old: str, new: str, label: str) -> str:
    if text.count(old) != 1:
        raise SystemExit(f"{label} anchor did not match exactly once")
    return text.replace(old, new)


def patch_p0_test(root: Path) -> None:
    path = root / "trillionnium/crates/trnm-game-server/tests/p0_boundary_contract.rs"
    text = path.read_text(encoding="utf-8")
    text = replace_once(text, KEYWORD, KEYWORD_PATCHED, "keyword")
    text = replace_once(text, OLD_GUARD, NEW_GUARD, "statement-let guard")
    text = replace_once(text, INLINE_FIXTURE, PATTERN_FIXTURE, "accepted fixture")
    path.write_text(text, encoding="utf-8", newline="\n")


def patch_startup_boundary(root: Path) -> Path:
    path = root / (
        "trillionnium/crates/trnm-game-server/src/lib_parts/"
        "configuration_and_migrations/part_01.rs"
    )
    text = path.read_text(encoding="utf-8")
    text = replace_once(
        text,
        CONFIG_HELPER_ANCHOR,
        CONFIG_HELPER,
        "CEX readiness helper insertion",
    )
    text = replace_once(
        text,
        CONFIG_CALL_OLD,
        CONFIG_CALL_NEW,
        "CEX readiness call isolation",
    )
    path.write_text(text, encoding="utf-8", newline="\n")
    return path


def patch_operator_policy(root: Path) -> None:
    migration = root / (
        "trillionnium/crates/trnm-game-server/migrations/"
        "0018_online_settlement_operator_controls_v1.sql"
    )
    text = migration.read_text(encoding="utf-8")
    text = replace_once(
        text,
        POLICY_SEED_ANCHOR,
        POLICY_SEQUENCE_SYNC,
        "operator policy identity sequence synchronization",
    )
    migration.write_text(text, encoding="utf-8", newline="\n")

    test = root / (
        "trillionnium/crates/trnm-game-server/tests/"
        "settlement_operator_controls_database.rs"
    )
    text = test.read_text(encoding="utf-8")
    text = replace_once(
        text,
        OPERATOR_APPLY,
        OPERATOR_REAPPLY,
        "operator migration idempotence regression",
    )
    test.write_text(text, encoding="utf-8", newline="\n")


def update_direct_source_manifest(root: Path, source: Path) -> None:
    manifest_path = root / (
        "trillionnium/crates/trnm-game-server/src/lib_parts/manifest.json"
    )
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    relative = source.relative_to(
        root / "trillionnium/crates/trnm-game-server/src"
    ).as_posix()
    matches = [entry for entry in manifest["parts"] if entry.get("path") == relative]
    if len(matches) != 1:
        raise SystemExit("configuration direct-source manifest entry is not unique")
    data = source.read_bytes()
    matches[0]["bytes"] = len(data)
    matches[0]["sha256"] = hashlib.sha256(data).hexdigest()
    manifest_path.write_text(
        json.dumps(manifest, indent=2) + "\n",
        encoding="utf-8",
        newline="\n",
    )


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: patch-p0-statement-let.py ROOT", file=sys.stderr)
        return 64
    root = Path(sys.argv[1]).resolve()
    patch_p0_test(root)
    source = patch_startup_boundary(root)
    patch_operator_policy(root)
    update_direct_source_manifest(root, source)
    print("WORLD_P0_OPERATOR_BATCH_PATCH=PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
