# TRNM Online Production v2

Status: implemented and deployed on the local closed-alpha host. Automated
local production evidence is green. Human usability, a second physical host,
KMS/HSM custody, a public protected edge and staffed moderation remain external
release gates.

Production v2 uses contract `trnm_online_production_v2`, build
`trnm-online-production-2026.07-v2`. Exact Production v1 and Operations v1/v2
protocol/build pairs remain compatible.

## Signer possession and registry convergence

The isolated signer now answers an authenticated, short-lived challenge by
signing a binding that includes the challenge, issuer, active key, provider,
public key fingerprint and expiry. The game server verifies that Ed25519
signature locally, then asks CEX through the game-authority credential for the
same key's active registry row. Startup and readiness fail closed unless issuer,
key ID, algorithm and public-key SHA-256 all agree.

This closes the gap between “the signer process answers” and “the signer holds
the private half of the public key CEX currently trusts.” The reported provider
is still `file_seed`; `key_non_exportable=false`, `kms_hsm_attested=false` and
external-provider attestation remains false.

## Distributed admission and concurrent startup

Request admission is PostgreSQL-backed rather than process-local. Fixed windows
are keyed by a hash of caller identity, method and normalized route class; raw
sessions and credentials are never persisted. Two server instances therefore
share one control-plane quota, while the existing bounded data-plane multiplier
remains explicit. Database failure returns 503 and the limit fails closed.

Concurrent fleet startup exposed a real migration deadlock during acceptance.
All schema revisions now run inside one transaction protected by a PostgreSQL
transaction-scoped advisory lock. Two instances can cold-start together without
racing DDL. Maintenance records bounded per-instance capacity/admission samples
and expires old windows/samples.

## Host evidence and honest HA boundary

A moderator-protected host challenge persists and returns a hash over the
challenge, current `physical_host_id`, instance ID/epoch, region and observation
time. It makes the one-host fact auditable and gives a second-host drill a
concrete comparison point. It is explicitly not a hardware-root identity,
quorum or cross-host failover attestation. Current evidence still reports one
distinct physical host and `cross_host_failover_attested=false`.

## Moderation shifts and case ownership

The moderation control plane can start and heartbeat a bounded shift, claim an
open report or pending appeal, reject duplicate ownership and refuse shift close
while a claim is unresolved. Existing report and appeal resolution paths close
the associated claim. Stale shifts expire in maintenance. This proves durable
queue ownership and handoff mechanics, not a staffed team or real SLA delivery.

## Native production view and delayed spectating

F2 now loads a player-safe Production v2 view containing season end, region,
admission state, signer provider/registry convergence and measured healthy host
count. F10 accepts a target-bound spectator token without rendering or writing
it to evidence; the in-memory token is cleared after exchange. F11 reads only
eligible delayed authoritative frames. The UI uses an explicit two-line control
legend and smaller body type so the production/spectator state remains readable
in the acceptance window.

## Acceptance

- `scripts/check-trnm-online-production-v2-e2e.sh` first runs the full
  Production v1 positive-value compatibility path, then verifies signer
  possession/CEX registry convergence, a real native delayed-spectator window,
  shared admission across two concurrently started instances, capacity samples,
  host challenge evidence and a moderation shift claim/resolve/close lifecycle.
- Final automated run: `online-production-v2-1783925971-3957`; post-rotation
  compatibility match `a72233fc-8946-4eed-b3df-282b60137c47`, Production v1
  compatibility run `online-production-v1-1783925971-22023`, signer key
  `trnm-online-ed25519-production-1783925994-9823`, spectator grant
  `5aca414f-f4e3-4083-94a6-4dc4bb57bf8a`, moderation shift
  `b8d97efa-c542-4c32-bd90-310e00454688` and appeal
  `0c423437-cd44-4c15-b460-7846057f8d08`.
- The distributed 30/minute probe sends fifteen requests through each of two
  instances and receives 429 on the shared thirty-first request. Both instances
  produce capacity samples and the database still reports one physical host.
- Exact historical Operations v2 remains green in
  `online-operations-v2-1783926608-5566` (31 replay frames, two commands,
  appeal/revocation and season rotation/archive), while same-ID epoch fencing
  remains green in `online-operations-fencing-1783926651-27567` (epoch
  172 -> 173 -> 174 and stale route 503).
- `scripts/prepare-trnm-online-production-v2-external-gates.sh` produces a
  no-credit packet for real people, a second host, KMS/HSM, public edge and
  staffed moderation evidence.

## Honest boundary

The final run is automated and local. It contains no completed two-player human
session, independently powered second machine, hardware/non-exportable signer,
public WAF/DDoS/capacity result or staffed moderation shift. Non-loopback binding
continues to fail closed. Ranked remains zero RPG XP, zero items and zero CEX
value. Public registration verification, MFA/passkey, cross-platform keychain,
chat/guild, public inventory custody/listing, customer support and legal approval
remain blocked; the public player market stays disabled.
