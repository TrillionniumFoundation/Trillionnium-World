---
status: current-candidate
owner: trillionnium-world-operations
last_reviewed: 2026-09-08
review_due: 2026-10-08
applies_to: trillionnium-world-plan-v4-v6-convergence
---

# Trillionnium World Operations Manual

## Scope and authority

This manual covers only the Trillionnium World game-product repository: the native client, deterministic RPG/RTS domain, the World-local compatibility server, settlement capture/worker/apply components, release packaging, and the bounded World-domain authority migration workspace. It does not operate Chain consensus, PoUW/PoCO workers, CEX custody, Nakama canonical online ordering, or Integration release locks.

World owns deterministic game-domain behavior and unsigned outcome material. Nakama owns canonical online admission, total ordering, idempotency, restart recovery, archive roots, and `MatchCompletedV1` signing. CEX owns wallet/ledger settlement and custody. Chain owns ingress/finality. Integration owns the exact multi-repository release lock. Operations must stop rather than cross one of these boundaries.

## Truth hierarchy

Before any operation, read and bind the exact commit/tree named by:

1. `PROJECT_BOUNDARY.md` and `PROJECT_BOUNDARY.json`;
2. `CURRENT_PLAN.md`;
3. `docs/catalog.json` and `docs/README.md`;
4. the applicable ADR, protocol, database, security, and runbook documents;
5. `docs/status/CURRENT.md` and the explicitly selected machine execution snapshot.

A status page, local log, workflow definition, or older artifact cannot promote a later source object. Missing, stale, skipped, cancelled, or identity-unbound evidence is a blocker.

## Supported validation entrypoints

Run from the repository root on a lane-compliant branch:

```bash
bash scripts/project-preflight.sh --audit
./scripts/check_trnm_game_product.sh
./scripts/check_trnm_authority_boundary.sh
./scripts/check_trnm_runtime_configuration.sh
./scripts/check_trnm_settlement_outbox_contract.sh
./scripts/check_trnm_settlement_transaction_boundary.sh
python3 scripts/check-trnm-world-documentation.py
python3 scripts/check-trnm-world-module-documentation.py
python3 scripts/check-trnm-world-detailed-documentation.py
python3 scripts/check-trnm-world-transition-conformance.py
```

The exact plan and workstream may require additional PostgreSQL, packaging, fault, supply-chain, prospective-merge, or cross-repository checks. A reduced command set may be used for diagnosis, but never for release credit.

## Runtime roles and process boundaries

The supported service roles are deliberately separate:

- `trnm-game-server`: World-local compatibility authority and game-domain persistence. It must not hold CEX/signer network I/O under mutable database row locks.
- `trnm-settlement-worker`: claims durable jobs, performs transaction-free remote execution, stores verified results, and applies them under a live exact lease.
- `trnm-entitlement-signer`: owns only the signing boundary and its private key material. The game server and moderator role must not read that key.
- moderator/operator tooling: bounded, audited recovery and policy operations only; it must not bypass immutable intent, lease, receipt, or campaign fences.
- `trnm-first-contact`: untrusted native client. Its save and command journals provide local continuity, not canonical online or wallet authority.

The target canonical online runtime is Nakama. World-local online endpoints remain a compatibility laboratory and migration/rollback surface.

## Configuration and secret ownership

Use the repository templates under `config/` and the role-separated systemd units under `deploy/systemd/`. Production must use an explicit verified release selector and dedicated service identities. Do not source environment from a sibling checkout, encode personal home directories, share moderator/game/signer credentials, or enable development binary fallback in production.

At minimum, bind and record:

- exact release artifact and SHA-256;
- World commit and tree;
- protocol/build/ruleset/content identities;
- PostgreSQL server version, system identifier, timeline, and migration checksums;
- service identity, audience, secret version, and expiry without exposing secret values;
- CEX, signer, Nakama, Chain, and Integration component revisions where applicable.

Secret issuance, rotation, revocation, KMS/HSM, workload identity, and break-glass approval remain independently reviewable evidence classes.

## Startup sequence

A production-like startup must be fail-closed and ordered:

1. verify the release selector, artifact hashes, configuration ownership/mode, and role identity;
2. establish PostgreSQL connectivity with the intended role;
3. acquire the required migration/host/fleet fences and verify the complete catalog contract;
4. reconcile hot/cold journals, database summary, terminal witnesses, and poison state;
5. start signer and settlement worker with distinct audiences;
6. start the compatibility server with admission disabled until readiness is true;
7. expose readiness only after migrations, fences, journals, repository state, and required dependencies are healthy;
8. admit traffic only through the approved routing owner.

A partially applied migration, unknown database lineage, missing journal witness, stale process, unsupported protocol, or uncertain remote result keeps readiness false.

## Readiness and observability

Readiness is an authorization decision, not merely a process-liveness check. It must fail closed on migration drift, lost host or lease fencing, poisoned durable state, unsupported protocol/build pairs, inaccessible required storage, or settlement schema mismatch.

The minimum observable set includes:

