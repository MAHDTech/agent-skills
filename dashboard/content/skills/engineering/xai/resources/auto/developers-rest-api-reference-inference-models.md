+++
title = "developers-rest-api-reference-inference-models"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Inference API

# Models

## GET /v1/models

List all models available to the authenticating API key, including model names (ID), creation times, and pricing.

### Response Body

* `data` (array\<object>, required) — A list of models with with minimalized information.

  * `aliases` (array\<string>, required) — Alias ID(s) of the model that user can use in a request's model field.

  * `cached_prompt_text_token_price` (integer | null) — Price of a prompt text token (in USD cents per 100 million tokens) that was cached previously.

  * `cached_prompt_text_token_price_long_context` (integer | null) — Price of the cached prompt text token for long context requests (USD cents per 100 million tokens).

  * `completion_text_token_price` (integer | null) — Price of the completion text token in USD cents per 100 million tokens.

  * `completion_text_token_price_long_context` (integer | null) — Price of the completion text token for long context requests (USD cents per 100 million tokens).

  * `context_length` (integer | null) — The maximum context length supported by the model, in tokens.

  * `created` (integer, required) — Model creation time in Unix timestamp.

  * `id` (string, required) — Model ID. Obtainable from \<https://console.x.ai/team/default/models> or \<https://docs.x.ai/docs/models>.

  * `image_price` (integer | null) — Price per image in USD cents (image generation models).

  * `long_context_threshold` (integer | null) — Token count at or above which the long context prices apply.

  * `object` (string, required) — The object type, which is always \`"model"\`.

  * `owned_by` (string, required) — Owner of the model.

  * `prompt_image_token_price` (integer | null) — Price of the prompt image token in USD cents per 100 million tokens.

  * `prompt_text_token_price` (integer | null) — Price of the prompt text token in USD cents per 100 million tokens.

  * `prompt_text_token_price_long_context` (integer | null) — Price of the prompt text token for long context requests (USD cents per 100 million tokens).

* `object` (string, required) — The object type of \`data\` field, which is always \`"list"\`.

\*\*Response example:\*\*

```json
{
  "data": [
    {
      "id": "latest",
      "aliases": [],
      "context_length": 131072,
      "created": 1776556800,
      "object": "model",
      "owned_by": "xai",
      "prompt_text_token_price": 12500,
      "cached_prompt_text_token_price": 2000,
      "prompt_image_token_price": 12500,
      "completion_text_token_price": 25000
    },
    {
      "id": "grok-420-reasoning",
      "aliases": [],
      "context_length": 256000,
      "created": 1768003200,
      "object": "model",
      "owned_by": "xai",
      "prompt_text_token_price": 20000,
      "cached_prompt_text_token_price": 2000,
      "prompt_image_token_price": 0,
      "completion_text_token_price": 80000,
      "prompt_text_token_price_long_context": 40000,
      "completion_text_token_price_long_context": 160000,
      "long_context_threshold": 128000
    },
    {
      "id": "grok-imagine-image",
      "aliases": [],
      "context_length": 1024,
      "created": 1769472000,
      "object": "model",
      "owned_by": "xai",
      "image_price": 200000000
    }
  ],
  "object": "list"
}
```

***

## GET /v1/models/\{model\_id}

Get information about a model with its model\_id, including pricing.

### Path Parameters

* `model_id` (string, required) — ID of the model to get.

### Response Body

* `aliases` (array\<string>, required) — Alias ID(s) of the model that user can use in a request's model field.

* `cached_prompt_text_token_price` (integer | null) — Price of a prompt text token (in USD cents per 100 million tokens) that was cached previously.

* `cached_prompt_text_token_price_long_context` (integer | null) — Price of the cached prompt text token for long context requests (USD cents per 100 million tokens).

* `completion_text_token_price` (integer | null) — Price of the completion text token in USD cents per 100 million tokens.

* `completion_text_token_price_long_context` (integer | null) — Price of the completion text token for long context requests (USD cents per 100 million tokens).

* `context_length` (integer | null) — The maximum context length supported by the model, in tokens.

* `created` (integer, required) — Model creation time in Unix timestamp.

* `id` (string, required) — Model ID. Obtainable from \<https://console.x.ai/team/default/models> or \<https://docs.x.ai/docs/models>.

* `image_price` (integer | null) — Price per image in USD cents (image generation models).

* `long_context_threshold` (integer | null) — Token count at or above which the long context prices apply.

* `object` (string, required) — The object type, which is always \`"model"\`.

* `owned_by` (string, required) — Owner of the model.

* `prompt_image_token_price` (integer | null) — Price of the prompt image token in USD cents per 100 million tokens.

* `prompt_text_token_price` (integer | null) — Price of the prompt text token in USD cents per 100 million tokens.

* `prompt_text_token_price_long_context` (integer | null) — Price of the prompt text token for long context requests (USD cents per 100 million tokens).

\*\*Response example:\*\*

```json
{
  "id": "latest",
  "created": 1776556800,
  "object": "model",
  "owned_by": "xai",
  "prompt_text_token_price": 12500,
  "cached_prompt_text_token_price": 2000,
  "prompt_image_token_price": 12500,
  "completion_text_token_price": 25000
}
```

***

## GET /v1/language-models

List all chat and image understanding models available to the authenticating API key with full information. Additional information compared to /v1/models includes modalities, fingerprint and alias(es).

### Response Body

* `models` (array\<object>, required) — Array of available language models.

  * `aliases` (array\<string>, required) — Alias ID(s) of the model that user can use in a request's model field.

  * `cached_prompt_text_token_price` (integer, required) — Price of a prompt text token (in USD cents per 100 million tokens) that was cached previously.

  * `cached_prompt_text_token_price_long_context` (integer, required) — Price of the cached prompt text token for long context requests (USD cents per 100 million tokens).
    When 0, falls back to cached\_prompt\_text\_token\_price.

  * `completion_text_token_price` (integer, required) — Price of the completion text token in USD cents per 100 million token.

  * `completion_text_token_price_long_context` (integer, required) — Price of the completion text token for long context requests (USD cents per 100 million tokens).
    When 0, the standard completion\_text\_token\_price applies.

  * `created` (integer, required) — Creation time of the model in Unix timestamp.

  * `fingerprint` (string, required) — Fingerprint of the xAI system configuration hosting the model.

  * `id` (string, required) — Model ID. Obtainable from \<https://console.x.ai/team/default/models> or \<https://docs.x.ai/docs/models>.

  * `input_modalities` (array\<string>, required) — The input modalities supported by the model, e.g. \`"text"\`, \`"image"\`.

  * `long_context_threshold` (integer, required) — Token count at or above which the long context prices apply.
    When 0, the model has no long context pricing tier.

  * `object` (string, required) — The object type, which is always \`"model"\`.

  * `output_modalities` (array\<string>, required) — The output modalities supported by the model, e.g. \`"text"\`, \`"image"\`.

  * `owned_by` (string, required) — Owner of the model.

  * `prompt_image_token_price` (integer, required) — Price of the prompt image token in USD cents per 100 million token.

  * `prompt_text_token_price` (integer, required) — Price of the prompt text token in USD cents per 100 million token.

  * `prompt_text_token_price_long_context` (integer, required) — Price of the prompt text token for long context requests (USD cents per 100 million tokens).
    When 0, the standard prompt\_text\_token\_price applies at all context lengths.

  * `search_price` (integer, required) — Price of the search in USD cents per 100 million searches.

  * `version` (string, required) — Version of the model.

\*\*Response example:\*\*

```json
{
  "models": [
    {
      "id": "latest",
      "fingerprint": "fp_777a9f8466",
      "created": 1776556800,
      "object": "model",
      "owned_by": "xai",
      "version": "1.0",
      "input_modalities": [
        "text",
        "image"
      ],
      "output_modalities": [
        "text"
      ],
      "prompt_text_token_price": 12500,
      "cached_prompt_text_token_price": 2000,
      "prompt_image_token_price": 12500,
      "completion_text_token_price": 25000,
      "search_price": 0,
      "prompt_text_token_price_long_context": 0,
      "cached_prompt_text_token_price_long_context": 0,
      "completion_text_token_price_long_context": 0,
      "long_context_threshold": 0,
      "aliases": [
        "grok-4.3-latest",
        "grok-latest"
      ]
    },
    {
      "id": "grok-420-reasoning",
      "fingerprint": "fp_5319828d69",
      "created": 1768003200,
      "object": "model",
      "owned_by": "xai",
      "version": "1.0",
      "input_modalities": [
        "text"
      ],
      "output_modalities": [
        "text"
      ],
      "prompt_text_token_price": 20000,
      "cached_prompt_text_token_price": 2000,
      "prompt_image_token_price": 0,
      "completion_text_token_price": 80000,
      "search_price": 0,
      "prompt_text_token_price_long_context": 40000,
      "cached_prompt_text_token_price_long_context": 0,
      "completion_text_token_price_long_context": 160000,
      "long_context_threshold": 128000,
      "aliases": []
    }
  ]
}
```

***

## GET /v1/language-models/\{model\_id}

Get full information about a chat or image understanding model with its model\_id.

### Path Parameters

* `model_id` (string, required) — ID of the model to get.

### Response Body

* `aliases` (array\<string>, required) — Alias ID(s) of the model that user can use in a request's model field.

* `cached_prompt_text_token_price` (integer, required) — Price of a prompt text token (in USD cents per 100 million tokens) that was cached previously.

* `cached_prompt_text_token_price_long_context` (integer, required) — Price of the cached prompt text token for long context requests (USD cents per 100 million tokens).
  When 0, falls back to cached\_prompt\_text\_token\_price.

* `completion_text_token_price` (integer, required) — Price of the completion text token in USD cents per 100 million token.

* `completion_text_token_price_long_context` (integer, required) — Price of the completion text token for long context requests (USD cents per 100 million tokens).
  When 0, the standard completion\_text\_token\_price applies.

* `created` (integer, required) — Creation time of the model in Unix timestamp.

* `fingerprint` (string, required) — Fingerprint of the xAI system configuration hosting the model.

* `id` (string, required) — Model ID. Obtainable from \<https://console.x.ai/team/default/models> or \<https://docs.x.ai/docs/models>.

* `input_modalities` (array\<string>, required) — The input modalities supported by the model, e.g. \`"text"\`, \`"image"\`.

* `long_context_threshold` (integer, required) — Token count at or above which the long context prices apply.
  When 0, the model has no long context pricing tier.

* `object` (string, required) — The object type, which is always \`"model"\`.

* `output_modalities` (array\<string>, required) — The output modalities supported by the model, e.g. \`"text"\`, \`"image"\`.

* `owned_by` (string, required) — Owner of the model.

* `prompt_image_token_price` (integer, required) — Price of the prompt image token in USD cents per 100 million token.

* `prompt_text_token_price` (integer, required) — Price of the prompt text token in USD cents per 100 million token.

* `prompt_text_token_price_long_context` (integer, required) — Price of the prompt text token for long context requests (USD cents per 100 million tokens).
  When 0, the standard prompt\_text\_token\_price applies at all context lengths.

* `search_price` (integer, required) — Price of the search in USD cents per 100 million searches.

* `version` (string, required) — Version of the model.

\*\*Response example:\*\*

```json
{
  "id": "latest",
  "fingerprint": "fp_156d35dcaa",
  "created": 1743724800,
  "object": "model",
  "owned_by": "xai",
  "version": "1.0.0",
  "input_modalities": [
    "text"
  ],
  "output_modalities": [
    "text"
  ],
  "prompt_text_token_price": 20000,
  "cached_prompt_text_token_price": 0,
  "prompt_image_token_price": 0,
  "completion_text_token_price": 100000,
  "aliases": [
    "grok-4",
    "grok-4-latest"
  ]
}
```

***

## GET /v1/image-generation-models

List all image generation models available to the authenticating API key with full information. Additional information compared to /v1/models includes modalities, fingerprint and alias(es).

### Response Body

* `models` (array\<object>, required) — Array of available image generation models.

  * `aliases` (array\<string>, required) — Alias ID(s) of the model that user can use in a request's model field.

  * `created` (integer, required) — Model creation time in Unix timestamp.

  * `fingerprint` (string, required) — Fingerprint of the xAI system configuration hosting the model.

  * `id` (string, required) — Model ID.

  * `image_price` (integer, required) — Price of a single image in USD cents.

  * `input_modalities` (array\<string>, required) — The input modalities supported by the model.

  * `max_prompt_length` (integer, required)

  * `object` (string, required) — The object type, which is always \`"model"\`.

  * `output_modalities` (array\<string>, required) — The output modalities supported by the model.

  * `owned_by` (string, required) — Owner of the model.

  * `version` (string, required) — Version of the model.

\*\*Response example:\*\*

```json
{
  "models": [
    {
      "id": "grok-imagine-image",
      "fingerprint": "fp_ca78641a52",
      "max_prompt_length": 1024,
      "created": 1738961600,
      "object": "model",
      "owned_by": "xai",
      "version": "1.0.0",
      "prompt_text_token_price": 100000,
      "prompt_image_token_price": 100000,
      "generated_image_token_price": 100000,
      "aliases": []
    }
  ]
}
```

***

## GET /v1/image-generation-models/\{model\_id}

Get full information about an image generation model with its model\_id.

### Path Parameters

* `model_id` (string, required) — ID of the model to get.

### Response Body

* `aliases` (array\<string>, required) — Alias ID(s) of the model that user can use in a request's model field.

* `created` (integer, required) — Model creation time in Unix timestamp.

* `fingerprint` (string, required) — Fingerprint of the xAI system configuration hosting the model.

* `id` (string, required) — Model ID.

* `image_price` (integer, required) — Price of a single image in USD cents.

* `input_modalities` (array\<string>, required) — The input modalities supported by the model.

* `max_prompt_length` (integer, required)

* `object` (string, required) — The object type, which is always \`"model"\`.

* `output_modalities` (array\<string>, required) — The output modalities supported by the model.

* `owned_by` (string, required) — Owner of the model.

* `version` (string, required) — Version of the model.

\*\*Response example:\*\*

```json
{
  "id": "grok-imagine-image",
  "fingerprint": "fp_ca78641a52",
  "max_prompt_length": 1024,
  "created": 1737961600,
  "object": "model",
  "owned_by": "xai",
  "version": "1.0.0",
  "prompt_text_token_price": 100000,
  "prompt_image_token_price": 100000,
  "generated_image_token_price": 100000,
  "aliases": []
}
```

***

## GET /v1/video-generation-models

List all video generation models available to the authenticating API key with full information.

### Response Body

* `models` (array\<object>, required) — Array of available video generation models.

  * `aliases` (array\<string>, required) — Alias ID(s) of the model that user can use in a request's model field.

  * `created` (integer, required) — Model creation time in Unix timestamp.

  * `fingerprint` (string, required) — Fingerprint of the xAI system configuration hosting the model.

  * `id` (string, required) — Model ID.

  * `input_modalities` (array\<string>, required) — The input modalities supported by the model (e.g. "text", "image").

  * `object` (string, required) — The object type, which is always \`"model"\`.

  * `output_modalities` (array\<string>, required) — The output modalities supported by the model (e.g. "video").

  * `owned_by` (string, required) — Owner of the model.

  * `version` (string, required) — Version of the model.

\*\*Response example:\*\*

```json
{
  "models": [
    {
      "id": "grok-imagine-video",
      "fingerprint": "fp_898ae9f31c",
      "created": 1743724800,
      "object": "model",
      "owned_by": "xai",
      "version": "1.0.0",
      "input_modalities": [
        "text",
        "image"
      ],
      "output_modalities": [
        "video"
      ],
      "aliases": []
    }
  ]
}
```

***

## GET /v1/video-generation-models/\{model\_id}

Get full information about a video generation model with its model\_id.

### Path Parameters

* `model_id` (string, required) — ID of the model to get.

### Response Body

* `aliases` (array\<string>, required) — Alias ID(s) of the model that user can use in a request's model field.

* `created` (integer, required) — Model creation time in Unix timestamp.

* `fingerprint` (string, required) — Fingerprint of the xAI system configuration hosting the model.

* `id` (string, required) — Model ID.

* `input_modalities` (array\<string>, required) — The input modalities supported by the model (e.g. "text", "image").

* `object` (string, required) — The object type, which is always \`"model"\`.

* `output_modalities` (array\<string>, required) — The output modalities supported by the model (e.g. "video").

* `owned_by` (string, required) — Owner of the model.

* `version` (string, required) — Version of the model.

\*\*Response example:\*\*

```json
{
  "id": "grok-imagine-video",
  "fingerprint": "fp_898ae9f31c",
  "created": 1743724800,
  "object": "model",
  "owned_by": "xai",
  "version": "1.0",
  "input_modalities": [
    "text",
    "image"
  ],
  "output_modalities": [
    "video"
  ],
  "aliases": []
}
```
