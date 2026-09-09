use std::{
    fs,
    ops::Range,
    path::{Path, PathBuf},
};

fn rust_files(root: &Path, out: &mut Vec<PathBuf>) {
    let entries =
        fs::read_dir(root).unwrap_or_else(|error| panic!("read {}: {error}", root.display()));
    for entry in entries {
        let entry = entry.expect("read source directory entry");
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

fn raw_string_open(bytes: &[u8], start: usize) -> Option<(usize, usize)> {
    let mut cursor = start;
    if bytes.get(cursor) == Some(&b'b') || bytes.get(cursor) == Some(&b'c') {
        cursor += 1;
    }
    if bytes.get(cursor) != Some(&b'r') {
        return None;
    }
    cursor += 1;
    let hashes_start = cursor;
    while bytes.get(cursor) == Some(&b'#') {
        cursor += 1;
    }
    (bytes.get(cursor) == Some(&b'"')).then_some((cursor + 1, cursor - hashes_start))
}

fn mask_rust_comments_and_literals(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut masked = bytes.to_vec();
    let blank = |masked: &mut [u8], index: usize| {
        if masked[index] != b'\n' && masked[index] != b'\r' {
            masked[index] = b' ';
        }
    };
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        if bytes[cursor..].starts_with(b"//") {
            while cursor < bytes.len() && bytes[cursor] != b'\n' {
                blank(&mut masked, cursor);
                cursor += 1;
            }
            continue;
        }
        if bytes[cursor..].starts_with(b"/*") {
            let mut depth = 0usize;
            while cursor < bytes.len() {
                if bytes[cursor..].starts_with(b"/*") {
                    depth += 1;
                    blank(&mut masked, cursor);
                    if cursor + 1 < bytes.len() {
                        blank(&mut masked, cursor + 1);
                    }
                    cursor += 2;
                    continue;
                }
                if bytes[cursor..].starts_with(b"*/") {
                    blank(&mut masked, cursor);
                    if cursor + 1 < bytes.len() {
                        blank(&mut masked, cursor + 1);
                    }
                    cursor += 2;
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        break;
                    }
                    continue;
                }
                blank(&mut masked, cursor);
                cursor += 1;
            }
            continue;
        }
        if let Some((payload_start, hashes)) = raw_string_open(bytes, cursor) {
            let opening_end = payload_start;
            for index in cursor..opening_end {
                blank(&mut masked, index);
            }
            cursor = payload_start;
            loop {
                if cursor >= bytes.len() {
                    break;
                }
                if bytes[cursor] == b'"'
                    && cursor + 1 + hashes <= bytes.len()
                    && bytes[cursor + 1..cursor + 1 + hashes]
                        .iter()
                        .all(|byte| *byte == b'#')
                {
                    for index in cursor..cursor + 1 + hashes {
                        blank(&mut masked, index);
                    }
                    cursor += 1 + hashes;
                    break;
                }
                blank(&mut masked, cursor);
                cursor += 1;
            }
            continue;
        }
        let string_prefix =
            if bytes[cursor..].starts_with(b"b\"") || bytes[cursor..].starts_with(b"c\"") {
                2
            } else if bytes[cursor] == b'"' {
                1
            } else {
                0
            };
        if string_prefix > 0 {
            for index in cursor..cursor + string_prefix {
                blank(&mut masked, index);
            }
            cursor += string_prefix;
            let mut escaped = false;
            while cursor < bytes.len() {
                let byte = bytes[cursor];
                blank(&mut masked, cursor);
                cursor += 1;
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == b'"' {
                    break;
                }
            }
            continue;
        }
        if bytes[cursor] == b'\'' {
            let mut end = cursor + 1;
            let mut escaped = false;
            while end < bytes.len() && end.saturating_sub(cursor) <= 8 {
                let byte = bytes[end];
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == b'\'' {
                    for index in cursor..=end {
                        blank(&mut masked, index);
                    }
                    cursor = end + 1;
                    break;
                }
                end += 1;
            }
            if cursor > end.saturating_sub(8) {
                continue;
            }
        }
        cursor += 1;
    }
    String::from_utf8(masked).expect("masking Rust source preserves UTF-8")
}

