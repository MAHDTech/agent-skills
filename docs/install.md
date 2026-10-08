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

If you are working on the skills in this repository, install them from your local working tree so you can iterate live.

1. Clone the repository:

   ```bash
   git clone https://github.com/MAHDTech/agent-skills.git
   cd agent-skills
   ```

2. Install all live skills into ask's three standard targets:

   ```bash
   ask skills --action install
   ```

3. Install one directory containing a `SKILL.md` file:

   ```bash
   ask skills --action install skills/engineering/domain-modeling
   ```

4. Uninstall all matching repository skills in those targets:

   ```bash
   ask skills --action uninstall
   ```

The repository is authoritative for matching skill names. Bulk install discovers the live `skills/` tree, including draft skills in `in-progress/`, and excludes `skills-archive/`. Install replaces matching destinations, including untracked copies and broken links, and creates backups under the target's `.backups/` directory.

Bulk uninstall matches names from both the live and archived catalog. It removes matching directories, symlinks, and registry entries even if an installation was not tracked by ask. It leaves unrelated names, backups, and the source repository untouched. Repeated bulk uninstall is safe when matching installations are already absent. Bulk operations continue processing other skills and return a failure if any operation fails.

The `--action install` and `--action uninstall` forms default to all three standard targets. Use `--target antigravity`, `--target claude`, `--target cursor`, or `--target /path/to/skills` to select one destination. To remove one tracked skill, use `ask skills --action uninstall domain-modeling`.

The existing `ask skills install <source>` and `ask skills uninstall <skill>` subcommands retain their Antigravity-only default. Both accept `--target`.

### Synchronization

```bash
# Inspect the plan without modifying repository files or installed skills
ask skills --action sync --dry-run

# Update installations and regenerate repository artifacts
ask skills --action sync

# Equivalent subcommand form
ask skills sync --dry-run
```

Sync reconciles the live catalog with all three standard targets. It installs missing skills, updates managed installations, and regenerates derived repository files and dashboard content. On Unix, installation uses symlinks; other platforms default to copies. It does not auto-detect installed agent applications.

The current target resolver uses these locations:

| Target                  | Directory                                                              |
| ----------------------- | ---------------------------------------------------------------------- |
| Antigravity             | `~/.agents/skills/`                                                    |
| Cursor                  | `~/.cursor/skills/`                                                    |
| Claude Desktop on Linux | `$XDG_CONFIG_HOME/claude/skills/`, normally `~/.config/claude/skills/` |
| Claude Desktop on macOS | `~/Library/Application Support/Claude/skills/`                         |

The `claude` option currently selects Claude Desktop, not the `~/.claude/skills/` directory described by older versions of this guide. There is no separate OpenCode or Goose target, and sync does not register skills in Gemini configuration.

Sync prints the reason for each planned action. The CLI uses a repository-wins policy: changed copies, redirected or broken links, untracked destinations, and older repository versions are replaced with the repository's version, with backups. Managed links from another checkout are pointed at the current checkout. Repeated sync converges to an up-to-date installation.

Sync also removes names in `skills-archive/` from targets, including untracked copies. It leaves unrelated installed names alone. If a name is present in both live and archived catalogs, the live catalog wins.

Run sync from a persistent checkout. Links installed from a temporary worktree break when that worktree is deleted. A later sync from this checkout repairs them.

`SKILLS_REPO_ONLY=1`, `PRE_COMMIT`, or `CI` skips target synchronization on a normal sync, while dry-run still inspects targets. The standalone sync engine retains its conservative conflict policy by default; repository-wins behavior is selected by the CLI.
