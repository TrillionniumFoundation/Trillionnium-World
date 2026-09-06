# Trillionnium World Protocol Contracts

Current candidate contracts:

- `trnm-world-transition-v1.md` — strict deterministic World transition boundary;
- `schemas/trnm-world-transition-v1.schema.json` — closed machine schema;
- `vectors/trnm-world-transition-v1.json` — positive/hash vectors;
- `vectors/trnm-world-transition-negative-v1.json` — adversarial canonical vectors;
- `trnm-world-http-api-v1.md` — HTTP ownership/auth/idempotency/resource contract;
- `trnm-world-websocket-v1.md` — compatibility stream/full/delta/reconnect contract;
- `trnm-world-error-catalog-v1.md` — stable error/retry semantics;
- `trnm-world-compatibility-matrix-v1.md` — version admission and retirement;
- `trnm-match-evidence-commitment-v1.md` — ownership limits for completion evidence;
- `trnm-settlement-receipt-recovery-v1.md` — signer/CEX ambiguity lookup contract.

A protocol document is not verification. Exact implementation/schema/vector
conformance and independent cross-repository evidence remain mandatory.
