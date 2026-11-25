---
name: store-plan
description: Save an agreed conversation plan, decisions, action items, and open questions to a reviewable Markdown file in docs/plans/. Use when a durable repository plan is needed rather than a temporary handoff.
argument-hint: "[description]"
metadata:
  group: planning-pipeline
---

# Store Plan

Capture the current conversation into a structured, reviewable plan file.

## Instructions

### 1. Gather context

Scan the full conversation for:

- **Decisions** - what was agreed on and why
- **Action items** - concrete next steps with owners/priorities
- **Architecture or design choices** - trade-offs discussed
- **Risks and open questions** - unresolved items
- **Phases or ordering** - any sequencing that was discussed

If the user passed a description as an argument (e.g., `/store-plan auth refactor`), normalize it to lowercase `snake_case` as `{description}`. Otherwise, infer a 2-4 word lowercase `snake_case` slug from the conversation topic.

### 2. Generate the plan document

Build the document using the format defined in [Plan format and example](#plan-format-and-example). Not every section is required - **omit sections that have no content** rather than leaving them empty. The goal is a document that someone (including future-you) can pick up cold and understand what was decided, what's next, and why.

### 3. Confirm and write

Present the draft and its target path to the user, and ask if they want to adjust anything. Once they approve, write the file to the path defined in [Output path](#output-path), creating `docs/plans/` if needed, and report the path.

## Output path

```text
docs/plans/{YYYY}_{MM}_{DD}_{description}.md
```

Format `{description}` in lowercase `snake_case` (words separated by underscores, no spaces or punctuation).

Examples:

- `docs/plans/2026_04_02_auth_refactor.md`
- `docs/plans/2026_04_02_frontend_split.md`
- `docs/plans/2026_04_02_migration_engine_redesign.md`

## Plan format and example

The full document structure (section by section), the formatting rules (status circles, no tables, headings, code references), and a complete worked example live in [resources/manual/plan-format.md](resources/manual/plan-format.md).

Read that file and follow its format when generating the plan. In short: lead with a ToC and an italic tl;dr, then include only the sections that have real content - Context, Decisions Made, Plan Overview, Action Items (grouped by P0/P1/P2), per-phase detail, Risks & Open Questions, and References. Use colored status circles as list indicators and structured lists instead of Markdown tables.
