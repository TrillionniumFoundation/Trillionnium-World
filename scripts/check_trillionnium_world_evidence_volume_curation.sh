#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/development/trillionnium-world-evidence-volume-curation-2026-07-07.md"
DOC="$ROOT/$DOC_REL"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-evidence-volume-curation.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-evidence-volume-curation.md"
mkdir -p "$ACCEPTANCE_DIR"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq -- "$needle" "$path"; then
    echo "[FAIL] $path missing required text: $needle" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "[FAIL] missing evidence volume curation doc: $DOC" >&2
  exit 1
fi
if [[ ! -d "$S5_DIR" ]]; then
  echo "[FAIL] missing S5 evidence directory: $S5_DIR" >&2
  exit 1
fi

require_text "$DOC" "Status: local evidence-volume curation plan."
require_text "$DOC" "Do not delete, compress, move, archive, rewrite, or prune acceptance evidence"
require_text "$DOC" "Preserve \`acceptance/S5_native_bevy_device/latest\` as the source of truth."
require_text "$DOC" '| `reviewer_summary` |'
require_text "$DOC" '| `live_player_screen` |'
require_text "$DOC" '| `representative_visuals` |'
require_text "$DOC" '| `raw_visual_archive_candidate` |'
require_text "$DOC" '| `external_evidence_blockers` |'

s5_latest_kib="$(du -sk "$S5_DIR" | awk '{print $1}')"
file_count="$(find "$S5_DIR" -type f | wc -l | tr -d ' ')"
large_file_count="$(find "$S5_DIR" -type f -size +10M | wc -l | tr -d ' ')"
ppm_file_count="$(find "$S5_DIR" -type f -name '*.ppm' | wc -l | tr -d ' ')"
png_file_count="$(find "$S5_DIR" -type f -name '*.png' | wc -l | tr -d ' ')"
json_file_count="$(find "$S5_DIR" -type f -name '*.json' | wc -l | tr -d ' ')"

