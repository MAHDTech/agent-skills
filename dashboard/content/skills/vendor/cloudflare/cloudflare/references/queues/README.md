+++
title = "README"
[extra]
skill = false
category = "vendor/cloudflare"
mermaid = false
skill_name = "cloudflare"
+++

# Cloudflare Queues

Use Queues to decouple producers from asynchronous consumers and buffer bursts of work. Design consumers for duplicate delivery; use Workflows when the task needs durable multi-step orchestration.

Fetch the relevant documentation below before implementing. Treat current Cloudflare docs as the source of truth for API signatures, acknowledgement semantics, configuration, limits, and pricing.

## Choose a consumer

- Use a Worker push consumer when processing runs on Workers.
- Use an HTTP pull consumer when processing runs in another environment; plan for polling, visibility timeouts, and acknowledgement leases.
- Choose a message encoding the consumer can decode. Check serialization and compatibility-date behavior before sending existing application objects.

See [How Queues works](https://developers.cloudflare.com/queues/reference/how-queues-works/index.md) and [delivery guarantees](https://developers.cloudflare.com/queues/reference/delivery-guarantees/index.md) before choosing ordering or deduplication strategies.

## Read by task

| Task | Reference |
|------|-----------|
| Create queues, bind producers, and configure consumers | [configuration.md](@/skills/vendor/cloudflare/cloudflare/references/queues/configuration.md) |
| Send messages and implement acknowledgement or retries | [api.md](@/skills/vendor/cloudflare/cloudflare/references/queues/api.md) |
| Buffer APIs, defer jobs, or integrate with storage and orchestration | [patterns.md](@/skills/vendor/cloudflare/cloudflare/references/queues/patterns.md) |
| Diagnose delivery failures, duplicates, or capacity issues | [gotchas.md](@/skills/vendor/cloudflare/cloudflare/references/queues/gotchas.md) |

For a first application, fetch [Getting started](https://developers.cloudflare.com/queues/get-started/index.md). Retrieve [limits](https://developers.cloudflare.com/queues/platform/limits/index.md) and [pricing](https://developers.cloudflare.com/queues/platform/pricing/index.md) before sizing throughput, retention, or cost; plan-specific values are not maintained here.

