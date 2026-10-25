+++
title = "invocation-compatibility"
[extra]
skill = false
category = "authoring"
mermaid = false
skill_name = "skill-creator"
+++

# Skill invocation compatibility

Verified against official documentation and native Goose source on 2026-10-25. This reference guides authoring of promoted custom skills. Vendor snapshots retain upstream policy; archived skills retain historical metadata.

## Shared discovery text

All hosts below use the `SKILL.md` frontmatter `description` to tell the model when a skill applies. Discovery normally exposes metadata first and loads the body when selected. Catalog budgets, permission settings, and invocation controls can change which descriptions reach the model.

Write one or two sentences covering the capability and concrete trigger, usually 100-220 characters. That range is an editing target, not a validation limit. Front-load the main use case; omit keyword inventories and feature lists. Keep explicit-only intent in the shared description because some hosts have no verified enforcement mechanism.

Codex's `interface.short_description` in `agents/openai.yaml` is a user-facing UI summary. It does not replace the model-facing `SKILL.md` description. [OpenAI's skill guide](https://learn.chatgpt.com/docs/build-skills) distinguishes those fields and warns that large catalogs can shorten or omit initial descriptions.

## Provider support

| Host                    | Selection and loading                                                                                                                                      | Invocation control                                                                                                                                                                                                                                                                                                                      |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code             | Matches the description; loads the body when invoked. Users can invoke a skill with `/skill-name`.                                                         | `disable-model-invocation: true` prevents autonomous loading and omits its description from the normal model catalog. `user-invocable: false` instead disables direct user invocation. [Claude Code documentation](https://code.claude.com/docs/en/skills#control-who-invokes-a-skill)                                                  |
| Codex                   | Matches the shared description. Explicit invocation uses `$skill-name` or the `/skills` selector.                                                          | `agents/openai.yaml` supports `policy.allow_implicit_invocation: false`; explicit invocation still works. The default is `true`. [OpenAI documentation](https://learn.chatgpt.com/docs/build-skills)                                                                                                                                    |
| Antigravity CLI and IDE | The shared guide says the agent sees names and descriptions and loads relevant instructions. The CLI also converts skills into slash commands.             | No equivalent for disabling implicit selection is documented in the inspected guide. [Antigravity skills guide](https://antigravity.google/docs/skills)                                                                                                                                                                                 |
| Antigravity CLI         | Provides skill slash commands in the interactive prompt.                                                                                                   | `disable-slash-command: true` hides the skill from the slash menu and `/name` resolution while leaving model discovery and invocation available. This is the opposite direction from an implicit invocation opt-out. IDE support for this field was not established. [CLI changelog](https://antigravity.google/docs/changelog?tab=cli) |
| OpenCode 1              | Lists skill names and descriptions in its `skill` tool; the model loads instructions on demand.                                                            | Its documentation explicitly says unknown frontmatter is ignored and metadata does not control automatic selection. Skill permissions allow, ask, or deny access; denial hides and rejects the skill rather than preserving manual-only loading. [OpenCode 1 documentation](https://opencode.ai/docs/skills/)                           |
| OpenCode V2             | Advertises the description to the model; users can explicitly select `@skill-id`.                                                                          | `metadata.opencode/autoinvoke: false` omits autonomous advertisement. `disable-model-invocation: true` is an alias, and the OpenCode metadata wins if both are set. Explicit loading remains possible. This behavior is documented for V2, not V1. [OpenCode V2 documentation](https://dev.opencode.ai/v2/docs/skills/)                 |
| Goose native            | Adds skill names and descriptions to session instructions and loads the body through its skill loader. Explicit requests and `/skills name` are supported. | No per-skill invocation flag is documented. The inspected native loader does not enforce either policy field discussed here. [Goose documentation](https://goose-docs.ai/docs/guides/context-engineering/using-skills/), [native loader](https://github.com/aaif-goose/goose/blob/main/crates/goose/src/skills/client.rs)               |

The native Goose conclusion also follows from its [frontmatter parser](https://github.com/aaif-goose/goose/blob/main/crates/goose-agent/src/skills/discovery.rs), which reads `name`, `description`, and `metadata`, and its client, which advertises all discovered native and builtin skills without an invocation-policy filter. This is source inspection of `main` on the verification date, not a claim about every released version or delegated provider. A Goose configuration that runs another coding agent may inherit that agent's behavior.

Absence from the inspected Antigravity documentation is an evidence limit, not proof that every implementation ignores every undocumented field. Do not advertise Codex's `allow_implicit_invocation` as portable or invent an Antigravity equivalent without new official evidence.

## Repository policy

Every promoted custom skill must record an explicit YAML boolean in `agents/openai.yaml`:

| Intended selection | `SKILL.md`                                            | `agents/openai.yaml`                      |
| ------------------ | ----------------------------------------------------- | ----------------------------------------- |
| Automatic          | Omit `disable-model-invocation` or set it to `false`. | `policy.allow_implicit_invocation: true`  |
| Explicit only      | Set `disable-model-invocation: true`.                 | `policy.allow_implicit_invocation: false` |

For an explicit-only skill, state the same intent near the start of the body:

```text
Use this skill only when the user explicitly requests its workflow.
```

That instruction is advisory on hosts without enforcement. A manual skill is not universally absent from model context, and automatic selection is not authorization to perform its actions.

Prefer explicit initiation for publishing, lifecycle operations, large orchestration workflows, and deliberate catalog entry points. Prefer automatic selection for focused references and bounded helpers needed within an authorized task. An interactive interview or conflict-resolution helper may remain automatic when its description limits selection to the requested interview or an authorized parent operation that needs it.

Inside the active devenv shell, verify the catalog contract with:

```bash
scripts/run-tests.sh -E 'test(promoted_custom_skills_have_matching_explicit_invocation_policies)'
```

The integration test checks policy presence and parity across promoted custom skills. Review representative matching requests and nearby non-matching requests separately to assess description scope. The test does not determine whether a skill should be automatic or authorize any workflow action.

