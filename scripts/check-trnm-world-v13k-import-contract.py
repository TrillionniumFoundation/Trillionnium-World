#!/usr/bin/env python3
"""Execute the importer's offline contract/fault tests; never publish source."""
from pathlib import Path
import subprocess
import sys

if __name__ == "__main__":
    result = subprocess.run(
        [sys.executable, str(Path(__file__).with_name("test-trnm-world-v13k-import.py"))],
        check=False,
        timeout=120,
    )
    if result.returncode:
        raise SystemExit(result.returncode)
    print("TRNM_WORLD_V13K_IMPORT_CONTRACT=PASS (offline contract tests only)")
