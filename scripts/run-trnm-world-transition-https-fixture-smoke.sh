#!/usr/bin/env bash
set -euo pipefail

root=${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)}
module="$root/tools/trnm-world-transition-https-v1"
evidence=${TRNM_WORLD_FIXTURE_EVIDENCE_DIR:-"$root/run/world-transition-https-fixture-v1"}

for command_name in go docker openssl curl python3 sha256sum cmp git; do
  command -v "$command_name" >/dev/null 2>&1 || {
    printf 'missing required command: %s\n' "$command_name" >&2
    exit 1
  }
done
docker info >/dev/null

temporary=$(mktemp -d)
container="trnm-world-fixture-smoke-$$"
image="trnm-world-transition-https-fixture:smoke-$$"
mkdir -p "$temporary/image" "$temporary/certs" "$temporary/cache"
rm -rf "$evidence"
mkdir -p "$evidence"

cleanup() {
  status=$?
  trap - EXIT INT TERM
  set +e
  if [[ ! -f "$evidence/container.log" ]]; then
    docker logs "$container" >"$evidence/container.log" 2>&1 || true
  fi
  docker rm -f "$container" >/dev/null 2>&1 || true
  docker image rm "$image" >/dev/null 2>&1 || true
  rm -rf "$temporary"
  exit "$status"
}
trap cleanup EXIT INT TERM

(
  cd "$module"
  CGO_ENABLED=0 go build -trimpath -ldflags='-s -w -buildid=' \
    -o "$temporary/image/trnm-world-transition-https-v1" \
    ./cmd/trnm-world-transition-https-v1
)
cp "$module/Dockerfile.prebuilt" "$temporary/image/Dockerfile"
docker build --network=none -t "$image" "$temporary/image" >"$evidence/docker-build.log"
docker image inspect "$image" >"$evidence/image-inspect.json"

openssl req -x509 -newkey rsa:2048 -sha256 -days 1 -nodes \
  -keyout "$temporary/certs/server.key" \
  -out "$temporary/certs/server.crt" \
  -subj '/CN=localhost' \
  -addext 'subjectAltName=DNS:localhost,IP:127.0.0.1' \
  >"$evidence/openssl.stdout" 2>"$evidence/openssl.stderr"
chmod 0644 "$temporary/certs/server.crt" "$temporary/certs/server.key"
chmod 0777 "$temporary/cache"

port=$(python3 - <<'PY'
import socket
with socket.socket() as listener:
    listener.bind(("127.0.0.1", 0))
    print(listener.getsockname()[1])
PY
)
token=$(openssl rand -hex 32)

docker run -d --name "$container" \
  --user "$(id -u):$(id -g)" \
  --read-only \
  --security-opt no-new-privileges:true \
  --cap-drop ALL \
  -p "127.0.0.1:${port}:7443" \
  -v "$temporary/certs:/certs:ro" \
  -v "$temporary/cache:/results" \
  -e TRNM_WORLD_FIXTURE_LISTEN=:7443 \
  -e TRNM_WORLD_FIXTURE_TLS_CERT=/certs/server.crt \
  -e TRNM_WORLD_FIXTURE_TLS_KEY=/certs/server.key \
  -e TRNM_WORLD_FIXTURE_BEARER_TOKEN="$token" \
  -e TRNM_WORLD_FIXTURE_RESULT_DIR=/results \
  "$image" >/dev/null

for _ in $(seq 1 40); do
  if curl --silent --show-error --fail \
    --tlsv1.3 --tls-max 1.3 \
    --cacert "$temporary/certs/server.crt" \
    "https://127.0.0.1:${port}/healthz" >"$evidence/health.json"; then
    break
  fi
  sleep 0.25
done
curl --silent --show-error --fail \
  --tlsv1.3 --tls-max 1.3 \
  --cacert "$temporary/certs/server.crt" \
  "https://127.0.0.1:${port}/healthz" >"$evidence/health.json"

python3 - "$evidence/request.json" <<'PY'
import hashlib, json, pathlib, sys

