---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-005
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Service Identity and Secret Contract v1

## Goals

- one audience and minimum privilege per identity;
- no shared root secret across roles;
- short-lived, revocable credentials where supported;
- signer private key never enters the game server, client, settlement worker, or repository;
- production uses workload identity/mTLS/KMS/HSM rather than static development tokens;
- rotation preserves durable idempotency and recovery.

## Roles

| Role | May do | Must not do |
| --- | --- | --- |
| native player | player-scoped game actions | operator/sign/settle/DB access |
| World compatibility authority | private match/campaign authority | export signer key, claim Nakama canonical completion |
| settlement worker | claim jobs, signer/CEX lookup/submit, apply receipt | moderate players, alter immutable identity, export seed |
| entitlement signer | validate envelope and sign exact entitlement | query/modify campaign, settle ledger, admit match |
| CEX game authority | submit/lookup game-owned intents | sign World completion, mutate World campaign directly |
| moderator | bounded moderation/appeal actions | sign entitlement, settle wallet, migration/admin DB |
| migration operator | apply checksum-bound migrations | ordinary player/settlement use |
| backup operator | encrypted backup/WAL/restore workflow | gameplay/moderation/signing |
| observability reader | read approved metrics/logs | raw secrets or business mutation |
| break-glass | narrowly scoped emergency operation | routine use or unrecorded bypass |

## Credential properties

Each credential records:

- issuer, subject, audience, scopes/roles;
- environment and service identity;
- issue/not-before/expiry;
- unique token/key ID;
- rotation generation;
- revocation source;
- storage owner and file/secret permissions;
- logging/redaction classification;
- emergency replacement owner.

Static development tokens use independent random values and mode-0600 storage.
They do not grant production evidence.

## Secret storage

Forbidden:

- repository commits, issue/PR bodies, workflow inputs/artifacts;
- command-line arguments visible to process listings;
- query strings;
- logs, panic payloads, replay files, screenshots;
- one environment variable reused for multiple role audiences;
- sibling-repository `.env` sourcing;
- personal home paths embedded in distributed units.

Production target:

- workload identity or mTLS for service authentication;
- KMS/HSM/non-exportable key for entitlement signer;
- secret manager injection into dedicated service user context;
- private runtime/config/state directories;
- audited access and automatic rotation.

## Signer custody

- only signer process can access private key handle/seed;
- key ID/public key/fingerprint/algorithm are attested to CEX registry;
- game server/worker receive only signed response and receipt hash;
- readiness proves possession of the exact active key without exporting it;
- rotation supports active/next/revoked states and exact duplicate recovery;
- old key remains verifiable for retained receipts under policy;
- compromise triggers revocation, incident, usage inventory, fraud review, and
  safe recovery of pending stable request identities.

## Rotation procedure

1. Open approved ticket and identify role/audience/environment.
2. Generate new credential/key in owning secure context.
3. Register next public identity and overlap window where required.
4. Deploy consumers that accept current+next without changing business IDs.
5. Activate new producer credential.
6. Verify readiness, lookup, idempotent retry, and negative cross-role tests.
7. Revoke old credential and prove rejection.
8. Remove old material after retention/rollback policy.
9. Record exact artifacts, timestamps, actors, and limitations.

Settlement `remote_request_id`, intent ID/hash, receipt identity, and operator
evidence never change because a transport credential rotates.

## Role-compromise negative tests

- moderator credential cannot sign or submit ledger intent;
- game-authority credential cannot call signer operator/custody endpoints;
- signer credential cannot mutate campaigns or moderation;
- settlement worker cannot apply without DB lease/CAS fences;
- backup/observability credentials are read-only;
- expired/revoked/wrong-audience tokens fail closed;
- development credentials are rejected in production profile;
- one role cannot derive another role secret.

## Break-glass

Break-glass requires:

- named incident and approving principals;
- short expiry and exact command/scope;
- no bypass of immutable identities or evidence append-only rules;
- full audit and post-incident review;
- immediate rotation/revocation afterward.

## Acceptance

- clean-host deployment uses dedicated service users/credentials;
- mTLS/workload-identity and KMS/HSM evidence is exact and independently reviewed;
- rotation/revocation and role-compromise matrices pass;
- no secret appears in repository, artifacts, logs, or status records;
- public/custody gates remain blocked until production evidence is attached.
