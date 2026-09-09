---
status: current-candidate
owner: trillionnium-world-architecture
last_reviewed: 2026-09-08
review_due: 2026-10-08
supersedes: []
---

# ADR-0003: World-domain authority is distinct from Nakama canonical online authority

## Context

Trillionnium World is extracting World business-state mutation and deterministic domain decisions from CEX into a bounded World-owned service/workspace. Separately, ADR-0001 assigns canonical online participant admission, total ordering, idempotency, restart recovery, archive roots, and `MatchCompletedV1` signing to Nakama. Without an explicit decision, the phrase “World authority” can be misread as authorization to create a second canonical online match runtime.

## Decision

A World-domain authority component may own deterministic World state, command validation/decision under a versioned ruleset, map/content lookup, persistence of World-owned aggregates, projections, and an internal versioned transition API. It may replace World mutation that was historically embedded in CEX.

It must not own or infer:

- public player admission or participant/session identity;
- the canonical online global command/event order;
- online duplicate identity across reconnects and generations;
- canonical restart/recovery history or archive roots;
- canonical match-completion signing;
- Chain inclusion/finality;
- wallet/ledger balance, custody, or settlement success;
- cross-repository release qualification.

Nakama remains the sole target canonical online owner. CEX remains the sole wallet/ledger owner. Chain and Integration retain their existing responsibilities.

## Interface model

The target online call chain is:

```text
player
  -> Nakama admission/session/order/idempotency
  -> versioned World-domain transition request
  -> deterministic World decision/state/output hashes
  -> Nakama canonical event/archive/completion binding
  -> separate World settlement intent capture
  -> signer/CEX exact lookup-or-submit
```

During migration, a CEX-owned adapter may invoke the internal World-domain API only for the explicitly bounded cutover path. This does not make CEX the target online admission owner, and the adapter must be removed or narrowed after reconciliation. The World server must not expose a new public player route that bypasses Nakama.

## Identity separation

The following identities are distinct and may never be silently aliased:

- Nakama session/member/control and canonical sequence identities;
- World command/transition/event and aggregate revision identities;
- campaign, match, ruleset, content, build, and component-lock identities;
- settlement intent, remote request, signer entitlement, and CEX receipt identities;
- Chain transaction/inclusion/finality identities.

Every API binds the identities it consumes and returns. A hash from one authority domain is not evidence for another domain.

## Concurrency and single-writer rule

At any instant, each mutable World aggregate has one admitted writer epoch. Cutover requires database-enforced writer fencing, stable aggregate/event IDs, immutable idempotency bindings, and post-wait replay rechecks. World and legacy CEX mutation must not both remain enabled. Nakama may sequence multiple player intents, but World state mutation still occurs through the one fenced World-domain writer.

## Persistence and projections

World-domain storage is the system of record for World aggregates after cutover. CEX and Nakama may retain source-versioned read projections or caches, but they may not locally mutate or reinterpret World state. Projection lag, source revision, and rebuild behavior must be visible. Exact migration reconciliation binds counts, stable IDs, canonical bytes/hashes, owners, privileges, and database lineage.

## Failure behavior

- If Nakama cannot prove admission/order/generation, it does not call World as canonical traffic.
- If World cannot prove its writer/database fence or expected aggregate revision, it rejects without mutation.
- If a World transition response is lost, the caller recovers by immutable request/event identity rather than resubmitting altered bytes.
- If signer/CEX outcome is uncertain, settlement uses exact lookup-before-submit outside mutable World transactions.
- If component revisions diverge, Integration blocks promotion.
- If cutover or reconciliation is ambiguous, both public promotion and the new writer remain disabled until the owner resolves the exact evidence.

## Cutover sequence

1. freeze exact World/Nakama/CEX/Chain/Integration component revisions and contracts;
2. install and verify the World-domain store/catalog with admission disabled;
3. copy and reconcile World aggregates/events by stable ID and canonical hash;
4. run CEX-old-path and World-new-path shadow comparison without dual mutation;
5. stop legacy World mutation and acquire the new writer epoch;
6. route only the approved internal adapter/Nakama path;
7. drain or quarantine in-flight legacy work and reconcile settlement intents/receipts;
8. retain bounded rollback evidence and independently approve promotion;
9. remove embedded CEX World mutation and disable the compatibility route after the retirement window.

## Rollback

Rollback is an explicit writer-epoch transition, not simultaneous dual-write. Stop new admission, drain/quarantine in-flight work, reconcile exact aggregate/event/settlement identities, fence the current writer, restore the reviewed prior store/routing state, and prove that the superseded writer cannot publish. A rollback does not move canonical online authority away from Nakama or custody away from CEX.

## Consequences

The seven-crate World-domain authority workspace can be reviewed as a coherent internal domain service, while the existing game server remains a compatibility enclave and Nakama remains canonical online authority. API names, documentation, tests, and deployment manifests must use “World-domain” or “compatibility” where ambiguity exists. Any proposal to move a forbidden responsibility into World requires a new ADR and approval from every affected repository owner.

## Required verification

Acceptance requires module contracts, strict protocol schemas/vectors, exact source and prospective-merge tests, PostgreSQL catalog/fencing/replay hostile tests, cross-repository shadow comparison, no-dual-writer reconciliation, rollback rehearsal, protected repository controls, and fresh independent review. This ADR alone grants no deployment, public-online, settlement, release, or production authorization.
