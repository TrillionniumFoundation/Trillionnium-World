# TRNM Online Authority v3

Updated: 2026-07-15

## Contract and compatibility

Online Authority v3 is the current realtime match-authority contract:

- protocol `trnm_online_authority_v3`;
- build `trnm-online-authority-2026.07-v3`;
- state-stream subprotocol `trnm-online-stream-v1`.

The server accepts only exact protocol/build pairs. The exact Authority v2 pair,
`trnm_online_authority_v2` / `trnm-online-authority-2026.07-v2`, remains accepted
during the rolling-compatibility window; crossed v2/v3 protocol and build pairs
fail closed. Product, Operations and Production retain their independently
versioned v2 control-plane contracts. Authority v3 is not a Production v3
contract.

Offline saves remain a separate authority domain and cannot become an online
character, ranking, wallet entitlement or tradeable inventory source.

## Source and deployment status

The current feature-branch Authority v3 source completed its serial source gate
on 2026-07-15: locked workspace/all-target tests pass, the game-server library
passes 119/119 tests, workspace/all-target Clippy passes with `-D warnings`, and
format plus diff checks are clean. The isolated fault-evidence shell contract v2
passes. A transaction-wrapped V13 rehearsal against the live database verifies
pre-forgery rejection, atomic running fail-close plus marker creation, exact
summary maintenance, marker immutability and cursor-drift detection, then rolls
the migration and all fixture changes back. These are source and rollback-
rehearsal results, not deployment or black-box runtime credit.

Source-gate credit does not by itself establish deployment. Any runtime claim
for this tranche must cite a verified release-v2 ID and a post-promotion run
whose commit/tree/binary match that bundle; older E2E/netem artifacts remain
historical unless a row explicitly binds the newer release. Release-v2 binds
the exact commit, Git tree, binary, source manifest and Cargo/rustc toolchain
digest. The checker accepts only that contract and rejects legacy v1 or
toolchain-unbound bundles. These controls are provenance/integrity metadata
rather than a cryptographic release signature.

## Player input and server total order

Each member owns an independent monotonic `input_sequence`. The server validates
that cursor under the match transaction, assigns the shared `sequence` used as
the authoritative total order, and advances the member cursor and match
cursor/revision atomically. Two members may therefore submit from the same
observed match revision without predicting or competing for one client-owned
global sequence.

V3 records `client_observed_tick` and applies an accepted order at the current
authoritative simulation tick. `accepted_tick` is that effective tick. V2
clients supplied a future `target_tick` and their receipts retained that
requested-target value, but it did not create a delayed execution queue; the
legacy value must not be interpreted as a four- or eight-second application
delay. Command IDs remain request-fingerprint-bound: an exact retry returns its
stored receipt, and reuse with a different authenticated request is rejected.

The legacy `sequence` and `target_tick` request fields remain on the shared wire
shape for rolling compatibility and replay. V3 clients must also provide
`input_sequence` and `client_observed_tick`; their legacy global sequence is not
the authority cursor used to serialize V3 input.

## Realtime state stream

An authenticated member opens
`GET /v1/online/matches/:match_id/stream` with the player session in the
`x-trnm-player-session` header. The session is never placed in the URL. The
request and `Sec-WebSocket-Protocol` header must select
`trnm-online-stream-v1` with the current Authority v3 build.

The server sends a `full_snapshot` first. Subsequent messages are either a
top-level `snapshot_delta`, another full snapshot, or `resync_required`.
Messages carry an actor-generation UUID and monotonic state sequence; deltas
also bind the base state sequence, base snapshot hash and base tick. The native
client permits an actor-generation change only through a full reset, rejects
sequence, revision, tick or hash regression, and recomputes the decoded
`MissionSimV1` snapshot hash before publishing it to the game.

The server emits a full keyframe after crossing a 100-tick boundary and also
uses a full snapshot when the encoded delta would be at least 80 percent of the
full message. It revalidates identity every 60 seconds, sends a ping every 15
seconds, limits a member to two streams for one match, and bounds global stream
admission from configured match capacity. The stream is state-only; client text
or binary data frames are rejected.

While the stream is connected, the native client does not run the former 10 Hz
full-snapshot poll. A disconnected stream reconnects with bounded backoff, and
the client uses a low-rate HTTP full-snapshot refresh as recovery fallback. The
latest snapshot occupies one replaceable slot rather than an unbounded render
queue.

## Reconnect and receipt gaps

