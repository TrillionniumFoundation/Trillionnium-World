# Automation and Evidence Scripts

Scripts in this tree provide build, test, fault, packaging, release-review, and evidence helpers.

Rules:

- fail closed when required inputs or evidence paths are missing;
- resolve the repository root dynamically; never hard-code a developer workstation path;
- print exact commands, commit identity, UTC timestamp, environment scope, and artifact paths;
- do not convert warnings into release credit;
- make destructive operations explicit and bounded;
- a script PASS closes only the release row named by its contract.

Canonical documentation validation:

```bash
python3 scripts/check_docs_quality.py
```
