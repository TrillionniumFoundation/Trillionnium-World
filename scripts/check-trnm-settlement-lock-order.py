#!/usr/bin/env python3
"""Guard known settlement row-lock order in source; not a database deadlock proof."""
from __future__ import annotations

import argparse
import importlib.util
import json
from pathlib import Path
import re
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
MODULE = ROOT / "scripts/check-trnm-settlement-transaction-boundary.py"
spec = importlib.util.spec_from_file_location("trnm_settlement_lexical_boundary", MODULE)
if spec is None or spec.loader is None:
    raise RuntimeError("settlement source lexer is unavailable")
lexer = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = lexer
spec.loader.exec_module(lexer)

SOURCE = "trillionnium/crates/trnm-game-server/src/settlement_worker_legacy.rs"
TEST_SOURCE = "trillionnium/crates/trnm-game-server/src/settlement_lock_order_tests.rs"
CONTRACT = "docs/database/trnm-world-lock-order-v2.json"
DOCS = ("docs/database/trnm-world-lock-order-v1.md", "docs/database/trnm-world-postgres-contract-v1.md")
ORDER = ("migration", "host", "fleet", "match", "terminal_ack", "members", "campaigns", "capture", "serialization", "jobs", "operator")
PHASE = ["match", "terminal_ack", "members", "campaigns", "capture", "jobs"]
FIELDS = {"schema", "status", "owner", "source_basis", "release_effect", "production_authorization", "order", "checked_phases", "unlocked_hint", "scope", "remaining_evidence"}


