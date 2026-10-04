//! Fail-closed production configuration preflight for the World-domain service.
//!
//! Passing this check means only that startup material is structurally valid.
//! It does not prove adapter behavior, deployment health, independent review,
//! cross-repository compatibility, or production authorization.

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeSet;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

mod production_preflight;

const CONFIG_SCHEMA: &str = "trillionnium_world_production_config_v1";
const AUTHORITY_MODE: &str = "nakama_only";
const PRODUCTION_AUTHORIZATION: &str = "not_granted";
const MAX_CONFIG_BYTES: u64 = 128 * 1024;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProductionConfig {
    schema: String,
    authority_mode: String,
    environment: String,
    deployment_id: String,
    component_lock_id: String,
    source_commit: String,
    source_tree: String,
    binary_sha256: String,
    nakama_base_url: String,
    cex_base_url: String,
    chain_base_url: String,
    postgres_dsn_file: PathBuf,
    session_verification_key_file: PathBuf,
    tls_ca_file: PathBuf,
    evidence_root: PathBuf,
    metrics_bind: String,
    request_timeout_ms: u64,
    max_in_flight: u32,
}

fn require_text(value: &str, field: &str) -> Result<()> {
    if value.is_empty()
        || value.trim() != value
        || value.len() > 512
        || value.bytes().any(|byte| byte.is_ascii_control())
    {
        bail!("{field} must be nonempty, trimmed, bounded text without control bytes");
    }
    Ok(())
}

fn require_lower_hex(value: &str, length: usize, field: &str) -> Result<()> {
    if value.len() != length
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        bail!("{field} must contain exactly {length} lowercase hexadecimal characters");
    }
    Ok(())
}

fn endpoint_host(value: &str, field: &str) -> Result<String> {
    production_preflight::endpoint_host(value)
        .map_err(|reason| anyhow::anyhow!("{field}: {reason}"))
}

fn require_absolute_file(path: &Path, field: &str, secret: bool) -> Result<()> {
    if !path.is_absolute() {
        bail!("{field} must be an absolute path");
    }
    let metadata = production_preflight::ordinary_absolute_path(path, false)
        .map_err(|reason| anyhow::anyhow!("{field}: {reason}"))?;
    if metadata.len() == 0 {
        bail!("{field} must be nonempty");
    }
    if secret {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o077 != 0 {
                bail!("{field} grants group or world permissions");
            }
        }
    }
    Ok(())
}

fn validate(config: &ProductionConfig) -> Result<Vec<String>> {
    if config.schema != CONFIG_SCHEMA {
        bail!("schema must be {CONFIG_SCHEMA}");
    }
    if config.authority_mode != AUTHORITY_MODE {
        bail!("authority_mode must be {AUTHORITY_MODE}");
    }
    require_text(&config.environment, "environment")?;
    require_text(&config.deployment_id, "deployment_id")?;
    require_text(&config.component_lock_id, "component_lock_id")?;
    require_lower_hex(&config.source_commit, 40, "source_commit")?;
    require_lower_hex(&config.source_tree, 40, "source_tree")?;
    require_lower_hex(&config.binary_sha256, 64, "binary_sha256")?;

    let hosts = [
        endpoint_host(&config.nakama_base_url, "nakama_base_url")?,
        endpoint_host(&config.cex_base_url, "cex_base_url")?,
        endpoint_host(&config.chain_base_url, "chain_base_url")?,
    ];
    if hosts.iter().collect::<BTreeSet<_>>().len() != hosts.len() {
        bail!("Nakama, CEX, and Chain endpoints must have distinct hosts");
    }

    require_absolute_file(&config.postgres_dsn_file, "postgres_dsn_file", true)?;
    require_absolute_file(
        &config.session_verification_key_file,
        "session_verification_key_file",
        true,
    )?;
    require_absolute_file(&config.tls_ca_file, "tls_ca_file", false)?;
    if !config.evidence_root.is_absolute() {
        bail!("evidence_root must be an absolute path");
    }
    production_preflight::ordinary_absolute_path(&config.evidence_root, true)
        .map_err(|reason| anyhow::anyhow!("evidence_root: {reason}"))?;

    let metrics: SocketAddr = config
        .metrics_bind
        .parse()
        .context("metrics_bind must be a literal SocketAddr")?;
    if !metrics.ip().is_loopback() || metrics.port() == 0 {
        bail!("metrics_bind must be loopback with a nonzero port");
    }
    if !(100..=30_000).contains(&config.request_timeout_ms) {
        bail!("request_timeout_ms must be between 100 and 30000");
    }
    if !(1..=10_000).contains(&config.max_in_flight) {
        bail!("max_in_flight must be between 1 and 10000");
    }

    Ok(hosts.to_vec())
}

fn main() -> Result<()> {
    let path = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("TRNM_WORLD_PRODUCTION_CONFIG").map(PathBuf::from))
        .context("usage: trnm-world-production-preflight <absolute-config.json>")?;
    if !path.is_absolute() {
        bail!("production config path must be absolute");
    }
    let bytes = production_preflight::read_bounded_config(&path, MAX_CONFIG_BYTES)
        .map_err(|reason| anyhow::anyhow!("production config: {reason}"))?;
    let config: ProductionConfig =
        serde_json::from_slice(&bytes).context("production config is not strict schema JSON")?;
    let endpoint_hosts = validate(&config)?;

    let summary = json!({
        "schema": CONFIG_SCHEMA,
        "status": "production_configuration_preflight_passed",
        "authority_mode": AUTHORITY_MODE,
        "endpoint_intake_policy": production_preflight::ENDPOINT_POLICY,
        "dns_egress_verified": false,
        "filesystem_checks_are_point_in_time": true,
        "environment": config.environment,
        "deployment_id": config.deployment_id,
        "component_lock_id": config.component_lock_id,
        "source_commit": config.source_commit,
        "source_tree": config.source_tree,
        "binary_sha256": config.binary_sha256,
        "endpoint_hosts": endpoint_hosts,
        "metrics_bind": config.metrics_bind,
        "request_timeout_ms": config.request_timeout_ms,
        "max_in_flight": config.max_in_flight,
        "secret_values_emitted": false,
        "fixture_fallback_allowed": false,
        "production_authorization": PRODUCTION_AUTHORIZATION,
    });
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
