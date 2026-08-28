#!/usr/bin/env bash
set -euo pipefail

root=${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)}
module="$root/tools/trnm-world-transition-https-v1"

bash "$root/scripts/check-trnm-world-transition-https-fixture.sh" "$root"
bash "$root/scripts/test-trnm-world-transition-https-fixture-negative.sh" "$root"

unformatted=$(gofmt -l "$module")
if [[ -n "$unformatted" ]]; then
  printf 'unformatted Go files:\n%s\n' "$unformatted" >&2
  exit 1
fi

(
  cd "$module"
  go test ./... -count=1
  go test -race ./internal/fixture -count=1
  go vet ./...
  CGO_ENABLED=0 go build -trimpath -ldflags='-s -w -buildid=' \
    -o /tmp/trnm-world-transition-https-v1 ./cmd/trnm-world-transition-https-v1
)

printf '%s\n' 'World transition HTTPS fixture aggregate local gate: passed'
