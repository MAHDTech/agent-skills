# ADR 0005: Deterministic Generated Artifact Synchronization Engine

<!-- cspell:words syncer Syncer -->

Date: 2026-10-25. Status: Accepted. Implementation has been approved.

## Context

Multiple repository-tracked files derive their contents from the `skills/` and `skills-archive/` catalogs:

- `README.md` (catalog tables, badges, CLI/TUI overview, and top-level `docs/*.md` index)
- `agents/AGENTS.md` (agent discovery index for custom and vendor skills)
- `skills.sh.json` (downstream manifest for `npx skills add MAHDTech/agent-skills`)
- `dashboard/content/skills/` (Zola section and skill pages, copied resources, and archive pages)
- `dashboard/static/.well-known/agent-skills/index.json` and `dashboard/static/.well-known/skills/index.json` (agent skill discovery manifests)

Maintaining these derived files by hand or coupling repository artifact generation with host-machine symlink installation caused drift in CI, pre-commit hooks, and linked git worktrees. As noted in the [skills-archive plan](../plans/skills-archive.md), deterministic generated-artifact synchronization needed a dedicated Rust engine in `skills-core` separate from local environment linking.

## Decision

We will centralize all repository-tracked catalog artifact generation in `ArtifactsEngine` within [`crates/skills-core/src/artifacts.rs`](../../crates/skills-core/src/artifacts.rs) and keep it strictly decoupled from host-machine environment synchronization in [`crates/skills-core/src/sync.rs`](../../crates/skills-core/src/sync.rs):

1. **Dedicated `ArtifactsEngine` (`crates/skills-core/src/artifacts.rs`):**
   - Provide `ArtifactsOptions` (`repo_only`, `skip_dashboard`, `no_stage`, `output_root`), `ArtifactsSummary`, and `ArtifactsEngine` (aliased as `ArtifactsGenerator`) with constructors `new`, `with_options`, and `from_env`.
   - `ArtifactsEngine::generate_all` discovers all catalog skills via `SkillParser::discover_skills`, partitions them into `live_skills` (`SkillTree::Live`) and `archived_skills` (`SkillTree::Archive`), and deterministically writes:
     - `agents/AGENTS.md` (`generate_agents_md`): preserves existing YAML frontmatter and lists promoted live custom skills by category followed by `Vendor Skills` grouped by vendor.
     - `README.md` (`generate_readme`): renders repository badges, CLI/TUI reference, sorted top-level `docs/*.md` links, and promoted live skills.
     - `skills.sh.json` (`generate_skills_sh_json`): serializes `SkillsManifest` containing promoted live skills grouped by category for `skills.sh`.
     - `dashboard/content/skills/` (`generate_dashboard_content`): when `!skip_dashboard && !repo_only`, writes Zola `_index.md` section and page files for promoted live skills and the weighted-last `Archive` section (`dashboard/content/skills/archive/<category>/<name>/` with `archived`, `replaced_by`, and `source_url` metadata), rewrites relative cross-skill links (`rewrite_skill_links`), escapes Tera/Zola shortcodes (`escape_zola_shortcodes`), and copies sibling Markdown docs and `resources/`.
     - `dashboard/static/.well-known/agent-skills/index.json` (RFC v0.2.0) and `dashboard/static/.well-known/skills/index.json` (v0.1.0) (`generate_discovery_index`): when `!skip_dashboard && !repo_only`, generates discovery manifests and SHA-256 digests for custom promoted live skills.
   - When `!no_stage && output_root.is_none()`, `stage_files` stages the updated paths via `git add --` so pre-commit sync hooks include regenerated files in the active commit.
2. **Non-Destructive Staging (`output_root`):**
   - `ArtifactsOptions::output_root` redirects all generated outputs to an alternate directory (such as a temporary `StagingDir` during `ask dashboard lint`) without modifying or staging files in the working tree.
3. **CLI Call Sites (`crates/ask-cli`):**
   - In [`crates/ask-cli/src/commands/skills.rs`](../../crates/ask-cli/src/commands/skills.rs), `run_sync` executes `ArtifactsEngine::from_env(&root).generate_all()?` whenever `!args.dry_run`, and `run_download_resources` executes `ArtifactsEngine::with_options(&root, ArtifactsOptions { no_stage: true, ..ArtifactsOptions::from_env() }).generate_all()?` after updating vendor snapshots or documentation resources.
   - In [`crates/ask-cli/src/commands/dashboard_site.rs`](../../crates/ask-cli/src/commands/dashboard_site.rs), `build_site`, `check_lint_at` (via `StagingDir` and `output_root`), and `sync_live_reload_inputs` invoke `ArtifactsEngine::with_options(root, options).generate_all()?`.
4. **Separation from `SkillSyncer` (`crates/skills-core/src/sync.rs`):**
   - `SkillSyncer` in [`crates/skills-core/src/sync.rs`](../../crates/skills-core/src/sync.rs) handles only host-machine reconciliation across `TargetEnvironment` variants (`ClaudeDesktop`, `Cursor`, `Antigravity`, `AntigravityAcp`, and `Custom`) through two-phase planning (`create_plan` producing `SyncPlan`) and execution (`execute_plan` producing `SyncSummary`), managing symlinks or copies, `InstalledSkillsRegistry`, conflict resolution, and orphan pruning.
   - In [`crates/ask-cli/src/commands/skills.rs`](../../crates/ask-cli/src/commands/skills.rs), `should_sync_repo_only` detects CI, pre-commit hook environments (`PRE_COMMIT`, `SKILLS_REPO_ONLY`), and linked git worktrees, skipping `SkillSyncer` so automated checks never mutate the developer's host `~/.claude`, `~/.cursor`, or `~/.gemini` directories while still running `ArtifactsEngine`.

## Consequences

### Positive

- Single source of truth: all repository indexes, manifests, and Zola content pages are deterministically derived from `skills/` and `skills-archive/` in one pass.
- Safe automation: CI, pre-commit hooks, and isolated agent git worktrees can run `ask skills sync` and `ask dashboard lint` without mutating host-level agent skill installations.
- Non-destructive drift verification: `ArtifactsOptions::output_root` enables `ask dashboard lint` to verify committed artifacts in a temporary staging directory before checking `git status --porcelain`.

### Negative / Tradeoffs

- Adding, editing, or archiving a skill requires regenerating and committing the derived repository artifacts (`README.md`, `agents/AGENTS.md`, `skills.sh.json`, `dashboard/content/skills/`, and `dashboard/static/.well-known/`).
