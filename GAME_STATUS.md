# TRNM Game Status

Updated: 2026-07-13

This is the one-page status source for the current native RPG + real-time-strategy product. The older finite checklist in `docs/development/trnm-deep-rpg-complete-rts-v1-dod.md` is retained as a completed historical baseline, not as a claim that the broader deep-RPG + complete-RTS vision is 100%. The current local CEX economy integration, Online Authority v2, Online Product v2, Online Operations v2 and Online Production v2 slices are enumerated below; none grants Android or public-launch credit.

Release denominators are separated by
`docs/development/trnm-native-game-release-gates-v1.md`: software alpha,
commercial single-player, trusted CEX settlement and public player market are
four different gates.

The bounded v2 checklist remains historical evidence; this page records the newer runtime state directly and does not create another artificial "100%" contract.

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
- `trnm_online_authority_v2`, build `trnm-online-authority-2026.07-v2`.
- `trnm_online_product_v2`, build `trnm-online-product-2026.07-v2` (v1 private-lobby requests remain compatible).
- `trnm_online_production_v2`, build `trnm-online-production-2026.07-v2`; exact Production v1 and Operations v1/v2 protocol/build pairs remain compatible.

## Product shell

- title NEW/LOAD/CONTINUE, independent slots, corrupt-slot isolation and resume guard;
- authoritative pause, journal, progressive guide and character identity confirmation;
- low motion, input mode, three control-scheme profiles, subtitles/high contrast and live master-volume control;
- desktop installer assets under `packaging/` and `scripts/install_trnm_desktop.sh`;
- deterministic performance matrix at `scripts/check_trnm_perf_matrix.sh`.

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

Online Authority v2 is a bounded network vertical slice. Two distinct CEX
player sessions bring separate PostgreSQL campaigns into one co-op match
against the deterministic AI. The server owns both campaign JSON documents,
authored-map seed, `MissionSimV1`, tick, member-to-unit control sets, command
sequence/idempotency, snapshots, terminal result and per-member settlement.
Each member receives an independently persisted XP/inventory provenance event.
Restart does not return authority to the client; authenticated reconnect returns
the full current snapshot plus a bounded persisted command-receipt gap.

Positive online wallet rewards are match/result/participant-bound Ed25519
`ServerSignedValueEntitlementV2` values. Online Production v1 removes the
private seed from the game-server process: a separate loopback-authenticated
signer revalidates the authoritative binding, owns the mode-600 seed and
persists exactly-once signing receipts; CEX loads only the active/revoked public
issuer registry. The native client uses bounded same-request retries, refreshes stale
target ticks only after the server proves the original was not accepted, and
still disables local simulation/settlement while attached.

Online Authority v2 alone is not a public online game claim. Online Product v2
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
The allocated game is still the same Authority v2 server-owned simulation and
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
own capacity, heartbeat, region and match assignment; stale ownership can move
once under PostgreSQL row lock with a durable failover record. This is same-host
two-process proof, not cross-host or regional HA. See
`docs/development/trnm-online-operations-v1.md`.

Online Operations v2 turns the replay index into a member-authorized playback
package with initial/checkpoint/terminal simulations, ordered commands and a
recomputed result/participant/command hash. The protected season control plane
creates scheduled seasons, atomically activates one, closes the prior season
and archives only integrity-clear ranks. Enforcement appeals are authenticated,
one-per-enforcement, due within 72 hours and can revoke the enforcement through
an audited moderator decision. Fleet ownership now binds both instance ID and a
monotonic epoch; five-second leases, fenced tick writes, drain/activate/offline
controls and epoch-aware failover prevent an older same-ID process from
continuing authority. Non-loopback binds fail closed while KMS/HSM and public
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

## Completion wording and later scopes

- the 2026-07-11 finite v1 checklist is a completed historical acceptance baseline;
- the broader historical ambition of a deeply simulated Jianghu-like RPG plus a feature-complete RTS has no honest fixed 100% endpoint; current claims must enumerate implemented systems and remaining scopes instead of converting that ambition into a percentage;
- human observation and non-developer sessions remain required for commercial usability claims and cannot be replaced by automated online E2E;
- final composed soundtrack, richer effects and mix: pending (basic original playback is complete);
- installer smoke across target distributions: pending;
- public beta/commercial launch, full social/season rewards, staffed safety and cross-host networking remain blocked.

## Current local evidence

- eight-crate unit/integration/E2E suite: 131/131 passing (41 Campaign, 19 First Contact, 13 RPG, 8 existing protocols, 32 RTS, 9 closed-loop E2E, 2 online protocol and 7 game-server tests);
- Online Authority v2 E2E: two real CEX sessions bring separate campaigns into one match, receive disjoint unit control, reject exact-ID altered replay, sequence skip, old build and control theft, recover via authenticated command-gap replay through a real systemd restart, each gain 80 XP plus two inventory units, and each settle a server-owned 25-credit Ed25519 reward; 15/15 PostgreSQL commands have unique persisted request fingerprints and the match owns two progression events;
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
- network impairment E2E: the same full two-session/restart/settlement gate passes port-scoped loopback netem at 50 ms/1%, 100 ms/3% and 200 ms/5%; this is bounded local evidence, not a public-network SLO;
- final native attach smoke: run `online-native-1783897487-24605`, match `7e7c14cc-13b6-43c7-a770-44cff29d8d7d`; two distinct release windows first traverse product registration/lobby/invite/ready/allocation, then each produce an independently attributed fingerprinted command; this is automated input evidence, not a human multiplayer session;
- CEX full workspace: 356 passing, including Ed25519 issuer rejection, active issuer fingerprint authorization and Argon2id/legacy credential migration tests; 16 detached-runtime black-box probes remain explicitly ignored and `consumer-entry-api` remains 161/161. The persistent cross-process gate additionally proves new accounts, reward exactly-once, byte-identical replay across ledger/consumer restart, held-escrow refund, committed chargeback, wallet/cursor recovery and PostgreSQL uniqueness;
- workspace Clippy with `-D warnings`: passing;
- product boundary: green (8 game / 12 platform / legacy working tree absent); CEX depends on `trnm-economy-protocol` and no longer depends on removed `trnm-world-api`, `trnm-world-domain` or `trnm-world-projection` crates;
- release build and desktop installer smoke: passing;
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
cargo test --manifest-path trillionnium/Cargo.toml --workspace --all-targets
cargo clippy --manifest-path trillionnium/Cargo.toml --workspace --all-targets -- -D warnings
scripts/check_trnm_perf_matrix.sh
cargo build --manifest-path trillionnium/Cargo.toml --release -p trnm-first-contact
```

Historical World review/evidence documents from July 7-9 are under `docs/archive/world-review-2026-07/`; they are not current gameplay truth.
