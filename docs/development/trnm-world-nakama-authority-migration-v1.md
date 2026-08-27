# World to Nakama Authority Migration v1

Status: **current implementation design**  
Owners: World, Nakama and Integration maintainers  
Decision: ADR-0001  
Plan: `trnm-world-development-plan-v3.md`

## 1. Objective

Move externally authoritative online match lifecycle from the migration-era World-local authority to Nakama without a flag day and without a period in which both systems can order, sign, publish or settle the same match.

World remains authoritative for deterministic game-domain rules, simulation, authored content and unsigned outcome material. Nakama becomes authoritative for online admission, participant framing, global order, idempotency, restart recovery, canonical roots and signed completion evidence.

## 2. Authority profiles

Every match records one immutable authority profile at creation.

| Profile | May create new matches | External authority | Settlement/completion credit |
| --- | --- | --- | --- |
| `world_legacy_local_alpha_v1` | Initially yes, then drain-only | World-local server | Local-alpha only; no target public credit |
| `nakama_shadow_verify_v1` | No externally credited matches | Nakama externally; World unsigned verifier only | Nakama path only |
| `nakama_closed_alpha_v1` | Yes for allowlisted closed alpha | Nakama | Versioned Nakama completion evidence |
| `nakama_target_v1` | Yes after all target gates | Nakama | Target evidence/CEX/Chain adapters |
| `quarantined_v1` | No | None | No new value or public result |

An authority profile is not inferred from service availability, build ID or routing. It is durably selected before the first admitted command and cannot change in place.

## 3. Immutable match identity and routing

The match envelope contains:

- globally unique match ID;
- authority profile and authority contract version;
- World ruleset/content digest;
- participant authorization reference owned by Nakama for target profiles;
- creation timestamp and environment;
- exact World/Nakama/Integration component-lock identifier;
- settlement policy version;
- legacy marker when applicable.

Routing rules:

- clients ask the control plane for an authority endpoint; they do not select World versus Nakama locally;
- target-profile matches never fall back to World after Nakama rejection or outage;
- legacy matches remain on their original profile until terminal, failed-closed or quarantined;
- a missing/unknown profile fails closed;
- retries retain the original match ID and authority profile;
- rollback of a deployment may route **new** matches to an earlier approved Nakama version, never to a simultaneous World authority.

## 4. World deterministic runtime contract

World publishes a Bevy-free versioned contract with:

- ruleset/content digest;
- canonical initial game-domain state input;
- canonical ordered command payload supplied by Nakama;
- deterministic transition/output;
- snapshot/outcome hash;
- unsigned game-domain outcome facts;
- replay-domain material and bounded archive hints;
- canonical serialization and resource limits;
- golden vectors and negative fixtures.

The contract rejects or ignores no authority field silently. The following fields are forbidden at the World boundary:

- authoritative participant roster or role claims;
- global event/command sequence ownership;
- command idempotency ownership;
- canonical event, roster or archive roots;
- completion signature or authority key ID;
- Chain finality or inclusion proof;
- direct wallet mutation.

World may echo opaque correlation IDs but must not reinterpret them as its authority.

## 5. Migration phases

### M0 — scope freeze and inventory

- enforce ADR-0001 in World CI;
- inventory World endpoints, database tables, journal files, credentials, clients and CEX bindings;
- classify current active matches and evidence by legacy authority profile;
- freeze new authority features in World;
- record retirement owners and retention requirements.

Exit: no unowned authority surface; boundary-negative fixtures pass.

### M1 — deterministic runtime extraction

- extract pure request/transition/result types;
- freeze canonical hashing and golden vectors;
- run legacy World authority through the extracted runtime without changing external behavior;
- prove existing replay/result hashes or version their intentional change;
- publish schema and compatibility contract.

Rollback: revert extraction while retaining the previous legacy profile. No Nakama target match exists yet.

Exit: World local behavior and extracted runtime agree on the accepted vector corpus.

