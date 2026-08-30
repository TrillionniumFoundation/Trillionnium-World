#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

python3 - <<'PY'
from __future__ import annotations

import re
from pathlib import Path

workflow_root = Path('.github/workflows')
workflows = sorted([*workflow_root.glob('*.yml'), *workflow_root.glob('*.yaml')])
if not workflows:
    raise SystemExit('no GitHub Actions workflows found')

full_sha = re.compile(r'^[0-9a-f]{40}$')
forbidden_commands = (
    'git push',
    'git commit',
    'git tag',
    'gh pr merge',
    'gh api --method put',
    'gh api --method post',
    'cargo fix',
    'cargo clippy --fix',
    'clippy --fix',
)

errors: list[str] = []
checked_actions = 0
for path in workflows:
    text = path.read_text(encoding='utf-8')
    lowered = text.lower()

    if re.search(r'(?m)^\s*contents\s*:\s*write\s*(?:#.*)?$', lowered):
        errors.append(f'{path}: contents: write is forbidden in validation workflows')
    if re.search(r'(?m)^\s*pull-requests\s*:\s*write\s*(?:#.*)?$', lowered):
        errors.append(f'{path}: pull-requests: write is forbidden in validation workflows')
    if re.search(r'(?m)^\s*actions\s*:\s*write\s*(?:#.*)?$', lowered):
        errors.append(f'{path}: actions: write is forbidden in validation workflows')

    for command in forbidden_commands:
        if command in lowered:
            errors.append(f'{path}: forbidden source/repository mutation command: {command}')

    for lineno, line in enumerate(text.splitlines(), start=1):
        stripped = line.strip()
        if not stripped.startswith('uses:') and ' uses:' not in line:
            continue
        value = stripped.split('uses:', 1)[1].strip().strip('"\'')
        if value.startswith('./'):
            continue
        if '@' not in value:
            errors.append(f'{path}:{lineno}: action reference has no immutable revision')
            continue
        revision = value.rsplit('@', 1)[1]
        checked_actions += 1
        if not full_sha.fullmatch(revision):
            errors.append(f'{path}:{lineno}: action is not pinned to a full commit SHA: {value}')

    for match in re.finditer(r'(?m)^\s*persist-credentials\s*:\s*(\S+)', text):
        if match.group(1).strip('"\'').lower() != 'false':
            errors.append(f'{path}: checkout persist-credentials must be false')

if checked_actions == 0:
    errors.append('no third-party action references were checked')

if errors:
    raise SystemExit('\n'.join(errors))

print(f'ci_read_only_boundary=passed workflows={len(workflows)} actions={checked_actions}')
PY
