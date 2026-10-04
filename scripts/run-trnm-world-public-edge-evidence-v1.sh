#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat >&2 <<'EOF'
usage: run-trnm-world-public-edge-evidence-v1.sh OUTPUT_DIR

Required environment:
  TRNM_PUBLIC_EDGE_URL             authorized https:// endpoint
  TRNM_PUBLIC_EDGE_AUTHORIZATION   reviewed nonempty target authorization file
  TRNM_PUBLIC_EDGE_CONFIG          non-secret edge/WAF configuration record
  TRNM_PUBLIC_EDGE_THRESHOLDS      predeclared machine-readable thresholds
  TRNM_PUBLIC_EDGE_HEALTH_PATH     bounded health/readiness path

Optional:
  TRNM_PUBLIC_EDGE_RATE_REQUESTS   default 60, maximum 300
  TRNM_PUBLIC_EDGE_CONCURRENCY     default 8, maximum 32
  TRNM_PUBLIC_EDGE_BODY_BYTES      default 1048576, maximum 8388608
  TRNM_PUBLIC_EDGE_EVALUATOR       executable evaluation hook

The harness is intentionally bounded. It is not a DDoS utility and refuses
unapproved targets, non-HTTPS URLs, unbounded request counts and private or
loopback destinations unless the authorization explicitly permits laboratory
public-edge validation.
EOF
  exit 64
}

