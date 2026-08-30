from __future__ import annotations

import re
from common import ROOT, read

compat = read(ROOT / "docs/protocol/trnm-world-compatibility-matrix-v1.md").lower()
for marker in [
    "unknown combinations fail closed",
    "missing field",
    "wildcard",
    "owner and approver",
    "start date",
    "end date",
    "retirement record",
]:
    if marker not in compat:
        raise SystemExit(f"compatibility contract marker missing: {marker}")

pg = read(ROOT / "docs/database/trnm-world-postgres-contract-v1.md").lower()
catalog = read(ROOT / "docs/database/trnm-world-stored-procedure-catalog-v1.md").lower()
locks = read(ROOT / "docs/database/trnm-world-lock-order-v1.md").lower()
for marker in [
    "postgresql 16.4",
    "global lock order",
    "postconditions",
    "retry:",
    "privilege model",
    "pitr/failover contract",
    "old primary",
]:
    if marker not in pg:
        raise SystemExit(f"PostgreSQL contract marker missing: {marker}")
for marker in ["preconditions", "mutations and returned result", "retryable/nonretryable sqlstates"]:
    if marker not in catalog:
        raise SystemExit(f"stored-procedure catalogue marker missing: {marker}")
for marker in ["lock order", "settlement", "campaign", "terminal"]:
    if marker not in locks:
        raise SystemExit(f"lock-order contract marker missing: {marker}")

clean_host = read(ROOT / "docs/operations/trnm-world-clean-host-install-contract-v1.md").lower()
for marker in [
    "exact selector",
    "install",
    "upgrade",
    "rollback",
    "uninstall",
    "sibling repository",
    "environment acceptance",
]:
    if marker not in clean_host:
        raise SystemExit(f"clean-host contract marker missing: {marker}")

portable_files = []
for subtree in ("packaging", "deploy"):
    base = ROOT / subtree
    if base.exists():
        portable_files.extend(path for path in base.rglob("*") if path.is_file())
scripts_root = ROOT / "scripts"
if scripts_root.exists():
    for path in scripts_root.iterdir():
        if path.is_file() and path.name.startswith(
            ("install", "uninstall", "package", "build", "promote", "rollback")
        ):
            portable_files.append(path)
for path in portable_files:
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        continue
    if re.search(r"/home/[^/$({\s]+|/Users/[^/$({\s]+|C:\\Users\\[^\\%${\s]+", text):
        raise SystemExit(f"personal absolute path found in portable surface: {path}")
print(f"database_install_contracts=passed portable_files={len(portable_files)}")
