# ADR 0003: Managed vendor snapshots in the shared catalog

Date: 2026-10-24. Status: Accepted during the design interview. Implementation has been approved.

## Context

Vendor repositories publish complete Agent Skills collections and update them frequently. Maintaining local copies by hand or running a separate installer for every vendor would split this repository's declared catalog from the skills actually used.

The shared catalog must contain the complete set of custom and declared vendor skills at any repository commit, so they can be used and tested together. Consumers must also retain the existing `npx skills add MAHDTech/agent-skills` installation path.

Git submodules give clean upstream revision tracking, but the inspected `npx skills` acquisition code does not initialize them. A wrapper around vendor installation commands would install content per machine without placing it in the shared catalog. See the [primary-source research](../research/declarative-vendor-skills.md).

## Decision

Commit managed vendor snapshots as part of the shared catalog:

- A YAML source manifest identifies Git repositories and their skill directory paths. A Rust module in `skills-core` uses the existing Git executable for acquisition and authentication. No Node or npm runtime is required.
- Git records the downloaded content; no skill lockfile, persisted hash inventory, or restore-from-lock command is needed.
- Each source contributes all its skills, including future additions discovered during updates.
- Store vendor skills under `skills/vendor/<vendor>/<skill>/`. Present a Vendor Skills section grouped by source in the dashboard and generated indexes.
- Preserve upstream files unchanged. Give only managed vendor content explicit exceptions from local authoring rules; retain import validation for usable frontmatter, contained paths, required files, and name collisions.
- Reject duplicate active skill names within or across sources, including custom skills. Report the conflicting sources before replacing snapshots.
- Treat vendor folders as disposable generated output. Updates regenerate them and overwrite local edits. Customizations belong outside managed vendor paths.
- Mirror upstream removals by removing the corresponding managed snapshots. Previous versions remain in Git history. Custom skills retain their archive policy.
- `ask skills download-resources` advances declared vendor branches and refreshes documentation. It also regenerates repository catalog artifacts. Local agent linking remains an `ask skills sync` operation.
- Prepare each vendor before replacement. Failed downloads retain that vendor's last successful snapshot and allow other sources to advance. This supersedes the earlier all-or-nothing proposal. Validate unique active names across the combined catalog before replacement. Documentation downloads are a separate operation.
- Ordinary installs, syncs, builds, and tests consume the committed vendor snapshots without advancing upstream revisions.
- Include daily and manually runnable vendor updates that maintain one combined PR on a stable automation branch. Later runs refresh the same open PR rather than failing or opening duplicates. A failed source retains any successful snapshot already pending in that PR. Merging remains a review step.

The [implementation plan](../plans/declarative-vendor-skills.md) records the evolving schema, automation scope, remaining questions, and acceptance checks.

## Consequences

Every repository commit contains the actual vendor content consumers will install. Update reviews can inspect content changes, new skills, and removals together with the rest of the catalog.

The repository will contain more files. Discovery, lint, installation, generated indexes, and dashboard rendering need explicit vendor ownership rather than assuming every skill follows local authoring conventions.

Local edits inside managed vendor folders will be overwritten during regeneration. Supporting files, executable permissions, applicable licenses, and notices must survive import and individual skill installation. Vendor content must be excluded from the documentation downloader and resource cleanup.

Updates need staging per source, validation of the combined catalog, and rollback for local replacement failures. Sources may reflect different successful download times after an outage. Source namespacing does not eliminate installed-name collisions, so incompatible collections must be resolved before an update can be applied.
