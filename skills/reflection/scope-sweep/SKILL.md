---
name: scope-sweep
description: Check finished work for incomplete tasks, missed requirements, unhandled scope, and operational follow-ups. Use before declaring a scope complete, citing evidence for each remaining gap.
---

# Scope Sweep

Run a final completeness pass after the core work and its review, before declaring the scope done. The question is whether everything in scope was delivered, not whether the delivered code is correct.

## Process

1. Restate the scope: the user's original request, any agreed plan or spec, and follow-ups added during the session.
2. Search the changed files and the session for unfinished markers: TODO, FIXME, placeholder values, stubbed functions, skipped tests.
3. Check each item below against that scope, citing a file, line, command, or message as evidence.

## Check for

- **Incomplete tasks**: work you planned or promised but did not finish.
- **Missed requirements**: anything stated or clearly implied in the request that no change handles.
- **Unhandled scope**: related files, callers, docs, configuration, or migrations the change should have touched.
- **Open questions**: decisions you made on the user's behalf that they should confirm.
- **Testing gaps**: new behavior with no test or verification run.
- **Operational follow-ups**: deployment, configuration, monitoring, rollback, or documentation steps someone still has to perform.

## Report

List each gap with its evidence and a proposed action, grouped as "finish now" or "hand to the user". Close with the single biggest remaining gap. If the scope is complete, say so and name the evidence.
