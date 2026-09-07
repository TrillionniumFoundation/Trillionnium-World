    let reconciliation_state = state.clone();
            Ok(TerminalJournalReconciliationOutcome::FailedClosed) => {
                if let Err(update_error) = sqlx::query::query(
        Ok(Some(initialized)) => {
                    tick: loaded.simulation.tick,
    .fetch_optional(&mut *transaction)
        .try_get::<i64, _>("authoritative_tick")
                "private terminal stage does not match its exact terminal HWM witness".to_string(),
const MAX_PUBLISHED_TICK_RECOVERY_STEPS: u64 = 10_000;
        return false;
    let Some(mut durable) = load_match_actor(state, match_id).await? else {