fn function_ranges(source: &str) -> Vec<(String, Range<usize>)> {
    let masked = mask_rust_comments_and_literals(source);
    let bytes = masked.as_bytes();
    let mut bodies = Vec::new();
    let mut cursor = 0usize;
    while let Some(relative) = masked[cursor..].find("fn ") {
        let start = cursor + relative;
        let name_start = start + 3;
        let Some(open_relative) = masked[name_start..].find('{') else {
            break;
        };
        let open = name_start + open_relative;
        let signature = &masked[name_start..open];
        if signature.contains(';') {
            cursor = open + 1;
            continue;
        }
        let name_end = signature
            .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
            .unwrap_or(signature.len());
        let name = signature[..name_end].to_string();
        if name.is_empty() {
            cursor = open + 1;
            continue;
        }

        let mut depth = 0i64;
        let mut end = None;
        for (offset, byte) in bytes[open..].iter().copied().enumerate() {
            match byte {
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
            bodies.push((name, start..end));
            cursor = end;
        } else {
            break;
        }
    }
    bodies
}

fn first_call_argument(code: &str, open: usize) -> Option<&str> {
    let bytes = code.as_bytes();
    let mut round = 0i64;
    let mut square = 0i64;
    let mut curly = 0i64;
    for (offset, byte) in bytes.get(open + 1..)?.iter().copied().enumerate() {
        match byte {
            b'(' => round += 1,
            b')' if round == 0 && square == 0 && curly == 0 => {
                return Some(code[open + 1..open + 1 + offset].trim())
            }
            b')' => round -= 1,
            b'[' => square += 1,
            b']' => square -= 1,
            b'{' => curly += 1,
            b'}' => curly -= 1,
            b',' if round == 0 && square == 0 && curly == 0 => {
                return Some(code[open + 1..open + 1 + offset].trim())
            }
            _ => {}
        }
    }
    None
}

fn reconciliation_calls(code: &str) -> Vec<(usize, &str)> {
    let marker = ".reconcile_economy(";
    let mut calls = Vec::new();
    let mut cursor = 0usize;
    while let Some(relative) = code[cursor..].find(marker) {
        let marker_start = cursor + relative;
        let open = marker_start + marker.len() - 1;
        if let Some(argument) = first_call_argument(code, open) {
            calls.push((marker_start, argument));
        }
        cursor = open + 1;
    }
    calls
}

fn borrowed_identifier(argument: &str) -> Option<&str> {
    let argument = argument.trim().strip_prefix('&')?.trim_start();
    let argument = argument.strip_prefix("mut ").unwrap_or(argument).trim();
    (!argument.is_empty()
        && argument
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_'))
    .then_some(argument)
}

fn identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn keyword_at(bytes: &[u8], start: usize, keyword: &[u8]) -> bool {
    bytes.get(start..start + keyword.len()) == Some(keyword)
        && (start == 0 || !identifier_byte(bytes[start - 1]))
        && bytes
            .get(start + keyword.len())
            .is_none_or(|byte| !identifier_byte(*byte))
}

fn captured_backend_constructor(value: &str) -> bool {
    let value = value.trim_start();
    value.starts_with("CapturedReceiptBackend {")
        || value.starts_with("CapturedReceiptBackend::")
        || value.contains("::CapturedReceiptBackend {")
        || value.contains("::CapturedReceiptBackend::")
}

fn simple_let_binding(code: &str, start: usize) -> Option<(usize, String, bool)> {
    let bytes = code.as_bytes();
    if !keyword_at(bytes, start, b"let") {
        return None;
    }
    let mut cursor = start + 3;
    if !bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
        return None;
    }
    while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
        cursor += 1;
    }
    if keyword_at(bytes, cursor, b"mut") {
        cursor += 3;
        while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
            cursor += 1;
        }
    }
    let name_start = cursor;
    while bytes.get(cursor).is_some_and(|byte| identifier_byte(*byte)) {
        cursor += 1;
    }
    if cursor == name_start {
        return None;
    }
    let name = code[name_start..cursor].to_string();

    let initializer_start = loop {
        match bytes.get(cursor).copied()? {
            b'=' if bytes.get(cursor + 1) != Some(&b'=')
                && cursor.checked_sub(1).and_then(|index| bytes.get(index)) != Some(&b'=') =>
            {
                break cursor + 1;
            }
            b';' => return None,
            _ => cursor += 1,
        }
    };

    let mut round = 0i64;
    let mut square = 0i64;
    let mut curly = 0i64;
    cursor = initializer_start;
    while let Some(byte) = bytes.get(cursor).copied() {
        match byte {
            b'(' => round += 1,
            b')' => round -= 1,
            b'[' => square += 1,
            b']' => square -= 1,
            b'{' => curly += 1,
            b'}' => curly -= 1,
            b';' if round == 0 && square == 0 && curly == 0 => {
                let initializer = &code[initializer_start..cursor];
                return Some((cursor + 1, name, captured_backend_constructor(initializer)));
            }
            _ => {}
        }
        cursor += 1;
    }
    None
}