### M2 — independent consumers and component lock

- Integration independently verifies World vectors;
- Nakama consumes the exact runtime contract;
- component lock binds exact World commit/tree/schema/vector digest and Nakama consumer revision;
- malformed, over-limit and authority-leaking payloads fail closed;
- compatibility matrix names supported contract windows and retirement dates.

Rollback: stop consumer validation; no externally authoritative Nakama match is created.

Exit: independent implementations produce identical canonical results.

### M3 — Nakama shadow verification

- selected legacy matches are duplicated into an **unsigned, non-settling** World/Nakama comparison harness;
- only the original legacy World path is externally authoritative for those pre-existing matches;
- comparison output cannot publish completion evidence, mutate CEX or submit Chain ingress;
- mismatch quarantines the comparison result and opens an incident; it never switches authority mid-match.

Alternatively, for a new Nakama-authoritative test match, Nakama is the sole external authority and World-local authority may only execute an isolated unsigned verifier instance. The two shadow directions must never be mixed.

Rollback: disable shadow work. Authoritative path is unchanged.

Exit: sustained vector/live comparison passes within exact deterministic rules, with mismatch triage and resource bounds.

### M4 — Nakama closed-alpha admission and order

- new allowlisted matches use `nakama_closed_alpha_v1`;
- Nakama owns participant authorization, roles, command IDs, global order and retry results;
- World runtime consumes already ordered commands;
- target clients reconnect to Nakama only;
- legacy clients/matches remain explicitly routed to the legacy profile and receive no target release credit.

Rollback: stop creation of new closed-alpha matches and continue/terminate existing Nakama matches through Nakama. Do not send them to World.

Exit: admission, duplicate command, disconnect/reconnect and process-restart tests pass.

### M5 — recovery, archive roots and signed completion

- Nakama persists sufficient authority state for restart recovery;
- Nakama owns canonical event/roster/archive roots;
- Nakama constructs and signs versioned completion evidence;
- World unsigned outcome hash is one bound input, not the completion authority;
- Integration verifies exact root/signature vectors and component lock;
- private-key custody, rotation and revocation are documented and attested.

Rollback: stop new matches on the affected Nakama version; existing matches complete or quarantine under Nakama. A signature downgrade cannot restore World signing.

Exit: crash windows, archive reconstruction, key rotation and tamper tests pass.

### M6 — CEX and Chain adapter migration

- CEX accepts target rewards only when bound to valid Nakama completion evidence and the World game-domain outcome contract;
- Chain ingress consumes the published target evidence contract and owns finality/inclusion;
- World does not call target Chain mutation directly;
- idempotency binds match/completion/intent identity across adapters;
- local legacy settlement is separately labeled and cannot be confused with target evidence.

Rollback: disable new value/Chain submission and keep completed matches pending. Do not reissue under a different authority identity.

Exit: ambiguous commit, duplicate submission, invalid signature/root and finality-boundary tests pass.

### M7 — drain and retire World-local authority

- disable creation of new legacy matches;
- enumerate all remaining waiting/running/terminal/pending legacy matches;
- each match receives terminal completion, explicit fail-close or quarantine disposition;
- revoke World-local online authority credentials;
- remove client routing and service admission to legacy endpoints;
- preserve database/journal/evidence under approved retention and verification tooling;
- remove or permanently compile/runtime-gate legacy authority after retention obligations are met.

Exit: no supported client can create or reconnect a new active legacy match; no active legacy credential remains; historical verification grants no current authority.

## 6. No-dual-authority protocol

For every match, the following tuple is immutable:

```text
(match_id, authority_profile, authority_contract_version, component_lock_id)
```

The system fails closed when:

- two active authority leases claim the tuple;
- a command receipt comes from a different profile/version;
- a completion signature does not bind the tuple;
- CEX/Chain adapter evidence uses a different profile or component lock;
- a client attempts cross-profile reconnect;
- rollback would make a second authority reachable.

