#!/usr/bin/env python3
"""Conservative source-static checks for directly compiled settlement phases.

This is not a Rust compiler, call-graph proof, database test or execution record.
Fixture mode checks only the explicitly supplied source directory. Full mode
also checks the reviewed source/migration/test/workflow contract surfaces.
"""
from __future__ import annotations

import argparse
from dataclasses import dataclass
from pathlib import Path, PurePosixPath
import re
import sys
import tomllib

MAX_FILE_BYTES = 1024 * 1024
MAX_BUNDLE_BYTES = 8 * 1024 * 1024
MAX_FILES = 256
MAX_TOKENS = 1_000_000
MAX_NESTING = 256
REMOTE_METHODS = {"authorize_settlement_intent", "submit_authorized_settlement_intent"}
RAW_STRING = re.compile(r'(?:br|cr|r)(#{0,255})"')
IDENT = re.compile(r'(?:r#)?[A-Za-z_][A-Za-z_0-9]*')
CHAR = re.compile(r"(?:b)?'(?:[^'\\\r\n]|\\(?:[nrt0'\"\\]|x[0-9A-Fa-f]{2}|u\{[0-9A-Fa-f_]{1,8}\}))'")
SQL_LOCK = re.compile(r'\bfor\s+(?:(?:no\s+key|key)\s+)?(?:update|share)\b', re.I)


class BoundaryFailure(ValueError):
    pass


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise BoundaryFailure(reason)


@dataclass(frozen=True)
class Token:
    kind: str
    value: str


def quoted(text: str, start: int) -> tuple[str, int]:
    """Decode ordinary Rust string escapes used in checked source and SQL."""
    out: list[str] = []
    i = start + 1
    simple = {'n': '\n', 'r': '\r', 't': '\t', '0': '\0', '"': '"', "'": "'", '\\': '\\'}
    while i < len(text):
        ch = text[i]
        if ch == '"':
            return ''.join(out), i + 1
        if ch != '\\':
            out.append(ch)
            i += 1
            continue
        i += 1
        require(i < len(text), 'unterminated string escape')
        ch = text[i]
        if ch in simple:
            out.append(simple[ch])
            i += 1
        elif ch == 'x':
            value = text[i + 1:i + 3]
            require(re.fullmatch(r'[0-9A-Fa-f]{2}', value) is not None, 'invalid hex escape')
            out.append(chr(int(value, 16)))
            i += 3
        elif ch == 'u':
            match = re.match(r'u\{([0-9A-Fa-f_]{1,8})\}', text[i:])
            require(match is not None, 'invalid Unicode escape')
            digits = match[1].replace('_', '')
            require(1 <= len(digits) <= 6, 'invalid Unicode scalar width')
            scalar = int(digits, 16)
            require(scalar <= 0x10FFFF and not 0xD800 <= scalar <= 0xDFFF, 'invalid Unicode scalar')
            out.append(chr(scalar))
            i += len(match[0])
        elif ch in '\r\n':
            if ch == '\r':
                require(text[i:i + 2] == '\r\n', 'invalid continuation')
                i += 1
            i += 1
            while i < len(text) and text[i] in ' \t\r\n':
                i += 1
        else:
            raise BoundaryFailure('unsupported string escape')
    raise BoundaryFailure('unterminated string literal')


