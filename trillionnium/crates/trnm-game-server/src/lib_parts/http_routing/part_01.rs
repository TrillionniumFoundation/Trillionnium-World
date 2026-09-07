pub fn build_router(state: AppState) -> Router {
    let body_limit = state.request_body_limit_bytes as usize;
    Router::new()
        .route("/health", get(health))
        .route("/v1/online/readiness", get(readiness))
        .route("/v1/online/campaigns/connect", post(connect_campaign))
        .route("/v1/online/matches", post(create_match))
        .route("/v1/online/matches/join", post(join_match))
        .route("/v1/online/matches/:match_id/start", post(start_match))
        .route(
            "/v1/online/matches/:match_id/commands",
            post(submit_command),
        )
        .route("/v1/online/matches/:match_id/snapshot", post(get_snapshot))
        .route(
            "/v1/online/matches/:match_id/stream",
            get(stream::stream_match),
        )
        .route(
            "/v1/online/matches/:match_id/reconnect",
            post(reconnect_match),
        )
        .route("/v1/product/lobbies", post(create_lobby))
        .route("/v1/product/lobbies/:lobby_id/view", post(get_lobby))
        .route(
            "/v1/product/lobbies/:lobby_id/invites",
            post(invite_to_lobby),
        )
        .route(
            "/v1/product/lobbies/invites/accept",
            post(accept_lobby_invite),
        )
        .route("/v1/product/lobbies/:lobby_id/ready", post(set_lobby_ready))
        .route("/v1/product/lobbies/:lobby_id/queue", post(queue_lobby))
        .route(
            "/v1/product/solo-queue/join",
            post(product_v2::join_solo_queue),
        )
        .route(
            "/v1/product/solo-queue/status",
            post(product_v2::get_solo_queue),
        )
        .route(
            "/v1/product/solo-queue/cancel",
            post(product_v2::cancel_solo_queue),
        )
        .route("/v1/product/rating", post(product_v2::get_rating))
        .route(
            "/v1/product/social/friends/request",
            post(product_v2::request_friend),
        )
        .route(
            "/v1/product/social/friends/resolve",
            post(product_v2::resolve_friend),
        )
        .route("/v1/product/social/block", post(product_v2::set_block))
        .route("/v1/product/social/view", post(product_v2::get_social))
        .route("/v1/product/reports", post(product_v2::create_report))
        .route(
            "/v1/product/moderation/reports/resolve",
            post(product_v2::resolve_report),
        )
        .route(
            "/v1/operations/leaderboard",
            post(operations_v1::get_leaderboard),
        )
        .route("/v1/operations/replays", post(operations_v1::get_replay))
        .route(
            "/v1/operations/replays/playback",
            post(operations_v1::get_replay_playback),
        )
        .route(
            "/v1/operations/replays/latest/playback",
            post(operations_v1::get_latest_replay_playback),
        )
        .route(
            "/v1/operations/reports/replay",
            post(operations_v1::create_replay_report),
        )
        .route(
            "/v1/operations/moderation/queue",
            post(operations_v1::moderation_queue),
        )
        .route(
            "/v1/operations/moderation/action",
            post(operations_v1::moderate_case),
        )
        .route(
            "/v1/operations/enforcements/appeals",
            post(operations_v1::create_enforcement_appeal),
        )
        .route(
            "/v1/operations/moderation/appeals",
            post(operations_v1::enforcement_appeal_queue),
        )
        .route(
            "/v1/operations/moderation/appeals/resolve",
            post(operations_v1::resolve_enforcement_appeal),
        )
        .route(
            "/v1/operations/seasons/admin",
            post(operations_v1::admin_season),
        )
        .route(
            "/v1/operations/fleet/route",
            post(operations_v1::route_fleet),
        )
        .route(
            "/v1/operations/fleet/admin",
            post(operations_v1::admin_fleet),
        )
        .route(
            "/v1/production/seasons/automation",
            post(production_v1::configure_season_automation),
        )
        .route(
            "/v1/production/spectators/invites",
            post(production_v1::create_spectator_invite),
        )
        .route(
            "/v1/production/spectators/invites/accept",
            post(production_v1::accept_spectator_invite),
        )
        .route(
            "/v1/production/spectators/playback",
            post(production_v1::spectator_playback),
        )
        .route(
            "/v1/production/player/status",
            post(production_v1::player_production_status),
        )
        .route(
            "/v1/production/host-attestation",
            post(production_v1::host_attestation),
        )
        .route(
            "/v1/production/moderation/shifts/start",
            post(production_v1::start_moderation_shift),
        )
        .route(
            "/v1/production/moderation/shifts/heartbeat",
            post(production_v1::heartbeat_moderation_shift),
        )
        .route(
            "/v1/production/moderation/claims",
            post(production_v1::claim_moderation_case),
        )
        .route(
            "/v1/production/moderation/shifts/close",
            post(production_v1::close_moderation_shift),
        )
        .route(
            "/v1/production/status",
            get(production_v1::production_status),
        )
        .layer(DefaultBodyLimit::max(body_limit))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            production_rate_limit,
        ))
        .with_state(state)
}

