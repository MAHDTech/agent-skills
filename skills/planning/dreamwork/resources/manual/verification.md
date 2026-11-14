# Verification planning

This manual is for the Architect writing the gate plan. Children follow the
execution rules in the [child contract](child-contract.md).

## Gate plan

Write `gates.json` in the plan from the project contract and each issue's acceptance
criteria. The skill supplies no default commands. Per gate record:

- `id`, `required`, and the acceptance criteria it proves;
- `command` as an argument array, `cwd` relative to the checkout root, and
  non-secret `environment`;
- `run_by`: any of `implementer`, `reviewer`, `run_verification`;
- `source`: the `AGENTS.md` heading or document the rule came from;
- generated inputs and services it needs (see preflight inventory).

Every acceptance criterion maps to at least one gate, a captured visual artifact, or
a UAT checklist item. Never mark a gate optional to get it passing; weakening or
removing a gate is Tier C.

## Automate before asking the human

The human should see only what agents cannot judge. For each issue:

- **Full journeys.** For UI and workflow changes, gates or scripted checks cover
  entering the feature, leaving it, and doing something useful in the destination,
  across every adapter or transport the change touches. API success and matching
  text do not prove navigation works.
- **Visual capture.** Plan which UI states the Implementer captures (screenshots or
  recordings, with viewport, scale and theme recorded) so the council can review
  design and UX acceptance.
- **Substitutes.** Where the project approves one (for example a virtual
  authenticator or a device emulator), plan the automated substitute and keep the
  real human-only check in the UAT checklist.
- **Fixtures.** Fixtures must include every real prerequisite. A diagnosis about
  fixture state needs the actual request and stored state captured first.

## Preflight inventory

Before approval, inventory what the planned gates need so gaps surface before an
expensive build:

- **tools**, with versions where the project constrains them;
- **generated inputs**: path, consuming gates, and the approved preparation command
  that builds them from committed sources (existence alone does not prove freshness);
- **services**: startup, readiness, isolation settings (ports, state directories),
  teardown, and which gates use them;
- **evidence inputs** a gate consumes, with revisions or hashes;
- **access**: secrets, sign-ins and devices, settled while the human is present
  (autonomy manual);
- **UAT environment**: the human's browser, device and GPU availability for the
  launch command.

Preparation runs only through approved commands and never authorises dependency,
tooling or product changes outside scope.

## Final gates

Mark the gates `run_verification` runs at the run-branch head before UAT. Usually
these are the project's full suite and hooks. Reuse an issue's gate evidence only
where the project allows it for an unchanged tree.

## Evidence sufficiency

- A required gate passes only with a reliable zero exit code on the matching
  revision and a hashed log.
- Live gates must show they exercised their prerequisites. A silent skip or zero
  tests run is not a pass; use the project's enforcement setting where one exists.
- Shared-resource tests must isolate or serialize so leftover work cannot affect
  later checks.
- Human observations are recorded verbatim at UAT; agent screenshots support but
  never replace them for human-only checks.
