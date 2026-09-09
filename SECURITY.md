# Security Policy

## Supported scope

Security review follows the exact current candidate and release identity. The
current repository posture is technical alpha; public online and the public
player market are not enabled.

Supported reports include vulnerabilities in:

- native client and save/replay handling;
- deterministic simulation and protocol parsing;
- compatibility game server and WebSocket/HTTP surfaces;
- PostgreSQL migrations, fencing, journals, settlement outbox, and operator controls;
- packaging, installers, CI, dependencies, signatures, and provenance;
- credential separation and signer/CEX integration.

Historical material under archive/legacy/excluded platform paths is not an
active production surface, but boundary bypasses involving it are relevant.

## Reporting

Do not open a public issue containing exploitable details, live credentials,
private keys, tokens, personal data, or unredacted production artifacts.

Use the repository's private security-advisory channel when enabled, or contact
the Foundation security owner through a verified private organization channel.
Include:

- affected repository, commit/tree, build and deployment profile;
- vulnerability class and trust boundary;
- minimal reproducible steps or proof;
- impact and preconditions;
- redacted logs/artifact hashes;
- proposed mitigation where available;
- disclosure coordination needs.

If no private channel is available, open a minimal public issue stating only
that a private security contact is required. Do not include the exploit.

## Response

The security owner will:

1. acknowledge and assign a private tracking ID;
2. reproduce against an exact identity;
3. classify affected authority, value, privacy, availability, and supply-chain scope;
4. contain exposure and rotate/revoke credentials where needed;
5. develop reviewed positive/negative/fault tests;
6. release signed fixes and migration/rollback guidance;
7. coordinate disclosure after affected users/operators can update;
8. record limitations and unresolved external dependencies.

No source-only fix is called deployed until exact production evidence exists.

## Safe harbor

Good-faith research that avoids privacy harm, service disruption, value theft,
persistence, credential retention, and unnecessary data access will be evaluated
under the Foundation's current disclosure policy. Do not test against public or
third-party systems without explicit authorization.

## Secret incidents

A suspected secret leak triggers immediate revocation/rotation, usage inventory,
log/artifact redaction review, settlement ambiguity reconciliation, and incident
tracking. Never commit a replacement secret. Rotation must preserve durable
idempotency identities and receipt lineage.

## No public-market authorization

Nothing in this policy authorizes testing with real player funds, custody,
listings, or public market activity. Those surfaces remain disabled until their
separate security, legal, fraud, dispute, and commercial gates pass.
