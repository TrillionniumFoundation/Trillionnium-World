use reqwest::blocking::Client;
use std::{env, process::ExitCode, time::Duration};
use trnm_online_protocol::{
    OnlineModerationActionRequest, OnlineModerationActionView, OnlineModerationQueueRequest,
    OnlineModerationQueueView,
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
        _ => {
            return Err(
                "usage: trnm-moderation-console list [status] | action REPORT_ID DECISION SCOPE HOURS RESOLUTION..."
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
