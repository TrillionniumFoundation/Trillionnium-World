# Current Trillionnium World Plan

The executable product foundation remains:

`docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`

The current convergence board and machine denominator ledger are:

- `docs/development/TRILLIONNIUM_WORLD_CLOSURE_EXECUTION_BOARD_V5.md`
- `docs/development/trnm-world-gap-closure-ledger-v6.json`
- `docs/integration/trnm-world-cex-current-pending-lock-v2.json`
- `RELEASE_READINESS.md`

The authoritative current execution snapshot is:

- `docs/status/world-plan-v4-execution-truth-2026-09-08.json`

<!-- trnm-current-execution-snapshot: docs/status/world-plan-v4-execution-truth-2026-09-08.json -->

Historical machine-readable plans, ledgers and coordination locks remain provenance only:

- `docs/development/trillionnium-world-development-plan-2026-08-29.json`
- `docs/development/trnm-world-gap-closure-ledger-v4.json`
- `docs/development/trnm-world-gap-closure-ledger-v5.json`
- `docs/status/world-v4-convergence-state-2026-08-30.json`
- `docs/integration/trnm-world-cex-sequence-50-pending-lock-v1.json`

Where a historical identity conflicts with the selected snapshot, V6 ledger or current coordination lock, the current files govern. Accepted ADRs and `PROJECT_BOUNDARY.*` remain binding.

## Operative candidate

- Repository: `TrillionniumFoundation/Trillionnium-World`
- Pull request: `#104`
- Branch: `fix/world-convergence-v6-20260908`
- Base observed: `main@0f6117a56263bcb5bf89e34b8bcda557a7da2e6d`
- Head observed before this truth update: `867518486d88cde6b0f3107ce73934d83161027c`
- Head tree observed before this truth update: `dbb1b966b3e36feca81bb4ff72b3f0a186b3ae2b`
- Current candidate toolchain: `1.98.1`
- Historical qualification toolchain: `1.98.0`

Committing this truth update necessarily moves the branch head. The values above are an explicit pre-update observation, not execution credit for the later commit. Every later head and actual prospective merge require fresh read-back and non-empty execution.

PR #104 supersedes PR #46, the stacked convergence PR #103 and the separately closed World-domain authority PR #60. No CI, review, governance, cross-repository, release or production credit transfers from a superseded surface.

## Binding architecture decisions

- **World** owns deterministic game-domain behavior, authored content, the native client, World aggregate mutation under one fenced writer epoch, player-facing economic intents, World outcome hashes and unsigned replay/outcome material.
- **Nakama** owns target online admission, participant/session identity, canonical total order, durable online idempotency, reconnect/restart recovery, archive roots and `MatchCompletedV1` signing.
- **CEX** owns wallet/ledger settlement and custody.
- **Chain** owns ingress, consensus, inclusion and finality.
- **Integration** owns exact cross-repository component locks, compatibility matrices and release evidence.
- `trillionnium/crates/trnm-game-server` remains a `world_legacy_local_alpha` compatibility enclave.
- `trillionnium/crates/world-authority` is an isolated seven-crate World-domain cutover candidate. Fixture adapters and the file-backed server are non-production.
- External settlement follows capture -> transaction-free remote execution -> fenced apply. Signer, CEX or network I/O under mutable World rows is prohibited.
- CI may validate and upload immutable evidence but may not modify candidate semantics, move source refs, self-approve, synthesize statuses, bypass protection, tag, release or deploy.

ADR `docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md` is binding. A World-domain service cannot become a second canonical online authority.

## Current source interpretation

The historical immutable artifact proves only its exact source tree and Rust 1.98.0 environment. Current ordinary source is directly reviewable: semantic game-server `build.rs` and `src/lib.rs.in` authority are absent and Cargo no longer declares that build script.

The convergence candidate contains:

- substantive detailed designs and local contracts for all eight active game-product crates;
- detailed contracts for all seven isolated World-domain authority crates;
- a World-only root operations manual, current release decision and strict document catalogue;
- PostgreSQL base, hardening and complete catalog-fingerprint contracts;
- cutover/provenance contracts and source/adapter/PostgreSQL gates;
- strict JSON and machine-truth validation;
- read-only CI definitions that bind source and prospective-merge objects.

These source facts are `closed_candidate`, not independently validated. They do not inherit the historical 1.98.0 artifact result. The final current head must run with Rust 1.98.1.

## Current repository observations

Live read-back on 2026-09-08 shows:

```text
normal repository-native candidate workflow runs  0
repository rulesets                              0
main protected                                   true
main required contexts                           old three-context set
```

The current observed required contexts are:

```text
trnm-game-ci
trnm-world-p0-boundaries
trnm-world-status-evidence
```

They do not match `docs/governance/main-protection-contract-v2.json`. The managed GitHub connection cannot read or change the administration surfaces needed to prove Actions policy, runner allocation, billing/quota/suspension, detailed review enforcement or no-bypass behavior. These are server-configuration blockers, not source gaps.

## Current CEX dependency observation

The coordination-only current observation is `docs/integration/trnm-world-cex-current-pending-lock-v2.json`.

CEX PR #29 is still open Draft on sequence 53 at commit `04491cff7cb317324f57a10de07872d39f3e56c2`, tree `f8897db7f5879a8ae84a2ce90ec169e5df9eccb9`. It remains unqualified and has `production_authorization=not_granted`. This observation does not select it for release or enable World adoption.

## Ordered remaining blockers

1. Restore World Actions scheduling and obtain non-empty terminal-success Rust 1.98.1, PostgreSQL, transition, documentation, package, source-boundary and supply-chain execution on one unchanged final head and its actual prospective merge object.
2. Apply and read back server-side main protection, current required contexts, stale-review dismissal, conversation resolution, linear/no-force-push/no-bypass controls; then obtain fresh independent approval on the unchanged final tuple.
3. Select and qualify one immutable CEX revision with non-empty execution, manifest, SBOM/provenance, custody evidence and independent approval.
4. Qualify World, CEX, Nakama, Chain and Integration independently; pin exact accepted commits/trees/artifacts and close transition divergence, no-dual-writer, fault, cutover and rollback evidence.
5. Replace fixture/file-backed World-domain adapters with independently reviewed production identity/session, PostgreSQL repository, ledger, evidence, metrics, routing and service-deployment adapters.
6. Complete the remaining `include!` ownership-part migration into true Rust module/trait/visibility boundaries through test-preserving review tranches.
7. Obtain deployment, public-edge, cross-host recovery/endurance, custody/KMS, human/accessibility, privacy, legal, support, commercial and final human go/no-go evidence from their accountable authorities.

Public online operation, public player markets, trusted settlement and commercial release remain **NO-GO / disabled**. Production authorization remains **not granted**.

## Executable truth and integrity checks

```bash
python3 scripts/check-trnm-world-execution-truth.py
python3 scripts/test-trnm-world-execution-truth.py
python3 scripts/check-trnm-world-machine-truth.py
python3 scripts/test-trnm-world-machine-truth.py
python3 scripts/check-trnm-world-active-closure.py
python3 scripts/test-trnm-world-active-closure.py
python3 scripts/check-trnm-world-ci-integrity.py
python3 scripts/test-trnm-world-ci-integrity.py
python3 scripts/check-trnm-world-detailed-documentation.py
python3 scripts/check-trnm-world-authority-documentation.py
python3 scripts/check-trnm-world-cex-pending-lock.py
```

`docs/status/CURRENT.md` is a deterministic rendering of the selected snapshot, not a fresh GitHub query. A local operator may regenerate it with `--write` only after an authorized snapshot/pointer update; CI rejects that option.

Production authorization remains **not granted**.
