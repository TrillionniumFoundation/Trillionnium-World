---
status: current-candidate
owner: trillionnium-world-server
last_reviewed: 2026-09-08
review_due: 2026-10-08
authority_profile: world_legacy_local_alpha
---

# Trillionnium World HTTP API v1

## Scope

This document inventories the HTTP surface currently wired by `trnm-game-server`. The server is a bounded compatibility and migration enclave; none of these routes creates canonical public-online authority. Nakama remains the intended owner of admission, canonical order, idempotency, restart recovery, archive roots and `MatchCompletedV1` signing.

## Machine source

The exact route set is `trnm-world-http-route-inventory-v1.json`. `scripts/check-trnm-world-http-route-inventory.py` compares that inventory with `build_router` and fails on a missing, extra, method-changed or handler-changed route. The check also requires the global request-body limit and rate-limit middleware.

## Common transport contract

- JSON requests use UTF-8 and must satisfy the Rust request type selected by the handler. Unknown-field behavior remains type-specific until generated schemas cover every message.
- Player/session, moderator, worker and signer audiences are distinct. A route name does not waive handler-level authorization.
- The configured global body limit applies before handler extraction. Route-specific limits may be narrower.
- Redirects and unbounded response bodies are forbidden for signer/CEX adapters.
- Stable errors carry a bounded code and retry classification; raw credentials and unbounded reflected input are not returned.
- Mutation requests bind protocol/build and expected revision/cursor fields where the corresponding Rust type requires them.

## Route inventory

