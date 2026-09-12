# Packaging

This tree contains distribution metadata and installer inputs.

Packaging gates must verify exact source commit and clean-tree policy, complete payload/runtime dependencies, license/provenance inventory, safe archive paths/file modes, per-file size and SHA-256 integrity, reproducible version metadata, and platform signing/notarization status.

A Linux archive does not satisfy Windows/macOS signing or public distribution gates. Unsigned artifacts must be described as development or internal evidence only.
