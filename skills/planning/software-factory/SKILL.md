---
name: software-factory
description: Run a feature through four approval gates - Product, Architecture, Program Design, Slices - so every decision that matters is made and signed off before implementation code exists.
disable-model-invocation: true
---

# Software Factory

Make every decision that matters while changing it still costs a sentence. Once thousands of lines exist, the same change costs a rewrite - and the sessions that produce design docs are context-light, so that is where you get the most model intelligence.

Four gates, in order. Each ends at a written doc and an explicit approval, and the work stops there until the approval lands.

```text
Gate 1 Product  →  Gate 2 Architecture  →  Gate 3 Program Design  →  Gate 4 Slices
```

The default failure this prevents is **horizontal building**: all the schema, then all the services, then all the UI, and a 2,000-line diff nobody can review. Gate 4 cuts the work vertically instead.

## Files and state

Everything for one feature lives in `docs/plans/<feature-slug>/`:

```text
docs/plans/<feature-slug>/
  00-status.md          gate approvals + pointer to the slices
  01-product.md
  mockups/              one plain HTML file per screen
  02-architecture.md
  03-program-design.md
```

Create `00-status.md` before Gate 1 and update it at every approval - its shape is in [`resources/manual/status-file.md`](resources/manual/status-file.md).

**Resume rule.** Before anything else, look for `docs/plans/<feature-slug>/00-status.md`. If it exists, read every doc in that folder, then start from the first gate not marked APPROVED. An approved gate is redone only when the user asks or a later gate invalidated it.

## The approval protocol

Run this at the end of every gate, unchanged:

1. Write the gate doc to disk.
2. Summarise it in the chat as at most ten bullets of **decisions**, plus the doc path. The doc is the artifact; the chat is the pointer.
3. Ask: **"Approve Gate N, or what should change?"**
4. Treat only a clear yes as approval. Anything else is a revision: change the doc, then ask again.
5. On approval, mark the gate APPROVED with the date in `00-status.md`.

**Backtracking.** When a later gate proves an approved decision wrong, stop there, correct the earlier doc, set that gate back to in progress, and win its approval again before resuming.

## Gate 1 - Product

The user's problem, in the user's words. Databases, endpoints, schemas, and file names belong to Gate 2 - when one surfaces here, park it there and carry on.

Interview the user with `/grilling` until the answers are sharp, fill the template in [`resources/manual/gate-1-product.md`](resources/manual/gate-1-product.md), and save it as `01-product.md`.

Where there is a UI, build one plain HTML file per screen in `mockups/` - no framework, no build step, thrown away once the real screens exist. Iterate on them with the user until the screens are right.

Run the approval protocol.

## Gate 2 - Architecture

Read the existing code that this feature touches first - design against the real codebase, not an imagined one.

Fill the template in [`resources/manual/gate-2-architecture.md`](resources/manual/gate-2-architecture.md) and save it as `02-architecture.md`.

Run the approval protocol.

## Gate 3 - Program Design

The gate everyone skips, and the one that pays: it surfaces the decisions the agent would otherwise make silently, mid-implementation, where challenging them is expensive.

Fill the template in [`resources/manual/gate-3-program-design.md`](resources/manual/gate-3-program-design.md) and save it as `03-program-design.md`. Types and signatures carry no bodies - a human reads them in seconds and says "right" or "wrong".

Run the approval protocol.

## Gate 4 - Slices

Slice the approved design into **tracer bullets** and hand off:

> "Gates 1–3 are approved and the docs are in `docs/plans/<feature-slug>/`. Run `/to-tickets` to slice this into tracer-bullet tickets, then come back."

`/to-tickets` owns the slice shape. This gate adds one ordering constraint: **slice 1 is the tracer bullet** - a stubbed response wired end to end, doing almost nothing but running and visible. Slice 2 replaces the stubs with the happy path. Every slice after that adds one capability and ends in a working, testable state.

Review the resulting tickets with the user and run the approval protocol on the slice plan. Record where the tickets live in `00-status.md` - the tracker owns slice progress from here, so the status file points at it rather than copying it.

Then build them: `/implement`, which drives `/tdd` per slice. After each slice, prove it runs - execute it, curl it, or drive the UI - and show the user the result before asking whether to continue or re-steer.

## Standing rules

- **Compact at every boundary.** At the end of each gate and each slice, make sure nothing decided lives only in the chat. Say plainly that this is a safe point to start a fresh session - a new one must be able to continue from the docs alone. When the harness warns that context is low, compact immediately, wherever you are.
- **Keep the human in the code.** Small slices, reviewable diffs. When the user has gone a long stretch without reading any code, say so at a slice boundary - losing touch with the codebase costs weeks, right when the agent hits the bug it cannot solve.
- **Record what outlives the feature.** When a gate produces a decision bigger than this feature, use `/domain-modeling` to write it down as an ADR. Facts that live outside the repo but that a future session needs - env var names, test accounts, third-party dashboards - belong in `docs/external/`. Files on disk are free context.

> The four-gate playbook and the compact-at-every-boundary rule come from Dex Horthy (HumanLayer), via his appearance on David Ondrej's podcast.
