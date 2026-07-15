use super::*;
use axum::http::HeaderValue;
use chrono::{DateTime, Utc};
use sqlx::row::Row;
use trnm_online_protocol::{
    OnlineEnforcementAppealCreateRequest, OnlineEnforcementAppealQueueRequest,
    OnlineEnforcementAppealQueueView, OnlineEnforcementAppealResolveRequest,
    OnlineEnforcementAppealView, OnlineFleetAdminRequest, OnlineFleetAdminView,
    OnlineFleetInstanceView, OnlineFleetRouteRequest, OnlineFleetRouteView,
    OnlineIntegritySignalView, OnlineLeaderboardEntry, OnlineLeaderboardView,
    OnlineModerationActionRequest, OnlineModerationActionView, OnlineModerationCaseView,
    OnlineModerationQueueRequest, OnlineModerationQueueView, OnlineOperationsAccessRequest,
    OnlineReplayAccessRequest, OnlineReplayCommandView, OnlineReplayFrameView,
    OnlineReplayPlaybackView, OnlineReplayReportCreateRequest, OnlineReplayView, OnlineReportView,
    OnlineSeasonAdminRequest, OnlineSeasonAdminView, OnlineSeasonView, ONLINE_OPERATIONS_BUILD,
};

const MODERATOR_HEADER: &str = "x-trnm-moderator";

// Operations projections are public durable truth. Keep this predicate in one CTE so every
// replay, report, leaderboard and season archive query verifies the same terminal publication
// boundary without issuing per-row validation queries. Settlement equality is deliberately
// strict here: a projection is hidden until the ACK has caught up with durable settlement.
const EXACT_ACKNOWLEDGED_TERMINAL_MATCHES_CTE: &str =
    "with exact_acknowledged_terminal_matches as (
        select m.match_id, m.result_json, m.result_hash, m.snapshot_hash
          from trnm_online_matches m
          join trnm_online_terminal_publication_acks a on a.match_id = m.match_id
         where m.phase = 'complete'
           and m.checkpoint_sequence = m.next_sequence
           and m.result_json is not null
           and m.result_hash is not null
           and m.settlement_state in ('pending', 'settled')
           and m.terminal_publication_state = 'acknowledged'
           and a.local_tombstone_state = 'sealed'
           and m.terminal_publication_actor_generation is not null
           and a.actor_generation = m.terminal_publication_actor_generation
           and a.instance_id = m.assigned_instance_id
           and a.actor_epoch = m.assigned_instance_epoch
           and a.physical_host_id = m.assigned_physical_host_id
           and a.authoritative_tick = m.authoritative_tick
           and a.next_sequence = m.next_sequence
           and a.match_revision = m.match_revision
           and a.next_input_sequences = coalesce(
               (select jsonb_object_agg(
                    member.player_id,
                    to_jsonb(member.next_input_sequence)
                    order by member.player_id
                ) from trnm_online_match_members member
                 where member.match_id = m.match_id),
               '{}'::jsonb
           )
           and a.snapshot_hash = m.snapshot_hash
           and a.phase = 'complete'
           and a.result_hash = m.result_hash
           and a.published_settlement_state = m.settlement_state
    )";

fn with_exact_acknowledged_terminal_matches(query: &str) -> String {
    format!("{EXACT_ACKNOWLEDGED_TERMINAL_MATCHES_CTE}\n{query}")
}

#[cfg(test)]
fn operations_terminal_projection_is_visible(
    terminal_publication_state: &str,
    exact_ack_matches: bool,
) -> bool {
    terminal_publication_state == "acknowledged" && exact_ack_matches
}

fn require_operations(protocol: &str, build: &str) -> Result<(), ApiError> {
    if trnm_online_protocol::validate_operations_contract(protocol, build).is_err() {
        return Err(api_error(
            StatusCode::UPGRADE_REQUIRED,
            "Online Operations endpoint requires a supported protocol and exact build",
            false,
        ));
    }
    Ok(())
}

fn require_moderator(state: &AppState, headers: &HeaderMap) -> Result<(), ApiError> {
    let supplied = headers
        .get(MODERATOR_HEADER)
        .and_then(|value: &HeaderValue| value.to_str().ok())
        .unwrap_or_default();
    if supplied.is_empty() || supplied != state.moderator_token.as_str() {
        return Err(api_error(
            StatusCode::UNAUTHORIZED,
            "moderator credential is required",
            false,
        ));
    }
    Ok(())
}

pub(super) async fn heartbeat_fleet(state: &AppState) -> Result<(), String> {
    if !state.database_host_authority.is_healthy() {
        return Err("PostgreSQL host authority is fail-closed".to_string());
    }
    let identity = &state.database_host_authority.identity;
    let updated = sqlx::query_scalar::query_scalar::<_, bool>(ONLINE_HEARTBEAT_FLEET_V1_SQL)
        .bind(state.instance_id.as_str())
        .bind(state.instance_epoch)
        .bind(state.physical_host_id.as_str())
        .bind(state.region.as_str())
        .bind(state.public_endpoint.as_str())
        .bind(ONLINE_OPERATIONS_BUILD)
        .bind(state.capacity)
        .bind(identity.owner_nonce)
        .bind(&identity.application_name)
        .bind(identity.backend_pid)
        .bind(identity.backend_started_at)
        .bind(&identity.database_system_identifier)
        .bind(identity.database_timeline_id)
        .bind(identity.database_postmaster_started_at)
        .bind(identity.leader_lock_key)
        .bind(identity.barrier_lock_key)
        .fetch_one(&state.pool)
        .await
        .map_err(|error| error.to_string())?;
    if !updated {
        return Err("fleet instance epoch was fenced by a newer process".to_string());
    }
    Ok(())
}

pub(super) async fn route_fleet(
    State(state): State<AppState>,
    Json(request): Json<OnlineFleetRouteRequest>,
) -> Result<Json<OnlineFleetRouteView>, ApiError> {
    require_operations(&request.protocol_version, &request.build_id)?;
    if request.preferred_region.trim().is_empty() || request.preferred_region.len() > 80 {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "preferred region is invalid",
            false,
        ));
    }
    heartbeat_fleet(&state).await.map_err(|error| {
        api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            format!("fleet heartbeat failed: {error}"),
            true,
        )
    })?;
    let healthy_count: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_fleet_instances
         where status = 'active' and lease_expires_at > now()
           and active_matches < capacity",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let row = sqlx::query::query(
        "select instance_id, physical_host_id, region, public_endpoint, capacity, active_matches, status,
                extract(epoch from (now() - heartbeat_at))::bigint as heartbeat_age_seconds,
                instance_epoch,
                extract(epoch from (lease_expires_at - now()))::bigint as lease_remaining_seconds
         from trnm_online_fleet_instances
         where status = 'active' and lease_expires_at > now()
           and active_matches < capacity
         order by case when region = $1 then 0 else 1 end,
                  (active_matches::numeric / capacity::numeric), instance_id
         limit 1",
    )
    .bind(&request.preferred_region)
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| {
        api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "no healthy fleet instance has capacity",
            true,
        )
    })?;
    let selected = fleet_view(&row)?;
    Ok(Json(OnlineFleetRouteView {
        protocol_version: request.protocol_version,
        build_id: request.build_id,
        cross_region_fallback: selected.region != request.preferred_region,
        selected,
        healthy_instances: u32::try_from(healthy_count).unwrap_or(u32::MAX),
    }))
}

