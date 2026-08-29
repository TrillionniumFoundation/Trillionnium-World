use std::{
    env, fs,
    path::{Path, PathBuf},
};

const LIB_HEADER: &str = r#"#![recursion_limit = "512"]

mod cex;
mod map;
mod operations_v1;
mod product_v2;
mod production_v1;
mod published_tick_journal;
pub mod signer_protocol;
mod stream;

"#;

fn fail(message: impl AsRef<str>) -> ! {
    panic!(
        "WORLD-P0 source transform failed closed: {}",
        message.as_ref()
    );
}

fn replace_once(source: &mut String, old: &str, new: &str, label: &str) {
    let count = source.matches(old).count();
    match count {
        1 => *source = source.replacen(old, new, 1),
        0 if source.contains(new) => {}
        _ => fail(format!(
            "{label}: expected one reviewed source shape, found {count}"
        )),
    }
}

fn replace_all_at_least(source: &mut String, old: &str, new: &str, minimum: usize, label: &str) {
    let count = source.matches(old).count();
    if count < minimum {
        fail(format!(
            "{label}: expected at least {minimum} reviewed source shapes, found {count}"
        ));
    }
    *source = source.replace(old, new);
}

fn rewrite_migration_includes(source: &str) -> String {
    let needle = "include_str!(\"../migrations/";
    source
        .lines()
        .map(|line| {
            let Some(start) = line.find(needle) else {
                return line.to_string();
            };
            let path_start = start + needle.len();
            let Some(end_offset) = line[path_start..].find("\");") else {
                fail(format!("malformed migration include: {line}"));
            };
            let path_end = path_start + end_offset;
            let relative = &line[path_start..path_end];
            format!(
                "{}include_str!(concat!(::std::env!(\"CARGO_MANIFEST_DIR\"), \"/migrations/{}\"));{}",
                &line[..start],
                relative,
                &line[path_end + 3..]
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn generate_game_server(out_dir: &Path) {
    let template_path = PathBuf::from("src/lib.rs.in");
    let mut source = fs::read_to_string(&template_path)
        .unwrap_or_else(|error| fail(format!("read {}: {error}", template_path.display())));

    replace_once(
        &mut source,
        "const MIGRATION_V15: &str = include_str!(\"../migrations/0015_online_realtime_hot_path_v1.sql\");",
        r#"const MIGRATION_V15: &str = include_str!("../migrations/0015_online_realtime_hot_path_v1.sql");
const MIGRATION_V16: &str = include_str!("../migrations/0016_online_settlement_outbox_v1.sql");
const MIGRATION_V17: &str =
    include_str!("../migrations/0017_online_settlement_worker_runtime_v1.sql");
const MIGRATION_V18: &str =
    include_str!("../migrations/0018_online_settlement_operator_controls_v1.sql");
const MIGRATION_V19: &str =
    include_str!("../migrations/0019_online_settlement_quarantine_v1.sql");"#,
        "game-server migration constants",
    );
    replace_once(
        &mut source,
        "        (15, \"0015_online_realtime_hot_path_v1\", MIGRATION_V15),\n",
        r#"        (15, "0015_online_realtime_hot_path_v1", MIGRATION_V15),
        (16, "0016_online_settlement_outbox_v1", MIGRATION_V16),
        (
            17,
            "0017_online_settlement_worker_runtime_v1",
            MIGRATION_V17,
        ),
        (
            18,
            "0018_online_settlement_operator_controls_v1",
            MIGRATION_V18,
        ),
        (
            19,
            "0019_online_settlement_quarantine_v1",
            MIGRATION_V19,
        ),
"#,
        "game-server migration ledger",
    );

    let legacy_loop = r#"    let settlement_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(500));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            if let Err(error) = settle_pending_matches(&settlement_state, 2).await {
                tracing::error!(%error, "online authority settlement remains pending");
            }
        }
    });

"#;
    if source.contains(legacy_loop) {
        source = source.replacen(legacy_loop, "", 1);
    } else if source.contains("settle_pending_matches(&settlement_state") {
        fail("legacy settlement loop drifted from the reviewed source shape");
    }

    let legacy_signature =
        "pub async fn settle_pending_matches(state: &AppState, limit: i64) -> Result<u64, String> {";
    let fail_closed_signature =
        "pub async fn settle_pending_matches(_state: &AppState, _limit: i64) -> Result<u64, String> {";
    if let Some(start) = source.find(legacy_signature) {
        let end_marker = "\nasync fn persist_campaign(\n";
        let Some(relative_end) = source[start..].find(end_marker) else {
            fail("cannot find reviewed end of legacy settlement function");
        };
        let end = start + relative_end;
        let replacement = r#"/// Compatibility API retained only to fail closed for downstream callers.
///
/// Terminal economic settlement is owned by the independently deployed
/// `trnm-settlement-worker`. The game-server process must never execute signer
/// or CEX I/O, mutate campaign economic queues, or advance the terminal
/// settlement marker itself.
pub async fn settle_pending_matches(_state: &AppState, _limit: i64) -> Result<u64, String> {
    Err(
        "terminal settlement is owned by trnm-settlement-worker; in-process settlement is prohibited"
            .to_string(),
    )
}
"#;
        source.replace_range(start..end, replacement);
    } else if !source.contains(fail_closed_signature) {
        fail("legacy settlement function is neither reviewed legacy nor fail-closed form");
    }

    if source.contains("reconcile_economy(&state.cex") {
        fail("game-server still contains synchronous CEX reconciliation");
    }
    if source.contains("settle_pending_matches(&settlement_state") {
        fail("game-server still schedules in-process settlement");
    }

    let Some(body) = source.strip_prefix(LIB_HEADER) else {
        fail("game-server crate header drifted from the reviewed template");
    };
    let generated = rewrite_migration_includes(body);
    fs::write(out_dir.join("trnm_game_server_lib_generated.rs"), generated)
        .unwrap_or_else(|error| fail(format!("write generated game server: {error}")));
}

