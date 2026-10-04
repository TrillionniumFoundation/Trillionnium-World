---
status: current-candidate
owner: trillionnium-world
applies_to_plan: trillionnium-world-development-2026-08-29-v4
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World System Context v1

## Purpose

This document defines the system boundary around Trillionnium World and prevents game-domain code from silently absorbing online authority, Chain finality, wallet custody or cross-repository release responsibilities.

## Context map

```text
Players / Operators
        |
        v
Native World Client
  - presentation and input
  - local settings/save slots
  - deterministic offline orchestration
  - online command intent submission
        |
        +------------------------+
        |                        |
        v                        v
Nakama Canonical Runtime     World Deterministic Runtime
  - admission                  - ruleset/content interpretation
  - participant roles          - command validation
  - canonical order            - deterministic state transition
  - idempotency                - World state/outcome hashes
  - restart recovery           - unsigned replay/outcome material
  - canonical roots                  |
  - MatchCompletedV1                 |
        |                             |
        +-------------+---------------+
                      v
               Integration Lock
          exact revisions + evidence
             /          |          \
            v           v           v
        Chain          CEX       Operations
  ingress/finality  wallet/ledger  deployment/edge
```

## Trust boundaries

### Client boundary

The client is untrusted for canonical online state, participant identity, settlement success and finality. Client-side journals provide retry continuity, not authority. Any client-supplied result, balance, receipt or completion claim must be independently verified.

### World boundary

World is trusted only for deterministic game-domain behavior under an exact ruleset/content/component lock. World output remains unsigned and noncanonical with respect to online admission/order/completion unless Nakama binds it into its own evidence.

### Nakama boundary

Nakama is accountable for canonical online sequencing and completion evidence. Nakama must consume a versioned World contract and must not infer compatibility from an unchecked sibling path or release label.

### Settlement boundary

World produces game-owned economic intents. CEX is authoritative for wallet/ledger state. The settlement worker may transport intents and verified receipts, but cannot manufacture ledger success. Remote success and campaign application are separate durable states.

### Chain boundary

Chain finality requires a Chain-owned receipt bound to canonical ingress and AppHash/finality. World or Nakama hashes alone do not establish Chain finality.

### Integration boundary

Integration owns exact multi-repository component locks and cross-system evidence. A World-only test cannot promote a cross-repository claim.

## Runtime components owned here

| Component | Responsibility | Persistence | Authority posture |
| --- | --- | --- | --- |
| `trnm-first-contact` | native client and player experience | local save/settings/journal | client, never canonical online authority |
| `trnm-campaign-core` | campaign/save/progression aggregate | serialized campaign state | game-domain aggregate |
| `trnm-rts-sim` | deterministic battle simulation | snapshots/replay material | deterministic World rules |
| `trnm-online-protocol` | compatibility wire vocabulary | none | compatibility only |
| `trnm-game-server` | World-local laboratory authority and migration evidence | PostgreSQL + host journal | `world_legacy_local_alpha` enclave |
| `trnm-economy-protocol` | typed game intent/receipt vocabulary | embedded in campaign/outbox | no wallet custody |
| `trnm-settlement-worker` | capture/remote execution/fenced apply | PostgreSQL outbox/capture/jobs | transport and apply coordinator only |
| `trnm-world-transition-contract` | authority-neutral deterministic boundary | none | unsigned World contract |

## Data ownership matrix

| Data | System of record |
| --- | --- |
| Authored map/quest/unit/rules content | World repository/release artifact |
| Offline campaign state | World client/campaign persistence |
| Online canonical event sequence | Nakama |
| World deterministic state/output | World transition result under exact revision |
| Online canonical archive root | Nakama |
| Match completion signature | Nakama |
| Chain inclusion/finality | Chain |
| Wallet balance and ledger receipt | CEX |
| Cross-repository evidence manifest | Integration |
| Public deployment topology | Operations/Integration evidence |

## Forbidden shortcuts

- World constructing or signing `MatchCompletedV1`.
- Nakama reimplementing World rules without conformance and component locking.
- Client-provided battle result directly mutating campaign or wallet state.
- Game-server synchronous CEX execution under a mutable database transaction.
- World or Nakama claiming Chain finality from local hashes.
- A generated status file granting public, custody, human or commercial credit.
- Production depending on sibling source checkouts.

## Failure ownership

| Failure | Accountable recovery owner |
| --- | --- |
| deterministic rule rejection | World contract |
| admission/session/order conflict | Nakama |
| World transition unavailable | Nakama retry/fail-close policy plus World service owner |
| settlement response loss | settlement outbox + signer/CEX lookup contracts |
| campaign apply conflict | World settlement apply transaction |
| Chain inclusion/finality failure | Chain/Integration |
| public ingress or regional outage | Operations/Integration |
| custody or key compromise | CEX/security owner |

## Change control

Any change that moves a row in the ownership matrix requires:

1. an ADR;
2. migration and rollback impact;
3. protocol compatibility impact;
4. security/threat impact;
5. evidence and release-gate impact;
6. owner approval from every affected repository.