fn fleet_view(row: &sqlx_postgres::PgRow) -> Result<OnlineFleetInstanceView, ApiError> {
    Ok(OnlineFleetInstanceView {
        instance_id: row.try_get("instance_id").map_err(internal_db)?,
        physical_host_id: row.try_get("physical_host_id").map_err(internal_db)?,
        region: row.try_get("region").map_err(internal_db)?,
        public_endpoint: row.try_get("public_endpoint").map_err(internal_db)?,
        capacity: row.try_get::<i32, _>("capacity").map_err(internal_db)? as u32,
        active_matches: row
            .try_get::<i32, _>("active_matches")
            .map_err(internal_db)? as u32,
        status: row.try_get("status").map_err(internal_db)?,
        heartbeat_age_seconds: row.try_get("heartbeat_age_seconds").map_err(internal_db)?,
        instance_epoch: row
            .try_get::<i64, _>("instance_epoch")
            .map_err(internal_db)? as u64,
        lease_remaining_seconds: row
            .try_get("lease_remaining_seconds")
            .map_err(internal_db)?,
    })
}

pub(super) async fn admin_fleet(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineFleetAdminRequest>,
) -> Result<Json<OnlineFleetAdminView>, ApiError> {
    require_moderator(&state, &headers)?;
    if request.instance_id.is_empty()
        || request.instance_id.len() > 120
        || !matches!(request.action.as_str(), "activate" | "drain" | "offline")
        || !(10..=500).contains(&request.reason.trim().chars().count())
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "fleet admin action, instance or reason is invalid",
            false,
        ));
    }
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let active_matches: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_matches
         where phase = 'running' and assigned_instance_id = $1",
    )
    .bind(&request.instance_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if request.action == "offline" && active_matches > 0 {
        return Err(api_error(
            StatusCode::CONFLICT,
            "instance cannot go offline while it owns running matches; drain first",
            true,
        ));
    }
    let status = match request.action.as_str() {
        "activate" => "active",
        "drain" => "draining",
        "offline" => "offline",
        _ => unreachable!(),
    };
    let updated = sqlx::query::query(
        "update trnm_online_fleet_instances set status = $2,
            drain_reason = case when $2 = 'active' then null else $3 end
         where instance_id = $1",
    )
    .bind(&request.instance_id)
    .bind(status)
    .bind(request.reason.trim())
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    if updated.rows_affected() != 1 {
        return Err(api_error(
            StatusCode::NOT_FOUND,
            "fleet instance was not found",
            false,
        ));
    }
    let audit_id = Uuid::new_v4();
    sqlx::query::query(
        "insert into trnm_online_fleet_admin_audit (
            audit_id, instance_id, action, reason
         ) values ($1, $2, $3, $4)",
    )
    .bind(audit_id)
    .bind(&request.instance_id)
    .bind(&request.action)
    .bind(request.reason.trim())
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(OnlineFleetAdminView {
        audit_id: audit_id.to_string(),
        instance_id: request.instance_id,
        status: status.to_string(),
        active_matches: u32::try_from(active_matches).unwrap_or(u32::MAX),
    }))
}

pub(super) async fn active_season(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
) -> Result<(String, OnlineSeasonView), String> {
    let row = sqlx::query::query(
        "select season_id, display_name, status, rules_version, starts_at, ends_at
         from trnm_online_seasons where status = 'active' and starts_at <= now()
           and ends_at > now() for update",
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "ranked operations require exactly one active season".to_string())?;
    let season_id: String = row
        .try_get("season_id")
        .map_err(|error| error.to_string())?;
    let starts_at: DateTime<Utc> = row
        .try_get("starts_at")
        .map_err(|error| error.to_string())?;
    let ends_at: DateTime<Utc> = row.try_get("ends_at").map_err(|error| error.to_string())?;
    Ok((
        season_id.clone(),
        OnlineSeasonView {
            season_id,
            display_name: row
                .try_get("display_name")
                .map_err(|error| error.to_string())?,
            status: row.try_get("status").map_err(|error| error.to_string())?,
            rules_version: row
                .try_get("rules_version")
                .map_err(|error| error.to_string())?,
            starts_at_epoch: starts_at.timestamp(),
            ends_at_epoch: ends_at.timestamp(),
        },
    ))
}

fn season_rating_after(host_rating: i32, guest_rating: i32, host_won: bool) -> (i32, i32) {
    super::product_v2::coupled_elo_after(host_rating, guest_rating, host_won)
}

