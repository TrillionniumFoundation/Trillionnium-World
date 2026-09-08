---
status: current-candidate
owner: trillionnium-world
last_reviewed: 2026-09-08
review_due: 2026-09-15
production_authorization: not_granted
---

# Trillionnium World Closure Execution Board V5

This board converts the development plan into an ordered closure loop. It distinguishes work that can be closed by repository source from evidence that must be produced by GitHub server configuration, another repository, a deployed environment or accountable humans.

## Lane A — repository truth and documentation

Exit criteria:

- root README, operations and release decision are World-specific;
- eight active crates have a summary README and detailed technical design;
- module inventory exactly matches `trillionnium/Cargo.toml`;
- local Markdown links resolve and current docs contain no personal paths;
- stale semantic-source-generation statements fail the gate;
- PR #46 remains the only current review surface.

Repository implementation: this tranche adds `docs/modules/contracts-v2.json`, eight detailed designs, ADR-0003 and a strengthened documentation checker. Hosted execution remains a separate gate.

## Lane B — deterministic contract and simulation

Exit criteria:

- strict canonical JSON vectors and independent interpreter pass;
- RTS strict intake is adopted at every untrusted boundary;
- exact tick ordering, tie-breaks and RNG consumption are specified;
- deterministic cross-toolchain and Nakama shadow results have zero unexplained divergence.

Current source contains substantial contract/vector work. Runtime adoption, hosted exact-head runs and Nakama shadow evidence remain open.

## Lane C — compatibility server and settlement

Exit criteria:

- ordinary reviewable source with no semantic build generation;
- true module/trait boundaries replace textual ownership parts where correctness requires isolation;
- capture, remote execution and apply remain separated;
- all PostgreSQL, lease, quarantine, shutdown, response-loss and PITR matrices pass;
- a CEX-selected immutable revision is bound and independently reviewed.

The ordinary-source denominator is a candidate. Semantic module extraction, CEX qualification and deployed fault evidence remain open.

## Lane D — CI, repository governance and review

Exit criteria:

- final exact head and prospective merge have non-empty successful runs;
- toolchain is pinned to the current safe point release;
- required contexts are applied/read back on protected `main`;
- direct/force push, stale approvals, unresolved conversations and unaudited bypass are blocked;
- a fresh independent approval is bound to the unchanged final tuple.

Files can define the desired policy but cannot prove GitHub server enforcement. Missing workflow runs remain a blocker.

## Lane E — cross-repository authority cutover

Exit criteria:

- Nakama is the sole admission/order/idempotency/recovery/archive/completion authority;
- World exposes only deterministic domain transitions/projections;
- CEX is only wallet/ledger/custody authority;
- Integration locks exact revisions and evidence;
- drain, cutover, rollback and old-authority disablement pass.

ADR-0003 fixes the intended boundary. Implementation and evidence remain owned by the respective repositories.

## Lane F — deployment, product and organizational evidence

Exit criteria:

- reproducible signed multi-OS artifacts and clean-host operations;
- cross-host failover, PITR, old-primary isolation, public edge and 24-hour endurance;
- consented human/accessibility sessions;
- moderation/support/incident drills;
- privacy, security, custody, licensing, legal and commercial approval.

These rows cannot be closed by repository text or fabricated fixtures.

## Stop conditions

Keep the candidate draft and unmerged when any required check is absent, any exact identity moved after evidence, an independent objection remains, a cross-repository lock is stale, or external/human evidence is missing. Valid outcomes are `MODULE_CLOSED_CANDIDATE`, `SERVER_CONFIGURATION_REQUIRED`, `BLOCKED_UPSTREAM`, `EXTERNAL_EVIDENCE_REQUIRED`, `BASE_DRIFT` or `STOP_CONDITION`.
