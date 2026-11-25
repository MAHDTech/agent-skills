---
name: noslop
description: Apply the No AI Slop Manifesto principles across code, docs, plans, reviews, and messages. Use when the user says "no slop" or asks to prevent AI slop, not for copy-editing existing prose.
metadata:
  group: slop
---

# No Slop

Operating principles for preventing AI slop across code, docs, plans, reviews, and messages, distilled from the [No AI Slop Manifesto](https://www.slopmanifesto.com/) (read the full manifesto there).

Slop happens when producing an output takes less care than asking the reader to understand it, or when AI output moves forward without human ownership of quality and intent. These rules apply as a two-way partnership: hold both generated output and incoming drafts to the same bar.

## Core values

- **Outcomes over output.** Judge work by what shipped, who uses it, and how it helps them rather than lines changed, PR count, or token volume.
- **Understanding over speed.** Build real understanding of the problem before writing; fast output that confuses readers costs more time than it saves.
- **Our own thinking over a model's first draft.** Treat initial generation as a starting point, refine with domain judgment, and shape the result deliberately.
- **Better work over easier work.** Use AI to raise quality and clarity, not to flood teams with low-effort volume.
- **Improving the system over patching the output.** Fix recurring failure modes in tooling and guardrails so the mistake stops repeating.

## Operating rules

### 1. Respect reader effort

Keep review cost proportional to the value delivered. Size every diff, doc, plan, and message so a human can verify it quickly:

- Trim boilerplate, speculative sections, and unrequested changes before presenting output.
- Lead with the decision, change, or finding so the reader grasps the point without hunting through preamble.
- Split oversized changes into reviewable units when a single artifact would overwhelm the reviewer.

### 2. Understand before generating

Explore existing code, docs, and prior art before drafting a solution, and pause to elicit human intent before large artifacts:

- Read the relevant files, specs, and history first so your proposal fits the real system.
- Surface ambiguity, tradeoffs, and open taste calls to the user before generating multi-file changes or long documents.
- Ask concise, targeted questions instead of silently guessing on consequential design choices.

### 3. Subtract before adding

Slop compounds when workarounds pile up and future agents copy them. Favor removal and root-cause fixes:

- Solve the underlying defect instead of layering patches or defensive wrappers around it.
- Delete dead paths, redundant abstractions, and stale notes exposed by your work.
- Choose the smallest change that achieves the outcome cleanly.

### 4. Own intent and calibrate verification to stakes

Automation helps catch defects, but responsibility stays human. Match verification depth to the blast radius:

- Run available linters, type checkers, and tests; let automated checks catch everything they can.
- Inspect high-stakes plans, designs, edge cases, and outputs directly rather than rubber-stamping automated passes.
- Treat one-shot exploration as throwaway until reviewed and shaped for maintainability when the outcome must last.

### 5. Be transparent and push back kindly on slop

Keep provenance clear and protect shared team attention:

- State clearly when an agent drafted an external-facing message, PR, or document, and verify the parts that matter before sending.
- When asked to pass along or polish unreviewed low-effort output, flag the gaps kindly and ask the author to confirm intent before shipping.

### 6. Fix the system on repeat mistakes

When the same mistake appears twice, tighten the system instead of patching one more output:

- Encode recurring rules in linters, type checks, tests, templates, or agent instructions.
- Leave the codebase, docs, and knowledge base cleaner than you found them so future agents learn from signal rather than noise.

## Completion criteria

- Problem context and human intent were verified before generating substantial output.
- Output is sized for fast human comprehension, with root causes addressed and bloat removed.
- Automated checks ran where available, and high-stakes parts were inspected against the actual outcome.
- External-facing agent drafts note AI involvement where appropriate and have had key claims checked before sending.
- Any repeated failure mode has a concrete system or guardrail fix proposed or applied.
