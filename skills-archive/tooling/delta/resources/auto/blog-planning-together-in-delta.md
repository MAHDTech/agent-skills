---
title: "Your plan.md is missing the conversation — Delta Blog"
url: "https://delta.dev/blog/planning-together-in-delta"
description: "How we plan with agents before generating a line of code."
---

Blog

# Your plan.md is missing the conversation

![](https://avatars.githubusercontent.com/u/1789?v=4)Nathan SoboOctober 1, 2026

Agents can blast out a multi-thousand-line diff in seconds, but they’ll happily build the wrong thing just as efficiently if you’re not clear about what you’re building.

Before Delta, our planning happened in silos. Some made its way into a `plan.md`, while the rest stayed in someone’s head until they tried to recreate it in a prompt. The result was only a snapshot that didn’t have all the back-and-forth that got us there. At best, that slowed us down; at worst, we forgot why we’d made a decision and ended up going in circles or building the wrong thing.

Now all our planning starts in Delta.

In this example, we’re in the late planning phases on a new Delta repository that indexes the contributions of all your threads against the evolving code. When I logged on in the morning, Antonio had already been working for hours from Italy. Cole and I joined him in the same thread to catch up and work through the plan together.

![Nathan, Antonio, and Cole work through the plan together in a shared Delta thread.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/planning-in-delta/initial-interaction-l.webp)![Nathan, Antonio, and Cole work through the plan together in a shared Delta thread.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/planning-in-delta/initial-interaction-d.webp)

Nathan, Antonio, and Cole work through the plan together in a shared Delta thread.

We still didn’t have a diff. We don’t always plan this extensively, but when working on more complex problems, we tend to work in “token space” for quite a while before we commit to generating code.

Antonio had Fable generate a summary of the decisions made during his morning in Italy, and then the three of us left [comments](https://delta.dev/docs/agents/comments) directly on the parts of the plan we wanted to question.

The conversation became a group edit. Cole asked, “What’s a two-backed ID?” Fable admitted the term was jargon it had invented.

Antonio corrected Fable’s explanation of how the index root worked; I followed with a question about choosing a deeper root, and Fable revised its recap. Later, Antonio questioned another name Fable had introduced.

The three of us clarified the concept together, and Fable folded it back into the plan. By the end, all four of us understood what we were building and why.

I continued the discussion while Cole started an [isolated subthread](https://delta.dev/docs/agents/threads#send-a-message-to-a-subthread) to explore the data structures. People and agents can both start subthreads with context from the parent. We can enter one to work with its agent directly, then bring what it discovers back to the parent thread.

![Cole explores the data structures in an isolated subthread alongside the original planning conversation.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/planning-in-delta/coles-subthread-l.webp)![Cole explores the data structures in an isolated subthread alongside the original planning conversation.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/planning-in-delta/coles-subthread-d.webp)

Cole explores the data structures in an isolated subthread alongside the original planning conversation.

## [From planning to implementation](#from-planning-to-implementation)

Our 60 prompts in token space eventually turned into a substantial diff. By the time Antonio returned the next morning, Cole had built and reviewed the first pieces of the new index, then left the remaining design question in a comment for him. Nobody had to re-read blocks of context to catch up.

While we collaborated, a subagent worked in an [isolated worktree](https://delta.dev/docs/concepts/worktrees) so it could implement its part without disrupting the work in the parent thread.

Antonio asked Astra to catch him up on everything Cole had done since he left. Astra summarized the progress, confirmed what had already been committed, and brought the unresolved question to the top. Antonio could respond in place, then continue implementing from the plan we had already developed together.

![Antonio returns to the thread, asks Astra for a recap, and finds the remaining design question waiting for him.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/planning-in-delta/antonio-catch-up.webp-l.webp)![Antonio returns to the thread, asks Astra for a recap, and finds the remaining design question waiting for him.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/planning-in-delta/antonio-catch-up.webp-d.webp)

Antonio returns to the thread, asks Astra for a recap, and finds the remaining design question waiting for him.

There was no handoff. Antonio didn’t have to piece together the previous day’s discussion from a `plan.md` or reconstruct the intent behind Cole’s changes from a diff. He reopened the same thread. The plan, the code, the comments, and the agents that worked on them were all there, ready to continue the implementation.

![Cole continues working in the parent thread while the agent delegates the next implementation block to a subagent.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/planning-in-delta/implementation-subthread-l.webp)![Cole continues working in the parent thread while the agent delegates the next implementation block to a subagent.](https://images.zed.dev/cdn-cgi/image/width=5760,quality=75,format=auto/blog/planning-in-delta/implementation-subthread-d.webp)

Cole continues working in the parent thread while the agent delegates the next implementation block to a subagent.

A plan document captures decisions, and a commit captures changes, but neither preserves the whole conversation that shaped the work.

In Delta, the thread is the unit of work. Planning, implementation, and [review](https://delta.dev/docs/agents/review-and-sync) unfold in the same living context, where people and agents can continue without reconstructing what everyone already knew.

On this page

1.  [From planning to implementation](#from-planning-to-implementation)

## More from the blog

*   ### [Please Stop Sending Me Google Docs](/blog/please-stop-sending-me-google-docs)
    
    ![Franciska Dethlefsen](https://avatars.githubusercontent.com/u/10458212?v=4 "Franciska Dethlefsen")Oct 8, 2026
    
*   ### [Introducing Delta](https://zed.dev/blog/introducing-delta)
    
    Featured![Nathan Sobo](https://avatars.githubusercontent.com/u/1789?v=4 "Nathan Sobo")Aug 12, 2026
    
*   ### [The Diff: September 2026](/blog/the-diff-2026-09)
    
    ![Joseph Lyons](https://avatars.githubusercontent.com/u/19867440?v=4 "Joseph Lyons")Oct 7, 2026
    

[Download Delta NowD](/download)[Open DeltaD](/threads)

Zed Industries © 2026

[Terms](https://zed.dev/terms)[Privacy](https://zed.dev/privacy)[Brand](/brand)