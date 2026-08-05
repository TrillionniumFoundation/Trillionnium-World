# World Match Evidence Boundary v1

## Status

World is not the Nakama match-evidence signer and does not construct a Chain
research command. The earlier World-local signer and root implementation were
removed because they created a second real-time authority and used a roster
encoding incompatible with the canonical Nakama contract.

The authoritative match contract is owned and versioned by
`Trillionnium-Nakama`. Chain ingress, finality, inclusion proofs, and research
command semantics are owned by `Trillionnium-Chain`. Cross-repository evidence
and component locks are owned by `Trillionnium-Integration`.

## World-owned output

World may produce unsigned, deterministic game-domain material for a match:

- gameplay commands and payloads accepted by the selected ruleset;
- deterministic result and outcome facts;
- replay artifacts and game-owned archive locations;
- ruleset/content digests;
- a game-owned outcome hash whose canonical serialization is versioned by
  World.

These values are inputs to the authoritative runtime. They do not prove event
ordering, participant admission, archive completeness, or finality.

## Nakama-owned authority

Nakama alone owns:

- one-time signed match authorization consumption;
- participant roster identity and role framing;
- global event sequence and match version;
- command idempotency and restart recovery;
- canonical event, roster, and archive roots;
- `MatchCompletedV1` construction and authority signing.

World must never load the Nakama authority private key or re-sign completion
evidence. Consumers retrieve the durable archive through Nakama's authenticated
archive RPC and independently reproduce its roots before accepting the signed
completion.

## Chain boundary

World does not import Chain crates through sibling filesystem paths. A future
game-to-Chain adapter must consume an immutable published or exact-revision
contract and submit through canonical Chain ingress. Until that ingress and an
AppHash/finality-bound receipt are versioned, no World flow may claim canonical
settlement.