fn captured_backend_binding_before(code: &str, call_start: usize, identifier: &str) -> bool {
    let prefix = &code[..call_start];
    let bytes = prefix.as_bytes();
    let mut scopes: Vec<Vec<(String, bool)>> = vec![Vec::new()];
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'{' => {
                scopes.push(Vec::new());
                cursor += 1;
            }
            b'}' => {
                if scopes.len() > 1 {
                    scopes.pop();
                }
                cursor += 1;
            }
            _ if keyword_at(bytes, cursor, b"let") => {
                if let Some((end, name, captured)) = simple_let_binding(prefix, cursor) {
                    scopes
                        .last_mut()
                        .expect("scope stack is never empty")
                        .push((name, captured));
                    cursor = end;
                } else {
                    cursor += 1;
                }
            }
            _ => cursor += 1,
        }
    }

    for scope in scopes.iter().rev() {
        if let Some((_, captured)) = scope.iter().rev().find(|(name, _)| name == identifier) {
            return *captured;
        }
    }
    false
}

fn call_uses_captured_backend(code: &str, call_start: usize, argument: &str) -> bool {
    captured_backend_constructor(
        argument
            .trim()
            .strip_prefix('&')
            .map(str::trim_start)
            .unwrap_or_default(),
    ) || borrowed_identifier(argument)
        .is_some_and(|identifier| captured_backend_binding_before(code, call_start, identifier))
}

fn transaction_reconciliation_counts(source: &str) -> (usize, usize, Vec<String>) {
    let code = mask_rust_comments_and_literals(source);
    let sql_lock_markers = ["FOR UPDATE", "for update", "FOR SHARE", "for share"];
    let external_markers = [
        ".execute_authoritative(",
        ".authorize_settlement_intent(",
        ".submit_authorized_settlement_intent(",
        ".readiness().await",
        ".send().await",
        ".blocking_client",
    ];

    let mut reconcile_count = 0usize;
    let mut captured_transaction_reconcile_count = 0usize;
    let mut violations = Vec::new();
    for (name, range) in function_ranges(source) {
        let raw_body = &source[range.clone()];
        let code_body = &code[range];
        let compact_code = code_body
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>();
        let has_external = external_markers
            .iter()
            .any(|marker| code_body.contains(marker));
        let has_transaction = compact_code.contains(".begin().await")
            || compact_code.contains("Transaction<'")
            || sql_lock_markers
                .iter()
                .any(|marker| raw_body.contains(marker));
        let calls = reconciliation_calls(code_body);
        reconcile_count += calls.len();
        if has_transaction {
            for (call_start, argument) in calls {
                if call_uses_captured_backend(code_body, call_start, argument) {
                    captured_transaction_reconcile_count += 1;
                } else {
                    violations.push(format!(
                        "{name} reconciles economy under a transaction without passing a value constructed as CapturedReceiptBackend"
                    ));
                }
            }
        }
        if has_external && has_transaction {
            violations.push(format!(
                "{name} performs remote settlement I/O while owning a database transaction"
            ));
        }
    }
    (
        reconcile_count,
        captured_transaction_reconcile_count,
        violations,
    )
}

