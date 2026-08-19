#### Quickstart

# Quickstart

Welcome! In this guide, we'll walk you through the basics of using the xAI API, from creating an account to making your first request.

## Step 1: Create an xAI account

Sign up for an account at [accounts.x.ai](https://accounts.x.ai/sign-up?redirect=cloud-console), then load it with credits to start using the API.

## Step 2: Generate an API key

Create an API key via the [API Keys page](https://console.x.ai/team/default/api-keys), then export it or add it as an environment variable.

```bash
export XAI_API_KEY="your_api_key"
```

Or add it to a `.env` file in your project directory:

```bash
XAI_API_KEY=your_api_key
```

## Step 3: Install an SDK

Pick your language and install the SDK:

```bash customLanguage="pythonXAI"
pip install xai-sdk
```

```bash customLanguage="pythonOpenAISDK"
pip install openai
```

```bash customLanguage="javascriptAISDK"
npm install ai @ai-sdk/xai zod
```

```bash customLanguage="javascriptOpenAISDK"
npm install openai
```

## Step 4: Make your first request

Send a coding prompt to [Grok Build](https://docs.x.ai/build/overview) (`grok-4.6`) and get a response. The same model powers agentic coding in Grok Build and is available on the API in early access:

```bash
curl https://api.x.ai/v1/responses \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-4.6",
    "input": "Fix this function and explain the bug: function median(a){a.sort();return a[a.length/2]}"
  }'
```

```python customLanguage="pythonXAI"
import os
from xai_sdk import Client
from xai_sdk.chat import user

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(model="grok-4.6")
chat.append(user("Fix this function and explain the bug: function median(a){a.sort();return a[a.length/2]}"))

print(chat.sample().content)
```

```python customLanguage="pythonOpenAISDK"
from openai import OpenAI

client = OpenAI(
    api_key="<YOUR_XAI_API_KEY_HERE>",
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="grok-4.6",
    input="Fix this function and explain the bug: function median(a){a.sort();return a[a.length/2]}",
)

print(response.output_text)
```

```javascript customLanguage="javascriptAISDK"
import { xai } from '@ai-sdk/xai';
import { generateText } from 'ai';

const { text } = await generateText({
  model: xai.responses('grok-4.6'),
  prompt: 'Fix this function and explain the bug: function median(a){a.sort();return a[a.length/2]}',
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
  input: 'Fix this function and explain the bug: function median(a){a.sort();return a[a.length/2]}',
});

console.log(response.output_text);
```

For multi-turn chat, reasoning, and [structured outputs](https://docs.x.ai/developers/model-capabilities/text/structured-outputs), see the [Text Generation Guide](https://docs.x.ai/developers/model-capabilities/text/generate-text). For agentic coding workflows, see the [Grok Build overview](https://docs.x.ai/build/overview). For AI teammates on a cloud computer, see [Grok Bot](https://docs.x.ai/grok-bot/overview).

## Step 5: Generate an image

Use the Imagine API to generate images from text prompts:

```python customLanguage="pythonXAI"
import os
import xai_sdk

client = xai_sdk.Client(api_key=os.getenv("XAI_API_KEY"))

response = client.image.sample(
    prompt="A futuristic city skyline at sunset",
    model="grok-imagine-image-2.0",
)

print(response.url)
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
    prompt="A futuristic city skyline at sunset",
)

print(response.data[0].url)
```

```javascript customLanguage="javascriptAISDK"
import { xai } from '@ai-sdk/xai';
import { generateImage } from 'ai';

const { image } = await generateImage({
  model: xai.image('grok-imagine-image-2.0'),
  prompt: 'A futuristic city skyline at sunset',
});

console.log(image.base64);
```

```javascript customLanguage="javascriptOpenAISDK"
import OpenAI from 'openai';

const client = new OpenAI({
  apiKey: process.env.XAI_API_KEY,
  baseURL: 'https://api.x.ai/v1',
});

const response = await client.images.generate({
  model: 'grok-imagine-image-2.0',
  prompt: 'A futuristic city skyline at sunset',
});

console.log(response.data[0].url);
```

```bash
curl -X POST https://api.x.ai/v1/images/generations \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
    "model": "grok-imagine-image-2.0",
    "prompt": "A futuristic city skyline at sunset"
  }'
```

For more advanced use cases like batch generation, aspect ratio control, and image editing, check out the [Image Generation Guide](https://docs.x.ai/developers/model-capabilities/images/generation).

## What's next

Now that you've made your first request, explore what Grok can do:

### Resources

* [Streaming](https://docs.x.ai/developers/model-capabilities/text/streaming) - Stream responses in real time
* [Files & Collections](https://docs.x.ai/developers/files) - Upload documents and build RAG pipelines
* [Tools](https://docs.x.ai/developers/tools/overview) - Web search, X search, code execution, and function calling
* [Models](https://docs.x.ai/developers/models) - Compare available models and their capabilities
* [Pricing](https://docs.x.ai/developers/pricing) - Tools, batch API, and other platform pricing
* [Grok Bot](https://docs.x.ai/grok-bot/overview) - AI teammates on a persistent cloud computer
* [Grok Build](https://docs.x.ai/build/overview) - Agentic coding CLI and API