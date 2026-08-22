#### Tools

# Image Generation Tool

The image generation tool lets Grok create and edit images with [Grok Imagine](https://docs.x.ai/developers/model-capabilities/images/generation) as part of a conversation. It uses the latest Imagine image models (`grok-imagine-image-2.0`). You hand the model the tool; it decides when to call it, writes the image prompt, picks an aspect ratio, and returns the finished image alongside its text response. Because the tool runs server-side, the model can also chain calls—generating an image and then editing it—within a single request.

If you already have the exact prompt and want direct control over aspect ratio and resolution, call the [image generation](https://docs.x.ai/developers/model-capabilities/images/generation) and [image editing](https://docs.x.ai/developers/model-capabilities/images/editing) endpoints instead. Reach for the tool when image creation is one step in a larger conversational or agentic workflow.

## SDK support

| SDK/API | Tool Name |
|---------|-----------|
| xAI SDK | `image_generation` |
| OpenAI Responses API | `image_generation` |

This tool is also supported in all Responses API compatible SDKs. The Vercel AI SDK does not yet expose the image generation tool.

## Basic usage

Add `image_generation` to `tools` and ask for an image. In the xAI SDK, each generated image is exposed on `response.image_outputs` as decoded bytes you can write straight to a file. In the Responses API, each image arrives as an `image_generation_call` output item whose `result` field carries the base64-encoded image with no data-URL prefix, so you can decode it directly.

```bash customLanguage="bash"
curl https://api.x.ai/v1/responses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
  "model": "grok-4.6",
  "input": "Generate an image of a corgi surfing a big wave, in the style of a Japanese woodblock print",
  "tools": [
    {
      "type": "image_generation"
    }
  ]
}' | jq -r '.output[] | select(.type == "image_generation_call") | .result' \
  | base64 --decode > corgi_surfing.jpg
```

```python customLanguage="pythonXAI"
import os

from xai_sdk import Client
from xai_sdk.chat import user
from xai_sdk.tools import image_generation

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(
    model="grok-4.6",
    tools=[image_generation()],
)
chat.append(user("Generate an image of a corgi surfing a big wave, in the style of a Japanese woodblock print"))
response = chat.sample()

print(response.content)
with open("image.jpeg", "wb") as f:
    f.write(response.image_outputs[0].image)
```

```python customLanguage="pythonOpenAISDK"
import base64
import os

from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="grok-4.6",
    input="Generate an image of a corgi surfing a big wave, in the style of a Japanese woodblock print",
    tools=[{"type": "image_generation"}],
)

image_data = [
    output.result
    for output in response.output
    if output.type == "image_generation_call"
]

if image_data:
    with open("corgi_surfing.jpg", "wb") as f:
        f.write(base64.b64decode(image_data[0]))
```

```python customLanguage="pythonRequests"
import base64
import os

import requests

url = "https://api.x.ai/v1/responses"
headers = {
    "Content-Type": "application/json",
    "Authorization": f"Bearer {os.getenv('XAI_API_KEY')}",
}
payload = {
    "model": "grok-4.6",
    "input": "Generate an image of a corgi surfing a big wave, in the style of a Japanese woodblock print",
    "tools": [{"type": "image_generation"}],
}
response = requests.post(url, headers=headers, json=payload)
data = response.json()

for item in data["output"]:
    if item["type"] == "image_generation_call":
        with open("corgi_surfing.jpg", "wb") as f:
            f.write(base64.b64decode(item["result"]))
```

```javascript customLanguage="javascriptOpenAISDK"
import fs from "fs";
import OpenAI from "openai";

const client = new OpenAI({
  apiKey: process.env.XAI_API_KEY,
  baseURL: "https://api.x.ai/v1",
});

const response = await client.responses.create({
  model: "grok-4.6",
  input:
    "Generate an image of a corgi surfing a big wave, in the style of a Japanese woodblock print",
  tools: [{ type: "image_generation" }],
});

const imageData = response.output
  .filter((output) => output.type === "image_generation_call")
  .map((output) => output.result);

if (imageData.length > 0) {
  fs.writeFileSync("corgi_surfing.jpg", Buffer.from(imageData[0], "base64"));
}
```

A completed `image_generation_call` output item looks like this:

```json
{
  "type": "image_generation_call",
  "id": "ig_d817cfd0-4f39-9cb3-bda2-44e538841ef2_call-a1b4dd05",
  "status": "completed",
  "prompt": "A corgi surfing a big wave, Japanese woodblock print style",
  "result": "/9j/4AAQSkZJRgABAQAAAQABAAD..."
}
```

The `prompt` field shows the prompt the model wrote for the image model, useful for understanding and debugging what was generated. Item IDs are prefixed `ig_` for generations and `ie_` for edits.

The tool takes no size or format parameters; the model picks an aspect ratio for each call. To control it, ask in your request ("in a 9:16 vertical aspect ratio") and the generated image will match.

## The action parameter

By default the model can both generate new images and edit existing ones. The optional `action` parameter restricts this:

| Action | Behavior |
|--------|----------|
| `auto` | Default. The model can generate and edit images |
| `generate` | Text-to-image generation only |
| `edit` | Image editing only |

For example, to let the model create images but never modify ones already in the conversation:

```bash customLanguage="bash"
curl https://api.x.ai/v1/responses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
  "model": "grok-4.6",
  "input": "Generate an image of a hot air balloon over the desert",
  "tools": [
    {
      "type": "image_generation",
      "action": "generate"
    }
  ]
}'
```

```python customLanguage="pythonXAI"
chat = client.chat.create(
    model="grok-4.6",
    tools=[image_generation(action="generate")],
)
chat.append(user("Generate an image of a hot air balloon over the desert"))
response = chat.sample()
```

```python customLanguage="pythonOpenAISDK"
response = client.responses.create(
    model="grok-4.6",
    input="Generate an image of a hot air balloon over the desert",
    tools=[{"type": "image_generation", "action": "generate"}],
)
```

## Editing input images

With `action` set to `edit` (or the default `auto`), the model can edit any image already in the conversation: images you attach as input as well as images it generated earlier. Edits produce `image_generation_call` items with an `ie_` ID prefix.

```bash customLanguage="bash"
curl https://api.x.ai/v1/responses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
  "model": "grok-4.6",
  "input": [
    {
      "role": "user",
      "content": [
        {
          "type": "input_text",
          "text": "Edit this image so it looks like a watercolor painting."
        },
        {
          "type": "input_image",
          "image_url": "https://docs.x.ai/assets/api-examples/images/style-realistic.png"
        }
      ]
    }
  ],
  "tools": [
    {
      "type": "image_generation",
      "action": "edit"
    }
  ]
}'
```

```python customLanguage="pythonXAI"
import os

from xai_sdk import Client
from xai_sdk.chat import image, user
from xai_sdk.tools import image_generation

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(
    model="grok-4.6",
    tools=[image_generation(action="edit")],
)
chat.append(
    user(
        "Edit this image so it looks like a watercolor painting.",
        image("https://docs.x.ai/assets/api-examples/images/style-realistic.png"),
    )
)
response = chat.sample()

with open("image.jpeg", "wb") as f:
    f.write(response.image_outputs[0].image)
```

```python customLanguage="pythonOpenAISDK"
import base64
import os

from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="grok-4.6",
    input=[
        {
            "role": "user",
            "content": [
                {
                    "type": "input_text",
                    "text": "Edit this image so it looks like a watercolor painting.",
                },
                {
                    "type": "input_image",
                    "image_url": "https://docs.x.ai/assets/api-examples/images/style-realistic.png",
                },
            ],
        }
    ],
    tools=[{"type": "image_generation", "action": "edit"}],
)

image_data = [
    output.result
    for output in response.output
    if output.type == "image_generation_call"
]

if image_data:
    with open("watercolor.jpg", "wb") as f:
        f.write(base64.b64decode(image_data[0]))
```

## Multi-turn editing

Images generated on a previous turn stay editable on follow-up turns. Continue the conversation — append the previous response to the chat in the xAI SDK, or pass `previous_response_id` in the Responses API — and the model can refine its earlier images by reference:

```python customLanguage="pythonXAI"
import os

from xai_sdk import Client
from xai_sdk.chat import user
from xai_sdk.tools import image_generation

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(
    model="grok-4.6",
    tools=[image_generation()],
)

# Turn 1: generate an image
chat.append(user("Generate an image of a lighthouse on a rocky coast"))
response = chat.sample()
with open("image.jpeg", "wb") as f:
    f.write(response.image_outputs[0].image)

# Turn 2: edit the image from the previous turn
chat.append(response)
chat.append(user("Make it night time with a full moon"))
followup = chat.sample()
with open("edited_image.jpeg", "wb") as f:
    f.write(followup.image_outputs[0].image)
```

```python customLanguage="pythonOpenAISDK"
import base64
import os

from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="grok-4.6",
    input="Generate an image of a lighthouse on a rocky coast",
    tools=[{"type": "image_generation"}],
)

image_data = [
    output.result
    for output in response.output
    if output.type == "image_generation_call"
]

if image_data:
    with open("lighthouse.jpg", "wb") as f:
        f.write(base64.b64decode(image_data[0]))

# Follow up: edit the image from the previous turn
followup = client.responses.create(
    model="grok-4.6",
    previous_response_id=response.id,
    input="Make it night time with a full moon",
    tools=[{"type": "image_generation"}],
)

image_data_followup = [
    output.result
    for output in followup.output
    if output.type == "image_generation_call"
]

if image_data_followup:
    with open("lighthouse_night.jpg", "wb") as f:
        f.write(base64.b64decode(image_data_followup[0]))
```

If you manage conversation state yourself instead of using `previous_response_id`, pass the previous turn's output items (including the `image_generation_call` items) back verbatim in `input`; the images they carry remain editable on the next request.

## Combining with other tools

The image generation tool composes with the other server-side tools. Include several tools in the same request and the model orchestrates them within a single agentic loop, feeding what one tool found into the next. Here it looks up a fact with [web search](https://docs.x.ai/developers/tools/web-search) first, then writes an image prompt from what it learned:

```bash customLanguage="bash"
curl https://api.x.ai/v1/responses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
  "model": "grok-4.6",
  "input": "Find out which team won the most recent FIFA World Cup, then generate an image of a celebratory poster for that team, in a vintage travel-poster style.",
  "tools": [
    {
      "type": "web_search"
    },
    {
      "type": "image_generation"
    }
  ]
}' | jq -r '.output[] | select(.type == "image_generation_call") | .result' \
  | base64 --decode > champions_poster.jpg
```

```python customLanguage="pythonXAI"
import os

from xai_sdk import Client
from xai_sdk.chat import user
from xai_sdk.tools import image_generation, web_search

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(
    model="grok-4.6",
    tools=[web_search(), image_generation()],
)
chat.append(
    user(
        "Find out which team won the most recent FIFA World Cup, then generate an "
        "image of a celebratory poster for that team, in a vintage travel-poster style."
    )
)
response = chat.sample()

print(response.content)
with open("image.jpeg", "wb") as f:
    f.write(response.image_outputs[0].image)

# Per-tool invocation counts for the request
print(response.server_side_tool_usage)
```

```python customLanguage="pythonOpenAISDK"
import base64
import os

from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="grok-4.6",
    input=(
        "Find out which team won the most recent FIFA World Cup, then generate an "
        "image of a celebratory poster for that team, in a vintage travel-poster style."
    ),
    tools=[
        {"type": "web_search"},
        {"type": "image_generation"},
    ],
)

for output in response.output:
    if output.type == "web_search_call":
        print(f"Web search: {output.action}")
    elif output.type == "image_generation_call":
        print(f"Image prompt: {output.prompt}")
        with open("champions_poster.jpg", "wb") as f:
            f.write(base64.b64decode(output.result))
    elif output.type == "message":
        print(output.content[0].text)
```

The response output interleaves the tool calls in the order they ran: a `web_search_call` item, a message answering the factual question with citations, and an `image_generation_call` item carrying the poster.

The same pattern works with [X search](https://docs.x.ai/developers/tools/x-search), [code execution](https://docs.x.ai/developers/tools/code-execution), and your own client-side functions. See [Advanced Usage](https://docs.x.ai/developers/tools/advanced-usage#tool-combinations) for more tool combination patterns.

## Streaming

When streaming, each image generation call emits progress events—`in_progress`, then `generating`, then `completed`—followed by a `response.output_item.done` event whose item carries the base64 result. Partial image previews are not emitted.

In the xAI SDK, pass `include=["verbose_streaming"]` to watch tool calls as they happen; the decoded images are available on the accumulated response via `response.image_outputs` once the stream ends.

```python customLanguage="pythonXAI"
import os

from xai_sdk import Client
from xai_sdk.chat import user
from xai_sdk.tools import get_tool_call_type, image_generation

client = Client(api_key=os.getenv("XAI_API_KEY"))

chat = client.chat.create(
    model="grok-4.6",
    tools=[image_generation()],
    include=["verbose_streaming"],
)
chat.append(user("Generate an image of an origami fox in a paper forest"))

for response, chunk in chat.stream():
    for tool_call in chunk.tool_calls:
        if get_tool_call_type(tool_call) == "image_generation_tool":
            print(f"\nGenerating image: {tool_call.function.arguments}")
    if chunk.content:
        print(chunk.content, end="", flush=True)

# The accumulated response carries the decoded images once the stream ends
with open("image.jpeg", "wb") as f:
    f.write(response.image_outputs[0].image)
```

```python customLanguage="pythonOpenAISDK"
import base64
import os

from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

stream = client.responses.create(
    model="grok-4.6",
    input="Generate an image of an origami fox in a paper forest",
    tools=[{"type": "image_generation"}],
    stream=True,
)

for event in stream:
    if event.type.startswith("response.image_generation_call."):
        # in_progress -> generating -> completed
        print(f"Image generation status: {event.type.rsplit('.', 1)[-1]}")
    elif event.type == "response.output_item.done" and event.item.type == "image_generation_call":
        # The base64 image rides on the final output item
        with open("origami_fox.jpg", "wb") as f:
            f.write(base64.b64decode(event.item.result))
    elif event.type == "response.output_text.delta":
        print(event.delta, end="", flush=True)
```

```javascript customLanguage="javascriptOpenAISDK"
import fs from "fs";
import OpenAI from "openai";

const client = new OpenAI({
  apiKey: process.env.XAI_API_KEY,
  baseURL: "https://api.x.ai/v1",
});

const stream = await client.responses.create({
  model: "grok-4.6",
  input: "Generate an image of an origami fox in a paper forest",
  tools: [{ type: "image_generation" }],
  stream: true,
});

for await (const event of stream) {
  if (event.type.startsWith("response.image_generation_call.")) {
    // in_progress -> generating -> completed
    console.log(`Image generation status: ${event.type.split(".").pop()}`);
  } else if (
    event.type === "response.output_item.done" &&
    event.item.type === "image_generation_call"
  ) {
    // The base64 image rides on the final output item
    fs.writeFileSync(
      "origami_fox.jpg",
      Buffer.from(event.item.result, "base64")
    );
  } else if (event.type === "response.output_text.delta") {
    process.stdout.write(event.delta);
  }
}
```

## Related

* [Image Generation](https://docs.x.ai/developers/model-capabilities/images/generation) — Generate images directly with the images endpoint
* [Image Editing](https://docs.x.ai/developers/model-capabilities/images/editing) — Edit images with natural language
* [Tools Overview](https://docs.x.ai/developers/tools/overview) — All built-in tools
* [Streaming & Sync](https://docs.x.ai/developers/tools/streaming-and-sync) — Streaming behavior of tool-enabled requests
* [Pricing](https://docs.x.ai/developers/pricing#tools-pricing) — Tool invocation costs