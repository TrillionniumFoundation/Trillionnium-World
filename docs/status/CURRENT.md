# Trillionnium World Current Status

> Generated from `docs/status/world-gates-v1.json`. Do not edit gate claims in this file.

- As of: `2026-08-27`
- Source plan: `docs/development/trnm-world-development-plan-v3.md`
- Public online: **NO-GO**
- Public player market: **disabled**

| Gate | Status | Authority profile | Primary blockers |
| --- | --- | --- | --- |
| `deterministic_runtime_alpha` | **implemented** | `world_game_domain` | exact remote golden-vector evidence is not registered in this record |
| `native_software_alpha` | **blocked** | `world_native_client` | current integration commit has not completed the remote package and platform evidence matrix |
| `trusted_cex_settlement` | **blocked** | `world_legacy_local_alpha_plus_cex` | settlement capture/execute/apply implementation and remote ambiguous-commit fault evidence are not fully registered<br>production credential custody is not attested |
| `closed_online_nakama` | **blocked** | `nakama_target` | Nakama-only admission, ordering, idempotency, recovery and signed completion are not fully migrated<br>exact World/Nakama/Integration compatibility evidence is not registered |
| `public_online` | **no_go** | `nakama_target` | closed-online authority migration is incomplete<br>public TLS, WAF/DDoS, KMS/HSM, replicated durability, 24-hour endurance, capacity and human gates are incomplete |
| `public_player_market` | **disabled** | `separate_future_approval` | public online is no-go<br>custody, market abuse, legal, economic and governance approvals are absent |
| `commercial_single_player` | **blocked** | `world_native_client` | distribution, accessibility, support, legal and external human evidence are incomplete |

## Explicit limitations

### `deterministic_runtime_alpha`

- source implementation is not equivalent to remote verification or deployment

### `native_software_alpha`

- no Windows/macOS signed distribution credit
- no external human usability credit

### `trusted_cex_settlement`

- local/single-node profile only
- no public wallet or market credit

### `closed_online_nakama`

- World-local Online Authority remains migration-era legacy_local_alpha
- no dual-authority release credit

### `public_online`

- loopback/local evidence only
- cross-host RPO=0 is not implemented or attested
- no public ingress approval

### `public_player_market`

- must not be enabled by a game-source or local test change

### `commercial_single_player`

- technical-alpha evidence does not imply commercial readiness

## Interpretation

`implemented` is a source status, not remote verification, deployment, operational evidence or release readiness. A promoted status requires exact-commit remote evidence accepted by the gate schema. Gate schema v1 intentionally prevents public-online, public-market, trusted-settlement and closed-Nakama promotion.
