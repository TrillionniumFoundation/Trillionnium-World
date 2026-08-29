# Retired CI Surfaces

The active World repository no longer runs legacy L1, sidecar, Web4, agent-user,
self-heal or source-convergence workflows. Their exact historical content
remains available through Git history.

They were retired because they either targeted components outside the active
8-crate game workspace or allowed CI to modify, push or tag the candidate it
was validating. Current CI is read-only and listed in `.github/workflows/`.
