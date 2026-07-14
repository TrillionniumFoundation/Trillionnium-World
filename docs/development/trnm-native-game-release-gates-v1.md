# TRNM Native Game Release Gates v1

Updated: 2026-07-14

This matrix prevents software-alpha, commercial single-player, trusted CEX
settlement and public player-market claims from using the same denominator.
Historical World, Web/Matrix, Android and mainnet artifacts do not satisfy any
native-game row unless that row explicitly names them.

## Gate A — native software alpha

Status: **green**.

- eight current game crates; legacy working tree absent;
- deterministic RPG -> RTS -> RPG loop, atomic saves and replay;
- revision 12 economy state and protocol 2.3.0;
- local test, Clippy, format, release-build and native-window evidence.

The current feature-branch P0 tranche also passes the locked workspace/all-target
suite and strict workspace/all-target Clippy after correcting one test-fixture
initialization exposed by the first Clippy run. It includes ten-map lifecycle
replacement, RTS/campaign `MouseOnly` intents and a clean-snapshot immutable
game-server release contract. That tranche is not yet promoted or deployed, so
it does not replace the earlier runtime/window evidence.

## Gate B — commercial single-player candidate

Status: **blocked on external usability and distribution evidence**.

- three independent five-second observers remain required;
- one non-developer 10–15 minute session remains required;
- distribution, accessibility and support matrices remain release decisions.

Automated `MouseOnly` traversal and ten-map lifecycle coverage close source-code
defects; they do not satisfy the real-window or human-observation rows.

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

## Gate E — public online RPG + RTS

Status: **blocked**.

Online Authority v3 is the current local dedicated-server realtime slice. It
retains independent cloud campaigns/progression/inventory provenance,
PostgreSQL command state, idempotency/version/control-set enforcement,
authenticated reconnect, restart recovery, server-owned results and CEX
settlement. V3 adds per-player input cursors with a server-assigned total order,
current-tick application, a hash-checked WebSocket full/delta state stream and a
native durable exact-command journal. The exact Authority v2 pair remains
accepted only as rolling compatibility.

Earlier exact-v2 local netem profiles reach 200 ms/5% loss. The current v3
100-ms RTT/1%-loss profile additionally measures 20 command-submit-to-hash-
verified-stream effects at p95 256 ms/max 364 ms and passes its p95 300-ms
gate. This is still loopback laboratory evidence, not public-network capacity
or regional evidence. A separate database-only 100-ms RTT run in
`run/online-latency/pg-rtt100-1783989631-2381685/decision.json` definitively
fails the realtime gate: effect p95 is 3,158 ms, ACK p99 is 3,137 ms and actor
drift reaches 848.48 ticks despite a complete, exactly-once-settled match. That
run predates the current asynchronous-persistence source implementation. The
actor now validates/prepares synchronously, permits only one command in a
bounded per-match persistence worker, keeps uncommitted speculative state
private and freezes the old public cursor while PostgreSQL is pending.
Command-affected publication and its receipt remain behind durable completion;
after a failed or duplicate persistence result, the actor reloads durable
authority and deterministically catches up to its private safe tick before it
can publish again.
This removes the database wait from the 10-Hz actor cadence in source, but
command effect/ACK latency is still database-latency-bounded and the 100-ms
database profile has not yet been rerun against a promoted build.

The source also durably records each publicly visible tick/cursor/hash high-water
before publication and deterministically reconstructs it on local restart. This
is a mode-600, fsync/rename, single-writer **single-host** boundary; it is not a
replicated log, consensus or evidence of cross-host accepted-input RPO=0. The
locked workspace/all-target suite and strict Clippy pass, but the new Authority
source is not deployed and has not passed PostgreSQL kill-point/rollback drills.

Online Product v1 additionally proves a closed-alpha invite/login/rotation/
suspension-appeal lifecycle plus private two-member lobby, ready state and
`coop_vs_ai` allocation. Online Product v2 proves a native login/queue/launch
shell, ranked solo pairing, opposing human authority, persistent MMR,
friends/blocks and authenticated match-report resolution. Online Operations v2
adds native text/Linux-keyring login, auditable season rotation/archive,
integrity-verified replay frames/F9 inspection, replay-bound moderation,
enforcement appeals/SLA observability and epoch-fenced same-host fleet leases.
Online Production v1 removes private key material from the game process and
supports audited signer rotation. Online Production v2 adds signed
signer-possession/CEX-registry convergence, PostgreSQL-distributed admission,
serialized concurrent startup migrations, capacity samples, durable moderation
shifts, host challenge evidence and a native delayed-spectator view. These
remain local closed-alpha controls: the current signer is file-backed, not
KMS/HSM, and exactly one physical host is healthy.

Gate E remains blocked. It still requires an independent clean 24-hour
active-match endurance result, a post-change injected database-latency result,
client-command and server-published-tick journal crash/restart,
kill-before-ACK, database rollback and ambiguous-commit drills, plus continued exact-v2
rollback coverage while that compatibility is advertised. The 2026-07-14
preserved-v2 server/writer probe against V10 passed; it does not remove the need
to keep that probe in the release matrix. Active v2 matches must be drained
before v3 ownership because cross-version active-match takeover is not yet
proven. There is also no public self-registration, verified-contact
recovery, party queue, chat/guild, staffed moderation, KMS/HSM, public
TLS/WAF/DDoS evidence, off-host backup retention, cross-host multi-region HA or
human multiplayer usability result.

The clean-snapshot release bundle/check/promotion contract is implemented and
its mocked shell contract passes, but the current source has not been bundled,
promoted or restarted. Provenance hardening does not itself grant Gate E credit.

## Monetary policy

- local soft credits are bound and cannot convert to wallet credits;
- quest/chapter/ending rewards remain `LocalSoftOnly` with zero-value CEX audit;
- battle `DualTrack` wallet issuance is server-entitled and capped at 100 per
  event and 300 per UTC budget day; `CompleteContract` is always zero-value;
- public player listings remain disabled by the protocol policy manifest;
- seller proceeds are unavailable until the reversible payout window matures.

## Human truth boundary

Automated evidence cannot fill participant names or observations. Until the
human packet is complete, report Gate A and Gate C as green while Gates B, D
and E remain blocked.
