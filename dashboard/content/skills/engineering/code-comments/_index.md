+++
title = "code-comments"
description = "Write concise, high-signal code comments and prune comment bloat. Use when writing new code, editing existing files, or auditing code comments via /code-comments to keep comments minimal, explain non-obvious intent (\"why, not what\"), and avoid restating self-explanatory code."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Code Comments

Guidelines and audit workflows for writing concise, high-signal code comments.

Code should speak for itself. High quality code relies on expressive naming, clean modularization, and type signatures to communicate intent. Comments are reserved strictly for non-obvious context that the code cannot express on its own.

## When to Use This Skill

- Writing new code or editing existing files (automated model behavioral guide).
- Reviewing diffs or PRs for comment hygiene and verbosity.
- Explicitly requested via `/code-comments` or `/code-comments <path>` to audit and prune bloated, redundant, or noisy comments.

## Core Rules

### 1. Why, Not What

Comments must explain **intent, constraints, invariants, or edge-case rationale** - never restate the syntax or mechanics of what the code is doing.

- **Bad:** Restating what is already visible in syntax.

  ```typescript
  // Loop through items and push active items to the array
  for (const item of items) {
    if (item.isActive) {
      activeItems.push(item)
    }
  }
  ```

- **Good:** Code is self-explanatory; no comment needed.

  ```typescript
  const activeItems = items.filter((item) => item.isActive)
  ```

- **Good:** Comment explains a non-obvious workaround, external constraint, or invariant.

  ```typescript
  // Upstream API truncates payloads at 64KB without returning an error code
  const CHUNK_SIZE_BYTES = 64 * 1024
  ```

### 2. Refactor Over Comment

If code requires a paragraph to explain its mechanics, refactor the code instead of adding comments:

- Rename cryptic variables and parameters to descriptive names.
- Extract complex boolean conditions into well-named helper booleans or functions.
- Break long, multi-purpose functions into small, single-purpose units.

### 3. Brevity Over Verbosity

- **No essays:** Keep comments strictly to 1 or 2 concise lines. Never write multi-paragraph dissertations for a few lines of implementation.
- **No decorative banners or headers:** Avoid ASCII art, dividing lines (`// ====================`), and redundant section headers.
- **No redundant docstrings:** Do not duplicate type signatures in JSDoc or docstrings (e.g. `@param {string} name The name parameter`). Use docstrings only on public APIs where consumer-facing behavior is non-obvious.

### 4. Zero Tolerance for Stale Comments & AI Tells

- **No commented-out code:** Delete unused or dead code; version control tracks history.
- **No changelogs / authorship in code:** Do not add "Modified by AI on date" or inline changelog notes.
- **Never use em-dashes:** Do not use Unicode U+2014 em-dashes in comments. Use standard hyphens (`-`), colons, commas, or parentheses.

---

## Comment Decision Matrix

When writing or reviewing comments, classify each comment into one of four actions:

| Category                 | Indicator                                                                           | Action                                                        |
| :----------------------- | :---------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| **Redundant / Obvious**  | Restates function name, variable assignment, loop, or type                          | **Delete immediately**                                        |
| **Bloated Essay**        | Multi-sentence explanation of straightforward logic                                 | **Trim to 1 concise line** focusing on the "why"              |
| **Cryptic Code Smell**   | Long comment explaining convoluted logic                                            | **Refactor code** (rename, extract helper) and remove comment |
| **Legitimate Invariant** | Explains upstream bugs, mathematical formulas, hardware quirks, or security reasons | **Keep** as a brief, single-line note                         |

---

## Interactive Audit Workflow (`/code-comments`)

When the user runs `/code-comments` or requests a comment audit:

1. **Target Discovery:**
   - If a file or path is specified (e.g. `/code-comments src/auth.ts`), scan that file.
   - If no target is given, inspect the current working changes (`git diff` or modified files).

2. **Audit & Classify:**
   - Scan all comments in the target scope.
   - Categorize each comment using the Comment Decision Matrix (Delete / Trim / Refactor / Keep).

3. **Present Summary & Plan:**
   - Show a concise table of findings:
     - File path and line numbers
     - Current comment text
     - Proposed action (Delete, Trim to single line, or Refactor)
     - Rationale

4. **Surgical Execution:**
   - Apply edits cleanly without touching unrelated formatting or logic.
   - Verify code compiles and passes tests before finishing.

