+++
title = "developers-model-capabilities-imagine-files-outputs"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Model Capabilities

# Persisting Generated Output

Use `storage_options` on any Imagine request to **persist the generated asset** to your [Files API](https://docs.x.ai/developers/files) storage, where you can retrieve it later through the authenticated Files API. On top of that, set `storage_options.public_url` to additionally generate a **permanent, shareable public URL** for the asset — an unauthenticated link that anyone can open and that stays alive until you revoke it.

Storage and public URL creation are independent: you can store privately without a public URL, or store with a public URL. Either way, the response also includes the ephemeral generation URL that's always returned by default.

## Quick Start

Generate an image, persist it to Files, and get a shareable public URL — all in one call:

```python customLanguage="pythonXAI"
import os
import xai_sdk

client = xai_sdk.Client(api_key=os.getenv("XAI_API_KEY"))

response = client.image.sample(
    prompt="A serene Japanese garden in winter",
    model="grok-imagine-image-quality",
    storage_options={"filename": "garden.jpg", "public_url": True},
)

# Ephemeral, short-lived URL.
print(f"Ephemeral:  {response.url}")

# Persistent file in your Files API storage.
print(f"File ID:    {response.file_output.file_id}")

# Permanent, shareable public URL.
print(f"Public URL: {response.public_url}")
```

```bash
curl -s -X POST https://api.x.ai/v1/images/generations \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-imagine-image-quality",
    "prompt": "A serene Japanese garden in winter",
    "response_format": "url",
    "storage_options": {
      "filename": "garden.jpg",
      "public_url": true
    }
  }'
```

The response body looks like this:

```json
{
  "data": [
    {
      "url": "https://imgen.x.ai/xai-imgen/xai-tmp-imgen-abc123.jpg",
      "file_output": {
        "file_id": "file_7de029f4-eb66-42ee-87f8-b2a9d9e7466a",
        "filename": "garden.jpg",
        "public_url": "https://files-cdn.x.ai/ZsqeMtdcSYWPPHTQdxXDKQ/file_7de029f4-eb66-42ee-87f8-b2a9d9e7466a.jpg"
      }
    }
  ]
}
```

Key things to note:

* `data[i].url` is the **ephemeral** Imagine URL — short-lived, fine for immediate use. It's always returned, regardless of whether `storage_options` is specified.
* `data[i].file_output.file_id` is the stable Files API identifier for the generated asset.
* `data[i].file_output.public_url` is the **permanent** public URL. Use this for sharing, embedding, and long-term storage in your app.

## `storage_options` Reference

| Field                      | Type                        | Description                                                                                                                                |
| -------------------------- | --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `filename`                 | string (**required**)       | Filename for the stored file. The extension on the public URL path is derived from this filename.                                         |
| `expires_after`            | integer (optional)          | Seconds from now until the stored file expires. Must be between `3600` (1 hour) and `2592000` (30 days). Omit for permanent storage.       |
| `public_url`               | boolean or object (optional) | Set to `true` to create a public URL with default settings, or pass an object to configure it. Omit (or `false`) to store privately.      |
| `public_url.expires_after` | integer (optional)          | Seconds from now until the public URL expires. Must be between `3600` (1h) and `2592000` (30d). See [Expiry Behaviour](#expiry-behaviour). |

`filename` is required. Passing `storage_options={"filename": "..."}` with no other fields persists the asset privately: no expiry on the stored file and no public URL. You can always call [`create_public_url`](https://docs.x.ai/developers/files/public-urls) on the stored `file_id` later if you change your mind.

```python customLanguage="pythonXAI"
response = client.image.sample(
    prompt="A red circle on a white background",
    model="grok-imagine-image-quality",
    storage_options={"filename": "circle.jpg"},  # store privately, no public URL
)

print(response.file_output.file_id)    # file_...
print(response.file_output.filename)   # circle.jpg
print(response.public_url)             # None — public URL was not requested
```

```bash
curl -s -X POST https://api.x.ai/v1/images/generations \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-imagine-image-quality",
    "prompt": "A red circle on a white background",
    "response_format": "url",
    "storage_options": {"filename": "circle.jpg"}
  }'
```

## Expiry Behaviour

`storage_options` exposes two independent expiry knobs: `storage_options.expires_after` controls when the **stored file** auto-deletes, and `storage_options.public_url.expires_after` controls when the **public URL** auto-revokes. Omit `public_url.expires_after` and the URL inherits the file's expiry (or never expires if the file has none).

A public URL can never outlive its file, and both values must be between **1 hour and 30 days**. See [Public URLs → Expiry Behaviour](https://docs.x.ai/developers/files/public-urls#expiry-behaviour) for the full rules; the examples below show how the two knobs combine on an Imagine request.

```python customLanguage="pythonXAI"
import os
import xai_sdk

client = xai_sdk.Client(api_key=os.getenv("XAI_API_KEY"))

# Permanent file, public URL expires in 24h.
response = client.image.sample(
    prompt="A futuristic city skyline at night",
    model="grok-imagine-image-quality",
    storage_options={"filename": "skyline.jpg", "public_url": {"expires_after": 86400}},  # 24h
)
print(response.file_output.file_id)                # file_...
print(response.public_url)                         # https://files-cdn.x.ai/<token>/file_....jpg
print(response.file_output.public_url_expires_at)  # protobuf Timestamp, ~24h from now

# File and public URL both expire in 2h (URL inherits the file's expiry).
response = client.image.sample(
    prompt="A futuristic city skyline at night",
    model="grok-imagine-image-quality",
    storage_options={"filename": "skyline.jpg", "expires_after": 7200, "public_url": True},
)
print(response.file_output.file_id)                # file_...
print(response.public_url)
print(response.file_output.expires_at)             # ~2h from now
print(response.file_output.public_url_expires_at)  # matches file's expires_at

# File expires in 24h, public URL expires in 1h (independent, shorter).
response = client.image.sample(
    prompt="A futuristic city skyline at night",
    model="grok-imagine-image-quality",
    storage_options={
        "filename": "skyline.jpg",
        "expires_after": 86400,
        "public_url": {"expires_after": 3600},
    },
)
print(response.file_output.file_id)                # file_...
print(response.public_url)
print(response.file_output.expires_at)             # ~24h from now
print(response.file_output.public_url_expires_at)  # ~1h from now (URL dies before file)
```

```bash
# Permanent file, 24h public URL
curl -s -X POST https://api.x.ai/v1/images/generations \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-imagine-image-quality",
    "prompt": "A futuristic city skyline at night",
    "response_format": "url",
    "storage_options": {
      "filename": "skyline.jpg",
      "public_url": {"expires_after": 86400}
    }
  }'

# 2h file, public URL inherits the same expiry
curl -s -X POST https://api.x.ai/v1/images/generations \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-imagine-image-quality",
    "prompt": "A futuristic city skyline at night",
    "response_format": "url",
    "storage_options": {
      "filename": "skyline.jpg",
      "expires_after": 7200,
      "public_url": true
    }
  }'
```

## The `file_output` Response

Every Imagine response with `storage_options` set includes a `file_output` block on each generated asset:

| Field                   | Always present                                                        | Meaning                                                                                                                     |
| ----------------------- | --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `file_id`               | yes                                                                   | Stable Files API identifier. Use this for `client.files.*` operations.                                                      |
| `filename`              | yes                                                                   | The filename you provided in `storage_options`.                                                                            |
| `expires_at`            | only when the file has an expiration set                              | Unix timestamp when the file expires.                                                                                       |
| `public_url`            | only when `storage_options.public_url` was set and creation succeeded | The permanent shareable URL.                                                                                                |
| `public_url_expires_at` | only when the public URL has an expiry                                | Unix timestamp when the public URL dies. Absent for permanent URLs.                                                         |
| `public_url_error`      | only on partial failure                                               | Human-readable error when the asset was stored but public URL creation failed. See [Public URL Errors](#public-url-errors). |

The Python SDK also surfaces `response.public_url` and `response.public_url_error` as top-level shortcuts on `ImageResponse` and `VideoResponse`.

## Multiple Outputs (`n > 1`)

When you request multiple images in a single call, each image gets its own `file_id` and its own `public_url` with a unique token. The files are completely independent — revoking or deleting one does not affect the others.

```python customLanguage="pythonXAI"
import os
import xai_sdk

client = xai_sdk.Client(api_key=os.getenv("XAI_API_KEY"))

responses = client.image.sample_batch(
    prompt="A cat wearing a hat, four different art styles",
    model="grok-imagine-image-quality",
    n=4,
    storage_options={"filename": "cat-styles.jpg", "public_url": True},
)

for r in responses:
    print(r.file_output.file_id, r.public_url)
```

```bash
curl -s -X POST https://api.x.ai/v1/images/generations \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-imagine-image-quality",
    "prompt": "A cat wearing a hat, four different art styles",
    "n": 4,
    "response_format": "url",
    "storage_options": {"filename": "cat-styles.jpg", "public_url": true}
  }'
```

## Storing Image Edit Outputs

`storage_options` works on `/v1/images/edits` the same way as `/v1/images/generations`. The edited result is stored as a new file.

```python customLanguage="pythonXAI"
import os
import xai_sdk

client = xai_sdk.Client(api_key=os.getenv("XAI_API_KEY"))

response = client.image.sample(
    prompt="Add a party hat to the dog",
    model="grok-imagine-image-quality",
    image_url="https://docs.x.ai/assets/api-examples/images/style-realistic.png",
    storage_options={"filename": "party-dog.png", "public_url": True},
)

print(response.file_output.file_id)  # new file, not the input
print(response.public_url)
```

```bash
curl -s -X POST https://api.x.ai/v1/images/edits \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-imagine-image-quality",
    "prompt": "Add a party hat to the dog",
    "image": {
      "url": "https://docs.x.ai/assets/api-examples/images/style-realistic.png"
    },
    "response_format": "url",
    "storage_options": {"filename": "party-dog.png", "public_url": true}
  }'
```

## Storing Video Outputs

The video endpoints (`/v1/videos/generations`, `/v1/videos/edits`, `/v1/videos/extensions`) use the same `storage_options` shape. Since video generation is asynchronous, `file_output.public_url` is populated on the **completed** response after the video finishes generating.

```python customLanguage="pythonXAI"
import os
import xai_sdk

client = xai_sdk.Client(api_key=os.getenv("XAI_API_KEY"))

# SDK handles polling automatically and returns the completed video.
response = client.video.generate(
    prompt="A ball bouncing slowly on a flat surface",
    model="grok-imagine-video",
    duration=5,
    storage_options={"filename": "bouncing-ball.mp4", "public_url": True},
)

print(response.url)                   # ephemeral vidgen URL
print(response.file_output.file_id)   # file_...
print(response.public_url)            # https://files-cdn.x.ai/<token>/file_....mp4
```

```bash
# Start the generation
REQUEST_ID=$(curl -s -X POST https://api.x.ai/v1/videos/generations \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "grok-imagine-video",
    "prompt": "A ball bouncing slowly on a flat surface",
    "duration": 5,
    "storage_options": {"filename": "bouncing-ball.mp4", "public_url": true}
  }' | jq -r '.request_id')

# Poll until done
while true; do
  RESULT=$(curl -s "https://api.x.ai/v1/videos/$REQUEST_ID" \
    -H "Authorization: Bearer $XAI_API_KEY")
  STATUS=$(echo "$RESULT" | jq -r '.status')
  if [ "$STATUS" = "done" ]; then
    echo "$RESULT" | jq '.video.file_output'
    break
  fi
  sleep 5
done
```

Image-to-video, video editing, and video extension all accept `storage_options` the same way.

## Public URL Errors

In rare cases, public URL creation can fail even when asset generation and storage succeed — for example, on a transient infrastructure issue or when the team has hit its [active-URL quota](https://docs.x.ai/developers/files/public-urls#limitations). Rather than tearing down the whole request, the response still includes a valid `file_output.file_id` and the original asset URL, and only `public_url` is replaced by `public_url_error`:

```json
{
  "data": [
    {
      "url": "https://imgen.x.ai/.../xai-tmp-imgen-abc123.jpg",
      "file_output": {
        "file_id": "file_abc123",
        "filename": "poster.jpg",
        "public_url_error": "Public URL creation timed out. The file was stored successfully."
      }
    }
  ]
}
```

If you see `public_url_error`, the file is still in your storage — you can retry public URL creation directly via the [Files API](https://docs.x.ai/developers/files/public-urls) without regenerating the asset:

```python customLanguage="pythonXAI"
import os
import xai_sdk

client = xai_sdk.Client(api_key=os.getenv("XAI_API_KEY"))

response = client.image.sample(
    prompt="A vintage poster",
    model="grok-imagine-image-quality",
    storage_options={"filename": "poster.jpg", "public_url": True},
)

if response.public_url_error:
    # The image is stored — just retry the public URL on the file directly.
    resp = client.files.create_public_url(response.file_output.file_id)
    public_url = resp.public_url
else:
    public_url = response.public_url

print(public_url)
```

## Managing Files Created via Imagine

Files created through `storage_options` are full first-class Files API files. Use the [Files API](https://docs.x.ai/developers/files/managing-files) to list, retrieve, update, and delete them, and use the public URL endpoints to revoke or re-create the URL after the fact:

```python customLanguage="pythonXAI"
import os
import xai_sdk

client = xai_sdk.Client(api_key=os.getenv("XAI_API_KEY"))

response = client.image.sample(
    prompt="A futuristic city",
    model="grok-imagine-image-quality",
    storage_options={"filename": "city.jpg", "public_url": True},
)
file_id = response.file_output.file_id

# Inspect file metadata
file = client.files.get(file_id)

# Stop sharing publicly (keeps the file in your storage)
client.files.revoke_public_url(file_id)

# Generate a new public URL later with a different expiry
client.files.create_public_url(file_id, expires_after=604800)  # 7 days

# Delete the file entirely (also tears down any active public URL)
client.files.delete(file_id)
```

```bash
FILE_ID="<file_output.file_id from the generation response>"

# Stop sharing publicly (file stays in your storage)
curl -s -X POST "https://api.x.ai/v1/files/$FILE_ID/public-url/revoke" \
  -H "Authorization: Bearer $XAI_API_KEY"

# Re-create with a 7-day expiry
curl -s -X POST "https://api.x.ai/v1/files/$FILE_ID/public-url" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"expires_after": 604800}'

# Delete the file (also revokes the public URL)
curl -s -X DELETE "https://api.x.ai/v1/files/$FILE_ID" \
  -H "Authorization: Bearer $XAI_API_KEY"
```

## Limitations

* **Up to 1,000 active public URLs per team.** Hitting the cap sets `public_url_error` on the response; revoke unused URLs to free up slots.
* **Expiry constraints** (see [Expiry Behaviour](#expiry-behaviour)):
  * Both `expires_after` values must be between **1 hour and 30 days**.
  * `public_url.expires_after` must be ≤ the file's `expires_after`.
* **Custom filenames affect the public URL path.** Passing `storage_options.filename = "my-cover.png"` makes the public URL end in `.png`. The stored content type is still determined by the generated asset, not the filename.
* **`response_format` does not affect storage.** `storage_options` runs whether you request `url` or `b64_json`.
* **Public URLs are independent of the ephemeral URL.** Both are returned in the same response but have separate lifecycles. Revoking the public URL does not affect the ephemeral URL.
* **All validation is synchronous.** Invalid storage configs are rejected before generation starts, so you never waste compute on a malformed request.

## Related

* [Files API Integration](https://docs.x.ai/developers/model-capabilities/imagine/files) — Overview + capstone example showing inputs and outputs together.
* [Referencing Files as Input](https://docs.x.ai/developers/model-capabilities/imagine/files/inputs) — The input side: pass stored `file_id`s in place of URLs.
* [Files → Public URLs](https://docs.x.ai/developers/files/public-urls) — Public URL lifecycle for any file, regardless of how it was created.
* [Managing Files](https://docs.x.ai/developers/files/managing-files) — Upload, list, retrieve, update, and delete files.
{% endraw %}
