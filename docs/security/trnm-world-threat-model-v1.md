---
status: current-candidate
owner: trillionnium-world-security
applies_to_plan: trillionnium-world-development-2026-08-29-v4
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Threat Model v1

## Security objectives

1. A client cannot create canonical online state, completion, balance or settlement evidence.
2. World and Nakama cannot both own the same canonical order/root/signature.
3. One economic intent produces at most one durable value effect.
4. Stale processes, leases and database primaries cannot publish after takeover.
5. Corrupt or malicious data fails closed without blocking unrelated players indefinitely.
6. Credentials remain least-privilege, audience-bound, rotatable and absent from evidence/logs.
7. Release claims cannot exceed their exact evidence class.

## Assets

- authored content/ruleset revisions;
- deterministic state, replay and outcome material;
- campaign saves and progression;
- Nakama sessions, canonical event log and completion keys;
- settlement intents, signer receipts and CEX receipts;
- wallet/ledger balances;
- PostgreSQL data, WAL/PITR and host journals;
- service credentials and signing keys;
- release artifacts, component locks and evidence manifests;
- player privacy, support and moderation records.

## Adversaries

- malicious or modified client;
- replaying player or bot;
- compromised player/session token;
- compromised game-server or settlement-worker credential;
- compromised moderator credential;
- compromised CI dependency/action;
- stale process or old database primary;
- malicious/buggy upstream service returning ambiguous responses;
- insider with repository or operator access;
- public-network attacker causing DoS, request amplification or response truncation;
- accidental operator error.

## Trust boundaries and threats

### Client -> Nakama/World

Threats:

- forged participant/controller identity;
- command sequence skipping/replay;
- oversized/deep/ambiguous payloads;
- forged BattleResult or settlement success;
- journal tampering or symlink attacks.

Controls:

- Nakama-owned admission and sequence;
- strict contract grammar and resource budgets;
- exact idempotency IDs;
- client artifacts treated as proposals/evidence only;
- mode/ownership/link checks and atomic local journal writes;
- server-side result derivation and receipt validation.

### Nakama -> World transition

Threats:

- crossed contract/ruleset/content revision;
- authority fields hidden in opaque JSON;
- canonicalization divergence;
- nondeterministic output;
- World service equivocation.

Controls:

- exact component lock;
- strict decoded canonical JSON;
- stable negative vectors;
- deterministic state/outcome hashes;
- shadow comparison and divergence quarantine;
- unsigned World output; Nakama retains canonical authority.

### World settlement worker -> signer/CEX

Threats:

- duplicate value after response loss;
- malformed 2xx treated as permanent failure;
- 409 race without lookup;
- stale worker result after lease expiry;
- intent substitution or hash mismatch;
- token leakage in logs/error bodies;
- slow remote call blocking all accounts.

Controls:

- stable remote request ID and immutable intent hash;
- lookup-before-submit;
- ambiguity classified retryable;
- live lease fencing on every mutation;
- exact receipt validation;
- bounded error evidence and redaction;
- per-key serialization plus unrelated-key bounded concurrency;
- shutdown/drain and quarantine.

### PostgreSQL and host journal

Threats:

- old primary or process continues writes;
- migration checksum drift;
- lock-order deadlock;
- capture/apply partial commit;
- evidence deletion/cascade;
- PITR restores inconsistent lineages;
- journal corruption or rollback.

Controls:

- advisory/ownership fencing and epochs;
- migration ledger checksums;
- documented global lock graph;
- atomic capture/apply transactions and exact CAS;
- restrictive FKs and append-only evidence;
- lineage/timeline checks after restore;
- hot/cold journal witnesses and corruption fail-close.

### CI/repository

Threats:

- workflow modifies tested source after review;
- floating action/toolchain compromise;
- direct main push or admin bypass;
- generated status overclaim;
- malicious dependency/vendor patch;
- unsigned release substitution.

Controls:

- validation workflows `contents: read`;
- immutable action/toolchain pins;
- server-side ruleset, review and required checks;
- negative status/evidence fixtures;
- SBOM/licence/advisory/source policy;
- signed provenance and verified release selectors.

### Operations/public edge

Threats:

- credential reuse/cross-role escalation;
- public DoS and resource amplification;
- missing moderation/support response;
- backup theft or restore failure;
- KMS/HSM misuse;
- privacy overcollection.

Controls required before public promotion:

- workload identity or mTLS;
- WAF/DDoS/rate/body/concurrency budgets;
- staffed escalation and appeal drills;
- encrypted backup/PITR with restore rehearsal;
- KMS/HSM policy, rotation and audit;
- data minimization, retention and deletion policy.

## Abuse cases

| Abuse case | Required fail-closed behavior |
| --- | --- |
| altered retry reuses command/intent ID | reject identity conflict |
| escaped forbidden authority key | reject after key decoding |
| signer/CEX committed but response lost | lookup exact receipt, no second durable effect |
| worker lease expires mid-request | stale worker cannot persist; next owner recovers by lookup |
| one corrupt campaign row | quarantine exact scope; unrelated accounts continue |
| CI attempts push/tag | workflow policy check fails |
| World claims `MatchCompletedV1` | authority boundary check fails |
| old primary returns after PITR/failover | ownership/timeline fence prevents publication |
| public-market flag changed in game source | gate/status validator rejects without external approvals |

## Security evidence

Source tests are necessary but insufficient. Promotion requires:

- SAST/dependency/secret scans;
- protocol fuzz and negative corpus;
- PostgreSQL/fault-injection matrix;
- credential rotation/revocation and role-compromise tests;
- backup/PITR/old-primary isolation;
- public-edge load/abuse tests;
- independent security review;
- incident-response rehearsal;
- exact evidence record and expiry.

## Residual risk posture

Until KMS/HSM, public-edge, cross-host, 24-hour, independent review and human/operations evidence close, the project remains technical alpha and public online/player markets remain NO-GO/disabled.