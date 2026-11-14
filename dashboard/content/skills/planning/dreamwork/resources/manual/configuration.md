+++
title = "configuration"
[extra]
skill = false
category = "planning"
mermaid = false
skill_name = "dreamwork"
+++

# Configuration

Configuration holds the user's model preferences and workflow budgets. It holds no
project facts: paths, gates, services, issue conventions and UAT launch belong to
the [project contract](@/skills/planning/dreamwork/resources/manual/project-contract.md).

## Resolution

1. A config path named in the request.
2. Otherwise the project file `.agents/dreamwork.json` in the initiating repository,
   layered over the user file `~/.agents/dreamwork.json`. Either may be absent.
3. If neither exists, propose a user file from [the example](config.example.json)
   and stop before dispatch.

Layering replaces whole entries: a project file that defines `roles.implementer`
replaces the user's `roles.implementer`; one that defines `council` replaces the
whole council; each `workflow` key replaces only that key. The result is the
effective config. Snapshot it into the plan proposal; that snapshot, not the live
files, governs the run.

## Schema (`version: 4`)

- `version`: integer `4`.
- `roles.researcher`, `roles.implementer`, `roles.reviewer`: each
  `{ "targets": [target, ...] }` with at least one target. Order is preference;
  later targets are pre-approved fallbacks.
- `council.seats`: two or three `{ "name", "family", "targets": [target, ...] }`.
  `family` is the model vendor or lineage (for example `anthropic`, `openai`,
  `google`), not the provider instance: one harness can serve several vendors'
  models. Every target in a seat must be from that seat's family, and no two seats
  share a family, so a fallback never collapses the council's diversity. `name`
  labels the seat in score tables.
- `council.min_seats`: integer from 2 to the number of seats. A final or amendment
  council proceeds only with at least this many seats responding.
- A target is `{ "providerInstanceId", "model", "options" }`: nonempty strings and
  an object of string or boolean values matching `orchestrator_capabilities`. It maps
  directly to `delegate_task.target`. Provider instances name harness connections,
  not vendors.
- `workflow.max_parallel_issues`: integer of at least 1.
- `workflow.max_repair_rounds`: integer of at least 0, per issue, counting every
  product repair (reviewer, council, reconciliation edits beyond mechanical conflict
  resolution). UAT repairs are counted separately.
- `workflow.repair_extension_rounds`: integer of at least 0. Extra rounds granted
  once per issue when the extension rule in [autonomy](@/skills/planning/dreamwork/resources/manual/autonomy.md) holds.
- `workflow.max_council_rounds`: integer of at least 1 per issue.
- `workflow.max_infrastructure_recovery_attempts`: integer of at least 0, per issue
  (per run for audits and planning tasks), separate from repairs.

Booleans are not integers. Reject unknown keys, missing fields, wrong types and
unsupported versions before dispatch.

Recommended: the Reviewer's model family differs from the Implementer's, and the
council includes at least one family used by neither.

## Validation and changes

Validate every target, including fallbacks, during planning. Use catalog search for
models absent from the short list. Record unavailable targets in the plan; a role
with no available target blocks approval of work that needs it. Never validate the
Architect against config.

Mid-run, switching to the next listed fallback follows the infrastructure rules in
[records](@/skills/planning/dreamwork/resources/manual/records.md#infrastructure-recovery). Any other change (a new model, new
options, a higher budget) is a Tier C decision: it parks affected work for the UAT
packet or waits for an explicit user message. Config changes never switch the
Architect.

## Migrating from version 3

Version 3 had single-target roles, a single reviewer for all review, `project.*`
paths and no council. To migrate: wrap each role target in `targets`, add `council`,
move `project.runs_dir`, `project.local_state_dir` and `protocol_files` facts into
the repository's `AGENTS.md` (or accept the contract defaults), and add the new
`workflow` keys. Explain the change and get explicit agreement before rewriting a
config file. Runs started under version 3 stay under version 3 records; resuming
one with this skill needs an explicit migration that reconstructs counters and
approval identity before dispatch.

