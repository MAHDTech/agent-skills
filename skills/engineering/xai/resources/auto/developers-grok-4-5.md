#### Get Started

# Grok 4.5

Grok 4.5 is SpaceXAI's frontier model built for coding, agentic tasks, and knowledge work. It was trained in SpaceXAI's data centers in Memphis with new datasets spanning science, engineering, and math.

> Grok 4.5 isn't available in the API console for EU users yet. Availability is expected later this month.

## Using the API

If you already have an [API key](https://console.x.ai/team/default/api-keys), set the model name to `grok-4.5`:

```python customLanguage="pythonXAI"
import os
from xai_sdk import Client
from xai_sdk.chat import user

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(model="grok-4.5")
chat.append(user("Find and fix the bug, then explain it: function median(a){a.sort();return a[a.length/2]}"))

response = chat.sample()
print(response.content)
```

```javascript customLanguage="javascriptAISDK"
import { xai } from '@ai-sdk/xai';
import { generateText } from 'ai';

const { text } = await generateText({
  model: xai.responses('grok-4.5'),
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
  model: 'grok-4.5',
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
    "model": "grok-4.5",
    "input": "Find and fix the bug, then explain it: function median(a){a.sort();return a[a.length/2]}"
  }'
```

New to the xAI API? Follow the [Quickstart](https://docs.x.ai/developers/quickstart) to create an account and make your first request.

## At a glance

| Property | Value |
|----------|-------|
| Model name | `grok-4.5` |
| Knowledge cutoff | February 1, 2026 |
| Input price | $2.00 / 1M tokens |
| Output price | $6.00 / 1M tokens |
| Reasoning | Low, medium, or high (default high)  |
| APIs | [Responses API](https://docs.x.ai/developers/rest-api-reference/inference/chat#create-new-response), [Chat Completions](https://docs.x.ai/developers/rest-api-reference/inference/chat#chat-completions) |
| Tools | [Function calling](https://docs.x.ai/developers/tools/function-calling), [web search](https://docs.x.ai/developers/tools/web-search), [X search](https://docs.x.ai/developers/tools/x-search), [code execution](https://docs.x.ai/developers/tools/code-execution) |

Full context window, rate limits, and live pricing for your team are on the [model detail page](https://docs.x.ai/developers/models/grok-4.5) and [Pricing](https://docs.x.ai/developers/pricing).

For benchmark results and cost-versus-score comparisons, see the [announcement](https://x.ai/news/grok-4-5).

## Important details

* **We highly recommend setting a [`prompt_cache_key`](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/maximizing-cache-hits)** (Responses API; `x-grok-conv-id` header on Chat Completions). It routes a conversation's requests to the same server, making cache hits reliable; without it you often pay full input price on a cache-cold server. See [What Breaks Caching](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/multi-turn) for common mistakes.
* **Long agent loops** additionally benefit from [context compaction](https://docs.x.ai/developers/advanced-api-usage/context-compaction); for tool-heavy workloads see [function calling](https://docs.x.ai/developers/tools/function-calling).

## Where it runs

* **xAI API**: get a key from the [console](https://console.x.ai/)
* **Grok Build**: the default model of the [coding agent](https://docs.x.ai/build/overview), on the API and CLI
* **Cursor**: available on all plans
* **Office add-ins**: default model in the Word, PowerPoint, and Excel add-ins
* **Model gateways**: OpenRouter, Vercel, Cloudflare, Snowflake, and Databricks Mosaic

## Learn more

* [Reasoning](https://docs.x.ai/developers/model-capabilities/text/reasoning#the-reasoning_effort-parameter) - controlling `reasoning_effort`
* [Announcement](https://x.ai/news/grok-4-5) - launch post with demos and full benchmark figures
* [Models](https://docs.x.ai/developers/models) - compare available models and their capabilities
* [Pricing](https://docs.x.ai/developers/pricing) - token pricing for all models