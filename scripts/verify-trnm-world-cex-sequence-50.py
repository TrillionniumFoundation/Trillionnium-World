#!/usr/bin/env python3
"""Verify the exact CEX Sequence-50 candidate before World adoption.

This verifier is read-only. It requires non-empty hosted execution, immutable
artifact binding, protected-main governance and an eligible independent review.
It never writes statuses, branches, source, reviews, tags, releases or deployments.
"""

from __future__ import annotations

import argparse
import io
import json
import os
from pathlib import Path, PurePosixPath
import sys
import urllib.error
import urllib.parse
import urllib.request
import zipfile

REPOSITORY = "TrillionniumFoundation/CEX"
PULL_REQUEST = 24
COMMIT = "dc0862b8cf88a1f4e6328d519947e19b81122de0"
TREE = "762e33a3f16c14347a44cec1d862a8e0ab447ad8"
BRANCH = "fix/cex-v12-seq45-full-gap-closure-20260902"
MIGRATION = "0088_enforce_provider_terminal_evidence_binding.sql"
TRIGGER_SEQUENCE = 50
REQUIRED_CONTEXTS = {
    "cex-v12/postgres-migration",
    "cex-v12/rust-format",
    "cex-v12/rust-test",
    "cex-v12/rust-clippy",
    "cex-v12/rust-release",
    "cex-v12/gateway-exact-reserve",
    "cex-v12/execution-settlement",
    "cex-v12/provider-reconciliation",
    "cex-v12/aggregate-qualification",
}


class VerificationFailure(RuntimeError):
    pass


def api(path: str, token: str) -> dict | list:
    request = urllib.request.Request(
        "https://api.github.com" + path,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "User-Agent": "trnm-world-cex-sequence-50-verifier/v1",
            "X-GitHub-Api-Version": "2022-11-28",
        },
    )
    try:
        with urllib.request.urlopen(request, timeout=120) as response:
            raw = response.read()
            return json.loads(raw) if raw else {}
    except urllib.error.HTTPError as error:
        detail = error.read().decode("utf-8", "replace")
        raise VerificationFailure(f"GitHub GET {path} failed {error.code}: {detail[:4000]}") from error


def download(path: str, token: str) -> bytes:
    request = urllib.request.Request(
        "https://api.github.com" + path,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "User-Agent": "trnm-world-cex-sequence-50-verifier/v1",
            "X-GitHub-Api-Version": "2022-11-28",
        },
    )
    try:
        with urllib.request.urlopen(request, timeout=180) as response:
            return response.read()
    except urllib.error.HTTPError as error:
        detail = error.read().decode("utf-8", "replace")
        raise VerificationFailure(f"artifact download failed {error.code}: {detail[:4000]}") from error


def safe_zip_inventory(payload: bytes) -> tuple[list[str], list[dict]]:
    names: list[str] = []
    documents: list[dict] = []
    with zipfile.ZipFile(io.BytesIO(payload)) as bundle:
        for info in bundle.infolist():
            normalized = info.filename.replace("\\", "/")
            member = PurePosixPath(normalized)
            if member.is_absolute() or ".." in member.parts:
                raise VerificationFailure(f"unsafe artifact member: {normalized}")
            if info.file_size > 256 * 1024 * 1024:
                raise VerificationFailure(f"oversized artifact member: {normalized}")
            names.append(normalized)
            if normalized.lower().endswith(".json") and info.file_size <= 16 * 1024 * 1024:
                try:
                    documents.append(json.loads(bundle.read(info).decode("utf-8")))
                except (UnicodeDecodeError, json.JSONDecodeError):
                    continue
    return names, documents


def text_contains_binding(document: dict) -> bool:
    text = json.dumps(document, sort_keys=True, separators=(",", ":"))
    return (
        COMMIT in text
        and TREE in text
        and MIGRATION in text
        and str(TRIGGER_SEQUENCE) in text
        and all(context in text for context in REQUIRED_CONTEXTS)
    )


def run_context(job: dict) -> str:
    name = str(job.get("name") or "")
    for context in REQUIRED_CONTEXTS:
        if name == context or name.endswith(context) or context.endswith(name):
            return context
    return name


