# Current Trillionnium World Plan

The current executable product plan remains:

`docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`

Its binding convergence interpretation remains:

- `docs/development/TRILLIONNIUM_WORLD_PLAN_V4_CONVERGENCE_ADDENDUM_2026-08-30.md`
- `docs/development/TRILLIONNIUM_WORLD_PLAN_V4_EXECUTION_UPDATE_2026-08-30.md`

The authoritative current execution snapshot is:

- `docs/status/world-plan-v4-execution-truth-2026-09-02.json`

The older machine-readable plan, gap ledger, and convergence-state files remain historical planning inputs. Where their candidate identity or execution state conflicts with the current execution snapshot, the current execution snapshot governs.

## Operative candidate

- Repository: `TrillionniumFoundation/Trillionnium-World`
- Pull request: `#46`
- Branch: `fix/world-plan-v4-development-closure-20260831`
- Source qualification base: `5605cfb8861aa923f69ff032ddbff7d035bccb0c`
- Qualification control head: `68e9631b3fc3f75f332497f8d0551608bf0e1413`
- Qualified source tree: `5e613185f5a2abda42df371f3755e73667717309`
- Qualified source patch SHA-256: `ba49dba1e7fbf842f146ac399647e188faafcfbd5ce3ad17425ef88850e0199f`
- Rust toolchain: `1.98.0`

PR `#39` and earlier Plan V4 branches are superseded and must not be used as current truth sources.

## Binding architecture decisions

- **World** owns deterministic game-domain behavior, authored content, the native client, player-facing economy intents, World outcome hashes, and unsigned replay/outcome material.
- **Nakama** owns target online admission, canonical total order, idempotency, restart recovery, archive roots, and `MatchCompletedV1` signing.
- **Chain** owns ingress/finality, **CEX** owns wallet/ledger settlement and custody, and **Integration** owns exact cross-repository component locks and release evidence.
- The World-local online server is a `world_legacy_local_alpha` compatibility enclave and must not expand into a second canonical public authority.
- External settlement follows capture -> transaction-free remote execution -> fenced apply. Signer, CEX, or network I/O under mutable match or campaign locks is prohibited.
- CI may validate and upload evidence but may not rewrite candidate semantics, self-approve, synthesize statuses, bypass protection, tag, release, deploy, or promote a candidate.
- Source, workflow, CODEOWNERS, plan text, or ruleset documentation is not remote/server evidence.

## Current closure interpretation

The immutable qualification artifact proves that the direct-source candidate passed its bound source and test gates. It does not prove that those bytes are present in PR `#46`.

Until the exact qualified source bytes are attached to the operative branch and revalidated on its final exact head:

- `WORLD-P0-009` remains `publication_blocked`, not source-closed;
- `WORLD-P1-001` remains `publication_blocked`, not source-closed;
- semantic `build.rs` and `src/lib.rs.in` authority must remain treated as open debt on the live branch;
- empty check collections and a repository with zero workflow runs receive no exact-head verification credit.

The current CEX dependency is PR `TrillionniumFoundation/CEX#24`, commit `dc0862b8cf88a1f4e6328d519947e19b81122de0`, tree `762e33a3f16c14347a44cec1d862a8e0ab447ad8`, migration head `0088_enforce_provider_terminal_evidence_binding.sql`, sequence `50`. Its authoritative jobs currently stop before runner allocation and therefore do not grant a qualified component lock or production authorization.

## Ordered remaining blockers

1. Publish the exact qualified World source tree into PR `#46` without changing its bytes or using administrator bypass.
2. Restore World Actions scheduling and obtain non-empty Rust 1.98, PostgreSQL, transition-contract, package, source-boundary, and supply-chain evidence on the final exact head and prospective merge object.
3. Restore CEX runner allocation and obtain its complete non-empty exact-head qualification, manifest, SBOM, provenance, and independent approval.
4. Apply server-side main protection and required checks; obtain fresh independent review of each final exact head.
5. Bind World, CEX, Nakama, Chain, and Integration to immutable qualified revisions and close fault/divergence evidence.
6. Obtain deployment, custody, public-edge, cross-host recovery/endurance, human/accessibility, privacy, legal, support, commercial, and final human go/no-go evidence from their actual authorities.

Public online operation, public player markets, trusted settlement, and commercial release remain **NO-GO / disabled** until every dependency row has independently verified exact evidence. Production authorization remains **not granted**.