| Method | Path | Handler | Surface |
|---|---|---|---|
| `GET` | `/health` | `health` | `health` |
| `GET` | `/v1/online/readiness` | `readiness` | `online` |
| `POST` | `/v1/online/campaigns/connect` | `connect_campaign` | `online` |
| `POST` | `/v1/online/matches` | `create_match` | `online` |
| `POST` | `/v1/online/matches/join` | `join_match` | `online` |
| `POST` | `/v1/online/matches/:match_id/start` | `start_match` | `online` |
| `POST` | `/v1/online/matches/:match_id/commands` | `submit_command` | `online` |
| `POST` | `/v1/online/matches/:match_id/snapshot` | `get_snapshot` | `online` |
| `GET` | `/v1/online/matches/:match_id/stream` | `stream::stream_match` | `online` |
| `POST` | `/v1/online/matches/:match_id/reconnect` | `reconnect_match` | `online` |
| `POST` | `/v1/product/lobbies` | `create_lobby` | `product` |
| `POST` | `/v1/product/lobbies/:lobby_id/view` | `get_lobby` | `product` |
| `POST` | `/v1/product/lobbies/:lobby_id/invites` | `invite_to_lobby` | `product` |
| `POST` | `/v1/product/lobbies/invites/accept` | `accept_lobby_invite` | `product` |
| `POST` | `/v1/product/lobbies/:lobby_id/ready` | `set_lobby_ready` | `product` |
| `POST` | `/v1/product/lobbies/:lobby_id/queue` | `queue_lobby` | `product` |
| `POST` | `/v1/product/solo-queue/join` | `product_v2::join_solo_queue` | `product` |
| `POST` | `/v1/product/solo-queue/status` | `product_v2::get_solo_queue` | `product` |
| `POST` | `/v1/product/solo-queue/cancel` | `product_v2::cancel_solo_queue` | `product` |
| `POST` | `/v1/product/rating` | `product_v2::get_rating` | `product` |
| `POST` | `/v1/product/social/friends/request` | `product_v2::request_friend` | `product` |
| `POST` | `/v1/product/social/friends/resolve` | `product_v2::resolve_friend` | `product` |
| `POST` | `/v1/product/social/block` | `product_v2::set_block` | `product` |
| `POST` | `/v1/product/social/view` | `product_v2::get_social` | `product` |
| `POST` | `/v1/product/reports` | `product_v2::create_report` | `product` |
| `POST` | `/v1/product/moderation/reports/resolve` | `product_v2::resolve_report` | `product` |
| `POST` | `/v1/operations/leaderboard` | `operations_v1::get_leaderboard` | `operations` |
| `POST` | `/v1/operations/replays` | `operations_v1::get_replay` | `operations` |
| `POST` | `/v1/operations/replays/playback` | `operations_v1::get_replay_playback` | `operations` |
| `POST` | `/v1/operations/replays/latest/playback` | `operations_v1::get_latest_replay_playback` | `operations` |
| `POST` | `/v1/operations/reports/replay` | `operations_v1::create_replay_report` | `operations` |
| `POST` | `/v1/operations/moderation/queue` | `operations_v1::moderation_queue` | `operations` |
| `POST` | `/v1/operations/moderation/action` | `operations_v1::moderate_case` | `operations` |
| `POST` | `/v1/operations/enforcements/appeals` | `operations_v1::create_enforcement_appeal` | `operations` |
| `POST` | `/v1/operations/moderation/appeals` | `operations_v1::enforcement_appeal_queue` | `operations` |
| `POST` | `/v1/operations/moderation/appeals/resolve` | `operations_v1::resolve_enforcement_appeal` | `operations` |
| `POST` | `/v1/operations/seasons/admin` | `operations_v1::admin_season` | `operations` |
| `POST` | `/v1/operations/fleet/route` | `operations_v1::route_fleet` | `operations` |
| `POST` | `/v1/operations/fleet/admin` | `operations_v1::admin_fleet` | `operations` |
| `POST` | `/v1/production/seasons/automation` | `production_v1::configure_season_automation` | `production` |
| `POST` | `/v1/production/spectators/invites` | `production_v1::create_spectator_invite` | `production` |
| `POST` | `/v1/production/spectators/invites/accept` | `production_v1::accept_spectator_invite` | `production` |
| `POST` | `/v1/production/spectators/playback` | `production_v1::spectator_playback` | `production` |
| `POST` | `/v1/production/player/status` | `production_v1::player_production_status` | `production` |
| `POST` | `/v1/production/host-attestation` | `production_v1::host_attestation` | `production` |
| `POST` | `/v1/production/moderation/shifts/start` | `production_v1::start_moderation_shift` | `production` |
| `POST` | `/v1/production/moderation/shifts/heartbeat` | `production_v1::heartbeat_moderation_shift` | `production` |
| `POST` | `/v1/production/moderation/claims` | `production_v1::claim_moderation_case` | `production` |
| `POST` | `/v1/production/moderation/shifts/close` | `production_v1::close_moderation_shift` | `production` |
| `GET` | `/v1/production/status` | `production_v1::production_status` | `production` |

## Authority and side effects

`/health` and readiness/status routes expose bounded health projections; they do not grant admission. Online and product mutations are compatibility behavior only. Operations and production routes require their explicit private/moderator service identities where implemented. Wallet/ledger effects never occur directly in an HTTP handler: settlement is captured durably, executed outside mutable database transactions and applied under exact fences.

## Idempotency and concurrency

Command, lobby, report, replay, moderation and settlement identities are type-specific. Exact duplicates may return the existing result where the contract says so; altered reuse of an identity fails closed. Per-match mutations are serialized by the authority actor/database fences, while unrelated matches/accounts may progress within bounded pools. Handlers must not hold mutable match or campaign row locks across network calls.

## Compatibility and retirement

Protocol identity and build identity are separate. The V2/V3 compatibility enclave remains laboratory-only. New canonical endpoints belong in Nakama, not in this router. Retirement requires caller inventory, drain, rollback and endpoint-disablement evidence. Any route addition/removal or method/handler change updates the route inventory, this document, tests and release impact in the same pull request.

## Evidence boundary

Route-source agreement proves only that documentation matches the reviewed router. It does not prove handler semantics, authorization, deployed TLS, public capacity, cross-host recovery or release eligibility. Those require their assigned tests and external evidence.
