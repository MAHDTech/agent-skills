---
name: handoff
description: Summarize the current conversation into a temporary handoff for another agent or session. Use when transferring unfinished work; reference durable artifacts, suggest relevant skills, and redact secrets.
argument-hint: "What will the next session be used for?"
metadata:
  group: planning-pipeline
  source: mattpocock/skills
  license: MIT
---

# Handoff

Write a handoff document summarising the current conversation so a fresh agent can continue the work. Save it to the operating system's temporary directory - **not** the current workspace - so it isn't committed by accident, and return the exact saved file path to the user.

This handoff is deliberately **throwaway** - a temporary baton for the next agent. For a durable plan that lives in the repo and gets committed, use `/store-plan` instead.

If the user passed arguments, treat them as a description of what the next session will focus on and tailor the document accordingly.

## Required sections

Include these sections in the handoff document (keep each concise and omit filler):

- **Goal**: What the effort is trying to achieve and what the next session should focus on.
- **Completed work**: Decisions reached and changes already landed or verified.
- **Remaining work and next steps**: Concrete ordered actions for the next agent to take first.
- **Open questions**: Unresolved blockers, trade-offs, or decisions still waiting on input.
- **Artifact pointers**: Paths or URLs to existing specs, plans, ADRs, issues, commits, or diffs. Reference the source of truth rather than duplicating its content.
- **Suggested skills**: Skills the next agent should invoke (for example `/prioritize-backlog`, `/wayfinder`, or whichever fit the work ahead) and why each is relevant.

## Redaction

Redact any sensitive information, such as API keys, passwords, tokens, or personally identifiable information. If a secret is load-bearing for the next session, reference where it lives (e.g., an env var name or secrets manager path) rather than the value itself.

## Done when

- The handoff file is saved in the OS temporary directory outside the current workspace.
- Every required section is present, with durable artifacts referenced by path or URL rather than copied.
- All secrets and PII are verified redacted, and the exact saved file path is reported to the user.

---

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).
