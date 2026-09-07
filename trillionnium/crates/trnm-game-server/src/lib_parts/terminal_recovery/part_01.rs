@@TRNM_TERM1_SLOT_0@@
            .try_get("fleet_epoch_current")
            .map_err(|error| error.to_string())?,
        healthy_fleet_instances: row
            .try_get("healthy_fleet_instances")
            .map_err(|error| error.to_string())?,
        active_matches: row
@@TRNM_TERM1_SLOT_1@@
        lock_current_fleet_epoch(&mut transaction, state, true).await?;
    }
    mark_terminal_ack_sealed(&mut *transaction, tombstone).await?;
    if let Some(state) = runtime_state {
        lock_current_fleet_epoch_for_commit(&mut transaction, state, true).await?;
    }
@@TRNM_TERM1_SLOT_2@@
            load_local_terminal_ack_startup_page(pool, physical_host_id, after_match_id).await?;
        if page.is_empty() {
            break;
        }
        let page_len = page.len();
        for evidence in page {
@@TRNM_TERM1_SLOT_3A@@
        .map_err(|_| "abandonment cold witness count exceeds u64".to_string())?;
    let cold_count = u64::try_from(journal.cold_witness_count()?)
        .map_err(|_| "combined cold witness count exceeds u64".to_string())?;
    if !database_summary.all_sealed()
        || database_summary.terminal_total_count != terminal_count
        || database_summary.abandonment_total_count != abandonment_count
@@TRNM_TERM1_SLOT_3B@@
