+++
title = "skill-creator"
description = "Use when creating a new skill or editing an existing one in this repo - how to name it, place it in the right category, write its frontmatter and description, and structure it with progressive disclosure. Covers model-invoked vs user-invoked skills, the six skill failure modes, and the lint/sync workflow to run after adding or renaming a skill."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "authoring"
mermaid = false
+++


# Skill Creator

A skill exists to wrangle determinism out of a stochastic system. **Predictability** - the agent taking the same _process_ every run, not producing the same output - is the root virtue; every convention below serves it. Write the skill so the next run behaves like the last one.

**Bold terms** are defined in [`GLOSSARY.md`](@/skills/authoring/skill-creator/resources/manual/GLOSSARY.md); look them up there for the full meaning.

## Where a skill lives

Every skill is a directory holding a `SKILL.md`, placed by what it does:

```text
skills/<category>/<name>/SKILL.md
```

- **`<category>`** is one of the nine topic buckets: `engineering`, `game-development`, `planning`, `review`, `github`, `reflection`, `writing`, `authoring`, `tooling`. Category comes from the directory - never from a frontmatter key.
- Two **lifecycle buckets** sit outside the topic tree: `in-progress/` holds drafts, and `deprecated/` holds retired skills.

**Promotion** is the payoff of living in a topic bucket: only skills under the nine categories appear in the generated README and index. A skill in `in-progress/` or `deprecated/` is deliberately excluded - move it into a topic bucket to promote it, move it to `deprecated/` to retire it.

## Naming

The name is prefix-free kebab-case, and it must equal the directory basename (`skills/writing/proofread/` is named `proofread`).

- **Verb-first for an action** the skill performs (`sculpt-code`, `git-resolve-conflicts`); **a noun for a body of knowledge** it holds (`tdd`, `agent-guidelines`).
- **Keep only a genuine subject scope** as a prefix - `gh-` for GitHub API work, `git-` for git operations. These name a real tool the skill acts on; a project or subsystem name is a genuine scope too (`tars-` for the `.tars` backlog tooling, so `tars-backlog-*` is correct). Drop taxonomy prefixes like `cmd-`, `brain-`, or `sys-`; the category directory already carries that signal.
- 1–64 characters, and it must not contain "anthropic" or "claude".

## Canonical frontmatter

The minimum is two keys:

```yaml
---
name: kebab-case-name
description: What the skill does AND when to reach for it, in the user's own words.
---
```

`name` matches the directory. `description` is **model-facing** and does the invocation work (see below). Optional keys, each added only when earned:

- **`disable-model-invocation: true`** - makes the skill **user-invoked** (see below).
- **`argument-hint`** - a short usage hint for a skill that takes an argument.
- **`context: fork`** with **`agent: <type>`** (used together, e.g. `agent: general-purpose`) - runs the skill as a subagent in its own context, so a long or noisy run does not silt up the caller's window.
- **`metadata:`** - a flat string→string map for provenance. Use `source` and `license` on any skill adapted from an outside project (as this one carries `source: mattpocock/skills`, `license: MIT`).
- **`resources:`** - a YAML **list** of source URLs. It is functional, not decorative: `skills --action download-resources` reads it (see `bin/skills/downloader.ts`) to (re)fetch the vendored docs into the skill's `resources/auto/` directory (see the structure rule below), and many reference skills rely on it. Keep it intact; never strip it.

That is the complete allowed set, so the frontmatter stays small. Distinct from the above are the **legacy** keys `custom:`, `triggers:`, `category:`, and `type:` - forbidden. Earlier skills carry them mid-migration; a new or edited skill drops them, putting triggers into the `description` prose and taking the category from the directory. Do not confuse these forbidden legacy keys with the real, functional `resources:` and `metadata:` keys above.

## Invocation

One axis splits every skill - who can reach it:

- A **model-invoked** skill keeps its **description**, so the agent can fire it autonomously _and_ other skills can reach it (you can still type its name too). It pays a permanent **context load**: the description sits in the window every turn. Mechanics: omit `disable-model-invocation`, and write a description with rich trigger phrasing ("Use when the user wants…, mentions…, asks for…").
- A **user-invoked** skill strips the description from the agent's reach: only you, typing its name, can invoke it - and no other skill can. Zero context load, but it spends **cognitive load**: _you_ are the index that must remember it exists. Mechanics: set `disable-model-invocation: true`, and make the `description` a human-facing one-line summary with the trigger lists stripped.

Choose model-invocation only when the agent must reach the skill on its own, or another skill must. The test is _could the agent usefully reach for this by itself?_ - reuse is a reason to extract a skill, not the test for whether it is model-invoked. If it only ever fires by hand, make it user-invoked and pay no context load.

## Writing the description

A model-invoked **description** does two jobs: state what the skill is, and list the **branches** that should trigger it. Every word adds context load, so prune it harder than the body.

- **Front-load the skill's leading word** - the description is where it does its invocation work.
- **One trigger per branch.** Synonyms that rename a single branch are **duplication** - collapse them and keep only genuinely distinct branches.
- **Cut identity already stated in the body.** Keep the description to triggers plus any "when another skill needs…" reach clause.
- **No em-dashes.** Never use em-dashes (Unicode U+2014) in skill names, frontmatter descriptions, or skill bodies; use standard hyphens (`-`), colons, commas, or restructure sentences.

## Structure and progressive disclosure

A skill's content is ranked by how immediately the agent needs it - the **information hierarchy**, a ladder with three rungs:

