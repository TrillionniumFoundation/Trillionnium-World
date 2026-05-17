#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/public-network-deploy-evidence.json"
PORT="${TRILLIONNIUM_WORLD_DEPLOY_DRILL_PORT:-18790}"
BIND_ADDR="127.0.0.1:$PORT"
STATE_FILE="$ACCEPTANCE_DIR/public-deploy-drill-state.json"
HEALTH_EVIDENCE="$ACCEPTANCE_DIR/public-deploy-health.json"
HOME_EVIDENCE="$ACCEPTANCE_DIR/public-deploy-home.json"
COMMAND_EVIDENCE="$ACCEPTANCE_DIR/public-deploy-command.json"
LDD_EVIDENCE="$ACCEPTANCE_DIR/public-deploy-binary-ldd.txt"
ENV_EXAMPLE="$ACCEPTANCE_DIR/trnm-world-server.env.example"
SYSTEMD_UNIT="$ACCEPTANCE_DIR/trnm-world-server.service.example"
REVERSE_PROXY="$ACCEPTANCE_DIR/reverse-proxy.caddyfile.example"
RUNBOOK="$ACCEPTANCE_DIR/public-deploy-runbook.md"
LOG_FILE="$ACCEPTANCE_DIR/public-deploy-local-drill.log"

mkdir -p "$ACCEPTANCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo build -p trnm-world-server --release
)

BIN="$ROOT/target/release/trnm-world-server"
if [[ ! -x "$BIN" ]]; then
  printf 'release binary missing: %s\n' "$BIN" >&2
  exit 1
fi

BINARY_SHA256="$(sha256sum "$BIN" | awk '{print $1}')"
BINARY_SIZE_BYTES="$(stat -c '%s' "$BIN")"
ldd "$BIN" >"$LDD_EVIDENCE"

cat >"$ENV_EXAMPLE" <<EOF
TRNM_WORLD_BIND=127.0.0.1:8787
TRNM_WORLD_ACTOR_ID=local-player
TRNM_WORLD_STATE_FILE=/var/lib/trillionnium-world/world-state.json
TRNM_WORLD_PUBLIC_BASE_URL=https://world.example.invalid
TRNM_WORLD_PUBLIC_NETWORK_EXPOSURE_APPROVED=false
EOF

cat >"$SYSTEMD_UNIT" <<EOF
[Unit]
Description=Trillionnium World standalone server
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
EnvironmentFile=/etc/trillionnium-world/trnm-world-server.env
ExecStart=/opt/trillionnium/bin/trnm-world-server serve --bind \${TRNM_WORLD_BIND} --actor-id \${TRNM_WORLD_ACTOR_ID} --state-file \${TRNM_WORLD_STATE_FILE}
Restart=on-failure
RestartSec=5
DynamicUser=yes
StateDirectory=trillionnium-world
NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/var/lib/trillionnium-world

[Install]
WantedBy=multi-user.target
EOF

cat >"$REVERSE_PROXY" <<EOF
world.example.invalid {
  encode zstd gzip
  reverse_proxy 127.0.0.1:8787
  header {
    X-Content-Type-Options nosniff
    Referrer-Policy no-referrer
  }
}
EOF

cat >"$RUNBOOK" <<EOF
# Trillionnium World Public Deploy Runbook

This is a deployment readiness package, not proof of live public exposure.

## Required before public exposure

- Set a real domain and TLS endpoint.
- Confirm public exposure approval.
- Install the release binary under /opt/trillionnium/bin/trnm-world-server.
- Install the env file under /etc/trillionnium-world/trnm-world-server.env.
- Install the systemd unit from trnm-world-server.service.example.
- Configure reverse proxy from reverse-proxy.caddyfile.example or equivalent.
- Run /health and /world/home probes through the public URL.
- Attach monitoring, backup, and rollback evidence.

## Stop / rollback

- Remove the reverse-proxy route.
- systemctl stop trnm-world-server.
- Restore the previous state file from backup.
EOF

rm -f "$STATE_FILE" "$HEALTH_EVIDENCE" "$HOME_EVIDENCE" "$COMMAND_EVIDENCE" "$LOG_FILE"
"$BIN" serve --bind "$BIND_ADDR" --state-file "$STATE_FILE" --reset-state >"$LOG_FILE" 2>&1 &
SERVER_PID=$!
cleanup() {
  kill "$SERVER_PID" >/dev/null 2>&1 || true
  wait "$SERVER_PID" >/dev/null 2>&1 || true
}
trap cleanup EXIT

for _ in $(seq 1 80); do
  if curl -fsS "http://$BIND_ADDR/health" >"$HEALTH_EVIDENCE" 2>/dev/null; then
    break
  fi
  sleep 0.1
done

curl -fsS "http://$BIND_ADDR/health" >"$HEALTH_EVIDENCE"
curl -fsS "http://$BIND_ADDR/world/home" >"$HOME_EVIDENCE"
curl -fsS "http://$BIND_ADDR/world/command?direction=east&actor_id=local-player" >"$COMMAND_EVIDENCE"
grep -q 'trillionnium_world_dev_runtime_v1' "$HEALTH_EVIDENCE"
grep -q 'file_backed_json' "$HEALTH_EVIDENCE"
grep -q 'starter-studio' "$COMMAND_EVIDENCE"
cleanup
trap - EXIT

jq -n \
  --arg contract_version "trillionnium_world_public_deploy_readiness_v1" \
  --arg status "local_public_deploy_drill_green" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg release_binary "$BIN" \
  --arg binary_sha256 "$BINARY_SHA256" \
  --arg binary_size_bytes "$BINARY_SIZE_BYTES" \
  --arg bind_addr "$BIND_ADDR" \
  --arg state_file "$STATE_FILE" \
  --arg health_evidence "$HEALTH_EVIDENCE" \
  --arg home_evidence "$HOME_EVIDENCE" \
  --arg command_evidence "$COMMAND_EVIDENCE" \
  --arg ldd_evidence "$LDD_EVIDENCE" \
  --arg env_example "$ENV_EXAMPLE" \
  --arg systemd_unit "$SYSTEMD_UNIT" \
  --arg reverse_proxy "$REVERSE_PROXY" \
  --arg runbook "$RUNBOOK" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trnm_world_public_deploy_readiness_drill",
    public_network_ready: false,
    public_network_exposure_performed: false,
    public_network_blocking_reason: "external/public exposure requires explicit approval, real host/domain/TLS, monitoring, backup, rollback, and live public URL probes",
    release: {
      binary_path: $release_binary,
      binary_sha256: $binary_sha256,
      binary_size_bytes: ($binary_size_bytes | tonumber),
      ldd_evidence: $ldd_evidence
    },
    local_drill: {
      bind_addr: $bind_addr,
      state_file: $state_file,
      health_evidence: $health_evidence,
      home_evidence: $home_evidence,
      command_evidence: $command_evidence,
      command_mutation_verified: true,
      file_backed_repository_verified: true
    },
    deploy_artifacts: {
      env_example: $env_example,
      systemd_unit_example: $systemd_unit,
      reverse_proxy_example: $reverse_proxy,
      runbook: $runbook
    },
    live_public_requirements: [
      "explicit_operator_approval",
      "target_host",
      "domain_dns",
      "tls_certificate",
      "public_url_health_probe",
      "monitoring_alerts",
      "backup_restore",
      "rollback_drill"
    ]
  }' >"$SUMMARY_FILE"

printf 'TRILLIONNIUM_WORLD_PUBLIC_DEPLOY_DRILL_READY %s\n' "$SUMMARY_FILE"