pub(super) async fn finalize_ranked_operations(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    match_id: Uuid,
    outcome: BattleOutcome,
    result_hash: &str,
    final_snapshot_hash: &str,
) -> Result<(), String> {
    let match_row = sqlx::query::query(
        "select match_mode, map_id, season_id from trnm_online_matches where match_id = $1",
    )
    .bind(match_id)
    .fetch_one(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    if match_row
        .try_get::<String, _>("match_mode")
        .map_err(|error| error.to_string())?
        != "ranked_pvp"
    {
        return Ok(());
    }
    let season_id: String = match_row
        .try_get::<Option<String>, _>("season_id")
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "ranked match is missing its start-bound season".to_string())?;
    let members = sqlx::query::query(
        "select player_id, account_id, member_role from trnm_online_match_members
         where match_id = $1 order by case member_role when 'host' then 0 else 1 end",
    )
    .bind(match_id)
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    if members.len() != 2 {
        return Err("ranked operations require exactly two match members".to_string());
    }
    let host: String = members[0]
        .try_get("player_id")
        .map_err(|error| error.to_string())?;
    let guest: String = members[1]
        .try_get("player_id")
        .map_err(|error| error.to_string())?;
    let participants = vec![host.clone(), guest.clone()];
    let command_rows = sqlx::query::query(
        "select sequence, command_id, request_hash from trnm_online_commands
         where match_id = $1 order by sequence, command_id",
    )
    .bind(match_id)
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    let command_fingerprints = command_rows
        .iter()
        .map(|row| {
            Ok(json!({
                "sequence": row.try_get::<i64, _>("sequence").map_err(|error| error.to_string())?,
                "command_id": row.try_get::<String, _>("command_id").map_err(|error| error.to_string())?,
                "request_hash": row.try_get::<Option<String>, _>("request_hash").map_err(|error| error.to_string())?,
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let replay_hash = format!(
        "{:x}",
        Sha256::digest(
            serde_json::to_vec(&json!({
                "match_id": match_id,
                "result_hash": result_hash,
                "participants": participants,
                "commands": command_fingerprints,
            }))
            .map_err(|error| error.to_string())?
        )
    );
    let map_id: String = match_row
        .try_get("map_id")
        .map_err(|error| error.to_string())?;
    sqlx::query::query(
        "insert into trnm_online_replay_index (
            match_id, season_id, result_hash, replay_hash, command_count,
            map_id, build_id, participant_ids, final_snapshot_hash
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         on conflict (match_id) do nothing",
    )
    .bind(match_id)
    .bind(&season_id)
    .bind(result_hash)
    .bind(&replay_hash)
    .bind(i32::try_from(command_rows.len()).unwrap_or(i32::MAX))
    .bind(map_id)
    .bind(ONLINE_AUTHORITY_BUILD)
    .bind(json!(participants))
    .bind(final_snapshot_hash)
    .execute(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;

    let high_signal: bool = sqlx::query_scalar::query_scalar(
        "select exists(select 1 from trnm_online_integrity_signals
         where match_id = $1 and status = 'open' and severity = 'high')",
    )
    .bind(match_id)
    .fetch_one(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    // Integrity review quarantines the affected players from leaderboard reads, but the global
    // and season projections must still apply the same result atomically. TODO(rating-ledger):
    // actioned/voided history needs versioned projection replay before it can claim rollback.
    let integrity_state = if high_signal { "under_review" } else { "clear" };
    sqlx::query::query(
        "update trnm_online_rating_events set season_id = $2, integrity_state = $3
         where match_id = $1",
    )
    .bind(match_id)
    .bind(&season_id)
    .bind(integrity_state)
    .execute(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;

    for member in &members {
        sqlx::query::query(
            "insert into trnm_online_season_ratings (season_id, player_id, account_id)
             values ($1, $2, $3) on conflict (season_id, player_id) do nothing",
        )
        .bind(&season_id)
        .bind(
            member
                .try_get::<String, _>("player_id")
                .map_err(|error| error.to_string())?,
        )
        .bind(
            member
                .try_get::<Uuid, _>("account_id")
                .map_err(|error| error.to_string())?,
        )
        .execute(&mut **transaction)
        .await
        .map_err(|error| error.to_string())?;
    }
    let ratings = sqlx::query::query(
        "select player_id, rating from trnm_online_season_ratings
         where season_id = $1 and player_id = any($2) order by player_id for update",
    )
    .bind(&season_id)
    .bind(vec![host.clone(), guest.clone()])
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?;
    let mut before = BTreeMap::new();
    for row in ratings {
        before.insert(
            row.try_get::<String, _>("player_id")
                .map_err(|error| error.to_string())?,
            row.try_get::<i32, _>("rating")
                .map_err(|error| error.to_string())?,
        );
    }
    let host_before = *before
        .get(&host)
        .ok_or_else(|| "host season rating is missing".to_string())?;
    let guest_before = *before
        .get(&guest)
        .ok_or_else(|| "guest season rating is missing".to_string())?;
    let host_won = outcome == BattleOutcome::Victory;
    let (host_after, guest_after) = season_rating_after(host_before, guest_before, host_won);
    for (player, won, after) in [
        (&host, host_won, host_after),
        (&guest, !host_won, guest_after),
    ] {
        sqlx::query::query(
            "update trnm_online_season_ratings set rating = $3,
                wins = wins + case when $4 then 1 else 0 end,
                losses = losses + case when $4 then 0 else 1 end,
                matches = matches + 1, updated_at = now()
             where season_id = $1 and player_id = $2",
        )
        .bind(&season_id)
        .bind(player)
        .bind(after)
        .bind(won)
        .execute(&mut **transaction)
        .await
        .map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub(super) async fn get_leaderboard(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineOperationsAccessRequest>,
) -> Result<Json<OnlineLeaderboardView>, ApiError> {
    require_operations(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let (season_id, season) = active_season(&mut transaction)
        .await
        .map_err(|error| api_error(StatusCode::CONFLICT, error, false))?;
    let leaderboard_sql = with_exact_acknowledged_terminal_matches(
        "select rank, player_id, rating, wins, losses, matches from (
            select rank() over (order by rating desc, wins desc, player_id) as rank,
                   player_id, rating, wins, losses, matches
            from trnm_online_season_ratings season_rating where season_id = $1
              and season_rating.matches = (
                  select count(*)::integer from trnm_online_rating_events projection_event
                   where projection_event.season_id = season_rating.season_id
                     and projection_event.player_id = season_rating.player_id
              )
              and not exists(select 1 from trnm_online_rating_events event
                where event.season_id = season_rating.season_id
                  and event.player_id = season_rating.player_id
                  and (event.integrity_state <> 'clear'
                    or not exists (
                        select 1 from exact_acknowledged_terminal_matches exact_match
                         where exact_match.match_id = event.match_id
                           and exact_match.result_hash = event.result_hash
                    )))
         ) ranked order by rank, player_id limit 100",
    );
    let rows = sqlx::query::query(&leaderboard_sql)
        .bind(&season_id)
        .fetch_all(&mut *transaction)
        .await
        .map_err(internal_db)?;
    let entries = rows
        .iter()
        .map(leaderboard_entry)
        .collect::<Result<Vec<_>, _>>()?;
    let requester_sql = with_exact_acknowledged_terminal_matches(
        "select rank, player_id, rating, wins, losses, matches from (
            select rank() over (order by rating desc, wins desc, player_id) as rank,
                   player_id, rating, wins, losses, matches
            from trnm_online_season_ratings season_rating where season_id = $1
              and season_rating.matches = (
                  select count(*)::integer from trnm_online_rating_events projection_event
                   where projection_event.season_id = season_rating.season_id
                     and projection_event.player_id = season_rating.player_id
              )
              and not exists(select 1 from trnm_online_rating_events event
                where event.season_id = season_rating.season_id
                  and event.player_id = season_rating.player_id
                  and (event.integrity_state <> 'clear'
                    or not exists (
                        select 1 from exact_acknowledged_terminal_matches exact_match
                         where exact_match.match_id = event.match_id
                           and exact_match.result_hash = event.result_hash
                    )))
         ) ranked where player_id = $2",
    );
    let requester_row = sqlx::query::query(&requester_sql)
        .bind(&season_id)
        .bind(&request.player_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(OnlineLeaderboardView {
        protocol_version: request.protocol_version,
        build_id: request.build_id,
        season,
        entries,
        requester: requester_row.as_ref().map(leaderboard_entry).transpose()?,
    }))
}

fn leaderboard_entry(row: &sqlx_postgres::PgRow) -> Result<OnlineLeaderboardEntry, ApiError> {
    Ok(OnlineLeaderboardEntry {
        rank: row.try_get::<i64, _>("rank").map_err(internal_db)? as u32,
        player_id: row.try_get("player_id").map_err(internal_db)?,
        rating: row.try_get("rating").map_err(internal_db)?,
        wins: row.try_get::<i32, _>("wins").map_err(internal_db)? as u32,
        losses: row.try_get::<i32, _>("losses").map_err(internal_db)? as u32,
        matches: row.try_get::<i32, _>("matches").map_err(internal_db)? as u32,
    })
}

fn season_view(row: &sqlx_postgres::PgRow) -> Result<OnlineSeasonView, ApiError> {
    let starts_at: DateTime<Utc> = row.try_get("starts_at").map_err(internal_db)?;
    let ends_at: DateTime<Utc> = row.try_get("ends_at").map_err(internal_db)?;
    Ok(OnlineSeasonView {
        season_id: row.try_get("season_id").map_err(internal_db)?,
        display_name: row.try_get("display_name").map_err(internal_db)?,
        status: row.try_get("status").map_err(internal_db)?,
        rules_version: row.try_get("rules_version").map_err(internal_db)?,
        starts_at_epoch: starts_at.timestamp(),
        ends_at_epoch: ends_at.timestamp(),
    })
}

pub(super) async fn archive_season(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    season_id: &str,
) -> Result<u64, ApiError> {
    let archive_sql = with_exact_acknowledged_terminal_matches(
        "insert into trnm_online_season_snapshots (
            season_id, player_id, final_rank, rating, wins, losses, matches
         ) select season_id, player_id,
                  row_number() over (order by rating desc, wins desc, player_id),
                  rating, wins, losses, matches
           from trnm_online_season_ratings rating
          where season_id = $1
            and rating.matches = (
                select count(*)::integer from trnm_online_rating_events projection_event
                 where projection_event.season_id = rating.season_id
                   and projection_event.player_id = rating.player_id
            )
            and not exists (
            select 1 from trnm_online_rating_events event
             where event.season_id = rating.season_id
               and event.player_id = rating.player_id
               and (event.integrity_state <> 'clear'
                 or not exists (
                     select 1 from exact_acknowledged_terminal_matches exact_match
                      where exact_match.match_id = event.match_id
                        and exact_match.result_hash = event.result_hash
                 ))
          ) on conflict (season_id, player_id) do nothing",
    );
    let result = sqlx::query::query(&archive_sql)
        .bind(season_id)
        .execute(&mut **transaction)
        .await
        .map_err(internal_db)?;
    Ok(result.rows_affected())
}

pub(super) async fn admin_season(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineSeasonAdminRequest>,
) -> Result<Json<OnlineSeasonAdminView>, ApiError> {
    require_moderator(&state, &headers)?;
    if request.season_id.is_empty()
        || request.season_id.len() > 80
        || !request
            .season_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        || !matches!(request.action.as_str(), "create" | "activate" | "close")
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "season admin action or season_id is invalid",
            false,
        ));
    }
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    sqlx::query::query("select pg_advisory_xact_lock(hashtext('trnm-online-season-admin'))")
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
    let previous_active_season_id: Option<String> = sqlx::query_scalar::query_scalar(
        "select season_id from trnm_online_seasons where status = 'active' for update",
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?;
    let mut archived_entries = 0u64;
    match request.action.as_str() {
        "create" => {
            let display_name = request.display_name.as_deref().unwrap_or_default().trim();
            let rules_version = request.rules_version.as_deref().unwrap_or_default().trim();
            let starts_at = request
                .starts_at_epoch
                .and_then(|value| DateTime::<Utc>::from_timestamp(value, 0))
                .ok_or_else(|| api_error(StatusCode::BAD_REQUEST, "starts_at is invalid", false))?;
            let ends_at = request
                .ends_at_epoch
                .and_then(|value| DateTime::<Utc>::from_timestamp(value, 0))
                .ok_or_else(|| api_error(StatusCode::BAD_REQUEST, "ends_at is invalid", false))?;
            if !(3..=80).contains(&display_name.chars().count())
                || !(3..=80).contains(&rules_version.chars().count())
                || ends_at <= starts_at
                || ends_at <= Utc::now()
            {
                return Err(api_error(
                    StatusCode::BAD_REQUEST,
                    "season name, rules or time window is invalid",
                    false,
                ));
            }
            sqlx::query::query(
                "insert into trnm_online_seasons (
                    season_id, display_name, status, rules_version, starts_at, ends_at
                 ) values ($1, $2, 'scheduled', $3, $4, $5)",
            )
            .bind(&request.season_id)
            .bind(display_name)
            .bind(rules_version)
            .bind(starts_at)
            .bind(ends_at)
            .execute(&mut *transaction)
            .await
            .map_err(|error| {
                if error
                    .as_database_error()
                    .is_some_and(|database| database.is_unique_violation())
                {
                    api_error(StatusCode::CONFLICT, "season_id already exists", false)
                } else {
                    internal_db(error)
                }
            })?;
        }
        "activate" => {
            let ranked_busy: bool =
                sqlx::query_scalar::query_scalar(super::product_v2::RANKED_SEASON_BUSY_SQL)
                    .fetch_one(&mut *transaction)
                    .await
                    .map_err(internal_db)?;
            if ranked_busy {
                return Err(api_error(
                    StatusCode::CONFLICT,
                    "season activation is blocked while ranked tickets or matches are active",
                    true,
                ));
            }
            if previous_active_season_id.as_deref() == Some(request.season_id.as_str()) {
                return Err(api_error(
                    StatusCode::CONFLICT,
                    "season is already active",
                    false,
                ));
            }
            let target_valid: bool = sqlx::query_scalar::query_scalar(
                "select exists(select 1 from trnm_online_seasons
                 where season_id = $1 and status = 'scheduled' and ends_at > now())",
            )
            .bind(&request.season_id)
            .fetch_one(&mut *transaction)
            .await
            .map_err(internal_db)?;
            if !target_valid {
                return Err(api_error(
                    StatusCode::CONFLICT,
                    "only a non-expired scheduled season can be activated",
                    false,
                ));
            }
            if let Some(previous) = previous_active_season_id.as_deref() {
                archived_entries = archive_season(&mut transaction, previous).await?;
                sqlx::query::query(
                    "update trnm_online_seasons set status = 'closed',
                        ends_at = greatest(starts_at + interval '1 second', now())
                     where season_id = $1 and status = 'active'",
                )
                .bind(previous)
                .execute(&mut *transaction)
                .await
                .map_err(internal_db)?;
            }
            sqlx::query::query(
                "update trnm_online_seasons set status = 'active', starts_at = now()
                 where season_id = $1 and status = 'scheduled'",
            )
            .bind(&request.season_id)
            .execute(&mut *transaction)
            .await
            .map_err(internal_db)?;
        }
        "close" => {
            let ranked_busy: bool =
                sqlx::query_scalar::query_scalar(super::product_v2::RANKED_SEASON_BUSY_SQL)
                    .fetch_one(&mut *transaction)
                    .await
                    .map_err(internal_db)?;
            if ranked_busy {
                return Err(api_error(
                    StatusCode::CONFLICT,
                    "season close is blocked while ranked tickets or matches are active",
                    true,
                ));
            }
            if previous_active_season_id.as_deref() != Some(request.season_id.as_str()) {
                return Err(api_error(
                    StatusCode::CONFLICT,
                    "season is not active",
                    false,
                ));
            }
            archived_entries = archive_season(&mut transaction, &request.season_id).await?;
            sqlx::query::query(
                "update trnm_online_seasons set status = 'closed',
                    ends_at = greatest(starts_at + interval '1 second', now())
                 where season_id = $1 and status = 'active'",
            )
            .bind(&request.season_id)
            .execute(&mut *transaction)
            .await
            .map_err(internal_db)?;
        }
        _ => unreachable!(),
    }
    let audit_id = Uuid::new_v4();
    sqlx::query::query(
        "insert into trnm_online_season_admin_audit (
            audit_id, action, season_id, previous_active_season_id, detail
         ) values ($1, $2, $3, $4, $5)",
    )
    .bind(audit_id)
    .bind(&request.action)
    .bind(&request.season_id)
    .bind(&previous_active_season_id)
    .bind(json!({"archived_entries": archived_entries}))
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    let row = sqlx::query::query(
        "select season_id, display_name, status, rules_version, starts_at, ends_at
         from trnm_online_seasons where season_id = $1",
    )
    .bind(&request.season_id)
    .fetch_one(&mut *transaction)
    .await
    .map_err(internal_db)?;
    let season = season_view(&row)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(OnlineSeasonAdminView {
        audit_id: audit_id.to_string(),
        season,
        previous_active_season_id,
        archived_entries: u32::try_from(archived_entries).unwrap_or(u32::MAX),
    }))
}

fn replay_view(row: &sqlx_postgres::PgRow) -> Result<OnlineReplayView, ApiError> {
    Ok(OnlineReplayView {
        match_id: row
            .try_get::<Uuid, _>("match_id")
            .map_err(internal_db)?
            .to_string(),
        season_id: row.try_get("season_id").map_err(internal_db)?,
        result_hash: row.try_get("result_hash").map_err(internal_db)?,
        replay_hash: row.try_get("replay_hash").map_err(internal_db)?,
        command_count: row
            .try_get::<i32, _>("command_count")
            .map_err(internal_db)? as u32,
        map_id: row.try_get("map_id").map_err(internal_db)?,
        build_id: row.try_get("build_id").map_err(internal_db)?,
        participant_ids: serde_json::from_value(
            row.try_get::<Value, _>("participant_ids")
                .map_err(internal_db)?,
        )
        .map_err(internal_serialization)?,
        final_snapshot_hash: row.try_get("final_snapshot_hash").map_err(internal_db)?,
    })
}

async fn fetch_replay(pool: &PgPool, match_id: Uuid) -> Result<OnlineReplayView, ApiError> {
    let replay_sql = with_exact_acknowledged_terminal_matches(
        "select replay.match_id, replay.season_id, replay.result_hash, replay.replay_hash,
                replay.command_count, replay.map_id, replay.build_id, replay.participant_ids,
                replay.final_snapshot_hash
           from trnm_online_replay_index replay
           join exact_acknowledged_terminal_matches exact_match
             on exact_match.match_id = replay.match_id
            and exact_match.result_hash = replay.result_hash
            and exact_match.snapshot_hash = replay.final_snapshot_hash
          where replay.match_id = $1",
    );
    let row = sqlx::query::query(&replay_sql)
        .bind(match_id)
        .fetch_optional(pool)
        .await
        .map_err(internal_db)?
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "replay index not found", false))?;
    replay_view(&row)
}

