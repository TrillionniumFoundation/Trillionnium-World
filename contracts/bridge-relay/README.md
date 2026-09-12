# bridge-relay

`bridge-relay` is the deterministic Rust MVP for proof-, nonce-, and finality-gated relay semantics.

Detailed design: [`../../docs/modules/contracts/bridge-relay-design.md`](../../docs/modules/contracts/bridge-relay-design.md)

## Purpose

The crate fixes a bounded state-machine contract for relay message identity, source and destination domains, proof binding, validator policy, nonce consumption, settlement finalization, replay protection, and normalized audit output. It is useful for unit, property, and future host-conformance work before a chain-specific runtime is selected.

## Authority and non-goals

This crate is not a production bridge, light client, validator, relayer fleet, custody contract, finality oracle, or deployed Chain component. A caller-supplied proof, height, receipt, or finality value has no authority until a separately versioned verifier and accountable source-chain contract validate it. Cross-chain value transfer and public-mainnet operation remain disabled unless explicitly scoped and independently approved.

## Public contract

The public surface includes typed chain, bridge, message, proof, validator, nonce, action, and settlement identifiers; relay and finalization requests; deterministic decisions and stable errors; configuration/version guards; and normalized audit events. Exact retry reuses the same relay identity and canonical material. Field names, proof hash domains, signature formats, finality policy, nonce windows, and error precedence are versioned compatibility material.

## State and invariants

Relay identity permanently binds source and destination domains, action, nonce, payload digest, proof digest, verifier/configuration version, and finality context. A nonce or proof cannot be reused for altered material; finality cannot regress; settlement finalization is terminal; unsupported domains and proof versions fail closed; and every rejected command preserves complete state. The MVP never infers finality from local time, file presence, or an unverified assertion.

## Host and durability boundary

The current implementation is an in-memory state machine. Production adoption requires chain-specific header and proof verification, reorganization policy, canonical checkpoints, durable request and decision records, atomic state-plus-audit publication, restart and replay recovery, old-writer fencing, bounded relayer queues, gas/storage accounting, and independently reviewed deployment artifacts. The pure state machine performs no RPC, SQL, filesystem, wall-clock, secret, or wallet operation while mutating.

## Verification

Run:

```bash
cargo test --manifest-path contracts/Cargo.toml --workspace --all-targets --locked
cargo clippy --manifest-path contracts/Cargo.toml --workspace --all-targets --locked -- -D warnings
python3 scripts/check-trnm-world-contract-module-documentation.py
```

Coverage must include exact and altered duplicate relay identities, nonce replay and window behavior, source/destination mismatch, proof and signature mismatch, insufficient or regressing finality, configuration-version drift, terminal finalization, bounds, state preservation, audit normalization, and property/fuzz cases. Real-chain, reorganization, host ABI, custody, and production evidence remain open.
