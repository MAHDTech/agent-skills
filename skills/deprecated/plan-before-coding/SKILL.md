---
name: plan-before-coding
description: Intercepts requests to ensure the AI agent and human work back and forth to create an implementation plan before diving into code. Use when the user invokes "magic words" or wants to collaboratively iterate on a plan before writing code.
---

> **Deprecated - use `/grilling` instead.** Grilling is the reusable interview-before-coding primitive and covers this collaborative planning need without harness-specific tooling. Kept here for reference only.

# Magic Words (Iterative Planning)

When the user triggers this skill, it indicates they want a highly collaborative, iterative planning session rather than an immediate coded solution.

Your primary purpose right now is to **STOP AND ASK QUESTIONS**.

## Rules of Engagement

1. **Do not write code** or execute workspace-modifying commands.
2. **Review the context** of what the user is trying to accomplish.
3. **Identify knowledge gaps**: Ask clarifying questions to surface any hidden complexities, design decisions, edge cases, and requirements.
4. **Use `todowrite` tool**: You MUST explicitly call the `todowrite` tool to generate the initial plan outline. Do not write the plan in a standard markdown list. Use the `todowrite` tool to create a visual, interactive checklist of the implementation plan. Leave the status of these tasks as `pending`.
5. **Always pause**: Force a conversational turn. Present your understanding and wait for the user to answer your questions and approve the `todowrite` plan outline before you perform any actual execution or mark any task as `in_progress`.
