+++
title = "developers-tools-citations"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Tools

# Citations

The agent tools API provides two types of citation information: **All Citations** (a complete list of all sources encountered) and **Inline Citations** (markdown-style links embedded directly in the response text).

## All Citations

The `citations` attribute on the `response` object provides a comprehensive list of URLs for all sources the agent encountered during its search process. This list is **always returned by default** — no additional configuration is required.

Citations are automatically collected from successful tool executions and provide full traceability of the agent's information sources. They are returned when the agentic request completes.

Note that not every URL in this list will necessarily be directly referenced in the final answer. The agent may examine a source during its research process and determine it is not sufficiently relevant to the user's query, but the URL will still appear in this list for transparency.

```pythonWithoutSDK
response.citations
```

```output
[
'https://x.com/i/user/1912644073896206336',
'https://x.com/i/status/1975607901571199086',
'https://x.ai/news',
'https://docs.x.ai/developers/release-notes',
...
]
```

## Inline Citations

Inline citations are **markdown-style links** (e.g., `[[1]](https://x.ai/news)`) inserted directly into the response text at the points where the model references sources. In addition to these visible links, **structured metadata** is available on the response object with precise positional information.

**Important**: Enabling inline citations does not guarantee that the model will cite sources on every answer. The model decides when and where to include citations based on the context and nature of the query.

### Configuring Inline Citations

Inline citation behavior differs between the **Responses API** and the **xAI Python SDK** (gRPC chat API).

The **Responses API** behaviour applies to the following clients:

* cURL against `/v1/responses`
* Python (OpenAI SDK)
* JavaScript (AI SDK via `xai.responses()`)
* JavaScript (OpenAI SDK)

| | Responses API(cURL, Python/JS OpenAI SDK, JS AI SDK) | xAI Python SDK |
|---|---|---|
| **Default** | Enabled — response text may include `[[N]](https://docs.x.ai/developers/tools/url)` links without extra configuration | Disabled — omit `include`, or do not pass `"inline_citations"` |
| **Enable** | Enabled by default, no additional action needed. | Pass `include=["inline_citations"]` to the `chat.create()` method |
| **Disable** | Pass `include=["no_inline_citations"]` | Disabled by default |

When inline citations are disabled, the response text will not contain any `[[N]](https://docs.x.ai/developers/tools/url)` markdown links. The `annotations` field on `output_text` content blocks may still be present, but annotations only list sources encountered during search — they will not have positional references into the response text.

#### Enabled (default for Responses API; opt-in for xAI Python SDK)

```bash customLanguage="bash" highlightedLines="10"
# Inline citations are enabled by default for the Responses API
curl https://api.x.ai/v1/responses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
  "model": "grok-4.6",
  "input": [
    {"role": "user", "content": "What is xAI?"}
  ],
  "tools": [{"type": "web_search"}]
}'
```

```python customLanguage="pythonXAI" highlightedLines="14"
import os

from xai_sdk import Client
from xai_sdk.chat import user
from xai_sdk.tools import web_search, x_search

client = Client(api_key=os.getenv("XAI_API_KEY"))
chat = client.chat.create(
    model="grok-4.6",
    tools=[
        web_search(),
        x_search(),
    ],
    include=["inline_citations"],  # Enable inline citations (opt-in for xAI Python SDK)
)

chat.append(user("What is xAI?"))
response = chat.sample()

# Access the response text (includes inline citation markdown)
print(response.content)
```

```python customLanguage="pythonOpenAISDK" highlightedLines="15"
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="grok-4.6",
    input=[
        {"role": "user", "content": "What is xAI?"}
    ],
    tools=[
        {"type": "web_search"},  # inline citations are enabled by default
    ],
)

# Get the message output with inline citations
for item in response.output:
    if item.type == "message":
        for content in item.content:
            if content.type == "output_text":
                print(content.text)
```

```javascript customLanguage="javascriptAISDK" highlightedLines="8"
import { xai } from '@ai-sdk/xai';
import { generateText } from 'ai';

const { text, sources } = await generateText({
  model: xai.responses('grok-4.6'),
  prompt: 'What is xAI?',
  tools: {
    web_search: xai.tools.webSearch(), // inline citations are enabled by default
  },
});

// Text includes inline citation markdown
console.log(text);

// Sources contain all citation URLs
console.log('Sources:', sources);
```

```javascript customLanguage="javascriptOpenAISDK" highlightedLines="13"
import OpenAI from 'openai';

const client = new OpenAI({
  apiKey: process.env.XAI_API_KEY,
  baseURL: 'https://api.x.ai/v1',
});

const response = await client.responses.create({
  model: 'grok-4.6',
  input: [
    { role: 'user', content: 'What is xAI?' }
  ],
  tools: [{ type: 'web_search' }], // inline citations are enabled by default
});

// Get the message with inline citations
for (const item of response.output) {
  if (item.type === 'message') {
    for (const content of item.content) {
      if (content.type === 'output_text') {
        console.log(content.text);
      }
    }
  }
}
```

