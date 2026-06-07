# Trillionnium World 统一开发文档 v1

**项目**：Trillionnium World
**目标仓库**：<https://github.com/TrillionniumFoundation/Trillionnium>
**当前 Native 客户端主线**：`/home/qian/.openclaw/workspace/Trillionnium/trillionnium/crates/trnm-world-bevy`
**CEX 状态**：legacy incubator / adapter evidence only；不再作为 Trillionnium World 玩家客户端入口
**合并来源**：Dropbox `rust_geo_mmo_development_doc_v0_4_execution_ready.md`、CEX 已落地代码、CEX 现有 Trillionnium/World 文档与运行证据
**当前代码基线**：CEX `53687d7 test: gate trillionnium world adapter readiness`
**文档状态**：统一执行版；CEX source-of-evidence 已同步到 Term Exchange typed receipt runtime closeout，后续只能作为迁移参考和 adapter evidence，新的试玩/client/account 产品入口必须回到 Trillionnium 主仓与 Bevy native client
**修订日期**：2026-05-13

---

## 0. 一句话结论

Trillionnium World 的正确路线不是从零重写一个“概念 MMO”，而是以 CEX 中已经跑通并有证据的 Rust 世界状态、Ledger 结算、真实地图投影、Matrix/Web 双入口、Rust-owned UI fragment、运行门禁与 E2E 证据为当前事实基线，再吸收 Dropbox v0.4 文档里的 MMO 阶段化、地图合规、证据包、ADR、Bevy/移动端、Cell/AOI/QUIC/H3 等长线设计，形成一条可执行迁移路线：

```text
CEX 已验证垂直切片
  -> Trillionnium World 独立模块边界
  -> Trillionnium 主仓文档/接口/证据迁移
  -> Rust-owned Web/Matrix 全量安全等价
  -> 独立 World Server / API / 数据 schema
  -> 合规真实地图包与生产地图管线
  -> Native/Bevy 移动客户端实验门
  -> Cell/AOI/实时多人 MMO 硬化
  -> Alpha / Beta / 商业化准入
```

当前最重要的原则：**已经落地并跑绿的 CEX 代码是真实进度，Dropbox v0.4 是目标工程制度和长线 MMO 蓝图；本文档把二者合并，而不是用愿景覆盖事实。**

### 0.1 2026-05-18 客户端边界修正

这条线现在必须按 `trillionnium_world_client_boundary_v1` 执行：

- `trnm-world-bevy` 是 Trillionnium World 的 native 试玩客户端和玩家入口。
- CEX 只能作为 legacy evidence adapter / migration reference，不能再作为玩家试玩 runtime。
- 账号注册、登录、profile、session、revoke 等能力可以从 CEX 的已验证实现迁移或抽象，但产品归属必须是 Trillionnium-owned account API，并由 Bevy native client 消费。
- 手工试玩默认命令是 `scripts/run_trillionnium_world_bevy_client.sh`；不得用 CEX `consumer-entry-api` / `/world` web shell 替代 Bevy 试玩。
- 新的防漂移 gate 是 `scripts/check_trillionnium_world_client_boundary.sh`，release-review CI 必须跑它。

---

## 1. 当前事实基线

### 1.1 已落地代码与运行状态

CEX 当前已经不是纯文档阶段。可作为 Trillionnium World 的孵化实现，具备以下已验证能力：

- Rust workspace 多服务架构：`identity-service`、`ledger-service`、`gateway-service`、`execution-service`、`audit-service`、`capability-service`、`consumer-entry-api`、`matrix-entry-adapter`、Matrix bot poller/relay。
- `consumer-entry-api` 已承载 Trillionnium League / World 的核心产品垂直切片。
- `WorldState` 已从 `LeagueState` 中拆出第一层世界边界，并保留旧 JSON/持久化兼容。
- World 数据已覆盖：地图节点、玩家位置、角色、战术会话、公司、商店、listing、purchase、work order、delivery/accept/reject/reopen/cancel、faction、economy event、route runner、tactics、task/NPC loop。
- Ledger-backed commerce 已具备 reserve / seller settlement / buyer consume / refund / seller chargeback / recovery / fail-closed 的回归测试与 runtime gate。
- `/world`、`/app`、Matrix commands、web session/CSRF、本地 production runtime、SQL snapshot、health/metrics、browser/web/first-human E2E 均已存在。
- 当前 Rust-owned UI 工作已推进到 `/world` shell、route UI、route-runner UI、live-event/task-route cards、map popup、map support cards、map focus/action rail。

### 1.2 最新已验证证据

最新本地证据来自 CEX 本地 production runtime：

- Head：`53687d7 test: gate trillionnium world adapter readiness`
- CEX production runtime adapters now export `GET /v1/trillionnium/world/adapters/readiness`; Trillionnium-main consumes that JSON through `scripts/check_trillionnium_world_cex_adapter_readiness.sh` so release review can verify the production adapter bridge without importing CEX service internals.
- SQL snapshot：`run/linux-runtime/entry-config/league-state-snapshot.sql`
- 最新 SQL state hash：`sha256:c438f35e5d1722175e2e095ad877025bc9c7fbbaef7d258f1b7a14089e4370d2`
- Normalized runtime dual-write/read-switch：temp DB `cex_normalized_runtime_1778661039_319748`，world-home / client-feed / client-app overlay gates green，`world_term_exchange_receipt_rows_after_direct_write=8`
- Web E2E：`run/league-web/web-e2e-summary-1778661371.json`，`ok=true`，client feed `source_count=8`，Term Exchange receipt projection source `normalized_sql_client_feed_read_model`，receipt item count 4
- Browser E2E：`run/league-browser/browser-e2e-summary-1778568045-1918845.json`，`ok=true`，request failure gate green，unclassified=0
- First-human E2E：`run/first-human-session/browser-e2e-summary-1778568310-1924693.json`，`ok=true`，request/page errors=0
- Full Rust tests：`cargo test -p consumer-entry-api -- --nocapture --test-threads=1`，153 passed
- Clippy：`cargo clippy -p consumer-entry-api --all-targets -- -D warnings` passed
- Local production status：8 个 HTTP service + worker healthy
- 最新 honest assessment 基线：technical 约 `9.8/10`，first internal beta `8.5/10`，commercial release `7.0/10`

重要限制：first-beta 9+ 与 commercial 8+ 仍需要真实 cohort / commercial drill / multi-node 或 live-traffic 证据，不能靠 localhost 继续刷分。当前 `scripts/check-production-readiness.sh` 的代码/运行时检查已通过，但 DB backup/restore drill 与 monitoring deploy metadata evidence 已超过 24h freshness 窗口，刷新前不能宣称 production signoff 当前为 green。

---

## 2. 产品定义

### 2.1 Trillionnium World 是什么

Trillionnium World 是一个基于真实地理骨架的开放世界 AI/Agent 地理冒险 MMO。真实地图提供道路、区域、POI、地理分区与位置语义；游戏系统在其上生成任务、NPC、门派/阵营、资产、公司、商店、路线、事件、战斗、协作和经济结算。

产品由五个互相连接的品牌域组成：

| 域 | 当前 CEX 状态 | 长线形态 |
| --- | --- | --- |
| Trillionnium World | `/world` 已可玩，Rust WorldState 为权威 | 独立 World server、真实地图区域、多人 Cell/AOI |
| Trillionnium League | 已有比赛、提交、评分、奖励、review hold、route runner | 世界内赛事、赛季、任务联盟、竞技经济 |
| Trillionnium Craft | 资产/公司/商店/listing/work order 已有 | 建造、制造、公司经营、Agent 产业链 |
| Trillionnium Ledger | CEX ledger 已接入 World commerce | Trillionnium 链/账本/审计/结算一体化 |
| Trillionnium Agents | Agent residents、party、handoff 已有 | 可雇佣 Agent、NPC 社会、自动任务与队伍 |

### 2.2 MVP 定义

MVP 不是“完整开放世界上线”，而是证明下面八件事：

1. Rust 权威世界状态可以稳定承载移动、任务、经济、战斗和 UI projection。
2. 客户端只提交意图，服务端决定结果。
3. 真实地图作为游戏骨架是合规、可署名、可缓存、可降级的。
4. 玩家能完成 10 分钟内的新手闭环：进入世界、移动、练功/交互、战斗/任务、获得奖励、进入下一条路线。
5. 经济系统不会复制货币/物品，不会在 ledger 失败时推进世界状态。
6. 浏览器/Web/Matrix 只是输入、focus、event bridge，不是世界状态来源。
7. 每个 Gate 都有可复跑证据，不靠主观“看起来可以”。
8. 运维有 health/metrics、E2E、runbook、备份恢复和失败动作。

---

## 3. 不可妥协架构原则

这些规则同时来自 Dropbox v0.4 和 CEX 已经落地的技术路线：

1. **Rust source of truth**：`WorldState`、Rust projection、Rust command handler 是权威；浏览器、Matrix、未来 Bevy 客户端只提交 intent。
2. **UI Rust-owned**：`/world` 的玩家可见状态、action rail、route card、task card、popup、support card 逐步由 Rust server-rendered fragment / projection 生成；JS 只做事件绑定、focus 选择、可视化 fallback。
3. **Ledger fail-closed**：经济结算、refund、chargeback、review release、completion reward 都必须以 ledger 成功或可恢复状态为推进条件。
4. **地图逻辑和渲染分离**：真实地图/OSM 是 muted context layer；游戏语义和位置 mutation 由 Rust world graph 决定。
5. **OSM 合规先行**：当前只允许 fixture 模式。live Overpass / Geofabrik ingestion、生产公网 OSM tile 直接使用、MapLibre promotion 都必须另有合规/缓存/回滚证据。
6. **不能复制 Hero Tan / 白金英雄坛说**：只能作为公开参考做 mechanics/layout study；不得复制代码、文本、NPC、任务表、资产、数据。
7. **不能复制 MedievalWar/Phaser 资产**：当前只借鉴 permissive tactics patterns；不 vendoring 源码/资产，除非单独法律/产品决策。
8. **分数不能靠本地假证据抬升**：first-beta、commercial、technical 9.9+ 必须要真实 cohort、commercial drill、multi-node/live traffic。
9. **证据包优先**：没有证据包，不算 Gate 通过。
10. **兼容迁移**：CEX 已跑通路径先保留，再抽象/迁移；不做 full clone-then-replace。

---

## 4. 当前 CEX 架构

### 4.1 Runtime 拓扑

```text
Telegram / Matrix / Web
  -> matrix-bot-poller / matrix-bot-relay
  -> matrix-entry-adapter
  -> consumer-entry-api
       - League / World API
       - Web shell `/league`, `/app`, `/world`
       - Rust WorldState / projection / command handlers
       - health / metrics / production readiness gates
  -> ledger-service
  -> gateway-service / execution-service / audit-service / capability-service / identity-service
  -> local-production runtime + SQL snapshot / normalized repository gates
```

### 4.2 关键文件归属

| 领域 | CEX 文件 |
| --- | --- |
| World state / domain structs | `services/consumer-entry-api/src/lib.rs` |
| Default fixtures / repository | `services/consumer-entry-api/src/league_repository.rs` |
| OSM provider seam | `services/consumer-entry-api/src/openstreetmap_geodata.rs` |
| Map projection | `services/consumer-entry-api/src/world_map_projection.rs` |
| Route projection | `services/consumer-entry-api/src/world_route_projection.rs`、`world_routes.rs` |
| World web shell | `services/consumer-entry-api/src/world_web_shell.rs` |
| Shared map adapter/runtime JS/CSS | `services/consumer-entry-api/src/real_world_map_shell.rs` |
| Tactics/mechanics | `services/consumer-entry-api/src/world_tactics.rs` |
| Runtime gates | `services/consumer-entry-api/src/health_metrics.rs`、`world_map_optimization.rs` |
| Tests | `services/consumer-entry-api/src/tests.rs` |
| Browser E2E | `scripts/playwright/trillionnium-browser-e2e.mjs` |
| Web E2E | `scripts/check-trillionnium-league-web-e2e.sh` |
| First-human E2E | `scripts/check-trillionnium-first-human-session.sh` |
| Production readiness/signoff | `scripts/check-production-readiness.sh`、`scripts/check-production-signoff.sh` |

### 4.3 当前已落地 World contract 家族

