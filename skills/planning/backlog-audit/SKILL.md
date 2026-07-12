---
name: backlog-audit
description: Audit the codebase for bugs, features, security issues, or technical debt, and generate structured issue files in `.tars/issues/todo/`. Reach for this when requested to perform a codebase audit, search for bugs and tasks, or populate the backlog.
---

# Backlog Audit

Audit the codebase to identify bugs, features, security issues, or technical debt, and generate structured backlog tickets.

This skill operates in a Hub-and-Spoke topology using sub-agents to analyze codebase modules in parallel, and then synthesizes the results.

## Targets and Paths

- All issue tickets are stored locally relative to the project root in `.tars/issues/todo/`
- Standard ticket folders also include `.tars/issues/done/` and `.tars/issues/failed/`
- Ticket files are saved only to disk. Since `.tars/` must be gitignored, do **NOT** stage or commit ticket files to git.

## Audit Workflow

### 1. Codebase Segmentation

Analyze the project's directory structure (using tools like `list_dir` or looking at workspace files) and dynamically partition the codebase into up to 5 logical modules (e.g., frontend, backend handlers, shared configs, hooks, CI/CD pipelines).

### 2. Spawn Spokes

Spawn `research` subagents (up to a maximum of 5 in parallel) to audit each identified module.

Equip each subagent with:

- **TypeName**: `research`
- **Workspace**: `branch`
- **Prompt**:

  ```text
  You are auditing the following codebase module: <MODULE_PATH_OR_GLOB>

  Deeply analyze it for:
  1. Bugs & Issues: Edge cases, unhandled errors, TOCTOU/race conditions, and security vulnerabilities.
  2. Maintainability & DRY: Duplicate logic, hardcoded values, complex loops, and areas needing refactoring.
  3. Automated Testing: Test coverage gaps, missing integration suites, and fragile mock patterns.
  4. Features & Enhancements: Programmatic tools or utility endpoints that would improve user/agent experience.

  STRICT ISOLATION CONSTRAINT: You must NEVER check out the source/main branch, commit directly to the source/main branch, or attempt to merge branches. You must operate strictly within your local isolated workspace.

  Report your findings with concrete file paths, functions, and line ranges.
  ```

### 3. Synthesis & Ticket Generation (Hub Only)

Once the subagents report back, collect all findings:

1. **Deduplicate**: Combine overlapping findings.
2. **Filter**: Check against existing tickets in `.tars/issues/todo/`, `.tars/issues/done/`, and `.tars/issues/failed/` to avoid duplicates.
3. **Determine ID**: Scan those three folders to find the highest 3-digit sequential ID (e.g. `001`, `002`), and allocate subsequent numbers (e.g., `003.md`, `004.md`).
4. **Generate Tickets**: For each verified finding, write a new ticket file to `.tars/issues/todo/` using the standard format:
   - **Filename**: `XXX.md` (3-digit ID)
   - **Format**: YAML frontmatter + standard headings (see template below). Make sure to list the files/components touched in the frontmatter `files` list to assist in dynamic batching.
5. **Cleanup**: As the Hub, clean up each subagent's worktree and branch immediately:
   - Run `git worktree remove --force <path>`
   - Run `git branch -D <branch-name>`

## Ticket Format Template

```markdown
---
id: XXX
name: "Short descriptive name"
description: "Detailed description of the issue"
component: "module/name"
estimation: "2h"
dependencies: []
status: todo
batch: null
files:
  - path/to/file1.ts
  - path/to/file2.ts
---

# XXX — Issue Name

## Description

Detailed description here.

## Tasks

- [ ] Task 1
- [ ] Task 2

## Acceptance Criteria

- [ ] Criterion 1
- [ ] Criterion 2

### Evidence

(Will be filled in by implementation Spoke)

## References

- Specific files/lines/functions
```
