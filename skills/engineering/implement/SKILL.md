---
name: implement
description: Implement a piece of work based on a spec or set of tickets.
disable-model-invocation: true
metadata:
  source: mattpocock/skills
  license: MIT
---

# Implement

Implement the work described in the spec or tickets, then verify and land it. This skill is a thin orchestrator - it chains the focused skills together rather than reinventing them.

## Flow

1. **Implement.** Work through the spec or tickets. Use `/tdd` wherever it fits, at pre-agreed seams: write the failing test, make it pass, refactor.
2. **Verify.** Run typechecking and single test files regularly as you go, then run the full test suite once at the end. Run these through your project's tooling (for example `devenv --no-tui shell -- <cmd>`). Don't move on while checks are red.
3. **Review.** Use `/code-review` to review the work before committing.
4. **Commit.** Commit to the current branch using a conventional commit message.

Keep each step honest: don't skip Verify or Review, and don't commit over failing checks.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).
