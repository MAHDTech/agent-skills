---
name: characterization-tests
description: Pin down the existing behaviour of untested or legacy code with characterization (golden-master / approval) tests before you change it, so refactors and upgrades stay safe. Use when you must modify, refactor, or upgrade code that has no tests, want to capture current behaviour as an oracle even where it looks wrong, or need a safety net before a risky change. Distinct from /tdd, which specifies new behaviour test-first; this tests around existing behaviour. Pairs with /sculpt-code and /upgrade-dependencies.
---

# Characterization Tests

Legacy or untested code you are about to change is a cliff with no guardrail. A **characterization test** captures what the code _currently does_ - its **oracle** is present behaviour, not correct behaviour - so any change that alters that behaviour turns a test red. Build the net first, then change the code.

This is test-_around_ existing behaviour. For test-_first_ design of _new_ behaviour, run /tdd. Once the net is green, /sculpt-code and /upgrade-dependencies are safe to run against it.

Run the suite through the project's toolchain - for example `devenv --no-tui shell -- <cmd>` - so red and green mean the same thing every run.

## Phase 1 - Find the seam

A **seam** is a place where you can observe the code's behaviour without reaching inside it (shared vocabulary with /tdd and /codebase-design). Pick the **highest** seam that still covers the code you are about to change - a whole function, module, HTTP endpoint, or CLI invocation - so you pin behaviour without depending on internal structure you are about to move.

If the code will not run in isolation because it is wired straight to a clock, network, database, or global state, that is a dependency to break, not a reason to test deeper. Introduce a seam - inject the dependency, wrap the call, subclass-and-override - so the code runs under a harness you control.

Completion: you can invoke the target from a test and observe its output, without a live clock, network, or datastore you do not control.

## Phase 2 - Capture the oracle

Write a test that calls the code and asserts something you know is wrong - empty, zero, a placeholder. Run it. The failure message hands you the code's _actual_ output. Paste that observed value back as the expected value. The test now documents reality.

For wide output - a rendered page, a large object graph, a generated file - do not hand-transcribe. Capture the whole output as a **golden master** (a.k.a. an approval snapshot) and diff future runs against it, storing the approved output as a checked-in fixture.

Record present behaviour faithfully, **even behaviour that looks like a bug**. You are documenting what _is_, so you can tell when a change moves it. Correctness comes later and deliberately.

Completion: green test(s) whose expected values were read from the code's real output, not from what you think it should be.

## Phase 3 - Pin the corners

One passing test pins one path. Cover the branches you are about to touch:

- Feed inputs that exercise each branch through the change region; use a coverage tool to see which lines your tests actually hit.
- Pin every source of nondeterminism the output depends on - freeze the clock, seed the RNG, fix collection ordering, stub network and generated IDs. An unpinned characterization test is flaky and worthless.
- When an assertion **surprises** you - behaviour you did not expect - stop and flag it to the user. That is a bug-or-feature decision, not something to normalise silently into the expected value.

Completion: every branch in the change region is covered and deterministic, and the suite is green twice in a row.

## Phase 4 - Change behaviour deliberately

The net is live. Now change the code:

- **Pure refactor** (behaviour must not change): the characterization tests stay green, untouched. A red test means you changed behaviour by accident - revert and take a smaller step. Run /sculpt-code.
- **Intended behaviour change**: update the affected expected values, or re-approve the golden master, in the **same commit** as the code change - so the diff shows exactly which behaviour moved and a reviewer can see it.

If a later upgrade or refactor breaks a characterization test you did not mean to touch, run /diagnosing-bugs with that red test as the ready-made feedback loop.
