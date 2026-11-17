#### API Reference

# Overview

The SpaceXSI REST API is compatible with the OpenAI REST API. This reference is generated from the OpenAPI specification and organised by resource.

## Base URLs and authentication

| API | Base URL | Authenticate with |
| --- | --- | --- |
| Inference (responses, chat completions, images, videos, voice, files, batches, models) | `https://api.x.ai` | `Authorization: Bearer \<API key\>` |
| Collections management | `https://management-api.x.ai` | `Authorization: Bearer \<Management API key\>` |
| Collections search | `https://api.x.ai` | `Authorization: Bearer \<API key\>` |
| Management (API keys, teams, billing, audit) | `https://management-api.x.ai` | `Authorization: Bearer \<Management API key\>` |

API keys are created on the [API Keys page](https://console.x.ai/team/default/api-keys?utm_source=docs\&utm_medium=referral\&utm_campaign=developers-rest-api-reference-inference\&utm_content=api-keys) of the SpaceXSI Console. Management keys are created on the [Management Keys page](https://console.x.ai/team/default/management-keys?utm_source=docs\&utm_medium=referral\&utm_campaign=developers-rest-api-reference-inference\&utm_content=management-keys); see [Using Management API](/developers/management-api-guide).

## Inference API

* [Responses](/developers/rest-api-reference/inference/responses)
* [Chat Completions](/developers/rest-api-reference/inference/chat-completions)
* [Images](/developers/rest-api-reference/inference/images)
* [Videos](/developers/rest-api-reference/inference/videos)
* [Voice](/developers/rest-api-reference/inference/voice)
* [Files](/developers/rest-api-reference/files)
* [Batches](/developers/rest-api-reference/inference/batches)
* [Models](/developers/rest-api-reference/inference/models)
* [Account](/developers/rest-api-reference/inference/other)
* [Legacy & Deprecated](/developers/rest-api-reference/inference/legacy)

## Management-key APIs

* [Collections API](/developers/rest-api-reference/collections)
* [Management API](/developers/rest-api-reference/management)

For status codes and their likely causes, see [Debugging Errors](/developers/debugging).

