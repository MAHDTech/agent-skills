# Project contract

Dreamwork needs a set of project facts. It reads them from the applicable
`AGENTS.md` files and the documents they link to. It never brings them from another
project, the skill or the model config.

Resolve each fact during planning and record the resolved value with its source
(file and heading) in the plan. If a fact is missing or two sources conflict, ask
the human one question at a time with a recommendation, and offer the exact
`AGENTS.md` text that would settle it so the next run does not ask again. Edit
`AGENTS.md` only if the human approves that text as part of the plan.

## Facts

| Fact | What to resolve | Default if `AGENTS.md` is silent |
| --- | --- | --- |
| Run records | Tracked directory for run records | `docs/dreamwork/runs` |
| Local state | Ignored runtime directory for live state, staging and raw logs | `.agents/dreamwork` (must be git-ignored; never force-add) |
| Run branch | Branch name pattern for staged work | `dreamwork/<run-id>` |
| Command environment | How to run build, lint and test commands (shells, wrappers, required variables) | none: ask |
| Gates | Required checks per change type, final checks before landing, and the pre-commit hook command | none: ask |
| Services | Startup, readiness, isolation (ports, state directories), and teardown for live checks | none, if no live checks are planned |
| Issues | Tracker, stable IDs, labels, dependencies, lifecycle states and transitions, review dossier location, link and index rules | none: ask (see [issues](issues.md)) |
| Grouping documents | Whether the project groups issues into epics/waves, where they live, their format and how a run updates them | none: selection by IDs, labels or prompt only |
| Commits | Message convention, hook environment, attribution | the repository's existing history style |
| Evidence | What may be committed, path portability, secret handling | repository-relative paths, no secrets, raw logs local |
| UAT launch | Command and checkout to start the app for a human, how to reach it (URL, window), and teardown | none: ask during planning if any issue has UAT items |
| Visual capture | How agents capture screenshots or recordings of UI states for review | none: plan per issue or note the gap |
| Human-only checks | Checks that cannot be automated (physical keys, real devices, taste) and approved automated substitutes | none |
| Specs and decisions | Where component specs and settled decisions live, and which to read per area | whatever `AGENTS.md` routes to |

Defaults are generic conveniences. State them in the plan so approval covers them.

## Rules

- Applicable `AGENTS.md` instructions are mandatory even when they repeat or exceed
  this skill. When they conflict with this skill on project matters (commands,
  paths, conventions), the project wins. On workflow authority (human gates, review
  independence, landing), this skill wins; raise the conflict in the plan.
- Resolve paths against the initiating repository root and reject anything that
  escapes it after resolving symlinks. Run records and local state must be disjoint,
  and local state must contain no issue sources or instruction files.
- Pass the resolved contract to children as text and paths in their briefs. A child
  rereads `AGENTS.md` in its own checkout and reports any difference.
- Dreamwork handles one repository per run. Escalate multi-repository requests
  during planning.

## Suggested `AGENTS.md` section

Offer something like this when facts are missing. Fill in real values; never commit
placeholders.

```markdown
## Dreamwork

- Run records: `docs/dreamwork/runs`; local state: `.agents/dreamwork` (git-ignored).
- Command environment: <how to run commands>.
- Gates: <per change type>; final gates: <list>; hooks: <command>.
- Services: <start, readiness, isolation variables, teardown>.
- UAT launch: <command, checkout, URL or window, teardown>.
- Visual capture: <screenshot or recording method>.
- Human-only checks: <list and approved substitutes>.
```
