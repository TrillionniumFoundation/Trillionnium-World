pub fn validate_operations_bind_addr(bind_addr: SocketAddr) -> Result<(), String> {
    if !bind_addr.ip().is_loopback() {
        return Err(
            "public/non-loopback game-server bind is blocked until KMS/HSM custody, edge rate limiting, DDoS protection and an approved deployment attestation are implemented"
                .to_string(),
        );
    }
    Ok(())
}

fn api_error(status: StatusCode, message: impl Into<String>, recoverable: bool) -> ApiError {
    ApiError {
        status,
        body: OnlineAuthorityError {
            error: message.into(),
            recoverable,
            authoritative_revision: None,
        },
    }
}

fn reject_command_during_drain(state: &AppState) -> Result<(), ApiError> {
    command_drain_error(*state.draining.borrow(), *state.shutdown.borrow()).map_or(Ok(()), Err)
}

fn command_drain_error(draining: bool, shutdown: bool) -> Option<ApiError> {
    (draining || shutdown).then(|| {
        api_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "game server is draining; retry this command on active authority",
            true,
        )
    })
}

