+++
title = "skill-creator"
description = "Use when creating a new skill or editing an existing one in this repo - how to name it, place it in the right category, write its frontmatter and description, and structure it with progressive disclosure. Covers model-invoked vs user-invoked skills, anti-slop conventions, failure modes, and the lint/sync workflow to run after adding or renaming a skill."
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
- One **lifecycle bucket** sits inside the tree: `in-progress/` holds drafts. Retired skills leave `skills/` entirely for the top-level **archive**, `skills-archive/<category>/<name>/`, which keeps the original category.

**Promotion** is the payoff of living in a topic bucket: only skills under the nine categories appear in the generated README, index, and installer. A skill in `in-progress/` is deliberately excluded - move it into a topic bucket to promote it. To retire one, use `/skill-archive`, which moves it to `skills-archive/` where it stays readable on the dashboard but is never installed.

## Naming

The name is prefix-free kebab-case, and it must equal the directory basename (`skills/writing/proofread/` is named `proofread`).

- **Verb-first for an action** the skill performs (`sculpt-code`, `git-resolve-conflicts`); **a noun for a body of knowledge** it holds (`tdd`, `agent-guidelines`).
- **Keep only a genuine subject scope** as a prefix - `gh-` for GitHub API work, `git-` for git operations. These name a real tool the skill acts on; a project or subsystem name is a genuine scope too (`acme-` for skills that only make sense inside the Acme tool, so `acme-deploy` is correct). Drop taxonomy prefixes like `cmd-`, `brain-`, or `sys-`; the category directory already carries that signal.
- 1 to 64 characters matching `^[a-z0-9]+(-[a-z0-9]+)*$`, and it must not contain "anthropic" or "claude".

## Canonical frontmatter

The minimum is two keys:

```yaml
---
name: kebab-case-name
description: What the skill does AND when to reach for it, in the user's own words.
---
```

`name` matches the directory (1 to 64 characters). `description` is **model-facing** and does the invocation work (1 to 1024 characters, see below). Optional keys, each added only when earned:

- **`disable-model-invocation: true`** - makes the skill **user-invoked** (see below).
- **`argument-hint`** - a short usage hint for a skill that takes an argument.
- **`context: fork`** with **`agent: <type>`** (used together, e.g. `agent: general-purpose`) - runs the skill as a subagent in its own context, so a long or noisy run does not silt up the caller's window.
- **`metadata:`** - a flat string→string map for provenance and grouping. Use `source` and `license` on any skill adapted from an outside project (as this one carries `source: mattpocock/skills`, `license: MIT`). Use `group: <group-name>` to declare membership in a cohesive cross-reference group (standard groups: `authoring`, `github`, `planning-pipeline`, `review`, `opencode`, `tars`, `nutanix`). Skills are self-contained by default; cross-references via `/skill-name` mentions or relative markdown links are only permitted between skills in the same group (`skill-router` is exempt).
- **`resources:`** - a YAML **list** of source URLs. It is functional, not decorative: `ask skills download-resources` reads it (implemented in `crates/skills-core/src/downloader.rs`) to (re)fetch vendored docs into the skill's `resources/auto/` directory (see the structure rule below), and reference skills rely on it. Keep it intact; never strip it.

That is the complete allowed set, so the frontmatter stays small. Distinct from the above are the **legacy** keys `custom:`, `triggers:`, `category:`, and `type:` - forbidden. Earlier skills carry them mid-migration; a new or edited skill drops them, putting triggers into the `description` prose and taking the category from the directory. Do not confuse these forbidden legacy keys with the real, functional `resources:` and `metadata:` keys above.

## Invocation

One axis splits every skill - who can reach it:

- A **model-invoked** skill keeps its **description**, so the agent can fire it autonomously _and_ other skills can reach it (you can still type its name too). It pays a permanent **context load**: the description sits in the window every turn. Mechanics: omit `disable-model-invocation`, and write a description with rich trigger phrasing ("Use when the user wants..., mentions..., asks for...").
- A **user-invoked** skill strips the description from the agent's reach: only you, typing its name, can invoke it - and no other skill can. Zero context load, but it spends **cognitive load**: _you_ are the index that must remember it exists. Mechanics: set `disable-model-invocation: true`, and make the `description` a human-facing one-line summary with the trigger lists stripped.

