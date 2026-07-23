#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest_path="$repo_root/trillionnium/Cargo.toml"
target_dir="$repo_root/target/release"
output_dir="$repo_root/run/distribution"
platform=""
require_clean=false

while (( $# > 0 )); do
  case "$1" in
    --target-dir)
      target_dir="${2:?--target-dir requires a path}"
      shift 2
      ;;
    --output-dir)
      output_dir="${2:?--output-dir requires a path}"
      shift 2
      ;;
    --platform)
      platform="${2:?--platform requires a value}"
      shift 2
      ;;
    --require-clean)
      require_clean=true
      shift
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

for command in cargo git gzip jq rustc sha256sum tar; do
  command -v "$command" >/dev/null || {
    echo "required packaging command is unavailable: $command" >&2
    exit 1
  }
done

if [[ -z "$platform" ]]; then
  case "$(rustc -vV | sed -n 's/^host: //p')" in
    x86_64-unknown-linux-*) platform="linux-x86_64" ;;
    aarch64-unknown-linux-*) platform="linux-aarch64" ;;
    *)
      echo "unsupported automatic package platform; pass --platform explicitly" >&2
      exit 1
      ;;
  esac
fi

case "$platform" in
  linux-x86_64|linux-aarch64) ;;
  *)
    echo "unsupported package platform: $platform" >&2
    exit 1
    ;;
esac

source_commit="$(git -C "$repo_root" rev-parse HEAD)"
source_short="${source_commit:0:12}"
source_epoch="$(git -C "$repo_root" show -s --format=%ct HEAD)"
source_dirty=false
if [[ -n "$(git -C "$repo_root" status --porcelain)" ]]; then
  source_dirty=true
fi
if [[ "$require_clean" == true && "$source_dirty" == true ]]; then
  echo "refusing to package a dirty source tree" >&2
  exit 1
fi

metadata_file="$(mktemp)"
cleanup_metadata() {
  rm -f "$metadata_file"
}
trap cleanup_metadata EXIT
cargo metadata --manifest-path "$manifest_path" --locked --format-version 1 \
  >"$metadata_file"
version="$(jq -er '.packages[] | select(.name == "trnm-first-contact") | .version' \
  "$metadata_file")"
bundle_name="trnm-game-${platform}-${version}-${source_short}"

mkdir -p "$output_dir"
work_dir="$(mktemp -d "$output_dir/.trnm-package.XXXXXX")"
cleanup() {
  rm -rf "$work_dir"
  cleanup_metadata
}
trap cleanup EXIT
bundle_dir="$work_dir/$bundle_name"
mkdir -p "$bundle_dir/bin" "$bundle_dir/assets" "$bundle_dir/docs" \
  "$bundle_dir/licenses" "$bundle_dir/share/applications"

for binary in trnm-first-contact trnm-online-product trnm-game-server trnm-entitlement-signer; do
  install -Dm755 "$target_dir/$binary" "$bundle_dir/bin/$binary"
done
cp -a "$repo_root/assets/first_contact" "$bundle_dir/assets/"
install -Dm644 "$repo_root/GAME_STATUS.md" "$bundle_dir/docs/GAME_STATUS.md"
install -Dm644 "$repo_root/trillionnium/Cargo.lock" "$bundle_dir/Cargo.lock"
install -Dm644 "$repo_root/packaging/trnm-first-contact.desktop" \
  "$bundle_dir/share/applications/trnm-first-contact.desktop"
install -Dm644 "$repo_root/packaging/TRILLIONNIUM-INTERNAL-LICENSE-NOTICE.txt" \
  "$bundle_dir/licenses/TRILLIONNIUM-INTERNAL-LICENSE-NOTICE.txt"
install -Dm644 "$repo_root/packaging/linux-runtime-requirements.txt" \
  "$bundle_dir/RUNTIME_REQUIREMENTS.txt"
install -Dm644 "$repo_root/trillionnium/vendor/wayland-scanner/LICENSE.txt" \
  "$bundle_dir/licenses/wayland-scanner-LICENSE.txt"

jq '{contract_version:"trnm_third_party_license_inventory_v1",
      dependency_count:(.packages | length),
      dependencies:(.packages | map({name,version,source:(.source // "workspace"),
        license:(.license // "UNKNOWN")}) | sort_by(.name,.version,.source))}' \
  "$metadata_file" >"$bundle_dir/licenses/third-party-licenses.json"

cat >"$bundle_dir/trnm-first-contact" <<'LAUNCHER'
#!/usr/bin/env bash
set -euo pipefail
bundle_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export TRNM_ASSET_ROOT="$bundle_root/assets"
exec "$bundle_root/bin/trnm-first-contact" "$@"
LAUNCHER
chmod 755 "$bundle_dir/trnm-first-contact"

files_json="$work_dir/files.jsonl"
: >"$files_json"
while IFS= read -r -d '' file; do
  relative="${file#"$bundle_dir/"}"
  jq -cn --arg path "$relative" \
    --arg sha256 "$(sha256sum "$file" | awk '{print $1}')" \
    --argjson bytes "$(stat -c '%s' "$file")" \
    '{path:$path,sha256:$sha256,bytes:$bytes}' >>"$files_json"
done < <(find "$bundle_dir" -type f -print0 | sort -z)

jq -n --slurpfile files "$files_json" \
  --arg version "$version" --arg platform "$platform" \
  --arg source_commit "$source_commit" --argjson source_dirty "$source_dirty" \
  --argjson source_date_epoch "$source_epoch" \
  '{contract_version:"trnm_game_distribution_manifest_v1",
    product:"Trillionnium: First Contact",version:$version,platform:$platform,
    source_commit:$source_commit,source_dirty:$source_dirty,
    source_date_epoch:$source_date_epoch,public_launch_ready:false,
    distribution_scope:"internal_evaluation",files:$files}' \
  >"$bundle_dir/manifest.json"

(
  cd "$bundle_dir"
  while IFS= read -r -d '' file; do
    sha256sum "${file#./}"
  done < <(find . -type f ! -name SHA256SUMS -print0 | sort -z)
) >"$bundle_dir/SHA256SUMS"
(
  cd "$bundle_dir"
  sha256sum -c SHA256SUMS >/dev/null
)

archive="$output_dir/$bundle_name.tar.gz"
archive_tmp="$work_dir/$bundle_name.tar.gz"
tar --sort=name --mtime="@$source_epoch" --owner=0 --group=0 --numeric-owner \
  -C "$work_dir" -cf - "$bundle_name" | gzip -n >"$archive_tmp"
mv "$archive_tmp" "$archive"
archive_sha256="$(sha256sum "$archive" | awk '{print $1}')"
printf '%s  %s\n' "$archive_sha256" "$(basename "$archive")" >"$archive.sha256"

jq -n --arg archive "$archive" --arg sha256 "$archive_sha256" \
  --arg bundle "$bundle_name" --arg version "$version" --arg platform "$platform" \
  --arg source_commit "$source_commit" --argjson source_dirty "$source_dirty" \
  '{status:"passed",contract_version:"trnm_game_distribution_package_v1",
    archive:$archive,archive_sha256:$sha256,bundle:$bundle,version:$version,
    platform:$platform,source_commit:$source_commit,source_dirty:$source_dirty,
    public_launch_ready:false}'
