+++
title = "sculpt-code"
description = "Refactor a scoped diff or selected files for clearer naming, control flow, structure, and less duplication or dead code. Use for cleanup that preserves behavior."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Sculpt code

Reshape code quality across eight dimensions while preserving existing behavior unless explicitly told otherwise. Scope to branch changes by default (`git diff <base-branch>...HEAD`), or accept explicit file/directory targets.

## Instructions

1. **Determine scope** (ask if unclear):
   - Branch diff: `git diff <base-branch>...HEAD --name-only`
   - Staged changes: `git diff --cached --name-only`
   - Explicit files: user-provided list
2. **Verify a green baseline** before editing by running the repository's tests, typecheck, and linter for the scoped area (or note if no suite exists and pin critical behavior first).
3. **Read all scoped files in full** before refactoring so changes match existing module patterns rather than just the diff.
4. **Review each dimension** below, citing `file_path:line_number` for each change.
5. **Apply fixes directly** and report the changes made.
6. **Re-run tests, typecheck, and linter** after refactoring to confirm behavior is preserved and all checks pass.

## Dimensions

### 1. Dead code and surface area

- Remove genuinely unused imports, functions, classes, and exception types.
- **Verify before delete**: grep the entire codebase before removing any symbol.
- Remove commented-out code (Git history preserves it).
- Remove pass-through wrappers, single-use utility classes, and single-implementation factories that add indirection without hiding complexity or defining an active boundary; keep an abstraction when it enforces a real contract or test seam, hides non-trivial invariant logic, or has a documented second implementation in progress.

### 2. Naming and clarity

- Choose intent-revealing names; replace vague identifiers (`data`, `temp`, `result`, `x`, `val`, `info`) with domain terms.
- Keep terminology consistent across the codebase (use one term consistently, such as `repo` or `repository`, `config` or `configuration`).
- Name functions after their observable outcome or action.
- Prefer the language's idiomatic emptiness and truthiness checks (for example `if not items:` in Python) when they read as clearly as the explicit form.
- Name boolean variables and predicates as questions (`is_valid`, `has_permission`, `should_retry`).

### 3. File and function size

- Treat functions over ~50 lines as extraction candidates.
- Treat files over ~400 lines as splitting candidates.
- When a function relies on heading comments to separate internal "sections", extract those sections into named helper functions.
- Co-locate related helpers in the same module.

### 4. Nesting and control flow

- Cap nesting at 3 levels by flattening with guard clauses and early returns.
- Replace deeply nested `if`/`else` chains with early returns.
- Extract complex boolean conditions into named variables or predicate functions.
- Replace nested loops with comprehensions or helper functions when doing so improves clarity.

### 5. Idiomatic patterns

- Use language-native constructs (for example list comprehensions in Python, `map`/`filter` in JS where idiomatic) and standard-library functions over hand-rolled equivalents.
- Match the repository's established architecture, idioms, and error-handling conventions.

### 6. Reuse opportunities

- Grep the codebase before adding a helper to reuse existing utilities.
- Duplication thresholds: extract a magic value into a named constant at 2+ uses; extract repeated logic into a helper at 3+ occurrences (two copies can stay inline).
- Replace hand-rolled reimplementations of installed library functions with the library call.

### 7. TODO hygiene

Match the project's existing TODO convention (use its prefix taxonomy if adopted; otherwise stick to plain `TODO:`, `FIXME:`, or `HACK:`).

- Remove stale or resolved TODOs.
- Complete existing in-scope TODOs when trivial and safe.
- Keep remaining valid TODOs focused by ensuring each states **what** is deferred and **why**; avoid adding speculative TODOs for hypothetical future cleanups.

### 8. Readability and cognitive load

- Keep comments focused on **why** (invariants, workarounds, external constraints) and delete comments that restate the code.
- Trim verbose rationale comments to one or two direct lines.
- Use blank lines between logical blocks.
- Preserve `IMPORTANT`, `NOTE`, `CRITICAL`, and `DEV_NOTE` markers (tighten the wording while keeping the tag).
- Keep links, issue references, and external citations intact.

## Output format

For each file changed, show:

```text
### file_path

Changes made:
- [dimension] description of change (line X)
- [dimension] description of change (line Y)
```

End with a summary: files touched, lines removed, TODOs resolved/removed, helpers extracted, and verification commands run.

## Leave unchanged

- Abstractions that hide real complexity, define an active contract or test seam, or support a documented second implementation in progress.
- Public type annotations and signatures that define external contracts.
- Test code (unless explicitly requested).
- Intentional domain-specific patterns established by the team.

## Larger refactors

For a bigger cleanup (removing accumulated debt, untangling oversized modules, or collapsing duplicated logic across many files), work in small, reviewable passes rather than one sweeping diff:

1. **Map before editing.** Survey the target area first: noisy modules, duplicated logic, dead code, public contracts, and existing tests.
2. **One theme per pass.** Pick a single cleanup theme per pass (dead code, control flow, helper extraction, or one stale pattern).
3. **State the behavior and the check.** Before each pass, name the preserved behavior, the structural change, and the smallest check that verifies it; run that check after every pass.
4. **Keep migrations separate.** Treat framework migrations, dependency upgrades, and architecture moves as separate tasks outside a refactor pass, backed by tests or characterization tests.

## Completion criteria

- A green baseline check (tests, typecheck, and lint applicable to the repo) ran before editing, or the absence of automated checks was explicitly noted.
- Every refactor preserves existing behavior, respects the `## Leave unchanged` boundaries, and cites `file_path:line_number` in the summary.
- All applicable tests, typechecks, and linters pass after the final edit with zero new warnings or unrelated changes.

