+++
title = "gh-release-notes"
description = "Generate human-readable release notes for a version from the merged history and cut a GitHub release with the gh CLI — gather the commit and PR range since the last tag, group changes by type, write curated notes, and publish. Use when the user wants to cut a release, tag a version, write a changelog or release notes, or summarise what shipped since the last tag. Pairs well with conventional-commit history but does not require it."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "github"
mermaid = false
+++


# Generate Release Notes and Cut a Release

Turn the history merged since the last release into notes a human actually wants to read, then publish them as a GitHub release. The pairing skill for shipping is `/gh-create-pr` — this picks up after those PRs have merged.

## 1. Establish the range

Fetch tags, then find the previous release to diff against:

```bash
git fetch origin --tags --prune
PREV_TAG=$(git describe --tags --abbrev=0 2>/dev/null)
```

The range is `$PREV_TAG..HEAD`. If no tag exists yet (first release), the range is the entire history — use `git log HEAD` and note it is the initial release. Decide the new version string (e.g. `v1.4.0`) with the user if they have not given one.

## 2. Gather the changes

Collect from both the commit log and the merged PRs — PR titles and labels are usually more human than raw commit subjects:

```bash
git log "$PREV_TAG..HEAD" --no-merges --pretty='%s (%h) @%an'
```

```bash
BASE=$(gh repo view --json defaultBranchRef -q '.defaultBranchRef.name')   # default branch; do not assume main
git log -1 --format=%as "$PREV_TAG"   # date of the previous tag
gh pr list --state merged --base "$BASE" --search "merged:>=<that-date>" \
  --json number,title,labels,author,mergedAt
```

`$BASE` is the repo's default branch, derived above — never hard-code `main`.

## 3. Group by type

Sort every change into sections, in this order. Determine each entry's type from whatever signal is present — conventional-commit prefix, PR label, or, failing both, the PR title and its diff:

| Section          | Conventional prefix                                | PR label                 | Fallback signal                                 |
| ---------------- | -------------------------------------------------- | ------------------------ | ----------------------------------------------- |
| Breaking changes | `!` suffix or `BREAKING CHANGE:` footer            | `breaking`               | Removes/renames a public API, changes a default |
| Features         | `feat`                                             | `feature`, `enhancement` | Adds new user-visible capability                |
| Fixes            | `fix`                                              | `bug`, `bugfix`          | Corrects broken behaviour                       |
| Performance      | `perf`                                             | `performance`            | Speed/memory improvement                        |
| Housekeeping     | `chore`, `ci`, `build`, `refactor`, `docs`, `test` | `chore`, `dependencies`  | Internal-only; fold or drop                     |

Conventional prefixes make this exact, but never require them — infer the section when the history is free-form. Omit or collapse pure noise (version bumps, lockfile updates, CI tweaks) into a short Housekeeping line rather than listing each one.

## 4. Write the notes

Rewrite each entry as a user-facing line, not a commit dump:

- Say what changed for the reader, not which files moved. Start with a verb: "Added…", "Fixed…", "Renamed…".
- Link the PR by number (`#123`) so GitHub auto-links it, and credit the author.
- Lead the document with **Breaking changes** and any required migration steps — those are the lines readers cannot skip.
- Drop entries a reader would not care about; a curated 12-line changelog beats an exhaustive 200-line one.

Write the result to a file (e.g. `RELEASE_NOTES.md`) so you can publish it verbatim.

## 5. Publish the release

Set `$VERSION` to the new tag you decided on in step 1:

```bash
VERSION=v1.2.3
```

To scaffold a first draft from GitHub's own diff, then edit it into the grouped form above:

```bash
gh release create "$VERSION" --generate-notes --draft
```

To publish the curated notes you wrote:

```bash
gh release create "$VERSION" --title "$VERSION" --notes-file RELEASE_NOTES.md
```

`gh release create` creates the tag on the current commit (or pass `--target <sha>`). Add `--draft` to review on GitHub before it goes live, or `--prerelease` for an RC. Confirm the release URL it prints, and that the tag points at the intended commit.

Done when the release is published (or drafted, if requested) with grouped, human-readable notes and the correct tag.

