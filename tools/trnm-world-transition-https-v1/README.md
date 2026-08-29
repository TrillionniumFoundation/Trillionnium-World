# Trillionnium World transition HTTPS fixture v1

This isolated, standard-library-only Go module exposes the exact
`trnm_world_transition_v1` envelope over a bounded TLS 1.3 HTTPS surface for
World/Game fault and shadow evidence.

It is deliberately a **fixture**, not the production World rules engine. Its
only deterministic domain is:

- state: `{"counter": signed_i64}`;
- command: `{"delta": signed_i64, "kind": "advance" | "reject"}`;
- ruleset revision: `blackbox-ruleset-v1`;
- content revision: `blackbox-content-v1`.

The fixture verifies exact canonical JSON, payload hashes, request hashes,
transition hashes, outcome hashes, resource limits and forbidden authority
surfaces. Successful result bytes are persisted by request hash using:

```text
exclusive same-directory temporary file
-> file fsync
-> atomic hard-link publication that cannot replace an existing result
-> directory fsync
-> temporary-link removal
-> directory fsync
```

The result directory must be a real private directory rather than a symlink,
and cached result paths must be regular owner-only files rather than symlinks.
Two processes racing on the same request hash therefore preserve the first
committed bytes; a retry after upstream success and downstream response loss
receives those exact bytes.

## HTTP surface

- `GET /healthz`
- `POST /v1/transition` — bearer protected
- `GET /v1/result/<request_hash>` — bearer protected
- `GET /v1/stats` — bearer protected

The server requires TLS 1.3 exactly. It has no database, player session,
participant admission, global-order, completion-signing, wallet, settlement or
finality capability.

## Configuration

```text
TRNM_WORLD_FIXTURE_LISTEN=:7443
TRNM_WORLD_FIXTURE_TLS_CERT=/run/trnm/world/tls.crt
TRNM_WORLD_FIXTURE_TLS_KEY=/run/trnm/world/tls.key
TRNM_WORLD_FIXTURE_BEARER_TOKEN=<32..4096 bytes>
TRNM_WORLD_FIXTURE_RESULT_DIR=/var/lib/trnm-world-fixture/results
TRNM_WORLD_FIXTURE_MAX_REQUEST_BYTES=<optional bounded integer>
```

`TRNM_WORLD_FIXTURE_RESULT_DIR` must be an absolute writable directory owned by
the non-root runtime user. It must not be shared across hosts; multi-host
fencing remains an explicit external gate. Filesystems that cannot provide
same-directory hard-link publication are rejected rather than weakened to an
overwriting fallback.

## Validation

```bash
gofmt -w .
go test ./... -count=1
go test -race ./internal/fixture -count=1
go vet ./...
CGO_ENABLED=0 go build -trimpath -o trnm-world-transition-https-v1 \
  ./cmd/trnm-world-transition-https-v1
```

To build the final scratch image, place the static binary beside
`Dockerfile.prebuilt` and run Docker with network disabled.