| Contract | 状态 |
| --- | --- |
| `trillionnium_world_rust_owned_ui_shell_v1` | `/world` first shell / keypad / core UI Rust-owned |
| `trillionnium_text_adventure_keypad_movement_v1` | 方向键探索循环已可玩 |
| `trillionnium_world_play_first_exploration_loop_v1` | 首屏四问/探索 loop 已进 E2E |
| `trillionnium_world_objective_travel_v1` | Rust graph objective travel 已落地 |
| `trillionnium_world_skill_practice_loop_v1` | mentor skill practice 已落地 |
| `trillionnium_world_rust_route_ui_fragments_v1` | route task graph / route-flow action Rust-owned |
| `trillionnium_world_rust_route_runner_ui_fragments_v1` | route-runner handoff cards/actions Rust-owned |
| `trillionnium_world_rust_live_task_ui_fragments_v1` | live-event / avatar-task-route cards Rust-owned |
| `trillionnium_world_rust_map_popup_ui_fragments_v1` | marker/runner/avatar popups Rust-owned |
| `trillionnium_world_rust_map_support_ui_fragments_v1` | density/stream/region/tile/POI/prefetch/cluster cards Rust-owned |
| `trillionnium_world_rust_map_focus_ui_fragments_v1` | focus summary/detail/action rail Rust-owned |
| `openstreetmap_geodata_v1` | fixture OSM geodata source-of-truth |
| `openstreetmap_provider_readiness_v1` | fixture-ready/live-fail-closed gate |
| `openstreetmap_geodata_freshness_v1` | fixture freshness / live staleness gate |
| `openstreetmap_attribution_presence_v1` | OSM/ODbL attribution presence gate |
| `trillionnium_browser_request_failure_gate_v1` | Browser E2E request-failure hard gate |
| `trillionnium_route_runner_handoff_v1` | handoff/feed/world loop gate |
| `trillionnium_world_public_commercial_product_v1` | commercial readiness product gate |

---

## 5. Dropbox v0.4 与 CEX 当前实现的合并关系

Dropbox v0.4 是完整 MMO 工程蓝图，很多内容是未来阶段，不应误标为当前已实现。合并后的解释如下：

| Dropbox v0.4 主题 | 当前 CEX 状态 | 本文档处理方式 |
| --- | --- | --- |
| Rust 全栈 MMO | 已有 Rust CEX 多服务 + World vertical slice | 作为当前孵化实现 |
| Bevy Android client | Native Bevy client shell 已在 Trillionnium 主仓落地；Android 设备矩阵仍未通过 | 作为 S5 实验 Gate，不阻塞当前 Web Rust-owned 收口 |
| QUIC + WebSocket fallback | 当前 World 不是实时 socket MMO | 作为 standalone server / Cell 阶段 ADR |
| H3 / region-local / AOI | 当前有 map viewport/tile/region projection，不是 full H3 Cell server | 作为 M4/M5 迁移目标 |
| map_pack 签名 | 当前是 fixture OSM + attribution gate | 纳入 M1.5/M4 地图包 Gate |
| economy idempotency/recovery | CEX 已有大量 ledger/world commerce 回归 | 作为已落地基线继续强化 |
| acceptance evidence package | CEX `run/*` 已有证据；Dropbox `acceptance/*` 更规范 | 合并为统一证据制度 |
| chat/UGC 风控 | CEX 当前重点不是公开聊天 | 外部测试前硬 Gate |
| privacy/data retention | CEX 有审计/运行基础，需文档化 | 纳入 Alpha 准入 |
| public test readiness | CEX 有 beta/commercial gate，但真实 cohort/drill 未补 | 保留为 blocker |

---

## 6. 目标系统架构

### 6.1 当前阶段：CEX Incubation Architecture

```text
CEX consumer-entry-api
  ├─ WorldState / LeagueState / Ledger integration
  ├─ /world web shell
  ├─ /app mobile-style shell
  ├─ Matrix command projection
  ├─ Rust-owned UI fragments
  ├─ OSM fixture provider
  ├─ Health / metrics / playability gates
  └─ SQL snapshot / normalized repository path
```

该阶段目标是把 World 的产品闭环、Rust source-of-truth、经济安全、真实地图合规姿态和 UI 所有权做实。

### 6.2 中期阶段：Trillionnium World Server

从 CEX 中抽出独立模块，但保留接口兼容：

```text
trillionnium-world-server
  ├─ world-domain
  ├─ world-projection
  ├─ world-command
  ├─ world-ledger-adapter
  ├─ world-map-provider
  ├─ world-ui-fragments
  ├─ world-api
  └─ world-evidence-gates
```

抽离原则：

- 先复制 contract / tests / evidence gate，再搬 implementation。
- 外部行为、JSON contract、E2E 先不变。
- CEX 仍可作为 adapter/host，直到 Trillionnium 主仓 runtime 可替代。
- 当前 Trillionnium 主仓已开始落独立 dev runtime：`trnm-world-server serve --bind 127.0.0.1:8787` 暴露 `trillionnium_world_dev_runtime_v1`，包含 `/health`、`/world/home`、`/world/state`、`/world/command`、`/world/tactics-command`、`/world/full-split` 等开发端点；WorldState 仍由 Rust server 持有，客户端只提交 intent。`trillionnium_world_dev_file_repository_v1` 通过 `--state-file` 和 `dev-runtime-repository-smoke` 验证重启后状态仍可读回。

### 6.3 长期阶段：Standalone MMO Architecture

```text
Native/Web Client
  -> Gateway / Transport adapter
  -> Router
  -> Cell Server(s)
  -> WorldState shards / AOI / route systems
  -> Economy / Ledger / Audit
  -> Map service / map_pack CDN / attribution service
  -> Ops / metrics / replay / recovery
  -> Trillionnium chain settlement bridge
```

Dropbox v0.4 的 Bevy、QUIC、H3、Cell、map_pack、AOI、mobile release 进入这一阶段，而不是覆盖当前 CEX Web-first 已验证路线。

---

## 7. 阶段路线图

### S0 — 保住 CEX 事实基线（当前）

**目标**：不破坏当前 green runtime。

验收：

- `git status` 仅允许明确 untracked 临时目录。
- `cargo fmt --all -- --check`
- `cargo check -p consumer-entry-api`
- `cargo test -p consumer-entry-api -- --nocapture --test-threads=1`
- `cargo clippy -p consumer-entry-api -- -D warnings`
- `CEX_ENV_FILE=run/local-production/.env scripts/runtime-manager-linux.sh restart && status`
- Web / Browser / First-human E2E green。

### S1 — 完成 `/world` Rust-owned UI 全量安全等价

已完成：shell、route、runner、live/task、popup、support、focus/action rail。

剩余：

- 审计并迁移仍由浏览器拼接的次级 dashboard / commerce / timeline / support 面板。
- 将 heavy secondary panels 改为 Rust projection + lazy hydration。
- 明确 `/app` 的 shared fallback 边界，避免 `/world` 回退成 JS-owned。

验收：

- `/world` 玩家可见核心 UI 均有 `data-render-owner="rust_world_ui_renderer"` 或明确 fallback 标记。
- Browser JS 无权威状态 mutation，只做 input/focus/event bridge。
- E2E 覆盖 post-move、delta、304、weak-network cache、request-failure gate。

### S2 — World domain 模块边界

目标：把 `WorldState` 从 CEX 的 League/CEX 混合语境中进一步模块化。

任务：

- 拆分 `WorldTopologyState`、`WorldPresenceState`、`WorldCommerceState`、`WorldContractState`、`WorldReputationState`、`WorldRoutingState`、`WorldEventState`。
- 让 systems own mutation：navigation、task、combat、commerce、contract、reputation、route projection。
- 把 indexes 从 helper 推进到显式 derived read model。
- 保留 serde compatibility 和 SQL snapshot/normalized repository gates。

### S3 — Trillionnium 主仓文档/API 对齐

目标：让 <https://github.com/TrillionniumFoundation/Trillionnium> 有一份能承接 CEX World 的开发文档、接口清单和迁移计划。

任务：

- 将本文档同步到 Trillionnium 主仓 `docs/development/`。
- 新建 World API contract index。
- 新建 World evidence/runbook index。
- 标注 CEX 当前为 incubator/source-of-evidence。
- 不在主仓宣称 CEX 中尚未迁移的代码已存在。
- 继续扩展 `trnm-world-server` 独立 dev runtime，直到 Web/Native/Matrix 都能直接打到 Trillionnium-side API surface，而不是只靠 CLI smoke。
- 当前 Trillionnium 主仓新增 standalone browser parity gate：`scripts/check_trillionnium_world_browser_parity.sh` 会打开 `/world/play`，从浏览器直接执行进入世界、移动、训练、战斗、任务、状态读回，并写入 `acceptance/S3_browser_parity/latest/browser-parity.json`。
- 当前 Trillionnium 主仓新增 repository adapter boundary gate：`scripts/check_trillionnium_world_repository_adapter_boundary.sh` 生成 SQLite/Postgres schema，执行 SQLite read/write smoke，并写入 `acceptance/S3_repository_adapter/latest/repository-adapter-boundary.json`；这证明 repository 可替换边界，不等同于已接入生产托管数据库。

### S4 — 地图合规与 map_pack Gate

目标：从 fixture OSM 走向可测试真实地图包，但不直接开 live ingestion。

任务：

- ADR：地图数据源、授权、缓存、离线、署名、敏感 POI、地理围栏、下架流程。
- `map_pack_manifest_signed.json`：canonical manifest + Ed25519 signature + key_id + key rotation + revocation。
- Attribution screenshots：Web/Native/Matrix 关键界面都要有。
- 敏感 POI filter report。
- 不满足时继续 fixture-only。
- 当前 Trillionnium 主仓已提供 `scripts/check_trillionnium_world_map_pack_gate.sh`，能对 fixture map_pack 生成 unsigned manifest、Ed25519 dev signature、attribution evidence、sensitive POI report 和 S4 summary；这只解除 fixture signed manifest 缺口，不等同于生产公网地图包准入。
- 当前 Trillionnium 主仓新增 production map-pack route：`docs/development/trillionnium-world-production-map-pack-adr-v1.md` 与 `scripts/check_trillionnium_world_production_map_pack_route.sh` 覆盖 key rotation/revocation、attribution screenshot plan、takedown/rollback drill；它的 `production_map_pack_route_green` 仍不等同于 `production_map_pack_public_ready_green`。
- 当前 Trillionnium 主仓新增 `scripts/check_trillionnium_world_map_modeling_gate.sh`，在 fixture map_pack 上派生 buildings / roads / greenery / terrain 四类模型并写入 `acceptance/S4_map_pack_gate/latest/map-modeling-gate.json`；它证明建模管线存在，但仍明确 `fixture_only=true`、`live_ingestion_enabled=false`、`runtime_clients_fetch_public_osm_directly=false`。该 artifact 现在也由 packet integrity 的 `map_modeling_gate_semantics` 直接校验 fixture map_pack modeling layers、no-live-ingestion 边界和 production/public evidence blockers。

### S5 — Native/Bevy Mobile Gate（当前引擎路径）

目标：把 Trillionnium World 接入 Native/Bevy 游戏引擎开发路径，同时保持 Rust server/API/projection 作为权威。

当前 Trillionnium 主仓要求 Native/Bevy client shell 直接进入开发路径：Bevy 只消费 Rust World API/projection，并把玩家输入作为 intent 交回 Rust command layer。当前已能构建 aarch64 Android `cdylib`，导出 `ANativeActivity_onCreate` / `android_main`，并在 Android platform jar 可用时生成已签名 debug APK；Android 真机矩阵和真实渲染/输入/资源包验证继续作为 S5 gate 收证据。

当前 Native/Bevy host 端追加 `scripts/check_trillionnium_world_bevy_vertical_slice.sh`、`scripts/check_trillionnium_world_bevy_first_playable.sh` 与 `scripts/check_trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay.sh`。前两者覆盖当前房间/出口、目标任务、触控按钮、训练/战斗/任务动作、Rust intent-only authority、开局指引和保存读回；keyboard replay gate 生成 `acceptance/S5_native_bevy_device/latest/bevy-build-branch-title-route-all-branch-keyboard-replay.json`，把 force/agility/craft 三条 build-title route 的键盘输入序列在 fresh Bevy runtime 中重放，并要求事件签名与最终 runtime 完全一致。该 keyboard replay artifact 现在以 `keyboard_replay_green` status 进入 packet，并由 `keyboard_replay_semantics` 直接校验三条 route 的 exact input signature path、final runtime state 和 no-credit 边界。

