+++
title = "developers-model-capabilities-video-image-to-video"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Model Capabilities

# Image-to-Video

Transform a still image into a video by providing a source image along with an optional prompt. The model animates the image content based on your instructions. On `grok-imagine-video-1.5`, image-to-video supports native 1080p.

You can provide the source image as:

* A **public URL** pointing to an image
* A **base64-encoded data URI** (e.g., `data:image/jpeg;base64,...`)
* A **`file_id`** from the [Files API](https://docs.x.ai/developers/files) — see [Imagine → Files API Integration](https://docs.x.ai/developers/model-capabilities/imagine/files/inputs)

The demo below shows this in action; hold to animate a still image:

In the Vercel AI SDK, the `prompt` parameter accepts an object with `image` and `text` fields for image-to-video generation. The `image` field can be a URL string, base64-encoded string, `Uint8Array`, `ArrayBuffer`, or `Buffer`.

## Related

* [Video Generation](https://docs.x.ai/developers/model-capabilities/video/generation) — Generate videos from text prompts
* [Reference-to-Video](https://docs.x.ai/developers/model-capabilities/video/reference-to-video) — Guide a video with reference images
* [Model page: grok-imagine-video-1.5](https://docs.x.ai/developers/models/grok-imagine-video-1.5)
* [Videos API](https://docs.x.ai/developers/rest-api-reference/inference/videos) — Video generation endpoints
* [Video Editing](https://docs.x.ai/developers/model-capabilities/video/editing) — Edit existing videos
* [API Reference](https://docs.x.ai/developers/rest-api-reference) — Full endpoint documentation
* [Imagine API Landing Page](https://x.ai/api/imagine) — Showcase of the Imagine API in action
{% endraw %}
