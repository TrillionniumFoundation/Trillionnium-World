            scheduled_tick.saturating_add(1) as i64,
        view.assigned_instance_epoch = Some(high_water.actor_epoch);
    async fn failed_closed_running_high_water_remains_a_tracked_rollback_witness() {
