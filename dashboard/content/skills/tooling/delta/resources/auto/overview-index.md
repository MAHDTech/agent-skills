+++
title = "overview-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
+++

---
title: "Delta — A Multiplayer Environment for Coding with Agents"
url: "https://delta.dev"
description: "Build and review software with teammates and agents in shared threads, with every conversation and code change kept in context."
---

From the creators of Zed

# Delta brings meaningnathanmaxantonioback to your code

Your conversations and code live in one place. Delta anchors every change, so your reasoning lives on with the results.

[DownloadD](/download)[Open DeltaD](/threads)[Learn moreabout Delta in the docsL](/docs)

Pinned

Trace dropped keystrokes in GPUI inputs

Review element cache invalidation rules

Projects

zed

gpui

Cache SumTree node summaries

Investigate cache invalidation Trace summary propagation paths Check cursor invalidation callers

Fix IME candidate window positioning

Add scroll wheel phase events on macOS

Simplify Entity subscription cleanup

Reproduce stale text after window resize

cosmic-text

Threads

Summarize feedback on my first GPUI PR

Find a small rendering issue to work on

Explain retained mode rendering

Compare Metal frame capture tools

Draft release notes for accessibility fixes

![nathansobo](https://github.com/nathansobo.png)nathansoboZed

Cache SumTree node summaries

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)![maxbrunsfeld](https://images.zed.dev/utils/max.png)![as-cii](https://images.zed.dev/utils/antonio.jpeg)

Share

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)

Cursor::seek recomputes each internal node's summary on every descent — for a rope of a few MB that's most of our seek time. Can we cache the summaries on the nodes and invalidate on edit?

Thought 3 times, ran 1 search, read 2 files

Edit crates/sum\_tree/src/sum\_tree.rs

```
    Internal {        height: u8,        summary: T::Summary,        summary_cache: SummaryCache<T::Summary>,        children: Vec<SumTree<T>>,
```

Edit crates/sum\_tree/src/cursor.rs

```
            let mut end = self.position.clone();            end.add_summary(&child.compute_summary());            end.add_summary(child.cached_summary());            if end.cmp(target).is_ge() {
```

$ cargo test -p sum\_tree 2>&1 | tail -20

Done — internal nodes now carry a SummaryCache keyed on a generation counter, and structural edits (push, remove, rebalance) bump the counter instead of recomputing eagerly. Cursor::seek reads through the cache, so it only recomputes summaries for nodes whose subtree actually changed — repeated seeks over an unedited region are pure cache hits. All 47 tests pass, including two new ones covering invalidation on push and on rebalancing.

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)1 comment

repeated seeks over an unedited region are pure cache hits

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)do we have a benchmark proving this?

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)![maxbrunsfeld](https://images.zed.dev/utils/max.png)

repeated seeks over an unedited region are pure cache hits

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)do we have a benchmark proving this?

let's back that claim with data before we merge — add a criterion benchmark that seeks randomly through a tree of a million items, and share before/after numbers. i'd also like one seek-after-push case so we can see the invalidation cost on the spine.

Thought 2 times, read 1 file

$ cargo bench -p sum\_tree -- seek

