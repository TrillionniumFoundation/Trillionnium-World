#![forbid(unsafe_code)]
#![allow(clippy::pedantic)]

use serde_json::Value;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;
use trnm_world_runtime_adapter::{
    canonical_json_bytes, parse_strict_json, RtsMissionExecutor, RuntimeError, RuntimeSelection,
    WorldRuntimeV1, DEFAULT_MAX_ADVANCE_STEPS,
};
use trnm_world_runtime_host::runtime_observation;

struct Config {
    ruleset_id: String,
    ruleset_version: String,
    ruleset_digest: String,
    content_digest: String,
    max_advance_steps: u64,
    input: Option<PathBuf>,
    observe: bool,
    implementation_id: Option<String>,
    implementation_revision: Option<String>,
}

fn main() -> ExitCode {
    if env::args().any(|argument| matches!(argument.as_str(), "--help" | "-h")) {
        print_help();
        return ExitCode::SUCCESS;
    }
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            if write_value(&error.as_json()).is_err() {
                eprintln!("{error}");
            }
            ExitCode::from(64)
        }
    }
}

fn run() -> Result<u8, RuntimeError> {
    let config = parse_args()?;
    let input = read_input(config.input.as_deref())?;
    let request = parse_strict_json(&input)?;
    let selection = RuntimeSelection::new(
        config.ruleset_id,
        config.ruleset_version,
        config.ruleset_digest,
        config.content_digest,
    )?;
    let executor = RtsMissionExecutor::new(config.max_advance_steps)?;
    let runtime = WorldRuntimeV1::new(selection, executor);
    let started = Instant::now();
    let (response, execution_failed) = match runtime.execute_value(&request) {
        Ok(response) => (response, false),
        Err(error) => (error.as_json(), true),
    };
    let elapsed_micros = started.elapsed().as_micros().min(i64::MAX as u128);
    let elapsed_micros = u64::try_from(elapsed_micros).map_err(|_| {
        RuntimeError::new(
            "resource_limit_exceeded",
            "execution duration exceeds the observation range",
        )
    })?;
    let output = if config.observe {
        runtime_observation(
            config.implementation_id.as_deref().ok_or_else(|| {
                RuntimeError::new(
                    "invalid_host_configuration",
                    "--implementation-id is required with --observe",
                )
            })?,
            config.implementation_revision.as_deref().ok_or_else(|| {
                RuntimeError::new(
                    "invalid_host_configuration",
                    "--implementation-revision is required with --observe",
                )
            })?,
            &request,
            response,
            elapsed_micros,
        )?
    } else {
        response
    };
    write_value(&output)?;
    Ok(if execution_failed && !config.observe { 2 } else { 0 })
}

