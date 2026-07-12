# TRNM Online Product v1

Updated: 2026-07-13

## Product contract

Online Product v1 is a closed-alpha two-player co-op product slice built on
Online Authority v2. It is not public PvP or a seamless MMO. The product
control-plane contract is `trnm_online_product_v1`, build
`trnm-online-product-2026.07-v1`; the allocated battle continues to use the
version-pinned authority v2 protocol.

Ownership remains explicit:

- CEX ledger service owns registration invites, player credential/session,
  account ownership, suspension and appeal state;
- `trnm-game-server` owns lobby, invite, membership, ready revision, match
  allocation and the authoritative campaign/battle state;
- clients own input and presentation only.

## Closed-alpha account lifecycle

An administrator issues a time-bounded, database-backed registration invite
with a bounded use count. Registration consumes one use in the same transaction
that creates the ledger account and player identity. A consumed, expired,
revoked or unknown invite fails closed.

New credentials use randomly salted Argon2id. Legacy high-entropy recovery-key
hashes remain readable for migration, and the next successful rotation writes
Argon2id. Five failed logins in one 15-minute window create a durable five-minute
lock. Login does not distinguish unknown player IDs from wrong credentials.

Credential rotation increments the recovery generation and atomically revokes
every existing session. Suspension also revokes all sessions. A suspended
player can submit one pending appeal by proving its credential; only a scoped
ledger administrator can approve or reject it. Approval reactivates the
identity and appends the immutable identity audit chain.

This is a software lifecycle drill, not evidence of production email/phone
verification, MFA, staffed support or a real human appeal SLA.

## Lobby and match allocation

A player with an online cloud campaign can create one active private lobby.
Membership creation is serialized by a PostgreSQL advisory transaction lock.
The owner issues a 15-minute high-entropy invite bound to one target player.
Only that authenticated target can accept it with a campaign owned by the same
player/account session.

Lobby mutation uses an optimistic `lobby_revision`. Stale revisions, a stolen
invite, a third member, duplicate active lobby, non-member reads and non-owner
queue requests fail closed. Both members must explicitly become ready. The
owner can then enqueue `coop_vs_ai`; one durable allocation creates the two
authority members and starts the match. Allocation failure deletes the partial
waiting match and reopens the lobby.

Authenticated members can poll the lobby view. Matched lobbies and allocation
rows remain as audit evidence.

## Acceptance

`scripts/check-trnm-online-product-v1-e2e.sh` proves, with fresh PostgreSQL
rows and real services:

- invalid and consumed registration invites are rejected;
- Argon2id credentials, durable login lock, login, rotation and old-session
  revocation;
- suspension, denied login, credential-bound appeal, admin approval and new
  device login;
- stolen invite, duplicate active lobby, stale revision and non-owner queue
  rejection;
- private invite, two ready members and exactly one `coop_vs_ai` allocation;
- systemd restart/reconnect, 15 authoritative commands, two independent cloud
  progression events, two Ed25519 entitlements and two 25-credit wallets.

`scripts/check-trnm-online-native-two-client.sh` uses the same product
registration/lobby/allocation route before launching two independent release
windows and requiring one fingerprinted command from each control set. It is
automated rendering/input evidence, not a human multiplayer usability session.

## Honest boundary

Public registration, verified contact channels, MFA/passkeys, account linking,
staffed appeal operations, public lobby discovery, solo queue pairing, PvP/MMR,
seasons, chat/friends/guild/moderation, multi-region fleet, control-plane request
receipt idempotency under packet loss, DDoS protection, KMS/HSM custody, public
inventory custody/listings and legal approval remain blocked.
