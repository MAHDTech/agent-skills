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

## OpenAI compatibility and client setup

xAI APIs provide OpenAI SDK compatibility. Most integration tasks use official OpenAI client libraries configured with xAI endpoints:

- **Base URL:** `https://api.x.ai/v1`
- **Environment Variable:** `XAI_API_KEY`
- **SDK Setup Example (TypeScript / JavaScript):**

  ```typescript
  import OpenAI from "openai"

  const openai = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1"
  })
  ```

- **SDK Setup Example (Python):**

  ```python
  import os

  from openai import OpenAI

  client = OpenAI(
      api_key=os.environ.get("XAI_API_KEY"),
      base_url="https://api.x.ai/v1",
  )
  ```

## Documentation lookup

When configuring an xAI API integration, follow this lookup order:

1. Read the guide page in `resources/auto/` (for example `developers-models.md`, `developers-tools-function-calling.md`, `developers-tools-web-search.md`, `developers-model-capabilities-text-structured-outputs.md`, or `developers-model-capabilities-images-understanding.md`).
2. Verify exact endpoint paths, parameter schemas, and request payloads in `resources/auto/openapi.json.md`.
3. Take model identifiers from the current models page (`resources/auto/developers-models.md`, or `https://docs.x.ai/developers/models` when it may be stale), not from memory or examples, and confirm the selected model supports the intended feature (vision input, server-side search tools, function calling, reasoning).
4. Load `XAI_API_KEY` from environment variables or a secret manager.
5. When a page is missing locally or a newly announced feature is absent, check the upstream index at `https://docs.x.ai/llms.txt`. To fetch a page online, request its `.md` URL without an `Accept: text/markdown` header; that combination returns 404.

## Core integration patterns

1. **Model selection:** Verify current model strings and pricing and rate-limit tiers on the models, pricing, and rate-limits pages listed in the index.
2. **Streaming & Tool Calling:** Use standard OpenAI-style `stream: true` or `tools` arrays for function calling.
3. **Structured Outputs:** Use JSON mode or JSON schema response formats when precise downstream parsing is required.
4. **Vision & Multimodal:** Pass image URLs or base64 data to vision-supported Grok models following standard message content block arrays.

## Completion criteria

An xAI API integration task is complete when:

1. Target model identifiers, endpoints, and parameters have been validated against the current docs pages and `resources/auto/openapi.json.md`.
2. Client initializations correctly target `https://api.x.ai/v1` and rely on environment-provided `XAI_API_KEY`.
3. Error handling covers common API response codes (e.g. rate limits, token limit exceeded, invalid key).
4. Code implementations have been verified against integration tests or mock executions.

