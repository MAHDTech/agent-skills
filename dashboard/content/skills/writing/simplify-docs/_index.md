+++
title = "simplify-docs"
description = "Simplify a README, setup guide, or technical reference for fast scanning and execution. Use for structural documentation rewrites that lead with the shortest successful path, previewing edits for approval."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "writing"
mermaid = false
+++


# Simplify Docs

Rewrite documentation so it is simple, fast to scan, and easy to execute.

## Primary goal

- Bias toward simple docs.
- Start with the shortest successful path.
- Preserve original meaning and factual content while reordering for clarity.

## Section order

Use only the sections that add value for the specific document, in this order:

1. Quickstart (always first)
2. Main workflow
3. Configuration and reference lists (when relevant)
4. Alternative paths (only when they are true user or code path alternatives)
5. Code structure (only when repository layout helps task completion)
6. Troubleshooting (only for real recurring issues)
7. Extra details at the end inside `<details><summary>...</summary>...</details>`

## Approval gate

- Before editing any file, show a change preview for approval.
- Use this exact preview structure for each file:
  - Additions
  - Removals
  - Changes
- Show the concrete text to be added, removed, or changed.
- Wait for explicit user approval before applying edits.
- Apply only the approved edits.
- If approval is not granted, leave the files unchanged.

## Core rules

- Optimize for both human readers and agent readers.
- Maximize copy-paste command snippets.
- Use lists for structured information.
- Use Mermaid diagrams when they improve understanding of flow or architecture.
- Label a section "Optional" only when it is an actual alternate path.
- Put background material at the end.

## Command block rules

- Do not place comments inside bash code blocks.
- Put labels outside code blocks.

Correct pattern:

Start the server:

```bash
make start_server
```

Run database migrations:

```bash
make migrate_db
```

Incorrect pattern (separate steps combined in one unlabeled block):

```bash
make start_server
make migrate_db
```

## Configuration list format

If environment variables or configuration values are required, use this format:

- **`APP_ENV`**: Runtime environment (Default: `development`)
- **`API_URL`**: Base URL for API calls (Default: `http://localhost:8000`)

## Details block rule

Move long explanations, implementation notes, and low-priority reference material into details blocks:

```markdown
<details>
<summary>Deep dive: request lifecycle</summary>

Detailed explanation here.

</details>
```

## Deliverable

- Return the final output as Markdown (`.md`).
- Keep all important content, but simplify, reorder, and prioritize for quick execution.