- active/initializing/terminalizing match counts and ages;
- command queue depth, persistence latency, rejected-command reasons, and reconnect gaps;
- settlement pending, leased, retryable, remote-succeeded, pending-apply, applied, dead-letter, and quarantined counts;
- oldest eligible/pending-apply age, attempts, lease expiry, and stale-owner rejection;
- PostgreSQL pool saturation, query latency, lock waits, migration identity, timeline, and backup age;
- journal corruption/fence/readiness transitions;
- package/build/ruleset/content and component-lock identity.

Use `docs/operations/trnm-world-observability-slo-v1.md`, `docs/operations/trnm-game-server-runtime-configuration.md`, and the settlement runbooks for exact signals and escalation semantics.

## Settlement lifecycle

The only permitted lifecycle is:

```text
terminal game state
  -> short capture transaction
  -> durable settlement job
  -> committed claim/lease
  -> transaction-free signer/CEX lookup or submit
  -> durable remote result
  -> short fenced campaign apply transaction
  -> immutable evidence and reconciliation
```

Lookup-before-submit and immutable request/intent identities are mandatory after response loss or ambiguity. A stale or expired worker may not complete, retry, dead-letter, or apply. Exact duplicates may converge idempotently; altered duplicates, receipt/hash/account/audience mismatches, and unsupported versions fail closed. One poisoned scope is quarantined without blocking unrelated accounts.

## Shutdown and drain

On SIGTERM or an authorized drain:

1. stop new public/player/moderator/capture admission;
2. mark the instance draining and stop creating new actors/jobs;
3. bound in-flight HTTP, WebSocket, command persistence, and remote settlement work;
4. flush or safely abandon private authority state according to the journal contract;
5. preserve live lease and remote-attempt evidence rather than fabricating completion;
6. stop worker and signer after their bounded joins;
7. emit a terminal summary bound to the exact process, instance, database, and artifact identities.

SIGKILL and cancellation recovery must be demonstrated by fault tests; a graceful-path log is not proof of crash convergence.

## Incident classes

Treat these as immediate admission blockers:

- database fence, timeline, system-identifier, or migration-catalog disagreement;
- duplicate active writer epoch or old-primary write capability;
- command, transition, snapshot, replay, receipt, or journal hash mismatch;
- uncertain signer/CEX outcome without exact lookup recovery;
- stale lease or stale generation attempting mutation;
- protocol/build/ruleset/content mismatch;
- leaked or cross-audience credential;
- public routing to the compatibility enclave outside the approved plan;
- missing required evidence or protection context.

Quarantine the smallest exact scope, retain immutable evidence, and escalate to the accountable owner. Do not hand-edit leases, migration ledgers, terminal acknowledgements, or settlement success.

## Backup, PITR, and failover

Backup/PITR acceptance requires more than row counts. After restore or failover:

1. prove the selected backup/WAL lineage and retention policy;
2. verify database system identifier, timeline, postmaster identity, migration ledger, catalog fingerprint, owners, privileges, functions, triggers, constraints, and indexes;
3. fence the old primary and all old processes before admission;
4. reconcile journals, terminal witnesses, captures, remote receipts, pending applies, and operator evidence;
5. allow leases to expire/take over through the normal protocol;
6. run the exact smoke and hostile recovery matrix on immutable binaries;
7. obtain independent approval before promotion.

See `docs/operations/trnm-world-backup-pitr-v1.md` and `docs/database/trnm-world-postgres-contract-v1.md`.

## Rollback and authority cutover

Rollback must preserve one canonical online owner and one mutable World-domain writer. The cutover sequence must bind exact World, Nakama, CEX, Chain, and Integration revisions; stop old admission; drain or explicitly quarantine active work; reconcile stable IDs/counts/hashes; prove no dual writer; switch routing under a reviewed fence; and retain a bounded reverse path.

The separate World-domain authority workspace may replace World business-state mutation formerly embedded in CEX. It does not replace Nakama canonical online ordering or CEX ledger custody. `docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md` is the binding interpretation for that distinction.

## Release and evidence capture

For each accepted claim, retain:

- claim and evidence-class ID;
- exact source head/tree, prospective merge parents/object, binaries, toolchain, workflow/action and dependency identities;
- environment and topology;
- start/end UTC timestamps;
- commands, thresholds, injected faults, exit codes, job/step conclusions, and raw logs;
- artifact names, byte sizes, and cryptographic digests;
- limitations, expiry/review date, and independent reviewer.

Repository-authored text cannot satisfy cross-host, public-network, custody/security, human, privacy/legal, commercial, support, or final go/no-go evidence. Production authorization remains `not_granted` until those accountable authorities provide real records.

## Forbidden operations

This repository must not be used to:

- operate or validate Chain consensus/BFT/PoUW/PoCO worker flows;
- load a Nakama completion private key or sign canonical `MatchCompletedV1`;
- mutate CEX wallet/ledger state locally;
- bypass review, required checks, conversation resolution, branch protection, or no-bypass rules;
- synthesize commit statuses or reinterpret CEX/other-repository runs as World-native checks;
- force-push, directly develop on `main`, self-approve, tag, release, or deploy an unqualified candidate.
