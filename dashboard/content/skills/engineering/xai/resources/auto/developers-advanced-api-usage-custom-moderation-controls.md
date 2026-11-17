+++
title = "developers-advanced-api-usage-custom-moderation-controls"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Advanced API Usage

# Custom Moderation Controls

Add a `guidelines` array to a request to set how Grok further handles specific content categories. For each category you include, you choose a further enhanced behavior. Today the only behavior is `decline`. Categories you leave out continue to keep the model's default moderation behavior.

Grok follows the guidelines as it writes its response, in its own words. The response has the same shape as a response without guidelines.

## Categories and behaviors

Each entry in `guidelines` has three fields: `type`, which is always `"policy"`, a `category`, and the `behavior` you want for that category.

| Category | Behavior | What Grok does |
|----------|----------|----------------|
| `nsfw` | `decline` | Avoids sexual innuendo or sexualized conversations. Still answers educational and clinical questions about sexual topics. |
| `violence` | `decline` | Avoids depictions of graphic violence. Engages with history and news without graphic detail. |
| `profanity` | `decline` | Writes without using profanity. Declines when the request is for profanity alone, or when the request is highly offensive. |

The table describes each behavior at a high level. SpaceXSI may refine the details over time.

A category you leave out keeps the model's default behavior, so list only the categories you want to change.

## Set guidelines on a request

This example request sets `violence` and `nsfw` to `decline`.

```python customLanguage="pythonOpenAISDK"
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="grok-4.7",
    input="Write a short story about a knight.",
    extra_body={
        "guidelines": [
            {"type": "policy", "category": "violence", "behavior": "decline"},
            {"type": "policy", "category": "nsfw", "behavior": "decline"},
        ],
    },
)

print(response.output_text)
```

```bash customLanguage="bash"
curl https://api.x.ai/v1/responses \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-4.7",
    "input": "Write a short story about a knight.",
    "guidelines": [
      { "type": "policy", "category": "violence", "behavior": "decline" },
      { "type": "policy", "category": "nsfw", "behavior": "decline" }
    ]
  }'
```

```javascript customLanguage="javascriptOpenAISDK"
import OpenAI from "openai";

const client = new OpenAI({
  apiKey: process.env.XAI_API_KEY,
  baseURL: "https://api.x.ai/v1",
});

const response = await client.responses.create({
  model: "grok-4.7",
  input: "Write a short story about a knight.",
  guidelines: [
    { type: "policy", category: "violence", behavior: "decline" },
    { type: "policy", category: "nsfw", behavior: "decline" },
  ],
});

console.log(response.output_text);
```

```python customLanguage="pythonXAI"
import os

from xai_sdk import Client
from xai_sdk.chat import user

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(
    model="grok-4.7",
    guidelines=[
        {"type": "policy", "category": "violence", "behavior": "decline"},
        {"type": "policy", "category": "nsfw", "behavior": "decline"},
    ],
)
chat.append(user("Write a short story about a knight."))

response = chat.sample()
print(response.content)
```

The OpenAI SDKs do not define `guidelines`. With the OpenAI Python SDK, pass it in `extra_body`. The OpenAI JavaScript SDK sends it with the other request parameters. On Chat Completions, add the same `guidelines` array as a top-level field of the request body, at the same level as `messages`.

Guidelines apply only to the request that carries them. To apply them to a whole conversation, send them on every request in it. In the xAI Python SDK, `chat.create` stores them on the chat, so every `sample` call sends them.

## Requests that cannot use guidelines

Guidelines currently do not work with multi-agent models, such as `grok-4.20-multi-agent`, or with deferred completions. A request with no server-side tools and no file attachments also cannot use guidelines if it sets `stop` or sets `n` greater than 1. The API rejects these requests with `400` before the model runs. The error message names the model or the option.

## Errors

The API checks `guidelines` before it runs the model. If it cannot apply them, it returns an error and does not run the model.

| Status | Cause |
|--------|-------|
| `422` | The `category` or `behavior` is not a known value. The error lists the valid values. |
| `400` | The same category appears more than once. |
| `400` | The `type` is not `"policy"`. |
| `400` | The request cannot use guidelines, as described in [Requests that cannot use guidelines](#requests-that-cannot-use-guidelines). The message names the model or the option. |

In the xAI Python SDK, an unknown category or behavior raises `ValueError` before the SDK sends the request. The API returns the other errors as a gRPC `INVALID_ARGUMENT` status.

## Supported APIs

`guidelines` is accepted on [Chat Completions](/developers/rest-api-reference/inference/chat-completions) and the [Responses API](/developers/rest-api-reference/inference/responses), and on `chat.create` in the xAI Python SDK.


