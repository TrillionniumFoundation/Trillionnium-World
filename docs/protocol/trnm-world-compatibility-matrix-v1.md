---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-002
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Compatibility and Retirement Matrix v1

## Principles

- Protocol, schema, ruleset, content, build, database, and deployment are separate identities.
- Semver alone never authorizes compatibility.
- Compatibility is an explicitly tested matrix, not a parser fallback.
- Unknown combinations fail closed with stable errors.
- An advertised N-1 window has an owner, start date, end date, drain rule, and rollback evidence.

## Current matrix

| Surface | Current | Compatibility | Destination | Retirement gate |
| --- | --- | --- | --- | --- |
| World deterministic transition | `trnm_world_transition_v1` | exact v1 only | published World contract | Nakama + independent vectors green |
| RTS simulation/checkpoint | `trnm_rts_sim_v16` / checkpoint v16 | versioned readers only where tested | deterministic World runtime | save/replay migration corpus |
| campaign save | schema revision 12 | explicit migration chain | World campaign domain | backup + migration report |
| online compatibility authority | v3, exact-v2 rolling paths | laboratory/private only | Nakama canonical authority | drain/cutover/rollback rehearsal |
| online product | v2 with bounded v1 private-lobby compatibility | private profile | Nakama-owned product runtime | usage inventory + replacement |
| online operations/production | v2 plus exact tested legacy pairs | private profile | target platform services | exact component lock |
| settlement outbox | migrations 16–19 | exact schema ledger checks | transaction-free worker | deployed fault/PITR evidence |
| signer entitlement | signed entitlement v2 | exact issuer/key registry | isolated signer + CEX | key rotation and custody evidence |

## Admission algorithm

A consumer admits a component pair only when all declared identities match an
approved matrix row:

```text
contract version
schema revision
ruleset revision
content revision
producer build/source digest
consumer build/source digest
database migration ledger
feature/capability set
deployment profile
```

A missing field, wildcard, unrecognized capability, or crossed build/protocol
pair is rejection, not implicit downgrade.

## Rolling upgrade

1. Publish the matrix and exact old/new artifacts.
2. Prove old reader/new writer and new reader/old writer only for the intended direction.
3. Drain operations that cannot transfer live ownership.
4. Deploy readers before writers where additive schema requires it.
5. Monitor old-version usage and error codes.
6. End admission at the declared deadline.
7. Remove compatibility code only after retained data/replay/save inventory is migrated or archived.

Completed-match compatibility does not prove active-match actor takeover.

## Database compatibility

- Migrations are append-only and checksum-bound.
- Old writers are either proven safe, fenced, or drained before migration.
- Rollback means application rollback against a compatible schema, or a separately rehearsed restore/PITR—not editing an applied migration.
- Timeline/system identifier changes invalidate old-primary authority until re-fenced.

## Native client compatibility

A client displays a clear update/unsupported message when the server contract is
outside its admitted matrix. It never silently switches from Nakama canonical
online to World compatibility authority, or from CEX settlement to local value,
without an explicit user-visible profile.

## Retirement record

Each retired row records:

- owner and approver;
- usage inventory and last observed use;
- exact final reader/writer versions;
- data/replay/save migration outcome;
- disabled endpoint/feature flag;
- rollback expiry;
- retained security/evidence obligations.

## Acceptance

- every advertised pair has positive and crossed-negative tests;
- dates and owners are explicit before compatibility is advertised publicly;
- old paths cannot create canonical roots/signatures after authority cutover;
- status pages derive admitted combinations from machine-readable component locks.
