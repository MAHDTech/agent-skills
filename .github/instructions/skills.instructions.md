---
applyTo: "skills/**,skills-archive/**"
---

# Skill Authoring & Review Rules

## Layout & Naming

- Active custom skills live at `skills/<category>/<name>/SKILL.md` under one of the 10 categories (`engineering`, `game-development`, `planning`, `review`, `github`, `reflection`, `writing`, `authoring`, `tooling`, `guards`) or `skills/in-progress/<name>/SKILL.md` for drafts.
- Folder names are prefix-free kebab-case and must exactly match the frontmatter `name`.
- Retired skills live in `skills-archive/<category>/<name>/SKILL.md` with `metadata.archived` (date) and optional `metadata.replaced-by`.

## Frontmatter & Invocation Policy

- `SKILL.md` requires YAML frontmatter with at least `name` and `description`. Do not use legacy `custom:`, `triggers:`, `category:`, or `type:` fields.
- Every promoted custom skill must declare `policy.allow_implicit_invocation` explicitly in `agents/openai.yaml`, matching the inverse of `disable-model-invocation` in `SKILL.md` (omitted means `false`).

## Self-Contained & Cross-Reference Policy

- Cross-references across skills (`/skill-name` invocations or relative Markdown links) are forbidden unless both skills explicitly declare the same `metadata.group`.
- The seven valid groups are `authoring`, `github`, `planning-pipeline`, `review`, `guards`, `nutanix`, and `slop`.
- `skills/authoring/skill-router/` and `skills-archive/` are exempt as cross-reference sources. Live skills must never cross-reference archived skills.

## Resources & Markdown

- Split supporting files under `resources/auto/` (downloader-owned via frontmatter `resources:` URLs) and `resources/manual/` (hand-authored). Never place files directly in `resources/`.
- Never use em-dashes (U+2014) or absolute `file:///` links.
- Every fenced code block must include a language tag (MD040) and be preceded and followed by a blank line (MD031).
- After editing a custom skill, regenerate `dashboard/content/` and `dashboard/static/.well-known/` via `devenv --no-tui shell -- ask dashboard lint --fix` (and `ask skills sync` if name, description, or routing changed).

## Vendor Snapshots (`skills/vendor/**`)

- Files under `skills/vendor/<vendor>/<skill>/` are managed from `vendor-skills.yaml` and overwritten on refresh.
- Do not flag vendor snapshots for local authoring conventions (em-dashes, markdownlint, cspell, resource layout, `agents/openai.yaml`, or cross-skill grouping); only check unique active names and safe import boundaries.
