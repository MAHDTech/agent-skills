+++
title = "skill-router"
description = "Route to the right skill for the task at hand - a map of every skill in this collection and when to reach for it."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "authoring"
mermaid = false
+++


# Skill Router

You don't remember every skill, so ask.

This is a map of the whole collection: a **main flow** that most delivery work travels, a few **on-ramps** that feed onto it, and the standalone skills grouped by what they do. Each entry has a one-line _reach for this when…_ so you can jump straight to the fit.

_This index must be kept in sync - when a skill is added, renamed, or removed, update this file._ The `/skill-creator` and `/deprecate-skill` skills reference this maintenance rule; `/skill-audit` checks it.

## The main flow: idea → ship

The route most work travels - you have an idea and want it built:

```text
/grilling  →  /to-spec  →  /to-tickets  →  /implement (drives /tdd)  →  /code-review  →  ship via /gh-create-pr
```

- `/grilling` - reach for this when a plan or design needs pressure-testing first; it interviews you one question at a time, walking every branch of the decision tree.
- `/to-spec` - reach for this when a settled conversation should become a spec (PRD) on the tracker: synthesis, no interview.
- `/to-tickets` - reach for this when a spec or plan should be split into tracer-bullet tickets, each declaring its blocking edges.
- `/implement` - reach for this when a spec or ticket is ready to build; it drives `/tdd` internally, one red-green slice at a time.
- `/tdd` - reach for this when you want a behaviour built test-first, red-green-refactor, without a full spec.
- `/code-review` - reach for this when you want the diff since a fixed point reviewed on two axes, Standards and Spec.

Ship via `/gh-create-pr` (see _GitHub and git_).

### Underneath the flow

Two vocabulary layers the flow leans on - reach for them directly when the _words_, not the process, are the problem:

- `/codebase-design` - reach for this when shaping a module's interface or deciding where a seam goes; the deep-module vocabulary.
- `/domain-modeling` - reach for this when the domain's terms are fuzzy or overloaded; it pins the ubiquitous language in a `CONTEXT.md` glossary and records ADRs.

## On-ramps

A starting situation that generates work, then merges onto the main flow:

- `/grill-me-with-docs` - reach for this when tackling an ambiguous problem from scratch; it runs a relentless decision interview while maintaining a `CONTEXT.md` glossary and capturing ADRs.
- `/diagnosing-bugs` - reach for this when something's broken, flaky, or slow; it builds a red-capable feedback loop before hypothesising, then fixes with a regression test.
- `/gh-triage` - reach for this when the GitHub backlog piles up and issues or external PRs need turning into agent-ready briefs.
- `/wayfinder` - reach for this when the work is too big for one session and the way to the destination isn't visible yet.
- `/software-factory` - reach for this when a feature is big enough that every decision should be signed off before code exists; it runs four approval gates, then rejoins the flow at `/to-tickets`.
- `/research` - reach for this when you need facts gathered and verified against primary sources, delegated to a background agent while you keep working.

## Shaping the backlog

Before the main flow - deciding _what_ to build and _how big_ it is:

- `/prioritize-backlog` - reach for this when a pile of work needs ordering, cutting, and sequencing by value against cost so the next thing to do is obvious.
- `/estimate-work` - reach for this when a plan or ticket set needs defensible sizing and estimates, with the uncertainty surfaced rather than hidden.

## Backlog loop automation

Orchestrating the lifecycle of codebase issues in a continuous loop:

- `/tars-backlog-loop` - reach for this when you want to run a complete loop of auditing the codebase, triaging tickets, and executing implementations.
- [/tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md) - reach for this to reset the environment before starting a full backlog loop: shared git integrity, spoke root, opaque gate commands (via devenv skill when applicable), baseline gate smoke, `run.env` freeze, and cleanup of orphaned workspaces/branches.
- `/tars-backlog-audit` - reach for this when you want to dynamically partition the codebase, run parallel sub-agents to scan for bugs/vulnerabilities/features, and synthesize issues into `.tars/issues/todo/`.
- `/tars-backlog-triage` - reach for this when you want parallel sub-agents to verify the accuracy and readiness of pending issues in `.tars/issues/todo/`.
- `/tars-backlog-implement` - reach for this when you want to dynamically batch and implement triaged issues using parallel sub-agents in isolated clones, with hub `tars-gate` / spoke `tars-spoke` runners.
- [/tars-backlog-review](@/skills/review/tars-backlog-review/_index.md) - reach for this during the implementation phase of the backlog loop to assess a subagent's code changes for compliance with the spec and repo standards.
- `/tars-backlog-create-issue` - reach for this when you need to create a backlog ticket in `.tars/issues/todo/` following the standard frontmatter, markdown sections, and review guidelines.

## Changing existing code safely

Not new features - working on code that already exists:

- `/sculpt-code` - reach for this to reshape code: naming, readability, structure, shrinking surface area, and larger staged debt-reduction refactors, all without changing behaviour.
- `/characterization-tests` - reach for this when you must change untested or legacy code; it pins current behaviour as a safety net before you touch it.
- `/upgrade-dependencies` - reach for this when bumping a dependency, stepping a major version, migrating a framework, or updating a runtime.
- `/optimize-performance` - reach for this when code is too slow and you want to make it deliberately faster, profile-first (a _regression_ is a bug - use `/diagnosing-bugs`).
- `/productionize-app` - reach for this when an app needs hardening into a production-ready deployment.

## Crossing sessions

