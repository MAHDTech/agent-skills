---
name: code-comments
description: Audit and rewrite code comments in a specified file, path, or diff so they state non-obvious intent, constraints, and invariants. Use when the user asks for a comment audit or comment cleanup.
---

# Code Comments

Code should speak for itself. High quality code relies on expressive naming, clean modularization, and type signatures to communicate intent. Comments are reserved strictly for non-obvious context that the code cannot express on its own.

## Core rules

### 1. Why, not what

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

### 2. Refactor over comment

If code requires a paragraph to explain its mechanics, refactor the code instead of adding comments:

- Rename cryptic variables and parameters to descriptive names.
- Extract complex boolean conditions into well-named helper booleans or functions.
- Break long, multi-purpose functions into small, single-purpose units.

### 3. Brevity over verbosity

- Keep each comment to one or two lines stating the reason.
- Let function and module names carry structure; plain code with blank lines separates sections.
- Write docstrings on public APIs whose consumer-facing behavior is non-obvious, describing behavior and constraints; the type signature already documents parameter types.

### 4. Current comments only

- Delete commented-out code and dead code; version control keeps the history.
- Keep authorship and change history in commits, so comments describe the code as it is now.
- Use hyphens, colons, commas, or parentheses for asides in comments, not em-dashes (U+2014).

---

## Comment decision matrix

When writing or reviewing comments, classify each comment into one of four actions:

| Category                 | Indicator                                                                           | Action                                                        |
| :----------------------- | :---------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| **Redundant / Obvious**  | Restates function name, variable assignment, loop, or type                          | **Delete**                                                    |
| **Bloated Essay**        | Multi-sentence explanation of straightforward logic                                 | **Trim to 1 concise line** focusing on the "why"              |
| **Cryptic Code Smell**   | Long comment explaining convoluted logic                                            | **Refactor code** (rename, extract helper) and remove comment |
| **Legitimate Invariant** | Explains upstream bugs, mathematical formulas, hardware quirks, or security reasons | **Keep** as a brief, single-line note                         |

---

## Audit workflow

When the user requests a comment audit or cleanup:

1. Find the target.
   - If a file or path is specified, scan that scope.
   - If no target is given, inspect the current working changes (`git diff` or modified files).

2. Audit and classify.
   - Scan every comment in the target scope.
   - Categorize each comment using the comment decision matrix (Delete / Trim / Refactor / Keep).

3. Present the plan and wait for approval.
   - Show a concise table of findings: file path and line numbers, current comment text, proposed action, and rationale.
   - Ask the user to approve, adjust, or drop items. Apply edits directly without this pause only when the user has already asked you to apply the changes in that scope.

4. Apply the approved edits.
   - Change only the approved comments and any approved refactors, leaving unrelated formatting and logic untouched.
   - Run the project's build and tests; done when they pass and every approved row in the table is applied.
