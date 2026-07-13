# TRNM Online Production v1

Status: implemented and deployed on the local closed-alpha host. Automated
production-control evidence is green. Real people, a second physical host,
public edge traffic and KMS/HSM custody remain external release gates.

Production v1 uses contract `trnm_online_production_v1`, build
`trnm-online-production-2026.07-v1`. Operations v1/v2 remain accepted only with
their exact historical protocol/build pairs.

## Isolated value signer and rotation

`trnm-game-server` no longer reads, receives or parses an Ed25519 private seed.
Positive battle value is sent over a loopback-authenticated contract to the
separate `trnm-entitlement-signer` process. The signer alone reads the mode-600
seed, revalidates battle/result/participant bindings, the 1-100 credit event
budget and ten-minute envelope, then persists an immutable PostgreSQL signing
receipt. A request ID is exactly-once: identical retry returns the receipt even
after expiry or key rotation; altered retry returns 409.

`rotate-trnm-entitlement-signer-key.sh` atomically generates a new key, updates
the CEX public registry, restarts signer and ledger, optionally revokes the old
public key and does not restart the game server. This is an external-signer and
rotation boundary, not a KMS/HSM attestation. The current signer still reads a
local file seed.

## Production ingress

Axum enforces a 256-KiB body envelope and persistent-process fixed-window rate
limits. Control-plane routes default to 600 requests/minute per hashed session
and path. Snapshot/command/reconnect data-plane routes receive a bounded 20x
envelope so normal authoritative polling is not mistaken for abuse. A probe
with a 30/minute control limit proves 429 and an oversized request proves 413.
Non-loopback bind remains fail-closed because there is no approved WAF/rate
limit/DDoS deployment attestation.

## Season automation and safety SLA

A moderator may opt a scheduled season into automatic activation. The worker
uses the existing season advisory lock. Ranked tickets or created/running
matches defer rotation with a durable reason and rate-limited audit; once idle,
the prior integrity-clear board is archived and the due season activates
atomically. Matches remain bound to the season selected at start.

Overdue pending enforcement appeals generate one durable SLA escalation.
Approval or rejection closes it. This proves escalation software and audit,
not a staffed moderation team or a real response-time promise.

## Targeted delayed spectating

A match member may issue a high-entropy, target-player-bound, single-use invite
with a 30-600 second delay. The target authenticates with their own account and
receives a scoped read-only grant. Playback returns only authoritative frames
whose server creation time is older than the delay; commands, future frames and
the terminal frame remain withheld until eligible. A participant or stolen
token cannot become an unrelated spectator. This is closed-alpha delayed
spectating, not public discovery or unrestricted live broadcast.

## Fleet host identity

Fleet registration, match ownership and failover audit now carry a stable
hashed `physical_host_id` in addition to instance ID, epoch and region. This
makes real cross-host evidence measurable and prevents two processes on the
same machine from being counted as two physical hosts. The current environment
reports exactly one healthy physical host; no paired second node exists, so
cross-host failover is not claimed.

## Acceptance

- `scripts/check-trnm-online-production-v1-e2e.sh`: full value path before and
  after key rotation, signer idempotency/tamper rejection, private-key absence
  from the game process, delayed spectator grants, busy/deferred/automatic
  season transition, SLA escalation, 429/413 ingress gates and host count.
- `scripts/prepare-trnm-online-production-v1-second-host.sh`: creates the
  explicit pending packet for an actual second physical host; it grants no HA
  credit by itself.
- Product v1 remains the positive-value compatibility gate. Ranked play remains
  zero XP, zero items and zero CEX value.

Final local evidence is run `online-production-v1-1783921848-17452`. It rotates
from `trnm-online-ed25519-production-1783920029-14665` to
`trnm-online-ed25519-production-1783921873-20455` without changing the game
server PID, then completes match `57411b00-de57-4333-9464-b66d3cdc8aed`
with two new-key signing receipts and two CEX entitlements. Exact Operations v2
compatibility remains green in `online-operations-v2-1783920778-1497`,
same-ID fencing in `online-operations-fencing-1783920820-17769`, and native F9
playback in `online-operations-native-replay-1783920865-28041`.

## Honest boundary

No two independent non-developer players completed the prepared 10-15 minute
session. No second physical host, public edge/WAF/DDoS provider, KMS/HSM,
staffed moderation shift, verified public registration, MFA/passkey,
cross-platform keychain, chat/guild, public inventory custody/listing, support
or legal approval exists in this local evidence. Public binding and the public
player market remain blocked.
