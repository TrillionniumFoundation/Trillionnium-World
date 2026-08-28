# World transition HTTPS fixture v1

## Plan position

This is the next non-UI World slice after the frozen
`trnm_world_transition_v1` contract. It gives the World owning repository an
executable transport fixture so TrillionniumGame and Integration can run real
response-loss, restart and stale-fencing evidence without treating a Game-owned
mock as the World component.

## Exact boundary

The service owns only deterministic fixture execution and unsigned result
material. It cannot authenticate players, assign participant roles, choose the
canonical global sequence, update match versions, own idempotency, create
canonical roots, sign `MatchCompletedV1`, mutate a wallet or settle value.

The implementation is isolated under:

```text
tools/trnm-world-transition-https-v1/
```

It uses only the Go standard library and has no dependency on Bevy, the World
client, the compatibility game server, Nakama, PostgreSQL or another repository
checkout.

## Transaction and ambiguity model

For a canonical request, the request hash is the durable idempotency key.
Before a successful result is exposed, the service writes exact result bytes to
an owning-directory result store using file fsync, atomic rename and directory
fsync. A repeated request returns the stored bytes and never recomputes a
second, potentially divergent result.

The response-drop proxy remains an Integration/Game test component. The World
fixture itself does not simulate a network failure after persistence; it simply
provides the durable upstream behavior needed to prove exact retry.

## Supported fixture domain

```text
ruleset: blackbox-ruleset-v1
content: blackbox-content-v1
state schema: trnm.blackbox.state.v1
command schema: trnm.blackbox.move.v1
```

The state contains one signed-i64 counter. `advance` applies a bounded signed
integer delta and advances the deterministic tick by one. `reject` emits the
stable `domain_rejected` result.

This finite domain is intentionally not promoted as the production RTS ruleset.

## Automated acceptance

The exact-head workflow must run:

- source and authority boundary checks;
- malicious negative fixtures;
- Go formatting;
- unit tests;
- race tests;
- vet;
- static binary build;
- scratch image build with network disabled;
- a TLS 1.3 container smoke test proving exact duplicate response bytes;
- artifact hashes and image identity.

## Promotion blockers

This slice remains a fixture candidate until all of the following are present:

- exact-head World source/container workflow success;
- exact-head Game runtime workflow success;
- Integration exact World/Game component lock;
- deployed happy, response-loss, external-wait, after-reservation and
  after-verify evidence;
- representative accepted/rejected/restart/load corpus;
- zero unexplained deterministic divergence;
- multi-host fencing and endurance evidence;
- compatibility admission drain and rollback rehearsal.

Public online and public player markets remain disabled. This fixture cannot
satisfy production World ruleset, release, human, commercial or public-edge
evidence.
