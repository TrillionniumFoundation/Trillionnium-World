# Trillionnium World authority cutover workspace

This workspace restores only the standalone World **server-side authority boundary** that was previously extracted from CEX. It deliberately does not restore the retired Bevy client, legacy RTS stack, release-review packet machinery, or a second player product.

## Ownership

`TrillionniumFoundation/Trillionnium-World` owns World domain state, command decisions, map-provider contracts, projections, runtime API contracts and the standalone server. CEX may authenticate ingress, normalize requests, invoke the versioned API and cache source-versioned read projections. CEX must not mutate World state locally after cutover.

## Current qualification state

- Source state: `cutover_candidate`.
- Production authorization: `not_granted`.
- Development fixture adapters and the file-backed repository are allowed only for deterministic parity, restart and replay tests.
- A production deployment must provide fail-closed identity/session, durable repository, ledger, evidence and metrics adapters and must prove migration reconciliation with no dual writer.

## Included crates

- `trnm-world-domain`
- `trnm-world-command`
- `trnm-world-projection`
- `trnm-world-map-provider`
- `trnm-world-ui-fragments`
- `trnm-world-api`
- `trnm-world-server`

The source was recovered from Git commit `d44d8930c917b55da7b23eb19e9645feb8f4ee59` into a separate bounded workspace so current native First Contact and online-authority crates remain unchanged.

## Validation

From the repository root:

```bash
bash scripts/check-trnm-world-authority-cutover.sh
```

The gate checks the exact seven-crate dependency closure, rejects external/sibling-repository path dependencies, runs format/test/Clippy, and executes adapter, full-split and restart/reload repository smokes. A green result qualifies the source boundary only; production cutover still requires the evidence listed in `docs/contracts/trillionnium-world-authority-cutover-v1.json`.
