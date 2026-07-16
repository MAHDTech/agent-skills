+++
title = "backlog-create-issue"
description = "Use when creating a new backlog issue/ticket in the `.tars/issues/todo/` directory, defining its YAML frontmatter, headings, tasks, acceptance criteria, evidence collection, and triage review expectations."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


# Backlog Create Issue

This skill defines the canonical structure, format, and content expectations for creating issue tickets in the local backlog. Standardizing this structure ensures coding agents can parse, batch, implement, and review tickets consistently.

## Targets and Paths

- All backlog tickets are stored in the `.tars/issues/todo/` directory relative to the project root.
- File names must follow the format `XXX.md`, where `XXX` is a sequential 3-digit ID starting from `001`.

## Canonical Ticket Format

Every backlog ticket must follow this template:

```markdown
---
id: XXX
name: "Short descriptive name of the issue"
description: "Detailed description of the issue"
component: "module/name"
priority: "low" | "medium" | "high"
type: "bug" | "feature" | "refactor" | "security"
estimation: "2h"
dependencies: []
status: todo
batch: null
files:
  - path/to/file1.ts
---

# XXX — Issue Name

## Description

A comprehensive explanation of the problem, the context in which it occurs, and why the change is necessary. State any known side effects or codebase coordinates.

## Tasks

Detailed list of concrete developer tasks to complete:

- [ ] Task 1 (e.g. Add validation logic)
- [ ] Task 2 (e.g. Write integration test)

## Acceptance Criteria

Explicit, checkable conditions that must be satisfied for the ticket to be considered complete:

- [ ] Criterion 1 (e.g. `devenv test` passes without error)
- [ ] Criterion 2 (e.g. Invalid input is caught and returns exit code 1)

## Evidence

_(This section starts empty when the ticket is created. It is populated by the implementation agent before completion.)_

Must contain command logs, test runs, or code diffs demonstrating that all Acceptance Criteria are met.

## Review

_(This section is empty when the ticket is created. It is appended by the triage agent during backlog-triage.)_
```

## Review & Verification Guidelines

### During Backlog Triage (`backlog-triage`)

The triage agent will review the ticket and append a `## Review` section containing:

- **Codebase Check**: Verification that all files/directories referenced in the ticket actually exist.
- **Hallucination Check**: Ensuring no deprecated APIs or incorrect function signatures are referenced.
- **Readiness Verdict**: A list of findings or a clear statement that the ticket is ready for implementation.

### During Backlog Implementation (`backlog-implement`)

The implementation agent must:

1. Complete all checkboxes in the `## Tasks` and `## Acceptance Criteria` sections.
2. Run `devenv test` (or the project's test command) and verify the tests pass.
3. Document terminal outputs or test run success under the `## Evidence` section.
4. Conventional commits must be used when committing the changes.

