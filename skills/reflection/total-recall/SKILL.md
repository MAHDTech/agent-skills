---
name: total-recall
description: Record a human-reviewable decision trail for long-running, autonomous, or high-risk tasks. Writes an append-only TSV decision log (ts, phase, decision, why, evidence, result) and audits it against runtime transcripts. Use when running multi-phase migrations, autonomous loops, risky refactors, or when requested via /total-recall.
metadata:
  source: mahdtech/agent-skills
  license: MIT
---

# Total Recall

Record a permanent, human-reviewable memory trail of what was decided, why, and on what evidence during autonomous or long-running work.

When an agent works unattended, across multiple phases, or makes irreversible technical choices, a human reviewer needs to reconstruct what happened without reading thousands of lines of transcript or re-running the entire workflow. Maintain a single canonical decision trail so the work is auditable, reproducible, and verifiable.

## The Log Format

A single TSV (Tab-Separated Values) file with one row per decision or checkpoint:

- **ts:** ISO8601 UTC timestamp (`YYYY-MM-DDTHH:MM:SSZ`).
- **phase:** The current lifecycle phase or work stream (e.g. `discovery`, `harness`, `refactor`, `verification`).
- **decision:** What was chosen or executed, expressed in a single line.
- **why:** The plain-language rationale. State the direct reason or constraint without buzzwords or robotic jargon.
- **evidence:** A concrete pointer that verifies the claim (e.g. commit SHA, PR number, `file:line`, test output path, screenshot artifact). Never multi-line prose.
- **result:** The concrete outcome or verified state (e.g. `tests green`, `reverted`, `pixel-diff 0`, `INCONCLUSIVE`, `open`).

Start a clean decision log by copying the header from `resources/manual/decision-log-template.tsv`.

### Example Decision Log

| ts                     | phase     | decision                                              | why                                                         | evidence                           | result                                |
| :--------------------- | :-------- | :---------------------------------------------------- | :---------------------------------------------------------- | :--------------------------------- | :------------------------------------ |
| `2026-05-24T09:02:00Z` | `frame`   | Counted work first, about 100 components and 75 hours | Needed scope bounds before starting autonomous run          | `commit 3a9f1c2`                   | Identified 5 blockers before starting |
| `2026-05-24T09:40:00Z` | `harness` | Captured baseline screenshots before modifying styles | Ensure visual regression harness catches deviations         | `scripts/snapshot.sh`, `baseline/` | Saved 120 reference snapshots         |
| `2026-05-24T11:15:00Z` | `widget`  | Migrated component styles to tailwind utilities       | Keep changes surgical and behavior identical                | `commit 7c21e0a`, `pixel-diff 0`   | Visual diff 0, tests pass             |
| `2026-05-24T12:30:00Z` | `widget`  | Discarded subagent output due to empty visual diffs   | Verified source files directly rather than trusting summary | `worktree reset`                   | Reverted and added strict validation  |

## Logging a Row

Write entries in plain, direct language as if briefing a colleague. Avoid AI buzzwords, puffery, and abstract corporate speak (the [unslop](../../writing/unslop/SKILL.md) skill applies to decision logs too).

### Using the Helper Script

Use the included helper script to guarantee well-formed TSV entries and guard against formula injection:

```bash
skills/reflection/total-recall/resources/manual/log.sh <logfile> <phase> <decision> <why> <evidence> <result>
```

The script automatically:

1. Stamped ISO8601 UTC timestamps.
2. Initializes headers on first use if the log file does not exist.
3. Strips internal tab and newline characters.
4. Prefixes cells starting with `=`, `+`, `-`, or `@` with a single quote (`'`) to prevent spreadsheet formula injection.

### What to Log

- **Major technical forks:** Choices between architectural approaches or trade-offs.
- **Milestone checkpoints:** Completion of discrete units with their associated test verification.
- **Pivots and rollbacks:** Any approach abandoned or reverted, along with the triggering evidence.
- **Blockers and anomalies:** Unresolved issues or unexpected environment behaviors surfaced.
- **Loop iterations:** In autonomous loops, log one row per iteration summary.

Do not log trivial, self-evident edits (e.g. fixing a typo, running a standard linter).

## Where the Log Lives

- **Working artifact (default):** Keep the file at `decisions.tsv` in the repository root or `.audit/<task-slug>.tsv` for parallel tasks. Keep it local and uncommitted for routine work.
- **Committed record:** Commit the log when working on large cross-system migrations, high-risk refactors, or when human reviewers explicitly require a persistent audit trail.

## Core Rules

1. **One row, one decision:** Every row must represent a single, focused checkpoint fitting entirely on one line.
2. **Append-only integrity:** Never edit or delete existing rows. If a prior decision is reversed or found flawed, append a new row recording the pivot and rationale.
3. **Concrete evidence pointers:** Evidence must point to a reproducible artifact, commit SHA, file line, or test run.
4. **No em-dashes:** Do not use Unicode U+2014 em-dashes in log messages or documentation. Use standard hyphens, colons, or commas.

## Transcript Audit

Before finishing a run, verify that the log reflects ground truth by auditing it against the conversation transcript:

1. **Verify actions:** Confirm every logged decision corresponds to an actual command or file modification.
2. **Verify evidence:** Check that listed commits, test outputs, or files actually exist and prove the claimed result.
3. **Check for missing pivots:** If an approach was attempted, failed, and abandoned during the run, ensure that pivot is recorded.
4. **Prune noise:** Remove trivial or redundant rows that add cognitive load without audit value.

## Cross-Model Review

For high-stakes tasks where subagent execution is available, spawn a subagent on an alternative model family to provide fresh-eyes review:

1. Pass the decision log and execution summary to the reviewing subagent.
2. Have the reviewer flag:
   - Weak, missing, or unverifiable evidence pointers.
   - Skipped verification steps or unverified assertions.
   - High-risk choices or subtle scope creep.
3. Conclude the final summary with an `Attention` section citing findings from the reviewer.

## Reviewing the Trail

- **In the terminal:** Render formatted TSV tables using `column`:

  ```bash
  column -s$'\t' -t decisions.tsv | less -S
  ```

- **On GitHub / Web:** GitHub natively renders committed TSV files as interactive, sortable tables.
