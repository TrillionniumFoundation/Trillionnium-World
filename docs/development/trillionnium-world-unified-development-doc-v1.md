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
- 当前 Trillionnium 主仓新增 `scripts/check_trillionnium_world_map_modeling_gate.sh`，在 fixture map_pack 上派生 buildings / roads / greenery / terrain 四类模型并写入 `acceptance/S4_map_pack_gate/latest/map-modeling-gate.json`；它证明建模管线存在，但仍明确 `fixture_only=true`、`live_ingestion_enabled=false`、`runtime_clients_fetch_public_osm_directly=false`。

### S5 — Native/Bevy Mobile Gate（当前引擎路径）

目标：把 Trillionnium World 接入 Native/Bevy 游戏引擎开发路径，同时保持 Rust server/API/projection 作为权威。

当前 Trillionnium 主仓要求 Native/Bevy client shell 直接进入开发路径：Bevy 只消费 Rust World API/projection，并把玩家输入作为 intent 交回 Rust command layer。当前已能构建 aarch64 Android `cdylib`，导出 `ANativeActivity_onCreate` / `android_main`，并在 Android platform jar 可用时生成已签名 debug APK；Android 真机矩阵和真实渲染/输入/资源包验证继续作为 S5 gate 收证据。

当前 Native/Bevy host 端追加 `scripts/check_trillionnium_world_bevy_vertical_slice.sh`、`scripts/check_trillionnium_world_bevy_first_playable.sh` 与 `scripts/check_trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay.sh`。前两者覆盖当前房间/出口、目标任务、触控按钮、训练/战斗/任务动作、Rust intent-only authority、开局指引和保存读回；keyboard replay gate 生成 `acceptance/S5_native_bevy_device/latest/bevy-build-branch-title-route-all-branch-keyboard-replay.json`，把 force/agility/craft 三条 build-title route 的键盘输入序列在 fresh Bevy runtime 中重放，并要求事件签名与最终 runtime 完全一致。

2026-05-17 的 S5 host-side 可玩性证据又补了三层：`scripts/check_trillionnium_world_bevy_action_coach.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-action-coach.json`，要求 `Enter/NumpadEnter` action coach 依次引导 `TALK -> TRAIN -> MOVE:north -> FIGHT`；`scripts/check_trillionnium_world_bevy_player_hud_debug_layer.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-player-hud-debug-layer.json`，把玩家 HUD 与 DEBUG/INPUT diagnostics 分层；`scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-live-window-screenshot-sequence.json`，验证 11 帧 live window 截图、frame sequence 和 contact sheet 非空变化。这些都是 host 端 Native/Bevy 可玩闭环、输入协议和玩家可读性的本地证据，不替代 Android 真机 S5 evidence。

