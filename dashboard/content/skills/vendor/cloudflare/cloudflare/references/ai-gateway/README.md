+++
title = "README"
[extra]
skill = false
category = "vendor/cloudflare"
mermaid = false
skill_name = "cloudflare"
+++

# Cloudflare AI Gateway

Use AI Gateway to observe and control requests to AI providers through caching, rate limiting, logging, and routing.

Fetch the linked documentation before choosing endpoints, authentication headers, SDK options, model names, or limits. Keep implementation details in the current docs.

## Choose a task

| Task | Reference |
|------|-----------|
| Create a gateway or choose authentication and provider credentials | [Configuration](@/skills/vendor/cloudflare/cloudflare/references/ai-gateway/configuration.md) |
| Integrate an SDK, direct HTTP, or a Worker binding | [SDK integration](@/skills/vendor/cloudflare/cloudflare/references/ai-gateway/sdk-integration.md) |
| Configure caching, rate limits, security, billing, or logging | [Features](@/skills/vendor/cloudflare/cloudflare/references/ai-gateway/features.md) |
| Add fallbacks, conditional routing, or traffic splits | [Dynamic routing](@/skills/vendor/cloudflare/cloudflare/references/ai-gateway/dynamic-routing.md) |
| Diagnose failed requests, caching, or missing logs | [Troubleshooting](@/skills/vendor/cloudflare/cloudflare/references/ai-gateway/troubleshooting.md) |

For new single-model calls, start with the [REST API](https://developers.cloudflare.com/ai-gateway/usage/rest-api/index.md) or [Workers bindings](https://developers.cloudflare.com/ai-gateway/usage/worker-binding-methods/index.md), depending on the runtime. Preserve provider-native integrations when their API shape is needed; use the corresponding [provider guide](https://developers.cloudflare.com/ai-gateway/usage/providers/index.md).

The [legacy Unified API](https://developers.cloudflare.com/ai-gateway/usage/chat-completion/index.md) is deprecated for single-model calls but remains required for dynamic routes. Check the task before changing an existing endpoint.

Gateway authentication and upstream provider credentials are separate concerns. Choose the endpoint first, then follow its authentication and billing requirements in [configuration](@/skills/vendor/cloudflare/cloudflare/references/ai-gateway/configuration.md).

## Related references

- [Workers AI](@/skills/vendor/cloudflare/cloudflare/references/workers-ai/README.md) — model inference.
- [Agents SDK documentation](https://developers.cloudflare.com/agents/index.md) — stateful agents.
- [Vectorize](@/skills/vendor/cloudflare/cloudflare/references/vectorize/README.md) — vector search.

