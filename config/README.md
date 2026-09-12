# Repository Configuration

Configuration in this tree must be environment-neutral, reviewable, and free of secrets.

- Commit examples or safe defaults only.
- Document precedence between files, environment variables, and command-line flags.
- Invalid security- or authority-sensitive values must fail closed.
- Production credentials, private keys, tokens, database URLs, and customer identifiers must never be committed.
- Changes that alter limits, economics, trust, or release behavior require an update to the owning module README and active gap/release documents.
