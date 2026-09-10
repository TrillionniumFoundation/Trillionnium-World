#!/usr/bin/env python3
"""Fail-closed GitHub PR governance read-back for one exact live head."""

from __future__ import annotations

import argparse
import json
import os
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from typing import Any

API = "https://api.github.com"
GRAPHQL = "https://api.github.com/graphql"
SUCCESSFUL_CONCLUSIONS = {"success"}


class GovernanceError(RuntimeError):
    pass


def parser() -> argparse.ArgumentParser:
    value = argparse.ArgumentParser()
    value.add_argument("--repository", required=True)
    value.add_argument("--pull-request", required=True, type=int)
    value.add_argument("--expected-head", required=True)
    value.add_argument("--expected-base", default="main")
    value.add_argument("--author", required=True)
    value.add_argument("--eligible-reviewer", action="append", default=[])
    value.add_argument("--required-reviewer", action="append", default=[])
    value.add_argument("--request-missing", action="store_true")
    return value


def token() -> str:
    value = os.environ.get("GITHUB_TOKEN", "")
    if not value:
        raise GovernanceError("GITHUB_TOKEN is absent")
    return value


def headers() -> dict[str, str]:
    return {
        "Accept": "application/vnd.github+json",
        "Authorization": f"Bearer {token()}",
        "Content-Type": "application/json",
        "X-GitHub-Api-Version": "2022-11-28",
    }


def request(method: str, url: str, payload: Any | None = None) -> Any:
    data = None
    if payload is not None:
        data = json.dumps(payload, separators=(",", ":")).encode("utf-8")
    operation = urllib.request.Request(url, data=data, method=method, headers=headers())
    try:
        with urllib.request.urlopen(operation, timeout=60) as response:
            raw = response.read()
    except urllib.error.HTTPError as error:
        body = error.read().decode("utf-8", errors="replace")[:1000]
        raise GovernanceError(
            f"GitHub API {method} {url} failed: HTTP {error.code}: {body}"
        ) from error
    if not raw:
        return None
    return json.loads(raw)


def rest(repository: str, suffix: str, method: str = "GET", payload: Any | None = None) -> Any:
    return request(method, f"{API}/repos/{repository}{suffix}", payload)


def paged(repository: str, suffix: str, key: str | None = None) -> list[Any]:
    result: list[Any] = []
    for page in range(1, 101):
        separator = "&" if "?" in suffix else "?"
        value = rest(repository, f"{suffix}{separator}per_page=100&page={page}")
        items = value.get(key, []) if key is not None else value
        if not isinstance(items, list):
            raise GovernanceError(f"paginated response is not a list: {suffix}")
        result.extend(items)
        if len(items) < 100:
            return result
    raise GovernanceError(f"pagination exceeded 100 pages: {suffix}")


def graphql(query: str, variables: dict[str, Any]) -> dict[str, Any]:
    value = request("POST", GRAPHQL, {"query": query, "variables": variables})
    if value.get("errors"):
        raise GovernanceError(f"GitHub GraphQL returned errors: {value['errors']}")
    data = value.get("data")
    if not isinstance(data, dict):
        raise GovernanceError("GitHub GraphQL data is absent")
    return data


def unresolved_threads(owner: str, name: str, number: int) -> list[str]:
    query = """
      query($owner: String!, $name: String!, $number: Int!, $after: String) {
        repository(owner: $owner, name: $name) {
          pullRequest(number: $number) {
            reviewThreads(first: 100, after: $after) {
              nodes {
                id
                isResolved
                isOutdated
                comments(first: 1) { nodes { url body author { login } } }
              }
              pageInfo { hasNextPage endCursor }
            }
          }
        }
      }
    """
    after: str | None = None
    result: list[str] = []
    while True:
        data = graphql(query, {
            "owner": owner,
            "name": name,
            "number": number,
            "after": after,
        })
        repository = data.get("repository") or {}
        pull = repository.get("pullRequest") or {}
        threads = pull.get("reviewThreads") or {}
        nodes = threads.get("nodes") or []
        for node in nodes:
            if node.get("isResolved") or node.get("isOutdated"):
                continue
            comments = ((node.get("comments") or {}).get("nodes") or [])
            first = comments[0] if comments else {}
            author = (first.get("author") or {}).get("login", "unknown")
            url = first.get("url", node.get("id", "unknown"))
            body = str(first.get("body", "")).replace("\n", " ")[:160]
            result.append(f"{author}: {url}: {body}")
        page = threads.get("pageInfo") or {}
        if not page.get("hasNextPage"):
            return result
        after = page.get("endCursor")
        if not after:
            raise GovernanceError("review thread pagination cursor is absent")


