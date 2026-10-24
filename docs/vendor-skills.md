# Vendor skills

Declare upstream skill collections in `vendor-skills.yaml`. The Rust `ask` CLI downloads complete skill folders through Git and commits them alongside the custom catalog. No Node, npm, or skill lockfile is required.

```yaml
version: 1
sources:
  - id: cloudflare
    repository: https://github.com/cloudflare/skills.git
    ref: main
    path: skills
```

The source `id` becomes the directory under `skills/vendor/`. `repository` supports HTTPS and SSH Git URLs; absolute local repositories are also supported for testing. `ref` is optional and defaults to the repository's default branch. `path` defaults to `skills` and must stay within the repository. Every skill in the declared collection is imported.

The initial downloader supports flat collections of complete skill directories. Nested collections and symlinks are reported as unsupported rather than partially imported. The failed source retains its previous snapshot. Scripts are copied with their permissions and are never executed during acquisition.

Optional `license-files` selects repository-relative license or notice files to accompany each installed skill. Use it when the skills have a different license from the repository root. Otherwise, root license and notice files are included automatically; existing files inside a skill are preserved.

```bash
ask skills download-resources
```

This refreshes vendor snapshots and custom-skill documentation, then regenerates the catalog indexes and dashboard content. To refresh only vendors and catalog artifacts:

```bash
ask skills download-resources --vendors-only
```

Vendor snapshots are generated output. Successful refreshes overwrite local edits and remove skills deleted upstream. A failed vendor retains its previous complete snapshot; other vendors continue updating, and the command reports skipped sources. Duplicate active skill names remain an error and must be resolved before publishing the catalog.

Ordinary installation, `ask skills sync`, lint, builds, and tests use the checked-in snapshots. Git restores the complete catalog for any repository commit. `ask skills clean-resources` and documentation downloads leave vendor-owned files untouched.

## Daily update PR

The scheduled workflow runs daily and can also be started manually. It maintains one combined PR on `automation/vendor-skills`. An unmerged PR is refreshed in place on subsequent runs. Unchanged content produces no additional commits or PRs.

The updater can retain successful snapshots already pending in that PR when a later download fails:

```bash
ask skills download-resources --vendors-only --pending-ref origin/automation/vendor-skills
```

The pending ref must already be available in the local checkout. A changed source declaration or newer base-branch edits take precedence over pending snapshots. No additional state file is needed.

The workflow uses the repository's update GitHub App credentials, with the existing `UPDATE_APP_ID` / `APP_ID` and `UPDATE_APP_PRIVATE_KEY` / `APP_PRIVATE_KEY` fallbacks. Git access to vendor repositories uses existing Git authentication. Download failures appear in the run summary; validation still prevents publishing a conflicting catalog.
