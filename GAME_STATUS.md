# TRNM Game Status

Updated: 2026-07-23

This is the one-page status source for the current native RPG + real-time-strategy product. The older finite checklist in `docs/development/trnm-deep-rpg-complete-rts-v1-dod.md` is retained as a completed historical baseline, not as a claim that the broader deep-RPG + complete-RTS vision is 100%. The current local CEX economy integration, Online Authority v3 with exact-v2 rolling compatibility, Online Product v2, Online Operations v2 and Online Production v2 slices are enumerated below; none grants Android or public-launch credit.

Release denominators are separated by
`docs/development/trnm-native-game-release-gates-v1.md`: software alpha,
commercial single-player, trusted CEX settlement and public player market are
four different gates.

The bounded v2 checklist remains historical evidence; this page records the newer runtime state directly and does not create another artificial "100%" contract.

## Current decision

- Promoted online-authority baseline: `a3e1d6d7f`. The 2026-07-23 client candidate passes 59 focused First Contact tests, focused strict Clippy, format, Bash syntax and ShellCheck after the v3 evidence hardening. A clean-source native run, the complete locked workspace sweep and CI promotion remain required before this tranche receives release credit.
- Engineering posture: technical alpha. Player-facing posture: pre-alpha. Public RPG+RTS MMO/commercial beta: **NO-GO**.
- Active route: the current worktree closes the Stage 0 source tranche (real-clock frame/input/network timing v3, background CEX economy I/O, canonical status and a verifiable Linux bundle) and starts Stage 1 with a responsive player-first shell for one coherent 10–15 minute `NEW -> RPG -> RTS -> debrief -> town` vertical slice. New endpoints, systems and evidence-only scripts remain frozen unless they remove a current P0.
- Current P0s: release-bound 30/60 FPS plus input/network runtime evidence, clean isolated 24-hour authority evidence, real-human comprehension/play evidence, Windows/macOS signing and public distribution, and public multi-host/regional operations.
- `RELEASE_READINESS.md` remains the repository-wide chain/mainnet release verdict; it is not a second native-game status page. Native product scope and evidence are canonical here.

## Product boundary

- Product workspace: eight crates in `trillionnium/Cargo.toml`.
- Stable game-owned economy boundary: `trnm-economy-protocol`.
- Native client: `trnm-first-contact`.
- RPG/world vocabulary: `trnm-rpg-core`.
- Campaign, save, progression and settlement authority: `trnm-campaign-core`.
- Player-order contract: `trnm-rts-protocol`.
- Bevy-free deterministic battle authority: `trnm-rts-sim`.
- Versioned online wire contract: `trnm-online-protocol`.
- PostgreSQL-backed dedicated campaign/RTS authority: `trnm-game-server`.
- Historical legacy game workspace: absent; see `docs/archive/frozen-legacy-final-index-2026-07-11.md`.
- CEX settlement backend: optional HTTP runtime behind the game-owned protocol; the native client never uses the CEX Web/Matrix game shell.

## Current authored scope

### RPG

- twenty connected rooms across Mirror City, Signal Road, Glass Basin and the Ashen Fringe, with story locks and authoritative route planning; Glass Basin and Ashen Fringe each contain four traversable rooms rather than a two-node label;
- three exclusive clean-room sects, three mentors and ten relationship NPC definitions;
- fifteen authored regional quests across courier, investigation, supply, hunt, escort and training-trial archetypes; every quest owns a structurally distinct per-quest DAG whose reachable nodes drive real movement, mutually exclusive or skippable route choices, persisted consequence flags, deadline failure/retry and Direct/Diplomatic/Resourceful outcomes; all 45 approaches run route authority and Direct paths run the actual encounter rather than inserting a cleared flag;
- seven RPG encounter definitions, encounter-specific original combat logs, telegraphed enemy move trees, bleed/exposure/guard/stagger states and three techniques per sect; the player freely selects primary or secondary techniques during combat, can form a combo, receives typed intent-counter bonuses and keeps independent persistent mastery;
- seven attributes, three origins, three growth paths, three mastery titles and fifteen skill definitions;
- twenty-two shop/economy items, eight crafting recipes, equipment durability and repair;
- world time, stamina, rations, water, injuries, trust, rank, journal and route planning; all ten NPCs move between work/civic/rest rooms, expose five relationship stages plus three dialogue intents, and participate in persisted pairwise bonds, work output, social events and memories; autonomous produce/migrate/ally/conflict/publish-task goals now alter rooms, production, dialogue, relationships and world flags rather than being write-only;
- four persistent regional markets with distinct stock/demand, inventory/production/consumption/quest/social-event effects and multi-leg inter-region caravans; caravans occupy real world rooms along authored routes, retain integrity/risk/incidents and can be visibly escorted or intercepted before delivery;
- three independent five-quest chapters with chapter-specific protagonists and playable scene rooms; each chapter requires two authored testimony/confrontation beats before its irreversible choice, and all five endings continue through four-beat playable epilogues with fifteen ending-specific follow-up scenes plus a closing beat into persistent post-ending world state;
- three atomic save slots and schema migration for existing saves/settings.

The RPG layer uses only clean-room mechanics study of 白金英雄坛说. No source code, text, maps, NPC/task tables, art, music or proprietary data is copied.

### RTS