def tokens(text: str) -> list[Token]:
    """Tokenize the bounded source subset, preserving strings as non-code tokens.

    Handles nested block comments, raw/byte strings, chars and lifetimes. It
    deliberately does not accept a tokenization pass as Rust syntax validation.
    """
    require(len(text.encode('utf-8')) <= MAX_BUNDLE_BYTES, 'source byte budget exceeded')
    out: list[Token] = []
    i = 0
    while i < len(text):
        require(len(out) < MAX_TOKENS, 'source token budget exceeded')
        if text[i].isspace():
            i += 1
            continue
        if text.startswith('//', i):
            end = text.find('\n', i + 2)
            i = len(text) if end < 0 else end + 1
            continue
        if text.startswith('/*', i):
            depth = 1
            i += 2
            while depth:
                require(i < len(text), 'unterminated block comment')
                if text.startswith('/*', i):
                    depth += 1
                    require(depth <= MAX_NESTING, 'comment depth budget exceeded')
                    i += 2
                elif text.startswith('*/', i):
                    depth -= 1
                    i += 2
                else:
                    i += 1
            continue
        raw = RAW_STRING.match(text, i)
        if raw:
            close = '"' + raw[1]
            end = text.find(close, raw.end())
            require(end >= 0, 'unterminated raw string')
            out.append(Token('string', text[raw.end():end]))
            i = end + len(close)
            continue
        char = CHAR.match(text, i)
        if char:
            out.append(Token('char', char[0]))
            i = char.end()
            continue
        if text[i] == '"' or text[i:i + 2] in {'b"', 'c"'}:
            start = i if text[i] == '"' else i + 1
            value, i = quoted(text, start)
            out.append(Token('string', value))
            continue
        identifier = IDENT.match(text, i)
        if identifier:
            value = identifier[0]
            out.append(Token('id', value[2:] if value.startswith('r#') else value))
            i = identifier.end()
            continue
        out.append(Token('punct', text[i]))
        i += 1
    return out


def is_token(token: Token, value: str) -> bool:
    return token.kind in {'id', 'punct'} and token.value == value


def group_end(stream: list[Token], start: int) -> int:
    closing = {'(': ')', '[': ']', '{': '}'}
    require(stream[start].kind == 'punct' and stream[start].value in closing, 'expected token group')
    stack: list[str] = []
    for i in range(start, len(stream)):
        t = stream[i]
        if t.kind != 'punct':
            continue
        if t.value in closing:
            stack.append(closing[t.value])
            require(len(stack) <= MAX_NESTING, 'token group depth exceeded')
        elif t.value in closing.values():
            require(bool(stack) and stack.pop() == t.value, 'mismatched source delimiter')
            if not stack:
                return i
    raise BoundaryFailure('unterminated source group')


def safe_relative(value: str) -> str:
    p = PurePosixPath(value)
    require(bool(p.parts) and not p.is_absolute() and p.as_posix() == value
            and not any(x in {'..', '.git'} for x in p.parts)
            and '\\' not in value and ':' not in value and '\0' not in value,
            'unsafe source path')
    return value


class SourceBundle:
    def __init__(self, root: Path):
        require(not root.is_symlink() and root.is_dir(), 'source root missing or linked')
        self.root = root.resolve()
        self.seen: set[str] = set()
        self.bytes = 0
        self.token_count = 0

    def read(self, relative: str) -> str:
        safe_relative(relative)
        p = self.root
        for part in PurePosixPath(relative).parts:
            p /= part
            require(not p.is_symlink(), 'source path is linked')
        require(p.is_file(), f'missing source: {relative}')
        with p.open('rb') as handle:
            data = handle.read(MAX_FILE_BYTES + 1)
        require(0 < len(data) <= MAX_FILE_BYTES and bool(data.strip()), 'empty or oversized source')
        return data.decode('utf-8')

    def bundle(self, relative: str) -> list[Token]:
        relative = safe_relative(relative)
        require(relative not in self.seen, 'duplicate or cyclic semantic include')
        require(len(self.seen) < MAX_FILES, 'included source count exceeded')
        self.seen.add(relative)
        text = self.read(relative)
        self.bytes += len(text.encode('utf-8'))
        require(self.bytes <= MAX_BUNDLE_BYTES, 'included source byte budget exceeded')
        stream = tokens(text)
        self.token_count += len(stream)
        require(self.token_count <= MAX_TOKENS, 'included token budget exceeded')
        result: list[Token] = []
        i = 0
        while i < len(stream):
            if is_token(stream[i], 'include') and i + 1 < len(stream) and is_token(stream[i + 1], '!'):
                require(i + 2 < len(stream) and is_token(stream[i + 2], '('), 'unsupported semantic include delimiter')
                end = group_end(stream, i + 2)
                args = stream[i + 3:end]
                if args and is_token(args[-1], ','):
                    args = args[:-1]
                if len(args) == 1 and args[0].kind == 'string':
                    child = safe_relative(args[0].value)
                    child = (PurePosixPath(relative).parent / child).as_posix()
                else:
                    expected = ['concat', '!', '(', 'env', '!', '(', None, ')', ',', None, ')']
                    require(len(args) == len(expected) and all(
                        target is None or is_token(token, target) for token, target in zip(args, expected)),
                        'unsupported semantic include expression')
                    require(args[6] == Token('string', 'CARGO_MANIFEST_DIR') and args[9].kind == 'string',
                            'semantic include environment is not crate-bound')
                    child = args[9].value
                    require(child.startswith('/src/'), 'crate include must remain under src')
                    child = safe_relative(child[1:])
                result.extend(self.bundle(child))
                i = end + 1
            else:
                result.append(stream[i])
                i += 1
        return result


