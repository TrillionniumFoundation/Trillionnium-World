---
status: current-candidate
owner: trillionnium-world-release
applies_to_plan: trillionnium-world-development-2026-08-29-v4
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Release Gate Matrix v2

## Status vocabulary

- `planned` — no reviewed implementation.
- `implemented` — source exists and local invariants are represented.
- `validated` — exact-head evidence of the required class is accepted.
- `release_eligible` — every dependency, review and operational condition for the stated denominator is green.
- `blocked` — one or more required rows are open.
- `no_go` / `disabled` — explicit promotion prohibition.

## Evidence precedence

```text
source_static < unit < database_black_box < single_host_runtime
< cross_repository_integration < cross_host < public_network
< human / custody_security / commercial_legal
```

The ordering is not transitive proof. For example, a public-network run does not automatically satisfy human or custody rows.

## Gate matrix

| Gate | Source state | Required evidence | Current decision |
| --- | --- | --- | --- |
| G0 repository truth/governance | Plan v4 candidate | docs/static + observed GitHub ruleset + review | blocked: server controls unverified |
| G1 deterministic transition contract | integration in this branch | Rust + independent conformance + exact-head CI | blocked until strict parser/vectors and CI pass |
| G2 settlement source/runtime | durable outbox candidate | Rust/PostgreSQL + deployed signer/CEX/process faults | blocked |
| G3 Nakama canonical authority | external owner | cross-repo shadow, drain, cutover, rollback, component lock | blocked upstream |
| G4 portable native software alpha | partial | Linux/Windows/macOS package/install/runtime evidence | blocked |
| G5 commercial single-player | technical alpha | independent human/accessibility/support/privacy/legal | blocked |
| G6 trusted CEX settlement | source candidate | custody + deployed ambiguity/PITR/retention + review | blocked |
| G7 closed online Nakama | not cut over | sole Nakama authority + exact Integration lock | blocked |
| G8 public online | not eligible | G7 + cross-host/public edge/capacity/endurance/moderation/human | **NO-GO** |
| G9 public player market | disabled | G8 + custody/fraud/dispute/commercial/privacy/legal/governance | **disabled** |

## G0 — Repository truth and governance

Required:

- canonical README/current plan/docs index;
- documentation path/link/status validation;
- no validation workflow with repository write permissions;
- CODEOWNERS with valid independent principal/team;
- server-side ruleset protecting `main`;
- required exact check contexts, stale-review dismissal and conversation resolution;
- direct/force push and deletion disabled;
- bypass absent or break-glass audited.

Source files may close only the first three rows. GitHub server settings require observed API evidence.

## G1 — Deterministic transition contract

Required:

- strict canonical parser/encoder;
- schema and stable errors;
- positive and adversarial negative vectors;
- resource budgets;
- Rust and independent implementation byte/hash equality;
- no authority credential or external I/O surface;
- exact-head workflow artifact and independent review.

Promotion effect: deterministic contract `MODULE_CLOSED_CANDIDATE`, not online authority.

## G2 — Settlement source/runtime

Required:

- capture commit before remote visibility;
- stable remote request identity;
- live lease fencing on every mutation;
- lookup-before-submit and malformed-success/conflict recovery;
- poison quarantine and unrelated-key progress;
- SIGINT/SIGTERM bounded drain;
- exact apply CAS and checked revision;
- operator replay/retention/alert controls;
- PostgreSQL tests and deployed fault matrix;
- backup/PITR/old-primary evidence;
- exact artifacts and reviewer approval.

Promotion effect: settlement module candidate only. Trusted settlement remains G6.

## G3/G7 — Nakama authority and closed online

Required:

- exact World contract consumed by Nakama;
- zero unexplained shadow divergence;
- Nakama-only admission/order/idempotency/recovery/root/signature;
- active World-local drain or separately proven takeover;
- cutover, rollback and disablement rehearsal;
- exact Integration component lock.

World-local compatibility evidence cannot satisfy this gate.

## G4 — Native software alpha

Required per target:

- signed/checksummed package;
- clean install, launch, save/load, upgrade, rollback and uninstall;
- GPU/driver/input/display matrix;
- crash and recovery evidence;
- dependency/licence/SBOM/provenance;
- exact binary/source/toolchain binding.

Linux evidence does not imply Windows/macOS.

## G5 — Commercial single-player

Required:

- three five-second observers;
- non-developer unguided vertical slice;
- accessibility/input matrix;
- support and incident paths;
- privacy/data-retention review;
- licence/content ownership;
- commercial/legal approval.

Automated screenshots or tests cannot satisfy human rows.

## G6 — Trusted CEX settlement

Required:

- G2 source/runtime validated;
- CEX owner implementation and exact build lock;
- signer/CEX response-loss, malformed-success and conflict matrix;
- credential separation, rotation and revocation;
- KMS/HSM or explicitly approved custody profile;
- backup/PITR/receipt retention and restore;
- fraud/dispute/chargeback boundaries for enabled intent classes;
- independent security/release review.

No public wallet/market credit follows automatically.

## G8 — Public online

Required:

- G7 closed online;
- replicated multi-host durability and failover;
- public TLS/mTLS edge, WAF/DDoS and abuse controls;
- regional capacity and load tests;
- clean 24-hour endurance in isolated resource domain;
- monitoring/on-call/incident and staffed moderation;
- multiplayer human comprehension/recovery;
- privacy/security approvals.

Any missing row keeps the decision NO-GO.

## G9 — Public player market

Required:

- G8 public online;
- custody and listing ownership;
- fraud and market-abuse controls;
- dispute/refund/chargeback/support operations;
- economic limits and emergency disablement;
- privacy, commercial, legal and governance approval.

Enablement is an explicit external owner decision and cannot be toggled by game source or a generated status artifact.

## Exact evidence binding

Each green row cites an evidence record conforming to `trnm-world-evidence-record-v1.md`. The release reviewer must verify:

- exact commit/tree and clean source;
- exact binary/image/package and source manifest;
- toolchain/dependency/component lock;
- environment/topology and timestamps;
- metrics/thresholds and raw artifact hashes;
- limitations/expiry;
- independent reviewer decision.

## Current decision

As of 2026-08-29:

- deterministic and settlement source work is active but not exact-head validated;
- repository server governance is unverified;
- Nakama/Integration, deployed, human, public, custody and commercial rows are open;
- public online remains **NO-GO**;
- public player market remains **disabled**.