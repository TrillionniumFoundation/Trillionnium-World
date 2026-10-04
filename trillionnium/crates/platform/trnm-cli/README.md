# `trnm-cli`

Status: **migration-only / alpha platform component**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

## Purpose

Platform command-line wallet, query, transaction, and operator interaction surface.

It owns command parsing, local query/transaction UX, validation, and safe exit/error reporting. It is not a production-secure keystore, HSM, remote signer, multisig, or recovery product.

## Invariants

- Commands never print or log private secrets.
- Invalid amounts, nonces, addresses, signatures, and incompatible versions fail before submission.
- Exit codes distinguish success, validation, transport, authority, and integrity failure.
- Human-readable output does not replace stable machine-readable output for automation.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-cli --locked
cargo run --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-cli -- --help
```

## Open gaps

Secure keystore, offline signing, import/export/recovery, multisig/HSM/remote signer, fee/nonce safety, rotation, and compromise-response flows remain P0 for public mainnet. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
