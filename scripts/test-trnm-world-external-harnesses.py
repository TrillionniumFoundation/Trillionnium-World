#!/usr/bin/env python3
"""Offline fail-closed tests for World external evidence harnesses.

These tests validate argument and safety boundaries only. They never create
cross-host, public-network, human, custody or commercial evidence.
"""
from __future__ import annotations

import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
CROSS = ROOT / "scripts/run-trnm-world-cross-host-evidence-v1.sh"
AGENT = ROOT / "scripts/trnm-world-cross-host-agent-v1.sh"
ENDURANCE = ROOT / "scripts/run-trnm-world-endurance-24h-v1.sh"
PUBLIC = ROOT / "scripts/run-trnm-world-public-edge-evidence-v1.sh"


def clean_environment() -> dict[str, str]:
    result = {
        "PATH": os.environ.get("PATH", "/usr/bin:/bin"),
        "HOME": os.environ.get("HOME", "/tmp"),
        "LANG": "C.UTF-8",
        "LC_ALL": "C.UTF-8",
    }
    return result


class HarnessSafetyTests(unittest.TestCase):
    def run(self, script: Path, *args: str, environment: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:  # type: ignore[override]
        return subprocess.run(
            ["bash", str(script), *args],
            cwd=ROOT,
            env=environment or clean_environment(),
            capture_output=True,
            text=True,
            timeout=20,
            check=False,
        )

    def test_shell_syntax(self) -> None:
        for script in (CROSS, AGENT, ENDURANCE, PUBLIC):
            with self.subTest(script=script.name):
                result = subprocess.run(["bash", "-n", str(script)], capture_output=True, text=True, timeout=10)
                self.assertEqual(result.returncode, 0, result.stderr)

    def test_cross_host_rejects_missing_configuration(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            result = self.run(CROSS, tmp)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("required", result.stderr.lower())

    def test_cross_host_rejects_same_hosts_before_network(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            package = root / "package"; package.write_text("package")
            agent = root / "agent"; agent.write_text("#!/bin/sh\nexit 0\n"); agent.chmod(0o700)
            authorization = root / "authorization"; authorization.write_text("authorized")
            fault = root / "fault"; fault.write_text("fault plan")
            known = root / "known_hosts"; known.write_text("example invalid-key")
            environment = clean_environment()
            environment.update({
                "TRNM_HOST_A": "same-host",
                "TRNM_HOST_B": "same-host",
                "TRNM_WORLD_PACKAGE": str(package),
                "TRNM_CROSS_HOST_AGENT": str(agent),
                "TRNM_CROSS_HOST_AUTHORIZATION": str(authorization),
                "TRNM_FAULT_PLAN": str(fault),
                "TRNM_REMOTE_EVIDENCE_ROOT": "/var/tmp/trnm-evidence",
                "TRNM_SSH_KNOWN_HOSTS": str(known),
            })
            result = self.run(CROSS, str(root / "out"), environment=environment)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("hosts must differ", result.stderr)

    def test_endurance_rejects_short_duration_before_workload(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            paths = {}
            for name in ("authorization", "thresholds", "binary", "config", "topology"):
                path = root / name; path.write_text(name); paths[name] = path
            workload = root / "workload"; workload.write_text("#!/bin/sh\nexit 0\n"); workload.chmod(0o700)
            metrics = root / "metrics"; metrics.write_text("#!/bin/sh\necho '{}'"); metrics.chmod(0o700)
            environment = clean_environment()
            environment.update({
                "TRNM_ENDURANCE_AUTHORIZATION": str(paths["authorization"]),
                "TRNM_ENDURANCE_WORKLOAD": str(workload),
                "TRNM_ENDURANCE_METRICS": str(metrics),
                "TRNM_ENDURANCE_THRESHOLDS": str(paths["thresholds"]),
                "TRNM_WORLD_BINARY": str(paths["binary"]),
                "TRNM_ENDURANCE_CONFIG": str(paths["config"]),
                "TRNM_ENDURANCE_TOPOLOGY": str(paths["topology"]),
                "TRNM_ENDURANCE_SECONDS": "60",
            })
            result = self.run(ENDURANCE, str(root / "out"), environment=environment)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("at least 86400", result.stderr)

    def test_public_edge_rejects_non_https_before_network(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            authorization = root / "authorization.json"; authorization.write_text("{}")
            config = root / "config"; config.write_text("config")
            thresholds = root / "thresholds"; thresholds.write_text("thresholds")
            environment = clean_environment()
            environment.update({
                "TRNM_PUBLIC_EDGE_URL": "http://example.com",
                "TRNM_PUBLIC_EDGE_AUTHORIZATION": str(authorization),
                "TRNM_PUBLIC_EDGE_CONFIG": str(config),
                "TRNM_PUBLIC_EDGE_THRESHOLDS": str(thresholds),
                "TRNM_PUBLIC_EDGE_HEALTH_PATH": "/health",
            })
            result = self.run(PUBLIC, str(root / "out"), environment=environment)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("must be HTTPS", result.stderr)

    def test_public_edge_rejects_unbounded_request_count(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            authorization = root / "authorization.json"; authorization.write_text("{}")
            config = root / "config"; config.write_text("config")
            thresholds = root / "thresholds"; thresholds.write_text("thresholds")
            environment = clean_environment()
            environment.update({
                "TRNM_PUBLIC_EDGE_URL": "https://example.com",
                "TRNM_PUBLIC_EDGE_AUTHORIZATION": str(authorization),
                "TRNM_PUBLIC_EDGE_CONFIG": str(config),
                "TRNM_PUBLIC_EDGE_THRESHOLDS": str(thresholds),
                "TRNM_PUBLIC_EDGE_HEALTH_PATH": "/health",
                "TRNM_PUBLIC_EDGE_RATE_REQUESTS": "1000000",
            })
            result = self.run(PUBLIC, str(root / "out"), environment=environment)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("1..300", result.stderr)

    def test_remote_agent_rejects_unknown_phase(self) -> None:
        result = self.run(AGENT, "unknown")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("unsupported", result.stderr)


if __name__ == "__main__":
    unittest.main(verbosity=2)
