---
name: agents
description: Single source of truth for agent instructions
---

# Available Skills

## Engineering

- **acp**: Build, integrate, or debug Agent Client Protocol agents and clients with official SDKs. Use for ACP sessions, tool calls, schemas, transports, or protocol migrations.
- **casita**: Use for Casita CLI or Rust API work: immutable object storage, imports, roots and retention, verified sync, Casitar archives, integrity checks, or garbage collection.
- **characterization-tests**: Capture existing behavior of untested or legacy code before a refactor or upgrade. Use for deterministic characterization tests or golden masters, including surprising current behavior.
- **code-comments**: Write or review code comments for non-obvious intent, constraints, and invariants. Use to trim redundant or stale comments and audit comment quality in a specified scope or diff.
- **codebase-design**: Design or improve module interfaces and test seams. Use when simplifying caller requirements, reducing shallow abstractions, or making modules easier to test and maintain.
- **diagnosing-bugs**: Diagnose hard bugs, flaky failures, and performance regressions with a reproducible feedback loop. Use to isolate the cause, verify a fix, and add a meaningful regression test.
- **domain-modeling**: Clarify domain terms and relationships while designing software. Use to resolve conflicting terminology, stress-test scenarios, update a glossary, or record a consequential ADR.
- **egui**: Build or debug Rust interfaces using egui or eframe. Use for widget layout, custom painting, themes, retained state, or verifying egui API signatures.
- **model-context-protocol**: Build or debug MCP servers with official SDKs. Use for tool schemas, resources, prompts, lifecycle negotiation, stdio or HTTP transports, and MCP Inspector testing.
- **nutanix-api-v4**: Integrate or debug Nutanix v4 REST APIs and official SDKs. Use for OData queries, namespaces, version compatibility, or migrating legacy Nutanix API integrations.
- **nutanix-enterprise-ai**: Configure or troubleshoot Nutanix Enterprise AI (NAI/GPT-in-a-Box). Use for platform deployment, model serving, inference gateways, RBAC, or NAI MCP connectors.
- **nutanix-files**: Configure or troubleshoot Nutanix Files servers and SMB/NFS shares. Use for directory integration, Smart Tiering, recovery, analytics, performance, or Files APIs.
- **nutanix-kubernetes-platform**: Configure, operate, or troubleshoot Nutanix Kubernetes Platform (NKP), Konvoy, or Kommander. Use for NKP cluster lifecycle, image builds, air-gapped installs, or Insights.
- **nutanix-objects**: Configure or troubleshoot Nutanix Objects storage. Use for its S3 APIs, buckets, IAM, WORM policies, replication, cloud tiering, monitoring, or Objects Browser.
- **optimize-performance**: Improve an existing hot path using benchmarks and profiling. Use for latency, throughput, or memory targets; route newly introduced slowdowns to regression diagnosis.
- **productionize-app**: Prepare an entire application for production when explicitly requested. Audit and fix configuration, reliability, security, observability, packaging, and deployment readiness.
- **research**: Research a technical question using primary sources and save cited findings in the repo. Use for documentation, API, specification, or source-code fact gathering.
- **sculpt-code**: Refactor a scoped diff or selected files for clearer naming, control flow, structure, and less duplication or dead code. Use for cleanup that preserves behavior.
- **spacetime**: Develop or debug Spacetime/SpacetimeDB modules and real-time clients in Rust, C#, TypeScript, or C++. Use for schemas, backend functions, subscriptions, CLI work, bindings, or migrations.
- **tdd**: Build features or fixes test-first using a red-to-green loop. Use for behavior-focused integration tests, choosing public test seams, and avoiding tests coupled to internals.
- **upgrade-dependencies**: Upgrade packages, frameworks, or language/runtime versions using release notes and a green test baseline. Use for version bumps, breaking migrations, and lockfile changes.
- **xai**: Integrate or debug xAI/Grok APIs and SDKs. Use for client configuration, model capabilities, streaming, tool calling, structured output, vision, or search integration.