Authority v3 reconnect requires an explicit `next_receipt_sequence`. The server
rejects a future cursor, returns the current full snapshot, and returns at most
256 persisted receipts from the requested gap together with
`replay_from_sequence`, `next_receipt_sequence` and `replay_truncated`.

The reconnect transaction locks the match before its member row, bounds replay
below the locked durable cursor, and checks receipt continuity. Published actor
state is used only when its sequence and revision align with that durable
cursor; otherwise recovery uses the durable checkpoint or latest persisted
post-command state and fails closed if the authority cursor cannot be
reconstructed consistently.

Network delivery does not guarantee that the HTTP receipt arrives before the
corresponding state-stream update. Clients identify both using the durable
command/input and total-order cursors rather than assuming cross-connection
arrival order.

## Migration V10 and rolling writers

Migration `0010_online_realtime_input_v1.sql` adds
`trnm_online_match_members.next_input_sequence`,
`trnm_online_commands.input_sequence` and
`trnm_online_commands.client_observed_tick`. Existing commands are backfilled
in per-player server-total order, and existing member cursors are initialized
from their command counts.

The member cursor is non-null and non-negative. Command `input_sequence` remains
nullable only for the v2/v3 rolling-writer window and has a partial unique index
on `(match_id, player_id, input_sequence)`. A compatibility trigger assigns and
advances the member cursor when an exact v2 writer omits the new field. A later
fleet-wide contraction to `NOT NULL` requires retiring v2 rollback support; it
must not be folded into this migration.

This schema is designed for an exact-v2 rolling writer, not for arbitrary old
builds or indefinite downgrade support. On 2026-07-14 the preserved v2 server
and writer completed a full match against V10, and the preserved v2 client also
completed a full match against the v3 server. Those black-box probes must stay
in the release matrix while exact-v2 rollback is advertised; one passing run is
not indefinite downgrade support.

An already-running v2-owned match is not currently approved for live takeover
by a v3 instance. Rollout must drain v2-owned active matches before transferring
fleet ownership; the completed-match compatibility probes do not prove a
cross-version actor-generation handoff.

## Native durable command journal

The native client stores `trnm_online_command_journal_v1` under
`$XDG_STATE_HOME/trillionnium/online`, or the corresponding
`$HOME/.local/state` path, scoped by match/player/account. An explicit test path
may be supplied by environment. The journal contains command requests and
cursors but never the player session or another credential.

A command is inserted into the journal before it enters the bounded worker
queue. Journal replacement uses a mode-600 temporary file, file `fsync`, atomic
rename and parent-directory `fsync`; a sibling advisory lock prevents two
client processes from owning one journal. Invalid or truncated state is
quarantined and attachment fails closed. At most 16 exact attempts may be
pending.

On process start the worker replays pending attempts in FIFO order. Transport
errors, HTTP 408/429, recoverable authority errors and server errors retry the
same exact request with bounded backoff. Only an explicit player-input-cursor
conflict creates a new attempt and command ID for the same durable intent, and
the replacement is persisted before sending. A receipt is removed from the
journal only after its protocol, match, player, command, input cursor, observed
tick, revision and snapshot hash bind back to the exact pending request.

## Readiness

Readiness includes PostgreSQL, CEX, signer/registry, fleet epoch, database-pool
headroom and authority-clock checks. The current source judges clock health from
a recent 20-tick window of cadence drift and scheduling lateness plus last-wake
freshness; cumulative lifetime drift remains diagnostic and no longer creates a
permanent readiness failure after a recovered pause. Sustained slow cadence
still fails closed. Readiness also compares running matches assigned to the
current instance/epoch with its local actor registry and fails when an actor
drifts by two ticks, its published state is stale for two tick intervals, or the
active-match query fails. An idle green readiness response is a deployment smoke
result, not evidence of active-match endurance.

## Asynchronous command persistence

The match actor performs command validation and preparation before submitting
one prepared command to a bounded per-match persistence worker. Capacity is one
and each actor permits exactly one durable command in flight. While PostgreSQL
is pending, the private command lane may continue to simulate, but the old
public cursor is frozen. This prevents an old-cursor/high-tick publication from
surviving a crash beside a newly committed command snapshot at a lower tick.
Speculative state cannot enter periodic, shutdown, terminal, fencing or failure
checkpoints.

