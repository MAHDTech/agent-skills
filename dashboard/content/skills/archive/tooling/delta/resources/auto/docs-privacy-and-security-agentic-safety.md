+++
title = "docs-privacy-and-security-agentic-safety"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Agentic Safety"
url: "https://delta.dev/docs/privacy-and-security/agentic-safety"
description: "The current limits of Delta's agent safeguards."
---

1.  Privacy & Security
2.  Agentic Safety

# Agentic Safety

The current limits of Delta's agent safeguards.

On this page

1.  [Agent permissions](#agent-permissions)
2.  [Agent sandboxing](#agent-sandboxing)
3.  [Delta worktree trust](#delta-worktree-trust)
4.  [Planned safeguards](#planned-safeguards)

Delta is in early access and does not yet have the agent safeguards described below. Use Delta with care. Early-access use is at your own risk and subject to Delta’s [Early Access Agreement](https://delta.dev/early-access).

## [Agent permissions](#agent-permissions)

Delta does not have an agent permission system. Based on your instructions, the agent acts autonomously and does not ask before calling tools. This includes potentially destructive calls.

## [Agent sandboxing](#agent-sandboxing)

Delta does not sandbox agents. An agent has unrestricted access to the device where it runs.

## [Delta worktree trust](#delta-worktree-trust)

Delta does not ask you to review settings or configuration in a shared repository or Delta worktree before loading them. Code in the repository can run automatically:

*   An executable `.agents/prepare` script, or its `.delta/prepare` override, runs when Delta first mounts a managed checkout. Adopted existing checkouts are skipped.
*   On macOS and Linux, an `.envrc` that applies to the checkout runs through `direnv` when an interactive terminal or an agent command starts, and Delta allows it if `direnv` has not seen it before.
*   Rules files such as `AGENTS.md` and skills under `.agents/skills/` or `.delta/skills/` become instructions to the agent.

Review these files before working in a repository you do not trust. See [Terminals](/docs/agents/terminals#how-the-agent-runs-commands) and [Delta Worktrees](/docs/concepts/worktrees).

## [Planned safeguards](#planned-safeguards)

Agent permissions, sandboxing, and Delta worktree trust controls are on the roadmap. We will update this page as they ship.
