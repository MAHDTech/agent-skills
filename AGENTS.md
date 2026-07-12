# Agent Skills Repository

## Overview

This repository houses AI Agent skills that work across multiple agent tools: _Claude Code_, _OpenCode_, _Goose_, and _Antigravity CLI_. They all read the same Anthropic-style `skills/<name>/SKILL.md` format, so a single skill runs everywhere.

See the [README.md](./README.md) for an overview of the project and available skills.

## Structure

- Skills live under `skills/<category>/<name>/SKILL.md`, grouped by topic (engineering, planning, review, github, reflection, writing, authoring, tooling), with `in-progress/` and `deprecated/` lifecycle buckets.
- Names are prefix-free kebab-case; each folder name matches the skill `name` in its frontmatter.
- Consumers install with `npx skills add MAHDTech/agent-skills`. See [docs/install.md](./docs/install.md) for details.

## Development

- Environment: This project requires `devenv`. Always check if `devenv` is active. Run ad-hoc commands inside the devenv shell with `devenv shell -- <command>`.
  - **CRITICAL LINTING RULE:** NEVER run individual linters directly (e.g., `devenv shell -- markdownlint .`). All linters and pre-commit hooks MUST be configured and run inside `devenv.nix`.
  - **CRITICAL TESTING RULE:** ALWAYS run tests via `devenv test` or the `run-tests` wrappers. This is the single guaranteed path.
  - If a specific linter or pre-commit check doesn't exist, check the devenv MCP server or devenv agent docs. If you STILL don't find it, ask the user for confirmation.
- Runtime: `bun`. Use `bun` for all scripts.
- CLI Skills Tool: `bun run bin/skills.ts` (also `bun run skills`). Use this to lint and sync skills, and to symlink them for local development.

## Dashboard

- To build: `dashboard --action build` — syncs content, builds CSS, renders the site, indexes it (all-in-one).
- To serve: `dashboard --action serve` — builds once, then serves with live reload. (Uses Zola & Tailwind.)
- CSS only: `dashboard --action css` (escape hatch; `build` already does this).

## Behaviour

Behavioural guidelines to reduce common LLM coding mistakes.

**Tradeoff:** These guidelines bias toward caution over speed. For trivial tasks, use judgment.

### 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:

- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

### 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

### 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:

- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:

- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

### 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:

- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:

```text
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

---

**These guidelines are working if:** fewer unnecessary changes in diffs, fewer rewrites due to over-complication, and clarifying questions come before implementation rather than after mistakes.
