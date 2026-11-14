# Installation Guide

These skills are cross-compatible with Claude Code, OpenCode, Goose, and Antigravity CLI. All four read the same Anthropic-style `skills/<name>/SKILL.md` format.

## For users

Install with the [skills.sh](https://skills.sh) CLI:

[![skills.sh](https://skills.sh/b/MAHDTech/agent-skills)](https://skills.sh/MAHDTech/agent-skills)

```bash
# NPM users
npx skills add MAHDTech/agent-skills

# Bun users
bunx skills add MAHDTech/agent-skills
```

The installer auto-detects which agents you have installed (Claude Code, OpenCode, Goose, Antigravity), then lets you pick the skills and the agents you want. Selected skills are installed by symlink, so updates flow through automatically.

To update later:

```bash
# Using npm
npx skills update

# Using Bun
bunx skills update
```

## For developers

If you are working on the skills in this repository, install them from your local working tree so you can iterate live. The `ask` CLI is provided by the `devenv` shell environment, so prefix commands with `devenv --no-tui shell -- <command>` or run them after entering `devenv --no-tui shell`.

1. Clone the repository:

   ```bash
   git clone https://github.com/MAHDTech/agent-skills.git
   cd agent-skills
   ```

2. Install all live skills into every standard target whose agent tool is installed:

   ```bash
   devenv --no-tui shell -- ask skills --action install
   ```

3. Install one directory containing a `SKILL.md` file:

   ```bash
   devenv --no-tui shell -- ask skills --action install skills/engineering/domain-modeling
   ```

4. Uninstall all matching repository skills in those targets:

   ```bash
   devenv --no-tui shell -- ask skills --action uninstall
   ```

The repository is authoritative for matching skill names. Bulk install discovers the live `skills/` tree, including draft skills in `in-progress/`, and excludes `skills-archive/`. Install replaces matching destinations, including untracked copies and broken links, and creates backups under the target's `.backups/` directory.

Bulk uninstall matches names from both the live and archived catalog. It removes matching directories, symlinks, and registry entries even if an installation was not tracked by ask. It leaves unrelated names, backups, and the source repository untouched. Repeated bulk uninstall is safe when matching installations are already absent. Bulk operations continue processing other skills and return a failure if any operation fails.

The `--action install` and `--action uninstall` forms default to every detected standard target. Use `--target agents`, `--target claude`, `--target antigravity`, `--target antigravity-acp`, `--target cursor`, or `--target /path/to/skills` to select one destination. To remove one tracked skill, use `devenv --no-tui shell -- ask skills --action uninstall domain-modeling`.

The existing `devenv --no-tui shell -- ask skills install <source>` and `devenv --no-tui shell -- ask skills uninstall <skill>` subcommands default to the shared `~/.agents/skills/` target. Both accept `--target`.

### Synchronization

```bash
# Inspect the plan without modifying repository files or installed skills
devenv --no-tui shell -- ask skills --action sync --dry-run

# Update installations and regenerate repository artifacts
devenv --no-tui shell -- ask skills --action sync

# Equivalent subcommand form
devenv --no-tui shell -- ask skills sync --dry-run
```

Sync reconciles the live catalog with every detected standard target. It installs missing skills, updates managed installations, and regenerates derived repository files and dashboard content. On Unix, installation uses symlinks; other platforms default to copies. A target is skipped, with a message, when its tool's configuration directory (`~/.claude`, `~/.gemini`, or `~/.cursor`) does not exist, so sync never creates directories for tools you do not use. The shared `~/.agents/skills/` target is always synced. Pass `--target` to force a specific destination.

The same command removes obsolete installations sourced from this repository's live or archived catalogs. Renaming `spacetimedb` to `spacetime`, for example, installs the new skill and removes the old installation and registry entry. Cleanup also finds old symlinks that are missing from the registry, including dangling links and relative symlinks. Removed installations are backed up under the target's `.backups/` directory.

Sync preserves untracked directories such as `~/.agents/skills/test1/`, installations from other catalogs, and symlinks pointing outside this repository. A manual skill whose name matches a live repository skill still follows the existing conflict policy: the repository version replaces it with a backup. Use `--dry-run` to inspect installations, updates, and removals without changing files.

No single directory is read by every agent harness. The target resolver uses the smallest set of global directories that covers them:

| Target (`--target`)                      | Directory                           | Read by                                                     |
| ---------------------------------------- | ----------------------------------- | ----------------------------------------------------------- |
| `agents` (also `codex`, `opencode`, ...) | `~/.agents/skills/`                 | Codex, OpenCode, Goose, Grok, Cline, Zed, Pi, Warp          |
| `claude` (also `claude-code`)            | `~/.claude/skills/`                 | Claude Code; also OpenCode, Goose, and Grok                 |
| `antigravity` (also `agy`)               | `~/.gemini/antigravity-cli/skills/` | Antigravity CLI; T3 Code's Antigravity provider             |
| `antigravity-acp`                        | `~/.gemini/config/skills/`          | Antigravity IDE and Antigravity over ACP, including T3 Code |
| `cursor`                                 | `~/.cursor/skills/`                 | Cursor; also Grok                                           |

ACP clients launch the same agent binary (or an adapter around it), so an agent reached over ACP reads the same directories as its interactive CLI. Codex and Grok over ACP are covered by `~/.agents/skills/`; Antigravity over ACP by the two `~/.gemini/` targets.

OpenCode, Goose, Grok, and Codex have no dedicated target because they already read `~/.agents/skills/`. Tools that read more than one of these directories deduplicate skills by name. Sync does not modify Gemini configuration files.

Earlier releases installed the `claude` target into `$XDG_CONFIG_HOME/claude/skills/` (or `~/Library/Application Support/Claude/skills/` on macOS), which no tool reads. A default sync removes this repository's links from those directories and leaves anything else in place.

Sync prints the reason for each planned action. The CLI uses a repository-wins policy: changed copies, redirected or broken links, untracked destinations, and older repository versions are replaced with the repository's version, with backups. Managed links from another checkout are pointed at the current checkout. Repeated sync converges to an up-to-date installation.

Sync also removes names in `skills-archive/` from targets, including untracked copies. It leaves unrelated installed names alone. If a name is present in both live and archived catalogs, the live catalog wins.

Run sync from a persistent checkout. Links installed from a temporary worktree break when that worktree is deleted. A later sync from this checkout repairs them.

`SKILLS_REPO_ONLY=1`, `PRE_COMMIT`, `CI`, or running inside a linked Git worktree skips target synchronization on a normal sync, while dry-run still inspects targets. The standalone sync engine retains its conservative conflict policy by default; repository-wins behavior is selected by the CLI.

### Antigravity ACP in T3 Code

T3 Code's Antigravity provider reads global skills from `~/.gemini/config/skills/`
and `~/.gemini/antigravity-cli/skills/`. It does not read global `~/.agents/skills/`.
See the upstream [Antigravity provider documentation](https://github.com/pingdotgg/t3code/blob/main/docs/user/providers-antigravity.md#skills-and-attachments)
and [skill discovery implementation](https://github.com/pingdotgg/t3code/blob/main/apps/server/src/provider/Drivers/AntigravitySkills.ts).

Install the live catalog into the ACP location:

```bash
devenv --no-tui shell -- ask skills --action install --target antigravity-acp
```

For one skill, use `devenv --no-tui shell -- ask skills install domain-modeling --target antigravity-acp`.
Bulk install and sync also include this target by default. Skills are installed as
individual links on Unix, with each `SKILL.md` directly inside its skill directory.
The `antigravity` target installs into `~/.gemini/antigravity-cli/skills/` for the Antigravity CLI.
Run the installation on the environment where T3 Code runs its Antigravity provider.
