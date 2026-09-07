async fn lock_current_fleet_epoch(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    state: &AppState,
    allow_draining: bool,
) -> Result<(), String> {
    require_database_host_authority(transaction, &state.database_host_authority).await?;
    let row = sqlx::query::query(
        "select instance_epoch, status, physical_host_id,
                lease_expires_at > now() as lease_current
         from trnm_online_fleet_instances where instance_id = $1",
    )
    .bind(state.instance_id.as_str())
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "fleet instance registration is missing".to_string())?;
    let instance_epoch: i64 = row
        .try_get("instance_epoch")
        .map_err(|error| error.to_string())?;
    let status: String = row.try_get("status").map_err(|error| error.to_string())?;
    let physical_host_id: String = row
        .try_get("physical_host_id")
        .map_err(|error| error.to_string())?;
    let lease_current: bool = row
        .try_get("lease_current")
        .map_err(|error| error.to_string())?;
    let status_current = status == "active" || (allow_draining && status == "draining");
    if instance_epoch != state.instance_epoch
        || physical_host_id != *state.physical_host_id
        || !status_current
        || !lease_current
    {
        return Err(
            "fleet instance epoch/physical host is fenced, expired or not routable".to_string(),
        );
    }
    Ok(())
}

async fn lock_current_fleet_epoch_for_commit(
    transaction: &mut sqlx::transaction::Transaction<'_, sqlx_postgres::Postgres>,
    state: &AppState,
    allow_draining: bool,
) -> Result<(), String> {
    require_database_host_authority(transaction, &state.database_host_authority).await?;
    let row = sqlx::query::query(
        "select instance_epoch, status, physical_host_id,
                lease_expires_at > now() as lease_current
         from trnm_online_fleet_instances where instance_id = $1 for share",
    )
    .bind(state.instance_id.as_str())
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "fleet instance registration is missing".to_string())?;
    let instance_epoch: i64 = row
        .try_get("instance_epoch")
        .map_err(|error| error.to_string())?;
    let status: String = row.try_get("status").map_err(|error| error.to_string())?;
    let physical_host_id: String = row
        .try_get("physical_host_id")
        .map_err(|error| error.to_string())?;
    let lease_current: bool = row
        .try_get("lease_current")
        .map_err(|error| error.to_string())?;
    let status_current = status == "active" || (allow_draining && status == "draining");
    if instance_epoch != state.instance_epoch
        || physical_host_id != *state.physical_host_id
        || !status_current
        || !lease_current
    {
        return Err(
            "fleet instance epoch/physical host changed before transaction commit".to_string(),
        );
    }
    Ok(())
}

fn conflict(message: impl Into<String>, revision: u64) -> ApiError {
    ApiError {
        status: StatusCode::CONFLICT,
        body: OnlineAuthorityError {
            error: message.into(),
            recoverable: true,
            authoritative_revision: Some(revision),
        },
    }
}

