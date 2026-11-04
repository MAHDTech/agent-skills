# Contributing to Agent Skills

<!-- cspell:words alnum versionrc -->

Contributions to the multi-agent skills catalog and the `ask` CLI tooling are welcome.
Read [README.md](./README.md) for the project overview, [docs/install.md](./docs/install.md) for installation details, and [AGENTS.md](./AGENTS.md) for the canonical repository rules.

## Branching

- Target the `trunk` branch for all pull requests and dependency updates, as configured in [.github/dependabot.yml](./.github/dependabot.yml) and [.github/workflows/ci.yaml](./.github/workflows/ci.yaml).
- Create topic branches from `trunk` using a prefix that matches the change type:
  - `feat/<topic>` for new skills, CLI features, or dashboard capabilities.
  - `fix/<topic>` for bug fixes in skills, crates, or scripts.
  - `docs/<topic>` for documentation updates.
  - `chore/<topic>` for maintenance, dependency updates, or configuration tasks.

## Commit Messages and Pull Request Titles

- Format both commit messages and pull request titles using Conventional Commits: `type(scope): description`.
- Omit `(scope)` when a change spans multiple areas (`type: description`), and append `!` before the colon (`type(scope)!: description` or `type!: description`) for breaking changes.
- Use only the lowercase commit types defined in [.versionrc](./.versionrc):
  - `feat`: new features or skills (included in the changelog under Features).
  - `fix`: bug fixes (included in the changelog under Fixes).
  - `build`: build system or external dependency changes.
  - `chore`: routine maintenance and housekeeping.
  - `ci`: continuous integration workflow or script changes.
  - `docs`: documentation-only changes.
  - `style`: formatting or whitespace changes that do not alter behaviour.
  - `refactor`: code restructuring that neither fixes a bug nor adds a feature.
  - `perf`: performance improvements.
  - `test`: adding or updating tests.
- Match scopes against the regular expression `[[:alnum:]]+(?:[-_/][[:alnum:]]+)*` and keep commit header lines within the 80-character limit defined in [.versionrc](./.versionrc).
- Commit messages are validated automatically by the `convco` and `commitizen` git hooks configured in [devenv.nix](./devenv.nix).

```text
feat(skills): add cargo-doc verification skill
fix(ask-cli): handle missing resources directory during sync
docs: add CONTRIBUTING.md guide
chore(deps): update rust crate dependencies
```

## Developer Environment and Workflow

- This repository uses `devenv` ([devenv.nix](./devenv.nix)) to provide the Rust toolchain, the `ask` CLI wrappers, and git hooks.
- Run commands inside the developer environment with `devenv --no-tui shell -- <command>`, always passing `--no-tui` in automated or AI agent environments to disable the interactive terminal interface.
- Run the workspace test suite using `devenv --no-tui test` or [./scripts/run-tests.sh](./scripts/run-tests.sh), which executes `cargo nextest run --workspace --all-targets --locked` and `cargo test --workspace --doc --locked`.
- Run all configured git hooks and linters across the repository using `devenv --no-tui shell -- prek run -a` (or `devenv --no-tui shell -- prek run --all-files`).
- Never run individual linters directly outside `prek` (see [skills/tooling/prek/SKILL.md](./skills/tooling/prek/SKILL.md)), and never use or add the deprecated standalone `pre-commit` CLI binary instead of `prek`.

```bash
devenv --no-tui shell -- prek run -a
devenv --no-tui test
```

## Working with Skills and the Dashboard

- Live skills reside under `skills/<category>/<name>/SKILL.md` (with `skills/in-progress/` for drafts and `skills/vendor/<vendor>/<skill>/` for upstream snapshots declared in [vendor-skills.yaml](./vendor-skills.yaml)), while retired skills move to `skills-archive/<category>/<name>/`.
- Use the `ask` CLI (`cargo run -p ask-cli --` or `ask` inside `devenv`) to validate and synchronize skills and dashboard assets:
  - `ask skills lint`: validate skill frontmatter, directory naming, resource layout, and cross-reference rules.
  - `ask skills sync`: synchronize local skill links and generated catalog artifacts.
  - `ask dashboard lint`: verify committed dashboard content matches generated artifacts.
  - `ask dashboard build`: sync content, compile Tailwind CSS, render the Zola site, and build the Pagefind search index.
  - `ask dashboard serve`: build the dashboard once and serve it locally with live reload.
- Keep skills self-contained: cross-references between skills are forbidden unless both skills declare membership in the same `metadata.group` (`authoring`, `github`, `planning-pipeline`, `review`, `opencode`, or `guards`), with `skill-router` and `skills-archive/` exempt only as sources and live skills never referencing archived skills.
- Declare `policy.allow_implicit_invocation` explicitly in `agents/openai.yaml` for every promoted custom skill so that it matches the inverse of `disable-model-invocation` in `SKILL.md` (where omitting `disable-model-invocation` means `false`).
- Place skill supporting files under `resources/auto/` (managed by `ask skills download-resources` and `ask skills clean-resources`) or `resources/manual/` (hand-authored), and never place files directly inside `resources/`.

## Pull Request Review and CI Checks

- Every pull request targeting `trunk` runs the CI validation workflow in [.github/workflows/ci.yaml](./.github/workflows/ci.yaml), which executes `prek run --all-files` and `./scripts/run-tests.sh`.
- Use relative repository paths for all internal links (for example, `./AGENTS.md`) and never include local `file://` URLs.
- Never use em-dashes (Unicode U+2014) in Markdown files, code comments, commit messages, or pull request descriptions; use standard hyphens (`-`), colons, commas, parentheses, or separate sentences instead.
- Comply with `markdownlint` rules in all Markdown files:
  - Always specify an explicit language tag on fenced code blocks (`MD040`), such as `bash`, `text`, `yaml`, `json`, or `rust`.
  - Always surround fenced code blocks with blank lines (`MD031`), including when nested inside lists.
- Ensure all changes pass `cspell` (checked against the `en-gb` dictionary and [project-words.txt](./project-words.txt)), `lychee`, `prettier`, and `editorconfig-checker` via `prek` before requesting review.