class LockOrderFailure(ValueError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise LockOrderFailure(message)


def unique_object(pairs: list[tuple[str, object]]) -> dict:
    result = {}
    for key, value in pairs:
        require(key not in result, f"duplicate contract key: {key}")
        result[key] = value
    return result


def reject_constant(value: str) -> None:
    raise LockOrderFailure(f"non-finite JSON value: {value}")


def code_equal(stream, start: int, values: tuple[str, ...]) -> bool:
    part = stream[start:start + len(values)]
    return len(part) == len(values) and all(lexer.is_token(token, value) for token, value in zip(part, values))


def call_position(stream, name: str) -> int:
    matches = [i for i in range(len(stream) - 1) if code_equal(stream, i, (name, "(")) and not (i and lexer.is_token(stream[i - 1], "fn"))]
    require(len(matches) == 1, f"expected one direct call to {name}")
    return matches[0]


def query(stream, prefix: str):
    matches = []
    for i, token in enumerate(stream):
        if token.kind != "string":
            continue
        normalized = " ".join(token.value.lower().split())
        if not normalized.startswith(prefix):
            continue
        # A bare/comment/string declaration is not an executed SQL statement.
        if not i or not lexer.is_token(stream[i - 1], "("):
            continue
        before = stream[max(0, i - 16):i - 1]
        if not any(t.kind == "id" and t.value in {"query", "query_scalar"} for t in before):
            continue
        end = next((j for j in range(i + 1, len(stream)) if lexer.is_token(stream[j], ";")), len(stream))
        chain = stream[i + 1:end]
        if not lexer.has_method(chain, {"fetch_optional", "fetch_all", "fetch_one", "execute"}):
            continue
        require(any(t.kind == "id" and t.value == "await" for t in chain), "SQL query is not awaited")
        matches.append((i, normalized, chain))
    require(len(matches) == 1, f"expected one executed SQL query: {prefix}")
    return matches[0]


def contract_and_docs(reader) -> dict:
    data = json.loads(reader.read(CONTRACT), object_pairs_hook=unique_object, parse_constant=reject_constant)
    require(type(data) is dict and set(data) == FIELDS, "contract fields changed")
    require(data["schema"] == "trnm_world_settlement_lock_order_v2", "wrong lock-order contract")
    require(data["status"] == "implemented_candidate_unverified" and data["release_effect"] == "none" and data["production_authorization"] == "not_granted", "source contract overclaims evidence")
    require(data["owner"] == "trillionnium-world" and isinstance(data["source_basis"], str) and re.fullmatch(r"[0-9a-f]{40}", data["source_basis"]) is not None, "invalid source owner/basis")
    rows = data["order"]
    require(type(rows) is list and len(rows) == len(ORDER), "lock class inventory changed")
    for rank, (row, identity) in enumerate(zip(rows, ORDER), 1):
        require(type(row) is dict and set(row) == {"id", "rank", "description"}, "invalid lock class")
        require(row["id"] == identity and type(row["rank"]) is int and row["rank"] == rank, "lock rank reversal or type mismatch")
        require(isinstance(row["description"], str) and len(row["description"]) >= 8 and "\n" not in row["description"], "invalid lock description")
    require(data["checked_phases"] == {"capture_match": PHASE, "apply_capture": PHASE}, "phase lock trace changed")
    require(data["unlocked_hint"] == {"function": "apply_capture", "table": "public.trnm_online_settlement_captures", "revalidate": ["capture_id", "match_id", "state"]}, "hint revalidation contract changed")
    require(isinstance(data["scope"], str) and len(data["scope"]) >= 40, "missing evidence scope")
    require(type(data["remaining_evidence"]) is list and len(data["remaining_evidence"]) >= 6 and all(isinstance(x, str) and len(x) >= 20 for x in data["remaining_evidence"]), "missing runtime/external evidence boundaries")
    block = "<!-- trnm-lock-order-v2:start -->\n```text\n" + "\n".join(f'{r["rank"]}. {r["description"]}' for r in rows) + "\n```\n<!-- trnm-lock-order-v2:end -->"
    for name in DOCS:
        text = reader.read(name)
        require(text.count("<!-- trnm-lock-order-v2:start -->") == 1 and text.count("<!-- trnm-lock-order-v2:end -->") == 1 and block in text, f"lock-order document drift: {name}")
        require("capture -> match ->" not in text, f"obsolete apply lock reversal: {name}")
    return data


def validate(root: Path) -> None:
    reader = lexer.SourceBundle(root)
    contract_and_docs(reader)
    stream = lexer.tokens(reader.read(SOURCE))
    bodies = {name: lexer.function(stream, name) for name in ("capture_match", "apply_capture", "lock_terminal_rows", "load_campaign_rows", "load_capture_jobs")}
    for name, body in bodies.items():
        require(not lexer.remote_calls(body), f"remote settlement call under row locks: {name}")
        expected = {"capture_match": 1, "apply_capture": 2, "lock_terminal_rows": 2, "load_campaign_rows": 1, "load_capture_jobs": 1}[name]
        require(sum(t.kind == "string" and lexer.SQL_LOCK.search(t.value) is not None for t in body) == expected, f"unexpected explicit SQL lock surface: {name}")
    capture = bodies["capture_match"]
    match_query = query(capture, "select match_id from public.trnm_online_matches")
    insert = query(capture, "insert into public.trnm_online_settlement_captures")
    require("for update skip locked" in match_query[1], "capture match is not locked")
    require(match_query[0] < call_position(capture, "lock_terminal_rows") < call_position(capture, "load_campaign_rows") < insert[0], "capture row-lock order reversed")
    apply = bodies["apply_capture"]
    hint = query(apply, "select match_id from public.trnm_online_settlement_captures")
    match_query = query(apply, "select match_id from public.trnm_online_matches")
    locked = query(apply, "select match_id, terminal_identity_hash, campaign_fences_json,")
    require(lexer.SQL_LOCK.search(hint[1]) is None, "routing hint acquired an early row lock")
    require("where match_id = $1 for update" in match_query[1], "apply match lock missing")
    require("where capture_id = $1 and match_id = $2 and state = 'active' for update skip locked" in locked[1], "locked capture must revalidate hint identity and active state")
    require(any(code_equal(locked[2], i, (".", "bind", "(", "match_id", ")")) for i in range(len(locked[2]))), "capture SQL does not bind hinted match")
    require(hint[0] < match_query[0] < call_position(apply, "lock_terminal_rows") < call_position(apply, "load_campaign_rows") < locked[0] < call_position(apply, "load_capture_jobs"), "apply row-lock order reversed")
    terminal = bodies["lock_terminal_rows"]
    ack = query(terminal, "select match_id from public.trnm_online_terminal_publication_acks")
    members = query(terminal, "select player_id from public.trnm_online_match_members")
    require(ack[0] < members[0] and "for update" in ack[1] and "order by player_id for update" in members[1], "ACK/member lock order missing")
    campaigns = query(bodies["load_campaign_rows"], "select campaign.campaign_id, campaign.campaign_revision,")
    require("order by campaign.campaign_id, member.player_id for update of campaign" in campaigns[1], "campaign locks must follow campaign identity, not player identity")
    jobs = query(bodies["load_capture_jobs"], "select job_id, campaign_id, intent_id, intent_hash,")
    require("order by campaign_id, job_id for update" in jobs[1], "job ordering is not total")
    tests = lexer.tokens(reader.read(TEST_SOURCE))
    for name in ("actual_settlement_functions_obey_lock_order", "apply_does_not_hold_capture_before_match", "campaigns_lock_by_campaign_not_player"):
        lexer.function(tests, name)
    require(any(code_equal(stream, i, ("mod", "lock_order_database_tests", ";")) for i in range(len(stream))), "database regressions are not wired")
    require(any(code_equal(stream, i, ("#", "[", "path", "=")) and i + 5 < len(stream) and stream[i + 4] == lexer.Token("string", "settlement_lock_order_tests.rs") and lexer.is_token(stream[i + 5], "]") for i in range(len(stream))), "database test source path is missing")
    call_position(lexer.function(tests, "apply_does_not_hold_capture_before_match"), "apply_capture")
    call_position(lexer.function(tests, "campaigns_lock_by_campaign_not_player"), "load_campaign_rows")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    args = parser.parse_args()
    try:
        validate(args.root)
    except (LockOrderFailure, lexer.BoundaryFailure, OSError, UnicodeError, ValueError, TypeError, KeyError) as error:
        print(f"TRNM settlement lock order: FAIL: {error}", file=sys.stderr)
        return 1
    print("TRNM settlement lock order: PASS (known source regression only; no Rust/PostgreSQL/hosted credit)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
