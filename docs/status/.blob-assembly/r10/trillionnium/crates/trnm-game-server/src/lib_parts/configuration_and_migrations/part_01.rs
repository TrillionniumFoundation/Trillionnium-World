__TRNM_SLOT_0__
                .try_get::<String, _>("migration_name")
                .map_err(|error| error.to_string())?;
            let recorded_checksum = recorded
                .try_get::<String, _>("checksum_sha256")
                .map_err(|error| error.to_string())?;
            migration_ledger_entry_is_applied(
                version,
                label,
                &checksum,
                Some((&recorded_name, &recorded_checksum)),
            )?;
            continue;
        }
        sqlx::raw_sql::raw_sql(sql)
            .execute(&mut *migrations)
            .await
            .map_err(|error| format!("migrate V{version} {label} PostgreSQL: {error}"))?;
        sqlx::query::query(
            "insert into public.trnm_online_schema_migrations (
                migration_version, migration_name, checksum_sha256
             ) values ($1, $2, $3)",
        )
        .bind(version)
        .bind(label)
        .bind(&checksum)
        .execute(&mut *migrations)
        .await
        .map_err(|error| format!("record migration V{version} {label}: {error}"))?;
    }
    migrations
        .commit()
        .await
        .map_err(|error| format!("commit Online Production migrations: {error}"))
}

async fn publish_database_host_authority_identity(
    connection: &mut PgConnection,
    identity: &DatabaseHostAuthorityIdentity,
) -> Result<(), String> {
    sqlx::query::query(
        "insert into trnm_online_physical_host_authorities (
            physical_host_id, owner_nonce, application_name, backend_pid,
            backend_started_at, database_system_identifier, database_timeline_id,
            database_postmaster_started_at, leader_lock_key, barrier_lock_key, claimed_at
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, now())
         on conflict (physical_host_id) do update set
            owner_nonce = excluded.owner_nonce,
            application_name = excluded.application_name,
            backend_pid = excluded.backend_pid,
            backend_started_at = excluded.backend_started_at,
            database_system_identifier = excluded.database_system_identifier,
            database_timeline_id = excluded.database_timeline_id,
            database_postmaster_started_at = excluded.database_postmaster_started_at,
            leader_lock_key = excluded.leader_lock_key,
            barrier_lock_key = excluded.barrier_lock_key,
            claimed_at = now()",
    )
    .bind(&identity.physical_host_id)
    .bind(identity.owner_nonce)
    .bind(&identity.application_name)
    .bind(identity.backend_pid)
    .bind(identity.backend_started_at)
    .bind(&identity.database_system_identifier)
    .bind(identity.database_timeline_id)
    .bind(identity.database_postmaster_started_at)
    .bind(identity.leader_lock_key)
    .bind(identity.barrier_lock_key)
    .execute(&mut *connection)
    .await
    .map_err(|error| format!("publish PostgreSQL host-authority identity: {error}"))?;
    Ok(())
}

