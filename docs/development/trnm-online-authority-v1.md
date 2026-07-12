# TRNM Online Authority v1

Updated: 2026-07-12

## Product decision

The first network product is a shared-campaign two-client co-op instance
against the existing deterministic RTS AI. It is not a seamless MMO and not a
public PvP launch. This preserves the strong RPG -> RTS -> RPG loop while
moving value-bearing gameplay state from local saves to a dedicated authority.

## Ownership

- native client: input, camera, rendering and presentation only while attached;
- `trnm-game-server`: campaign slot, map seed, match membership, controlled
  units, command sequence, `MissionSimV1`, snapshot, result and online reward;
- CEX: player session/account ownership, wallet, entitlement verification,
  ledger, receipt and reconciliation;
- PostgreSQL: durable authority for online campaign/match/member/command rows;
- offline mode: independent local authority and never the source of a public
  character, ranking, wallet entitlement or tradeable inventory.

## Wire and persistence

`trnm-online-protocol` 1.0.0 fixes
`trnm_online_authority_v1` / `trnm-online-authority-2026.07-v1`. A client must
present an active CEX player session owning the requested player/account. Build
or protocol mismatch is HTTP 426. Each match has one global monotonic command
sequence, a unique command id, expected match revision and bounded target-tick
window. Duplicate ids return the stored receipt; skipped sequences, stale
revisions and subjects outside the member control set fail closed. A duplicate
ID is idempotent only when its authenticated player and full request fingerprint
match the first accepted request; altered replays return conflict.

Migration `0001_online_authority_v1.sql` owns four TRNM tables:

- `trnm_online_campaigns`;
- `trnm_online_matches`;
- `trnm_online_match_members`;
- `trnm_online_commands`.

The service never falls back to memory. It starts only when PostgreSQL and CEX
identity/settlement readiness are green. The current local deployment listens
on `127.0.0.1:7005` via `trnm-game-server.service`.

## Native attach mode

Set `TRNM_ONLINE_AUTHORITY_URL`, `TRNM_ONLINE_MATCH_ID`,
`TRNM_CEX_ACTOR_ID`, `TRNM_CEX_ACCOUNT_ID` and
`TRNM_CEX_PLAYER_SESSION`. The Bevy client attaches to the server snapshot,
filters selection to the server-assigned units, sends primary RTS commands over
the online protocol, polls snapshots, and disables local simulation stepping,
checkpoint settlement and CEX reward creation. Missing/expired identity or
transport failure leaves the client fail-closed instead of resuming locally.

## Acceptance

`scripts/check-trnm-online-authority-e2e.sh` provisions two real CEX identities
and sessions, creates/joins/starts one match, verifies disjoint two-unit control,
then proves:

- duplicate command exactly-once behavior;
- altered duplicate-ID rejection via persisted request fingerprint;
- sequence skip, old build and cross-member control rejection;
- PostgreSQL state recovery across a real systemd game-server restart;
- a real command-driven terminal victory under server ticks;
- server-owned bounded victory reward -> CEX signed entitlement -> atomic
  intent/ledger/receipt -> wallet reconciliation;
- one campaign, one match, two members, unique command sequences and settled
  terminal state in PostgreSQL.

`scripts/check-trnm-online-native-two-client.sh` adds the presentation/input
boundary: it launches two real `trnm-first-contact` release processes on an
isolated X11 display, attaches each process with its own session and control
set, injects one command into each distinct window and requires PostgreSQL to
attribute one fingerprinted command to each authenticated player. This is an
automated native two-window smoke, not a human multiplayer session.

## Honest boundary

The guest controls companion units in the host campaign and has no independent
progression in v1. Public matchmaking, invitation UX, social systems, human PvP,
ranking, seasons, spectator product, fleet/regions, cross-host HA, KMS issuer
keys, broad network-chaos SLOs, public inventory custody and player listings are
not implemented. Real observers and a non-developer session remain pending.
