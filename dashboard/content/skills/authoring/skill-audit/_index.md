+++
title = "skill-audit"
description = "Audit the custom skill catalog for invocation policies, descriptions, frontmatter, references, duplicates, and lifecycle drift."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "authoring"
mermaid = false
+++


# Skill Audit

**Invocation:** Start only when the user explicitly requests this workflow.

A periodic health sweep of the custom `skills/` tree - run it on a cadence, not per change. Validating one skill you just wrote is `/skill-creator`'s job; this walks the _collection_ and judges it as a set. The output is a **findings list**, not silent edits: audit, report, then route each finding to the skill that fixes it.

Vendor snapshots under `skills/vendor/` belong to their upstream authors. Exclude them from description and invocation-policy audits, edits, and local overrides. Archived skills and drafts are inactive and also outside this audit.

The tree is the **single source of truth**; the generated README, `agents/AGENTS.md`, dashboard, and `skills.sh.json` are downstream of it.

## Before you start

Two mechanical gates come first - they are cheap and catch drift a read-through would miss:

1. `devenv --no-tui shell -- ask skills lint` - frontmatter, naming, placement. Record every error; these are findings.
2. `devenv --no-tui shell -- ask skills sync`, then check `git status`. A diff means the generated artifacts had drifted from the tree - the drift is itself a finding.

## The audit

Walk the whole collection. Each pass carries an exhaustive bound - every skill, every reference, no sampling.

1. **Inventory.** List every promoted custom skill - one per `skills/<category>/<name>/`, excluding `vendor/` and the `in-progress/` **lifecycle bucket** (archived skills live outside `skills/`, under `skills-archive/`, and are out of scope) - grouped by category. This inventory is what the rest of the audit checks against.
2. **Per-skill review.** Apply the checklist below to _every_ skill in the inventory.
3. **Cross-reference integrity.** Collect every `/skill-name` mentioned in any `SKILL.md` and confirm each resolves to a promoted skill of that name. A reference to an archived or nonexistent skill is a **dangling reference** - record it. Then confirm the `/skill-router` index names every promoted skill exactly once, and names nothing archived or gone.

   ```bash
   # Promoted custom skill entry points (the inventory).
   rg --files skills -g SKILL.md -g '!skills/vendor/**' -g '!skills/in-progress/**'
   # Slash references are candidates: distinguish commands from paths before reconciling.
   rg -o --no-filename '/[a-z][a-z0-9]+(-[a-z0-9]+)*' skills \
     -g SKILL.md -g '!skills/vendor/**' -g '!skills/in-progress/**' | sort -u
   ```

4. **Collection-level review.** Across the whole set, look for:
   - **Duplication or conflict** - two skills covering the same **branch**, or giving contradictory guidance. Recommend a merge or a sharper boundary between them.
   - **Retirement candidates** - a skill superseded, stale, or that no realistic task would reach. Route to `/skill-archive`.
   - **Gaps** - a recurring task with no skill to serve it. Route to `/skill-creator`.
5. **Report.** Emit one findings list. Each finding names the skill(s) it touches, the problem, and a recommended action - edit or create via `/skill-creator`, retire via `/skill-archive`, fix a reference in place.

## Per-skill checklist (reference)

For each skill in the inventory:

- **Frontmatter valid** - `name` equals the directory, kebab-case, prefix-free, ≤64 chars, no "claude"/"anthropic"; `description` is one line, ≤1024 chars, stating what it does _and_ when to use it. Canonical keys only - no legacy `custom:`, `triggers:`, `category:`, or `type:`.
- **Categorised right** - sits in the topic bucket matching what it does, and is not stranded in `in-progress/` while still live, nor sitting in `skills-archive/` while something still routes to it.
- **Description earns its load** - one or two concise sentences, usually 100-220 characters, stating the task and a useful selection boundary. Keep version histories, command inventories, and synonym lists in the body. Codex `interface.short_description` is a separate UI label, not the discovery description.
- **Invocation is deliberate** - record automatic or explicit-only selection and its rationale for every custom skill. Check required helpers remain reachable from approved parent workflows. Publishing, lifecycle changes, and large orchestration workflows need explicit initiation; bounded references and task helpers can be automatic. Loading a skill grants no additional action authorization.
- **Policies agree** - every promoted custom skill has an explicit boolean `policy.allow_implicit_invocation` in `agents/openai.yaml`, equal to the inverse of `disable-model-invocation` (default `false`). Explicit-only bodies state their user-request boundary for hosts without a verified policy control. Use `/skill-creator`'s compatibility reference when evaluating host-specific behavior.
- **Body is lean** - no **sediment** (stale lines), no **sprawl** (simply too long), no **duplication** (one meaning, one place), no **no-op** (default behaviour restated), no **negation** (steer by the positive target).
- **References resolve** - its `/skill-name` mentions and sibling-file pointers all hit live targets.

## Done when

Every skill in the inventory has been through the checklist, every `/skill-name` reference has been resolved to a live skill or recorded as dangling, the router has been checked against the inventory, and the findings list names each problem with a recommended action and the skill it belongs to.

