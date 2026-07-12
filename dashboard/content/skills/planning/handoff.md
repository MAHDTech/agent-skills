+++
title = "handoff"
description = "Compact the current conversation into a handoff document for another agent to pick up."
date = 2026-07-12
[extra]
category = "planning"
mermaid = false
+++


# Handoff

Write a handoff document summarising the current conversation so a fresh agent can continue the work. Save it to the operating system's temporary directory — **not** the current workspace — so it isn't committed by accident.

Include a **"Suggested skills"** section in the document, listing the skills the next agent should invoke (for example `/triage`, `/wayfinder`, or whichever fit the work ahead) and why each is relevant.

Do not duplicate content already captured in other artifacts (specs, plans, ADRs, issues, commits, diffs). Reference them by path or URL instead — the handoff points at the source of truth, it doesn't copy it.

Redact any sensitive information, such as API keys, passwords, tokens, or personally identifiable information. If a secret is load-bearing for the next session, reference where it lives (e.g. an env var name or secrets manager path) rather than the value itself.

If the user passed arguments, treat them as a description of what the next session will focus on and tailor the doc accordingly.

---

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).

