---
name: to-spec
description: Synthesize the current conversation into a product spec or PRD and publish it to the project's issue tracker. Use when asked to formalize discussed requirements and testing decisions without a new interview.
disable-model-invocation: true
metadata:
  group: planning-pipeline
  source: mattpocock/skills
  license: MIT
---

# To Spec

**Invocation:** Start only when the user explicitly requests this workflow.

Take the current conversation context and codebase understanding and produce a spec (you may know this document as a PRD). Synthesize what you already know rather than running a new interview; check in with the user only to confirm testing seams (step 2) and review the draft before publishing (step 3).

Work from your project's conventions: its issue tracker (GitHub via `gh`, Linear, or local files), triage label vocabulary, domain glossary, and any ADRs in the area you are touching. If those conventions have not been established, ask the user before publishing.

## Process

1. Explore the repo to understand the current state of the codebase, if you have not already. Use the project's domain glossary throughout the spec, and respect any ADRs in the area you are touching.

2. Sketch out the seams at which you will test the feature. Prefer existing seams to new ones, and use the highest seam possible. If new seams are needed, propose them at the highest point you can. The fewer seams across the codebase, the better - the ideal number is one.

   Confirm the seams with the user in a single message, then continue.

3. Draft the spec using the template below and present it to the user for review and confirmation before publishing to the external issue tracker (skip the confirmation pause only if the user already asked to publish directly). Once confirmed, publish it to the project's issue tracker, apply the project's ready-for-agent triage label (or its equivalent) - no need for additional triage - and report the created issue URL or local file path.

Once the spec is approved, slice it into tracer-bullet tickets with `/to-tickets`.

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

Refer to modules, interfaces, and contracts by name rather than specific file paths or code snippets, which go stale quickly.

Exception: if exploratory prototyping produced a snippet that encodes a
decision more precisely than prose can (a state machine, reducer, schema, or
type shape), inline it within the relevant decision and note briefly that it
came from a prototype. Trim to the decision-rich parts - not a working demo,
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

## Done when

- Every template section (`Problem Statement`, `Solution`, `User Stories`, `Implementation Decisions`, `Testing Decisions`, `Out of Scope`, `Further Notes`) is populated or explicitly noted as none.
- The user confirmed the testing seams and approved the drafted spec before publishing (unless they already requested direct publication).
- The spec is published with the project's ready-for-agent triage label (or equivalent) applied, and the created issue URL or local file path has been returned to the user.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).
