#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$HOME/.config/systemd/user"
BACKUP_DIR="$(mktemp -d "$ROOT_DIR/run/online-systemd-rollback.XXXXXX")"
rollback() {
  local status=$?
  if [[ "$status" -ne 0 ]]; then
    for unit in trnm-game-server.service trnm-entitlement-signer.service; do
      if [[ -f "$BACKUP_DIR/$unit" ]]; then
        install -m 0644 "$BACKUP_DIR/$unit" "$HOME/.config/systemd/user/$unit"
      fi
    done
    systemctl --user daemon-reload || true
    systemctl --user restart trnm-entitlement-signer.service || true
    systemctl --user restart trnm-game-server.service || true
    echo "Online unit deployment failed; previous unit files were restored from $BACKUP_DIR" >&2
  fi
  exit "$status"
}
trap rollback EXIT
for unit in trnm-game-server.service trnm-entitlement-signer.service; do
  if [[ -f "$HOME/.config/systemd/user/$unit" ]]; then
    install -m 0644 "$HOME/.config/systemd/user/$unit" "$BACKUP_DIR/$unit"
  fi
done
install -m 0644 "$ROOT_DIR/deploy/systemd/trnm-game-server.service" \
  "$HOME/.config/systemd/user/trnm-game-server.service"
install -m 0644 "$ROOT_DIR/deploy/systemd/trnm-entitlement-signer.service" \
  "$HOME/.config/systemd/user/trnm-entitlement-signer.service"
systemctl --user daemon-reload
systemctl --user enable trnm-entitlement-signer.service
systemctl --user restart trnm-entitlement-signer.service
for _ in $(seq 1 40); do
  curl -fsS http://127.0.0.1:7010/v1/signer/readiness \
    | jq -e '.status == "ok" and .contract_version == "trnm_entitlement_signer_v1" and
      .database_pool_saturation_healthy == true and .database_pool_max_connections == 4' \
      >/dev/null 2>&1 && break
  sleep 0.25
done
curl -fsS http://127.0.0.1:7010/v1/signer/readiness \
  | jq -e '.status == "ok" and .contract_version == "trnm_entitlement_signer_v1" and
    .database_pool_saturation_healthy == true and .database_pool_max_connections == 4' >/dev/null
systemctl --user enable trnm-game-server.service
systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  curl -fsS http://127.0.0.1:7005/v1/online/readiness \
    | jq -e '.status == "ok" and .tick_rate_hz == 10 and
      .database_pool_saturation_healthy == true and .database_pool_max_connections == 8' \
      >/dev/null 2>&1 && break
  sleep 0.5
done
curl -fsS http://127.0.0.1:7005/v1/online/readiness \
  | jq -e '.status == "ok" and .tick_rate_hz == 10 and
    .database_pool_saturation_healthy == true and .database_pool_max_connections == 8' >/dev/null
TRNM_REQUIRE_INSTALLED_RESOURCE_BUDGETS=1 \
  "$ROOT_DIR/scripts/check-trnm-online-resource-budgets.sh" >/dev/null
trap - EXIT
