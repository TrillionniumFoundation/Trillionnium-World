---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P0-009
  - WORLD-P1-007
last_reviewed: 2026-09-05
review_due: 2026-09-19
release_effect: none
---

# Direct-source settlement static boundary v2

## Scope and dependencies

The existing hyphenated settlement boundary entry point must inspect ordinary
Git-tracked Rust after the fixed v13k source import. Its previous implementation
required `settlement_worker.rs.in`, semantic `build.rs`, generated `OUT_DIR`
entry points and a retired workflow. It also ignored the optional fixture
source argument supplied by its own negative-test driver.

This change repairs the static and runtime-status checkers, their tests and the combined entry point. It does not alter
the pinned 73 source writes, the two required deletions, the qualified artifact,
settlement runtime, migrations, signing payloads, remote account balances,
repository protection, authorization or release state. Apply and validate it
separately from exact qualified-source publication. It intentionally rejects an
old live checkout that still uses semantic generation.

Runtime prerequisites are Python 3.11 or later, Bash, and the directly compiled
source set selected by the current plan. The Python implementation uses only
the standard library. Rust, PostgreSQL and hosted workflows are not invoked by
this checker and receive no execution credit from it.

## Public command interface

From the repository root:

```bash
bash scripts/check-trnm-settlement-transaction-boundary.sh
bash scripts/check-trnm-settlement-transaction-boundary.sh full
bash scripts/check-trnm-settlement-transaction-boundary.sh scan-only
bash scripts/test-trnm-settlement-transaction-boundary-negative.sh
```

The first two commands run the full source-static contract. Repository
`scan-only` checks ordinary sources, migration registration and known direct
phase boundaries without the additional migration/test/workflow inventories.

For a deliberately isolated source fixture containing `lib.rs`:

```bash
bash scripts/check-trnm-settlement-transaction-boundary.sh scan-only FIXTURE_SOURCE_DIRECTORY
```

The supplied directory is actually used; it never falls back to repository
sources. The equivalent Python command additionally accepts `--root` for an
explicit repository under review. A fixture argument in `full` mode, an unknown
mode, extra positional arguments or a missing fixture fails nonzero. Default
execution is read-only. No form imports objects, writes a ref, regenerates
source, starts services or contacts an external API.

## Source composition

The game-server `src/lib.rs` and settlement-worker `src/settlement_worker.rs`
are expanded independently. Only ordinary `include!("relative.rs")` and the
reviewed crate-root `include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/..."))`
forms are supported. Dynamic expressions and `OUT_DIR` includes fail closed.

Every opened source must be nonempty UTF-8 and remain within the declared
crate root. Absolute, parent-traversal, backslash, drive-qualified and `.git`
paths are rejected. Symlink files/directories, dangling links, missing files,
duplicate semantic includes and cycles also fail. This is an intentionally
narrow source-composition contract, not a general implementation of Rust macros.

The checker rejects the reappearance of semantic `build.rs`, `src/lib.rs.in`
and `src/settlement_worker.rs.in`, including dangling links, and a Cargo package
that opts into a build script. Ordinary source uses no generated authority
names. The compatibility settlement function must retain its explicit error
and worker-owned failure reason; this assertion is not a control-flow proof.

## Lexical and phase contract

The bounded lexical scanner distinguishes comments, ordinary/byte/C strings,
raw strings, character literals, raw identifiers and lifetime punctuation.
Nested block comments and string-contained braces must not change function
boundaries. Text inside comments, strings or a macro definition/invocation
cannot establish an ordinary phase function. Each named phase must have exactly
one ordinary implementation in the inspected bundle.

`capture_match` and `apply_capture` must contain a recognized transaction
opener or explicit `Transaction` type and must contain no direct signer/CEX
execution symbol. `process_claimed_job` must contain actual calls to both
`authorize_settlement_intent` and `submit_authorized_settlement_intent`, rather
than just comments, strings or function references. It must not open a recognized
transaction, carry an explicit transaction type, or contain a decoded SQL
`FOR UPDATE`, `FOR SHARE`, `FOR NO KEY UPDATE` or `FOR KEY SHARE` clause.
Whitespace, raw identifiers, qualified names and supported string escapes do
not hide these checked forms.

The complete 16–19 migration names must occur in the actual ordinary registration
functions: game `run_database_migrations`, worker 16–17
`apply_worker_migrations_locked`, and worker 18–19
`apply_worker_migrations_v2_locked`. Merely retaining an `include_str!` of the
SQL file elsewhere cannot satisfy that registration assertion.

## Additional full-mode inventories

Full mode retains checks for CEX receipt lookups and the fail-closed synchronous
backend, no blocking CEX client, signer receipt route, restrictive settlement
lineage foreign keys, stable remote identity and live-lease SQL markers,
remote-success versus campaign-application state, append-only operator evidence,
and the source tests for capture and operator replay.

Required direct-source test functions are identified as ordinary functions, not
by a generated-source constant that no longer exists. Qualification declarations
are read from the permanent V4 workflow files. They must select the required
settlement tests (explicitly or through game-server/workspace all-target tests),
format checking and strict all-target Clippy. Commented/echoed commands do not
satisfy the command inventory; source-write permission is rejected.

This workflow inventory is deliberately not a full YAML/shell interpreter.
Other CI-integrity gates and GitHub execution are still required. SQL source
markers are inventory checks, not SQL parsing, privilege verification or a
proof of transactional behavior.

