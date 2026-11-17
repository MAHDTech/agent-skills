+++
title = "gh-release-notes"
description = "Curate release notes from commits and merged PRs since the last tag, then create a GitHub release. Use when asked to cut or publish a versioned release, with draft and prerelease options."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "github"
mermaid = false
+++


# Generate Release Notes and Cut a Release

**Invocation:** Start only when the user explicitly requests this workflow.

Turn the history merged since the last release into notes a human actually wants to read, then publish them as a GitHub release. The pairing skill for shipping is `/gh-create-pr` - this picks up after those PRs have merged.

## 1. Establish the range

Fetch tags, then find the previous release to diff against:

```bash
git fetch origin --tags --prune
PREV_TAG=$(git describe --tags --abbrev=0 2>/dev/null)
```

The range is `$PREV_TAG..HEAD`. If no tag exists yet (first release), the range is the entire history - use `git log HEAD` and note it is the initial release. Decide the new version with the user if they have not given one, and set it:

```bash
VERSION=v1.4.0
```

## 2. Gather the changes

Collect from both the commit log and the merged PRs - PR titles and labels are usually more human than raw commit subjects:

```bash
git log "$PREV_TAG..HEAD" --no-merges --pretty='%s (%h) @%an'
```

```bash
BASE=$(gh repo view --json defaultBranchRef -q '.defaultBranchRef.name')   # default branch; do not assume main
git log -1 --format=%as "$PREV_TAG"   # date of the previous tag
gh pr list --state merged --base "$BASE" --search "merged:>=<that-date>" \
  --json number,title,labels,author,mergedAt
```

## 3. Group by type

Sort every change into sections, in this order. Determine each entry's type from whatever signal is present - conventional-commit prefix, PR label, or, failing both, the PR title and its diff:

| Section          | Conventional prefix                                | PR label                 | Fallback signal                                 |
| ---------------- | -------------------------------------------------- | ------------------------ | ----------------------------------------------- |
| Breaking changes | `!` suffix or `BREAKING CHANGE:` footer            | `breaking`               | Removes/renames a public API, changes a default |
| Features         | `feat`                                             | `feature`, `enhancement` | Adds new user-visible capability                |
| Fixes            | `fix`                                              | `bug`, `bugfix`          | Corrects broken behaviour                       |
| Performance      | `perf`                                             | `performance`            | Speed/memory improvement                        |
| Housekeeping     | `chore`, `ci`, `build`, `refactor`, `docs`, `test` | `chore`, `dependencies`  | Internal-only; fold or drop                     |

Conventional prefixes make this exact, but never require them - infer the section when the history is free-form. Omit or collapse pure noise (version bumps, lockfile updates, CI tweaks) into a short Housekeeping line rather than listing each one.

## 4. Write the notes

Rewrite each entry as a user-facing line, not a commit dump:

- Say what changed for the reader, not which files moved. Start with a verb: "Added…", "Fixed…", "Renamed…".
- Link the PR by number (`#123`) so GitHub auto-links it, and credit the author.
- Lead the document with **Breaking changes** and any required migration steps - those are the lines readers cannot skip.
- Drop entries a reader would not care about; a curated 12-line changelog beats an exhaustive 200-line one.

Write the result to a temporary file outside the working tree, so it is never swept into a commit:

```bash
NOTES_FILE=$(mktemp --suffix=.md)
```

To start from GitHub's own summary, fetch its generated notes without creating a release, then edit them into the grouped form above:

```bash
gh api "repos/{owner}/{repo}/releases/generate-notes" \
  -f tag_name="$VERSION" -f previous_tag_name="$PREV_TAG" --jq .body > "$NOTES_FILE"
```

Omit `-f previous_tag_name` for the first release.

## 5. Publish the release

Create the release once, using `$VERSION` from step 1:

```bash
gh release create "$VERSION" --title "$VERSION" --notes-file "$NOTES_FILE"
```

`gh release create` creates the tag on the current commit (or pass `--target <sha>`). Add `--draft` to review on GitHub before it goes live, or `--prerelease` for an RC. To publish a draft later, run `gh release edit "$VERSION" --draft=false`. Confirm the release URL it prints, and that the tag points at the intended commit, then delete `$NOTES_FILE`.

Done when the release is published (or drafted, if requested) with grouped, human-readable notes and the correct tag.