#[test]
fn settlement_external_io_is_not_owned_by_a_database_transaction_function() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&root, &mut files);
    assert!(!files.is_empty(), "game-server source tree is empty");

    let mut reconcile_count = 0usize;
    let mut captured_transaction_reconcile_count = 0usize;
    let mut violations = Vec::new();
    for path in files {
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let (reconciliations, captured, file_violations) =
            transaction_reconciliation_counts(&source);
        reconcile_count += reconciliations;
        captured_transaction_reconcile_count += captured;
        violations.extend(
            file_violations
                .into_iter()
                .map(|violation| format!("{}::{violation}", path.display())),
        );
    }

    assert!(
        reconcile_count > 0,
        "the economy reconciliation marker disappeared; update ADR-0002 and this reviewed contract with its replacement"
    );
    assert!(
        captured_transaction_reconcile_count > 0,
        "the reviewed captured-receipt reconciliation boundary disappeared; update this contract with its replacement"
    );
    assert!(
        violations.is_empty(),
        "settlement transaction boundary violations: {violations:?}"
    );
}

#[test]
fn captured_receipt_exception_is_bound_to_the_actual_argument_value() {
    let accepted = r#"
        async fn apply() {
            let mut transaction = pool.begin().await?;
            let captured = CapturedReceiptBackend::from_receipt(receipt);
            campaign.reconcile_economy(&captured, 8)?;
            transaction.commit().await?;
        }
    "#;
    let (_, captured, violations) = transaction_reconciliation_counts(accepted);
    assert_eq!(captured, 1);
    assert!(violations.is_empty());

    let accepted_after_block = r#"
        async fn apply_capture() {
            let mut transaction = pool
                .begin()
                .await?;
            if receipt.is_none() {
                return Ok(());
            }
            let backend = CapturedReceiptBackend {
                receipt,
                wallet_snapshot,
            };
            campaign.reconcile_economy(&backend, 1)?;
            transaction.commit().await?;
        }
    "#;
    let (_, captured, violations) = transaction_reconciliation_counts(accepted_after_block);
    assert_eq!(captured, 1);
    assert!(violations.is_empty());

    let rejected = [
        r#"
            async fn comment_injection() {
                // CapturedReceiptBackend is not used here.
                let mut transaction = pool.begin().await?;
                campaign.reconcile_economy(&remote, 8)?;
                transaction.commit().await?;
            }
        "#,
        r#"
            async fn string_injection() {
                let _claim = "CapturedReceiptBackend";
                let mut transaction = pool.begin().await?;
                campaign.reconcile_economy(&remote, 8)?;
                transaction.commit().await?;
            }
        "#,
        r#"
            async fn irrelevant_variable() {
                let captured = CapturedReceiptBackend::from_receipt(receipt);
                let mut transaction = pool.begin().await?;
                campaign.reconcile_economy(&remote, 8)?;
                transaction.commit().await?;
            }
        "#,
        r#"
            async fn mixed_receivers() {
                let captured = CapturedReceiptBackend::from_receipt(receipt);
                let mut transaction = pool.begin().await?;
                campaign.reconcile_economy(&captured, 8)?;
                campaign.reconcile_economy(&remote, 8)?;
                transaction.commit().await?;
            }
        "#,
        r#"
            async fn closed_scope_binding() {
                let mut transaction = pool.begin().await?;
                {
                    let backend = CapturedReceiptBackend::from_receipt(receipt);
                }
                campaign.reconcile_economy(&backend, 8)?;
                transaction.commit().await?;
            }
        "#,
        r#"
            async fn shadowed_binding() {
                let mut transaction = pool.begin().await?;
                let backend = CapturedReceiptBackend::from_receipt(receipt);
                let backend = remote;
                campaign.reconcile_economy(&backend, 8)?;
                transaction.commit().await?;
            }
        "#,
    ];
    for source in rejected {
        let (_, _, violations) = transaction_reconciliation_counts(source);
        assert!(!violations.is_empty(), "unsafe fixture unexpectedly passed");
    }
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
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
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
    let plan =
        fs::read_to_string(repository.join("docs/development/trnm-world-development-plan-v3.md"))
            .expect("read current World plan");
    let gates = fs::read_to_string(repository.join("docs/status/world-gates-v1.json"))
        .expect("read World gate registry");

    assert!(plan.contains("public online remains **NO-GO**"));
    assert!(plan.contains("public player market remains **disabled**"));
    assert!(gates.contains("\"id\": \"public_online\""));
    assert!(gates.contains("\"status\": \"no_go\""));
    assert!(gates.contains("\"id\": \"public_player_market\""));
    assert!(gates.contains("\"status\": \"disabled\""));
}