fn parse_args() -> Result<Config, RuntimeError> {
    let mut args = env::args().skip(1);
    let mut ruleset_id = None;
    let mut ruleset_version = None;
    let mut ruleset_digest = None;
    let mut content_digest = None;
    let mut max_advance_steps = None;
    let mut input = None;
    let mut observe = false;
    let mut implementation_id = None;
    let mut implementation_revision = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--ruleset-id" => set_once(
                &mut ruleset_id,
                next_value(&mut args, "--ruleset-id")?,
                "--ruleset-id",
            )?,
            "--ruleset-version" => set_once(
                &mut ruleset_version,
                next_value(&mut args, "--ruleset-version")?,
                "--ruleset-version",
            )?,
            "--ruleset-digest" => set_once(
                &mut ruleset_digest,
                next_value(&mut args, "--ruleset-digest")?,
                "--ruleset-digest",
            )?,
            "--content-digest" => set_once(
                &mut content_digest,
                next_value(&mut args, "--content-digest")?,
                "--content-digest",
            )?,
            "--max-advance-steps" => {
                let raw = next_value(&mut args, "--max-advance-steps")?;
                let value = raw.parse::<u64>().map_err(|error| {
                    RuntimeError::new(
                        "invalid_host_configuration",
                        format!("invalid --max-advance-steps: {error}"),
                    )
                })?;
                set_once(&mut max_advance_steps, value, "--max-advance-steps")?;
            }
            "--input" => set_once(
                &mut input,
                PathBuf::from(next_value(&mut args, "--input")?),
                "--input",
            )?,
            "--observe" => {
                if observe {
                    return Err(RuntimeError::new(
                        "invalid_host_configuration",
                        "--observe was supplied more than once",
                    ));
                }
                observe = true;
            }
            "--implementation-id" => set_once(
                &mut implementation_id,
                next_value(&mut args, "--implementation-id")?,
                "--implementation-id",
            )?,
            "--implementation-revision" => set_once(
                &mut implementation_revision,
                next_value(&mut args, "--implementation-revision")?,
                "--implementation-revision",
            )?,
            unknown => {
                return Err(RuntimeError::new(
                    "invalid_host_configuration",
                    format!("unknown argument: {unknown}"),
                ));
            }
        }
    }

    if !observe && (implementation_id.is_some() || implementation_revision.is_some()) {
        return Err(RuntimeError::new(
            "invalid_host_configuration",
            "implementation identity arguments require --observe",
        ));
    }

    Ok(Config {
        ruleset_id: required(ruleset_id, "--ruleset-id")?,
        ruleset_version: required(ruleset_version, "--ruleset-version")?,
        ruleset_digest: required(ruleset_digest, "--ruleset-digest")?,
        content_digest: required(content_digest, "--content-digest")?,
        max_advance_steps: max_advance_steps.unwrap_or(DEFAULT_MAX_ADVANCE_STEPS),
        input,
        observe,
        implementation_id,
        implementation_revision,
    })
}

fn set_once<T>(slot: &mut Option<T>, value: T, flag: &str) -> Result<(), RuntimeError> {
    if slot.is_some() {
        return Err(RuntimeError::new(
            "invalid_host_configuration",
            format!("{flag} was supplied more than once"),
        ));
    }
    *slot = Some(value);
    Ok(())
}

fn next_value(
    args: &mut impl Iterator<Item = String>,
    flag: &str,
) -> Result<String, RuntimeError> {
    args.next().ok_or_else(|| {
        RuntimeError::new(
            "invalid_host_configuration",
            format!("{flag} requires a value"),
        )
    })
}

fn required<T>(value: Option<T>, flag: &str) -> Result<T, RuntimeError> {
    value.ok_or_else(|| {
        RuntimeError::new(
            "invalid_host_configuration",
            format!("missing required argument {flag}"),
        )
    })
}

fn read_input(path: Option<&Path>) -> Result<String, RuntimeError> {
    if let Some(path) = path {
        return fs::read_to_string(path).map_err(|error| {
            RuntimeError::new(
                "invalid_host_configuration",
                format!("read {}: {error}", path.display()),
            )
        });
    }
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).map_err(|error| {
        RuntimeError::new(
            "invalid_host_configuration",
            format!("read request from stdin: {error}"),
        )
    })?;
    Ok(input)
}

fn write_value(value: &Value) -> Result<(), RuntimeError> {
    let bytes = canonical_json_bytes(value)?;
    let mut stdout = io::stdout().lock();
    stdout.write_all(&bytes).map_err(|error| {
        RuntimeError::new(
            "invalid_host_configuration",
            format!("write response to stdout: {error}"),
        )
    })?;
    stdout.write_all(b"\n").map_err(|error| {
        RuntimeError::new(
            "invalid_host_configuration",
            format!("write response terminator: {error}"),
        )
    })?;
    Ok(())
}

fn print_help() {
    println!(
        "trnm-world-runtime-exec \\\n  --ruleset-id ID \\\n  --ruleset-version VERSION \\\n  --ruleset-digest HEX64 \\\n  --content-digest HEX64 \\\n  [--max-advance-steps N] [--input PATH] \\\n  [--observe --implementation-id ID --implementation-revision HEX40]\n\n\
Reads one strict trnm_world_runtime_v1 execute request from PATH or stdin.\n\
Raw mode exits 2 for a deterministic runtime rejection. Observation mode emits\n\
a trnm_world_runtime_observation_v1 packet and exits 0 for both success and\n\
deterministic rejection so a shadow comparator can compare both paths."
    );
}
