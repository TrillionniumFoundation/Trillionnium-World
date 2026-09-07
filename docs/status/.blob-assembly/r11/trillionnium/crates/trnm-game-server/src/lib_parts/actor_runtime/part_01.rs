        .filter(|match_id| !bounded_transitions.contains(match_id))
            simulation: loaded.simulation.clone(),
            .map_err(|error| error.to_string())? as u64;
    if computed_hash != expected_hash {
