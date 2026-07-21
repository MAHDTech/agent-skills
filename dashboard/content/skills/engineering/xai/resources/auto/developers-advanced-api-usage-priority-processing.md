+++
title = "developers-advanced-api-usage-priority-processing"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Advanced API Usage

# Priority Processing

Priority Processing gives your xAI API requests higher scheduling priority, which typically results in lower time-to-first-token (TTFT) and faster inter-token latency (ITL), especially during periods of high demand. Add `service_tier: "priority"` to any request body to opt in—no capacity reservations or advance provisioning required. The parameter is supported on text inference endpoints: Chat Completions and Responses.

When priority capacity is available, requests are scheduled ahead of standard traffic. The response always includes a `service_tier` field indicating whether priority was granted; check it to confirm.

## How it works

Add the `service_tier` field to any supported request. The API returns the tier that was actually used in the response, so you can confirm the upgrade took effect.

The `service_tier` field accepts the following values:

| Value | Meaning |
|-------|---------|
| `"default"` | Standard processing. This is the same as omitting the field entirely. |
| `"priority"` | Request higher scheduling priority at a premium token price. |

Priority requests are billed at a premium per-token rate. Cache discounts still apply to cached input tokens before the multiplier. For current per-model rates and the exact priority premium, see the [Pricing](https://docs.x.ai/developers/pricing) page.

## Quick start

Pass `service_tier: "priority"` in your request body. The response includes a `service_tier` field confirming which tier was used.

```bash customLanguage="bash"
curl https://api.x.ai/v1/responses \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-4.5",
    "input": "Explain the Riemann hypothesis in one paragraph.",
    "service_tier": "priority"
  }'
```

```python customLanguage="pythonXAI"
import os

from xai_sdk import Client
from xai_sdk.chat import user

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(
    model="grok-4.5",
    service_tier="priority",
)
chat.append(user("Explain the Riemann hypothesis in one paragraph."))

response = chat.sample()

print(response.content)
print(f"Tier used: {response.service_tier}")
```

```python customLanguage="pythonOpenAISDK"
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="grok-4.5",
    input="Explain the Riemann hypothesis in one paragraph.",
    service_tier="priority",
)

print(response.output_text)
print(f"Tier used: {response.service_tier}")
```

```javascript customLanguage="javascriptOpenAISDK"
import OpenAI from "openai";

const client = new OpenAI({
  apiKey: process.env.XAI_API_KEY,
  baseURL: "https://api.x.ai/v1",
});

const response = await client.responses.create({
  model: "grok-4.5",
  input: "Explain the Riemann hypothesis in one paragraph.",
  service_tier: "priority",
});

console.log(response.output_text);
console.log(`Tier used: ${response.service_tier}`);
```

The response includes `"service_tier": "priority"` when the request was served at the priority tier, or `"service_tier": "default"` if it was served at the default tier instead. You are only billed at the priority rate when the response confirms `"priority"`.

```json customLanguage="json"
{
  "id": "resp_abc123",
  "model": "grok-4.5",
  "service_tier": "priority",
  "usage": {
    "input_tokens": 42,
    "output_tokens": 156,
    "cost_in_usd_ticks": 37756000
  }
}
```

## Best practices

* **Latency-sensitive paths first** — Priority Processing is most valuable for user-facing requests where response time directly affects experience. Background jobs, evaluations, and bulk processing are better served by the [Batch API](https://docs.x.ai/developers/advanced-api-usage/batch-api).
* **Monitor the `service_tier` field** — Log the returned tier to track how often your requests are served at priority versus default and to correlate with your latency metrics.
* **Combine with prompt caching** — Cached input tokens are discounted before the priority multiplier is applied, so [prompt caching](https://docs.x.ai/developers/advanced-api-usage/prompt-caching) and priority processing complement each other well.
