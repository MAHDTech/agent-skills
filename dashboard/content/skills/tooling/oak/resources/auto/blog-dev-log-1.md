+++
title = "blog-dev-log-1"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: Notes from building Oak — version control for agents. Branch-per-session workflows, team and project scoping, and lazy mounts for agent-driven development.
title: Oak dev blog
image: https://oak.space/social-card-217ab603.png
---

Blog

Blog Dev log #2: August to October Dev log #1: six weeks since launch Git is forever. I'm building Oak anyways.

Changes from August 4 to October 6, 2026 across [oak/oak](https://oak.space/oak/oak) and [oak/oakspace](https://oak.space/oak/oakspace). About 115 merged branches. CLI v0.102.0 → v0.105.0. Previous: [Dev log #1](https://oak.space/blog#dev-log-1).

## Pricing

- Paid plans and their limits moved behind a feature flag. Every account gets 100 GB during the beta.
- Docs now include an example session, answers about the CI runner, and a price ballpark.

## Data safety fixes

- **Merge base on partial history (0.104.0).** On a shallow or partially backfilled commit graph, 0.103.0 could pick a too-old merge base, so `oak pull` could re-apply a change main had reverted and merge previews could predict against the wrong base. A merge base computed on a partial graph is now either correct or reported unavailable ( `IncompleteAncestry`, exit 5).
- **`oak pull` never drops local-only commits (0.105.0).** Against a local `oak serve`, push → commit → pull could reset the branch and discard the unpushed commit. Pull now checks that the new head contains the local tip before writing anything, and otherwise keeps, re-parents, or enters the conflict flow. `--force` parks the old tip as `<branch>.orphaned-\<ts\>`. Hosted oak.space was not affected.
- **`oak close --remote`** no longer resets a local branch that is ahead of the remote.
- **Hosted merge** uses compare-and-swap on the resolved heads, so a concurrent merge or a push during a merge can’t lose acknowledged work.
- **Zero-byte files (0.102.1).** Repos containing empty files are clonable again, and the server no longer silently omits blobs it can’t ship.
- Diff patches keep `\ No newline at end of file` markers and CRLF bytes so they apply exactly.

## Content integrity

- Clone runs a bounded integrity proof before transfer: header-only history traversal with per-request budgets, one pinned branch-head snapshot for the whole clone, and `412` if the branch moves mid-clone.
- If the proof runs out of budget, clone reports *inconclusive* (exit 8, `integrity_inconclusive`) instead of missing content, and suggests `--shallow`. `oak doctor --json` reports `verified` / `failed` / `inconclusive`.
- Admission is rate-limited per principal, repository and profile; `429`s carry `Retry-After` and the client retries within 60 s.
- Publication walks the complete commit graph; the server rejects commits whose parent or merge parent is absent.
- Blob ingest and publication were hardened end to end.

## Security

- A checkout’s repository API key is bound to the server that issued it and is never sent to another remote. Previously, `oak push -r \<other\>` or `OAK_REMOTE=\<other\>` sent it there. If you did that, rotate the repository’s key.
- Both installers ( `install.sh`, `install.ps1`) verify Minisign signatures on the release and the mounter before replacing the existing binary, using a pinned Minisign bootstrap.
- Fixed a stored XSS in URL-backed 3D previews.
- Centralized security headers ( `base-uri`, nosniff, framing, referrer, HSTS) on every HTML response.
- Anonymous web feedback removed; the web form requires login. CLI feedback is unchanged.
- Protected production deploys: signed grants, protected secrets kept out of command strings, least-privilege AWS boundary, signed crash-recoverable bootstrap.
- The `oak_token` session cookie now persists for the token’s lifetime instead of the browser session.

## Review without a checkout

- `oak branch review|diff|triage --remote` pins to immutable source and target heads and fetches only the content it needs, without touching local refs.
- Remote branch diffs include hunks and per-file omission reasons; remote file reads are supported.
- Remote log: `oak log --remote --json` walks a branch’s history without fetching it.
- Whole-branch JSON contribution review.
- Preview merging several independent sibling branches in order, with conflict/incomplete/stale outcomes.
- Review distinguishes unavailable bytes from binary or large files and rejects unknown comparison targets.
- Review won’t recommend closing a branch that has uncommitted work in the current checkout.
- `merge_allowed` in review and triage is based on CI for the exact head.

## Clone and sync

- `oak clone --branch NAME --expected-head FULL` clones one branch at an exact head.
- `oak clone --detached` lands on the default branch without creating a personal branch.
- `oak clone --from` seeds a clone from a local checkout.
- Sparse clones negotiate cone-aware blob withholding and keep full identities for out-of-cone files.
- `oak switch -c` in a fresh shallow clone skips the ancestry walk when main hasn’t moved (about 176 s → under 1 s on oak/oak).
- Equal-tree refreshes leave unchanged files in place, keeping compiler caches warm.
- Local `oak serve` honors `depth` on `GET /pull`.
- New flags: `oak pull --json`, `oak pull --branch-only`, `oak desc --append`, `oak push --plan --json`.

## CI

- `oak ci trigger` for an exact commit with an idempotency key.
- Exact-run cancellation; `oak ci cancel --superseded`.
- `oak ci wait --current`, opt-in progress events, compact summaries without logs, and `run_url` in JSON.
- CI step timings recorded by the server.
- Build cache kept inside the runner’s disk, sandbox SDK pinned to its image, and CLI CI builds tuned to avoid disk exhaustion.

## Agent-facing CLI

- JSON outputs carry receipts with explicit identity and confirmed/uncertain outcomes for push, merge, commit, finish, clone and CI.
- Structured `oak merge` output stays parseable during post-merge refresh (progress goes to stderr).
- `oak change capture --json` records an immutable local change; `oak change export` writes it to a verified self-contained archive.
- `oak file inspect --at HEAD|HASH --json PATH` for pinned file evidence.
- `oak space inventory` lists local checkouts and mounts; fenced close of many branches at once.
- `oak diff`: git-compatible add/delete headers, `--check`, `--name-only` with `--print`.
- `oak restore` accepts literal `HEAD`.
- `oak commit` on a detached HEAD refuses.
- The log and diff viewers support less and emacs keys.
- Credential lock no longer steals from a live owner; concurrent logins don’t lose credentials.
- `oak finish --json` reports `description_synced` truthfully.

## Feedback

- Ten-state feedback workflow, spam-safe exports, and automatic linking/closure from merged branch descriptions that reference `fb-N`.
- Edits are atomic with revisions and an audit trail.
- Admin CLI: `feedback list/show`, conditional transitions, manual link/unlink.
- Web feedback preserves drafts across auth, rate-limit and network errors and returns accurate `Retry-After`.

## Removed

- The “software factory” orchestration layer and its `factory` flag.

## Other

- `@public` path principal: a private repo can open-source part of its tree.
- Homepage rewritten around three differentiators: mount instead of clone, messageless commits, large files without LFS.
- Public docs and README updated to match what ships (agent skill, CI, shell completion, feedback).
- Yanked transitive dependency ( `multer`) removed from release builds.

[Claim your username](https://oak.space/signup)

### get updates by email

Roughly monthly updates about development and cool things.

Subscribe

or join [discord](https://discord.gg/UUPfUaeDnS) · follow on [x.com](https://x.com/oakdotspace)

Changes from June 22 to August 3, 2026 across [oak/oak](https://oak.space/oak/oak) (core and CLI) and [oak/oakspace](https://oak.space/oak/oakspace) (the oak.space server). About 190 commits. CLI v0.99 → v0.102.

## Windows

- Windows x86\_64 build. Install from PowerShell: `irm https://oak.space/install.ps1 | iex`. Installs `oak.exe` to `%USERPROFILE%\.local\bin` and adds it to `PATH`.
- Clone, push, pull, commit, branch and merge work.
- `oak mount` works on Windows via the [Projected File System](https://learn.microsoft.com/en-us/windows/win32/projfs/projected-file-system) (an optional Windows feature, enabled once per machine).
- The Unix installer serves native Intel-macOS binaries instead of ARM builds under Rosetta.
- Supported: macOS (Apple Silicon, Intel), Linux x86\_64, Windows x86\_64. Linux ARM64 binaries are published but not yet picked by the installer.

## Native CI

Workflows in `.oak/workflows/*.yml` are parsed, scheduled and logged by the server; jobs run in Cloudflare Sandboxes driven by a Durable Object. oakspace’s own build and deploy moved off GitHub Actions onto it on July 6.

- Per-repo secrets, encrypted at rest (ChaCha20-Poly1305), plus `env:` maps at workflow and job level.
- Dispatch is acked immediately; a `RunCoordinator` Durable Object drives jobs from an alarm loop so runs can’t get stuck in “running”.
- Live logs over SSE for every lifecycle event. ANSI colors render.
- Cancellation of in-flight runs from the UI or API.
- Build caching: the sandbox image includes the pinned Rust toolchain, bun, nextest and zstd; workflows get a per-job `cache:` block backed by R2, uploaded in streamed parts.
- Per-plan quotas: metered minutes, concurrency limits with queueing, run-duration caps.
- Merge gate: squash merges are blocked on red CI, shown on the branch slide-over, with an explicit force path.
- CLI: `oak ci runs`, `status`, `logs`, `rerun`, and `oak merge --wait`.
- The dedicated CI page was removed. Run status is a chip on each branch row and merged-commit row.
- Fixed multi-line CI secrets (e.g. SSH keys) being corrupted. Cause: Cloudflare’s Sandbox SDK indents every line of multi-line env values. Worked around and reported upstream.

## `oak feedback`

- New command. Accepts `-m`, a file, or stdin; opens `$EDITOR` with no input; works without a TTY. Returns a reference `fb-N`.
- Storage moved from Cloudflare D1 to the main Postgres. Added a Discord outbox, admin triage view, update API (so agents can triage their own reports), and a spam status.
- Over a hundred items filed so far, mostly by agents. Many commits in this period reference the `fb-N` they close.

## Path permissions in the repo

Declared in `.oak/PERMISSIONS` (CODEOWNERS-shaped), always read from the `main` tip:

```
infra/secrets/**   @zdgeier @group/sec-team
scripts/           @group/ops
scripts/public/    *
vault/
```

- Narrow a subtree to specific people or groups while the rest of the repo stays open.
- Widen a subtree with `@public` so a private repo can publish an open-source part.
- Grants can name org-level groups.
- The CLI reports unreadable paths as restricted, not missing.
- Moved from database tables to a repo file. Still behind the `path-permissions` flag.

## File previews and diffs

- New preview/diff support: archives and packages, spreadsheets (workbook grids), media containers (metadata diffs), 3D/CAD (VOX, X3D, 3MF, USD/USDZ; semantic summaries for IFC and STEP), scientific binaries (GRIB, NumPy, FITS, miniSEED), fonts (including visual diffs), markup, subtitles, calendars, contacts, CSS colors, PostScript.
- Audio: bounded WAV/AIFF/CAF PCM decoding with waveform, peak and RMS rendering, before/after diff panels, per-format playback help.
- Bounded chunk readers and byte-range support, so previewing a large file doesn’t download all of it.
- Diffs are syntax-highlighted with word-level change marks.
- Drag files onto the home page to create a repo, or onto a repo’s file list to commit them.

## Hash verification

- Hashed fields of stored commits are immutable and verifiable end to end.
- Verification happens at insert in the storage layer.
- Fixed pull/clone bugs: wire serialization that didn’t round-trip, backfill verifying a merge parent’s ancestors under the wrong branch name, Postgres timestamp truncation changing a hash, and a pull that hung after the branch was squash-merged remotely.
- Branch rename rewrote `commits.branch_name` (a hashed field) without re-hashing, so renamed branches failed verification on clone. The rename endpoint now returns 503. A branch-identity redesign was written, reviewed, and rejected; it’s checked in marked `REJECTED`.

## Mount, review, and CLI

- `oak mount --branch` mounts an existing remote branch. `oak switch` is mount-aware.
- Merge previews detect when a clean merge would also silently revert work on the target (four-tree merge-safety check).
- Shell completion.
- `oak log -S` / `-G` pickaxe search.
- Diff output reworked for agents: progressive disclosure and conflict-aware evidence.
- Content-similarity rename detection in every diff path.
- `oak skill install` installs the Oak agent skill into your agent’s skill directory.

## Releases

- v0.99.0, v0.100.0, v0.101.0, v0.102.0.
- Release publishing rebuilt in three steps: content-keyed reads, staged uploads via `/api/releases/stage`, content-addressed promotion; the legacy release-write endpoint is retired.
- Signing derives its identity from the P12, uses least-privilege tokens, and persists no git credentials.
- CI builds Windows and Intel-macOS targets on every commit.

## Behind a flag

- An agent orchestration layer (“software factory”) behind the `factory` flag, with a web UI for connecting agents, per-agent delivery secrets, and editing path permissions and factory config as a proposed branch.

## Other

- Closed branches can be reopened.
- Branch read receipts: who viewed a branch and when.
- Keyboard navigation for branch lists, file trees and the document pane.
- Per-repo Ideas board, which can be public on a private repo.
- Security: same-origin CSRF guard on every cookie-authenticated mutation; CI endpoints return 404 instead of 403 for private repos.
- Monitoring: growth graphs on the internal dashboard, five new Grafana alert rules, and removal of unused per-ASN metric series (11k of 13k).
- Legal: updated AUP, Terms and Privacy Policy; DMCA designated agent; illegal-content reporting; governing law moved from Delaware to Washington.
- `repo_pages.rs` (98,126 lines) split into 27 submodules.
- Dark-mode contrast fixes.

[Claim your username](https://oak.space/signup)

### get updates by email

Roughly monthly updates about development and cool things.

Subscribe

or join [discord](https://discord.gg/UUPfUaeDnS) · follow on [x.com](https://x.com/oakdotspace)

![The Oak web UI](https://oak.space/git-is-forever-ui.png)

This is a little backstory on why I'm making Oak, if you'd like to read more about the new version control system I've been working on go [here](https://oak.space).

Git is extremely good at what it was built for: humans making deliberate changes, reviewing patches, and preserving a distributed history that can survive decades. It was purpose-built for Linux kernel development and I don’t believe anyone can make a better tool for that. Git is forever.

But even though git is wonderful at many things, I started to wonder years ago if there could be something better. Almost everyone has their own problems with version control. What would a version control system look like if we created it from scratch today?

This started a four year long adventure where I built a version control system in my free-time called Jam, quit my job, and ended up selling it to another company very early on so I could continue working on it full-time.

That company unfortunately ended up shutting down in a little over a year.

However, I did get to see how good version control could feel if we just started over. There’s so much freedom in building on a new foundation. I’ve been thinking about version control systems every day since then.

With AI, I can finally build what I've been dreaming about. Version control systems are really hard to make, and even harder to make well. I’ve done way more in the previous four months than I did in the past four years.

Oak is the version control system I've been working on designed for your agents. It dramatically improves the speed and context your agents need when working on serious projects: 50% fewer VCS-related tokens and 90% faster per operation. With virtual mounts, agents locally and in the cloud no longer need a full copy of a repo to get working. You can work on many tasks in parallel without needing to download everything or fight worktrees. Version control shouldn't waste you or your agents time.

It should be fast, creative and fun to make things with your agents.

Oak is still early in development. There's no Windows build and missing plenty of features. No CI, no issues, no comments. We still use GitHub Actions for building Oak now, but we've been fully bootstrapped on Oak with no Git backup for several months.

It's currently made by [Zach Geier (me)](https://linkedin.com/in/zdgeier), with [Adam Morse](https://www.linkedin.com/in/mrmrs/) shaping the product and visual system. It’s built on an [open-source core and CLI](https://oak.space/oak/oak). You can self host with \`oak serve\` and export your repo to a git repo with \`oak export\` at any time.

And you can try it out now!

I would love to hear your ideas and thoughts. Please, [join the Discord](https://discord.gg/UUPfUaeDnS) if you're interested in following along or want to reach out.

— Zach

[![](https://i.ytimg.com/vi/0qizaR4JGUk/maxresdefault.jpg)](https://www.youtube.com/watch?v=0qizaR4JGUk)

[Claim your username](https://oak.space/signup)

### get updates by email

Roughly monthly updates about development and cool things.

Subscribe

or join [discord](https://discord.gg/UUPfUaeDnS) · follow on [x.com](https://x.com/oakdotspace)

```json
{
      "@context": "https://schema.org",
      "@type": "Blog",
      "name": "Oak dev blog",
      "url": "https://oak.space/blog",
      "description": "Notes from building Oak — version control for agents.",
      "publisher": {
        "@type": "Organization",
        "name": "Oakspace Inc.",
        "logo": { "@type": "ImageObject", "url": "https://oak.space/logo.png" }
      }
    }
```