A shadow verifier has a different non-authoritative execution identity and cannot produce a valid external authority receipt.

## 7. Data and evidence disposition

Legacy data classes:

- match/campaign rows;
- command events and replay frames;
- local hot/cold publication journal evidence;
- terminal ACK/abandonment markers;
- progression/rating events;
- CEX intent/receipt bindings;
- operational/moderation records.

For each class document:

- system of record and schema version;
- retention duration and legal/product rationale;
- encryption/access principal;
- export and verification format;
- deletion/compaction preconditions;
- whether it can grant current release, settlement or ranking credit;
- PITR/rollback detection behavior;
- migration owner and completion evidence.

Historical legacy evidence remains verifiable but is never silently re-signed as target Nakama evidence.

## 8. Credential migration

| Credential | Legacy owner | Target owner/action |
| --- | --- | --- |
| World local match authority token | World | Revoke after drain |
| Nakama match signing key | None in World | Nakama/KMS-HSM custody only |
| World runtime service identity | World | Narrow invoke/contract scope |
| CEX game intent principal | World legacy | Separate legacy and target adapter audiences |
| Moderator credential | Operations | Independent from game/signer principals |
| Chain ingress principal | Target adapter | Never placed in World runtime |

No shared root secret may derive multiple target roles. Rotation is rehearsed before public credit.

## 9. Failure and rollback matrix

| Event | Required behavior |
| --- | --- |
| Nakama unavailable before match creation | Reject/queue target creation; no World fallback |
| Nakama dies during target match | Recover or quarantine under Nakama profile |
| World runtime unavailable | Nakama keeps authority, retries bounded runtime invocation or quarantines |
| World/Nakama deterministic mismatch | Stop publication/settlement for affected target match; incident and quarantine |
| Completion signing unavailable | Match may be terminal internally but remains unpublished/unsettled |
| CEX unavailable | Completion remains durable; exact intent retries later |
| Chain unavailable/ambiguous | Preserve exact submission identity; query/retry under Chain contract |
| Deployment rollback | Stop new affected matches; existing matches stay with original authority profile |
| Legacy server failure during drain | Same-host legacy recovery or explicit fail-close/quarantine; never Nakama takeover of that match |
| Credential compromise | Revoke affected profile/version, stop new matches, quarantine unverifiable evidence |

## 10. Required tests

- canonical World vectors independently verified by Integration and Nakama;
- authority-field rejection and resource-limit tests;
- match-profile immutability and cross-profile reconnect rejection;
- duplicate command and global-order restart recovery;
- shadow verifier cannot sign, settle, publish or influence routing;
- Nakama crash at every durable boundary;
- archive root reconstruction and tamper detection;
- completion signature key rotation/revocation;
- CEX exact-once and Chain ambiguous submission;
- deployment rollback with active matches;
- complete legacy drain inventory and credential revocation test.

## 11. Observability

Metrics/logs are partitioned by authority profile and contract version:

- match creation/admission/rejection;
- active/waiting/terminal/quarantined counts;
- command duplicate/order/recovery outcomes;
- runtime invocation latency and deterministic mismatch;
- completion publication/signing state;
- CEX/Chain pending age and retry classification;
- remaining legacy matches and credential status.

No dashboard may aggregate legacy and target authority into one unlabeled success percentage.

## 12. PR sequence

1. World runtime types/hash/schema/vectors.
2. Integration reference verifier and component-lock schema.
3. Nakama consumer and negative authority-field tests.
4. Shadow harness with non-authority proof.
5. Nakama admission/roster.
6. Nakama command ordering/idempotency.
7. Nakama restart recovery/archive.
8. Completion roots/signing and key custody.
9. CEX target-evidence adapter.
10. Chain ingress adapter.
11. Client routing for new target matches.
12. Legacy creation disable, drain/quarantine, credential revocation and endpoint retirement.

Each PR changes one authority capability and names its rollback boundary.
