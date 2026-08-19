+++
title = "developers-cost-tracking"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Key Information

# Cost Tracking

Every inference response from the xAI API includes the exact cost you were charged for that request, returned via a `cost_in_usd_ticks` field in the `usage` object of chat completions, Responses API, image generation, and video generation responses.

The cost is per-request: each call returns what that individual request cost, whether it's a simple completion, a streaming response, or an agentic loop with server-side tools. This is the actual amount billed, after all applicable discounts (including [prompt caching](https://docs.x.ai/developers/advanced-api-usage/prompt-caching) reductions) have been applied, and inclusive of all token costs and server-side tool invocation costs. No estimation or after-the-fact billing lookup required.

## How it works

The cost is expressed in **ticks**, where 1 USD = 10,000,000,000 ticks (10^10). To convert to dollars:

```text
cost_usd = cost_in_usd_ticks / 10,000,000,000
```

For example, a response with `"cost_in_usd_ticks": 37756000` cost $0.0038. An image generation with `"cost_in_usd_ticks": 200000000` cost $0.02.

Ticks exist for precision: they represent costs down to fractions of a cent without floating-point rounding, which matters when you're processing thousands of requests and need the totals to add up.

## Reading cost from a response

### xAI SDK

The xAI SDK provides a `cost_usd` convenience property that converts ticks to dollars automatically. The raw ticks are also accessible via `response.usage.cost_in_usd_ticks` if you need integer precision:

```python customLanguage="pythonXAI"
import os
from xai_sdk import Client
from xai_sdk.chat import user

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(
    model="grok-4.6",
    messages=[user("Say hello")],
)
response = chat.sample()

# Convenience property — ticks converted to dollars.
print(f"Cost: ${response.cost_usd:.6f}")

# Raw ticks for integer-precision accounting.
print(f"Cost (ticks): {response.usage.cost_in_usd_ticks}")
```

### Chat Completions and Responses API

The `usage` object in every REST completion and response includes `cost_in_usd_ticks`:

```json
"usage": {
    "input_tokens": 199,
    "output_tokens": 1,
    "total_tokens": 200,
    "cost_in_usd_ticks": 158500
}
```

```bash customLanguage="bash"
curl https://api.x.ai/v1/responses \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-4.6",
    "input": "Say hello"
  }' | jq '.usage.cost_in_usd_ticks'
```

```python customLanguage="pythonOpenAISDK"
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

completion = client.chat.completions.create(
    model="grok-4.6",
    messages=[{"role": "user", "content": "Say hello"}],
)

# cost_in_usd_ticks is available directly on the usage object.
cost_ticks = completion.usage.cost_in_usd_ticks
cost_usd = cost_ticks / 1e10
print(f"Cost: ${cost_usd:.6f}")
```

```javascript customLanguage="javascriptOpenAISDK"
import OpenAI from "openai";

const client = new OpenAI({
  apiKey: process.env.XAI_API_KEY,
  baseURL: "https://api.x.ai/v1",
});

const completion = await client.chat.completions.create({
  model: "grok-4.6",
  messages: [{ role: "user", content: "Say hello" }],
});

const costTicks = completion.usage.cost_in_usd_ticks;
const costUsd = costTicks / 1e10;
console.log(`Cost: $${costUsd.toFixed(6)}`);
```

> [!NOTE]
>
> The Vercel AI SDK (`@ai-sdk/xai`) does not currently surface `cost_in_usd_ticks` in its response metadata. To access it, use the OpenAI SDK or the raw REST API directly.

### Streaming

When using the xAI SDK for streaming, each chunk carries a running `cost_in_usd_ticks` total; the last chunk reflects the final cost for the request. The assembled `Response` object carries this automatically.

When using the OpenAI SDK or the REST API, set `stream_options: { include_usage: true }` on the request. Cost is only included in the final chunk (with empty `choices`); intermediate chunks do not contain usage data.

```python customLanguage="pythonXAI"
import os
from xai_sdk import Client
from xai_sdk.chat import user

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(
    model="grok-4.6",
    messages=[user("Tell me a joke")],
)

for response, chunk in chat.stream():
    print(chunk.content, end="", flush=True)
print()

# After the stream completes, cost is on the final response.
print(f"Cost: ${response.cost_usd:.6f}")
```

```python customLanguage="pythonOpenAISDK"
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

stream = client.chat.completions.create(
    model="grok-4.6",
    messages=[{"role": "user", "content": "Tell me a joke"}],
    stream=True,
    stream_options={"include_usage": True},
)

for chunk in stream:
    if chunk.usage:
        cost_ticks = chunk.usage.cost_in_usd_ticks
        print(f"\nCost: ${cost_ticks / 1e10:.6f}")
    elif chunk.choices:
        print(chunk.choices[0].delta.content or "", end="", flush=True)
```

