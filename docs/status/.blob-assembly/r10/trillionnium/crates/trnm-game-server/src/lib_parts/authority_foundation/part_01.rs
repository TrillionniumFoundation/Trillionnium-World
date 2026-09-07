__TRNM_SLOT_0__
const TERMINAL_ACK_GAP_SCAN_LIMIT: i64 = 256;
const TERMINAL_ACK_STARTUP_PAGE_SIZE: i64 = 512;
const ONLINE_COMMAND_COMMIT_V2_SQL: &str =
    "select * from public.trnm_online_commit_actor_command_v2(
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
            $21, $22, $23, $24, $25, $26, $27
         )";
const ONLINE_HEARTBEAT_FLEET_V1_SQL: &str = "select public.trnm_online_heartbeat_fleet_v1(
            $1, $2, $3, $4, $5, $6, $7, $8,
            $9, $10, $11, $12, $13, $14, $15, $16
         )";
const ONLINE_CHECKPOINT_ACTOR_V1_SQL: &str = "select public.trnm_online_checkpoint_actor_v1(
            $1, $2, $3, $4, $5, $6, $7, $8, $9,
            $10, $11, $12, $13, $14, $15, $16, $17, $18
         )";
const MATCH_ACTOR_FENCE_OWNERSHIP_SQL: &str = "select exists(
        select 1 from trnm_online_matches m
        join trnm_online_fleet_instances f
          on f.instance_id = m.assigned_instance_id
         and f.instance_epoch = m.assigned_instance_epoch
        where m.match_id = $1
          and m.assigned_instance_id = $2
          and m.assigned_instance_epoch = $3
          and m.assigned_physical_host_id = $4
          and f.physical_host_id = $4
          and f.status in ('active', 'draining')
          and f.lease_expires_at > now()
    )";
const DUPLICATE_COMMAND_RECEIPT_SQL: &str = "select
        c.sequence, c.input_sequence, c.player_id, c.request_hash,
        c.accepted_match_revision, c.accepted_snapshot_hash, c.target_tick,
        c.client_observed_tick,
        m.match_revision as current_match_revision,
        m.next_sequence as current_next_sequence,
        m.checkpoint_sequence, m.phase, m.assigned_physical_host_id,
        exists(
            select 1 from trnm_online_terminal_publication_acks a
            where a.match_id = m.match_id
              and m.phase = 'complete'
              and m.terminal_publication_state = 'acknowledged'
              and a.local_tombstone_state = 'sealed'
              and m.assigned_physical_host_id = $3
              and m.terminal_publication_actor_generation is not null
              and a.actor_generation = m.terminal_publication_actor_generation
              and a.instance_id = m.assigned_instance_id
              and a.actor_epoch = m.assigned_instance_epoch
              and a.physical_host_id = m.assigned_physical_host_id
              and a.authoritative_tick = m.authoritative_tick
              and a.next_sequence = m.next_sequence
              and a.match_revision = m.match_revision
              and a.snapshot_hash = m.snapshot_hash
              and a.phase = 'complete'
              and a.result_hash = m.result_hash
              and a.published_settlement_state = m.settlement_state
              and a.next_input_sequences = coalesce(
                  (select jsonb_object_agg(
                      mm.player_id,
                      mm.next_input_sequence order by mm.player_id
                   )
                   from trnm_online_match_members mm
                   where mm.match_id = m.match_id),
                  '{}'::jsonb
              )
        ) as terminal_publication_acked,
        (select mm.next_input_sequence from trnm_online_match_members mm
         where mm.match_id = c.match_id and mm.player_id = c.player_id)
           as current_member_input_sequence
 from trnm_online_commands c
 join trnm_online_matches m on m.match_id = c.match_id
 where c.match_id = $1 and c.command_id = $2";
const DATABASE_LINEAGE_SQL: &str = "select
        system.system_identifier::text as system_identifier,
        checkpoint.timeline_id::integer as timeline_id,
        pg_current_wal_flush_lsn()::text as wal_flush_lsn
      from pg_control_system() system
      cross join pg_control_checkpoint() checkpoint";