pub(super) async fn get_replay(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineReplayAccessRequest>,
) -> Result<Json<OnlineReplayView>, ApiError> {
    require_operations(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let match_id = Uuid::parse_str(&request.match_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "match_id must be a UUID", false))?;
    let replay_member_sql = with_exact_acknowledged_terminal_matches(
        "select exists(select 1 from trnm_online_match_members
         join exact_acknowledged_terminal_matches exact_match
           on exact_match.match_id = trnm_online_match_members.match_id
         where trnm_online_match_members.match_id = $1
           and player_id = $2 and account_id = $3)",
    );
    let member: bool =
        sqlx::query_scalar::query_scalar(&replay_member_sql)
            .bind(match_id)
            .bind(&request.player_id)
            .bind(Uuid::parse_str(&request.account_id).map_err(|_| {
                api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false)
            })?)
            .fetch_one(&state.pool)
            .await
            .map_err(internal_db)?;
    if !member {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "replay access requires match membership",
            false,
        ));
    }
    Ok(Json(fetch_replay(&state.pool, match_id).await?))
}

pub(super) async fn get_replay_playback(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineReplayAccessRequest>,
) -> Result<Json<OnlineReplayPlaybackView>, ApiError> {
    require_operations(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let match_id = Uuid::parse_str(&request.match_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "match_id must be a UUID", false))?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let playback_member_sql = with_exact_acknowledged_terminal_matches(
        "select exists(select 1 from trnm_online_match_members
         join exact_acknowledged_terminal_matches exact_match
           on exact_match.match_id = trnm_online_match_members.match_id
         where trnm_online_match_members.match_id = $1
           and player_id = $2 and account_id = $3)",
    );
    let member: bool = sqlx::query_scalar::query_scalar(&playback_member_sql)
        .bind(match_id)
        .bind(&request.player_id)
        .bind(account_id)
        .fetch_one(&state.pool)
        .await
        .map_err(internal_db)?;
    if !member {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "replay playback requires match membership",
            false,
        ));
    }
    let replay = fetch_replay(&state.pool, match_id).await?;
    if replay.command_count > 2048 {
        return Err(api_error(
            StatusCode::PAYLOAD_TOO_LARGE,
            "replay command count exceeds the bounded playback envelope",
            false,
        ));
    }
    let commands_sql = with_exact_acknowledged_terminal_matches(
        "select command_row.sequence, command_row.command_id, command_row.player_id,
                command_row.request_hash, command_row.target_tick, command_row.order_json,
                command_row.accepted_snapshot_hash
           from trnm_online_commands command_row
           join exact_acknowledged_terminal_matches exact_match
             on exact_match.match_id = command_row.match_id
          where command_row.match_id = $1
          order by command_row.sequence, command_row.command_id
          limit 2049",
    );
    let command_rows = sqlx::query::query(&commands_sql)
        .bind(match_id)
        .fetch_all(&state.pool)
        .await
        .map_err(internal_db)?;
    if command_rows.len() > 2048 {
        return Err(api_error(
            StatusCode::PAYLOAD_TOO_LARGE,
            "replay command timeline exceeds the bounded playback envelope",
            false,
        ));
    }
    let command_fingerprints = command_rows
        .iter()
        .map(|row| {
            Ok(json!({
                "sequence": row.try_get::<i64, _>("sequence").map_err(internal_db)?,
                "command_id": row.try_get::<String, _>("command_id").map_err(internal_db)?,
                "request_hash": row.try_get::<Option<String>, _>("request_hash").map_err(internal_db)?,
            }))
        })
        .collect::<Result<Vec<_>, ApiError>>()?;
    let recomputed_hash = format!(
        "{:x}",
        Sha256::digest(
            serde_json::to_vec(&json!({
                "match_id": match_id,
                "result_hash": &replay.result_hash,
                "participants": &replay.participant_ids,
                "commands": command_fingerprints,
            }))
            .map_err(internal_serialization)?
        )
    );
    let commands = command_rows
        .iter()
        .map(|row| {
            Ok(OnlineReplayCommandView {
                sequence: row.try_get::<i64, _>("sequence").map_err(internal_db)? as u64,
                player_id: row.try_get("player_id").map_err(internal_db)?,
                target_tick: row.try_get::<i64, _>("target_tick").map_err(internal_db)? as u64,
                request_hash: row
                    .try_get::<Option<String>, _>("request_hash")
                    .map_err(internal_db)?
                    .unwrap_or_default(),
                accepted_snapshot_hash: row
                    .try_get("accepted_snapshot_hash")
                    .map_err(internal_db)?,
                order: row.try_get("order_json").map_err(internal_db)?,
            })
        })
        .collect::<Result<Vec<_>, ApiError>>()?;
    let frames_sql = with_exact_acknowledged_terminal_matches(
        "select frame.tick, frame.snapshot_hash, frame.simulation_json, frame.frame_kind
           from trnm_online_replay_frames frame
           join exact_acknowledged_terminal_matches exact_match
             on exact_match.match_id = frame.match_id
          where frame.match_id = $1
          order by frame.tick
          limit 513",
    );
    let frame_rows = sqlx::query::query(&frames_sql)
        .bind(match_id)
        .fetch_all(&state.pool)
        .await
        .map_err(internal_db)?;
    if frame_rows.len() > 512 {
        return Err(api_error(
            StatusCode::PAYLOAD_TOO_LARGE,
            "replay frame timeline exceeds the bounded playback envelope",
            false,
        ));
    }
    let frames = frame_rows
        .iter()
        .map(|row| {
            Ok(OnlineReplayFrameView {
                tick: row.try_get::<i64, _>("tick").map_err(internal_db)? as u64,
                snapshot_hash: row.try_get("snapshot_hash").map_err(internal_db)?,
                frame_kind: row.try_get("frame_kind").map_err(internal_db)?,
                simulation: row.try_get("simulation_json").map_err(internal_db)?,
            })
        })
        .collect::<Result<Vec<_>, ApiError>>()?;
    let result_sql = with_exact_acknowledged_terminal_matches(
        "select exact_match.result_json
           from exact_acknowledged_terminal_matches exact_match
          where exact_match.match_id = $1",
    );
    let result: Value = sqlx::query_scalar::query_scalar::<_, Option<Value>>(&result_sql)
        .bind(match_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(internal_db)?
        .flatten()
        .ok_or_else(|| api_error(StatusCode::CONFLICT, "match result is not complete", true))?;
    let final_frame_matches = frames.last().is_some_and(|frame| {
        frame.frame_kind == "terminal"
            && replay.final_snapshot_hash.as_deref() == Some(frame.snapshot_hash.as_str())
    });
    let integrity_verified = recomputed_hash == replay.replay_hash
        && commands.len() == replay.command_count as usize
        && frames
            .first()
            .is_some_and(|frame| frame.frame_kind == "initial")
        && final_frame_matches;
    if !integrity_verified {
        return Err(api_error(
            StatusCode::CONFLICT,
            "authoritative replay package failed integrity verification",
            false,
        ));
    }
    Ok(Json(OnlineReplayPlaybackView {
        replay,
        commands,
        frames,
        result,
        integrity_verified,
    }))
}

