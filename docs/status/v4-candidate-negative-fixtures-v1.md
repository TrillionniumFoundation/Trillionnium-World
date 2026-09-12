# V4 Candidate Negative Fixture Contract

`test-trnm-world-v4-candidate-negative.py` uses only temporary files and passes
them to `check-trnm-world-v4-candidate.py --status ...`. It never overwrites,
stages, commits, pushes, tags, or promotes repository source.

The fixture suite must reject:

- promotion before exact-head evidence;
- invented commit/tree identities;
- invented workflow runs, artifacts, or reviewers;
- release/public-online/public-market overclaims;
- hidden required checks.

The source status intentionally leaves candidate commit/tree and all future
evidence arrays empty. GitHub server evidence is recorded only after the exact
head actually runs and passes, and then only in a separately reviewed evidence
update.
