# Glossary — Authoring a Skill

The domain model for what makes a skill great in this repo. A skill exists to wrangle determinism out of a stochastic system; the root virtue is **Predictability**, and every term below is a lever on it. This is the disclosed reference for [`SKILL.md`](../SKILL.md).

The terms are grouped by axis: **Repo Conventions** (where a skill lives and how it is named), **Invocation** (how a skill is reached), **Information Hierarchy** (how its content is arranged), **Steering** (how the agent's runtime behaviour is shaped), and **Pruning** (how it is kept lean). Each **failure mode** lives beside the lever that cures it, tagged _failure mode_.

**Bold terms** in any definition are themselves defined in this glossary; find them by their heading.

## Predictability

The degree to which a skill makes the agent behave the same _way_ on every run — the same process, not the same output (a brainstorming skill should _predictably_ diverge; its tokens vary, its behaviour does not). The root virtue every other term serves — cost and maintainability are symptoms of it, not rivals.

_Avoid:_ consistency, reliability, robustness, output-determinism

## Repo Conventions

Where a skill lives, and how it is named and framed in this repo.

### Category

The topic bucket a skill lives in — one of `engineering`, `planning`, `review`, `github`, `reflection`, `writing`, `authoring`, `tooling`. Set by the directory (`skills/<category>/<name>/`), never by a frontmatter key. A skill's category decides its section in the generated index.

_Avoid:_ tag, type, kind, group

### Lifecycle Bucket

A directory outside the topic tree that holds skills not ready for or retired from **promotion**: `in-progress/` for drafts, `deprecated/` for retired skills. A skill moves between a lifecycle bucket and a **category** to change its promotion state.

_Avoid:_ folder, stage, status

### Promotion

The state of a skill appearing in the generated README and index, earned by living under one of the eight **category** buckets. Skills in a **lifecycle bucket** (`in-progress/`, `deprecated/`) are deliberately excluded. Move a skill into a category to promote it; move it to `deprecated/` to retire it.

_Avoid:_ publishing, listing, inclusion

### Canonical Frontmatter

The YAML header a skill in this repo is allowed to carry: `name` and `description` always, plus the earned optional keys (`disable-model-invocation`, `argument-hint`, `context`/`agent`, `metadata`). It excludes the legacy `custom:` block and the `triggers:`/`category:`/`type:` keys — **Category** comes from the directory and triggers live in the **description**.

_Avoid:_ header, metadata block, yaml

## Invocation

How a skill is reached — and the two loads you pay for the choice.

### Model-Invoked

A skill that keeps its **description**, so the agent can see it and fire it autonomously — and the human can still type its name, so model-invocation always _includes_ user reach. Pays a permanent **context load** on every turn in exchange for that discoverability. Reachable by other skills, because the description that makes it agent-discoverable makes it invocable. Pick it only when the agent must reach the skill on its own; if it never fires except by hand, drop the description and pay no context load.

_Avoid:_ ability, tool, capability

### User-Invoked

A skill with its **description** stripped — invisible to the agent and reachable only by the human typing its name (user-_only_, where **model-invoked** is user-_and-agent_). Set by `disable-model-invocation: true`. Trades agent-discoverability for zero **context load**. Because it has no description, nothing but the human can reach it: no other skill can fire it.

_Avoid:_ procedure, workflow, command

### Description

The skill's model-facing trigger, and the one **context pointer** a **model-invoked** skill is forced to keep loaded at all times. Its presence _is_ the invocation axis: keep it and the skill is model-invoked (and reachable by other skills); set `disable-model-invocation: true` and it becomes human-facing, the skill **user-invoked**. The source of a model-invoked skill's **context load**.

_Avoid:_ frontmatter, summary

### Context Pointer

A reference held in the agent's context that names some out-of-context material and encodes the condition for reaching it. The **description** is the top-level context pointer (context window → skill); pointers to disclosed files are the same object one level down. Its wording, not the target, decides _when_ the agent reaches — and _how reliably_. A must-have target behind a weakly worded pointer is a variance bug: fix the wording first, and inline the material only if sharpening fails.

_Avoid:_ link, reference, import

### Context Load

The cost a **model-invoked** skill imposes on the agent's context window — its **description**, always loaded, spending both tokens and attention. What **user-invoked** skills escape by having no description, and the brake on splitting into more model-invoked skills.

_Avoid:_ token cost, context bloat

### Cognitive Load

The cost a **user-invoked** skill imposes on the human — what they must hold in their head: which skills exist and when to reach for each (the human is the index). What **model-invocation** removes by being agent-discoverable, and the brake on splitting into more user-invoked skills. Not a cost to minimise: it is the price of human agency. Spend it where human judgement matters; remove it where it does not.

_Avoid:_ human index, burden, overhead

### Router Skill

A **user-invoked** skill whose job is to point at your other user-invoked skills — naming each and when to reach for it — so the human has one skill to remember instead of many. In this repo, `/skill-router` plays that role and is updated whenever a skill is added or renamed. It can only hint, never fire them: user-invoked skills have no **description**. The cure for **cognitive load** when user-invoked skills multiply.

_Avoid:_ dispatcher, menu, registry, index

### Granularity

How finely you divide skills. Finer division spends one of the two loads: more **model-invoked** skills spend **context load**; more **user-invoked** skills spend **cognitive load**. Two cuts guide the division. By **invocation**, split off a model-invoked skill where you have a distinct **leading word** to trigger it. By **sequence**, split a run of **steps** where a step's **post-completion steps** need hiding. Beware the reverse: merging sequences exposes each step's post-completion steps to what follows, inviting premature completion.

_Avoid:_ chunking, modularity

## Information Hierarchy

How a skill's content is arranged, and how far down the ladder each piece sits.

### Information Hierarchy (the ladder)

A skill's content ranked by how immediately the agent needs it — a single ladder with three rungs: **steps** (in `SKILL.md`, primary), **reference** in `SKILL.md` (secondary), and **reference** disclosed to a sibling file (behind a **context pointer**). A skill with no steps uses just the bottom two rungs — often a legitimately flat peer-set, which is a fine arrangement, not a smell. Independent of invocation: a skill can be model- or user-invoked whether it is all steps, all reference, or both. Keep the top legible; push down it whatever you can.

_Avoid:_ structure, organization, layout

### Steps

The ordered actions the agent performs — when a skill has them, the primary tier of its content, the part that earns its place in `SKILL.md`. Not every skill has steps: a skill can be all steps, all **reference**, or both, independent of invocation. Every step ends on a **completion criterion**, clear or vague.

_Avoid:_ workflow, instructions, choreography

### Reference

Material the agent refers to on demand — definitions, facts, parameters, examples, conditional instructions. When a skill has **steps** it is secondary to them; when a skill has none it is the entire content. Reached via **context pointers**, and the prime candidate for **progressive disclosure** into a sibling file (`resources/`, `GLOSSARY.md`).

_Avoid:_ supporting material, docs, background

### External Reference

**Reference** that lives outside `SKILL.md` — a sibling file with no **description** and no **steps**, reached only by a **context pointer**. The home for material only some **branches** need, and (as a disclosed file) still part of the skill. Pushing reference here is how the top of the ladder stays legible.

_Avoid:_ doc, resource, knowledge base

### Progressive Disclosure

Moving **reference** down the ladder — out of `SKILL.md` and behind a **context pointer** — so the top stays legible. Not primarily a token optimisation; it is how the **information hierarchy** is protected. Licensed by **branching**: disclose what only some branches need, inline what every path needs, and if a pointer fires unreliably on must-have material, sharpen its wording before pulling it back inline.

_Avoid:_ lazy loading, chunking

### Co-location

Keeping the material an agent needs at once in one place — a concept's definition, rules, and caveats under a single heading, not scattered across the file — so reading one part brings its neighbours with it. The within-file companion to the **information hierarchy**: the hierarchy ranks _how far down_ a piece sits; co-location decides _what sits beside it_ once there. Distinct from **duplication**: that repeats one meaning in two places, where scattering fragments a single meaning across many.

_Avoid:_ grouping, clustering, cohesion

## Steering

The levers that shape the agent's runtime behaviour toward **Predictability**.

### Branch

A distinct way a skill can be invoked — a case the skill handles — so different runs take different paths through it. A skill with many steps may carry many branches; a linear one has none. The test that licenses **progressive disclosure**: inline what every branch needs, disclose what only some reach.

_Avoid:_ path, case, fork

### Leading Word

A compact concept — also called a _Leitwort_ — already living in the model's pretraining, that the agent thinks with while running the skill. It encodes a behavioural principle in the fewest tokens by invoking priors the model already holds (e.g. _seam_, _fog of war_, _tracer bullet_). Repeated as a token, never re-explained as a sentence, it accumulates a distributed definition across the skill. Coining your own works only if you define it clearly; a made-up word recruits no priors, so reach for an existing one first.

A leading word serves **predictability** twice. In the body it anchors _execution_ — the agent reaches for the same behaviour every time the concept appears. In the **description** it anchors _invocation_ — and when the same word lives in your prompts, docs, and codebase, the agent links that shared language to the skill and fires it more reliably.

_Avoid:_ keyword, term, motif

### Completion Criterion

The condition that tells the agent a unit of work is done — the target it judges against. Two properties make it a lever. Its **clarity** (can the agent tell done from not-done?) resists **premature completion**; this axis needs _steps_ to bite. Its **demand** (how much it requires) sets **legwork** — "every modified model accounted for" forces thorough work where "produce a change list" does not — and this axis is _not_ step-bound: it can bind a body of flat **reference** too ("every rule applied"), which is how a skill with no steps still carries an exhaustiveness bar. The strongest criteria are both checkable and exhaustive.

_Avoid:_ done condition, exit condition, stopping rule

### Legwork

The work an agent does behind the scenes within a single step — reading files, exploring the codebase, digging up what it needs rather than offloading to the user. It lives below the step structure: never written as its own step, latent in the wording. The within-step counterpart to **post-completion steps**' across-step pull. Raised by a **leading word** (_comprehensive_, _thorough_) or a **completion criterion** that demands the work be exhaustive. Goes thin either when that demand is missing or when **premature completion** cuts the step short.

_Avoid:_ scope, effort, diligence, coverage

### Post-Completion Steps

The **steps** that follow the current step. Visible, they pull the agent forward into **premature completion** — the more it sees, the stronger the tug; the defence is to hide them by splitting the sequence across a real context boundary.

_Avoid:_ horizon, lookahead

### Premature Completion

_Failure mode._ Ending the current step before it is genuinely done, because the agent's attention slips to being done rather than to the work. A between-steps failure: it needs **steps** to occur. A tug-of-war between visible **post-completion steps** (the pull forward) and the **completion criterion**'s clarity (the resistance). Reach for the two levers in order: **sharpen the bound first** — local and cheap. Only when the criterion is irreducibly fuzzy _and_ you observe the rush do you **hide the later steps** by splitting — and hiding works only across a real context boundary (a user-invoked hand-off or a `context: fork` subagent; an inline model-invoked call clears nothing).

_Avoid:_ premature closure, the rush, rushing, shortcutting

### Negation

_Failure mode._ Steering by prohibition — telling the agent what _not_ to do — which drags the forbidden behaviour into context and makes it _more_ available, not less. _Don't think of an elephant_, and the elephant is all there is. The ban half-reads as an instruction to do the thing. Cure: prompt the **positive** — describe the target behaviour so the banned one is never spoken. A prohibition earns its place only as a hard guardrail on a behaviour you cannot phrase positively; even then, pair it with the positive target.

_Avoid:_ ironic rebound, don't-prompting, the pink elephant

## Pruning

Keeping a skill lean — each remedy paired with the failure it cures.

### Single Source of Truth

The desired state where each meaning lives in exactly one authoritative place, so a change to the skill's behaviour is a change in one place. **Duplication** is its violation.

_Avoid:_ home, canonical location

### Duplication

_Failure mode._ The same meaning given more than one **single source of truth**. It costs maintenance (change one place, you must change the others), costs tokens, and inflates prominence — repeating a meaning weights it on the ladder past its real rank. The accidental inverse of a **leading word**, which raises attention on purpose by repeating a token, never the meaning.

_Avoid:_ repetition, redundancy

### Relevance

Whether a line still bears on what the skill does — the lens for what to keep. A line loses relevance either by never bearing on the task (mere exposition, or a **branch** that should be disclosed) or by going stale as the behaviour or world it describes changes. Shorter skills are easier to keep relevant, because each line is cheaper to check. Distinct from **no-op**: relevance asks whether a line bears on the task, not whether it changes behaviour.

_Avoid:_ load-bearing, staleness, freshness

### Sediment

_Failure mode._ Layers of old content that settle in a skill and are never cleared, because adding feels safe and removing feels risky — so stale lines accumulate and you must core down through them to find what is still live. The default fate of any skill without a pruning discipline; the slow erosion of **relevance**, as opposed to **duplication**'s repeated meaning.

_Avoid:_ accretion, bloat, cruft, rot

### Sprawl

_Failure mode._ A skill that is simply too long — too many lines in `SKILL.md` — independent of whether they are stale or repeated. Even an all-live, all-unique skill can sprawl. It costs readability, maintainability, and tokens. The cure is the **information hierarchy**: push **reference** down behind **context pointers**, and split by **branch** or sequence so each path carries only what it needs. Distinct from **sediment** (length from stale accumulation) and **duplication** (length from repeated meaning) — sprawl is length itself, whatever its cause.

_Avoid:_ bloat, length, size, verbosity

### No-Op

_Failure mode._ An instruction that changes nothing because the model already does it by default — you pay load to tell the agent what it would do anyway. The test: does a line change behaviour versus the default? A line can be perfectly **relevant** and still be a no-op. A **leading word** too weak to beat the default is a no-op (_be thorough_ when the agent is already thorough-ish), and the fix is a stronger word that passes the verdict (_relentless_), not a different technique. Model-relative, not reader-relative: settle a disagreement by running the skill, not by debate.

_Avoid:_ redundant instruction, restating the obvious, belaboring
