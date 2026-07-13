#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$HOME/.config/systemd/user"
install -m 0644 "$ROOT_DIR/deploy/systemd/trnm-game-server.service" \
  "$HOME/.config/systemd/user/trnm-game-server.service"
install -m 0644 "$ROOT_DIR/deploy/systemd/trnm-entitlement-signer.service" \
  "$HOME/.config/systemd/user/trnm-entitlement-signer.service"
systemctl --user daemon-reload
systemctl --user enable trnm-entitlement-signer.service
systemctl --user restart trnm-entitlement-signer.service
for _ in $(seq 1 40); do
  curl -fsS http://127.0.0.1:7010/health >/dev/null 2>&1 && break
  sleep 0.25
done
curl -fsS http://127.0.0.1:7010/health >/dev/null
systemctl --user enable trnm-game-server.service
systemctl --user restart trnm-game-server.service
