---
status: current
owner: trillionnium-world
applies_to:
  - WORLD-P0-002
  - WORLD-P0-003
  - WORLD-P0-004
  - deterministic-runtime
  - nakama-shadow
last_reviewed: 2026-08-28
review_due: 2026-09-11
---

# World → Nakama deterministic adapter and shadow program v1

## Decision

World publishes one unsigned, deterministic, language-neutral game-domain
contract. Nakama consumes that contract while retaining sole ownership of
participant admission, global event order, command idempotency, restart
recovery, canonical roots and signed completion evidence.

The World-local game server remains `legacy_local_alpha` laboratory machinery.
It is not promoted into a second canonical online protocol.

## Implemented World-side boundary

```text
contracts/world-runtime/v1/
  trnm-world-runtime-v1.schema.json
  trnm-world-shadow-v1.schema.json
  golden-vectors.json
  shadow-vectors.json
  error-catalog.json
  compatibility-matrix.json

contracts/world-runtime/rust/
  strict canonical JSON
  exact ruleset/content selection
  deterministic RTS executor

contracts/world-runtime/host/
  trnm-world-runtime-exec
  trnm-world-runtime-shadow-diff
  observation and typed divergence library
```

The execution host is Bevy-free and contains no transport, persistence,
signing or online-authority dependency. It accepts an already ordered local
batch and never creates a match-global cursor.

## Request and response binding

Every execution request binds:

- `contract_version`;
- exact ruleset id, version and digest;
- exact authored-content digest;
- deterministic initial state;
- contiguous local batch ordinals and typed command payloads.

Every successful response binds:

- initial-state hash;
- command-batch hash;
- final state and hash;
- game-owned outcome and hash;
- unsigned replay material and hash.

The shadow observation adds:

- exact implementation commit;
- full request hash under `trnm.world.shadow.v1.request`;
- success or deterministic error response;
- canonical response byte count;
- measured duration for evidence budgets.

## Shadow comparison semantics

A shadow pair is equivalent only when:

1. both observations are self-validating and contain no forbidden authority
   material;
2. request hashes match;
3. both paths either succeed or reject deterministically;
4. success paths match exact ruleset/content/input bindings, final state,
   outcome and replay material by both value and hash;
5. rejection paths match stable error code and recoverability;
6. candidate duration and response-size budgets pass.

Diagnostic error wording is not canonical and may differ. A claimed hash that
does not bind its material invalidates the observation rather than producing a
normal divergence.

## Evidence layers

| Layer | Owner | Current state | Credit |
|---|---|---|---|
| Runtime schema/canonical vectors | World | source implemented | source conformance only |
| World Rust executor | World | source implemented | deterministic implementation only |
| Independent Python shadow verifier | World repository, separate language | source implemented | comparator cross-check only |
| Independent Nakama consumer | Nakama | pending | required before adapter acceptance |
| Exact component lock | Integration | pending | required before cross-repository credit |
| Fixture shadow matrix | World/Nakama/Integration | pending | required before cutover |
| Load/resource shadow matrix | World/Nakama/Integration | pending | required before cutover |
| Authority drain/cutover rehearsal | Nakama/Integration | pending | required before canonical cutover |

## Promotion policy

An implementation revision is quarantined when any of the following occurs:

- invalid canonical JSON or material hash;
- authority-boundary field appears in World material;
- unknown contract/ruleset/content digest;
- request binding differs;
- unexplained state, outcome, replay or rejection divergence;
- candidate resource budget exceeds its approved limit;
- evidence lacks exact source/binary/environment/component-lock identity.

No majority vote, retry count or later matching sample can erase an unexplained
divergence. The failing pair remains evidence and requires a classified root
cause.

## Open cross-repository work

World-side source implementation does not complete WORLD-P0-003/004. Remaining:

- independent Nakama consumer and authenticated adapter;
- Integration vendor/component lock;
- fixed cross-language production RTS fixtures;
- load and failure matrix;
- new-match admission cutover;
- active compatibility-match drain;
- authority-disablement and rollback rehearsal.

Public online and public player markets remain disabled while these rows are
open.
