# Out of Scope

This directory is a decision log of things deliberately left out of the project. It records requests that were considered and rejected, so the same idea does not get re-litigated from scratch every time it comes up.

## The pattern

- One Markdown file per rejected request or theme.
- Each file explains **why this is out of scope** with a clear rationale, not just a "no".
- Each file keeps a **Prior requests** list linking to the issues, pull requests, or discussions where the idea was raised.

When someone proposes something that has already been declined, point them here instead of re-arguing it. If the reasoning changes, update the relevant file (or delete it) rather than silently reversing course.

## File template

```markdown
# <Short title of the rejected request>

## Why this is out of scope

<A few sentences explaining the reasoning: what was asked, and why it does
not belong in this project. Name the trade-off and any alternative.>

## Prior requests

- <link to issue / PR / discussion> — short note on what was asked
```

## Adding an entry

1. Copy the template above into a new file, e.g. `.out-of-scope/my-topic.md`.
2. Fill in the rationale and link the request that prompted it.
3. When the topic resurfaces, append the new request to the **Prior requests** list.

See [example.md](./example.md) for an illustrative entry.
