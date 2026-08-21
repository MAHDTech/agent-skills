---
name: tars-backlog-create-issue
description: Use when creating a new backlog issue/ticket in the `.tars/issues/todo/` directory, defining its YAML frontmatter, headings, tasks, acceptance criteria, evidence collection, and triage review expectations.
disable-model-invocation: true
---

# Backlog Create Issue

This skill defines the canonical structure, format, and content expectations for creating issue tickets in the local backlog. Standardizing this structure ensures coding agents can parse, batch, implement, and review tickets consistently.

## Targets and Paths

- All backlog tickets are stored in the `.tars/issues/todo/` directory relative to the project root.
- File names must follow the format `XXX.md`, where `XXX` is a sequential 3-digit ID (e.g., `001.md`, `002.md`, etc.).
- When creating a new issue, check `.tars/issues/todo/`, `.tars/issues/done/`, `.tars/issues/failed/`, and `.tars/issues/wont-do/` to find the highest existing 3-digit ID. The new file must be named with the next sequential ID (`highest_id + 1`), padded to 3 digits. Do not assume `001` or overwrite existing issue files unless the directories are entirely empty or do not exist.
- `.tars/issues/wont-do/` holds retired and superseded tickets. It is not a failure state, but it **is** terminal: a ticket that depends on one parked there can never be scheduled. Scan it for ID allocation so a retired ID is never reused.
- Ticket files are created locally on disk only. Since `.tars/` must be gitignored, do **NOT** stage, commit, or force-add ticket files to git.

## Canonical Ticket Format

Every backlog ticket must follow this template. **It is meant to be copied verbatim and then edited**, so every value below is a real, valid value rather than a list of alternatives - the permitted alternatives are documented in **Frontmatter field rules** immediately after.

```markdown
---
id: 43
name: "Short descriptive name of the issue"
description: "Detailed description of the issue"
component: "module/name"
priority: "medium"
type: "bug"
estimation: "2h"
# optional; "high" enables the spoke checkpoint protocol
complexity: "normal"
# optional; "high" forces a full implementation review
risk: "normal"
dependencies: []
status: "todo"
attempts: 0
branch: null
batch: null
files:
  - path/to/file1.ts
# optional; path or path#symbol this ticket uniquely owns
owns: []
---

> [!IMPORTANT]
> **`files:` is mandatory and must not be empty.** It is an input to the File rule that decides which tickets may be implemented in parallel - see [tars-backlog-implement](../../engineering/tars-backlog-implement/SKILL.md). A ticket without it cannot be batched with anything and must run alone, so an omitted list silently serialises the backlog at best, and at worst gets treated as "no conflicts" and batched into a collision.
>
> List every file the work will touch, including ones changed only incidentally: test files, fixtures, barrel/index re-exports, lockfiles, and shared append-only files such as a spellcheck dictionary. Under-declaring is the failure that matters; over-declaring only costs a little parallelism.
>
> **`owns:` (optional)** lists modules or exports this ticket is the authority for, as `path` or `path#SymbolName` (e.g. `packages/shared/src/git.ts#CONVENTIONAL_COMMIT_TYPES`). Overlapping `owns:` between tickets serialises them like overlapping `files:`. Use it when two tickets might both add the same constant or edit the same shared implementation even if their `files:` lists look disjoint.
>
> **Soft dependencies:** if ticket B needs an export or behaviour ticket A will introduce, put A in B's `dependencies:` even when the files do not overlap. "Must already be merged" is not the same as "must not collide on disk." Write the edge as an inline array on one line (`dependencies: [12]`) - a multi-line list parses as empty and the edge is lost silently.
>
> **`risk: high`** (security, hooks, auth, shared core) and **`complexity: high`** (large multi-file work) steer review depth and spoke checkpointing during implement - set them honestly.

# XXX - Issue Name

## Description

A comprehensive explanation of the problem, the context in which it occurs, and why the change is necessary. State any known side effects or codebase coordinates.

## Tasks

Detailed list of concrete developer tasks to complete:

