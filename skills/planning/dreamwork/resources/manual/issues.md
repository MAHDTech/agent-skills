# Issues

## Project conventions

Resolve the issue facts in the [project contract](project-contract.md): the
authoritative tracker (local files or an external service), stable IDs, labels,
dependency representation, lifecycle states and transitions, ownership, review
dossier location, and link and index rules. Dreamwork prescribes no ID scheme,
folder layout or tracker.

Use an external tracker only through available tools and authorised access. Plan
approval grants no remote write authority; if a required transition needs it, settle
that during planning rather than silently switching to local files.

Record the resolved mapping per issue in the plan: stable ID, source path or
locator, current state, dependencies, permitted transitions and how to apply them,
review dossier destination, and a safe path key for artifact paths.

## Selection

Resolve explicit IDs, label selectors, a grouping document or a fresh prompt.

### Grouping documents (epics and waves)

Some projects group issues into epics with ordered waves (see the project contract). When
the request names an epic and wave, the wave's issue list is the selection and one run
delivers exactly that wave:

- Read the epic and wave from the project's grouping document; never infer membership from
  labels or numbering.
- Order inside the wave comes from each issue's dependencies, not its position in the list.
- Issues the document marks as apart must not run at the same time; record them as an
  ordering constraint in the plan.
- An issue that depends on an unfinished issue in a later wave, or outside the epic, is a
  missing prerequisite: propose it before approval as for any selection.
- Moving an issue between waves after approval is a Tier C change.
- At landing, update the grouping document's checklist as the project prescribes.

### Other selectors

Several labels mean their intersection unless the human asks for a union. Show the resolved list before
approval; summarise already-completed matches separately. An explicitly selected
completed issue needs new scope, not automatic reimplementation.

Order by the dependency graph, not by ID. Propose unfinished prerequisites outside
the selection and show the expansion. Missing IDs, cycles, conflicting entry rules
and unverified "done" prerequisites block approval. For audits these are findings
to report, not prerequisites to fix.

Fresh prompts produce draft issues following project conventions, committed in the
proposal. Flag issues too broad for one review cycle (many files, several security
or recovery boundaries) and propose a split with staged acceptance.

Issue IDs, order and plans freeze at approval. Later label edits never expand a run.
Adding or dropping issues mid-run is Tier C ([autonomy](autonomy.md)): the affected
issue parks while independent issues continue.

## Lifecycle

- **Proposal commit:** drafts in the project's pending state.
- **Approval commit:** selected issues move to the project's active state.
- **Landing bookkeeping commit:** landed issues move to the completed state; review
  dossiers move to the project location with every round, response, council receipt
  and UAT decision preserved; indexes and links are updated.
- Parked or UAT-rejected issues keep their active state, with the reason recorded in
  run records. Move them back only if the human says so.

Completion requires independent review acceptance, final council acceptance, run
verification, explicit UAT approval and landing on the captured branch. The
Architect applies transitions; children never change issue state.

Follow project link conventions. Avoid links that break when files move between
lifecycle locations.
