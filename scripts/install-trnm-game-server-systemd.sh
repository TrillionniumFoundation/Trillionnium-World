#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_HOME="$HOME/.config/trillionnium-world"
STATE_HOME="$HOME/.local/state/trillionnium-world"
SYSTEMD_HOME="$HOME/.config/systemd/user"
START_SERVICES=0

usage() {
  cat <<'USAGE'
Usage: scripts/install-trnm-game-server-systemd.sh [--start]

Without --start, render and enable the user units but do not start them. The
installer creates private environment files from examples when they do not
exist. Replace every REPLACE_* value before using --start.
USAGE
}

case "${1:-}" in
  "") ;;
  --start) START_SERVICES=1 ;;
  -h|--help)
    usage
    exit 0
    ;;
  *)
    usage >&2
    exit 64
    ;;
esac
[[ $# -le 1 ]] || {
  usage >&2
  exit 64
}

case "$ROOT_DIR" in
  *$'\n'*|*$'\r'*|*$'\t'*|*' '*)
    echo "World checkout path must not contain whitespace for systemd rendering: $ROOT_DIR" >&2
    exit 1
    ;;
esac

for command in systemctl sed install grep curl jq mktemp seq sleep; do
  command -v "$command" >/dev/null 2>&1 || {
    echo "required command is unavailable: $command" >&2
    exit 1
  }
done

mkdir -p "$CONFIG_HOME" "$STATE_HOME" "$SYSTEMD_HOME" "$ROOT_DIR/run"
chmod 700 "$CONFIG_HOME" "$STATE_HOME"
BACKUP_DIR="$(mktemp -d "$STATE_HOME/systemd-rollback.XXXXXX")"

units=(
  trnm-entitlement-signer.service
  trnm-game-server.service
  trnm-settlement-worker.service
)
declare -A unit_existed=()
rollback() {
  local status=$?
  if [[ "$status" -ne 0 ]]; then
    systemctl --user stop trnm-settlement-worker.service >/dev/null 2>&1 || true
    for unit in "${units[@]}"; do
      if [[ "${unit_existed[$unit]:-0}" == "1" ]]; then
        install -m 0644 "$BACKUP_DIR/$unit" "$SYSTEMD_HOME/$unit"
      else
        rm -f "$SYSTEMD_HOME/$unit"
        systemctl --user disable "$unit" >/dev/null 2>&1 || true
      fi
    done
    systemctl --user daemon-reload || true
    if [[ "$START_SERVICES" == "1" ]]; then
      systemctl --user restart trnm-entitlement-signer.service || true
      systemctl --user restart trnm-game-server.service || true
      if [[ "${unit_existed[trnm-settlement-worker.service]:-0}" == "1" ]]; then
        systemctl --user restart trnm-settlement-worker.service || true
      fi
    fi
    echo "TRNM World user-unit deployment failed; previous units were restored from $BACKUP_DIR" >&2
  fi
  exit "$status"
}
trap rollback EXIT

for unit in "${units[@]}"; do
  if [[ -f "$SYSTEMD_HOME/$unit" ]]; then
    unit_existed[$unit]=1
    install -m 0644 "$SYSTEMD_HOME/$unit" "$BACKUP_DIR/$unit"
  else
    unit_existed[$unit]=0
  fi
done

escape_sed_replacement() {
  printf '%s' "$1" | sed -e 's/[&|]/\\&/g'
}

world_root_escaped="$(escape_sed_replacement "$ROOT_DIR")"
config_home_escaped="$(escape_sed_replacement "$CONFIG_HOME")"
state_home_escaped="$(escape_sed_replacement "$STATE_HOME")"

render_unit() {
  local source="$1"
  local destination="$2"
  local temporary="$BACKUP_DIR/$(basename "$destination").rendered"
  sed \
    -e "s|@TRNM_WORLD_ROOT@|$world_root_escaped|g" \
    -e "s|@TRNM_CONFIG_HOME@|$config_home_escaped|g" \
    -e "s|@TRNM_STATE_HOME@|$state_home_escaped|g" \
    "$source" >"$temporary"
  if grep -q '@TRNM_.*@' "$temporary"; then
    echo "unresolved systemd template placeholder in $source" >&2
    return 1
  fi
  install -m 0644 "$temporary" "$destination"
}

render_unit \
  "$ROOT_DIR/deploy/systemd/trnm-game-server.service" \
  "$SYSTEMD_HOME/trnm-game-server.service"
