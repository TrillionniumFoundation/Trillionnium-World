---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-002
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World HTTP API Contract v1

## Scope

This document governs HTTP surfaces owned by the World repository. It separates:

1. deterministic World transition APIs;
2. World-local compatibility-enclave APIs;
3. internal settlement worker/signer integration;
4. target Nakama canonical APIs, which are not owned here.

A route existing in `trnm-game-server` does not make World the destination
online authority.

## API classes

| Class | Owner | Exposure | Authority |
| --- | --- | --- | --- |
| deterministic transition | World | internal service | game-domain transition only |
| player compatibility | World enclave | laboratory/private | legacy local alpha only |
| operator compatibility | World enclave | private | bounded moderation/operations |
| settlement worker | World + CEX contract | private service | outbox capture/execute/apply |
| signer | isolated signer service | loopback/private | entitlement signing only |
| canonical match | Nakama | target online | admission/order/recovery/completion |

## Mandatory HTTP envelope

Every versioned JSON endpoint must publish:

- method and path;
- request/response schema ID;
- protocol version independent from build ID;
- authentication audience and scope;
- maximum header/body/response bytes;
- timeout and retry policy;
- idempotency identity;
- stable error code;
- logging redaction rules;
- ownership and release profile.

Unknown JSON fields fail closed unless a specific schema version explicitly
permits them.

## Authentication

- Player endpoints use a player session issued/verified by the owning identity service.
- Game authority, settlement worker, moderator, and signer credentials have different audiences and cannot substitute for one another.
- Bearer material is never accepted in query strings, logs, replay files, or diagnostic payloads.
- Public deployment requires TLS and service-to-service mTLS/workload identity; loopback HTTP is development/single-host evidence only.

## Idempotency

| Operation | Durable identity |
| --- | --- |
| World transition | `request_hash` plus caller transition ID |
| online compatibility command | member input sequence + command ID + fingerprint |
| signer request | stable `remote_request_id` |
| CEX intent | immutable `intent_id` + intent SHA-256 |
| operator replay | replay request ID + exact job/capture/intent identities |

A transport retry reuses the same identity. Reusing one identity with different
material is a conflict and fails closed.

## Error envelope

New or migrated endpoints use:

```json
{
  "contract_version": "trnm_world_http_error_v1",
  "error_code": "stable_machine_code",
  "reason_code": "bounded_reason",
  "retryable": false,
  "request_id": "correlation-id",
  "detail": "bounded, redacted diagnostic"
}
```

HTTP status is transport classification; callers branch on stable machine code.
A timeout, malformed 2xx body, or 409 after a possible remote commit is not
proof of failure. Settlement callers perform exact lookup before resubmission.

## Resource ceilings

Default ceilings unless a stricter route contract applies:

| Resource | Ceiling |
| --- | ---: |
| request headers | 32 KiB |
| ordinary JSON request | 256 KiB |
| transition state | 2 MiB |
| transition command | 128 KiB |
| replay response page | 2 MiB |
| error body retained | 64 KiB |
| identifier | 160 bytes |
| diagnostic detail | 256 bytes |

## Compatibility enclave rules

Compatibility routes must:

- carry explicit `world_legacy_local_alpha` status;
- reject non-loopback/public binds unless a separately approved deployment profile exists;
- never construct canonical `MatchCompletedV1`;
- never load a Nakama authority key;
- never be advertised as public, multi-host, or Chain-finalized;
- publish retirement owner, usage inventory, drain plan, and disable switch.

## OpenAPI generation

The target artifact is `docs/protocol/openapi/trnm-world-http-v1.openapi.json`.
It must be generated or checked against route fixtures and must include every
current World-owned endpoint. Until that artifact and conformance tests land,
`WORLD-P1-002` remains open.

## Acceptance

- implementation examples validate against the published schema;
- auth audience and idempotency are explicit per route;
- unknown/crossed protocol versions return stable errors;
- body limits are enforced before allocation amplification;
- logs contain no token, seed, private key, full credential, or unnecessary personal data;
- exact-head HTTP conformance runs in CI;
- Nakama-owned canonical APIs are not duplicated in World.
