+++
title = "docs-agents-subagents"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
+++

---
title: "Subagents"
url: "https://delta.dev/docs/agents/subagents"
description: "Delegate independent tasks, follow subagent work, and understand their access to your files."
---

1.  Using Delta
2.  Subagents

# Subagents

Delegate independent tasks, follow subagent work, and understand their access to your files.

On this page

1.  [When to use subagents](#when-to-use-subagents)
2.  [Profiles](#profiles)
3.  [Access to your files](#access-to-your-files)
4.  [Follow their work](#follow-their-work)
5.  [Models and configuration](#models-and-configuration)
6.  [What’s next](#whats-next)

Ask the agent to delegate independent parts of your task to subagents. Each subagent works in a separate conversation and reports back to the agent in your original thread, called the parent thread.

## [When to use subagents](#when-to-use-subagents)

Use subagents for independent work that can happen alongside the main task, such as investigating separate parts of a codebase, implementing independent parts of a change, or getting another agent to check an existing change.

Ask the agent to use subagents and describe the work you want delegated. Delegation defaults to **Only When Asked**. To let the agent delegate whenever it finds that useful, choose **Max** in **Settings > Subagents > Delegation**. Choose **Off** to disable delegation.

Keep tightly dependent steps in the main thread when the next step needs the result of the previous one. Delegating that work adds a handoff rather than letting tasks run in parallel.

## [Profiles](#profiles)

A profile describes a subagent’s role and chooses its model. Delta includes three profiles:

*   **Worker** implements changes with full tool access.
*   **Scout** gathers information from the codebase or the web.
*   **Reviewer** checks existing changes.

Delta selects default models for these profiles. You can change each profile’s model and thinking effort in **Settings > Subagents**. See [Agent profiles settings](/docs/configuration/settings#agent-profiles) for model choices and adding custom profiles.

## [Access to your files](#access-to-your-files)

By default, Worker and Reviewer subagents use isolated copies of the parent’s [Delta worktrees](/docs/concepts/worktrees). Their changes do not merge back automatically. Ask the parent agent to merge the child’s work when you want to bring it in. Scout uses the parent’s checkout directly, so its edits take effect immediately.

Use **Configure with Delta** in **Settings > Subagents** to change a profile’s worktree mode. See [Agent profiles settings](/docs/configuration/settings#agent-profiles).

Scout and Reviewer do not have file-editing tools, but both can change files through terminal commands. Don’t treat either profile as read-only. A Scout’s terminal commands can affect the parent’s checkout while the main agent is still working.

## [Follow their work](#follow-their-work)

New subagents run as ordinary subthreads nested under their parent in the sidebar. Open one to read its conversation, send a message to steer it, or continue the work yourself. Each starts with a fresh conversation and the task context supplied by its parent, rather than inheriting the parent’s transcript.

Messages between agents appear as folded **Sent Agent Message** and **Received Agent Message** events; expand an event to read the message. Agents can also [message another thread](/docs/agents/threads#send-information-to-another-agent). The parent receives the subagent’s final result automatically. You can send a follow-up in the child thread or ask the parent to message it. Worker subagents can delegate to children of their own; Scout and Reviewer cannot. Each child’s final result goes to its immediate parent. Subagents created before Delta 0.20.0 keep their previous automatic-merge behavior.

Press `Cmd+F` on macOS or `Ctrl+F` on Linux and Windows to search the focused subagent conversation. Press `Escape` there to stop that subagent.

New agent-created subthreads appear with **Plain** children in the desktop sidebar filter. **Subagents** remains available for older attached subagents, alongside **Reviews** and **Land** children. An expanded parent shows the six newest matching children across these kinds. Choose **Show N more** to reveal the rest.

Click a subagent to open it as primary, or hold `Cmd` on macOS or `Ctrl` on Linux and Windows while clicking to open it in a tab. You can also use the sidebar row’s **Open in Tab** button. Closing a subagent tab does not stop its work.

On desktop and web, **View All Subthreads** (`Cmd+J` on macOS or `Ctrl+J` on Linux and Windows) opens a picker for the parent’s children. Filter by kind, and press `Tab` to show or hide archived children. The picker remembers your selected filter when reopened.

## [Models and configuration](#models-and-configuration)

In **Settings > Subagents**, choose each profile’s default model and thinking effort. To use a different model for a particular provider, open that provider’s **Model Preferences** in **LLM Providers**.

Delta checks for model choices in this order:

1.  Your provider-specific model choice.
2.  Your default model choice for the profile.
3.  Delta’s built-in default.

If you haven’t set a choice, Delta moves to the next option. Worker defaults to the parent’s model. With ChatGPT sign-in, your plan affects the built-in Scout and Reviewer defaults; explicit profile model choices take precedence.

Use [Subagents settings](/docs/configuration/settings#subagents) to control delegation, concurrency limits, and whether the parent can override the profile’s model choice.

## [What’s next](#whats-next)

*   [Settings](/docs/configuration/settings#subagents) covers delegation controls and custom profiles.
*   [Models & Providers](/docs/agents/models-and-providers) covers connecting providers and choosing models.
*   [Review & Sync Changes](/docs/agents/review-and-sync) covers reviewing the resulting changes and bringing them into your repository.
