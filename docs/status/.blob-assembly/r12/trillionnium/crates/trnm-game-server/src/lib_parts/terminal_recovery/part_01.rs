        .map_err(|_| format!("readiness database summary {column} is negative"))
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
                    seal_abandonment_in_database(
