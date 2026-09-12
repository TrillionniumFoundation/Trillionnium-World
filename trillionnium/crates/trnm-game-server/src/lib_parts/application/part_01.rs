fn mission_for_map(map_id: &str) -> Result<CampaignMission, ApiError> {
    match map_id {
        "first_contact" => Ok(CampaignMission::FirstContact),
        "iron_delta" => Ok(CampaignMission::IronDeltaSkirmish),
        "night_watch_crossing" => Ok(CampaignMission::NightWatchCrossingSkirmish),
        "glass_basin" => Ok(CampaignMission::GlassBasinSkirmish),
        "ember_orchard" => Ok(CampaignMission::EmberOrchardSkirmish),
        "salt_marsh" => Ok(CampaignMission::SaltMarshSkirmish),
        "cinder_crown" => Ok(CampaignMission::CinderCrownSkirmish),
        _ => Err(api_error(
            StatusCode::BAD_REQUEST,
            "map is not in the Online Authority authored allowlist",
            false,
        )),
    }
}

fn prepare_campaign_seed(
    campaign: &mut CampaignSaveV1,
    map_id: &str,
    map: BattleMapSeedV1,
) -> Result<BattleSeedV1, ApiError> {
    if map_id == "first_contact" {
        campaign
            .move_to(trnm_campaign_core::CampaignRoom::MentorHall)
            .and_then(|_| campaign.talk_to_mentor())
            .and_then(|_| campaign.train_with_mentor())
            .and_then(|_| campaign.equip_starter_weapon())
            .and_then(|_| campaign.move_to(trnm_campaign_core::CampaignRoom::ExpeditionGate))
            .and_then(|_| campaign.accept_first_contact_quest())
            .map_err(|error| api_error(StatusCode::CONFLICT, error.to_string(), false))?;
    } else {
        campaign
            .prepare_standalone_skirmish()
            .map_err(|error| api_error(StatusCode::CONFLICT, error.to_string(), false))?;
        campaign.active_mission = mission_for_map(map_id)?;
    }
    campaign
        .start_first_contact_battle(map)
        .map_err(|error| api_error(StatusCode::CONFLICT, error.to_string(), false))
}

fn member_units(
    seed: &BattleSeedV1,
    member_tag: &str,
    slot_offset: usize,
) -> (
    Vec<trnm_campaign_core::BattleUnitSeedV1>,
    BTreeMap<String, String>,
) {
    let mut id_map = BTreeMap::new();
    let units = seed.party[..2]
        .iter()
        .enumerate()
        .map(|(index, source)| {
            let mut unit = source.clone();
            unit.unit_id = format!("{member_tag}:{}", source.unit_id);
            unit.spawn_slot = format!("party_{}", slot_offset + index);
            id_map.insert(unit.unit_id.clone(), source.unit_id.clone());
            unit
        })
        .collect();
    (units, id_map)
}

fn distributed_admission_bucket_key(identity: &str, method: &str, endpoint_class: &str) -> String {
    format!(
        "{:x}",
        Sha256::digest(format!("{identity}:{method}:{endpoint_class}").as_bytes())
    )
}

fn is_operational_probe_path(path: &str) -> bool {
    matches!(path, "/health" | "/v1/online/readiness")
}

async fn production_rate_limit(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let request_started = Instant::now();
    let path = request.uri().path();
    // Liveness and readiness are infrastructure control-plane probes. Sending
    // them through the durable player admission window adds a database write,
    // lets business traffic starve the probe before its fail-closed checks run,
    // and can feed orchestrator restarts back into a saturated service.
    if is_operational_probe_path(path) {
        return next.run(request).await;
    }
    let command_request = path.ends_with("/commands");
    if command_request {
        // A valid running command commits its distributed admission counter in
        // the same V15 PostgreSQL statement as the command event. Keeping the
        // generic admission write here would force two protocol round trips
        // before the authoritative effect can be streamed.
        let response = next.run(request).await;
        let total_ms = u64::try_from(request_started.elapsed().as_millis()).unwrap_or(u64::MAX);
        if total_ms > 300 {
            tracing::warn!(
                total_ms,
                admission_ms = 0_u64,
                handler_ms = total_ms,
                status = response.status().as_u16(),
                "slow online command request"
            );
        }
        return response;
    }
    let effective_limit = if path.ends_with("/snapshot")
        || path.ends_with("/commands")
        || path.ends_with("/reconnect")
        || path.ends_with("/stream")
    {
        state.rate_limit_per_minute.saturating_mul(20)
    } else {
        state.rate_limit_per_minute
    };
    let identity = request
        .headers()
        .get(PLAYER_SESSION_HEADER)
        .or_else(|| request.headers().get("x-trnm-moderator"))
        .and_then(|value| value.to_str().ok())
        .unwrap_or("anonymous");
    let endpoint_class = path.split('/').take(5).collect::<Vec<_>>().join("/");
    let request_class = if effective_limit == state.rate_limit_per_minute {
        "control"
    } else {
        "data"
    };
    let key =
        distributed_admission_bucket_key(identity, request.method().as_str(), &endpoint_class);
    let admission_started = Instant::now();
    let count = sqlx::query_scalar::query_scalar::<_, i64>(
        "insert into trnm_online_admission_windows (
            bucket_key, window_started_at, request_class, request_count,
            rejection_count, last_instance_id
         ) values ($1, date_trunc('minute', now()), $2, 1, 0, $3)
         on conflict (bucket_key, window_started_at) do update set
            request_count = trnm_online_admission_windows.request_count + 1,
            last_instance_id = excluded.last_instance_id, updated_at = now()
         returning request_count",
    )
    .bind(&key)
    .bind(request_class)
    .bind(state.instance_id.as_str())
    .fetch_one(&state.pool)
    .await;
    let admission_ms = u64::try_from(admission_started.elapsed().as_millis()).unwrap_or(u64::MAX);
    if admission_ms > 300 {
        tracing::warn!(
            admission_ms,
            request_class,
            endpoint_class,
            "slow distributed admission commit"
        );
    }
    let count = match count {
        Ok(value) => value,
        Err(error) => {
            tracing::error!(%error, "distributed admission failed closed");
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": "distributed admission is unavailable"})),
            )
                .into_response();
        }
    };
    if count > i64::from(effective_limit) {
        if let Err(error) = sqlx::query::query(
            "update trnm_online_admission_windows set
                rejection_count = rejection_count + 1, updated_at = now()
             where bucket_key = $1 and window_started_at = date_trunc('minute', now())",
        )
        .bind(&key)
        .execute(&state.pool)
        .await
        {
            tracing::error!(%error, "distributed admission rejection audit failed");
        }
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({
                "error": "production request rate limit exceeded",
                "retry_after_seconds": 60,
            })),
        )
            .into_response();
    }
    next.run(request).await
}

