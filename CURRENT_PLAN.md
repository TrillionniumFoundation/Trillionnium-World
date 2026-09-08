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

Historical machine-readable plans, ledgers and coordination locks remain provenance only. Where historical identity conflicts with the selected snapshot, V6 ledger or current coordination lock, the current files govern. Accepted ADRs and `PROJECT_BOUNDARY.*` remain binding.

## Operative candidate

- Repository: `TrillionniumFoundation/Trillionnium-World`
- Pull request: `#104`
- Branch: `fix/world-convergence-v6-20260908`
- Base observed: `main@0f6117a56263bcb5bf89e34b8bcda557a7da2e6d`
- Head observed before this truth update: `1c69b9a2c5ffdf00ef803ca182a0a75712f6536c`
- Head tree observed before this truth update: `f9052a18dfd9f8011b69377cd7984fc20f7482ae`
- Prospective merge observed before this truth update: `4b8021602cb444cc60520e5b7ab09d64262c7735`
- Current candidate toolchain: `1.98.1`
- Historical qualification toolchain: `1.98.0`

Committing this truth update necessarily moves the branch head. The values above are an explicit pre-update observation, not execution credit for the later commit. Every later head and actual prospective merge require fresh read-back and non-empty execution.

PR #104 supersedes earlier World convergence surfaces, including closed unmerged PR #60. No CI, review, governance, cross-repository, release or production credit transfers from a superseded object.

## Binding architecture decisions

- **World** owns deterministic game-domain behavior, authored content, native client behavior, World aggregate mutation under one fenced writer epoch, player-facing economic intents, World outcome hashes and unsigned replay/outcome material.
- **Nakama** owns target online admission, participant/session identity, canonical total order, durable online idempotency, reconnect/restart recovery, archive roots and `MatchCompletedV1` signing.
- **CEX** owns wallet/ledger settlement and custody.
- **Chain** owns ingress, consensus, inclusion and finality.
- **Integration** owns exact cross-repository component locks, compatibility matrices and release evidence.
- `trillionnium/crates/trnm-game-server` remains a `world_legacy_local_alpha` compatibility enclave.
- `trillionnium/crates/world-authority` remains an isolated World-domain cutover candidate; fixture adapters and the file-backed server are non-production.
- External settlement follows capture -> transaction-free remote execution -> fenced apply. Signer, CEX or network I/O under mutable World rows is prohibited.
- CI may validate and upload immutable evidence but may not move refs, self-approve, synthesize statuses, bypass protection, tag, release or deploy.

ADR `docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md` remains binding. A World-domain service cannot become a second canonical online authority.

## Current source interpretation

The historical immutable artifact proves only its exact source tree and Rust 1.98.0 environment. Current ordinary source is directly reviewable: semantic game-server `build.rs` and `src/lib.rs.in` authority are absent and Cargo no longer declares that build script.

The convergence candidate contains:

- substantive detailed designs and local contracts for all eight active game-product crates;
- detailed contracts for all seven isolated World-domain authority crates;
- a World-only operations manual, current release decision and strict document catalogue;
- PostgreSQL base, hardening and complete catalog-fingerprint contracts;
- cutover/provenance contracts and source/adapter/PostgreSQL gates;
- a bounded strict JSON parser with duplicate-key, non-finite, UTF-8, size, depth and node-budget rejection;
- an unconditional pull-request documentation gate whose trigger surface cannot be bypassed by changing a validator input;
- read-only CI definitions that bind source and prospective-merge objects.

These are `closed_candidate` source facts, not independently executed qualification. They do not inherit the historical Rust 1.98.0 artifact result. The final current head must run with Rust 1.98.1.

## Current repository observations

Live read-back before this update showed:

```text
normal repository-native candidate workflow runs  0
repository rulesets                              0
main protected                                   true
main required contexts                           old three-context set
```

The observed required contexts remain:

```text
trnm-game-ci
trnm-world-p0-boundaries
trnm-world-status-evidence
```

They do not match `docs/governance/main-protection-contract-v2.json`. The managed GitHub App cannot create the final Ruleset or change administration surfaces needed for latest-push approval, stale-review dismissal, conversation resolution, current required contexts, linear/no-force-push controls and no-bypass behavior. These are server-configuration blockers, not source claims.

## Current CEX dependency observation

The coordination-only current observation is `docs/integration/trnm-world-cex-current-pending-lock-v2.json`.

CEX PR #53 is the single Sequence 54 Draft candidate on branch `integration/cex-v12-sequence54-20260908`:

```text
head              882014451c22026dfe4848248f2b26a9b46ac049
tree              cb5b69f79eeb4e6d2032b93115d3e25c88957903
base              db75c74094748a1139fd64b0360e122d8ec797a0
prospective merge c09cf4783f878e909ca8b47bf83ade4f7607136d
```

Its bounded RustSec policy, all-feature/release-surface controls and dedicated anti-regression wiring are present in source. Its hosted jobs still fail before source steps, its live Rulesets collection remains empty, `main` remains unprotected and fresh independent approvals are absent. It remains unqualified with `production_authorization=not_granted`. This observation does not select it for release or enable World adoption.

## Current upstream coordination observations

- TrillionniumGame/Nakama PR #63 remains Draft at `6dd5a101c67d78572a9dcaa3ab6ff96ef1966850`; repository-controlled source/CI is substantially qualified, while conflict-free specialist capacity, effective governance and production evidence remain open.
- Trillionnium-Chain PR #62 remains Draft at `521726e47ce0efad6b732a5fc31afcd2cc52593d`; required workflow, independent campaign/audit and activation denominators remain open.
- Trillionnium-Integration PR #8 is the live coordination surface. Every World, CEX, Nakama or Chain movement requires a new exact Integration repin before acceptance; an Integration coordination pin is not component qualification.

## Ordered remaining blockers

1. Restore World Actions scheduling and obtain non-empty terminal-success Rust 1.98.1, PostgreSQL, transition, documentation, package, source-boundary and supply-chain execution on one unchanged final head and its actual prospective merge.
2. Apply and read back server-side main protection, current required contexts, stale-review dismissal, conversation resolution, linear/no-force-push/no-bypass controls; then obtain fresh independent approval on the unchanged final tuple.
3. Qualify the immutable CEX revision above with non-empty execution, candidate manifest, SBOM/provenance, custody evidence and independent approval.
4. Qualify World, CEX, Nakama and Chain independently; have Integration pin exact accepted commits, trees, artifacts and contract bytes and close transition divergence, no-dual-writer, fault, cutover and rollback evidence.
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
