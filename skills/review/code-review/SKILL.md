---
name: code-review
description: Review the changes since a fixed point (a commit, branch, tag, or merge-base) along two axes — Standards, meaning does the code follow this repo's documented conventions plus a baseline of common code smells, and Spec, meaning does the code do what the originating issue, ticket, or PRD asked for. Runs both reviews as parallel sub-agents and reports them side by side without merging or reranking them. Use when the user wants to review a branch, a pull request, work-in-progress changes, or asks to review the diff since some point.
metadata:
  source: mattpocock/skills
  license: MIT
---

<!-- cspell:ignore mattpocock Fowler PRD rerank reranking -->

# Code Review

Two-axis review of the diff between `HEAD` and a fixed point the user supplies:

- **Standards** — does the code conform to this repo's documented conventions, and to a baseline of common code smells?
- **Spec** — does the code faithfully implement the originating issue, ticket, or PRD?

Both axes run as **parallel sub-agents** so they don't pollute each other's context; this skill then aggregates their findings.

This review leans on your project's own conventions — where issues and tickets live (the issue tracker), and where specs and standards docs are kept. If none of that is obvious from the repo itself, ask the user before you start rather than guessing.

Run any repo command through the project's environment (in this repo, `devenv shell -- <cmd>`). The diff is read with `git`; fetch issues and pull requests with the `gh` CLI.

## Process

### 1. Pin the fixed point

Whatever the user said is the fixed point — a commit SHA, branch name, tag, `main`, `HEAD~5`, and so on. If they didn't specify one, ask for it.

Capture the diff command once: `git diff <fixed-point>...HEAD` (three-dot, so the comparison is against the merge-base). Also note the list of commits via `git log <fixed-point>..HEAD --oneline`.

Before going further, confirm the fixed point resolves (`git rev-parse <fixed-point>`) and the diff is non-empty. A bad ref or an empty diff should fail here — not later, inside two parallel sub-agents.

### 2. Identify the spec source

Look for the originating spec, in this order:

1. Issue or ticket references in the commit messages — conventional-commit footers such as `Closes #45`, or a `#123` in the body. Fetch the referenced issue with `gh issue view <number>`, or the pull-request description with `gh pr view`.
2. A path the user passed as an argument.
3. A PRD or spec file under `docs/`, `specs/`, `plans/`, or a scratch directory, matching the branch name or feature. In this repo, specs and tickets are typically produced by `/to-spec` and `/to-tickets`, so that is the artefact the Spec axis checks against.
4. If nothing is found, ask the user where the spec is. If they say there isn't one, the **Spec** sub-agent skips and reports "no spec available".

### 3. Identify the standards sources

Anything in the repo that documents how code should be written — a `CONTRIBUTING.md`, an `AGENTS.md` or `CLAUDE.md`, a coding-standards doc, or the conventions the repo's tooling already encodes.

On top of whatever the repo documents, the Standards axis always carries a **smell baseline** — a fixed set of common code smells that applies even when a repo documents nothing. It lives in [code-smells.md](resources/code-smells.md). Two rules bind it:

- **The repo overrides.** A documented repo convention always wins; where it endorses something the baseline would flag, suppress the smell.
- **Always a judgement call.** Each smell is a labelled heuristic ("possible Feature Envy"), never a hard violation — and, like any standard here, skip anything the repo's tooling already enforces.

### 4. Spawn both sub-agents in parallel

Send a single message with two sub-agent calls — one Standards, one Spec — so they run concurrently. A general-purpose sub-agent suits both.

**Standards sub-agent prompt** — include:

- The full diff command and commit list.
- The list of standards-source files you found in step 3, **plus the full contents of [code-smells.md](resources/code-smells.md) pasted in** — the sub-agent has no other access to that baseline.
- The brief: "Report — per file or hunk where relevant — (a) every place the diff violates a documented standard: cite the standard (file plus the rule); and (b) any baseline smell you spot: name it and quote the hunk. Distinguish hard violations from judgement calls — documented-standard breaches can be hard, but baseline smells are always judgement calls, and a documented repo convention overrides the baseline. Skip anything tooling enforces. Under 400 words."

**Spec sub-agent prompt** — include:

- The diff command and commit list.
- The path or fetched contents of the spec.
- The brief: "Report: (a) requirements the spec asked for that are missing or partial; (b) behaviour in the diff that wasn't asked for (scope creep); (c) requirements that look implemented but where the implementation looks wrong. Quote the spec line for each finding. Under 400 words."

If the spec is missing, skip the Spec sub-agent and note this in the final report.

### 5. Aggregate

Present the two reports under `## Standards` and `## Spec` headings, verbatim or lightly cleaned. Do **not** merge or rerank findings — the two axes are deliberately separate (see [Why two axes](#why-two-axes)).

End with a one-line summary: total findings per axis, and the worst issue _within each axis_ (if any). Don't pick a single winner across axes — that is exactly the reranking the separation exists to prevent.

## Why two axes

A change can pass one axis and fail the other:

- Code that follows every standard but implements the wrong thing → **Standards pass, Spec fail.**
- Code that does exactly what the issue asked but breaks the project's conventions → **Spec pass, Standards fail.**

Reporting them separately stops one axis from masking the other.

## Related skills

`code-review` stands on its own, but it sits next to narrower passes worth reaching for:

- `/pr-build-context` — gather high-signal context on a pull request before the review.
- `/pr-build-context`, `/pr-create-test-plan`, `/pr-edge-cases` — build reviewer context on a branch, draft a manual test plan, or probe edge cases and failure modes.
- `/diagnosing-bugs` — when the Spec axis surfaces a real defect, hand it off to run the defect down.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).
