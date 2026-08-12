---
name: optimize-performance
description: Improve the performance of a hot path with measurement-first discipline — profile to find the real bottleneck, set a target, change one thing, re-measure, and stop when the target is met. Use when code is too slow, memory-hungry, or throughput-bound and you want to make it deliberately faster without ever guessing at the cause. For a performance regression — code that was fast and got slow — run /diagnosing-bugs instead, which treats the slowdown as a bug to bisect.
---

# Optimize Performance

You cannot make faster what you cannot measure. Every phase here defends one rule: **measure, don't guess.** Profile to find where the resource actually goes, change one thing, and let the numbers — not intuition — say whether it worked.

This is deliberate optimisation of code that was never fast enough. For code that _was_ fast and got slow — a performance **regression** — run /diagnosing-bugs, which bisects the slowdown like any other bug.

Run measurements through the project's toolchain — for example `devenv --no-tui shell -- <cmd>` — so results are comparable across runs.

## Phase 1 — Set the target

Optimisation with no target never ends. Before profiling, write down:

- The **metric** that matters to the user: latency (p50 / p99), throughput, memory footprint, allocation count, startup time — pick one.
- The **workload** it is measured on.
- The **current** number and the **target** number. "p99 under 200ms" or "2× the throughput", never just "faster".

Without a target you cannot tell success from a rabbit hole, and you will trade readability for speed nobody needs.

## Phase 2 — Build the benchmark harness

A **benchmark harness** is the performance counterpart of a test's feedback loop: one command that runs a **representative workload** and prints the metric.

- **Representative** — production-like data sizes, shapes, and distributions. A microbenchmark over a ten-element list lies about behaviour on a million.
- **Repeatable** — run it several times, report the median and the spread. Warm up first if the runtime has a JIT or cache, quiet the machine, and pin what you can.
- **Comparable** — the same workload every run, so two measurements can be diffed.

Take the **baseline** measurement now and record it. Every later change is judged against this number.

Completion: one command you have run at least once that prints the metric on the representative workload, with the baseline recorded.

## Phase 3 — Profile to find the bottleneck

Never optimise from a guess about where the resource goes — you will be wrong, and you will pay readability for nothing. **Profile** the workload and read where the cost is actually spent: a CPU / sampling profiler, a heap / allocation profiler, a query plan or slow-query log, a flame graph — whatever fits the metric.

Rank the costs. The **bottleneck** is the biggest slice. **Amdahl's law**: speeding up code that is 5% of the runtime caps your win at 5%, however clever the change — so optimising **cold code** is wasted effort. Attack the hot slice first.

Completion: you can name the specific function, query, or allocation site that dominates the metric, backed by profiler evidence.

## Phase 4 — Change one thing, re-measure

Change **one** thing against the bottleneck — a better algorithm or data structure, caching a repeated computation, batching I/O, cutting allocations, hoisting work out of a loop. Then re-run the harness and compare to the baseline:

- **Faster past the noise** → keep it, make it the new baseline, and re-profile — the bottleneck has probably moved.
- **No real gain, or lost in the noise** → revert. An unmeasured "optimisation" is only risk and added complexity.

One variable per measurement, or you will not know which change earned the win.

## Phase 5 — Stop at the target

Stop when the metric meets the **target** from Phase 1. Then confirm:

- [ ] Target met on the representative workload, shown against the recorded baseline.
- [ ] The full test suite still passes — you changed _how_ the code computes, not _what_ it computes.
- [ ] Readability cost accounted for. A change that beats a target you had already met, at the price of legibility, is a bad trade — revert it. Where fast code reads as strange, leave a comment saying why.

Chasing speed past the target spends readability for a win nobody asked for.
