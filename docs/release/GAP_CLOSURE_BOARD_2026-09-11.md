# Trillionnium Gap-Closure Board — 2026-09-11

Document class: active release/engineering closure board  
Baseline reviewed: `main@0f6117a56263bcb5bf89e34b8bcda557a7da2e6d`  
Repository release truth remains: [`../../RELEASE_READINESS.md`](../../RELEASE_READINESS.md)

## Decision rule

A row is **closed** only when every exit criterion has exact commit-bound evidence. Documentation, source presence, a local smoke test, or a historical PASS cannot substitute for deployment, endurance, human, public-network, security, legal, or operator evidence.

Statuses are `CLOSED-IN-BRANCH`, `OPEN-ENGINEERING`, `OPEN-EVIDENCE`, and `SCOPE-DECISION`.

## A. Repository governance and documentation

| ID | Status | Gap and exit criteria |
| --- | --- | --- |
| DOC-001 | CLOSED-IN-BRANCH | Substantive root README with status, boundaries, commands, and truth sources; link gate passes |
| DOC-002 | CLOSED-IN-BRANCH | All Rust workspaces and non-Rust operational surfaces recorded in `docs/modules.json` |
| DOC-003 | CLOSED-IN-BRANCH | Every game, platform, and contract workspace member has a code-adjacent README |
| DOC-004 | CLOSED-IN-BRANCH | `docs/` declared canonical; `trillionnium/docs/` scoped; canonical index contains only live links |
| DOC-005 | CLOSED-IN-BRANCH | Automated module coverage, link, and personal-path gate passes on a clean checkout |
| GOV-001 | CLOSED-IN-BRANCH | Repository-relative boundary defines game/platform/contracts/Web4 authority |
| SEC-001 | CLOSED-IN-BRANCH | Root private vulnerability-reporting policy exists |
| REPO-001 | CLOSED-IN-BRANCH | Probe/patch/placeholder residue reviewed and removed |

These rows close repository governance defects only. They do not change the release verdict.

## B. Native game Gate B — commercial single-player

| ID | Status | Required exit evidence |
| --- | --- | --- |
| GAME-PERF-001 | OPEN-EVIDENCE | Release hardware matrix at 30/60 FPS with frame-time p50/p95/p99, input latency, map/unit profiles, clean commit, artifact digest, and raw traces |
| GAME-HUMAN-001 | OPEN-EVIDENCE | Three independent five-second observers plus one unguided non-developer 10–15 minute session; private participant records and raw outcomes |
| GAME-ACCESS-001 | OPEN-ENGINEERING | Accessibility matrix for input modes, contrast, subtitles, motion, scaling, keyboard/pointer traversal, and regressions |
| GAME-DIST-001 | OPEN-ENGINEERING | Windows/macOS build, signing/notarization, install/update/uninstall, rollback, license, and reputation checks |
| GAME-SUPPORT-001 | SCOPE-DECISION | Supported OS/GPU/input, save support, crash/privacy, support, and patch policy approved |

## C. Public online RPG + RTS

| ID | Status | Required exit evidence |
| --- | --- | --- |
| ONLINE-ENDURANCE-001 | OPEN-EVIDENCE | Independent clean 24-hour active-match endurance run with predefined readiness, drift, queue, DB, journal, CPU/memory thresholds |
| ONLINE-CRASH-001 | OPEN-EVIDENCE | Client/server journal crash/restart, kill-before-ACK, DB rollback, ambiguous commit, disk-full/corruption, and fencing matrix |
| ONLINE-COMPAT-001 | OPEN-EVIDENCE | Exact-v2 matrix retained until formal retirement; active v2 matches drain before v3 ownership |
| ONLINE-IDENTITY-001 | OPEN-ENGINEERING | Public registration, verified recovery, suspension/appeal, device/session rotation, privacy/retention, and abuse controls |
| ONLINE-SOCIAL-001 | SCOPE-DECISION | Party queue, chat/guild, reports, staffed moderation, and appeal SLA implemented or explicitly excluded |
| ONLINE-KEYS-001 | OPEN-ENGINEERING | KMS/HSM or approved remote signer, rotation/revocation/compromise/backup/dual control and possession proof |
| ONLINE-EDGE-001 | OPEN-EVIDENCE | Public TLS, WAF/DDoS/rate limit, rotation, external penetration test, and public-network latency/loss |
| ONLINE-HA-001 | OPEN-ENGINEERING | Off-host retention and cross-host replicated accepted-input durability with RPO=0, fencing/STONITH, restore, multi-region failover |
| ONLINE-CAPACITY-001 | OPEN-EVIDENCE | Multi-host admission/backpressure/recovery/cost/saturation/SLO matrix |
| ONLINE-HUMAN-001 | OPEN-EVIDENCE | Human multiplayer onboarding, comprehension, reconnect, report/appeal, latency, and accessibility sessions |