1. **In-skill step** - an ordered action in `SKILL.md`: what the agent does, in order. The primary tier.
2. **In-skill reference** - a definition, rule, or fact in `SKILL.md`, consulted on demand. Often a flat peer-set (every rule of a review on one rung), which is a fine arrangement, not a smell.
3. **External reference** - reference pushed out of `SKILL.md` into a sibling file, reached by a **context pointer** and loaded only when the pointer fires (this skill discloses its definitions to `GLOSSARY.md`).

**Progressive disclosure** is the move down the ladder - out of `SKILL.md` into a linked file - so the top stays legible. Siblings live beside `SKILL.md` under `resources/`, which splits by ownership:

```text
skills/<category>/<name>/
  SKILL.md          # entry point - steps and top-tier reference
  resources/        # optional; holds ONLY these two subdirectories:
    auto/           # downloader-owned - (re)fetched from the `resources:` URLs; never hand-edit
    manual/         # hand-authored scripts, docs, references, and static files
```

Never place a file directly in `resources/`: every resource lives under `auto/` (managed by `download-resources`, safe to wipe and reproduce) or `manual/` (yours, tooling never touches it). `skills --action lint` enforces this, and `clean-resources` deletes only `auto/`.

**Branching** is the disclosure test: inline what every branch needs, and push behind a pointer what only some branches reach. A pointer's _wording_, not its target, decides when and how reliably the agent follows it - a must-have behind a weak pointer is a variance bug, so sharpen the wording before pulling material back inline.

## Completion criteria

Every step ends on a **completion criterion** - the condition that tells the agent the work is done. Make it:

- **Checkable** - can the agent tell done from not-done? "Understanding reached" cannot; "every changed file has a test" can.
- **Exhaustive where it matters** - "every modified model accounted for", not "produce a change list". A vague bound invites **premature completion**.

A demanding criterion drives thorough **legwork** - the digging the agent does within a step - and it binds flat reference too ("every rule applied"), which is how a skill with no steps still carries an exhaustiveness bar.

## Leading words

A **leading word** is a compact concept already living in the model's pretraining that the agent thinks with while running the skill (e.g. _seam_, _fog of war_, _tracer bullet_). Repeated as a token - never re-explained as a sentence - it accumulates a distributed definition and anchors a whole region of behaviour in the fewest tokens, by recruiting priors the model already holds.

It serves predictability twice. In the body it anchors _execution_: the agent reaches for the same behaviour every time the word appears. In the description it anchors _invocation_: when the same word lives in your prompts, docs, and code, the agent links that shared language to the skill and fires it more reliably. Reach for an existing word first; a coined one recruits no priors and costs definition tokens.

Hunt for restatements a leading word retires: a triad spelled out three times, or a sentence gesturing at one idea, each **collapses** into a single token - fewer tokens _and_ a sharper hook.

## Prompt the positive

State the target behaviour, not the banned one. **Negation** backfires: _don't think of an elephant_ names the elephant and makes it more available. Describe what to do ("write one-line comments") so the forbidden pattern is never spoken. Keep a prohibition only as a hard guardrail you cannot phrase positively - and even then pair it with the positive target.

## Stay host-agnostic

A skill runs across multiple agent runtimes - Claude Code, OpenCode, Goose, Antigravity CLI - so it must bind to no single host's tooling. Name the **capability**, not the product: "your task-tracking tool", "your agent's subagent mechanism", never one runtime's command, tool name, or built-in. Never bake in a personal or absolute path; keep paths repo-relative. A skill that reads the same on every host stays predictable on every host.

## When to split

**Granularity** is how finely you divide skills; each cut spends one of the two loads, so split only when the cut earns it. Two cuts:

- **By invocation** - split off a model-invoked skill when a distinct **leading word** should trigger it on its own, or another skill must reach it. You pay context load for the new always-loaded description, so that independent reach has to be worth it.
- **By sequence** - split a run of **steps** when the steps still ahead (a step's **post-completion steps**) tempt the agent to rush the one in front of it. Hiding them behind a real context boundary - a user-invoked hand-off or a `context: fork` subagent - encourages more legwork on the current task.

## Failure modes

Diagnose a misbehaving skill against these:

- **Premature completion** - ending a step before it is genuinely done, attention slipping to _being done_. Defence, in order: sharpen the completion criterion first (cheap, local); only if it is irreducibly fuzzy _and_ you observe the rush, hide the later steps by splitting the sequence.
- **Duplication** - the same meaning in more than one place. Costs maintenance and tokens, and inflates a meaning's rank on the ladder. Keep each meaning in a **single source of truth**.
- **Sediment** - stale layers that settle because adding feels safe and removing feels risky. The default fate of any skill without a pruning discipline; check every line for **relevance**.
- **Sprawl** - a skill simply too long, even when every line is live and unique. The cure is the ladder: disclose reference behind pointers, and split by branch or sequence so each path carries only what it needs.
- **No-op** - a line the model already obeys by default, so you pay load to say nothing. The test: does it change behaviour versus the default? A weak leading word (_be thorough_) is a no-op; the fix is a stronger word (_relentless_), not a different technique.
- **Negation** - steering by prohibition, which drags the forbidden behaviour into context and makes it _more_ available. Prompt the positive.

Prune sentence by sentence: run the no-op test on each sentence in isolation, and when one fails, delete the whole sentence rather than trim words from it. Be aggressive - most prose that fails should go, not be rewritten.

## After adding or renaming a skill

Regenerate the derived artifacts and the router:

1. `devenv --no-tui shell -- skills --action lint` - validate frontmatter, naming, and placement.
2. `devenv --no-tui shell -- skills --action sync` - regenerate the README, `agents/AGENTS.md`, the Zola dashboard, and `skills.sh.json`.
3. Update the `/skill-router` index so the new or renamed skill is reachable.

Done when lint passes, sync leaves no further diff, and the router names the skill.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).

