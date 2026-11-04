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
- **[agent-router](skills/engineering/agent-router/SKILL.md)** - Configure, route, or troubleshoot AgentRouter AI gateways, unified LLM endpoints, and MCP or A2A servers, including the gateway powering Nutanix Enterprise AI (NAI).
- **[casita](skills/engineering/casita/SKILL.md)** - Use for Casita CLI or Rust API work: immutable object storage, imports, roots and retention, verified sync, Casitar archives, integrity checks, or garbage collection.
- **[characterization-tests](skills/engineering/characterization-tests/SKILL.md)** - Capture existing behavior of untested or legacy code before a refactor or upgrade. Use for deterministic characterization tests or golden masters, including surprising current behavior.
- **[code-comments](skills/engineering/code-comments/SKILL.md)** - Write or review code comments for non-obvious intent, constraints, and invariants. Use to trim redundant or stale comments and audit comment quality in a specified scope or diff.
- **[codebase-design](skills/engineering/codebase-design/SKILL.md)** - Design or improve module interfaces and test seams. Use when simplifying caller requirements, reducing shallow abstractions, or making modules easier to test and maintain.
- **[diagnosing-bugs](skills/engineering/diagnosing-bugs/SKILL.md)** - Diagnose hard bugs, flaky failures, and performance regressions with a reproducible feedback loop. Use to isolate the cause, verify a fix, and add a meaningful regression test.
- **[domain-modeling](skills/engineering/domain-modeling/SKILL.md)** - Clarify domain terms and relationships while designing software. Use to resolve conflicting terminology, stress-test scenarios, update a glossary, or record a consequential ADR.
- **[egui](skills/engineering/egui/SKILL.md)** - Build or debug Rust interfaces using egui or eframe. Use for widget layout, custom painting, themes, retained state, or verifying egui API signatures.
- **[icp](skills/engineering/icp/SKILL.md)** - Build, secure, test, and deploy Internet Computer (ICP) canisters and full-stack dapps in Motoko, Rust, and TypeScript. Use for canister architecture, stable memory, ICRC and chain-key tokens (ckBTC, ckETH, ckSOL), Internet Identity, vetKeys, HTTPS outcalls, or icp CLI workflows.
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
- **[prioritize-backlog](skills/planning/prioritize-backlog/SKILL.md)** - Rank, cut, cluster, and sequence ideas or tickets by value, effort, risk, and dependencies. Use for generic backlog grooming with value-effort, RICE, or WSJF; excludes the automated .guards/issues backlog.
- **[software-factory](skills/planning/software-factory/SKILL.md)** - Guide a feature through Product, Architecture, Program Design, and Slice approval gates, then implement verified vertical slices. Use when asked for the full design-to-delivery factory workflow.
- **[store-plan](skills/planning/store-plan/SKILL.md)** - Save an agreed conversation plan, decisions, action items, and open questions to a reviewable Markdown file in docs/plans/. Use when a durable repository plan is needed rather than a temporary handoff.
- **[to-spec](skills/planning/to-spec/SKILL.md)** - Synthesize the current conversation into a product spec or PRD and publish it to the project's issue tracker. Use when asked to formalize discussed requirements and testing decisions without a new interview.
- **[to-tickets](skills/planning/to-tickets/SKILL.md)** - Split a plan or spec into approved vertical implementation tickets with blocking dependencies, then publish to local files or an issue tracker. Use for buildable slices, excluding .guards/issues automation.
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

### GUARDS

Autonomous GUARDS software factory, debugging, and persistent goal workflows.

- **[guards-debug](skills/guards/guards-debug/SKILL.md)** - Debug and recover the agy-guards Antigravity plugin from outside an active run using agy-guards doctor, inspect, monitor, GUARDS state logs, and ~/.gemini session transcripts. Use for stuck or failing GUARDS sessions, hooks, spokes, or factory shifts.
- **[guards-goal](skills/guards/guards-goal/SKILL.md)** - Set or resume a persistent goal and pursue it until every requirement is verified or a blocker prevents further work. Use when the user invokes /guards-goal or explicitly requests a persistent goal.
- **[guards-run-factory](skills/guards/guards-run-factory/SKILL.md)** - Run the GUARDS factory unattended over a selected repository backlog through headless agy batches, peer review, and rework. Use when explicitly starting a bounded factory shift, with optional automated merging.

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

#### splunk

Skills maintained by this vendor.

