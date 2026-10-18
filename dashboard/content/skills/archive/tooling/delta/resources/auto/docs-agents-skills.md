+++
title = "docs-agents-skills"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Skills"
url: "https://delta.dev/docs/agents/skills"
description: "Teach agents reusable workflows with modular skill files."
---

1.  Using Delta
2.  Skills

# Skills

Teach agents reusable workflows with modular skill files.

On this page

1.  [Manage skills](#manage-skills)
2.  [Create a skill with the agent](#create-a-skill-with-the-agent)
3.  [Where skills live](#where-skills-live)
4.  [Anatomy of a skill](#anatomy-of-a-skill)
5.  [Invoke a skill directly](#invoke-a-skill-directly)
6.  [Configure a Land skill](#configure-a-land-skill)
7.  [When to write one](#when-to-write-one)
8.  [What’s next](#whats-next)

Skills are modular instructions that extend what an agent knows how to do. Each skill is a Markdown file that describes a capability, such as reviewing code or running a project’s test suite. When a request matches a skill’s description, the agent loads the full instructions and follows them.

## [Manage skills](#manage-skills)

In the desktop app, open **Settings > Skills**. Choose **Global** for your personal skills or a project from the scope menu for its skills.

*   Use **Open Folder in Zed** to add a skill in the selected scope.
*   Use a skill’s **Open in Zed** button to edit its `SKILL.md`.
*   Skills that couldn’t load show their path and error. Open the file in Zed to fix it.
*   Use **Delete Skill** and confirm to move the entire skill folder to the Trash, including supporting files. This also removes it from other agent tools that use that location.

The Skills manager is not available in the browser.

## [Create a skill with the agent](#create-a-skill-with-the-agent)

Type `/create-skill` in the composer and describe the workflow you want to reuse. Specify whether it belongs to an attached project or your personal skills. The agent helps write or update the `SKILL.md` and checks its format. Personal skills are stored on your computer, so browser turns cannot use them.

## [Where skills live](#where-skills-live)

*   **Project skills** live in the repository under `.agents/skills/` and are shared with everyone who works in that project.
*   **Personal skills** live under `~/.agents/skills/` and follow you across projects on that machine.

The common layout is `.agents/skills/\<name\>/SKILL.md`. You can also group skills into nested category folders, such as `.agents/skills/releases/\<name\>/SKILL.md`. The folder containing `SKILL.md` is the skill folder.

For skills used only in Delta, use `.delta/skills/`. See [Delta-specific overrides](/docs/configuration/project-setup#delta-specific-overrides).

Delta also ships built-in skills. `/change-guide` creates a structured walkthrough of an existing code change, and the agent can search your other threads for relevant context on its own. To ask about Delta itself, mention `@Delta` in any message; see [Getting Started](/docs/getting-started#ask-delta-about-delta). When a project skill and a personal skill share a name, the project skill wins.

## [Anatomy of a skill](#anatomy-of-a-skill)

A skill is a folder containing a `SKILL.md` that opens with YAML frontmatter, followed by the instructions:

```
---
name: release-checklist
description: Cut and verify a release. Use when asked to prepare a release.
---

Steps for the agent to follow...
```

`name` uses lowercase letters, numbers, and hyphens. The agent reads the description to decide whether the skill applies, so write it the way you’d tell a colleague when to reach for the workflow.

Two optional frontmatter fields control how a skill is invoked:

*   `user-invocable` (default `true`): whether the skill appears in the composer’s `/` menu.
*   `disable-model-invocation` (default `false`): when `true`, the agent never loads the skill on its own; it runs only when you invoke it.

A skill folder can also carry supporting files, such as scripts or templates, and the instructions can point to them.

## [Invoke a skill directly](#invoke-a-skill-directly)

Type `/` in the composer to pick a skill by name. The same menu also lists built-in commands such as `/compact`, `/model`, `/thinking`, `/fast`, `/add-project`, `/subthread`, and `/review`. Typing or pasting a full skill command such as `/release-checklist` attaches that skill when you confirm it or type a space after the name; until then the picker stays open so you can check the match. When there’s room, the selected skill’s description appears beside the picker.

With [Send With Modifier](/docs/configuration/settings#messages) enabled, press `Cmd+Enter` on macOS or `Ctrl+Enter` on Linux and Windows to accept the highlighted skill and send your message. Press `Enter` to insert the skill without sending.

The agent runs the attached instructions without waiting for a matching request. This works well for workflows such as “cut a release” or “review this PR,” where you want to choose exactly when the skill runs.

If you invoke a skill that can’t load, Delta stops before sending and shows the file path and reason. Fix the skill file, then send the same draft again. An unrelated invalid skill does not block sending. **Land Changes** ignores invalid skills that aren’t marked for Land, but stops if a broken skill is marked for Land or Delta can’t read or parse its file.

## [Configure a Land skill](#configure-a-land-skill)

To make a skill eligible for [Land Changes](/docs/agents/review-and-sync#land-changes), add `metadata.delta-action: land` to its frontmatter. For example, an existing landing skill can start with:

```
---
name: land
description: >-
  Land this project's changes only when explicitly requested.
  Do not use for review, preparation, or skill installation.
metadata:
  delta-action: land
---
```

Keep your project’s landing instructions below the frontmatter. The metadata value must be lowercase `land`; naming the skill or its folder `land` is not enough.

**Land Changes** opens a dedicated subthread, then runs the eligible skill. If several qualify, the agent lists them; explicitly invoke your choice through the slash menu in that subthread. Skills with `user-invocable: false` are hidden from the menu. To make a skill selectable, omit that field or set it to `true`. A conversational reply naming a skill does not attach it.

Invoking a skill with `/land` directly runs it in the current conversation; the metadata alone does not create a subthread.

If you don’t have a landing workflow yet, choose **Land Changes** to set one up. Approving setup creates or updates the skill, but does not land any changes.

## [When to write one](#when-to-write-one)

Write a skill when you find yourself giving an agent the same instructions twice for a particular kind of task. Common examples:

*   A code review checklist tuned to your codebase.
*   A release or deployment procedure.
*   Style and voice rules for public-facing writing.

For standing rules that should apply whenever the agent works in a repository or one of its folders, use an `AGENTS.md` file instead. See [Settings](/docs/configuration/settings#rules).

## [What’s next](#whats-next)

*   [Threads](/docs/agents/threads) covers the conversations where skills come into play.
*   [Settings](/docs/configuration/settings) covers Delta’s other customization points.
