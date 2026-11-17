+++
title = "pr-create-test-plan"
description = "Draft TEST_PLAN.md for a branch's real user flows and integration changes using the project's commands. Use when a reviewer needs manual verification steps and a quick smoke test before merging."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "review"
mermaid = false
+++


# PR Test Plan

Generate a manual test plan for the changes in the current branch. The plan covers what a developer or reviewer needs to **manually verify**: real user flows, integration behavior, and observable outcomes.

## Instructions

### Step 1: Detect base branch

Try these methods in order:

```bash
BASE_BRANCH=$(gh repo view --json defaultBranchRef -q '.defaultBranchRef.name' 2>/dev/null)
```

```bash
BASE_BRANCH=$(git remote show origin 2>/dev/null | grep "HEAD branch" | cut -d: -f2 | xargs)
```

If both fail, ask the user.

### Step 2: Gather change context

Run all of these and capture the results:

```bash
git diff $BASE_BRANCH...HEAD --name-only
git diff $BASE_BRANCH...HEAD --stat
git log $BASE_BRANCH..HEAD --oneline
```

### Step 3: Detect project tooling

Check what's available in the project so you can reference real commands (not generic guesses):

- **Task runner targets**: `make help`, or the targets in `Makefile`, `justfile`, or `Taskfile.yml`
- **Package manager**: Look for `pyproject.toml` (uv/pip), `package.json` (npm/pnpm), `Cargo.toml` (cargo), `go.mod` (go)
- **Test runners**: Look for `pytest.ini`, `pyproject.toml [tool.pytest]`, `jest.config.*`, `.mocharc.*`
- **Project docs**: Read `AGENTS.md`, `CONTRIBUTING.md`, or `README.md` for project-specific test/build instructions
- **CI config**: Check `.github/workflows/`, `Makefile`, or `Taskfile.yml` for existing test commands

**Prefer the project's task runner targets and documented commands over raw tool invocations.** If the project defines a `test-unit` target, use it instead of calling the test runner directly.

### Step 4: Categorize changes and confirm with user

Group changed files into categories. Common categories (adapt based on actual changes):

- **Feature code** -- new commands, API routes, services, UI components
- **Configuration / docs** -- config files, markdown, schemas, manifests
- **Tests** -- new or modified test files
- **Build / deploy** -- Makefiles, CI, Dockerfiles, scripts
- **Deletions** -- removed files or deprecated code

Present the detected categories to the user with a summary of what changed in each. Ask them to confirm or adjust before generating the full plan.

Example confirmation format:

```text
I found 3 change areas in this branch:

1. Export command -- new --format flag on the export command (2 source files)
2. Docs restructuring -- README rewrite, new reference docs, deleted shell scripts
3. Test fixes -- E2E test stability improvements (4 test files)

Should I generate the test plan for all 3, or would you like to adjust?
```

### Step 5: Generate the test plan

For each confirmed category, write one test section. Read [plan-format.md](@/skills/review/pr-create-test-plan/resources/manual/plan-format.md) first and apply all of it: severity markers, step formatting, what to include and what to leave to unit tests, the closing quick smoke test, and style.

### Step 6: Write output

1. **Write the plan** to `TEST_PLAN.md` in the repo root, or to a path the user names. It is a review artifact: leave it uncommitted and say so in the summary.
2. **Print a summary** to the terminal showing the section count and a one-liner per section.

Terminal summary format:

```text
Wrote TEST_PLAN.md (uncommitted) with 4 sections:

  1. Export Command -- 4 test steps (🔴×2, 🟢×1, 🔵×1)
  2. Docs Restructuring -- 3 test steps (🔴×1, 🟢×2)
  3. Automated Tests -- 2 test steps (🟢×2)
  4. Quick Smoke Test -- 3 commands
```

