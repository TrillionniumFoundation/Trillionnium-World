# TRNM Online Operations v1

Updated: 2026-07-13

## Contract and scope

Online Operations v1 adds the operational product layer above Online Product
v2. Its API contract is `trnm_online_operations_v1`, build
`trnm-online-operations-2026.07-v1`. Authority v2 and Product v1/v2 remain
version-pinned and backward compatible.

This is a same-host multi-process operations slice, not a public-region or
commercial-launch claim.

## Native login and credential custody

The native `trnm-online-product` window no longer requires player ID or
credential environment variables. It accepts real keyboard text, switches
between player and credential fields with Tab, masks the credential and logs
or writes neither its value nor the player session into evidence.

F6 stores the credential in the Linux kernel user keyring through `keyctl`
stdin, F7 restores it and F8 removes it. The credential never appears in a
process argument. The environment path remains supported for closed-alpha
automation. This is Linux kernel-keyring evidence, not Windows Credential
Manager, macOS Keychain or a cross-platform passkey implementation.

## Season, leaderboard and replay provenance

The database owns one active season, per-season ratings and a top-100
leaderboard with an authenticated requester rank. Ranked terminal settlement
writes the all-time result, season association and an authoritative replay
index in the same transaction. The replay hash binds match ID, result hash,
participants and every ordered command request fingerprint.

Replay access requires match membership. A replay-bound report must present
the exact authoritative replay hash; altered hashes fail closed. An open
high-severity replay report moves both result events to `under_review` and
removes affected players from the public season board. Dismissal clears the
hold; action marks the events `voided`, so they remain excluded. Stored
all-time/season aggregates are retained for audit rather than destructively
rewritten.

## Integrity and moderation

Queue tickets store only a SHA-256 device identifier. Same-device tickets do
not pair. Pairing also enforces a ten-minute repeat-opponent cooldown and a
maximum of three unique mutual matches in 24 hours. The third permitted match
creates a medium-severity repeat-opponent signal.

The protected `trnm-moderation-console` lists report cases with replay and
integrity context. An action writes an append-only moderation audit and may
create a bounded ranked/online suspension; ranked queue entry checks the
active enforcement. This is deterministic minimum tooling, not staffed
moderation, behavior ML, chat/voice safety or an appeals SLA.

Ranked matches still grant zero campaign XP/items and zero CEX value while the
integrity and commercial policies remain pre-release.

## Fleet, routing and failover

Each game-server instance registers an ID, region, endpoint, capacity, build
and heartbeat. Matches receive an explicit instance/region owner. Only the
healthy owner advances them; a second instance may claim the row after the
owner heartbeat is older than three seconds. PostgreSQL row locks preserve one
writer and every takeover writes a failover record.

The route endpoint selects a healthy instance with remaining capacity,
preferring the requested region and otherwise returning an explicit
cross-region fallback. No eligible capacity returns 503. This proves
same-machine two-process ownership/failover; it does not prove separate hosts,
fencing against a partitioned old primary, regional quorum, load balancers or
public DDoS capacity.

## Acceptance

- `scripts/check-trnm-online-operations-v1-e2e.sh`: active season,
  leaderboard, exact replay hash, tamper rejection, replay report, leaderboard
  hold, console action, audit and ranked suspension.
- `scripts/check-trnm-online-operations-v1-fleet.sh`: two live instances,
  owner stop, heartbeat-expiry takeover, capacity 503, cross-region fallback,
  terminal replay/season result and zero value.
- `scripts/check-trnm-online-operations-v1-native-login.sh`: real X11 text
  entry, masked credential, kernel-keyring store, process restart/recovery,
  forget and two structurally rendered frames.
- `scripts/check-trnm-online-operations-v1-anti-collusion.sh`: same-device
  exclusion, three real ranked matches, repeat signal and fourth-pair daily
  rejection.
- `scripts/prepare-trnm-online-operations-v1-human-session.sh`: creates a
  no-credit 10-15 minute two-human observation packet. It never marks the
  human gate complete.

## Honest boundary

Two independent human players and an observer have not yet completed the
session packet. Public account verification, MFA/passkeys, cross-platform
keychains, chat/guild/presence, full season administration/rewards,
spectating, sophisticated anti-cheat/collusion, staffed safety operations,
cross-host/region HA, public capacity/DDoS, KMS/HSM, player inventory listings,
support and legal approval remain blocked.
