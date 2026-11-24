---
name: git-resolve-conflicts
description: Resolve existing merge, rebase, or cherry-pick conflicts using file context and both sides' intent. Use when conflicts block authorized work; stage confident fixes and escalate ambiguous behavior.
metadata:
  group: github
---

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

Get the big picture before touching individual conflicts, excluding lock files:

```bash
git show --stat "$OTHER"
git diff "$OTHER"...HEAD -- ":(exclude)*.lock" ":(exclude)package-lock.json" ":(exclude)pnpm-lock.yaml"
```

## 2. Map all conflicts

Inventory every unmerged path (including binary and delete/modify `DU`/`UD` conflicts) and every conflict marker location before resolving any of them:

```bash
git diff --name-only --diff-filter=U
git diff --check
```

- Create a task list grouped by file, noting total count and patterns (same subsystem, paired changes, generated or binary files).

## 3. Build context per conflict

For each conflict, before classifying or resolving:

1. **Read the full file** and 20-40 lines around the conflict to understand ordering conventions, uniqueness constraints, completeness requirements, and related tests.
2. **Trace commit history on both sides** to understand each side's intent:

   ```bash
   git log --oneline HEAD.."$OTHER" -- <file>
   git log --oneline "$OTHER"..HEAD -- <file>
   ```

3. **Check cascading implications** when the conflict touches a signature, type, constant, or export:

   ```bash
   git grep -w "<symbol_name>"
   ```

4. **Assess business logic impact**: scope (mechanical vs runtime behavior), risk (nothing / tests / production / data integrity), and novelty (both sides added behavior vs one cleanly supersedes the other).

## 4. Classify each conflict

| Tier                                   | When                                                                                                                                                    | Action                                                                                                                                     |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| **Tier 1 - Auto-resolve**              | Non-overlapping additions, formatting-only, one side is strict superset, lock/generated files                                                           | Resolve immediately. No developer input needed.                                                                                            |
| **Tier 2 - Resolve + state rationale** | Intent is inferable from context, combining both is clearly right but requires care, test file conflicts                                                | Resolve, then state rationale using the format below. Do not block on confirmation.                                                        |
| **Tier 3 - Escalate before resolving** | Can't determine correct behavior from context, critical path code, silent behavior discard, architectural divergence, cascading multi-file implications | **Stop.** Show conflict, explain both sides' intent, state the ambiguity, offer 2-3 options with trade-offs. Wait for developer direction. |

**Tier 2 rationale format:**

> Resolved `file:line`. [How]. Rationale: [why]. Flag if wrong.

## 5. Resolve by tier

- **Tier 1:** Edit the file to the clean final state, removing all conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`) and verifying valid syntax.
- **Tier 2:** Apply the construct-type merge strategies and post-merge checks in [conflict-reference.md](resources/manual/conflict-reference.md).
- **Tier 3:** Present the conflicting chunks with surrounding context, commit-history intent from step 3, the specific ambiguity, and 2-3 options with trade-offs, then wait for user direction. Once directed, restate the plan in one sentence before editing and re-evaluate remaining Tier 2 conflicts if new context emerged.
- **Third-implementation escalation:** When neither side's code is complete and a new implementation is required rather than combining both, present a 3-5 bullet plan and wait for approval before writing.

## 6. Verify and stage

Confirm zero unmerged paths and zero remaining conflict markers:

```bash
git diff --name-only --diff-filter=U
git diff --check
```

- Run fast lint or type checks if available (skip slow integration suites).
- Stage resolved files individually: `git add <specific_files>` - **not** `git add .`.
- Leave the commit (and `git rebase --continue` or `git cherry-pick --continue`) to the user.

## 7. Report and hand off

Format the final summary table, status emoji legend, and severity-grouped flags section (including cascading risks and affected files outside the conflict set) using [conflict-reference.md](resources/manual/conflict-reference.md).

Done when `git diff --name-only --diff-filter=U` and `git diff --check` report zero unmerged paths or conflict markers for every resolved file, resolved files are staged individually, fast checks are reported, any Tier 3 escalations await user direction with concrete options, and the status-table report with cascading-risk flags is delivered.
