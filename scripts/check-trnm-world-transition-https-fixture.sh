#!/usr/bin/env bash
set -euo pipefail

root=${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)}
module="$root/tools/trnm-world-transition-https-v1"
status="$root/docs/development/world-transition-https-fixture-v1-status.json"
design="$root/docs/development/WORLD_TRANSITION_HTTPS_FIXTURE_V1.md"

fail() {
  printf 'World transition HTTPS fixture gate failed: %s\n' "$*" >&2
  exit 1
}

required=(
  tools/trnm-world-transition-https-v1/go.mod
  tools/trnm-world-transition-https-v1/README.md
  tools/trnm-world-transition-https-v1/Dockerfile.prebuilt
  tools/trnm-world-transition-https-v1/cmd/trnm-world-transition-https-v1/main.go
  tools/trnm-world-transition-https-v1/internal/fixture/canonical.go
  tools/trnm-world-transition-https-v1/internal/fixture/contract.go
  tools/trnm-world-transition-https-v1/internal/fixture/store.go
  tools/trnm-world-transition-https-v1/internal/fixture/server.go
  tools/trnm-world-transition-https-v1/internal/fixture/fixture_test.go
  tools/trnm-world-transition-https-v1/internal/fixture/store_security_test.go
  docs/development/WORLD_TRANSITION_HTTPS_FIXTURE_V1.md
  docs/development/world-transition-https-fixture-v1-status.json
)
for path in "${required[@]}"; do
  [[ -f "$root/$path" ]] || fail "missing $path"
done

python3 - "$status" <<'PY'
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
value = json.loads(path.read_text())
if value.get("contract_version") != "trnm_world_transition_https_fixture_delivery_v1":
    raise SystemExit("unexpected status contract")
if value.get("owner") != "TrillionniumFoundation/Trillionnium-World":
    raise SystemExit("World owning repository drift")
if value.get("base_branch") != "feature/world-deterministic-transition-contract-v1":
    raise SystemExit("fixture base branch drift")
if value.get("activation") != "fixture_only":
    raise SystemExit("fixture activation was promoted")
authority = value.get("authority", {})
for key in (
    "can_authenticate_players",
    "can_assign_participant_roles",
    "can_set_global_sequence",
    "can_set_match_version",
    "can_set_command_idempotency",
    "can_create_canonical_roots",
    "can_sign_completion",
    "can_settle_value",
    "cutover_authorized",
    "closed_online_promoted",
    "public_online_enabled",
    "public_player_market_enabled",
):
    if authority.get(key) is not False:
        raise SystemExit(f"authority/release flag must remain false: {key}")
required_limitations = {
    "fixture_ruleset_not_production_world_rules",
    "no_multi_host_fencing_proof",
    "no_24_hour_endurance",
    "no_authority_cutover",
    "no_public_online",
}
if not required_limitations.issubset(set(value.get("limitations", []))):
    raise SystemExit("fixture limitations were weakened")
if "Trillionnium Chain" not in set(value.get("scope_exclusions", [])):
    raise SystemExit("Chain scope exclusion was removed")
PY

grep -q '^module github.com/TrillionniumFoundation/Trillionnium-World/tools/trnm-world-transition-https-v1$' "$module/go.mod" || fail 'module identity drift'
if grep -q '^require ' "$module/go.mod"; then
  fail 'fixture gained a third-party Go dependency'
fi

grep -q 'tls.VersionTLS13' "$module/internal/fixture/server.go" || fail 'TLS 1.3 boundary is missing'
grep -q 'MinVersion: tls.VersionTLS13' "$module/internal/fixture/server.go" || fail 'TLS minimum drifted'
grep -q 'MaxVersion: tls.VersionTLS13' "$module/internal/fixture/server.go" || fail 'TLS maximum drifted'
grep -q 'hmac.Equal' "$module/internal/fixture/server.go" || fail 'bearer comparison is not constant-time'
grep -q 'io.LimitReader' "$module/internal/fixture/server.go" || fail 'request body is not bounded'
grep -q 'object keys are duplicated or not strictly ascending' "$module/internal/fixture/canonical.go" || fail 'strict object-key order check is missing'
grep -q 'signed-i64 integers' "$module/internal/fixture/canonical.go" || fail 'signed-i64-only JSON profile is missing'
grep -q 'forbiddenAuthorityKeys' "$module/internal/fixture/contract.go" || fail 'authority denylist is missing'
grep -q 'RequestHashDomain' "$module/internal/fixture/contract.go" || fail 'request hash domain is missing'
grep -q 'TransitionHashDomain' "$module/internal/fixture/contract.go" || fail 'transition hash domain is missing'
grep -q 'OutcomeHashDomain' "$module/internal/fixture/contract.go" || fail 'outcome hash domain is missing'

for token in 'temporary.Sync()' 'publishResultNoReplace' 'os.Link' 'os.Lstat' 'directory.Sync()'; do
  grep -q "$token" "$module/internal/fixture/store.go" || fail "durable no-replace result publication lacks $token"
done
if grep -q 'os.Rename' "$module/internal/fixture/store.go"; then
  fail 'result publication may replace an already committed result'
fi
grep -q 'TestPublishResultNoReplacePreservesCommittedBytes' "$module/internal/fixture/store_security_test.go" || fail 'no-replace collision regression test is missing'
grep -q 'TestResultStoreRejectsSymlinkedResult' "$module/internal/fixture/store_security_test.go" || fail 'symlinked-result regression test is missing'

grep -q '^FROM scratch$' "$module/Dockerfile.prebuilt" || fail 'final container is not scratch'
grep -q '^USER 65532:65532$' "$module/Dockerfile.prebuilt" || fail 'final container is not non-root'
if grep -R -nE 'database/sql|crypto/ed25519|google.golang.org/grpc|gorilla/websocket|github.com/.+/(wallet|settlement)|type MatchCompletedV1|func SignCompletion' "$module" --include='*.go'; then
  fail 'fixture source acquired a forbidden authority, database, signing, or settlement capability'
fi

for phrase in \
  'not the production World rules engine' \
  'cannot authenticate players' \
  'Public online and public player markets remain disabled'; do
  grep -Fq "$phrase" "$module/README.md" "$design" || fail "missing honest limitation: $phrase"
done

printf '%s\n' 'World transition HTTPS fixture boundary: passed'
