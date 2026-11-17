# Test plan format

Rules for each section of `TEST_PLAN.md`. Apply every rule to every confirmed category.

## Severity markers

Tag every test step with one of these emojis in the step title:

| Emoji | Meaning          | When to use                                                                  |
| ----- | ---------------- | ---------------------------------------------------------------------------- |
| 🔴    | **Critical**     | Core functionality - if this fails, the feature is broken                    |
| 🟢    | **Expected**     | Standard behavior that should work - moderate confidence but worth verifying |
| 🔵    | **Nice-to-have** | Polish, UX, non-blocking - skip if short on time                             |

Example: `1a. 🔴 **Create an account end-to-end**`

## Formatting rules

- **Numbered sections** with separator lines (`---`) between them
- **Numbered sub-steps** within each section (1a, 1b, 1c...)
- Each sub-step has an **emoji tag + bold title** describing what to test
- Each sub-step has a **copy-paste command** in a fenced code block (or manual UI steps if applicable)
- Each sub-step has a **"Verify:"** line stating what success looks like
- **One command per code block** -- never stack multiple commands in one block with comments between them
- Use the project's **task runner targets** when available instead of raw tool commands
- For commands requiring env vars, put them inline: `API_URL=http://localhost:8000 make test-e2e`

## What to include

Include what a human must verify manually:

- Happy-path user flows end-to-end (the main thing the feature does)
- Integration points - does component A actually talk to component B correctly?
- State transitions - does data persist, propagate, and display correctly across the system?
- Resumption / retry behavior - if a multi-step process fails midway, does retry work?
- UI rendering - does the new section/field/page show up and look right?
- API response shape - do new fields appear in real responses?
- Existing behavior preserved - does the change break anything that was already working?

Leave these to unit tests:

- Invalid input validation (wrong types, missing fields, malformed data)
- Boundary values and off-by-one checks
- Error message wording verification
- Permission/auth edge cases (401/403 responses)
- Schema validation failures

## Quick smoke test section

Always end with a "Quick Smoke Test" section -- the 2-3 commands a reviewer would run if they only have 60 seconds. Tag each with the appropriate emoji.

## Style

- **Bold the what**, plain text the how
- No fluff -- every step must verify something real that a human needs to see
- Copy-paste ready -- a reviewer should never need to edit a command
- Separate code blocks -- one command per block, bold header above it