fn generate_settlement_worker(out_dir: &Path) {
    let template_path = PathBuf::from("src/settlement_worker.rs.in");
    let mut source = fs::read_to_string(&template_path)
        .unwrap_or_else(|error| fail(format!("read {}: {error}", template_path.display())));

    replace_once(
        &mut source,
        "const MIGRATION_V17: &str = include_str!(\"../migrations/0017_online_settlement_worker_runtime_v1.sql\");",
        r#"const MIGRATION_V17: &str = include_str!("../migrations/0017_online_settlement_worker_runtime_v1.sql");
const MIGRATION_V18: &str =
    include_str!("../migrations/0018_online_settlement_operator_controls_v1.sql");
const MIGRATION_V19: &str =
    include_str!("../migrations/0019_online_settlement_quarantine_v1.sql");"#,
        "settlement-worker migration constants",
    );
    replace_once(
        &mut source,
        r#"        (
            17_i32,
            "0017_online_settlement_worker_runtime_v1",
            MIGRATION_V17,
        ),
"#,
        r#"        (
            17_i32,
            "0017_online_settlement_worker_runtime_v1",
            MIGRATION_V17,
        ),
        (
            18_i32,
            "0018_online_settlement_operator_controls_v1",
            MIGRATION_V18,
        ),
        (
            19_i32,
            "0019_online_settlement_quarantine_v1",
            MIGRATION_V19,
        ),
"#,
        "settlement-worker migration ledger",
    );
    replace_once(
        &mut source,
        "pub async fn run(config: WorkerConfig) -> Result<(), String> {",
        "#[allow(dead_code)]\nasync fn run_legacy_disabled(config: WorkerConfig) -> Result<(), String> {",
        "legacy settlement worker run loop retirement",
    );

    let generated = rewrite_migration_includes(&source);
    fs::write(
        out_dir.join("trnm_settlement_worker_generated.rs"),
        generated,
    )
    .unwrap_or_else(|error| fail(format!("write generated settlement worker: {error}")));
}

