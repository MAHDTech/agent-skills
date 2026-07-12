+++
title = "to-spec"
description = "Turn the current conversation into a spec (sometimes called a PRD) and publish it to your project's issue tracker — no interview, just synthesis of what you have already discussed."
date = 2026-07-12
[extra]
category = "planning"
mermaid = false
+++


# To Spec

Take the current conversation context and codebase understanding and produce a spec (you may know this document as a PRD). Do NOT interview the user — just synthesize what you already know.

Work from your project's conventions: its issue tracker (GitHub via `gh`, Linear, or local files), triage label vocabulary, domain glossary, and any ADRs in the area you are touching. If those conventions have not been established, ask the user before publishing.

## Process

1. Explore the repo to understand the current state of the codebase, if you have not already. Use the project's domain glossary throughout the spec, and respect any ADRs in the area you are touching.

2. Sketch out the seams at which you will test the feature. Prefer existing seams to new ones, and use the highest seam possible. If new seams are needed, propose them at the highest point you can. The fewer seams across the codebase, the better — the ideal number is one.

   Check with the user that these seams match their expectations.

3. Write the spec using the template below, then publish it to your project's issue tracker. Apply the project's ready-for-agent triage label (or its equivalent) — no need for additional triage.

## Spec template

```markdown
## Problem Statement

The problem the user is facing, from the user's perspective.

## Solution

The solution to the problem, from the user's perspective.

## User Stories

A long, numbered list of user stories. Each user story should be in the format:

1. As an <actor>, I want a <feature>, so that <benefit>

For example:

1. As a mobile bank customer, I want to see the balance on my accounts, so that I can make better-informed decisions about my spending.

This list should be extensive and cover all aspects of the feature.

## Implementation Decisions

A list of the implementation decisions that were made. This can include:

- The modules that will be built or modified
- The interfaces of those modules that will change
- Technical clarifications from the developer
- Architectural decisions
- Schema changes
- API contracts
- Specific interactions

Do NOT include specific file paths or code snippets — they go stale quickly.

Exception: if exploratory prototyping produced a snippet that encodes a
decision more precisely than prose can (a state machine, reducer, schema, or
type shape), inline it within the relevant decision and note briefly that it
came from a prototype. Trim to the decision-rich parts — not a working demo,
just the important bits.

## Testing Decisions

A list of the testing decisions that were made. Include:

- A description of what makes a good test (only test external behaviour, not implementation details)
- Which modules will be tested
- Prior art for the tests (similar types of tests already in the codebase)

## Out of Scope

A description of the things that are out of scope for this spec.

## Further Notes

Any further notes about the feature.
```

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).

