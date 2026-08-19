+++
title = "developers-grok-4-6"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Get Started

# Grok 4.6

Grok 4.6 is SpaceXAI's frontier model built for coding, agentic tasks, and knowledge work.

## Using the API

If you already have an [API key](https://console.x.ai/team/default/api-keys), set the model name to `grok-4.6`:

```python customLanguage="pythonXAI"
import os
from xai_sdk import Client
from xai_sdk.chat import user

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(model="grok-4.6")
chat.append(user("Find and fix the bug, then explain it: function median(a){a.sort();return a[a.length/2]}"))

response = chat.sample()
print(response.content)
```

```javascript customLanguage="javascriptAISDK"
import { xai } from '@ai-sdk/xai';
import { generateText } from 'ai';

const { text } = await generateText({
  model: xai.responses('grok-4.6'),
  prompt:
    'Find and fix the bug, then explain it: function median(a){a.sort();return a[a.length/2]}',
});

console.log(text);
```

```javascript customLanguage="javascriptOpenAISDK"
import OpenAI from 'openai';

const client = new OpenAI({
  apiKey: process.env.XAI_API_KEY,
  baseURL: 'https://api.x.ai/v1',
});

const response = await client.responses.create({
  model: 'grok-4.6',
  input: [
    {
      role: 'user',
      content:
        'Find and fix the bug, then explain it: function median(a){a.sort();return a[a.length/2]}',
    },
  ],
});

console.log(response.output_text);
```

```bash customLanguage="bash"
curl https://api.x.ai/v1/responses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
    "model": "grok-4.6",
    "input": "Find and fix the bug, then explain it: function median(a){a.sort();return a[a.length/2]}"
  }'
```

New to the xAI API? Follow the [Quickstart](https://docs.x.ai/developers/quickstart) to create an account and make your first request.

## At a glance

| Property | Value |
|----------|-------|
| Model name | `grok-4.6` |
| Context window | 500,000 tokens |
| Knowledge cutoff | February 1, 2026 |
| Modalities | Text and image input; text output |
| Output limit | No text output limit |
| Input price | $2.00 / 1M tokens |
| Output price | $6.00 / 1M tokens |
| Reasoning | Low, medium, high (default), or xhigh  |
| APIs | [Responses API](https://docs.x.ai/developers/rest-api-reference/inference/chat#create-new-response), [Chat Completions](https://docs.x.ai/developers/rest-api-reference/inference/chat#chat-completions) |
| Tools | [Function calling](https://docs.x.ai/developers/tools/function-calling), [web search](https://docs.x.ai/developers/tools/web-search), [X search](https://docs.x.ai/developers/tools/x-search), [code execution](https://docs.x.ai/developers/tools/code-execution) |

Rate limits and live pricing for your team are on the [model detail page](https://docs.x.ai/developers/models/grok-4.6) and [Pricing](https://docs.x.ai/developers/pricing).

For benchmark results and demos, see the [announcement](https://x.ai/news/grok-4-6).

## Important details

* **We highly recommend setting a [`prompt_cache_key`](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/maximizing-cache-hits)** (Responses API; `x-grok-conv-id` header on Chat Completions). It routes a conversation's requests to the same server, making cache hits reliable; without it you often pay full input price on a cache-cold server. See [What Breaks Caching](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/multi-turn) for common mistakes.
* **Long agent loops** additionally benefit from [context compaction](https://docs.x.ai/developers/advanced-api-usage/context-compaction); for tool-heavy workloads see [function calling](https://docs.x.ai/developers/tools/function-calling).

## Where it runs

* **xAI API**: get a key from the [console](https://console.x.ai/)
* **Grok Build**: the default model of the [coding agent](https://docs.x.ai/build/overview), on the API and CLI
* **Cursor**: available on all plans
* **Model gateways**: OpenRouter, Vercel, and Cloudflare

## Learn more

* [Reasoning](https://docs.x.ai/developers/model-capabilities/text/reasoning#the-reasoning_effort-parameter) - controlling `reasoning_effort`, including `"xhigh"`
* [Announcement](https://x.ai/news/grok-4-6) - launch post with demos and full benchmark figures
* [Models](https://docs.x.ai/developers/models) - compare available models and their capabilities
* [Pricing](https://docs.x.ai/developers/pricing) - token pricing for all models
{% endraw %}
