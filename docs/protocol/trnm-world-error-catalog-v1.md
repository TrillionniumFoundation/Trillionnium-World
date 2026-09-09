---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-002
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Stable Error Catalogue v1

## Rules

- Machine behavior branches on `error_code`, never free-form detail.
- `reason_code` narrows a stable class but does not carry secrets.
- Retryability is explicit; HTTP status alone is insufficient.
- Detail is UTF-8, control-free, redacted, and byte-bounded.
- Adding a code is backward-compatible only when unknown-code behavior is defined.
- Changing meaning or retryability requires a new protocol/contract revision.

## Common classes

| Code | Default retry | Meaning |
| --- | --- | --- |
| `invalid_request` | no | malformed envelope or identifier |
| `unsupported_protocol` | no | contract/protocol not supported |
| `unknown_ruleset_revision` | no | World ruleset unavailable |
| `unknown_content_revision` | no | content revision unavailable |
| `payload_hash_mismatch` | no | bytes do not match declared digest |
| `invalid_canonical_payload` | no | strict canonical profile rejected bytes |
| `forbidden_authority_surface` | no | payload crosses ownership boundary |
| `resource_budget_exceeded` | no | published byte/count/time budget exceeded |
| `unauthenticated` | after reauth | no valid credential |
| `unauthorized` | no | credential lacks exact resource/scope |
| `conflict` | lookup first | durable identity reused or possible duplicate commit |
| `stale_revision` | after refresh | expected revision/hash no longer current |
| `stale_generation` | after reroute | process/worker generation lost its fence |
| `sequence_gap` | after resync | input or stream continuity missing |
| `lease_expired` | re-claim only | worker lease is no longer live |
| `ambiguous_remote_outcome` | lookup first | request may have committed remotely |
| `dead_letter` | operator only | retry budget exhausted or permanent rejection |
| `quarantined` | after review/retry time | poison scope isolated from normal work |
| `server_draining` | bounded retry | admission stopped for shutdown/cutover |
| `internal_unavailable` | yes | transient internal dependency unavailable |

## Settlement-specific reasons

```text
capture_not_committed
capture_fence_changed
terminal_identity_changed
campaign_revision_changed
campaign_state_hash_changed
serialization_key_busy
lease_lost
signer_lookup_unavailable
signer_response_malformed
cex_lookup_unavailable
cex_response_malformed
receipt_binding_mismatch
campaign_apply_failed
quarantine_active
operator_replay_not_authorized
```

A malformed 2xx signer/CEX response is `ambiguous_remote_outcome`, not permanent
failure. A 409 after a stable request identity triggers exact receipt lookup.

## Deterministic transition codes

The exact `trnm_world_transition_v1` catalogue remains:

```text
invalid_contract_version
invalid_request
unknown_ruleset_revision
unknown_content_revision
payload_hash_mismatch
invalid_canonical_payload
forbidden_authority_surface
resource_budget_exceeded
invalid_command
domain_rejected
nondeterministic_output
internal_unavailable
```

## Unknown code behavior

Clients:

1. preserve the raw code for diagnostics;
2. treat an unknown code as nonretryable unless the envelope separately and safely marks retryability;
3. do not infer success;
4. surface a safe generic message;
5. record protocol/build identities for compatibility triage.

## Observability

Metrics use bounded labels. Raw IDs, player names, tokens, request bodies, and
error detail are not metric labels. Logs may include hashed/scoped identifiers
and exact machine codes under the data-classification policy.

## Acceptance

- implementation enums/fixtures round-trip to the catalogue;
- retry loops are tested per code;
- unknown/crossed versions fail safely;
- no code aliases unrelated protocol concepts;
- error detail cannot leak secrets or exceed its byte budget.
