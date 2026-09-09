---
status: source-candidate-rust-and-hosted-validation-pending
owner: trillionnium-world
work_items: [WORLD-P0-002, WORLD-P0-010]
last_reviewed: 2026-09-05
review_due: 2026-09-19
release_effect: none
---

# Transition negative-corpus binding v1

## Defect and bounded repair

At observed PR #46 head `9a57222d1eacc7059e549df9c62a79046e8ae8ea`, the
independent Python checker compares decoded authority keys case-sensitively.
The published JSON includes 24 rejection vectors, including three mixed-case
keys. The old checker accepts `case_folded_nakama_key` and fails its own corpus.
The Rust integration test separately hardcodes 21 rejection inputs and omits
those three published cases. Test presence does not establish corpus coverage.

The repair applies ASCII-only lowercasing at the Python decoded-key boundary,
including nested objects/arrays. It does not apply Unicode normalization or
Unicode case folding and does not reject authority words used only as values.
This matches the existing Rust parser's ASCII key policy. Contract versions,
production parser behavior, hashes, published JSON and resource limits do not
change.

## Single corpus and generated test data

The source of negative inputs remains:

`docs/protocol/vectors/trnm-world-transition-negative-v1.json`

The checked-in Rust data is:

`trillionnium/contracts/trnm-world-transition-v1/tests/fixtures/negative_vectors.rs`

Python renders each input as an ASCII Rust byte literal of the exact UTF-8
bytes. Quotes, backslashes, controls and non-ASCII bytes cannot change Rust
literal boundaries. Case names must be unique ASCII identifiers. The fixture
also records the SHA-256 of the complete original JSON bytes, including spacing
and final newline. Its `rustfmt::skip` attributes apply to test-data constants
only, so formatting cannot silently rewrite the renderer's byte identity.

The default Python conformance command regenerates in memory and compares the
fixture bytes. Missing, changed or stale data fails nonzero. It then executes
all published Python vectors and the pre-existing schema/source checks. It
never rewrites files. The Rust integration test checks the same raw JSON digest,
case-name uniqueness and minimum corpus size, and sends every fixture input to
`parse_canonical_bytes`. Both gates are required; a digest assertion alone does
not prove that a hand-modified list contains every case.

These gates prove accept/reject coverage only. Matching stable diagnostic codes
or error precedence across languages remains separate work. The `error` field
is required nonempty metadata, not a claimed cross-language diagnostic test.

## Operator regeneration

From repository root, print a proposed test-data update to a separate file:

```bash
python3 scripts/check-trnm-world-transition-conformance.py --print-negative-fixture > /tmp/trnm-negative-fixture.rs
```

Compare that output to the checked-in fixture, review changes to the published
corpus and explicitly copy the approved test data into the fixture path. Print
mode is not validation and emits no success record. There is no production
`build.rs`, source-template transform, semantic rewrite, automatic commit,
network call or repository write in this generator. No dependency is added.

Validation commands:

```bash
python3 scripts/check-trnm-world-transition-conformance.py
python3 scripts/test-trnm-world-transition-conformance.py
cargo test --manifest-path trillionnium/contracts/trnm-world-transition-v1/Cargo.toml --locked
cargo fmt --manifest-path trillionnium/contracts/trnm-world-transition-v1/Cargo.toml -- --check
cargo clippy --manifest-path trillionnium/contracts/trnm-world-transition-v1/Cargo.toml --locked -- -D warnings
```

The existing transition CI path invokes the Python conformance command and Rust
package tests. The additional Python unit suite can run independently; its
presence is not a claim that any hosted workflow has executed it.

## Evidence and integration limits

The local unit suite exercises casing, nesting, false-positive avoidance,
ASCII/Unicode policy, exact literal encoding, corpus identity, malformed
metadata, fixture deletion/tampering, read-only execution and the actual CLI.
Subtests within one test method are not counted as separate test functions.
The Rust test harness was authored here but must still compile and run on the
final exact head and prospective merge. Python tests and source inspection do
not prove Rust execution, compiler behavior, runtime adapter adoption, database
safety or cross-repository conformance.

This tranche does not publish the complete fixed v13k 73-write/two-deletion
source package. Its checker is a successor to that package's ASCII-folding fix
and must be preserved when reconciling the original artifact. The immutable
artifact retains its original hashes; its qualification cannot be relabelled
as qualification of this changed checker/harness or the live PR.

Required unresolved work: full direct-source publication, nonblocking and
reviewed-toolchain successor qualification, real exact-head/merge CI, server
controls, independent review, Nakama/CEX/Integration bindings, and external
release evidence. Public-online, player markets and production authorization
remain unchanged and ungranted.

## Change and rollback

Corpus additions require regeneration and both language gates on the same
unchanged candidate. A protocol/ruleset semantic change follows its own ADR,
version and compatibility policy; this fixture does not authorize one.
Rollback this checker, its unit tests and the Rust fixture/harness together.
Do not restore case-sensitive authority matching or omit a published negative
case merely to satisfy an old local expectation.
