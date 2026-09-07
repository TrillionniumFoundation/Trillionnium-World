        durable_recovery_tick,
        if !matches!(
    let publication_journal = state.published_tick_journal.clone();
                    ) {
                            "checkpoint writer failed during command persistence",
                            format!("terminal durable publication failed: {error}"),
                    tracing::error!(%match_id, %error, "terminal marker committed but public tuple release failed closed");
                            pending.accepted_revision,
                        .map_err(|_| "command persistence recovery timed out".to_string())
                        terminal: true,
                let command_id = persistence.request.command_id.clone();