2026-05-17 的 S5 host-side 可玩性证据又补了三层：`scripts/check_trillionnium_world_bevy_action_coach.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-action-coach.json`，要求 `Enter/NumpadEnter` action coach 依次引导 `TALK -> TRAIN -> MOVE:north -> FIGHT`，现在以 `action_coach_green` status 进入 packet，并由 `action_coach_semantics` 直接校验 focused-action 执行链和 no-credit 边界；`scripts/check_trillionnium_world_bevy_player_hud_debug_layer.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-player-hud-debug-layer.json`，把玩家 HUD 与 DEBUG/INPUT diagnostics 分层，现在以 `player_hud_debug_layer_green` status 进入 packet，并由 `player_hud_debug_layer_semantics` 直接校验玩家 HUD、DEBUG/INPUT diagnostics 分层、final runtime 和 no-credit 边界；`scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-live-window-screenshot-sequence.json`，验证 11 帧 live window 截图、frame sequence、slot write、runtime texture manifest/probe/handle chain 和 contact sheet 非空变化。该 live-window artifact 以 `live_window_screenshot_sequence_green` status 进入 release-review packet，并由 packet integrity 的 `live_window_screenshot_sequence_semantics` 直接校验；`scripts/check_trillionnium_world_bevy_live_window_mouse_hit_test_sequence.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-live-window-mouse-hit-test-sequence.json`，现在以 `native_bevy_live_window_mouse_hit_test_sequence` 进入 packet，并由 `live_window_mouse_hit_test_sequence_semantics` 直接校验 XTest 鼠标点击、可见 Bevy button center、ordered frame changes、slot-A persistence、contact sheet 和 Android S5 no-claim 边界；`scripts/check_trillionnium_world_bevy_sprite_texture_sampling.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-sprite-texture-sampling.json`，现在以 `sprite_texture_sampling_green` status 进入 packet，并由 `sprite_texture_sampling_semantics` 直接校验 CPU-side Bevy Image/TextureAtlas sampling、asset-store registration、sprite bindings、四层 scene/material slots 和 no-credit 边界；`scripts/check_trillionnium_world_bevy_live_window_sampled_texture_correlation.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-live-window-sampled-texture-correlation.json`，现在以 `live_window_sampled_texture_correlation_green` status 进入 packet，并由 `live_window_sampled_texture_correlation_semantics` 直接校验 CPU texture-atlas sampling、same runtime manifest/handle IDs、map/hud/actor/feedback 四层 live-window 相关性和 no-credit 边界；`scripts/check_trillionnium_world_bevy_render_asset_eligibility.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-render-asset-eligibility.json`，现在以 `render_asset_eligibility_green` status 进入 packet，并由 `render_asset_eligibility_semantics` 直接校验 host-side Bevy Image/TextureAtlas 的 MAIN_WORLD + RENDER_WORLD eligibility、sprite render references、sampled-live correlation 和 no extraction/GPU/S5/public/OpenRA-copy 边界；这些都是 host 端 Native/Bevy 可玩闭环、输入协议和玩家可读性的本地证据，不替代 Android 真机 S5 evidence，也不声明 GPU upload、render-world extraction completed、production-ready UI 或 public-launch ready。

Classic modeling foundation 三个基础证据现在也进入 packet-level semantic chain：`scripts/check_trillionnium_world_bevy_classic_art_pack.sh`、`scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh` 和 `scripts/check_trillionnium_world_bevy_classic_isometric_modeling.sh` 分别由 `classic_asset_pack_semantics`、`classic_manifest_lint_semantics`、`classic_isometric_modeling_semantics` / `classic_isometric_modeling_ppm_semantics` 直接校验 project-owned manifest/PPM atlas、frame/scene/actor/clip lint、orthographic isometric depth-sorted modeling、非空 PPM evidence 和 low-spec renderer CEX/wgpu no-credit 边界。Classic visual foundation 三个基础证据也进入 packet-level semantic chain：`scripts/check_trillionnium_world_bevy_classic_scene_preview.sh`、`scripts/check_trillionnium_world_bevy_classic_model_catalog.sh` 和 `scripts/check_trillionnium_world_bevy_classic_renderer_probe.sh` 分别以 `classic_scene_preview_green`、`classic_model_catalog_green`、`classic_renderer_probe_green` status 进入 release-review packet，并由 `classic_scene_preview_semantics` / `classic_scene_preview_ppm_semantics`、`classic_model_catalog_semantics` / `classic_model_catalog_ppm_semantics`、`classic_renderer_probe_semantics` / `classic_renderer_probe_ppm_semantics` 直接校验 manifest 场景面板、模型目录 frame、renderer probe HUD/actor 像素、PPM 尺寸和 no-credit 边界。

Classic performance budget 两个基础证据现在也进入 packet-level semantic chain：`scripts/check_trillionnium_world_bevy_classic_input_frame_budget.sh` 和 `scripts/check_trillionnium_world_bevy_classic_render_budget.sh` 分别由 `classic_input_frame_budget_semantics` 与 `classic_render_budget_semantics` 直接校验 `NativeControlAction::Move -> apply_live_native_action -> classic_draw_scene` 输入响应链、p95/max input-frame 与 render budget、manifest-backed frame selection、非空 low-spec renderer samples 和 CEX/wgpu no-credit 边界。

Classic playtest runner / launcher 也进入 packet-level semantic chain：`scripts/check_trillionnium_world_bevy_classic_playtest_runner_status.sh` 由 `classic_playtest_runner_status_semantics` 直接校验 live `trillionnium-bevy-playtest.service`、release `trnm-world-bevy run` 二进制、low-spec classic renderer env、manifest/override paths、工作目录和 CEX path rejection；`scripts/check_trillionnium_world_bevy_classic_playtest_launcher.sh` 由 `classic_playtest_launcher_semantics` 直接校验 `CAMPAIGN:START/CONTINUE/REPLAY` title actions、campaign slot persistence、`league-coliseum` open-world resume、live release runner、classic renderer env 和 CEX/S5 no-credit boundary。

2026-05-19 的 low-spec classic renderer 继续按 RTS 标准补齐本地证据：`scripts/check_trillionnium_world_bevy_classic_rts_control_loop.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.json` 与 `.ppm`，要求控制组 1 同时覆盖多单位选择、移动目的地、编队线、攻击目标、命令 marker、攻击反馈、小地图、资源条、命令面板、战争迷雾/可视范围、生产队列、建筑队列、训练/建造进度、资源消耗反馈、单位血量卡、目标血量、技能/命令卡和冷却反馈像素。随后 `scripts/check_trillionnium_world_bevy_classic_rts_live_input_sequence.sh` 把 `RTS:SELECT -> RTS:QUEUE -> RTS:MOVE -> RTS:ATTACK -> RTS:ABILITY` 接到 `apply_live_native_action_with_source(classic_rts_live_input)`，并生成 `acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-input-sequence.json` 与 `.ppm`，要求五段真实 live input 全部 accepted，最终 command queue / production queue / attack target / active ability / target health 都由 native runtime 状态证明。`scripts/check_trillionnium_world_bevy_classic_rts_pathing_formation.sh` 继续把 live move 输入推进到路径和编队反馈：`RTS:MOVE:8,4:wedge` 必须生成可见 path tiles、blocked tile detour、formation slots、command marker 与 selection rings。`scripts/check_trillionnium_world_bevy_classic_rts_collision_engagement.sh` 再要求 blocked-detour 后的单位分散格、攻击接敌范围、contact flash 和 attack feedback 都由 native runtime 状态驱动画出来。`scripts/check_trillionnium_world_bevy_classic_rts_target_aggro_focus.sh` 继续把 `RTS:ATTACK:arena_creep_attack` 与 `RTS:ABILITY:focus_fire` 推进到目标优先级、aggro 锁定、集火单位队列、威胁条和对应 overlay 像素。`scripts/check_trillionnium_world_bevy_classic_rts_economy_build.sh` 再把 `RTS:QUEUE:harvest:gold_vein`、`RTS:QUEUE:build:watch_tower@7,4` 和 `RTS:QUEUE:train:worker` 推进到资源采集、工人路线、返仓、建造蓝图、建造进度和生产队列证据。`scripts/check_trillionnium_world_bevy_classic_rts_selection_minimap.sh` 继续把 `RTS:SELECT:box:frontline`、`RTS:MOVE:minimap:9,2:rally`、`RTS:SELECT:2` 和 `RTS:MOVE:6,5:split` 推进到框选 tile、两组控制组绑定、小地图 rally、分路命令状态与对应 overlay 像素。`scripts/check_trillionnium_world_bevy_classic_rts_build_lifecycle.sh` 再把 `RTS:QUEUE:build`、`complete`、`repair` 和 `cancel` 推进到建筑放置、完成、维修、取消退款、结构血量和对应 overlay 像素。`scripts/check_trillionnium_world_bevy_classic_rts_tech_tree.sh` 继续把 `RTS:QUEUE:faction:mirror_guard`、`research:wayfinder_code@town_hall`、`upgrade:iron_lacing@training_hall` 和 `unlock:relay_guard` 推进到阵营基地、科技依赖、研究进度、升级完成、单位/建筑解锁和对应 overlay 像素。`scripts/check_trillionnium_world_bevy_classic_rts_projectile_ability.sh` 再把 `RTS:ATTACK:arena_creep_attack`、`RTS:ABILITY:focus_fire` 和 `RTS:ABILITY:guard_break` 推进到远程投射轨迹、命中点、技能范围、伤害 tick、护甲/护盾结算和对应 overlay 像素。`scripts/check_trillionnium_world_bevy_classic_rts_ai_skirmish_pressure.sh` 继续把 `RTS:QUEUE:ai:skirmish_wave`、`RTS:MOVE:8,4:wedge`、`RTS:ATTACK:arena_creep_attack` 和 `RTS:ABILITY:guard_break` 推进到 AI 压力波、压迫路线、玩家反制线、撤退点、压力条和对应 overlay 像素。`scripts/check_trillionnium_world_bevy_classic_rts_objective_victory_loop.sh` 再把 `RTS:QUEUE:objective:claim:relay_beacon@6,5` 和 `RTS:QUEUE:objective:extract:relay_beacon@9,2` 推进到占点、撤离、胜利结算、失败风险压低和对应 overlay 像素。十二组 gate 均已纳入 `scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh` 与 release-review CI，用来防止只堆静态美术而没有 RTS 可操作反馈；它们仍是 host-side Bevy classic 证据，不声明 Android 真机或公网发布 ready。

同日继续追加第十三组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_creep_camp_terrain_route.sh` 把 `RTS:QUEUE:scout:creep_camp@8,3`、`RTS:MOVE:8,3:wedge`、`RTS:ATTACK:forest_creep_camp`、`RTS:ABILITY:guard_break` 和 `RTS:QUEUE:camp:clear:forest_creep_camp@8,3` 推进到侦察野怪营地、地形路线、瓶颈 tile、清营状态、侦察揭示条、第二目标/扩张 tile 与对应 overlay 像素；它同样纳入 `scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh` 与 release-review CI。

同日继续追加第十四组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_fog_scouting_intel.sh` 把 `RTS:QUEUE:recon:scout_enemy_base@10,2`、`RTS:MOVE:9,2:rally`、`RTS:QUEUE:recon:sweep:enemy_base@10,2`、`RTS:QUEUE:recon:watchtower_scan@7,4` 和 `RTS:QUEUE:recon:mark:enemy_base@10,2` 推进到侦察小队、fog reveal、敌方建筑/单位情报、intel log、可见度条与小地图情报 overlay；它继续保持 Bevy native source-of-truth，不引入任何外部 RTS IP 资产或 CEX player runtime。

