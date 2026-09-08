# Current Trillionnium World Plan

The current executable product plan remains:

`docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`

Its binding convergence interpretation remains:

- `docs/development/TRILLIONNIUM_WORLD_PLAN_V4_CONVERGENCE_ADDENDUM_2026-08-30.md`
- `docs/development/TRILLIONNIUM_WORLD_PLAN_V4_EXECUTION_UPDATE_2026-08-30.md`

The authoritative current execution snapshot is:

- `docs/status/world-plan-v4-execution-truth-2026-09-08.json`

<!-- trnm-current-execution-snapshot: docs/status/world-plan-v4-execution-truth-2026-09-08.json -->

The older machine-readable plan, gap ledger and convergence state remain required planning or historical inputs:

- `docs/development/trillionnium-world-development-plan-2026-08-29.json`
- `docs/development/trnm-world-gap-closure-ledger-v4.json`
- `docs/status/world-v4-convergence-state-2026-08-30.json`

Where an older candidate identity or execution state conflicts with the selected snapshot, the selected snapshot governs. Accepted ADRs and `PROJECT_BOUNDARY.*` remain binding.

## Operative candidate

- Repository: `TrillionniumFoundation/Trillionnium-World`
- Pull request: `#104`
- Branch: `fix/world-convergence-v6-20260908`
- Base observed: `main@0f6117a56263bcb5bf89e34b8bcda557a7da2e6d`
- Head observed before this truth update: `d939a555edd1a0873287859aaeab4e028a2debc4`
- Head tree observed before this truth update: `3e8d7f881a143566ea9d83f02ba21e9558b517da`
- Prospective merge observed before this truth update: `1e0fa3dfdd03c58bb06ed959768db0160ed64ee8`
- Historical source qualification base: `5605cfb8861aa923f69ff032ddbff7d035bccb0c`
- Historical qualification control head: `68e9631b3fc3f75f332497f8d0551608bf0e1413`
- Historical qualified source tree: `5e613185f5a2abda42df371f3755e73667717309`
- Historical qualified source patch SHA-256: `ba49dba1e7fbf842f146ac399647e188faafcfbd5ce3ad17425ef88850e0199f`
- Historical qualification toolchain: `1.98.0`

Committing this snapshot necessarily moves the branch head. The values above are an explicit pre-update observation, not a claim about the later commit. Every later head and its actual prospective merge object require fresh read-back and execution.

PR #104 supersedes PR #46, the stacked convergence PR #103 and the separately closed World-domain authority PR #60. No CI, review, governance, cross-repository, release or production credit transfers from any superseded surface.

## Binding architecture decisions

- **World** owns deterministic game-domain behavior, authored content, the native client, World aggregate mutation under one fenced writer epoch, player-facing economic intents, World outcome hashes and unsigned replay/outcome material.
- **Nakama** owns target online admission, participant/session identity, canonical total order, durable online idempotency, reconnect/restart recovery, archive roots and `MatchCompletedV1` signing.
- **CEX** owns wallet/ledger settlement and custody.
- **Chain** owns ingress, consensus, inclusion and finality.
- **Integration** owns exact cross-repository component locks, compatibility matrices and release evidence.
- `trillionnium/crates/trnm-game-server` remains a `world_legacy_local_alpha` compatibility enclave.
- `trillionnium/crates/world-authority` is an isolated seven-crate World-domain cutover candidate. Its fixture adapters and file-backed server are non-production, and its Cargo metadata retains `production_authorization=not_granted`.
- External settlement follows capture -> transaction-free remote execution -> fenced apply. Signer, CEX or network I/O under mutable World rows is prohibited.
- CI may validate and upload immutable evidence but may not modify candidate semantics, move source refs, self-approve, synthesize statuses, bypass protection, tag, release or deploy.

ADR `docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md` is binding. A World-domain service cannot become a second canonical online authority.

## Current source interpretation

The historical immutable artifact proves only its bound source tree and toolchain. The ordinary game-server source is now published directly: semantic `build.rs` and `src/lib.rs.in` authority are absent, Cargo no longer declares that build script, and correctness source is Git-tracked.

The convergence candidate additionally contains:

