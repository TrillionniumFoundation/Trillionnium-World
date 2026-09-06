---
status: current-candidate
owner: trillionnium-world
contract: trnm_world_canonical_json_profile_v1
applies_to_plan: trillionnium-world-development-2026-08-29-v4
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Determinism and Canonical JSON Profile v1

## Purpose

World and Nakama must derive identical accept/reject bytes and hashes from exact transition inputs. “Minified JSON” is insufficient: a parser must prove grammar, decoded key order, duplicate rejection, numeric range, escape minimality and depth before bytes receive canonical status.

## Deterministic execution envelope

A deterministic transition is a pure function of:

```text
contract revision
ruleset revision
content revision
prior canonical state bytes
command ID
command canonical payload bytes
expected deterministic tick
```

It must not depend on:

- wall clock;
- process or host identity;
- locale/timezone;
- OS iteration order;
- network/database state;
- floating-point nondeterminism in authoritative calculations;
- random input not carried explicitly in the request;
- mutable global state;
- authority credentials.

## Canonical JSON grammar profile

### Root

The root is an object or array. Scalars are allowed only as nested values.

### Encoding

- UTF-8 only;
- no BOM;
- no trailing bytes;
- no insignificant whitespace;
- one exact JSON value;
- maximum nesting depth 128.

### Objects

- keys are compared after JSON escape decoding;
- keys are strictly ascending by Unicode scalar sequence/UTF-8 lexical policy published in vectors;
- duplicate decoded keys are rejected;
- key escapes must be minimal;
- object order in accepted bytes is normative.

### Arrays

Array order is semantic and preserved exactly.

### Numbers

Only signed-i64 decimal integers are allowed:

```text
0
-1
9223372036854775807
-9223372036854775808
```

Rejected:

```text
-0
01
+1
1.0
1e3
NaN
Infinity
9223372036854775808
-9223372036854775809
```

### Strings

- valid UTF-8 after escape decoding;
- control characters must be escaped;
- use short escapes for quote, reverse solidus, backspace, form feed, newline, carriage return and tab;
- other U+0000–U+001F characters use lowercase `\u00xx`;
- printable noncontrol Unicode is emitted directly;
- unnecessary `\uXXXX` escapes are noncanonical;
- unpaired surrogates are invalid.

### Literals

Only lowercase `true`, `false` and `null` are valid.

## Exact validation algorithm

An implementation claiming canonical input must:

1. reject empty, oversized or invalid UTF-8 bytes;
2. parse one JSON value with explicit depth tracking;
3. reject unsupported number form before numeric conversion;
4. detect duplicate decoded keys while parsing;
5. require decoded keys in strict ascending order;
6. recursively reject forbidden authority keys after decoding;
7. re-encode the parsed value under this profile;
8. require byte-for-byte equality with the original input;
9. compute SHA-256 only after all checks pass.

Bracket balancing or whitespace scanning alone is not a parser and cannot grant canonical status.

## Forbidden authority keys

The transition contract rejects decoded keys including:

- `nakama_session_token`
- `nakama_private_key`
- `match_authority_private_key`
- `canonical_archive_root`
- `chain_finality`
- `chain_app_hash`
- `match_completed_v1`
- `participant_admission_receipt`
- `global_event_cursor`

The scan is recursive and exact-key based. Escaping a key, changing raw byte case where prohibited by schema, or nesting it does not bypass the boundary. Game payload schemas should additionally use explicit allowlists.

## Resource budgets

| Material | Maximum canonical bytes |
| --- | ---: |
| prior/next state | 2 MiB each |
| command payload | 128 KiB |
| replay material | 2 MiB |
| outcome material | 512 KiB |
| rejection detail | 256 UTF-8 bytes |
| nesting depth | 128 |

Rulesets may publish lower limits but never exceed contract maxima without a new contract revision.

## Domain-separated hashes

```text
request_hash = SHA256(
  "trnm.world.transition.request.v1\n" || canonical_request_json
)

world_transition_hash = SHA256(
  "trnm.world.transition.accepted.v1\n" || canonical_accepted_facts_json
)

world_outcome_hash = SHA256(
  "trnm.world.outcome.v1\n" || canonical_outcome_binding_json
)
```

Length-prefixing must be used whenever concatenating variable components outside a self-delimiting canonical JSON object.

A hash is lower-case 64-character hexadecimal.

## State and replay separation

Canonical gameplay state, replay log and transport cursor are separate concepts:

- state hash covers deterministic gameplay state required for the next transition;
- replay root covers ordered command/replay material;
- transport cursor belongs to Nakama;
- checkpoints bind state hash, replay position/root and revision without embedding unbounded historical replay in every state snapshot.

This separation prevents quadratic checkpoint growth and authority confusion.

## Deterministic containers and arithmetic

- authoritative maps/sets use stable ordering or explicit sorted iteration;
- integer overflow behavior is explicit and checked; saturation is used only when the contract defines saturation;
- authoritative floating point is forbidden unless a separately versioned reproducibility proof exists;
- random choices use a named algorithm/version and explicit seed in canonical state/request;
- time advances through deterministic ticks, not host time;
- platform-dependent paths, file metadata and process IDs never enter hashes.

## Positive vectors

Vectors bind:

- exact input bytes;
- parsed semantic value;
- expected canonical bytes;
- payload SHA-256;
- request/transition/outcome domain hashes;
- accepted/rejected exact response bytes.

## Mandatory negative vectors

At minimum:

- malformed missing value;
- trailing comma/data;
- whitespace outside strings;
- duplicate key;
- decoded-unsorted key;
- escaped duplicate key;
- leading zero and `-0`;
- float/exponent/NaN/overflow;
- nonminimal string escape;
- invalid UTF-8/unpaired surrogate;
- excessive depth;
- decoded forbidden authority key;
- size above each budget;
- hash mismatch;
- unknown contract/ruleset/content revision.

## Cross-language conformance

Promotion requires at least two independent implementations. They must:

- consume the same raw vector files;
- produce byte-identical results;
- reject every negative vector with a stable code family;
- record implementation revision/toolchain;
- run under the exact Integration component lock.

One implementation copied line-for-line into another language is not strong independent evidence.

## Compatibility

- Contract, ruleset, content and release revisions are distinct.
- Unknown revisions fail closed.
- Changing canonical grammar, key order, number model, escaping, hash preimage or stable errors requires a new contract version.
- Adding payload fields requires a new payload schema and vectors.
- Retirement requires usage inventory, shadow evidence, drain/rollback and Integration approval.