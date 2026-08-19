---
name: tars-backlog-audit
description: Audit the codebase for bugs, features, security issues, or technical debt, and generate structured issue files in `.tars/issues/todo/`. Reach for this when requested to perform a codebase audit, search for bugs and tasks, or populate the backlog.
disable-model-invocation: true
---

# Backlog Audit

Audit the codebase to identify bugs, features, security issues, or technical debt, and generate structured backlog tickets.

This skill operates in a Hub-and-Spoke topology using sub-agents to analyze codebase modules in parallel, and then synthesizes the results.

## Targets and Paths

- All issue tickets are stored locally relative to the project root in `.tars/issues/todo/`
- Standard ticket folders also include `.tars/issues/done/`, `.tars/issues/failed/`, and `.tars/issues/wont-do/` (retired or superseded tickets)
- Ticket files are saved only to disk. Since `.tars/` must be gitignored, do **NOT** stage, commit, or force-add ticket files to git.

## Topic Branch Workflow (Hub Only)

All backlog operations must run from a topic branch, never the default branch. Audit subagents are read-only and take no branch of their own - they read the topic branch's working tree as the Hub has it checked out. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](../tars-backlog-prepare/SKILL.md) for the full policy and commands.

## Audit Workflow

### 1. Codebase Segmentation

Analyze the project's directory structure (by listing directories or inspecting the workspace files) and dynamically partition the codebase into up to 5 logical modules (e.g., frontend, backend handlers, shared configs, hooks, CI/CD pipelines).

### 2. Spawn Spokes

Spawn read-only research subagents (up to a maximum of 5 in parallel) to audit each identified module. They read the **parent working tree directly** - no clone, no worktree, no workspace setup or teardown.

Audit agents analyse and report back by message; they never commit. An isolated workspace would buy nothing and cost real setup on every batch, while giving each agent its own view of a repository that nobody is modifying. Reading the parent in place also means gitignored files such as `.tars/` are simply present, with no transfer step.

The safety this gives up is enforced by checking rather than by construction - see the cleanliness assertion in step 3.

Equip each subagent with:

- **Role**: `Audit-<MODULE_NAME>` (substitute a short, alphanumeric descriptor of the audited module)
- **Prompt**:

  ````text
  You are auditing the following codebase module: <MODULE_PATH_OR_GLOB>

  Deeply analyze it for:
  1. Bugs & Issues: Edge cases, unhandled errors, TOCTOU/race conditions, and security vulnerabilities.
  2. Maintainability, Naming & Style: Code smells, duplicate logic, or formatting/naming style violations *only if they violate repository formatting/naming standards* (e.g. linter rules, hook configs, or the repository's own documented contributor/agent guidelines, whatever file it keeps them in). Avoid logging subjective preferences that are not backed by repository standard configurations.
  3. Automated Testing: Test coverage gaps, missing integration suites, and fragile mock patterns.
  4. Features & Enhancements: Programmatic tools or utility endpoints that would improve user/agent experience.

  STRICT READ-ONLY CONSTRAINT: You are reading the user's live working tree, which is shared with other agents. You must NEVER modify, create, or delete any file, and never run a command that writes to the repository - in particular never run a formatter, a hook runner, or any test that generates artefacts. Never check out, commit to, or merge any branch. Read and report only.

  Verification & Output Format:
  - If the module is clean, stable, and conforms fully to the above criteria, reply with exactly:
    NO_ISSUES_FOUND
  - Otherwise, for each finding, you must verify the existence of the issue in the codebase and output it using this Markdown template:
    ### Finding: <Short Title>
    - **File**: <FilePath> (relative to project root)
    - **Symbol**: <REQUIRED - the enclosing function, method, constant, type, or exported name,
      e.g. verifyBashPermission or ALLOWED_KEYS_BY_KIND. If the finding is genuinely file-level
      (a missing config key, an absent test file), say so explicitly instead of inventing one.>
    - **Snippet**:
      ```<language>
      <exact code snippet from the file - copy it verbatim, do not paraphrase>
      ```
    - **Lines**: <OPTIONAL hint, e.g. 42-55. PERISHABLE - any landed change reorders the file and
      invalidates it. Never make it the only way to locate the finding.>
    - **Description**: <Detailed explanation of the issue, what standard/spec it violates, and how to fix it. Refer to the symbol by name in this prose, not to a line number.>

  The symbol and the snippet are the durable anchors: they survive the file moving. The line
  range does not, so it is a convenience only.
  ````

### 3. Synthesis & Ticket Generation (Hub Only)

Once the subagents report back, collect all findings:

1. **Verify Findings (CRITICAL ANTI-HALLUCINATION CHECK)**:
   - For each reported finding, the Hub must verify the existence of the file, then locate the reported **symbol** inside it by searching for the name - not by jumping to the reported line range.
   - Confirm that the actual code in the parent repository matches the provided snippet and contains the reported issue.
   - A `**Lines**` hint that has drifted is **not** grounds to discard the finding. Re-locate by symbol and snippet; if both hold, the finding stands and the stale hint is simply dropped. Only a finding whose symbol and snippet cannot be found is fabricated.
   - Discard any findings that fail this check, contain fabricated files or symbols, or represent subjective styling preferences not violating repository standard files.
2. **Deduplicate**: Combine overlapping findings.
3. **Filter**: Check against existing tickets in `.tars/issues/todo/`, `.tars/issues/done/`, `.tars/issues/failed/`, and `.tars/issues/wont-do/` to avoid duplicates. A finding already parked in `wont-do/` was retired deliberately - do not re-raise it without saying why the decision changed.
4. **Determine ID (CRITICAL FOR COLLISION PREVENTION)**:
   - Scan every ticket folder: `.tars/issues/todo/`, `.tars/issues/done/`, `.tars/issues/failed/`, and `.tars/issues/wont-do/` (if any folders do not exist, treat them as empty). A retired ticket still owns its ID; reusing it makes two different tickets answer to the same dependency edge.
   - Find all files in these directories that match the 3-digit pattern `XXX.md` (where `XXX` is a number like `001`, `042`, etc.).
   - Extract the numeric ID from each file name (e.g., `042.md` corresponds to `42`).
   - Find the absolute maximum ID used across every ticket folder.
   - The ID for the first new issue must be `max_id + 1` (e.g., if the highest is `042.md`, the next must be `043.md`).
   - **CRITICAL**: Never assume the backlog starts at `001` or overwrite existing issue files. Only start at `001` if every ticket folder is completely empty or does not exist.
   - Allocate subsequent new tickets sequentially (e.g., `043.md`, `044.md`, `045.md`).
5. **Generate Tickets**: For each verified finding, write a new ticket file to `.tars/issues/todo/` following the guidelines and structure defined in the [tars-backlog-create-issue](../tars-backlog-create-issue/SKILL.md) skill.
   - **Filename**: `XXX.md` (3-digit ID, padded with leading zeros, e.g., `043.md`)
   - **Cite the symbol, not the line.** Carry the finding's `**Symbol**` and `**Snippet**` into the ticket's `## Description`. Drop the `**Lines**` hint rather than writing it into ticket prose - by the time the ticket is implemented, other tickets will have landed and moved the file. Line-grain coordinates belong only in `owns:`, as `path#Symbol`.
   - **Never write a batch number into ticket prose**, and set `batch: null` at creation - batches are allocated at dispatch time, not at audit time.
   - **Frontmatter shapes are load-bearing**: `dependencies:` must be an inline array on one line, `files:` a two-space-indented block list. See **Frontmatter field rules** in [tars-backlog-create-issue](../tars-backlog-create-issue/SKILL.md); getting these wrong is not a formatting nit, it silently drops the data.
   - The batching rules that consume `files:`, `owns:`, and `dependencies:` are stated canonically in [tars-backlog-implement](../../engineering/tars-backlog-implement/SKILL.md). Do not restate them in a ticket.
6. **No Findings Output**: If no findings were reported or all were discarded during verification, output a clear status message to the user: "Audit complete. The codebase is clean, stable, and conforms to all standards. No new issues were logged."
7. **CRITICAL CLEANLINESS ASSERTION**: Audit agents read the parent working tree in place, so the Hub must confirm they left it untouched - run this after every batch, whether the agents succeeded, failed, or timed out:

   ```bash
   git status --porcelain
   ```

   If anything changed, an agent violated its read-only constraint. Restore the tree and warn the user:

   ```bash
   git reset --hard && git clean -fd
   ```

   This is safe here because spoke workspaces live outside the repository tree entirely (see `TARS_SPOKE_ROOT` in [tars-backlog-prepare](../tars-backlog-prepare/SKILL.md)), so no in-flight work can be caught by it. There are no worktrees or branches to tear down.