[[ $# -eq 1 ]] || usage
out="$1"
: "${TRNM_PUBLIC_EDGE_URL:?required}"
: "${TRNM_PUBLIC_EDGE_AUTHORIZATION:?required}"
: "${TRNM_PUBLIC_EDGE_CONFIG:?required}"
: "${TRNM_PUBLIC_EDGE_THRESHOLDS:?required}"
: "${TRNM_PUBLIC_EDGE_HEALTH_PATH:?required}"
requests="${TRNM_PUBLIC_EDGE_RATE_REQUESTS:-60}"
concurrency="${TRNM_PUBLIC_EDGE_CONCURRENCY:-8}"
body_bytes="${TRNM_PUBLIC_EDGE_BODY_BYTES:-1048576}"

[[ "$TRNM_PUBLIC_EDGE_URL" =~ ^https://[^/]+(/.*)?$ ]] || { echo "target must be HTTPS" >&2; exit 1; }
[[ "$TRNM_PUBLIC_EDGE_HEALTH_PATH" =~ ^/[A-Za-z0-9._~!$&'()*+,;=:@%/-]*$ ]] || { echo "unsafe health path" >&2; exit 1; }
[[ "$requests" =~ ^[0-9]+$ ]] && (( requests >= 1 && requests <= 300 )) || { echo "request count must be 1..300" >&2; exit 1; }
[[ "$concurrency" =~ ^[0-9]+$ ]] && (( concurrency >= 1 && concurrency <= 32 )) || { echo "concurrency must be 1..32" >&2; exit 1; }
[[ "$body_bytes" =~ ^[0-9]+$ ]] && (( body_bytes >= 1024 && body_bytes <= 8388608 )) || { echo "body bytes must be 1024..8388608" >&2; exit 1; }
for path in "$TRNM_PUBLIC_EDGE_AUTHORIZATION" "$TRNM_PUBLIC_EDGE_CONFIG" "$TRNM_PUBLIC_EDGE_THRESHOLDS"; do
  [[ -f "$path" && ! -L "$path" && -s "$path" ]] || { echo "missing, linked or empty input: $path" >&2; exit 1; }
done
for command in curl openssl python3 sha256sum; do command -v "$command" >/dev/null || { echo "missing command: $command" >&2; exit 1; }; done

python3 - "$TRNM_PUBLIC_EDGE_URL" "$TRNM_PUBLIC_EDGE_AUTHORIZATION" <<'PY'
import ipaddress, json, pathlib, socket, sys, urllib.parse
url=urllib.parse.urlsplit(sys.argv[1])
auth_path=pathlib.Path(sys.argv[2])
raw=auth_path.read_text(encoding='utf-8')
try:
    auth=json.loads(raw)
except json.JSONDecodeError as error:
    raise SystemExit(f'invalid authorization JSON: {error}')
if auth.get('schema') != 'trnm_world_public_edge_authorization_v1':
    raise SystemExit('unexpected authorization schema')
if auth.get('authorized') is not True:
    raise SystemExit('target authorization is not granted')
if auth.get('production_authorization') not in {'not_granted','granted_by_explicit_human_decision'}:
    raise SystemExit('authorization boundary is missing')
allowed=auth.get('allowed_hosts')
if not isinstance(allowed,list) or url.hostname not in allowed:
    raise SystemExit('target host is not in the authorization allowlist')
expires=auth.get('expires_at_utc')
if not isinstance(expires,str):
    raise SystemExit('authorization expiry is missing')
from datetime import datetime, timezone
if datetime.fromisoformat(expires.replace('Z','+00:00')).astimezone(timezone.utc) <= datetime.now(timezone.utc):
    raise SystemExit('authorization has expired')
allow_private=auth.get('allow_private_or_loopback') is True
for family, _, _, _, address in socket.getaddrinfo(url.hostname, url.port or 443, type=socket.SOCK_STREAM):
    ip=ipaddress.ip_address(address[0])
    if (ip.is_private or ip.is_loopback or ip.is_link_local or ip.is_unspecified or ip.is_reserved) and not allow_private:
        raise SystemExit(f'private/loopback/reserved target is not authorized: {ip}')
PY

umask 077
mkdir -p "$out"/{inputs,tls,baseline,rate,body,slow-client,summary}
[[ -z "$(find "$out" -type f -print -quit)" ]] || { echo "output directory must not contain files" >&2; exit 1; }
cp "$TRNM_PUBLIC_EDGE_AUTHORIZATION" "$out/inputs/authorization.json"
cp "$TRNM_PUBLIC_EDGE_CONFIG" "$out/inputs/edge-configuration.record"
cp "$TRNM_PUBLIC_EDGE_THRESHOLDS" "$out/inputs/thresholds.record"
sha256sum "$TRNM_PUBLIC_EDGE_AUTHORIZATION" "$TRNM_PUBLIC_EDGE_CONFIG" "$TRNM_PUBLIC_EDGE_THRESHOLDS" > "$out/inputs/SHA256SUMS"

host="$(python3 -c 'import urllib.parse,sys; print(urllib.parse.urlsplit(sys.argv[1]).hostname)' "$TRNM_PUBLIC_EDGE_URL")"
port="$(python3 -c 'import urllib.parse,sys; print(urllib.parse.urlsplit(sys.argv[1]).port or 443)' "$TRNM_PUBLIC_EDGE_URL")"
origin="https://$host"
[[ "$port" == 443 ]] || origin="$origin:$port"
health_url="$origin$TRNM_PUBLIC_EDGE_HEALTH_PATH"
started="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
printf '%s\n' "$started" > "$out/summary/started-at.txt"

# TLS identity and protocol observations. No private key material is read.
timeout 30 openssl s_client -connect "$host:$port" -servername "$host" -showcerts </dev/null > "$out/tls/s_client.txt" 2> "$out/tls/s_client.stderr"
openssl s_client -connect "$host:$port" -servername "$host" </dev/null 2>/dev/null \
  | openssl x509 -noout -fingerprint -sha256 -subject -issuer -serial -dates > "$out/tls/certificate.txt"
for version in -tls1_2 -tls1_3; do
  set +e
  timeout 20 openssl s_client "$version" -connect "$host:$port" -servername "$host" </dev/null > "$out/tls/${version#-}.txt" 2> "$out/tls/${version#-}.stderr"
  printf '%s\n' "$?" > "$out/tls/${version#-}.exit-code"
  set -e
done

# Baseline response, headers and bounded timings.
curl --fail-with-body --silent --show-error --location --max-redirs 0 --connect-timeout 10 --max-time 30 \
  --output "$out/baseline/body.bin" --dump-header "$out/baseline/headers.txt" \
  --write-out '{"http_code":%{http_code},"remote_ip":"%{remote_ip}","ssl_verify_result":%{ssl_verify_result},"time_connect":%{time_connect},"time_appconnect":%{time_appconnect},"time_starttransfer":%{time_starttransfer},"time_total":%{time_total},"size_download":%{size_download}}\n' \
  "$health_url" > "$out/baseline/metrics.json"

# Bounded parallel burst. Each request records only code/timing/size.
seq 1 "$requests" | xargs -P "$concurrency" -I{} sh -c '
  curl --silent --show-error --output /dev/null --connect-timeout 10 --max-time 30 \
    --write-out "{} %{http_code} %{time_total} %{size_download} %{remote_ip}\\n" "$1"
' _ "$health_url" > "$out/rate/results.txt"

# Oversized-body rejection probe against the explicitly authorized health path.
# The payload is deterministic and bounded by the hard 8 MiB ceiling above.
python3 - "$body_bytes" "$out/body/payload.bin" <<'PY'
import pathlib, sys
size=int(sys.argv[1]); pathlib.Path(sys.argv[2]).write_bytes(b'X'*size)
PY
set +e
curl --silent --show-error --output "$out/body/response.bin" --dump-header "$out/body/headers.txt" \
  --connect-timeout 10 --max-time 30 --request POST --header 'Content-Type: application/octet-stream' \
  --data-binary "@$out/body/payload.bin" --write-out '%{http_code} %{time_total} %{size_download}\n' \
  "$health_url" > "$out/body/metrics.txt" 2> "$out/body/stderr.log"
printf '%s\n' "$?" > "$out/body/curl-exit-code.txt"
set -e

# One bounded slow-client probe sends headers in small chunks; it is limited to
# one connection and 15 seconds so it tests timeout policy without load abuse.
python3 - "$host" "$port" "$TRNM_PUBLIC_EDGE_HEALTH_PATH" "$out/slow-client/result.json" <<'PY'
import json, socket, ssl, sys, time
host=sys.argv[1]; port=int(sys.argv[2]); path=sys.argv[3]; out=sys.argv[4]
context=ssl.create_default_context()
started=time.monotonic(); received=b''; error=None
try:
    with socket.create_connection((host,port),timeout=5) as raw:
        raw.settimeout(5)
        with context.wrap_socket(raw,server_hostname=host) as sock:
            chunks=[f'GET {path} HTTP/1.1\r\n'.encode(), f'Host: {host}\r\n'.encode(), b'User-Agent: trnm-world-authorized-probe\r\n', b'Connection: close\r\n\r\n']
            for chunk in chunks:
                sock.sendall(chunk); time.sleep(2)
            while len(received)<65536:
                part=sock.recv(min(4096,65536-len(received)))
                if not part: break
                received+=part
except Exception as exc:
    error=type(exc).__name__+': '+str(exc)
result={'elapsed_seconds':time.monotonic()-started,'received_bytes':len(received),'response_prefix_sha256':__import__('hashlib').sha256(received).hexdigest(),'error':error}
open(out,'w').write(json.dumps(result,sort_keys=True)+'\n')
PY

rm -f "$out/body/payload.bin"

if [[ -n "${TRNM_PUBLIC_EDGE_EVALUATOR:-}" ]]; then
  [[ -f "$TRNM_PUBLIC_EDGE_EVALUATOR" && ! -L "$TRNM_PUBLIC_EDGE_EVALUATOR" && -x "$TRNM_PUBLIC_EDGE_EVALUATOR" ]] || { echo "unsafe public-edge evaluator" >&2; exit 1; }
  sha256sum "$TRNM_PUBLIC_EDGE_EVALUATOR" > "$out/inputs/evaluator.sha256"
  "$TRNM_PUBLIC_EDGE_EVALUATOR" "$out/inputs/thresholds.record" "$out" > "$out/summary/evaluation.json"
  python3 - "$out/summary/evaluation.json" <<'PY'
import json, pathlib, sys
value=json.loads(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8'))
if value.get('all_thresholds_passed') is not True:
    raise SystemExit('public-edge thresholds did not pass')
PY
else
  echo "TRNM_PUBLIC_EDGE_EVALUATOR is required before this run can become PASS evidence" >&2
  exit 1
fi

ended="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
printf '%s\n' "$ended" > "$out/summary/ended-at.txt"
find "$out" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/summary/ARTIFACT-SHA256SUMS"
cat > "$out/summary/RESULT.txt" <<EOF
TRNM_WORLD_PUBLIC_EDGE_ORCHESTRATION=COMPLETE
url=$TRNM_PUBLIC_EDGE_URL
health_path=$TRNM_PUBLIC_EDGE_HEALTH_PATH
started_at_utc=$started
ended_at_utc=$ended
requests=$requests
concurrency=$concurrency
production_authorization=not_granted
independent_security_review_required=true
strict_evidence_validation_required=true
EOF

echo "Public-edge harness completed. Independent security review and strict evidence validation remain required."