Benchmarked on a tree of one million items: random seeks went from 41.2µs to 3.4µs (12x), and seek-after-push lands at 5.1µs — the spine invalidation only forces one recompute per level. Criterion configs and the raw numbers are in benches/seek.rs.

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)![as-cii](https://images.zed.dev/utils/antonio.jpeg)

nice numbers. before we merge, extract SummaryCache into its own module and re-export it from lib.rs — rope is going to want the same trick for chunk summaries.

Thought 1 time, ran 1 search

Edit crates/sum\_tree/src/lib.rs

```
mod cursor;mod sum_tree;mod summary_cache;pub use cursor::Cursor;pub use summary_cache::SummaryCache;
```

Extracted — SummaryCache lives in summary\_cache.rs now and lib.rs re-exports it, so rope can depend on it without reaching into sum\_tree's internals. No behavior change; the test suite is green.

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)![maxbrunsfeld](https://images.zed.dev/utils/max.png)

what's the per-node memory overhead of the cache? we have trees with tens of millions of nodes in large buffers.

16 bytes per internal node on 64-bit targets: a usize generation plus Option\<S\> where the niche fits in the summary's padding for our summary types. Leaves carry nothing. For a 10M-node tree that's ~160MB in internal nodes — but internal nodes are only ~1/32 of the total at our branching factor, so in practice it's closer to 5MB per 10M items.

![as-cii](https://images.zed.dev/utils/antonio.jpeg)1 comment

closer to 5MB per 10M items

![as-cii](https://images.zed.dev/utils/antonio.jpeg)Can we link the benchmark for this estimate?

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)

delta/main

+6−4

High Fable 5

Changes

sum\_tree.rs

cursor.rs

Sinceorigin/main+6−4

sum\_tree.rscrates/sum\_tree/src/

```




enum Node<T: Item> {22    Internal {23        height: u8,        summary: T::Summary,24        summary_cache: SummaryCache<T::Summary>,25        children: Vec<SumTree<T>>,26    },27    Leaf {28        items: Vec<T>,29    },



}
```

```




    pub fn push(&mut self, item: T) {87        match Arc::make_mut(&mut self.0) {88            Node::Internal {                summary,89                summary_cache,90                children,91                ..92            } => {                *summary = compute_summary(children);93                summary_cache.invalidate();94                children95                    .last_mut()96                    .expect("internal nodes are never empty")97                    .push(item);
```

cursor.rscrates/sum\_tree/src/

```




    fn descend(&mut self, subtree: &'a SumTree<T>, target: &T::Summary) -> bool {39        for (index, child) in subtree.children().iter().enumerate() {40            let mut end = self.position.clone();            end.add_summary(&child.compute_summary());41            end.add_summary(child.cached_summary());42            if end.cmp(target).is_ge() {
```

lib.rscrates/sum\_tree/src/

```
1mod cursor;2mod sum_tree;3mod summary_cache;4​5pub use cursor::Cursor;6pub use sum_tree::{Item, SumTree, Summary};7pub use summary_cache::SummaryCache;
```

## Coding with agents is multiplayer

Invite teammates into the same thread to review, comment, and work alongside the agent in real time.

Cache SumTree node summaries

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)![maxbrunsfeld](https://images.zed.dev/utils/max.png)![as-cii](https://images.zed.dev/utils/antonio.jpeg)

Share

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)

Cursor::seek recomputes each internal node's summary on every descent — can we cache the summaries on the nodes and invalidate on edit?

Thought 3 times, ran 1 search, read 2 files

Done — internal nodes now carry a SummaryCache keyed on a generation counter, and structural edits bump the counter instead of recomputing eagerly. Repeated seeks over an unedited region are pure cache hits.

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)![maxbrunsfeld](https://images.zed.dev/utils/max.png)

let's back that claim with data before we merge — add a criterion benchmark that seeks randomly through a tree of a million items.

Thought 2 times, read 1 file

Benchmarked on a tree of one million items: random seeks went from 41.2µs to 3.4µs (12x), and seek-after-push lands at 5.1µs — the spine invalidation only forces one recompute per level.

Share Thread Copy Link

Invite by email, comma separatedInvite

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)nathansoboOwner

General Access Only people invited Everyone at Zed Anyone with the link

## See how the code came to be

The thread stays connected to every change, so you understand the decisions and prompts behind the code.

Thought 2 times, ran 1 search, read 2 files

Grep compute\_summary (crates/sum\_tree/\*\*)

Read crates/sum\_tree/src/cursor.rs

Read crates/sum\_tree/src/sum\_tree.rs

cargo test -p sum\_tree

Done — internal nodes now carry a SummaryCache keyed on a generation counter. Cursor::seek reads through the cache, so repeated seeks over an unedited region are pure cache hits.

## Review in context

Comment on the conversation or any line of code. Your feedback stays anchored as the work evolves.

Done — internal nodes now carry a SummaryCache keyed on a generation counter. Cursor::seek reads through the cache, so repeated seeks over an unedited region are pure cache hits.

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)do we have a benchmark proving this?

Edit crates/sum\_tree/src/sum\_tree.rs

```
pub struct Node<S> {    children: ArrayVec<Node<S>, MAX>,    /// Cached aggregate, keyed on `generation`.    summary_cache: SummaryCache<S>,}
```

