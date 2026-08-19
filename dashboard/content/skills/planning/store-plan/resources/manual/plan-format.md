+++
title = "plan-format"
[extra]
skill = false
category = "planning"
mermaid = false
skill_name = "store-plan"
+++

{% raw %}
# Plan Document Format & Example

Reference material for the store-plan skill: the section-by-section document structure, the formatting rules, and a full worked example. Follow this format when generating the plan.

## Document Structure

### Header Block

```markdown
<!-- omit in toc -->

# {Title}

Date: YYYY-MM-DD
Source: conversation with your coding agent

- [Table of Contents entries...]
```

Always include a Table of Contents. Omit the top-level heading from the ToC using `<!-- omit in toc -->`.

### tldr

_Single italicized sentence (max 120 chars) summarizing the most important outcome or decision._

Place immediately after the ToC, before any sections.

### Context

Brief background on what prompted this plan. 2-4 bullets max. Include:

- What problem or goal kicked off the conversation
- Any constraints or deadlines mentioned
- Relevant prior work or plans referenced

### Decisions Made

A list summarizing key decisions from the conversation:

- 🟢 **Decision**: What was decided. **Rationale**: Why this choice was made.
- 🔴 **Decision**: What was rejected. **Rationale**: Why it was not chosen.

> 🟢 Approved · 🟡 Tentative · 🔴 Rejected · ⚪ Deferred

### Plan Overview

High-level summary of phases or work items:

- 🟢 **Phase 1**: `module.py` - What this phase does. **Priority**: P0
- 🟡 **Phase 2**: `client/js/` - What this phase does. **Priority**: P1
- ⚪ **Future**: TBD - Deferred work. **Priority**: P2

> 🟢 Ready to start · 🟡 Needs more detail · 🔴 Blocked · ⚪ Deferred

Priority labels: **P0** (do first), **P1** (do next), **P2** (backlog)

### Action Items

Checkboxed list grouped by priority. Each item should be concrete and actionable:

**P0 - Do first:**

- [ ] Action item with `file_path` reference if applicable
- [ ] Another action item

**P1 - Do next:**

- [ ] Action item
- [ ] Action item

**P2 - Backlog:**

- [ ] Action item

### Phases Sections

For multi-phase plans, expand each phase with:

```markdown
## Phase N: {Name}

_One-line italicized summary of what this phase achieves._

**Goal:** What success looks like

**Steps:**

- [ ] Step 1 - description
- [ ] Step 2 - description

**Verify:** How to confirm this phase is done (e.g., `cd server && uv run pytest`)

**Key files:**

- `path/to/file.py`: What changes
```

Only include phases that were actually discussed. Don't invent phases.

### Risks Open Questions

- 🔴 **Risk or blocker**: What breaks if unaddressed. **Next step**: Mitigation or next step.
- 🟡 **Open question**: What it affects. **Owner**: Who needs to answer.
- ⚪ **Nice-to-know**: Low impact context for later.

> 🔴 High risk · 🟡 Medium / open question · ⚪ Low / informational

### References

Bullet list of links, prior plans, or external resources mentioned:

- `docs/plans/prior_plan.md` - what it covers
- URL or resource - what it covers

## Formatting Rules

### Status Circles

Use colored circles consistently as indicators in lists:

- 🟢 Approved / Ready / Complete
- 🟡 Tentative / In Progress
- 🔴 Rejected / Blocked / High Risk
- ⚪ Deferred / Informational

### Avoid Tables

- DO NOT use Markdown tables. They are difficult for AI agents to process efficiently.
- Use structured lists with bold keys instead.
- For comparisons or overviews, use bullet points with sub-bullets for details.

### General

- **Checkboxes** (`- [ ]`): Use for all actionable items so the user can track progress
- **Italicized tl;drs**: Every major section can optionally have a one-line italic summary
- **Headings**: Use `##` for major sections, `###` for subsections. Don't go deeper than `####`
- **Code references**: Always use backticks for file names, paths, commands, functions, config keys
- **Bullet points over paragraphs**: Break any explanation longer than 2 sentences into bullets
- **No filler**: Skip "this section covers..." preambles. Lead with the content
- **`<!-- omit in toc -->`**: Use on the top-level `#` heading only

