+++
title = "developers-model-capabilities-imagine-files"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Model Capabilities

# Files API Integration

The Imagine API integrates with the [Files API](https://docs.x.ai/developers/files) in two directions:

* **Inputs** — anywhere an Imagine endpoint accepts a public URL or base64-encoded image/video, you can substitute a `file_id` from your Files storage. No need to make the file public or re-upload bytes on every call.
* **Outputs** — Imagine can persist the generated asset to your Files storage and optionally create a permanent, shareable public URL for it, alongside the ephemeral generation URL that's always returned by default.

By default, Imagine requests return an **ephemeral** URL on `imgen.x.ai` or `vidgen.x.ai` that's only good for fetching the asset shortly after generation. The Files integration lets you both reuse stored inputs and persist outputs in the same request.

> [!TIP]
>
> Need to make an already uploaded file public, or want fine-grained control over create/revoke flows? See the standalone [Files → Public URLs](https://docs.x.ai/developers/files/public-urls) docs.

## Putting it together: an iterative loop

The clearest payoff for using inputs and outputs together is chaining Imagine calls without ever leaving the Files API. Reference a stored `file_id` directly from a previous call's `file_output` — no need to download, re-upload, or make the file public.

```python customLanguage="pythonXAI"
import os
import xai_sdk

client = xai_sdk.Client(api_key=os.getenv("XAI_API_KEY"))

# 1. Generate an image and store it
gen = client.image.sample(
    prompt="A futuristic city skyline at night",
    model="grok-imagine-image-quality",
    storage_options={"filename": "city.jpg"},  # store privately, no public URL needed
)
city = gen.file_output.file_id

# 2. Edit the stored image without re-uploading
edit = client.image.sample(
    prompt="Add neon signs to the buildings",
    model="grok-imagine-image-quality",
    image_file_id=city,
    storage_options={"filename": "city-neon.jpg"},
)

# 3. Animate the edited result into a video
vid = client.video.generate(
    prompt="A camera pulls back through the city",
    model="grok-imagine-video",
    duration=5,
    image_file_id=edit.file_output.file_id,
)

print(vid.url)
```

## Related

* [Referencing Files as Input](https://docs.x.ai/developers/model-capabilities/imagine/files/inputs) — Full reference + examples for the input direction.
* [Persisting Generated Output](https://docs.x.ai/developers/model-capabilities/imagine/files/outputs) — Full reference + examples for the output direction.
* [Files → Public URLs](https://docs.x.ai/developers/files/public-urls) — Public URL lifecycle for any file, regardless of how it was created.
* [Managing Files](https://docs.x.ai/developers/files/managing-files) — Upload, list, retrieve, update, and delete files.