## Game Development

- **bevy-development**: Build or modernize Bevy games and simulations in Rust. Use for ECS design, scheduling, input, assets, messages, performance, or migration between Bevy APIs.
- **bevy-egui**: Integrate egui into Bevy with bevy_egui. Use for UI scheduling, HUDs, debug panels, input capture, image textures, multi-window contexts, or rendering UI to game surfaces.

## Planning

- **estimate-work**: Estimate a plan or ticket set using relative sizes or time ranges, reference history, named assumptions, and risk. Use when sizing work, forecasting delivery, or deciding whether uncertainty needs a spike.
- **grilling**: Interview the user one decision at a time before coding. Use when explicitly asked to stress-test a design, or when an explicitly started software-factory or wayfinder workflow requires its planning interview.
- **handoff**: Summarize the current conversation into a temporary handoff for another agent or session. Use when transferring unfinished work; reference durable artifacts, suggest relevant skills, and redact secrets.
- **prioritize-backlog**: Rank, cut, cluster, and sequence ideas or tickets by value, effort, risk, and dependencies. Use for generic backlog grooming with value-effort, RICE, or WSJF; excludes the automated .tars/issues backlog.
- **software-factory**: Guide a feature through Product, Architecture, Program Design, and Slice approval gates, then implement verified vertical slices. Use when asked for the full design-to-delivery factory workflow.
- **store-plan**: Save an agreed conversation plan, decisions, action items, and open questions to a reviewable Markdown file in docs/plans/. Use when a durable repository plan is needed rather than a temporary handoff.
- **to-spec**: Synthesize the current conversation into a product spec or PRD and publish it to the project's issue tracker. Use when asked to formalize discussed requirements and testing decisions without a new interview.
- **to-tickets**: Split a plan or spec into approved vertical implementation tickets with blocking dependencies, then publish to local files or an issue tracker. Use for buildable slices, excluding .tars/issues automation.
- **wayfinder**: Chart a large, ambiguous effort as an issue-tracker map of investigation tickets. Use when asked to map unknown decisions or work one frontier ticket per session, rather than slice known implementation work.

## Review

- **code-review**: Review a diff against a fixed commit, branch, tag, or merge-base for repository standards and spec compliance. Use for branch or PR review with two parallel agents, reporting the axes separately.
- **pr-build-context**: Prepare a briefing before reviewing a pull request: explain the diff, key files, risks, and questions for the author. Use for PR context, or a whole-repository overview when on its default branch.
- **pr-create-test-plan**: Draft TEST_PLAN.md for a branch's real user flows and integration changes using the project's commands. Use when a reviewer needs manual verification steps and a quick smoke test before merging.
- **pr-edge-cases**: Probe branch changes, uncommitted work, or specified files for logic errors, missing tests, boundary cases, races, and integration failures. Use to identify what could break before a merge, with file-line findings.
- **rfc-review**: Review an RFC or design document before approval for problem clarity, relevant compliance concerns, security, and performance. Use to return only the three to five most consequential issues with concrete suggestions.

## GitHub

