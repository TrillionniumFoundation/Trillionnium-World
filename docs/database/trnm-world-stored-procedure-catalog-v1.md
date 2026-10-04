---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-003
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Stored Procedure Catalogue v1

## Purpose

PostgreSQL functions in the compatibility authority and settlement system are
part of the correctness boundary. This catalogue defines the contract every
procedure must expose. Source remains authoritative for the exact current
signature until generated catalogue tooling lands.

## Contract fields

Every procedure record must identify:

- schema/name/signature and migration owner;
- security mode and fixed `search_path`;
- required role/privileges;
- preconditions and row/advisory locks;
- mutations and returned result;
- idempotency/fencing identity;
- retryable/nonretryable SQLSTATEs;
- supported PostgreSQL versions;
- index/query-plan dependencies;
- fault and rolling-writer tests.

`SECURITY DEFINER` requires an explicit threat review and least-privilege grant.
Unqualified object lookup is forbidden.

## Migration ledger

### `trnm_online_schema_migrations`

Records exact version, name, SQL SHA-256, and application time. Startup takes the
global migration advisory lock; a name/checksum mismatch fails closed.

## Authority hot path classes

| Class | Representative responsibility | Required fence |
| --- | --- | --- |
| host authority | one database/physical-host authority process | system ID/timeline/host/owner nonce/advisory locks |
| command commit | admission, input sequence, total compatibility order, state revision | member cursor + instance/generation + fingerprint |
| checkpoint | exact tick/revision/hash publication | current actor/host/epoch |
| terminal publication | staged terminal result and publication ACK | complete tuple + hot/cold witness |
| fail-close | abandonment and quarantine | atomic match marker + witness sequence |
| fleet | lease/heartbeat/drain/route | instance ID + epoch + physical host |

These remain compatibility-enclave contracts; they do not create Nakama
canonical authority.

## Settlement procedures

### Identity and serialization

```text
trnm_online_remote_request_id_v1
trnm_online_settlement_serialization_key_v1
```

Pure/stable functions producing immutable remote request and account/campaign
serialization identities.

### Claim and remote execution fencing

```text
trnm_online_claim_settlement_job_v2
trnm_online_store_settlement_authorization_v1
trnm_online_begin_settlement_remote_attempt_v1
trnm_online_complete_settlement_job_v1
trnm_online_retry_settlement_job_v1
trnm_online_dead_letter_settlement_job_v1
```

All mutations require the exact live lease owner/generation and unexpired lease.
Claim v1 is retired fail-closed.

### Quarantine

```text
trnm_online_settlement_scope_quarantined_v1
trnm_online_record_settlement_quarantine_v1
trnm_online_quarantine_claimed_settlement_job_v1
trnm_online_resolve_settlement_quarantine_v1
```

Quarantine isolates poison work; resolution is privileged and audited. It never
creates a receipt, applies campaign state, or resets immutable identities.

### Operator controls

```text
trnm_online_append_settlement_operator_policy_v1
trnm_online_authorize_settlement_replay_v1
```

Policy/replay evidence is append-only. Replay is exact-identity-bound,
receipt-free, dead-letter-only, and authorizes at most one additional remote
attempt under the retained stable request identity.

## Required views

```text
trnm_online_settlement_job_status_v1
trnm_online_settlement_metrics_v1
trnm_online_settlement_quarantine_status_v1
trnm_online_settlement_operator_policy_current_v1
```

Views must distinguish remote success from campaign application and expose
backlog age, expired leases, retries, dead letters, quarantine, and operator
activity without leaking credentials or raw sensitive payloads.

## SQLSTATE policy

| SQLSTATE | Meaning |
| --- | --- |
| `22023` | invalid argument/policy request |
| `23514` | immutable identity/check constraint conflict |
| `55000` | object/state not in required operational state |
| `0A000` | explicitly retired/unsupported procedure |
| `P0002` | exact requested object absent |

Database/network errors outside this catalogue remain retryable only when the
caller can prove idempotent/fenced recovery.

## Privilege policy

- Public role receives no operator replay/resolution or policy mutation rights.
- Runtime roles receive only procedures/tables required by their audience.
- Readiness/metrics roles are read-only.
- Migration role is separate from ordinary runtime.
- CEX/signer credentials do not imply database operator privilege.

## Generated catalogue target

The target machine artifact is:

`docs/database/generated/trnm-world-postgresql-api-v1.json`

It should be generated from migrations plus explicit metadata and compared to
this contract in CI. `WORLD-P1-003` remains open until every current procedure,
view, trigger, privilege, SQLSTATE, and index dependency is inventoried and the
rolling/rollback matrix is green.
