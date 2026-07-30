# Agent Skills

[![skills.sh](https://skills.sh/b/MAHDTech/agent-skills)](https://skills.sh/MAHDTech/agent-skills)

Working on my _skill issues_.

![skill issues](./docs/images/skill-issues.png)

These are my personal agent skills and attempt to be cross-compatible with Antigravity, Claude Code, Goose and OpenCode.

## Install

```bash
# Using npm
npx skills add MAHDTech/agent-skills

# Using Bun
bunx skills add MAHDTech/agent-skills
```

## Documentation

- [dashboard](docs/dashboard.md)
- [install](docs/install.md)
- [usage](docs/usage.md)

## Available Skills

### Engineering

The core build, debug, and delivery loop.

- **[acp](skills/engineering/acp/SKILL.md)** — Expert guidance for Agent Client Protocol (ACP) — building, integrating, or debugging ACP agents and clients, implementing JSON-RPC 2.0 schemas, managing session lifecycles, elicitation, tool calls, v1/v2 migration, and using official SDKs (TypeScript, Rust, Python, Java, Kotlin). Use when working with ACP protocols, building ACP-compatible AI agent servers or client applications, or inspecting ACP specifications and RFDs.
- **[characterization-tests](skills/engineering/characterization-tests/SKILL.md)** — Pin down the existing behaviour of untested or legacy code with characterization (golden-master / approval) tests before you change it, so refactors and upgrades stay safe. Use when you must modify, refactor, or upgrade code that has no tests, want to capture current behaviour as an oracle even where it looks wrong, or need a safety net before a risky change. Distinct from /tdd, which specifies new behaviour test-first; this tests around existing behaviour. Pairs with /sculpt-code and /upgrade-dependencies.
- **[codebase-design](skills/engineering/codebase-design/SKILL.md)** — Shared vocabulary and principles for designing deep modules. Use when designing or improving a module's interface, hunting for deepening opportunities, deciding where a seam goes, making code more testable or easier for an agent to navigate, or when another skill needs the deep-module vocabulary.
- **[diagnosing-bugs](skills/engineering/diagnosing-bugs/SKILL.md)** — A disciplined loop for hard bugs and performance regressions. Use when the user says debug or diagnose this, or reports something broken, throwing, failing, flaky, or slow. Insists on building a tight, red-capable feedback loop before hypothesising, then reproduces, minimises, ranks hypotheses, instruments, fixes with a regression test, and runs a post-mortem.
- **[domain-modeling](skills/engineering/domain-modeling/SKILL.md)** — Build and sharpen a project's domain model as you design — pin down the ubiquitous language in a CONTEXT.md glossary and record hard-to-reverse architectural decisions as ADRs. Use when you want to nail down domain terminology, challenge fuzzy or conflicting terms, invent edge-case scenarios to stress-test the model, or record an architectural decision — or when another skill needs to maintain the domain model.
- **[implement](skills/engineering/implement/SKILL.md)** — Implement a piece of work based on a spec or set of tickets.
- **[nutanix-api-v4](skills/engineering/nutanix-api-v4/SKILL.md)** — Expert guidance, API endpoints, SDK usage, OData filtering and sorting, namespace specifications, release updates, and troubleshooting for Nutanix v4 APIs (AOS 7.0+, Prism Central pc.2024.3+). Use when developing, integrating, or troubleshooting with Nutanix v4 REST APIs, Python/Go/Java/JavaScript v4 SDKs, OData parameters, or migrating off deprecated legacy Nutanix APIs (v1, v2, v3).
- **[nutanix-files](skills/engineering/nutanix-files/SKILL.md)** — Expert guidance, CLI commands, architecture reference, file server & share management, SMB/NFS protocols, directory services, smart tiering, disaster recovery, analytics, performance tuning, and troubleshooting for Nutanix Files (v5.0). Use when working with Nutanix Files, SMB/NFS file shares, file analytics, Smart Tiering, Metro Availability, Files REST APIs, or file server performance optimization.
- **[nutanix-kubernetes-platform](skills/engineering/nutanix-kubernetes-platform/SKILL.md)** — Expert guidance, CLI commands, architecture reference, cluster lifecycle operations, air-gapped installation, image building, upgrades, and troubleshooting for Nutanix Kubernetes Platform (NKP v2.18), including Konvoy, Kommander, and NKP Insights. Use when working with NKP, Nutanix Kubernetes Platform, Konvoy, Kommander, nkp CLI commands, or Nutanix Kubernetes cluster management.
- **[nutanix-objects](skills/engineering/nutanix-objects/SKILL.md)** — Expert guidance, CLI commands, architecture reference, bucket lifecycle management, S3 API operations, streaming replication, IAM configuration, monitoring, and troubleshooting for Nutanix Objects (v5.3). Use when working with Nutanix Objects, S3 object storage, Nutanix Objects Browser, bucket policies, WORM storage, cloud tiering, or Nutanix Objects REST APIs.
- **[optimize-performance](skills/engineering/optimize-performance/SKILL.md)** — Improve the performance of a hot path with measurement-first discipline — profile to find the real bottleneck, set a target, change one thing, re-measure, and stop when the target is met. Use when code is too slow, memory-hungry, or throughput-bound and you want to make it deliberately faster without ever guessing at the cause. For a performance regression — code that was fast and got slow — run /diagnosing-bugs instead, which treats the slowdown as a bug to bisect.
- **[productionize-app](skills/engineering/productionize-app/SKILL.md)** — Transform an application into a production-ready deployment through systematic analysis, hardening, and framework-specific optimization. Use when preparing an app to ship — auditing readiness, tightening config, and getting it deployable to a target platform.
- **[research](skills/engineering/research/SKILL.md)** — Investigate a question against high-trust primary sources — official docs, source code, specs, first-party APIs — and capture the findings as a cited Markdown file in the repo. Use when you want a topic researched, docs or API facts gathered and verified, or the reading legwork delegated to a background agent while you keep working.
- **[sculpt-code](skills/engineering/sculpt-code/SKILL.md)** — Reshape code for readability, naming, structure, TODOs, and reduced surface area — and take on larger cleanups like removing engineering debt, untangling oversized modules, and collapsing duplicated logic — all without changing behaviour. Use when you want to clean up or refactor code, from a quick readability pass to a staged debt-reduction effort.
- **[spacetimedb](skills/engineering/spacetimedb/SKILL.md)** — Expert guidance for developing, publishing, and debugging SpacetimeDB database modules (in Rust, C#, TypeScript, C++) and connecting real-time clients. Use when asked to write SpacetimeDB table schemas, reducers, views, schedule/event tables, or when using the spacetime CLI, generating client SDK bindings, or implementing WebSocket subscriptions.
- **[tars-backlog-implement](skills/engineering/tars-backlog-implement/SKILL.md)** — Implement pending backlog issues from `.tars/issues/todo/` in parallel, conflict-free batches using isolated clones. Reach for this when asked to implement backlog issues, execute tasks from tickets in parallel, or resolve the issue queue.
- **[tdd](skills/engineering/tdd/SKILL.md)** — Test-driven development done red-green-refactor. Use when building a feature or fixing a bug test-first, writing integration tests, deciding what to test and where the test seams go, or avoiding brittle implementation-coupled tests. Covers what a good test is, the anti-patterns to avoid, and the rules of the red-green loop.
- **[upgrade-dependencies](skills/engineering/upgrade-dependencies/SKILL.md)** — Safely upgrade dependencies, frameworks, or a language/runtime version across a codebase, including risky major-version bumps and framework migrations. Use when bumping a package, stepping a major version, migrating a framework, or updating a runtime — read the changelog first, move in small reversible steps behind a green safety net, handle transitive and lockfile changes, and stage the rollout. Lean on /characterization-tests for the net and /diagnosing-bugs when an upgrade breaks something.
- **[xai](skills/engineering/xai/SKILL.md)** — Build applications with xAI and Grok APIs, SDKs, model selection, function calling, vision, live web search, and OpenAI-compatible client configurations. Use when integrating Grok models or xAI API endpoints, querying xAI capabilities, or configuring xAI API SDKs.

### Game Development

Game engines and game-development workflows.

- **[bevy-development](skills/game-development/bevy-development/SKILL.md)** — Expert guidance for building 2D and 3D games in Bevy 0.19, Rust's data-driven ECS game engine — components, systems, scheduling, states, queries, input, assets, messages, and performance. Use when writing or modernizing Bevy code, laying out ECS data and systems, scheduling or gating systems, handling input/assets/messages, or migrating off deprecated Bevy APIs like bundles, delta_seconds, EventReader, or Parent.

### Planning

Turn ideas into specs, tickets, and multi-session plans.

- **[estimate-work](skills/planning/estimate-work/SKILL.md)** — Size and estimate a body of work to set expectations and sequence delivery — turn a plan or ticket set into defensible estimates with ranges, named assumptions, and surfaced risk instead of false-precise point numbers. Use when the user wants to estimate or size work, forecast how long something will take, choose between relative sizing and time-based estimates, calibrate against past delivery, or decide whether to spike before committing.
- **[grilling](skills/planning/grilling/SKILL.md)** — Interview the user relentlessly, one question at a time, to stress-test a plan or design before any code is written. Use when the user wants to pressure-test an approach, resolve open design decisions, or asks you to 'grill me', 'poke holes in this', 'stress-test this plan', or 'interview me about this design'. Walk every branch of the decision tree, look up facts in the codebase, and put each real decision to the user with a recommended answer before proceeding.
- **[handoff](skills/planning/handoff/SKILL.md)** — Compact the current conversation into a handoff document for another agent to pick up.
- **[prioritize-backlog](skills/planning/prioritize-backlog/SKILL.md)** — Groom and prioritise a backlog — order, cut, cluster, and sequence a pile of work by value against cost and risk so the next thing to do is obvious. Use when the user wants to prioritise or rank a backlog, decide what to build next, trim or triage a pile of ideas, apply a lens like value-vs-effort, RICE, or cost-of-delay/WSJF, or sequence work around dependencies.
- **[store-plan](skills/planning/store-plan/SKILL.md)** — Capture the current conversation's plan, decisions, and action items into a structured, reviewable markdown file in the project's plans/ directory.
- **[tars-backlog-audit](skills/planning/tars-backlog-audit/SKILL.md)** — Audit the codebase for bugs, features, security issues, or technical debt, and generate structured issue files in `.tars/issues/todo/`. Reach for this when requested to perform a codebase audit, search for bugs and tasks, or populate the backlog.
- **[tars-backlog-create-issue](skills/planning/tars-backlog-create-issue/SKILL.md)** — Use when creating a new backlog issue/ticket in the `.tars/issues/todo/` directory, defining its YAML frontmatter, headings, tasks, acceptance criteria, evidence collection, and triage review expectations.
- **[tars-backlog-loop](skills/planning/tars-backlog-loop/SKILL.md)** — Coordinate the full backlog lifecycle by sequentially executing tars-backlog-audit, tars-backlog-triage, and tars-backlog-implement to resolve all issues. Reach for this when asked to run a full backlog loop, converge on a complete project goal, or manage the overall ticket pipeline.
- **[tars-backlog-prepare](skills/planning/tars-backlog-prepare/SKILL.md)** — Prepare to run the tars-backlog-loop by verifying repository integrity, resolving isolated spoke workspaces, and cleaning up orphaned clones and branches. Reach for this to reset the environment before starting a full backlog loop.
- **[tars-backlog-triage](skills/planning/tars-backlog-triage/SKILL.md)** — Triage pending backlog issues in `.tars/issues/todo/` to verify their accuracy, check for hallucinations, and add review notes. Reach for this when requested to triage tickets, verify backlog accuracy, or prepare issues for implementation.
- **[to-spec](skills/planning/to-spec/SKILL.md)** — Turn the current conversation into a spec (sometimes called a PRD) and publish it to your project's issue tracker — no interview, just synthesis of what you have already discussed.
- **[to-tickets](skills/planning/to-tickets/SKILL.md)** — Break a plan, spec, or the current conversation into a set of tracer-bullet tickets, each declaring its blocking edges, and publish them to your project's issue tracker — as one file per ticket locally, or as native blocking links on a real tracker.
- **[wayfinder](skills/planning/wayfinder/SKILL.md)** — Plan a huge chunk of work — more than one agent session can hold — as a shared map of investigation tickets on your issue tracker, and resolve them one at a time until the way to the destination is clear.

### Review

Review diffs, pull requests, and test plans.

- **[code-review](skills/review/code-review/SKILL.md)** — Review the changes since a fixed point (a commit, branch, tag, or merge-base) along two axes — Standards, meaning does the code follow this repo's documented conventions plus a baseline of common code smells, and Spec, meaning does the code do what the originating issue, ticket, or PRD asked for. Runs both reviews as parallel sub-agents and reports them side by side without merging or reranking them. Use when the user wants to review a branch, a pull request, work-in-progress changes, or asks to review the diff since some point.
- **[pr-build-context](skills/review/pr-build-context/SKILL.md)** — Build high-signal context for a pull request before review — diff analysis, risk assessment, key files, and questions for the author. Use when you want a briefing on a PR (or the whole repo when on the base branch) before reviewing it.
- **[pr-create-test-plan](skills/review/pr-create-test-plan/SKILL.md)** — Generate a manual test plan for a branch's changes — hands-on verification of real user flows and integration behaviour, not unit-test edge cases. Use when you want a copy-paste test plan a reviewer can run by hand before merging a PR.
- **[pr-edge-cases](skills/review/pr-edge-cases/SKILL.md)** — Review branch changes for test gaps, logic edge cases, failure modes, and integration risks. Use when you want the changes on a branch probed for what breaks — untested paths, boundary conditions, race conditions, and integration hazards — before merging.
- **[rfc-review](skills/review/rfc-review/SKILL.md)** — Review an RFC or design doc for problem clarity (SCQA), compliance, security, and performance, and return the few most important issues. Use when reviewing someone's RFC or design proposal before it's approved.
- **[tars-backlog-review](skills/review/tars-backlog-review/SKILL.md)** — Review the code implementation of a backlog ticket on a subagent branch, assessing compliance with spec/acceptance criteria and repo standards before merge. Reach for this during the implementation phase of the backlog loop.

### GitHub

GitHub and git workflows via the gh CLI.

- **[gh-create-issue](skills/github/gh-create-issue/SKILL.md)** — Create structured GitHub issues from conversation context using gh CLI
- **[gh-create-pr](skills/github/gh-create-pr/SKILL.md)** — Generate a PR title and description, then commit, create/update the PR on approval
- **[gh-release-notes](skills/github/gh-release-notes/SKILL.md)** — Generate human-readable release notes for a version from the merged history and cut a GitHub release with the gh CLI — gather the commit and PR range since the last tag, group changes by type, write curated notes, and publish. Use when the user wants to cut a release, tag a version, write a changelog or release notes, or summarise what shipped since the last tag. Pairs well with conventional-commit history but does not require it.
- **[gh-resolve-pr-comments](skills/github/gh-resolve-pr-comments/SKILL.md)** — Triage and resolve GitHub PR review comments with categorized action plans and approval-gated execution
- **[gh-submit-review](skills/github/gh-submit-review/SKILL.md)** — Post a completed code review to a GitHub PR via the gh CLI — pick the event (approve, request-changes, or comment), attach inline line comments, and a summary body. Use when you have finished reviewing someone else's PR and need to submit the verdict, when a review pass (e.g. /code-review or /pr-edge-cases) produced findings to publish, or when the user asks to approve, request changes on, or leave review comments on a pull request.
- **[gh-triage](skills/github/gh-triage/SKILL.md)** — Triage a GitHub backlog — move issues and external PRs through a state machine of triage roles (categorise, verify, grill if needed, write agent-ready briefs) using the gh CLI and GitHub labels. Use when triaging your GitHub issues and PRs, deciding what's ready for an agent or a human, or turning a backlog into agent-ready briefs.
- **[git-resolve-conflicts](skills/github/git-resolve-conflicts/SKILL.md)** — Resolve merge conflicts systematically with context-aware 3-tier classification and escalation protocol
- **[git-update-branch](skills/github/git-update-branch/SKILL.md)** — Bring a feature branch up to date with its base branch safely — fetch first, detect the base, choose rebase vs merge deliberately, use --force-with-lease, and hand conflicts off cleanly. Use when a branch has fallen behind its base, when the user wants to rebase or merge in the latest from the base/default branch, when a PR reports merge conflicts or an out-of-date branch, or when deciding between rebase and merge for an update.

### Reflection

Self-critique and review of your own work.

- **[critical-thinking](skills/reflection/critical-thinking/SKILL.md)** — Analyze your own immediately preceding response with rigorous, skeptical critical thinking — surfacing flaws, hidden assumptions, logical gaps, and overlooked risks. Use when you want your last answer stress-tested for weaknesses before the user acts on it.
- **[scope-sweep](skills/reflection/scope-sweep/SKILL.md)** — Final pass to identify missed items, edge cases, and risks before considering a scope done. Use as a final breadth pass before calling a scope or task done — to catch missed items, overlooked requirements, edge cases, and risks after the core work is complete and reviewed.
- **[self-review](skills/reflection/self-review/SKILL.md)** — Self-review after implementation — surface missed work, simplification opportunities, and idiomatic improvements

### Writing

Proofreading and documentation polish.

- **[proofread](skills/writing/proofread/SKILL.md)** — Proofread posts before publishing for spelling, grammar, repetition, logic, weak arguments, broken links, and optionally reformat for skimmability
- **[simplify-docs](skills/writing/simplify-docs/SKILL.md)** — Simplify documentation for clarity and readability with approval-gated edits. Use when asked to simplify, rewrite, restructure, or de-clutter a doc — README, setup guide, reference or technical documentation — so it scans faster and leads with the shortest successful path.

### Authoring

Create and maintain the skills themselves.

- **[deprecate-skill](skills/authoring/deprecate-skill/SKILL.md)** — Retire a skill cleanly — move its directory into skills/deprecated/, add a note pointing to its replacement, purge or redirect every inbound /skill-name reference (the router included), then re-run lint and sync. Use when you want to deprecate, retire, remove, replace, or merge away a skill, or a /skill-audit flagged one for retirement. Covers when to deprecate vs delete vs merge and how to avoid dangling references. Cross-references /skill-creator and /skill-router.
- **[skill-audit](skills/authoring/skill-audit/SKILL.md)** — Periodically audit the whole skill collection for health — validate each skill's frontmatter, clarity, and category, verify every cross-reference resolves, and surface duplicates, conflicts, retirement candidates, and missing-skill gaps. Use when you want to audit or health-check the skills, spring-clean the collection, confirm the router and cross-references are accurate, or find skills to merge, split, retire, or create. Hands findings to /skill-creator to fix or create and /deprecate-skill to retire.
- **[skill-creator](skills/authoring/skill-creator/SKILL.md)** — Use when creating a new skill or editing an existing one in this repo — how to name it, place it in the right category, write its frontmatter and description, and structure it with progressive disclosure. Covers model-invoked vs user-invoked skills, the six skill failure modes, and the lint/sync workflow to run after adding or renaming a skill.
- **[skill-router](skills/authoring/skill-router/SKILL.md)** — Route to the right skill for the task at hand — a map of every skill in this collection and when to reach for it.

### Tooling

Environments, CLIs, and agent conventions.

- **[agent-guidelines](skills/tooling/agent-guidelines/SKILL.md)** — Behavioural guidelines for AI agents to reduce common LLM coding mistakes. Use before and while implementing to avoid common LLM coding pitfalls — over-engineering, unrequested refactors, silent assumptions, and unverified changes.
- **[antigravity](skills/tooling/antigravity/SKILL.md)** — Provides a comprehensive guide, quick reference, and sitemap for Google Antigravity (AGY), including the Antigravity CLI (agy), Antigravity 2.0, Antigravity IDE, Python SDK, slash commands, keybindings, and customizations (skills, rules, MCP, sidecars). Activate this skill when the user asks questions about how to use, configure, or customize Antigravity, AGY, the agy CLI, the Antigravity IDE, or Antigravity 2.0.
- **[devenv](skills/tooling/devenv/SKILL.md)** — Strict guidelines for using devenv for shell and dependency management. Use when a repo contains a devenv.nix or devenv.yaml file, or when running commands in a devenv environment.
- **[install-skills](skills/tooling/install-skills/SKILL.md)** — Install, update, and manage agent skills from a GitHub-hosted collection with the skills.sh CLI (`npx skills add <owner>/<repo>`), across every runtime the collection targets — Claude Code, OpenCode, Goose, and Antigravity CLI. Use when adding skills to an agent, updating a stale copy, verifying an install, wiring the same collection into another runtime, or troubleshooting a skill that will not show up or that collides by name.
- **[opencode](skills/tooling/opencode/SKILL.md)** — This skill provides comprehensive guidance for using OpenCode, the open-source AI coding agent. Use this skill when working with OpenCode CLI commands, keyboard shortcuts, agents (build/plan), slash commands, tools, skills, MCP servers, or configuration. Automatically triggered when OpenCode-specific questions or tasks are detected.
- **[opencode-acp](skills/tooling/opencode-acp/SKILL.md)** — Control OpenCode directly via the Agent Client Protocol (ACP). Start sessions, send prompts, resume conversations, and manage OpenCode updates. Use when driving OpenCode headless or programmatically over ACP — starting a session, sending prompts, resuming a prior session, or checking and triggering OpenCode updates.
- **[pagefind](skills/tooling/pagefind/SKILL.md)** — Use when configuring, indexing, or troubleshooting Pagefind static search for websites. Covers config options, CLI flags, and client integrations.
- **[prek](skills/tooling/prek/SKILL.md)** — Run pre-commit hooks using the prek CLI tool. Differentiates between 'pre-commit' (the git hooks concept/lifecycle stage) and 'prek' (the actual binary command; pre-commit CLI is deprecated, use prek). Use when you need to run, configure, check, or troubleshoot git pre-commit hooks.
- **[scratchpad](skills/tooling/scratchpad/SKILL.md)** — Keep one-off, experimental, or temporary scripts and files out of the source tree by writing them to a single, consistent session scratch location, then cleaning them up. Use when creating throwaway, experimental, or temporary scripts/files during a task.
- **[tailwind](skills/tooling/tailwind/SKILL.md)** — Use when writing Tailwind CSS utility classes, compiling Tailwind bundles, or configuring Tailwind CSS v4 CSS-first themes and variables.
- **[tauri](skills/tooling/tauri/SKILL.md)** — Expert reference and development guidelines for Tauri v2+ cross-platform desktop and mobile apps. Use when the user mentions Tauri, src-tauri, tauri v2, tauri.conf.json, or capabilities.json, or asks to build, modify, or debug Tauri application settings, IPC commands, or capabilities.
- **[zed](skills/tooling/zed/SKILL.md)** — Find, search, and manage Zed editor configuration settings, keybindings, and language server configurations. Use when configuring Zed settings, creating or updating ~/.config/zed/settings.json or .zed/settings.json, editing keymap.json, or searching for Zed options and documentation.
- **[zola](skills/tooling/zola/SKILL.md)** — Expert reference and development guidelines for the Zola static site generator. Use when the user asks to modify, build, serve, or customize Zola sites, themes, templates, shortcodes, or Zola configuration files (such as config.toml).
