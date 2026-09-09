---
status: implemented-candidate-unverified
owner: trillionnium-world
work_items: [WORLD-P1-002]
last_reviewed: 2026-09-05
review_due: 2026-09-19
release_effect: none
---

# RTS order intake v1: implementation contract

## Scope and compatibility

This is an **opt-in input policy**, `trnm_rts_order_intake_v1`, around the existing
`trnm_rts_order_protocol_v1` command vocabulary. It does not rename that vocabulary,
change its serializer/fingerprint bytes, change a simulation ruleset, or reinterpret
old replay files. Legacy `RtsFrameOrder::Deserialize` remains a compatibility
reader; it is not a strict or bounded untrusted-input boundary.

The new entry is `trnm_rts_protocol::strict::decode(&[u8])`, returning
`Result<ValidatedOrder, IntakeError>`. `ValidatedOrder` has a private constructor,
`as_order()` for a borrowed command and `into_order()` for ownership transfer.
Successful intake means only the shape and resource checks below passed. It is
not player authentication, controller ownership, sequence admission, idempotency,
map reachability, affordability, simulation success, or authority evidence.

**Runtime adoption remains open.** No existing client/server/Nakama route is
silently switched by this tranche. Before a route opts in, its owning adapter must
publish the selected intake profile and pass exact Rust/client/server negative
conformance on a pinned component set. A strict rejection must never fall back to
legacy parsing. Existing save/replay readers retain their named compatibility path.

## Wire envelope and implementation

```json
{"intake_contract":"trnm_rts_order_intake_v1","order":{"contract":"trnm_rts_order_protocol_v1","frame":1,"player_id":"player","subject_actor_ids":["hero"],"kind":"hold","source":"local_input"}}
```

Inputs are one UTF-8 JSON object. JSON whitespace and ordinary field ordering are
allowed; this envelope is **not** the separate canonical World-transition JSON
profile. The implementation uses typed Serde decoding without an intermediate
`serde_json::Value`, so duplicate fields cannot be overwritten before validation.
Custom object-only visitors reject positional arrays at the envelope, order and
coordinate-object boundaries. All three objects deny unknown fields. Invalid
UTF-8, unpaired Unicode surrogates, trailing material, null required fields,
wrong scalar types and unknown enum spellings are rejected.

| Field | Required / type | Meaning and validation owner |
|---|---|---|
| `intake_contract` | Required string | Must be exactly `trnm_rts_order_intake_v1`; no fallback |
| `order.contract` | Required string | Must equal `trnm_rts_order_protocol_v1` |
| `order.frame` | Required unsigned 32-bit JSON integer | Range 0–4,294,967,295; not a canonical online sequence |
| `order.player_id` | Required identifier | Shape only; authenticated ownership is checked by the caller |
| `order.subject_actor_ids` | Required identifier array | 1–256 unique entries; order is preserved, not sorted |
| `order.kind` | Required enum | One of the 28 command names below |
| `order.source` | Required enum | `local_input` or `replay`; caller-supplied, not authorization |
| `order.queued` | Optional boolean | Defaults to false; explicit null is rejected |
| `order.target_tile` | Optional object or null | Exactly integer `x`, `y`, each in signed 32-bit range |
| `order.target_actor_id` | Optional identifier or null | Empty string is not a missing target |
| `order.target_rule_id` | Optional identifier or null | A typed rule name; actual rule existence is a simulation check |
| `order.queue_id` | Optional identifier or null | Required for queued orders and specified job commands |
| `order.formation_id` | Optional identifier or null | Actual formation compatibility is checked by simulation |
| `order.raw_command_label` | Optional string or null | At most 256 UTF-8 bytes; display-only, never identity/authority |

Missing optional fields and explicit null are equivalent except `queued`. All
identifiers are nonempty, at most 160 UTF-8 bytes, and contain neither ASCII
control/space U+0000–U+0020 nor DEL U+007F. Unicode is preserved exactly; there is no
locale-dependent case conversion or normalization. Distinct Unicode spellings
remain distinct IDs. Null identifiers are allowed only in optional fields.

`frame`, `x` and `y` are typed integer inputs, not coercible strings or booleans.
Fractional/exponent forms, including `1.0` and `1e0`, are not accepted integer wire
forms. Geometry bounds deliberately remain with the named map/ruleset: a signed
coordinate fitting `i32` can pass intake but fail simulation validation.

## Command shape matrix

These are minimum structural requirements inherited from the legacy validator,
not permission checks or exhaustive gameplay preconditions. Additional *known*
optional fields are not silently removed. This policy does not invent mutual
exclusion where the legacy command contract permits both actor and tile targets.

| Command kinds | Required non-null command fields |
|---|---|
| `move`, `attack_move`, `patrol`, `recon` | `target_tile` |
| `attack`, `focus_fire`, `harvest`, `capture`, `extract`, `repair` | At least one of `target_actor_id`, `target_tile` |
| `ability`, `assign_group`, `append_group`, `remove_group`, `recall_group` | `target_rule_id` |
| `build` | `target_rule_id` and `target_tile` |
| `train`, `research`, `upgrade` | `target_rule_id` and `queue_id` |
| `cancel_queued_order`, `cancel_job`, `pause_job`, `resume_job`, `promote_job` | `queue_id` |
| `set_rally` | `queue_id` and `target_tile` |
| `set_stance` | `target_rule_id` is `hold_fire`, `guard` or `aggressive` |
| `stop`, `hold` | No additional target requirement |