pub(super) async fn get_latest_replay_playback(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineOperationsAccessRequest>,
) -> Result<Json<OnlineReplayPlaybackView>, ApiError> {
    require_operations(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let latest_replay_sql = with_exact_acknowledged_terminal_matches(
        "select replay.match_id from trnm_online_replay_index replay
         join trnm_online_match_members member on member.match_id = replay.match_id
         join exact_acknowledged_terminal_matches exact_match
           on exact_match.match_id = replay.match_id
          and exact_match.result_hash = replay.result_hash
          and exact_match.snapshot_hash = replay.final_snapshot_hash
         where member.player_id = $1 and member.account_id = $2
         order by replay.created_at desc limit 1",
    );
    let match_id: Uuid = sqlx::query_scalar::query_scalar(&latest_replay_sql)
        .bind(&request.player_id)
        .bind(account_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(internal_db)?
        .ok_or_else(|| {
            api_error(
                StatusCode::NOT_FOUND,
                "no completed replay was found",
                false,
            )
        })?;
    get_replay_playback(
        State(state),
        headers,
        Json(OnlineReplayAccessRequest {
            protocol_version: request.protocol_version,
            build_id: request.build_id,
            player_id: request.player_id,
            account_id: request.account_id,
            match_id: match_id.to_string(),
        }),
    )
    .await
}

pub(super) async fn create_replay_report(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineReplayReportCreateRequest>,
) -> Result<(StatusCode, Json<OnlineReportView>), ApiError> {
    require_operations(&request.protocol_version, &request.build_id)?;
    if !matches!(
        request.category.as_str(),
        "cheating" | "harassment" | "griefing" | "name" | "other"
    ) || request.detail.trim().chars().count() < 10
        || request.detail.chars().count() > 2000
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "report category or detail is invalid",
            false,
        ));
    }
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    let match_id = Uuid::parse_str(&request.match_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "match_id must be a UUID", false))?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let replay = fetch_replay(&state.pool, match_id).await?;
    if replay.replay_hash != request.replay_hash {
        return Err(api_error(
            StatusCode::CONFLICT,
            "replay hash does not match authoritative index",
            false,
        ));
    }
    let report_pair_sql = with_exact_acknowledged_terminal_matches(
        "select exists(select 1 from trnm_online_match_members reporter
         join trnm_online_match_members target on target.match_id = reporter.match_id
         join exact_acknowledged_terminal_matches exact_match
           on exact_match.match_id = reporter.match_id
         where reporter.match_id = $1 and reporter.player_id = $2 and reporter.account_id = $3
           and target.player_id = $4)",
    );
    let pair: bool = sqlx::query_scalar::query_scalar(&report_pair_sql)
        .bind(match_id)
        .bind(&request.player_id)
        .bind(account_id)
        .bind(&request.target_player_id)
        .fetch_one(&state.pool)
        .await
        .map_err(internal_db)?;
    if !pair || request.player_id == request.target_player_id {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "replay report requires two distinct match members",
            false,
        ));
    }
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let signal_id = Uuid::new_v4();
    sqlx::query::query(
        "insert into trnm_online_integrity_signals (
            signal_id, match_id, player_ids, signal_kind, severity, evidence
         ) values ($1, $2, $3, 'manual_review', 'high', $4)",
    )
    .bind(signal_id)
    .bind(match_id)
    .bind(json!([request.player_id, request.target_player_id]))
    .bind(json!({"replay_hash": replay.replay_hash, "category": request.category}))
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    let report_id = Uuid::new_v4();
    let create_report_sql = with_exact_acknowledged_terminal_matches(
        "insert into trnm_online_reports (
            report_id, reporter_player_id, target_player_id, match_id, category,
            detail, replay_hash, season_id, integrity_signal_id
         ) select $1, $2, $3, $4, $5, $6, $7, $8, $9
             from exact_acknowledged_terminal_matches exact_match
            where exact_match.match_id = $4
         returning report_id, reporter_player_id, target_player_id, match_id,
                   category, status, resolution",
    );
    let row = sqlx::query::query(&create_report_sql)
        .bind(report_id)
        .bind(&request.player_id)
        .bind(&request.target_player_id)
        .bind(match_id)
        .bind(&request.category)
        .bind(request.detail.trim())
        .bind(&request.replay_hash)
        .bind(&replay.season_id)
        .bind(signal_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|error| {
            if error
                .as_database_error()
                .is_some_and(|database| database.is_unique_violation())
            {
                return api_error(StatusCode::CONFLICT, "duplicate replay report", false);
            }
            internal_db(error)
        })?
        .ok_or_else(|| {
            api_error(
                StatusCode::NOT_FOUND,
                "report source is not exactly published",
                false,
            )
        })?;
    sqlx::query::query(
        "update trnm_online_rating_events set integrity_state = 'under_review'
         where match_id = $1 and integrity_state = 'clear'",
    )
    .bind(match_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok((
        StatusCode::CREATED,
        Json(super::product_v2::report_view(&row)?),
    ))
}