- ten authored 40x24 maps: four campaign missions plus Iron Delta, Night Watch Crossing, Glass Basin, Ember Orchard, Salt Marsh Divide and Cinder Crown Siege skirmishes;
- two typed factions with twelve unit archetypes;
- ten structure definitions and ten technology definitions, all mapped into authoritative runtime kinds/jobs and filtered by the selected player faction;
- authoritative worker logistics, shared resource nodes, supply, power, construction, repair, production, research and upgrades for both sides: player and enemy jobs are one side-tagged `SimJob` model; both sides now use one worker movement/gather/cargo/return function, one source-checked authority-job command inlet, one job progress/completion scheduler and the same builder/site executor. Side-specific unit/technology effects and AI strategic selection remain adapters/controllers rather than second job authorities;
- fog/recon, control groups, queued orders, formations, stance/patrol/stop, veterancy and deterministic congestion recovery;
- typed escort, defend, extract, destroy and capture objectives;
- Story / Standard / Veteran pressure and deterministic adaptive AI;
- independent title-menu skirmish setup with map, faction matchup, starting-resource, Objective/Score/Annihilation and selectable deterministic-seed configuration, plus durable loot and normal one-time RPG settlement; the campaign gate remains an additional in-world entry;
- twelve typed unit abilities and ten distinct structure functions are executed by the authority rather than existing as names or stat-only catalog rows;
- battle orders export to chunk-hashed `trnm_battle_replay_v2`, automatically migrate a verified v1 file in the client path, retain up to 65,536 orders, persist hashed seek checkpoints, save/load independently verified disk chunk directories and expose pause/play, checkpoint seek, 1x/2x/4x/8x plus a free W/A/S/D camera; the long-match gate verifies 1,200 orders across three chunks, over twice the legacy window;
- the balance gate runs 64 terminal-oriented samples across four canonical balance YAML maps, faction swaps, spawn swaps and four meaningful simulation seeds while exercising player/enemy spending, harvesting, production and research; it reports faction win delta, resource-efficiency delta, technology delta and terminal counts by map in addition to pressure/time metrics;
- a real authored-map Standard Annihilation path uses unmodified campaign growth attributes, sustained harvesting, supply, production, research, recon, field aid and typed anti-structure abilities to destroy the actual enemy force/base, verifies replay, settles once and returns through the real client debrief key path; K then M/T/Y/U/I then Enter remains covered as the complete setup/deploy chain;
- the atlas manifest loads an original transparent 8x3 identity atlas with twelve unique unit frames and ten unique structure frames. Every identity now also receives deterministic family-specific breathing/recoil geometry motion while retaining the existing six/five base sprite rows; this improves live distinction but is still not a claim of 22 wholly independent hand-drawn frame sets.

## Stable contracts

- `trnm_campaign_save_v1`, schema revision 12;
- `term_exchange_protocol_v2` / `term_exchange_backend_v2`;
- `trnm_battle_seed_v8`;
- `trnm_battle_result_v2`;
- `trnm_settlement_receipt_v1`;
- `trnm_rts_sim_v16` / `trnm_rts_sim_checkpoint_v16`;
- `trnm_player_settings_v2`.
- `trnm_online_authority_v3`, build `trnm-online-authority-2026.07-v3`; exact `trnm_online_authority_v2` / `trnm-online-authority-2026.07-v2` remains accepted during the rolling-compatibility window.
- `trnm_online_product_v2`, build `trnm-online-product-2026.07-v2` (v1 private-lobby requests remain compatible).
- `trnm_online_production_v2`, build `trnm-online-production-2026.07-v2`; exact Production v1 and Operations v1/v2 protocol/build pairs remain compatible.

## Product shell

- title NEW/LOAD/CONTINUE, independent slots, corrupt-slot isolation and resume guard;
- the title, resume and town shell now use a responsive 92%-width player-first hierarchy with a dedicated current objective, compact player actions and bounded button rows. A 1280x720 software-rendered X11 smoke pass covers title -> resume -> market without overlap or unsupported separator glyphs; this is visual engineering evidence, not human usability credit;
- authoritative pause, journal, progressive guide and character identity confirmation;
- low motion, input mode, three control-scheme profiles, subtitles/high contrast and live master-volume control;
- the current source implements real Bevy command/campaign buttons and shared
  keyboard, command-card and right-click intents. `MouseOnly` can traverse the
  authored new/continue -> character -> mentor/training -> equipment -> gate ->
  mission -> RTS -> debrief loop and can issue RTS move/attack/harvest orders;
- map transitions now retire the prior map's scoped terrain, resources,
  landmarks, structures, units, selection/objective and transient render
  entities before rebuilding from the selected YAML. Automated coverage checks
  all ten maps for IDs, coordinates, camera/objective state and stale entities;
- desktop installer assets under `packaging/` and `scripts/install_trnm_desktop.sh`;
- CI Linux distribution packaging at `scripts/package-trnm-game-release.sh` now bundles all four release binaries, First Contact assets, a portable launcher, desktop metadata, locked dependencies, internal/third-party license inventory, runtime requirements, a source/version manifest and SHA-256 checksums. `scripts/check-trnm-game-package.sh` rejects unsafe paths, links, missing payloads, hash/size drift and dirty-source CI bundles. This closes the complete-artifact shape for Linux only, not Windows/macOS signing or public distribution;
- deterministic performance matrix at `scripts/check_trnm_perf_matrix.sh`.