def canonical(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"), sort_keys=True).encode()
state = {"counter": 0}
command = {"delta": 1, "kind": "advance"}
request = {
    "command": {
        "command_id": "fixture-smoke-command-1",
        "payload": {
            "canonical_json": command,
            "schema_id": "trnm.blackbox.move.v1",
            "sha256": hashlib.sha256(canonical(command)).hexdigest(),
        },
    },
    "content_revision": "blackbox-content-v1",
    "contract_version": "trnm_world_transition_v1",
    "expected_tick": 0,
    "previous_state": {
        "canonical_json": state,
        "schema_id": "trnm.blackbox.state.v1",
        "sha256": hashlib.sha256(canonical(state)).hexdigest(),
    },
    "ruleset_revision": "blackbox-ruleset-v1",
    "transition_id": "fixture-smoke-transition-1",
}
pathlib.Path(sys.argv[1]).write_bytes(canonical(request))
PY

for attempt in first second; do
  curl --silent --show-error --fail \
    --tlsv1.3 --tls-max 1.3 \
    --cacert "$temporary/certs/server.crt" \
    -H "Authorization: Bearer $token" \
    -H 'Content-Type: application/json' \
    --data-binary "@$evidence/request.json" \
    "https://127.0.0.1:${port}/v1/transition" \
    >"$evidence/result-${attempt}.json"
done
cmp "$evidence/result-first.json" "$evidence/result-second.json"

request_hash=$(python3 - "$evidence/request.json" "$evidence/result-first.json" <<'PY'
import hashlib, json, pathlib, sys

def canonical(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"), sort_keys=True).encode()
def domain(name, material):
    return hashlib.sha256(name.encode() + b"\n" + material).hexdigest()
request_bytes = pathlib.Path(sys.argv[1]).read_bytes()
result_bytes = pathlib.Path(sys.argv[2]).read_bytes()
result = json.loads(result_bytes)
expected_request = domain("trnm.world.transition.request.v1", request_bytes)
assert result["contract_version"] == "trnm_world_transition_v1"
assert result["request_hash"] == expected_request
assert result["next_tick"] == 1
assert result["next_state"]["canonical_json"] == {"counter": 1}
for name in ("next_state", "replay_material", "outcome_material"):
    assert result[name]["sha256"] == hashlib.sha256(canonical(result[name]["canonical_json"])).hexdigest()
outcome_material = {
    "content_revision": result["content_revision"],
    "outcome_payload_hash": result["outcome_material"]["sha256"],
    "outcome_schema_id": result["outcome_material"]["schema_id"],
    "ruleset_revision": result["ruleset_revision"],
}
assert result["world_outcome_hash"] == domain("trnm.world.outcome.v1", canonical(outcome_material))
facts = dict(result)
transition_hash = facts.pop("world_transition_hash")
assert transition_hash == domain("trnm.world.transition.accepted.v1", canonical(facts))
assert canonical(result) == result_bytes
print(expected_request)
PY
)

curl --silent --show-error --fail \
  --tlsv1.3 --tls-max 1.3 \
  --cacert "$temporary/certs/server.crt" \
  -H "Authorization: Bearer $token" \
  "https://127.0.0.1:${port}/v1/result/${request_hash}" \
  >"$evidence/result-lookup.json"
cmp "$evidence/result-first.json" "$evidence/result-lookup.json"

curl --silent --show-error --fail \
  --tlsv1.3 --tls-max 1.3 \
  --cacert "$temporary/certs/server.crt" \
  -H "Authorization: Bearer $token" \
  "https://127.0.0.1:${port}/v1/stats" \
  >"$evidence/stats.json"
python3 - "$evidence/stats.json" <<'PY'
import json, sys
stats = json.load(open(sys.argv[1]))
assert stats["computed"] == 1
assert stats["accepted"] == 1
assert stats["cache_hits"] >= 1
assert stats["cutover_authorized"] is False
assert stats["public_online_enabled"] is False
assert stats["public_player_market_enabled"] is False
PY

commit=$(git -C "$root" rev-parse HEAD)
tree=$(git -C "$root" rev-parse 'HEAD^{tree}')
image_id=$(python3 - "$evidence/image-inspect.json" <<'PY'
import json, sys
value=json.load(open(sys.argv[1])); print(value[0]["Id"])
PY
)
python3 - "$evidence/report.json" "$commit" "$tree" "$image_id" <<'PY'
import datetime as dt, json, pathlib, sys
path, commit, tree, image = sys.argv[1:]
report = {
    "contract_version": "trnm_world_transition_https_fixture_smoke_v1",
    "generated_at": dt.datetime.now(dt.timezone.utc).isoformat(),
    "repository": "TrillionniumFoundation/Trillionnium-World",
    "commit": commit,
    "tree": tree,
    "image_id": image,
    "tls_profile": "TLS_1_3_only",
    "exact_duplicate_result": True,
    "durable_result_lookup": True,
    "fixture_only": True,
    "cutover_authorized": False,
    "public_online_enabled": False,
    "public_player_market_enabled": False,
    "limitations": [
        "single host smoke",
        "fixture ruleset rather than production World rules",
        "no Game or Integration runtime in this artifact",
        "no multi-host fencing",
        "no endurance or public-edge evidence",
    ],
}
pathlib.Path(path).write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
PY

docker logs "$container" >"$evidence/container.log" 2>&1

find "$evidence" -type f ! -name SHA256SUMS -print0 \
  | sort -z \
  | xargs -0 sha256sum \
  >"$evidence/SHA256SUMS"
sha256sum --check "$evidence/SHA256SUMS" >/dev/null
printf 'World transition HTTPS fixture container smoke: PASS (%s)\n' "$evidence"
