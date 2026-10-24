# Declarative vendor skills research

<!-- cspell:words gitsubmodule vinext -->

Researched on 2026-10-24 for a proposed `ask` feature. This records evidence and
architecture options, not accepted design decisions or an implementation plan.
Upstream source snapshots inspected: `vercel-labs/skills` at
`a80e4c9d12d607631249d4d7d059f835d4b2f13a` and `cloudflare/skills` at
`86505e44f87d801aa1864a3c4109e9a8ae5c409f`. Documentation can change independently
of published packages. The inspected CLI source identifies itself as version
1.7.4 and requires Node.js 22.20.0 or newer. [CLI package metadata](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/package.json).

## Verified upstream behavior

Cloudflare explicitly supports `npx skills add https://github.com/cloudflare/skills`
and cloning/copying its skill folders. Its native plugins also install MCP
configuration, so importing Agent Skills alone is a narrower capability than
installing the Cloudflare plugin. [Cloudflare installation documentation](https://github.com/cloudflare/skills/blob/86505e44f87d801aa1864a3c4109e9a8ae5c409f/README.md).

The CLI supports GitHub shorthand, Git URLs, repository subdirectories and local
directories. Project installation is the default; `--global` changes the scope.
`--skill '*'` selects all skills; `--agent` selects agents; `--yes` skips prompts;
`--copy` changes the installation method. `--all` selects all skills and all
agents, so it would be inappropriate as shorthand for a configured agent list.
Updates can explicitly select project or global scope. [Command reference](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/docs/commands.md).

Agent destinations differ. Claude Code uses `.claude/skills/` in a project and
`~/.claude/skills/` globally; Codex uses `.agents/skills/` and
`~/.agents/skills/`; OpenCode uses `.agents/skills/` and
`~/.config/opencode/skills/`; Goose uses `.goose/skills/` and
`~/.config/goose/skills/`. Automatic detection exists, but a declarative wrapper
should configure destinations explicitly to avoid machine-dependent results.
[Supported agents](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/docs/agents.md).

Discovery searches recognized containers such as `skills/` to three directory
levels, supporting `skills/category/category/name/SKILL.md`. A discovered skill
stops descent below its directory. The repository root receives a shallower
search. Plugin manifests can declare additional paths. `--full-depth` enables
broader discovery; ordinary discovery also falls back to recursive search when
no skills were found. Consequently, placing vendor repositories outside normal
containers can hide them when this repository already contains custom skills.
[Discovery documentation](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/docs/creating-skills.md).

The clone-based discovery path excludes installed project skills under agent
directories when they are tracked in `skills-lock.json`. It also deduplicates by
skill name by default. Installing vendor content into `.agents/skills/` is
therefore an unreliable way to republish that content through this repository.
Discovery uses a different path for cached GitHub snapshots, so that exclusion
should not be generalized to every internal installation route.
[Clone-based discovery](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/src/skills.ts).

Git sources accept `#ref`; `owner/repo@skill` selects a skill rather than a
revision. Git cloning first attempts a shallow branch/tag clone; a full
40-character commit SHA has a fetch-by-SHA fallback. None of those clone paths
initializes submodules. Inference: committed submodule pointers alone will not
make vendor skills available to downstream `npx skills add MAHDTech/agent-skills`
consumers. [Source parsing](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/src/source-parser.ts),
[Git acquisition](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/src/git.ts).

Unpinned GitHub sources can use a `skills.sh` snapshot download path. Explicit
refs bypass this ref-agnostic cache and use Git acquisition instead. This gives
a wrapper an existing route to installing a chosen commit, but the wrapper must
still choose and record that commit. [Snapshot acquisition](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/src/blob.ts).

The project lockfile is `skills-lock.json`: entries include source, optional
ref, skill path and a hash of the skill folder. The global lock normally lives
at `~/.agents/.skill-lock.json`, or under `XDG_STATE_HOME/skills`, and records a
folder hash plus installation timestamps. A mutable branch ref is not converted
into a persisted resolved commit by these schemas. Updates retain the recorded
ref and skill path. Inference: these installation records are insufficient as
the sole lock for reproducible vendor snapshots when tracking a branch.
[Project lock schema](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/src/local-lock.ts),
[Global lock schema](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/src/skill-lock.ts),
[Update source reconstruction](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/src/update-source.ts).

Skills require their supporting directories. Cloudflare's Durable Objects skill
references `references/rules.md`, `references/testing.md` and
`references/workers.md`, outside this repository's `resources/auto` and
`resources/manual` convention. The CLI recursively copies skill content, retains
file permissions, dereferences symlinks and skips certain files/directories
including `metadata.json` and `.git`. It can skip broken symlinks. Therefore CLI
installation preserves useful skill support files but is not a byte-exact
repository mirror. [Cloudflare skill](https://github.com/cloudflare/skills/blob/86505e44f87d801aa1864a3c4109e9a8ae5c409f/skills/durable-objects/SKILL.md),
[Installer implementation](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/src/installer.ts).

Cloudflare's repository carries Apache-2.0. Section 4 requires supplying the
license, retaining applicable notices, marking modified files and preserving
applicable NOTICE attribution when present. Proposed consequence: retain vendor
content unchanged, record provenance separately, and ensure license/notice files
are distributed with each independently installed skill. Merely keeping the
upstream root license elsewhere in this repository may not accompany a selected
skill during downstream installation. [Cloudflare license](https://github.com/cloudflare/skills/blob/86505e44f87d801aa1864a3c4109e9a8ae5c409f/LICENSE).

## Architecture options

The key product decision is whether this repository republishes vendor skills or
only declares which upstream skills each machine should install.

| Option                  | Repository contents                                         | Downstream installation                                                                    | Main cost                                                              |
| ----------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------- |
| Declarative CLI wrapper | Manifest and installation records                           | Consumers must run `ask`; existing `npx` installs do not transitively install dependencies | Node/CLI dependency, agent placement and lock ownership                |
| Committed snapshots     | Manifest, resolved lock and complete selected skill folders | Compatible when generated folders fit upstream discovery rules                             | Repository size, content PRs, license handling and vendor-aware checks |
| Git submodules          | Manifest or `.gitmodules` plus Git pointers                 | Requires an explicit recursive checkout or materialization step                            | Checkout complexity and downstream `npx` incompatibility               |

Provisional recommendation: committed snapshots fit a single repository intended
to remain installable through `npx skills add MAHDTech/agent-skills`. A wrapper is
simpler if only this user's machines need third-party installation. The user
must choose this boundary before a backend or schema is finalized.

For snapshots, candidate design responsibilities are a YAML manifest describing
source/ref/skill selection, an `ask` lock recording the resolved commit and
selected source paths, and generated vendor folders discoverable by downstream
installers. A mutable tracking ref and immutable resolved commit should be
separate fields. Import complete skill directories without rewriting vendor
frontmatter, references or punctuation. Repository-specific authoring checks
need an explicit vendor policy; acquisition checks should still validate paths,
required files and collisions. Decide separately whether all newly added upstream
skills enter automatically, what happens to removals, and whether manual vendor
edits block replacement. These are proposals, not current behavior.

## Automated update pull requests

Dependabot supports `gitsubmodule`, but its published ecosystem list has no
native `ask` YAML manager. Renovate has an opt-in submodule manager and a regex
custom manager that can extract custom dependency declarations; its `git-refs`
datasource can track Git refs. A custom manifest integration would still need to
materialize changed snapshots and validate them, rather than only edit a
revision string. [Dependabot ecosystems](https://docs.github.com/en/code-security/reference/supply-chain-security/supported-ecosystems-and-repositories),
[Renovate submodules](https://docs.renovatebot.com/modules/manager/git-submodules/),
[Renovate custom manager](https://docs.renovatebot.com/modules/manager/regex/),
[Renovate Git refs](https://docs.renovatebot.com/modules/datasource/git-refs/).

A scheduled GitHub Actions workflow is the most direct candidate: resolve
tracking refs, update only vendor locks/content, run the configured checks and
create or update a stable PR branch. The existing
[devenv update workflow](../../.github/workflows/devenv-update.yaml) supplies a
repository precedent. The `create-pull-request` action supports maintaining an
existing PR and restricting committed paths. Current GitHub documentation says
PR events created with `GITHUB_TOKEN` can start approval-required workflow runs;
a GitHub App token permits automatic runs without that prompt. Prefer the
repository's existing authentication pattern when implementing.
[Pull request action](https://github.com/peter-evans/create-pull-request),
[GitHub workflow triggering](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/trigger-a-workflow).

The options above record the initial research. Subsequent design decisions chose
committed snapshots without a separate lockfile, using system Git from Rust.
The implementation plan owns those decisions; lockfile suggestions above are
historical alternatives.

## Initial vendor verification

Cloudflare's default branch is `main`, with skills under `skills/`. Its
repository license is Apache-2.0, also declared in its plugin metadata.
The `cloudflare` skill links to sibling folders such as
`../nextjs-on-cloudflare/SKILL.md`, `../gateway-workers/SKILL.md` and
`../workers-profiling/SKILL.md`, and to many bundled `references/` files.
Keep sibling skill folders together and retain complete skill directories.
The `nextjs-on-cloudflare` skill instructs agents to obtain additional skills
from `cloudflare/vinext`; this is an instruction in the imported content, not a
dependency automatically resolved by the downloader.
[Cloudflare repository](https://github.com/cloudflare/skills),
[Plugin metadata](https://github.com/cloudflare/skills/blob/main/.claude-plugin/plugin.json),
[Cloudflare skill](https://github.com/cloudflare/skills/blob/main/skills/cloudflare/SKILL.md),
[Next.js skill](https://github.com/cloudflare/skills/blob/main/skills/nextjs-on-cloudflare/SKILL.md).

SpacetimeDB's default branch is `master`; the source repository is
`https://github.com/clockworklabs/SpacetimeDB`, and `skills` is a subtree rather
than a separate Git repository. Its plugin manifest declares 11 skill folders.
Each listed skill's frontmatter name matches its folder name, including the
generic name `mcp`, which initially collided with this repository's custom MCP
skill. The user approved renaming the custom skill to `model-context-protocol`,
preserving the vendor name and unique catalog names.
[SpacetimeDB repository](https://github.com/clockworklabs/SpacetimeDB),
[Declared skill paths](https://github.com/clockworklabs/SpacetimeDB/blob/master/.claude-plugin/marketplace.json).

| Folder and frontmatter name | Source                                                                                              |
| --------------------------- | --------------------------------------------------------------------------------------------------- |
| `cli`                       | [Skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/cli/SKILL.md)               |
| `concepts`                  | [Skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/concepts/SKILL.md)          |
| `cpp-server`                | [Skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/cpp-server/SKILL.md)        |
| `csharp-client`             | [Skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/csharp-client/SKILL.md)     |
| `csharp-server`             | [Skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/csharp-server/SKILL.md)     |
| `mcp`                       | [Skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/mcp/SKILL.md)               |
| `rust-server`               | [Skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/rust-server/SKILL.md)       |
| `typescript-client`         | [Skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/typescript-client/SKILL.md) |
| `typescript-server`         | [Skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/typescript-server/SKILL.md) |
| `unity`                     | [Skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/unity/SKILL.md)             |
| `unreal`                    | [Skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/unreal/SKILL.md)            |

All 11 SpacetimeDB skill files declare `license: Apache-2.0`. Its root
`LICENSE.txt` instead declares Business Source License 1.1 with provisions for
subdirectory-specific licenses. Direct checks for `skills/LICENSE`,
`skills/LICENSE.txt`, `skills/cli/LICENSE` and `skills/cli/LICENSE.txt` returned 404. These checks do not prove the absence of every possible nested license.
Preserve explicit skill licensing and inspect the acquired tree before choosing
which license text to distribute; applying the root BSL label to all imported
skills would contradict their own metadata. The MCP skill names the sibling
`cli` and `concepts` skills in its prose, and Unity and Unreal similarly refer
to the server skills, so renaming requires care beyond directory names.
[Root license](https://github.com/clockworklabs/SpacetimeDB/blob/master/LICENSE.txt),
[MCP skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/mcp/SKILL.md),
[Unity skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/unity/SKILL.md),
[Unreal skill](https://github.com/clockworklabs/SpacetimeDB/blob/master/skills/unreal/SKILL.md).

The proposed daily update PR must preserve the last successful snapshots from
its open branch when a later acquisition fails. Starting each run solely from
the default branch and retaining its copies would discard successful updates
already awaiting review. Restore the existing PR's vendor snapshots before
refreshing vendors, or otherwise carry forward successful unmerged results.
This is a workflow design implication of partial-failure retention, not a
behavior supplied by the upstream skill installer.
