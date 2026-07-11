#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
prefix="${PREFIX:-$HOME/.local}"
build=true

for arg in "$@"; do
  case "$arg" in
    --no-build) build=false ;;
    --prefix=*) prefix="${arg#--prefix=}" ;;
    *) echo "unknown argument: $arg" >&2; exit 2 ;;
  esac
done

if [[ "$build" == true ]]; then
  cargo build --manifest-path "$repo_root/trillionnium/Cargo.toml" --release -p trnm-first-contact
fi

install -Dm755 "$repo_root/target/release/trnm-first-contact" "$prefix/lib/trillionnium/trnm-first-contact"
install -Dm644 "$repo_root/packaging/trnm-first-contact.desktop" "$prefix/share/applications/trnm-first-contact.desktop"
mkdir -p "$prefix/share/trillionnium/assets"
cp -a "$repo_root/assets/." "$prefix/share/trillionnium/assets/"

launcher="$prefix/bin/trnm-first-contact"
mkdir -p "$(dirname "$launcher")"
sed \
  -e "s|@BINARY@|$prefix/lib/trillionnium/trnm-first-contact|g" \
  -e "s|@ASSET_ROOT@|$prefix/share/trillionnium/assets|g" \
  "$repo_root/packaging/trnm-first-contact-launcher.in" > "$launcher"
chmod 755 "$launcher"

echo "TRNM desktop install complete: $launcher"
