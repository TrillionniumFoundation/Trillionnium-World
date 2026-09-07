__TRNM_CHUNK_0__
                    .terminal_acknowledged_high_waters
                    .write()
                    .await
                    .remove(&match_id);
                state
                    .failed_closed_high_waters
                    .write()
                    .await
__TRNM_CHUNK_1__
        initialization.ready.send_replace(true);
        return Err("match actor initialization reservation was replaced".to_string());
    }
    if !match_actor_install_is_allowed(*state.draining.borrow(), *state.shutdown.borrow()) {
        registry.initializing.remove(&match_id);
        reservation.disarm();
        initialization.ready.send_replace(true);
        return Err("game server began draining before match actor installation".to_string());
__TRNM_CHUNK_2__
             from trnm_online_matches
             where match_id = $1 for update",
    )
    .bind(match_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| error.to_string())?
    else {
__TRNM_CHUNK_3__
        let high_water = high_water.as_ref().ok_or_else(|| {
            "private terminal stage has no host-journal publication witness".to_string()
        })?;
        if high_water.phase != "complete"
            || high_water.instance_id != *state.instance_id
            || high_water.actor_epoch != state.instance_epoch
            || high_water.physical_host_id != *state.physical_host_id
            || high_water.tick != staged.authoritative_tick
__TRNM_CHUNK_4__
                "{label} realtime accepted/observed/order ticks do not match its post-command state"
            ));
        }
    } else if target_tick < simulation.tick
        || target_tick > simulation.tick.saturating_add(200)
        || order_tick != target_tick
    {
        return Err(format!(
__TRNM_CHUNK_5__
