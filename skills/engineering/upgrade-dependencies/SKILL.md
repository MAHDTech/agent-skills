---
name: upgrade-dependencies
description: Safely upgrade dependencies, frameworks, or a language/runtime version across a codebase, including risky major-version bumps and framework migrations. Use when bumping a package, stepping a major version, migrating a framework, or updating a runtime — read the changelog first, move in small reversible steps behind a green safety net, handle transitive and lockfile changes, and stage the rollout. Lean on /characterization-tests for the net and /diagnosing-bugs when an upgrade breaks something.
---

# Upgrade Dependencies

An upgrade is a behaviour change you did not write. Read what is coming, move in small reversible steps behind a green safety net, and you can pinpoint the one bump that broke something instead of debugging a pile of them at once.

The same discipline covers a single package bump, a major-version jump, a framework migration, and a language or runtime update — only the blast radius grows.

Run the suite and the upgrade commands through the project's toolchain — for example `devenv shell -- <cmd>` — so the environment is the same before and after.

## Phase 1 — Establish the safety net

Before touching a single version, get the affected code **green**: run the suite and confirm it passes on the current versions. That green is your **safety net** — the signal that an upgrade changed behaviour.

If the code path you are upgrading has no tests, that net does not exist yet. Build it first with /characterization-tests, which pins current behaviour so an upgrade that alters it turns red.

Completion: a green test suite — or fresh characterization tests — covering the code that touches the dependency, run at least once on the current versions.

## Phase 2 — Read before you upgrade

An upgrade's risk lives in its **changelog**. Before changing anything, read across the version range you are crossing for:

- The **migration guide**, release notes, or upgrade guide for any major version.
- **Breaking changes** and removed APIs — what you _must_ change.
- **Deprecations** — what still works but is on notice.
- Security advisories, if this is a security-driven bump.

Note every breaking change that touches your code. That list is the work.

## Phase 3 — Upgrade in small reversible steps

One change per step, each ending in a commit you could revert:

- **One dependency at a time.** Bump ten at once and a failure could come from any of them.
- **Step through majors, don't leap.** Prefer N → N+1 → N+2, letting the suite pass at each stop, over jumping several majors at once — migration guides are written per major.
- **Mind the lockfile.** Review the **transitive** dependency changes an upgrade drags in, not just the direct bump — a shifted sub-dependency breaks as easily as a direct one. Regenerate the lockfile the project's way so the whole tree resolves.
- **Keep upgrades out of feature commits.** A dependency bump sharing a commit with a feature hides which one broke a later test.

Completion: each intended version bumped, the dependency tree resolves cleanly, and every step is an isolated commit.

## Phase 4 — Fix forward against the net

Run the suite after each step. Work the breaks the changelog predicted — renamed APIs, changed defaults, removed options — until it is green again.

When a break is **mysterious** — a test fails and the changelog gave no warning — run /diagnosing-bugs, using the red test as the ready-made feedback loop, rather than guessing at the upgrade.

Completion: the suite is green on the new versions, with every failure understood rather than muted.

## Phase 5 — Stage the rollout

For a risky bump — a major framework, a runtime, anything on a hot path — green tests are not proof it is safe in production. Stage it:

- Roll out behind a flag, to a canary, or to a fraction of traffic before the whole fleet.
- Watch the signals that matter — errors, latency, resource use — against the pre-upgrade baseline.
- Keep the revert path ready until it is proven: a pinned previous version you can roll back to.

For a low-risk patch or a dev-only tool, this phase is a quick judgement call, not a ceremony.
