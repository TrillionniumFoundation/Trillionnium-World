#![allow(dead_code)]

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn read_nonempty(path: &Path) -> String {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("read direct source {}: {error}", path.display()));
    assert!(
        !source.is_empty(),
        "direct source {} is empty",
        path.display()
    );
    source
}

fn record_path<'a>(record: &'a Value, field: &str) -> &'a str {
    record
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("direct-source manifest record has no {field}"))
}

pub fn read_crate_source_bundle(relative: &str) -> String {
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let entry = crate_root.join(relative);
    let mut source = read_nonempty(&entry);
    let manifest_path = entry
        .parent()
        .expect("source entrypoint has a parent")
        .join("lib_parts/manifest.json");
    if !manifest_path.exists() {
        return source;
    }
    let manifest: Value = serde_json::from_str(&read_nonempty(&manifest_path))
        .expect("direct-source manifest is valid JSON");
    assert_eq!(
        manifest.get("semantic_generation").and_then(Value::as_bool),
        Some(false),
        "direct-source manifest must prohibit semantic generation"
    );
    let mut listed = BTreeSet::new();
    for field in ["parts", "nested_test_parts"] {
        let records = manifest
            .get(field)
            .and_then(Value::as_array)
            .unwrap_or_else(|| panic!("direct-source manifest {field} is not an array"));
        for record in records {
            let relative = record_path(record, "path");
            assert!(relative.starts_with("lib_parts/"));
            let path = entry.parent().expect("entrypoint parent").join(relative);
            let bytes = fs::read(&path)
                .unwrap_or_else(|error| panic!("read direct source {}: {error}", path.display()));
            assert_eq!(
                record.get("bytes").and_then(Value::as_u64),
                Some(bytes.len() as u64),
                "direct-source byte length drifted for {relative}"
            );
            let actual_sha = format!("{:x}", Sha256::digest(&bytes));
            assert_eq!(
                record.get("sha256").and_then(Value::as_str),
                Some(actual_sha.as_str()),
                "direct-source SHA-256 drifted for {relative}"
            );
            assert!(listed.insert(path));
            source.push('\n');
            source.push_str(std::str::from_utf8(&bytes).expect("direct source is UTF-8"));
        }
    }
    let parts_dir = entry.parent().expect("entrypoint parent").join("lib_parts");
    let mut discovered = BTreeSet::new();
    fn walk(directory: &Path, output: &mut BTreeSet<PathBuf>) {
        for entry in fs::read_dir(directory)
            .unwrap_or_else(|error| panic!("walk direct source {}: {error}", directory.display()))
        {
            let path = entry.expect("directory entry is readable").path();
            if path.is_dir() {
                walk(&path, output);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                output.insert(path);
            }
        }
    }
    walk(&parts_dir, &mut discovered);
    assert_eq!(
        discovered, listed,
        "direct-source manifest/file set drifted"
    );
    source
}

pub fn read_settlement_worker_bundle() -> String {
    [
        read_crate_source_bundle("src/settlement_worker.rs"),
        read_crate_source_bundle("src/settlement_worker_legacy.rs"),
        read_crate_source_bundle("src/settlement_worker_runtime_v2.rs"),
    ]
    .join("\n")
}