After durable commit, the actor releases its current speculative state through
the published-tick barrier and only then completes the HTTP receipt. Duplicate
or error races reload durable authority and deterministically replay autonomous
simulation from that durable state to the last private safe tick instead of
copying the speculative snapshot or rolling the actor back by several seconds.
The actor then re-baselines its rolling cadence window, so one recovered database
pause does not poison readiness for the actor lifetime while sustained slow
cadence still fails closed. A bounded receipt cache gives an exact queued retry
the same idempotent result. This design removes PostgreSQL waiting from actor
cadence, but the effect and receipt for a command remain bounded by its database
transaction. It has source tests and strict Clippy coverage; no post-change
100-ms database RTT or black-box rollback result exists yet.

## Published-tick crash recovery

Before an actor state or a newly accepted command receipt crosses the public
boundary, the server records a stable journal owner, physical host and instance
identity, match ID, actor generation/fleet epoch, tick, phase, global command
cursor, match revision, every member input cursor, receipt-replayability flag
and snapshot hash in a bounded single-writer journal.
The writer performs mode-600 temporary-file creation, file `fsync`, atomic
replacement and parent-directory `fsync` under a mode-700 run directory held by
one process lock. A second host-identity lock also prevents two journal roots on
one physical host from silently becoming separate durability domains; production
must therefore use one canonical journal directory per host. Actor simulation
does not perform filesystem I/O: publication candidates occupy a replaceable
slot and update the public watch only after the writer acknowledges durability.
An exact command snapshot is not coalesced away, and its HTTP receipt remains
behind the same acknowledgement barrier.

On local crash recovery the server starts from the PostgreSQL checkpoint or
latest exact durable post-command simulation. Both sources are independently
checked against their persisted tick/hash and the latest command must be exactly
`next_sequence - 1`. Recovery rejects journal or member cursors beyond the
database, deterministically steps autonomous simulation to the recorded
high-water tick, and verifies its snapshot hash before opening the actor. The
one legal DB-commit/journal-ACK crash window is bridged from the prior durable
cursor and checked as exactly one global/revision/member advance. Corruption,
rollback, missing bridge state or a hash mismatch fails closed.

The writer isolates a logical rejection to its match; only uncertain filesystem
durability closes the global writer. Every journal operation and actor worker
has a hard wall-clock deadline. A timed-out journal operation poisons the writer
for the rest of the process. Every queued request captures the current poison
generation and rechecks it before I/O, after I/O and before acknowledgement, so
a late successful `fsync` cannot let an already queued publication cross a
timeout boundary. Actor teardown uses bounded joins followed by abort so a stuck
database or filesystem worker cannot keep an old generation alive beside its
replacement. Poisoning also raises a process-fatal watch signal. The service
drains admission and actors within fixed deadlines and then exits nonzero via
the supervisor path, deliberately bypassing Tokio runtime teardown so an
uninterruptible `spawn_blocking` filesystem call cannot hang process restart.

A terminal receipt crosses the public boundary only after two database phases
and two filesystem phases. The first fenced transaction writes a private
terminal stage but does not change campaign, rating, season, replay, settlement
or public match truth. The terminal HWM is then `fsync`ed. A second fenced
transaction atomically applies every terminal projection, switches the match to
`complete` and inserts the exact publication ACK. The ACK binds actor
generation, full instance/physical-host/epoch ownership, terminal tick,
global/revision/member cursors, snapshot hash, phase, result hash and published
settlement state. Finally the exact HWM is sealed as a cold ACK tombstone and
the marker advances from `hot_pending` to `sealed`; only then may watch state or
the terminal receipt be released. A match merely being `complete` is never
enough for public credit or an HTTP-200 duplicate.

The hot HWM is the crash witness between database ACK commit and cold seal.
Startup may reconstruct a cold record from database state only for explicitly
marked pre-upgrade `legacy_bootstrap_pending` ACKs. A normal `hot_pending` row
must have the exact hot or already-written cold witness, and `sealed` must have
the exact cold witness; missing evidence or PostgreSQL rollback fails closed.
The cold record includes the complete HWM, result/settlement, immutable ACK
time, database system identifier, timeline and a post-commit flushed WAL upper
bound. `pending` may advance atomically to `settled`, but never regress. Cold
records do not consume the bounded 10,000-match hot set and live reconciliation
does not scan the entire history. Tombstone deletion is not implemented in the
live server; a future maintenance path must require an independently proven
PITR floor for the same database lineage and timeline whose WAL position is
later than every deleted tombstone. PostgreSQL statements, journal operations
and lock acquisition remain deadline-bound.

