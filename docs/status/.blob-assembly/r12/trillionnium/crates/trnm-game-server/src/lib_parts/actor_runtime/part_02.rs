    let publication_journal = state.published_tick_journal.clone();
                            format!("terminal durable publication failed: {error}"),
                        .map_err(|_| "command persistence recovery timed out".to_string())
