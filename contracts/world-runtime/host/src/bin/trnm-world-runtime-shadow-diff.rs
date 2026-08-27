#![forbid(unsafe_code)]
#![allow(clippy::pedantic)]

use serde_json::Value;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use trnm_world_runtime_adapter::{canonical_json_bytes, parse_strict_json, RuntimeError};
use trnm_world_runtime_host::compare_shadow_value;

fn main() -> ExitCode {
    if env::args().any(|argument| matches!(argument.as_str(), "--help" | "-h")) {
        print_help();
        return ExitCode::SUCCESS;
    }
    match run() {
        Ok(equivalent) => ExitCode::from(if equivalent { 0 } else { 1 }),
        Err(error) => {
            if write_value(&error.as_json()).is_err() {
                eprintln!("{error}");
            }
            ExitCode::from(64)
        }
    }
}

fn run() -> Result<bool, RuntimeError> {
    let input_path = parse_args()?;
    let input = read_input(input_path.as_deref())?;
    let input = parse_strict_json(&input)?;
    let report = compare_shadow_value(&input)?;
    let equivalent = report
        .get("equivalent")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            RuntimeError::new(
                "output_contract_violation",
                "shadow report is missing equivalent",
            )
        })?;
    write_value(&report)?;
    Ok(equivalent)
}

fn parse_args() -> Result<Option<PathBuf>, RuntimeError> {
    let mut args = env::args().skip(1);
    let mut input = None;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--input" => {
                if input.is_some() {
                    return Err(RuntimeError::new(
                        "invalid_host_configuration",
                        "--input was supplied more than once",
                    ));
                }
                let path = args.next().ok_or_else(|| {
                    RuntimeError::new(
                        "invalid_host_configuration",
                        "--input requires a path",
                    )
                })?;
                input = Some(PathBuf::from(path));
            }
            unknown => {
                return Err(RuntimeError::new(
                    "invalid_host_configuration",
                    format!("unknown argument: {unknown}"),
                ));
            }
        }
    }
    Ok(input)
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
            format!("read shadow input from stdin: {error}"),
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
            format!("write shadow report: {error}"),
        )
    })?;
    stdout.write_all(b"\n").map_err(|error| {
        RuntimeError::new(
            "invalid_host_configuration",
            format!("write shadow report terminator: {error}"),
        )
    })?;
    Ok(())
}

fn print_help() {
    println!(
        "trnm-world-runtime-shadow-diff [--input PATH]\n\n\
Reads one strict trnm_world_shadow_input_v1 object from PATH or stdin.\n\
Exit 0 means equivalent, exit 1 means a typed divergence, and exit 64 means\n\
the input or a claimed response failed contract validation."
    );
}
