---
name: skill-router
description: Browse the skill catalog and choose a workflow for a task, with a map of delivery stages and specialist skills grouped by topic. Use only when the user explicitly requests routing or browsing the catalog.
disable-model-invocation: true
metadata:
  group: authoring
  source: mattpocock/skills
  license: MIT
---

# Skill Router

**Invocation:** Start only when the user explicitly requests this workflow.

You don't remember every skill, so ask.

This is a map of the whole collection: a **main flow** that most delivery work travels, a few **on-ramps** that feed onto it, and the standalone skills grouped by what they do. Each skill has exactly one entry with a one-line _reach for this when…_ so you can jump straight to the fit.

_This index must be kept in sync - when a skill is added, renamed, or removed, update this file._ The `/skill-creator` and `/skill-archive` skills reference this maintenance rule; `/skill-audit` checks it.

## The main flow: idea → ship

The route most work travels - you have an idea and want it built:

```text
/grilling  →  /to-spec  →  /to-tickets  →  /tdd  →  /code-review  →  ship via /gh-create-pr
```

- `/grilling` - reach for this when a plan or design needs pressure-testing first; it interviews you one decision at a time. Run it with `--with-docs` to keep a `CONTEXT.md` glossary and record ADRs while you talk.
- `/to-spec` - reach for this when a settled conversation should become a spec (PRD) on the tracker: synthesis, no interview.
- `/to-tickets` - reach for this when a spec or plan should be split into tracer-bullet tickets, each declaring its blocking edges.
- `/tdd` - reach for this when you want a behaviour built test-first, red-green-refactor, without a full spec.
- `/code-review` - reach for this when you want the diff since a fixed point reviewed on two axes, Standards and Spec.

Ship via `/gh-create-pr` (see _GitHub and git_).

### Underneath the flow

Vocabulary the flow leans on - reach for these directly when the _words_, not the process, are the problem:

- `/codebase-design` - reach for this when shaping a module's interface or deciding where a seam goes; the deep-module vocabulary.
- `/code-comments` - reach for this when a file, path, or diff needs its comments audited so they state intent, constraints, and invariants.

## On-ramps

A starting situation that generates work, then merges onto the main flow:

- `/diagnosing-bugs` - reach for this when something's broken, flaky, or slow; it builds a red-capable feedback loop before hypothesising, then fixes with a regression test.
- `/gh-triage` - reach for this when the GitHub backlog piles up and issues or external PRs need turning into agent-ready briefs.
- `/wayfinder` - reach for this when the work is too big for one session and the way to the destination isn't visible yet.
- `/dreamwork` - reach for this only in T3 Code, when a set of issues should be planned with you, then implemented, reviewed, and council-checked autonomously in isolated worktrees, returning to you for UAT before landing.
- `/research` - reach for this when you want a question answered from primary sources and saved as a cited note in the repo.
- `/understand-codebase` - reach for this when you want an evidence-backed teardown of a source repository: what it does, how it's architected, strengths, weaknesses, and transferable learnings.
- `/reverse-engineer-binary` - reach for this when source code is absent and you need tiered CLI triage and critique of a compiled binary, Electron/ASAR bundle, APK, or packaged app.

## Shaping the backlog

Before the main flow - deciding _what_ to build and _how big_ it is:

- `/prioritize-backlog` - reach for this when a pile of work needs ordering, cutting, and sequencing by value against cost so the next thing to do is obvious.
- `/estimate-work` - reach for this when a plan or ticket set needs defensible sizing and estimates, with the uncertainty surfaced rather than hidden.

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

- `/self-review` - reach for this right after implementing, to surface unfinished work and code that breaks project conventions.
- `/scope-sweep` - reach for this as a final completeness pass before calling a scope done, to catch missed requirements and unfinished tasks.
- `/critical-thinking` - reach for this when you want your own last response analysed for flaws, biases, and unstated assumptions.
- `/wtf` - reach for this when your last message did not land, to re-pitch it in plain language rather than explain it again.
- `/total-recall` - reach for this when you want an append-only TSV decision trail recorded during long-running or autonomous work.

## Reviewing deeper

Beyond `/code-review` on the main flow:

- `/pr-build-context` - reach for this when you want a high-signal briefing on a PR (or the whole repo) before reviewing it.
- `/pr-edge-cases` - reach for this when branch changes need probing for test gaps, edge cases, and failure modes.
- `/pr-create-test-plan` - reach for this when you want a manual, hands-on test plan for a PR's changes.
- `/rfc-review` - reach for this when reviewing an RFC or design doc for problem clarity (SCQA), compliance, security, and performance.

## GitHub and git

- `/gh-create-issue` - reach for this when conversation context should become a structured GitHub issue.
- `/gh-create-pr` - reach for this to ship: generate a PR title and description, then commit and open the PR on approval.
- `/gh-pr-comments` - reach for this when posting PR or issue comments, inline review notes, or thread replies on a user's behalf; it owns the attribution format and secret preflight scan.
- `/gh-babysit-pr` - reach for this when you want to monitor an open PR through CI checks, review bots, and base-branch updates with `pr-wait`.
- `/gh-takeover` - reach for this when adopting an unfinished PR, branch, or agent handoff to completion while preserving contributor credit.
- `/gh-submit-review` - reach for this when a finished review should be posted to a GitHub PR (approve / request-changes / comment) with inline notes.
- `/gh-resolve-pr-comments` - reach for this when PR review comments need triaging and resolving behind an approval gate.
- `/git-resolve-conflicts` - reach for this when a merge, rebase, or cherry-pick leaves conflicts to resolve.
- `/git-update-branch` - reach for this when a branch or stale PR has diverged from its base and needs conceptual-conflict checks before a deliberate rebase or merge.
- `/gh-release-notes` - reach for this when you want release notes or a changelog for a version and to cut a GitHub release.