These MouseOnly and map-lifecycle statements are source and automated-test
facts. The current real-window smoke pass validates rendering only; pointer
feel, hit targets, five-second observers and an unguided non-developer session
have not yet passed.

Revision 12 separates local soft credits, CEX wallet credits, bound items,
tradeable items and ephemeral RTS resources. It persists account binding,
wallet snapshot, a bounded economic-intent outbox, a separate priority
compensation lane, verified receipts, explicit `ValueEvent` payout policies,
idempotency keys, dead letters, trade lifecycle and reconciliation cursor.
Offline play uses `OfflineLocalEconomyBackend`; connected play sends the same
typed intents to CEX. Quest, chapter, ending, battle and future trade values
are recorded with `LocalSoftOnly`, `WalletOnly` or explicit `DualTrack`
semantics; only `DualTrack` deliberately issues both local and wallet value.
Battle wallet issuance requires CEX-verified `ServerSignedValueEntitlementV1`
and is transactionally capped at 100 credits per event and 300 per UTC budget
day; positive `CompleteContract` is rejected and local soft credits are
permanently non-convertible.
Connected tradeable market purchases require explicit buyer and seller ledger
accounts and use buyer Reserve -> escrow hold -> atomic seller commit. Seller
proceeds remain reserved through a reversible payout window. Refund
and chargeback use the priority lane, refund held escrow or reverse a committed
trade, and roll back delivered inventory before compensation. Recoverable
network/ledger failures hold progression and survive save reload; malformed or
mismatched receipts fail closed.
Connected campaign and intent identifiers are deterministically scoped by the
bound CEX account, so different players' default local save names cannot
collide in the global idempotency ledger.

Online Authority v3 is a bounded realtime network vertical slice. Two distinct CEX
player sessions bring separate PostgreSQL campaigns into one co-op match
against the deterministic AI. The server owns both campaign JSON documents,
authored-map seed, `MissionSimV1`, tick, member-to-unit control sets, command
idempotency, per-player input cursors, a shared server total order, snapshots,
terminal result and per-member settlement. V3 commands apply at the current
authoritative tick. A session-header-authenticated WebSocket sends hash-checked
full snapshots and top-level deltas; the native client verifies actor generation,
state/base sequence, tick and decoded `MissionSimV1` hash before publishing state.
Each member receives an independently persisted XP/inventory provenance event.
Restart does not return authority to the client; authenticated reconnect returns
the full current snapshot plus a bounded, continuity-checked persisted
command-receipt gap. The native client writes each exact command attempt to a
mode-600, atomically replaced, process-locked journal before its bounded worker
queue and retries the same request after recoverable transport failures.

The current source also separates authority simulation cadence from command
PostgreSQL persistence. Actor-side validation feeds one bounded per-match
persistence worker with at most one durable command in flight; uncommitted
speculative state remains private while the old public cursor is frozen. Durable
commit releases the current command lane only after the host-local journal has
acknowledged tick/hash/phase/global/revision/per-member cursors, and only then
returns the command receipt. Failed or duplicate persistence reloads the durable
authority and deterministically replays autonomous ticks to the private safe
high-water instead of publishing a rollback. Terminal receipts additionally
wait for a private terminal stage, terminal HWM `fsync`, one fenced transaction
that atomically applies every terminal projection plus the exact publication
ACK, and a durable cold ACK tombstone. The ACK binds generation, full
instance/host/epoch ownership, terminal tick/hash/result/settlement and every
authority cursor; database `complete` alone cannot acknowledge a duplicate.
Hot HWM state remains the crash witness until the cold tombstone is sealed.
An exact running fail-close is now committed atomically with its abandonment
marker, then sealed through the same cold-first saga; waiting cleanup remains a
strict database-only path and pre-V13 adoption is explicit. Terminal and
abandonment witnesses share a manifest-v2 global sequence, count and tagged
latest sentinel, with exact startup overlap recovery and O(1) database-summary
gating. Cold evidence is not removed by live compaction; deletion requires an
explicit database-lineage and PITR-retention proof. Journal/database operations,
checkpoint barriers and worker joins are bounded; uncertain durability poisons
readiness, and SIGTERM stops command/HTTP admission before actor flush. One
physical host is restricted to one authority process and one canonical journal
root. Historical complete rows without an exact full-tuple ACK are quarantined
from campaign, replay, rating, leaderboard and settlement credit rather than
being grandfathered. Release `a3e1d6d7ffe4-e777cf2c4aa5-5524a70f78e8`
promoted this authority tranche and passed the release-bound 100-ms PostgreSQL
RTT profile. PostgreSQL kill-before-ACK, rollback and ambiguous-commit matrices
remain open. Current post-release client work adds fail-closed frame/input/network timing v3,
moves connected CEX reconciliation off the Bevy update thread and introduces
the responsive player-first shell. It passes the full locked workspace test and
strict-lint sweep plus a local rendered smoke pass, but has no promoted runtime
or human-play credit yet.

Positive online wallet rewards are match/result/participant-bound Ed25519
`ServerSignedValueEntitlementV2` values. Online Production v1 removes the
private seed from the game-server process: a separate loopback-authenticated
signer revalidates the authoritative binding, owns the mode-600 seed and
persists exactly-once signing receipts; CEX loads only the active/revoked public
issuer registry. Exact Authority v2 requests remain accepted for rolling
compatibility, but their future `target_tick` value was request metadata rather
than a delayed execution queue. The attached native V3 client still disables
local simulation and settlement.

