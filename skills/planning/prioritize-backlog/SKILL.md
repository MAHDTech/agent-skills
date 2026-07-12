---
name: prioritize-backlog
description: Groom and prioritise a backlog — order, cut, cluster, and sequence a pile of work by value against cost and risk so the next thing to do is obvious. Use when the user wants to prioritise or rank a backlog, decide what to build next, trim or triage a pile of ideas, apply a lens like value-vs-effort, RICE, or cost-of-delay/WSJF, or sequence work around dependencies.
---

# Prioritize Backlog

A backlog is a pile until it's groomed. Grooming turns the pile into an ordered queue whose **top is unambiguously the next thing to do** — ordered by **value** against **cost** and **risk**, low-value items cut, and no blocker ever sitting above the work it gates.

## Prepare the items

Pull every candidate into one list. Each item is a discrete unit of **value** — an outcome someone wants, not a task ("users can reset their password", not "add a reset endpoint"). Split anything too big to compare against its neighbours; merge slivers that only deliver together.

## Pick a lens and score

Score every item through **one** lens — mixing lenses makes the ranks incomparable. Each trades effort for fidelity; pick the cheapest that actually separates your items:

- **Value vs effort** — plot each item on value against effort. Do the high-value / low-effort quadrant first; interrogate the low-value / high-effort one. Fastest, coarsest.
- **RICE** — score Reach × Impact × Confidence ÷ Effort. Adds reach and an explicit confidence discount, so a wide, well-understood item beats a narrow, speculative one of equal appeal.
- **Cost of delay / WSJF** — rank by cost-of-delay ÷ duration (weighted shortest job first). Reach for it when _timing_ dominates — a deadline, a compounding cost, a closing window — because it prices what waiting costs, not just what doing is worth.

These are options, not gospel. Whatever the lens, the axis is the same: value earned against cost and risk paid.

## Cut before you order

The highest-leverage move in grooming is subtraction. For each low-scoring item ask: if it never ships, who notices? If the honest answer is nobody, cut it. Cut duplicates, items overtaken by newer work, and anything whose value has decayed below its cost. A backlog that only grows is a graveyard — its tail is never reached, so keeping it costs attention for nothing.

## Cluster and sequence

- **Cluster** items that share a component, a theme, or a user journey, so you batch related work instead of thrashing between contexts.
- **Sequence** by score, then reorder for dependencies: no item may sit above one that **blocks** it, however high it scores. Walk the list top-down and pull each blocker above the work it gates. When a low-value item is the only thing unblocking a high-value one, its real priority is borrowed from what it unblocks.

## Keep it small

Cap the groomed, ordered zone — only the next chunk needs to be sharp. Everything past the line stays coarse: a rough parking lot you re-groom, not a queue you maintain. Re-groom on a cadence (each iteration, each intake) rather than letting the pile drift.

## Handoffs

- Once the top items are ordered and sharp, split them into buildable slices with `/to-tickets`.
- When an item is too big to see the end of — you can't yet state its value or its cost — it isn't a backlog item but an investigation; chart it with `/wayfinder`.
