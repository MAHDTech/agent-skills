# Plan: Declarative vendor skills

Date: 2026-10-24. Branch: `feat/declarative-vendor-skills`.

Status: Implementation approved, including local tests, commit, push, and PR creation.

## Design

A YAML manifest declares vendor Git URLs, branches, and skill directory paths. A Rust module within `skills-core` retrieves complete skill folders using the existing Git executable. No Node, npm, or skill lockfile is required. Git records the exact catalog at every repository commit.

Import all skills, including future additions, under `skills/vendor/<vendor>/<skill>/`. Preserve upstream files and their support files, permissions, licenses, and notices. Display a Vendor Skills section grouped by source in the README, agent index, manifest, and dashboard. Give vendor files scoped exceptions from local authoring rules while retaining import validation and unique active skill names.

`ask skills download-resources` updates vendors, downloads custom-skill documentation, and regenerates catalog artifacts. A vendor-only mode supports automation. Ordinary installs, syncs, lint, builds, and tests consume the committed snapshots. Documentation cleanup must never modify vendor-owned files.

Prepare each source in temporary storage before replacement. Successful updates overwrite local edits and mirror upstream removals. Failed downloads retain that vendor's previous successful snapshot and allow other vendors to advance. This replaces the earlier all-or-nothing proposal. Reject a combined catalog with duplicate active names before replacing snapshots. Bound cleanup to managed vendor directories.

Run daily and on manual dispatch, maintaining one combined PR on a stable automation branch. Refresh an existing unmerged PR rather than creating duplicates. Preserve snapshots already pending in that PR when a later download fails, rather than reverting them to the older base-branch copy. Reconcile changed declarations and base-branch edits before reusing pending snapshots. Create no PR or needless commits when files do not change. Merge remains a review step.

The manifest and Git branch state are sufficient. No file-hash inventory, restore-from-lock command, timestamp, or generated provenance file is needed. Report skipped vendors and fetched upstream commits in command output and the PR description.

## Initial vendors

```yaml
version: 1
sources:
  - id: cloudflare
    repository: https://github.com/cloudflare/skills.git
    ref: main
    path: skills
  - id: spacetimedb
    repository: https://github.com/clockworklabs/SpacetimeDB.git
    ref: master
    path: skills
```

SpacetimeDB's `mcp` skill conflicts with the custom `mcp` skill. The naming resolution is pending user input; implement independent functionality while waiting.

## Implementation sequence

1. Add the Rust manifest and downloader module with fixture-repository tests for copying, replacement, failures, collisions, and pending PR retention.
2. Integrate vendor discovery, installation, lint boundaries, resource ownership, and generated catalogs.
3. Add normal and vendor-only CLI paths, documentation, and daily PR automation.
4. Resolve the initial naming collision, import both vendor collections, and download documentation resources.
5. Review scope, run configured checks and local tests, commit, push, and open a PR.

## Acceptance checks

- All initial vendor skills and support files can be acquired without Node or npm.
- Unchanged content creates no diff, including when unrelated upstream commits advance.
- A failed source retains its previous snapshot while another vendor updates successfully.
- Upstream additions and removals appear after successful refreshes; local edits are regenerated.
- Collisions report both origins and do not publish an ambiguous catalog.
- Vendor files remain unchanged by formatting hooks, documentation downloads, or cleanup.
- All generated catalogs and local installation discover the source-grouped vendor layout.
- An unmerged daily PR is updated in place; failed vendors retain successful snapshots already pending there.
- Tests use `devenv --no-tui test` or the `scripts/run-tests.sh` wrapper inside the development environment. Linters run through configured hooks.

See the [research](../research/declarative-vendor-skills.md) and [architecture decision](../adr/0003-managed-vendor-snapshots.md).
