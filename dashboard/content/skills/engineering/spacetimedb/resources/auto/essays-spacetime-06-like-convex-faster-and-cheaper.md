+++
title = "essays-spacetime-06-like-convex-faster-and-cheaper"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

# Why I would choose Spacetime over Convex: speed, cost, and room to grow

I love a backend that makes the first version easy and gives me reasons to stay excited about the hundredth. Spacetime is my favorite expression of that idea.

Convex helped make an excellent idea familiar: the backend can own both application state and the work of keeping clients current. Spacetime takes that productive model and puts transactional application logic inside the database engine. That architecture is the reason I would choose it first.

I want the convenience, the performance headroom, and an operating bill that leaves room for the product to succeed. The evidence below makes a strong case that Spacetime can deliver that combination for compute-heavy applications.

“Like Convex, faster and cheaper” gets my attention. The actual numbers are why I would tell another engineer to build on Spacetime.

## Faster, on a specified workload

In Clockwork Labs' published May 2026 account-transfer benchmark, SpacetimeDB Standalone processed **303,920 transactions per second** under contention. Self-hosted Convex processed **127**. The ratio of those published means is approximately **2,393×**.

With uniform account selection, the figures were **279,025** and **1,140 TPS**, or approximately **245×**. Those are completed-transfer throughput ratios, not per-request latency improvements. [Benchmark report](https://spacetimedb.com/blog/benchmarking).

The result is vendor-produced and specific to these implementations. The headline tests give Spacetime 40 in-flight requests per client while Convex's clients do not pipeline. A separate uniform-workload run with Convex pipelining reaches about 1,154 TPS. Spacetime's results use single-node Standalone, so they should not be presented as a managed-cloud performance guarantee. [Methodology and caveats](03-performance-that-changes-the-design.md).

These are enormous throughput differences for the measured workload. I think any team building around busy shared state should take them seriously. For me, they move Spacetime to the front of the line for SaaS products, collaborative tools, live operations software, games, and agent coordination.

## Cheaper, with the assumptions visible

Pricing checked September 18, 2026, in USD. Convex figures use the published US East rates. These are modeled bills, not measurements of an application deployed to both clouds.

Spacetime Maincloud Pro costs **$25/month**, includes **100,000 TeV** of usage credit, and charges **$1 per additional 2,592 TeV**. The plan lists ten included collaborators per database. All metered resources draw from the same credit pool. [Spacetime pricing](https://spacetimedb.com/pricing).

Convex Professional costs **$25/developer/month**, includes **25 million function calls**, then charges **$2 per additional million**. Other resources have their own allowances and rates. [Convex pricing](https://www.convex.dev/pricing).

For Spacetime, I used the checked-in calculator's default reducer model: 10,000 bytes scanned, 1,000 bytes written, 100 index seeks, and 1 million CPU instructions per call. At the published energy rates, that is **840 TeV per million modeled calls**. This is a workload assumption, not a fixed reducer-call tariff. [Spacetime pricing and calculator](https://spacetimedb.com/pricing).

Now assume one developer, 1 GB of table/database storage for a 30-day month, 10 GB of egress, no action compute, no file or search charges, and Convex database I/O within its included allowance. The modeled calls below are each platform's billable calls; one user action need not produce the same count on both platforms.

| Monthly billable calls | Spacetime Pro, modeled | Convex Professional, modeled |                 Difference |
| ---------------------- | ---------------------: | ---------------------------: | -------------------------: |
| 100 million            |                 $25.00 |                      $175.00 |   $150.00; about 86% lower |
| 1 billion              |                $312.27 |                    $1,975.00 | $1,662.73; about 84% lower |

Here is the first row in enough detail to check on a napkin.

Spacetime's modeled compute consumes 100 × 840 = **84,000 TeV**. Storage uses **2,592 TeV**, and egress uses **2,000 TeV**. Total: **88,592 TeV**, within the 100,000-TeV credit. The bill remains **$25**.

Convex's modeled bill is **$25 + (100 − 25) × $2 = $175**. The specified storage and egress fit inside Professional's listed allowances.

At one billion calls, Spacetime uses **844,592 TeV** including the same storage and egress. Its modeled bill becomes **$25 + (844,592 − 100,000) / 2,592 = $312.27**. Convex's becomes **$25 + (1,000 − 25) × $2 = $1,975**.

For five developers at 100 million calls, the same assumptions put Convex Professional at **$275** and Spacetime Pro at **$25**, provided the team fits the included Spacetime collaborators. That comparison reflects the different seat models as well as usage.

## What I would put into the budget

I would budget against the actual resource profile of the application. That is how the numbers above become useful engineering decisions.

An expensive reducer consumes more energy. Wide subscriptions increase egress. Large resident tables increase storage charges. A read-heavy application, a large archive, or an action-heavy integration can have a very different bill. Convex's much cheaper published storage rate can matter enormously for a storage-heavy product.

Function counting also matters. Convex includes reactive query executions in billable function usage; Spacetime meters the underlying resource work. Counting only the clicks in a UI would give neither system a trustworthy estimate. [Convex's usage documentation](https://docs.convex.dev/production/state/limits).

For the explicit compute-heavy scenarios above, Spacetime has a substantial modeled cost advantage. Its published transfer benchmark separately shows a substantial throughput advantage. Together, those give me two strong reasons to choose the architecture; they remain separate measurements and calculations.

## Why I care as someone trying to ship a product

Cost and performance affect the features I am willing to build. A product might be better with more frequent progress updates, richer shared state, or more automation. I want to explore those ideas with an architecture that gives me room to work, then measure the actual resource costs.

I also want the team to spend less time anticipating the next infrastructure rewrite. A fast, integrated backend gives me a compelling starting point for keeping the application in one understandable model. That is valuable to an independent developer and to a company hiring its next ten engineers.

This is the combination that makes me evangelical about Spacetime. I can choose it because development feels good, and also point to serious performance results and attractive cost scenarios. I do not have to treat those as unrelated conversations.

For a new app where I am choosing between Convex and Spacetime, I would choose Spacetime unless a specific requirement pushes me elsewhere. I want the architecture with this much headroom behind the programming model. I think many more teams should want it, too.


