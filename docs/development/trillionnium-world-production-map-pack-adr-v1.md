# Trillionnium World Production Map Pack ADR v1

## Status

Accepted as the production route. This ADR does not approve live public map ingestion by itself.

## Decision

Trillionnium World uses signed map packs as the deployable map artifact. Runtime clients consume a verified manifest and attribution payload; they do not fetch live public OSM data directly during gameplay.

## Production Rules

- Every map pack has a canonical manifest, SHA-256 checksums, an Ed25519 signature, a `key_id`, and a generated timestamp.
- Key rotation is mandatory: keep active and next public keys in the release bundle; reject revoked keys.
- Attribution evidence is mandatory across Web, Native/Bevy, and Matrix/read-only surfaces before public map-pack promotion.
- Sensitive POI filtering and geofence/downstream takedown policy must run before a public bundle is accepted.
- Rollback must be manifest-based: restore the previous signed manifest and revoke the bad key or bad package id.
- Live Overpass/Geofabrik ingestion remains blocked until a separate ingestion/cache/legal review gate exists.
- Public-ready map-pack promotion must pass `scripts/check_trillionnium_world_production_map_pack_public_evidence.sh`; this gate consumes operator-supplied evidence and never performs live ingestion itself.

## Required Evidence

- `production-map-pack-route.json`
- primary and next Ed25519 public keys
- verified signature output for active and next keys
- revocation list
- attribution screenshot evidence or screenshot plan for Web/Native/Matrix
- takedown and rollback drill output
- `production-map-pack-public-evidence.json` generated from real operator evidence

## Public Launch Rule

`production_map_pack_route_green` only proves the route and local drill. Public launch still requires `production_map_pack_public_ready_green` through `trillionnium_world_production_map_pack_public_evidence_gate_v1` with real attribution screenshots, approved production data source evidence, cache policy, sensitive POI/geofence review, key custody, public distribution/revocation, rollback, and operator signoff.
