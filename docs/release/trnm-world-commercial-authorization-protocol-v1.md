---
status: current-candidate
owner: trillionnium-world-release
applies_to_gap: WORLD-V5-020
last_reviewed: 2026-09-09
review_due: 2026-10-08
production_authorization: not_granted
implementation_conformance: not-implied
---

# Trillionnium World commercial authorization protocol v1

## Purpose and non-authority

This protocol defines the minimum independently reviewable evidence required before any Trillionnium World commercial, public-online or public-player-market claim may be authorized. It is an evidence contract, not an authorization. Its presence, a completed template, a source change, a CI result, a local fixture, an internal approval comment or a component pin cannot grant launch, sale, custody, public routing or market authority.

The default and current result is fail closed:

```text
commercial_authorization=not_granted
public_online=false
public_player_market=false
trusted_settlement=false
production_authorization=not_granted
```

## Exact release tuple

A commercial decision must bind one immutable release tuple containing at least:

- accepted World, Nakama, CEX, Chain and Integration commit and tree identities;
- actual prospective-merge or accepted mainline identities as applicable;
- immutable binary, package, container, SBOM and provenance digests;
- protocol, database, rules, content, map, asset and localization digests;
- deployment topology, regions, public endpoints and accountable environment identities;
- the exact evidence-record schema and strict validator version;
- the decision time, effective window, expiry and rollback selector.

Any movement of a bound source, artifact, protocol, deployment, policy or accountable reviewer invalidates the decision until a new complete record is accepted.

## Required accountable decisions

No single person or automated system may satisfy all commercial rows. The evidence bundle requires explicit, independently attributable decisions from the accountable owners for:

1. product scope and player-facing promises;
2. security, incident response and public-edge abuse controls;
3. privacy, data protection, retention and deletion;
4. source, asset, map, audio, font and third-party licensing;
5. terms of service, age policy, jurisdiction and regulatory posture;
6. payments, refunds, taxation, pricing and finance controls where value is involved;
7. moderation, support, appeal and service-continuity operations;
8. accessibility and consented human validation;
9. custody/KMS/HSM controls when trusted settlement or wallet value is in scope;
10. final release management and executive go/no-go.

Author identity, repository administrator identity, code ownership and organizational authority are distinct. Self-approval or an approval outside the reviewer’s accountable domain receives no credit.

## Product and market scope

The accepted record must enumerate every externally promised capability and explicitly state whether each is enabled, disabled, region-limited, invitation-only, experimental or deferred. At minimum it must cover:

- offline/local gameplay;
- Nakama-canonical online gameplay;
- compatibility-laboratory routes;
- wallet-backed rewards or settlement;
- player-to-player transfer, exchange or market behavior;
- downloadable content, updates and rollback;
- map/geodata and user-generated content;
- telemetry, accounts, moderation and support.

A capability omitted from the record is disabled. Fixture, local-alpha, compatibility and diagnostic surfaces may not be relabelled as production.

## Privacy and data protection evidence

The privacy decision must bind a complete data inventory and flow map covering collection, purpose, legal basis, consent where required, processors, cross-border transfer, encryption, access, retention, deletion, export, incident notification and data-subject request handling. It must include account, session, gameplay, telemetry, moderation, support, payment and geolocation/map data as applicable.

The record must identify supported jurisdictions and any excluded regions. A generic privacy policy URL, an unsigned checklist or a future implementation plan is insufficient.

## Licensing and provenance evidence

The licensing decision must bind the exact release inventory for source, dependencies, assets, maps/geodata, text, translations, audio, fonts, generated material and vendored code. It must retain license texts, notices, attribution, source/derivative lineage, usage restrictions and takedown contacts.

Repository visibility does not imply redistribution, modification or commercial rights. Missing or ambiguous provenance blocks authorization.

## Payments, value and consumer obligations

When payments, wallet-backed rewards, transferable assets or a player market are in scope, the record must bind:

- the CEX/custody decision and exact settlement contract;
- pricing, currency, refund, chargeback and dispute behavior;
- tax and accounting ownership;
- age and jurisdiction restrictions;
- sanctions/fraud/abuse controls as applicable;
- operator reconciliation, quarantine and rollback procedures;
- player-visible distinction between local soft value, pending settlement and verified wallet value.

A World source event, local receipt, screenshot or successful remote HTTP response cannot establish ledger success or custody authorization.

## Moderation, support and operational readiness

The bundle must identify staffed ownership, service hours and escalation paths for abuse, cheating, account recovery, payment disputes, privacy requests, accessibility reports, safety incidents and outage communication. It must bind current runbooks, alert routes, incident severity, retention rules and rollback/disablement authority.

A mailbox, placeholder issue, unstaffed rota or draft runbook is not operational evidence.

## Required evidence record

The final record must conform to the strict evidence schema accepted by `scripts/check-trnm-world-external-evidence.py` and include:

- `gap_id=WORLD-V5-020`;
- immutable release tuple and artifact hashes;
- evidence class `commercial_legal`;
- accountable approver identities and conflict declarations;
- signed or otherwise independently verifiable decision artifacts;
- limitations, jurisdictions, capability matrix and known exclusions;
- effective time, expiry and revocation conditions;
- rollback/disablement selector and incident contact;
- verification result from a reviewer who did not author the candidate or the evidence parser.

The repository template is intentionally rejected as evidence until all required fields contain real, externally verifiable values.

## Acceptance rule

`WORLD-V5-020` may move to `closed` only when all of the following are true:

1. every applicable upstream technical, governance, deployment, custody and human denominator is already independently closed on the same release tuple;
2. all accountable commercial/legal decisions are present and non-conflicting;
3. the strict validator accepts the immutable record and rejects altered, expired, crossed-tuple and placeholder variants;
4. an independent reviewer verifies the referenced source artifacts rather than only the record text;
5. the final human go/no-go explicitly grants the bounded commercial scope.

Technical readiness alone does not imply commercial authorization. Commercial authorization cannot repair a failed technical or security gate.

## Invalidation and revocation

Authorization expires or is revoked on any source/artifact/protocol/deployment movement, approver withdrawal, newly discovered license or privacy defect, material incident, unsupported jurisdiction change, custody-key event, expired evidence, untested rollback change or capability expansion outside the accepted scope.

Revocation must disable affected public routes, markets and value operations without requiring a new client release where feasible. The prior evidence remains provenance only and cannot be copied to a successor tuple.

## No-credit rules

```text
source exists                 != commercial authorization
document exists               != accountable approval
CI success                    != legal approval
repository administrator      != independent reviewer
public repository             != commercial license
local value                   != wallet value
payment response              != reconciled ledger effect
review requested              != approval
approval on old head          != approval on current head
component pin                 != component qualification
commercial approval           != production authorization
```

Until a complete immutable record is accepted and independently verified, this protocol preserves `commercial_authorization=not_granted` and all public/commercial capabilities remain disabled.