- **gh-babysit-pr**: Monitor an open pull request through CI checks, review bots, and base-branch updates with pr-wait. Use only when the user explicitly asks to watch, monitor, or babysit a PR.
- **gh-create-issue**: Create a structured GitHub issue from the current conversation with gh CLI. Use when asked to file an issue that preserves the problem, discussion, constraints, and agreed implementation plan.
- **gh-create-pr**: Draft a problem-first Conventional Commit PR title and description with secret scanning and REST fallback. Use when asked to create or update a PR after approval.
- **gh-pr-comments**: Format and preflight-scan GitHub comments posted on a user's behalf with account-aware attribution. Use when posting PR comments, inline review notes, or thread replies.
- **gh-release-notes**: Curate release notes from commits and merged PRs since the last tag, then create a GitHub release. Use when asked to cut or publish a versioned release, with draft and prerelease options.
- **gh-resolve-pr-comments**: Triage and address human and AI bot PR review comments, push back on false positives, and reply before resolving threads. Use when asked to handle PR feedback.
- **gh-submit-review**: Publish a GitHub PR review with verdict, inline findings, secret scanning, and human-account attribution. Use when asked to submit a completed PR review.
- **gh-takeover**: Adopt and finish an inherited pull request, branch, or agent handoff while preserving contributor credit. Use only when the user explicitly asks to take over existing work.
- **gh-triage**: Triage GitHub issues and external PRs: categorize, verify claims, interview as needed, and post agent-ready briefs. Use when asked to change backlog states or run a maintainer triage session.
- **git-resolve-conflicts**: Resolve existing merge, rebase, or cherry-pick conflicts using file context and both sides' intent. Use when conflicts block authorized work; stage confident fixes and escalate ambiguous behavior.
- **git-update-branch**: Assess stale or diverged PRs for conceptual conflicts and update a branch against its base via rebase or merge. Use when asked to modernize or update a branch or PR.

## Reflection

- **critical-thinking**: Stress-test your immediately preceding response for hidden assumptions, weak evidence, logical gaps, and practical risks. Use to challenge your own advice before the user acts, in the response's language.
- **scope-sweep**: Run a final breadth check after the core work and review to catch missed requirements, incomplete tasks, edge cases, and operational risks. Use before declaring a scope complete, with evidence for remaining gaps.
- **self-review**: Review your implementation diff for unfinished work, non-idiomatic code, duplication, and unnecessary complexity. Use after a task to apply focused fixes, with file-and-line evidence for each change.
- **total-recall**: Maintain an append-only TSV decision trail and audit its evidence against the transcript. Use when explicitly requesting a persistent audit trail for autonomous work, multi-phase migrations, or risky refactors.
- **wtf**: Rewrite your own last response more briefly and plainly when the user signals confusion, such as "huh" or "in English". Use for confusion about your message, not requests to explain an error, output, or regex.

## Writing

- **proofread**: Proofread a post before publication for spelling, grammar, repetition, links, and weak reasoning. Use for error correction that preserves the author's voice; ask before logic changes or a skimmability pass.
- **simplify-docs**: Simplify a README, setup guide, or technical reference for fast scanning and execution. Use for structural documentation rewrites that lead with the shortest successful path, previewing edits for approval.
- **unslop**: Edit writing to remove AI cliches, filler, robotic phrasing, and decorative formatting while preserving meaning and facts. Use when text needs a natural human voice, varied rhythm, and plain language.

## Authoring

- **skill-archive**: Retire a custom skill by archiving its files, preserving history, updating inbound references, and regenerating the catalog.
- **skill-audit**: Audit the custom skill catalog for invocation policies, descriptions, frontmatter, references, duplicates, and lifecycle drift.
- **skill-creator**: Create or update custom agent skills with concise discovery descriptions, matching invocation policies, supporting resources, and catalog checks.
- **skill-router**: Browse the skill catalog and choose a workflow for a task, with a map of delivery stages and specialist skills grouped by topic.

## Tooling

