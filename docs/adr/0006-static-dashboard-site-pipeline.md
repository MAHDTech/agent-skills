# ADR 0006: Static Dashboard Site Pipeline with Tailwind CSS, Zola, and Pagefind

Date: 2026-10-25. Status: Accepted. Implementation has been approved.

## Context

Following the migration of developer tooling to Rust ([ADR 0001](0001-rust-tooling-migration.md)) and the unified `ask` CLI design ([ADR 0002](0002-unified-cli-and-tui.md)), the legacy `bin/dashboard` script needed to be replaced by native Rust subcommands under `ask dashboard`.

As noted in the [skills-archive plan](../plans/skills-archive.md), `ask dashboard` serves two distinct workflows:

1. Fast terminal catalog telemetry (inspecting skill counts, token budgets, category/target distribution, and catalog health scores in the terminal or CI).
2. Full static site compilation, live-reload local serving, search indexing, and committed-artifact drift linting using Tailwind CSS v4, Zola, and Pagefind.

Coupling in-memory catalog telemetry with external static-site binaries would slow down terminal status checks and require `tailwindcss`, `zola`, and `pagefind` for simple metadata queries.

## Decision

We will split `ask dashboard` into an in-memory telemetry engine in `skills-core` and an external static-site orchestration pipeline in `ask-cli`:

1. **Command Dispatch (`crates/ask-cli/src/commands/dashboard.rs`):**
   - `run(args, format)` in [`crates/ask-cli/src/commands/dashboard.rs`](../../crates/ask-cli/src/commands/dashboard.rs) routes both `--action <summary|build|serve|css|lint>` flags (used by `devenv.nix` scripts) and `DashboardCommands::{Summary, Build, Serve, Css, Lint}` subcommands, defaulting to `run_summary(format)` when invoked without a subcommand.
2. **External Tool Orchestration (`crates/ask-cli/src/commands/dashboard_site.rs`):**
   - `dashboard_site.rs` in [`crates/ask-cli/src/commands/dashboard_site.rs`](../../crates/ask-cli/src/commands/dashboard_site.rs) orchestrates `tailwindcss` (Tailwind CSS v4), `zola`, `pagefind`, and `git` provisioned by `devenv.nix` via `exec_tool` and `exec_tool_capture`, surfacing `format_tool_not_found_error` diagnostics that guide developers to run inside `devenv --no-tui shell`.
3. **Static Site Pipeline Subcommands (`crates/ask-cli/src/commands/dashboard_site.rs`):**
   - `run_build` (`build_site`):
     1. Runs `ArtifactsEngine::with_options(root, options).generate_all()` to synchronize repository and Zola content artifacts.
     2. Compiles Tailwind CSS v4 from `dashboard/css/input.css` into `dashboard/static/build/css/generated.css`.
     3. Queries git history across `skills/` and `skills-archive/` to write skill creation and last-modified dates into the untracked `dashboard/skill_dates.toml` sidecar (`write_skill_dates`).
     4. Cleans stale output directories (`clean_output_dir`) and builds the static site via `zola --root dashboard build`.
     5. Copies raw `SKILL.md` files for discovery-eligible custom promoted live skills into `.well-known/agent-skills/<name>/SKILL.md` and `.well-known/skills/<name>/SKILL.md` (`materialize_well_known`).
     6. Indexes the compiled HTML with `pagefind --site <public_dir>` (`--output-subdir pagefind` for production builds or `--output-path dashboard/static/pagefind` in `serve_mode`).
   - `run_css`: compiles standalone Tailwind CSS v4 (`dashboard/css/input.css` to `dashboard/static/build/css/generated.css`) without running a full Zola build.
   - `run_lint` (`check_lint_at` and `check_zola_site`): runs `ArtifactsEngine` into a temporary `StagingDir` (`output_root`), byte-compares generated `dashboard/content/skills` and `dashboard/static/.well-known` trees against the working tree (`compare_generated_trees`), checks `git status --porcelain` for untracked or unstaged drift (`porcelain_drift`), and validates internal links and templates with `zola --root dashboard check --skip-external-links`.
   - `run_serve`: runs an initial `build_site(&root, None, true)` so `dashboard/static/pagefind` exists before Zola starts, spawns `tailwindcss --watch` (`spawn_tailwind`) and `zola --root dashboard serve` (`spawn_zola`), and runs `serve_event_loop` polling `skills/`, `dashboard/content/` (excluding generated `dashboard/content/skills`), and `dashboard/themes/` every 300ms with a 300ms debounce to invoke `sync_live_reload_inputs` and `rebuild_search_indexes`.
4. **Separation from Terminal Catalog Telemetry (`crates/skills-core/src/dashboard.rs`):**
   - `DashboardEngine` and `DashboardSummary` in [`crates/skills-core/src/dashboard.rs`](../../crates/skills-core/src/dashboard.rs) compute pure in-memory catalog metrics (`total_skills`, `active_skills`, `inactive_skills`, `promoted_skills`, `lifecycle_skills`, prompt and total token estimates via `calculate_prompt_tokens` and `calculate_total_tokens`, `HealthScore`, `CategoryDistribution`, `TargetDistribution`, and `SkillMetric`) without invoking external binaries or touching `dashboard/public`.
   - `run_summary` in [`crates/ask-cli/src/commands/dashboard.rs`](../../crates/ask-cli/src/commands/dashboard.rs) formats `DashboardSummary` as terminal tables (`comfy_table`), plain text, JSON, or YAML for `ask dashboard summary` and bare `ask dashboard`.

## Consequences

### Positive

- All-in-one static site workflow: `ask dashboard build` and `ask dashboard serve` coordinate artifact generation, Tailwind CSS v4, git freshness dates, Zola compilation, `.well-known` materialization, and Pagefind search indexing behind a single command.
- Non-destructive CI verification: `ask dashboard lint` validates both committed artifact freshness (via `StagingDir`) and Zola template/link integrity without dirtying the working tree.
- Clean separation of concerns: `DashboardEngine` in `skills-core` remains fast and free of external binary dependencies, while `dashboard_site.rs` in `ask-cli` encapsulates subprocess orchestration.

### Negative / Tradeoffs

- Running `ask dashboard build`, `serve`, `css`, or `lint` requires the external `tailwindcss`, `zola`, `pagefind`, and `git` binaries on `PATH` (provisioned automatically by `devenv`).
- `dashboard/skill_dates.toml` and `dashboard/static/build/css/generated.css` are generated at build time and excluded from git tracking so routine builds do not dirty the working tree.
