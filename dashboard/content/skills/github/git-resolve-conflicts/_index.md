+++
title = "git-resolve-conflicts"
description = "Resolve existing merge, rebase, or cherry-pick conflicts using file context and both sides' intent. Use when conflicts block authorized work; stage confident fixes and escalate ambiguous behavior."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "github"
mermaid = false
+++


# Resolve Git Conflicts

Resolve the conflicts left by an in-progress merge, rebase, or cherry-pick using a structured, context-aware approach. Resolve what the evidence supports confidently, state the reasoning for non-trivial resolutions, and escalate when the correct behavior is ambiguous.

## 1. Verify state

Start from an in-progress operation with unresolved conflicts. Run `git status` to confirm conflicts exist, then identify the operation and the incoming commit:

```bash
for ref in MERGE_HEAD REBASE_HEAD CHERRY_PICK_HEAD; do
  git rev-parse -q --verify "$ref" >/dev/null && echo "$ref"
done
```

Set `OTHER` to the ref that exists. The sides read differently by operation:

- **Merge** (`MERGE_HEAD`): "ours" (`HEAD`) is the current branch; "theirs" (`MERGE_HEAD`) is the branch being merged in.
- **Rebase** (`REBASE_HEAD`): the sides are swapped. "Ours" (`HEAD`) is the upstream plus the commits already replayed; "theirs" (`REBASE_HEAD`) is the branch's own commit being replayed.
- **Cherry-pick** (`CHERRY_PICK_HEAD`): "ours" (`HEAD`) is the current branch; "theirs" is the commit being picked.

Get the big picture before touching individual conflicts: the change being applied, and what `HEAD` changed since the two sides diverged, excluding lock files:

```bash
git show --stat "$OTHER"
git diff "$OTHER"...HEAD -- ":(exclude)*.lock" ":(exclude)package-lock.json" ":(exclude)pnpm-lock.yaml"
```

## 2. Map all conflicts

Inventory every conflict before resolving any of them.

```bash
git diff --name-only --diff-filter=U
```

```bash
rg "<<<<<<< " --line-number
```

- Create a task list with one entry per conflict chunk, grouped by file
- Note patterns across the conflict set:
  - Same subsystem? Paired changes? Generated files?
  - How many conflicts total? Are they concentrated or spread across the codebase?

## 3. Build context per conflict

For **each** conflict, before classifying or resolving:

### 3a. Read the full file

Not just the markers. Understand the function/block's role in the file.

### 3b. Trace commit history on both sides

```bash
git log --oneline HEAD.."$OTHER" -- <file>
git log --oneline "$OTHER"..HEAD -- <file>
```

Read commit messages and diffs to understand **intent** on each side.

### 3c. Examine surrounding code

Read 20-40 lines around the conflict. Identify invariants:

- Ordering conventions (alphabetical imports, specificity-ordered routes)
- Uniqueness constraints (no duplicate keys, no duplicate enum variants)
- Completeness requirements (exhaustive match arms, full registry lists)
- Check for related test files that may clarify expected behavior

### 3d. Check cascading implications

If the conflict is in a signature, type, constant, or export:

```bash
rg -w "<symbol_name>"
```

Find all usages to identify downstream impact.

### 3e. Assess business logic impact

Answer three questions for each conflict:

1. **Scope**: Mechanical (formatting/imports/whitespace) or runtime behavior change?
2. **Risk**: If resolved wrong, what breaks? (nothing / tests / production / data integrity)
3. **Novelty**: Both sides added new behavior? Or one cleanly supersedes the other?

## 4. Classify each conflict

| Tier                                   | When                                                                                                                                                    | Action                                                                                                                                     |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| **Tier 1 - Auto-resolve**              | Non-overlapping additions, formatting-only, one side is strict superset, lock/generated files                                                           | Resolve immediately. No developer input needed.                                                                                            |
| **Tier 2 - Resolve + state rationale** | Intent is inferable from context, combining both is clearly right but requires care, test file conflicts                                                | Resolve, then present rationale (see format below). Don't block on confirmation.                                                           |
| **Tier 3 - Escalate before resolving** | Can't determine correct behavior from context, critical path code, silent behavior discard, architectural divergence, cascading multi-file implications | **Stop.** Show conflict, explain both sides' intent, state the ambiguity, offer 2-3 options with trade-offs. Wait for developer direction. |

