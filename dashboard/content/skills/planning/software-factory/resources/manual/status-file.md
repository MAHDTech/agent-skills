+++
title = "status-file"
[extra]
skill = false
category = "planning"
mermaid = false
skill_name = "software-factory"
+++

# Status file

Lives at `docs/plans/<feature-slug>/00-status.md`. Created before Gate 1, updated at every approval. It holds gate state and nothing else - slice progress lives on the issue tracker, and this file points at it.

```markdown
# Status: <feature name>

- Gate 1 - Product: pending | in progress | APPROVED <YYYY-MM-DD>
- Gate 2 - Architecture: pending | in progress | APPROVED <YYYY-MM-DD>
- Gate 3 - Program Design: pending | in progress | APPROVED <YYYY-MM-DD>
- Gate 4 - Slice plan: pending | in progress | APPROVED <YYYY-MM-DD>

## Slices

<where the tickets live: tracker query, label, milestone, or local path>

## Notes for a fresh session

<anything decided in conversation that the gate docs do not already carry>
```

The Notes section is the compaction target: at every gate and slice boundary, move anything that exists only in the chat into it, or into the gate doc it belongs to.

