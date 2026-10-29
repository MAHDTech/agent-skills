---
name: gh-takeover
description: "Adopt and finish an inherited pull request, branch, or agent handoff while preserving contributor credit. Use only when the user explicitly asks to take over existing work."
disable-model-invocation: true
metadata:
  group: github
  source: t3dotgg/tstack
  license: MIT
---

# GitHub Takeover

**Invocation:** Start only when the user explicitly requests this workflow.

Take ownership of finishing an inherited pull request, branch, or agent handoff toward its original goal while preserving every human contributor's credit.

## 1. Audit inherited context against actual code

Inspect the full source context before changing code:

```bash
git status --short
git rev-parse HEAD
```

- Read the source PR, branch commits, or handoff thread, including inline review comments, bot findings, and the user's later corrections.
- Reconcile every handoff claim against the checked-out `HEAD`, uncommitted working tree changes, and current diff; treat handoff notes as context rather than proof.
- Judge the existing implementation on its merits instead of defending flawed structure or assumptions just because a prior agent or contributor wrote them.

## 2. Choose continuation shape and preserve contributor credit

Select the continuation strategy that matches the user's authorization:

- **Replacement PR (default for external or stalled branches):** Create a new branch starting from the original commits and add your fixes as separate commits on top so the inherited work and your changes remain distinct.
- **In-place or stacked continuation:** Push directly to the existing branch or stack onto it only when the user explicitly asked for that layout.
- **Source PR boundary:** Never overwrite unrelated local changes, and never edit, close, or merge the original PR without explicit authorization.

Preserve human contributor credit at every step:

- Keep original Git commit authorship and existing `Co-authored-by:` trailers intact when importing commits.
- Credit the original human contributors and link the source PR in the continuation PR description.
- When the repository or user squash-merges the PR, carry `Co-authored-by: Name <verified-email>` trailers into the squash commit using verified identities from Git commit history (`git log --format='%an <%ae>'`) or verified GitHub `noreply` addresses - never guessed email addresses. Model attribution never substitutes for human contributor credit.

## 3. Modernize and rework within scope

1. Run `/git-update-branch` to check whether base-branch changes since divergence solved, superseded, or shifted the design, and integrate the latest base branch cleanly.
2. Rework whatever mechanics are broken or overcomplicated to deliver the original goal, keeping unrelated features out of scope.
3. Simplify the implementation where possible without dropping intended behavior, edge-case contracts, or public surfaces.
4. Recheck the source PR before shipping in case new commits or review context landed while you worked.

## 4. Verify realistic workflows

- Reproduce the original bug or exercise realistic user-facing workflows end-to-end through the repository's documented environment wrapper (for example `devenv --no-tui shell -- <command>`).
- Treat green mocked unit tests as insufficient on their own to prove the feature works in practice.
- Record concrete command results, and state plainly anything you could not verify locally.

## 5. Ship, monitor, and report

- When finishing includes opening or updating a pull request, run `/gh-create-pr` to draft and publish it behind its approval gate.
- When asked to see the PR through CI and review rather than stopping at filing, continue with `/gh-babysit-pr`.
- Format any PR comments or review replies posted on the user's behalf with `/gh-pr-comments`.
- Close out with a concise report covering:
  - What you inherited (`HEAD` SHA, source PR or branch, and open issues found)
  - What you changed and why
  - How you verified the workflow (plus any unverified surface)
  - How contributor authorship and trailers were preserved
  - Any remaining decision for the user

Done when the original goal is verified against the current base branch, contributor credit is intact, and the requested branch or PR state is delivered.

> Adapted from [t3dotgg/tstack](https://github.com/t3dotgg/tstack) (MIT).
