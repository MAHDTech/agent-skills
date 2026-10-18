+++
title = "docs-privacy-and-security-privacy"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "AI Privacy & Telemetry"
url: "https://delta.dev/docs/privacy-and-security/privacy"
description: "What Delta sends to model providers and records as product telemetry."
---

1.  Privacy & Security
2.  AI Privacy & Telemetry

# AI Privacy & Telemetry

What Delta sends to model providers and records as product telemetry.

On this page

1.  [AI requests](#ai-requests)
2.  [Data use and training](#data-use-and-training)
3.  [Telemetry](#telemetry)
4.  [Crash and error reporting](#crash-and-error-reporting)

Delta is built by Zed and operates under the same [Privacy Policy](https://zed.dev/privacy-policy) and [Terms of Service](https://zed.dev/terms) as the Zed editor. Delta’s [Early Access Agreement](https://delta.dev/early-access) also applies during the public beta.

Delta stores repository and thread data to provide synchronization and collaboration. [Data Storage & Deletion](/docs/privacy-and-security/data-storage) explains what Delta stores, where it lives, and how to request deletion. The sections below cover data sent to model providers and product telemetry.

## [AI requests](#ai-requests)

When you send a message to an agent, Delta sends a model request to your selected model provider. Requests to hosted models go through Zed Cloud; requests to a provider you connected yourself, with an API key or a subscription sign-in, go directly to that provider. For ChatGPT and Grok subscriptions, Zed Cloud stores the OAuth grant and gives Delta short-lived access tokens; the model request itself still goes directly to the provider. The request can include conversation history, collaborator messages and comments, attached images, tool calls and results, terminal or command output, and file content the agent reads while working. The agent can also search your other threads and read their transcripts with a tool, so a request can carry content from past threads, in bounded excerpts. The thread you are working in is excluded from that tool.

A request includes the context provided to the agent for its current work. Delta does not automatically send every file in the repository.

Delta uses the same model providers, request paths, and provider agreements as Zed. See [AI Privacy](https://zed.dev/docs/ai/privacy-and-security) for provider-specific data handling, retention, and training commitments.

Delta redacts recognized secret values before storing thread content or sending it to a model provider. See [Secret redaction](/docs/privacy-and-security/security#secret-redaction).

## [Data use and training](#data-use-and-training)

Delta stores repository and thread data to synchronize your work across machines and enable collaboration. Delta does not use your code or conversations to train AI models.

Model providers handle AI requests under the same agreements and policies as Zed. See [AI Privacy](https://zed.dev/docs/ai/privacy-and-security) for provider-specific no-training and retention commitments.

For Delta’s own storage and deletion practices, see [Data Storage & Deletion](/docs/privacy-and-security/data-storage).

## [Telemetry](#telemetry)

Delta uses Zed’s telemetry infrastructure. Because signing in is required to use Delta, Delta telemetry is associated with your authenticated account. Delta does not currently provide a setting to disable telemetry.

Delta collects product-usage metadata such as which features are used, agent outcomes, model selection, and thread or Delta worktree activity. This telemetry does not include your conversations, source code, repository names, remote URLs, or terminal output. When Delta shows you an error, the error’s message text is included in telemetry.

See [Telemetry in Zed](https://zed.dev/docs/telemetry) for information about the shared telemetry infrastructure and data handling. The opt-out controls described there do not apply to Delta.

## [Crash and error reporting](#crash-and-error-reporting)

Delta reports crashes and visible errors to Sentry, and there is no setting to disable this. Error reports carry the error’s message text, where in Delta’s code the error occurred, the app version, and similar diagnostic metadata. Crash reports include the crash message and a memory dump of the app at the moment it crashed, which can contain thread content. When you are signed in, reports carry the same identifier as telemetry, your Zed and GitHub usernames, and internal identifiers for the thread and Delta worktrees involved (the identifiers, not their contents).
