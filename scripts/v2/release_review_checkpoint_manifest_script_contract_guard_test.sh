#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh"

while IFS= read -r line; do
  [[ -z "$line" ]] && continue
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] release review checkpoint manifest script missing contract line: $line" >&2
    exit 1
  fi
done <<'REQUIRED_LINES'
trillionnium_world_release_review_checkpoint_manifest_v1
release-review-checkpoint-manifest.json
release-review-checkpoint-manifest.md
git status --porcelain=v1 -uall
--slurpfile entries "$ENTRIES_FILE"
cex_adapter_readiness
release_review_surface
external_evidence_validators
native_bevy_host_playability
map_pack_repository_boundary
code_surface
repo_infra_dev_environment
generated_acceptance_evidence
docs_planning
uncategorized_count
release-review-ci-gate.json
cex-production-adapter-readiness.json
checkpoint_manifest_only_not_public_launch_evidence
groups_current_working_tree_only
does_not_commit_or_stage_files
CEX adapter readiness is incubator runtime adapter evidence, not real external public-launch evidence.
REQUIRED_LINES

if grep -Eq 'git (add|commit|push)|gh pr create|curl .*-X POST|rm -rf' "$SCRIPT"; then
  echo "[FAIL] checkpoint manifest script must stay read-only and local" >&2
  exit 1
fi

echo "[PASS] release review checkpoint manifest script keeps git-status source, grouped WIP slices, evidence snapshots, and read-only boundary"
