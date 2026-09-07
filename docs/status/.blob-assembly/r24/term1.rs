#[derive(Debug)]
struct ApiError {
    status: StatusCode,
    body: OnlineAuthorityError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status;
        let mut response = (status, Json(self.body)).into_response();
        if matches!(
            status,
            StatusCode::SERVICE_UNAVAILABLE | StatusCode::TOO_MANY_REQUESTS
        ) {
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, HeaderValue::from_static("1"));
        }
        response
    }
}

struct DurableTerminalCompactionView {
    phase: String,
    simulation_tick: u64,
    simulation_terminal: bool,
    simulation_hash: String,
    durable_snapshot_hash: String,
    authoritative_tick: Option<u64>,
    next_sequence: Option<u64>,
    checkpoint_sequence: Option<u64>,
    match_revision: Option<u64>,
    result_valid: bool,
    settlement_state: String,
    next_input_sequences: BTreeMap<String, u64>,
    assigned_instance_id: Option<String>,
    assigned_instance_epoch: Option<i64>,
    assigned_physical_host_id: Option<String>,
}

fn terminal_result_matches_simulation(
    simulation: &MissionSimV1,
    result_value: Option<Value>,
    result_hash: Option<&str>,
    match_mode: &str,
) -> Result<bool, String> {
    let Some(result_value) = result_value else {
        return Ok(false);
    };
    let result = serde_json::from_value::<BattleResultV1>(result_value)
        .map_err(|error| format!("decode terminal result: {error}"))?;
    let (expected, expected_hash) = derive_terminal_result(simulation, match_mode)?;
    Ok(result == expected && expected_hash == result_hash.unwrap_or_default())
}

fn derive_terminal_result(
    simulation: &MissionSimV1,
    match_mode: &str,
) -> Result<(BattleResultV1, String), String> {
    let mut result = simulation
        .clone()
        .into_result()
        .map_err(|error| format!("derive terminal result: {error}"))?;
    if result.outcome == BattleOutcome::Victory && match_mode != "ranked_pvp" {
        result.resource_delta = result.resource_delta.max(25);
    }
    let result_hash = result
        .computed_hash()
        .map_err(|error| format!("hash terminal result: {error}"))?;
    Ok((result, result_hash))
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

//C0A
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
