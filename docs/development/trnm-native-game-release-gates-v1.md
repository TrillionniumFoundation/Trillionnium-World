# TRNM Native Game Release Gates v1

Updated: 2026-07-12

This matrix prevents software-alpha, commercial single-player, trusted CEX
settlement and public player-market claims from using the same denominator.
Historical World, Web/Matrix, Android and mainnet artifacts do not satisfy any
native-game row unless that row explicitly names them.

## Gate A — native software alpha

Status: **green**.

- six current game crates; legacy working tree absent;
- deterministic RPG -> RTS -> RPG loop, atomic saves and replay;
- revision 12 economy state and protocol 2.3.0;
- local test, Clippy, format, release-build and native-window evidence.

## Gate B — commercial single-player candidate

Status: **blocked on external usability and distribution evidence**.

- three independent five-second observers remain required;
- one non-developer 10–15 minute session remains required;
- distribution, accessibility and support matrices remain release decisions.

## Gate C — trusted system-market CEX settlement

Status: **green for the persistent single-node local production profile**.

- PostgreSQL fail-fast ledger and atomic intent/receipt persistence;
- seller payout remains reserved through the reversible window;
- native Bevy input -> CEX HTTP -> PostgreSQL -> restart -> UI projection E2E;
- rotating recovery credential, backup/restore and cross-instance idempotency gates;
- server-signed value authorization, player/account/device session ownership,
  physical base backup, WAL/PITR and same-host promotion evidence;
- public discovery remains disabled.

## Gate D — public player market

Status: **blocked / disabled**.

Required before enabling:

- real-user registration, recovery, suspension and appeal drills;
- signed inventory custody and listing ownership proof;
- matching fairness, anti-cheat, abuse/rate-limit and fraud controls;
- dispute, chargeback, customer-support and seller-collateral operations;
- multi-host replication/fencing/HA, capacity, live-traffic and public-network security evidence;
- human usability, commercial and legal approval.

The current trusted seller UUID path must never be described as a public
player market.

## Monetary policy

- local soft credits are bound and cannot convert to wallet credits;
- quest/chapter/ending rewards remain `LocalSoftOnly` with zero-value CEX audit;
- battle `DualTrack` wallet issuance is server-entitled and capped at 100 per
  event and 300 per UTC budget day; `CompleteContract` is always zero-value;
- public player listings remain disabled by the protocol policy manifest;
- seller proceeds are unavailable until the reversible payout window matures.

## Human truth boundary

Automated evidence cannot fill participant names or observations. Until the
human packet is complete, report Gate A and Gate C as green while Gate B and
Gate D remain blocked.