Choose model-invocation only when the agent must reach the skill on its own, or another skill must. Set `disable-model-invocation: true` when a skill meets any of these operational criteria:

1. **High blast-radius or destructive actions**: operations such as deleting branches, wiping worktrees, or resetting databases.
2. **Coarse multi-phase pipelines**: heavy orchestration workflows that require explicit human initiation and oversight.
3. **Interactive interview protocols**: question-and-answer interrogation flows (such as interactive design interviews).
4. **Meta-catalogs and indexes**: reference maps and router indexes (such as `/skill-router`).

## Writing the description

A model-invoked **description** does two jobs: state what the skill is, and list the **branches** that should trigger it. Every word adds context load, so prune it harder than the body.

Follow the canonical 3-part description pattern:

```yaml
description: [1. Capability verb phrase]. Use when [2. Explicit triggers, symptoms, user keywords]. Do not use for [3. Negative boundary / disambiguation]; use /sibling-skill instead.
```

- **Front-load the skill's leading word** - the description is where it does its invocation work.
- **One trigger per branch.** Synonyms that rename a single branch are **duplication** - collapse them and keep only genuinely distinct branches.
- **Cut identity already stated in the body.** Keep the description to triggers plus any "when another skill needs..." reach clause.
- **No frontmatter slop or puffery.** Never begin descriptions with "This skill provides...", "Expert guidance for...", "Expert reference...", or "Comprehensive guide...". Lead immediately with the capability verb phrase (e.g. `Build...`, `Audit...`, `Deploy...`). Avoid buzzwords like "comprehensive", "robust", or "seamless". `ask skills lint` checks for these forbidden patterns.
- **No em-dashes.** Never use em-dashes (Unicode U+2014) in skill names, frontmatter descriptions, or skill bodies; use standard hyphens (`-`), colons, commas, or restructure sentences. `ask skills lint --fix` can automatically sanitize em-dashes.

## Structure and progressive disclosure

A skill's content is ranked by how immediately the agent needs it - the **information hierarchy**, a ladder with three rungs:

1. **In-skill step** - an ordered action in `SKILL.md`: what the agent does, in order. The primary tier.
2. **In-skill reference** - a definition, rule, or fact in `SKILL.md`, consulted on demand. Often a flat peer-set (every rule of a review on one rung), which is a fine arrangement, not a smell.
3. **External reference** - reference pushed out of `SKILL.md` into a sibling file, reached by a **context pointer** and loaded only when the pointer fires (this skill discloses its definitions to `GLOSSARY.md`).

**Progressive disclosure** is the move down the ladder - out of `SKILL.md` into a linked file - so the top stays legible. Siblings live beside `SKILL.md` under `resources/`, which splits by ownership:

```text
skills/<category>/<name>/
  SKILL.md          # entry point - steps and top-tier reference
  agents/           # optional; harness-specific configurations
    openai.yaml     # OpenAI Codex / ChatGPT app UI metadata and policy
  resources/        # optional; holds ONLY these two subdirectories:
    auto/           # downloader-owned - (re)fetched from the `resources:` URLs; never hand-edit
    manual/         # hand-authored scripts, docs, references, and static files
```

Never place a file directly in `resources/`: every resource lives under `auto/` (managed by `ask skills download-resources`, safe to wipe and reproduce) or `manual/` (yours, tooling never touches it). `ask skills lint` enforces this, and `ask skills clean-resources` deletes only `auto/`.

**Markdown and Linking Rules**:

- **Code fence language tags (MD040)**: Never use bare triple backticks. Always declare a language tag (` ```bash `, ` ```json `, ` ```text `, ` ```rust `).
- **Blank lines around fences (MD031)**: Every code block must be preceded and followed by a blank line, including when nested inside lists.
- **No absolute file links**: Never use `file:///` URLs referencing local paths. Always use relative repository paths (e.g. `../../review/code-review/SKILL.md`).
- **Heading formatting**: Headings must use plain text sentence case without bold markup (e.g. `### 1. Audit environment`, never `### 1. **Phase 1: Initial Assessment**`). Bold markup inside headings is prohibited and flagged by `ask skills lint`.
- **No tautological list items**: Never use bold list lead-ins that merely repeat the item name (e.g. `- **Performance:** Optimize performance...`). State concrete details or use direct prose.

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