同日继续追加第十五组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_enemy_base_tech_pressure.sh` 把侦察后的 `enemy:tech`、`enemy:train`、`counter:research`、`counter:fortify` live inputs 推进到敌方基地科技升级、敌方出兵压力波、玩家反制科技、防御结构就绪、压力警戒条与小地图压力线；它把 fog/intel 结果接到后续对局决策，而不是停在发现敌方基地。

同日继续追加第十六组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_army_production_rally.sh` 把 `army:supply`、`army:train`、`army:rally`、`army:assign` live inputs 推进到人口上限、多批训练、出兵集合、控制组编队、兵种组合日志、低配 HUD 供给条与小地图 rally 线；它把基地生产从单次排队扩成可读的部队补员循环。

同日继续追加第十七组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_base_assault_resolution.sh` 把 `army`、`move`、`attack`、`assault:breach` live inputs 接成 control group 3 推进敌方基地的闭环，要求集合后的部队沿 assault path 攻击 `enemy_barracks`，同时显示敌方建筑血条、破防进度、基地结果状态、奖励日志、场景路径 overlay 与小地图 assault 线；它继续保持 Bevy-native source-of-truth 和 IP-clean 美术/命名，不引入外部 RTS 资产。

同日继续追加第十八组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_battle_aftermath.sh` 把 base assault 之后的 `aftermath:destroy`、`aftermath:promote`、`aftermath:next` live inputs 接到建筑毁坏、废墟/烟尘、老兵升级、战果状态、奖励结算和下一步行动提示；它让“打掉敌方兵营”之后有可读的战后反馈和继续操作入口，而不是只停在血条归零。

同日继续追加第十九组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_commander_progression.sh` 把 aftermath reward 接到原创指挥官成长：`commander:loot` 生成战利品拾取，`commander:level` 提升 `mirror_captain` 并给技能点，`commander:ability:rally_aura` 消耗技能点生成 aura tile 和技能按钮反馈；它借鉴经典 RTS“英雄成长”的可玩结构，但命名、数据和像素资产均为 Trillionnium 原创。

同日继续追加第二十组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_expansion_counterattack.sh` 把 `secure_expansion` 下一步落成可操作扩张经营：`expansion:claim` 占领 `forest_relay`，`expansion:build` 建成 `relay_outpost`，`expansion:workers` 接入二矿收入曲线，`expansion:defend` 触发并守住敌方 `counter_wave`；它把指挥官 aura、二基地经济、敌方反扑和防守结果收进同一条 Bevy-native live input 证据链。

2026-05-20 继续追加第二十一组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_tier_two_siege_push.sh` 把守住二基地后的 `tier2:tech`、`tier2:upgrade`、`tier2:train`、`tier2:enemy_fortify` 和 `tier2:push` live inputs 推进到二本科技建筑、攻城升级、攻城单位生产、敌方加固据点、推进路线和攻城伤害反馈；它让扩张经济转化为下一轮基地突破能力，同时继续保持 Trillionnium 原创命名、像素和 Bevy-native source-of-truth。

同日继续追加第二十二组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_siege_breach_counterplay.sh` 把二本攻城从“打出破口”推进到“敌方修门/侧翼反扑/玩家守线/最终破门”的反应闭环：`tier2:breach` 打开 `gate_bulwark` 破口窗口，`tier2:enemy_repair` 和 `tier2:enemy_flank` 驱动敌方反制，`tier2:hold` 用指挥官光环稳住攻城线，`tier2:finish` 结算 100% 破门、奖励和下一步 `enter_inner_lane`；整条证据仍来自 Bevy native live input 与 Trillionnium-owned low-spec 渲染。

同日继续追加第二十三组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_inner_lane_breakthrough.sh` 把 `enter_inner_lane` 下一步落成内线战役推进：`tier2:inner_route` 进入内线通道，`tier2:inner_gate` 标记内门锁，`tier2:inner_supply` 送入补给队，`tier2:inner_split` 建立分兵路线，`tier2:inner_clear` 清掉二线守军，`tier2:inner_secure` 占领 `signal_core` 并给出下一步 `press_central_keep`；它把破门胜利继续接成有补给、分兵、二线防守和核心目标的中后期循环。

同日继续追加第二十四组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_central_keep_pressure.sh` 把 `press_central_keep` 做成中央要塞压制前置：`tier2:keep_route` 标记通往 `central_keep` 的路线，`tier2:keep_shield` 读出 `mirror_ward` 护盾，`tier2:keep_guard` 揭示守卫线，`tier2:keep_siege` 排出最终攻城线，`tier2:keep_pressure` 把护盾压到 24%、要塞血量压到 58% 并给出下一步 `break_central_keep`；整条仍由 Bevy native live input 和 Trillionnium-owned 低配渲染证据约束。

同日继续追加第二十五组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_central_keep_breakthrough.sh` 把 `break_central_keep` 做成终局破要塞/占领结算：`tier2:keep_breach` 打开中央要塞破口，`tier2:guardian_counter` 触发最终守卫反扑，`tier2:keep_hold` 稳住攻城线，`tier2:keep_break` 把要塞血量打到 0，`tier2:keep_claim` 完成 `classic_rts_victory:central_keep` 并给出下一步 `restore_mirror_city`；它把前面的压制 loop 接成可验证胜利闭环。

同日继续追加第二十六组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_mirror_city_restoration.sh` 把 `restore_mirror_city` 做成胜利后的恢复交接：`tier2:restore_city` 恢复 `mirror_city` 四个区域，`tier2:rebuild_core` 重建 `signal_core` 等核心设施，`tier2:assign_garrison` 配置守军，`tier2:victory_handoff` 进入 `classic_rts_restored:mirror_city` 并给出下一步 `open_world_after_action`；这个 gate 只覆盖胜利后的新增 live inputs，前置胜利链由 `central_keep_breakthrough` dependency gate 约束。

同日继续追加第二十七组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_open_world_after_action.sh` 把 `open_world_after_action` 做成 RTS 胜利后的开放世界恢复：`tier2:open_world` 打开回城路线，`tier2:open_world_route` 接回 `league-coliseum` 路线导演 / 任务面板，`tier2:open_world_resume` 恢复 `arena_outdoor` 房间、`task-fixture-first-route` active task、combat contextual deck；这个 gate 证明 RTS 终局不是停在 next-action 字符串，而是回到 Rust-owned open-world surface。

同日继续追加第二十八组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_campaign_handoff.sh` 把前面分段 gate 重新压成一条 live native input 全链路：从初始 RTS control group、objective victory、creep camp、recon、enemy tech/counter、army rally、base assault、aftermath、commander、expansion、tier-two siege、inner lane、central keep、Mirror City restoration，一直跑到 `league-coliseum` 的 open-world resume；这个 gate 专门防止分段证据只靠 dependency helper 串联，而没有一条连续可回放路线，并且要求最终 open-world handoff 通过 native snapshot save/restore 后仍保持 route director、active task 和 combat contextual deck。

继续追加第二十九组 RTS gate：`scripts/check_trillionnium_world_bevy_classic_rts_campaign_entry.sh` 把这条 campaign handoff 推到玩家可见入口。Bevy title surface 现在暴露 `CAMPAIGN:START`、`CAMPAIGN:CONTINUE`、`CAMPAIGN:REPLAY` 三个 native actions；`START/REPLAY` 从标题菜单触发完整 73 步 campaign handoff、写入 `NativePlayableSaveSnapshot`，`CONTINUE` 从 campaign slot 恢复到 `league-coliseum` open-world handoff 并要求 `CONTINUE:SESSION` 解锁地图控制。它纳入 classic playtest readiness 与 release-review CI，防止 campaign 只存在于 evidence CLI、玩家入口不可达。

继续追加第三十组 playtest gate：`scripts/check_trillionnium_world_bevy_classic_playtest_launcher.sh` 把玩家启动条件合成一个单一证据：live `trillionnium-bevy-playtest.service` 必须运行 release `trnm-world-bevy`、使用 classic low-spec renderer/manifest、不带 CEX runtime path；标题页必须有 `CAMPAIGN:START/CONTINUE/REPLAY`，campaign slot 必须可写入并恢复到 `league-coliseum` / `arena_outdoor` / `COMBAT:attack`。它纳入 classic playtest readiness 与 release-review CI，防止“入口存在”和“服务运行”分别为绿但无法交付给测试玩家。

继续追加 playtest handoff 交付层：`scripts/check_trillionnium_world_bevy_classic_playtest_handoff_readiness.sh` 聚合 full classic playtest readiness、launcher、runner status、observability readiness，证明本机 release runner、标题入口、campaign slot、open-world resume 和观测证据可以作为 human-playtest handoff 使用；`scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh` 进一步生成 `acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.json` 与 `.md`，checksum-binding handoff/readiness/launcher/runner/observability 五个 artifact，并明确不授予 public launch、S5 真机或 OpenRA natural replay/headless parity credit。

继续追加 UI / 地图引擎 / 建模全量对齐矩阵：`scripts/check_trillionnium_world_ui_map_modeling_full_alignment.sh` 生成 `acceptance/S6_public_launch/latest/trnm-world-ui-map-modeling-full-alignment.json` 与 `.md`，把 Bevy human-playtest handoff、map/UI/modeling readiness、isometric/model catalog、fixture map modeling、production map-pack public evidence、S5 real-device validation 和 public-launch readiness 放进同一份证据。当前它只能给 `host_side_ui_map_modeling_aligned_public_evidence_blocked`：本机 UI、fixture map modeling、原创低规格建模已对齐，但 production map-pack、S5 真机和公开发布证据仍 blocked。

继续按桌面优先顺序追加真机/本机窗口测试层：`scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-desktop-real-machine-readiness.json` 与 `.md`，刷新并绑定 release runner、桌面 X11 live-window 截图序列、XTest 键盘输入、live-window pixel/texture correlation、sampled texture correlation 和 handoff packet；它明确 `desktop_before_mobile_gate=true` 且 `android_s5_real_device_not_required_gate=true`，移动端 S5 仍放最后。

继续追加第三十一组 RTS visual gate：`scripts/check_trillionnium_world_bevy_classic_rts_visual_fidelity.sh` 专门回应“画面还不像成熟 RTS、建模和 NPC 动作粗糙”的问题。它不复制《魔兽争霸 III》的资产、命名或 UI 图形，只把成熟 RTS 的信息架构作为质量方向：实际 classic scene renderer 现在要画底部指挥栏、选中单位头像/状态卡、3x3 command grid、队伍行动日志、单位轮廓高光、NPC attack/carry/idle 的动作差异，并由 `mature_rts_hud_gate`、`model_fidelity_gate`、`npc_animation_gate` 和 `original_art_policy_gate` 卡住。

继续追加第三十二组 RTS command-affordance gate：`scripts/check_trillionnium_world_bevy_classic_rts_command_affordance.sh` 把“像即时战略”的操作反馈补进实际战场渲染路径。它要求 live native input 依次经过拖选、右键移动、攻击目标、能力热键确认，并在 `classic_draw_scene` 中画出拖选 marquee、右键落点、攻击光标、鼠标箭头、控制组/热键条、命令确认反馈；仍然只使用原创 Trillionnium 低配 2.5D/isometric RTS 表达，不复制《魔兽争霸 III》的光标、UI 图形、资产、文本、名称或模型。

继续追加第三十三组 RTS action-cadence gate：`scripts/check_trillionnium_world_bevy_classic_rts_action_cadence.sh` 专门压住 NPC 动作粗糙的问题。实际单位素材生成和 `classic_draw_scene` 现在必须呈现攻击前摇、命中、收招、工人搬运起伏、待机呼吸和脚底拖影节奏；evidence contact sheet 由六帧真实场景渲染构成，并用 `windup_gate`、`strike_gate`、`recovery_gate`、`carry_bob_gate`、`idle_breath_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 阻止只改静态截图或引入外部 RTS IP 素材。

继续追加第三十四组 RTS unit-model-depth gate：`scripts/check_trillionnium_world_bevy_classic_rts_unit_model_depth.sh` 专门压住低配单位建模像方块人的问题。guard/worker/creep 的原创像素素材和实际 `classic_draw_scene` 渲染现在必须有轮廓 rim、护甲/硬边层、角色道具、面部暗面、脚底接触阴影和身体层叠阴影；它通过 `rim_gate`、`armor_gate`、`role_prop_gate`、`face_shade_gate`、`ground_contact_gate`、`layer_shadow_gate`、`role_coverage_gate` 与 `original_art_policy_gate` 保证建模层次不是只靠 HUD 描述。

继续追加第三十五组 RTS action-sequence gate：`scripts/check_trillionnium_world_bevy_classic_rts_action_sequence.sh` 把“看起来像在动”推进到可回放的动作阶段。实际 `classic_draw_scene` 必须由 combat event / runtime tick 选出 idle、windup、strike、recovery、carry_up、carry_down 六段，并在同一低配 2.5D 战场里画出前摇轨迹、命中爆点、收招回弹、搬运上下拍和帧残影；`sequence_phase_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证它是 Trillionnium 原创单位在本地渲染器里的动作序列，而不是静态贴图或外部 RTS IP 素材。

