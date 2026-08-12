+++
title = "developers-advanced-api-usage-prompt-caching-how-it-works"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Prompt Caching

# How It Works

The cache works from the **start of your messages array**. When a request arrives, the system checks how many messages at the beginning match a previous request exactly — that matching portion is the "prefix" and gets served from cache:

1. **First request** — The full prompt is processed and cached
2. **Subsequent requests** — If the prompt prefix matches, the cached portion is reused (a cache *hit*)
3. **Billing** — Cached tokens are billed at a reduced rate

> [!WARNING]
>
> Prompt caching is not 100% guaranteed. Cache entries can be evicted due to memory pressure, and requests may be routed to different servers. Use `x-grok-conv-id` to maximize cache hit rates.

## Example

**Request 1:**

```text
[system] "You are a helpful assistant."
[user] "What is the capital of France?"
[assistant] "The capital of France is Paris."
```

**Request 2:**

```text
[system] "You are a helpful assistant."       ← cached
[user] "What is the capital of France?"       ← cached
[assistant] "The capital of France is Paris." ← cached
[user] "What about Germany?"                  ← new
```

The first 3 messages match Request 1 exactly, so they're served from cache. Only the new message is computed.

## Next

* [Maximizing Cache Hits](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/maximizing-cache-hits)
* [What Breaks Caching](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/multi-turn)
* [Usage & Pricing](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/usage-and-pricing)
* [Best Practices & FAQ](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/best-practices)
{% endraw %}
