+++
title = "xai"
description = "Integrate or debug xAI/Grok APIs and SDKs. Use for client configuration, model capabilities, streaming, tool calling, structured output, vision, or search integration."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# xAI and Grok API integration

xAI models and SDK capabilities change often, so verify endpoints, model identifiers, parameters, and feature support against current documentation before writing code. The local `resources/auto/` directory holds the documentation index (`resources/auto/llms.txt`), one file per page it lists (named after the page path, such as `resources/auto/developers-models.md` for the API guides, `build-*` for Grok Build, and `cookbook-*` for the cookbook), and the REST API schema (`resources/auto/openapi.json.md`).

## API endpoints and SDK setup

Default to the **Responses API** (`/v1/responses`) or the native **`xai-sdk`** (Python) for all new integrations. Server-side agentic tools (`web_search`, `x_search`, code execution, remote MCP), stateful conversations (`previous_response_id`), and encrypted reasoning (`include: ["reasoning.encrypted_content"]`) require the Responses API or `xai-sdk`.

Chat Completions (`/v1/chat/completions`, `client.chat.completions.create(...)`) is a **legacy** endpoint (function calling only, no server-side search tools or encrypted reasoning). Use it only when maintaining existing integrations; consult `resources/auto/developers-model-capabilities-text-comparison.md` and `resources/auto/developers-model-capabilities-legacy-chat-completions.md` when migrating.

- **Base URL:** `https://api.x.ai/v1`
- **Environment variable:** `XAI_API_KEY`
- **Supported SDKs:**
  - **OpenAI SDK (TypeScript / JavaScript & Python):** `openai` pointed at `https://api.x.ai/v1`, calling `client.responses.create(...)`.
  - **xAI native SDK (Python):** `xai-sdk` (`pip install xai-sdk`, `from xai_sdk import Client`).
  - **Vercel AI SDK (TypeScript / JavaScript):** `@ai-sdk/xai` (`npm install ai @ai-sdk/xai`) with `xai.responses("<model-id>")` (avoid legacy `xai("<model-id>")`).

### OpenAI-compatible Responses API (TypeScript / JavaScript)

```typescript
import OpenAI from "openai"

const client = new OpenAI({
  apiKey: process.env.XAI_API_KEY,
  baseURL: "https://api.x.ai/v1"
})

const response = await client.responses.create({
  model: "<model-id>",
  input: "Summarize the key changes in this diff."
})
console.log(response.output_text)
```

### OpenAI-compatible Responses API (Python)

```python
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.environ.get("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="<model-id>",
    input="Summarize the key changes in this diff.",
)
print(response.output_text)
```

### Native `xai-sdk` (Python)

```python
import os
from xai_sdk import Client
from xai_sdk.chat import user

client = Client(api_key=os.getenv("XAI_API_KEY"))
chat = client.chat.create(model="<model-id>")
chat.append(user("Summarize the key changes in this diff."))
print(chat.sample().content)
```

### Vercel AI SDK (`@ai-sdk/xai`, TypeScript / JavaScript)

```typescript
import {xai} from "@ai-sdk/xai"
import {generateText} from "ai"

const {text} = await generateText({
  model: xai.responses("<model-id>"),
  prompt: "Summarize the key changes in this diff."
})
console.log(text)
```

Replace `<model-id>` with a current identifier verified in `resources/auto/developers-models.md`.

## Documentation lookup

When configuring an xAI API integration, follow this lookup order:

1. Read the guide page in `resources/auto/` (for example `developers-quickstart.md`, `developers-models.md`, `developers-model-capabilities-text-comparison.md`, `developers-tools-function-calling.md`, `developers-tools-web-search.md`, `developers-tools-x-search.md`, `developers-model-capabilities-text-structured-outputs.md`, or `developers-model-capabilities-images-understanding.md`).
2. Verify exact endpoint paths, parameter schemas, and request payloads in `resources/auto/openapi.json.md`.
3. Take model identifiers from the current models page (`resources/auto/developers-models.md`, or `https://docs.x.ai/developers/models` when it may be stale), not from memory or examples, and confirm the selected model supports the intended feature (vision input, server-side search tools, function calling, reasoning).
4. Load `XAI_API_KEY` from environment variables or a secret manager.
5. When a page is missing locally or a newly announced feature is absent, check the upstream index at `https://docs.x.ai/llms.txt`. To fetch a page online, request its `.md` URL without an `Accept: text/markdown` header; that combination returns 404.

## Core integration patterns

1. **Model selection:** Verify current model strings and pricing and rate-limit tiers on the models, pricing, and rate-limits pages listed in the index.
2. **Server-side tools and function calling:** Pass server-side tools (`{"type": "web_search"}`, `{"type": "x_search"}` in Responses API; `web_search()` / `x_search()` in `xai-sdk`; `xai.tools.webSearch()` in `@ai-sdk/xai`) alongside custom function tools as needed.
3. **Reasoning and streaming:** Use Responses API (`include: ["reasoning.encrypted_content"]`, `stream: true`) or `xai-sdk` (`chat.stream()`) for reasoning models, and configure generous HTTP timeouts for long reasoning turns.
4. **Structured outputs:** Use JSON schema response formats (`resources/auto/developers-model-capabilities-text-structured-outputs.md`) when precise downstream parsing is required.
5. **Vision and multimodal:** Pass image URLs or base64 data to vision-supported Grok models using Responses API input blocks or `xai-sdk` helpers.

## Completion criteria

An xAI API integration task is complete when:

1. Target model identifiers, endpoints, and parameters have been validated against the current docs pages and `resources/auto/openapi.json.md`.
2. New integrations use `/v1/responses` (`client.responses.create(...)` or `xai.responses(...)`) or `xai-sdk` (using legacy `/v1/chat/completions` only when explicitly maintaining legacy code), target `https://api.x.ai/v1`, and load `XAI_API_KEY` from the environment or a secret manager.
3. Error handling covers common API response codes (e.g., rate limits, context/token limits, invalid key) and sets appropriate timeouts for reasoning models.
4. Code implementations have been verified against integration tests or mock executions.