- `/handoff` - reach for this when a thread is full or you need to branch off; it compacts the conversation into a document a fresh session can pick up.
- `/store-plan` - reach for this when the current plan, decisions, and action items should be saved as a markdown file under `docs/plans/` for later.

## Reflection and quality

- `/self-review` - reach for this right after implementing, to surface missed work and simplification opportunities.
- `/scope-sweep` - reach for this as a final breadth pass before calling a scope done, to catch missed items, edge cases, and risks.
- `/critical-thinking` - reach for this when you want your own last response analysed for flaws, biases, and unstated assumptions.
- `/wtf` - reach for this when your last message did not land, to re-pitch it in plain language rather than explain it again.

## Reviewing deeper

Beyond `/code-review` on the main flow:

- `/pr-build-context` - reach for this when you want a high-signal briefing on a PR (or the whole repo) before reviewing it.
- `/pr-edge-cases` - reach for this when branch changes need probing for test gaps, edge cases, and failure modes.
- `/pr-create-test-plan` - reach for this when you want a manual, hands-on test plan for a PR's changes.
- `/rfc-review` - reach for this when reviewing an RFC or design doc for problem clarity (SCQA), compliance, security, and performance.

## GitHub and git

- `/gh-create-issue` - reach for this when conversation context should become a structured GitHub issue.
- `/gh-create-pr` - reach for this to ship: generate a PR title and description, then commit and open the PR on approval.
- `/gh-submit-review` - reach for this when a finished review should be posted to a GitHub PR (approve / request-changes / comment) with inline notes.
- `/gh-resolve-pr-comments` - reach for this when PR review comments need triaging and resolving behind an approval gate.
- `/git-resolve-conflicts` - reach for this when a merge or rebase leaves conflicts to resolve.
- `/git-update-branch` - reach for this when a branch has fallen behind its base and needs a deliberate rebase or merge.
- `/gh-release-notes` - reach for this when you want release notes or a changelog for a version and to cut a GitHub release.

## Game and database development

- `/bevy-development` - reach for this when working in the Bevy game engine (Rust ECS, systems, assets, states, scheduling).
- `/spacetimedb` - reach for this when working with SpacetimeDB database modules (schemas, reducers, views) or real-time client subscriptions.

## Cloud, infrastructure, and AI models

- `/nutanix-api-v4` - reach for this when developing, integrating, or troubleshooting with Nutanix v4 REST APIs or SDKs.
- `/nutanix-files` - reach for this when managing Nutanix Files, SMB/NFS file shares, Smart Tiering, or file server APIs.
- `/nutanix-kubernetes-platform` - reach for this when operating, deploying, upgrading, or troubleshooting Nutanix Kubernetes Platform (NKP).
- `/nutanix-objects` - reach for this when managing Nutanix Objects S3 storage, bucket policies, replication, or REST APIs.
- `/xai` - reach for this when building applications with xAI and Grok APIs, SDKs, function calling, vision, or live search.

## Writing

- `/proofread` - reach for this before publishing a post: spelling, grammar, logic, weak arguments, broken links.
- `/simplify-docs` - reach for this when documentation needs to read more clearly, with edits gated on approval.
- `/unslop` - reach for this when editing text to cut AI tells, tropes, puffery, and robotic boilerplate, and inject authentic human voice.

## Authoring

- `/skill-creator` - reach for this when creating or editing a skill in this repo: naming, placement, frontmatter, and the lint/sync workflow.
- `/skill-audit` - reach for this to health-check the whole collection: cross-references, categorisation, duplicates, retirement candidates, and gaps.
- `/deprecate-skill` - reach for this when retiring a skill: move it to `deprecated/`, repoint references, and re-run lint/sync.
- `/skill-router` - this map; reach for it when you can't remember which skill fits.

## Tooling

- `/agent-guidelines` - reach for this when you want work checked against the behavioural guidelines for agents.
- `/antigravity` - reach for this when you need to research or verify Google Antigravity specifications, command-line flags, rules, subagents, or hooks.
- `/devenv` - reach for this when a repo uses devenv for its shell and dependencies.
- `/prek` - reach for this when running, configuring, or troubleshooting pre-commit hooks (note: pre-commit CLI is deprecated, use prek).
- `/opencode` - reach for this when driving the OpenCode CLI: commands, agents, tools, MCP, config.
- [/acp](@/skills/engineering/acp/_index.md) - reach for this when building, integrating, or debugging Agent Client Protocol (ACP) agents, clients, JSON-RPC 2.0 schemas, or SDKs.
- `/opencode-acp` - reach for this when controlling OpenCode over the Agent Client Protocol.
- [/pagefind](@/skills/tooling/pagefind/_index.md) - reach for this when configuring, indexing, or troubleshooting Pagefind static search for websites.
- `/scratchpad` - reach for this when temporary or experimental scripts need a gitignored `scratch/` home.
- `/install-skills` - reach for this when installing, updating, or managing skills from a collection across agent runtimes.
- [/tailwind](@/skills/tooling/tailwind/_index.md) - reach for this when writing Tailwind CSS utility classes, compiling Tailwind bundles, or configuring Tailwind CSS v4 CSS-first themes and variables.
- `/tauri` - reach for this when working on Tauri v2+ cross-platform desktop and mobile apps (IPC commands, capabilities configuration, Rust backend lib.rs registration).
- [/zola](@/skills/tooling/zola/_index.md) - reach for this when modifying, building, serving, or customizing Zola static sites, themes, templates, or config files.
- [/zed](@/skills/tooling/zed/_index.md) - reach for this when finding, searching, or managing Zed editor settings, keybindings, LSP, and configuration files.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).

