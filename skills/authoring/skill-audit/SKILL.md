---
name: skill-audit
description: Periodically audit the whole skill collection for health — validate each skill's frontmatter, clarity, and category, verify every cross-reference resolves, and surface duplicates, conflicts, retirement candidates, and missing-skill gaps. Use when you want to audit or health-check the skills, spring-clean the collection, confirm the router and cross-references are accurate, or find skills to merge, split, retire, or create. Hands findings to /skill-creator to fix or create and /deprecate-skill to retire.
---

# Skill Audit

A periodic health sweep of the whole `skills/` tree — run it on a cadence, not per change. Validating one skill you just wrote is `/skill-creator`'s job; this walks the _collection_ and judges it as a set. The output is a **findings list**, not silent edits: audit, report, then route each finding to the skill that fixes it.

The tree is the **single source of truth**; the generated README, `agents/AGENTS.md`, dashboard, and `skills.sh.json` are downstream of it.

## Before you start

Two mechanical gates come first — they are cheap and catch drift a read-through would miss:

1. `devenv shell -- skills --action lint` — frontmatter, naming, placement. Record every error; these are findings.
2. `devenv shell -- skills --action sync`, then check `git status`. A diff means the generated artifacts had drifted from the tree — the drift is itself a finding.

## The audit

Walk the whole collection. Each pass carries an exhaustive bound — every skill, every reference, no sampling.

1. **Inventory.** List every promoted skill — one per `skills/<category>/<name>/`, excluding the `in-progress/` and `deprecated/` **lifecycle buckets** — grouped by category. This inventory is what the rest of the audit checks against.
2. **Per-skill review.** Apply the checklist below to _every_ skill in the inventory.
3. **Cross-reference integrity.** Collect every `/skill-name` mentioned in any `SKILL.md` and confirm each resolves to a promoted skill of that name. A reference to a deprecated or nonexistent skill is a **dangling reference** — record it. Then confirm the `/skill-router` index names every promoted skill exactly once, and names nothing deprecated or gone.

   ```bash
   # promoted skill names (the inventory)
   ls -d skills/*/*/ | grep -vE '/(in-progress|deprecated)/' | xargs -n1 basename
   # every /skill-name reference to reconcile against it
   grep -rhoE '/[a-z][a-z0-9]+(-[a-z0-9]+)*' skills --include=SKILL.md | sort -u
   ```

4. **Collection-level review.** Across the whole set, look for:
   - **Duplication or conflict** — two skills covering the same **branch**, or giving contradictory guidance. Recommend a merge or a sharper boundary between them.
   - **Retirement candidates** — a skill superseded, stale, or that no realistic task would reach. Route to `/deprecate-skill`.
   - **Gaps** — a recurring task with no skill to serve it. Route to `/skill-creator`.
5. **Report.** Emit one findings list. Each finding names the skill(s) it touches, the problem, and a recommended action — edit or create via `/skill-creator`, retire via `/deprecate-skill`, fix a reference in place.

## Per-skill checklist (reference)

For each skill in the inventory:

- **Frontmatter valid** — `name` equals the directory, kebab-case, prefix-free, ≤64 chars, no "claude"/"anthropic"; `description` is one line, ≤1024 chars, stating what it does _and_ when to use it. Canonical keys only — no legacy `custom:`, `triggers:`, `category:`, or `type:`.
- **Categorised right** — sits in the topic bucket matching what it does, and is not stranded in `in-progress/` or `deprecated/` while still live.
- **Description earns its load** — one trigger per branch, leading word front-loaded, no identity already stated in the body.
- **Body is lean** — no **sediment** (stale lines), no **sprawl** (simply too long), no **duplication** (one meaning, one place), no **no-op** (default behaviour restated), no **negation** (steer by the positive target).
- **References resolve** — its `/skill-name` mentions and sibling-file pointers all hit live targets.

## Done when

Every skill in the inventory has been through the checklist, every `/skill-name` reference has been resolved to a live skill or recorded as dangling, the router has been checked against the inventory, and the findings list names each problem with a recommended action and the skill it belongs to.