2026-05-19 的 low-spec classic renderer 继续按 RTS 标准补齐本地证据：`scripts/check_trillionnium_world_bevy_classic_rts_control_loop.sh` 生成 `acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.json` 与 `.ppm`，要求控制组 1 同时覆盖多单位选择、移动目的地、编队线、攻击目标、命令 marker、攻击反馈、小地图、资源条、命令面板、战争迷雾/可视范围、生产队列、建筑队列、训练/建造进度、资源消耗反馈、单位血量卡、目标血量、技能/命令卡和冷却反馈像素。随后 `scripts/check_trillionnium_world_bevy_classic_rts_live_input_sequence.sh` 把 `RTS:SELECT -> RTS:QUEUE -> RTS:MOVE -> RTS:ATTACK -> RTS:ABILITY` 接到 `apply_live_native_action_with_source(classic_rts_live_input)`，并生成 `acceptance/S5_native_bevy_device/latest/bevy-classic-rts-live-input-sequence.json` 与 `.ppm`，要求五段真实 live input 全部 accepted，最终 command queue / production queue / attack target / active ability / target health 都由 native runtime 状态证明。`scripts/check_trillionnium_world_bevy_classic_rts_pathing_formation.sh` 继续把 live move 输入推进到路径和编队反馈：`RTS:MOVE:8,4:wedge` 必须生成可见 path tiles、blocked tile detour、formation slots、command marker 与 selection rings。`scripts/check_trillionnium_world_bevy_classic_rts_collision_engagement.sh` 再要求 blocked-detour 后的单位分散格、攻击接敌范围、contact flash 和 attack feedback 都由 native runtime 状态驱动画出来。`scripts/check_trillionnium_world_bevy_classic_rts_target_aggro_focus.sh` 继续把 `RTS:ATTACK:arena_creep_attack` 与 `RTS:ABILITY:focus_fire` 推进到目标优先级、aggro 锁定、集火单位队列、威胁条和对应 overlay 像素。`scripts/check_trillionnium_world_bevy_classic_rts_economy_build.sh` 再把 `RTS:QUEUE:harvest:gold_vein`、`RTS:QUEUE:build:watch_tower@7,4` 和 `RTS:QUEUE:train:worker` 推进到资源采集、工人路线、返仓、建造蓝图、建造进度和生产队列证据。`scripts/check_trillionnium_world_bevy_classic_rts_selection_minimap.sh` 继续把 `RTS:SELECT:box:frontline`、`RTS:MOVE:minimap:9,2:rally`、`RTS:SELECT:2` 和 `RTS:MOVE:6,5:split` 推进到框选 tile、两组控制组绑定、小地图 rally、分路命令状态与对应 overlay 像素。`scripts/check_trillionnium_world_bevy_classic_rts_build_lifecycle.sh` 再把 `RTS:QUEUE:build`、`complete`、`repair` 和 `cancel` 推进到建筑放置、完成、维修、取消退款、结构血量和对应 overlay 像素。`scripts/check_trillionnium_world_bevy_classic_rts_tech_tree.sh` 继续把 `RTS:QUEUE:faction:mirror_guard`、`research:wayfinder_code@town_hall`、`upgrade:iron_lacing@training_hall` 和 `unlock:relay_guard` 推进到阵营基地、科技依赖、研究进度、升级完成、单位/建筑解锁和对应 overlay 像素。九组 gate 均已纳入 `scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh` 与 release-review CI，用来防止只堆静态美术而没有 RTS 可操作反馈；它们仍是 host-side Bevy classic 证据，不声明 Android 真机或公网发布 ready。

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
- `scripts/check_trillionnium_world_public_launch_readiness.sh` 汇总 Native/Bevy keyboard replay、action coach、player HUD/debug layer、live-window screenshot sequence、sprite texture sampling、sampled texture live-window correlation、render asset eligibility、S5 真机、S4 map_pack、cohort、商业演练、多节点/公网部署证据；缺证据时只能输出 blocked，不能宣称全网上线 ready。
- `scripts/check_trillionnium_world_release_signoff_summary.sh` 生成 `acceptance/S6_public_launch/latest/release-signoff-summary.json`，给 CI/人工评审一个直接入口：它必须看到 Native/Bevy keyboard replay、action coach、player HUD/debug layer、live-window screenshot sequence、sprite texture sampling、sampled texture live-window correlation、render asset eligibility 与 CEX adapter readiness green，且 public-launch readiness 已消费这些本地证据，同时继续标明 S5 真机、公网、cohort、商业演练等外部证据 blocker。
- `scripts/check_trillionnium_world_release_review_quickcheck.sh` 是上层快速入口：一条命令刷新 public-launch readiness 与 release signoff summary，再写入 `acceptance/S6_public_launch/latest/release-review-quickcheck.json`。默认只把 Native/Bevy replay、texture/render local playability、CEX adapter readiness 或消费链路断裂视为失败，真实外部上线证据继续作为 blocker；传 `--require-ready` 时 public launch 未 ready 也会失败。
- `scripts/check_trillionnium_world_release_review_status.sh` 生成 `acceptance/S6_public_launch/latest/release-review-status.json` 与 `.md`，把 release review 已绿项和仍需真实外部证据的 blocker 压成一屏清单。
- `scripts/check_trillionnium_world_public_launch_evidence_intake.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-evidence-intake.json` 与 `.md`，把 S5 真机、production map-pack、首批 beta、商业 drill、多节点/真实流量延迟、公网部署这 6 个外部证据项压成可收集清单、采集命令和 env hook；它不做 live map ingestion，也不做公网暴露。
- `scripts/check_trillionnium_world_public_launch_evidence_kit.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-evidence-kit.json` 与 `.md`，刷新并列出 6 个 blocker 的 no-credit evidence templates、env hook 和 validator command；模板不能作为 public-launch credit。
- `scripts/check_trillionnium_world_public_launch_operator_handoff.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-operator-handoff.json` 与 `.md`，把 6 个 blocker 的采集 action、template、validator command、bundle template、负向 fixtures 与 sha256 汇总成交接包；它只服务真实证据采集，不授予 public-launch credit。
- `scripts/check_trillionnium_world_public_launch_template_negative_fixtures.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-template-negative-fixtures.json`，把 no-credit templates 直接喂给严格字段级 validators，要求 S5/map-pack/cohort+commercial/external-ops 全部失败，防止模板误清 blocker。
- `scripts/check_trillionnium_world_public_launch_evidence_bundle.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-evidence-bundle.json` 与 `.md`，支持 `TRILLIONNIUM_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_PATH` 指向单个真实证据 manifest；它会把 manifest 内的 6 个 evidence path 分发给字段级 validators，全部 green 且 operator signoff 后才给 public-launch credit。
- `scripts/check_trillionnium_world_public_launch_bundle_negative_fixtures.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-bundle-negative-fixtures.json`，构造一个 status/signoff 看似 green 但 evidence path 指向 templates 的 bundle manifest，并要求 bundle gate 在 `--require-ready` 下拒绝它。
- `scripts/check_trillionnium_world_public_launch_blocker_consistency.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-blocker-consistency.json`，校验 public-launch readiness blocker、evidence intake item 与各字段级 validator status 三方一致；任何未知 blocker 或漂移都会挡住 release packet/CI handoff。
- `scripts/check_trillionnium_world_cohort_commercial_evidence.sh` 生成 `acceptance/S6_public_launch/latest/cohort-commercial-evidence.json`，对 `TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH` 与 `TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH` 指向的真实证据做字段级验证，避免只写 green status 就被 public-launch readiness 接受。
- `scripts/check_trillionnium_world_external_ops_evidence.sh` 生成 `acceptance/S6_public_launch/latest/external-ops-evidence.json`，对 `TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH` 与 `TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH` 指向的真实证据做字段级验证；本地 latency/deploy drill 会被记录但不能作为 public launch credit。
- `scripts/check_trillionnium_world_s5_device_evidence.sh --require-device` 是 S5 Android 真机采集入口：ADB 看到在线设备后安装 signed debug APK，采集 screenshot、gfxinfo/frame、logcat、lifecycle、crash-free window，并写入 `acceptance/S5_native_bevy_device/latest/s5-device-evidence.json`。
- `scripts/check_trillionnium_world_s5_real_device_evidence.sh` 生成 `acceptance/S5_native_bevy_device/latest/s5-real-device-evidence-validation.json`，对 S5 真机 evidence 做字段级验证；必须有真机 screenshot、gfxinfo/frame、logcat、lifecycle、crash-free 和设备序列证据，host-side replay 不能作为真机 credit。
- `scripts/check_trillionnium_world_public_launch_status_only_fixtures.sh` 生成 `acceptance/S6_public_launch/latest/public-launch-status-only-fixtures.json`，用 status-only 假绿证据撞 S5、production map-pack、首批 beta/商业 drill、外部 ops validators，确保所有字段级门禁拒绝伪绿证据。
- `scripts/check_trillionnium_world_release_review_convergence.sh` 生成 `acceptance/S6_public_launch/latest/release-review-convergence.json`，先刷新 status/quickcheck/signoff/public-launch，再确认 release review 的脚本、README/开发文档、workflow guard、JSON/Markdown evidence 没有断链。
- `scripts/check_trillionnium_world_release_review_packet.sh` 生成 `acceptance/S6_public_launch/latest/release-review-packet.json` 与 `.md`，刷新 convergence 后汇总关键 evidence 路径、contract/status 与 sha256，作为 release review handoff 包。
- `scripts/check_trillionnium_world_release_review_packet_integrity.sh` 生成 `acceptance/S6_public_launch/latest/release-review-packet-integrity.json`，默认先刷新 packet，再重新计算 artifact sha256/bytes/contract/status，防止 handoff 包和底层 evidence 漂移；`--no-refresh` 可只校验现有 packet。
- `scripts/check_trillionnium_world_release_review_ci_gate.sh` 生成 `acceptance/S6_public_launch/latest/release-review-ci-gate.json`，聚合 packet integrity、release-review 静态 guards、README 链接和 workflow script refs，作为本地 CI/评审交接总入口。
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
