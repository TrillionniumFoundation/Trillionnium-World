#[derive(Debug)]
    Ok(TerminalAckDatabaseEvidence {
            .try_get("journal_owner_id")
            acknowledged_at_unix_ms: commit.acknowledged_at_unix_ms,
        high_water,
            reconciled.terminal_acknowledged.insert(evidence.match_id);
    let database_summary =
                .await?)
        }
        PublishedTickColdWitness::FailedClosedAbandonment(tombstone) => {
            let Some(evidence) = load_abandonment_database_evidence_by_match(
                &mut *connection,
                tombstone.high_water.match_id,
            )
            .await?
            else {
                return Ok(false);
            };
            Ok(evidence.local_tombstone_state == "sealed"
                && abandonment_tombstone_matches_evidence(&tombstone, &evidence, &lineage))
        }
    }
}

async fn campaign_has_unacknowledged_progression<'e, E>(
    executor: E,
    campaign_id: &str,
) -> Result<bool, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    sqlx::query_scalar::query_scalar(CAMPAIGN_HAS_UNACKNOWLEDGED_PROGRESSION_SQL)
        .bind(campaign_id)
        .fetch_one(executor)
        .await
        .map_err(|error| error.to_string())
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct HistoricalTerminalProjectionQuarantine {
    legacy_terminal_match_count: i64,
    campaign_projection_polluted: bool,
    rating_projection_polluted: bool,
}

impl HistoricalTerminalProjectionQuarantine {
    fn public_credit_is_clean(self) -> bool {
        self.legacy_terminal_match_count == 0
            && !self.campaign_projection_polluted
            && !self.rating_projection_polluted
    }
}

async fn terminal_publication_marker_matches_high_water<'e, E>(
    executor: E,
    high_water: &PublishedTickHighWater,
    durable_result_hash: &str,
    durable_settlement_state: &str,
) -> Result<bool, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    sqlx::query_scalar::query_scalar(
        "select exists(
            select 1
              from trnm_online_terminal_publication_acks a
             where a.match_id = $1
               and a.actor_generation = $2
               and a.actor_epoch = $3
               and a.authoritative_tick = $4
               and a.next_sequence = $5
               and a.match_revision = $6
        .copied()