Online Authority v3 alone is not a public online game claim. Online Product v2
supplies ranked solo pairing, opposing human authority, MMR and a minimum
friend/block/report control plane. Online Operations v2 adds auditable season
rotation/archive, a held/void-aware leaderboard, integrity-verified replay
frames and native F9 inspection, Linux kernel-keyring login, enforcement
appeals/SLA metrics, moderation audit and epoch-fenced same-host fleet leases.
Online Production v2 adds signed signer-possession challenges checked against
the active CEX public-key registry, PostgreSQL-distributed admission and
capacity sampling, serialized concurrent migrations, host challenge evidence,
durable moderation shifts and native delayed-spectator controls. There is still
no public beta, party queue, chat/guild,
staffed safety operation or cross-host/regional HA. The current signer is not
KMS/HSM custody. Offline local saves cannot be mixed with online characters.

Online Product v1 adds a closed-alpha control plane without changing the
authority boundary. Database-backed single-use registration invites create the
ledger account and identity atomically. New credentials use Argon2id; rotation
revokes old sessions, five failed logins create a durable temporary lock, and a
suspended player can submit one credential-bound appeal for scoped-admin
resolution. Private two-member lobbies own target-bound invites, optimistic
revisions, explicit ready state and one durable `coop_vs_ai` match allocation.
The allocated game is still the same protocol-neutral server-owned simulation and
settlement. See `docs/development/trnm-online-product-v1.md`.

Online Product v2 adds a native Bevy product shell and ranked head-to-head
authority. F1 logs in with a protected launcher credential, F2 connects the
cloud character, F3 joins solo queue and F5 launches the scoped game client.
Pairing is rating-banded, block-aware, repeat-opponent-cooled and ticket-expired.
The second member commands a real opposing simulation side; AI is disabled for
ranked matches. Terminal MMR is a two-row, result-hash-bound, zero-sum event.
Ranked play intentionally grants no campaign or CEX value while collusion and
commercial policy remain open. Friends, blocks, match-bound reports and scoped
moderator resolution are persistent. See
`docs/development/trnm-online-product-v2.md`.

Online Operations v1 makes the launcher accept native text credentials and use
the Linux kernel user keyring without rendering/logging the secret. Ranked
terminal state now writes an active-season rating association and a replay hash
over result, participants and ordered command fingerprints. Replay-bound reports
hold affected leaderboard rows; the protected moderation console persists audit,
void/dismiss decisions and bounded ranked/online enforcement. Pairing excludes
same-device tickets, applies a ten-minute opponent cooldown, permits no more than
three unique mutual matches per 24 hours and signals the third. Fleet instances
own capacity, heartbeat, region and match assignment. The current host-local
publication journal permits one authority process per physical host and blocks
active-match cross-host takeover; database fencing therefore does not yet
establish cross-host or regional HA. See
`docs/development/trnm-online-operations-v1.md`.

Online Operations v2 turns the replay index into a member-authorized playback
package with initial/checkpoint/terminal simulations, ordered commands and a
recomputed result/participant/command hash. The protected season control plane
creates scheduled seasons, atomically activates one, closes the prior season
and archives only integrity-clear ranks. Enforcement appeals are authenticated,
one-per-enforcement, due within 72 hours and can revoke the enforcement through
an audited moderator decision. Fleet ownership now binds both instance ID and a
monotonic epoch; five-second leases, fenced tick writes, drain/activate/offline
controls plus the host-local journal lock prevent an older same-ID process from
continuing authority. Cross-host active-match transfer remains blocked until
the publication log is replicated. Non-loopback binds fail closed while KMS/HSM and public
edge security remain absent. See
`docs/development/trnm-online-operations-v2.md`.

Online Production v1 moves the entitlement private key into a separately
hardened signer service. The game server sends an unsigned authoritative
envelope; the signer independently enforces amount/time/battle bindings and
stores a payload-bound receipt. Identical retries remain idempotent across
rotation and altered retries fail 409. Key rotation changes signer and CEX
registry state without restarting the game server. The production worker also
enforces request-body and per-session/path request limits, defers automatic
season transition while ranked state is active, emits one escalation per
overdue appeal and exposes target-bound, single-use spectator grants whose
frames remain server-time delayed. Fleet ownership now records a hashed
physical-host ID, so two local processes cannot be counted as two hosts. See
`docs/development/trnm-online-production-v1.md`.

Online Production v2 makes readiness prove that the isolated signer possesses
the private half of the exact active Ed25519 key CEX trusts. Admission windows
are shared through PostgreSQL across instances and fail closed on database
failure; migrations serialize under a transaction-scoped advisory lock, and
maintenance records bounded capacity samples. Host challenges make the current
one-host fact auditable, while moderation shifts own report/appeal claims until
resolution. The native shell exposes player-safe production status plus F10/F11
targeted delayed spectating without persisting the invite token. The provider
still reports `file_seed`, healthy host count remains one, and public-edge,
KMS/HSM, cross-host and staffed-safety flags remain false. See
`docs/development/trnm-online-production-v2.md`.

