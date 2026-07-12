# TRNM Online Authority v2

Updated: 2026-07-13

## Scope

v2 advances the bounded two-client co-op slice without claiming a public
online game. Each authenticated member now owns a separate PostgreSQL campaign,
progression state and inventory. One authoritative match can combine units from
both campaigns, while terminal settlement maps reports back to each member's
original unit identifiers and writes one immutable progression-provenance event
per player/account/campaign.

The wire contract is `trnm_online_authority_v2`, build
`trnm-online-authority-2026.07-v2`. Offline saves remain a separate authority
domain and cannot become an online character, ranking, wallet entitlement or
tradeable inventory source.

## Match value boundary

Positive online battle rewards use `ServerSignedValueEntitlementV2` and
Ed25519. `trnm-game-server` holds a 32-byte private seed in a mode-600 local
runtime file; the CEX ledger receives only a public issuer registry. The signed
payload binds issuer/key, actor/account, intent/amount/day, match, rules, build,
result hash, participant hash and one-time nonce. CEX requires the exact active
`key_id`/issuer pair and rejects malformed signatures, unknown keys, revoked
keys or changed match metadata before ledger mutation.

The legacy symmetric v1 entitlement remains compatible with the existing
trusted native/offline integration. The Online Authority v2 path does not call
the CEX entitlement-issuing endpoint and no signing secret enters a client.
Local key generation is `scripts/init-trnm-online-authority-keys.sh`; production
KMS/HSM custody and automated rotation remain a release gate.

## Reconnect and impairment

An authenticated member can call the reconnect route with its last acknowledged
sequence and snapshot hash. The server rejects future acknowledgements, records
the reconnect, returns the current full authority snapshot and replays at most
256 persisted command receipts from the requested gap. The native client uses
bounded same-request retries; if a lost request was not accepted and its target
tick expires, it refreshes authority and reschedules the semantic order. If the
first request reached the server but its response was lost, the unchanged
command id/request fingerprint returns the stored receipt.

`scripts/check-trnm-online-network-chaos.sh` applies a port-scoped Linux netem
qdisc to game-server traffic and runs the complete two-session match at:

- 50 ms latency / 1% packet loss;
- 100 ms latency / 3% packet loss;
- 200 ms latency / 5% packet loss.

Every profile must still prove restart/reconnect, strict command idempotency,
two independent progression events, terminal settlement and two reconciled CEX
wallets. The script refuses to replace a non-default loopback qdisc and always
removes its temporary qdisc on exit.

## Persistence additions

Migration `0002_online_authority_v2.sql` adds member campaign ownership,
member-specific settlement seeds/unit maps, reconnect state and
`trnm_online_progression_events`. The event table uniquely binds each match to
one event per player and account, stores result hash plus XP/reputation/inventory
deltas, and references the resulting campaign revision.

## Honest boundary

v2 proves independent local-host online characters, inventories, asymmetric
match entitlements and bounded impairment recovery. It does not provide public
registration UX, MFA/recovery/appeal drills, lobby/matchmaking, party/friends/
chat/guild, MMR/seasons/spectating, public inventory custody/listings, server
fleet/regions, cross-host HA/fencing, DDoS controls, KMS custody, moderation,
customer support, legal approval or human multiplayer usability evidence.
