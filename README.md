# Agent Skills

[![skills.sh](https://skills.sh/b/MAHDTech/agent-skills)](https://skills.sh/MAHDTech/agent-skills)

Working on my _skill issues_.

<img src="./docs/images/skill-issues.png" alt="skill issues" width="400">

These are my personal agent skills and attempt to be cross-compatible with Antigravity, Claude Code, Goose and OpenCode.

## Status

[![CI][badge-ci]][workflow-ci]
[![Cargo CRAP CI][badge-crap-ci]][workflow-crap]
[![Release Automation][badge-release]][workflow-release]
[![Dashboard Deployment][badge-deploy]][workflow-deploy]
[![SecOps][badge-secops]][workflow-secops]
[![License][badge-license]](LICENSE)

[badge-ci]: https://img.shields.io/github/actions/workflow/status/MAHDTech/agent-skills/ci.yaml?branch=trunk&event=push&label=CI&style=flat-square
[workflow-ci]: https://github.com/MAHDTech/agent-skills/actions/workflows/ci.yaml
[badge-crap-ci]: https://img.shields.io/github/actions/workflow/status/MAHDTech/agent-skills/cargo-crap.yaml?branch=trunk&event=push&label=Cargo%20CRAP%20CI&style=flat-square
[workflow-crap]: https://github.com/MAHDTech/agent-skills/actions/workflows/cargo-crap.yaml
[badge-release]: https://img.shields.io/github/actions/workflow/status/MAHDTech/agent-skills/release.yaml?branch=trunk&event=push&label=Release%20Automation&style=flat-square
[workflow-release]: https://github.com/MAHDTech/agent-skills/actions/workflows/release.yaml
[badge-deploy]: https://img.shields.io/github/actions/workflow/status/MAHDTech/agent-skills/deploy.yaml?branch=trunk&event=push&label=Dashboard&style=flat-square
[workflow-deploy]: https://github.com/MAHDTech/agent-skills/actions/workflows/deploy.yaml
[badge-secops]: https://img.shields.io/github/actions/workflow/status/MAHDTech/agent-skills/secops.yaml?branch=trunk&label=SecOps&style=flat-square
[workflow-secops]: https://github.com/MAHDTech/agent-skills/actions/workflows/secops.yaml
[badge-license]: https://img.shields.io/github/license/MAHDTech/agent-skills?style=flat-square

## Install

```bash
# Using npm
npx skills add MAHDTech/agent-skills

# Using Bun
bunx skills add MAHDTech/agent-skills
```

## CLI and TUI

The unified `ask` binary provides command-line catalog management and an interactive terminal interface.

```bash
# Launch interactive terminal user interface
ask

# Or explicitly launch the TUI
ask tui
```

### Global Options

- `-v, --verbose`: Increase logging verbosity.
- `-q, --quiet`: Suppress logging output.
- `-f, --format <FORMAT>`: Output format (plain, json, yaml, table).
- `-c, --config <PATH>`: Path to optional configuration file.
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

### Skills Commands

- `ask skills list`: List discovered skills in table, plain, json, or yaml formats.
- `ask skills show <NAME>`: Display skill metadata, arguments, and full instructions.
- `ask skills install <SOURCE>`: Install or symlink skills into detected agent environments.
- `ask skills uninstall <SKILL>`: Remove installed skill symlinks from target environments.
- `ask skills lint`: Validate frontmatter schema, file structure, and naming rules.
- `ask skills sync`: Reconcile tool symlinks and regenerate derived documentation.
- `ask skills download-resources`: Fetch external referenced resources across all skills.
- `ask skills clean-resources`: Remove downloaded resources to return to clean state.

### Dashboard Commands

- `ask dashboard summary`: Display catalog metrics, category counts, and tree distribution.
- `ask dashboard build`: Generate static documentation site with search index.
- `ask dashboard serve`: Launch local preview server and Tailwind watcher.
- `ask dashboard css`: Compile Tailwind CSS stylesheets.
- `ask dashboard lint`: Verify committed documentation matches generated artifacts.

### TUI View Modes

- `Explorer`: Navigate skills by category with live preview and filter support.
- `Inspector`: Inspect detailed metadata, frontmatter, and skill instructions.
- `Linter`: Review catalog validation issues and rule violations interactively.
- `Runner`: Execute skill workflows and catalog synchronization directly.

## Development

Enter the development shell or execute test suites using devenv:

```bash
# Enter interactive development shell
devenv --no-tui shell

# Run workspace test suite and pre-commit checks
devenv --no-tui test
```

## Documentation

- [dashboard](docs/dashboard.md)
- [install](docs/install.md)
- [usage](docs/usage.md)
- [vendor-skills](docs/vendor-skills.md)

## Available Skills

### Engineering

The core build, debug, and delivery loop.

- **[acp](skills/engineering/acp/SKILL.md)** - Build, integrate, or debug Agent Client Protocol agents and clients with official SDKs. Use for ACP sessions, tool calls, schemas, transports, or protocol migrations.
- **[casita](skills/engineering/casita/SKILL.md)** - Use for Casita CLI or Rust API work: immutable object storage, imports, roots and retention, verified sync, Casitar archives, integrity checks, or garbage collection.
- **[characterization-tests](skills/engineering/characterization-tests/SKILL.md)** - Capture existing behavior of untested or legacy code before a refactor or upgrade. Use for deterministic characterization tests or golden masters, including surprising current behavior.
- **[code-comments](skills/engineering/code-comments/SKILL.md)** - Write or review code comments for non-obvious intent, constraints, and invariants. Use to trim redundant or stale comments and audit comment quality in a specified scope or diff.
- **[codebase-design](skills/engineering/codebase-design/SKILL.md)** - Design or improve module interfaces and test seams. Use when simplifying caller requirements, reducing shallow abstractions, or making modules easier to test and maintain.
- **[diagnosing-bugs](skills/engineering/diagnosing-bugs/SKILL.md)** - Diagnose hard bugs, flaky failures, and performance regressions with a reproducible feedback loop. Use to isolate the cause, verify a fix, and add a meaningful regression test.
- **[domain-modeling](skills/engineering/domain-modeling/SKILL.md)** - Clarify domain terms and relationships while designing software. Use to resolve conflicting terminology, stress-test scenarios, update a glossary, or record a consequential ADR.
- **[egui](skills/engineering/egui/SKILL.md)** - Build or debug Rust interfaces using egui or eframe. Use for widget layout, custom painting, themes, retained state, or verifying egui API signatures.
- **[model-context-protocol](skills/engineering/model-context-protocol/SKILL.md)** - Build or debug MCP servers with official SDKs. Use for tool schemas, resources, prompts, lifecycle negotiation, stdio or HTTP transports, and MCP Inspector testing.
- **[nutanix-api-v4](skills/engineering/nutanix-api-v4/SKILL.md)** - Integrate or debug Nutanix v4 REST APIs and official SDKs. Use for OData queries, namespaces, version compatibility, or migrating legacy Nutanix API integrations.
- **[nutanix-enterprise-ai](skills/engineering/nutanix-enterprise-ai/SKILL.md)** - Configure or troubleshoot Nutanix Enterprise AI (NAI/GPT-in-a-Box). Use for platform deployment, model serving, inference gateways, RBAC, or NAI MCP connectors.
- **[nutanix-files](skills/engineering/nutanix-files/SKILL.md)** - Configure or troubleshoot Nutanix Files servers and SMB/NFS shares. Use for directory integration, Smart Tiering, recovery, analytics, performance, or Files APIs.
- **[nutanix-kubernetes-platform](skills/engineering/nutanix-kubernetes-platform/SKILL.md)** - Configure, operate, or troubleshoot Nutanix Kubernetes Platform (NKP), Konvoy, or Kommander. Use for NKP cluster lifecycle, image builds, air-gapped installs, or Insights.
- **[nutanix-objects](skills/engineering/nutanix-objects/SKILL.md)** - Configure or troubleshoot Nutanix Objects storage. Use for its S3 APIs, buckets, IAM, WORM policies, replication, cloud tiering, monitoring, or Objects Browser.
- **[optimize-performance](skills/engineering/optimize-performance/SKILL.md)** - Improve an existing hot path using benchmarks and profiling. Use for latency, throughput, or memory targets; route newly introduced slowdowns to regression diagnosis.
- **[productionize-app](skills/engineering/productionize-app/SKILL.md)** - Prepare an entire application for production when explicitly requested. Audit and fix configuration, reliability, security, observability, packaging, and deployment readiness.
- **[research](skills/engineering/research/SKILL.md)** - Research a technical question using primary sources and save cited findings in the repo. Use for documentation, API, specification, or source-code fact gathering.
- **[sculpt-code](skills/engineering/sculpt-code/SKILL.md)** - Refactor a scoped diff or selected files for clearer naming, control flow, structure, and less duplication or dead code. Use for cleanup that preserves behavior.
- **[spacetime](skills/engineering/spacetime/SKILL.md)** - Develop or debug Spacetime/SpacetimeDB modules and real-time clients in Rust, C#, TypeScript, or C++. Use for schemas, backend functions, subscriptions, CLI work, bindings, or migrations.
- **[tdd](skills/engineering/tdd/SKILL.md)** - Build features or fixes test-first using a red-to-green loop. Use for behavior-focused integration tests, choosing public test seams, and avoiding tests coupled to internals.
- **[upgrade-dependencies](skills/engineering/upgrade-dependencies/SKILL.md)** - Upgrade packages, frameworks, or language/runtime versions using release notes and a green test baseline. Use for version bumps, breaking migrations, and lockfile changes.
- **[xai](skills/engineering/xai/SKILL.md)** - Integrate or debug xAI/Grok APIs and SDKs. Use for client configuration, model capabilities, streaming, tool calling, structured output, vision, or search integration.

### Game Development

Game engines and game-development workflows.

- **[bevy-development](skills/game-development/bevy-development/SKILL.md)** - Build or modernize Bevy games and simulations in Rust. Use for ECS design, scheduling, input, assets, messages, performance, or migration between Bevy APIs.
- **[bevy-egui](skills/game-development/bevy-egui/SKILL.md)** - Integrate egui into Bevy with bevy_egui. Use for UI scheduling, HUDs, debug panels, input capture, image textures, multi-window contexts, or rendering UI to game surfaces.

### Planning

Turn ideas into specs, tickets, and multi-session plans.

- **[estimate-work](skills/planning/estimate-work/SKILL.md)** - Estimate a plan or ticket set using relative sizes or time ranges, reference history, named assumptions, and risk. Use when sizing work, forecasting delivery, or deciding whether uncertainty needs a spike.
- **[grilling](skills/planning/grilling/SKILL.md)** - Interview the user one decision at a time before coding. Use when explicitly asked to stress-test a design, or when an explicitly started software-factory or wayfinder workflow requires its planning interview.
- **[handoff](skills/planning/handoff/SKILL.md)** - Summarize the current conversation into a temporary handoff for another agent or session. Use when transferring unfinished work; reference durable artifacts, suggest relevant skills, and redact secrets.
- **[prioritize-backlog](skills/planning/prioritize-backlog/SKILL.md)** - Rank, cut, cluster, and sequence ideas or tickets by value, effort, risk, and dependencies. Use for generic backlog grooming with value-effort, RICE, or WSJF; excludes the automated .tars/issues backlog.
- **[software-factory](skills/planning/software-factory/SKILL.md)** - Guide a feature through Product, Architecture, Program Design, and Slice approval gates, then implement verified vertical slices. Use when asked for the full design-to-delivery factory workflow.
- **[store-plan](skills/planning/store-plan/SKILL.md)** - Save an agreed conversation plan, decisions, action items, and open questions to a reviewable Markdown file in docs/plans/. Use when a durable repository plan is needed rather than a temporary handoff.
- **[to-spec](skills/planning/to-spec/SKILL.md)** - Synthesize the current conversation into a product spec or PRD and publish it to the project's issue tracker. Use when asked to formalize discussed requirements and testing decisions without a new interview.
- **[to-tickets](skills/planning/to-tickets/SKILL.md)** - Split a plan or spec into approved vertical implementation tickets with blocking dependencies, then publish to local files or an issue tracker. Use for buildable slices, excluding .tars/issues automation.
- **[wayfinder](skills/planning/wayfinder/SKILL.md)** - Chart a large, ambiguous effort as an issue-tracker map of investigation tickets. Use when asked to map unknown decisions or work one frontier ticket per session, rather than slice known implementation work.

### Review

Review diffs, pull requests, and test plans.

- **[code-review](skills/review/code-review/SKILL.md)** - Review a diff against a fixed commit, branch, tag, or merge-base for repository standards and spec compliance. Use for branch or PR review with two parallel agents, reporting the axes separately.
- **[pr-build-context](skills/review/pr-build-context/SKILL.md)** - Prepare a briefing before reviewing a pull request: explain the diff, key files, risks, and questions for the author. Use for PR context, or a whole-repository overview when on its default branch.
- **[pr-create-test-plan](skills/review/pr-create-test-plan/SKILL.md)** - Draft TEST_PLAN.md for a branch's real user flows and integration changes using the project's commands. Use when a reviewer needs manual verification steps and a quick smoke test before merging.
- **[pr-edge-cases](skills/review/pr-edge-cases/SKILL.md)** - Probe branch changes, uncommitted work, or specified files for logic errors, missing tests, boundary cases, races, and integration failures. Use to identify what could break before a merge, with file-line findings.
- **[rfc-review](skills/review/rfc-review/SKILL.md)** - Review an RFC or design document before approval for problem clarity, relevant compliance concerns, security, and performance. Use to return only the three to five most consequential issues with concrete suggestions.

### GitHub

GitHub and git workflows via the gh CLI.

- **[gh-babysit-pr](skills/github/gh-babysit-pr/SKILL.md)** - Monitor an open pull request through CI checks, review bots, and base-branch updates with pr-wait. Use only when the user explicitly asks to watch, monitor, or babysit a PR.
- **[gh-create-issue](skills/github/gh-create-issue/SKILL.md)** - Create a structured GitHub issue from the current conversation with gh CLI. Use when asked to file an issue that preserves the problem, discussion, constraints, and agreed implementation plan.
- **[gh-create-pr](skills/github/gh-create-pr/SKILL.md)** - Draft a problem-first Conventional Commit PR title and description with secret scanning and REST fallback. Use when asked to create or update a PR after approval.
- **[gh-pr-comments](skills/github/gh-pr-comments/SKILL.md)** - Format and preflight-scan GitHub comments posted on a user's behalf with account-aware attribution. Use when posting PR comments, inline review notes, or thread replies.
- **[gh-release-notes](skills/github/gh-release-notes/SKILL.md)** - Curate release notes from commits and merged PRs since the last tag, then create a GitHub release. Use when asked to cut or publish a versioned release, with draft and prerelease options.
- **[gh-resolve-pr-comments](skills/github/gh-resolve-pr-comments/SKILL.md)** - Triage and address human and AI bot PR review comments, push back on false positives, and reply before resolving threads. Use when asked to handle PR feedback.
- **[gh-submit-review](skills/github/gh-submit-review/SKILL.md)** - Publish a GitHub PR review with verdict, inline findings, secret scanning, and human-account attribution. Use when asked to submit a completed PR review.
- **[gh-takeover](skills/github/gh-takeover/SKILL.md)** - Adopt and finish an inherited pull request, branch, or agent handoff while preserving contributor credit. Use only when the user explicitly asks to take over existing work.
- **[gh-triage](skills/github/gh-triage/SKILL.md)** - Triage GitHub issues and external PRs: categorize, verify claims, interview as needed, and post agent-ready briefs. Use when asked to change backlog states or run a maintainer triage session.
- **[git-resolve-conflicts](skills/github/git-resolve-conflicts/SKILL.md)** - Resolve existing merge, rebase, or cherry-pick conflicts using file context and both sides' intent. Use when conflicts block authorized work; stage confident fixes and escalate ambiguous behavior.
- **[git-update-branch](skills/github/git-update-branch/SKILL.md)** - Assess stale or diverged PRs for conceptual conflicts and update a branch against its base via rebase or merge. Use when asked to modernize or update a branch or PR.

### Reflection

Self-critique and review of your own work.

- **[critical-thinking](skills/reflection/critical-thinking/SKILL.md)** - Stress-test your immediately preceding response for hidden assumptions, weak evidence, logical gaps, and practical risks. Use to challenge your own advice before the user acts, in the response's language.
- **[scope-sweep](skills/reflection/scope-sweep/SKILL.md)** - Run a final breadth check after the core work and review to catch missed requirements, incomplete tasks, edge cases, and operational risks. Use before declaring a scope complete, with evidence for remaining gaps.
- **[self-review](skills/reflection/self-review/SKILL.md)** - Review your implementation diff for unfinished work, non-idiomatic code, duplication, and unnecessary complexity. Use after a task to apply focused fixes, with file-and-line evidence for each change.
- **[total-recall](skills/reflection/total-recall/SKILL.md)** - Maintain an append-only TSV decision trail and audit its evidence against the transcript. Use when explicitly requesting a persistent audit trail for autonomous work, multi-phase migrations, or risky refactors.
- **[wtf](skills/reflection/wtf/SKILL.md)** - Rewrite your own last response more briefly and plainly when the user signals confusion, such as "huh" or "in English". Use for confusion about your message, not requests to explain an error, output, or regex.

### Writing

Proofreading and documentation polish.

- **[proofread](skills/writing/proofread/SKILL.md)** - Proofread a post before publication for spelling, grammar, repetition, links, and weak reasoning. Use for error correction that preserves the author's voice; ask before logic changes or a skimmability pass.
- **[simplify-docs](skills/writing/simplify-docs/SKILL.md)** - Simplify a README, setup guide, or technical reference for fast scanning and execution. Use for structural documentation rewrites that lead with the shortest successful path, previewing edits for approval.
- **[unslop](skills/writing/unslop/SKILL.md)** - Edit writing to remove AI cliches, filler, robotic phrasing, and decorative formatting while preserving meaning and facts. Use when text needs a natural human voice, varied rhythm, and plain language.

### Authoring

Create and maintain the skills themselves.

- **[skill-archive](skills/authoring/skill-archive/SKILL.md)** - Retire a custom skill by archiving its files, preserving history, updating inbound references, and regenerating the catalog.
- **[skill-audit](skills/authoring/skill-audit/SKILL.md)** - Audit the custom skill catalog for invocation policies, descriptions, frontmatter, references, duplicates, and lifecycle drift.
- **[skill-creator](skills/authoring/skill-creator/SKILL.md)** - Create or update custom agent skills with concise discovery descriptions, matching invocation policies, supporting resources, and catalog checks.
- **[skill-router](skills/authoring/skill-router/SKILL.md)** - Browse the skill catalog and choose a workflow for a task, with a map of delivery stages and specialist skills grouped by topic.

### Tooling

Environments, CLIs, and agent conventions.

- **[agent-guidelines](skills/tooling/agent-guidelines/SKILL.md)** - Apply coding discipline before and during implementation: state assumptions, keep changes minimal and scoped, and verify behavior against concrete success criteria.
- **[antigravity](skills/tooling/antigravity/SKILL.md)** - Look up Google Antigravity/AGY CLI, IDE, and SDK behavior. Use for its settings, commands, keybindings, agent permissions, hooks, or plugin integration.
- **[cargo-crap](skills/tooling/cargo-crap/SKILL.md)** - Measure Rust change risk with cargo-crap and coverage data. Use for CRAP scores, cargo-llvm-cov integration, complexity regression gates, CI reports, or badges.
- **[cargo-doc](skills/tooling/cargo-doc/SKILL.md)** - Verify Rust crate APIs with local cargo doc output or docs.rs. Use to inspect exact signatures, methods, traits, and feature-gated items for the project version.
- **[devenv](skills/tooling/devenv/SKILL.md)** - Use when a repository has devenv.nix or devenv.yaml, or when running devenv commands. Follow its shell, dependency, SecretSpec, hook, and test-runner conventions.
- **[herdr](skills/tooling/herdr/SKILL.md)** - Operate Herdr panes, workspaces, or agent sessions when HERDR_ENV=1. Also use for Herdr setup, configuration, keybindings, and troubleshooting questions.
- **[install-skills](skills/tooling/install-skills/SKILL.md)** - Install, update, remove, or troubleshoot agent skill collections with the skills.sh CLI when explicitly requested, including runtime wiring and install verification.
- **[jj](skills/tooling/jj/SKILL.md)** - Use Jujutsu (jj) in repositories with .jj, or for jj-specific tasks. Inspect history, shape changes, write revsets/filesets, resolve conflicts, and manage bookmarks or remotes.
- **[nix-shell](skills/tooling/nix-shell/SKILL.md)** - Run missing ad-hoc CLI utilities in ephemeral nix shell environments when Nix is installed. Use for one-off diagnostics or conversions, keeping project dependencies in its toolchain.
- **[oak](skills/tooling/oak/SKILL.md)** - Use Oak version control in repositories with .oak, or for oak.space tasks. Manage change stacks, conflicts, workspaces, dependency mounts, spaces, and Oak CI.
- **[pagefind](skills/tooling/pagefind/SKILL.md)** - Configure, build, or troubleshoot Pagefind search for a static website. Use for indexing options, content exclusions or weighting, generated assets, and client integration.
- **[prek](skills/tooling/prek/SKILL.md)** - Run, configure, or troubleshoot Git pre-commit hooks with prek. Use for hook execution, installation, validation, and cache management; distinguish the hook stage from its CLI.
- **[scratchpad](skills/tooling/scratchpad/SKILL.md)** - Manage temporary scripts, experiment files, and debugging artifacts during a task. Use one session scratch directory, keep the source tree clean, and remove temporary files afterward.
- **[tailwind](skills/tooling/tailwind/SKILL.md)** - Style components or build CSS with Tailwind CSS v4. Use for utility classes, CSS-first theme variables, imports, and CLI stylesheet compilation.
- **[zed](skills/tooling/zed/SKILL.md)** - Configure or troubleshoot Zed editor settings, keybindings, and language servers. Use to verify Zed option names and update user or project settings in the correct scope.
- **[zola](skills/tooling/zola/SKILL.md)** - Build, serve, or customize an existing Zola site. Use for Tera templates, shortcodes, taxonomies, config.toml options, theme changes, or Zola build and link checks.

### TARS

Autonomous software factory and persistent goal workflows.

- **[tars-debug](skills/tars/tars-debug/SKILL.md)** - Debug and recover the tars-agy Antigravity plugin from outside an active run using tars-agy doctor, inspect, monitor, TARS state logs, and ~/.gemini session transcripts. Use for stuck or failing TARS sessions, hooks, spokes, or factory shifts.
- **[tars-goal](skills/tars/tars-goal/SKILL.md)** - Set or resume a persistent goal and pursue it until every requirement is verified or a blocker prevents further work. Use when the user invokes /tars-goal or explicitly requests a persistent goal.
- **[tars-run-factory](skills/tars/tars-run-factory/SKILL.md)** - Run the TARS factory unattended over a selected repository backlog through headless agy batches, peer review, and rework. Use when explicitly starting a bounded factory shift, with optional automated merging.

### Vendor Skills

#### cloudflare

Skills maintained by this vendor.

- **[agents-sdk](skills/vendor/cloudflare/agents-sdk/SKILL.md)** - Build, debug, or review Cloudflare Agents SDK applications using the agents package.
- **[basin](skills/vendor/cloudflare/basin/SKILL.md)** - Build and troubleshoot Cloudflare Basin analytics workflows with Basin Pipelines, Basin Catalog, and Basin SQL. Use for streaming data into R2 Iceberg tables, managing catalogs, or querying those tables; also use for requests using the former Data Platform, Pipelines, R2 Data Catalog, or R2 SQL names.
- **[cf-cli](skills/vendor/cloudflare/cf-cli/SKILL.md)** - Cloudflare's official cf CLI (beta) for the public Cloudflare API and Workers projects. Use for finding and running Cloudflare commands from the terminal — sign-in and profiles, cf cli search, cf schema, cf deploy/dev/build, resource commands for DNS, zones, D1, KV, and R2 — and for choosing between cf and Wrangler in a project.
- **[cloudflare](skills/vendor/cloudflare/cloudflare/SKILL.md)** - Discover and choose Cloudflare products for apps, APIs, AI agents, storage, networking, and security. Use for architecture and product selection, including when the user describes a need without naming a Cloudflare product; then find the relevant skill or documentation.
- **[cloudflare-email-service](skills/vendor/cloudflare/cloudflare-email-service/SKILL.md)** - Implement or troubleshoot Cloudflare Email Sending and Email Routing integrations and their delivery configuration.
- **[cloudflare-one](skills/vendor/cloudflare/cloudflare-one/SKILL.md)** - Design, configure, troubleshoot, or review Cloudflare One Zero Trust and SASE deployments. Use cloudflare-one-migrations for migration planning from other vendors.
- **[cloudflare-one-migrations](skills/vendor/cloudflare/cloudflare-one-migrations/SKILL.md)** - Assess and plan migrations from existing VPN, SWG, or SASE platforms to Cloudflare One, including policy mapping, parity gaps, and rollout.
- **[durable-objects](skills/vendor/cloudflare/durable-objects/SKILL.md)** - Build, debug, or review Cloudflare Durable Objects code for persistent state and coordination.
- **[gateway-workers](skills/vendor/cloudflare/gateway-workers/SKILL.md)** - Build, debug, or review Cloudflare Workers invoked by the Forward action in Gateway HTTP policies, including the GatewayMiddleware entrypoint, Gateway types, context, and continuation.
- **[k2](skills/vendor/cloudflare/k2/SKILL.md)** - Build and troubleshoot Cloudflare K2 or K2 Streams durable logs. Use for stream setup, producing from Workers or HTTP, configuring retention and inputs, and consuming through subscriptions.
- **[nextjs-on-cloudflare](skills/vendor/cloudflare/nextjs-on-cloudflare/SKILL.md)** - Build, migrate, and deploy Next.js apps on Cloudflare Workers with vinext. Use when starting a Next.js project on Cloudflare, moving an existing app to Workers, choosing between vinext and OpenNext, or setting up vinext for Workers. For setup, migration, or deployment, install vinext's upstream skills with `npx skills add cloudflare/vinext` if missing, then read and follow the applicable skill and docs.
- **[sandbox-migrate-to-next](skills/vendor/cloudflare/sandbox-migrate-to-next/SKILL.md)** - Migrate Cloudflare Sandbox apps from stable @cloudflare/sandbox to @cloudflare/sandbox@next (SDK 1.0 preview). Use sandbox-next for apps already on the preview.
- **[sandbox-next](skills/vendor/cloudflare/sandbox-next/SKILL.md)** - Build or maintain Cloudflare Sandbox apps on @cloudflare/sandbox@next (SDK 1.0 preview). Use sandbox-migrate-to-next when porting a stable app.
- **[sandbox-stable](skills/vendor/cloudflare/sandbox-stable/SKILL.md)** - Build or maintain Cloudflare Sandbox apps on the stable @cloudflare/sandbox package. Use sandbox-next for preview apps and sandbox-migrate-to-next for stable-to-preview migrations.
- **[turnstile-spin](skills/vendor/cloudflare/turnstile-spin/SKILL.md)** - Set up, repair, or migrate to Cloudflare Turnstile bot verification in an existing frontend and backend, including server-side Siteverify.
- **[web-perf](skills/vendor/cloudflare/web-perf/SKILL.md)** - Audit, diagnose, or optimize website loading and interaction performance, Core Web Vitals, and Lighthouse performance scores.
- **[workers-best-practices](skills/vendor/cloudflare/workers-best-practices/SKILL.md)** - Cloudflare Workers best practices for production applications. Use when writing, reviewing, or configuring Workers.
- **[workers-profiling](skills/vendor/cloudflare/workers-profiling/SKILL.md)** - Profile or debug CPU usage and memory allocation in deployed Cloudflare Workers and Durable Objects. Then use this information to optimize your code.
- **[wrangler](skills/vendor/cloudflare/wrangler/SKILL.md)** - Run or troubleshoot Wrangler CLI commands in projects that have a wrangler.jsonc, wrangler.json, or wrangler.toml file, including local development, Previews, deployment, and resource management. For new projects and projects without a Wrangler configuration file, use the cf CLI instead.

#### spacetimedb

Skills maintained by this vendor.

- **[cli](skills/vendor/spacetimedb/cli/SKILL.md)** - SpacetimeDB CLI reference for initializing projects, building modules, publishing databases, querying data, and managing servers
- **[concepts](skills/vendor/spacetimedb/concepts/SKILL.md)** - Understand SpacetimeDB architecture and core concepts. Use when learning SpacetimeDB or making architectural decisions.
- **[cpp-server](skills/vendor/spacetimedb/cpp-server/SKILL.md)** - SpacetimeDB C++ server module SDK reference. Use when writing tables, reducers, or module logic in C++.
- **[csharp-client](skills/vendor/spacetimedb/csharp-client/SKILL.md)** - SpacetimeDB C#/.NET client SDK reference. Use when building C# clients that connect to SpacetimeDB (console, desktop, or any .NET app).
- **[csharp-server](skills/vendor/spacetimedb/csharp-server/SKILL.md)** - SpacetimeDB C# server module SDK reference. Use when writing tables, reducers, or module logic in C#.
- **[mcp](skills/vendor/spacetimedb/mcp/SKILL.md)** - Operate a running SpacetimeDB database through MCP tools rather than the CLI - list databases, read schemas, run SQL, and call reducers. Use when the client exposes spacetimedb MCP tools and the task is to inspect or change data in a live database.
- **[rust-server](skills/vendor/spacetimedb/rust-server/SKILL.md)** - SpacetimeDB Rust server module SDK reference. Use when writing tables, reducers, or module logic in Rust.
- **[typescript-client](skills/vendor/spacetimedb/typescript-client/SKILL.md)** - SpacetimeDB TypeScript/React client SDK reference. Use when building web clients that connect to SpacetimeDB.
- **[typescript-server](skills/vendor/spacetimedb/typescript-server/SKILL.md)** - SpacetimeDB TypeScript server module SDK reference. Use when writing tables, reducers, or module logic in TypeScript.
- **[unity](skills/vendor/spacetimedb/unity/SKILL.md)** - Integrate SpacetimeDB with Unity game projects. Use when building Unity clients with MonoBehaviour lifecycle, FrameTick, and PlayerPrefs token persistence.
- **[unreal](skills/vendor/spacetimedb/unreal/SKILL.md)** - SpacetimeDB Unreal Engine client SDK reference. Use when building Unreal Engine clients that connect to SpacetimeDB.
