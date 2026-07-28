+++
title = "tars-backlog-audit"
description = "Audit the codebase for bugs, features, security issues, or technical debt, and generate structured issue files in `.tars/issues/todo/`. Reach for this when requested to perform a codebase audit, search for bugs and tasks, or populate the backlog."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "planning"
mermaid = false
+++


# Backlog Audit

Audit the codebase to identify bugs, features, security issues, or technical debt, and generate structured backlog tickets.

This skill operates in a Hub-and-Spoke topology using sub-agents to analyze codebase modules in parallel, and then synthesizes the results.

## Targets and Paths

- All issue tickets are stored locally relative to the project root in `.tars/issues/todo/`
- Standard ticket folders also include `.tars/issues/done/` and `.tars/issues/failed/`
- Ticket files are saved only to disk. Since `.tars/` must be gitignored, do **NOT** stage, commit, or force-add ticket files to git.

## Topic Branch Workflow (Hub Only)

All backlog operations must run from a topic branch, never the default branch. Audit subagents are read-only and take no branch of their own — they read the topic branch's working tree as the Hub has it checked out. See the canonical **Topic Branch Verification** section in [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md) for the full policy and commands.

## Audit Workflow

### 1. Codebase Segmentation

Analyze the project's directory structure (by listing directories or inspecting the workspace files) and dynamically partition the codebase into up to 5 logical modules (e.g., frontend, backend handlers, shared configs, hooks, CI/CD pipelines).

### 2. Spawn Spokes

Spawn read-only research subagents (up to a maximum of 5 in parallel) to audit each identified module. They read the **parent working tree directly** — no clone, no worktree, no workspace setup or teardown.

Audit agents analyse and report back by message; they never commit. An isolated workspace would buy nothing and cost real setup on every batch, while giving each agent its own view of a repository that nobody is modifying. Reading the parent in place also means gitignored files such as `.tars/` are simply present, with no transfer step.

The safety this gives up is enforced by checking rather than by construction — see the cleanliness assertion in step 3.

Equip each subagent with:

- **Role**: `Audit-<MODULE_NAME>` (substitute a short, alphanumeric descriptor of the audited module)
- **Prompt**:

  ````text
  You are auditing the following codebase module: <MODULE_PATH_OR_GLOB>

  Deeply analyze it for:
  1. Bugs & Issues: Edge cases, unhandled errors, TOCTOU/race conditions, and security vulnerabilities.
  2. Maintainability, Naming & Style: Code smells, duplicate logic, or formatting/naming style violations *only if they violate repository formatting/naming standards* (e.g. eslint rules, prek check configs, or guidelines in AGENTS.md). Avoid logging subjective preferences that are not backed by repository standard configurations.
  3. Automated Testing: Test coverage gaps, missing integration suites, and fragile mock patterns.
  4. Features & Enhancements: Programmatic tools or utility endpoints that would improve user/agent experience.

  STRICT READ-ONLY CONSTRAINT: You are reading the user's live working tree, which is shared with other agents. You must NEVER modify, create, or delete any file, and never run a command that writes to the repository — in particular never run a formatter, a hook runner, or any test that generates artefacts. Never check out, commit to, or merge any branch. Read and report only.

  Verification & Output Format:
  - If the module is clean, stable, and conforms fully to the above criteria, reply with exactly:
    NO_ISSUES_FOUND
  - Otherwise, for each finding, you must verify the existence of the issue in the codebase and output it using this Markdown template:
    ### Finding: <Short Title>
    - **File**: <FilePath> (relative to project root)
    - **Lines**: <LineRange> (e.g. 42-55)
    - **Snippet**:
      ```<language>
      <exact code snippet from the file>
      ```
    - **Description**: <Detailed explanation of the issue, what standard/spec it violates, and how to fix it.>
  ````

### 3. Synthesis & Ticket Generation (Hub Only)

Once the subagents report back, collect all findings:

1. **Verify Findings (CRITICAL ANTI-HALLUCINATION CHECK)**:
   - For each reported finding, the Hub must verify the existence of the file and inspect the referenced line range in the codebase.
   - Confirm that the actual code in the parent repository matches the provided snippet and contains the reported issue.
   - Discard any findings that fail this check, contain fabricated files/lines, or represent subjective styling preferences not violating repository standard files.
2. **Deduplicate**: Combine overlapping findings.
3. **Filter**: Check against existing tickets in `.tars/issues/todo/`, `.tars/issues/done/`, and `.tars/issues/failed/` to avoid duplicates.
4. **Determine ID (CRITICAL FOR COLLISION PREVENTION)**:
   - Scan all three folders: `.tars/issues/todo/`, `.tars/issues/done/`, and `.tars/issues/failed/` (if any folders do not exist, treat them as empty).
   - Find all files in these directories that match the 3-digit pattern `XXX.md` (where `XXX` is a number like `001`, `042`, etc.).
   - Extract the numeric ID from each file name (e.g., `042.md` corresponds to `42`).
   - Find the absolute maximum ID used across all three folders.
   - The ID for the first new issue must be `max_id + 1` (e.g., if the highest is `042.md`, the next must be `043.md`).
   - **CRITICAL**: Never assume the backlog starts at `001` or overwrite existing issue files. Only start at `001` if all three folders are completely empty or do not exist.
   - Allocate subsequent new tickets sequentially (e.g., `043.md`, `044.md`, `045.md`).
5. **Generate Tickets**: For each verified finding, write a new ticket file to `.tars/issues/todo/` following the guidelines and structure defined in the [tars-backlog-create-issue](@/skills/planning/tars-backlog-create-issue/_index.md) skill.
   - **Filename**: `XXX.md` (3-digit ID, padded with leading zeros, e.g., `043.md`)
6. **No Findings Output**: If no findings were reported or all were discarded during verification, output a clear status message to the user: "Audit complete. The codebase is clean, stable, and conforms to all standards. No new issues were logged."
7. **CRITICAL CLEANLINESS ASSERTION**: Audit agents read the parent working tree in place, so the Hub must confirm they left it untouched — run this after every batch, whether the agents succeeded, failed, or timed out:

   ```bash
   git status --porcelain
   ```

   If anything changed, an agent violated its read-only constraint. Restore the tree and warn the user:

   ```bash
   git reset --hard && git clean -fd
   ```

   This is safe here because spoke workspaces live outside the repository tree entirely (see `TARS_SPOKE_ROOT` in [tars-backlog-prepare](@/skills/planning/tars-backlog-prepare/_index.md)), so no in-flight work can be caught by it. There are no worktrees or branches to tear down.

