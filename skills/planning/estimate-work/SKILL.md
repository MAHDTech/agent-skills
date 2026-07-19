---
name: estimate-work
description: Size and estimate a body of work to set expectations and sequence delivery — turn a plan or ticket set into defensible estimates with ranges, named assumptions, and surfaced risk instead of false-precise point numbers. Use when the user wants to estimate or size work, forecast how long something will take, choose between relative sizing and time-based estimates, calibrate against past delivery, or decide whether to spike before committing.
---

# Estimate Work

An estimate sets expectations and sequences delivery — and its job is as much to **surface uncertainty** as to produce a number. A false-precise point estimate hides the risk that matters; a defensible estimate carries its **range**, its **assumptions**, and the unknowns it's betting on.

## Decompose until estimable

You can't size what you can't picture. Break the work down until each piece is small enough to reason about concretely — a scale where you've done something like it before. If a piece resists that, and you genuinely don't know how hard it is, that isn't an estimate, it's a **spike** (see below). Decomposition also cancels error: many small estimates miss high and low independently and partly wash out, where one big guess has nowhere to average.

## Size relative, or absolute — know the trade

Two ways to size, each with a cost:

- **Relative sizing** — rank items against each other (story points, T-shirt S/M/L, or plain "about twice that one"). Fast, resists false precision, and calibrates against your own throughput. But it needs a **velocity** — a size-to-time translation — before anyone gets a date, and it drifts if the scale isn't re-anchored.
- **Absolute time** — estimate hours or days directly. Reads as a date immediately, but invites false precision and is anchored to _who_ does the work, so it doesn't transfer between people.

Default to relative for a body of work you'll sequence; reach for absolute only when a single owner needs a near-term calendar commitment.

## Give ranges, not points

State every estimate as a **range** that carries your confidence — best case, likely, worst case — never a lone number the world will read as a promise. The spread _is_ the information: a tight range says you understand the work, a wide one says you don't yet, and both are honest. When one number is unavoidable, quote the confident end, not the hopeful one.

## Calibrate against reality

Anchor to the **reference class**, not to optimism: how long did _similar_ work actually take last time? Historical throughput beats bottom-up addition because it already includes the interruptions, rework, and review a fresh guess omits. Absent your own history, widen the range — the outside view is the correction for the planning fallacy, where every estimate slides toward the sunny path.

## Name what you're betting on

Every estimate stands on **assumptions** — the API behaves, the data's clean, the owner's free. Write them down beside the number; an unnamed assumption is a silent risk. Treat each unknown as a **risk** with a size and a likelihood, surfaced rather than buried in padding — padding hides the risk inside a fudged number, where a named risk can be discussed, mitigated, or spiked.

## Spike, don't guess

When the uncertainty is too large to estimate — you can't even bound the range — **spike** instead: a timeboxed investigation whose deliverable is _knowledge_, not the feature. Timebox it so one open question can't swallow the schedule, and estimate the real work only once the spike closes and its output is a better estimate.

## Handoffs

- Estimating a plan or ticket set? Size each slice as you cut it with `/to-tickets` — buildable slices are the natural estimation unit.
- Sizing is not ranking: this skill sizes the work, `/prioritize-backlog` orders it by value against cost. Estimate, then prioritise.
- When a body of work is too foggy to decompose or size at all, it isn't an estimate but an investigation; chart it with `/wayfinder`.
