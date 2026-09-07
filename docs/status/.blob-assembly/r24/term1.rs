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

//C09
}

async fn mark_terminal_ack_sealed<'e, E>(
    executor: E,
    tombstone: &PublishedTickAckTombstone,
) -> Result<(), String>
where
    E: sqlx::executor::Executor<'e, Database = Postgres>,
{
    let high_water = &tombstone.high_water;
    let tick = i64::try_from(high_water.tick)
        .map_err(|_| "terminal tombstone tick exceeds PostgreSQL range".to_string())?;
    let next_sequence = i64::try_from(high_water.next_sequence)
        .map_err(|_| "terminal tombstone sequence exceeds PostgreSQL range".to_string())?;
    let match_revision = i64::try_from(high_water.match_revision)
        .map_err(|_| "terminal tombstone revision exceeds PostgreSQL range".to_string())?;
    let acknowledged_at_unix_ms = i64::try_from(tombstone.acknowledged_at_unix_ms)
        .map_err(|_| "terminal tombstone timestamp exceeds PostgreSQL range".to_string())?;
    let sealed = sqlx::query::query(
        "update trnm_online_terminal_publication_acks a
            set local_tombstone_state = 'sealed'
           from trnm_online_matches m
          where a.match_id = $1 and m.match_id = a.match_id
            and a.local_tombstone_state in (
                'legacy_bootstrap_pending', 'hot_pending', 'sealed'
            )
            and a.actor_generation = $2
            and a.instance_id = $3
            and a.actor_epoch = $4
            and a.physical_host_id = $5
            and a.authoritative_tick = $6
            and a.next_sequence = $7
            and a.match_revision = $8
            and a.next_input_sequences = $9
            and a.snapshot_hash = $10
            and a.phase = 'complete'
            and a.result_hash = $11
            and (
                a.published_settlement_state = $12
                or ($12 = 'pending' and a.published_settlement_state = 'settled')
            )
            and floor(extract(epoch from a.acknowledged_at) * 1000)::bigint = $13
            and m.phase = 'complete'
            and m.checkpoint_sequence = m.next_sequence
            and m.result_hash is not null
            and m.settlement_state in ('pending', 'settled')
            and m.terminal_publication_state = 'acknowledged'
            and m.terminal_publication_actor_generation = a.actor_generation
            and m.assigned_instance_id = a.instance_id
            and m.assigned_instance_epoch = a.actor_epoch
            and m.assigned_physical_host_id = a.physical_host_id
            and m.authoritative_tick = a.authoritative_tick
            and m.next_sequence = a.next_sequence
            and m.match_revision = a.match_revision
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
            and m.snapshot_hash = a.snapshot_hash
            and m.result_hash = a.result_hash
            and m.settlement_state = a.published_settlement_state
        returning a.match_id",
    )
    .bind(high_water.match_id)
    .bind(high_water.actor_generation)
    .bind(&high_water.instance_id)
    .bind(high_water.actor_epoch)
    .bind(&high_water.physical_host_id)
    .bind(tick)
    .bind(next_sequence)
    .bind(match_revision)
    .bind(
        serde_json::to_value(&high_water.next_input_sequences)
            .map_err(|error| error.to_string())?,
    )
    .bind(&high_water.snapshot_hash)
    .bind(&tombstone.result_hash)
    .bind(&tombstone.settlement_state)
    .bind(acknowledged_at_unix_ms)
    .fetch_optional(executor)
    .await
    .map_err(|error| error.to_string())?;
    if sealed.is_none() {
        return Err(
            "cold terminal tombstone could not seal its exact durable ACK authority".to_string(),
        );
    }
    Ok(())
}
//C0B
}

//C0C
}

//C0D
}

//C0E
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
