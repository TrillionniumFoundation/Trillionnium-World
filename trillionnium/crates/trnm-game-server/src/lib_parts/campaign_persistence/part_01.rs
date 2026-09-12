async fn persist_campaign(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    campaign: &CampaignSaveV1,
) -> Result<(), ApiError> {
    persist_campaign_string(transaction, campaign).await
}

async fn persist_campaign_string(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    campaign: &CampaignSaveV1,
) -> Result<(), ApiError> {
    let state_hash = hash_json(campaign)?;
    sqlx::query::query(
        "update trnm_online_campaigns set campaign_revision = $2, schema_revision = $3,
            state_hash = $4, campaign_json = $5, updated_at = now()
         where campaign_id = $1",
    )
    .bind(&campaign.campaign_id)
    .bind(campaign.revision as i64)
    .bind(i32::from(campaign.schema_revision))
    .bind(state_hash)
    .bind(serde_json::to_value(campaign).map_err(internal_serialization)?)
    .execute(&mut **transaction)
    .await
    .map_err(internal_db)?;
    Ok(())
}

async fn ensure_campaign_progression_is_published<'e, E>(
    executor: E,
    campaign_id: &str,
) -> Result<(), ApiError>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    if campaign_has_unacknowledged_progression(executor, campaign_id)
        .await
        .map_err(|error| {
            api_error(
                StatusCode::SERVICE_UNAVAILABLE,
                format!("campaign terminal publication lookup failed: {error}"),
                true,
            )
        })?
    {
        return Err(api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "campaign progression is quarantined behind an unacknowledged terminal publication",
            false,
        ));
    }
    Ok(())
}

async fn ensure_campaign_owner(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    campaign_id: &str,
    player_id: &str,
    account_id: Uuid,
) -> Result<(), ApiError> {
    let owner = sqlx::query::query(
        "select player_id, account_id from trnm_online_campaigns where campaign_id = $1",
    )
    .bind(campaign_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "cloud campaign not found", false))?;
    if owner
        .try_get::<String, _>("player_id")
        .map_err(internal_db)?
        != player_id
        || owner
            .try_get::<Uuid, _>("account_id")
            .map_err(internal_db)?
            != account_id
    {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "cloud campaign does not belong to the authenticated player/account",
            false,
        ));
    }
    ensure_campaign_progression_is_published(&mut **transaction, campaign_id).await?;
    Ok(())
}

async fn lock_player_lobby_scope(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    player_id: &str,
) -> Result<(), ApiError> {
    sqlx::query::query("select pg_advisory_xact_lock(hashtextextended($1, 0))")
        .bind(format!("trnm-online-lobby:{player_id}"))
        .execute(&mut **transaction)
        .await
        .map_err(internal_db)?;
    Ok(())
}

async fn ensure_player_has_no_active_lobby(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    player_id: &str,
) -> Result<(), ApiError> {
    let active: bool = sqlx::query_scalar::query_scalar(
        "select exists(
            select 1 from trnm_online_lobby_members m
            join trnm_online_lobbies l on l.lobby_id = m.lobby_id
            where m.player_id = $1 and l.status in ('open', 'queued')
         )",
    )
    .bind(player_id)
    .fetch_one(&mut **transaction)
    .await
    .map_err(internal_db)?;
    if active {
        return Err(api_error(
            StatusCode::CONFLICT,
            "player already belongs to an active lobby",
            true,
        ));
    }
    Ok(())
}

async fn lock_lobby(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    lobby_id: Uuid,
) -> Result<sqlx_postgres::PgRow, ApiError> {
    sqlx::query::query(
        "select owner_player_id, owner_account_id, status, lobby_revision, map_id
         from trnm_online_lobbies where lobby_id = $1 for update",
    )
    .bind(lobby_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "lobby not found", false))
}

fn require_lobby_owner(
    lobby: &sqlx_postgres::PgRow,
    player_id: &str,
    account_id: Uuid,
) -> Result<(), ApiError> {
    if lobby
        .try_get::<String, _>("owner_player_id")
        .map_err(internal_db)?
        != player_id
        || lobby
            .try_get::<Uuid, _>("owner_account_id")
            .map_err(internal_db)?
            != account_id
    {
        return Err(api_error(
            StatusCode::FORBIDDEN,
            "only the authenticated lobby owner may perform this operation",
            false,
        ));
    }
    Ok(())
}

