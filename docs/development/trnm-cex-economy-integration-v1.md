# TRNM Native Game / CEX Economy Integration v1

Status: current contract as of 2026-07-12.

## Authority boundary

TRNM keeps all high-frequency gameplay simulation local and deterministic:
soft credits, bound inventory, regional stock/demand, NPC production,
caravans, loot generation and RTS resources. CEX owns wallet credits,
tradeable-asset settlement, reserve/settle/consume/refund/chargeback, audit
receipts and wallet reconciliation. The CEX Web/Matrix World shell is not a
game client.

The stable protocol is owned by
`trillionnium/crates/trnm-economy-protocol`. CEX consumes that crate through a
workspace path dependency and does not depend on removed `trnm-world-api`,
`trnm-world-domain` or `trnm-world-projection` crates.

## Asset semantics

| Value | Transferability | Authority |
| --- | --- | --- |
| TRNM soft credits | bound | `trnm-campaign-core` |
| CEX wallet credits | tradeable | CEX ledger |
| Equipment/quest items | bound | `trnm-campaign-core` |
| Catalog material items | tradeable | Term Exchange/CEX |
| RTS cyan/supply/power | ephemeral | `trnm-rts-sim` |

No RTS tick, NPC work tick or regional-market tick calls CEX.

## Save and recovery contract

`trnm_campaign_save_v1` schema revision 10 persists:

- `economy_mode` and account binding;
- wallet snapshot;
- bounded pending-intent outbox;
- verified receipts and idempotency keys;
- hard-fail dead letters;
- pending trade lifecycle;
- reconciliation cursor.

Battle settlement remains locally atomic and queues one `ReleaseReward` intent
per non-duplicate positive reward. Offline play reconciles through
`OfflineLocalEconomyBackend`. Connected play posts to CEX and advances only on
a receipt bound to the exact intent/term with a matching progression class.
Network/ledger failure stays pending across save/reload. A malformed or
mismatched receipt is moved to the dead-letter lane and cannot advance state.

Tradeable purchases use buyer `Reserve`, seller `Settle`, then buyer `Consume`.
Connected purchases require explicit valid ledger account IDs for both buyer
and market seller. Cancellation/recovery emits typed Refund or Chargeback
intents. Player-to-player listing discovery remains a later product surface;
the current client exposes the typed system-market settlement path only.

## CEX HTTP surface

- `GET /v1/trillionnium/term-exchange/kernel/manifest`
- `GET /v1/trillionnium/economy/adapters/readiness`
- `POST /v1/trillionnium/economy/intents`
- `POST /v1/trillionnium/economy/wallet`

The old `/v1/trillionnium/world/adapters/readiness` route remains a compatibility
alias but no longer defines current product readiness.

## Native client configuration

- `TRNM_CEX_BASE_URL` (default `http://127.0.0.1:8090`)
- `TRNM_CEX_ENTRY_TOKEN` when ingress auth is enabled
- `TRNM_CEX_ACCOUNT_ID`
- `TRNM_CEX_ACTOR_ID` (defaults to the campaign character ID)
- `TRNM_CEX_MARKET_ACCOUNT_ID` for connected tradeable purchases

`Ctrl+F7` binds/reconciles; `Ctrl+Shift+F7` starts the current selected
tradeable purchase. The RPG UI shows soft balance, wallet available/reserved,
outbox, verified receipts and dead letters separately.

## Verification

TRNM tests prove schema migration, asset separation, one-intent outbox
deduplication, crash/reload retry, recoverable hold, corrupt-receipt dead letter
and Reserve -> Settle -> Consume. CEX tests start a real in-process ledger and
prove reward exactly-once, duplicate replay, reserve/refund,
reserve/chargeback, wallet reconciliation, typed receipt audit, service-down
fail-closed behavior and invalid-protocol rejection.

This is local integration evidence, not a claim of public CEX deployment,
production account provisioning, commercial/legal readiness or public player
trading.
