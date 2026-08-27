+++
title = "devenv-supported-process-managers-eaf14ace"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Alternative process managers

If an existing workflow depends on a specific external process manager,
devenv can integrate with:

- [process-compose](devenv-process-compose-a7637492.md) — feature-rich supervision with
  a TUI
- [overmind](devenv-overmind-fb9c5c1f.md) — Procfile-based supervision with tmux
  integration
- [mprocs](devenv-mprocs-00d07789.md) — a cross-platform TUI process runner
- [hivemind](devenv-hivemind-5127010d.md) — a small Procfile process manager
- [honcho](devenv-honcho-535851c7.md) — a Python Foreman port

These integrations are compatibility options. Their behavior and
supported features are determined by the external manager and may differ
from the native manager.

