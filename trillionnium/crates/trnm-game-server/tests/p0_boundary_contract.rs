use std::{fs, path::{Path, PathBuf}};

fn rust_files(root: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(root).unwrap_or_else(|error| panic!("read {}: {error}", root.display()));
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}

fn function_bodies(source: &str) -> Vec<(&str, &str)> {
    let bytes = source.as_bytes();
    let mut bodies = Vec::new();
    let mut cursor = 0usize;
    while let Some(relative) = source[cursor..].find("fn ") {
        let start = cursor + relative;
        let name_start = start + 3;
        let Some(open_relative) = source[name_start..].find('{') else {
            break;
        };
        let open = name_start + open_relative;
        let signature = &source[name_start..open];
        if signature.contains(';') {
            cursor = open + 1;
            continue;
        }
        let name_end = signature
            .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
            .unwrap_or(signature.len());
        let name = &signature[..name_end];
        if name.is_empty() {
            cursor = open + 1;
            continue;
        }

        let mut depth = 0i64;
        let mut in_string = false;
        let mut escaped = false;
        let mut end = None;
        for (offset, byte) in bytes[open..].iter().copied().enumerate() {
            if in_string {
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == b'"' {
                    in_string = false;
                }
                continue;
            }
            match byte {
                b'"' => in_string = true,
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(open + offset + 1);
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(end) = end {
            bodies.push((name, &source[start..end]));
            cursor = end;
        } else {
            break;
        }
    }
    bodies
}

#[test]
fn settlement_external_io_is_not_owned_by_a_database_transaction_function() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&root, &mut files);
    assert!(!files.is_empty(), "game-server source tree is empty");

    let transaction_markers = [
        ".begin().await",
        "Transaction<'",
        "Transaction <'",
        "FOR UPDATE",
        "for update",
        "FOR SHARE",
        "for share",
    ];
    let external_markers = [
        ".reconcile_economy(",
        ".execute_authoritative(",
        ".blocking_client",
    ];

    let mut reconcile_count = 0usize;
    let mut violations = Vec::new();
    for path in files {
        let source = fs::read_to_string(&path).unwrap();
        for (name, body) in function_bodies(&source) {
            let has_external = external_markers.iter().any(|marker| body.contains(marker));
            let has_transaction = transaction_markers.iter().any(|marker| body.contains(marker));
            if body.contains(".reconcile_economy(") {
                reconcile_count += 1;
            }
            if has_external && has_transaction {
                violations.push(format!("{}::{name}", path.display()));
            }
        }
    }

    assert!(
        reconcile_count > 0,
        "the synchronous economy backend marker disappeared; update this reviewed contract with its replacement"
    );
    assert!(
        violations.is_empty(),
        "external settlement work shares a transaction-owning function: {violations:?}"
    );
}

#[test]
fn world_source_does_not_acquire_target_nakama_key_or_completion_signer_custody() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&root, &mut files);

    let forbidden = [
        "TRNM_NAKAMA_AUTHORITY_PRIVATE_KEY",
        "NAKAMA_AUTHORITY_PRIVATE_KEY",
        "WORLD_MATCH_COMPLETED_SIGNING_KEY",
        "sign_match_completed_v1",
        "WorldMatchCompletedSigner",
        "world_canonical_roster_root",
        "world_canonical_event_root",
        "world_canonical_archive_root",
        "world_chain_finality_proof",
        "world_chain_inclusion_proof",
    ];

    let mut violations = Vec::new();
    for path in files {
        let source = fs::read_to_string(&path).unwrap();
        for marker in forbidden {
            if source.contains(marker) {
                violations.push(format!("{}: {marker}", path.display()));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "World crossed the target authority/custody boundary: {violations:?}"
    );
}

#[test]
fn current_plan_keeps_public_online_and_market_fail_closed() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let plan = fs::read_to_string(
        repository.join("docs/development/trnm-world-development-plan-v3.md"),
    )
    .unwrap();
    let gates = fs::read_to_string(repository.join("docs/status/world-gates-v1.json")).unwrap();

    assert!(plan.contains("public online remains **NO-GO**"));
    assert!(plan.contains("public player market remains **disabled**"));
    assert!(gates.contains("\"id\": \"public_online\""));
    assert!(gates.contains("\"status\": \"no_go\""));
    assert!(gates.contains("\"id\": \"public_player_market\""));
    assert!(gates.contains("\"status\": \"disabled\""));
}
