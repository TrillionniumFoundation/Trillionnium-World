---
status: current-candidate
owner: trillionnium-world
contract: trnm_settlement_receipt_recovery_v1
applies_to:
  - WORLD-P0-001
  - trnm-entitlement-signer
  - CEX economy settlement adapter
last_reviewed: 2026-08-28
review_due: 2026-09-11
---

# TRNM Settlement Receipt Recovery v1

## 1. Purpose

This contract closes the unsafe retry gap where a signer or CEX operation may
commit durably but its HTTP response is lost. A timeout, reset, gateway error,
or worker process failure is not evidence that the remote operation failed.

The mandatory strategy is **lookup before submit**:

```text
lookup exact immutable request
  -> found: validate and reuse the durable receipt
  -> absent: submit exactly once under the same identity
  -> lookup unavailable/ambiguous: retry later without submitting
```

The contract does not grant trusted-settlement or production credit by itself.
The CEX lookup endpoint remains an owner-repository dependency until its exact
implementation, tests, deployment, and evidence are reviewed.

## 2. Identity model

The following identities remain separate:

- `intent_id` — game-owned economic identity;
- `intent_hash` — SHA-256 of the exact authorized `EconomicIntent` JSON bytes;
- `remote_request_id` — stable signer request identity derived from match,
  campaign, and intent ID, independent of capture generation;
- `job_id` — capture-scoped local worker row;
- `receipt_id` and receipt hash — durable terminal remote result identity.

Changing a capture or lease must never change `intent_id`, `intent_hash`, or
`remote_request_id`.

## 3. Signer receipt lookup

### Request

```http
GET /v1/signer/receipts/{request_id}
x-trnm-signer-auth: <independent signer credential>
```

`request_id` is the exact durable `remote_request_id` stored by the settlement
outbox. The signer validates its bounded ASCII form before querying PostgreSQL.

### Responses

- `200 OK` — exact `EntitlementSignResponse` previously committed for the
  request ID;
- `404 Not Found` — no durable signer receipt exists and a sign request may be
  attempted;
- `401 Unauthorized` — caller is not the signer-service principal;
- `400 Bad Request` — malformed request identity;
- `5xx` or transport failure — ambiguous lookup; the caller must retry later and
  must not start a new sign request in the same attempt.

The returned response binds:

- contract version;
- request ID;
- signed payload hash;
- signing receipt hash;
- signer key ID and issuer;
- signature.

The World client reconstructs the unsigned entitlement from immutable capture
material, injects the returned key/signature, validates the entitlement shape,
recomputes the signed payload SHA-256, and recomputes the signing receipt hash.
A mismatch is permanent and fails closed.

### Submit fallback

Only an exact `404` permits:

```http
POST /v1/signer/sign
x-trnm-signer-auth: <independent signer credential>
Content-Type: application/json
```

The entitlement nonce equals the durable signer `request_id`; the economic
`intent_id` remains the game-owned intent and is not aliased to the transport
request identity. Replaying one request ID with different entitlement material
returns a conflict.

## 4. CEX settlement receipt lookup

### Candidate request

```http
GET /v1/trnm/economy/receipts/by-intent?intent_id=<intent_id>
x-trnm-game-authority: <game-authority credential>
x-trnm-intent-sha256: <64 lowercase hex>
```

The endpoint is owned by CEX. World supplies the exact authorized intent ID and
SHA-256. The hash is computed over the exact JSON bytes submitted to the CEX
intent endpoint.

### Candidate response

```json
{
  "contract_version": "trnm_cex_settlement_receipt_lookup_v1",
  "intent_id": "...",
  "intent_hash": "...",
  "receipt": {
    "protocol_version": "term_exchange_protocol_v2",
    "receipt_id": "...",
    "intent_id": "...",
    "term_id": "..."
  }
}
```

Required semantics:

- `200 OK` only when the durable receipt binds the exact intent ID and hash;
- `404 Not Found` only when no durable intent/receipt exists;
- `409 Conflict` when the intent ID exists under different immutable bytes;
- authentication and authorization failures are permanent;
- `5xx` or transport failure is an ambiguous lookup and does not permit a POST.

World validates the wrapper contract, intent ID, intent hash, and complete
`EconomicReceipt::validate_for` binding before accepting it.

### Submit fallback

Only an exact `404` permits:

```http
POST /v1/trnm/economy/intents
x-trnm-game-authority: <game-authority credential>
x-trnm-intent-sha256: <same exact hash>
Content-Type: application/json
```

The CEX implementation must durably commit the intent identity, hash, and
receipt before returning success. A retry always uses the same intent ID, hash,
and payload.

## 5. Recovery sequence

For a positive reward:

1. claim the durable World job under a live owner/generation lease;
2. query signer receipt by `remote_request_id`;
3. if absent, submit the exact sign request;
4. persist the validated authorization under the same live lease;
5. query CEX receipt by `intent_id + intent_hash`;
6. if absent, submit the exact authorized intent;
7. persist the validated CEX receipt under the same live lease;
8. apply the receipt to campaign progression in a separate exact transaction.

A worker restart begins again at step 2 or step 5. It never assumes that the
last remote operation failed merely because the local response was absent.

## 6. Fail-closed rules

The following conditions are permanent failures or operator-visible conflicts:

- one request ID maps to different signer payload bytes;
- one intent ID maps to a different intent hash;
- returned signer request/payload/receipt hashes do not recompute exactly;
- returned CEX wrapper ID/hash differs from the durable intent;
- returned receipt fails `validate_for`;
- credentials or authority audience are invalid;
- lookup returns malformed or unknown contract data.

A remote lookup timeout remains retryable. It must not be converted to “not
found.”

## 7. Evidence requirements

Promotion requires all of:

- exact-commit World unit tests proving lookup-before-submit;
- signer response-loss fixture with one durable sign and one recovered receipt;
- CEX response-loss fixture with one durable intent commit and one recovered
  receipt;
- mismatched ID/hash negative fixtures;
- PostgreSQL worker lease and account-serialization tests;
- an owner-repository CEX implementation and exact component lock;
- deployed black-box response-loss tests against the exact signer/CEX artifacts;
- reviewer signoff and bounded limitations.

Until the CEX owner endpoint and deployed matrix pass, this contract is a
candidate integration boundary, not trusted-settlement evidence.
