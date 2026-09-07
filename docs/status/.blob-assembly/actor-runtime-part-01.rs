__TRNM_CHUNK_0__
        initialization.ready.send_replace(true);
        return Err("match actor initialization reservation was replaced".to_string());
    }
    if !match_actor_install_is_allowed(*state.draining.borrow(), *state.shutdown.borrow()) {
        registry.initializing.remove(&match_id);
        reservation.disarm();
        initialization.ready.send_replace(true);
        return Err("game server began draining before match actor installation".to_string());
    }
    registry.initializing.remove(&match_id);
    reservation.disarm();
    match initialized {
        Ok(Some(initialized)) => {
            let handle = initialized.handle.clone();
            registry.actors.insert(match_id, handle.clone());
            initialization.ready.send_replace(true);
            drop(registry);
            let actor_state = state.clone();
            tokio::spawn(async move {
                run_match_actor(actor_state, match_id, initialized).await;
__TRNM_CHUNK_1__
        let high_water = high_water.as_ref().ok_or_else(|| {
            "private terminal stage has no host-journal publication witness".to_string()
        })?;
        if high_water.phase != "complete"
            || high_water.instance_id != *state.instance_id
            || high_water.actor_epoch != state.instance_epoch
            || high_water.physical_host_id != *state.physical_host_id
            || high_water.tick != staged.authoritative_tick
            || high_water.next_sequence != staged.next_sequence
            || high_water.match_revision != staged.match_revision
            || high_water.next_input_sequences != next_input_sequences
            || high_water.snapshot_hash != staged.snapshot_hash
        {
            return Err(
                "private terminal stage does not match its exact terminal HWM witness".to_string(),
            );
        }
        None
    } else if let Some(high_water) = high_water.as_ref() {
        if high_water.phase != "running" {
__TRNM_CHUNK_2__