const LOCAL_TERMINAL_ACK_GAPS_SQL: &str = "select m.match_id
       from trnm_online_matches m
      where m.phase = 'complete'
        and m.assigned_physical_host_id = $1
        and m.terminal_publication_state <> 'legacy_quarantined'
        and not exists (
            select 1
              from trnm_online_terminal_publication_acks a
             where a.match_id = m.match_id
               and m.checkpoint_sequence = m.next_sequence
               and m.result_hash is not null
               and m.settlement_state in ('pending', 'settled')
               and m.terminal_publication_state = 'acknowledged'
               and a.local_tombstone_state = 'sealed'
               and m.terminal_publication_actor_generation is not null
               and a.actor_generation = m.terminal_publication_actor_generation
               and a.instance_id = m.assigned_instance_id
               and a.actor_epoch = m.assigned_instance_epoch
               and a.physical_host_id = m.assigned_physical_host_id
               and a.authoritative_tick = m.authoritative_tick
               and a.next_sequence = m.next_sequence
               and a.match_revision = m.match_revision
               and a.next_input_sequences = (
                   select coalesce(
                       jsonb_object_agg(
                           mm.player_id,
                           to_jsonb(mm.next_input_sequence)
                           order by mm.player_id
                       ),
                       '{}'::jsonb
                   )
                     from trnm_online_match_members mm
                    where mm.match_id = m.match_id
               )
               and a.snapshot_hash = m.snapshot_hash
               and a.phase = 'complete'
               and a.result_hash = m.result_hash
               and a.published_settlement_state = m.settlement_state
        )
      order by m.match_id
      limit $2";
const EXACT_TERMINAL_PUBLICATION_MARKER_SQL: &str = "select exists(
        select 1
          from trnm_online_terminal_publication_acks a
          join trnm_online_matches m on m.match_id = a.match_id
         where a.match_id = $1
           and m.phase = 'complete'
           and m.checkpoint_sequence = m.next_sequence
           and m.result_hash is not null
           and m.settlement_state in ('pending', 'settled')
           and m.terminal_publication_state = 'acknowledged'
           and a.local_tombstone_state = 'sealed'
           and m.terminal_publication_actor_generation is not null
           and a.actor_generation = m.terminal_publication_actor_generation
           and a.instance_id = m.assigned_instance_id
           and a.actor_epoch = m.assigned_instance_epoch
           and a.physical_host_id = m.assigned_physical_host_id
           and a.authoritative_tick = m.authoritative_tick
           and a.next_sequence = m.next_sequence
           and a.match_revision = m.match_revision
           and a.snapshot_hash = m.snapshot_hash
           and a.phase = 'complete'
           and a.result_hash = m.result_hash
           and a.published_settlement_state = m.settlement_state
           and a.next_input_sequences = coalesce(
               (select jsonb_object_agg(
                   mm.player_id,
                   mm.next_input_sequence order by mm.player_id
                )
                  from trnm_online_match_members mm
                 where mm.match_id = m.match_id),
               '{}'::jsonb
           )
    )";
const CAMPAIGN_HAS_UNACKNOWLEDGED_PROGRESSION_SQL: &str = "select exists(
        select 1
          from trnm_online_progression_events event
         where event.campaign_id = $1
           and not exists (
               select 1
                 from trnm_online_matches m
                 join trnm_online_terminal_publication_acks a on a.match_id = m.match_id
                where m.match_id = event.match_id
                  and m.phase = 'complete'
                  and m.checkpoint_sequence = m.next_sequence
                  and m.result_hash = event.result_hash
                  and m.settlement_state in ('pending', 'settled')
                  and m.terminal_publication_state = 'acknowledged'
                  and a.local_tombstone_state = 'sealed'
                  and m.terminal_publication_actor_generation is not null
                  and a.actor_generation = m.terminal_publication_actor_generation
                  and a.instance_id = m.assigned_instance_id
                  and a.actor_epoch = m.assigned_instance_epoch
                  and a.physical_host_id = m.assigned_physical_host_id
                  and a.authoritative_tick = m.authoritative_tick
                  and a.next_sequence = m.next_sequence
                  and a.match_revision = m.match_revision
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
                  and a.snapshot_hash = m.snapshot_hash
                  and a.phase = 'complete'
                  and a.result_hash = m.result_hash
                  and a.published_settlement_state = m.settlement_state
           )
    )";
