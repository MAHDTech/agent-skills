+++
title = "gate-3-program-design"
[extra]
skill = false
category = "planning"
mermaid = false
skill_name = "software-factory"
+++

# Gate 3 - Program Design

Saved as `docs/plans/<feature-slug>/03-program-design.md`. This is the gate that catches the decisions an agent otherwise makes silently at implementation time.

```markdown
# Program Design: <feature name>

## Files

<every file created or changed, one line each on why the code lives there>

## Types and signatures

<code blocks defining the types, interfaces, and method signatures - no bodies.
A reader scans these and says "right" or "wrong" in seconds.>

## Call stack

<for each main flow, what calls what from top to bottom>

## Test plan

<the test case names and what each one asserts, written before any of them exist.
Name the seam each test drives - prefer the highest existing seam.>

## Least confident decisions

<a numbered list of the calls most worth challenging now, while changing them is free.
Being unable to name any is a sign the design has not been thought through yet.>
```

The last section is the point of the gate: put it in front of the user explicitly during the approval protocol, because it is the cheapest moment this feature will ever have to change its mind.

