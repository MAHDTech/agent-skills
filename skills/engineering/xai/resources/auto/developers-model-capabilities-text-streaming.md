#### Model Capabilities

# Streaming

Streaming outputs is **supported by all models with text output capability** (Chat, Image Understanding, etc.). It is **not supported by models with image output capability** (Image Generation).

Streaming outputs uses [Server-Sent Events (SSE)](https://en.wikipedia.org/wiki/Server-sent_events) that let the server send back the delta of content in event streams.

Streaming responses are beneficial for providing real-time feedback, enhancing user interaction by allowing text to be displayed as it's generated.

To enable streaming, you must set `"stream": true` in your request.

> [!CAUTION]
>
> When using streaming output with reasoning models, you might want to **manually override request
> timeout** to avoid prematurely closing connection.

```pythonXAI
import os

from xai_sdk import Client
from xai_sdk.chat import user, system

client = Client(
    api_key=os.getenv('XAI_API_KEY'),
    timeout=3600, # Override default timeout with longer timeout for reasoning models
)

chat = client.chat.create(model="grok-4.6")
chat.append(
    system("You are Grok, a helpful and maximally truthful AI built by xAI."),
)
chat.append(
    user("Explain how neural networks learn in two sentences.")
)

for response, chunk in chat.stream():
    print(chunk.content, end="", flush=True) # Each chunk's content
    print(response.content, end="", flush=True) # The response object auto-accumulates the chunks

print(response.content) # The full response
```

```pythonOpenAISDK
import os
import httpx
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
    timeout=httpx.Timeout(3600.0) # Timeout after 3600s for reasoning models
)

stream = client.chat.completions.create(
    model="grok-4.6",
    messages=[
        {"role": "system", "content": "You are Grok, a helpful and maximally truthful AI built by xAI."},
        {"role": "user", "content": "Explain how neural networks learn in two sentences."},
    ],
    stream=True # Set streaming here
)

for chunk in stream:
    print(chunk.choices[0].delta.content, end="", flush=True)
```

```javascriptOpenAISDK
import OpenAI from "openai";
const openai = new OpenAI({
    apiKey: "<api key>",
    baseURL: "https://api.x.ai/v1",
    timeout: 360000, // Timeout after 3600s for reasoning models
});

const stream = await openai.chat.completions.create({
    model: "grok-4.6",
    messages: [
        { role: "system", content: "You are Grok, a helpful and maximally truthful AI built by xAI." },
        {
            role: "user",
            content: "Explain how neural networks learn in two sentences.",
        }
    ],
    stream: true
});

for await (const chunk of stream) {
    console.log(chunk.choices[0].delta.content);
}
```

```javascriptAISDK
import { xai } from '@ai-sdk/xai';
import { streamText } from 'ai';

const result = streamText({
  model: xai.responses('grok-4.6'),
  system:
    "You are Grok, a helpful and maximally truthful AI built by xAI.",
  prompt: 'Explain how neural networks learn in two sentences.',
});

for await (const chunk of result.textStream) {
  process.stdout.write(chunk);
}
```

```bash
curl https://api.x.ai/v1/chat/completions \\
-H "Content-Type: application/json" \\
-H "Authorization: Bearer $XAI_API_KEY" \\
-m 3600 \\
-d '{
    "messages": [
        {
            "role": "system",
            "content": "You are Grok, a helpful and maximally truthful AI built by xAI."
        },
        {
            "role": "user",
            "content": "Explain how neural networks learn in two sentences."
        }
    ],
    "model": "grok-4.6",
    "stream": true
}'
```

You'll get the event streams like these:

```json
data: {
    "id":"<completion_id>","object":"chat.completion.chunk","created":<creation_time>,
    "model":"grok-4.6",
    "choices":[{"index":0,"delta":{"content":"Ah","role":"assistant"}}],
    "usage":{"prompt_tokens":41,"completion_tokens":1,"total_tokens":42,
    "prompt_tokens_details":{"text_tokens":41,"audio_tokens":0,"image_tokens":0,"cached_tokens":0}},
    "system_fingerprint":"fp_xxxxxxxxxx"
}

data: {
    "id":"<completion_id>","object":"chat.completion.chunk","created":<creation_time>,
    "model":"grok-4.6",
    "choices":[{"index":0,"delta":{"content":",","role":"assistant"}}],
    "usage":{"prompt_tokens":41,"completion_tokens":2,"total_tokens":43,
    "prompt_tokens_details":{"text_tokens":41,"audio_tokens":0,"image_tokens":0,"cached_tokens":0}},
    "system_fingerprint":"fp_xxxxxxxxxx"
}

data: [DONE]
```

It is recommended that you use a client SDK to parse the event stream.

Example streaming responses in Python/Javascript:

```
Neural networks learn by adjusting connection weights to minimize prediction error. Through backpropagation, they propagate gradients backward through layers so each weight updates in the direction that improves accuracy on training data.
```