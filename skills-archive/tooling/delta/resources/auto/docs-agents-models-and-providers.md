---
title: "Models & Providers"
url: "https://delta.dev/docs/agents/models-and-providers"
description: "Configure which models power Delta's agents."
---

1.  Using Delta
2.  Models & Providers

# Models & Providers

Configure which models power Delta's agents.

On this page

1.  [Hosted models](#hosted-models)
2.  [Sign in with a subscription](#sign-in-with-a-subscription)
3.  [Add an API key](#add-an-api-key)
4.  [Connect a compatible endpoint](#connect-a-compatible-endpoint)
    1.  [Add a provider](#add-a-provider)
    2.  [Choose the base URL](#choose-the-base-url)
    3.  [Configure models](#configure-models)
    4.  [Authentication and custom headers](#authentication-and-custom-headers)
    5.  [Compatibility and troubleshooting](#compatibility-and-troubleshooting)
5.  [Model roles](#model-roles)
6.  [Choose a model](#choose-a-model)
    1.  [Automatic fallback](#automatic-fallback)
7.  [Context windows and compaction](#context-windows-and-compaction)
8.  [What’s next](#whats-next)

Delta’s agents are powered by large language models. You can use models included with your Zed plan, sign in with a subscription you already have, or bring your own API keys.

## [Hosted models](#hosted-models)

Paid [plans](/docs/account/plans-and-pricing) include access to hosted models with usage metered on a token basis. No API keys are required.

In Delta Web, use the model selector to choose which organization supplies your hosted models.

## [Sign in with a subscription](#sign-in-with-a-subscription)

Three providers connect through an existing subscription instead of an API key:

*   **ChatGPT**: sign in with your ChatGPT Plus or Pro subscription to use OpenAI models.
*   **GitHub Copilot**: sign in with your GitHub account to use models through Copilot Chat.
*   **Grok**: sign in with an eligible Grok or X subscription to use Grok models.

Connect any of them from Delta’s settings. ChatGPT and Grok connections sync through your Delta account, so those models are available on every device where you sign in. GitHub Copilot credentials stay on the device where you connect them.

**Settings > LLM Providers** shows the connected account and any account details the provider makes available.

On desktop, **ChatGPT Subscription** also shows your remaining usage limits and reset times when available. Choose **Subscription usage** to open ChatGPT’s usage page.

If you reconnect ChatGPT or Grok with a different account, Delta refreshes model access and uses the new account for subsequent requests.

> **Coming soon:** Sign in with a Claude subscription.

## [Add an API key](#add-an-api-key)

Add your provider’s API key in **Settings > LLM Providers**. Anthropic, OpenAI, Google AI Studio, OpenRouter, OpenCode Zen, and OpenCode Go support API-key connections. Delta saves provider keys in your account settings and syncs them across devices where the provider is supported.

Google AI Studio API keys are supported only in the desktop app, not in the browser. Requests go to Google; the models do not run on your computer. The available Google AI Studio model list refreshes without requiring a Delta release.

Generate a key in your provider’s dashboard, then paste it into that provider’s **API key** field. On desktop, if Delta detects an existing key, choose **Import existing key** beneath the field to copy it into settings. This can import an exported environment variable, or an OpenCode key saved by OpenCode. Importing copies the current value rather than linking to its source: changing the original key later does not change the key in Delta. To rotate or remove it, use **Settings > LLM Providers**.

OpenRouter shows a curated set of models that Delta has evaluated for agent work, along with supported models that match Zed-hosted equivalents. You can also add an exact OpenRouter model ID. Delta marks those user-added models as not evaluated, because their agentic capabilities can vary.

On desktop, OpenRouter’s **Allowed Providers** accepts comma-separated provider slugs to restrict which providers serve your requests. Leave it blank to use your OpenRouter account defaults. Set **Provider Order** to **Highest throughput** to prioritize faster providers, or leave it at **Default**.

> **Important:** Keep API keys out of your repositories. Add them in **Settings > LLM Providers**, not in project files.

## [Connect a compatible endpoint](#connect-a-compatible-endpoint)

Connect Delta to a local model server, an inference provider, or a gateway that implements an OpenAI- or Anthropic-compatible API.

Custom providers are currently only supported on desktop.

### [Add a provider](#add-a-provider)

1.  Open **Settings > LLM Providers** and choose **Add provider**.
2.  Enter a provider name. Delta uses this name in the model picker. For common providers, Delta tries to prefill the API URL, API mode, and models where possible.
3.  Check the prefilled **API URL**, or enter your provider’s [base URL](#choose-the-base-url).
4.  Enter an **API key** if your endpoint requires one. You can leave it blank for an endpoint that does not require authentication.
5.  Check the **API mode**, or choose the one your endpoint supports. See [Choose the base URL](#choose-the-base-url) for the available modes and their request paths.
6.  Check the models listed under **Models**, or choose **Add model** to enter one manually. See [Configure models](#configure-models) for the model fields and options.
7.  If your gateway requires additional HTTP headers, add them under **Custom headers**.
8.  Choose **Save provider**, then select one of its models in the thread’s model picker.

### [Choose the base URL](#choose-the-base-url)

Enter the base URL. Delta adds the path for the selected API mode:

API mode

Path Delta adds

Example base URL

OpenAI Chat Completions

`/chat/completions`

`https://models.example.com/v1`

OpenAI Responses

`/responses`

`https://models.example.com/v1`

Anthropic Messages

`/v1/messages`

`https://models.example.com`

For example, an OpenAI-compatible local server at `http://localhost:1234/v1` receives Chat Completions requests at `http://localhost:1234/v1/chat/completions`.

An Anthropic-compatible gateway at `https://models.example.com` receives Messages requests at `https://models.example.com/v1/messages`.

Use the base URL and API mode documented by your provider.

### [Configure models](#configure-models)

Each model needs:

*   **ID:** The exact model identifier accepted by the endpoint.
*   **Maximum tokens:** The model’s context window.
*   **Maximum output tokens:** The maximum number of tokens the model can generate in a single LLM response (text, tool call).

For models that support them, you can also configure **Reasoning efforts**, **Parallel tool calls**, and **Interleaved reasoning**. Leave optional capabilities unset unless your model and endpoint support them.

For OpenAI-compatible endpoints, **Reasoning efforts** accepts arbitrary effort names, such as `adaptive`. Of course, it must be accepted by your provider.

For an **Anthropic Messages** model, **Prompt caching** offers **Disabled**, **Explicit**, and **Automatic**. Explicit adds cache points with a five-minute lifetime; Automatic requires an endpoint that supports a one-hour cache lifetime. Choose a mode your endpoint supports.

### [Authentication and custom headers](#authentication-and-custom-headers)

Enter your API key in the **API key** field. Delta stores it in your account settings, which sync across devices and use an encrypted local cache.

If your gateway requires additional HTTP headers, add them under **Custom headers**. Header values are part of your account settings, which sync across devices and use an encrypted local cache. A local endpoint still needs to be reachable from the computer making the request.

Use `{thread_id}` in a header value to insert the requesting thread’s ID, for example to group requests by conversation in your gateway. We prefill this value for some common providers.

### [Compatibility and troubleshooting](#compatibility-and-troubleshooting)

Delta assumes that custom models support tool calls and images.

If a connection fails:

*   **Check the request URL.** Do not include `/chat/completions`, `/responses`, or `/v1/messages` in the base URL—Delta adds that path.
*   **Check the API mode.** Chat Completions, Responses, and Messages are different protocols.
*   **Check authentication.** Confirm the API key and any headers required by your gateway.
*   **Check the model ID and limits.** Use the identifier and token limits supported by your endpoint.
*   **For local servers, check that the server is running** and reachable from the computer running Delta.

If model discovery finds nothing, try adding the model manually.

## [Model roles](#model-roles)

Subagents use profiles to choose models. See [Subagents](/docs/agents/subagents#profiles) for the Worker, Scout, and Reviewer defaults and [model configuration](/docs/agents/subagents#models-and-configuration) for how profile choices and provider-specific preferences interact.

Delta also chooses provider-managed models for its own background work, such as titles and summaries. These are separate from your subagent profile choices. Use **Models Shown in Picker** in a provider’s settings to choose which models appear when you select a model for a thread.

Use the section’s switch to show or hide all currently listed models at once. Its label is **All Shown**, **Some Hidden**, or **All Hidden**, depending on your selection. Bulk changes apply to the current list; new models appear by default.

## [Choose a model](#choose-a-model)

You can switch which model a thread uses at any point; the conversation continues with the new model. Use faster models for quick, well-scoped tasks and higher-capability models for open-ended work across many files.

For models that support extended thinking, the model picker has a **Thinking** toggle and a **Thinking Effort** menu. Thinking starts enabled in new threads. Higher effort suits harder problems at the cost of speed. Delta remembers the default you choose for each model and applies it to new threads too.

[In a shared thread](/docs/collaboration/collaborate-thread), each participant reaches the thread’s model through their own providers: if you lack the exact provider a collaborator chose, Delta uses the equivalent model from a provider you do have when one is available. Otherwise, the selector marks the recorded model as unavailable. You can keep reading and commenting, but sending asks you to select an available model first.

### [Automatic fallback](#automatic-fallback)

When you run a turn on your computer and a Claude Fable model declines the request, Delta switches the thread to Claude Opus 5 and continues when that fallback is available from the same provider. Delta shows a notification explaining the change. If the fallback is unavailable, Delta shows the refusal.

## [Context windows and compaction](#context-windows-and-compaction)

Every model has a context window: the amount of conversation it can read in one request. Compaction replaces older conversation history with a smaller representation so a long thread can continue.

Click the circular context-usage indicator beside the send controls to open **Context Window**. There, inspect usage and turn **Automatic Compaction** on or off when the model supports it. When automatic compaction is on, drag **Compact at** to set its threshold. The circular indicator shows progress toward that threshold; hover over it for the percentage.

Choose **View Context Breakdown** to inspect how much of the prompt is used by instructions, messages, thinking, and tools when that information is available. The breakdown shows sizes in bytes and their share of the prompt, not token counts or cost.

Choose **Compact Now** from **Context Window** to compact the current thread. On desktop, it is also available from the command palette. Delta explains when manual compaction is unavailable, for example while a response is generating or when there is too little conversation to compact. If compaction fails, use **Retry** to try again without losing the conversation.

If the provider does not support native compaction, Delta asks the model to write a portable conversation summary. Native compaction behaves differently across providers:

*   Supported Anthropic models can compact automatically. **Compact Now** activates once the conversation is large enough to compact. Anthropic compaction includes a text summary and encrypted metadata for the backend that created it. When you switch to a different provider, Delta keeps the summary and leaves out metadata that the new provider cannot read.
*   Supported OpenAI models can compact automatically. OpenAI’s native compaction is encrypted provider state rather than a portable text summary. Delta can reuse compatible state between the OpenAI API and a ChatGPT subscription. Other providers cannot read it, so Delta replays the original conversation when you switch away from OpenAI.

If the provider rejects the conversation as too large after a model switch, Delta asks the selected model to summarize a bounded transcript, then retries the turn once. The summary can still fail or remain too large. If the summary request fails, Delta can offer **Retry compaction with** followed by the selected model’s name to try again.

If OpenAI rejects encrypted context that the selected model cannot read, Delta offers **Switch Back and Retry** when it can identify the model that created the context.

## [What’s next](#whats-next)

*   [Plans & Usage](/docs/account/plans-and-pricing) explains how your plan affects model access and spend limits.
*   [Settings](/docs/configuration/settings) covers Delta’s other configuration options.