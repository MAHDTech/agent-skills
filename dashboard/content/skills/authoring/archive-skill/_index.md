+++
title = "archive-skill"
description = "Retire a skill cleanly - move its directory from skills/<category>/ into skills-archive/<category>/, mark it archived in frontmatter with what replaced it, purge or redirect every inbound /skill-name reference (the router included), then re-run lint and sync. Use when you want to archive, deprecate, retire, remove, replace, or merge away a skill, or a /skill-audit flagged one for retirement. Covers when to archive vs delete vs merge and how to avoid dangling references. Cross-references /skill-creator and /skill-router."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "authoring"
mermaid = false
+++


# Archive Skill

Retire a skill without deleting it and without leaving a **dangling reference** behind. The collection has two trees with the same shape:

```text
skills/<category>/<name>/SKILL.md            in use: installed, listed, routed to
skills-archive/<category>/<name>/SKILL.md    retired: readable on the dashboard, never installed
```

Moving a skill across is what retires it. Nothing under `skills-archive/` is installed by `skills --action install`, offered by `npx skills add MAHDTech/agent-skills`, or listed in the README, `agents/AGENTS.md`, or `skills.sh.json`. It stays on the dashboard, marked as archived, so consumers can still read it and fetch it from the repository by hand. A `sync` after the move removes any local links the skill had.

If a replacement is taking over, create it first with `/skill-creator`; archive only once the replacement exists.

## Pick the disposition first

Three ways a skill leaves the live set:

- **Archive** - retire but keep it readable (it shipped, it had users, or its history is worth preserving). `git mv` into `skills-archive/`. The default: shipped skills are never deleted.
- **Delete** - remove it entirely (a never-promoted `in-progress/` draft, a mistake, nothing worth keeping). `git rm` the directory. Not for anything that ever shipped.
- **Merge** - its value belongs inside another skill. Fold the content into the survivor, redirect references to the survivor, then archive or delete the emptied shell.

## Steps (archive)

1. **Move it, keeping the category.** The archive mirrors the live tree, so the skill lands under the same category directory it came from:

   ```bash
   mkdir -p skills-archive/<category>
   git mv skills/<category>/<name> skills-archive/<category>/<name>
   ```

   The directory basename is unchanged, so `name` still equals it and lint stays green.

2. **Mark it archived in frontmatter.** Add a `metadata` block (or extend the existing one) with the archive date and, when there is one, the successor's name. Both values are strings:

   ```yaml
   metadata:
     archived: "2026-09-04"
     replaced-by: "new-skill"
   ```

   `archived` is required on every skill under `skills-archive/`; `replaced-by` is optional and names a live skill. The dashboard renders its archived banner from these keys, so the body needs no note. Keep the rest of the frontmatter valid: `name` and `description` must still pass lint.

3. **Purge or redirect inbound references.** Sweep the live tree for the old name and fix each hit - repoint to the replacement, or remove the mention:

   ```bash
   grep -rn '/<name>' skills --include=SKILL.md
   ```

   - **Router** (`skills/authoring/skill-router/SKILL.md`) - remove the retired skill's entry. If a replacement took over its slot, the replacement's entry already covers it.
   - **Any other live skill** that referenced `/<name>` - a mention now pointing at an archived skill is a dangling reference; send it to the replacement or drop it. Skills are self-contained by default, so dropping the sentence is usually right.

4. **Regenerate the derived artifacts.** `devenv --no-tui shell -- skills --action lint`, then `devenv --no-tui shell -- skills --action sync`. Sync rebuilds the README, `agents/AGENTS.md`, `skills.sh.json`, and the dashboard from the tree, removes the local links for the archived skill, and stages what it changed; commit that.

## Avoiding dangling references

The router and cross-skill `/skill-name` mentions are the two places a retired name lingers. The grep in step 3 is the sweep; the check is the completion bound below - no _live_ skill may still name the archived one. Archived skills may reference each other and may reference live skills freely, but nothing live points at what you just archived. `/skill-audit` runs the same sweep across the whole collection if you want to catch strays later.

## Restoring

An archived skill comes back the same way it left: `git mv skills-archive/<category>/<name> skills/<category>/<name>`, delete the `archived` and `replaced-by` keys, re-add its router entry, then lint and sync.

## Done when

The skill lives under `skills-archive/<category>/<name>/`, its frontmatter carries `metadata.archived` (and `replaced-by` when a successor exists), no live skill (the router included) still references `/<name>`, lint passes, and a second `sync` leaves no further diff.

