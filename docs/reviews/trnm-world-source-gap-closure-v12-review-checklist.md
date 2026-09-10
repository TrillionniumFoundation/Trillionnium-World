---
status: current-candidate
owner: trillionnium-world-review
last_reviewed: 2026-09-10
review_due: 2026-09-24
implementation_conformance: not-implied
production_authorization: not_granted
---

# World source gap closure v12 review checklist

## Object binding

- Confirm the pull request base is `fix/world-convergence-v6-20260908` at the exact reviewed base commit.
- Confirm the head and actual GitHub prospective-merge object match every accepted run and artifact.
- Reject stale execution, prior-head approval, synthesized status, or evidence copied from another repository or branch.

## Documentation closure

- Confirm the `contracts/` workspace members are derived from `contracts/Cargo.toml`.
- Confirm every maintained contract crate has a local contract, detailed design, document-catalog entry, component-catalog entry, source path, owner, lifecycle, and release denominator.
- Run the positive checker and every hostile fixture.
- Confirm local contracts and detailed designs make no production, custody, mainnet, host-runtime, or deployment overclaim.

## CI-integrity closure

- Confirm the workflow directory contains exactly the 13 reviewed files declared by `scripts/check-trnm-world-ci-integrity.py`.
- Confirm all 40 emitted contexts are unique and the three protected names are owned only by `world-pr-native-admission-v1.yml`.
- Confirm the retired Authority workflow is absent and its substantive source/PostgreSQL checks remain reachable through `scripts/check-trnm-world-authority-cutover.sh` and the unified qualification entrypoints.
- Run all 17 hostile fixture classes covering inventory drift, context collision, matrix drift, permissions, mutable actions/runners, privileged triggers, source mutation, symlinks, invalid UTF-8, empty and oversized files, child failure, timeout, and accidental writes.

## Source qualification

- Run format, all-target tests, and strict Clippy for the active game-product workspace.
- Run the World-authority cutover source suite, including its format, all-target tests, Clippy, adapter-readiness, restart/reload, and provenance checks.
- Run format, all-target tests, and strict Clippy for the external-contract workspace.
- Run format, tests, strict Clippy, schemas, vectors, and independent conformance for the World transition contract.

## PostgreSQL qualification

- Verify the pinned admission service reports `server_version_num=160004`.
- Verify the service image is pinned to the reviewed PostgreSQL 16.4-alpine digest.
- Run the World-authority catalog, installer, fencing, replay, migration, privilege, and hostile-clone checks.
- Run every maintained settlement database integration target, including capture visibility, lease fencing, serialization, remote identity, operator controls, fault behavior, runtime v2, and worker behavior.
- Run the actual settlement lock-order test against a disposable database.

## Native admission and evidence

- Confirm `trnm-game-ci`, `trnm-world-p0-boundaries`, and `trnm-world-status-evidence` are emitted only by the reviewed matrix workflow object.
- Confirm `trnm-world-v12/required` depends on terminal success of the complete matrix.
- Confirm validation permissions remain `contents: read` and candidate source is never committed, pushed, tagged, merged, released, or deployed by CI.
- Confirm exact head and actual prospective merge are both fetched and identity-checked.
- Confirm retained artifacts include identity, toolchain/database version, raw outputs, SHA-256 manifest, run ID, run attempt, and 90-day retention.

## Independent acceptance

- Do not approve until non-empty jobs and steps exist for one unchanged final tuple.
- Review the first real compiler, test, Clippy, SQL, packaging, and artifact failures rather than assuming source correctness.
- Require a fresh eligible non-author approval after the final source push and resolution of every substantive review thread.
- Keep production authorization not granted and all public/trusted/commercial capabilities disabled.
