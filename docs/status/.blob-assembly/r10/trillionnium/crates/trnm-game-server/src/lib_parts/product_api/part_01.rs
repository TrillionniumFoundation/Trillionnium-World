         ) values ($1, $2, $3, $4, $5)",
    .bind(invite_id)
            .try_get::<Uuid, _>("invite_id")
    .bind(&map_id)
         where campaign_id = $1",
fn start_match_retry_is_idempotent(
    let member_rows = sqlx::query::query(
    .bind(serde_json::to_value(&seed).map_err(internal_serialization)?)
    if member.account_id != account_id {
    if row
        return Err(conflict("match revision changed", loaded.match_revision));
