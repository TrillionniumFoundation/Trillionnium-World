# Security Policy

## Reporting a vulnerability

Do not disclose a suspected vulnerability in a public issue, discussion, pull request, or chat log.

Use the repository's private **Report a vulnerability** / Security Advisory flow when available. Otherwise contact a repository administrator through an existing private organizational channel.

Include the affected commit/component/version, impact and prerequisites, minimal reproduction, redacted logs or artifacts, whether exploitation may already have occurred, and a safe follow-up channel. Never attach real private keys, recovery credentials, session tokens, database URLs, production customer data, or unredacted crash dumps.

## Security-sensitive surfaces

Explicit threat-model and negative-test review is required for wallet/signer/entitlement/custody/settlement; authentication/authorization/moderation/admin actions; online authority/fencing/journals/replay/database migrations; bridge finality and replay protection; oracle freshness and poisoning controls; governance/pause/upgrade/rollback; and package provenance/release signing.

## Supported posture

Only the current default branch receives routine security fixes. Historical documents and archived evidence are not supported implementations. The repository is currently alpha/pre-alpha and must not be represented as public-production hardened.

## Handling rules

- Fail closed at trust, integrity, version, or authority boundaries.
- Preserve idempotency and replay protection.
- Bind evidence and fixes to exact commits and artifact digests.
- Add regression tests for the exploit and nearby bypasses.
- Document upgrade, rollback, and partial-failure behavior.
- Rotate any credential that may have entered logs, commits, artifacts, or fixtures.
- Do not weaken a security gate merely to make CI green.

Coordinate public disclosure only after affected branches and artifacts are remediated and users/operators have a safe upgrade path.
