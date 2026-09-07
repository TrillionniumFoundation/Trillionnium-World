    sqlx::query::query("select pg_advisory_xact_lock($1)")
                sqlx::query_scalar::query_scalar::<_, bool>("select pg_try_advisory_lock($1)")
    if !exact {
        let terminal_ack_gaps =
