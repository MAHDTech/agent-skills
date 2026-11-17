+++
title = "overview-index"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

# Get started

Build with the Grok API — intelligent, fast, and cost-effective models across code, text, voice, image, and video.

## Models

We offer a range of models supporting multiple use cases and modalities.

### [Code API](/build/overview)

Agentic coding with Grok Build (grok-4.7), our coding model, on the API and CLI.

* Agentic coding workflows
* Powers Grok Build
* Available on the SpaceXSI API

### [Responses API](/developers/model-capabilities/text/generate-text)

Generate text, have conversations, use tools, and build AI-powered applications.

* Generate text
* Multi-turn chat
* Function calling

### [Voice API](/developers/model-capabilities/audio/text-to-speech)

Convert text to natural speech or transcribe audio with our voice models.

* Text to speech
* Speech to text
* Real-time voice

### [Image API](/developers/model-capabilities/images/generation)

Generate stunning images from text, edit existing images, and understand visual content.

* Generate images
* Edit images
* Precise control

### [Video API](/developers/model-capabilities/video/generation)

Bring an image to life, start from a simple text prompt, or refine a complex cinematic sequence.

* Generate videos
* Edit videos
* Precise control

## Quick Start

```python
from openai import OpenAI

client = OpenAI(
    api_key="YOUR_XAI_API_KEY",
    base_url="https://api.x.ai/v1",
)

response = client.responses.create(
    model="grok-4.7",
    input="Fix this function and explain the bug: function median(a){a.sort();return a[a.length/2]}",
)

print(response.output_text)
```

```bash
curl https://api.x.ai/v1/responses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
    "model": "grok-4.7",
    "input": "Fix this function and explain the bug: function median(a){a.sort();return a[a.length/2]}"
}'
```

## Get Started

* [Quickstart guide](/developers/quickstart)
* [Grok Build overview](/build/overview)
* [Grok Bot](/grok-bot/overview)
* [Models](/developers/models)
* [Pricing](/developers/pricing)

## Build

* [Function calling](/developers/tools/function-calling)
* [Web search](/developers/tools/web-search)
* [Structured outputs](/developers/model-capabilities/text/structured-outputs)
* [Batch API](/developers/advanced-api-usage/batch-api)

## Resources

* [API reference](/developers/rest-api-reference/inference)
* [Community integrations](/developers/community)
* [Release notes](/developers/release-notes)


