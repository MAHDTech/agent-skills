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


# Sculpt Code

Reshape code quality across eight dimensions. Scope to branch changes by default (`git diff <base-branch>...HEAD`), or accept explicit file/directory targets.

**Philosophy:** Write code for the next reader (human or agent). Minimize cognitive load. Prefer boring, obvious code over clever code. Every change must preserve business logic unless explicitly told otherwise.

## Instructions

1. **Determine scope** - ask if unclear:
   - Branch diff: `git diff <base-branch>...HEAD --name-only`
   - Staged changes: `git diff --cached --name-only`
   - Explicit files: user-provided list
2. **Read all changed files in full** before reviewing - understand existing patterns, not just the diff.
3. **Review each dimension** below. For each finding, cite `file_path:line_number`.
4. **Apply fixes directly** - this is a sculpting tool, not a report generator. Make the changes, show what you did.

## Dimensions

### 1. Dead Code & Surface Area

- Remove genuinely unused imports, functions, classes, and exception types
- **Verify before delete**: grep the entire codebase before removing anything
- Remove commented-out code; git history keeps it
- Remove premature abstractions: factories with 1 implementation, pass-through wrappers, single-use utility classes

### 2. Naming & Clarity

- Names must reveal intent - no `data`, `temp`, `result`, `x`, `val`, `info` without context
- Follow the codebase's existing conventions (don't mix `repo`/`repository`, `config`/`configuration`)
- Function names describe what they do, not how
- Prefer the language's idiomatic emptiness and truthiness checks (for example `if not items:` in Python) when they read as clearly as the explicit form
- Boolean variables/functions read as questions: `is_valid`, `has_permission`, `should_retry`

### 3. File & Function Size

- Functions over ~50 lines → candidate for extraction
- Files over ~400 lines → candidate for splitting
- Each function should have a single clear responsibility
- If you need a comment to separate "sections" within a function, those sections are probably separate functions
- Group related functions in the same file; don't scatter them

### 4. Nesting & Control Flow

- Max 3 levels of nesting - flatten with early returns and guard clauses
- Prefer early returns over deeply nested if/else chains
- Extract complex conditions into named booleans or predicate functions
- Replace nested loops with comprehensions or helper functions where it improves readability (not where it obscures it)

### 5. Idiomatic Patterns

- Use language-native constructs (e.g., list comprehensions in Python, `map`/`filter` in JS where idiomatic)
- Follow the repo's established patterns - don't introduce new paradigms for one function
- Prefer standard library over hand-rolled equivalents
- Match error handling style to the rest of the codebase

### 6. Reuse Opportunities

- Check if an existing helper already does what new code is doing - grep before writing
- Duplication thresholds: extract a magic value into a named constant at 2+ uses; extract repeated logic into a helper at 3+ occurrences (two copies of logic can stay inline)
- Flag cases where a library function was reimplemented

### 7. TODO Hygiene

Match the project's existing TODO convention. If it already uses a TODO-prefix taxonomy, follow that taxonomy; otherwise stick to a plain `TODO:` (plus the widely understood `FIXME:` / `HACK:`) rather than imposing a scheme the codebase hasn't adopted.

Each TODO must include:

- **What**: clear description
- **Why**: context on why it's deferred

Add missing TODOs for: known shortcuts, deferred work, temporary workarounds, and obvious improvement opportunities spotted during review. Remove stale or resolved TODOs.

### 8. Readability & Cognitive Load

- Comments explain **why**, not **what** (delete `# increment counter` above `counter += 1`)
- Trim noisy, bloated comments to one or two lines of rationale
- When a block of code needs a heading comment to explain what it does, extract it into a named function instead (see dimension 3)
- Strategic whitespace: blank lines between logical sections
- Preserve all `IMPORTANT`, `NOTE`, `CRITICAL`, `DEV_NOTE` markers - clean up the text, not the tag
- Keep links, issue references, and external references intact

## Output Format

For each file changed, show:

```text
### file_path

**Changes made:**
- [dimension] description of change (line X)
- [dimension] description of change (line Y)
```

End with a summary: files touched, lines removed, TODOs added/removed, helpers extracted.

## Leave unchanged

- Working abstractions (even if currently simple)
- Type annotations and signatures (they document contracts)
- Test code (unless explicitly asked)
- Forward-looking base classes if second implementation is likely soon
- Domain-specific patterns the team uses intentionally

## Larger refactors

For a bigger cleanup - removing accumulated engineering debt, untangling oversized modules, or collapsing duplicated logic across many files - work in small, reviewable passes rather than one sweeping diff:

1. **Map before editing.** Survey the messy area first: noisy modules, duplicated logic, dead code, public contracts, and the tests around them. Know what you're touching before you touch it.
2. **One theme per pass.** Pick a single cleanup theme at a time - delete dead code, simplify control flow, extract a helper, or modernise one stale pattern - not all at once.
3. **State the behaviour and the check.** Before each pass, name the current behaviour, the structural improvement, and the smallest check that proves behaviour stayed stable. Run that check after every pass.
4. **Keep migrations separate.** Framework migrations, dependency upgrades, and architecture moves are their own task, outside a refactor pass. Lean on a green test suite, or characterization tests that pin current behaviour when there isn't one, as the safety net.