def function(stream: list[Token], name: str) -> list[Token]:
    found: list[list[Token]] = []
    i = 0
    while i < len(stream):
        # Function-like text inside a macro definition/invocation does not
        # establish an ordinary phase implementation.
        if is_token(stream[i], 'macro_rules') and i + 3 < len(stream) and is_token(stream[i + 1], '!'):
            i = group_end(stream, i + 3) + 1
            continue
        if stream[i].kind == 'id' and i + 2 < len(stream) and is_token(stream[i + 1], '!') and stream[i + 2].value in {'(', '[', '{'}:
            i = group_end(stream, i + 2) + 1
            continue
        if is_token(stream[i], 'fn') and i + 1 < len(stream) and is_token(stream[i + 1], name):
            begin = i
            i += 2
            while i < len(stream) and not is_token(stream[i], '{'):
                require(not is_token(stream[i], ';'), f'{name} has no implementation')
                if stream[i].kind == 'punct' and stream[i].value in {'(', '['}:
                    i = group_end(stream, i) + 1
                else:
                    i += 1
            require(i < len(stream), f'unterminated {name} signature')
            end = group_end(stream, i)
            found.append(stream[begin:end + 1])
            i = end + 1
        else:
            i += 1
    require(len(found) == 1, f'expected one ordinary phase implementation: {name}')
    return found[0]


def has_method(stream: list[Token], names: set[str]) -> bool:
    return any(is_token(a, '.') and b.kind == 'id' and b.value in names and is_token(c, '(')
               for a, b, c in zip(stream, stream[1:], stream[2:]))


def remote_calls(stream: list[Token]) -> set[str]:
    # Conservative direct symbol check, including qualified/raw identifiers.
    # Indirect aliases and arbitrary transitive helper effects need Rust/runtime
    # evidence; this scanner makes no such proof claim.
    return {t.value for t in stream if t.kind == 'id' and t.value in REMOTE_METHODS}


def owns_transaction(stream: list[Token]) -> bool:
    return has_method(stream, {'begin', 'begin_with', 'transaction'}) or any(is_token(t, 'Transaction') for t in stream)


def scan_phases(stream: list[Token]) -> None:
    for name in ('capture_match', 'apply_capture'):
        phase = function(stream, name)
        require(owns_transaction(phase), f'{name} no longer owns a short transaction')
        require(not remote_calls(phase), f'{name} contains direct signer/CEX execution')
    execute = function(stream, 'process_claimed_job')
    require(not owns_transaction(execute), 'remote execution owns a transaction')
    require(not any(t.kind == 'string' and SQL_LOCK.search(t.value) for t in execute), 'remote execution contains a row-lock query')
    calls = {a.value for a, b in zip(execute, execute[1:])
             if a.kind == 'id' and a.value in REMOTE_METHODS and is_token(b, '(')}
    require(calls == REMOTE_METHODS, 'remote execution lost explicit signer/CEX calls')


def source_markers(text: str, expected: tuple[str, ...], label: str) -> None:
    missing = [x for x in expected if x not in text]
    require(not missing, f'{label} missing source markers: {missing}')