fn require_open_lobby_revision(
    lobby: &sqlx_postgres::PgRow,
    expected_revision: u64,
) -> Result<(), ApiError> {
    if lobby.try_get::<String, _>("status").map_err(internal_db)? != "open" {
        return Err(api_error(StatusCode::CONFLICT, "lobby is not open", false));
    }
    let revision = lobby
        .try_get::<i64, _>("lobby_revision")
        .map_err(internal_db)? as u64;
    if revision != expected_revision {
        return Err(conflict("lobby revision changed", revision));
    }
    Ok(())
}

async fn fetch_lobby_view(pool: &PgPool, lobby_id: Uuid) -> Result<OnlineLobbyView, ApiError> {
    let lobby = sqlx::query::query(
        "select display_name, owner_player_id, status, lobby_revision, map_id,
                queue_mode, match_id
         from trnm_online_lobbies where lobby_id = $1",
    )
    .bind(lobby_id)
    .fetch_optional(pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "lobby not found", false))?;
    let rows = sqlx::query::query(
        "select player_id, account_id, campaign_id, member_role, ready
         from trnm_online_lobby_members where lobby_id = $1
         order by case member_role when 'owner' then 0 else 1 end",
    )
    .bind(lobby_id)
    .fetch_all(pool)
    .await
    .map_err(internal_db)?;
    let members = rows
        .into_iter()
        .map(|row| {
            Ok(OnlineLobbyMemberView {
                player_id: row.try_get("player_id").map_err(internal_db)?,
                account_id: row
                    .try_get::<Uuid, _>("account_id")
                    .map_err(internal_db)?
                    .to_string(),
                campaign_id: row.try_get("campaign_id").map_err(internal_db)?,
                role: row.try_get("member_role").map_err(internal_db)?,
                ready: row.try_get("ready").map_err(internal_db)?,
            })
        })
        .collect::<Result<Vec<_>, ApiError>>()?;
    let status: String = lobby.try_get("status").map_err(internal_db)?;
    Ok(OnlineLobbyView {
        protocol_version: ONLINE_PRODUCT_PROTOCOL.to_string(),
        build_id: ONLINE_PRODUCT_BUILD.to_string(),
        lobby_id: lobby_id.to_string(),
        display_name: lobby.try_get("display_name").map_err(internal_db)?,
        owner_player_id: lobby.try_get("owner_player_id").map_err(internal_db)?,
        status: match status.as_str() {
            "open" => OnlineLobbyStatus::Open,
            "queued" => OnlineLobbyStatus::Queued,
            "matched" => OnlineLobbyStatus::Matched,
            _ => OnlineLobbyStatus::Closed,
        },
        lobby_revision: lobby
            .try_get::<i64, _>("lobby_revision")
            .map_err(internal_db)? as u64,
        map_id: lobby.try_get("map_id").map_err(internal_db)?,
        queue_mode: lobby.try_get("queue_mode").map_err(internal_db)?,
        members,
        match_id: lobby
            .try_get::<Option<Uuid>, _>("match_id")
            .map_err(internal_db)?
            .map(|value| value.to_string()),
    })
}

fn sha256_text(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

async fn fetch_match_view(pool: &PgPool, match_id: Uuid) -> Result<OnlineMatchView, ApiError> {
    let row = sqlx::query::query(
        "select match_id, join_code, phase, build_id, map_id, match_mode, rules_version, seed_hash,
                snapshot_hash, authoritative_tick, next_sequence, match_revision,
                result_hash, settlement_state
         from trnm_online_matches where match_id = $1",
    )
    .bind(match_id)
    .fetch_optional(pool)
    .await
    .map_err(internal_db)?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "match not found", false))?;
    let member_rows = sqlx::query::query(
        "select m.player_id, m.account_id, m.campaign_id, m.member_role,
                m.controlled_unit_ids, m.next_input_sequence,
                c.campaign_revision, c.campaign_json
         from trnm_online_match_members m
         join trnm_online_campaigns c on c.campaign_id = m.campaign_id
         where m.match_id = $1 order by m.member_role desc",
    )
    .bind(match_id)
    .fetch_all(pool)
    .await
    .map_err(internal_db)?;
    match_view_from_rows(&row, &member_rows)
}