def protection_problems(repository: str, branch: str) -> tuple[list[str], set[str], dict[str, Any]]:
    value = rest(repository, f"/branches/{urllib.parse.quote(branch, safe='')}/protection")
    problems: list[str] = []
    checks: set[str] = set()

    status = value.get("required_status_checks")
    if not isinstance(status, dict):
        problems.append("required status checks are absent")
    else:
        if status.get("strict") is not True:
            problems.append("required status checks are not strict")
        for context in status.get("contexts") or []:
            if isinstance(context, str) and context:
                checks.add(context)
        for item in status.get("checks") or []:
            context = item.get("context") if isinstance(item, dict) else None
            if isinstance(context, str) and context:
                checks.add(context)
        if not checks:
            problems.append("required status-check context set is empty")

    reviews = value.get("required_pull_request_reviews")
    if not isinstance(reviews, dict):
        problems.append("required pull-request reviews are absent")
    else:
        if reviews.get("dismiss_stale_reviews") is not True:
            problems.append("stale review dismissal is disabled")
        if reviews.get("require_code_owner_reviews") is not True:
            problems.append("CODEOWNER review is not required")
        count = reviews.get("required_approving_review_count")
        if not isinstance(count, int) or count < 1:
            problems.append("required approving review count is below one")
        if reviews.get("require_last_push_approval") is not True:
            problems.append("latest-push approval is disabled")

    if (value.get("enforce_admins") or {}).get("enabled") is not True:
        problems.append("administrator enforcement is disabled")
    if (value.get("required_conversation_resolution") or {}).get("enabled") is not True:
        problems.append("conversation resolution is not required")
    if (value.get("allow_force_pushes") or {}).get("enabled") is True:
        problems.append("force pushes are allowed")
    if (value.get("allow_deletions") or {}).get("enabled") is True:
        problems.append("protected branch deletion is allowed")

    summary = {
        "strict_status_checks": isinstance(status, dict) and status.get("strict") is True,
        "required_contexts": sorted(checks),
        "dismiss_stale_reviews": isinstance(reviews, dict)
        and reviews.get("dismiss_stale_reviews") is True,
        "require_code_owner_reviews": isinstance(reviews, dict)
        and reviews.get("require_code_owner_reviews") is True,
        "require_last_push_approval": isinstance(reviews, dict)
        and reviews.get("require_last_push_approval") is True,
        "required_approving_review_count": reviews.get("required_approving_review_count")
        if isinstance(reviews, dict)
        else None,
        "enforce_admins": (value.get("enforce_admins") or {}).get("enabled") is True,
        "conversation_resolution": (value.get("required_conversation_resolution") or {}).get("enabled") is True,
        "force_pushes_allowed": (value.get("allow_force_pushes") or {}).get("enabled") is True,
        "deletions_allowed": (value.get("allow_deletions") or {}).get("enabled") is True,
    }
    return problems, checks, summary


def required_context_problems(repository: str, head: str, required: set[str]) -> tuple[list[str], dict[str, str]]:
    runs = paged(repository, f"/commits/{head}/check-runs?filter=latest", "check_runs")
    statuses = paged(repository, f"/commits/{head}/statuses")
    observed: dict[str, str] = {}
    for item in statuses:
        context = item.get("context")
        state = item.get("state")
        if isinstance(context, str) and isinstance(state, str) and context not in observed:
            observed[context] = state
    for item in runs:
        name = item.get("name")
        status = item.get("status")
        conclusion = item.get("conclusion")
        if not isinstance(name, str):
            continue
        state = conclusion if status == "completed" else status
        if isinstance(state, str):
            observed[name] = state
    problems = [
        f"required context {context!r} is {observed.get(context, 'missing')}"
        for context in sorted(required)
        if observed.get(context) not in SUCCESSFUL_CONCLUSIONS and observed.get(context) != "success"
    ]
    return problems, {context: observed.get(context, "missing") for context in sorted(required)}