The client exposes local/wallet balances, pending intents, priority
compensations, value events, verified receipts and dead letters. `Ctrl+F7`
binds/reconciles with a player/account/device-scoped signed session from
`TRNM_CEX_*`; the distributable client no longer carries the shared CEX entry
token. Connected market
purchase also requires `TRNM_CEX_MARKET_ACCOUNT_ID`. High-frequency regional
markets, caravans, NPC production and RTS resources remain local deterministic
simulation and do not call CEX per tick. Public player listings remain gated;
the current path is a trusted system market, not an open player market.

The native client has a real buffered audio pipeline with two project-owned procedural WAV loops: Mirror City ambience and Signal battle pulse. Town and battle switch cleanly; ending/epilogue play uses a dedicated layered mix of both sources, and F8 updates all live players immediately. These are an original functional score system; additional composed cues, richer effects and a final mastered mix remain content work.

The control profiles alter live RTS input: Classic uses Q/W/E/R for move/attack/harvest/hold, Left Handed uses A/S/D/F, and Arrow Grid uses the four arrow keys (with WASD camera pan). They are not display-only labels.

The game-server release tooling now requires a clean Git snapshot and an
isolated locked/frozen build, and release-v2 binds the commit, Git tree, binary,
source manifest and Cargo/rustc toolchain digest. Its checker accepts only v2,
rejects legacy or toolchain-unbound bundles, rejects unsafe links, modes and
unexpected files, and promotion is atomic inside the release root. Runtime
selection fails closed when an explicit selector is invalid. The isolated shell
contract passes; these are provenance controls, not a cryptographic signature
or evidence that the current Authority v3 source has been bundled, promoted or
deployed.

## Completion wording and later scopes

- the 2026-07-11 finite v1 checklist is a completed historical acceptance baseline;
- the broader historical ambition of a deeply simulated Jianghu-like RPG plus a feature-complete RTS has no honest fixed 100% endpoint; current claims must enumerate implemented systems and remaining scopes instead of converting that ambition into a percentage;
- human observation and non-developer sessions remain required for commercial usability claims and cannot be replaced by automated online E2E;
- final composed soundtrack, richer effects and mix: pending (basic original playback is complete);
- installer smoke across target distributions: pending;
- public beta/commercial launch, full social/season rewards, staffed safety and cross-host networking remain blocked.

## Current local evidence

- committed baseline `a3e1d6d7f` passed the 2026-07-22 serial source audit:
  292 locked workspace/all-target tests, format and workspace/all-target Clippy
  with `-D warnings`. Earlier 2026-07-15 authority-specific gates also passed
  locked workspace/all-target tests, game-server library 119/119, locked
  workspace/all-target Clippy with `-D warnings`, format and diff checks. The
  fault-evidence shell contract v2 passes. A transaction-wrapped live-database
  V13 rehearsal verifies pre-forgery and naked-transition rejection, exact
  atomic fail-close plus marker creation, summary maintenance, immutability and
  cursor-drift detection, then rolls back the migration and fixture;
- promoted P0 authority scope includes bounded asynchronous Authority command
  persistence, private two-phase terminal staging, full-ownership exact ACKs,
  unified terminal/abandonment cold-witness rollback evidence, atomic exact
  running fail-close maintenance, historical projection quarantine, ten-map
  render lifecycle replacement, RTS and campaign `MouseOnly` intent paths, and
  release-v2 provenance. Human acceptance credit remains absent;
