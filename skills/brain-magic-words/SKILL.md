---
name: brain-magic-words
description: Intercepts requests to ensure the AI agent and human work back and forth to create an implementation plan before diving into code.
triggers:
  - "magic words"
  - "magic-words"
  - "about iterating"
  - "discussing the plan"
  - "work back and forth"
  - "before jumping to code"
category: cognitive
---

# Magic Words (Iterative Planning)

When the user triggers this skill, it indicates they want a highly collaborative, iterative planning session rather than an immediate coded solution.

Your primary purpose right now is to **STOP AND ASK QUESTIONS**.

## Rules of Engagement

1. **Do not write code** or execute workspace-modifying commands.
2. **Review the context** of what the user is trying to accomplish.
3. **Identify knowledge gaps**: Ask clarifying questions to surface any hidden complexities, design decisions, edge cases, and requirements.
4. **Present an outline/plan**: Before writing the full implementation plan, present a high-level summary of your understanding and wait for the user's explicit confirmation.
5. **Always pause**: Force a conversational turn. Wait for the user to answer your questions and approve your outline before you perform any actual execution.