继续追加第三十六组 RTS NPC behavior gate：`scripts/check_trillionnium_world_bevy_classic_rts_npc_behavior.sh` 把单帧动作阶段再推进到角色行为读感。实际 `classic_draw_scene` 现在必须按 runtime behavior event 画出 guard patrol / guard engage / worker work / worker carry / creep stalk / creep retreat 六种 NPC 行为标记和路线残影；`behavior_stage_gate`、`route_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证战场上能看出巡逻、接战、采集搬运、潜伏和撤退这些连续角色意图，同时仍只使用原创 Trillionnium 低配 RTS 表达。

继续追加第三十七组 RTS combat-impact gate：`scripts/check_trillionnium_world_bevy_classic_rts_combat_impact.sh` 把战斗从“单位在动”推进到“命中和结算一眼可读”。实际 `classic_draw_scene` 必须按 combat impact event 画出 hit_flash、stagger、damage_tick、death_fall、corpse_dissolve、victory_settle 六段，包括受击闪光、硬直拖动、血条掉点、倒地、尸体消散和胜利收束；`impact_stage_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证这些反馈来自 Trillionnium 本地渲染器和原创低配素材，而不是静态战报或外部 RTS IP 素材。

继续追加第三十八组 RTS locomotion-blend gate：`scripts/check_trillionnium_world_bevy_classic_rts_locomotion_blend.sh` 把单位移动从“瞬移到格子”推进到“路径、脚步、转身、编队滑移和刹停都能读出来”。实际 `classic_draw_scene` 必须按 locomotion event 画出 path_commit、footstep_left、footstep_right、turn_arc、formation_slide、arrival_brake 六段；`locomotion_stage_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证移动连续感来自 Trillionnium 原创低配 isometric renderer，而不是静态路线图或外部 RTS 动画数据。

继续追加第三十九组 RTS NPC transition gate：`scripts/check_trillionnium_world_bevy_classic_rts_npc_transition.sh` 把 NPC 从“切状态”推进到“状态之间有过渡动作”。实际 `classic_draw_scene` 必须按 transition event 画出 alert_turn、patrol_engage、work_carry、stalk_pounce、hit_recover、retreat_resume 六段；`transition_stage_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证巡逻警戒、接战、采集搬运、潜伏扑击、受击恢复和撤退回归都在原创 Trillionnium renderer 里可读。

继续追加第四十组 RTS depth-readability gate：`scripts/check_trillionnium_world_bevy_classic_rts_depth_readability.sh` 把战场从“有单位动作”推进到“单位、建筑、前景地形互相遮挡时仍能读懂”。实际 `classic_draw_scene` 必须按 depth event 画出 foreground_canopy、behind_silhouette、building_mask、target_priority、path_occlusion、terrain_cutaway 六段；`depth_stage_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证前景树冠、建筑遮挡、目标优先级、路径穿行和地形剖切都在原创 Trillionnium 低配 2.5D renderer 中可读，不借用外部 RTS IP 素材或 UI 表达。

继续追加第四十一组 RTS command-surface gate：`scripts/check_trillionnium_world_bevy_classic_rts_command_surface.sh` 把底部 RTS 面板从“有 HUD”推进到“能读选择、命令、冷却、目标和队列”。实际 `classic_draw_scene` 必须按 surface event 画出 selection_state、command_grid、cooldown_disabled、target_queue 四段，包括多单位卡选中框、控制组页签、九宫格命令 ready 态、冷却扫面、禁用格、目标信息面板和队列确认；`selection_surface_gate`、`command_grid_surface_gate`、`cooldown_disabled_surface_gate`、`target_queue_surface_gate`、`surface_stage_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证界面反馈来自原创 Trillionnium Bevy 低配 renderer，而不是静态 UI 拼图或外部 RTS IP 素材。

继续追加第四十二组 RTS structure-modeling gate：`scripts/check_trillionnium_world_bevy_classic_rts_structure_modeling.sh` 把建筑从“地图块/图标”推进到“结构状态一眼可读”。实际 `classic_draw_scene` 必须按 structure event 画出 foundation_shadow、scaffold、construction_spark、production_glow、damage_crack、repair_beam 六段，包括地基阴影、脚手架、建造火花、生产发光、受损裂纹和维修光束；`foundation_gate`、`scaffold_gate`、`construction_spark_gate`、`production_glow_gate`、`damage_crack_gate`、`repair_beam_gate`、`structure_stage_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证建筑建造、生产、受损和维修状态都在原创 Trillionnium Bevy 低配 renderer 中可读，不借用外部 RTS IP 素材、模型或动画数据。

继续追加第四十三组 RTS environment-life gate：`scripts/check_trillionnium_world_bevy_classic_rts_environment_life.sh` 把战场从“静态棋盘”推进到“有场景生命感”。实际 `classic_draw_scene` 必须按 environment event 画出 tree_sway、torch_flicker、water_shimmer、banner_flutter、resource_glint、ambient_dust 六段，包括树冠摆动、火把闪烁、水面高光、旗帜飘动、资源点闪光和行军尘土；`tree_sway_gate`、`torch_flicker_gate`、`water_shimmer_gate`、`banner_flutter_gate`、`resource_glint_gate`、`ambient_dust_gate`、`environment_stage_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证场景动态都来自原创 Trillionnium Bevy 低配 renderer，不借用外部 RTS IP 素材、模型或动画数据。

继续追加第四十四组 RTS worker-harvest-animation gate：`scripts/check_trillionnium_world_bevy_classic_rts_worker_harvest_animation.sh` 把资源采集从“路线/图标”推进到“工人动作循环可读”。实际 `classic_draw_scene` 必须按 harvest animation event 画出 approach、tool_swing、resource_pop、carry_load、dropoff_burst、return_path 六段，包括接近资源点、挥工具、资源弹出、负载搬运、交付爆点和返回路径；`approach_gate`、`tool_swing_gate`、`resource_pop_gate`、`carry_load_gate`、`dropoff_burst_gate`、`return_path_gate`、`harvest_stage_gate`、`economy_runtime_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证采集动作来自 Trillionnium 原创 Bevy 低配 renderer，并且仍绑定本地 economy runtime 的 worker assignment、harvest node、dropoff 和 resource delta。

继续追加第四十五组 RTS production-spawn-animation gate：`scripts/check_trillionnium_world_bevy_classic_rts_production_spawn_animation.sh` 把部队生产从“生产结果可见”推进到“出兵循环动作可读”。实际 `classic_draw_scene` 必须按 production spawn animation event 画出 queue_pulse、training_tick、spawn_door、rally_flag、formation_join、supply_flash 六段，包括队列脉冲、训练进度、出兵门、集结旗、编队归队和人口闪烁；`queue_pulse_gate`、`training_tick_gate`、`spawn_door_gate`、`rally_flag_gate`、`formation_join_gate`、`supply_flash_gate`、`production_stage_gate`、`production_runtime_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证出兵动作来自 Trillionnium 原创 Bevy 低配 renderer，并且仍绑定本地 army production/rally runtime 的 supply、batch、spawned unit、rally tile、training progress 与 control-group 状态。

继续追加第四十六组 RTS unit-status-portrait gate：`scripts/check_trillionnium_world_bevy_classic_rts_unit_status_portrait.sh` 把底部状态区从“有头像”推进到“选中对象状态可读”。实际 `classic_draw_scene` 必须按 unit status portrait event 画出 worker、guard、commander、creep_target、structure、multi_select 六类状态面板，包括 portrait frame、HP、能量、XP、buff/role 徽章、队列/命令状态；`portrait_frame_gate`、`health_bar_gate`、`mana_bar_gate`、`xp_bar_gate`、`buff_badge_gate`、`role_badge_gate`、`queue_badge_gate`、`status_stage_gate`、`status_runtime_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证 UI 反馈来自 Trillionnium 原创 Bevy 低配 renderer，并且仍绑定 native unit/structure/target runtime。

继续追加第四十七组 RTS selection-command-feedback gate：`scripts/check_trillionnium_world_bevy_classic_rts_selection_command_feedback.sh` 把玩家下令瞬间从“有按钮/有面板”推进到“操作反馈可读”。实际 `classic_draw_scene` 必须按 selection command feedback event 画出 marquee_start、selection_confirm、rally_preview、move_line、attack_lock、invalid_order 六类反馈，包括拖选框、选中确认、集结预览、移动命令线、攻击锁定和无效命令提示；`marquee_gate`、`confirm_gate`、`rally_gate`、`move_gate`、`attack_gate`、`error_gate`、`ack_gate`、`feedback_stage_gate`、`command_runtime_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证操作反馈来自 Trillionnium 原创 Bevy 低配 renderer，并且仍绑定 native selection/rally/move/attack/blocked-order runtime。

继续追加第四十八组 RTS ability-tooltip-telegraph gate：`scripts/check_trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph.sh` 把技能按钮从“能触发”推进到“下令前后的预期反馈可读”。实际 `classic_draw_scene` 必须按 ability tooltip telegraph event 画出 hover_tooltip、range_preview、cast_windup、cooldown_sweep、queue_explain、resource_warning 六类反馈，包括技能说明、范围预览、施放蓄力、冷却扫面、队列说明和资源/人口不足警告；`tooltip_gate`、`range_gate`、`windup_gate`、`cooldown_gate`、`queue_gate`、`warning_gate`、`telegraph_stage_gate`、`ability_runtime_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证技能反馈来自 Trillionnium 原创 Bevy 低配 renderer，并且仍绑定 native ability/cooldown/queue/resource runtime。

继续追加第四十九组 RTS control-group-hotkey-feedback gate：`scripts/check_trillionnium_world_bevy_classic_rts_control_group_hotkey_feedback.sh` 把编队与快捷键从“状态字段存在”推进到“键盘 RTS 操作反馈可读”。实际 `classic_draw_scene` 必须按 control group hotkey feedback event 画出 assign_group、recall_group、double_tap_camera、idle_worker_ping、production_hotkey、ability_hotkey_ack 六类反馈，包括编队写入、编队召回、双击镜头跳转、空闲工人提示、生产快捷键和技能快捷键确认；`assign_gate`、`recall_gate`、`camera_gate`、`idle_gate`、`production_gate`、`ability_gate`、`hotkey_stage_gate`、`hotkey_runtime_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证快捷键反馈来自 Trillionnium 原创 Bevy 低配 renderer，并且仍绑定 native control-group、production queue、ability cooldown 与 input feedback runtime。

继续追加第五十组 RTS scrollable-map gate：`scripts/check_trillionnium_world_bevy_classic_rts_scrollable_map.sh` 把地图从“单屏战场”推进到“可滚动大图”。实际 Bevy runtime 必须支持 Shift+WASD/方向键平移、边缘滚动、中键拖拽、滚轮缩放、小地图跳转、边界 clamp、map layer 投影和 HUD 固定；`keyboard_pan_gate`、`edge_scroll_gate`、`drag_pan_gate`、`wheel_zoom_gate`、`minimap_jump_gate`、`boundary_clamp_gate`、`map_layer_projection_gate`、`hud_fixed_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证镜头导航由 native RTS camera reducer 和原创低配 renderer 共同证明。

继续追加第五十一组 RTS camera-minimap-sync gate：`scripts/check_trillionnium_world_bevy_classic_rts_camera_minimap_sync.sh` 把可滚动镜头继续推进到“小地图与镜头状态同步可读”。实际 `classic_draw_scene` 必须按 camera/minimap sync stage 画出 viewport_rect、fog_reveal、selection_follow、control_group_recall、route_projection、zoom_sync 六类反馈，包括小地图视口框、战争迷雾揭示、选中单位跟随、编队召回、小地图路线投影和缩放后的视口框变化；`viewport_sync_gate`、`fog_reveal_gate`、`selection_follow_gate`、`control_group_sync_gate`、`route_projection_gate`、`zoom_rect_sync_gate`、`minimap_runtime_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证小地图反馈来自 Trillionnium 原创 Bevy 低配 renderer，并且仍绑定 native camera reducer、viewport rect、reveal tiles 和 selection-follow runtime。