async fn bootstrap_database_host_authority(
    database_url: &str,
    physical_host_id: &str,
) -> Result<Arc<DatabaseHostAuthorityFence>, String> {
    let mut connection = PgConnection::connect(database_url)
        .await
        .map_err(|error| format!("connect PostgreSQL host-authority session: {error}"))?;
    configure_database_connection(&mut connection)
        .await
        .map_err(|error| format!("configure PostgreSQL host-authority session: {error}"))?;
    let owner_nonce = Uuid::new_v4();
    let application_name = format!("trnm-host-fence:{owner_nonce}");
    sqlx::query::query("select set_config('application_name', $1, false)")
        .bind(&application_name)
        .execute(&mut connection)
        .await
        .map_err(|error| format!("name PostgreSQL host-authority session: {error}"))?;
    let identity = read_database_host_authority_identity(
        &mut connection,
        physical_host_id,
        owner_nonce,
        application_name,
    )
    .await?;
    let acquired_leader =
        sqlx::query_scalar::query_scalar::<_, bool>("select pg_try_advisory_lock($1)")
            .bind(identity.leader_lock_key)
            .fetch_one(&mut connection)
            .await
            .map_err(|error| format!("acquire PostgreSQL host leader lock: {error}"))?;
    if !acquired_leader {
        return Err(format!(
            "physical host {} already has a PostgreSQL authority leader",
            identity.physical_host_id
        ));
    }
    tokio::time::timeout(DATABASE_HOST_HANDOFF_TIMEOUT, async {
        loop {
            let acquired =
                sqlx::query_scalar::query_scalar::<_, bool>("select pg_try_advisory_lock($1)")
                    .bind(identity.barrier_lock_key)
                    .fetch_one(&mut connection)
                    .await
                    .map_err(|error| format!("acquire PostgreSQL host handoff barrier: {error}"))?;
            if acquired {
                return Ok::<(), String>(());
            }
            tokio::time::sleep(DATABASE_HOST_HANDOFF_POLL_INTERVAL).await;
        }
    })
    .await
    .map_err(|_| {
        format!(
            "PostgreSQL host handoff for {} exceeded {} seconds",
            identity.physical_host_id,
            DATABASE_HOST_HANDOFF_TIMEOUT.as_secs()
        )
    })??;
    run_database_migrations(&mut connection).await?;
    publish_database_host_authority_identity(&mut connection, &identity).await?;
    let released_barrier =
        sqlx::query_scalar::query_scalar::<_, bool>("select pg_advisory_unlock($1)")
            .bind(identity.barrier_lock_key)
            .fetch_one(&mut connection)
            .await
            .map_err(|error| format!("release PostgreSQL host handoff barrier: {error}"))?;
    if !released_barrier {
        return Err(
            "PostgreSQL host handoff barrier was not owned by bootstrap session".to_string(),
        );
    }
    Ok(Arc::new(DatabaseHostAuthorityFence {
        identity,
        connection: AsyncMutex::new(connection),
        healthy: AtomicBool::new(true),
        last_success: Mutex::new(Instant::now()),
    }))
}

async fn database_host_authority_is_exact(
    connection: &mut PgConnection,
    identity: &DatabaseHostAuthorityIdentity,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::query_scalar(
        "select exists (
            select 1
              from trnm_online_physical_host_authorities authority
              join pg_stat_activity activity on activity.pid = authority.backend_pid
              join pg_locks authority_lock on authority_lock.pid = authority.backend_pid
             where authority.physical_host_id = $1
               and authority.owner_nonce = $2
               and authority.application_name = $3
               and authority.backend_pid = $4
               and authority.backend_started_at = $5
               and authority.database_system_identifier = $6
               and authority.database_timeline_id = $7
               and authority.database_postmaster_started_at = $8
               and authority.leader_lock_key = $9
               and authority.barrier_lock_key = $10
               and activity.datname = current_database()
               and activity.backend_start = $5
               and activity.application_name = $3
               and authority_lock.locktype = 'advisory'
               and authority_lock.database = (
                   select oid from pg_database where datname = current_database()
               )
               and authority_lock.classid::bigint = (
                   ($9::bigint >> 32) & 4294967295::bigint
               )
               and authority_lock.objid::bigint = (
                   $9::bigint & 4294967295::bigint
               )
               and authority_lock.objsubid = 1
               and authority_lock.mode = 'ExclusiveLock'
               and authority_lock.granted
               and not pg_is_in_recovery()
               and (select system_identifier::text from pg_control_system()) = $6
               and (select timeline_id::bigint from pg_control_checkpoint()) = $7
               and pg_postmaster_start_time() = $8
        )",
    )
    .bind(&identity.physical_host_id)
    .bind(identity.owner_nonce)
    .bind(&identity.application_name)
    .bind(identity.backend_pid)
    .bind(identity.backend_started_at)
    .bind(&identity.database_system_identifier)
    .bind(identity.database_timeline_id)
    .bind(identity.database_postmaster_started_at)
    .bind(identity.leader_lock_key)
    .bind(identity.barrier_lock_key)
    .fetch_one(&mut *connection)
    .await
}

async fn admit_database_pool_connection(
    connection: &mut PgConnection,
    fence: &DatabaseHostAuthorityFence,
) -> Result<(), sqlx::Error> {
    configure_database_connection(connection).await?;
    if !fence.is_healthy() || !database_host_authority_is_exact(connection, &fence.identity).await?
    {
        return Err(sqlx::Error::Protocol(
            "PostgreSQL host authority is not exact before pool admission".to_string(),
        ));
    }
    sqlx::query::query("select pg_advisory_lock_shared($1)")
        .bind(fence.identity.barrier_lock_key)
        .execute(&mut *connection)
        .await?;
__TRNM_SLOT_2__
__TRNM_SLOT_3__
