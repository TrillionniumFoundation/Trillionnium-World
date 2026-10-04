---
status: current-candidate
owner: trillionnium-world
applies_to_plan: trillionnium-world-development-2026-08-29-v4
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# World Authority and State Ownership v1

## Rule

Every mutable or signed artifact has one accountable owner. Replicas, caches, projections and transport envelopes do not become co-authorities.

## Canonical artifact registry

| Artifact | Canonical owner | World may | World must not |
| --- | --- | --- | --- |
| ruleset/content revision | World | publish digest and compatibility | let callers substitute an unbound sibling checkout |
| deterministic prior/next state | World transition contract | validate and transform exact canonical bytes | claim online admission or order from the state hash |
| World transition hash | World | bind request, state, replay and outcome facts | call it a canonical online archive root |
| World outcome hash | World | bind ruleset/content/outcome material | call it match completion, finality or settlement proof |
| participant admission | Nakama | receive bounded role/controller material | authenticate canonical participants itself after cutover |
| global command sequence | Nakama | process the command selected by Nakama | maintain a competing canonical total order |
| command idempotency | Nakama | preserve command ID in World material | accept the same online command through an independent authority lane |
| match version/generation | Nakama | echo validated correlation data | promote a World-local generation as canonical after cutover |
| canonical archive root | Nakama | provide unsigned replay material | construct the canonical root |
| `MatchCompletedV1` | Nakama | provide deterministic result facts | construct, sign or store the signing key |
| Chain receipt/AppHash/finality | Chain | carry correlation hashes | infer finality from World/Nakama local state |
| economic intent | World | create immutable player-facing intent | mutate wallet/ledger state directly |
| wallet receipt/balance | CEX | validate and apply exact receipt to campaign projection | create, backdate or silently reinterpret ledger success |
| cross-repository component lock | Integration | publish exact World source/artifact identity | self-assert external compatibility |

## World-local compatibility enclave

The existing game server owns a laboratory-only local authority profile for migration and rollback evidence. Its state is not interchangeable with Nakama canonical state.

Allowed:

- deterministic regression tests;
- migration readers and compatibility clients;
- local single-host recovery evidence;
- drain and rollback rehearsal;
- bounded settlement capture against its own terminal evidence.

Forbidden:

- new public admission protocols;
- new canonical completion signature generation;
- importing Nakama private keys;
- advertising its local sequence/root as target canonical online truth;
- cross-host/public-release claims without separate evidence;
- enabling public player markets.

## Command lifecycle ownership

```text
1. Client proposes command intent.
2. Nakama authenticates participant and controller role.
3. Nakama allocates canonical sequence/idempotency decision.
4. World validates deterministic game-domain command against exact state.
5. World returns accepted/rejected deterministic material.
6. Nakama persists canonical event and state/archive references.
7. Nakama handles retry, reconnect, recovery and final completion.
```

World rejection codes are deterministic game-domain facts. Nakama transport/admission errors remain Nakama facts. The two catalogues must not overload one code with different authority meaning.

## Completion lifecycle ownership

World may emit:

- terminal deterministic state;
- terminal tick;
- replay material;
- result/outcome material;
- World state/transition/outcome hashes;
- ruleset/content identities.

Nakama alone binds:

- admitted participant set;
- canonical command interval and archive roots;
- match/version/generation identity;
- completion timestamp under its clock policy;
- completion signature.

Chain and CEX consume only the exact evidence their own contracts require.

## Rollback ownership

| Layer | Rollback authority |
| --- | --- |
| deterministic transition before Nakama commit | discard World output; Nakama keeps prior canonical state |
| Nakama canonical event after commit | Nakama recovery/compensation policy; never ask World to rewrite history |
| settlement remote success before campaign apply | outbox receipt lookup and fenced apply |
| campaign apply conflict | mark capture stale/dead-letter and recapture under unchanged remote identity |
| Chain submission | Chain/Integration retry under exact idempotency and finality contract |
| public deployment | Operations release rollback under exact artifact selector |

## Fencing requirements

- Nakama authority generation and ownership fence all canonical event writes.
- World settlement jobs use lease owner, generation and expiry.
- Campaign apply uses exact revision and state-hash CAS.
- Terminal settlement capture binds exact terminal publication identity.
- Old database primary or old process must be unable to publish after failover.
- A stale component lock cannot be promoted after any component head changes.

## Observability

Metrics and logs must carry, where applicable:

- correlation/transition/command ID;
- Nakama match/version/generation and sequence;
- World ruleset/content and transition hash;
- settlement capture/job/remote request ID and lease generation;
- CEX intent/receipt ID and hash;
- exact source/release/component lock;
- authority profile (`offline_world_v1`, `world_legacy_local_alpha`, `nakama_canonical`).

Player-facing UI must distinguish authority and settlement posture without exposing credentials or implying pending work is complete.

## Cutover invariant

The final cutover is valid only when:

1. new canonical admission is Nakama-only;
2. existing World-local active matches are drained or separately proven transferable;
3. World-local completion-signing and public admission paths are disabled;
4. exact shadow evidence has zero unexplained divergence;
5. Integration binds exact World/Nakama/CEX/Chain interfaces;
6. rollback and authority-disablement rehearsal passes.