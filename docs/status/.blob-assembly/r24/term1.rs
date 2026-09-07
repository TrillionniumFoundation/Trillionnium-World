//C00
}

//C01
}

//C02
}

//C03
        acknowledged_at_unix_ms,
    })
//C04
}

//C05
    }
    Ok(ReadinessDatabaseSummary {
//C06
}

//C07
        );
    }
//C08
}

async fn load_terminal_ack_database_evidence_by_match<'e, E>(
    executor: E,
    match_id: Uuid,
) -> Result<Option<TerminalAckDatabaseEvidence>, String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    let Some(row) = sqlx::query::query(TERMINAL_ACK_DATABASE_EVIDENCE_BY_MATCH_SQL)
        .bind(match_id)
        .fetch_optional(executor)
        .await
        .map_err(|error| error.to_string())?
    else {
        return Ok(None);
    };
    let tuple_exact = row
        .try_get::<bool, _>("tuple_exact")
        .map_err(|error| error.to_string())?;
    let evidence = terminal_ack_database_evidence_from_row(&row)?;
    if !tuple_exact {
        return Err(format!(
            "terminal ACK {} does not exactly match its durable match authority",
            evidence.match_id
        ));
    }
    Ok(Some(evidence))
}

async fn load_local_terminal_ack_startup_page(
    pool: &PgPool,
    physical_host_id: &str,
    after_match_id: Option<Uuid>,
) -> Result<Vec<TerminalAckDatabaseEvidence>, String> {
    let rows = sqlx::query::query(LOCAL_TERMINAL_ACK_STARTUP_PAGE_SQL)
        .bind(physical_host_id)
        .bind(after_match_id)
        .bind(TERMINAL_ACK_STARTUP_PAGE_SIZE)
        .fetch_all(pool)
        .await
        .map_err(|error| error.to_string())?;
    let mut evidence = Vec::with_capacity(rows.len());
    for row in rows {
        let tuple_exact = row
            .try_get::<bool, _>("tuple_exact")
            .map_err(|error| error.to_string())?;
        let item = terminal_ack_database_evidence_from_row(&row)?;
        if !tuple_exact {
            return Err(format!(
                "terminal ACK {} does not exactly match its durable match authority",
                item.match_id
            ));
        }
        evidence.push(item);
    }
    Ok(evidence)
}

//C0A
    Ok(())
}
//C0B
}

//C0C
}

//C0D
}

async fn mark_abandonment_sealed<'e, E>(
    executor: E,
    tombstone: &PublishedTickAbandonmentTombstone,
) -> Result<(), String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    let high_water = &tombstone.high_water;
    let tick = i64::try_from(high_water.tick)
        .map_err(|_| "abandonment tombstone tick exceeds PostgreSQL range".to_string())?;
    let next_sequence = i64::try_from(high_water.next_sequence)
        .map_err(|_| "abandonment tombstone sequence exceeds PostgreSQL range".to_string())?;
    let match_revision = i64::try_from(high_water.match_revision)
        .map_err(|_| "abandonment tombstone revision exceeds PostgreSQL range".to_string())?;
    let abandoned_at_unix_ms = i64::try_from(tombstone.abandoned_at_unix_ms)
        .map_err(|_| "abandonment tombstone timestamp exceeds PostgreSQL range".to_string())?;
    let sealed = sqlx::query::query(
        "update trnm_online_failed_closed_abandonment_markers a
            set local_tombstone_state = 'sealed'
           from trnm_online_matches m
          where a.match_id = $1 and m.match_id = a.match_id
            and a.local_tombstone_state in ('hot_pending', 'sealed')
            and a.journal_owner_id = $2
            and a.actor_generation = $3
            and a.instance_id = $4
            and a.actor_epoch = $5
            and a.physical_host_id = $6
            and a.authoritative_tick = $7
            and a.next_sequence = $8
            and a.match_revision = $9
            and a.next_input_sequences = $10
            and a.snapshot_hash = $11
            and a.failure_reason = $12
            and floor(extract(epoch from a.abandoned_at) * 1000)::bigint = $13
            and m.phase = 'failed_closed'
            and m.settlement_state = 'failed_closed'
            and m.terminal_publication_state = 'pending'
            and m.checkpoint_sequence = m.next_sequence
            and m.terminal_stage_simulation_json is null
            and m.terminal_stage_result_json is null
            and m.terminal_stage_result_hash is null
            and m.terminal_stage_snapshot_hash is null
            and m.terminal_stage_authoritative_tick is null
            and m.terminal_stage_next_sequence is null
            and m.terminal_stage_match_revision is null
            and m.terminal_staged_at is null
            and m.result_json is null
            and m.result_hash is null
            and m.terminal_publication_actor_generation is null
            and m.failure_reason = a.failure_reason
            and m.assigned_instance_id = a.instance_id
            and m.assigned_instance_epoch = a.actor_epoch
            and m.assigned_physical_host_id = a.physical_host_id
            and m.authoritative_tick = a.authoritative_tick
            and m.next_sequence = a.next_sequence
            and m.match_revision = a.match_revision
            and m.snapshot_hash = a.snapshot_hash
            and a.next_input_sequences = coalesce(
                (select jsonb_object_agg(
                    member.player_id,
                    to_jsonb(member.next_input_sequence)
                    order by member.player_id
                 )
                   from trnm_online_match_members member
                  where member.match_id = m.match_id),
                '{}'::jsonb
            )
            and not exists (
                select 1 from trnm_online_terminal_publication_acks terminal
                 where terminal.match_id = m.match_id
            )
        returning a.match_id",
    )
//C0F
}

//C10
    }

//C11
                }
            }
//C12
            break;
        }
//C13
        }
    }
//C14
    }

//C15
}

//C16
}

//C17