fn match_view_from_rows(
    row: &sqlx_postgres::PgRow,
    member_rows: &[sqlx_postgres::PgRow],
) -> Result<OnlineMatchView, ApiError> {
    let mut members = Vec::with_capacity(member_rows.len());
    for member in member_rows {
        let controlled: Value = member.try_get("controlled_unit_ids").map_err(internal_db)?;
        let campaign_value: Value = member.try_get("campaign_json").map_err(internal_db)?;
        let campaign: CampaignSaveV1 =
            serde_json::from_value(campaign_value).map_err(internal_serialization)?;
        members.push(OnlineMatchMemberView {
            player_id: member.try_get("player_id").map_err(internal_db)?,
            account_id: member
                .try_get::<Uuid, _>("account_id")
                .map_err(internal_db)?
                .to_string(),
            campaign_id: member.try_get("campaign_id").map_err(internal_db)?,
            role: member.try_get("member_role").map_err(internal_db)?,
            controlled_unit_ids: serde_json::from_value(controlled)
                .map_err(internal_serialization)?,
            campaign_revision: member
                .try_get::<i64, _>("campaign_revision")
                .map_err(internal_db)? as u64,
            level: campaign.progression.level,
            experience: campaign.progression.experience,
            inventory_count: campaign
                .progression
                .inventory
                .iter()
                .map(|stack| u64::from(stack.quantity))
                .sum(),
            next_input_sequence: member
                .try_get::<i64, _>("next_input_sequence")
                .map_err(internal_db)? as u64,
        });
    }
    let phase: String = row.try_get("phase").map_err(internal_db)?;
    Ok(OnlineMatchView {
        protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
        build_id: row.try_get("build_id").map_err(internal_db)?,
        match_id: row
            .try_get::<Uuid, _>("match_id")
            .map_err(internal_db)?
            .to_string(),
        join_code: row.try_get("join_code").map_err(internal_db)?,
        phase: match phase.as_str() {
            "waiting" => OnlineMatchPhase::Waiting,
            "running" => OnlineMatchPhase::Running,
            "complete" => OnlineMatchPhase::Complete,
            _ => OnlineMatchPhase::FailedClosed,
        },
        match_revision: row
            .try_get::<i64, _>("match_revision")
            .map_err(internal_db)? as u64,
        authoritative_tick: row
            .try_get::<i64, _>("authoritative_tick")
            .map_err(internal_db)? as u64,
        next_sequence: row
            .try_get::<i64, _>("next_sequence")
            .map_err(internal_db)? as u64,
        map_id: row.try_get("map_id").map_err(internal_db)?,
        match_mode: row.try_get("match_mode").map_err(internal_db)?,
        rules_version: row.try_get("rules_version").map_err(internal_db)?,
        seed_hash: row.try_get("seed_hash").map_err(internal_db)?,
        snapshot_hash: row.try_get("snapshot_hash").map_err(internal_db)?,
        members,
        result_hash: row.try_get("result_hash").map_err(internal_db)?,
        settlement_state: row.try_get("settlement_state").map_err(internal_db)?,
    })
}

fn campaign_view_from_row(row: &sqlx_postgres::PgRow) -> Result<OnlineCampaignView, ApiError> {
    let campaign_value: Value = row.try_get("campaign_json").map_err(internal_db)?;
    let campaign: CampaignSaveV1 =
        serde_json::from_value(campaign_value).map_err(internal_serialization)?;
    Ok(OnlineCampaignView {
        protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
        campaign_id: row.try_get("campaign_id").map_err(internal_db)?,
        player_id: row.try_get("player_id").map_err(internal_db)?,
        account_id: row
            .try_get::<Uuid, _>("account_id")
            .map_err(internal_db)?
            .to_string(),
        slot_key: row.try_get("slot_key").map_err(internal_db)?,
        campaign_revision: row
            .try_get::<i64, _>("campaign_revision")
            .map_err(internal_db)? as u64,
        schema_revision: row
            .try_get::<i32, _>("schema_revision")
            .map_err(internal_db)? as u16,
        state_hash: row.try_get("state_hash").map_err(internal_db)?,
        level: campaign.progression.level,
        experience: campaign.progression.experience,
        reputation: campaign.character.attributes.reputation,
        inventory: campaign
            .progression
            .inventory
            .iter()
            .map(|stack| OnlineInventoryStack {
                item_id: stack.item_id.clone(),
                quantity: stack.quantity,
            })
            .collect(),
        settled_match_count: campaign.settled_battle_ids.len(),
    })
}

fn internal_serialization(error: serde_json::Error) -> ApiError {
    api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false)
}

fn internal_db(error: sqlx::Error) -> ApiError {
    api_error(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Online Authority persistence failed: {error}"),
        false,
    )
}