## Voice and anti-slop discipline

A skill is an operational manual for an agent, not a corporate marketing deck, tutorial, or textbook. Keep instructions dense, procedural, and falsifiable:

- **Concrete mechanisms over vague qualifiers.** Ban lazy modifiers like "proper", "properly", "appropriate", or "thorough" unless paired with an exact invariant, command, or threshold. Instead of "Ensure proper error handling", specify "Return an explicit fallback error and log the response status code on network timeouts".
- **Eliminate AI crutch vocabulary.** Avoid buzzwords like "utilize", "leverage", "seamlessly", "furthermore", "delve", "testament to", "pivotal", or "evolving landscape". Prefer plain words: "use", "if", "because", or name the exact mechanism.
- **No corporate roadmap padding.** Omit fake roadmaps ("Immediate Next Steps within 48 hours / 1 week / 1 month"), generic "Best Practices" lists filled with platitudes, or hypothetical Q&A sections. Replace them with concrete verification steps and unambiguous completion criteria.
- **The copy-paste test.** If a sentence or bullet could appear in an unrelated project without changing a word, it contains zero specific signal. Delete it or anchor it to exact codebase paths, data models, and tools.

## Stay host-agnostic

A skill runs across multiple agent runtimes - Claude Code, OpenCode, Goose, Antigravity CLI, and OpenAI Codex - so the core `SKILL.md` must bind to no single host's tooling. Name the **capability**, not the product: "your task-tracking tool", "your agent's subagent mechanism", never one runtime's command, tool name, or built-in. Never bake in a personal or absolute path; keep paths repo-relative. A skill that reads the same on every host stays predictable on every host. Harness-specific UI metadata or policies belong exclusively in sidecars like `agents/openai.yaml`, keeping `SKILL.md` pure and universal.

## Harness compatibility: `agents/openai.yaml`

While `SKILL.md` is the universal, runtime-agnostic entry point, individual harnesses may define metadata sidecars under the `agents/` directory. OpenAI Codex and the ChatGPT desktop application read `agents/openai.yaml` to configure UI presentation, invocation policies, and tool dependencies:

```yaml
interface:
  display_name: "Skill Creator"
  short_description: "Create or update skills with progressive disclosure"
  default_prompt: "Use $skill-creator to create a new agent skill."

policy:
  allow_implicit_invocation: true
```

Key rules for `agents/openai.yaml`:

- **`display_name`**: Human-facing Title Case name for skill lists, pickers, and badges.
- **`short_description`**: Strictly between 25 and 64 characters long. No HTML angle brackets (`<` or `>`), no em-dashes, and unslop.
- **`default_prompt`**: Suggested prompt template for user invocation; must reference the skill name using `$skill-name` syntax.
- **`policy.allow_implicit_invocation`**: Set to `false` for user-invoked skills (`disable-model-invocation: true`), destructive operations, or specialized handoffs. Set to `true` (or omit) for standard model-invoked skills.
- **`dependencies.tools`**: Optional list declaring external tools such as MCP servers (`type: "mcp"`, `value`, `description`, `transport: "streamable_http"`, `url`).
- **Formatting**: Keys must be unquoted, all string values must be quoted in double quotes, and indentation must be 2 spaces.

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
- **Trigger ambiguity** - overly broad descriptions triggering on unrelated queries, or overly narrow descriptions that fail natural user prompts.
- **Host leakage** - baking host-specific tools, internal agent built-ins, or absolute paths into general skills.
- **Verification absence** - omitting runnable test or lint commands in completion criteria.
- **AI slop and puffery** - inflating procedural instructions with corporate, marketing, or academic filler ("comprehensive guidance", "robust frameworks", bolded slide-deck headings).
- **Tautological labeling** - bold list lead-ins that merely repeat the item name instead of adding distinct operational signal.
- **Generic filler** - boilerplate roadmaps or platitudinous checklists that apply to any project.

