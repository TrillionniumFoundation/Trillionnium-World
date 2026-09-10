#!/usr/bin/env python3
"""Apply the exact P0 scanner repair in a temporary candidate clone."""

from pathlib import Path
import sys

KEYWORD = '''fn keyword_at(bytes: &[u8], start: usize, keyword: &[u8]) -> bool {
    bytes.get(start..start + keyword.len()) == Some(keyword)
        && (start == 0 || !identifier_byte(bytes[start - 1]))
        && bytes
            .get(start + keyword.len())
            .is_none_or(|byte| !identifier_byte(*byte))
}
'''
KEYWORD_PATCHED = KEYWORD + '''
fn statement_let_at(bytes: &[u8], start: usize) -> bool {
    if !keyword_at(bytes, start, b"let") {
        return false;
    }
    let mut cursor = start;
    while cursor > 0 && bytes[cursor - 1].is_ascii_whitespace() {
        cursor -= 1;
    }
    cursor == 0 || matches!(bytes[cursor - 1], b'{' | b'}' | b';')
}
'''
OLD_GUARD = '''            _ if keyword_at(bytes, cursor, b"let") => {
'''
NEW_GUARD = '''            _ if statement_let_at(bytes, cursor) => {
'''
INLINE_FIXTURE = '''        r#"
            async fn inline_constructor() {
                let mut transaction = pool.begin().await?;
'''
PATTERN_FIXTURE = '''        r#"
            async fn pattern_let_before_captured_backend() {
                let mut transaction = pool.begin().await?;
                if let Err(error) = validate_receipt(receipt) {
                    return Err(error);
                }
                let backend = CapturedReceiptBackend {
                    receipt,
                    wallet_snapshot,
                };
                campaign.reconcile_economy(&backend, 1)?;
                transaction.commit().await?;
            }
        "#,
''' + INLINE_FIXTURE


def replace_once(text: str, old: str, new: str, label: str) -> str:
    if text.count(old) != 1:
        raise SystemExit(f"{label} anchor did not match exactly once")
    return text.replace(old, new)


def main() -> int:
    if len(sys.argv) != 2:
        return 64
    path = Path(sys.argv[1])
    text = path.read_text(encoding="utf-8")
    text = replace_once(text, KEYWORD, KEYWORD_PATCHED, "keyword")
    text = replace_once(text, OLD_GUARD, NEW_GUARD, "statement-let guard")
    text = replace_once(text, INLINE_FIXTURE, PATTERN_FIXTURE, "accepted fixture")
    path.write_text(text, encoding="utf-8", newline="\n")
    print("WORLD_P0_STATEMENT_LET_PATCH=PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
