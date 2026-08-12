+++
title = "gate-1-product"
[extra]
skill = false
category = "planning"
mermaid = false
skill_name = "software-factory"
+++

{% raw %}
# Gate 1 — Product

Saved as `docs/plans/<feature-slug>/01-product.md`. Written in the end user's language throughout; anything technical belongs to Gate 2.

```markdown
# Product: <feature name>

## Problem

<the problem the user has today, in the user's words — what they cannot do, or what
costs them time, and what they do instead right now>

## Success metric

<one real number tied to the business — conversion, latency, support tickets, revenue —
and how it will be measured after this ships>

## Announcement

<3–6 sentences announcing this feature to the people who will use it. Being unable to
write this is the signal that the wrong thing is being built.>

## Screens

<one line per mockup file in ./mockups/, saying what the screen is for — or "no UI">

## Out of scope

<what this feature deliberately does not do>
```

## Mockups

One plain HTML file per screen in `mockups/`, no framework and no build step. They exist to make the user point at something and say "yes, that" — they are thrown away once the real screens exist, so hardcode the content and skip the polish.

{% endraw %}