## D. Public market and economics

| ID | Status | Required exit evidence |
| --- | --- | --- |
| MARKET-CUSTODY-001 | OPEN-ENGINEERING | Signed inventory custody/listing ownership and fault-tested reserve→escrow→commit→reversal |
| MARKET-FRAUD-001 | OPEN-ENGINEERING | Anti-cheat, collusion, fraud, abuse, duplicate value, chargeback, seller collateral, and disputes |
| MARKET-OPS-001 | OPEN-EVIDENCE | Customer support, reconciliation, dispute/chargeback, recovery, suspension/appeal, and incident drills |
| MARKET-LEGAL-001 | SCOPE-DECISION | Commercial/legal/privacy/tax/regulatory approval for launch jurisdictions |
| ECON-FREEZE-001 | OPEN-ENGINEERING | Fee/anti-spam/sponsor/retention/override tuple frozen with governance and load evidence |

## E. Platform public-mainnet blockers

| ID | Status | Required exit evidence |
| --- | --- | --- |
| NET-001 | OPEN-ENGINEERING | Peer discovery/bootstrap, authenticated gossip, scoring/backpressure, state/snapshot sync, join/rejoin and adversarial tests |
| VALIDATOR-001 | OPEN-EVIDENCE | Genesis/key ceremony, bootstrap, replacement/rotation, upgrade/rollback, DR rebuild, signed multi-operator rehearsal |
| SIGNER-001 | OPEN-ENGINEERING | Secure keystore, offline signing, import/export/recovery, multisig/HSM/remote signer, safety and compromise runbook |
| INDEXER-001 | OPEN-ENGINEERING | Durable block/tx/account/event indexer, history, explorer API, replay bootstrap, archive/replica, query SLO/recovery |
| OBS-001 | OPEN-ENGINEERING | Unified metrics/exporters/dashboards/frozen alerts/incident labels and replay/rollback-linked drills |
| ECON-PLATFORM-001 | OPEN-ENGINEERING | Final admission, fee/sponsor/free-ingress, storage/retention pricing, abuse protection, frozen parameters |
| ORACLE-001 | SCOPE-DECISION | Live ingest/persistence/RPC/replay/capacity/source diversity/SLO if Day-1 |
| BRIDGE-001 | SCOPE-DECISION | Finality/proof/relayer/key/recovery/audit/monitoring/operator closure if Day-1 |
| VERIFIER-001 | SCOPE-DECISION | Deployable verifier packaging/trust/retry/DA/checkpoint/outage replay if Day-1 |
| GOVERNANCE-001 | OPEN-ENGINEERING | Typed registry, sensitive-parameter freeze, public upgrade/migration discipline and signed checkpoints |

## F. External-contract runtime

| ID | Status | Required exit evidence |
| --- | --- | --- |
| CONTRACT-ABI-001 | OPEN-ENGINEERING | Canonical versioned Host ABI and runtime spec implemented and compatibility-tested |
| CONTRACT-WASM-001 | OPEN-ENGINEERING | Deterministic WASM executor, gas/quota, storage delta, rollback, state-root, and RPC mapping |
| CONTRACT-SDK-001 | OPEN-ENGINEERING | SDK, generated bindings/examples, package/version policy, and negative conformance tests |
| CONTRACT-INTEGRATION-001 | OPEN-EVIDENCE | Golden replay across vault/bridge/governance/audit and node/state/RPC host boundary |
| CONTRACT-RELEASE-001 | SCOPE-DECISION | Explicit Day-1 inclusion/exclusion and signed artifacts/runbooks for included contracts |

## Execution order

1. Merge and require the governance/documentation gate.
2. Freeze the exact launch promise; resolve every `SCOPE-DECISION` row.
3. Close game performance/human/accessibility/desktop distribution.
4. Close online crash/endurance/key/edge/HA/capacity.
5. Close market/economics before public value transfer.
6. Close platform network/operator/signer/indexer/observability/economics before public mainnet.
7. Promote contracts only after Host ABI, deterministic runtime, SDK, and golden integration evidence.

## Evidence packet minimum

Every runtime closure packet records `gap_id`, `status`, `git_commit`, `git_tree`, clean-tree state, artifact SHA-256, UTC time, runner/OS/architecture/hardware/network, exact command, predefined thresholds, raw artifact path, rollback command, evidence scope, and exclusions. Human/legal evidence uses approved private records; this public board stores only review results and evidence identifiers.
