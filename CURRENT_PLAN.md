# Current Trillionnium World Plan

The executable product foundation remains:

`docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`

The historical machine inputs retained for provenance only are:

- `docs/development/trillionnium-world-development-plan-2026-08-29.json`
- `docs/development/trnm-world-gap-closure-ledger-v4.json`

The current convergence and evidence authorities are:

- `docs/release/trnm-world-external-execution-board-v1.json`
- `docs/development/trnm-world-gap-closure-ledger-v6.json`
- `docs/integration/trnm-world-cex-current-pending-lock-v2.json`
- `RELEASE_READINESS.md`
- `docs/status/world-plan-v4-execution-truth-2026-09-08.json`

Historical plans, ledgers and coordination locks remain provenance only. The V6 ledger, current external-execution board and current coordination lock govern current object selection; accepted ADRs and `PROJECT_BOUNDARY.*` remain binding. The qualification entrypoint rejects a missing authority file and rejects reintroduction of the nonexistent historical V5 board path.

## Operative candidate

- Repository: `TrillionniumFoundation/Trillionnium-World`
- Pull request: `#104`
- Branch: `fix/world-convergence-v6-20260908`
- Base observed: `main@0f6117a56263bcb5bf89e34b8bcda557a7da2e6d`
- Head observed before this truth update: `bd1603ea6087f71ee1fe2d7940c729931105c25f`
- Head tree observed before this truth update: `50f2b8d7a6fb75ad77210d3082a2362e4b210438`
- Prospective merge observed before this truth update: `98a6c53d98cac94dcf746704c8d6508ad0cb10fc`
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

The prior plan referenced `docs/development/TRILLIONNIUM_WORLD_CLOSURE_EXECUTION_BOARD_V5.md`, which does not exist in the candidate tree. The current authority is the tracked and catalogued `docs/release/trnm-world-external-execution-board-v1.json`; the exact truth qualification now checks this reference and the other current authority files fail closed.

## Current repository observations

The latest pre-update read-back showed zero repository-native pull-request workflow runs for the exact World head. The observed `main` protection retains an older three-context set and the repository Rulesets collection is empty. The managed GitHub App cannot install the complete protection contract or prove latest-push approval, stale-review dismissal, conversation resolution, force-push/deletion denial and no-bypass behavior. These are server-control blockers, not source claims.

Draft PR `TrillionniumFoundation/TrillionniumGame#122` provides a temporary read-only external execution route for exact World head and true prospective-merge qualification. It is technical execution evidence only and cannot replace World-native required checks or protected admission.

## Current CEX dependency observation

The coordination-only current observation is `docs/integration/trnm-world-cex-current-pending-lock-v2.json`.

CEX PR #53 is the single Sequence 54 Draft candidate observed at the time of this update:

```text
head              df877364a38e66b6e8908f8ea0650dc1a6ef1d35
tree              4cf744c3ad586aa2381d6b9e389620e7f88e0b14
base              db75c74094748a1139fd64b0360e122d8ec797a0
prospective merge 81f3089fd2753cb8cab82431c8c77bc7e056ce61
```

CEX source-side RustSec and Sequence 54 admission material remains present, but hosted jobs still fail before source steps with `steps=null`; Rulesets are empty and current exact-head approvals are absent. World adoption and trusted settlement remain disabled. Any later CEX movement invalidates this observation and requires a fresh Integration repin.

## Current upstream coordination observations

- TrillionniumGame/Nakama PR #63 remains Draft at `6dd5a101c67d78572a9dcaa3ab6ff96ef1966850`, tree `01d528735ac10653eb811500e0febb2f4a6eef14`, prospective merge `b78a095a712f55746b4b4fe04ad6db8456baa98c`; repository-controlled source/CI is substantially closed, while conflict-free specialist capacity, governance and production evidence remain open.
- Trillionnium-Chain PR #62 remains Draft at `bbee7c0194e420a36c7747167000450083d5d951`, tree `f7b239de6bd3ec1bd782c429e7a141776314a078`, prospective merge `f2125748fd733b85f45567334e6e1b4aab80f86c`; required baseline is successful, while other native workflows, independent campaigns/audit and activation remain open.
- Trillionnium-Integration PR #8 remains the coordination surface at `07decde85fe0f29f37a078055196ffff46bca242`, prospective merge `d302c4b88b114a1e88a7139546982c851c54be81`, and must repin each final component movement. Exact pinning is not component qualification.

## Ordered remaining blockers

1. Obtain non-empty terminal-success Rust 1.98.1, PostgreSQL, transition, documentation, package, source-boundary and supply-chain execution on one unchanged final World head and actual prospective merge; restore World repository-native Actions scheduling for protected admission.
2. Apply and read back server-side main protection, current required contexts, stale-review dismissal, conversation resolution, linear/no-force-push/no-bypass controls; then obtain fresh independent approval on the unchanged final tuple.
3. Qualify the immutable final CEX object with non-empty execution, candidate manifest, SBOM/provenance, custody evidence and independent approval.
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
bash scripts/run-trnm-world-v6-qualification-v2.sh truth
```

`docs/status/CURRENT.md` is a deterministic rendering of the selected snapshot, not a fresh GitHub query. A local operator may regenerate it with `--write` only after an authorized snapshot/pointer update; CI rejects that option.

Production authorization remains **not granted**.
