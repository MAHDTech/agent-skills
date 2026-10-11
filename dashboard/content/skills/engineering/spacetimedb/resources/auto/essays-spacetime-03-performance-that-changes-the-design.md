+++
title = "essays-spacetime-03-performance-that-changes-the-design"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

# Spacetime's performance should change what we expect from a backend

Spacetime's performance makes me want to raise the bar for the entire backend industry.

I have no desire to spend my career making straightforward business operations fit inside an unnecessarily expensive architecture. I want to build ambitious applications, keep their rules understandable, and have enough execution headroom that every shared record does not immediately become a design crisis.

That is why Spacetime's performance matters to me. An application with enough transactional headroom can keep straightforward business rules straightforward. I can update shared state directly instead of immediately designing around contention, introducing a queue, or deciding which parts of the interface are allowed to be stale.

I think this has consequences well beyond games or specialized high-frequency systems. SaaS products have shared workspaces. Operations tools have busy queues. Commerce has inventory. Agents compete for tasks. Contention is part of useful software. An architecture that handles it well deserves a much broader audience.

The published numbers make that argument hard to ignore.

## What the published benchmark actually measured

Clockwork Labs' May 14, 2026 benchmark measures successful account-transfer transactions through complete backend stacks. This is a vendor-run benchmark, not an independent certification. The implementation and raw results are available for inspection.

The test covers two account-selection distributions: uniform selection, labeled α=0, and a power-law distribution with hot accounts, labeled α=1.5. The latter creates contention because many transfers touch the same records.

These are the published mean throughputs; ± denotes the reported standard deviation over the measurement window.

| Backend                            | Uniform selection, TPS | Contended selection, TPS |
| ---------------------------------- | ---------------------: | -----------------------: |
| SpacetimeDB Standalone, TypeScript |        279,025 ± 4,763 |          303,920 ± 4,712 |
| Bun + Postgres                     |           10,730 ± 146 |               2,773 ± 61 |
| Node.js + Postgres                 |            9,905 ± 224 |                 961 ± 26 |
| Node.js + Supabase                 |          7,362 ± 1,179 |               2,534 ± 57 |
| Convex, self-hosted                |            1,140 ± 118 |                 127 ± 53 |

Source: [published benchmark and methodology](https://spacetimedb.com/blog/benchmarking), backed by the [raw benchmark data](data/pipelined-off.tsv).

Using the rounded published means, Spacetime delivered about **2,393× Convex's throughput**, **316× Node.js + Postgres's throughput**, and **110× Bun + Postgres's throughput** in the contended workload. With uniform selection, the corresponding ratios were about **245×**, **28×**, and **26×**.

Those are throughput ratios for these implementations and configurations. They are not latency ratios, universal database speed rankings, or promises about a production application.

## The setup belongs beside the result

The local tests used an Intel 14900K machine. Runs lasted five minutes, with a 30-second warmup excluded from the steady-state analysis. Spacetime ran as a single-node Standalone deployment; the reported Convex results were also self-hosted. These results do not measure replicated Maincloud against managed Convex Cloud.

There is an important load-generation difference: the headline comparison uses 64 clients with up to 40 requests in flight per client for Spacetime, while the listed competitors use 64 clients without that pipelining. Separate tests enabled pipelining for competitors on the uniform workload. Convex's steady-state throughput there was about 1,154 TPS, versus about 1,140 without pipelining. That additional result helps explain the gap, but it does not erase the different settings in the headline table. [Pipelined raw statistics](data/pipelined-on.tsv).

This benchmark tests one transfer workload through particular application stacks. It does not test an optimized Postgres stored-procedure implementation, analytical SQL, every indexing strategy, or large subscription fan-out. Those are separate experiments.

## A faster architecture gives me more freedom

In a conventional backend, my application code often opens a transaction, reads rows, makes a decision, writes rows, and commits. Several of those steps can involve communication between the application server and database. Even when both are on the same machine, they remain separate runtimes with a protocol between them.

When transactions contend, time spent holding the contested state matters enormously. More connections cannot make two conflicting updates independent.

Spacetime puts the reducer next to the data. Reads, validation, and writes happen inside the engine. The transactional work no longer needs a network conversation between an application server and its database. Postgres functions can also reduce that conversation; Spacetime makes the integrated execution model the normal way I write the backend.

As a simple illustration, a serialized critical section lasting 10 milliseconds permits at most 100 such operations per second. At 10 microseconds, the arithmetic ceiling becomes 100,000. That is an explanatory model, not an additional measured result. It shows why locality can matter more than the number of machines on the invoice.

## Performance belongs in the developer experience

The latency results also keep the story honest. Under contention, the reported p50/p99 summaries were 7.39/11.7 milliseconds for Spacetime, 7.85/13.2 for Bun + Postgres, and 20.2/1,082 for Convex. Under uniform selection, Bun + Postgres had a lower reported p50 than Spacetime: 5.66 versus 8.08 milliseconds.

The achievement is dramatically more completed transactional work while retaining useful latency. That is exactly the combination I want when an application becomes busy.

This is why I consider performance part of the developer experience. Headroom can let me keep a business operation in one transaction. It can let me try a richer interaction before introducing batching or delayed updates. It can make a simpler design viable for longer. Those are product and engineering benefits, even when users never see a TPS graph.

I would love more teams to ask how much of their architecture exists because their chosen execution model made the obvious implementation too slow. Queues, caches, and services all have legitimate uses. I want to introduce them because the application needs their semantics, with enough performance available to make that a deliberate choice.

My recommendation is broader than “try this if you are building a game.” If you are starting a SaaS product, an internal tool, a collaborative application, or an agent backend, put Spacetime at the front of the evaluation. Load-test your actual reducers, subscriptions, durability configuration, and data volume. Start with the architecture that gives you a compelling shot at keeping the application simple.

I love Spacetime because its speed serves the way I want to write software. It makes me more ambitious about the product and less resigned to infrastructure work. I think that is a standard every modern backend should have to meet.