- **agent-guidelines**: Apply coding discipline before and during implementation: state assumptions, keep changes minimal and scoped, and verify behavior against concrete success criteria.
- **antigravity**: Look up Google Antigravity/AGY CLI, IDE, and SDK behavior. Use for its settings, commands, keybindings, agent permissions, hooks, or plugin integration.
- **cargo-crap**: Measure Rust change risk with cargo-crap and coverage data. Use for CRAP scores, cargo-llvm-cov integration, complexity regression gates, CI reports, or badges.
- **cargo-doc**: Verify Rust crate APIs with local cargo doc output or docs.rs. Use to inspect exact signatures, methods, traits, and feature-gated items for the project version.
- **devenv**: Use when a repository has devenv.nix or devenv.yaml, or when running devenv commands. Follow its shell, dependency, SecretSpec, hook, and test-runner conventions.
- **herdr**: Operate Herdr panes, workspaces, or agent sessions when HERDR_ENV=1. Also use for Herdr setup, configuration, keybindings, and troubleshooting questions.
- **install-skills**: Install, update, remove, or troubleshoot agent skill collections with the skills.sh CLI when explicitly requested, including runtime wiring and install verification.
- **jj**: Use Jujutsu (jj) in repositories with .jj, or for jj-specific tasks. Inspect history, shape changes, write revsets/filesets, resolve conflicts, and manage bookmarks or remotes.
- **nix-shell**: Run missing ad-hoc CLI utilities in ephemeral nix shell environments when Nix is installed. Use for one-off diagnostics or conversions, keeping project dependencies in its toolchain.
- **oak**: Use Oak version control in repositories with .oak, or for oak.space tasks. Manage change stacks, conflicts, workspaces, dependency mounts, spaces, and Oak CI.
- **pagefind**: Configure, build, or troubleshoot Pagefind search for a static website. Use for indexing options, content exclusions or weighting, generated assets, and client integration.
- **prek**: Run, configure, or troubleshoot Git pre-commit hooks with prek. Use for hook execution, installation, validation, and cache management; distinguish the hook stage from its CLI.
- **scratchpad**: Manage temporary scripts, experiment files, and debugging artifacts during a task. Use one session scratch directory, keep the source tree clean, and remove temporary files afterward.
- **tailwind**: Style components or build CSS with Tailwind CSS v4. Use for utility classes, CSS-first theme variables, imports, and CLI stylesheet compilation.
- **zed**: Configure or troubleshoot Zed editor settings, keybindings, and language servers. Use to verify Zed option names and update user or project settings in the correct scope.
- **zola**: Build, serve, or customize an existing Zola site. Use for Tera templates, shortcodes, taxonomies, config.toml options, theme changes, or Zola build and link checks.

## TARS

- **tars-goal**: Set or resume a persistent goal and pursue it until every requirement is verified or a blocker prevents further work. Use when the user invokes /tars-goal or explicitly requests a persistent goal.
- **tars-run-factory**: Run the TARS factory unattended over a selected repository backlog through headless agy batches, peer review, and rework. Use when explicitly starting a bounded factory shift, with optional automated merging.

## Vendor Skills

### cloudflare