fn integrity_view(row: &sqlx_postgres::PgRow) -> Result<OnlineIntegritySignalView, ApiError> {
    Ok(OnlineIntegritySignalView {
        signal_id: row
            .try_get::<Uuid, _>("signal_id")
            .map_err(internal_db)?
            .to_string(),
        match_id: row
            .try_get::<Option<Uuid>, _>("match_id")
            .map_err(internal_db)?
            .map(|value| value.to_string()),
        player_ids: serde_json::from_value(
            row.try_get::<Value, _>("player_ids").map_err(internal_db)?,
        )
        .map_err(internal_serialization)?,
        signal_kind: row.try_get("signal_kind").map_err(internal_db)?,
        severity: row.try_get("severity").map_err(internal_db)?,
        status: row.try_get("status").map_err(internal_db)?,
    })
}

async fn moderation_case(
    pool: &PgPool,
    report_row: &sqlx_postgres::PgRow,
) -> Result<OnlineModerationCaseView, ApiError> {
    let report = super::product_v2::report_view(report_row)?;
    let match_id = Uuid::parse_str(&report.match_id).map_err(|_| {
        api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "stored match UUID invalid",
            false,
        )
    })?;
    let replay = Some(fetch_replay(pool, match_id).await?);
    let moderation_signals_sql = with_exact_acknowledged_terminal_matches(
        "select signal.signal_id, signal.match_id, signal.player_ids,
                signal.signal_kind, signal.severity, signal.status
         from trnm_online_integrity_signals signal
         join exact_acknowledged_terminal_matches exact_match
           on exact_match.match_id = signal.match_id
         where signal.match_id = $1 order by signal.created_at",
    );
    let signal_rows = sqlx::query::query(&moderation_signals_sql)
        .bind(match_id)
        .fetch_all(pool)
        .await
        .map_err(internal_db)?;
    Ok(OnlineModerationCaseView {
        report,
        replay,
        integrity_signals: signal_rows
            .iter()
            .map(integrity_view)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

pub(super) async fn moderation_queue(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineModerationQueueRequest>,
) -> Result<Json<OnlineModerationQueueView>, ApiError> {
    require_moderator(&state, &headers)?;
    if !matches!(
        request.status.as_str(),
        "open" | "reviewed" | "actioned" | "dismissed"
    ) || !(1..=100).contains(&request.limit)
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "moderation queue filter is invalid",
            false,
        ));
    }
    let moderation_queue_sql = with_exact_acknowledged_terminal_matches(
        "select report.report_id, report.reporter_player_id, report.target_player_id,
                report.match_id, report.category, report.status, report.resolution
           from trnm_online_reports report
           join trnm_online_replay_index replay on replay.match_id = report.match_id
            and replay.replay_hash = report.replay_hash
           join exact_acknowledged_terminal_matches exact_match
             on exact_match.match_id = report.match_id
            and exact_match.result_hash = replay.result_hash
            and exact_match.snapshot_hash = replay.final_snapshot_hash
          where report.status = $1
          order by report.created_at
          limit $2",
    );
    let rows = sqlx::query::query(&moderation_queue_sql)
        .bind(&request.status)
        .bind(i64::from(request.limit))
        .fetch_all(&state.pool)
        .await
        .map_err(internal_db)?;
    let mut cases = Vec::with_capacity(rows.len());
    for row in &rows {
        cases.push(moderation_case(&state.pool, row).await?);
    }
    let open_report_count_sql = with_exact_acknowledged_terminal_matches(
        "select count(*)
           from trnm_online_reports report
           join trnm_online_replay_index replay on replay.match_id = report.match_id
            and replay.replay_hash = report.replay_hash
           join exact_acknowledged_terminal_matches exact_match
             on exact_match.match_id = report.match_id
            and exact_match.result_hash = replay.result_hash
            and exact_match.snapshot_hash = replay.final_snapshot_hash
          where report.status = 'open'",
    );
    let open_count: i64 = sqlx::query_scalar::query_scalar(&open_report_count_sql)
        .fetch_one(&state.pool)
        .await
        .map_err(internal_db)?;
    let open_signal_count_sql = with_exact_acknowledged_terminal_matches(
        "select count(*) from trnm_online_integrity_signals signal
          where signal.status = 'open'
            and (signal.match_id is null or exists (
                select 1 from exact_acknowledged_terminal_matches exact_match
                 where exact_match.match_id = signal.match_id
            ))",
    );
    let signal_count: i64 = sqlx::query_scalar::query_scalar(&open_signal_count_sql)
        .fetch_one(&state.pool)
        .await
        .map_err(internal_db)?;
    Ok(Json(OnlineModerationQueueView {
        cases,
        open_count: u32::try_from(open_count).unwrap_or(u32::MAX),
        under_review_signal_count: u32::try_from(signal_count).unwrap_or(u32::MAX),
    }))
}

