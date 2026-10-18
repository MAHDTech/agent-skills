+++
title = "pricing"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: "Oak is free while it&#39;s in beta: 100 GB of storage per organization, unlimited repos, branches, and members, and CI on your own runners. Large-file support, lazy mounts, oak export to plain git. Bring your own agent."
title: Pricing — Oak
image: https://oak.space/social-card-217ab603.png
---

100 GB storage, per organization

- Unlimited repositories, public or private
- Unlimited open branches — branch per session as hard as you like
- Unlimited members in an organization
- CI on your own machines, with GitHub Actions workflows
- Large files, lazy mounts, and `oak export` to plain git

[Get started](https://oak.space/signup)

## Will it stay free?

Not forever — Oak costs real money to run, and paid plans will come back. When they do we’ll say so well ahead of time, and nothing you’ve pushed becomes inaccessible. `oak export \<dest\>` replays your full history into a standard git repo at any time, so there’s no lock-in to worry about either way.

## What will it cost when billing returns?

If you need a number to take to a budget meeting: **plan for tens of dollars per user per month**, not hundreds and not single digits. Concretely, what we intend to charge is **$10 per month** for an individual and **$30 per seat per month** for a team — the same order of magnitude as a seat of GitHub Enterprise or GitLab Premium, and roughly a rounding error next to what the same seat spends on agent tokens.

A team seat is meant to include the storage and compute a team actually uses rather than metering you into a surprise: 100 GB of storage and 100 GB of egress per seat per month, pooled across the org. Pushing is free and always will be — we don’t charge you to hand us your data, no matter how many agents are committing.

These are our current intentions, not a contract: they may move before billing switches on, and there is nothing to buy today. What we will commit to is notice well ahead of any charge, and no data held hostage if the number doesn’t work for you. [Email me](mailto:zach@oak.space) if you need something firmer to evaluate a migration — a written quote for a team of a given size is a normal thing to ask for and we’d rather give you one than have you guess.

## What if I need more than 100 GB?

Ask. [Email me](mailto:zach@oak.space) and we’ll raise your limit — model weights and big binary assets are exactly the workload Oak is built for, and we’d rather host them than have you work around us.

## Does Oak use my code for AI?

No. Oak doesn’t train on your code and makes no AI calls on your behalf. Bring any coding agent — [Claude Code](https://claude.ai/code), Codex, Cursor — Oak is just the VCS they push to.