top_level_entries_json="$(
  du -sk "$S5_DIR"/* 2>/dev/null \
    | sort -nr \
    | awk 'NR <= 20' \
    | awk -v root="$ROOT/" '{size=$1; $1=""; sub(/^ /, ""); path=$0; sub(root, "", path); printf "%s\t%s\n", size, path}' \
    | jq -Rn '[inputs | select(length > 0) | split("\t") | {kib: (.[0] | tonumber), path: .[1]}]'
)"

top_large_files_json="$(
  find "$S5_DIR" -type f -printf '%s\t%p\n' \
    | sort -nr \
    | awk 'NR <= 30' \
    | awk -v root="$ROOT/" 'BEGIN {FS="\t"} {path=$2; sub(root, "", path); printf "%s\t%s\n", $1, path}' \
    | jq -Rn '[inputs | select(length > 0) | split("\t") | {bytes: (.[0] | tonumber), path: .[1]}]'
)"

extension_counts_json="$(
  find "$S5_DIR" -type f -printf '%f\n' \
    | awk '
      {
        ext = "no_ext";
        if ($0 ~ /\./) {
          n = split($0, parts, ".");
          ext = parts[n];
        }
        count[ext] += 1;
      }
      END {
        for (ext in count) {
          printf "%s\t%s\n", count[ext], ext;
        }
      }
    ' \
    | sort -nr \
    | jq -Rn '[inputs | select(length > 0) | split("\t") | {count: (.[0] | tonumber), extension: .[1]}]'
)"

handoff_slices_json="$(jq -nc '[
  {
    id: "reviewer_summary",
    status: "manifest_first",
    reviewer_use: "status and no-credit boundaries"
  },
  {
    id: "live_player_screen",
    status: "preserve_current_runner_artifacts",
    reviewer_use: "inspect the playable First Contact surface"
  },
  {
    id: "representative_visuals",
    status: "curate_without_deleting_raw",
    reviewer_use: "review visual/product coverage without opening every raw file"
  },
  {
    id: "raw_visual_archive_candidate",
    status: "blocked_on_explicit_archive_approval",
    reviewer_use: "deep audit only"
  },
  {
    id: "external_evidence_blockers",
    status: "blocked_on_real_external_evidence",
    reviewer_use: "confirm public launch remains blocked"
  }
]')"

jq -n \
  --arg contract_version "trillionnium_world_evidence_volume_curation_v1" \
  --arg status "evidence_volume_curation_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --arg s5_path "acceptance/S5_native_bevy_device/latest" \
  --argjson s5_latest_kib "$s5_latest_kib" \
  --argjson file_count "$file_count" \
  --argjson large_file_count "$large_file_count" \
  --argjson ppm_file_count "$ppm_file_count" \
  --argjson png_file_count "$png_file_count" \
  --argjson json_file_count "$json_file_count" \
  --argjson top_level_entries "$top_level_entries_json" \
  --argjson top_large_files "$top_large_files_json" \
  --argjson extension_counts "$extension_counts_json" \
  --argjson handoff_slices "$handoff_slices_json" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    green: true,
    doc_path: $doc_path,
    s5_evidence_path: $s5_path,
    s5_latest_kib: $s5_latest_kib,
    file_count: $file_count,
    large_file_count: $large_file_count,
    ppm_file_count: $ppm_file_count,
    png_file_count: $png_file_count,
    json_file_count: $json_file_count,
    top_level_entries: $top_level_entries,
    top_large_files: $top_large_files,
    extension_counts: $extension_counts,
    handoff_slices: $handoff_slices,
    handoff_slice_count: ($handoff_slices | length),
    evidence_volume_risk_active: ($s5_latest_kib > 10000000 and $large_file_count > 100),
    manifest_before_archive_required: true,
    source_evidence_preserved: true,
    deletion_performed: false,
    compression_performed: false,
    archive_movement_performed: false,
    public_launch_ready_claimed: false,
    android_s5_real_device_claimed: false,
    beta_cohort_evidence_claimed: false,
    production_ready_ui_claimed: false,
    commercial_launch_evidence_claimed: false,
    no_credit_boundary: "local evidence-volume inventory only; no delete, compress, archive, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, or public-network credit",
    source_of_truth: "Curation starts with a manifest over S5 local evidence volume and preserves all source artifacts until explicit cleanup/archive approval."
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_evidence_volume_curation_v1"
  and .status == "evidence_volume_curation_ready"
  and .green == true
  and .s5_latest_kib > 10000000
  and .file_count > 1000
  and .large_file_count > 100
  and .ppm_file_count > 100
  and .handoff_slice_count == 5
  and ([.handoff_slices[].id] == [
    "reviewer_summary",
    "live_player_screen",
    "representative_visuals",
    "raw_visual_archive_candidate",
    "external_evidence_blockers"
  ])
  and (.top_level_entries | length) >= 10
  and (.top_large_files | length) >= 10
  and .evidence_volume_risk_active == true
  and .manifest_before_archive_required == true
  and .source_evidence_preserved == true
  and .deletion_performed == false
  and .compression_performed == false
  and .archive_movement_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and (.no_credit_boundary | contains("local evidence-volume inventory only"))
  and (.source_of_truth | contains("preserves all source artifacts"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Evidence Volume Curation\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- S5 latest KiB: `%s`\n' "$(jq -r '.s5_latest_kib' "$SUMMARY")"
  printf -- '- files: `%s`, large files >10M: `%s`\n' \
    "$(jq -r '.file_count' "$SUMMARY")" \
    "$(jq -r '.large_file_count' "$SUMMARY")"
  printf -- '- PPM/PNG/JSON: `%s` / `%s` / `%s`\n' \
    "$(jq -r '.ppm_file_count' "$SUMMARY")" \
    "$(jq -r '.png_file_count' "$SUMMARY")" \
    "$(jq -r '.json_file_count' "$SUMMARY")"
  printf -- '- deletion performed: `%s`\n' "$(jq -r '.deletion_performed' "$SUMMARY")"
  printf -- '- compression performed: `%s`\n' "$(jq -r '.compression_performed' "$SUMMARY")"
  printf -- '- archive movement performed: `%s`\n' "$(jq -r '.archive_movement_performed' "$SUMMARY")"
  printf -- '- public launch ready claimed: `%s`\n' "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")"
  printf -- '- Android S5 real-device claimed: `%s`\n\n' "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")"
  printf '## Top Level Entries\n\n'
  jq -r '.top_level_entries[:10][] | "- `\(.path)`: \(.kib) KiB"' "$SUMMARY"
  printf '\n## Handoff Slices\n\n'
  jq -r '.handoff_slices[] | "- `\(.id)`: \(.status) - \(.reviewer_use)"' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_EVIDENCE_VOLUME_CURATION_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
