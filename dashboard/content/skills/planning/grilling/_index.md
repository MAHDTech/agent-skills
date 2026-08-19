+++
title = "grilling"
description = "Interview the user relentlessly, one question at a time, to stress-test a plan, design, or architecture before any code is written. Use when the user asks to 'grill me', 'poke holes in this', 'stress-test this plan', or 'interview me about this design'. Walk every branch of the design tree, look up facts in the codebase, and put each real decision to the user with a recommended answer before proceeding."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


# Grilling

Interview the user relentlessly about every aspect of a plan, feature, or architecture until you reach a shared understanding. Map the problem as a **design tree**: root decisions branch into dependent technical and product decisions.

## The Interview Rules

1. **One decision at a time.** Ask questions strictly one at a time. Never dump multiple questions in a single turn - asking several questions at once creates cognitive overload and tangles branching decisions.
2. **Always give a recommended answer.** Never ask open-ended questions without guidance. State the options, explain the trade-offs, and state your clear recommendation (e.g. `(Recommended)`).
3. **Lookup facts yourself; ask the user for decisions.**
   - **Facts belong to the agent.** If a detail can be determined by reading the codebase, configuration, git history, or documentation, search for it autonomously. Never ask the user questions you could answer yourself.
   - **Decisions belong to the user.** Scope, trade-offs, architecture choices, and product behavior require user intent.
4. **Use interactive question tools when available.** If the agent environment provides an interactive choice tool (e.g. `ask_question`), use it to present structured choices. Otherwise, format the decision cleanly in Markdown with bold options and a clear recommendation.
5. **Walk the branches systematically.** When a decision is made, advance down that specific branch of the design tree. Recompute the next unblocked decision until no unresolved branches remain.
6. **Do not enact code prematurely.** Continue the interview until the frontier of decisions is empty and the user confirms a shared understanding.

## Question Format (when tool is unavailable)

```text
❓ **<Decision Title>**

<Context, constraints, and trade-offs>

- **Option A (Recommended):** <Details and rationale>
- **Option B:** <Details and rationale>
```

## Handoffs

- When the interview concludes and the plan is locked in, hand off to `/to-spec` (to generate a PRD/spec) or `/to-tickets` (to decompose into implementation tasks).
- If the problem involves domain vocabulary modeling or requires formal Architectural Decision Records, use `/grill-me-with-docs`.

