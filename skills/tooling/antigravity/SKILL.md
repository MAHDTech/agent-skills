---
name: antigravity
description: Look up Google Antigravity/AGY CLI, IDE, and SDK behavior. Use for its settings, commands, keybindings, agent permissions, hooks, or plugin integration.
resources:
  - https://antigravity.google/docs/agent-settings.md
  - https://antigravity.google/docs/artifact-review.md
  - https://antigravity.google/docs/artifacts.md
  - https://antigravity.google/docs/boost.md
  - https://antigravity.google/docs/build-with-google.md
  - https://antigravity.google/docs/changelog.md
  - https://antigravity.google/docs/cli/artifacts.md
  - https://antigravity.google/docs/cli/best-practices.md
  - https://antigravity.google/docs/cli/commands/agents.md
  - https://antigravity.google/docs/cli/commands/changelog.md
  - https://antigravity.google/docs/cli/commands/codesearch.md
  - https://antigravity.google/docs/cli/commands/context.md
  - https://antigravity.google/docs/cli/commands/credits.md
  - https://antigravity.google/docs/cli/commands/diff.md
  - https://antigravity.google/docs/cli/commands/effort.md
  - https://antigravity.google/docs/cli/commands/permissions.md
  - https://antigravity.google/docs/cli/commands/resume.md
  - https://antigravity.google/docs/cli/commands/statusline.md
  - https://antigravity.google/docs/cli/commands/title.md
  - https://antigravity.google/docs/cli/commands/usage.md
  - https://antigravity.google/docs/cli/commands/voice.md
  - https://antigravity.google/docs/cli/conversations.md
  - https://antigravity.google/docs/cli/credits.md
  - https://antigravity.google/docs/cli/gcli-migration.md
  - https://antigravity.google/docs/cli/headless.md
  - https://antigravity.google/docs/cli/modes.md
  - https://antigravity.google/docs/cli/prompting.md
  - https://antigravity.google/docs/cli/reference.md
  - https://antigravity.google/docs/cli/statusline.md
  - https://antigravity.google/docs/cli/title.md
  - https://antigravity.google/docs/cli/troubleshooting.md
  - https://antigravity.google/docs/cli/using.md
  - https://antigravity.google/docs/cli/vim-editor-mode.md
  - https://antigravity.google/docs/commands/browser.md
  - https://antigravity.google/docs/commands/goal.md
  - https://antigravity.google/docs/commands/grill-me.md
  - https://antigravity.google/docs/commands/learn.md
  - https://antigravity.google/docs/commands/schedule.md
  - https://antigravity.google/docs/enterprise.md
  - https://antigravity.google/docs/faq.md
  - https://antigravity.google/docs/features.md
  - https://antigravity.google/docs/firebase-studio-migration.md
  - https://antigravity.google/docs/getting-started.md
  - https://antigravity.google/docs/hooks.md
  - https://antigravity.google/docs/ide/agent-side-panel.md
  - https://antigravity.google/docs/ide/allowlist-denylist.md
  - https://antigravity.google/docs/ide/browser.md
  - https://antigravity.google/docs/ide/browser-recordings.md
  - https://antigravity.google/docs/ide/extensions.md
  - https://antigravity.google/docs/ide/review-changes-editor.md
  - https://antigravity.google/docs/ide/separate-chrome-profile.md
  - https://antigravity.google/docs/ide/tab.md
  - https://antigravity.google/docs/ide/workflows.md
  - https://antigravity.google/docs/implementation-plan.md
  - https://antigravity.google/docs/install.md
  - https://antigravity.google/docs/marketplace.md
  - https://antigravity.google/docs/mcp.md
  - https://antigravity.google/docs/models.md
  - https://antigravity.google/docs/overview.md
  - https://antigravity.google/docs/permissions.md
  - https://antigravity.google/docs/plan.md
  - https://antigravity.google/docs/plans.md
  - https://antigravity.google/docs/plugins.md
  - https://antigravity.google/docs/projects.md
  - https://antigravity.google/docs/remote-control.md
  - https://antigravity.google/docs/rules.md
  - https://antigravity.google/docs/sandbox.md
  - https://antigravity.google/docs/screenshots.md
  - https://antigravity.google/docs/sdk/lifecycle.md
  - https://antigravity.google/docs/sdk/local-models.md
  - https://antigravity.google/docs/sdk/mcp.md
  - https://antigravity.google/docs/sdk/overview.md
  - https://antigravity.google/docs/sdk/personas.md
  - https://antigravity.google/docs/sdk/policies.md
  - https://antigravity.google/docs/sdk/structured-output.md
  - https://antigravity.google/docs/sdk/subagents.md
  - https://antigravity.google/docs/sdk/tools.md
  - https://antigravity.google/docs/settings.md
  - https://antigravity.google/docs/sidecars.md
  - https://antigravity.google/docs/skills.md
  - https://antigravity.google/docs/slash-commands.md
  - https://antigravity.google/docs/subagents.md
  - https://antigravity.google/docs/teamwork.md
  - https://antigravity.google/docs/walkthrough.md
---

# Google Antigravity documentation

Answer Antigravity questions from the vendored official docs in `resources/auto/` rather than from memory. Each file is the Markdown version of one page under `https://antigravity.google/docs/`, named after its path (for example `resources/auto/docs-hooks.md`, `resources/auto/docs-cli-reference.md`, `resources/auto/docs-sdk-subagents.md`).

Use this skill to research or verify Antigravity CLI flags, slash commands, settings, rules, skills, subagents, permissions, sandboxing, hooks, MCP, plugins, or the SDK, or to configure Antigravity workspace or user settings.

## Steps

1. Find the page: list or search `resources/auto/` by topic keyword (file names mirror the docs URL paths).
2. Read the matching file and answer from its text.
3. When no local file covers the topic, fetch `https://antigravity.google/llms.txt` for the current page index and read the page's `.md` URL (append `.md` to the docs path).

## Completion criteria

- Every fact, command, parameter, or schema in the answer matches the docs text.
- Each cited fact names its source page as a `https://antigravity.google/docs/...` URL.