## Budgets and errors

| Resource | Limit |
| --- | ---: |
| Each source file | 1 MiB |
| One expanded source bundle | 8 MiB |
| Included files per bundle | 256 |
| Lexical tokens per bundle | 1,000,000 |
| Nested comments/groups | 256 |
| Raw-string delimiter supported hashes | 0–255 |

Malformed supported literals, invalid scalar escapes, unbalanced inspected
groups, unsupported include expressions and exhausted budgets fail nonzero.
A successful message says `source-static only; no Rust/database/hosted evidence`.
No partial result is represented as success. A failed fixture/child test
propagates through the Bash driver because it uses `set -euo pipefail`.

## Tests and evidence boundary

The Python suite exercises positive and hostile phase fixtures, comment/string/
macro isolation, Rust lifetime and raw-literal handling, escaped SQL locks,
include safety and resource limits, the actual CLI and Bash argument forwarding,
and mutations of a reduced temporary copy of real source/migration/workflow
files. The original shell positive/negative fixture remains and runs before
the expanded suite. Imports do not write Python bytecode into the candidate.

Neither a fixture pass nor a source scan proves Rust type resolution, macro or
conditional-compilation semantics, arbitrary indirect/transitive network effects,
transaction lifetime on every path, process cancellation, actual SQL execution,
remote identity convergence or deployed safety. Semantic aliases, helper changes
and unsupported new syntax require review plus Rust/runtime tests; a source
scanner must not be used to waive those gates.

The fixed v13k artifact and this checker patch have separate identities. Local
passes on the qualified archive are not passes on the current remote PR head,
its prospective merge, or production. Hosted exact-head Rust/PostgreSQL,
independent review, cross-repository compatibility and all external release
requirements remain separately required by `CURRENT_PLAN.md`.

## Change, rollback and ownership

Changes to phase names, direct-source composition, migration registration,
recognized calls, budgets or test/workflow names must update this contract and
both positive and negative tests in the same review. Do not relax a failed
condition simply to obtain a green result. Preserve the first failing fixture
and add a targeted regression before changing scanner behavior.

Rollback this nine-file checker/status tranche together. Do not restore semantic
source generation or an obsolete workflow to satisfy an old scanner. A rollback
or checker edit never grants source-publication, hosted, custody, commercial or
production authorization.


## Runtime-status integration and deliberately unclosed feature gate

The runtime-status checker now reuses the same direct-source reader instead of
requiring `build.rs`, `cex.rs.in` or generated CEX/game-server/worker wrappers.
It retains source inventory requirements for shutdown/drain, quarantine,
serialization, remote recovery and qualification declarations. It additionally
validates bounded JSON with unique keys, finite values and correctly typed empty
evidence arrays. The status remains an unverified candidate with no release
effect and no recorded workflow, artifact or reviewer evidence.

The status branch/base identify the operative branch and the observed
`c93dad9ff07e5f26c059fb36abdf7095055388e1` input. They are not a claim that the
local patch or qualified payload is published at that commit. The obsolete
build-generation control is replaced by an ordinary-source control, and the
limitations explicitly defer current CEX identity to `CURRENT_PLAN.md`.

The combined underscore-named entry point now uses `run_exact_head_v4_checks`,
which is the existing status contract's exact CI gate name, rather than a
conflicting older alias. It also invokes both negative-test drivers; the active
V4 workflow already calls this entry point. No workflow source-write permission
or new publisher is introduced.

A passing static phase scan is not necessarily a passing combined gate. The
fixed v13k artifact's `trnm-game-server/Cargo.toml` (blob
`93749584aac91b7b7ad0569cb5baec4ecc68ec1c`) enables the `reqwest` `blocking`
feature. The repaired runtime-status checker still rejects that feature. The
current PR input's manifest does not enable it; exact import of the historical
artifact would restore it. Preserve this finding as an open source-policy
blocker, not an excuse to weaken the checker or alter a pinned artifact in
place. A reviewed correction must reconcile nonblocking production dependencies
with tooling requirements and obtain new exact-source Rust/all-target evidence.

The 71-test source suite includes a hostile blocking-feature fixture
and a separately labelled temporary nonblocking fixture. That fixture's pass
validates checker behavior only; the real artifact is not silently changed.
The 34 status negatives include the original cases plus malformed evidence
containers, stale source controls, invalid dates, duplicate JSON, non-finite
values, excessive size and linked files. The combined real-artifact gate must
remain FAIL until the source-policy blocker is resolved and requalified.

## Integration status at this repair

The live candidate selected for this repair is PR #46 at
`c93dad9ff07e5f26c059fb36abdf7095055388e1`. The runtime status record is
`planned`, with a mandatory `publish_reviewed_direct_source_and_successor_manifest`
gate. The historical `implemented_controls` field is retained for schema
compatibility, but its items are the required integration inventory, **not**
assertions that these controls are integrated or verified on the live branch.
The source-publication requirement may not be removed while the semantic
generator/template remains. The status checker rejects an unsupported promotion
to `implemented_pending_exact_commit_ci` as well as a fabricated remote pass.

This checker repair does not publish the blocked direct-source set. The fixed
artifact remains immutable; the separate nonblocking successor has different
bytes and needs new exact-head Rust/PostgreSQL qualification and independent
review. A pass in an isolated local derivative is not a pass of the remote PR.