const READINESS_DATABASE_SUMMARY_SQL: &str = "with terminal_ack_gaps as materialized (
        select m.match_id
          from trnm_online_matches m
         where m.phase = 'complete'
           and m.assigned_physical_host_id = $3
           and m.terminal_publication_state <> 'legacy_quarantined'
           and not exists (
               select 1
                 from trnm_online_terminal_publication_acks a
                where a.match_id = m.match_id
                  and m.checkpoint_sequence = m.next_sequence
                  and m.result_hash is not null
                  and m.settlement_state in ('pending', 'settled')
                  and m.terminal_publication_state = 'acknowledged'
                  and a.local_tombstone_state = 'sealed'
                  and m.terminal_publication_actor_generation is not null
                  and a.actor_generation = m.terminal_publication_actor_generation
                  and a.instance_id = m.assigned_instance_id
                  and a.actor_epoch = m.assigned_instance_epoch
                  and a.physical_host_id = m.assigned_physical_host_id
                  and a.authoritative_tick = m.authoritative_tick
                  and a.next_sequence = m.next_sequence
                  and a.match_revision = m.match_revision
                  and a.next_input_sequences = (
                      select coalesce(
                          jsonb_object_agg(
                              mm.player_id,
                              to_jsonb(mm.next_input_sequence)
                              order by mm.player_id
                          ),
                          '{}'::jsonb
                      )
                        from trnm_online_match_members mm
                       where mm.match_id = m.match_id
                  )
                  and a.snapshot_hash = m.snapshot_hash
                  and a.phase = 'complete'
                  and a.result_hash = m.result_hash
                  and a.published_settlement_state = m.settlement_state
           )
         order by m.match_id
         limit $4
    ), pending_terminal_seals as materialized (
        select a.match_id
          from trnm_online_terminal_publication_acks a
         where a.physical_host_id = $3
           and a.local_tombstone_state <> 'sealed'
         order by a.match_id
         limit $4
    ), pending_abandonment_seals as materialized (
        select marker.match_id
          from trnm_online_failed_closed_abandonment_markers marker
         where marker.physical_host_id = $3
           and marker.local_tombstone_state <> 'sealed'
         order by marker.match_id
         limit $4
    ), historical_projection as materialized (
        select
            (select count(*) from trnm_online_matches
              where terminal_publication_state = 'legacy_quarantined')
                as legacy_terminal_match_count,
            exists(
                select 1 from trnm_online_progression_events event
                join trnm_online_matches m on m.match_id = event.match_id
                where m.terminal_publication_state = 'legacy_quarantined'
            ) as campaign_projection_polluted,
            exists(
                select 1 from trnm_online_rating_events event
                join trnm_online_matches m on m.match_id = event.match_id
                where m.terminal_publication_state = 'legacy_quarantined'
            ) as rating_projection_polluted
    )
    select
        true as postgres_healthy,
        exists(
            select 1 from trnm_online_fleet_instances
             where instance_id = $1 and instance_epoch = $2
               and physical_host_id = $3
               and status in ('active', 'draining') and lease_expires_at > now()
        ) as fleet_epoch_current,
        (select count(*) from trnm_online_fleet_instances
          where status = 'active' and lease_expires_at > now())
            as healthy_fleet_instances,
        (select count(*) from trnm_online_matches
          where phase = 'running' and assigned_instance_id = $1
            and assigned_instance_epoch = $2 and assigned_physical_host_id = $3)
            as active_matches,
        (select count(*) from terminal_ack_gaps) as terminal_ack_gap_count,
        coalesce(
            (select array_agg(match_id order by match_id) from terminal_ack_gaps),
            array[]::uuid[]
        ) as terminal_ack_gap_match_ids,
        coalesce(
            (select array_agg(match_id order by match_id) from pending_terminal_seals),
            array[]::uuid[]
        ) as pending_terminal_seal_match_ids,
        coalesce(
            (select array_agg(match_id order by match_id) from pending_abandonment_seals),
            array[]::uuid[]
        ) as pending_abandonment_seal_match_ids,
        historical_projection.legacy_terminal_match_count,
        historical_projection.campaign_projection_polluted,
        historical_projection.rating_projection_polluted,
        coalesce(cold.terminal_total_count, 0)::bigint as terminal_total_count,
        coalesce(cold.terminal_sealed_count, 0)::bigint as terminal_sealed_count,
__TRNM_SLOT_2__
__TRNM_SLOT_3__
