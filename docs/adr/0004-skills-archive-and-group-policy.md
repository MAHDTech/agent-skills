# ADR 0004: Skills Archive Layout and Cohesive Group Cross-Reference Policy

<!-- cspell:words syncer Syncer -->

Date: 2026-09-04. Status: Accepted during the design interview. Implementation has been approved.

## Context

Previously, retired skills lived inside `skills/deprecated/`. Keeping retired skills inside the `skills/` directory tree created two structural problems:

1. External consumers installing skills with `npx skills add MAHDTech/agent-skills` (`skills.sh`) scan the entire `skills/` directory tree, so deprecated skills remained discoverable and installable unless filtered by custom flags.
2. Moving a skill into `skills/deprecated/` erased its original topic category (`engineering`, `planning`, `review`, `github`, `tooling`, and others), preventing the static dashboard from grouping retired skills by their domain origin.

At the same time, unconstrained cross-references between skills (via `/skill-name` slash command invocations or relative Markdown links) created implicit dependencies. When consumers installed a single skill or when a referenced skill moved into the archive, cross-skill links broke or left hidden runtime assumptions.

See the [skills-archive implementation plan](../plans/skills-archive.md) for the design interview record.

## Decision

We will replace `skills/deprecated/` with a top-level `skills-archive/` directory and enforce a cohesive group cross-reference policy across the catalog:

1. **Top-Level Archive Tree (`skills-archive/<category>/<name>/SKILL.md`):**
   - Retired custom skills move from `skills/<category>/<name>/` to `skills-archive/<category>/<name>/SKILL.md`, preserving their original `<category>` subdirectory.
   - Because `skills-archive/` sits outside `skills/`, external installers (`skills.sh`) and local installation commands (`ask skills install`, `ask skills sync`) exclude archived skills structurally.
2. **Domain Model and Parser Representation:**
   - In [`crates/skills-core/src/models.rs`](../../crates/skills-core/src/models.rs), `SkillTree::Live` (`skills`) and `SkillTree::Archive` (`skills-archive`) identify the catalog tree of each `Skill`, alongside optional `group` (`metadata.group`), `archived` (`metadata.archived`), and `replaced_by` (`metadata.replaced-by`) fields.
   - In [`crates/skills-core/src/parser.rs`](../../crates/skills-core/src/parser.rs), `ParsedSkill::into_skill` infers `SkillTree` from the relative path and forces `promoted = false` whenever `tree == SkillTree::Archive`.
3. **Archive Frontmatter and Lint Rules (`crates/skills-core/src/lint.rs`):**
   - `check_archived_metadata` (`archived-metadata`): every skill in `SkillTree::Archive` must declare `metadata.archived` as a valid `YYYY-MM-DD` date, and skills in `SkillTree::Live` must not set `metadata.archived`.
   - `check_duplicate_names` (`duplicate-skill-names`): skill names must remain globally unique across both `SkillTree::Live` and `SkillTree::Archive`.
   - `check_archived_replaced_by` (`archived-replaced-by`): when an archived skill declares `metadata.replaced-by`, the target name must resolve to an existing promoted skill in `SkillTree::Live`.
4. **Cohesive Group Cross-Reference Policy (`crates/skills-core/src/lint.rs`):**
   - Skills are self-contained by default. `check_cross_references` scans non-fenced lines for `/skill-name` command invocations (`scan_slash_commands`) and relative Markdown links (`scan_cross_reference_links`), delegating validation to `validate_cross_reference` (`cross-reference-group`).
   - Live skills must never reference archived skills.
   - Two live skills may cross-reference each other only when both declare the identical `metadata.group` value. The six standard cohesive groups are `authoring`, `github`, `planning-pipeline`, `review`, `opencode`, and `guards`.
   - The universal router (`skill-router`), managed vendor snapshots (`vendor_source().is_some()`), and archived skills (`SkillTree::Archive`) are exempt as cross-reference sources.
5. **Local Uninstall and Orphan Pruning (`crates/ask-cli/src/commands/skills.rs`):**
   - `repository_skill_names(root, archived)` enumerates both `skills/` and `skills-archive/` so bulk `ask skills uninstall` removes any local links or copies of retired skills owned by this repository.
   - `build_skill_syncer` configures `SkillSyncer` with `.with_prune_source(root.join("skills-archive"))`, ensuring `ask skills sync` automatically prunes local links or copies of skills that have moved to `skills-archive/`.
6. **Generated Artifacts and Dashboard Presentation:**
   - Exclude `SkillTree::Archive` skills from `README.md`, `agents/AGENTS.md`, `skills.sh.json`, and `.well-known` discovery manifests.
   - Render archived skills on the static dashboard in a collapsible, weighted-last `Archive` category (`dashboard/content/skills/archive/<category>/<name>/`) sub-grouped by original `<category>`, displaying an archive banner with the retirement date, optional replacement link, and GitHub source URL.

## Consequences

### Positive

- Structural exclusion guarantees `npx skills add MAHDTech/agent-skills` and local sync/install workflows never install retired skills.
- Preserving `<category>` under `skills-archive/<category>/<name>/` keeps historical organization intact on the dashboard without cluttering `README.md`, `agents/AGENTS.md`, or `skills.sh.json`.
- Compile-time and `ask skills lint` checks prevent broken replacement references, duplicate skill names, live-to-archive links, and undeclared cross-skill dependencies.
- Maintainers and contributors running `ask skills sync` automatically prune newly archived skills from their local agent directories.

### Negative / Tradeoffs

- Retiring a skill requires moving its directory to `skills-archive/<category>/<name>/`, setting `metadata.archived` (and optionally `metadata.replaced-by`), scrubbing live cross-references, and regenerating repository artifacts.
- Related skills that intentionally reference each other must explicitly maintain matching `metadata.group` frontmatter declarations.