def verify_execution(token: str) -> tuple[dict[str, dict], list[dict], list[dict]]:
    runs = api(f"/repos/{REPOSITORY}/actions/runs?head_sha={COMMIT}&per_page=100", token)
    workflow_runs = runs.get("workflow_runs", []) if isinstance(runs, dict) else []
    if not workflow_runs:
        raise VerificationFailure("CEX exact head has no workflow runs")

    contexts: dict[str, dict] = {}
    successful_runs: list[dict] = []
    all_jobs: list[dict] = []
    for run in workflow_runs:
        if run.get("head_sha") != COMMIT:
            continue
        jobs_payload = api(f"/repos/{REPOSITORY}/actions/runs/{run['id']}/jobs?filter=latest&per_page=100", token)
        jobs = jobs_payload.get("jobs", []) if isinstance(jobs_payload, dict) else []
        if not jobs:
            continue
        run_ok = run.get("status") == "completed" and run.get("conclusion") == "success"
        for job in jobs:
            all_jobs.append(job)
            context = run_context(job)
            if context not in REQUIRED_CONTEXTS:
                continue
            steps = job.get("steps") or []
            runner_id = job.get("runner_id") or 0
            runner_name = str(job.get("runner_name") or "")
            labels = job.get("labels") or []
            ok = (
                run_ok
                and job.get("status") == "completed"
                and job.get("conclusion") == "success"
                and runner_id != 0
                and bool(runner_name)
                and bool(steps)
                and all(step.get("status") == "completed" and step.get("conclusion") == "success" for step in steps)
            )
            candidate = {
                "context": context,
                "run_id": run["id"],
                "run_attempt": run.get("run_attempt"),
                "job_id": job.get("id"),
                "runner_id": runner_id,
                "runner_name": runner_name,
                "labels": labels,
                "step_count": len(steps),
                "success": ok,
            }
            existing = contexts.get(context)
            if existing is None or (candidate.get("run_attempt") or 0) > (existing.get("run_attempt") or 0):
                contexts[context] = candidate
        if run_ok:
            successful_runs.append(run)

    missing = sorted(REQUIRED_CONTEXTS - set(contexts))
    if missing:
        raise VerificationFailure(f"missing exact-head contexts: {missing}")
    failed = sorted(context for context, evidence in contexts.items() if not evidence["success"])
    if failed:
        raise VerificationFailure(f"contexts lack non-empty successful execution: {failed}")
    return contexts, successful_runs, all_jobs


def verify_artifacts(token: str, successful_runs: list[dict]) -> list[dict]:
    retained: list[dict] = []
    bound_manifest = False
    found_sbom = False
    found_provenance = False
    seen_ids: set[int] = set()
    for run in successful_runs:
        payload = api(f"/repos/{REPOSITORY}/actions/runs/{run['id']}/artifacts?per_page=100", token)
        for artifact in payload.get("artifacts", []) if isinstance(payload, dict) else []:
            artifact_id = int(artifact["id"])
            if artifact_id in seen_ids:
                continue
            seen_ids.add(artifact_id)
            if artifact.get("expired") is True or int(artifact.get("size_in_bytes") or 0) <= 0:
                continue
            digest_value = str(artifact.get("digest") or "")
            if not digest_value.startswith("sha256:") or len(digest_value) != 71:
                raise VerificationFailure(f"artifact {artifact_id} lacks SHA-256 digest")
            payload_bytes = download(f"/repos/{REPOSITORY}/actions/artifacts/{artifact_id}/zip", token)
            if not payload_bytes:
                raise VerificationFailure(f"artifact {artifact_id} is empty")
            names, documents = safe_zip_inventory(payload_bytes)
            lower_names = [name.lower() for name in names]
            found_sbom = found_sbom or any("sbom" in name for name in lower_names)
            found_provenance = found_provenance or any(
                marker in name for name in lower_names for marker in ("provenance", "attestation", "slsa")
            )
            if any(text_contains_binding(document) for document in documents):
                bound_manifest = True
            retained.append(
                {
                    "run_id": run["id"],
                    "artifact_id": artifact_id,
                    "name": artifact.get("name"),
                    "digest": digest_value,
                    "size_in_bytes": artifact.get("size_in_bytes"),
                    "members": names,
                }
            )
    if not retained:
        raise VerificationFailure("no retained exact-head artifact")
    if not bound_manifest:
        raise VerificationFailure("no artifact manifest binds the exact CEX identity and all contexts")
    if not found_sbom:
        raise VerificationFailure("no retained SBOM member")
    if not found_provenance:
        raise VerificationFailure("no retained provenance/attestation member")
    return retained


