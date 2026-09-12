# `web4-frontend`

Lifecycle in this repository: **historical-compatible subproject**  
World game-product release denominator: **none**

This retained Next.js 16 + React 19 application is not the native Trillionnium World player client and is not part of the active eight-crate World release denominator. Its source may be used for bounded compatibility, historical dashboard and migration work only when an explicit plan selects it. `docs/component-catalog.json` is the machine lifecycle authority.

A green Web4 preflight means only that this subproject passes its own frontend checks. It does not prove World/Nakama/CEX/Chain integration, write capability, deployment, public exposure or repository-wide release readiness.

> 文档统一入口：**[docs/README.md](./docs/README.md)**  
> 当前保留语义：默认使用 readonly API client；仅在显式 `?mode=mock` 时回退到本地 mock snapshot；不提供写路径。

## 快速开始（仅限明确选择的兼容/历史工作）

```bash
npm ci
npm run dev
```

打开 <http://localhost:3000>

## 文档入口与判读边界

优先按下面顺序查阅：

- [`../PROJECT_BOUNDARY.md`](../PROJECT_BOUNDARY.md)：World 仓库及跨系统 authority 边界；
- [`../docs/component-catalog.json`](../docs/component-catalog.json)：本子项目的当前 lifecycle/release-denominator 分类；
- [`docs/README.md`](./docs/README.md)：前端开发、API 合约、测试和运维文档；
- [`docs/developer-guide.md`](./docs/developer-guide.md)：本地启动、环境变量和提交流程；
- [`docs/operations-runbook.md`](./docs/operations-runbook.md)：子项目发布、回滚和排障；
- [`../RELEASE_READINESS.md`](../RELEASE_READINESS.md)：仓库级 release truth source。

若当前 checkout 含有 `../docs/reports/TRNM_WEB4_PLATFORM_SCORECARD_2026-03-31.md`，它只能描述对应历史快照的平台阶段，不等于当前状态或 release-ready 证明。当前不存在的历史 master 文档不得继续作为 truth source。

可用一句话记忆：

> `web4-frontend` 的绿灯只表示这个被明确选择的前端子项目预检通过；它既不是当前 World native client，也不使整个仓库 release-ready。

## 环境变量（可选）

```bash
cp web4-frontend/.env.example web4-frontend/.env.local
```

若当前目录已经是 `web4-frontend/`：

```bash
cp .env.example .env.local
```

可选变量包括：

- `NEXT_PUBLIC_QUERY_API_BASE_URL`
- `NEXT_PUBLIC_DASHBOARD_TASK_ID`
- `NEXT_PUBLIC_DASHBOARD_AUDIT_SUBJECT`
- `NEXT_PUBLIC_DASHBOARD_NORMALIZED_AUDIT_EVENT_LIMIT`
- `NEXT_PUBLIC_DASHBOARD_NORMALIZED_AUDIT_MAX_PAGES`

未设置时使用子项目文档中声明的默认值。默认值不能被解释为生产 API、真实账户或写路径。

## 常用命令

- `npm run dev`：本地开发；
- `npm run lint`：ESLint；
- `npm run typecheck`：TypeScript 类型检查；
- `npm run test` / `npm run test:unit`：Vitest；
- `npm run test:contract`：只读 API 合约适配层测试；
- `npm run test:e2e`：Playwright；
- `npm run ci:check`：统一子项目门禁，默认不跑 E2E；
- `CI_RUN_E2E=1 npm run ci:check`：显式开启 E2E；
- `npm run release:preflight`：子项目 lint/typecheck/test/contract/build；
- `npm run release:ready`：子项目版本、CHANGELOG 与 preflight 一致性。

## 发布边界

只有显式选中并拥有独立部署授权的 Web4 工作流才可运行：

```bash
npm ci
npm run release:ready
npm run start
```

即使命令成功，也只能形成 Web4 子项目证据。它不能授予：

- World native client 或公共游戏发布信用；
- 后端写路径、身份、结算或钱包信用；
- Nakama 在线 authority、Chain finality 或 Integration lock 信用；
- public-edge、隐私、法律、商业或生产授权。

## 对外表述

允许的最小口径：

```text
component=web4-frontend
lifecycle=historical-compatible-subproject
world_release_denominator=none
default_mode=readonly_api_client
mock_fallback=explicit_only
write_path=false
repository_release_ready=false
production_authorization=not_granted
```

任何重新激活都必须由当前 plan、组件目录、owner、测试/部署范围和 release gate 同时明确，不能仅通过修改 npm script 或历史文档完成。
