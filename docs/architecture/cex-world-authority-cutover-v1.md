# CEX → Trillionnium World authority cutover v1

Status: source restoration candidate; production authorization is not granted.

## Decision

World gameplay, topology, movement, tactics, commerce, company/shop/work-order and progression state belong to `TrillionniumFoundation/Trillionnium-World`. CEX is an authenticated ingress and settlement/control-plane integration edge. It must not remain a second World state authority.

The standalone World split previously existed under the frozen `legacy-game` workspace and was removed in commit `2757a0fce9877128393ab1b4ba2470988e733cff`. This change recovers only the seven server-side crates from parent commit `d44d8930c917b55da7b23eb19e9645feb8f4ee59` into `trillionnium/crates/world-authority/`. It does not recover the retired Bevy client, old RTS runtime or historical release packet stack.

## Runtime boundary

```text
Matrix/Web/native caller
        |
        v
CEX authenticated ingress + normalization
        |
        | versioned World request / idempotency key / actor identity
        v
Trillionnium World API
        |
        v
World command authority -> durable WorldRepository -> source-versioned projection
        |
        +--> CEX ledger adapter for exact settlement only
        +--> evidence/metrics sinks
```

CEX may cache projections only when the response carries its World contract/state version. A cache is never a write authority.

## Restored source boundary

| Crate | Responsibility |
| --- | --- |
| `trnm-world-domain` | World state, nodes, positions, tasks, receipts, characters and tactics domain |
| `trnm-world-command` | Intent validation and deterministic state transitions |
| `trnm-world-projection` | Source-versioned home/map/route/read projections |
| `trnm-world-map-provider` | Map fixture/provider contracts, attribution and runtime budgets |
| `trnm-world-ui-fragments` | Rust-owned rendering fragments; no client-side authority |
| `trnm-world-api` | Versioned DTOs plus identity/session/ledger/repository/evidence/metrics traits |
| `trnm-world-server` | Standalone HTTP/runtime adapter, parity smokes and restart/reload repository tests |

## Fail-closed profiles

Development may use deterministic fixture adapters and the file-backed repository for parity and restart/reload evidence. These adapters are explicitly ineligible for production credit.

Production must refuse startup unless all of the following are configured and verified:

1. external identity resolution and authenticated session guard;
2. durable, fenced repository with one World writer;
3. exact ledger reserve/release adapter;
4. evidence and metrics sinks;
5. idempotency/replay policy;
6. migration watermark and exact reconciliation report;
7. rollback target and tested restoration path.

No configuration flag may silently fall back from production to fixture or file-backed adapters.

## Migration protocol

1. Freeze CEX World writes and identify the final CEX source watermark.
2. Export state with stable actor/entity IDs and content hashes.
3. Import into the World-owned durable repository under a migration epoch.
4. Reconcile counts and per-record hashes; any mismatch blocks cutover.
5. Enable CEX adapter shadow reads while all writes remain on the old side.
6. At the cutover fence, disable old writes before enabling the World writer.
7. Exercise success, timeout, replay, partial outage and rollback cases.
8. Remove or quarantine CEX embedded writers after parity evidence is exact.

There is never an interval in which both repositories may accept authoritative writes for the same actor or entity.

## Qualification layers

### Source qualification

`bash scripts/check-trnm-world-authority-cutover.sh` verifies the seven-crate closure, no external path dependency, format, tests, Clippy, versioned adapter/full-split responses and file-repository restart/reload behavior.

### Cutover qualification

Source qualification alone does not close the cross-repository blocker. Cutover qualification additionally requires:

- a World implementation commit and exact-SHA green CI artifact;
- a CEX adapter commit with local World writers disabled;
- exact backfill/reconciliation evidence;
- cross-repository success/timeout/replay/outage/rollback evidence;
- proof that no state has two authoritative writers;
- removal or explicit quarantine of frozen CEX World source.

Until every row is present, the machine-readable contract remains `production_authorization: not_granted`.
