# trnm-world-transition-contract

This standalone, dependency-free Rust package is the reference implementation
for `trnm_world_transition_v1`.

It owns only deterministic World-domain transition material:

- exact contract, ruleset and content revisions;
- canonical prior state and command payloads;
- deterministic next state, replay and optional outcome material;
- domain-separated request, transition and World outcome hashes;
- stable typed rejection codes and resource budgets.

It does **not** own online admission, participant sessions, global event order,
command idempotency across the online service, canonical archive roots,
`MatchCompletedV1`, match-evidence signing, Chain finality or CEX settlement.
Those remain with their accountable systems.

## Test

```bash
cargo test --manifest-path trillionnium/contracts/trnm-world-transition-v1/Cargo.toml --locked
cargo clippy --manifest-path trillionnium/contracts/trnm-world-transition-v1/Cargo.toml --locked -- -D warnings
```

The package deliberately has no third-party dependencies. Canonical game payloads
are opaque, versioned JSON objects or arrays whose SHA-256 is verified at the
boundary. Full JSON/schema conformance is checked by
`scripts/check-trnm-world-transition-contract.sh` against the published schema
and golden vectors.
