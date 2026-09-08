# Current Trillionnium World Plan

The executable product foundation remains:

`docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`

The historical machine inputs retained for provenance only are:

- `docs/development/trillionnium-world-development-plan-2026-08-29.json`
- `docs/development/trnm-world-gap-closure-ledger-v4.json`

The current convergence and evidence authorities are:

- `docs/development/TRILLIONNIUM_WORLD_CLOSURE_EXECUTION_BOARD_V5.md`
- `docs/development/trnm-world-gap-closure-ledger-v6.json`
- `docs/integration/trnm-world-cex-current-pending-lock-v2.json`
- `RELEASE_READINESS.md`
- `docs/status/world-plan-v4-execution-truth-2026-09-08.json`

Historical plans, ledgers and coordination locks remain provenance only. The V6 ledger and current coordination lock govern current object selection; accepted ADRs and `PROJECT_BOUNDARY.*` remain binding.

## Operative candidate

- Repository: `TrillionniumFoundation/Trillionnium-World`
- Pull request: `#104`
- Branch: `fix/world-convergence-v6-20260908`
- Base observed: `main@0f6117a56263bcb5bf89e34b8bcda557a7da2e6d`
- Head observed before this truth update: `1acb5e2a4bf872fc487caa6dfe52c0b4c4fe32a9`
- Head tree observed before this truth update: `7e0511496a7b7d36dba0d845a1c134f064581552`
- Prospective merge observed before this truth update: `a03a6e2ddc9a62264410a04ca541b559e6be93be`
- Current candidate toolchain: `1.98.1`
- Historical qualification toolchain: `1.98.0`

Committing this truth update moves the branch head. These values are an explicit pre-update observation, not qualification credit for the successor. Every successor head and actual prospective merge require fresh read-back and non-empty execution.

PR #104 supersedes earlier World convergence surfaces, including closed unmerged PR #60. No CI, review, governance, cross-repository, release or production credit transfers from a superseded object.

## Binding architecture decisions

- **World** owns deterministic game-domain behavior, authored content, native client behavior, World aggregate mutation under one fenced writer epoch, player-facing economic intents, World outcome hashes and unsigned replay/outcome material.
- **Nakama** owns target online admission, identity, canonical total order, durable online idempotency, reconnect/restart recovery, archive roots and `MatchCompletedV1` signing.
- **CEX** owns wallet/ledger settlement and custody.
- **Chain** owns ingress, consensus, inclusion and finality.
- **Integration** owns exact cross-repository component locks, compatibility matrices and release evidence.
- `trillionnium/crates/trnm-game-server` remains a `world_legacy_local_alpha` compatibility enclave.
- `trillionnium/crates/world-authority` remains an isolated World-domain cutover candidate; fixture adapters and the file-backed server are non-production.
- External settlement follows capture -> transaction-free remote execution -> fenced apply. Signer, CEX or network I/O under mutable World rows is prohibited.
- CI may validate and upload immutable evidence but may not move refs, self-approve, synthesize statuses, bypass protection, tag, release or deploy.

ADR `docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md` remains binding. A World-domain service cannot become a second canonical online authority.

## Current source interpretation

The current ordinary source is reviewable and retains detailed contracts for all active and isolated World-authority crates, PostgreSQL base/hardening/catalog-fingerprint contracts, strict bounded JSON parsing, an unconditional pull-request documentation gate and read-only evidence workflows. These are `closed_candidate` source facts, not independently executed qualification. Historical Rust 1.98.0 artifact evidence does not transfer; the final candidate must execute under Rust 1.98.1.

## Current repository observations

The latest pre-update read-back showed zero repository-native pull-request workflow runs for the exact World head. The observed `main` protection retains an older three-context set and the repository Rulesets collection is empty. The managed GitHub App cannot install the complete protection contract or prove latest-push approval, stale-review dismissal, conversation resolution, force-push/deletion denial and no-bypass behavior. These are server-control blockers, not source claims.

## Current CEX dependency observation

The coordination-only current observation is `docs/integration/trnm-world-cex-current-pending-lock-v2.json`.

CEX PR #53 is the single Sequence 54 Draft candidate:

```text
head              882014451c22026dfe4848248f2b26a9b46ac049
tree              cb5b69f79eeb4e6d2032b93115d3e25c88957903
base              db75c74094748a1139fd64b0360e122d8ec797a0
prospective merge 7a93ca40eb9278b4da7dc64b9f773acd639b9410
```

The prospective merge is a GitHub-signed merge of the exact base and head and has tree `cb5b69f79eeb4e6d2032b93115d3e25c88957903`. CEX source-side RustSec admission remains present, but hosted jobs—including the latest P0 and Economy attempts—still fail with no runner and no source steps; Rulesets are empty, `main` is unprotected and current reviews request changes. World adoption and trusted settlement remain disabled.

## Current upstream coordination observations

- TrillionniumGame/Nakama PR #63 remains Draft at `6dd5a101c67d78572a9dcaa3ab6ff96ef1966850`; repository-controlled source/CI is substantially closed, while conflict-free specialist capacity, governance and production evidence remain open.
- Trillionnium-Chain PR #62 remains Draft at `521726e47ce0efad6b732a5fc31afcd2cc52593d`; required workflows, independent campaigns/audit and activation remain open.
- Trillionnium-Integration PR #8 is the coordination surface and must repin each final component movement. Exact pinning is not component qualification.

## Ordered remaining blockers

1. Restore World Actions scheduling and obtain non-empty terminal-success Rust 1.98.1, PostgreSQL, transition, documentation, package, source-boundary and supply-chain execution on one unchanged final head and actual prospective merge.
2. Apply and read back server-side main protection, current required contexts, stale-review dismissal, conversation resolution, linear/no-force-push/no-bypass controls; then obtain fresh independent approval on the unchanged final tuple.
3. Qualify the immutable CEX object above with non-empty execution, candidate manifest, SBOM/provenance, custody evidence and independent approval.
4. Qualify World, CEX, Nakama and Chain independently; have Integration pin exact accepted commits, trees, artifacts and contract bytes and close divergence, no-dual-writer, fault, cutover and rollback evidence.
5. Replace fixture/file-backed World authority adapters with independently reviewed production identity/session, PostgreSQL repository, ledger, evidence, metrics, routing and deployment adapters.
6. Complete remaining `include!` ownership-part migration into true Rust module/trait/visibility boundaries through test-preserving review tranches.
7. Obtain deployment, public-edge, cross-host recovery/endurance, custody/KMS, accessibility, privacy, legal, support, commercial and final human go/no-go evidence.

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
