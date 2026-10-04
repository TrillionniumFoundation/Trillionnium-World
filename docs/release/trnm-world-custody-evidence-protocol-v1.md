---
status: current-candidate
owner: cex-security-and-custody
applies_to_gap: WORLD-V5-019
last_reviewed: 2026-09-09
review_due: 2026-10-09
production_authorization: not_granted
---

# Trillionnium World custody and key-control evidence protocol v1

## Scope and authority

CEX and its accountable security/custody owners qualify wallet, ledger, signing-key and external settlement custody. World may define game economic intents and verify bounded receipts, but it cannot approve a key store, create a custody exception or infer ledger safety from local campaign state.

This protocol records evidence for one exact CEX artifact, World settlement contract, workload identity configuration, key-policy revision and deployment topology. No private key, recovery secret, seed phrase, bearer token or usable credential may be copied into the evidence package.

## Required identities

The evidence record binds:

- exact CEX commit, tree, deployable artifact, SBOM and provenance digests;
- exact World economy/settlement contract and vector digests;
- Integration component-lock candidate;
- cloud/account/project/region identifiers at a non-secret level;
- KMS/HSM service and key-policy revision;
- workload identities, audiences, roles and deployment identities;
- ledger/database schema and migration identities;
- monitoring, audit-log and incident-policy revisions.

Mutable aliases such as `latest`, an environment name alone or a branch name cannot identify the qualified component.

## Custody boundary

An accountable diagram and machine policy export must show:

1. where private key material is generated and retained;
2. whether export is technically prohibited;
3. which workloads may request signing/decryption and for which audience/purpose;
4. which humans may administer policy, deploy workloads, approve value movement or review logs;
5. how separation of duties and dual control are enforced where required;
6. how public keys, key versions and receipt-verification material are distributed;
7. how backup, disaster recovery and destruction work without exposing private material;
8. how compromised identities are revoked and isolated.

A software environment variable containing a long-lived private key is not an acceptable production custody boundary unless an independent security authority explicitly accepts the exact scope and compensating controls; such acceptance remains a release blocker, not an automatic pass.

## Key lifecycle evidence

Execute and retain non-secret evidence for:

- controlled key creation with algorithm, purpose and policy;
- activation and workload-audience restriction;
- test signing/decryption using synthetic non-value payloads;
- rotation to a new version while old valid receipts remain verifiable;
- rejection of stale, disabled, wrong-audience and wrong-purpose requests;
- revocation/disablement propagation and alerting;
- emergency suspension and recovery;
- destruction or scheduled destruction under dual control where applicable;
- public-key/version inventory and rollback behavior.

The test must prove that the World game server, public client and unrelated services cannot directly obtain or use custody credentials.

## Settlement and fraud-control matrix

Using synthetic or approved non-production value, test:

- exact idempotency identity and altered-retry rejection;
- response loss before and after remote commitment;
- lookup-before-submit and exact receipt recovery;
- duplicate concurrent requests;
- stale worker/lease/fence attempts;
- crossed account, tenant, currency, amount, backend and contract version;
- malformed, unsigned, wrong-key and expired receipts;
- ledger conservation and independent reconciliation;
- manual review/hold/release and fraud thresholds;
- chargeback/reversal or equivalent exceptional flow where the product permits it;
- operator override authorization, audit and expiry;
- incident isolation without blocking unrelated accounts.

Remote success and World campaign application remain separate durable states. Evidence must show that neither side manufactures completion from timeout, elapsed time or partial local state.

## Compromised-role drills

At minimum inject:

- leaked or revoked workload credential;
- unauthorized administrator or deployment identity;
- signer unavailable during an ambiguous settlement;
- audit-log delivery failure;
- stale key version after rotation;
- malicious altered retry under a valid prior identity;
- attempted use from the wrong environment, audience or network boundary.

Predeclared expected outcomes include fail-closed behavior, paging/alert delivery, credential revocation, bounded recovery and no duplicate value. Every drill records start/end time, actors, exact policy state, synthetic payload identity, logs and cleanup.

## Audit, monitoring and retention

Retain immutable or retention-controlled records for key administration, signing/decryption requests, policy changes, deployment identity changes, settlement decisions, reconciliation, alerts and incident actions. Evidence identifies log destinations and retention policy without embedding secrets or full personal financial data.

Monitoring must detect unusual request rate, wrong audience, disabled key use, repeated ambiguous outcomes, reconciliation mismatch, stale worker attempts and administrative policy changes. Alert delivery and escalation are tested, not assumed from configured rules.

## Independent review

The independent security reviewer must not be the sole executor, key-policy administrator, application author or final commercial approver. The review covers architecture, machine policy exports, drills, residual risks, compensating controls and evidence retention.

A repository source review, local mock signer or CI unit test cannot satisfy custody evidence. The final record is validated structurally with:

```bash
python3 scripts/check-trnm-world-external-evidence.py path/to/custody-record.json
```

## Acceptance and invalidation

PASS requires all applicable lifecycle, authorization, settlement, fraud, reconciliation and compromised-role criteria; zero unexplained duplicate value; no secret material in artifacts; independent approval; exact-object binding; and an unexpired record.

Changes to CEX artifact, key policy, key version policy, workload identity, audience, deployment topology, ledger schema, settlement contract or material monitoring/incident controls invalidate the affected qualification.

This protocol does not itself grant commercial or production authorization. A final accountable decision may rely on an accepted custody record, but cannot replace it.
