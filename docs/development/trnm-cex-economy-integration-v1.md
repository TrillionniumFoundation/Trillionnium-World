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
`trillionnium/crates/trnm-economy-protocol`, package version `2.3.0`. CEX
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

`trnm_campaign_save_v1` schema revision 12 persists:

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

Committed seller proceeds are credited and reserved together. They remain
unspendable through the 24-hour reversible window, so a chargeback consumes
the seller payout hold instead of depending on later seller liquidity. Matured
holds release lazily and atomically during seller wallet reconciliation.

Battle `DualTrack` issuance is capped at 100 wallet credits per event and 300
per UTC budget day. Positive `ReleaseReward` requires a
`ServerSignedValueEntitlementV1` issued by the trusted game authority and
verified by CEX against actor, account, battle source, intent, amount, expiry
and HMAC key. Entitlement consumption and the daily budget increment occur in
the same PostgreSQL transaction as the ledger entry and receipt.
`CompleteContract` is audit-only and CEX rejects any non-zero amount. Local
soft credits are bound and never convert to wallet credits.
Connected campaign and intent identifiers are deterministically namespaced by
the bound CEX account, so two players using the same local slot name cannot
collide in the global intent or idempotency keys.

## Persistent CEX runtime

CEX migrations `0027_add_trnm_native_economy_persistence.sql` and
`0028_add_trnm_seller_hold_and_identity_recovery.sql` and
`0029_add_trnm_value_entitlements_and_player_sessions.sql` own unique
PostgreSQL records for intent IDs, `(scope,key)` idempotency, one receipt per
intent, reconciliation cursors and escrow purchases. Ledger mutation and
intent/receipt storage commit in one SQL transaction. The formal
`cex-trnm-ledger.service` and `cex-trnm-consumer.service` user-systemd units run
release binaries with `LEDGER_FAIL_FAST=true`; PostgreSQL absence prevents the
ledger from opening its port and never activates the development memory
fallback.

Player economy calls use a signed `trnm_player_session_v1`, not a shared
consumer-entry token. CEX persists session hash, player/account ownership,
device, recovery generation, expiry and revocation. Recovery, suspension and
closure revoke all live sessions. The shared entry token remains an internal
service credential only and is not configured in the distributable client.

PostgreSQL runs with `wal_level=replica`, WAL archival and a separate archive
volume. The physical base-backup/PITR drill restores to a named restore point,
proves later writes are absent, promotes the restored instance and writes to
it. This is real same-host recovery evidence, not multi-host quorum, fencing
or regional HA. A five-minute maintenance timer releases matured seller holds,
alerts on overdue holds and rebuilds the consumer receipt projection from the
PostgreSQL receipt source.

## CEX HTTP surface

- `GET /v1/trillionnium/term-exchange/kernel/manifest`
- `GET /v1/trillionnium/economy/adapters/readiness`
- `POST /v1/trillionnium/economy/intents`
- `POST /v1/trillionnium/economy/wallet`

The old `/v1/trillionnium/world/adapters/readiness` route remains a compatibility
alias but no longer defines current product readiness.

## Native client configuration

- `TRNM_CEX_BASE_URL` (default `http://127.0.0.1:8090`)
- `TRNM_CEX_PLAYER_SESSION` (player/account/device scoped; provisioned outside
  the distributable client)
- `TRNM_CEX_ACCOUNT_ID`
- `TRNM_CEX_ACTOR_ID` (defaults to the campaign character ID)
- `TRNM_CEX_MARKET_ACCOUNT_ID` for connected tradeable purchases

`Ctrl+F7` binds/reconciles; `Ctrl+Shift+F7` starts the current selected
tradeable purchase; `Ctrl+Shift+F8` cancels the latest purchase through the
priority compensation lane without colliding with Linux virtual-terminal
shortcuts (`Ctrl+Alt+F7` remains a compatibility chord). The RPG UI shows soft balance, wallet available/reserved,
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
`scripts/check_trnm_native_client_cex_e2e.sh` additionally drives the native
Bevy input system, verifies the same UI projection before and after service
restart, and cancels through the client while checking PostgreSQL and inventory.
`scripts/check_trnm_real_window_cex_e2e.sh` starts a real X11 window, navigates
title -> character -> market by injected window input, captures rendered PNG
frames and verifies purchase/restart/cancel state. It is automated rendered
window evidence, not a human usability session.
`scripts/check-trnm-value-entitlement-and-session.sh` proves unsigned rewards,
positive contracts and wrong-account sessions are rejected; enforces 100/300
budgets; and proves recovery/suspension revoke sessions.
`scripts/check-trnm-postgres-pitr-failover.sh` proves physical base backup,
archived-WAL point-in-time recovery and writable promotion.

This is persistent local-production-profile integration evidence, not a claim
of high availability, public CEX exposure, commercial/legal readiness or
public player trading. Public listing, custody, matching, anti-abuse and
dispute/support operations remain blocked until human and release approval.
