+++
title = "grilling"
description = "Interview the user one decision at a time before coding, optionally recording a glossary and ADRs. Use when explicitly asked to stress-test a design, or when a wayfinder run needs its planning interview."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


# Grilling

Interview the user relentlessly about every aspect of a plan, feature, or architecture until you reach a shared understanding. Map the problem as a **design tree**: root decisions branch into dependent technical and product decisions.

## Interview rules

1. **One decision at a time.** Ask a single question per turn, so each answer can settle its branch before the next one opens.
2. **Recommend an answer.** State the options, explain the trade-offs, and mark your recommendation (e.g. `(Recommended)`).
3. **Look up facts yourself; ask the user for decisions.**
   - **Facts belong to the agent.** Answer anything the codebase, configuration, git history, or documentation can tell you by searching for it.
   - **Decisions belong to the user.** Scope, trade-offs, architecture choices, and product behavior require user intent.
4. **Use interactive question tools when available.** If the agent environment provides an interactive choice tool, use it to present structured choices. Otherwise, use the question format below.
5. **Walk the branches systematically.** When a decision is made, advance down that branch of the design tree. Recompute the next unblocked decision until no unresolved branches remain.
6. **Keep interviewing until the frontier is empty.** Start implementation only after every branch is resolved and the user confirms a shared understanding.

## Documentation mode (`--with-docs` or when requested)

When the user asks to "grill with docs", "record ADRs", or passes `--with-docs`, keep a glossary and decision records alongside the interview.

### Glossary (`CONTEXT.md`)

Record each term, boundary, and concept in `CONTEXT.md` as soon as it is clarified during the interview.

- **Check against existing terms.** Call out conflicting or overloaded words ("You said 'account' - do you mean Customer or User?").
- **Keep it in domain language.** `CONTEXT.md` is a ubiquitous-language glossary: definitions, boundary rules, and rejected terms. Implementation details belong in specs.

### Architectural decision records (ADRs)

Create an ADR when a settled decision meets all three criteria:

1. **Hard to reverse** - high cost to change later (e.g. database choice, communication protocol, core boundary).
2. **Surprising without context** - future engineers would ask "why did they build it this way?".
3. **Real trade-off** - chosen among genuine alternatives with distinct advantages and disadvantages.

Write each ADR with context, decision, and consequences. Follow the project's existing ADR location and numbering convention; if it has none, use `docs/adr/NNNN-<slug>.md`.

## Question format (when no choice tool is available)

```text
**<Decision title>**

<Context, constraints, and trade-offs>

- **Option A (Recommended):** <Details and rationale>
- **Option B:** <Details and rationale>
```

## Handoffs

- When the interview concludes and the plan is locked in, hand off to `/to-spec` (to generate a spec) or `/to-tickets` (to decompose into implementation tasks).
- In documentation mode, confirm the ADRs and the `CONTEXT.md` glossary are written before handing off.