- **agents-sdk**: Build, debug, or review Cloudflare Agents SDK applications using the agents package.
- **basin**: Build and troubleshoot Cloudflare Basin analytics workflows with Basin Pipelines, Basin Catalog, and Basin SQL. Use for streaming data into R2 Iceberg tables, managing catalogs, or querying those tables; also use for requests using the former Data Platform, Pipelines, R2 Data Catalog, or R2 SQL names.
- **cf-cli**: Cloudflare's official cf CLI (beta) for the public Cloudflare API and Workers projects. Use for finding and running Cloudflare commands from the terminal — sign-in and profiles, cf cli search, cf schema, cf deploy/dev/build, resource commands for DNS, zones, D1, KV, and R2 — and for choosing between cf and Wrangler in a project.
- **cloudflare**: Discover and choose Cloudflare products for apps, APIs, AI agents, storage, networking, and security. Use for architecture and product selection, including when the user describes a need without naming a Cloudflare product; then find the relevant skill or documentation.
- **cloudflare-email-service**: Implement or troubleshoot Cloudflare Email Sending and Email Routing integrations and their delivery configuration.
- **cloudflare-one**: Design, configure, troubleshoot, or review Cloudflare One Zero Trust and SASE deployments. Use cloudflare-one-migrations for migration planning from other vendors.
- **cloudflare-one-migrations**: Assess and plan migrations from existing VPN, SWG, or SASE platforms to Cloudflare One, including policy mapping, parity gaps, and rollout.
- **durable-objects**: Build, debug, or review Cloudflare Durable Objects code for persistent state and coordination.
- **gateway-workers**: Build, debug, or review Cloudflare Workers invoked by the Forward action in Gateway HTTP policies, including the GatewayMiddleware entrypoint, Gateway types, context, and continuation.
- **k2**: Build and troubleshoot Cloudflare K2 or K2 Streams durable logs. Use for stream setup, producing from Workers or HTTP, configuring retention and inputs, and consuming through subscriptions.
- **nextjs-on-cloudflare**: Build, migrate, and deploy Next.js apps on Cloudflare Workers with vinext. Use when starting a Next.js project on Cloudflare, moving an existing app to Workers, choosing between vinext and OpenNext, or setting up vinext for Workers. For setup, migration, or deployment, install vinext's upstream skills with `npx skills add cloudflare/vinext` if missing, then read and follow the applicable skill and docs.
- **sandbox-migrate-to-next**: Migrate Cloudflare Sandbox apps from stable @cloudflare/sandbox to @cloudflare/sandbox@next (SDK 1.0 preview). Use sandbox-next for apps already on the preview.
- **sandbox-next**: Build or maintain Cloudflare Sandbox apps on @cloudflare/sandbox@next (SDK 1.0 preview). Use sandbox-migrate-to-next when porting a stable app.
- **sandbox-stable**: Build or maintain Cloudflare Sandbox apps on the stable @cloudflare/sandbox package. Use sandbox-next for preview apps and sandbox-migrate-to-next for stable-to-preview migrations.
- **turnstile-spin**: Set up, repair, or migrate to Cloudflare Turnstile bot verification in an existing frontend and backend, including server-side Siteverify.
- **web-perf**: Audit, diagnose, or optimize website loading and interaction performance, Core Web Vitals, and Lighthouse performance scores.
- **workers-best-practices**: Cloudflare Workers best practices for production applications. Use when writing, reviewing, or configuring Workers.
- **workers-profiling**: Profile or debug CPU usage and memory allocation in deployed Cloudflare Workers and Durable Objects. Then use this information to optimize your code.
- **wrangler**: Run or troubleshoot Wrangler CLI commands in projects that have a wrangler.jsonc, wrangler.json, or wrangler.toml file, including local development, Previews, deployment, and resource management. For new projects and projects without a Wrangler configuration file, use the cf CLI instead.

### spacetimedb

- **cli**: SpacetimeDB CLI reference for initializing projects, building modules, publishing databases, querying data, and managing servers
- **concepts**: Understand SpacetimeDB architecture and core concepts. Use when learning SpacetimeDB or making architectural decisions.
- **cpp-server**: SpacetimeDB C++ server module SDK reference. Use when writing tables, reducers, or module logic in C++.
- **csharp-client**: SpacetimeDB C#/.NET client SDK reference. Use when building C# clients that connect to SpacetimeDB (console, desktop, or any .NET app).
- **csharp-server**: SpacetimeDB C# server module SDK reference. Use when writing tables, reducers, or module logic in C#.
- **mcp**: Operate a running SpacetimeDB database through MCP tools rather than the CLI - list databases, read schemas, run SQL, and call reducers. Use when the client exposes spacetimedb MCP tools and the task is to inspect or change data in a live database.
- **rust-server**: SpacetimeDB Rust server module SDK reference. Use when writing tables, reducers, or module logic in Rust.
- **typescript-client**: SpacetimeDB TypeScript/React client SDK reference. Use when building web clients that connect to SpacetimeDB.
- **typescript-server**: SpacetimeDB TypeScript server module SDK reference. Use when writing tables, reducers, or module logic in TypeScript.
- **unity**: Integrate SpacetimeDB with Unity game projects. Use when building Unity clients with MonoBehaviour lifecycle, FrameTick, and PlayerPrefs token persistence.
- **unreal**: SpacetimeDB Unreal Engine client SDK reference. Use when building Unreal Engine clients that connect to SpacetimeDB.
