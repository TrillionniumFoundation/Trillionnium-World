---
status: implemented-candidate-unverified
owner: trillionnium-world
work_items: [WORLD-P1-002]
last_reviewed: 2026-09-05
review_due: 2026-09-19
release_effect: none
---

# RTS intake: wire-shape repair and executable differential contract

## Defect and scope

The intake-v1 schema and independent reference require `order.kind` and
`order.source` to be JSON strings. The original `WireOrder` used derived Serde
enums directly. In the pinned `serde_json` 1.0.149 implementation, `deserialize_enum`
accepts an externally tagged object as well as a string, and its map variant
reader deserializes the unit payload. Consequently `{"hold":null}` and
`{"local_input":null}` can follow the enum path despite violating the published
wire shape. This finding is based on the pinned dependency source, not a claimed
local Rust execution. The relevant primary source is `serde-rs/json`, tag
`v1.0.149`, `src/de.rs`, `deserialize_enum` and `VariantAccess::unit_variant`.

`strict.rs` now deserializes these two fields as strings before converting the
spelling into the legacy enum. Object, array, null, boolean and numeric enum
representations return `invalid_encoding`. This is an implementation correction
to the existing string-only intake policy, not a new gameplay rule or a change
to the legacy replay reader. Valid command serialization and fingerprint material
must remain identical. No runtime adapter is enabled or implicitly migrated.

## Implementation and test boundaries

| Component | Responsibility |
|---|---|
| `trnm-rts-protocol/src/strict.rs` | Enforce string-only enum fields, object-only struct boundaries and the published intake-v1 checks |
| `trnm-rts-protocol/tests/strict_intake.rs` | Original 114-case corpus plus all 28 command enum-object aliases and both source aliases |
| `trnm-rts-protocol/examples/strict_intake_oracle.rs` | Execute the actual Rust decoder over raw bytes and emit code plus normalized legacy-order SHA-256 |
| `scripts/check-trnm-rts-intake-differential.py` | Generate deterministic adversarial inputs, run one real oracle process, compare every result and serialized-order hash with an independent reference |
| `scripts/test-trnm-rts-intake-differential.py` | Exercise runner failure handling with Python fixture executables, not Rust |

Paths in the first three rows are relative to `trillionnium/crates/`. The
normalization hash is a test observation only, never a new authority hash,
completion receipt, participant proof or wallet effect.

## Oracle protocol and budgets

The oracle is a local test executable, not a network service. Each stdin line
is lower-case hexadecimal for one raw input, followed by exactly one LF. Empty
raw input is represented by an empty line and must produce `invalid_encoding`.
Framing failures exit nonzero with a constant diagnostic; zero cases are not a
successful run. A case may contain at most 131,073 raw bytes so the input-limit
negative can be tested. There are at most 4,096 cases. Line reads are bounded
before allocation even if a caller never sends LF.

Each output line has exactly these fields:

```json
{"schema":"trnm_rts_intake_oracle_v1","sequence":0,"result":"accepted","order_sha256":"<64 lowercase hexadecimal characters>"}
```

On rejection `order_sha256` is null. `sequence` must be the exact zero-based
position, not a boolean, duplicate, missing or reordered value. The result count
must equal the input count. No diagnostic, additional field, truncated final
line, wrong schema, duplicate JSON member or malformed UTF-8 receives credit.

The Python runner requires POSIX process groups. It uses no shell, bounds input
at 16 MiB and combined stdout/stderr at 1 MiB, enforces a 30-second oracle deadline,
and kills/reaps the process group on timeout or exit. Binary input is limited to
64 MiB, rejects a symlink, and is SHA-256 checked before and after execution. This
is a test-run binding, not a full supply-chain or hostile-filesystem proof.
Stderr is not echoed or treated as success evidence.

## Deterministic matrix

The 361 cases include the unchanged 114 frozen cases, all 30 enum-object aliases,
field-type substitutions, UTF-8 identifier boundaries, duplicate/escaped keys,
subject and label limits, invalid raw UTF-8, a surrogate, integer spelling and
exact/over-limit raw input. Generation has no RNG or host-clock dependency.

The matrix digest uses ordered, length-prefixed case identifiers and raw bytes:
for each case append `u64be(id_bytes.len)`, `id_bytes`, `u64be(raw.len)`, `raw`,
then SHA-256 the complete sequence. The tested generation has digest
`acaf5778e76210c8111ab30137e3703eca044584595965c971659d5d6d2c5123`.
A changed digest requires review of the actual case set, not just a new count.

For accepted cases the reference independently supplies every legacy field in
declaration order, including absent optional fields as null and `queued=false`,
and compares the SHA-256 of those serialized bytes with the Rust observation.
Thus equal accept/reject counts with unequal serialization still fail.

## Commands and CI

```bash
python3 scripts/check-trnm-rts-intake-conformance.py --self-test
python3 scripts/test-trnm-rts-intake-differential.py
cargo +1.98.0 test --manifest-path trillionnium/Cargo.toml -p trnm-rts-protocol --all-targets --locked
cargo +1.98.0 build --manifest-path trillionnium/Cargo.toml -p trnm-rts-protocol --example strict_intake_oracle --locked
python3 scripts/check-trnm-rts-intake-differential.py \
  --oracle trillionnium/target/debug/examples/strict_intake_oracle \
  --result run/rts-intake-differential.json
```

The dedicated read-only RTS workflow builds the oracle and executes the full
comparison separately for the exact head and the validated prospective merge
object. The workflow pins the output directory so an inherited Cargo target
setting cannot silently select another binary. Each object's report is retained
beside its toolchain and command log; no CI step rewrites candidate source.

`--reference-only` is deliberately separate. It emits
`status=reference_checked_only`, `oracle_executed=false`. Omitting both an oracle
and that explicit flag fails. It cannot silently become a fallback when Cargo,
the binary or execution is missing. A successful actual comparison emits
`status=differential_passed` and the observed binary/output/matrix hashes.
The tool does not itself prove who compiled that binary: exact build logs,
source identity and independent review remain required evidence.

## Remaining acceptance

Local Python fixtures prove runner failure detection, not that Rust compiled or
agreed. Promotion still requires Rust format/tests/Clippy, actual oracle execution,
full-workspace compatibility, supported platform evidence and independent review.
Route adoption, authenticated/controller/sequence admission and proof that
rejected bytes never reach simulation remain open in the owning adapters.

`production_authorization=not_granted` and `runtime_wiring_proven=false` remain in
all reports. No generated status, fixture child or local comparison can satisfy
Nakama cutover, CEX settlement, deployed recovery, custody, human or release gates.