Every kind requires at least one subject. Every order with `queued=true` requires
a nonempty `queue_id`. Rule lookup, subject/controller binding, dead-unit checks,
fog-of-war, cooldowns, construction/queue budgets, coordinates, costs and phase
validity remain deterministic simulation responsibilities.

## Resource and concurrency contract

| Resource | Hard intake ceiling |
|---|---:|
| Entire raw envelope | 131,072 bytes, checked before JSON decoding |
| Every identifier | 160 UTF-8 bytes |
| Subject count | 256 |
| Display label | 256 UTF-8 bytes |
| Frame | Unsigned 32-bit integer |
| Coordinate component | Signed 32-bit integer |

The 128 KiB raw bound includes whitespace and escaping. Per-field bounds are
checked after decoding, so JSON escapes cannot increase allowed identifier bytes.
This API is synchronous and owns only temporary decoded values and a bounded
subject set. It has no mutable global state, asynchronous task, DB/file lock,
transaction, remote call, credential or cancellation continuation. Errors publish
no command and mutate no simulation. Transport adapters must apply the same raw
body ceiling before buffering; this function cannot undo allocations made by a
caller before invocation. CPU/memory microbenchmarks and production throughput
are not claimed; these ceilings are safety bounds, not measured SLOs.

## Stable failures and precedence

All failures are non-retryable for the same bytes under the same profile. Error
messages equal a static code; raw payloads and Serde diagnostics are not reflected.

| Code | Condition |
|---|---|
| `resource_budget_exceeded` | Raw envelope, subject count, identifier or label exceeds its ceiling |
| `invalid_encoding` | Syntax, duplicate/unknown field, object/scalar/type/enum or integer encoding fails |
| `unsupported_intake_contract` | Well-shaped envelope names another input policy |
| `unsupported_order_contract` | Well-shaped order names another vocabulary |
| `invalid_identifier` | Empty identifier or forbidden ASCII bytes |
| `duplicate_subject` | Repeated decoded subject identity |
| `invalid_shape` | Missing required target, queue, stance or subject according to the matrix |

Precedence: raw size; complete typed decoding; intake version; order version;
subject count; player identifier; subjects in input order (identifier then
uniqueness); actor/rule/queue/formation identifiers in that order; label size;
legacy shape validator. This ordering is normative for the independent vectors.
A transport may expose its own HTTP status, but must not reinterpret a rejection
as player authorization failure or retryable network ambiguity.

## Schema and evidence mapping

| Artifact | Purpose |
|---|---|
| `trillionnium/crates/trnm-rts-protocol/src/strict.rs` | Directly compiled input-policy implementation |
| `docs/protocol/schemas/trnm-rts-order-intake-v1.schema.json` | Structural JSON Schema |
| `docs/protocol/vectors/trnm-rts-order-intake-v1.json` | Frozen raw input and expected result corpus |
| `trillionnium/crates/trnm-rts-protocol/tests/strict_intake.rs` | Rust corpus, byte limits, preservation and compatibility regressions |
| `scripts/check-trnm-rts-intake-conformance.py` | Independent Python raw parser/reference and reference-checker regressions |
| `.github/workflows/trnm-world-rts-intake-contract.yml` | Read-only exact-head and PR merge-object test definition |

JSON Schema operates on decoded data: it cannot detect overwritten duplicate
keys, prove the original integer spelling, count UTF-8 bytes, or enforce a raw
input byte ceiling. Its `maxLength` is a structural bound, not a replacement for
the decoder. Schema success alone never grants strict-intake acceptance.

```bash
python3 scripts/check-trnm-rts-intake-conformance.py --self-test
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-rts-protocol --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-rts-protocol --all-targets --locked -- -D warnings
```

The corpus covers every command kind and every stable failure family, duplicate
and escaped-duplicate keys, duplicate null fields, unknown nested fields,
positional arrays, required-field omission, numeric/identifier bounds and valid
legacy bytes. Extra Rust tests exercise invalid UTF-8, exact 128 KiB limits,
UTF-8-byte versus character lengths, subject count, nonreflecting diagnostics and
legacy serialization preservation. Python reference success does not prove the
Rust implementation ran or agrees; the Rust corpus run is a separate required
gate. No test in this tranche proves runtime adoption, online authorization,
Nakama cutover, deployment, custody, human validation or release eligibility.

## Adoption and change checklist

Before use on an untrusted route: bind profile to the versioned endpoint and
component lock, enforce body bounds before buffering, call this decoder without
fallback, separately authenticate player/controller/source/sequence, then pass
only the admitted typed order to deterministic simulation. Add black-box tests
showing rejected input never reaches the simulation. Record route owner and
legacy usage/retirement inventory. None of these adapter tasks is implied closed
by the library implementation.

A change to limits, object/type rules, identifier policy, required fields or
stable error precedence requires a new intake version and vectors. A change to
order serialization/fingerprints or gameplay semantics requires the separate
order/ruleset version process. Removing the legacy reader requires save/replay
migration evidence; tightening it in place is prohibited by this contract.
