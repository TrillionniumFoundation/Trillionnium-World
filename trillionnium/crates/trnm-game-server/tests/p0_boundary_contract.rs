use std::{
    fs,
    ops::Range,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::Duration,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BindingKind {
    CapturedReceipt,
    Other,
}

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

fn identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn lifetime_or_label(bytes: &[u8], quote: usize) -> bool {
    let mut cursor = quote + 1;
    if !bytes
        .get(cursor)
        .is_some_and(|byte| identifier_start(*byte))
    {
        return false;
    }
    cursor += 1;
    while bytes.get(cursor).is_some_and(|byte| identifier_byte(*byte)) {
        cursor += 1;
    }
    bytes.get(cursor) != Some(&b'\'')
}

fn character_literal_end(bytes: &[u8], quote: usize) -> Option<usize> {
    let mut cursor = quote + 1;
    let mut escaped = false;
    while let Some(byte) = bytes.get(cursor).copied() {
        if !escaped && matches!(byte, b'\n' | b'\r') {
            return None;
        }
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == b'\'' {
            return Some(cursor);
        }
        cursor += 1;
    }
    None
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
            for index in cursor..payload_start {
                blank(&mut masked, index);
            }
            cursor = payload_start;
            while cursor < bytes.len() {
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

        let character_quote = if bytes[cursor..].starts_with(b"b'") {
            Some(cursor + 1)
        } else if bytes[cursor] == b'\'' && !lifetime_or_label(bytes, cursor) {
            Some(cursor)
        } else {
            None
        };
        if let Some(quote) = character_quote {
            if let Some(end) = character_literal_end(bytes, quote) {
                for index in cursor..=end {
                    blank(&mut masked, index);
                }
                cursor = end + 1;
            } else {
                // A malformed quote must not trap the source gate. Preserve it as
                // structural input and advance by one byte so the scan is total.
                cursor += 1;
            }
            continue;
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
        if start > 0 && identifier_byte(bytes[start - 1]) {
            cursor = start + 3;
            continue;
        }
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
    let mut stack = Vec::new();
    for (offset, byte) in bytes.get(open + 1..)?.iter().copied().enumerate() {
        match byte {
            b'(' | b'[' | b'{' => stack.push(byte),
            b')' if stack.is_empty() => {
                return Some(code[open + 1..open + 1 + offset].trim());
            }
            b')' | b']' | b'}' => {
                let expected = match byte {
                    b')' => b'(',
                    b']' => b'[',
                    _ => b'{',
                };
                if stack.pop() != Some(expected) {
                    return None;
                }
            }
            b',' if stack.is_empty() => {
                return Some(code[open + 1..open + 1 + offset].trim());
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

fn skip_ascii_whitespace(bytes: &[u8], mut cursor: usize) -> usize {
    while bytes
        .get(cursor)
        .is_some_and(|byte| byte.is_ascii_whitespace())
    {
        cursor += 1;
    }
    cursor
}

fn parse_identifier(bytes: &[u8], start: usize) -> Option<usize> {
    if !bytes.get(start).is_some_and(|byte| identifier_start(*byte)) {
        return None;
    }
    let mut cursor = start + 1;
    while bytes.get(cursor).is_some_and(|byte| identifier_byte(*byte)) {
        cursor += 1;
    }
    Some(cursor)
}

fn keyword_at(bytes: &[u8], start: usize, keyword: &[u8]) -> bool {
    bytes.get(start..start + keyword.len()) == Some(keyword)
        && (start == 0 || !identifier_byte(bytes[start - 1]))
        && bytes
            .get(start + keyword.len())
            .is_none_or(|byte| !identifier_byte(*byte))
}

fn consume_balanced_group(bytes: &[u8], open: usize) -> Option<usize> {
    let opener = *bytes.get(open)?;
    let expected_close = match opener {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        _ => return None,
    };
    let mut stack = vec![expected_close];
    let mut cursor = open + 1;
    while let Some(byte) = bytes.get(cursor).copied() {
        match byte {
            b'(' => stack.push(b')'),
            b'[' => stack.push(b']'),
            b'{' => stack.push(b'}'),
            b')' | b']' | b'}' => {
                if stack.pop() != Some(byte) {
                    return None;
                }
                if stack.is_empty() {
                    return Some(cursor + 1);
                }
            }
            _ => {}
        }
        cursor += 1;
    }
    None
}

fn only_ascii_whitespace(bytes: &[u8], cursor: usize) -> bool {
    bytes[cursor..]
        .iter()
        .all(|byte| byte.is_ascii_whitespace())
}

fn exact_captured_backend_constructor(value: &str) -> bool {
    let bytes = value.as_bytes();
    let mut cursor = skip_ascii_whitespace(bytes, 0);

    loop {
        let Some(identifier_end) = parse_identifier(bytes, cursor) else {
            return false;
        };
        let identifier = &value[cursor..identifier_end];
        cursor = skip_ascii_whitespace(bytes, identifier_end);

        if identifier == "CapturedReceiptBackend" {
            if bytes.get(cursor) == Some(&b'{') {
                return consume_balanced_group(bytes, cursor)
                    .is_some_and(|end| only_ascii_whitespace(bytes, end));
            }
            if !bytes
                .get(cursor..)
                .is_some_and(|remaining| remaining.starts_with(b"::"))
            {
                return false;
            }
            cursor = skip_ascii_whitespace(bytes, cursor + 2);
            let Some(method_end) = parse_identifier(bytes, cursor) else {
                return false;
            };
            cursor = skip_ascii_whitespace(bytes, method_end);
            if bytes.get(cursor) != Some(&b'(') {
                return false;
            }
            return consume_balanced_group(bytes, cursor)
                .is_some_and(|end| only_ascii_whitespace(bytes, end));
        }

        if !bytes
            .get(cursor..)
            .is_some_and(|remaining| remaining.starts_with(b"::"))
        {
            return false;
        }
        cursor = skip_ascii_whitespace(bytes, cursor + 2);
    }
}

fn statement_initializer_end(code: &str, start: usize) -> Option<usize> {
    let bytes = code.as_bytes();
    let mut stack = Vec::new();
    let mut cursor = start;
    while let Some(byte) = bytes.get(cursor).copied() {
        match byte {
            b'(' => stack.push(b')'),
            b'[' => stack.push(b']'),
            b'{' => stack.push(b'}'),
            b')' | b']' | b'}' => {
                if stack.pop() != Some(byte) {
                    return None;
                }
            }
            b';' if stack.is_empty() => return Some(cursor),
            _ => {}
        }
        cursor += 1;
    }
    None
}

fn simple_let_binding(code: &str, start: usize) -> Option<(usize, String, BindingKind)> {
    let bytes = code.as_bytes();
    if !keyword_at(bytes, start, b"let") {
        return None;
    }
    let mut cursor = start + 3;
    if !bytes
        .get(cursor)
        .is_some_and(|byte| byte.is_ascii_whitespace())
    {
        return None;
    }
    cursor = skip_ascii_whitespace(bytes, cursor);
    if keyword_at(bytes, cursor, b"mut") {
        cursor = skip_ascii_whitespace(bytes, cursor + 3);
    }

    let name_start = cursor;
    let name_end = parse_identifier(bytes, name_start)?;
    let name = code[name_start..name_end].to_string();
    cursor = name_end;

    let initializer_start = loop {
        let byte = *bytes.get(cursor)?;
        if byte == b';' {
            return None;
        }
        if byte == b'='
            && bytes.get(cursor + 1) != Some(&b'=')
            && cursor.checked_sub(1).and_then(|index| bytes.get(index)) != Some(&b'=')
        {
            break cursor + 1;
        }
        cursor += 1;
    };

    let initializer_end = statement_initializer_end(code, initializer_start)?;
    let initializer = &code[initializer_start..initializer_end];
    let kind = if exact_captured_backend_constructor(initializer) {
        BindingKind::CapturedReceipt
    } else {
        BindingKind::Other
    };
    Some((initializer_end + 1, name, kind))
}

fn simple_assignment(code: &str, start: usize) -> Option<(usize, String, BindingKind)> {
    let bytes = code.as_bytes();
    if start > 0 && identifier_byte(bytes[start - 1]) {
        return None;
    }
    let name_end = parse_identifier(bytes, start)?;
    let name = code[start..name_end].to_string();
    let cursor = skip_ascii_whitespace(bytes, name_end);
    if bytes.get(cursor) != Some(&b'=')
        || matches!(bytes.get(cursor + 1), Some(b'=') | Some(b'>'))
        || cursor.checked_sub(1).and_then(|index| bytes.get(index)) == Some(&b'=')
    {
        return None;
    }

    let initializer_start = cursor + 1;
    let initializer_end = statement_initializer_end(code, initializer_start)?;
    let kind = if exact_captured_backend_constructor(&code[initializer_start..initializer_end]) {
        BindingKind::CapturedReceipt
    } else {
        BindingKind::Other
    };
    Some((initializer_end + 1, name, kind))
}

fn record_assignment(scopes: &mut [Vec<(String, BindingKind)>], name: String, kind: BindingKind) {
    for scope in scopes.iter_mut().rev() {
        if scope.iter().rev().any(|(candidate, _)| candidate == &name) {
            scope.push((name, kind));
            return;
        }
    }
    if let Some(scope) = scopes.last_mut() {
        scope.push((name, kind));
    }
}

fn captured_backend_binding_before(code: &str, call_start: usize, identifier: &str) -> bool {
    let prefix = &code[..call_start];
    let bytes = prefix.as_bytes();
    let mut scopes: Vec<Vec<(String, BindingKind)>> = vec![Vec::new()];
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
                if let Some((end, name, kind)) = simple_let_binding(prefix, cursor) {
                    scopes
                        .last_mut()
                        .expect("scope stack is never empty")
                        .push((name, kind));
                    cursor = end;
                } else {
                    cursor += 1;
                }
            }
            _ if identifier_start(bytes[cursor]) => {
                if let Some((end, name, kind)) = simple_assignment(prefix, cursor) {
                    record_assignment(&mut scopes, name, kind);
                    cursor = end;
                } else {
                    cursor += 1;
                }
            }
            _ => cursor += 1,
        }
    }

    for scope in scopes.iter().rev() {
        if let Some((_, kind)) = scope.iter().rev().find(|(name, _)| name == identifier) {
            return *kind == BindingKind::CapturedReceipt;
        }
    }
    false
}

fn borrowed_identifier(argument: &str) -> Option<&str> {
    let argument = argument.trim().strip_prefix('&')?.trim_start();
    let argument = argument.strip_prefix("mut ").unwrap_or(argument).trim();
    (!argument.is_empty()
        && identifier_start(*argument.as_bytes().first()?)
        && argument.bytes().all(identifier_byte))
    .then_some(argument)
}

fn call_uses_captured_backend(code: &str, call_start: usize, argument: &str) -> bool {
    let borrowed = argument.trim().strip_prefix('&').map(str::trim_start);
    borrowed.is_some_and(exact_captured_backend_constructor)
        || borrowed_identifier(argument)
            .is_some_and(|identifier| captured_backend_binding_before(code, call_start, identifier))
}

fn transaction_reconciliation_counts(source: &str) -> (usize, usize, Vec<String>) {
    let code = mask_rust_comments_and_literals(source);
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
        let code_body = &code[range];
        let compact_code = code_body
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>();
        let has_transaction =
            compact_code.contains(".begin().await") || compact_code.contains("Transaction<'");
        let has_external = external_markers
            .iter()
            .any(|marker| compact_code.contains(marker));
        let calls = reconciliation_calls(code_body);
        reconcile_count += calls.len();

        if has_transaction {
            for (call_start, argument) in calls {
                if call_uses_captured_backend(code_body, call_start, argument) {
                    captured_transaction_reconcile_count += 1;
                } else {
                    violations.push(format!(
                        "{name} reconciles economy under a transaction without passing an exact CapturedReceiptBackend value"
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
    let accepted = [
        r#"
            async fn associated_constructor() {
                let mut transaction = pool.begin().await?;
                let backend = CapturedReceiptBackend::from_receipt(receipt);
                campaign.reconcile_economy(&backend, 8)?;
                transaction.commit().await?;
            }
        "#,
        r#"
            async fn qualified_constructor() {
                let mut transaction = pool.begin().await?;
                let backend = crate::CapturedReceiptBackend::from_receipt(receipt);
                campaign.reconcile_economy(&backend, 8)?;
                transaction.commit().await?;
            }
        "#,
        r#"
            async fn struct_constructor() {
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
        "#,
        r#"
            async fn inline_constructor() {
                let mut transaction = pool.begin().await?;
                campaign.reconcile_economy(
                    &CapturedReceiptBackend::from_receipt(receipt),
                    1,
                )?;
                transaction.commit().await?;
            }
        "#,
    ];
    for source in accepted {
        let (_, captured, violations) = transaction_reconciliation_counts(source);
        assert_eq!(captured, 1, "captured fixture was not recognized");
        assert!(
            violations.is_empty(),
            "captured fixture failed: {violations:?}"
        );
    }

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
        r#"
            async fn reassigned_binding() {
                let mut transaction = pool.begin().await?;
                let mut backend = CapturedReceiptBackend::from_receipt(receipt);
                backend = remote;
                campaign.reconcile_economy(&backend, 8)?;
                transaction.commit().await?;
            }
        "#,
        r#"
            async fn prefix_impostor() {
                let mut transaction = pool.begin().await?;
                let backend = CapturedReceiptBackendRemote::new(receipt);
                campaign.reconcile_economy(&backend, 8)?;
                transaction.commit().await?;
            }
        "#,
        r#"
            async fn qualified_decoy_argument() {
                let mut transaction = pool.begin().await?;
                campaign.reconcile_economy(
                    &{
                        let _decoy =
                            crate::CapturedReceiptBackend::from_receipt(receipt);
                        remote
                    },
                    8,
                )?;
                transaction.commit().await?;
            }
        "#,
        r#"
            async fn qualified_decoy_initializer() {
                let mut transaction = pool.begin().await?;
                let backend = {
                    let _decoy =
                        crate::CapturedReceiptBackend::from_receipt(receipt);
                    remote
                };
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
fn literal_masker_is_total_and_preserves_lifetime_syntax() {
    for source in [
        "type Ref<'a> = &'a ();",
        "type Ref<'a> = &'a ();\n",
        "fn borrow<'a>(value: &'a str) -> &'a str { value }",
        "fn label() { 'outer: loop { break 'outer; } }",
        "'",
        "'\\",
        "'unterminated",
    ] {
        let source = source.to_string();
        let expected_len = source.len();
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let _ = sender.send(mask_rust_comments_and_literals(&source));
        });
        let masked = receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("literal masker failed to make progress");
        assert_eq!(masked.len(), expected_len);
    }

    let source =
        "fn chars<'a>(value: &'a str) { let a = 'x'; let b = '\\''; let c = 'é'; let d = b'z'; }";
    let masked = mask_rust_comments_and_literals(source);
    assert!(masked.contains("'a"));
    assert!(!masked.contains("'x'"));
    assert!(!masked.contains("'\\''"));
    assert!(!masked.contains("'é'"));
    assert!(!masked.contains("b'z'"));
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