- substantive detailed designs and local contracts for all eight active game-product crates;
- a World-only root operations manual and machine current-document catalogue;
- the seven isolated World-domain authority crates;
- PostgreSQL base, hardening and complete catalog-fingerprint contracts;
- cutover/provenance contracts and source/adapter/PostgreSQL gates;
- per-crate authority-workspace contracts and detailed designs;
- ten reviewed read-only workflows with twenty-three unique static job contexts;
- CI integrity rules that allow only the non-ref-moving `git commit-tree` prospective object while rejecting ordinary commit, push, tag, merge, ref movement, mutable actions/runners and write permissions.

Those additions have not inherited the historical artifact's execution result. Therefore the converged source denominator remains open pending exact-head and true prospective-merge qualification on PR #104.

## Current remote observations

At `2026-09-08T07:57:22Z`, ordinary branch pushes, pull-request creation/update and reviewer requests had produced:

```text
repository-native workflow runs  0
commit statuses                  0
```

The workflow definitions are present, but definitions are not runs. Missing, skipped, cancelled, stale, base-only, synthetic or unrelated-repository checks receive no credit.

The managed GitHub connection cannot read or change the administration surfaces needed to prove repository/organization Actions policy, runner allocation, billing/quota/suspension, detailed branch protection or no-bypass enforcement. Those are live server-configuration blockers tracked by issues #58, #48 and #4.

## CEX dependency observation

Live read-back on 2026-09-08 shows `TrillionniumFoundation/CEX#29` still open as a Draft at commit `04491cff7cb317324f57a10de07872d39f3e56c2`, tree `f8897db7f5879a8ae84a2ce90ec169e5df9eccb9`, sequence 53. It remains unqualified and has `production_authorization=not_granted`.

That observation does not select or qualify the CEX revision for World. Issue #49 remains responsible for one immutable CEX contract selection, and Integration must bind the final accepted World/CEX/Nakama/Chain revisions.

## Ordered remaining blockers

1. Restore World Actions scheduling and obtain non-empty terminal Rust, PostgreSQL, transition, documentation, package, source-boundary and supply-chain execution on one unchanged final head and its actual prospective merge object.
2. Apply and read back server-side main protection, required contexts, stale-review dismissal, conversation resolution, linear/no-force-push/no-bypass controls; then obtain fresh independent approval on the unchanged final tuple.
3. Select and qualify one immutable CEX revision with non-empty execution, manifest, SBOM/provenance and independent approval.
4. Qualify World, CEX, Nakama, Chain and Integration independently; pin exact accepted commits/trees/artifacts and close transition divergence, no-dual-writer, fault, cutover and rollback evidence.
5. Replace fixture/file-backed World-domain adapters with independently reviewed production identity/session, PostgreSQL repository, ledger, evidence, metrics, routing and service-deployment adapters.
6. Obtain deployment, public-edge, cross-host recovery/endurance, custody/KMS, human/accessibility, privacy, legal, support, commercial and final human go/no-go evidence from their accountable authorities.

Public online operation, public player markets, trusted settlement and commercial release remain **NO-GO / disabled**. Production authorization remains **not granted**.

## Executable truth and integrity checks

`docs/status/CURRENT.md` is a deterministic rendering of the explicitly selected snapshot. It is not a fresh GitHub query and cannot promote evidence.

```bash
python3 scripts/check-trnm-world-execution-truth.py
python3 scripts/test-trnm-world-execution-truth.py
python3 scripts/test-trnm-world-qualified-checkout.py
python3 scripts/check-trnm-world-ci-integrity.py
python3 scripts/test-trnm-world-ci-integrity.py
python3 scripts/check-trnm-world-detailed-documentation.py
python3 scripts/check-trnm-world-authority-documentation.py
```

A local operator may regenerate the status view with `--write` only after an authorized snapshot/pointer update. CI rejects that option. All real source, Rust, PostgreSQL, package, fault and cross-repository gates remain separate.

The exact target-binding design is documented in `docs/development/trnm-world-ci-target-binding-v1.md`. The ten workflow files and twenty-three static contexts are a reviewed source inventory, not proof that GitHub parsed, scheduled or completed them.