## Interface and game development

- `/interface-design` - reach for this when planning, building, refining, or critiquing UI/UX across web, desktop, mobile, terminal interfaces, and games; framework specialists still own API details.
- `/bevy-development` - reach for this when working in the Bevy game engine (Rust ECS, systems, assets, states, scheduling).

## Protocols, platforms, and AI models

- `/acp` - reach for this when building, integrating, or debugging Agent Client Protocol (ACP) agents, clients, or SDKs.
- `/model-context-protocol` - reach for this when building or debugging MCP servers against the current stateless spec, including transports and Inspector testing.
- `/agent-router` - reach for this when configuring or troubleshooting AgentRouter AI gateways, unified LLM endpoints, or MCP and A2A routing (including the gateway inside Nutanix Enterprise AI).
- `/casita` - reach for this when working with Casita content-addressed repositories (immutable object graphs, CLI, Rust API, synchronization, or garbage collection).
- `/icp` - reach for this when building, securing, testing, or deploying Internet Computer (ICP) canisters and dapps in Motoko, Rust, or TypeScript.
- `/nutanix-api-v4` - reach for this when developing, integrating, or troubleshooting with Nutanix v4 REST APIs or SDKs.
- `/nutanix-enterprise-ai` - reach for this when deploying, configuring, serving, or troubleshooting LLMs and MCP servers with Nutanix Enterprise AI (NAI).
- `/nutanix-files` - reach for this when managing Nutanix Files, SMB/NFS file shares, Smart Tiering, or file server APIs.
- `/nutanix-kubernetes-platform` - reach for this when operating, deploying, upgrading, or troubleshooting Nutanix Kubernetes Platform (NKP).
- `/nutanix-objects` - reach for this when managing Nutanix Objects S3 storage, bucket policies, replication, or REST APIs.
- `/xai` - reach for this when building applications with xAI and Grok APIs, SDKs, function calling, vision, or live search.

## Writing

- `/unslop` - reach for this before publishing: cut AI tells and filler, then fix spelling, grammar, and broken links.
- `/simplify-docs` - reach for this when documentation needs restructuring to read more clearly, with edits gated on approval.

## Authoring

- `/skill-creator` - reach for this when creating or editing a skill in this repo: naming, placement, frontmatter, and the lint/sync workflow.
- `/skill-audit` - reach for this to health-check the whole collection: cross-references, categorisation, duplicates, retirement candidates, and gaps.
- `/skill-archive` - reach for this when retiring a skill: move it to `skills-archive/<category>/`, mark it archived, repoint references, and re-run lint/sync.
- `/skill-router` - this map; reach for it when you can't remember which skill fits.

## Tooling

- `/agent-guidelines` - reach for this while writing or editing code to apply coding discipline (state assumptions, keep changes minimal and scoped, and verify against concrete criteria), not when reviewing finished work.
- `/antigravity` - reach for this when you need to research or verify Google Antigravity specifications, command-line flags, rules, subagents, or hooks.
- `/devenv` - reach for this when a repo uses devenv for its shell and dependencies; every project command then runs through it.
- `/nix-shell` - reach for this on NixOS in a project without devenv, to run ad-hoc CLI utilities in ephemeral Nix shells.
- `/prek` - reach for this when running, configuring, or troubleshooting Git pre-commit hooks with prek instead of the pre-commit CLI.
- `/herdr` - reach for this when controlling the Herdr terminal multiplexer, managing workspaces, tabs, and panes, or coordinating coding agents.
- `/jj` - reach for this when working in a Jujutsu (`jj`) repository: revsets, filesets, bookmarks, conflicts, or syncing with GitHub.
- `/scratchpad` - reach for this when temporary or experimental files need one session scratch location outside the source tree.
- `/install-skills` - reach for this when installing, updating, or managing skills from a collection across agent runtimes.
- `/cargo-doc` - reach for this when generating, querying, or navigating local Rust documentation and docs.rs to confirm exact API signatures.
- `/cargo-crap` - reach for this when measuring and gating Change Risk Anti-Patterns (CRAP) across Rust codebases by combining complexity with test coverage.
- `/pagefind` - reach for this when configuring, indexing, or troubleshooting Pagefind static search for websites.
- `/tailwind` - reach for this when writing Tailwind CSS utility classes, compiling Tailwind bundles, or configuring Tailwind CSS v4 CSS-first themes.
- `/zola` - reach for this when modifying, building, serving, or customizing Zola static sites, themes, templates, or config files.
- `/zed` - reach for this when finding or changing Zed editor settings, keybindings, language servers, or per-language formatting.

## GUARDS

The agy-guards Antigravity plugin and its workflows:

- `/guards-goal` - reach for this when you want a persistent goal set or resumed and pursued until every requirement is verified.
- `/guards-run-factory` - reach for this to run the unattended GUARDS factory over a backlog: headless agy batches, reviews, and rework until it drains or needs a human.
- `/guards-debug` - reach for this from outside an active run when a GUARDS session, hook, spoke, or factory shift is stuck or failing.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).
