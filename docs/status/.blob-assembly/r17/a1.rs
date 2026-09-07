pub fn production_authority_tick_interval() -> Duration {
            Ok(TerminalJournalReconciliationOutcome::FailedClosed) => {
    match initialized {
    .fetch_optional(&mut *transaction)
                "private terminal stage does not match its exact terminal HWM witness".to_string(),
        || durable_next_input_sequences.len() != high_water.next_input_sequences.len()
    let initial = state
async fn reload_visible_actor_after_persistence_failure(
            durable_db_next_sequence,
            durable_db_match_revision,
            durable_db_next_input_sequences,
        ),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            // Once an fsync request was accepted, cancellation cannot prove
            // whether the blocking filesystem operation took effect. Poison
            // the host journal for this process so no replacement actor can
            // publish over an uncertain high-water until restart recovery.
            journal.fail_closed();
            Err(
                "published-tick journal operation exceeded its hard timeout and is failed closed"
                    .to_string(),
            )
        }
    };
    guard.complete();
    result
}

async fn seal_terminal_ack_with_timeout(
    journal: &PublishedTickJournal,
    input: PublishedTickAckTombstoneInput,
) -> Result<PublishedTickAckTombstone, String> {
    let mut guard = JournalOperationGuard::new(journal);
    let result = match tokio::time::timeout(
        MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
        journal.seal_terminal_ack(input),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            journal.fail_closed();
            Err(
                "published-tick ACK tombstone seal exceeded its hard timeout and is failed closed"
                    .to_string(),
            )
        }
    };
    guard.complete();
    result
}

async fn seal_abandonment_with_timeout(
    journal: &PublishedTickJournal,
    input: PublishedTickAbandonmentTombstoneInput,
) -> Result<PublishedTickAbandonmentTombstone, String> {
    let mut guard = JournalOperationGuard::new(journal);
    let result = match tokio::time::timeout(
        MATCH_ACTOR_WORKER_OPERATION_TIMEOUT,
        journal.seal_abandonment(input),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            journal.fail_closed();
            Err(
                "published-tick abandonment tombstone seal exceeded its hard timeout and is failed closed"
