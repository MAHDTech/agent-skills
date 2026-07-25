+++
title = "xai"
description = "Build applications with xAI and Grok APIs, SDKs, model selection, function calling, vision, live web search, and OpenAI-compatible client configurations. Use when integrating Grok models or xAI API endpoints, querying xAI capabilities, or configuring xAI API SDKs."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# xAI & Grok API Integration

Guidelines for discovering, configuring, and building software with xAI and Grok APIs, SDKs, and endpoints.

Because xAI models and SDK capabilities evolve rapidly, treat the vendored documentation in `resources/auto/` as the single source of truth for available endpoints, model identifiers, API parameters, and feature support.

## OpenAI Compatibility & Client Setup

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
  from openai import OpenAI

  client = OpenAI(
      api_key=os.environ.get("XAI_API_KEY"),
      base_url="https://api.x.ai/v1",
  )
  ```

## Documentation & API Option Lookup Protocol

When configuring xAI API integration for a task, follow this lookup protocol:

1. **Query Vendored Documentation:**
   Search the `resources/auto/` directory within this skill using pattern or string search tools to verify exact model names, endpoint paths, parameter schemas, and request payloads.
   - Example search targets: model aliases (`grok-3`, `grok-2-vision`), function calling schema (`tools`), search/web tools (`grok-search`), structured outputs (`response_format`), or vision capabilities.
2. **Confirm Model & Endpoint Capabilities:**
   Before making API requests, verify that the selected Grok model supports the intended feature (e.g., multimodal vision inputs vs text-only, web search integration, function calling).
3. **Handle API Credentials Safely:**
   Ensure `XAI_API_KEY` is loaded from environment variables or secure secret managers. Never hardcode credentials into source files.
4. **Update Offline Resources when Outdated:**
   If a newly announced Grok feature or model identifier is missing from local resources, trigger a resource download:

   ```bash
   skills --action download-resources --skill xai
   ```

## Core Integration Patterns

1. **Model Selection:** Verify current active model strings and pricing/rate-limit tiers in `resources/auto/`.
2. **Streaming & Tool Calling:** Use standard OpenAI-style `stream: true` or `tools` arrays for function calling.
3. **Structured Outputs:** Utilize JSON mode or JSON schema response formats when precise downstream parsing is required.
4. **Vision & Multimodal:** Pass image URLs or base64 data to vision-supported Grok models following standard message content block arrays.

## Completion Criteria

An xAI API integration task is complete when:

1. Target model identifiers, endpoints, and parameters have been validated against vendored docs in `resources/auto/`.
2. Client initializations correctly target `https://api.x.ai/v1` and rely on environment-provided `XAI_API_KEY`.
3. Error handling covers common API response codes (e.g. rate limits, token limit exceeded, invalid key).
4. Code implementations have been verified against integration tests or mock executions.