def verify_governance(token: str) -> dict:
    branch = api(f"/repos/{REPOSITORY}/branches/main", token)
    if branch.get("protected") is not True:
        raise VerificationFailure("CEX main is not protected")
    protection = api(f"/repos/{REPOSITORY}/branches/main/protection", token)
    checks = protection.get("required_status_checks") or {}
    observed = set(checks.get("contexts") or [])
    observed.update(item.get("context") for item in checks.get("checks") or [] if item.get("context"))
    if not REQUIRED_CONTEXTS.issubset(observed):
        raise VerificationFailure(f"CEX main lacks required contexts: {sorted(REQUIRED_CONTEXTS - observed)}")
    if checks.get("strict") is not True:
        raise VerificationFailure("CEX main checks are not strict")
    reviews = protection.get("required_pull_request_reviews") or {}
    if int(reviews.get("required_approving_review_count") or 0) < 1:
        raise VerificationFailure("CEX main does not require approval")
    if reviews.get("dismiss_stale_reviews") is not True:
        raise VerificationFailure("CEX main does not dismiss stale reviews")
    if reviews.get("require_last_push_approval") is not True:
        raise VerificationFailure("CEX main does not require last-push approval")
    if (protection.get("required_conversation_resolution") or {}).get("enabled") is not True:
        raise VerificationFailure("CEX main does not require conversation resolution")
    return {"branch": branch, "protection": protection}


def verify_review(token: str) -> dict:
    pull = api(f"/repos/{REPOSITORY}/pulls/{PULL_REQUEST}", token)
    if pull.get("head", {}).get("sha") != COMMIT or pull.get("head", {}).get("ref") != BRANCH:
        raise VerificationFailure("CEX PR exact head drifted")
    commit = api(f"/repos/{REPOSITORY}/git/commits/{COMMIT}", token)
    if commit.get("tree", {}).get("sha") != TREE:
        raise VerificationFailure("CEX commit tree drifted")
    source = api(f"/repos/{REPOSITORY}/commits/{COMMIT}", token)
    final_author = (source.get("author") or {}).get("login")
    final_committer = (source.get("committer") or {}).get("login")
    commit_time = ((source.get("commit") or {}).get("committer") or {}).get("date") or ""
    reviews = api(f"/repos/{REPOSITORY}/pulls/{PULL_REQUEST}/reviews?per_page=100", token)
    approvals = []
    for review in reviews if isinstance(reviews, list) else []:
        login = (review.get("user") or {}).get("login")
        if review.get("state") != "APPROVED" or not login:
            continue
        if login in {final_author, final_committer}:
            continue
        if str(review.get("submitted_at") or "") <= commit_time:
            continue
        approvals.append(review)
    if not approvals:
        raise VerificationFailure("no eligible independent exact-head approval")
    return {"pull": pull, "commit": commit, "approvals": approvals}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--result", type=Path, default=Path("cex-sequence-50-verification.json"))
    args = parser.parse_args()
    token = os.environ.get("TRNM_GITHUB_READ_TOKEN") or os.environ.get("GITHUB_TOKEN") or ""
    if not token:
        raise VerificationFailure("TRNM_GITHUB_READ_TOKEN or GITHUB_TOKEN is required")
    contexts, successful_runs, jobs = verify_execution(token)
    artifacts = verify_artifacts(token, successful_runs)
    governance = verify_governance(token)
    review = verify_review(token)
    result = {
        "schema": "trnm_world_cex_sequence_50_verification_v1",
        "repository": REPOSITORY,
        "pull_request": PULL_REQUEST,
        "commit": COMMIT,
        "tree": TREE,
        "branch": BRANCH,
        "migration": MIGRATION,
        "trigger_sequence": TRIGGER_SEQUENCE,
        "contexts": contexts,
        "artifact_count": len(artifacts),
        "artifacts": artifacts,
        "governance_verified": True,
        "independent_approval_verified": True,
        "world_adoption_eligible": True,
        "production_authorization": "not_granted_by_this_verifier",
    }
    args.result.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print("TRNM_WORLD_CEX_SEQUENCE_50_VERIFICATION=PASS")
    return 0


try:
    raise SystemExit(main())
except (VerificationFailure, OSError, ValueError, json.JSONDecodeError, zipfile.BadZipFile) as error:
    print(f"CEX sequence-50 verification failed closed: {error}", file=sys.stderr)
    raise SystemExit(1)
