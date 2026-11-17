+++
title = "self-review"
description = "Review your implementation diff for unfinished work, non-idiomatic code, and inconsistency with project conventions. Use after a task to apply focused fixes, with file-and-line evidence for each change."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "reflection"
mermaid = false
+++


# Self-Review

Post-implementation reflection pass. Run after completing a task to catch loose ends before calling it done.

## Instructions

1. **Determine scope** - unless the user specifies otherwise, review committed branch work plus every staged and unstaged change:
   - `git diff <base-branch>...HEAD --name-only` for commits on the branch your work targets
   - `git diff HEAD --name-only` for uncommitted work, staged and unstaged
   - Include new untracked files from `git status --short`
2. **Read all changed files in full** before reviewing.
3. **Answer each question below.** For every finding, cite `file_path:line_number` and fix it directly.
4. **If everything looks good**, say so briefly - don't invent busywork.

## Questions

### 1. Anything left undone?

- Are there TODOs, FIXMEs, or HACKs introduced in this diff that should be resolved now?
- Did you skip something the user asked for?
- Are there commented-out code blocks or placeholder values that shouldn't ship?
- Are there missing error cases, edge cases, or validations at system boundaries?
- Are there fallback/legacy code paths? If so, **ask the user explicitly** whether to keep, remove, or flag them - don't assume backward compatibility is wanted.

### 2. More idiomatic?

- Does the code follow the language's conventions and standard library patterns?
- Are there manual implementations of things the standard library or existing dependencies already provide?
- Does naming follow the project's existing conventions (check surrounding files)?
- Are there language-specific antipatterns? (e.g., Python: bare `except`, mutable default args; Go: exported names that shouldn't be; JS/TS: `any` types that should be narrowed)

### 3. Obvious clutter?

- Is there duplicated logic, a dead branch, or an abstraction with one caller introduced in this diff? Collapse it if the fix stays inside the diff; otherwise note it for the user.

## Output

For each question where you find something actionable:

- Show the finding with `file_path:line_number`
- Apply the fix directly
- One-line explanation of what changed and why

If nothing actionable is found, say: "Clean - nothing to follow up on."