pub(super) async fn moderate_case(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineModerationActionRequest>,
) -> Result<Json<OnlineModerationActionView>, ApiError> {
    require_moderator(&state, &headers)?;
    if !matches!(
        request.decision.as_str(),
        "reviewed" | "actioned" | "dismissed"
    ) || request.resolution.trim().chars().count() < 10
        || request.resolution.chars().count() > 2000
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "moderation action is invalid",
            false,
        ));
    }
    let report_id = Uuid::parse_str(&request.report_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "report_id must be a UUID", false))?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let moderate_report_sql = with_exact_acknowledged_terminal_matches(
        "update trnm_online_reports report
            set status = $2, resolution = $3, resolved_at = now()
           from trnm_online_replay_index replay,
                exact_acknowledged_terminal_matches exact_match
          where report.report_id = $1 and report.status = 'open'
            and replay.match_id = report.match_id
            and replay.replay_hash = report.replay_hash
            and exact_match.match_id = report.match_id
            and exact_match.result_hash = replay.result_hash
            and exact_match.snapshot_hash = replay.final_snapshot_hash
         returning report.report_id, report.reporter_player_id,
                   report.target_player_id, report.match_id, report.category,
                   report.status, report.resolution, report.integrity_signal_id",
    );
    let row = sqlx::query::query(&moderate_report_sql)
        .bind(report_id)
        .bind(&request.decision)
        .bind(request.resolution.trim())
        .fetch_optional(&mut *transaction)
        .await
        .map_err(internal_db)?
        .ok_or_else(|| api_error(StatusCode::CONFLICT, "report is not open", false))?;
    let target_player_id: String = row.try_get("target_player_id").map_err(internal_db)?;
    let signal_id: Option<Uuid> = row.try_get("integrity_signal_id").map_err(internal_db)?;
    if let Some(signal_id) = signal_id {
        sqlx::query::query(
            "update trnm_online_integrity_signals set status = $2, resolved_at = now()
             where signal_id = $1 and status = 'open'",
        )
        .bind(signal_id)
        .bind(if request.decision == "dismissed" {
            "dismissed"
        } else if request.decision == "actioned" {
            "confirmed"
        } else {
            "reviewed"
        })
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
    }
    sqlx::query::query(
        "update trnm_online_rating_events set integrity_state = $2
         where match_id = $1 and integrity_state = 'under_review'",
    )
    .bind(row.try_get::<Uuid, _>("match_id").map_err(internal_db)?)
    .bind(if request.decision == "dismissed" {
        "clear"
    } else if request.decision == "actioned" {
        "voided"
    } else {
        "under_review"
    })
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    let enforcement_id = if request.decision == "actioned" {
        if let Some(scope) = request.enforcement_scope.as_deref() {
            if !matches!(scope, "ranked" | "online")
                || !(1..=720).contains(&request.suspension_hours.unwrap_or_default())
            {
                return Err(api_error(
                    StatusCode::BAD_REQUEST,
                    "enforcement scope or duration is invalid",
                    false,
                ));
            }
            let enforcement_id = Uuid::new_v4();
            sqlx::query::query(
                "insert into trnm_online_enforcements (
                    enforcement_id, player_id, scope, reason, source_report_id, expires_at
                 ) values ($1, $2, $3, $4, $5,
                           now() + make_interval(hours => $6))",
            )
            .bind(enforcement_id)
            .bind(&target_player_id)
            .bind(scope)
            .bind(request.resolution.trim())
            .bind(report_id)
            .bind(request.suspension_hours.unwrap_or_default() as i32)
            .execute(&mut *transaction)
            .await
            .map_err(internal_db)?;
            Some(enforcement_id)
        } else {
            None
        }
    } else {
        None
    };
    let audit_id = Uuid::new_v4();
    sqlx::query::query(
        "insert into trnm_online_moderation_audit (
            audit_id, report_id, action, target_player_id, resolution
         ) values ($1, $2, $3, $4, $5)",
    )
    .bind(audit_id)
    .bind(report_id)
    .bind(&request.decision)
    .bind(&target_player_id)
    .bind(request.resolution.trim())
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    sqlx::query::query(
        "update trnm_online_moderation_case_claims set status = 'resolved', resolved_at = now()
         where case_kind = 'report' and case_id = $1 and status = 'claimed'",
    )
    .bind(report_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(OnlineModerationActionView {
        report: super::product_v2::report_view(&row)?,
        audit_id: audit_id.to_string(),
        enforcement_id: enforcement_id.map(|value| value.to_string()),
        target_player_id,
    }))
}

fn appeal_view(row: &sqlx_postgres::PgRow) -> Result<OnlineEnforcementAppealView, ApiError> {
    let created_at: DateTime<Utc> = row.try_get("created_at").map_err(internal_db)?;
    let due_at: DateTime<Utc> = row.try_get("due_at").map_err(internal_db)?;
    let status: String = row.try_get("status").map_err(internal_db)?;
    Ok(OnlineEnforcementAppealView {
        appeal_id: row
            .try_get::<Uuid, _>("appeal_id")
            .map_err(internal_db)?
            .to_string(),
        enforcement_id: row
            .try_get::<Uuid, _>("enforcement_id")
            .map_err(internal_db)?
            .to_string(),
        player_id: row.try_get("player_id").map_err(internal_db)?,
        status: status.clone(),
        detail: row.try_get("detail").map_err(internal_db)?,
        resolution: row.try_get("resolution").map_err(internal_db)?,
        created_at_epoch: created_at.timestamp(),
        due_at_epoch: due_at.timestamp(),
        overdue: status == "pending" && due_at < Utc::now(),
    })
}

