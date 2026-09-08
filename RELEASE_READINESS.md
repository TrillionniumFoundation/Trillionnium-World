---
status: current-candidate
owner: trillionnium-world-release
last_reviewed: 2026-09-08
review_due: 2026-09-15
production_authorization: not_granted
---

# Trillionnium World Release Readiness

## Current decision

**Not release-ready. Public online and the public player market remain NO-GO / disabled.**

The current Plan V4/closure candidate contains substantial source and documentation work, but release eligibility requires independent evidence on the final exact head and its exact cross-repository component lock. A green local command, source document or historical artifact cannot supply missing hosted, deployed, human, custody or commercial evidence.

## Denominator matrix

| Denominator | Current state | Closure authority |
|---|---|---|
| World ordinary source | Candidate source present | World exact-head review and CI |
| Deterministic transition contract | Candidate implementation/vectors | World + Nakama conformance |
| Native software alpha | Technical alpha | World release reviewers |
| Commercial single-player | Open | Product, accessibility, support, privacy and legal |
| Trusted CEX settlement | Open / upstream blocked | CEX + security + Integration |
| Nakama canonical online | Open / upstream blocked | Nakama + Integration |
| Repository governance | Server evidence open | GitHub organization administrators |
| Public edge and cross-host recovery | Open | Operations + Integration |
| Human/accessibility validation | Open | Independent product/accessibility reviewers |
| Public player market | Disabled | Custody, fraud, finance, privacy, legal and governance authorities |

## Source-level facts

- The active game-product workspace has eight crates and excludes `trillionnium/crates/platform`.
- Correctness source must be ordinary reviewable source. Semantic `build.rs`/`src/lib.rs.in` generation is forbidden.
- The World-local server is a `world_legacy_local_alpha` compatibility enclave and is not the target canonical public authority.
- Settlement follows capture -> transaction-free remote execution -> fenced apply.
- Validation workflows use read-only repository permissions and cannot self-promote a candidate.

These are architecture/source facts only; they do not imply successful hosted execution or release eligibility.

## Required repository closure

Before the repository candidate can be considered independently validated:

1. obtain non-empty exact-head and prospective-merge workflow runs on the final unchanged head;
2. run the pinned Rust toolchain, PostgreSQL 16.4, transition, package, source-boundary, documentation and supply-chain gates;
3. apply/read back the current server-side `main` protection contract;
4. obtain fresh independent approval on the final tuple;
5. retain immutable logs/artifacts and verify their digests.

## Required system closure

Before trusted online or settlement promotion:

- Nakama consumes the exact World transition contract and reports zero unexplained shadow divergence;
- Nakama is the sole admission/order/idempotency/recovery/archive/completion authority;
- CEX selects and qualifies an immutable settlement revision with custody and ambiguity-recovery evidence;
- Integration pins World, Nakama, CEX and Chain exact identities and rehearses drain/cutover/rollback;
- cross-host failover, PITR/old-primary isolation, public edge, capacity and 24-hour endurance evidence pass.

## Required human and organizational closure

Automation cannot replace:

- consented non-developer usability and accessibility sessions;
- moderation/support/appeal and incident drills;
- privacy/data-retention approval;
- security and custody approval;
- licensing, terms, jurisdiction and commercial go/no-go decisions.

## No-credit rules

The following implications are invalid:

```text
source exists              != exact-head CI passed
CI passed                  != deployment passed
deployment passed          != custody approved
automated screenshots      != human validation
review requested           != independent approval
documented branch policy   != server-side enforcement
repository closure         != production authorization
```

## Promotion record

Any future GO decision must bind one exact commit/tree, toolchain, dependency lock, binaries, component lock, environment/topology, evidence hashes, limitations, expiry and accountable human approvals. Until then, `production_authorization=not_granted` remains binding.