def main() -> int:
    args = parser().parse_args()
    problems: list[str] = []
    owner, name = args.repository.split("/", 1)
    pull = rest(args.repository, f"/pulls/{args.pull_request}")
    actual_head = ((pull.get("head") or {}).get("sha"))
    actual_base = ((pull.get("base") or {}).get("ref"))
    author = ((pull.get("user") or {}).get("login"))
    if pull.get("state") != "open" or pull.get("merged") is True:
        problems.append("pull request is not open and unmerged")
    if pull.get("draft") is not True:
        problems.append("pull request must remain Draft during qualification")
    if actual_head != args.expected_head:
        problems.append(f"pull-request head drift: expected {args.expected_head}, observed {actual_head}")
    if actual_base != args.expected_base:
        problems.append(f"pull-request base drift: expected {args.expected_base}, observed {actual_base}")
    if author != args.author:
        problems.append(f"pull-request author drift: expected {args.author}, observed {author}")

    reviews = paged(args.repository, f"/pulls/{args.pull_request}/reviews")
    exact_approvals: dict[str, int] = {}
    for review in reviews:
        login = ((review.get("user") or {}).get("login"))
        if not isinstance(login, str) or login == args.author:
            continue
        if review.get("state") == "APPROVED" and review.get("commit_id") == args.expected_head:
            exact_approvals[login] = int(review.get("id", 0))
    eligible = set(args.eligible_reviewer)
    required_reviewers = set(args.required_reviewer)
    if eligible and not (set(exact_approvals) & eligible):
        problems.append(
            "no eligible non-author exact-head approval; eligible=" + ",".join(sorted(eligible))
        )
    missing_required = sorted(required_reviewers - set(exact_approvals))
    if missing_required:
        problems.append("required exact-head approvers missing: " + ",".join(missing_required))

    requested = {
        item.get("login")
        for item in pull.get("requested_reviewers") or []
        if isinstance(item, dict)
    }
    candidates = (eligible | required_reviewers) - set(exact_approvals) - requested
    if args.request_missing and candidates:
        rest(
            args.repository,
            f"/pulls/{args.pull_request}/requested_reviewers",
            method="POST",
            payload={"reviewers": sorted(candidates)},
        )
        requested.update(candidates)

    threads = unresolved_threads(owner, name, args.pull_request)
    if threads:
        problems.append(f"unresolved review threads: {len(threads)}")

    protection, required_contexts, protection_summary = protection_problems(
        args.repository, args.expected_base
    )
    problems.extend(protection)
    context_problems, context_states = required_context_problems(
        args.repository, args.expected_head, required_contexts
    )
    problems.extend(context_problems)

    output = {
        "schema": "github.pr-governance-readback.v1",
        "repository": args.repository,
        "pull_request": args.pull_request,
        "state": pull.get("state"),
        "draft": pull.get("draft"),
        "merged": pull.get("merged"),
        "base": actual_base,
        "head": actual_head,
        "expected_head": args.expected_head,
        "author": author,
        "eligible_reviewers": sorted(eligible),
        "required_reviewers": sorted(required_reviewers),
        "exact_head_non_author_approvals": sorted(exact_approvals),
        "requested_reviewers": sorted(value for value in requested if isinstance(value, str)),
        "unresolved_review_threads": threads,
        "branch_protection": protection_summary,
        "required_context_states": context_states,
        "status": "ok" if not problems else "blocked",
        "problems": problems,
        "production_authorization": "not_granted",
    }
    print(json.dumps(output, indent=2, sort_keys=True))
    return 0 if not problems else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (GovernanceError, ValueError, KeyError) as error:
        print(json.dumps({
            "schema": "github.pr-governance-readback.v1",
            "status": "failed_closed",
            "problems": [str(error)],
            "production_authorization": "not_granted",
        }, indent=2, sort_keys=True))
        raise SystemExit(1)
