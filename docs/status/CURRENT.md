# Trillionnium World Current Status

> This view mirrors `docs/status/world-gates-v1.json`. Machine evidence, not this prose, controls promotion.

- As of: `2026-08-29`
- Source plan: `docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`
- Candidate branch: `fix/world-plan-gap-closure-v4`
- Public online: **NO-GO**
- Public player market: **disabled**

| Gate | Source status | Authority profile | Promotion blockers |
| --- | --- | --- | --- |
| Deterministic runtime alpha | **implemented** | World game domain | exact-head Rust/independent conformance, Nakama shadow, Integration lock |
| Native software alpha | **blocked** | World native client | exact-head workspace/package evidence, platform matrix, human/accessibility |
| Trusted CEX settlement | **implemented** | World compatibility enclave + CEX | exact-head PostgreSQL/Rust, CEX merge/artifact, deployed fault/PITR, review |
| Closed online | **blocked** | Nakama target | adapter/shadow, drain/cutover, exact component lock and rollback |
| Public online | **NO-GO** | Nakama target | public edge, KMS/HSM, multi-host, endurance, capacity, staffed/human gates |
| Public player market | **disabled** | separate approval | public online plus custody/fraud/dispute/support/legal/economic approval |
| Commercial single-player | **blocked** | World native client | multi-OS distribution, accessibility, support, legal, external human evidence |

## Source changes in the v4 candidate

- Plan v4, machine gap ledger, architecture, security, database, release, and runbook truth sources;
- strict `trnm_world_transition_v1` parser/API, independent Python conformance, schemas, and positive/negative vectors;
- settlement runtime v2 with SIGINT/SIGTERM admission stop, bounded drain, bounded parallel remote execution, poison-work quarantine, and migration 0019;
- malformed successful remote responses treated as ambiguous/retryable so lookup precedes any repeat submission;
- one campaign job per capture enforced in PostgreSQL;
- blocking reqwest feature removed from the game-server package;
- one read-only, SHA-pinned v4 workflow replacing self-mutating and legacy mixed-repository workflows.

## Interpretation rules

- `implemented` means source, schema, docs, and tests exist in the candidate.
- `verified_remote` requires a successful exact-head run and checksummed artifacts.
- `deployed` requires an exact immutable deployment artifact and environment binding.
- `operational` requires fault, restore, capacity, observability, and operator evidence.
- `release_ready` additionally requires all dependent human, public-network, commercial, legal, and custody rows.

No local fixture, source scanner, generated JSON, or automated screenshot may satisfy a human, public-network, cross-host, custody, or commercial row.