pub(super) async fn create_enforcement_appeal(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineEnforcementAppealCreateRequest>,
) -> Result<(StatusCode, Json<OnlineEnforcementAppealView>), ApiError> {
    require_operations(&request.protocol_version, &request.build_id)?;
    verify_identity(&state, &headers, &request.player_id, &request.account_id).await?;
    if !(20..=2000).contains(&request.detail.trim().chars().count()) {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "appeal detail must contain 20-2000 characters",
            false,
        ));
    }
    let enforcement_id = Uuid::parse_str(&request.enforcement_id).map_err(|_| {
        api_error(
            StatusCode::BAD_REQUEST,
            "enforcement_id must be a UUID",
            false,
        )
    })?;
    let account_id = Uuid::parse_str(&request.account_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "account_id must be a UUID", false))?;
    let owned: bool = sqlx::query_scalar::query_scalar(
        "select exists(select 1 from trnm_online_enforcements
         where enforcement_id = $1 and player_id = $2 and revoked_at is null
           and expires_at > now())",
    )
    .bind(enforcement_id)
    .bind(&request.player_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    if !owned {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "appeal requires an active enforcement owned by the player",
            false,
        ));
    }
    let row = sqlx::query::query(
        "insert into trnm_online_enforcement_appeals (
            appeal_id, enforcement_id, player_id, account_id, detail
         ) values ($1, $2, $3, $4, $5)
         returning appeal_id, enforcement_id, player_id, status, detail,
                   resolution, created_at, due_at",
    )
    .bind(Uuid::new_v4())
    .bind(enforcement_id)
    .bind(&request.player_id)
    .bind(account_id)
    .bind(request.detail.trim())
    .fetch_one(&state.pool)
    .await
    .map_err(|error| {
        if error
            .as_database_error()
            .is_some_and(|database| database.is_unique_violation())
        {
            api_error(
                StatusCode::CONFLICT,
                "enforcement already has an appeal",
                false,
            )
        } else {
            internal_db(error)
        }
    })?;
    Ok((StatusCode::CREATED, Json(appeal_view(&row)?)))
}

pub(super) async fn enforcement_appeal_queue(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineEnforcementAppealQueueRequest>,
) -> Result<Json<OnlineEnforcementAppealQueueView>, ApiError> {
    require_moderator(&state, &headers)?;
    if !matches!(request.status.as_str(), "pending" | "approved" | "rejected")
        || !(1..=100).contains(&request.limit)
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "appeal queue filter is invalid",
            false,
        ));
    }
    let rows = sqlx::query::query(
        "select appeal_id, enforcement_id, player_id, status, detail,
                resolution, created_at, due_at
         from trnm_online_enforcement_appeals where status = $1
         order by due_at, created_at limit $2",
    )
    .bind(&request.status)
    .bind(i64::from(request.limit))
    .fetch_all(&state.pool)
    .await
    .map_err(internal_db)?;
    let pending_count: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_enforcement_appeals where status = 'pending'",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    let overdue_count: i64 = sqlx::query_scalar::query_scalar(
        "select count(*) from trnm_online_enforcement_appeals
         where status = 'pending' and due_at < now()",
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_db)?;
    Ok(Json(OnlineEnforcementAppealQueueView {
        appeals: rows
            .iter()
            .map(appeal_view)
            .collect::<Result<Vec<_>, _>>()?,
        pending_count: u32::try_from(pending_count).unwrap_or(u32::MAX),
        overdue_count: u32::try_from(overdue_count).unwrap_or(u32::MAX),
    }))
}

pub(super) async fn resolve_enforcement_appeal(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<OnlineEnforcementAppealResolveRequest>,
) -> Result<Json<OnlineEnforcementAppealView>, ApiError> {
    require_moderator(&state, &headers)?;
    if !matches!(request.decision.as_str(), "approved" | "rejected")
        || !(10..=2000).contains(&request.resolution.trim().chars().count())
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "appeal decision or resolution is invalid",
            false,
        ));
    }
    let appeal_id = Uuid::parse_str(&request.appeal_id)
        .map_err(|_| api_error(StatusCode::BAD_REQUEST, "appeal_id must be a UUID", false))?;
    let mut transaction = state.pool.begin().await.map_err(internal_db)?;
    let row = sqlx::query::query(
        "update trnm_online_enforcement_appeals set status = $2, resolution = $3,
                resolved_at = now()
         where appeal_id = $1 and status = 'pending'
         returning appeal_id, enforcement_id, player_id, status, detail,
                   resolution, created_at, due_at",
    )
    .bind(appeal_id)
    .bind(&request.decision)
    .bind(request.resolution.trim())
    .fetch_optional(&mut *transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::CONFLICT, "appeal is not pending", false))?;
    let enforcement_id: Uuid = row.try_get("enforcement_id").map_err(internal_db)?;
    let player_id: String = row.try_get("player_id").map_err(internal_db)?;
    if request.decision == "approved" {
        sqlx::query::query(
            "update trnm_online_enforcements set revoked_at = now()
             where enforcement_id = $1 and revoked_at is null",
        )
        .bind(enforcement_id)
        .execute(&mut *transaction)
        .await
        .map_err(internal_db)?;
    }
    sqlx::query::query(
        "update trnm_online_appeal_escalations set status = 'closed', closed_at = now()
         where appeal_id = $1 and status <> 'closed'",
    )
    .bind(appeal_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    sqlx::query::query(
        "update trnm_online_moderation_case_claims set status = 'resolved', resolved_at = now()
         where case_kind = 'appeal' and case_id = $1 and status = 'claimed'",
    )
    .bind(appeal_id)
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    sqlx::query::query(
        "insert into trnm_online_moderation_audit (
            audit_id, action, target_player_id, resolution
         ) values ($1, $2, $3, $4)",
    )
    .bind(Uuid::new_v4())
    .bind(format!("appeal_{}", request.decision))
    .bind(player_id)
    .bind(request.resolution.trim())
    .execute(&mut *transaction)
    .await
    .map_err(internal_db)?;
    transaction.commit().await.map_err(internal_db)?;
    Ok(Json(appeal_view(&row)?))
}

#[cfg(test)]
mod tests {
    use super::{
        operations_terminal_projection_is_visible, season_rating_after,
        with_exact_acknowledged_terminal_matches,
    };

    #[test]
    fn operations_projections_require_acknowledged_state_and_an_exact_ack() {
        assert!(operations_terminal_projection_is_visible(
            "acknowledged",
            true
        ));
        assert!(!operations_terminal_projection_is_visible(
            "acknowledged",
            false
        ));
        assert!(!operations_terminal_projection_is_visible("pending", true));
        assert!(!operations_terminal_projection_is_visible(
            "legacy_quarantined",
            true
        ));
    }

    #[test]
    fn operations_projection_queries_use_the_strict_shared_ack_gate() {
        let sql = with_exact_acknowledged_terminal_matches("select 1");
        assert!(sql.contains("terminal_publication_state = 'acknowledged'"));
        assert!(sql.contains("a.local_tombstone_state = 'sealed'"));
        assert!(sql.contains("a.actor_generation = m.terminal_publication_actor_generation"));
        assert!(sql.contains("a.instance_id = m.assigned_instance_id"));
        assert!(sql.contains("a.actor_epoch = m.assigned_instance_epoch"));
        assert!(sql.contains("a.physical_host_id = m.assigned_physical_host_id"));
        assert!(sql.contains("a.published_settlement_state = m.settlement_state"));
        assert!(sql.ends_with("select 1"));
    }

    #[test]
    fn season_rating_projection_always_uses_the_global_coupled_result() {
        for (host, guest, host_won) in [
            (1000, 1000, true),
            (1000, 1000, false),
            (1000, 2000, true),
            (4999, 5000, true),
            (1, 0, false),
        ] {
            assert_eq!(
                season_rating_after(host, guest, host_won),
                super::super::product_v2::coupled_elo_after(host, guest, host_won),
            );
        }
        assert_eq!(season_rating_after(1000, 1000, true), (1016, 984));
    }
}