## Tracking cost across a conversation

`cost_in_usd_ticks` is per-request; it does not accumulate across turns. In a multi-turn conversation, sum the costs yourself:

```python customLanguage="pythonXAI"
import os
from xai_sdk import Client
from xai_sdk.chat import system, user

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(
    model="grok-4.6",
    messages=[system("You are a helpful assistant.")],
)

total_cost_usd = 0.0
while True:
    prompt = input("You: ")
    if prompt.lower() == "exit":
        break

    chat.append(user(prompt))
    response = chat.sample()
    print(f"Grok: {response.content}")
    chat.append(response)

    total_cost_usd += response.cost_usd or 0.0
    print(f"  (this turn: ${response.cost_usd or 0:.6f})")

print(f"Total session cost: ${total_cost_usd:.4f}")
```

```python customLanguage="pythonOpenAISDK"
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

messages = [{"role": "system", "content": "You are a helpful assistant."}]
total_cost_usd = 0.0

while True:
    prompt = input("You: ")
    if prompt.lower() == "exit":
        break

    messages.append({"role": "user", "content": prompt})
    completion = client.chat.completions.create(
        model="grok-4.6",
        messages=messages,
    )

    reply = completion.choices[0].message.content
    print(f"Grok: {reply}")
    messages.append({"role": "assistant", "content": reply})

    cost_ticks = completion.usage.cost_in_usd_ticks
    cost_usd = cost_ticks / 1e10
    total_cost_usd += cost_usd
    print(f"  (this turn: ${cost_usd:.6f})")

print(f"Total session cost: ${total_cost_usd:.4f}")
```

## Server-side tools

When a request uses server-side tools (web search, X search, code execution), the model may make multiple internal calls before returning a final answer. The returned `cost_in_usd_ticks` covers all token costs and all tool invocations from that request in a single value. No separate accumulation needed.

```python customLanguage="pythonXAI"
import os
from xai_sdk import Client
from xai_sdk.chat import user
from xai_sdk.tools import web_search, x_search

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(
    model="grok-4.6",
    tools=[web_search(), x_search()],
)
chat.append(user("What are people saying about xAI's latest announcement?"))

response = chat.sample()
print(response.content)

# Shows which server-side tools were invoked and how many times.
print(f"Tools used: {response.server_side_tool_usage}")
# Cost covers all model decodes + every tool call in the agentic loop.
print(f"Cost: ${response.cost_usd:.4f}")
```

```python customLanguage="pythonOpenAISDK"
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="grok-4.6",
    input="What are people saying about xAI's latest announcement?",
    tools=[
        {"type": "web_search"},
        {"type": "x_search"},
    ],
)

print(response.output_text)

# Cost covers all model decodes + every tool call in the agentic loop.
cost_ticks = response.usage.cost_in_usd_ticks
print(f"Cost: ${cost_ticks / 1e10:.4f}")
```

```bash customLanguage="bash"
curl https://api.x.ai/v1/responses \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-4.6",
    "tools": [{"type": "web_search"}, {"type": "x_search"}],
    "input": "What are people saying about xAI'\''s latest announcement?"
  }' | jq '{tools_used: .usage.num_server_side_tools_used, cost_in_usd_ticks: .usage.cost_in_usd_ticks}'
```

## Image and video generation

Image and video responses include the same `cost_in_usd_ticks` field in their `usage` object:

```bash customLanguage="bash"
# Image generation
curl https://api.x.ai/v1/images/generations \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-imagine-image-2.0",
    "prompt": "A cat on a rocket"
  }' | jq '.usage.cost_in_usd_ticks'
# => 200000000 ($0.02)
```

```python customLanguage="pythonXAI"
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Image generation
image = client.image.sample(
    model="grok-imagine-image-2.0",
    prompt="A cat on a rocket",
)
print(f"Image cost: ${image.cost_usd:.4f}")

# Video generation
video = client.video.generate(
    model="grok-imagine-video-1.5",
    prompt="A cat floating in space",
)
print(f"Video cost: ${video.cost_usd:.4f}")
```

```python customLanguage="pythonOpenAISDK"
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

response = client.images.generate(
    model="grok-imagine-image-2.0",
    prompt="A cat on a rocket",
)

cost_ticks = response.usage.cost_in_usd_ticks
print(f"Image cost: ${cost_ticks / 1e10:.4f}")
```

## Batch API

Batch results include per-request costs. You can sum them to get the total batch cost, or read the `cost_breakdown` on the batch object itself. See [Batch API](https://docs.x.ai/developers/advanced-api-usage/batch-api) for details.
{% endraw %}
