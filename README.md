# Agent Skills

[![skills.sh](https://skills.sh/b/MAHDTech/agent-skills)](https://skills.sh/MAHDTech/agent-skills)

My personal and public agent skills — cross-compatible with Claude Code, OpenCode, Goose, and Antigravity CLI.

## Install

```bash
npx skills add MAHDTech/agent-skills
```

## Documentation

- [install](docs/install.md)
- [usage](docs/usage.md)

## Available Skills

### Engineering

The core build, debug, and delivery loop.

- **[codebase-design](skills/engineering/codebase-design/SKILL.md)** — Shared vocabulary and principles for designing deep modules. Use when designing or improving a module's interface, hunting for deepening opportunities, deciding where a seam goes, making code more testable or easier for an agent to navigate, or when another skill needs the deep-module vocabulary.
- **[diagnosing-bugs](skills/engineering/diagnosing-bugs/SKILL.md)** — A disciplined loop for hard bugs and performance regressions. Use when the user says debug or diagnose this, or reports something broken, throwing, failing, flaky, or slow. Insists on building a tight, red-capable feedback loop before hypothesising, then reproduces, minimises, ranks hypotheses, instruments, fixes with a regression test, and runs a post-mortem.
- **[domain-modeling](skills/engineering/domain-modeling/SKILL.md)** — Build and sharpen a project's domain model as you design — pin down the ubiquitous language in a CONTEXT.md glossary and record hard-to-reverse architectural decisions as ADRs. Use when you want to nail down domain terminology, challenge fuzzy or conflicting terms, invent edge-case scenarios to stress-test the model, or record an architectural decision — or when another skill needs to maintain the domain model.
- **[implement](skills/engineering/implement/SKILL.md)** — Implement a piece of work based on a spec or set of tickets.
- **[productionize-app](skills/engineering/productionize-app/SKILL.md)** — Transform applications into production-ready deployments with systematic analysis, improvement, and framework-specific optimization
- **[refactor-codebase](skills/engineering/refactor-codebase/SKILL.md)** — Remove engineering debt, untangle large modules, collapse duplicated logic, and modernize legacy patterns without changing behavior. Use when refactoring a codebase, removing dead code, or cleaning up stale abstractions.
- **[research](skills/engineering/research/SKILL.md)** — Investigate a question against high-trust primary sources — official docs, source code, specs, first-party APIs — and capture the findings as a cited Markdown file in the repo. Use when you want a topic researched, docs or API facts gathered and verified, or the reading legwork delegated to a background agent while you keep working.
- **[sculpt-code](skills/engineering/sculpt-code/SKILL.md)** — Reshape code for readability, naming, structure, TODOs, and reduced surface area across any language
- **[tdd](skills/engineering/tdd/SKILL.md)** — Test-driven development done red-green-refactor. Use when building a feature or fixing a bug test-first, writing integration tests, deciding what to test and where the test seams go, or avoiding brittle implementation-coupled tests. Covers what a good test is, the anti-patterns to avoid, and the rules of the red-green loop.

### Planning

Turn ideas into specs, tickets, and multi-session plans.

- **[grilling](skills/planning/grilling/SKILL.md)** — Interview the user relentlessly, one question at a time, to stress-test a plan or design before any code is written. Use when the user wants to pressure-test an approach, resolve open design decisions, or asks you to 'grill me', 'poke holes in this', 'stress-test this plan', or 'interview me about this design'. Walk every branch of the decision tree, look up facts in the codebase, and put each real decision to the user with a recommended answer before proceeding.
- **[handoff](skills/planning/handoff/SKILL.md)** — Compact the current conversation into a handoff document for another agent to pick up.
- **[plan-before-coding](skills/planning/plan-before-coding/SKILL.md)** — Intercepts requests to ensure the AI agent and human work back and forth to create an implementation plan before diving into code. Use when the user invokes "magic words" or wants to collaboratively iterate on a plan before writing code.
- **[store-plan](skills/planning/store-plan/SKILL.md)** — Capture the current conversation's plan, decisions, and action items into a structured markdown file in the project's plans/ directory. Triggers on "store this plan", "save this plan for later", "document this for later", "write up what we discussed", "create a plan file", or "/cmd-store-plan".
- **[to-spec](skills/planning/to-spec/SKILL.md)** — Turn the current conversation into a spec (sometimes called a PRD) and publish it to your project's issue tracker — no interview, just synthesis of what you have already discussed.
- **[to-tickets](skills/planning/to-tickets/SKILL.md)** — Break a plan, spec, or the current conversation into a set of tracer-bullet tickets, each declaring its blocking edges, and publish them to your project's issue tracker — as one file per ticket locally, or as native blocking links on a real tracker.
- **[triage](skills/planning/triage/SKILL.md)** — Move issues and external PRs through a state machine of triage roles — categorise, verify, grill if needed, and write agent-ready briefs.
- **[wayfinder](skills/planning/wayfinder/SKILL.md)** — Plan a huge chunk of work — more than one agent session can hold — as a shared map of investigation tickets on your issue tracker, and resolve them one at a time until the way to the destination is clear.

### Review

Review diffs, pull requests, and test plans.

- **[code-review](skills/review/code-review/SKILL.md)** — Review the changes since a fixed point (a commit, branch, tag, or merge-base) along two axes — Standards, meaning does the code follow this repo's documented conventions plus a baseline of common code smells, and Spec, meaning does the code do what the originating issue, ticket, or PRD asked for. Runs both reviews as parallel sub-agents and reports them side by side without merging or reranking them. Use when the user wants to review a branch, a pull request, work-in-progress changes, or asks to review the diff since some point.
- **[pr-build-context](skills/review/pr-build-context/SKILL.md)** — Build high-signal PR context for review with diff analysis, risk assessment, and discussion questions
- **[pr-create-test-plan](skills/review/pr-create-test-plan/SKILL.md)** — Generate manual test plans for PR changes — focused on hands-on verification a developer would do, not unit-test edge cases
- **[pr-edge-cases](skills/review/pr-edge-cases/SKILL.md)** — Review branch changes for test gaps, logic edge cases, failure modes, and integration risks
- **[pr-prepare-review](skills/review/pr-prepare-review/SKILL.md)** — Prepare branch for code review by building context, identifying issues, and suggesting improvements

### GitHub

GitHub and git workflows via the gh CLI.

- **[gh-create-issue](skills/github/gh-create-issue/SKILL.md)** — Create structured GitHub issues from conversation context using gh CLI
- **[gh-create-pr](skills/github/gh-create-pr/SKILL.md)** — Generate a PR title and description, then commit, create/update the PR on approval
- **[gh-resolve-pr-comments](skills/github/gh-resolve-pr-comments/SKILL.md)** — Triage and resolve GitHub PR review comments with categorized action plans and approval-gated execution
- **[git-resolve-conflicts](skills/github/git-resolve-conflicts/SKILL.md)** — Resolve merge conflicts systematically with context-aware 3-tier classification and escalation protocol

### Reflection

Self-critique and review of your own work.

- **[critical-thinking](skills/reflection/critical-thinking/SKILL.md)** — Analyzes the agent's own previous response with rigorous critical thinking, looking for flaws, biases, and unstated assumptions.
- **[rfc-review](skills/reflection/rfc-review/SKILL.md)** — Review RFCs for problem clarity, compliance, security, and performance using SCQA framework
- **[scope-sweep](skills/reflection/scope-sweep/SKILL.md)** — Final pass to identify missed items, edge cases, and risks before considering a scope done
- **[self-review](skills/reflection/self-review/SKILL.md)** — Self-review after implementation — surface missed work, simplification opportunities, and idiomatic improvements

### Writing

Proofreading and documentation polish.

- **[proofread](skills/writing/proofread/SKILL.md)** — Proofread posts before publishing for spelling, grammar, repetition, logic, weak arguments, broken links, and optionally reformat for skimmability
- **[simplify-docs](skills/writing/simplify-docs/SKILL.md)** — Simplify documentation for clarity and readability with approval-gated edits

### Authoring

Create and maintain the skills themselves.

- **[skill-creator](skills/authoring/skill-creator/SKILL.md)** — Use when creating a new skill or editing an existing one in this repo — how to name it, place it in the right category, write its frontmatter and description, and structure it with progressive disclosure. Covers model-invoked vs user-invoked skills, the six skill failure modes, and the lint/sync workflow to run after adding or renaming a skill.
- **[skill-router](skills/authoring/skill-router/SKILL.md)** — Route to the right skill for the task at hand — a map of every skill in this collection and when to reach for it.

### Tooling

Environments, CLIs, and agent conventions.

- **[agent-guidelines](skills/tooling/agent-guidelines/SKILL.md)** — Behavioural guidelines for AI agents to reduce common LLM coding mistakes. Use when asked to follow the guidelines or verify work against them.
- **[bevy-development](skills/tooling/bevy-development/SKILL.md)** — Expert developer guidance on Bevy, Rust's data-driven game engine, covering ECS structure, input, assets, events, states, system scheduling, and performance tuning.
- **[devenv](skills/tooling/devenv/SKILL.md)** — Strict guidelines for using devenv for shell and dependency management. Use when a repo contains a devenv.nix or devenv.yaml file, or when running commands in a devenv environment.
- **[opencode](skills/tooling/opencode/SKILL.md)** — This skill provides comprehensive guidance for using OpenCode, the open-source AI coding agent. Use this skill when working with OpenCode CLI commands, keyboard shortcuts, agents (build/plan), slash commands, tools, skills, MCP servers, or configuration. Automatically triggered when OpenCode-specific questions or tasks are detected.
- **[opencode-acp](skills/tooling/opencode-acp/SKILL.md)** — Control OpenCode directly via the Agent Client Protocol (ACP). Start sessions, send prompts, resume conversations, and manage OpenCode updates.
- **[scratchpad](skills/tooling/scratchpad/SKILL.md)** — Enforces the use of a "scratch/" directory for all temporary or experimental AI agent scripts. Ensures the directory is .gitignored and instructs agents to clean up after completion.
