# Trillionnium World V4 No-Go Conditions

Promotion stops immediately when any condition below is true:

1. World and Nakama claim the same canonical cursor, root, recovery authority, or signature.
2. A signer/CEX/network request runs while mutable match or campaign rows are locked.
3. A stale or expired worker can authorize, complete, retry, dead-letter, or apply settlement.
4. One poison match, capture, or job can block unrelated settlement work.
5. A malformed successful response is treated as permanent failure rather than ambiguous lookup recovery.
6. Canonical JSON accepts invalid grammar, duplicate/unsorted keys, non-i64 numbers, nonminimal escapes, depth overflow, trailing bytes, or escaped authority keys.
7. CI can modify, commit, push, tag, merge, or self-promote candidate source.
8. Exact required checks are absent, stale, skipped, cancelled, or not bound to the current head.
9. Production can fall back to an unverified development binary.
10. Runtime roles share one root credential or expose signer private material.
11. Active-match cross-generation takeover is assumed rather than proven.
12. Source/local evidence is used as deployment, cross-host, public-network, human, custody, legal, or commercial credit.
13. Public online or the public player market is enabled without every dependency explicitly green.

A no-go result is a valid engineering outcome. It must remain visible in status,
release notes, and operator handoff until the owning gate closes with exact
evidence and independent review.