- historical Online Authority v2 E2E: two real CEX sessions bring separate campaigns into one match, receive disjoint unit control, reject exact-ID altered replay, sequence skip, old build and control theft, recover via authenticated command-gap replay through a real systemd restart, each gain 80 XP plus two inventory units, and each settle a server-owned 25-credit Ed25519 reward; 15/15 PostgreSQL commands have unique persisted request fingerprints and the match owns two progression events;
- Stage 0 four-match endurance evidence `run/online-capacity/capacity-1783972489-528/summary.json`: 7,306 seconds, 32 waves, 128/128 unique settled matches, 2,176 ACKs at p95 39 ms/max 160 ms, maximum absolute tick drift 0.70, 0 process restarts/OOM/database crash signals and 122 newly archived WAL segments with 0 new archive failures. This closes the two-hour gate only; a clean independent 24-hour run remains required;
- the formal 24-hour attempt `run/online-capacity/capacity-1784159702-574915978/summary.json` failed closed after 4,779 seconds: `passed=false`, seven failed operational samples, four residual run matches at decision time and `cleanup_restored=false`. The later run `capacity-1784396963-2295312903` was SIGKILLed after 3,129 sampled seconds and has no final summary. Both are invalid endurance evidence; neither may be cited as a partial 24-hour pass. Do not relaunch until heavy OpenClaw/build work is isolated from the authority evidence resource domain;
- Authority v3 baseline E2E `online-e2e-1783987972505`, match `999291c0-3be7-44e9-97fa-e1f36a9217d0`: 68 commands, two members submitting from one revision with independent input cursor 0/0 and contiguous server order 0/1, 32 command/reconnect races, real systemd restart/recovery, WebSocket Full/Delta and decoded hash/tick verification, 20 authoritative-effect samples at p95 24 ms/max 26 ms, 70 ACK samples at p95 24 ms/max 30 ms, zero sequence/cursor mismatch and complete two-member progression/settlement;
- post-terminal-consistency E2E `online-e2e-1783991413209`, match `cd21bbf6-eec3-4c76-a251-7b183b4da344`: terminal state is withheld until the atomic terminal checkpoint and durable phase/result/settlement view agree, then consumed through the real WebSocket before actor-generation shutdown; all 68 commands, 32 reconnect/command races, restart recovery, exact terminal duplicate, two progression rows and settlement pass with effect p95 21 ms/max 60 ms and ACK p95 28 ms/max 38 ms;
- Authority v3 impaired-network E2E `online-e2e-1783988354933`, match `3699c5eb-9fda-4808-adc7-2e6fda02e723`: port-scoped loopback netem applies 50 ms each way and 1% configured loss; 20 command-submit-to-hash-verified-stream samples pass the 300-ms hard gate at p95 256 ms/max 364 ms, 70 ACK samples report p95 140 ms/max 460 ms, accelerated-clock drift is -0.85 tick, all 68 commands remain fingerprinted/input-sequenced with zero duplicates or member-cursor mismatch, and the match settles. This is 100-ms RTT laboratory evidence, not public Internet or injected-PostgreSQL-latency evidence;
- release-bound injected-PostgreSQL-latency run `run/online-faults/pg-rtt100-a3e1d6d7ffe4-20260715T234918Z-053868/decision.json` is the canonical current result: release ID `a3e1d6d7ffe4-e777cf2c4aa5-5524a70f78e8`, 100-ms database RTT, 191/191 healthy readiness samples, ACK p95 244 ms/max 476 ms, authoritative-effect p95 256 ms/max 262 ms and maximum cumulative actor drift 1.0011 ticks. All v2 decision checks pass; this is local single-host evidence only;
- pre-refactor run `run/online-latency/pg-rtt100-1783989631-2381685/decision.json` remains preserved as a historical failure: effect p95/max 3,158/3,416 ms, ACK p95/p99 2,748/3,137 ms and actor drift 848.48 ticks. It motivated the asynchronous authority refactor but is no longer the latest black-box result;
- exact-v2 rollback writer probe `run/rollback-probe-v2/v2-rollback-probe-1783986957-24731/`: preserved v2 server/E2E binaries run on an isolated fleet instance against the applied V10 schema and complete match `3a310649-15a1-4af5-9d67-ecf74da4f286`; the compatibility trigger fills all 15 legacy input sequences, host/guest cursors are 9/6, total order is 0..14 with zero duplicates, and settlement/progression succeed without restarting the live v3 service;
- rolling v2-client-to-v3-server probe `run/rolling-v2-client-v3-server/v2-client-v3-server-1783988764-10407/`: the preserved v2 E2E client connects to the current v3 server and completes match `f24539f9-bfcf-4858-886d-e531edaf55dd` in 2,220 ticks with all 15 commands, reconnect/idempotency/control gates, V10 cursor invariants, two progression rows and two signed entitlements intact; cleanup restores the v3 100-ms service with zero active matches;
- the two exact-v2 probes cover completed old-writer and old-client operation, not live ownership transfer of an already-running v2 match to a v3 actor; deployments must drain v2-owned matches until that cross-version generation handoff has its own recovery matrix;
- Authority v3 native network smoke `online-native-1783988480-31652`, match `d276ec26-20d7-4bd9-b4cf-a09777496cfc`: two sequential release X11 client processes attach through Product v2 under 100-ms RTT/1% configured loss, write independently attributed authoritative commands and retain distinct control sets. Its legacy v1 report used a virtual clock and an asserted network-thread field; it does **not** satisfy the new v3 gate, which pins a 60-FPS real-clock average, 30-FPS slowest-one-percent floor, 100-ms hard stall ceiling, instrumented worker-network calls, netem packet counters and the exact native input-to-durable-ACK path. A fresh clean-source v3 native run is required. It is automated single-host evidence, not a human multiplayer session;
- Online Product v1 final release E2E: run `online-product-1783897456-380`, lobby `c510d1e0-287f-47c6-a898-826394c3b886`, match `9f5677b3-788a-49f5-b959-8fd1d895e8a7`; proves invalid/consumed registration invite rejection, Argon2id, durable login lock, credential rotation, suspension/appeal/reactivation, stolen invite, duplicate lobby, stale revision and non-owner queue rejection, then two ready members, one allocation, full Authority v2 victory, two progression events, two Ed25519 entitlements and two 25/0 wallets;
- Online Product v2 E2E: run `online-product-v2-1783906177-12562`, lobby `5ad94f3f-a3b4-4b76-8b36-58d31b55a260`, match `5038b8a4-c411-4147-936c-71805e0367e3`; proves friend acceptance, two-sided block-aware pairing, duplicate ticket rejection, opposing human control sets, cross-control rejection, both-side commands, systemd restart, terminal 1016/984 zero-sum MMR, two rating events, two zero-delta progression provenance rows, zero CEX value entitlements and authenticated report resolution;
- native Product v2 compatibility shell: run `online-product-native-v2-1783912317-12205`, match `f265309d-bf97-4ad0-990c-d6eefea2561e`; two distinct release product windows traverse F1 login, F2 cloud character, F3 ranked queue and F5 launch, then create two distinct release Authority windows. All four frames pass structural rendered-pixel gates after the acceptance script was tightened to reject obscured blank X11 captures. Credentials are not rendered and only scoped sessions cross into the game process; this remains automated evidence, not a human session;
- Online Operations v1 E2E: run `online-operations-v1-1783912615-5333`, match `c48b1745-80ce-406e-810f-798761790bad`, replay `b7ed141cf0471acf02659b9a1cc1c46cc2dfc739f4fb9470350a6c5b6b23bef2`; proves active season/leaderboard, exact replay access, tampered replay rejection, replay-bound report, leaderboard hold/void, moderator-console audit, 24-hour ranked enforcement and primary-region routing;
- Online Operations fleet E2E: run `online-operations-fleet-1783912998-19347`, match `775c2d8e-b0b5-4a77-9f68-bcfcdd42ac37`; two live processes start with primary ownership, stop the primary, wait for heartbeat expiry, transfer once to the backup, reject routing at capacity, complete through cross-region fallback and write replay/season events with zero CEX value. This is same-host evidence, not cross-host HA;
- Online Operations anti-collusion compatibility E2E: run `online-operations-collusion-1783916291-20971`; same-device pairing is rejected, three real ranked matches complete, the third produces a medium repeat-opponent signal, and the fourth daily pairing is rejected under Operations v2. The test backdates only synthetic event timestamps by eleven minutes between real matches to cross the ten-minute cooldown while retaining the 24-hour window;
- Online Operations native login: run `online-operations-native-login-1783912263-3475`; real X11 text input enters player/credential fields, the credential is masked and stored through kernel-keyring stdin, a fresh process restores it without a credential environment variable, F8 removes it, the active season loads and both captures pass non-black plus structural color gates. This is automated evidence, not a human session;
- final exact Online Operations v2 compatibility E2E after the Production v1 deployment: run `online-operations-v2-1783920778-1497`, match `615b682c-282d-4a4e-9f15-77106a5f2e47`, replay `80ca3d72fb038a6a2b2988c71e2764c0c994a838275a9d5432cd6bc34d529392`, report `e97d2294-562c-4f2f-8ca5-d83c9fb5403f`, appeal `b7d29c65-b17b-429c-941a-0ee44b775e25`; 31 frames and two commands pass recomputed integrity, duplicate appeal is rejected, the 72-hour queue and approval/revocation path pass, queued season rotation is rejected, and season `season-ops-v2-1783920808-22772` closes/audits the prior season while snapshot count exactly matches integrity-eligible rows;
- final Online Operations v2 same-ID fencing compatibility: run `online-operations-fencing-1783920820-17769`; epoch 85 is fenced by 86, the stale process returns 503, drain routing fails closed, activate/offline are audited, and primary recovery registers epoch 87. This is same-host duplicate-process evidence, not cross-host quorum;
- final Online Operations v2 native replay compatibility: run `online-operations-native-replay-1783920865-28041`, match `7cdbe7d5-29e9-4a35-9ebc-74256625bbda`, replay `afd9425314064d6f7631682e8f24e104899f269826125230bc6159016d2c014d`; F9 falls back from a later cancelled ticket to the member's latest completed authoritative replay, verifies 31 frames and renders the Operations v2 state through the structural X11 hard gate;
- latest native Operations v2 login compatibility: run `online-operations-native-login-1783915384-21209`; text/mask/kernel-keyring/restart/forget and two rendered frames remain green after the v2 protocol upgrade;
- final Online Production v1 E2E: run `online-production-v1-1783921848-17452`; match `40963c5b-0d5c-419b-bc31-ed24712a9749` settles two pre-rotation receipts/entitlements, signer retry is exactly-once and an altered payload returns 409, targeted spectator grant `d6654636-3d6a-428c-9bd9-08b1c0b7283d` withholds then releases the terminal frame after its 30-second server-time delay, season `season-production-1783921873-1334` defers while queued then auto-activates, and appeal `8117eb06-ea48-49fc-96cd-2b1386a9ae22` escalates once then closes. The key rotates from `trnm-online-ed25519-production-1783920029-14665` to `trnm-online-ed25519-production-1783921873-20455` without restarting the game server; post-rotation match `57411b00-de57-4333-9464-b66d3cdc8aed` writes two new-key signer receipts and two CEX entitlements. A 30/minute probe returns 429 and a 300-KiB body returns 413. Exactly one physical host is observed, so cross-host HA, public edge and KMS/HSM remain unclaimed;
- final Online Production v2 E2E: run `online-production-v2-1783925971-3957`, with nested Production v1 compatibility `online-production-v1-1783925971-22023` and post-rotation match `a72233fc-8946-4eed-b3df-282b60137c47`; a signed challenge proves possession of key `trnm-online-ed25519-production-1783925994-9823` and exact active CEX-registry fingerprint convergence. Fifteen requests through each of two concurrently started instances share one PostgreSQL admission window and the thirty-first returns 429; both instances produce capacity samples after serialized migration startup. The native F10/F11 window accepts single-use spectator grant `5aca414f-f4e3-4083-94a6-4dc4bb57bf8a`, clears the token, renders delayed terminal playback and passes structural/manual image review. Shift `b8d97efa-c542-4c32-bd90-310e00454688` claims appeal `0c423437-cd44-4c15-b460-7846057f8d08`, rejects duplicate claim and unresolved close, then closes after resolution. Exact Operations v2 compatibility `online-operations-v2-1783926608-5566` and same-ID fencing `online-operations-fencing-1783926651-27567` remain green. Exactly one physical host and provider `file_seed` are reported; humans, cross-host HA, KMS/HSM, public edge and real staffing remain unclaimed;
- human Operations packet is generated at `acceptance/online-operations-v2-human/latest/session-packet.json` with `pending_human_participants`, two non-developer players, one observer and automation credit false; no human completion is claimed. The Production v2 external packet at `acceptance/online-production-v2-external/latest/packet.json` separately keeps second-host, KMS/HSM, public-edge and staffed-safety gates pending with automation credit false;
- historical Authority v2 network impairment E2E passes port-scoped loopback netem at 50 ms/1%, 100 ms/3% and 200 ms/5%; those runs remain compatibility evidence, while the current v3 hash-verified effect measurements are reported separately above;
- historical native attach smoke `online-native-1783897487-24605`, match `7e7c14cc-13b6-43c7-a770-44cff29d8d7d`; two release windows traverse product registration/lobby/invite/ready/allocation, then each produce an independently attributed fingerprinted command; this remains automated input evidence, not a human multiplayer session;
- CEX full workspace: 356 passing, including Ed25519 issuer rejection, active issuer fingerprint authorization and Argon2id/legacy credential migration tests; 16 detached-runtime black-box probes remain explicitly ignored and `consumer-entry-api` remains 161/161. The persistent cross-process gate additionally proves new accounts, reward exactly-once, byte-identical replay across ledger/consumer restart, held-escrow refund, committed chargeback, wallet/cursor recovery and PostgreSQL uniqueness;
- current terminal/PITR source workspace/all-targets Clippy with `-D warnings`:
  passing on 2026-07-15;