继续追加第五十二组 RTS command-queue-path-preview gate：`scripts/check_trillionnium_world_bevy_classic_rts_command_queue_path_preview.sh` 把“下令后才知道结果”推进到“命令队列与路径预览可读”。实际 `classic_draw_scene` 必须按 command queue path preview stage 画出 queue_stack、shift_waypoints、rally_chain、attack_focus、build_reservation、cancel_repath 六类反馈，包括队列槽位、Shift 路点、集结链、攻击焦点、建造预留和取消/重寻路提示；`live_input_gate`、`queue_stack_gate`、`shift_waypoint_gate`、`rally_chain_gate`、`attack_focus_gate`、`build_reservation_gate`、`cancel_repath_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证这些反馈来自 accepted Bevy native RTS control actions 和原创低配 renderer，不复制外部 RTS UI/光标/模型/文本资产。

继续追加第五十三组 RTS formation-move-preview gate：`scripts/check_trillionnium_world_bevy_classic_rts_formation_move_preview.sh` 把“命令队列与路径预览可读”推进到“编队移动、拥挤避让、阻挡路径和落点 ghost 在下令前就可读”。实际 `classic_draw_scene` 必须按 formation move preview stage 画出 destination_ghost、wedge_spacing、line_reflow、collision_avoidance、split_avoidance、commit_spacing 六类反馈，包括落点 ghost、编队槽位、路径线、阻挡/绕行、分散避让和最终 spacing commit；`live_input_gate`、`destination_ghost_gate`、`wedge_spacing_gate`、`line_reflow_gate`、`collision_avoidance_gate`、`split_avoidance_gate`、`commit_spacing_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证这些预览来自 accepted Bevy native RTS move/control actions、runtime pathing/formation/collision state 和原创低配 renderer，不复制外部 RTS UI/光标/模型/文本资产。

