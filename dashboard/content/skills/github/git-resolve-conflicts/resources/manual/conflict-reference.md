+++
title = "conflict-reference"
[extra]
skill = false
category = "github"
mermaid = false
skill_name = "git-resolve-conflicts"
+++

# Conflict Resolution Reference

Reference material for `git-resolve-conflicts`: Tier 2 merge strategies by construct type and the final handoff report template.

## Tier 2 construct-type merge strategies

Apply the merge strategy that matches the conflicting construct:

| Construct type                            | Strategy                                                                                                  |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| Lists/registries (imports, routes, enums) | Union both sides, deduplicate, and preserve the file's ordering convention (alphabetical, grouped, etc.). |
| Function bodies (both added branches)     | Include all additions and preserve ordering by specificity (more specific before general).                |
| Config/struct (both added keys)           | Merge all keys; if the same key has different values on each side, escalate to Tier 3.                    |
| Parallel new code (functions, classes)    | Include both and order consistently with the file's existing conventions.                                 |

After combining, re-read the merged block as a human would and check for:

- Duplicated side effects
- Broken invariants (ordering, uniqueness, completeness)
- Mismatched types or signatures

## Handoff report template

Provide a summary table with a Status emoji column so risky items are impossible to overlook:

| Status | File | Line(s) | Tier  | Resolution        |
| ------ | ---- | ------- | ----- | ----------------- |
| ...    | ...  | ...     | 1/2/3 | Brief description |

### Status emoji legend

Use these exact status emojis:

| Emoji | Meaning                       | When to use                                                                          |
| ----- | ----------------------------- | ------------------------------------------------------------------------------------ |
| `✅`  | Safe / auto-resolved          | Tier 1 resolutions, trivial merges, no risk                                          |
| `🟢`  | Resolved with high confidence | Tier 2 where intent was clear from context                                           |
| `🟡`  | Resolved but needs review     | Tier 2 with lower confidence, subtle behavior changes, or dropped code               |
| `🔴`  | Escalated / blocked           | Tier 3, waiting for the user's direction                                             |
| `⚠️`  | Cascading risk                | Auto-merged files or downstream code that may be affected but wasn't in conflict set |

### Table rules

- Every row MUST have a status emoji (no blank status cells).
- Any resolution that drops code (even dead code) must be `🟡` or higher.
- Any Tier 2 resolution with confidence below ~80% must be `🟡`.
- Tier 3 is always `🔴`.

### Flags section

After the table, list flags grouped by severity. Each flag MUST start with its emoji:

- `🔴` Tier 3 escalations (blocking - need direction before proceeding)
- `🟡` Tier 2 lower-confidence resolutions (non-blocking, review recommended)
- `⚠️` Cascading implications found during symbol usage checks
- `⚠️` Architectural divergence detected between the two sides
- `⚠️` Files outside the conflict set that may be affected by the operation