Failed-closed running matches use the same durability direction. The release-
bound maintenance command opens the canonical journal under its host locks,
accepts only an exact monotonic PostgreSQL successor of the retained hot HWM,
`fsync`s that final HWM, and atomically commits `running -> failed_closed` plus
an exact abandonment marker. It then seals an immutable abandonment cold
witness before unlinking the hot record and finally advances the database marker
to `sealed`. Waiting matches have no running HWM and use a separately checked,
strictly idempotent database-only path. Missing post-V13 markers are invariant
violations; only the explicit maintenance adoption flag may bind an exact
pre-V13 failed-closed row to its retained hot witness.

Terminal and abandonment witnesses share one manifest-v2 global seal sequence,
total count and tagged latest sentinel while retaining their distinct payload
schemas and sharded directories. The reader remains compatible with manifest
v1, but the first new seal upgrades it to v2. Startup accepts only exact honest-
crash overlaps, verifies both witness kinds against the unified manifest and
database O(1) summary, and blocks hot revival, cross-kind coexistence, lineage,
timeline, WAL, count or latest-sentinel rollback.

This journal is deliberately a **single-physical-host durability boundary**.
The process holds both host-local filesystem locks and a PostgreSQL K1 leader
lock. Every connection-pool session holds a shared K2 handoff barrier for its
whole physical-session lifetime; a replacement must acquire K1 and then K2
exclusively before it can publish its identity or claim a fleet epoch. This
linearizes same-cluster takeover even across different UIDs or containers.
Listener binding, journal validation, migrations, dependencies and safe
compaction finish before the database fleet epoch is incremented. Fleet
heartbeat, actor assignment, fence monitor, command/checkpoint/terminal commits,
terminal markers and duplicate receipts also bind the local physical-host
identity, instance ID and epoch; changing only the host column fences an already
loaded actor.
Cross-physical-host active-match takeover is blocked until the journal is backed
by replicated consensus/shared durability with proven RPO=0. The current lease
and assignment tables are fencing infrastructure, not a cross-host HA claim.
K1/K2 are database-cluster-local: a writable PITR clone and its old primary can
each acquire their own locks. Promotion therefore additionally requires
external STONITH proving the old primary is no longer writable; the server does
not claim cross-cluster split-brain prevention.

On SIGTERM the server first stops command admission with a recoverable 503 and
`Retry-After`, stops accepting new HTTP connections, and only then asks actors
to flush. Shutdown checkpoint barriers and all worker joins are deadline-bound;
failure removes the actor from readiness/reconciliation rather than waiting
past the recovery cap.

## Honest boundary

Authority v3 is a local realtime vertical slice, not a public-network or
commercial-release claim.

- The isolated database-path run
  `run/online-latency/pg-rtt100-1783989631-2381685/decision.json` installed
  50 ms each way only after the match entered `running` and observed 7,190
  netem packets during the effect window. Effect p95/max rose to
  3,158/3,416 ms, ACK p95/p99 to 2,748/3,137 ms and actor drift to 848.48
  ticks. All three design thresholds failed on the pre-refactor deployed path.
  The asynchronous state machine described above now exists and passes the
  current serial source gate, but the
  old failure remains the latest black-box database-latency evidence until a
  promoted build passes the same profile.
- HTTP acceptance latency is not authoritative-effect latency. A 2026-07-14
  port-scoped 100-ms RTT/1%-loss run measured 20 command-submit-to-hash-verified
  stream effects at p95 256 ms/max 364 ms and passed the p95 300-ms gate. This
  does not establish the same SLO under public Internet routing or production
  concurrency. The separately injected PostgreSQL result above explicitly
  fails the realtime SLO and must not be presented as a passing profile.
- The stream currently sends JSON full snapshots or top-level deltas. It is not
  evidence of compressed binary transport, large public concurrency or regional
  capacity.
- Client-command and server-published-tick journal unit coverage plus the
  automated two-process native smoke do not replace PostgreSQL kill-before-ACK,
  restart, rollback, ambiguous-commit and multi-client black-box fault drills.
- Active-match v2-to-v3 ownership takeover is unproven and must remain a drain
  boundary until a cross-generation recovery matrix passes.
- An independent clean 24-hour endurance run remains required. Earlier bounded
  local netem and soak evidence does not substitute for the new v3 path's final
  active-match evidence.
- Public TLS/mTLS edge, WAF/DDoS protection, cross-host or multi-region HA,
  off-host backup retention, KMS/HSM custody, staffed moderation/support and
  human multiplayer usability remain release gates.