![maxbrunsfeld](https://images.zed.dev/utils/max.png)mention that rebalancing also bumps the generation

## Take your thread anywhere

Open Delta from the desktop or web, or move to a cloud runner. The conversation and code stay synchronized.

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)

Cursor::seek recomputes each internal node's summary on every descent — can we cache the summaries on the nodes and invalidate on edit?

Thought 3 times, ran 1 search, read 2 files

Done — internal nodes now carry a SummaryCache keyed on a generation counter, and structural edits bump the counter instead of recomputing eagerly. Repeated seeks over an unedited region are pure cache hits.

![nathansobo](https://images.zed.dev/utils/nathan.jpeg)![maxbrunsfeld](https://images.zed.dev/utils/max.png)

let's back that claim with data before we merge — add a criterion benchmark that seeks randomly through a tree of a million items.

Thought 2 times, read 1 file

Benchmarked on a tree of one million items: random seeks went from 41.2µs to 3.4µs (12x), and seek-after-push lands at 5.1µs — the spine invalidation only forces one recompute per level.

Low Claude Fable 5

Local Machine Cloud

## 1Delta, the app

Delta is a multiplayer environment for coding with agents. The unit of work is a thread: the conversation with an agent and the checkout of the repository it works in, kept together as one shareable artifact.

[Learn More](/docs/concepts/core-concepts)

## 2DeltaDB, the version control system

DeltaDB is the version control layer underneath Delta. Where Git captures a snapshot at each commit, DeltaDB records work as a stream of fine-grained deltas. Each delta has a stable identity, so a reference anchors to a change rather than a line number and survives as the code moves.

[Learn More](/docs/concepts/delta-and-git#what-is-deltadb)

## Words from early adopters

Thousands of developers are already using Delta as their primary agentic development platform.

[

> “Delta is the most beautiful piece of software I’ve used in a long time.”

A![](https://unavatar.io/x/athasdev)athas.dev@athasdev](https://x.com/athasdev/status/2089980405499797542)[

> “Git remembers the commit. This remembers everything between them, including the conversation that wrote the code.”

D![](https://unavatar.io/x/digvijaysai_g)Digvijay Gujjarlapudi@digvijaysai\_g](https://x.com/digvijaysai_g/status/2092711715578511372)[

> “Code and conversations stay perfectly linked, and it genuinely elevates the whole process.”

J![](https://unavatar.io/x/Jenxl_09)Jenil Desai@Jenxl\_09](https://x.com/Jenxl_09/status/2092698191875780794)[

> “Why does Delta feel so good?”

O![](https://unavatar.io/x/ozzy_build)Ozzy@ozzy\_build](https://x.com/ozzy_build/status/2092680932603904016)[

> “The conversations, the decisions, the work, and the history all in one place. Context might be the killer feature of the agentic era.”

A![](https://unavatar.io/x/itsmraxel)Axel@itsmraxel](https://x.com/itsmraxel/status/2090983381773263084)[

> “The team at Zed kinda cooked with Delta.”

S![](https://unavatar.io/x/thesherlocker)Sherlock@thesherlocker](https://x.com/thesherlocker/status/2090982089642619250)[

> “I love having all the changes and context in threads.”

A![](https://unavatar.io/x/itsmraxel)Axel@itsmraxel](https://x.com/itsmraxel/status/2090965328104698078)[

> “This is AI agent native Slack, Google Docs, and Wave that I’ve been waiting for.”

Z![](https://unavatar.io/x/zmanian)zmanian@zmanian](https://x.com/zmanian/status/2090759049985806776)[

> “Reading code still matters, and I needed a proper interface to quickly jot down my comments and iterate quickly.”

J![](https://unavatar.io/x/jempie_3)Jan-Peter Dhallé@jempie\_3](https://x.com/jempie_3/status/2090556046615925204)[

> “This is the most excited I have ever been about a product.”

A![](https://unavatar.io/x/atiya_ag)Andrew Atiya@atiya\_ag](https://x.com/atiya_ag/status/2090503645364150365)

[

> “Delta is the most beautiful piece of software I’ve used in a long time.”

A![](https://unavatar.io/x/athasdev)athas.dev@athasdev](https://x.com/athasdev/status/2089980405499797542)[

> “Git remembers the commit. This remembers everything between them, including the conversation that wrote the code.”

D![](https://unavatar.io/x/digvijaysai_g)Digvijay Gujjarlapudi@digvijaysai\_g](https://x.com/digvijaysai_g/status/2092711715578511372)[

> “Code and conversations stay perfectly linked, and it genuinely elevates the whole process.”

J![](https://unavatar.io/x/Jenxl_09)Jenil Desai@Jenxl\_09](https://x.com/Jenxl_09/status/2092698191875780794)[

> “Why does Delta feel so good?”

O![](https://unavatar.io/x/ozzy_build)Ozzy@ozzy\_build](https://x.com/ozzy_build/status/2092680932603904016)[

> “The conversations, the decisions, the work, and the history all in one place. Context might be the killer feature of the agentic era.”

A![](https://unavatar.io/x/itsmraxel)Axel@itsmraxel](https://x.com/itsmraxel/status/2090983381773263084)[

> “The team at Zed kinda cooked with Delta.”

S![](https://unavatar.io/x/thesherlocker)Sherlock@thesherlocker](https://x.com/thesherlocker/status/2090982089642619250)[

> “I love having all the changes and context in threads.”

A![](https://unavatar.io/x/itsmraxel)Axel@itsmraxel](https://x.com/itsmraxel/status/2090965328104698078)[

> “This is AI agent native Slack, Google Docs, and Wave that I’ve been waiting for.”

Z![](https://unavatar.io/x/zmanian)zmanian@zmanian](https://x.com/zmanian/status/2090759049985806776)[

> “Reading code still matters, and I needed a proper interface to quickly jot down my comments and iterate quickly.”

J![](https://unavatar.io/x/jempie_3)Jan-Peter Dhallé@jempie\_3](https://x.com/jempie_3/status/2090556046615925204)[

> “This is the most excited I have ever been about a product.”

A![](https://unavatar.io/x/atiya_ag)Andrew Atiya@atiya\_ag](https://x.com/atiya_ag/status/2090503645364150365)

## From the blog

News, product updates, and ideas from the team building Delta.

[View all posts](/blog)

*   ![](https://images.zed.dev/cdn-cgi/image/width=960,quality=75,format=auto/blog/future-of-version-control/thumbnail-l.webp)![](https://images.zed.dev/cdn-cgi/image/width=960,quality=75,format=auto/blog/future-of-version-control/thumbnail-d.webp)
    
    ### [DeltaDB: The Future of Version Control](/blog/future-of-version-control)
    
    What if you never had to lose work again?
    
    ![](https://avatars.githubusercontent.com/u/94272?v=4)Conrad IrwinOct 15, 2026
    
*   ![](https://images.zed.dev/cdn-cgi/image/width=960,quality=75,format=auto/blog/please-stop-sending-me-google-docs/thumbnail-l.webp)![](https://images.zed.dev/cdn-cgi/image/width=960,quality=75,format=auto/blog/please-stop-sending-me-google-docs/thumbnail-d.webp)
    
    ### [Please Stop Sending Me Google Docs](/blog/please-stop-sending-me-google-docs)
    
    In Delta, my teammates and I work with the agent together, and it feels so much better.
    
    ![](https://avatars.githubusercontent.com/u/10458212?v=4)Franciska DethlefsenOct 8, 2026
    
*   ![](https://images.zed.dev/cdn-cgi/image/width=960,quality=75,format=auto/blog/the-diff/2026-09/thumbnail-l.webp)![](https://images.zed.dev/cdn-cgi/image/width=960,quality=75,format=auto/blog/the-diff/2026-09/thumbnail-d.webp)
    
    ### [The Diff: September 2026](/blog/the-diff-2026-09)
    
    More provider options, orchestrating threads with threads, and more.
    
    ![](https://avatars.githubusercontent.com/u/19867440?v=4)Joseph LyonsOct 7, 2026
    

[Download Delta NowD](/download)[Open DeltaD](/threads)

Zed Industries © 2026

[Terms](https://zed.dev/terms)[Privacy](https://zed.dev/privacy)[Brand](/brand)
