#!/usr/bin/env bash
set -euo pipefail

archive="${1:?usage: check-trnm-game-package.sh <archive.tar.gz>}"
require_clean="${TRNM_REQUIRE_CLEAN_PACKAGE:-0}"

for command in diff find jq sha256sum tar; do
  command -v "$command" >/dev/null || {
    echo "required package-check command is unavailable: $command" >&2
    exit 1
  }
done
[[ -f "$archive" ]]
if [[ -f "$archive.sha256" ]]; then
  (
    cd "$(dirname "$archive")"
    sha256sum -c "$(basename "$archive").sha256" >/dev/null
  )
fi

while IFS= read -r entry; do
  [[ "$entry" != /* ]]
  [[ "/$entry/" != *"/../"* ]]
done < <(tar -tzf "$archive")

work_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$work_dir"
}
trap cleanup EXIT
tar -xzf "$archive" -C "$work_dir"
mapfile -t roots < <(find "$work_dir" -mindepth 1 -maxdepth 1 -type d -print)
[[ "${#roots[@]}" -eq 1 ]]
bundle_dir="${roots[0]}"
[[ -z "$(find "$work_dir" -mindepth 1 -maxdepth 1 ! -type d -print -quit)" ]]
[[ -z "$(find "$bundle_dir" -type l -print -quit)" ]]

for executable in \
  trnm-first-contact \
  bin/trnm-first-contact \
  bin/trnm-online-product \
  bin/trnm-game-server \
  bin/trnm-entitlement-signer \
  bin/trnm-settlement-worker; do
  [[ -x "$bundle_dir/$executable" ]]
done
for required in manifest.json SHA256SUMS Cargo.lock RUNTIME_REQUIREMENTS.txt \
  assets/first_contact/atlas.yaml assets/first_contact/maps/first_contact.yaml \
  licenses/TRILLIONNIUM-INTERNAL-LICENSE-NOTICE.txt \
  licenses/third-party-licenses.json licenses/wayland-scanner-LICENSE.txt \
  docs/GAME_STATUS.md share/applications/trnm-first-contact.desktop; do
  [[ -s "$bundle_dir/$required" ]]
done

(
  cd "$bundle_dir"
  sha256sum -c SHA256SUMS >/dev/null
)
jq -e '
  .contract_version == "trnm_game_distribution_manifest_v1"
  and .product == "Trillionnium: First Contact"
  and (.version | type == "string" and length > 0)
  and (.platform == "linux-x86_64" or .platform == "linux-aarch64")
  and (.source_commit | test("^[0-9a-f]{40}$"))
  and (.source_dirty | type == "boolean")
  and .public_launch_ready == false
  and .distribution_scope == "internal_evaluation"
  and (.files | length > 10)
  and ((.files | map(.path) | unique | length) == (.files | length))
  and all(.files[]; (.path | startswith("/") | not)
    and (.path | contains("../") | not)
    and (.sha256 | test("^[0-9a-f]{64}$"))
    and (.bytes | type == "number" and . >= 0))
' "$bundle_dir/manifest.json" >/dev/null
if [[ "$require_clean" == 1 ]]; then
  jq -e '.source_dirty == false' "$bundle_dir/manifest.json" >/dev/null
fi
jq -e '
  .contract_version == "trnm_third_party_license_inventory_v1"
  and .dependency_count == (.dependencies | length)
  and .dependency_count > 0
' "$bundle_dir/licenses/third-party-licenses.json" >/dev/null

expected_paths="$work_dir/expected-paths.txt"
actual_paths="$work_dir/actual-paths.txt"
jq -r '.files[].path' "$bundle_dir/manifest.json" | sort >"$expected_paths"
find "$bundle_dir" -type f ! -name manifest.json ! -name SHA256SUMS \
  -printf '%P\n' | sort >"$actual_paths"
diff -u "$expected_paths" "$actual_paths"

while IFS= read -r item; do
  path="$(jq -r .path <<<"$item")"
  expected_sha="$(jq -r .sha256 <<<"$item")"
  expected_bytes="$(jq -r .bytes <<<"$item")"
  [[ "$(sha256sum "$bundle_dir/$path" | awk '{print $1}')" == "$expected_sha" ]]
  [[ "$(stat -c '%s' "$bundle_dir/$path")" == "$expected_bytes" ]]
done < <(jq -c '.files[]' "$bundle_dir/manifest.json")

jq -n --arg archive "$archive" \
  --arg bundle "$(basename "$bundle_dir")" \
  --arg source_commit "$(jq -r .source_commit "$bundle_dir/manifest.json")" \
  --arg version "$(jq -r .version "$bundle_dir/manifest.json")" \
  '{status:"passed",contract_version:"trnm_game_distribution_check_v1",
    archive:$archive,bundle:$bundle,source_commit:$source_commit,version:$version,
    public_launch_ready:false}'