def workflow_contract(root: Path) -> None:
    reader = SourceBundle(root)
    paths = ('.github/workflows/trnm-world-gap-closure-v4.yml', '.github/workflows/trnm-world-v4-final-gates.yml')
    contents = [reader.read(p) for p in paths if (root / p).exists()]
    require(bool(contents), 'permanent V4 qualification workflow missing')
    lines = [line.strip() for text in contents for line in text.splitlines() if not line.lstrip().startswith('#')]
    require(not any(re.fullmatch(r'contents:\s*write', line) for line in lines), 'qualification workflow has source write permission')
    commands = [line for line in lines if line.startswith('cargo ')]
    targets = [line for line in commands if '-p trnm-game-server' in line or '--workspace' in line]
    tests = [line for line in targets if line.startswith('cargo test ') and '--locked' in line]
    all_targets = any('--all-targets' in line for line in tests)
    for target in ('settlement_game_server_boundary', 'settlement_operator_controls_database'):
        require(all_targets or any('--test ' + target in line for line in tests), f'workflow does not select {target}')
    require(any(line.startswith('cargo fmt ') and '-- --check' in line for line in commands), 'workflow format check missing')
    require(any(line.startswith('cargo clippy ') and '--all-targets' in line and '-- -D warnings' in line for line in targets),
            'workflow strict all-target Clippy missing')


