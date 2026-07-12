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
`trillionnium/crates/trnm-economy-protocol`, package version `2.1.0`. CEX
vendors that exact package version inside its own checkout, so it no longer
assumes a sibling `../Trillionnium` directory. CEX does not depend on removed
`trnm-world-api`, `trnm-world-domain` or `trnm-world-projection` crates. Its old
v1 protocol crate was removed from the workspace and replaced by an explicit
quarantine note; Git history remains the only migration reference.

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

`trnm_campaign_save_v1` schema revision 11 persists:

- `economy_mode` and account binding;
- wallet snapshot;
- bounded pending-intent outbox plus a separate priority compensation lane;
- verified receipts and idempotency keys;
- hard-fail dead letters;
- pending trade lifecycle;
- inventory rollback and explicit value-event payout policy;
- reconciliation cursor.

Quest, chapter, ending, battle and future player-trade values pass through a
single `ValueEvent` mapping. Each event declares `LocalSoftOnly`, `WalletOnly`
or `DualTrack`; only `DualTrack` intentionally issues both local and wallet
value. Battle settlement remains locally atomic and currently uses explicit
`DualTrack`, queuing one `ReleaseReward` intent per non-duplicate positive
reward. Quest/chapter/ending soft rewards use `CompleteContract` audit receipts
without duplicating their local credits into the wallet. Offline play reconciles through
`OfflineLocalEconomyBackend`. Connected play posts to CEX and advances only on
a receipt bound to the exact intent/term with a matching progression class.
Network/ledger failure stays pending across save/reload. A malformed or
mismatched receipt is moved to the dead-letter lane and cannot advance state.

Tradeable purchases use buyer `Reserve`, an escrow hold, then buyer-authorized
`Consume` to atomically commit the held amount to the seller. The seller is not
paid before consume succeeds.
Connected purchases require explicit valid ledger account IDs for both buyer
and market seller. A pre-commit cancellation refunds buyer escrow. A committed
cancellation first rolls back local inventory and then atomically debits the
seller and credits the buyer through `Chargeback`. Recovery intents use a
separate priority lane, so a failed ordinary FIFO head cannot block
compensation. Player-to-player listing discovery remains release-gated; the
current client exposes the trusted system-market settlement path only.

## Persistent CEX runtime

CEX migration `0027_add_trnm_native_economy_persistence.sql` owns unique
PostgreSQL records for intent IDs, `(scope,key)` idempotency, one receipt per
intent, reconciliation cursors and escrow purchases. Ledger mutation and
intent/receipt storage commit in one SQL transaction. The formal
`cex-trnm-ledger.service` and `cex-trnm-consumer.service` user-systemd units run
release binaries with `LEDGER_FAIL_FAST=true`; PostgreSQL absence prevents the
ledger from opening its port and never activates the development memory
fallback.

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

TRNM tests prove schema migration, asset separation, explicit value-event
policies, one-intent outbox deduplication, priority compensation over a failed
ordinary head, inventory rollback, crash/reload retry, recoverable hold,
corrupt-receipt dead letter and Reserve -> escrow -> Consume. CEX unit tests
retain in-process coverage, and
`scripts/check-trnm-native-economy-cross-process.sh` creates new persistent
accounts and proves reward exactly-once, identical replay before and after
service restart, held-escrow refund, committed escrow, chargeback reversal,
wallet/cursor recovery and unique PostgreSQL intent/receipt/entry records.

This is persistent local-production-profile integration evidence, not a claim
of high availability, public CEX exposure, commercial/legal readiness or
public player trading. Public listing, custody, matching, anti-abuse and
dispute/support operations remain blocked until human and release approval.