- product boundary: green (8 game / 12 platform / legacy working tree absent); CEX depends on `trnm-economy-protocol` and no longer depends on removed `trnm-world-api`, `trnm-world-domain` or `trnm-world-projection` crates;
- previously deployed baseline release build and desktop installer smoke:
  passing. Current terminal/PITR runtime credit must separately cite a strict
  release-v2 ID whose commit/tree/binary match the post-promotion evidence;
- current X230 isolated warm-cache matrix after explicit `--no-run` client-harness prewarming: RPG 1.39 s / 83 MiB, Campaign 0.60 s / 83 MiB, full 19-test First Contact package 89.57 s / 122 MiB, 64-sample RTS simulation 69.35 s / 83 MiB, new-save client journey 40.68 s / 122 MiB, Standard Annihilation 74.47 s / 122 MiB, authored-map adapter 12.72 s / 122 MiB, closed loop 13.52 s / 83 MiB and release build 49.44 s / 786 MiB. Every isolated row remains below the explicit 90-second / 4-GiB bound. The final Production v2 non-isolated full-workspace run recorded 99.30 s for First Contact (and 78.15 s for the 32-test RTS package), so that First Contact wall clock is not presented as passing the 90-second reference line; stopping the project window restores the isolated gate. The former 3.41-GiB client figure was rustc/linker RSS from compiling the test harness, not game-runtime memory; the gate separates compilation from runtime instead of misreporting it;
- release client service: active with a viewable native window after restart.
- persistent CEX recovery: PostgreSQL WAL archival, a physical 8.6-GiB base
  backup, named-restore-point PITR and writable same-host promotion are proven;
  this is not multi-host quorum/fencing or regional HA;