| Failure Mode              | Bad Pattern (Anti-Pattern)                                           | Good Pattern (Remedy)                                                                                                                                                                                                                    |
| :------------------------ | :------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Premature Completion**  | "Make sure all files are properly formatted."                        | "Run `cargo fmt --check` and verify exit code is 0 before proceeding."                                                                                                                                                                   |
| **Negation**              | "Do not use unwrap() or leave errors unhandled."                     | "Handle every Result branch explicitly using `match` or `?`."                                                                                                                                                                            |
| **No-Op**                 | "Be thorough and pay close attention to detail."                     | "Inspect every caller across the workspace with search tools before changing the signature."                                                                                                                                             |
| **Duplication**           | Defining the same review checklist in three different sub-steps.     | Extract the checklist into `resources/manual/CHECKLIST.md` and link via a context pointer.                                                                                                                                               |
| **Sediment**              | Leaving references to deprecated tools or dead directories.          | Audit every line against current tooling; delete stale references aggressively.                                                                                                                                                          |
| **Trigger Ambiguity**     | `description: Helps with tests.`                                     | `description: Write test-first unit and integration tests using red-green-refactor. Use when building a new feature test-first or fixing a bug with regression tests. Do not use for legacy untested code; use /characterization-tests.` |
| **Host Leakage**          | "Run the OpenCode subagent tool with argument X."                    | "Delegate the subtask to your agent's subagent mechanism."                                                                                                                                                                               |
| **Verification Absence**  | "Finish the task and notify the user."                               | "Run `devenv --no-tui test` and `ask skills lint`; confirm zero failures before reporting."                                                                                                                                              |
| **AI Slop / Puffery**     | `description: Expert guidance for developing robust, scalable apps.` | `description: Build and test scalable web services. Use when configuring...`                                                                                                                                                             |
| **Heading Bloat**         | `### 1. **Phase 1: Comprehensive Initial Discovery**`                | `### 1. Discover existing schema`                                                                                                                                                                                                        |
| **Tautological Labeling** | `- **Security:** Implement proper security measures.`                | `- **Security:** Sanitize inputs and restrict CORS origins to authorized hosts.`                                                                                                                                                         |
| **Generic Filler**        | Adding a 20-line 48-hour checklist of generic development steps.     | Define checkable invariants, command executions, and exact exit criteria.                                                                                                                                                                |

Prune sentence by sentence: run the no-op test on each sentence in isolation, and when one fails, delete the whole sentence rather than trim words from it. Be aggressive - most prose that fails should go, not be rewritten.

## After adding or renaming a skill

Regenerate derived artifacts, verify conventions, and update the router:

1. **Self-audit with the unslop lens**:
   - Check description: does it lead with an active verb without "This skill provides..." or "Expert guidance"?
   - Check headings: are all headings plain sentence case without bold markup (`**` or `__`)?
   - Check vocabulary: are AI crutch words like "comprehensive", "utilize", "seamlessly", "furthermore", or "properly" absent?
   - Check advice: does every rule name a concrete mechanism or verifiable invariant rather than a vague platitude?
2. Create or update `agents/openai.yaml` with valid `display_name`, `short_description` (25-64 chars), and `policy`.
3. `devenv --no-tui shell -- ask skills lint` - validate frontmatter, naming, placement, links, formatting, and anti-slop rules (run with `--fix` to sanitize em-dashes).
4. `devenv --no-tui shell -- ask skills sync` - update machine target symlinks and synchronize repository catalogs.
5. `devenv --no-tui shell -- ask dashboard build` - compile markdown, generate search indexes, and update dashboard artifacts.
6. Update the `/skill-router` index so the new or renamed skill is reachable.
7. `devenv --no-tui shell -- prek run --all-files` - run git pre-commit hooks (managed via `prek`, never standalone `pre-commit`).

Done when lint passes, sync leaves no further diff, and the router names the skill.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).

