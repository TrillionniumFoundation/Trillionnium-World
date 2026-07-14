# TRNM Online Product v2

Updated: 2026-07-14

## Contract and scope

Online Product v2 is the first local ranked multiplayer product slice. Its
control-plane contract is `trnm_online_product_v2`, build
`trnm-online-product-2026.07-v2`; v1 private-lobby requests remain accepted for
closed-alpha compatibility. Authority commands remain version-pinned to
the current Online Authority v3 exact protocol/build pair. The exact Authority
v2 pair remains accepted during the rolling-compatibility window.

This is not a public-beta or commercial-release claim. It is a local,
PostgreSQL-persistent, two-player ranked PvP slice with a native product shell.

## Ranked solo queue and PvP authority

An authenticated player with an owned cloud campaign can join one ranked solo
queue ticket. Pairing is serialized per map, requires a rating gap no greater
than 400, excludes either-direction blocks and excludes repeat opponents for
ten minutes. Unmatched tickets expire after fifteen minutes. Duplicate active
tickets fail closed.

Pairing creates two audit-linked tickets, a `ranked_pvp` lobby, one durable
allocation and one authoritative match. Current native launches use Authority
v3, while the exact v2 pair remains available for rolling compatibility. The
simulation moves the second player's seeded units to a real opposing authority
side, disables enemy AI and accepts separate control-set-checked commands for
both humans. Cross-side unit theft, withdrawal and non-member access fail
closed. Server restart retains the simulation and command state.

Ranked terminal state writes two immutable rating events. Initial MMR is 1000,
the current K factor is 32, and each event stores before/after/delta plus the
server result hash. The paired deltas are zero-sum. Ranked matches deliberately
grant no campaign XP/items and no CEX entitlement while collusion, seasons and
commercial policy remain unfinished; two zero-delta progression provenance
rows make that policy auditable.

## Native product shell

`trnm-online-product` is a real Bevy/X11 product window. F1 performs CEX login,
F2 binds the cloud character, F3 joins ranked solo queue, F4 cancels, and F5
launches the scoped Authority client in a second native window. Credentials
come from the protected launcher environment, are never rendered or written to
the evidence file, and are not passed to the game process; only the signed
player session is handed off.

This is a usable deployment shell, not yet a username/password text-entry or
OS-keychain UI. Registration invitations and credential provisioning remain an
out-of-band closed-alpha operation.

## Social and moderation minimum

Product v2 adds pending/accepted/rejected friend requests, player blocks,
authenticated match-bound reports and moderator-token-protected resolution.
A block rejects existing friendship state, revokes pending private invites and
prevents public pairing in either direction. Reports require distinct players
who both belonged to the referenced match; duplicate same-category reports are
rejected and every resolution remains stored.

This is an intake/audit mechanism, not staffed 24/7 moderation, chat filtering,
voice safety, appeals SLA or legal/commercial approval.

## Acceptance

`scripts/check-trnm-online-product-v2-e2e.sh` proves registration/login,
friend acceptance, two-sided blocks, block-aware solo pairing, duplicate-ticket
rejection, real opposing human authority, cross-control rejection, commands
from both sides, systemd restart, terminal result, 1016/984 zero-sum MMR, two
rating events, zero CEX entitlements and authenticated report resolution.

`scripts/check-trnm-online-product-v2-native.sh` drives two release product
windows through F1/F2/F3/F5, observes one paired match and then observes two
separate release game windows. All four captures must cross a non-black rendered
pixel threshold; mere window creation is not accepted. It is automated native
navigation/rendering evidence, not a human multiplayer usability result.

## Honest boundary

Public self-registration, verified email/phone, MFA/passkeys, OS keychain,
party queue, direct-challenge UX, chat/guild, presence, seasons/leaderboards,
spectating, replay reports, sophisticated anti-cheat/collusion, staffed safety
operations, multi-host fleet/region routing/HA, public capacity/DDoS evidence,
KMS/HSM, public inventory listings, support/legal approval and real-human
multiplayer sessions remain blocked.