#### Disabled (opt-out for Responses API; default for xAI Python SDK)

```python customLanguage="pythonOpenAISDK" highlightedLines="17"
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="grok-4.6",
    input=[
        {"role": "user", "content": "What is xAI?"}
    ],
    tools=[
        {"type": "web_search"},
    ],
    include=["no_inline_citations"],  # Disable inline citations
)

# Response text will not contain inline citation markdown
for item in response.output:
    if item.type == "message":
        for content in item.content:
            if content.type == "output_text":
                print(content.text)
```

```python customLanguage="pythonRequests" highlightedLines="12"
import os
import requests

response = requests.post(
    "https://api.x.ai/v1/responses",
    headers={
        "Content-Type": "application/json",
        "Authorization": f"Bearer {os.getenv('XAI_API_KEY')}",
    },
    json={
        "model": "grok-4.6",
        "include": ["no_inline_citations"],
        "input": [
            {"role": "user", "content": "What is xAI?"}
        ],
        "tools": [{"type": "web_search"}],
    },
)

data = response.json()
for item in data["output"]:
    if item["type"] == "message":
        for content in item["content"]:
            if content["type"] == "output_text":
                print(content["text"])
```

```bash customLanguage="bash" highlightedLines="6"
curl https://api.x.ai/v1/responses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
  "model": "grok-4.6",
  "include": ["no_inline_citations"],
  "input": [
    {"role": "user", "content": "What is xAI?"}
  ],
  "tools": [{"type": "web_search"}]
}'
```

### Markdown Citation Format

When inline citations are enabled, the model will insert markdown-style citation links directly into the response text:

```output
The latest announcements from xAI, primarily from their official X account (@xai) and website (x.ai/news), date back to November 19, 2025.[[1]](https://x.ai/news/)[[2]](https://x.ai/)[[3]](https://x.com/i/status/1991284813727474073)
```

When rendered as markdown, this displays as clickable links:

> The latest announcements from xAI, primarily from their official X account (@xai) and website (x.ai/news), date back to November 19, 2025.[\[1\]](https://x.ai/news/)[\[2\]](https://x.ai/)[\[3\]](https://x.com/i/status/1991284813727474073)

The format is `[[N]](https://docs.x.ai/developers/tools/url)` where:

* `N` is the sequential display number for the citation **starting from 1**
* `url` is the source URL

**Citation numbering**: Citation numbers always start from 1 and increment sequentially. If the same source is cited again later in the response, the original citation number will be reused.

### Image Embeds

When `enable_image_search` is enabled on the `web_search` tool, Grok may embed image results as Markdown images instead of numbered text citations:

```output
Here are images of Starship on the launch pad:
![Why the SpaceX Starship launch pad matters](https://www.astronomy.com/wp-content/uploads/2024/09/starship-test-flight-mission-scaled.jpg)
```

The format is `![alt](https://docs.x.ai/developers/tools/url)` where:

* `alt` is a short description or title for the image
* `url` is the image source URL

## Accessing Structured Inline Citation Data

Structured inline citation data provides precise positional information about each citation in the response text.

### Response Format

When inline citations are enabled, each `output_text` content block includes an `annotations` array with structured citation metadata (URL, character offsets, and label):

```json highlightedLines="16-45"
{
  "created_at": 1781829888,
  "completed_at": 1781829888,
  "id": "5808284d-ae14-9981-9289-73515f67ebda",
  "max_output_tokens": null,
  "model": "grok-4.6",
  "object": "response",
  "output": [
    ...
    {
      "content": [
        {
          "type": "output_text",
          "text": "**xAI is an artificial intelligence company founded by Elon Musk in March 2023.** Its stated mission is to \"understand the universe\" by building advanced AI systems that accelerate human scientific discovery.[[1]](https://x.ai/company)\n\n### Key Details\n- **Flagship product**: Grok, a family of frontier AI models focused on reasoning, code, voice, image generation, and video. These are trained on massive infrastructure, including what the company describes as the world's largest supercluster (Colossus). Grok powers chatbots, APIs, and multimodal tools available via a unified API.[[2]](https://x.ai/)\n- **Current status (as of mid-2026)**: xAI operates as a subsidiary of SpaceX following an acquisition in February 2026. It is also connected to the X social platform (formerly Twitter), which xAI effectively became the parent of in 2025. The company has expanded into data centers and enterprise AI offerings (e.g., integrations with Amazon Bedrock and Databricks).[[3]](https://en.wikipedia.org/wiki/XAI_(company))\n- **Headquarters and team**: Based in the Stanford Research Park in Palo Alto, California. It was initially founded with a team of AI researchers and is led by Elon Musk as CEO.\n\nxAI positions itself as building useful AI, distinct from other labs in its approach. Its official website (x.ai) highlights developer tools, API access, and ongoing model releases. Note that there is an unrelated blockchain/gaming project called Xai (xai.games), but the primary reference to \"xAI\" in this context is Musk's AI venture.[[4]](https://xai.games/)\n\nFor the latest updates, check x.ai or @xai on X.",
          "logprobs": [],
          "annotations": [
            {
              "type": "url_citation",
              "url": "https://x.ai/company",
              "start_index": 208,
              "end_index": 235,
              "title": "1"
            },
            {
              "type": "url_citation",
              "url": "https://x.ai/",
              "start_index": 585,
              "end_index": 605,
              "title": "2"
            },
            {
              "type": "url_citation",
              "url": "https://en.wikipedia.org/wiki/XAI_(company)",
              "start_index": 972,
              "end_index": 1022,
              "title": "3"
            },
            {
              "type": "url_citation",
              "url": "https://xai.games/",
              "start_index": 1555,
              "end_index": 1580,
              "title": "4"
            }
          ]
        }
      ],
      "id": "msg_5808284d-ae14-9981-9289-73515f67ebda",
      "role": "assistant",
      "type": "message",
      "status": "completed"
    }
  ],
  "parallel_tool_calls": true,
  "previous_response_id": null,
  "reasoning": {
    "effort": "low",
    "summary": "detailed"
  },
  ...
}
```

Each citation annotation contains:

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Always `"url_citation"` |
| `url` | string | The source URL |
| `start_index` | int | Character position where the citation starts in the response text |
| `end_index` | int | Character position where the citation ends (exclusive) |
| `title` | string | The citation label; for text citations, this is the visible citation number (e.g., "1", "2") |

Image embeds can also produce annotation metadata. The annotation `title` is not shown in the Markdown image.

```python customLanguage="pythonXAI"
# After streaming or sampling completes, access the structured inline citations:
for citation in response.inline_citations:
    print(f"Citation [{citation.id}]:")
    print(f"  Position: {citation.start_index} to {citation.end_index}")
    
    # Check citation type
    if citation.HasField("web_citation"):
        print(f"  Web URL: {citation.web_citation.url}")
    elif citation.HasField("x_citation"):
        print(f"  X URL: {citation.x_citation.url}")
```

```python customLanguage="pythonOpenAISDK"
# Access annotations from the response
for item in response.output:
    if item.type == "message":
        for content in item.content:
            if content.type == "output_text":
                for annotation in content.annotations:
                    print(f"Citation [{annotation.title}]:")
                    print(f"  URL: {annotation.url}")
                    print(f"  Position: {annotation.start_index} to {annotation.end_index}")
```

```javascript customLanguage="javascriptAISDK"
import { xai } from '@ai-sdk/xai';
import { streamText } from 'ai';

const { fullStream } = streamText({
  model: xai.responses('grok-4.6'),
  prompt: 'What is xAI?',
  tools: {
    web_search: xai.tools.webSearch(),
  },
});

// Access sources as they stream in
for await (const part of fullStream) {
  if (part.type === 'source' && part.sourceType === 'url') {
    console.log(`Citation: ${part.url}`);
  }
}
```

```javascript customLanguage="javascriptOpenAISDK"
// Access annotations from the response
for (const item of response.output) {
  if (item.type === 'message') {
    for (const content of item.content) {
      if (content.type === 'output_text') {
        for (const annotation of content.annotations) {
          console.log(`Citation [${annotation.title}]:`);
          console.log(`  URL: ${annotation.url}`);
          console.log(`  Position: ${annotation.start_index} to ${annotation.end_index}`);
        }
      }
    }
  }
}
```

```output
Citation [1]:
  Position: 37 to 76
  Web URL: https://x.ai/news/grok-4-fast
Citation [2]:
  Position: 124 to 171
  X URL: https://x.com/xai/status/1234567890
```

### Using Position Indices

The `start_index` and `end_index` values follow Python slice convention:

* **`start_index`**: Character position of the first `[` of the citation
* **`end_index`**: Character position immediately *after* the closing `)` (exclusive)

Extract the exact citation markdown from the response text using a simple slice:

```python customLanguage="pythonXAI"
content = response.content

for citation in response.inline_citations:
    # Extract the markdown link from the response text
    citation_text = content[citation.start_index:citation.end_index]
    print(f"Citation text: {citation_text}")
```

## Streaming Inline Citations

During streaming, inline citations are accumulated and available on the final response. The markdown links appear in real-time in the `chunk.content` as the model generates text:

```python customLanguage="pythonXAI"
for response, chunk in chat.stream():
    # Markdown links appear in chunk.content in real-time
    if chunk.content:
        print(chunk.content, end="", flush=True)
    
    # Inline citations can also be accessed per-chunk during streaming
    for citation in chunk.inline_citations:
        print(f"\nNew citation: [{citation.id}]")

# After streaming, access all accumulated inline citations
print("\n\nAll inline citations:")
for citation in response.inline_citations:
    url = ""
    if citation.HasField("web_citation"):
        url = citation.web_citation.url
    elif citation.HasField("x_citation"):
        url = citation.x_citation.url
    print(f"  [{citation.id}] {url}")
```
