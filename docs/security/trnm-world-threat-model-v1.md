---
status: current
owner: trillionnium-world-security
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Threat Model v1

## 1. Protected assets

- player identity/session and account bindings;
- canonical online command order and match completion;
- deterministic state/replay/outcome integrity;
- campaign progression, inventory and local saves;
- economic intents, entitlements, receipts and wallet balances;
- signer private keys and service credentials;
- PostgreSQL/journal/backup lineage;
- release artifacts, component locks and evidence;
- moderation, reports and appeals;
- player privacy and human-test data.

## 2. Trust boundaries

1. Native client ↔ Nakama/CEX/public edge.
2. Nakama ↔ World deterministic transition interface.
3. World game server/settlement worker ↔ PostgreSQL.
4. Settlement worker ↔ isolated signer.
5. Settlement worker ↔ CEX ledger.
6. Services ↔ filesystem journals/config/secrets.
7. CI/build system ↔ source/ref/artifact registry.
8. Operators ↔ moderation/settlement replay/production controls.
9. Integration ↔ exact cross-repository artifacts.

## 3. Adversaries

- malicious or modified client;
- compromised player session/device;
- replaying network observer;
- compromised moderator/operator credential;
- compromised game-server or settlement-worker credential;
- compromised signer or CEX endpoint;
- stale process/old primary after failover;
- malicious dependency/build runner;
- insider with repository or production access;
- malformed/oversized content intended to exhaust CPU, memory, database or
  storage.

## 4. Primary abuse cases and controls

### Forged game result or replay

Controls: Nakama canonical admission/order/archive/signature; World deterministic
hashes; exact component lock; replay verification; no client-authored terminal
settlement facts.

### Duplicate command or settlement

Controls: immutable command/intent/remote identities; database idempotency;
lookup-before-submit; live lease generation; exact Campaign CAS; receipt
validation; duplicate tests.

### Remote success with lost/malformed response

Controls: classify as ambiguous/retryable; exact receipt lookup before any
resend; stable request identity; bounded attempts; operator-visible state.

### Stale worker/old primary writes

Controls: lease owner/generation/expiry; instance/host/timeline fencing;
advisory locks; old-primary isolation; PITR recovery checks.

### Poison item blocks the fleet

Controls: per-item error isolation, durable quarantine/dead letter, bounded
batch/concurrency and alerts; no global loop termination.

### Authority smuggling through opaque payload

Controls: allowlisted schemas, strict canonical parser, decoded recursive field
checks, payload budgets and negative vectors.

### Partial mutation on rejected command

Controls: candidate-copy/validate/commit, mutation journal where required,
state-hash property tests, replay/RNG/resource counter invariants.

### Credential confused-deputy or privilege crossing

Controls: distinct audiences/roles, short-lived credentials, mTLS/workload
identity, KMS/HSM, rotation/revocation, no derived shared root secret, audited
break glass.

### CI or supply-chain candidate substitution

Controls: protected refs, immutable action/toolchain revisions, read-only CI,
no self-fix/push/tag, signed provenance/SBOM, exact head/tree/artifact binding,
independent review.

### DoS/resource amplification

Controls: body/frame/depth/collection limits, rate limits, timeouts, bounded
queues/pools/tasks, compression limits, query-plan baselines, WAF/DDoS and
capacity evidence.

### Privacy leakage

Controls: data minimization, bounded logs, token/key redaction, consented human
evidence, retention/deletion policy and role-based access.

## 5. Security release gates

- threat-model review at each authority/settlement/protocol change;
- secret and dependency scanning;
- credential rotation/revocation drills;
- response-loss, process-kill, failover/PITR and stale-writer tests;
- public-edge penetration/abuse testing before public exposure;
- KMS/HSM evidence before custody claims;
- staffed incident, moderation, support and appeal drills before commercial
  launch.

## 6. Incident minimums

Every incident record includes component/release identity, detection time,
blast radius, authority/custody impact, containment, credential/key actions,
data/evidence preservation, player notification decision, recovery validation
and follow-up owner/date.
