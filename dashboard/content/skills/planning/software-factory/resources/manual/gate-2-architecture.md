+++
title = "gate-2-architecture"
[extra]
skill = false
category = "planning"
mermaid = false
skill_name = "software-factory"
+++

{% raw %}
# Gate 2 - Architecture

Saved as `docs/plans/<feature-slug>/02-architecture.md`. Written after reading the code this feature touches, so every line describes the real system.

```markdown
# Architecture: <feature name>

## Fit

<which existing services, modules, or packages this touches, and how it joins them>

## Endpoints

<route + verb + purpose, one line each - or "none">

## Data

<new or changed tables, collections, or files, with an outline of the queries that will
hit them - enough to see whether the shape supports the access pattern>

## Flow

<the end-to-end call order for the main path: what calls what, in order>

## External

<third-party APIs, webhooks, and env var NAMES - never values - or "none">

## Rejected alternatives

<the designs considered and dropped, one line each on why>
```

Where a decision here will outlive this feature, record it as an ADR with `/domain-modeling` rather than burying it in this doc.

{% endraw %}
