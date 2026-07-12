+++
title = "skill-router"
description = "Route to the right skill for the task at hand — a map of every skill in this collection and when to reach for it."
[extra]
category = "authoring"
mermaid = false
+++


# Skill Router

You don't remember every skill, so ask.

This is a map of the whole collection: a **main flow** that most delivery work travels, a few **on-ramps** that feed onto it, and the standalone skills grouped by what they do. Each entry has a one-line _reach for this when…_ so you can jump straight to the fit.

_This index must be kept in sync — when a skill is added, renamed, or removed, update this file._ The `/skill-creator` skill references this maintenance rule.

## The main flow: idea → ship

The route most work travels — you have an idea and want it built:

```text
/grilling or /plan-before-coding  →  /to-spec  →  /to-tickets  →  /implement (drives /tdd)  →  /code-review  →  ship via /gh-create-pr
```

- `/grilling` — reach for this when a plan or design needs pressure-testing first; it interviews you one question at a time, walking every branch of the decision tree.
- `/plan-before-coding` — reach for this when you'd rather build the implementation plan collaboratively, back and forth, than be interrogated.
- `/to-spec` — reach for this when a settled conversation should become a spec (PRD) on the tracker: synthesis, no interview.
- `/to-tickets` — reach for this when a spec or plan should be split into tracer-bullet tickets, each declaring its blocking edges.
- `/implement` — reach for this when a spec or ticket is ready to build; it drives `/tdd` internally, one red-green slice at a time, and closes out with `/code-review`.
- `/tdd` — reach for this when you want a behaviour built test-first, red-green-refactor, without a full spec.
- `/code-review` — reach for this when you want the diff since a fixed point reviewed on two axes, Standards and Spec.

Ship via `/gh-create-pr` (see _GitHub and git_).

### Underneath the flow

Two vocabulary layers the flow leans on — reach for them directly when the _words_, not the process, are the problem:

- `/codebase-design` — reach for this when shaping a module's interface or deciding where a seam goes; the deep-module vocabulary.
- `/domain-modeling` — reach for this when the domain's terms are fuzzy or overloaded; it pins the ubiquitous language in a `CONTEXT.md` glossary and records ADRs.

## On-ramps

A starting situation that generates work, then merges onto the main flow:

- `/diagnosing-bugs` — reach for this when something's broken, flaky, or slow; it builds a red-capable feedback loop before hypothesising, then fixes with a regression test.
- `/triage` — reach for this when issues or external PRs you _didn't_ create pile up and need turning into agent-ready briefs.
- `/wayfinder` — reach for this when the work is too big for one session and the way to the destination isn't visible yet.
- `/research` — reach for this when you need facts gathered and verified against primary sources, delegated to a background agent while you keep working.

## Keeping the codebase healthy

Not feature work — upkeep:

- `/refactor-codebase` — reach for this when removing debt, untangling large modules, or collapsing duplicated logic without changing behaviour.
- `/sculpt-code` — reach for this for a lighter touch-up: naming, readability, structure, and shrinking surface area.
- `/productionize-app` — reach for this when an app needs hardening into a production-ready deployment.

## Crossing sessions

- `/handoff` — reach for this when a thread is full or you need to branch off; it compacts the conversation into a document a fresh session can pick up.
- `/store-plan` — reach for this when the current plan, decisions, and action items should be saved as a markdown file under `plans/` for later.

## Reflection and quality

- `/self-review` — reach for this right after implementing, to surface missed work and simplification opportunities.
- `/scope-sweep` — reach for this as a final pass before calling a scope done, to catch missed items, edge cases, and risks.
- `/critical-thinking` — reach for this when you want your own last response analysed for flaws, biases, and unstated assumptions.
- `/rfc-review` — reach for this when reviewing an RFC for problem clarity, compliance, security, and performance (SCQA).

## Reviewing deeper

Beyond `/code-review` on the main flow:

- `/pr-build-context` — reach for this when you want a high-signal briefing on a PR before reviewing it.
- `/pr-prepare-review` — reach for this when preparing your own branch for review, surfacing issues and improvements first.
- `/pr-edge-cases` — reach for this when branch changes need probing for test gaps, edge cases, and failure modes.
- `/pr-create-test-plan` — reach for this when you want a manual, hands-on test plan for a PR's changes.

## GitHub and git

- `/gh-create-issue` — reach for this when conversation context should become a structured GitHub issue.
- `/gh-create-pr` — reach for this to ship: generate a PR title and description, then commit and open the PR on approval.
- `/gh-resolve-pr-comments` — reach for this when PR review comments need triaging and resolving behind an approval gate.
- `/git-resolve-conflicts` — reach for this when a merge or rebase leaves conflicts to resolve.

## Writing

- `/proofread` — reach for this before publishing a post: spelling, grammar, logic, weak arguments, broken links.
- `/simplify-docs` — reach for this when documentation needs to read more clearly, with edits gated on approval.

## Authoring

- `/skill-creator` — reach for this when creating or editing a skill in this repo: naming, placement, frontmatter, and the lint/sync workflow.
- `/skill-router` — this map; reach for it when you can't remember which skill fits.

## Tooling

- `/agent-guidelines` — reach for this when you want work checked against the behavioural guidelines for agents.
- `/bevy-development` — reach for this when working in the Bevy game engine (Rust ECS, systems, assets, states, scheduling).
- `/devenv` — reach for this when a repo uses devenv for its shell and dependencies.
- `/opencode` — reach for this when driving the OpenCode CLI: commands, agents, tools, MCP, config.
- `/opencode-acp` — reach for this when controlling OpenCode over the Agent Client Protocol.
- `/scratchpad` — reach for this when temporary or experimental scripts need a gitignored `scratch/` home.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).

