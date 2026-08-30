from __future__ import annotations

from common import PROTOCOL_SOURCE, ROOT, STREAM_SOURCE, TEMPLATES, load, read

connect_path = ROOT / "docs/protocol/schemas/trnm-world-match-stream-connect-v1.schema.json"
messages_path = ROOT / "docs/protocol/schemas/trnm-world-match-stream-v1.schema.json"
connect = load(connect_path)
messages = load(messages_path)
templates = load(TEMPLATES)

stream_template = templates["get_match_stream"]
query_parameters = {
    parameter["name"]: parameter
    for parameter in stream_template["parameters"]
    if parameter["in"] == "query"
}
if set(query_parameters) != {
    "protocol_version",
    "build_id",
    "player_id",
    "account_id",
    "next_receipt_sequence",
    "last_snapshot_hash",
}:
    raise SystemExit("stream query parameter set drifted")
if query_parameters["protocol_version"]["schema"].get("const") != "trnm-online-stream-v1":
    raise SystemExit("stream query protocol drifted")
upgrade = stream_template["get"]["responses"].get("101", {})
if (
    upgrade.get("headers", {})
    .get("Sec-WebSocket-Protocol", {})
    .get("schema", {})
    .get("const")
    != "trnm-online-stream-v1"
):
    raise SystemExit("WebSocket upgrade subprotocol drifted")

if connect.get("additionalProperties") is not False:
    raise SystemExit("stream connect contract must reject unknown fields")
if set(connect.get("required", [])) != {
    "protocol_version",
    "build_id",
    "player_id",
    "account_id",
}:
    raise SystemExit("stream connect required fields drifted")
if connect["properties"]["protocol_version"].get("const") != "trnm-online-stream-v1":
    raise SystemExit("stream connect protocol drifted")

expected = {"full_snapshot", "snapshot_delta", "resync_required"}
observed = {
    messages["$defs"][name]["properties"]["message_type"]["const"]
    for name in expected
}
if observed != expected:
    raise SystemExit("stream message variants drifted")
for name in expected:
    if messages["$defs"][name].get("additionalProperties") is not False:
        raise SystemExit(f"stream message must reject unknown fields: {name}")

if messages.get("x-wire-contract") != {
    "client-data-frames": "rejected-with-close-1008",
    "reauth-seconds": 60,
    "send-timeout-seconds": 2,
    "server-ping-seconds": 15,
    "subprotocol": "trnm-online-stream-v1",
    "unknown-field": "reject-for-server-message-envelopes",
    "unknown-message-type": "reject-and-resync",
}:
    raise SystemExit("stream wire contract drifted")

protocol_source = read(PROTOCOL_SOURCE)
stream_source = read(STREAM_SOURCE)
for marker in [
    'pub const ONLINE_STREAM_PROTOCOL: &str = "trnm-online-stream-v1";',
    '#[serde(tag = "message_type", rename_all = "snake_case")]',
    "FullSnapshot {",
    "SnapshotDelta {",
    "ResyncRequired {",
]:
    if marker not in protocol_source:
        raise SystemExit(f"online protocol source marker missing: {marker}")
for marker in [
    "const STREAM_SEND_TIMEOUT: Duration = Duration::from_secs(2);",
    "const STREAM_REAUTH_INTERVAL: Duration = Duration::from_secs(60);",
    "const STREAM_PING_INTERVAL: Duration = Duration::from_secs(15);",
    "code: 1008,",
    "state-only stream rejects client data frames",
]:
    if marker not in stream_source:
        raise SystemExit(f"stream implementation marker missing: {marker}")
print("websocket_contract=passed variants=3")