**Tier 2 rationale format:**

> Resolved `file:line`. [How]. Rationale: [why]. Flag if wrong.

Tier 1 stays decisive, Tier 3 stays consultative when it matters, and Tier 2 covers the middle ground: resolve, but make the reasoning visible.

## 5. Resolve by tier

### Tier 1: Auto-resolve

- Edit the file to the desired final state
- Remove all conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`)
- Verify the result is syntactically clean

### Tier 2: Resolve with type-specific guidance

Apply the right merge strategy based on the construct type:

**Lists/registries** (imports, exports, routes, enum variants):

- Union both sides, deduplicate
- Maintain the file's existing ordering convention (alphabetical, grouped, etc.)

**Function bodies** (both added branches/conditions):

- Include all additions
- Respect ordering by specificity (more specific before more general)

**Config/struct** (both added keys):

- Merge all keys
- If the same key has different values, escalate to Tier 3

**Parallel new code** (both added new functions/classes):

- Include both
- Order consistently with the file's existing conventions

**After combining**: Re-read the result as a human would. Check for:

- Duplicated side effects
- Broken invariants (ordering, uniqueness, completeness)
- Mismatched types or signatures

### Tier 3: Escalate

- Show the user the conflicting chunks with surrounding context
- Explain what each side intended (based on commit history from step 3b)
- State the specific ambiguity ("Both sides modify the retry logic but with different strategies")
- Offer 2-3 resolution options with trade-offs
- **Wait for the user's direction before editing**

After receiving direction:

- Restate the plan in one sentence before editing
- Re-evaluate remaining Tier 2 conflicts if new context was revealed

### New code escalation

When neither side's code is complete and the right answer is a **third implementation** (not just combining both):

- Flag it explicitly with a 3-5 bullet plan describing the proposed implementation
- Wait for approval before writing

## 6. Verify and stage

After all conflicts are resolved:

```bash
rg "<<<<<<< "
```

Confirm zero remaining conflict markers.

- Run lint/type-check if fast (skip slow integration tests)
- Stage resolved files individually: `git add <specific_files>` -- **not** `git add .`
- Leave the commit (and `git rebase --continue` or `git cherry-pick --continue`) to the user

## 7. Reflect and hand off

Provide a summary table with a **Status** emoji column so risky items are impossible to overlook:

| Status | File | Line(s) | Tier  | Resolution        |
| ------ | ---- | ------- | ----- | ----------------- |
| …      | …    | …       | 1/2/3 | Brief description |

**Status emoji meanings** (use exactly these):

| Emoji | Meaning                       | When to use                                                                          |
| ----- | ----------------------------- | ------------------------------------------------------------------------------------ |
| `✅`  | Safe / auto-resolved          | Tier 1 resolutions, trivial merges, no risk                                          |
| `🟢`  | Resolved with high confidence | Tier 2 where intent was clear from context                                           |
| `🟡`  | Resolved but needs review     | Tier 2 with lower confidence, subtle behavior changes, or dropped code               |
| `🔴`  | Escalated / blocked           | Tier 3, waiting for the user's direction                                             |
| `⚠️`  | Cascading risk                | Auto-merged files or downstream code that may be affected but wasn't in conflict set |

**Rules:**

- Every row MUST have a status emoji -- no blank status cells
- Any resolution that **drops code** (even dead code) must be `🟡` or higher
- Any Tier 2 resolution with confidence below ~80% must be `🟡`
- Tier 3 is always `🔴`

### Flags section

After the table, list flags grouped by severity. Each flag MUST start with its emoji:

- `🔴` Tier 3 escalations (blocking -- need direction before proceeding)
- `🟡` Tier 2 lower-confidence resolutions (non-blocking but review recommended)
- `⚠️` Cascading implications found during step 3d
- `⚠️` Architectural divergence detected between the two sides
- `⚠️` Files that weren't in the conflict set but may be affected by the operation

Closing question: **Are there areas of the codebase this operation could affect that aren't in the conflict markers?**

