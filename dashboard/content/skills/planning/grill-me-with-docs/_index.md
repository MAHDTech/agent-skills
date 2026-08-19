+++
title = "grill-me-with-docs"
description = "Relentlessly interview the user to stress-test a problem, architecture, or design from scratch, while actively maintaining a domain glossary (CONTEXT.md) and recording major architectural decisions as ADRs. Use when starting a new initiative, tackling ambiguous domain problems, or when the user asks to grill with docs or ADRs."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


# Grill Me with Docs

The comprehensive entry point into an ambiguous problem or architectural initiative. This skill combines the relentless 1-at-a-time decision interview of `/grilling` with the active domain modeling and decision capture of `/domain-modeling`.

## The Workflow

```text
Problem / Idea
  │
  ├── 1. Relentless Interview Loop (1 decision at a time with recommendations)
  │     ├── Automated fact-finding via codebase / documentation
  │     └── User-confirmed decisions across the design tree
  │
  ├── 2. Live Domain Modeling
  │     └── Update CONTEXT.md whenever vocabulary or boundaries crystallize
  │
  ├── 3. Architectural Decision Records (ADRs)
  │     └── Create docs/adr/000X-*.md when hard-to-reverse trade-offs are settled
  │
  └── 4. Shared Understanding Confirmed → Handoff to /to-spec or /to-tickets
```

## 1. The Interview Loop

- **One decision at a time.** Walk the branches of the design tree sequentially.
- **Provide clear recommendations.** State options with pros/cons, flagging the recommended path.
- **Find facts autonomously.** Inspect the filesystem, code, and configs before asking.
- **Use interactive tooling.** Use `ask_question` or structured selection prompts where available.

## 2. Real-time Glossary (`CONTEXT.md`)

As terms, boundaries, and concepts are clarified during the interview, record them immediately in `CONTEXT.md`. Do not wait until the end of the session.

- **Check against existing terms:** Call out conflicting or overloaded words ("You said 'account' - do you mean Customer or User?").
- **Keep it free of implementation details:** `CONTEXT.md` is a ubiquitous language glossary, not a technical spec.
- Follow the format defined in [CONTEXT-FORMAT.md](@/skills/engineering/domain-modeling/resources/manual/CONTEXT-FORMAT.md).

## 3. Recording Architectural Decision Records (ADRs)

Only create an ADR when a settled decision meets all three criteria:

1. **Hard to reverse** - High cost to change later (e.g. database choice, communication protocol, core boundary).
2. **Surprising without context** - Future engineers would ask "why did they build it this way?".
3. **Real trade-off** - Chosen among genuine alternatives with distinct advantages and disadvantages.

When all three are met, create `docs/adr/000X-slug.md` using the format in [ADR-FORMAT.md](@/skills/engineering/domain-modeling/resources/manual/ADR-FORMAT.md).

## 4. Closing the Session

The session finishes when:

1. All open branches of the design tree are explored and settled.
2. The `CONTEXT.md` glossary reflects all defined domain terms.
3. Key hard-to-reverse choices are recorded in `docs/adr/`.
4. The user explicitly confirms shared understanding.

## Handoffs

- **To Technical Spec:** Run `/to-spec` to synthesize the settled architecture into an issue or PRD.
- **To Backlog Decomposition:** Run `/to-tickets` to break the decisions down into atomic, dependency-mapped tickets.
- **Pure Interview:** If no glossary or ADR documentation is needed, reach for `/grilling`.

