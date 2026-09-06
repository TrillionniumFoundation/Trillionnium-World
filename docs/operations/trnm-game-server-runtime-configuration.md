---
status: current
owner: trillionnium-world-operations
last_reviewed: 2026-08-27
---

# TRNM Game Server Runtime Configuration

## Installation model

`deploy/systemd/*.service` files are installation templates. They contain
`@TRNM_WORLD_ROOT@`, `@TRNM_CONFIG_HOME@`, and `@TRNM_STATE_HOME@` placeholders.
`scripts/install-trnm-game-server-systemd.sh` renders those placeholders for the
current checkout and user. Personal paths are never committed.

The installer creates, but does not overwrite:

- `$HOME/.config/trillionnium-world/game-server.env`;
- `$HOME/.config/trillionnium-world/entitlement-signer.env`.

It installs/enables units without starting them by default. `--start` validates
configuration, starts the signer, waits for readiness, starts the game server,
and verifies runtime resource budgets.

## Production binary policy

Production startup requires a release directory accepted by
`scripts/check-trnm-game-server-release.sh`.

Selection order:

1. explicit `TRNM_GAME_SERVER_RELEASE_DIR`;
2. `run/releases/trnm-game-server/current` when present;
3. development binary only when `TRNM_ALLOW_DEV_BINARY=1`.

A missing, empty, dangling or invalid production selector fails closed. A
production unit must not set `TRNM_ALLOW_DEV_BINARY=1`.

## Required separation

The following values are required and must be pairwise distinct:

- `TRNM_GAME_AUTHORITY_TOKEN`;
- `TRNM_MODERATOR_TOKEN`;
- `TRNM_ENTITLEMENT_SIGNER_TOKEN`.

Do not derive them by prefixing a shared administrator token. Each credential
needs its own owner, audience, rotation procedure and revocation path.

The signer environment file should not contain game-authority or moderator
credentials. The game-server environment file should not contain the signer
private seed.

## External repository boundary

Runtime scripts no longer source CEX helper files or assume a sibling CEX
checkout. CEX provides network contracts and credentials through explicit
configuration. World installation must be possible from a World release bundle
plus versioned external service endpoints.

## State and permissions

- Journal and runtime state belong under `$HOME/.local/state/trillionnium-world`.
- Environment and key files use mode `0600`.
- Service units set `UMask=0077` and use systemd filesystem protections.
- The entitlement private seed is readable only by the signer service context.
- Logs must not print environment values or HTTP authorization headers.

## Local development

For a local source build only:

```bash
export TRNM_ALLOW_DEV_BINARY=1
export DATABASE_URL='postgresql://...'
export TRNM_GAME_AUTHORITY_TOKEN='independent-game-authority-secret'
export TRNM_MODERATOR_TOKEN='independent-moderator-secret'
export TRNM_ENTITLEMENT_SIGNER_TOKEN='independent-signer-secret'
./scripts/run-trnm-game-server.sh
```

This mode grants no release or deployment credit. CI and production evidence
must use a verified release selector.

## Verification

Run:

```bash
./scripts/check_trnm_runtime_configuration.sh
bash -n scripts/run-trnm-game-server.sh
bash -n scripts/run-trnm-entitlement-signer.sh
bash -n scripts/install-trnm-game-server-systemd.sh
```