- receipt projection and seller payout maintenance run from a five-minute
  timer against the PostgreSQL source of truth.

The first clean release rebuild after adding the native rustls CEX client took 9m40s under the service host's constrained X230 environment; that is a developer compile cost, not installed-game startup. These are local-machine facts, not substitutes for the pending human session or a multi-distribution performance/installer matrix.

## Verification entry points

```bash
scripts/check_trnm_game_product.sh
scripts/check-trnm-online-authority-e2e.sh
scripts/check-trnm-online-product-v1-e2e.sh
scripts/check-trnm-online-product-v2-e2e.sh
scripts/check-trnm-online-product-v2-native.sh
scripts/check-trnm-online-operations-v1-e2e.sh
scripts/check-trnm-online-operations-v1-fleet.sh
scripts/check-trnm-online-operations-v1-native-login.sh
scripts/check-trnm-online-operations-v1-anti-collusion.sh
scripts/check-trnm-online-production-v1-e2e.sh
scripts/check-trnm-online-production-v2-e2e.sh
scripts/prepare-trnm-online-production-v1-second-host.sh
scripts/prepare-trnm-online-production-v2-external-gates.sh
scripts/prepare-trnm-online-operations-v1-human-session.sh
scripts/check-trnm-online-network-chaos.sh
scripts/check-trnm-online-native-two-client.sh
scripts/package-trnm-game-release.sh --require-clean
scripts/check-trnm-game-package.sh run/distribution/<archive>.tar.gz
cargo test --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked -- -D warnings
scripts/check-trnm-game-server-release-contract.sh
scripts/check_trnm_perf_matrix.sh
cargo build --manifest-path trillionnium/Cargo.toml --release -p trnm-first-contact
```

Historical World review/evidence documents from July 7-9 are under `docs/archive/world-review-2026-07/`; they are not current gameplay truth.