- [ ] Task 1 (e.g. Add validation logic)
- [ ] Task 2 (e.g. Write integration test)

## Acceptance Criteria

Explicit, checkable conditions that must be satisfied for the ticket to be considered complete:

- [ ] Criterion 1 (e.g. the project's full test gate passes without error)
- [ ] Criterion 2 (e.g. Invalid input is caught and returns exit code 1)

## Evidence

_(This section starts empty when the ticket is created. It is populated by the implementation agent before completion.)_

Must contain command logs, test runs, or code diffs demonstrating that all Acceptance Criteria are met.

## Review

_(This section is empty when the ticket is created. It is appended by the triage agent during tars-backlog-triage.)_

## Implementation Review

_(This section is empty when the ticket is created. It is appended by the tars-backlog-review agent upon completion of the implementation review.)_
```

## Frontmatter field rules

Ticket frontmatter is read by tooling that must tolerate malformed input in order to report it, so it recognises a **narrow set of shapes**. Anything outside them is not rejected loudly - it is silently misread. Follow these exactly.

| Field          | Required shape                            | What happens when it is wrong                                                     |
| -------------- | ----------------------------------------- | --------------------------------------------------------------------------------- |
| `id`           | bare integer, unquoted, unpadded (`43`)   | -                                                                                 |
| `status`       | quoted string                             | -                                                                                 |
| `batch`        | bare integer or `null` - **never quoted** | a quoted batch cannot be selected when a batch is dispatched by number            |
| `dependencies` | **inline array on one line**: `[1, 2]`    | a multi-line YAML list parses as **empty** - the dependency edges silently vanish |
| `files:`       | block list, items indented **two spaces** | not parsed; the ticket reads as having no files and gets batched into a collision |
| `owns:`        | same block list, or inline `[]`           | not parsed                                                                        |

Permitted values:

- `status`: `"todo"`, `"rework"`, `"done"`, `"failed"`, `"wont-do"`, `"in-review"`.
- `priority`: `"low"`, `"medium"`, `"high"`.
- `type`: `"bug"`, `"feature"`, `"refactor"`, `"security"`.
- `complexity`, `risk`: `"normal"`, `"high"`.

`"wont-do"` marks a retired or superseded ticket (filed under `.tars/issues/wont-do/`); `"in-review"` marks one whose implementation is awaiting review. Both are terminal for scheduling purposes: **a dependency on a ticket in `wont-do/` or `failed/` is an error, not a satisfied edge.**

> [!WARNING]
> **`dependencies:` must be the inline bracketed form.** Write `dependencies: [12, 19]` or `dependencies: []` on one line. This form:
>
> ```yaml
> dependencies:
>   - 12
>   - 19
> ```
>
> reads as **empty**. The ticket then looks unblocked, gets batched alongside the work it depends on, and fails in a way that looks like a code bug rather than a metadata bug.

## Citing code: symbols, not line numbers

Cite the **symbol** - the function, constant, type, or exported name - in ticket prose. A line number rots within hours: one landed ticket can shift a file by a hundred lines and invalidate the coordinates in every other ticket that pointed into it, including a directive written the same day.

- **Required**: the symbol name, e.g. "`verifyBashPermission` rejects an empty argv before checking the allowlist".
- **Optional**: a short verbatim snippet, which is the one anchor that survives file movement. Prefer it over a coordinate.
- **Avoid**: bare `path:NNN` line references in prose. Line-grain coordinates belong only in `owns:`, as `path#Symbol` - which names a symbol, not a line.

> [!CAUTION]
> **Never copy a 4-or-more-digit line number into a test file** - not in a test name, not in a comment, not in fixture data. Repositories commonly scan test sources for bare long digit runs (mock IDs, ticket numbers, fixture keys) and cannot tell a line reference from a real identifier. A line number pasted into a test comment collides with unrelated fixtures and reds the gate, costing a full verification cycle to diagnose.

## Optional binding sections

A ticket may carry a section headed `## Rework directives` (or similar wording marked **BINDING**). When present, **that section is what the implementing agent executes.** `## Review` is commentary that sits above it.

This matters when a ticket is corrected: a note added to `## Review` saying a value is stale does **not** change what gets implemented. The stale value inside the binding section must itself be edited. See the repair duty in [tars-backlog-triage](../tars-backlog-triage/SKILL.md).

## Batch and dependency rules

Do not restate the batching rules in a ticket. They are stated once, canonically, in the **Conflict-Free Batching** section of [tars-backlog-implement](../../engineering/tars-backlog-implement/SKILL.md); a ticket that duplicates them will disagree with them the first time they change.

Never write a batch number into ticket prose (rework directives, cross-references such as "#660 is batch 4"). `batch:` is reassigned every run and reset to `null` on rework, so a batch number in prose is stale almost immediately. Express the relationship as a `dependencies:` edge instead - it is machine-checkable and cannot drift. The single exception is the ``- `batch: N` - rationale`` bullet in `## Review`, which is a bare restatement of the frontmatter field and can therefore be corrected mechanically.

## Worked example

A complete ticket, valid as written:

```markdown
---
id: 43
name: "Reject empty argv in bash permission check"
description: "verifyBashPermission treats an empty command as allowed, bypassing the allowlist."
component: "policy/bash"
priority: "high"
type: "security"
estimation: "2h"
complexity: "normal"
risk: "high"
dependencies: [41]
status: "todo"
attempts: 0
branch: null
batch: null
files:
  - src/policy/bash-policy.ts
  - src/policy/bash-policy.test.ts
owns: []
---

# 043 - Reject empty argv in bash permission check

## Description

`verifyBashPermission` in `src/policy/bash-policy.ts` reads the first element of the parsed
argv and compares it against `ALLOWED_KEYS_BY_KIND`. When argv is empty the lookup yields
`undefined`, which the surrounding condition treats as a match, so an empty command string is
permitted rather than denied.

The fix belongs in `verifyBashPermission` itself rather than at its call sites: the constant
`ALLOWED_KEYS_BY_KIND` is consulted from three places and only this one guards emptiness.

Depends on #41, which introduces the `PolicyDenial` result type this fix returns.

## Tasks

- [ ] Make `verifyBashPermission` return a denial when the parsed argv is empty.
- [ ] Add a regression test covering empty and whitespace-only command strings.

## Acceptance Criteria

- [ ] An empty command string is denied, with a reason naming the empty-argv case.
- [ ] The project's full test gate passes without error.

## Evidence

## Review

## Implementation Review
```

## Review & Verification Guidelines

### During Backlog Triage (`tars-backlog-triage`)

The triage agent will review the ticket and append a `## Review` section containing:

- **Codebase Check**: Verification that all files/directories referenced in the ticket actually exist.
- **Hallucination Check**: Ensuring no deprecated APIs or incorrect function signatures are referenced.
- **Readiness Verdict**: A list of findings or a clear statement that the ticket is ready for implementation.
- **Footprint check**: `files:` complete vs body; `owns:` suggested when shared exports are involved; soft-deps called out.

### During Backlog Implementation (`tars-backlog-implement`)

The implementation agent must:

1. Complete all checkboxes in the `## Tasks` and `## Acceptance Criteria` sections.
2. Run verification using the repository commands frozen in `.tars/run.env` (spokes: targeted tests via `tars-spoke`; hub: full `tars-gate`).
3. Document terminal outputs or test run success under the `## Evidence` section.
4. Use the repository's commit message policy (conventional commits when the project requires them).

### During Backlog Review (`tars-backlog-review`)

The review agent will:

1. Inspect the code diff between the target branch and the implementation branch.
2. Verify that the implemented changes align with the ticket's `## Tasks` and `## Acceptance Criteria`.
3. Check the authenticity of the test logs and terminal output provided in `## Evidence`.
4. Ensure the changes adhere to codebase quality standards and conventions.
5. Append a `## Implementation Review` section containing the final verdict (`Approved` or `Request Rework`) and detailed feedback if rework is required.
