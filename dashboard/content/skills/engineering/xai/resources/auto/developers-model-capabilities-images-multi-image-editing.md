+++
title = "developers-model-capabilities-images-multi-image-editing"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Model Capabilities

# Multi-Image Editing

Use up to three source images for a single image edit. You can specify images in the order they are sent in the request. By default, the output aspect ratio follows the first input image. You can override this by setting the `aspect_ratio` parameter to a specific ratio, such as `"1:1"` or `"16:9"`.

Each source image can be a public URL, a base64-encoded data URI, or a `file_id` from the [Files API](https://docs.x.ai/developers/files) — and you can mix kinds within a single request. See [Imagine → Files API Integration](https://docs.x.ai/developers/model-capabilities/imagine/files/inputs) for `file_id` details and examples.

## Related

* [Image Generation](https://docs.x.ai/developers/model-capabilities/images/generation) — Generate images from text prompts
* [Image Editing](https://docs.x.ai/developers/model-capabilities/images/editing) — Edit a source image with natural language
* [API Reference](https://docs.x.ai/developers/rest-api-reference) — Full endpoint documentation
* [Imagine API Landing Page](https://x.ai/api/imagine) — Showcase of the Imagine API in action
{% endraw %}
