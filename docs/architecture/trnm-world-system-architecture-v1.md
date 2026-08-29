---
status: current
owner: trillionnium-world
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World System Architecture v1

## 1. Scope

This document describes the active game-product architecture and its external
authority boundaries. It does not describe the excluded legacy platform
workspace as an active component.

## 2. System context

```text
Players and operators
        |
        v
Native First Contact client
        |
        +--> Nakama canonical online authority
        |       |
        |       +--> World deterministic transition interface
        |       +--> canonical archive and MatchCompletedV1
        |
        +--> World local campaign/save for offline product
        |
        +--> CEX identity/wallet presentation through scoped contracts

World settlement services
  PostgreSQL capture -> settlement worker -> entitlement signer -> CEX ledger
                                 |
                                 +-> exact Campaign apply

Integration binds exact World/Nakama/Chain/CEX artifacts and evidence.
```

## 3. Component ownership

### Native client — `trnm-first-contact`

Owns rendering, input mapping, accessibility, local saves, player-facing
reconciliation state and deterministic simulation adapters. It never becomes a
canonical online authority and never carries shared service credentials.

### Campaign aggregate — `trnm-campaign-core`

Owns campaign progression, inventory, local/world economy state and durable save
invariants. It accepts only validated receipts/results and must preserve state
on rejected commands.

### Deterministic RTS — `trnm-rts-sim`

Owns Bevy-free deterministic battle state, orders, replay material and
checkpoints. It does not own network admission, canonical global order or
completion signing.

### Compatibility game server — `trnm-game-server`

Temporarily owns the World-local laboratory authority, PostgreSQL compatibility
state, signer service and settlement worker. Public canonical authority is not
added here. The enclave is retired after Nakama cutover.

### Nakama

Owns target online admission, participant roles, canonical order, online
idempotency, restart recovery, archive roots and completion signing. World
receives only versioned deterministic transition requests.

### CEX

Owns identity verification contracts, issuer registry, wallet/ledger execution,
receipts and custody. World owns intent creation and game-side application, not
ledger truth.

### Chain

Owns consensus/finality. World and CEX artifacts are not Chain-finalized until
bound by a Chain-owned inclusion/finality receipt.

### Integration

Owns exact component locks, cross-repository compatibility and release evidence
that spans components.

## 4. Canonical data ownership

| Data | Owner | Durable store | Consumers |
| --- | --- | --- | --- |
| Offline Campaign save | World | local atomic save | client |
| Deterministic battle state | World | state/checkpoint format | client, World rules adapter |
| Online participant/session | Nakama/CEX by contract | owning service | client, Nakama |
| Canonical online command order | Nakama | Nakama-owned persistence | World transition caller, replay |
| World transition/output hash | World | caller/archive material | Nakama, Integration |
| `MatchCompletedV1` | Nakama | canonical archive | Integration, Chain/CEX adapters |
| Settlement intent | World | Campaign/outbox | signer/CEX worker |
| Settlement receipt/wallet truth | CEX | CEX ledger | World exact apply |
| Cross-repository component lock | Integration | Integration registry | release system |

## 5. Online command lifecycle

```text
1. Client submits authenticated command to Nakama.
2. Nakama validates participant role, version and idempotency identity.
3. Nakama assigns canonical sequence and persists admission.
4. Nakama calls the exact World transition contract.
5. World validates canonical request bytes and deterministic rules.
6. World returns unsigned accepted/rejected material and World hashes.
7. Nakama persists the canonical event/archive update.
8. At terminal state Nakama constructs and signs MatchCompletedV1.
9. Integration binds exact components; downstream Chain/CEX work is separate.
```

No World-local sequence/root/signature competes with steps 2–8 after cutover.

## 6. Settlement lifecycle

```text
Capture transaction
  lock exact terminal/campaign rows
  validate terminal and campaign fences
  create immutable capture and jobs
  commit

Remote phase
  claim live lease
  lookup durable signer receipt
  sign only after exact not-found
  lookup durable CEX receipt
  submit only after exact not-found
  persist remote receipt under live lease

Apply transaction
  relock exact terminal/campaign/jobs
  revalidate capture and campaign CAS fences
  apply captured receipt to candidate Campaign values
  persist Campaigns, apply markers and terminal state atomically
  commit
```

Remote calls never occur in capture/apply transactions.

## 7. Failure domains

- Client crash: command journal and server idempotency recover without duplicate
  command effect.
- Nakama process/host failure: Nakama-owned persistence and fencing recover
  canonical order.
- World rules failure: stable rejection or unavailable code; no canonical
  publication occurs without a valid result.
- Settlement worker crash: lease expiry/takeover recovers immutable remote
  identity.
- Signer/CEX response loss: exact lookup recovers a durable receipt before any
  resend.
- PostgreSQL failover/PITR: old-primary fencing and lineage checks prevent stale
  writers.
- Poison data: durable quarantine prevents one item from blocking unrelated
  accounts.

## 8. Deployment profiles

### Development/local

May use loopback services, explicit development binary fallback and same-host
fault harnesses. It grants no public or cross-host credit.

### Candidate single-host

Uses immutable release selectors, distinct service users/credentials,
PostgreSQL, signer, settlement worker and exact evidence bindings. It remains
single-host evidence.

### Target production

Requires Nakama canonical authority, multi-host fencing/recovery, mTLS/workload
identity, KMS/HSM custody, public edge protection, capacity, backup/PITR,
monitoring/on-call and approved product/commercial gates.

## 9. Observability minimums

- authority admission/rejection/sequence/recovery metrics;
- deterministic transition latency, rejection code and divergence metrics;
- settlement pending/leased/retry/dead-letter/pending-apply age and counts;
- lease takeover, ambiguous lookup and operator replay events;
- PostgreSQL pool/lock/query-plan/migration health;
- client journal/reconnect/render/input latency;
- immutable audit events carrying component and release identities.

Metrics must not expose session tokens, private keys, raw credentials or
unnecessary personal data.
