---
name: deprecate-skill
description: Retire a skill cleanly - move its directory into skills/deprecated/, add a note pointing to its replacement, purge or redirect every inbound /skill-name reference (the router included), then re-run lint and sync. Use when you want to deprecate, retire, remove, replace, or merge away a skill, or a /skill-audit flagged one for retirement. Covers when to deprecate vs delete vs merge and how to avoid dangling references. Cross-references /skill-creator and /skill-router.
---

# Deprecate Skill

Retire a skill without leaving a **dangling reference** behind. Retiring de-promotes it - a skill in the `deprecated/` **lifecycle bucket** drops out of the generated README, `agents/AGENTS.md`, dashboard, and `skills.sh.json` on the next sync - so the real work is repointing everything that named it.

If a replacement is taking over, create it first with `/skill-creator`; deprecate only once the replacement exists.

## Pick the disposition first

Three ways a skill leaves the live set:

- **Deprecate** - retire but keep it readable (it had users, or its history is worth preserving). `git mv` into `skills/deprecated/`. The default.
- **Delete** - remove it entirely (a never-promoted `in-progress/` draft, a mistake, nothing worth keeping). `git rm` the directory. Not for anything that ever shipped.
- **Merge** - its value belongs inside another skill. Fold the content into the survivor, redirect references to the survivor, then deprecate or delete the emptied shell.

## Steps (deprecate)

1. **Move it.** `git mv skills/<category>/<name> skills/deprecated/<name>`. The directory basename is unchanged, so `name` still equals it and lint stays green. The move into the deprecated bucket is what de-promotes it.
2. **Add a deprecation note** at the top of its `SKILL.md` - one blockquote naming the replacement and the reason, e.g. `> Deprecated - superseded by /new-skill. Kept for reference.` Keep the frontmatter valid: lint covers deprecated skills too, so `name`, `description`, and placement must still pass.
3. **Purge or redirect inbound references.** Sweep the live tree for the old name and fix each hit - repoint to the replacement, or remove the mention:

   ```bash
   grep -rn '/<name>' skills --include=SKILL.md
   ```

   - **Router** (`skills/authoring/skill-router/SKILL.md`) - remove the retired skill's entry. If a replacement took over its slot, the replacement's entry already covers it.
   - **Any other skill** that referenced `/<name>` - a mention now pointing at a deprecated skill is a dangling reference; send it to the replacement or drop it.

4. **Regenerate the derived artifacts.** `devenv --no-tui shell -- skills --action lint`, then `devenv --no-tui shell -- skills --action sync`. Sync rebuilds the README, `agents/AGENTS.md`, `skills.sh.json`, and the dashboard from the tree and stages them; commit what it changes.

## Avoiding dangling references

The router and cross-skill `/skill-name` mentions are the two places a retired name lingers. The grep in step 3 is the sweep; the check is the completion bound below - no _promoted_ skill may still name the retired one. Deprecated skills may reference each other freely, but nothing live points at what you just retired. `/skill-audit` runs the same sweep across the whole collection if you want to catch strays later.

## Done when

The skill lives under `skills/deprecated/`, carries a note pointing to its replacement, no promoted skill (the router included) still references `/<name>`, lint passes, and a second `sync` leaves no further diff.