render_unit \
  "$ROOT_DIR/deploy/systemd/trnm-entitlement-signer.service" \
  "$SYSTEMD_HOME/trnm-entitlement-signer.service"
render_unit \
  "$ROOT_DIR/deploy/systemd/trnm-settlement-worker.service" \
  "$SYSTEMD_HOME/trnm-settlement-worker.service"

if [[ ! -e "$CONFIG_HOME/game-server.env" ]]; then
  install -m 0600 \
    "$ROOT_DIR/config/trnm-game-server.env.example" \
    "$CONFIG_HOME/game-server.env"
fi
if [[ ! -e "$CONFIG_HOME/entitlement-signer.env" ]]; then
  install -m 0600 \
    "$ROOT_DIR/config/trnm-entitlement-signer.env.example" \
    "$CONFIG_HOME/entitlement-signer.env"
fi
if [[ ! -e "$CONFIG_HOME/settlement-worker.env" ]]; then
  install -m 0600 \
    "$ROOT_DIR/config/trnm-settlement-worker.env.example" \
    "$CONFIG_HOME/settlement-worker.env"
fi
chmod 600 \
  "$CONFIG_HOME/game-server.env" \
  "$CONFIG_HOME/entitlement-signer.env" \
  "$CONFIG_HOME/settlement-worker.env"

systemctl --user daemon-reload
systemctl --user enable "${units[@]}"

if [[ "$START_SERVICES" != "1" ]]; then
  trap - EXIT
  printf '%s\n' \
    "Installed and enabled TRNM World units without starting them." \
    "Configure: $CONFIG_HOME/game-server.env" \
    "Configure: $CONFIG_HOME/entitlement-signer.env" \
    "Configure: $CONFIG_HOME/settlement-worker.env" \
    "Then run: $ROOT_DIR/scripts/install-trnm-game-server-systemd.sh --start"
  exit 0
fi

if grep -n 'REPLACE_' \
  "$CONFIG_HOME/game-server.env" \
  "$CONFIG_HOME/entitlement-signer.env" \
  "$CONFIG_HOME/settlement-worker.env"; then
  echo "replace every REPLACE_* configuration value before --start" >&2
  exit 1
fi

systemctl --user restart trnm-entitlement-signer.service
for _ in $(seq 1 40); do
  if curl -fsS --max-time 3 http://127.0.0.1:7010/v1/signer/readiness \
    | jq -e '.status == "ok" and .contract_version == "trnm_entitlement_signer_v1" and
      .database_pool_saturation_healthy == true and .database_pool_max_connections == 4' \
      >/dev/null 2>&1; then
    break
  fi
  sleep 0.25
done
curl -fsS --max-time 3 http://127.0.0.1:7010/v1/signer/readiness \
  | jq -e '.status == "ok" and .contract_version == "trnm_entitlement_signer_v1" and
    .database_pool_saturation_healthy == true and .database_pool_max_connections == 4' \
    >/dev/null

systemctl --user restart trnm-game-server.service
for _ in $(seq 1 60); do
  if curl -fsS --max-time 3 http://127.0.0.1:7005/v1/online/readiness \
    | jq -e '.status == "ok" and .tick_rate_hz == 10 and
      .database_pool_saturation_healthy == true and .database_pool_max_connections == 12 and
      .readiness_database_pool_saturation_healthy == true and
      .readiness_database_pool_min_connections == 4 and
      .readiness_database_pool_max_connections == 12' \
      >/dev/null 2>&1; then
    break
  fi
  sleep 0.5
done
curl -fsS --max-time 3 http://127.0.0.1:7005/v1/online/readiness \
  | jq -e '.status == "ok" and .tick_rate_hz == 10 and
    .database_pool_saturation_healthy == true and .database_pool_max_connections == 12 and
    .readiness_database_pool_saturation_healthy == true and
    .readiness_database_pool_min_connections == 4 and
    .readiness_database_pool_max_connections == 12' >/dev/null

systemctl --user restart trnm-settlement-worker.service
for _ in $(seq 1 40); do
  if systemctl --user is-active --quiet trnm-settlement-worker.service; then
    break
  fi
  sleep 0.25
done
systemctl --user is-active --quiet trnm-settlement-worker.service
sleep 1
systemctl --user is-active --quiet trnm-settlement-worker.service

TRNM_REQUIRE_INSTALLED_RESOURCE_BUDGETS=1 \
  "$ROOT_DIR/scripts/check-trnm-online-resource-budgets.sh" >/dev/null

trap - EXIT
printf '%s\n' \
  "TRNM World signer, compatibility game server, and transaction-free settlement worker are ready."
