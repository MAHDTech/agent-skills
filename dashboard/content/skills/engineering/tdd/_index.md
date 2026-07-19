+++
title = "tdd"
description = "Test-driven development done red-green-refactor. Use when building a feature or fixing a bug test-first, writing integration tests, deciding what to test and where the test seams go, or avoiding brittle implementation-coupled tests. Covers what a good test is, the anti-patterns to avoid, and the rules of the red-green loop."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Test-Driven Development

TDD is the red to green loop. This skill is the reference that makes that loop produce tests worth keeping: what a good test is, where tests go, the anti-patterns, and the rules of the loop. Every section applies on every cycle — consult them before and during the loop, not after.

When exploring the codebase, read any project context docs (a `CONTEXT.md`, architecture notes, or ADRs in the area you are touching) if they exist, so test names and interface vocabulary match the project's domain language and conventions.

For the deep-module vocabulary that governs where seams go, run `/codebase-design`.

## What a good test is

Tests verify behaviour through public interfaces, not implementation details. Code can change entirely; tests should not. A good test reads like a specification — "user can checkout with valid cart" tells you exactly what capability exists — and survives refactors because it does not care about internal structure.

See [tests.md](@/skills/engineering/tdd/resources/manual/tests.md) for examples and [mocking.md](@/skills/engineering/tdd/resources/manual/mocking.md) for mocking guidelines.

## Seams — where tests go

A **seam** is the public boundary you test at: the interface where you observe behaviour without reaching inside. Tests live at seams, never against internals.

**Test only at pre-agreed seams.** Before writing any test, write down the seams under test and confirm them with the user. No test is written at an unconfirmed seam. You cannot test everything — agreeing the seams up front is how testing effort lands on the critical paths and complex logic instead of every edge case.

Ask: "What's the public interface, and which seams should we test?"

## Anti-patterns

- **Implementation-coupled** — mocks internal collaborators, tests private methods, or verifies through a side channel (querying the database instead of using the interface). The tell: the test breaks when you refactor but behaviour has not changed.
- **Tautological** — the assertion recomputes the expected value the way the code does (`expect(add(a, b)).toBe(a + b)`, a snapshot derived by hand the same way, a constant asserted equal to itself), so it passes by construction and can never disagree with the code. Expected values must come from an independent source of truth — a known-good literal, a worked example, the spec.
- **Horizontal slicing** — writing all tests first, then all implementation. Bulk tests verify _imagined_ behaviour: you test the _shape_ of things rather than user-facing behaviour, the tests go insensitive to real changes, and you commit to test structure before understanding the implementation. Work in **vertical slices** instead — one test then one implementation then repeat, each test a **tracer bullet** that responds to what the last cycle taught you.

## Rules of the loop

- **Red before green.** Write the failing test first, then only enough code to pass it. Do not anticipate future tests or add speculative features.
- **One slice at a time.** One seam, one test, one minimal implementation per cycle.
- **Run tests through the project's toolchain.** In this repo that means the pinned environment — for example `devenv shell -- bun test`. Confirm the runner before you start looping so red and green are reliable signals.
- **Refactoring is not part of the loop.** It belongs to the review stage (run `/code-review`), not the red to green implementation cycle.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).

