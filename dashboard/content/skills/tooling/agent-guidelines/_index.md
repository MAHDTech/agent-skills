+++
title = "agent-guidelines"
description = "Apply coding discipline while implementing: state assumptions, keep changes minimal and scoped, and verify against concrete success criteria. Use when writing or editing code, not when reviewing finished work."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Agent Guidelines

Behavioral guidelines to reduce common LLM coding mistakes, derived from [Andrej Karpathy's observations on LLM coding pitfalls](https://x.com/karpathy/status/2015883857489522876).

**Tradeoff:** These guidelines bias toward caution over speed. For trivial tasks, use judgment.

## 1. Think before coding

State assumptions explicitly, surface confusion early, and name tradeoffs before writing code:

- State your assumptions up front; ask when uncertain rather than guessing.
- Present multiple valid interpretations to the user instead of picking silently.
- Propose a simpler alternative and push back when warranted.
- Stop when something is unclear, name the ambiguity, and ask before editing.

## 2. Simplicity first

Write the minimum code that solves the requested problem and leave out speculative work:

- Implement only the requested behavior; keep unrequested features, abstractions, and configuration options out of the diff.
- Write straightforward code for single-use paths rather than wrapping them in premature helpers.
- Handle realistic failure modes at system boundaries instead of adding defensive branches for impossible states.
- Rewrite an overly long solution when a shorter, clearer version achieves the same behavior.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical changes

Touch only what the task requires and clean up only what your own edits made unused:

- Keep adjacent code, comments, and formatting untouched; leave working unrelated code as-is.
- Match the surrounding codebase style, even when you would choose a different convention.
- Report unrelated dead code you notice in your summary rather than deleting it unasked.
- Remove imports, variables, and functions that your own changes left unused, while leaving pre-existing dead code alone unless asked.

The test: every changed line traces directly to the user's request.

## 4. Goal-driven execution

Turn each task into a verifiable goal and loop until the check passes:

- "Add validation" -> "Write tests for invalid inputs, then make them pass."
- "Fix the bug" -> "Write a test that reproduces it, then make it pass."
- "Refactor X" -> "Confirm tests pass before and after the refactor."

For multi-step tasks, state a brief plan with a concrete check per step:

```text
1. [Step] -> verify: [check]
2. [Step] -> verify: [check]
3. [Step] -> verify: [check]
```

Strong checks let you verify independently; vague goals ("make it work") force repeated clarification.

## 5. Comments explain why

- Comment only non-obvious context: invariants, external constraints, workarounds, and the reason behind a surprising choice.
- Keep comments to one or two lines; when code needs more explanation, use a clearer name or extract a small helper.

## Completion criteria

- Every assumption or ambiguity that could change behavior was stated or resolved before implementing.
- Every changed line in the diff traces directly to the request, with no speculative abstractions or unrelated reformatting.
- Every new or changed behavior is verified against a concrete command, test, or check before reporting done.

