# Triage templates

Templates for the plan and summaries that `gh-resolve-pr-comments` shows the user.

## Triage plan (step 5)

Omit any category with a count of zero.

```markdown
## PR Comment Triage: {pr_url}

### Fix ({count})

- [ ] **{file}:{line}** - {summary of minimal change}
  > {abbreviated reviewer or bot comment}

### Decline ({count})

- [ ] **{file}:{line}** - {file:line or behavior reason to push back; note as follow-up if valid out of scope}
  > {abbreviated reviewer or bot comment}

### Investigate ({count})

- [ ] **{file}:{line}** - {what to explore and why}
  > {abbreviated reviewer or bot comment}

### Discuss ({count})

- [ ] **{file}:{line}** - {question or design decision}
  > {abbreviated reviewer or bot comment}

### Acknowledge ({count})

- {comment summary} - will resolve, no code change needed

### Outdated ({count})

- {comment summary} - code has changed, will resolve
```

## Execution summary (step 6)

```markdown
## Changes Made

| File | Line | Category    | Action Taken                               |
| ---- | ---- | ----------- | ------------------------------------------ |
| …    | …    | Fix         | Deleted redundant branch and updated check |
| …    | …    | Decline     | Replied with file:line rationale           |
| …    | …    | Investigate | Verified caller invariant; replied         |
| …    | …    | Discuss     | User decided to defer                      |
```

## Final status (step 9)

```markdown
## Done

- {N} comments addressed ({declined_count} declined with rationale)
- {M} threads resolved on GitHub after confirmed replies
- Commit: {short_sha}
- PR: {pr_url}
```
