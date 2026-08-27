# TRNM Settlement Outbox Contract

Standalone, dependency-free Rust reference contract for the settlement outbox
state machine defined by ADR-0002.

It proves deterministic job identity, bounded lease duration, monotonic lease
generation, expired-lease takeover, stale-worker rejection, exact receipt
binding, idempotent success, retry scheduling and dead-letter exhaustion.

Run:

```bash
./scripts/check_trnm_settlement_outbox_contract.sh
```

This crate is intentionally outside the eight-product-crate workspace while the
runtime migration is being designed. It does not grant production settlement
credit until the database schema, async worker, runtime integration and complete
fault matrix are implemented.