继续追加第五十四组 RTS formation-move-execution gate：`scripts/check_trillionnium_world_bevy_classic_rts_formation_move_execution.sh` 把“下令前 preview 可读”推进到“下令后执行也可信”。实际 `classic_draw_scene` 必须按 formation move execution stage 画出 slot_claim、path_reservation、stagger_step、crowd_avoidance、blocked_reroute、arrival_lock 六类反馈，包括单位槽位声明、路径预留、错步移动、拥挤避让、阻挡重寻路和到达刹停/锁位；`live_input_gate`、`slot_claim_gate`、`path_reservation_gate`、`stagger_step_gate`、`crowd_avoidance_gate`、`blocked_reroute_gate`、`arrival_lock_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证执行反馈来自 accepted Bevy native RTS control actions、runtime pathing/formation/collision state、slot/reservation logs 和原创低配 renderer，不复制外部 RTS UI/光标/模型/文本资产。

继续追加第五十五组 RTS local-obstruction-recovery gate：`scripts/check_trillionnium_world_bevy_classic_rts_local_obstruction_recovery.sh` 把“编队执行可信”推进到“局部堵路恢复也可信”。实际 `classic_draw_scene` 必须按 local obstruction recovery stage 画出 detect_block、hold_queue、side_step、gap_claim、flow_resume 五类反馈，包括阻挡检测、后排排队、侧步让缝、空隙认领和不丢命令的恢复流动；`live_input_gate`、`detect_block_gate`、`hold_queue_gate`、`side_step_gate`、`gap_claim_gate`、`flow_resume_gate`、`scene_renderer_gate` 与 `original_art_policy_gate` 保证堵路恢复来自 accepted Bevy native RTS move/control actions、runtime blocked/disperse/slot/route state 和原创低配 renderer，不复制外部 RTS UI/光标/模型/文本资产。

补齐 classic RTS production art / asset / UI skin / interaction polish / full-screen UI replication / shell-meta UI replication / match setup UI replication / campaign outcome UI readiness / campaign UI continuity / in-match HUD state replication / session state continuity / combat readability pressure readiness / desktop review 十三组本机证据：`scripts/check_trillionnium_world_bevy_classic_rts_production_art_replication.sh` 要求实际 `classic_draw_scene` 呈现原创 guard/worker/creep/structure/environment/FX 的 sprite、轮廓、动作帧、材质提示和 live runtime 绑定，不把外部 RTS IP 的模型、命名或 UI 复制进来；`scripts/check_trillionnium_world_bevy_classic_rts_production_asset_atlas.sh` 把这些 production art 切成 texture-atlas 家族、frame、UV rect、runtime binding lane 和 source preview；`scripts/check_trillionnium_world_bevy_classic_rts_production_ui_skin.sh` 再把 atlas 绑定到 HUD chrome、command grid、小地图 bezel、unit card、tooltip panel、feedback markers、hotkey strip、status bars 八个 UI surface；`scripts/check_trillionnium_world_bevy_classic_rts_production_interaction_polish.sh` 继续把 UI skin 绑定到拖选、右键移动、攻击锁定、建造 ghost、队列路径和 scroll/minimap feedback；`scripts/check_trillionnium_world_bevy_classic_rts_full_screen_ui_replication.sh` 把 title/campaign entry、tactical viewport、map/minimap camera、production HUD skin、command interactions、build + tech tree、unit status card、ability/combat UI、campaign outcome 和 open-world handoff 十个 Rust/Bevy full-screen UI surface 绑成一张 1280x768 本机 runtime screen；`scripts/check_trillionnium_world_bevy_classic_rts_shell_meta_ui_replication.sh` 把 title/account、character create、session slot、save/load confirm、load/resume、session recovery、pause/resume、settings、input HUD、button hit-test 和 first-minute handoff 十二个 shell/meta runtime surface 继续绑定到内部 Rust/Bevy evidence；`scripts/check_trillionnium_world_bevy_classic_rts_match_setup_ui_replication.sh` 把 campaign actions、map select、faction select、spawn slots、resource rules、bot/difficulty、victory conditions、minimap preview、start ready 和 no-external boundary 十个 pre-match setup surface 绑定到 shell/meta、campaign entry、First Contact Basin map spec、map UI readiness 和 tech/faction evidence；`scripts/check_trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness.sh` 把 first-minute campaign entry、objective victory、base assault、battle aftermath 和 open-world route resume 绑成 player runtime campaign outcome screen evidence；`scripts/check_trillionnium_world_bevy_classic_rts_campaign_ui_continuity.sh` 再把 live native campaign handoff、16 帧 continuity capture、`league-coliseum` open-world resume、contextual combat action 与 restore 后 UI 状态绑成恢复后一致的 campaign UI continuity evidence；`scripts/check_trillionnium_world_bevy_classic_rts_in_match_hud_state_replication.sh` 把资源、选中单位、控制组、命令队列、小地图可见性、生产/建造队列、ability cooldown、combat alert 和 objective pressure 绑成 live in-match HUD state replication evidence；`scripts/check_trillionnium_world_bevy_classic_rts_session_state_continuity.sh` 把 match setup、slot A save/confirm、load-resume input lock、continue unlock、恢复后的 in-match HUD、campaign outcome reward state 与 `league-coliseum` open-world resume 串成 session state continuity evidence；`scripts/check_trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness.sh` 把 unit status portrait、selection command feedback、ability tooltip telegraph、depth readability 和 central keep pressure 五个 combat UI/pressure surface 绑成 player runtime combat pressure screen evidence；`scripts/check_trillionnium_world_bevy_classic_rts_production_desktop_review_packet.sh` 再把 production interaction polish 与 local Linux X11 desktop keyboard/mouse review packet、截图 contact sheet 和 mouse hit-test evidence 绑成可复查 handoff。前十二组 gate 纳入 `scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh`，十三组都纳入 release-review CI；keyboard replay、classic animation preview/selector、classic player motion、action coach、player HUD/debug layer、player UI rescue、live-window screenshot sequence、sprite texture sampling、live-window sampled texture correlation、render asset eligibility、classic playtest readiness、classic playtest launcher、campaign UI continuity JSON/PPM、full-screen UI replication、shell/meta UI replication、match setup UI replication、campaign outcome UI readiness、in-match HUD state replication、session state continuity、combat readability/pressure readiness、camera/minimap sync JSON/PPM、desktop review packet 和 live-window mouse hit-test sequence 都进入 release-review packet，artifact count 现在为 `113`，并由 packet integrity 的 `keyboard_replay_semantics`、`classic_animation_preview_semantics`、`classic_animation_preview_ppm_semantics`、`classic_animation_selector_semantics`、`classic_player_motion_probe_semantics`、`classic_player_motion_probe_ppm_semantics`、`action_coach_semantics`、`player_hud_debug_layer_semantics`、`player_ui_rescue_semantics`、`live_window_screenshot_sequence_semantics`、`live_window_mouse_hit_test_sequence_semantics`、`sprite_texture_sampling_semantics`、`live_window_sampled_texture_correlation_semantics`、`render_asset_eligibility_semantics`、`classic_playtest_readiness_semantics`、`classic_playtest_launcher_semantics`、`full_screen_ui_replication_semantics`、`shell_meta_ui_replication_semantics`、`match_setup_ui_replication_semantics`、`campaign_outcome_ui_readiness_semantics`、`campaign_ui_continuity_semantics`、`campaign_ui_continuity_ppm_semantics`、`in_match_hud_state_replication_semantics`、`session_state_continuity_semantics`、`combat_readability_pressure_readiness_semantics`、`classic_rts_camera_minimap_sync_semantics`、`classic_rts_camera_minimap_sync_ppm_semantics` 和 `production_desktop_review_packet_semantics` 直接校验。整条链只证明 host-side Bevy 原创素材/界面皮肤/键盘输入重放/animation preview-selector/player motion 输入到行走帧/操作反馈/player-first UI rescue/launcher/title action/campaign slot/open-world resume/全屏 UI / shell-meta UI / match setup UI / campaign outcome runtime screen / continuity restore / in-match HUD state / session resume 复刻/combat readability pressure runtime screen/camera-minimap sync/桌面本机复查与鼠标命中链路，不声明 production-ready UI、Android S5 真机、GPU upload、render-world extraction completed 或 public-launch ready。

Go 条件：

- 中端 Android 30 FPS。
- Android NativeActivity `.so` 与 signed debug APK artifact ready。
- 中文显示/输入、前后台、弱网、资源包、崩溃诊断通过。
- 客户端仅提交 intent；Rust server 仍权威。
- 能复用 Rust projection/API，不复制 Web 逻辑。

No-Go：不扩展大规模 Bevy 内容开发；先补 Android 真实设备矩阵、资源包、崩溃和输入证据。

### S6 — First Beta / Commercial Evidence

当前 blocker：

- `TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH` 真实 5-10 人 cohort，证据 JSON 必须声明 `status=first_beta_cohort_evidence_green`。
- `TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH` payment/refund/support/legal/operator/traffic drill，证据 JSON 必须声明 `status=commercial_launch_drill_evidence_green`。
- `scripts/check_trillionnium_world_cohort_commercial_schema.sh` 生成 cohort/commercial JSON schema 与模板，并写入 `acceptance/S6_public_launch/latest/cohort-commercial-evidence-schema.json`；模板不能声明 green，只能为真实证据采集提供标准格式。
- `scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh` 生成 `acceptance/S6_public_launch/latest/cohort-commercial-evidence-collection.json` 与 `.md`，列出真实 beta 参与者/session/feedback/signoff 与 payment/refund/support/legal/operator/traffic drill 采集项；它只生成 checklist，不保存个人原始隐私数据，也不授予 public-launch credit。
- `TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH` 或 live traffic latency。
- `scripts/check_trillionnium_world_external_ops_evidence_collection.sh` 生成 `acceptance/S6_public_launch/latest/external-ops-evidence-collection.json` 与 `.md`，把真实 multi-node/live-traffic latency、公网 domain/TLS/probe/monitoring/backup/rollback/signoff 证据列成采集清单；它只生成 checklist，不打开公网 route，也不给 public-launch credit。
- `TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH` 指向真实生产 map-pack 公共证据；证据 JSON 必须声明 `status=production_map_pack_public_ready_green`，并包含来源授权/ODbL、缓存策略、归因截图、敏感 POI、地理围栏、密钥托管、分发撤回、回滚和 operator signoff。
- `scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh` 生成 `acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence-collection.json` 与 `.md`，把真实 source/ODbL/cache/attribution/sensitive POI/geofence/key custody/distribution/rollback/signoff 证据项列成采集清单；它只生成 checklist，不执行 live ingestion。
- `scripts/check_trillionnium_world_production_map_pack_public_evidence.sh` 生成 `acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence.json`；默认只消费外部证据文件，不做 live Overpass/Geofabrik ingestion，也不允许运行时客户端直连公网 OSM。
- `scripts/check_trillionnium_world_public_launch_readiness.sh` 汇总 Native/Bevy keyboard replay、action coach、player HUD/debug layer、player UI rescue、live-window screenshot sequence、sprite texture sampling、sampled texture live-window correlation、render asset eligibility、S5 真机、S4 map_pack、cohort、商业演练、多节点/公网部署证据；缺证据时只能输出 blocked，不能宣称全网上线 ready。
- `scripts/check_trillionnium_world_release_signoff_summary.sh` 生成 `acceptance/S6_public_launch/latest/release-signoff-summary.json`，给 CI/人工评审一个直接入口：它必须看到 Native/Bevy keyboard replay、action coach、player HUD/debug layer、player UI rescue、live-window screenshot sequence、sprite texture sampling、sampled texture live-window correlation、render asset eligibility 与 CEX adapter readiness green，且 public-launch readiness 已消费这些本地证据，同时继续标明 S5 真机、公网、cohort、商业演练等外部证据 blocker。release packet integrity 现在以 `release_signoff_summary_semantics` 直接绑定这些 local Bevy/CEX/public-launch consumption gates、6 个 blocker 和 Android S5/public-launch no-claim 边界，防止 signoff summary 只靠 status/checksum 假绿；同时 `cex_adapter_readiness_semantics` 会直接校验 CEX adapter artifact 的 protocol/domain contract、6 个 adapter role、route/world count、repository/ledger/metric source 和 no-import boundary。
- `scripts/check_trillionnium_world_release_review_quickcheck.sh` 是上层快速入口：一条命令刷新 public-launch readiness 与 release signoff summary，再写入 `acceptance/S6_public_launch/latest/release-review-quickcheck.json`。默认只把 Native/Bevy replay、texture/render local playability、CEX adapter readiness 或消费链路断裂视为失败，真实外部上线证据继续作为 blocker；传 `--require-ready` 时 public launch 未 ready 也会失败。release packet integrity 现在以 `release_review_quickcheck_semantics` 直接绑定 quickcheck 的 public-launch/signoff refresh、local Bevy texture/render/CEX consumption gates、6 个 blocker 和 Android S5/public-launch no-claim 边界，防止上层 quickcheck 只靠 status/checksum 假绿。
- `scripts/check_trillionnium_world_release_review_status.sh` 生成 `acceptance/S6_public_launch/latest/release-review-status.json` 与 `.md`，把 release review 已绿项和仍需真实外部证据的 blocker 压成一屏清单。release packet integrity 现在以 `release_review_status_semantics` 直接绑定 quickcheck/signoff handoff、13 个 ready review items、6 个 external blockers 和 Android S5/public-launch no-claim 边界，防止 status checklist 只靠路径、checksum 或顶层状态假绿。
- `scripts/check_trillionnium_world_public_launch_evidence_intake.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-evidence-intake.json` 与 `.md`，把 S5 真机、production map-pack、首批 beta、商业 drill、多节点/真实流量延迟、公网部署这 6 个外部证据项压成可收集清单、采集命令和 env hook；它不做 live map ingestion，也不做公网暴露。
- `scripts/check_trillionnium_world_public_launch_evidence_kit.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-evidence-kit.json` 与 `.md`，刷新并列出 6 个 blocker 的 no-credit evidence templates、env hook 和 validator command；模板不能作为 public-launch credit。
- `scripts/check_trillionnium_world_public_launch_operator_handoff.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-operator-handoff.json` 与 `.md`，把 6 个 blocker 的采集 action、template、validator command、bundle template、负向 fixtures 与 sha256 汇总成交接包；它只服务真实证据采集，不授予 public-launch credit。
- `scripts/check_trillionnium_world_public_launch_template_negative_fixtures.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-template-negative-fixtures.json`，把 no-credit templates 直接喂给严格字段级 validators，要求 S5/map-pack/cohort+commercial/external-ops 全部失败，防止模板误清 blocker。
- `scripts/check_trillionnium_world_public_launch_evidence_bundle.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-evidence-bundle.json` 与 `.md`，支持 `TRILLIONNIUM_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_PATH` 指向单个真实证据 manifest；它会把 manifest 内的 6 个 evidence path 分发给字段级 validators，全部 green 且 operator signoff 后才给 public-launch credit。
- `scripts/check_trillionnium_world_public_launch_bundle_negative_fixtures.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-bundle-negative-fixtures.json`，构造一个 status/signoff 看似 green 但 evidence path 指向 templates 的 bundle manifest，并要求 bundle gate 在 `--require-ready` 下拒绝它。
- `scripts/check_trillionnium_world_public_launch_blocker_consistency.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-blocker-consistency.json`，校验 public-launch readiness blocker、evidence intake item 与各字段级 validator status 三方一致；release packet integrity 还会以 `public_launch_blocker_consistency_semantics` 直接绑定 6 个 blocker 的 blocked validator 状态，任何未知 blocker、漂移或 status-only 假绿都会挡住 release packet/CI handoff。
- `scripts/check_trillionnium_world_cohort_commercial_evidence.sh` 生成 `acceptance/S6_public_launch/latest/cohort-commercial-evidence.json`，对 `TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH` 与 `TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH` 指向的真实证据做字段级验证，避免只写 green status 就被 public-launch readiness 接受。
- `scripts/check_trillionnium_world_external_ops_evidence.sh` 生成 `acceptance/S6_public_launch/latest/external-ops-evidence.json`，对 `TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH` 与 `TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH` 指向的真实证据做字段级验证；本地 latency/deploy drill 会被记录但不能作为 public launch credit。
- `scripts/check_trillionnium_world_s5_device_evidence.sh --require-device` 是 S5 Android 真机采集入口：ADB 看到在线设备后安装 signed debug APK，采集 screenshot、gfxinfo/frame、logcat、lifecycle、locale/input-method、weak-network、APK resource/signature 和 crash-free window 证据，并写入 `acceptance/S5_native_bevy_device/latest/s5-device-evidence.json`。弱网必须通过 `TRILLIONNIUM_WORLD_S5_WEAK_NETWORK_EVIDENCE_PATH` 附上真实弱网实跑证据；普通 connectivity snapshot 不能拿 launch credit。
- `scripts/check_trillionnium_world_s5_real_device_evidence.sh` 生成 `acceptance/S5_native_bevy_device/latest/s5-real-device-evidence-validation.json`，对 S5 真机 evidence 做字段级验证；必须有真机 screenshot、gfxinfo/frame、logcat、lifecycle、CJK/输入、弱网、资源包/签名、crash-free 和设备序列证据，host-side replay 不能作为真机 credit。
- `scripts/check_trillionnium_world_public_launch_status_only_fixtures.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-status-only-fixtures.json`，用 status-only 假绿证据撞 S5、production map-pack、首批 beta/商业 drill、外部 ops validators，确保所有字段级门禁拒绝伪绿证据。
- `scripts/check_trillionnium_world_release_review_convergence.sh` 生成 `acceptance/S6_public_launch/latest/release-review-convergence.json`，先刷新 status/quickcheck/signoff/public-launch，再确认 release review 的脚本、README/开发文档、workflow guard、JSON/Markdown evidence 没有断链。release packet integrity 现在以 `release_review_convergence_semantics` 直接绑定 status refresh、docs/workflow/script guards、本地 Bevy/CEX 证据链、status Markdown boundary 和 Android S5/public-launch no-claim 边界，防止 convergence 只靠 contract/status/checksum 假绿。
- `scripts/check_trillionnium_world_release_review_packet.sh` 生成 `acceptance/S6_public_launch/latest/release-review-packet.json` 与 `.md`，刷新 convergence 后汇总关键 evidence 路径、contract/status 与 sha256，作为 release review handoff 包。
- `scripts/check_trillionnium_world_release_review_packet_integrity.sh` 生成 `acceptance/S6_public_launch/latest/release-review-packet-integrity.json`，默认先刷新 packet，再重新计算 artifact sha256/bytes/contract/status，防止 handoff 包和底层 evidence 漂移；`--no-refresh` 可只校验现有 packet。
- `scripts/check_trillionnium_world_release_review_ci_gate.sh` 生成 `acceptance/S6_public_launch/latest/release-review-ci-gate.json`，聚合 packet integrity、release-review 静态 guards、README 链接和 workflow script refs，作为本地 CI/评审交接总入口。
- `scripts/check_trillionnium_world_ui_map_modeling_full_alignment.sh` 生成 `acceptance/S6_public_launch/latest/trnm-world-ui-map-modeling-full-alignment.json` 与 `.md`，直接回答 UI 设计、地图引擎、建模设计是否全量对齐：host-side 对齐可以为 green，但 production map-pack、S5 真机、public launch 外部证据不齐时 `full_alignment_green` 必须保持 false；`--require-ready` 严格模式会在这些 blocker 未清时失败。
- `scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-desktop-real-machine-readiness.json` 与 `.md`，作为移动端之前的桌面真机/本机窗口总闸门；它只证明 local Linux desktop X11 window 与 release runner，不授予 Android S5 credit。
- `scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh` 生成 `acceptance/S6_public_launch/latest/release-review-checkpoint-manifest.json` 与 `.md`，按 CEX adapter readiness、release-review surface、外部证据 validators、Native/Bevy、本地 map/repository boundary、Rust/frontend code surface、dev infra、generated acceptance evidence 等切片整理当前 dirty working tree；它只做审查/提交前清单，不 stage、不 commit，也不宣称 public-launch evidence。
- `scripts/check_trillionnium_world_public_deploy_readiness.sh` 生成 release binary、systemd/env/reverse-proxy/runbook 和本机 deploy drill evidence；它不做真实公网暴露，真实公网 ready 仍需要目标主机、域名/TLS、监控、备份、回滚和公网 URL probe。
- `scripts/check_trillionnium_world_release_rollback_backup_drill.sh` 对 release server 执行本机状态备份、坏状态覆盖、备份恢复和恢复后读回，写入 `acceptance/S6_public_launch/latest/release-rollback-backup-drill.json`；它是本机运维演练，不替代托管数据库/异地备份。
- `scripts/check_trillionnium_world_release_latency_drill.sh` 对 release `trnm-world-server` 做本机并发延迟 drill，并把结果写入 `acceptance/S6_public_launch/latest/release-latency-drill.json`；它只能证明 local release latency，不替代多节点或真实公网流量证据。

没有这些证据，不提升 first-beta/commercial/technical 最高档评分。

### S7 — Chain/Ledger/Settlement Integration

目标：将 CEX ledger/world economy 与 Trillionnium 主链/结算能力对齐。

任务：

- World economy command 与 chain task/settlement interface 的桥接 ADR。
- 幂等键范围：`(command_kind, actor_scope, idempotency_key)` 或服务端签发。
- Review hold、refund、chargeback、recovery worker 进入链上/链下一致性模型。
- 保留 CEX ledger fail-closed 语义。

---

## 8. 统一验收证据制度

CEX 当前使用 `run/*`，Dropbox v0.4 使用 `acceptance/*`。统一后：

```text
acceptance/
  S0_cex_baseline/
    git_head.txt
    cargo_test.log
    clippy.log
    runtime_status.json
    sql_snapshot.json
    web_e2e_summary.json
    browser_e2e_summary.json
    first_human_e2e_summary.json
    owner_signoff.md
  S1_rust_owned_world_ui/
    fragment_contract_matrix.md
    browser_bridge_audit.md
    e2e_post_move_delta_304_report.md
    owner_signoff.md
  S4_map_pack_gate/
    map_data_decision_record.md
    license_review.md
    attribution_screenshots/
    sensitive_poi_filter_report.md
    geofence_test_report.md
    map_pack_manifest_signed.json
    owner_signoff.md
  S6_beta_commercial/
    first_beta_cohort_evidence.json
    commercial_launch_drill_evidence.json
    multi_node_latency_report.json
    owner_signoff.md
```

最低字段：

| 字段 | 要求 |
| --- | --- |
| `gate_name` | S0/S1/S2/S4/S6 等 |
| `git_head` | commit hash |
| `build_id` | Rust binary / frontend / config version |
| `protocol_contracts` | contract version list |
| `test_commands` | 可复跑命令 |
| `evidence_paths` | `run/*` 或 `acceptance/*` 路径 |
| `known_failures` | 已知失败与分类 |
| `rollback_plan` | 回滚/停损动作 |
| `owner` / `reviewer` | 责任人与验收人 |
| `signoff_time` | 签收时间 |

规则：没有证据包，不进入下一阶段；豁免项必须有过期时间。

---

## 9. Go / No-Go 闸门

| Gate | 当前状态 | Go 条件 | No-Go 动作 |
| --- | --- | --- | --- |
| CEX green baseline | Green | S0 命令全绿 | 停止迁移，先修 baseline |
| Rust-owned `/world` UI | In progress / strong green slices | 核心 `/world` 无 JS-owned state/UI | 继续 fragment migration，不做新功能膨胀 |
| Ledger economy | Green but must preserve | 所有 commerce/recovery fail-closed | 关闭市场/交易/GM 发放 |
| Map provider | Fixture green | 合规数据源、缓存、署名、下架、POI filter | 继续 fixture-only，不开 live ingestion |
| Map renderer | Leaflet live, MapLibre shadow only | fresh signoff + rollback drill | 不提升 MapLibre canary |
| First beta | Blocked on real cohort | 真实 5-10 人 evidence | 不宣称 beta 9+ |
| Commercial | Blocked on drill evidence | payment/refund/support/legal/operator/traffic drill | 不宣称 commercial 8+ |
| Native/Bevy | Native client shell green / device matrix pending | host + aarch64 Android compile green，真实 mobile gate 通过 | 不扩展大规模 Bevy 内容开发，先补设备矩阵证据 |
| MMO Cell/AOI | Future | multi-node/load/chaos 证据 | 不宣称实时 MMO readiness |

---

## 10. UI / 客户端策略

### 10.1 当前 Web UI 策略

- `/world` 是 game-first shell，不是 dashboard-first。
- Rust 生成 player-visible fragments。
- Browser JS 只做：
  - click/keyboard handler
  - focus selection
  - fetch map viewport/delta
  - bind map marker/popups
  - apply Rust-projected HTML
  - fallback for `/app` or degraded payloads
- 每次 UI 迁移都要加：contract constant、bootstrap payload、viewport/delta payload、DOM ownership attributes、unit test、Web/Browser E2E assertion。

### 10.2 当前 Native/Bevy 策略

Bevy 是实验 Gate，不是权威 runtime。当前已启动的事实是 Native Bevy client shell；继续推进时：

- 不复刻 Web 的 JS 逻辑。
- 直接消费 Rust World API/projection。
- Native client 只提交 intent。
- `scripts/check_trillionnium_world_s5_device_evidence.sh` 负责产出 `acceptance/S5_native_bevy_device/latest/s5-device-evidence.json`；公网发布 S5 credit 必须使用 `--require-device` 在真实 Android 设备上采集，再跑 `scripts/check_trillionnium_world_s5_real_device_evidence.sh --require-ready`。
- 地图包和配置包必须签名。
- Android release chain 要单独 ADR。

---

## 11. 地图与合规策略

当前：

- Active engine：`leaflet_openstreetmap_v1`
- Renderer adapter：`leaflet_renderer_adapter_v1`
- Runtime handle：`mapRuntime`
- Future candidate：`maplibre_gl_v1`
- Status：`shadow_only_not_user_facing`，canary=0
- OSM provider：fixture only
- Live Overpass / Geofabrik：disabled
- Public production OSM tile：forbidden until cache/vendor/self-host policy

未来真实地图 Gate 必须覆盖：

- 数据授权矩阵。
- ODbL attribution 和 derived database obligation。
- 缓存/离线授权。
- 敏感 POI filter。
- 地理围栏与地区政策。
- map_pack signing / revocation / rollback。
- Attribution screenshots。

---

## 12. 经济、结算与安全

CEX 已落地的强原则必须继承：

- 经济命令必须 idempotent。
- Ledger DB unavailable 时 fail-closed。
- Buyer reserve / consume / refund 与 seller settlement / chargeback 必须一致。
- Review hold 不允许刷 progression / standing。
- Completion reward / League reward 必须 ledger release 后推进。
- Failed chargeback/refund 必须可恢复，不得把世界状态误标为完成。
- In-memory fallback 只能用于明确 local-dev；production/local-production 必须 fail-fast。

未来增强：

- Command ledger table 明确 `pending/applying/applied/failed/recoverable/dead_letter`。
- Recovery worker 有 owner、lock、retry、dead letter。
- 多资产锁顺序固定，deadlock retry 有测试。
- GM 发放、价格、商店、掉落、配置包都需要审批和审计。

---

## 13. 测试与运行门禁

### 13.1 当前最小开发门禁

```bash
cargo fmt --all -- --check
cargo check -p consumer-entry-api
cargo test -p consumer-entry-api -- --nocapture --test-threads=1
cargo clippy -p consumer-entry-api -- -D warnings
node --check scripts/playwright/trillionnium-browser-e2e.mjs
bash -n scripts/check-trillionnium-league-web-e2e.sh \
  scripts/check-trillionnium-league-browser-e2e.sh \
  scripts/check-trillionnium-first-human-session.sh
git diff --check
```

### 13.2 Runtime/E2E 门禁

```bash
export CEX_ENV_FILE=run/local-production/.env
scripts/runtime-manager-linux.sh restart
scripts/runtime-manager-linux.sh status
scripts/check-trillionnium-league-sql-snapshot.sh
scripts/check-trillionnium-league-web-e2e.sh
scripts/check-trillionnium-league-browser-e2e.sh
scripts/check-trillionnium-first-human-session.sh
```

### 13.3 Signoff / maturity 门禁

根据改动范围追加：

- `scripts/check-trillionnium-ui-audit.sh`
- `scripts/check-trillionnium-playability-scorecard.sh`
- `scripts/check-trillionnium-world-real-user-beta.sh`
- `scripts/check-trillionnium-world-public-commercial-product.sh`
- `scripts/check-production-readiness.sh`
- `scripts/check-production-signoff.sh`
- Health/metrics load soak。

---

## 14. Backlog 映射

### Epic A — Rust-owned `/world` UI 完结

- A1 审计残留 `innerHTML` / browser card builders。
- A2 迁移 commerce panels。
- A3 迁移 timeline / secondary dashboard。
- A4 lazy hydrate heavy payload。
- A5 Browser E2E post-delta / weak-network / 304 coverage。

### Epic B — WorldState 模块化

- B1 分 sub-state。
- B2 系统化 command handlers。
- B3 indexes/read model 标准化。
- B4 JSON/SQL compatibility gates。

### Epic C — Trillionnium 主仓承接

- C1 同步本文档。
- C2 建 World API index。
- C3 建 World evidence index。
- C4 写 CEX incubator handoff ADR。

### Epic D — Map compliance / map_pack

- D1 地图数据 ADR。
- D2 map_pack manifest/signature。
- D3 attribution screenshots。
- D4 sensitive POI / geofence / takedown runbook。

### Epic E — Real beta evidence

- E1 5-10 人 cohort evidence。
- E2 commercial launch drill evidence。
- E3 multi-node/live latency evidence。
- E4 update human-playability assessment。

### Epic F — Native/mobile experiment

- F1 Bevy/Godot/PWA comparison ADR。
- F2 Android device matrix。
- F3 intent protocol prototype。
- F4 package/resource/signature gate。

---

- CEX runtime plugin split: `docs/development/trillionnium-cex-runtime-plugin-split-v1.md` (`trillionnium_cex_runtime_plugin_v1`)
- Term Exchange Kernel: `docs/development/trillionnium-term-exchange-kernel-v1.md` (`term_exchange_protocol_v1`, `trillionnium_term_exchange_kernel_v1`)
- Term Exchange backend adapter: `trillionnium_term_exchange_backend_adapter_v1`; first migrated paths are League reward settlement, World commerce settlement lifecycle, and World contract completion settlement.
- Typed receipt state: `TermExchangeReceiptState` persists adapter receipts into `LeagueState.term_exchange_receipts` and `WorldState.world_term_exchange_receipts` while legacy status fields remain compatible.
- Normalized receipt persistence/projection: migration `0026_add_term_exchange_receipt_tables.sql` adds `league_term_exchange_receipts` and `world_term_exchange_receipts`; repository snapshot SQL shadows typed receipt `status`/`progression_class`, and `upsert_normalized_term_exchange_receipt_tables` direct-writes both receipt tables during normalized final-cutover writes. World-home, client-feed, `/v1/client/app/:matrix_user_id` embedded feed, `/app` bootstrap shell, and JSON command-response `home` projections expose typed world receipt counts, progression-class groups, latest receipt metadata, and receipt feed items; normalized read models carry the same `trillionnium_term_exchange_receipt_projection_v1` object plus client-feed `term_exchange_receipts` snapshots, and the runtime endpoints now hydrate those receipt slices from normalized SQL when the read switch is active so final cutover can move one surface at a time. Runtime commerce/recovery gates now prefer typed receipts for reserve/settle/consume/refund/chargeback/reopen/contract-completion progression and health/playability counts, with legacy status strings retained only as compatibility fallbacks. Source-of-truth gates now explicitly require the client-app embedded-feed overlay and its startup validation alongside world-home and client-feed SQL read models.

## 15. 当前下一步

如果下一条指令是“继续”，优先做：

1. Term Exchange 方向：继续保持 legacy status API 兼容，同时把 normalized receipt read-model probes 推向 read-switch/final projection cutover。
2. World UI 方向：继续审计 `/world` 残留 browser-built secondary/dashboard/commerce/timeline UI，把剩余面板改成 Rust-owned fragment + lazy hydration。
3. 不新增 live OSM、不提升 MapLibre、不引入 Hero Tan copy workflow。
4. 若目标是评分提升，而不是 UI/player-value，则先补真实 evidence path：
   - `TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH`
   - `TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH`
   - `TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH`
5. 同步本文档到 Trillionnium 主仓 `docs/development/`，并在主仓 README/docs index 中标注 CEX incubator source。

---

## 16. Source Documents

本文档合并自：

- Dropbox：`/home/qian/Dropbox/rust_geo_mmo_development_doc_v0_4_execution_ready.md`
- CEX：`docs/trillionnium-world-development-progress-tree-v1.md`
- CEX：`docs/trillionnium-world-ecs-refactor-v1.md`
- CEX：`docs/trillionnium-world-map-optimization-execution-v1.md`
- CEX：`docs/trillionnium-real-world-map-engine-evaluation-v1.md`
- CEX：`docs/trillionnium-open-source-stack-reference-v1.md`
- CEX：`docs/trillionnium-open-source-hero-tan-shuo-base-selection-v1.md`
- CEX：`docs/trillionnium-open-source-tactics-base-selection-v1.md`
- CEX：`docs/trillionnium-league-implementation-plan-v1.md`
- CEX runtime evidence under `run/*`

---

## 17. 最终判断

Trillionnium World 当前已经有一个强于普通 MVP 文档的基础：CEX 里存在可运行、可测试、可审计、可恢复的 Rust-first 世界垂直切片。接下来不要推倒重来，也不要只写愿景。正确路径是：

1. 以 CEX green baseline 为事实。
2. 把 `/world` UI ownership 收口到 Rust。
3. 把 World domain 从 CEX 中模块化。
4. 把文档、API、证据迁移到 Trillionnium 主仓。
5. 再进入真实地图包、移动客户端、Cell/AOI、Alpha/商业化证据阶段。

这条路线同时保留 Dropbox v0.4 的工程纪律，也尊重 CEX 已经落地的代码现实。
