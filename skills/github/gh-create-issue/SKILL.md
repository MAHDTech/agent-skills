---
name: gh-create-issue
description: Create a structured GitHub issue from the current conversation with gh CLI. Use when asked to file an issue that preserves the problem, discussion, constraints, and agreed implementation plan.
disable-model-invocation: true
resources:
  - https://cli.github.com/manual/gh_issue
metadata:
  group: github
---

# Create GitHub Issue from Conversation

**Invocation:** Start only when the user explicitly requests this workflow.

Consult the `gh issue` manual in `resources/auto/manual-gh_issue.md` for more details.

Create a well-structured GitHub issue using `gh` CLI, populated with context from the current conversation.

## Instructions

1. **Determine the repo** -- confirm the working directory is a git repo with a GitHub remote:

   ```bash
   gh repo view --json nameWithOwner -q '.nameWithOwner'
   ```

   If this fails, ask the user which repo to file against.

2. **Synthesize the conversation** into a GitHub issue body following the structure below.

3. **Scan before posting.** Scan the full outgoing payload (title, body, quoted code, logs, and links) for secrets and private details, and redact or ask the user before continuing. Follow `/gh-pr-comments` for the full checklist when it is installed.

4. **Create the issue** from a body file outside the working tree, then remove it:

   ```bash
   body="$(mktemp)"
   # write the issue body to "$body"
   gh issue create --title "<concise title>" --body-file "$body"
   rm -f "$body"
   ```

5. **Report back** with the issue URL.

## Issue Body Structure

Build the issue body in this exact order:

### Section 1: What & Why

Open with a short (2-4 bullet) summary answering:

- **What** needs to happen?
- **Why** does it matter? (user pain, system constraint, business goal)

This section should stand alone -- a reader who stops here should understand the ask.

### Section 2: Context & Discussion

This is the bulk of the issue. Capture everything relevant from the conversation:

- **Background** -- what led to this discussion
- **Current behavior** -- how things work today (or don't)
- **Desired behavior** -- what success looks like
- **Constraints & decisions** -- trade-offs discussed, options rejected and why
- **Technical details** -- relevant code paths, configs, schemas, API shapes, error messages
- **Related work** -- links to PRs, issues, docs, or external references mentioned

**Formatting rules for this section:**

- Use `## Headings` and `### Subheadings` to organize by topic
- Use bullet points over paragraphs
- Use markdown tables for comparisons, option matrices, or structured data
- Embed images/screenshots if any were shared (use `![alt](url)` syntax)
- Wrap code snippets, commands, file paths, and config in fenced code blocks with language tags
- Use `<details><summary>...</summary>...</details>` to collapse verbose content (logs, full configs, large code blocks) so the issue stays scannable
- Preserve the substance of the discussion, reorganized by topic

### Section 3: Implementation Plan

End with the agreed plan (even if rough). Frame it for a future agent or developer picking this up cold:

- Number the steps
- Call out key files to create or modify
- Note any open questions or decisions still TBD (prefix with `[ ]`)
- If there are dependencies between steps, make the order explicit
- If no plan was agreed, write `## Implementation Plan\n\n_TBD -- no plan agreed on yet._`

## Labels & Assignment (optional)

- If the conversation implies a category (bug, feature, enhancement, tech-debt), add `--label` flags
- Do NOT assign unless the user explicitly asks

## Style Rules

- **Title**: imperative mood, under 70 chars (e.g., "Add retry logic to webhook delivery")
- **No fluff**: skip filler phrases like "It would be great if..." -- be direct
- **Bias to structure**: headers > paragraphs, bullets > prose, tables > lists-of-pairs
- **Preserve signal**: don't summarize away details that a future implementer would need
- **Mark uncertainty**: if something was discussed but not decided, say so explicitly with "TBD" or an open checkbox `[ ]`