- **[alerting-and-notable-workflows](skills/vendor/splunk/alerting-and-notable-workflows/SKILL.md)** - Give cited, advisory-only Splunk alert, scheduled-report action, Enterprise Security finding/notable, and risk-workflow readiness guidance from supplied evidence. Use for schedule, trigger, suppression, action, recipient, downstream, ownership, escalation, entitlement, and post-change validation decisions. Route SPL authoring and execution to Splunk Search, broad permissions and object lifecycle to Knowledge Object Governance, and active service incidents to platform operations.
- **[app-and-add-on-lifecycle-advisor](skills/vendor/splunk/app-and-add-on-lifecycle-advisor/SKILL.md)** - Give cited, advisory-only Splunk app and add-on lifecycle guidance and assess supplied compatibility, installation, upgrade, validation, deprecation, migration, and removal evidence. Use when a Splunk Cloud Platform or Splunk Enterprise administrator needs packaging or AppInspect guidance, environment-specific readiness classification, a non-mutating lifecycle plan, or safe-removal review for a named app/add-on and Splunk version. Route fact-only metadata lookup, platform upgrade execution, fleet rollout, vulnerability remediation, and knowledge-object governance beyond removal-impact checks to their owning workflows.
- **[custom-visualization-builder](skills/vendor/splunk/custom-visualization-builder/SKILL.md)** - Scaffold, build, package, and install a custom visualization into Splunk using the dashboard-studio-extension framework. Use when the user wants to create a new custom viz, add a visualization to an existing project, or migrate a legacy custom viz.
- **[dashboard-report-alert-performance-advisor](skills/vendor/splunk/dashboard-report-alert-performance-advisor/SKILL.md)** - Assess latency, queueing, timeouts, missed or duplicate schedules, dispatch and concurrency pressure, refresh overlap, and data-source fan-out across existing Splunk dashboards, reports, and alerts without changing them. Use when object load, freshness, scheduled execution, or downstream timing is degraded and the requester needs an evidence-bounded performance finding, product and provider boundary, validation plan, and owner handoff. Route one-search SPL cost, structural authoring, functional object repair, acceleration, and deployment remediation to their owners.
- **[data-model-and-search-acceleration](skills/vendor/splunk/data-model-and-search-acceleration/SKILL.md)** - Assess Splunk data-model and report-acceleration readiness, summary coverage, and troubleshooting evidence without changing a deployment. Use for data-model object readiness, acceleration enablement, build or lag state, incomplete summaries, report acceleration, summary sharing, and choosing whether an acceleration mechanism fits a repeated search objective. Require validated field and CIM prerequisites, preserve ownership and deployment boundaries, and route SPL, normalization, governance, one-search tuning, and active platform incidents to their owners.
- **[data-source-onboarding-advisor](skills/vendor/splunk/data-source-onboarding-advisor/SKILL.md)** - Orchestrate a new Splunk data source from requirements and supported collection-method selection through source, sourcetype, host, timestamp, event-boundary, indexed-event, ownership, and acceptance validation. Use for Splunk Cloud Platform or Splunk Enterprise source intake, applicability decisions, readiness, acceptance contracts, and exact implementation handoffs; do not use for endpoint setup, ingest troubleshooting, fleet rollout, pipeline architecture, field or CIM implementation, configuration, deployment, mutation, or remediation.
- **[deployment-server-and-forwarder-fleet-management](skills/vendor/splunk/deployment-server-and-forwarder-fleet-management/SKILL.md)** - Explain, plan, and diagnose Splunk Enterprise Deployment Server and 10.x Agent Management fleet behavior from public documentation and sanitized evidence. Use for terminology, deployment apps, server classes, client filters, phone-home, effective assignment, rollout verification, cache or reload behavior, scale tuning, fleet visibility, and Deployment Server delivery of Splunk Remote Upgrader content; do not use for unrelated forwarder data flow, HEC, cluster bundle/deployer work, or live mutations.
- **[field-extraction-and-cim-mapping](skills/vendor/splunk/field-extraction-and-cim-mapping/SKILL.md)** - Author, explain, diagnose, and validate Splunk search-time field extractions and mappings to Common Information Model (CIM) datasets from representative events, configuration, and search evidence. Use for automatic key-value extraction, regex or delimiter extraction, props.conf EXTRACT and REPORT/transforms.conf rules, SPL extraction commands, aliases, calculated fields, lookups, event types, tags, value normalization, CIM field mapping, and missing or incorrect normalization; do not use for deployment execution, ingestion transport, app installation, knowledge-object governance, data-model acceleration, or unrelated search/dashboard repair.
- **[forwarder-and-data-ingest-doctor](skills/vendor/splunk/forwarder-and-data-ingest-doctor/SKILL.md)** - Diagnose Splunk forwarder and data-ingest paths from sanitized, read-only evidence. Use for Universal or Heavy Forwarder transport, deployment-client effects on ingestion, HEC and other inputs, parsing and timestamps, event boundaries, routing, sourcetype or index mismatches, missing, delayed, duplicated, or dropped events, and queues or backpressure in Splunk Cloud Platform or Splunk Enterprise; do not use for live mutation, broad architecture, fleet policy, HEC token administration, search-time extraction, or provider-owned remediation.
- **[hec-setup-and-troubleshooting](skills/vendor/splunk/hec-setup-and-troubleshooting/SKILL.md)** - Set up and validate Splunk HTTP Event Collector (HEC), explain indexer acknowledgment and distributed HEC behavior, diagnose HEC no-data and HTTP delivery failures from sanitized evidence, and prepare bounded escalation handoffs. Use for Splunk Cloud Platform or Splunk Enterprise HEC tokens, endpoints, event or raw payloads, TLS, channels, ACK, health, authorization, queues, and delivery verification; do not use for non-HEC ingestion, broad architecture, allowlist changes, or service-side remediation.
- **[incident-diagnosis-specialist](skills/vendor/splunk/incident-diagnosis-specialist/SKILL.md)** - Diagnose post-triage, multi-component Splunk incidents from a supplied privacy-reviewed diagnostic packet. Use to align clocks and symptoms, preserve impact and contradictions, map dependencies, rank and test competing causal hypotheses, request the smallest safe read-only discriminator, validate bounded findings, and hand confirmed component failures to exact specialists. Do not use to collect raw diagnostics, troubleshoot or remediate one component, configure, restart, mutate, stress test, or claim an unverified cause or recovery.
- **[index-and-storage-management-advisor](skills/vendor/splunk/index-and-storage-management-advisor/SKILL.md)** - Design and assess Splunk index, retention, capacity, archive, storage-tier, bucket-lifecycle, and SmartStore plans from supplied requirements and evidence without changing a deployment. Use for index placement, searchable and archive windows, growth and capacity models, storage choices, restore planning, SmartStore readiness, and evidence-backed change validation. Route active incidents and provider-owned operations to their operational owners.
- **[indexer-cluster-health-and-troubleshooting](skills/vendor/splunk/indexer-cluster-health-and-troubleshooting/SKILL.md)** - Assess Splunk indexer-cluster membership, factor and searchability state, bucket fix-up evidence, cluster-bundle distribution, rolling-maintenance readiness, multisite behavior, and SmartStore interactions without changing a deployment. Use for cluster-manager or peer health, unmet replication or search factors, incomplete searchability, persistent fix-up, cluster-bundle state, rolling readiness, and cluster-specific incident routing. Preserve Cloud, Enterprise, evidence-collection, storage-design, upgrade-planning, and active-operations boundaries.
- **[ingestion-pipeline-design](skills/vendor/splunk/ingestion-pipeline-design/SKILL.md)** - Design implementation-ready Splunk ingestion pipelines from known source requirements. Use for input and collector choice, processor placement, ingestion-time parsing, transformation, filtering, routing, capacity, reliability, security, validation, ownership, and implementation handoff for Splunk Cloud Platform or Splunk Enterprise. Do not use for source-onboarding discovery, production configuration, SPL, HEC endpoint work, fleet rollout, deployment, mutation, or active incident remediation.
- **[knowledge-object-governance](skills/vendor/splunk/knowledge-object-governance/SKILL.md)** - Give cited public Splunk knowledge-object governance guidance and assess user-authorized inventory, ownership, orphan, ACL, naming, lifecycle, lookup, and search-head-cluster comparison evidence without changing a deployment. Use for shared lookups, sourcetypes, saved searches, macros, field extractions, aliases, props/transforms, CIM mappings, dashboards, reports, and related objects when an administrator needs a read-only hygiene report, safe review plan, or boundary route.
- **[license-and-capacity-planning-advisor](skills/vendor/splunk/license-and-capacity-planning-advisor/SKILL.md)** - Separate Splunk Enterprise entitlement lifecycle issues from measured license-capacity demand and produce an evidence-bounded, advisory-only plan grounded in current official Splunk documentation.
- **[report-authoring-specialist](skills/vendor/splunk/report-authoring-specialist/SKILL.md)** - Define evidence-bounded Splunk report contracts and readiness findings for scheduled and ad hoc reports without creating, running, scheduling, editing, or sending them. Use when a requester needs the business question, search-definition handoff, time range and timezone, cadence and schedule window, recipients and formats, permission prerequisites, acceptance evidence, Cloud or Enterprise ownership, or a report-specific incident handoff. Route SPL to Splunk Search, broad ownership and ACL work to Knowledge Object Governance, action delivery to Alerting and Notable Workflows, acceleration to Data Model and Search Acceleration, and active platform incidents to platform operations.
- **[search-and-dashboard-troubleshooter](skills/vendor/splunk/search-and-dashboard-troubleshooter/SKILL.md)** - Diagnose broken Splunk searches, reports, alerts, and dashboards from supplied evidence. Use when an existing search or saved object is empty, missing, wrong, inconsistent, invisible, not dispatching, not triggering, or not rendering and the user needs the first unsupported functional stage, the smallest next evidence, aligned read-only validation, and an exact authoring, governance, field, workflow, or performance handoff. Do not use for SPL, object authoring, implementation, optimization, configuration, mutation, or remediation.
- **[search-head-cluster-health-and-troubleshooting](skills/vendor/splunk/search-head-cluster-health-and-troubleshooting/SKILL.md)** - Assess Splunk Search Head Cluster captaincy, elections and Raft state, member health, configuration and artifact replication, scheduler and search availability, KV Store health and recovery boundaries, deployer bundle phases, member drift, and rolling-change readiness without changing a deployment. Use for SHC captain/member instability, replication inconsistency, KV Store symptoms, deployer propagation failures, rolling readiness, and SHC-specific incident routing. Preserve Cloud, Enterprise, evidence-collection, platform, search, app, knowledge-object, upgrade, and indexer-cluster boundaries.
- **[search-performance-optimizer](skills/vendor/splunk/search-performance-optimizer/SKILL.md)** - Diagnose and improve one existing functional Splunk search from supplied SPL and runtime evidence. Use when a search, report, dashboard panel, or scheduled search is slow, queued, expensive, resource-intensive, or prematurely finalized and the user needs evidence-backed query tuning, acceleration-fit analysis, workload separation, or a comparable before-and-after plan. Route new-search authoring, functional break/fix, governance, and deployment-wide operations to their owning workflows.
- **[splunk-cloud-admin-copilot](skills/vendor/splunk/splunk-cloud-admin-copilot/SKILL.md)** - Read Splunk Cloud Platform ACS state, assess maintenance or restart readiness without changing it, and execute one explicitly approved IPv4 CIDR add or remove for one feature-specific IP allowlist through the documented public ACS provider. Use when a Cloud admin needs exact-target preflight, a minimal allowlist mutation, readback, rollback, and a sanitized receipt; route every other administration write and specialist domain.
- **[splunk-dashboard-converter](skills/vendor/splunk/splunk-dashboard-converter/SKILL.md)** - Convert classic Splunk Simple XML dashboards (version 1) into Dashboard Studio (version 2). Takes classic Simple XML as input, preserves every SPL query verbatim, and returns the Studio JSON definition to the caller. Use when the user asks to convert, migrate, upgrade, modernize, port, or make a v2 / Dashboard Studio version of an existing classic Splunk dashboard, form, or Simple XML view.
- **[splunk-enterprise-administration-advisor](skills/vendor/splunk/splunk-enterprise-administration-advisor/SKILL.md)** - Provide cited, read-only Splunk Enterprise administration guidance for routine local users, roles, capabilities, configuration precedence, service ownership, maintenance readiness, and safe non-mutating validation. Use when an administrator needs to understand documented Enterprise behavior, compare supplied source-file, btool, or runtime observations, identify accountable role owners, or decide what evidence is missing before a safe change. Route identity-provider authentication, knowledge-object governance, index/storage, app lifecycle, cluster operations, upgrade/security, health/diag, platform incidents, license/capacity, and deployment-server or forwarder-fleet work to their owning skills.
- **[splunk-health-monitoring-and-diagnostic-collection](skills/vendor/splunk/splunk-health-monitoring-and-diagnostic-collection/SKILL.md)** - Answer cited questions about Splunk Cloud Monitoring Console, Splunk Enterprise Monitoring Console, splunkd health reports, health dashboards, health.log, and health endpoints; collect and normalize health evidence; guide privacy-aware diag and RapidDiag collection; and interpret supplied health signals into bounded hypotheses and support handoffs. Use for Splunk Cloud Platform or Splunk Enterprise deployment-health signals and diagnostic artifacts, not broad incident root-cause analysis, HEC-specific troubleshooting, general SPL execution, cluster remediation, uploads, tickets, or environment changes.
- **[splunk-identity-saml-readiness-advisor](skills/vendor/splunk/splunk-identity-saml-readiness-advisor/SKILL.md)** - Research current public Splunk sources and use optional existing-auth read-only stack evidence to diagnose SAML, LDAP, roles, capabilities, group mappings, login failures, and access readiness without changing identity configuration or handling credentials.
- **[splunk-product-question-navigator](skills/vendor/splunk/splunk-product-question-navigator/SKILL.md)** - Research and answer current Splunk product questions from public sources with explicit product, deployment, version, freshness, and evidence boundaries. Use for explanatory questions such as what a feature does, where it is available, which edition or version supports it, whether two products or versions are compatible, or what changed. Route live incidents, stack changes, SPL execution, account-specific decisions, and unpublished roadmap questions to their owning workflow.
- **[splunk-search](skills/vendor/splunk/splunk-search/SKILL.md)** - Run bounded, read-only Splunk SPL searches through splunkctl and return compact, evidence-backed results without exposing credentials or flooding context.
- **[splunk-setup-page-builder](skills/vendor/splunk/splunk-setup-page-builder/SKILL.md)** - Build a certification-compliant first-run setup page for a Splunk app or add-on that collects non-secret settings via the configurations REST endpoint and secrets via storage/passwords, wires app.conf/default.meta correctly, and avoids the deprecated setup.xml mechanism. Use when the user wants a "Set up your app" screen, first-run configuration wizard, or a way to collect an endpoint/credential/API token for a Splunkbase app or TA. Route general app packaging, compatibility, upgrade, or removal lifecycle guidance unrelated to the setup page itself to app-and-add-on-lifecycle-advisor, and broader knowledge-object governance beyond this app's own default.meta to knowledge-object-governance.
- **[upgrade-and-security-readiness-router](skills/vendor/splunk/upgrade-and-security-readiness-router/SKILL.md)** - Classify Splunk upgrade, advisory, vulnerability, compliance, and post-change incident requests; preserve the minimum applicability and ownership context; ask one route-changing question when necessary; and hand each outcome to the exact public-product, upgrade-readiness, vulnerability/compliance, platform-operations, Cloud-administration, app-lifecycle, or provider owner. This skill routes only. It does not answer product or advisory questions, assess exposure, plan or execute changes, implement controls, or troubleshoot incidents.
- **[upgrade-and-vulnerability-readiness-advisor](skills/vendor/splunk/upgrade-and-vulnerability-readiness-advisor/SKILL.md)** - Assess cited Splunk Enterprise and Splunk Cloud version, maintenance, and security-patch readiness. Use when a planned version change or Splunk security advisory needs one evidence-labeled applicability and readiness decision covering compatibility, prerequisites, sequencing, change control, recovery, validation, and Cloud-versus-Enterprise ownership without executing the change.
- **[upgrade-planning-and-execution-readiness](skills/vendor/splunk/upgrade-planning-and-execution-readiness/SKILL.md)** - Build cited, evidence-labeled Splunk Enterprise upgrade plans and Splunk Cloud support-assisted version-change readiness plans without performing the upgrade. Use for upgrade scope intake, supported-path and prerequisite research, dependency compatibility, prechecks, backups, sequencing, maintenance-window readiness, rollback or recovery posture, go/no-go criteria, rehearsal, and post-upgrade validation planning.
- **[vulnerability-remediation-and-compliance-readiness](skills/vendor/splunk/vulnerability-remediation-and-compliance-readiness/SKILL.md)** - Assess Splunk-related advisories, CVEs, scanner or package findings, remediation and exception evidence, and vulnerability or compliance readiness. Use when Splunk administrators, security operators, compliance owners, or reviewers need an evidence-backed environment exposure decision, remediation or exception plan, reviewer-ready pass/fail/unknown assessment, or cited guidance for documented Splunk vulnerability and compliance surfaces.
