use reqwest::blocking::Client;
use std::{env, process::ExitCode, time::Duration};
use trnm_online_protocol::{
    OnlineEnforcementAppealQueueRequest, OnlineEnforcementAppealQueueView,
    OnlineEnforcementAppealResolveRequest, OnlineEnforcementAppealView, OnlineFleetAdminRequest,
    OnlineFleetAdminView, OnlineModerationActionRequest, OnlineModerationActionView,
    OnlineModerationQueueRequest, OnlineModerationQueueView, OnlineSeasonAdminRequest,
    OnlineSeasonAdminView,
};

fn required(key: &str) -> Result<String, String> {
    env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("{key} is required"))
}

fn run() -> Result<(), String> {
    let base_url =
        env::var("TRNM_GAME_SERVER_URL").unwrap_or_else(|_| "http://127.0.0.1:7005".to_string());
    let token = required("TRNM_MODERATOR_TOKEN")?;
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(2))
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|error| error.to_string())?;
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        Some("list") => {
            let status = args.get(1).map(String::as_str).unwrap_or("open");
            let response = client
                .post(format!("{base_url}/v1/operations/moderation/queue"))
                .header("x-trnm-moderator", &token)
                .json(&OnlineModerationQueueRequest {
                    status: status.to_string(),
                    limit: 100,
                })
                .send()
                .map_err(|error| error.to_string())?;
            let status_code = response.status();
            let body = response.text().map_err(|error| error.to_string())?;
            if !status_code.is_success() {
                return Err(format!("moderation queue rejected ({status_code}): {body}"));
            }
            let view: OnlineModerationQueueView =
                serde_json::from_str(&body).map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&view).map_err(|error| error.to_string())?
            );
        }
        Some("action") if args.len() >= 6 => {
            let scope = match args[3].as_str() {
                "none" => None,
                value => Some(value.to_string()),
            };
            let hours = args[4]
                .parse::<u32>()
                .map_err(|_| "suspension hours must be an integer".to_string())?;
            let response = client
                .post(format!("{base_url}/v1/operations/moderation/action"))
                .header("x-trnm-moderator", &token)
                .json(&OnlineModerationActionRequest {
                    report_id: args[1].clone(),
                    decision: args[2].clone(),
                    resolution: args[5..].join(" "),
                    enforcement_scope: scope,
                    suspension_hours: if hours == 0 { None } else { Some(hours) },
                })
                .send()
                .map_err(|error| error.to_string())?;
            let status_code = response.status();
            let body = response.text().map_err(|error| error.to_string())?;
            if !status_code.is_success() {
                return Err(format!(
                    "moderation action rejected ({status_code}): {body}"
                ));
            }
            let view: OnlineModerationActionView =
                serde_json::from_str(&body).map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&view).map_err(|error| error.to_string())?
            );
        }
        Some("appeals") => {
            let status = args.get(1).map(String::as_str).unwrap_or("pending");
            let response = client
                .post(format!("{base_url}/v1/operations/moderation/appeals"))
                .header("x-trnm-moderator", &token)
                .json(&OnlineEnforcementAppealQueueRequest {
                    status: status.to_string(),
                    limit: 100,
                })
                .send()
                .map_err(|error| error.to_string())?;
            let status_code = response.status();
            let body = response.text().map_err(|error| error.to_string())?;
            if !status_code.is_success() {
                return Err(format!("appeal queue rejected ({status_code}): {body}"));
            }
            let view: OnlineEnforcementAppealQueueView =
                serde_json::from_str(&body).map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&view).map_err(|error| error.to_string())?
            );
        }
        Some("appeal") if args.len() >= 4 => {
            let response = client
                .post(format!(
                    "{base_url}/v1/operations/moderation/appeals/resolve"
                ))
                .header("x-trnm-moderator", &token)
                .json(&OnlineEnforcementAppealResolveRequest {
                    appeal_id: args[1].clone(),
                    decision: args[2].clone(),
                    resolution: args[3..].join(" "),
                })
                .send()
                .map_err(|error| error.to_string())?;
            let status_code = response.status();
            let body = response.text().map_err(|error| error.to_string())?;
            if !status_code.is_success() {
                return Err(format!("appeal action rejected ({status_code}): {body}"));
            }
            let view: OnlineEnforcementAppealView =
                serde_json::from_str(&body).map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&view).map_err(|error| error.to_string())?
            );
        }
        Some("season") if args.len() >= 3 => {
            let action = args[1].clone();
            let request = if action == "create" && args.len() == 7 {
                OnlineSeasonAdminRequest {
                    action,
                    season_id: args[2].clone(),
                    display_name: Some(args[3].clone()),
                    rules_version: Some(args[4].clone()),
                    starts_at_epoch: Some(
                        args[5]
                            .parse()
                            .map_err(|_| "season start must be epoch seconds".to_string())?,
                    ),
                    ends_at_epoch: Some(
                        args[6]
                            .parse()
                            .map_err(|_| "season end must be epoch seconds".to_string())?,
                    ),
                }
            } else if matches!(action.as_str(), "activate" | "close") && args.len() == 3 {
                OnlineSeasonAdminRequest {
                    action,
                    season_id: args[2].clone(),
                    display_name: None,
                    rules_version: None,
                    starts_at_epoch: None,
                    ends_at_epoch: None,
                }
            } else {
                return Err("season usage: season create ID NAME RULES START_EPOCH END_EPOCH | season activate ID | season close ID".to_string());
            };
            let response = client
                .post(format!("{base_url}/v1/operations/seasons/admin"))
                .header("x-trnm-moderator", &token)
                .json(&request)
                .send()
                .map_err(|error| error.to_string())?;
            let status_code = response.status();
            let body = response.text().map_err(|error| error.to_string())?;
            if !status_code.is_success() {
                return Err(format!("season action rejected ({status_code}): {body}"));
            }
            let view: OnlineSeasonAdminView =
                serde_json::from_str(&body).map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&view).map_err(|error| error.to_string())?
            );
        }
        Some("fleet") if args.len() >= 4 => {
            let response = client
                .post(format!("{base_url}/v1/operations/fleet/admin"))
                .header("x-trnm-moderator", &token)
                .json(&OnlineFleetAdminRequest {
                    action: args[1].clone(),
                    instance_id: args[2].clone(),
                    reason: args[3..].join(" "),
                })
                .send()
                .map_err(|error| error.to_string())?;
            let status_code = response.status();
            let body = response.text().map_err(|error| error.to_string())?;
            if !status_code.is_success() {
                return Err(format!("fleet action rejected ({status_code}): {body}"));
            }
            let view: OnlineFleetAdminView =
                serde_json::from_str(&body).map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&view).map_err(|error| error.to_string())?
            );
        }
        _ => {
            return Err(
                "usage: trnm-moderation-console list [status] | action REPORT_ID DECISION SCOPE HOURS RESOLUTION... | appeals [status] | appeal APPEAL_ID DECISION RESOLUTION... | season ... | fleet ACTION INSTANCE REASON..."
                    .to_string(),
            );
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