fn generate_cex(out_dir: &Path) {
    let template_path = PathBuf::from("src/cex.rs.in");
    let mut source = fs::read_to_string(&template_path)
        .unwrap_or_else(|error| fail(format!("read {}: {error}", template_path.display())));

    replace_once(
        &mut source,
        "const SETTLEMENT_OUTBOX_REQUIRED: &str =\n    \"external economy settlement is owned by trnm-settlement-worker; synchronous EconomyBackend I/O is prohibited\";",
        "const SETTLEMENT_OUTBOX_REQUIRED: &str =\n    \"external economy settlement is owned by trnm-settlement-worker; synchronous EconomyBackend I/O is prohibited\";\nconst MAX_REMOTE_ERROR_BODY_BYTES: usize = 64 * 1024;",
        "remote error-body budget",
    );

    let retryable_old = r#"fn retryable_status(status: StatusCode) -> bool {
    status == StatusCode::REQUEST_TIMEOUT
        || status == StatusCode::TOO_MANY_REQUESTS
        || status.as_u16() == 425
        || status.is_server_error()
}
"#;
    let retryable_new = r#"fn retryable_status(status: StatusCode) -> bool {
    status == StatusCode::REQUEST_TIMEOUT
        || status == StatusCode::TOO_MANY_REQUESTS
        || status == StatusCode::CONFLICT
        || status.as_u16() == 425
        || status.is_server_error()
}

async fn bounded_error_body(response: reqwest::Response) -> String {
    if response
        .content_length()
        .is_some_and(|length| length > MAX_REMOTE_ERROR_BODY_BYTES as u64)
    {
        return format!(
            "remote error body omitted: content-length exceeds {} bytes",
            MAX_REMOTE_ERROR_BODY_BYTES
        );
    }
    match response.bytes().await {
        Ok(bytes) => {
            let bounded = &bytes[..bytes.len().min(MAX_REMOTE_ERROR_BODY_BYTES)];
            String::from_utf8_lossy(bounded).into_owned()
        }
        Err(error) => format!("remote error body unavailable: {error}"),
    }
}
"#;
    replace_once(
        &mut source,
        retryable_old,
        retryable_new,
        "retryable remote status and body budget",
    );
    replace_all_at_least(
        &mut source,
        "let body = response.text().await.unwrap_or_default();",
        "let body = bounded_error_body(response).await;",
        4,
        "bounded remote error body",
    );

    for (old, new, label) in [
        (
            "ExternalSettlementError::Permanent(format!(\n                    \"decode isolated signer receipt lookup: {error}\"\n                ))",
            "ExternalSettlementError::Retryable(format!(\n                    \"decode isolated signer receipt lookup after success status: {error}\"\n                ))",
            "signer lookup ambiguous body",
        ),
        (
            "ExternalSettlementError::Permanent(format!(\n                    \"decode isolated signer response: {error}\"\n                ))",
            "ExternalSettlementError::Retryable(format!(\n                    \"decode isolated signer response after possible commit: {error}\"\n                ))",
            "signer submit ambiguous body",
        ),
        (
            "ExternalSettlementError::Permanent(format!(\n                    \"decode CEX receipt lookup: {error}\"\n                ))",
            "ExternalSettlementError::Retryable(format!(\n                    \"decode CEX receipt lookup after success status: {error}\"\n                ))",
            "CEX lookup ambiguous body",
        ),
        (
            "ExternalSettlementError::Permanent(format!(\"decode CEX receipt: {error}\"))",
            "ExternalSettlementError::Retryable(format!(\n                \"decode CEX receipt after possible commit: {error}\"\n            ))",
            "CEX submit ambiguous body",
        ),
    ] {
        replace_once(&mut source, old, new, label);
    }

    if source.contains("reqwest::blocking") || source.contains("blocking_client") {
        fail("generated CEX source reintroduced blocking HTTP");
    }
    fs::write(out_dir.join("trnm_cex_generated.rs"), source)
        .unwrap_or_else(|error| fail(format!("write generated CEX client: {error}")));
}

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs.in");
    println!("cargo:rerun-if-changed=src/settlement_worker.rs.in");
    println!("cargo:rerun-if-changed=src/settlement_worker_runtime_v2.rs");
    println!("cargo:rerun-if-changed=src/cex.rs.in");
    println!("cargo:rerun-if-changed=migrations");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is required"));
    generate_game_server(&out_dir);
    generate_settlement_worker(&out_dir);
    generate_cex(&out_dir);
}
