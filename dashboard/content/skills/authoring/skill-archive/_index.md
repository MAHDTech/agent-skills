+++
title = "skill-archive"
description = "Retire a custom skill by archiving its files, preserving history, updating inbound references, and regenerating the catalog."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "authoring"
mermaid = false
+++


# Skill Archive

**Invocation:** Start only when the user explicitly requests this workflow.

Retire a skill without deleting it and without leaving a **dangling reference** behind. The collection has two trees with the same shape:

```text
skills/<category>/<name>/SKILL.md            in use: installed, listed, routed to
skills-archive/<category>/<name>/SKILL.md    retired: readable on the dashboard, never installed
```

Moving a skill across is what retires it. Nothing under `skills-archive/` is installed by `ask skills install`, offered by `npx skills add MAHDTech/agent-skills`, or listed in the README, `agents/AGENTS.md`, or `skills.sh.json`. It stays on the dashboard, marked as archived, so consumers can still read it and fetch it from the repository by hand. A `sync` after the move removes any local links the skill had.

If a replacement is taking over, create it first with `/skill-creator`; archive only once the replacement exists.

## Pick the disposition first

Three ways a skill leaves the live set:

- **Archive** - retire but keep it readable (it shipped, it had users, or its history is worth preserving). `git mv` into `skills-archive/`. The default: shipped skills are never deleted.
- **Delete** - remove it entirely (a never-promoted `in-progress/` draft, a mistake, nothing worth keeping). `git rm` the directory. Not for anything that ever shipped.
- **Merge** - its value belongs inside another skill. Fold the content into the survivor, redirect references to the survivor, then archive or delete the emptied shell.
- **Vendor removal** - a skill under `skills/vendor/<vendor>/` is never archived. Remove its source entry from `vendor-skills.yaml` and `git rm -r skills/vendor/<vendor>` (to drop one skill but keep the vendor, add its exact name to that source's `excludes:` and `git rm -r skills/vendor/<vendor>/<skill>`); Git keeps the history.

## Steps (archive)

1. **Move it, keeping the category.** The archive mirrors the live tree, so the skill lands under the same category directory it came from:

   ```bash
   mkdir -p skills-archive/<category>
   git mv skills/<category>/<name> skills-archive/<category>/<name>
   ```

   The directory basename is unchanged, so `name` still equals it and lint stays green.

   Relative links from the moved skill to live skills now resolve inside `skills-archive/`; repoint them through the live tree (`../../../skills/<category>/<name>/SKILL.md`) so the link checker keeps passing.

2. **Mark it archived in frontmatter.** Add a `metadata` block (or extend the existing one) with the archive date and, when there is one, the successor's name. Both values are strings:

   ```yaml
   metadata:
     archived: "2026-09-04"
     replaced-by: "new-skill"
   ```

   `archived` is required on every skill under `skills-archive/`; `replaced-by` is optional and names a live skill. The dashboard renders its archived banner from these keys, so the body needs no note. Keep the rest of the frontmatter valid: `name` and `description` must still pass lint.

3. **Purge or redirect inbound references.** Sweep for the old name in skills and in repository docs, then fix each hit - repoint to the replacement, or remove the mention:

   ```bash
   rg -n -w '<name>' skills --glob 'SKILL.md' --glob '!skills/vendor/**'
   rg -n 'skills/<category>/<name>/' docs README.md CONTRIBUTING.md AGENTS.md
   ```

   - **Router** (`skills/authoring/skill-router/SKILL.md`) - remove the retired skill's entry. If a replacement took over its slot, the replacement's entry already covers it.
   - **Any other live skill** that names it - a `/<name>` mention, a relative link, or prose such as "use the `<name>` skill" - now points at an archived skill. Send it to the replacement or drop it. Skills are self-contained by default, so dropping the sentence is usually right.
   - **Repository docs** (audits, plans, ADRs) that link `skills/<category>/<name>/` - repoint the link to `skills-archive/<category>/<name>/` so the link checker passes; leave the historical text as written.

4. **Regenerate the derived artifacts.** `devenv --no-tui shell -- ask skills lint`, then `devenv --no-tui shell -- ask skills sync`. Sync rebuilds the README, `agents/AGENTS.md`, `skills.sh.json`, and the dashboard from the tree, and removes the local links for the archived skill.

5. **Verify.** `git add` the moved skill, the regenerated artifacts, and any repointed docs, then run `devenv --no-tui test`. The link checker and dashboard lint fail on any link or page you missed.

## Avoiding dangling references

The router and cross-skill `/skill-name` mentions are the two places a retired name lingers. The grep in step 3 is the sweep; the check is the completion bound below - no _live_ skill may still name the archived one. Archived skills may reference each other and may reference live skills freely, but nothing live points at what you just archived. `/skill-audit` runs the same sweep across the whole collection if you want to catch strays later.

## Restoring

An archived skill comes back the same way it left: `git mv skills-archive/<category>/<name> skills/<category>/<name>`, delete the `archived` and `replaced-by` keys, re-add its router entry, then lint and sync.

## Done when

The skill lives under `skills-archive/<category>/<name>/`, its frontmatter carries `metadata.archived` (and `replaced-by` when a successor exists), no live skill (the router included) and no repository doc still points at its old path, `devenv --no-tui test` passes, and a second `sync` leaves no further diff.

