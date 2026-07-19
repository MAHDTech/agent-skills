+++
title = "store-plan"
description = "Capture the current conversation's plan, decisions, and action items into a structured, reviewable markdown file in the project's plans/ directory."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


<!-- omit in toc -->

# Store Plan

Capture the current conversation into a structured, reviewable plan file.

- [Instructions](#instructions)
  - [1. Gather context](#1-gather-context)
  - [2. Generate the plan document](#2-generate-the-plan-document)
  - [3. Write and confirm](#3-write-and-confirm)
- [Output Path](#output-path)
- [Plan Format and Example](#plan-format-and-example)

## Instructions

### 1. Gather context

Scan the full conversation for:

- **Decisions** — what was agreed on and why
- **Action items** — concrete next steps with owners/priorities
- **Architecture or design choices** — trade-offs discussed
- **Risks and open questions** — unresolved items
- **Phases or ordering** — any sequencing that was discussed

If the user passed a description as an argument (e.g., `/store-plan auth refactor`), use it as `DESCRIPTION`. Otherwise, infer a 2-4 word slug from the conversation topic.

### 2. Generate the plan document

Build the document using the format defined in [Plan Format and Example](#plan-format-and-example). Not every section is required — **omit sections that have no content** rather than leaving them empty. The goal is a document that someone (including future-you) can pick up cold and understand what was decided, what's next, and why.

Present the draft to the user before writing.

### 3. Write and confirm

**Step 3a — Write the file:**

```bash
mkdir -p plans/
```

Write to the path defined in [Output Path](#output-path).

**Step 3b — Confirm with the user:**

Present the file path and a brief summary to the user. Ask if they want to adjust anything before finalizing.

## Output Path

```text
plans/{YYYY}_{MM}_{DD}_{description}.md
```

Examples:

- `plans/2026_04_02_auth_refactor.md`
- `plans/2026_04_02_frontend_split.md`
- `plans/2026_04_02_migration_engine_redesign.md`

## Plan Format and Example

The full document structure (section by section), the formatting rules (status circles, no tables, headings, code references), and a complete worked example live in [resources/manual/plan-format.md](@/skills/planning/store-plan/resources/manual/plan-format.md).

Read that file and follow its format when generating the plan. In short: lead with a ToC and an italic tl;dr, then include only the sections that have real content — Context, Decisions Made, Plan Overview, Action Items (grouped by P0/P1/P2), per-phase detail, Risks & Open Questions, and References. Use colored status circles as list indicators and structured lists instead of Markdown tables.

