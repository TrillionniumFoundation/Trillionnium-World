#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
PACKAGER="$ROOT_DIR/scripts/package-trnm-world-transition-contract.py"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT

fail() {
  printf 'TRNM World transition package failed: %s\n' "$*" >&2
  exit 1
}

[[ -f "$PACKAGER" ]] || fail 'deterministic contract packager is missing'
SOURCE_COMMIT="${TRNM_TRANSITION_SOURCE_COMMIT:-}"
SOURCE_TREE="${TRNM_TRANSITION_SOURCE_TREE:-}"
if [[ -z "$SOURCE_COMMIT" ]]; then
  SOURCE_COMMIT="$(git -C "$ROOT_DIR" rev-parse HEAD)"
fi
if [[ -z "$SOURCE_TREE" ]]; then
  SOURCE_TREE="$(git -C "$ROOT_DIR" rev-parse "${SOURCE_COMMIT}^{tree}")"
fi
[[ "$SOURCE_COMMIT" =~ ^[0-9a-f]{40}$ ]] || fail 'invalid source commit'
[[ "$SOURCE_TREE" =~ ^[0-9a-f]{40}$ ]] || fail 'invalid source tree'

for name in first second; do
  python3 "$PACKAGER" \
    --root "$ROOT_DIR" \
    --output-dir "$TEMP_ROOT/$name" \
    --source-commit "$SOURCE_COMMIT" \
    --source-tree "$SOURCE_TREE" \
    > "$TEMP_ROOT/$name.summary.json"
done

for artifact in \
  trnm-world-transition-v1.tar.gz \
  trnm-world-transition-v1.tar.gz.sha256 \
  trnm-world-transition-v1.manifest.json; do
  cmp "$TEMP_ROOT/first/$artifact" "$TEMP_ROOT/second/$artifact" \
    || fail "contract artifact is not reproducible: $artifact"
done

python3 - \
  "$TEMP_ROOT/first/trnm-world-transition-v1.tar.gz" \
  "$TEMP_ROOT/first/trnm-world-transition-v1.tar.gz.sha256" \
  "$TEMP_ROOT/first/trnm-world-transition-v1.manifest.json" \
  "$SOURCE_COMMIT" \
  "$SOURCE_TREE" <<'PY'
from __future__ import annotations

import gzip
import hashlib
import io
import json
import pathlib
import re
import sys
import tarfile

archive_path = pathlib.Path(sys.argv[1])
checksum_path = pathlib.Path(sys.argv[2])
external_manifest_path = pathlib.Path(sys.argv[3])
source_commit = sys.argv[4]
source_tree = sys.argv[5]
sha64 = re.compile(r"^[0-9a-f]{64}$")


def reject(message: str) -> None:
    raise SystemExit(message)


def expect(condition: bool, message: str) -> None:
    if not condition:
        reject(message)


archive_bytes = archive_path.read_bytes()
archive_sha256 = hashlib.sha256(archive_bytes).hexdigest()
checksum_fields = checksum_path.read_text(encoding="utf-8").strip().split()
expect(
    checksum_fields == [archive_sha256, archive_path.name],
    "archive sidecar checksum mismatch",
)

with gzip.GzipFile(fileobj=io.BytesIO(archive_bytes), mode="rb") as compressed:
    tar_bytes = compressed.read()

with tarfile.open(fileobj=io.BytesIO(tar_bytes), mode="r:") as archive:
    members = archive.getmembers()
    names = [member.name for member in members]
    expect(names == sorted(names), "archive members are not sorted")
    expect(len(names) == len(set(names)), "archive contains duplicate paths")
    expect(all(name.startswith("trnm-world-transition-v1/") for name in names), "archive root drift")
    expect(all(not name.startswith("/") and ".." not in pathlib.PurePosixPath(name).parts for name in names), "unsafe archive path")
    for member in members:
        expect(member.isfile(), f"non-regular archive member: {member.name}")
        expect(member.mtime == 0, f"nondeterministic mtime: {member.name}")
        expect(member.uid == 0 and member.gid == 0, f"nondeterministic owner: {member.name}")
        expect(member.uname == "" and member.gname == "", f"nondeterministic owner name: {member.name}")
        expect(member.mode == 0o644, f"unexpected archive mode: {member.name}")
    content = {
        member.name.removeprefix("trnm-world-transition-v1/"): archive.extractfile(member).read()
        for member in members
    }

expect("MANIFEST.json" in content, "archive manifest is missing")
manifest_bytes = content.pop("MANIFEST.json")
expect(manifest_bytes == external_manifest_path.read_bytes(), "internal/external manifest drift")
manifest = json.loads(manifest_bytes)
expect(manifest.get("bundle_version") == "trnm_world_transition_bundle_v1", "bundle version drift")
expect(manifest.get("contract_version") == "trnm_world_transition_v1", "contract version drift")
expect(manifest.get("source_commit") == source_commit, "bundle commit binding drift")
expect(manifest.get("source_tree") == source_tree, "bundle tree binding drift")
expect(manifest.get("next_dependency") == "WORLD-P0-003", "bundle next-dependency drift")
authority = manifest.get("authority_scope", {})
expect(authority == {
    "canonical_online_authority_claimed": False,
    "match_completed_v1_owned": False,
    "match_evidence_signing_key_owned": False,
}, "bundle authority scope overclaim")

entries = manifest.get("files")
expect(isinstance(entries, list) and entries, "bundle file manifest is empty")
expected_paths = [entry.get("path") for entry in entries]
expect(expected_paths == sorted(expected_paths), "manifest paths are not sorted")
expect(len(expected_paths) == len(set(expected_paths)), "manifest contains duplicate paths")
expect(set(expected_paths) == set(content), "archive/manifest file set drift")
for entry in entries:
    path = entry.get("path")
    payload = content[path]
    expected_hash = hashlib.sha256(payload).hexdigest()
    expect(sha64.fullmatch(str(entry.get("sha256", ""))) is not None, f"invalid manifest hash: {path}")
    expect(entry.get("sha256") == expected_hash, f"manifest hash mismatch: {path}")
    expect(entry.get("size") == len(payload), f"manifest size mismatch: {path}")

required = {
    "Cargo.lock",
    "Cargo.toml",
    "README.md",
    "rust-toolchain.toml",
    "src/canonical_json.rs",
    "src/contract.rs",
    "docs/protocol/trnm-world-transition-v1.md",
    "docs/protocol/schemas/trnm-world-transition-v1.schema.json",
    "docs/protocol/vectors/trnm-world-transition-v1.json",
    "docs/development/trnm-world-transition-contract-v1.json",
}
expect(set(content) == required, "bundle content allowlist drift")
expect(b"path = \"src/contract.rs\"" in content["Cargo.toml"], "bundle compiles wrong source root")
expect(b"channel = \"1.85.1\"" in content["rust-toolchain.toml"], "bundle toolchain drift")

print(
    json.dumps(
        {
            "archive_sha256": archive_sha256,
            "file_count": len(content),
            "source_commit": source_commit,
            "source_tree": source_tree,
        },
        sort_keys=True,
    )
)
PY

printf '%s\n' 'TRNM World exact-revision transition package passed.'
