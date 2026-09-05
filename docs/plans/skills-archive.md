# Plan: skills-archive

Date: 2026-09-04. Branch: `feat/skills-archive`. Decided through a `/grilling` interview; this file records the outcome.

## Goal

Retired skills are archived, never deleted. An archived skill stays readable on the dashboard, clearly marked as no longer in use, but is never installed: not by the local installer, not by `npx skills add MAHDTech/agent-skills`. Moving a skill into the archive and running `sync` removes it from the maintainer's machine.

## Decisions

| Decision                                     | Outcome                                                                                                                                                                                                                               | Why                                                                                                                                                                   |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Relationship to `skills/deprecated/`         | Replaced. The three deprecated skills move to the archive; the bucket is deleted.                                                                                                                                                     | One retirement concept. A top-level directory is outside the `skills/` scan that the skills.sh CLI performs, so exclusion is structural rather than a per-skill flag. |
| Archive layout                               | `skills-archive/<category>/<name>/SKILL.md`, original category kept.                                                                                                                                                                  | Mirrors the live tree; the dashboard can sub-group by origin.                                                                                                         |
| Workflow skill                               | `deprecate-skill` renamed in place to `archive-skill` and rewritten.                                                                                                                                                                  | The verb should match the directory. No archived copy: the old text described a directory that no longer exists.                                                      |
| Archive marker                               | Frontmatter `metadata.archived: "<date>"`, optional `metadata.replaced-by: "<name>"`.                                                                                                                                                 | Structured, so the dashboard banner is generated, and lint can require it.                                                                                            |
| Dashboard                                    | One collapsible Archive category weighted last in the Skills Catalog, sub-grouped by original category. Each page shows an archived banner naming the replacement and linking to the GitHub source folder.                            | Catalog stays "what I use"; consumers still find and fetch archived skills.                                                                                           |
| README, `agents/AGENTS.md`, `skills.sh.json` | Exclude the archive.                                                                                                                                                                                                                  | Dashboard is the archive's home.                                                                                                                                      |
| Cross-skill references                       | Skills are self-contained by default. Exceptions are declared groups via `metadata.group`; members may reference each other. `skill-router` is exempt. Proposed groups: authoring, github, planning-pipeline, review, opencode, tars. | Owner's policy; prefixes and categories do not capture the real groups.                                                                                               |
| First batch                                  | All seven `tars-backlog-*` skills, replaced by `tars-run-factory`.                                                                                                                                                                    | Superseded by the tars-agy plugin.                                                                                                                                    |
| Where the code goes                          | Rust only. No further TypeScript is written in this repo. Anything needing Rust is parked as issues in the Rust EPIC (#89).                                                                                                           | The devenv `skills` and `dashboard` wrappers already route to the Rust `ask-cli`; the TypeScript tooling is being decommissioned (#88).                               |

## Done in this change (markdown and git moves only)

- `skills-archive/` created; `plan-before-coding`, `pr-prepare-review`, `refactor-codebase`, and the seven `tars-backlog-*` skills moved with `metadata.archived` and `metadata.replaced-by` set. `skills/deprecated/` removed.
- `deprecate-skill` renamed to `archive-skill` and rewritten for the new layout.
- tars mentions scrubbed from `implement`, `devenv`, `skill-creator`, and the router. `skill-audit`, `skill-creator`, `AGENTS.md`, and `docs/usage.md` describe the archive instead of the deprecated bucket.
- README, `agents/AGENTS.md`, `skills.sh.json`, and dashboard content regenerated once by hand with the existing bun sync. The devenv wrappers currently call a Rust stub, so the lint and sync hooks are vacuous until the EPIC catches up; that is left alone deliberately.

## Parked in the Rust EPIC (#89)

1. skills-core: `skills-archive` discovery and category model, `archived`/`replaced-by` validation, `metadata.group` cross-reference lint (router exempt), plus the collection-wide self-contained sweep and group declarations.
2. skills-core: generated-artifact sync (README, `agents/AGENTS.md`, `skills.sh.json`, dashboard content) including the Archive section and banner. This was a gap in the EPIC: #80 covers drift detection, not artifact generation.
3. ask-cli: dashboard site build (Tailwind, Zola, Pagefind). Also a gap: #86's `ask dashboard` is a terminal summary.

Until (2) lands, the archive is not visible on the dashboard; the content is in place and marked, and the banner and section follow with the Rust sync.
