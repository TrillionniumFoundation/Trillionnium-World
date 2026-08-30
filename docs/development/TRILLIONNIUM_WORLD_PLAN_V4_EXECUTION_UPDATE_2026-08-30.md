---
status: current
owner: trillionnium-world
as_of: 2026-08-30
applies_to_candidate: fix/world-plan-v4-convergence-2026-08-30
supersedes_no_plan: true
---

# Trillionnium World Plan V4 — execution update

This update records facts observed after the V4 convergence addendum. It does not widen World authority or grant release credit.

## Exact candidate

- canonical Draft PR: `#39`;
- branch: `fix/world-plan-v4-convergence-2026-08-30`;
- base observed: `main@efcf0420f6edabc32b7f85332467f25e291cdc63`;
- remote head observed before the direct-source tranche: `5f76aceabe2b1c236c57fbe26c6b70e4bdb31903`;
- observed remote tree: `51ed6b28af909685769e9ff51a047f4d5aa52591`;
- PRs `#36`, `#37`, and `#38` are closed, unmerged, and superseded;
- `WORLD-P0-010` is closed only for the single-current-candidate claim.

## Corrected source truth

The CEX transport, settlement worker, and game-server library are now ordinary directly compiled reviewed source. The direct-source tranche:

- materializes the exact generated game-server body into `src/lib.rs`;
- removes `build.rs`, `src/lib.rs.in`, and the Cargo build-script declaration;
- replaces generator-shape and `OUT_DIR` tests with checks over directly compiled files;
- fixes literal-localhost and bracketed-IPv6 loopback normalization without weakening off-loopback HTTPS enforcement;
- migrates `trnm-online-e2e` away from the retired `reqwest::blocking` surface;
- retains all migration registrations and the prohibition on in-process terminal settlement.

`WORLD-P0-009` therefore advances from `source_open` to `source_implemented_unverified`. It does not become `source_verified` until the committed exact head passes the all-target Rust and mandatory PostgreSQL matrix and receives independent review.

`WORLD-P1-001` remains `source_open`. Documentation boundaries alone do not prove correctness-oriented module ownership; the catch-all game-server library must be decomposed without changing authority, lock order, terminal-publication, or settlement invariants.

## Gate repair completed in this tranche

`scripts/check-trnm-world-v4-convergence.py` now validates the observed machine counts instead of hard-coding the pre-convergence values. It:

- requires `WORLD-P0-010` closure evidence to match PR `#39`, its observed head/tree/base relation, and the closed/unmerged disposition of PRs `#36`–`#38`;
- requires `WORLD-P0-009` byte length, SHA-256, and Git blob identity to match every direct source and boundary-contract file;
- rejects any return of `build.rs`, `lib.rs.in`, `OUT_DIR`-compiled authority, or legacy settlement paths;
- accepts the directly compiled shape only when all mandatory migration and settlement-boundary markers exist;
- rejects source verification without non-empty exact-head runs and required checks;
- keeps upstream, human, public-network, custody, market, and commercial claims fail closed.

## Repository-control observation

For observed head `5f76aceabe2b1c236c57fbe26c6b70e4bdb31903`, GitHub returned zero status contexts and zero pull-request workflow runs. A read-only Chain relay subsequently checked out that immutable World revision, materialized the reviewed game-server body, packaged the complete working tree, and uploaded an artifact successfully. A separate Chain-to-World write probe failed with `403` for `github-actions[bot]`. The relay proves the materialization procedure executed; it is not World exact-head repository governance and grants no source-verification or release credit.

GitHub also returned an empty repository-ruleset collection. Branch protection could not be independently queried through the installed integration (`403`), `ProfHepta` is requested, and zero independent approvals are observed. The remaining repository-control gates require server-side actions not exposed by the current connector:

- enable repository/organization Actions execution;
- install a `main` ruleset with exact required checks;
- enforce independent code-owner approval after the last push;
- execute a negative merge rehearsal.

## Ordered continuation

1. Extract correctness-critical ownership modules with invariant tests.
2. Run non-empty exact-head Rust, PostgreSQL, canonical JSON, package, and supply-chain checks.
3. Apply and independently query server-side governance.
4. Hand upstream work to Nakama, Integration, and CEX owners using exact component locks.
5. Execute immutable deployed fault, PITR, endurance, public-edge, custody, multi-platform, human, and commercial evidence programs.

## Release posture

```text
technical source candidate: in progress
exact-head remote verification: repository control blocked
canonical Nakama online authority: blocked upstream
trusted deployed settlement: environment evidence required
public online: NO-GO
public player market: disabled
commercial release: NO-GO
```
