#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$HOME/.config/systemd/user"
install -m 0644 "$ROOT_DIR/deploy/systemd/trnm-game-server.service" \
  "$HOME/.config/systemd/user/trnm-game-server.service"
systemctl --user daemon-reload
systemctl --user enable trnm-game-server.service
systemctl --user restart trnm-game-server.service
