# TRNM Online Operations v2

Updated: 2026-07-14

Status: implemented locally, automated release evidence green, external human
and cross-host production gates still blocked.

Operations v2 extends the current Authority v3 and Product v1/v2. The exact
Authority v2 pair remains accepted during the rolling-compatibility window.
The Operations contract itself remains `trnm_online_operations_v2`, build
`trnm-online-operations-2026.07-v2`; Operations v1 requests remain accepted only
with the exact v1 protocol/build.

## Fenced fleet leases

Every server registration increments a durable `instance_epoch`. Heartbeats
renew a five-second lease only when the process still owns that epoch. Match
ownership stores both instance ID and epoch; tick and terminal writes require
both. A replacement process with the same instance ID therefore fences the
older process instead of sharing authority. An expired owner may transfer under
a PostgreSQL row lock only on the same physical host, where the replacement
must also acquire the process-lifetime publication-journal lock. A separate
host-identity lock prevents the same physical-host identity from opening two
different journal roots; deployments must configure one canonical journal
directory per host. Physical-host mismatch is rejected even when instance ID
and epoch happen to match. Active-match cross-host transfer fails closed until
that journal is replicated with RPO=0.

Fleet routing excludes expired, draining, offline and full instances. The
moderator control plane can drain, reactivate or offline a zero-match instance,
with a durable audit. This is same-host replacement infrastructure; cross-host
quorum and regional HA are not claimed.

Process SIGTERM enters an application drain before actor shutdown: new command
admission returns a recoverable 503 with `Retry-After`, the HTTP listener stops
accepting connections, and only then are bounded checkpoint barriers and worker
joins used to flush actors. A worker that exceeds its deadline is aborted and
the actor is removed from readiness rather than allowing overlapping authority.

## Replay playback

Ranked matches now persist the initial simulation, periodic authoritative
checkpoints and the terminal frame. Playback returns the member-authorized
command timeline, frames and result only after recomputing the replay hash,
command count and terminal snapshot binding. Native Product v2 adds F9 to
restore the allocated match, or safely select the authenticated member's latest
completed replay after a later ticket was cancelled, and show the verified
hash/frame/command summary.
Playback rejects packages above 2,048 commands or 512 frames instead of
silently truncating or creating an unbounded response.
This is post-match inspection, not public live spectating.

## Season operations

The moderator-token control plane creates scheduled seasons, activates one
under an advisory transaction lock, closes the prior active season and archives
only integrity-clear leaderboard entries. Every action is audited. Closed or
under-review entries cannot silently reappear as final rank. Each ranked match
binds its season at start, and activation/close fails while a ranked ticket or
match is active, so a mid-match rotation cannot misattribute a late result.
Season rewards,
calendar automation and commercial policy remain out of scope.

## Appeals and safety SLA

An enforced player may create one authenticated appeal for their own active
enforcement. Appeals have a 72-hour due time, pending/overdue queue counts and
moderator approve/reject actions. Approval revokes the enforcement and writes a
moderation audit. This proves software workflow and SLA observability, not a
staffed response team or real support SLA.

## Public security boundary

The server now refuses every non-loopback bind. Current Ed25519 custody remains
a local mode-600 seed file, not KMS/HSM. No public bind can be enabled by a
single environment flag while edge rate limiting, DDoS protection, KMS/HSM and
approved deployment attestation are absent.

## Acceptance

- `scripts/check-trnm-online-operations-v2-e2e.sh`: v1 compatibility, replay
  playback integrity, duplicate appeal rejection, SLA queue, enforcement
  revocation and season rotation/archive.
- `scripts/check-trnm-online-operations-v2-fencing.sh`: same-ID duplicate
  process fencing, stale route 503, monotonic epochs, drain/activate/offline and
  audit.
- `scripts/check-trnm-online-operations-v2-native-replay.sh`: real X11 F9
  replay inspection with a structural rendered-frame gate.
- `scripts/prepare-trnm-online-operations-v2-human-session.sh`: no-credit
  two-player/one-observer 10-15 minute packet.

## Honest boundary

No independent human players completed the packet. Cross-host/region HA,
public traffic/capacity/DDoS evidence, KMS/HSM custody, live spectating, staffed
moderation, verified public registration, chat/guild, player inventory listing,
support and legal approval remain blocked. Ranked matches still grant zero RPG
XP, zero inventory and zero CEX value.
