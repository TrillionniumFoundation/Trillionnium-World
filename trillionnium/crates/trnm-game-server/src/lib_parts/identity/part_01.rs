fn session_header(headers: &HeaderMap) -> Result<&str, ApiError> {
    headers
        .get(PLAYER_SESSION_HEADER)
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            api_error(
                StatusCode::UNAUTHORIZED,
                "player session is required",
                false,
            )
        })
}

async fn verify_identity(
    state: &AppState,
    headers: &HeaderMap,
    player_id: &str,
    account_id: &str,
) -> Result<cex::SessionVerifyResponse, ApiError> {
    let token = session_header(headers)?;
    let verified = state
        .cex
        .verify_session(token, player_id, account_id)
        .await
        .map_err(|error| api_error(StatusCode::UNAUTHORIZED, error, false))?;
    if verified.player_id != player_id || verified.account_id != account_id {
        return Err(api_error(
            StatusCode::UNAUTHORIZED,
            "verified session identity mismatch",
            false,
        ));
    }
    if verified.session_id.is_empty()
        || verified.device_id.is_empty()
        || verified.recovery_generation <= 0
        || verified.expires_at_epoch < chrono::Utc::now().timestamp()
    {
        return Err(api_error(
            StatusCode::UNAUTHORIZED,
            "verified session metadata is expired or incomplete",
            false,
        ));
    }
    Ok(verified)
}

fn hash_json<T: serde::Serialize>(value: &T) -> Result<String, ApiError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| api_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string(), false))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn validate_slot_key(value: &str) -> Result<(), ApiError> {
    if value.is_empty()
        || value.len() > 32
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "slot_key must be 1-32 ASCII letters, digits, '-' or '_'",
            false,
        ));
    }
    Ok(())
}

fn validate_command_id(value: &str) -> Result<(), ApiError> {
    if value.is_empty()
        || value.len() > 160
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':' | b'.'))
    {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "command_id must be 1-160 portable ASCII identifier characters",
            false,
        ));
    }
    Ok(())
}