## Example Output

```markdown
<!-- omit in toc -->

# QC Cache Redesign

Date: 2026-04-02
Source: conversation with your coding agent

- [tl;dr](#tldr)
- [Context](#context)
- [Decisions Made](#decisions-made)
- [Plan Overview](#plan-overview)
- [Action Items](#action-items)
- [Phase 1: Fingerprint-based invalidation](#phase-1-fingerprint-based-invalidation)
- [Phase 2: Background refresh](#phase-2-background-refresh)
- [Risks Open Questions](#risks-open-questions)

_tl;dr Replace time-based QC cache with fingerprint-based invalidation to eliminate stale results after migrations._

## Context

- QC results were cached with a 5-minute TTL, causing stale data after migration edits
- Users reported QC tab showing pre-migration outliers after applying fixes
- Prior plan: `docs/plans/2026_03_28_staleness_system.md`

## Decisions Made

- 🟢 **Decision**: Use content fingerprint, not timestamps. **Rationale**: Timestamps miss in-place edits to migration JSON.
- 🟢 **Decision**: Hash migration file + source data. **Rationale**: Captures both data changes and rule changes.
- 🟡 **Decision**: Consider background pre-warming. **Rationale**: Deferred - evaluate after P0 lands.
- 🔴 **Decision**: Rejected per-field granular caching. **Rationale**: Too complex for current data volume.

> 🟢 Approved · 🟡 Tentative · 🔴 Rejected · ⚪ Deferred

## Plan Overview

- 🟢 **Phase 1**: `server/core/qc_engine.py` - Fingerprint-based cache invalidation. **Priority**: P0
- 🟡 **Phase 2**: `server/core/qc_engine.py` - Background refresh on migration save. **Priority**: P1

> 🟢 Ready to start · 🟡 Needs more detail · 🔴 Blocked · ⚪ Deferred

## Action Items

**P0 - Do first:**

- [ ] Add `_compute_fingerprint()` to `server/core/qc_engine.py`
- [ ] Replace TTL check with fingerprint comparison in `run_qc_analysis()`
- [ ] Update tests in `server/tests/test_qc_cache.py`

**P1 - Do next:**

- [ ] Add background pre-warm after migration save in `server/core/migration_crud.py`

## Phase 1: Fingerprint-based invalidation

_Replace TTL caching with content-hash invalidation so QC results always reflect current data._

**Goal:** QC results invalidate immediately when migrations or source data change

**Steps:**

- [ ] Compute SHA-256 of migration JSON + source data file mtimes
- [ ] Store fingerprint alongside cached QC results
- [ ] Compare fingerprint on cache read; miss if different

**Verify:** `cd server && uv run pytest tests/test_qc_cache.py -v`

**Key files:**

- `server/core/qc_engine.py`: Add fingerprint logic, replace TTL check
- `server/tests/test_qc_cache.py`: Add fingerprint invalidation tests

## Phase 2: Background refresh

_Pre-warm QC cache after migration saves so the QC tab loads instantly._

**Goal:** QC tab shows fresh results without a loading spinner after migration edits

**Steps:**

- [ ] Fire async QC job after successful migration save
- [ ] Reuse existing `qc-start` job infrastructure

**Verify:** Save a migration, switch to QC tab - results should appear without "Running QC..."

## Risks Open Questions

- 🟡 **Risk**: Large datasets may slow fingerprint computation. **Impact**: QC tab load time. **Next step**: Benchmark with biggest dataset first.
- 🟡 **Open question**: Should fingerprint include excluded_files.json? **Impact**: Correctness. **Owner**: Decide during implementation.

> 🔴 High risk · 🟡 Medium / open question · ⚪ Low / informational
```

{% endraw %}