def check_repository(root: Path, full: bool) -> dict:
    crate = root / 'trillionnium/crates/trnm-game-server'
    reader = SourceBundle(crate)
    for relative in ('build.rs', 'src/lib.rs.in', 'src/settlement_worker.rs.in'):
        p = crate / relative
        require(not p.exists() and not p.is_symlink(), f'semantic template/generator remains: {relative}')
    manifest = tomllib.loads(reader.read('Cargo.toml'))
    require(manifest.get('package', {}).get('build', False) is False, 'Cargo build script remains enabled')
    game = reader.bundle('src/lib.rs')
    worker_reader = SourceBundle(crate)
    worker = worker_reader.bundle('src/settlement_worker.rs')
    for stream in (game, worker):
        require(not any(t.value in {'OUT_DIR', 'trnm_game_server_lib_generated.rs', 'trnm_settlement_worker_generated.rs'} for t in stream),
                'generated semantic authority returned')
    require(not remote_calls(game), 'game-server library invokes signer/CEX settlement directly')
    compatibility = function(game, 'settle_pending_matches')
    require(any(is_token(t, 'Err') for t in compatibility), 'legacy settlement API no longer fails closed')
    require(any(t.kind == 'string' and 'terminal settlement is owned by trnm-settlement-worker' in t.value for t in compatibility),
            'legacy settlement fail-close reason missing')
    scan_phases(worker)
    values = {t.value for t in worker if t.kind == 'id'}
    for key in ('expected_campaign_state_hash', 'authorization_request_id', 'entitlement_issued_at_epoch',
                'entitlement_expires_at_epoch', 'entitlement_nonce', 'ReceiptProgressionClass', 'RecoverableHold'):
        require(key in values, f'worker durable identity marker missing: {key}')
    require(any('campaign_applied_at' in t.value for t in worker if t.kind == 'string'), 'worker apply timestamp binding missing')
    # A file include or a test string does not establish migration registration.
    # Bind exact entries to the ordinary registration functions instead.
    game_registry = function(game, 'run_database_migrations')
    worker_registry = function(worker, 'apply_worker_migrations_locked')
    worker_registry_v2 = function(worker, 'apply_worker_migrations_v2_locked')
    for marker in ('0016_online_settlement_outbox_v1', '0017_online_settlement_worker_runtime_v1',
                   '0018_online_settlement_operator_controls_v1', '0019_online_settlement_quarantine_v1'):
        selected_worker = worker_registry_v2 if marker.startswith(('0018_', '0019_')) else worker_registry
        for stream in (game_registry, selected_worker):
            require(Token('string', marker) in stream, f'migration registry missing {marker}')
    if full:
        cex = reader.read('src/cex.rs')
        cex_tokens = tokens(cex)
        function(cex_tokens, 'lookup_signer_receipt')
        function(cex_tokens, 'lookup_authorized_settlement_receipt')
        source_markers(cex, ('CEX_SETTLEMENT_RECEIPT_LOOKUP_PATH', 'Err(SETTLEMENT_OUTBOX_REQUIRED.to_string())'), 'CEX recovery')
        require(not any(is_token(t, 'blocking_client') or is_token(t, 'blocking') for t in cex_tokens), 'blocking CEX client returned')
        signer = reader.read('src/bin/trnm-entitlement-signer.rs')
        require(Token('string', '/v1/signer/receipts/:request_id') in tokens(signer), 'signer receipt route missing')
        outbox = reader.read('migrations/0016_online_settlement_outbox_v1.sql').lower()
        require('on delete restrict' in outbox and 'on delete cascade' not in outbox, 'nonrestrictive settlement evidence foreign keys')
        worker_sql = reader.read('migrations/0017_online_settlement_worker_runtime_v1.sql')
        source_markers(worker_sql, (
            'trnm_online_remote_request_id_v1', 'pg_catalog.sha256(', 'trnm_online_claim_settlement_job_v2',
            'trnm_online_store_settlement_authorization_v1', 'trnm_online_begin_settlement_remote_attempt_v1',
            'trnm_online_complete_settlement_job_v1', 'trnm_online_retry_settlement_job_v1',
            'trnm_online_dead_letter_settlement_job_v1', 'lease_expires_at > pg_catalog.clock_timestamp()',
            "when job.state = 'succeeded' then 'pending_apply'", "when job.campaign_applied_at is not null then 'applied'",
            'trnm_online_settlement_metrics_v1'), 'worker migration')
        operator = ' '.join(reader.read('migrations/0018_online_settlement_operator_controls_v1.sql').lower().split())
        source_markers(operator, ('trnm_online_settlement_operator_replay', 'trnm_online_settlement_operator_replay_requests',
                                 'before update or delete', 'before truncate', 'remote_attempts', 'retention'), 'operator migration')
        boundary = tokens(reader.read('tests/settlement_game_server_boundary.rs'))
        function(boundary, 'direct_sources_never_restore_generated_authority')
        function(boundary, 'game_server_does_not_execute_terminal_economy_settlement')
        capture = reader.read('tests/settlement_capture_commit_boundary.rs')
        source_markers(capture, ('capture', 'claim', 'remote'), 'capture commit test')
        operator_test = tokens(reader.read('tests/settlement_operator_controls_database.rs'))
        function(operator_test, 'settlement_operator_replay_is_exact_audited_one_attempt_and_append_only')
        require(any(is_token(t, 'remote_attempts') or t.kind == 'string' and 'remote_attempts' in t.value for t in operator_test),
                'operator test remote attempt assertion missing')
        workflow_contract(root)
    return {'game_source_files': len(reader.seen), 'worker_source_files': len(worker_reader.seen),
            'evidence_class': 'source_static', 'rust_execution': False, 'database_execution': False, 'remote_evidence': False}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', nargs='?', choices=('full', 'scan-only'), default='full')
    parser.add_argument('fixture_source', nargs='?', type=Path)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    if args.fixture_source is not None and args.mode != 'scan-only':
        parser.error('a fixture source directory is allowed only in scan-only mode')
    try:
        if args.fixture_source is not None:
            fixture = SourceBundle(args.fixture_source)
            scan_phases(fixture.bundle('lib.rs'))
            scope = 'explicit fixture source only'
        else:
            result = check_repository(args.root, args.mode == 'full')
            scope = f"direct source: {result['game_source_files']} game / {result['worker_source_files']} worker files"
    except (BoundaryFailure, OSError, UnicodeError, ValueError, RecursionError) as error:
        print(f'TRNM settlement transaction boundary: FAIL: {error}', file=sys.stderr)
        return 1
    print(f'TRNM settlement transaction boundary: PASS ({scope}; source-static only; no Rust/database/hosted evidence)')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
